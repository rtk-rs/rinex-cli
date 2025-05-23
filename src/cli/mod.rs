use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    str::FromStr,
};

use anise::{
    constants::usual_planetary_constants::MEAN_EARTH_ANGULAR_VELOCITY_DEG_S,
    math::Vector6,
    prelude::{Frame, Orbit},
};

use itertools::Itertools;

use clap::{value_parser, Arg, ArgAction, ArgMatches, ColorChoice, Command};

use gnss_qc::prelude::{QcConfig, QcContext, QcReportType};
use rinex::prelude::Epoch;

mod fops;
mod positioning;
mod workspace;

pub use workspace::Workspace;

use fops::{cbin, diff, filegen, merge, split, tbin};

pub struct Cli {
    /// Arguments passed by user
    pub matches: ArgMatches,
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

/// Context defined by User.
pub struct Context {
    /// Quiet option
    pub quiet: bool,

    /// Data context defined by user.
    /// In differential opmode, this is the ROVER.
    pub data: QcContext,

    /// Context name is derived from the primary file loaded in Self,
    /// and mostly used in output products generation.
    pub name: String,

    /// Workspace is the place where this session will generate data.
    /// By default it is set to $WORKSPACE/$PRIMARYFILE.
    /// $WORKSPACE is either manually definedd by CLI or we create it (as is).
    /// $PRIMARYFILE is determined from the most major file contained in the dataset.
    pub workspace: Workspace,

    #[cfg(feature = "ppp")]
    /// (RX) [Orbit] to use, whether is was automatically picked up,
    /// or manually overwritten.
    pub rx_orbit: Option<Orbit>,
}

impl Context {
    /*
     * Utility to determine the most major filename stem,
     * to be used as the session workspace
     */
    pub fn context_stem(data: &QcContext) -> String {
        let ctx_major_stem: &str = data
            .primary_path()
            .expect("failed to determine a context name")
            .file_stem()
            .expect("failed to determine a context name")
            .to_str()
            .expect("failed to determine a context name");

        /*
         * In case $FILENAME.RNX.gz gz compressed, we extract "$FILENAME".
         * Can use .file_name() once https://github.com/rust-lang/rust/issues/86319  is stabilized
         */
        let primary_stem: Vec<&str> = ctx_major_stem.split('.').collect();
        primary_stem[0].to_string()
    }

    /// Creates file within session workspace
    fn create_file(&self, path: &Path) -> std::fs::File {
        std::fs::File::create(path).unwrap_or_else(|e| {
            panic!("failed to create {}: {:?}", path.display(), e);
        })
    }
}

impl Cli {
    /// Build new command line interface
    pub fn new() -> Self {
        let cmd =
                Command::new("rinex-cli")
                    .author("Guillaume W. Bres, <guillaume.bressaix@gmail.com>")
                    .version(env!("CARGO_PKG_VERSION"))
                    .about("RINEX post processing")
                    .long_about("RINEX-Cli is the command line interface
to operate the RINEX/SP3/RTK toolkit, until a GUI is made available.
Use it to analyze data, perform file operations and resolve navigation solutions.")
                    .arg_required_else_help(true)
                    .color(ColorChoice::Always)
                    .next_help_heading("Context")
                    .arg(Arg::new("filepath")
                        .long("fp")
                        .value_name("FILE")
                        .action(ArgAction::Append)
                        .required_unless_present("directory")
                        .help("Load a single file. See --help")
                        .long_help("Use this as many times as needed. 
Available operations and following behavior highly depends on input data. 
Supported formats are:
- Observation RINEX
- Navigation RINEX
- Meteo RINEX
- Clock RINEX (high precision clocks)
- SP3 (high precision orbits)
- IONEX (Ionosphere Maps)
- ANTEX (antenna calibration as RINEX)
- DORIS (special Observation RINEX)

Example (1): Load a single file
rinex-cli \\
    --fp test_resources/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz

Example (2): define a PPP compliant context
rinex-cli \\
    --fp test_resources/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \\
    --fp test_resources/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \\
    --fp test_resources/CLK/V3/GRG0MGXFIN_20201770000_01D_30S_CLK.CLK.gz \\ 
    --fp test_resources/SP3/GRG0MGXFIN_20201770000_01D_15M_ORB.SP3.gz
"))
                    .arg(Arg::new("directory")
                        .short('d')
                        .long("dir")
                        .value_name("DIRECTORY")
                        .action(ArgAction::Append)
                        .required_unless_present("filepath")
                        .help("Directory recursivel loader. See --help.")
                        .long_help("Use this as many times as needed. Default recursive depth is set to 5,
but you can extend that with --depth. Refer to -f for more information."))
                    .arg(Arg::new("depth")
                        .long("depth")
                        .action(ArgAction::Set)
                        .required(false)
                        .value_parser(value_parser!(u8))
                        .help("Extend maximal recursive search depth of -d. The default is 5.")
                        .long_help("The default recursive depth already supports hierarchies like:
/YEAR1
     /DOY0
          /STATION1
     /DOY1
          /STATION2
/YEAR2
     /DOY0
          /STATION1"))
                    .arg(Arg::new("quiet")
                        .short('q')
                        .long("quiet")
                        .action(ArgAction::SetTrue)
                        .help("Disable all terminal output. Disables automatic report opener (Web browser)."))
                    .arg(Arg::new("workspace")
                        .short('w')
                        .long("workspace")
                        .value_name("FOLDER")
                        .value_parser(value_parser!(PathBuf))
                        .help("Define custom workspace location. See --help.")
                        .long_help("The Workspace is where Output Products are to be generated.
By default the $RINEX_WORKSPACE variable is prefered if it is defined.
You can also use this flag to customize it. 
If none are defined, we will then try to create a local directory named \"WORKSPACE\" like it is possible in this very repo."))
                .arg(Arg::new("jpl-bpc")
                    .long("jpl-bpc")
                    .action(ArgAction::SetTrue)
                    .help("Force update or request upgrade to highest precision JPL daily model.
Requires internet access on each deployment!
Once downloaded (updated) a model is valid for a couple of days or weeks, but you should regularly update
to obtain highest precision."))
        .next_help_heading("Output customization")
        .arg(
            Arg::new("output-name")
                .short('o')
                .action(ArgAction::Set)
                .help("Customize output file or report name.
In analysis opmode, report is named index.html by default, this will redefine that.
In file operations (filegen, etc..) we can manually define output filenames with this option."))
            .arg(Arg::new("rnx2crx")
                .long("rnx2crx")
                .action(ArgAction::SetTrue)
                .help("Any (Observation RINEX) output products is compressed to CRINEX"))
            .arg(Arg::new("crx2rnx")
                .long("crx2rnx")
                .action(ArgAction::SetTrue)
                .help("Any (Observation CRINEX) output products is decompressed to readable RINEX"))
        .next_help_heading("Report customization")
        .arg(
            Arg::new("report-sum")
                .long("sum")
                .action(ArgAction::SetTrue)
                .help("Restrict report to summary header only (quicker rendition)")
        )
        .arg(
            Arg::new("report-force")
                .short('f')
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Force report synthesis.
By default, report synthesis happens once per input set (file combnation and cli options).
Use this option to force report regeneration.
This has no effect on file operations that do not synthesize a report."))
        .next_help_heading("Preprocessing")
            .arg(Arg::new("gps-filter")
                .short('G')
                .action(ArgAction::SetTrue)
                .help("Filters out all GPS vehicles"))
            .arg(Arg::new("glo-filter")
                .short('R')
                .action(ArgAction::SetTrue)
                .help("Filters out all Glonass vehicles"))
            .arg(Arg::new("gal-filter")
                .short('E')
                .action(ArgAction::SetTrue)
                .help("Filters out all Galileo vehicles"))
            .arg(Arg::new("bds-filter")
                .short('C')
                .action(ArgAction::SetTrue)
                .help("Filters out all BeiDou vehicles"))
            .arg(Arg::new("bds-geo-filter")
                .long("CG")
                .action(ArgAction::SetTrue)
                .help("Filter out all BeiDou Geo vehicles"))
            .arg(Arg::new("qzss-filter")
                .short('J')
                .action(ArgAction::SetTrue)
                .help("Filters out all QZSS vehicles"))
            .arg(Arg::new("irnss-filter")
                .short('I')
                .action(ArgAction::SetTrue)
                .help("Filters out all IRNSS vehicles"))
            .arg(Arg::new("sbas-filter")
                .short('S')
                .action(ArgAction::SetTrue)
                .help("Filters out all SBAS vehicles"))
            .arg(Arg::new("preprocessing")
                .short('P')
                .num_args(1..)
                .value_delimiter(';')
                .action(ArgAction::Append)
                .help("Filter designer. Refer to []."))
            .arg(Arg::new("nav")
                .long("nav")
                .action(ArgAction::Append)
                .help("Deploy one of the navigation condition. See --help")
                .long_help("Use --nav= to apply or select particular navigation conditions.
We support the following: 
   1. --nav=healthy          retain healthy (suitable for NAV) SV only
   2. --nav=unhealthy        retain unhealthy (non suitable for NAV) SV only
   3. --nav=testing          retain in-testing (usually non suitable for NAV) SV only
   4. --nav=gps:healthy      apply (1) to GPS only
   5. --nav=bds,gps:testing  apply (3) to BDS+GPS"))
            .next_help_heading("RINEX Repair")
                .arg(Arg::new("zero-repair")
                    .short('z')
                    .action(ArgAction::SetTrue)
                    .help("Remove all zero (=null) values. See --help")
                    .long_help("
Removes all zero (null) values from data records.
Specifically in NAV and OBS RINEX. Null NAV records are forbidden.
Null OBS RINEX are also most likely invalid.
To determine whether some null Observations exist (most likely invalid), simply
generate a first report and study the provided observations.
The `ppp` solver will most likely encounter Physical Non Sense Errors.
Null NAV RINEX content is also invalid by definition."))
            .next_help_heading("Receiver Antenna")
                .arg(Arg::new("rx-ecef")
                    .long("rx-ecef")
                    .value_name("\"x,y,z\" coordinates in ECEF !!KM!!")
                    .help("Define the (RX) antenna position manually, in kilometers ECEF.
Especially if your dataset does not define such position. 
Otherwise it gets automatically picked up."))
                .arg(Arg::new("rx-geo")
                    .long("rx-geo")
                    .value_name("\"lat,lon,alt\" Units: (ddeg, ddeg, !!KM!!)")
                    .help("Define the (RX) antenna position manualy, in decimal degrees and kilometers."))
                .next_help_heading("Exclusive Opmodes: you can only run one at a time.")
                .subcommand(filegen::subcommand());

        let cmd = cmd
            .subcommand(merge::subcommand())
            .subcommand(positioning::ppp_subcommand())
            .subcommand(positioning::rtk_subcommand())
            .subcommand(split::subcommand())
            .subcommand(diff::subcommand())
            .subcommand(cbin::subcommand())
            .subcommand(tbin::subcommand());
        Self {
            matches: cmd.get_matches(),
        }
    }

    /// Recursive browser depth
    pub fn recursive_depth(&self) -> usize {
        if let Some(depth) = self.matches.get_one::<u8>("depth") {
            *depth as usize
        } else {
            5
        }
    }

    /// Returns individual input ROVER -d
    pub fn rover_directories(&self) -> Vec<&String> {
        if let Some(dirs) = self.matches.get_many::<String>("directory") {
            dirs.collect()
        } else {
            Vec::new()
        }
    }

    /// Returns individual input ROVER -fp
    pub fn rover_files(&self) -> Vec<&String> {
        if let Some(fp) = self.matches.get_many::<String>("filepath") {
            fp.collect()
        } else {
            Vec::new()
        }
    }

    /// Returns individual input BASE STATION -d
    pub fn base_station_directories(&self) -> Vec<&String> {
        match self.matches.subcommand() {
            Some(("rtk", submatches)) => {
                if let Some(dir) = submatches.get_many::<String>("dir") {
                    dir.collect()
                } else {
                    Vec::new()
                }
            },
            _ => Vec::new(),
        }
    }
    /// Returns individual input BASE STATION -fp
    pub fn base_station_files(&self) -> Vec<&String> {
        match self.matches.subcommand() {
            Some(("rtk", submatches)) => {
                if let Some(fp) = submatches.get_many::<String>("fp") {
                    fp.collect()
                } else {
                    Vec::new()
                }
            },
            _ => Vec::new(),
        }
    }

    /// Returns list of preprocessing operations
    pub fn preprocessing(&self) -> Vec<&String> {
        if let Some(filters) = self.matches.get_many::<String>("preprocessing") {
            filters.collect()
        } else {
            Vec::new()
        }
    }

    /// Returns list of NAV filters
    pub fn nav_filters(&self) -> Vec<&String> {
        if let Some(filters) = self.matches.get_many::<String>("nav") {
            filters.collect()
        } else {
            Vec::new()
        }
    }

    pub fn gps_filter(&self) -> bool {
        self.matches.get_flag("gps-filter")
    }
    pub fn glo_filter(&self) -> bool {
        self.matches.get_flag("glo-filter")
    }
    pub fn gal_filter(&self) -> bool {
        self.matches.get_flag("gal-filter")
    }
    pub fn bds_filter(&self) -> bool {
        self.matches.get_flag("bds-filter")
    }
    pub fn bds_geo_filter(&self) -> bool {
        self.matches.get_flag("bds-geo-filter")
    }
    pub fn qzss_filter(&self) -> bool {
        self.matches.get_flag("qzss-filter")
    }
    pub fn sbas_filter(&self) -> bool {
        self.matches.get_flag("sbas-filter")
    }
    pub fn irnss_filter(&self) -> bool {
        self.matches.get_flag("irnss-filter")
    }
    pub fn zero_repair(&self) -> bool {
        self.matches.get_flag("zero-repair")
    }

    /// Parse 3D coordinates (tuplets)
    fn parse_3d_coordinates(desc: &String) -> (f64, f64, f64) {
        let content = desc.split(',').collect::<Vec<&str>>();
        if content.len() < 3 {
            panic!("expecting x, y and z coordinates (3D)");
        }

        let x = f64::from_str(content[0].trim())
            .unwrap_or_else(|e| panic!("failed to parse x coordinates: {}", e));

        let y = f64::from_str(content[1].trim())
            .unwrap_or_else(|e| panic!("failed to parse y coordinates: {}", e));

        let z = f64::from_str(content[2].trim())
            .unwrap_or_else(|e| panic!("failed to parse z coordinates: {}", e));

        (x, y, z)
    }

    /// Returns possible ECEF km triplet manually defined
    fn manual_ecef_km(&self) -> Option<(f64, f64, f64)> {
        let desc = self.matches.get_one::<String>("rx-ecef")?;
        let ecef = Self::parse_3d_coordinates(desc);
        Some(ecef)
    }

    fn manual_geodetic_ddeg_ddeg_km(&self) -> Option<(f64, f64, f64)> {
        let desc = self.matches.get_one::<String>("rx-geo")?;
        let geo = Self::parse_3d_coordinates(desc);
        Some(geo)
    }

    /// Returns RX Position possibly specified by user, in km ECEF.
    pub fn manual_rx_orbit(&self, epoch: Epoch, frame: Frame) -> Option<Orbit> {
        if let Some((x0_km, y0_km, z0_km)) = self.manual_ecef_km() {
            let pos_vel = Vector6::new(x0_km, y0_km, z0_km, 0.0, 0.0, 0.0);
            Some(Orbit::from_cartesian_pos_vel(pos_vel, epoch, frame))
        } else {
            let (lat_ddeg, long_ddeg, alt_km) = self.manual_geodetic_ddeg_ddeg_km()?;
            let orbit = Orbit::try_latlongalt(
                lat_ddeg,
                long_ddeg,
                alt_km,
                MEAN_EARTH_ANGULAR_VELOCITY_DEG_S,
                epoch,
                frame,
            )
            .unwrap_or_else(|e| panic!("physical error: {}", e));
            Some(orbit)
        }
    }

    /// True if File Operations to generate data is being deployed
    pub fn is_file_operation_run(&self) -> bool {
        matches!(
            self.matches.subcommand(),
            Some(("filegen", _))
                | Some(("merge", _))
                | Some(("split", _))
                | Some(("tbin", _))
                | Some(("cbin", _))
                | Some(("diff", _))
        )
    }
    /// True if forced report synthesis is requested
    pub fn force_report_synthesis(&self) -> bool {
        self.matches.get_flag("report-force")
    }

    /// Hash all critical parameters, defining a user session uniquely
    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        let mut string = self
            .rover_directories()
            .into_iter()
            .sorted()
            .chain(self.rover_files().into_iter().sorted())
            .chain(self.preprocessing().into_iter().sorted())
            .join(",");

        if let Some(custom) = self.custom_output_name() {
            string.push_str(custom);
        }

        // if let Some(geo) = self.manual_geodetic_ddeg() {
        //     string.push_str(&format!("{:?}", geo));
        // }

        // if let Some(ecef) = self.manual_ecef_km() {
        //     string.push_str(&format!("{:?}", ecef));
        // }

        string.hash(&mut hasher);
        hasher.finish()
    }

    /// Returns QcConfig from command line
    pub fn qc_config(&self) -> QcConfig {
        QcConfig {
            report: if self.matches.get_flag("report-sum") {
                QcReportType::Summary
            } else {
                QcReportType::Full
            },
            user_rx_ecef: None,
        }
    }
    /// Customized / manually defined output to be generated
    pub fn custom_output_name(&self) -> Option<&String> {
        self.matches.get_one::<String>("output-name")
    }

    /// True if jpl_bpc_update is requested
    pub fn jpl_bpc_update(&self) -> bool {
        self.matches.get_flag("jpl-bpc")
    }

    /// Internal seamless CRNX2RNX decompression
    pub fn crnx2rnx(&self) -> bool {
        self.matches.get_flag("crx2rnx")
    }

    /// Internal seamless RNX2CRX compression
    pub fn rnx2crnx(&self) -> bool {
        self.matches.get_flag("rnx2crx")
    }
}
