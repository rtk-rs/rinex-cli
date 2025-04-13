use std::str::FromStr;

use crate::Cli;
use gnss_qc::prelude::{
    Filter as QcFilter, NavFilter, Preprocessing, QcContext, Repair, RepairTrait, Rinex,
};

/// Apply all preprocessing ops described [Cli] to mutable [QcContext].
pub fn context_preprocessing(ctx: &mut QcContext, cli: &Cli) {
    let mut gnss_filters = Vec::<&str>::new();

    if cli.gps_filter() {
        gnss_filters.push("!=GPS");
        info!("GPS filtered out");
    }

    if cli.glo_filter() {
        gnss_filters.push("!=GLO");
        info!("Glonass filtered out");
    }

    if cli.gal_filter() {
        gnss_filters.push("!=Gal");
        info!("Galileo filtered out");
    }

    if cli.bds_filter() {
        gnss_filters.push("!=BDS");
        info!("BeiDou filtered out");
    }

    if cli.bds_geo_filter() {
        gnss_filters.push(">C05;<C55");
        info!("BeiDou GEO filtered out");
    }

    if cli.sbas_filter() {
        gnss_filters.push("!=geo");
        info!("SBAS filtered out..");
    }

    if cli.qzss_filter() {
        gnss_filters.push("!=QZSS");
        info!("QZSS filtered out");
    }

    if cli.irnss_filter() {
        gnss_filters.push("!=INRSSN");
        info!("IRNSS/LNAV filtered out");
    }

    for filter in gnss_filters {
        let filter = QcFilter::from_str(filter).unwrap();
        ctx.filter_mut(&filter);
    }

    // apply other filter specs
    for filt_str in cli.preprocessing() {
        let filter = QcFilter::from_str(filt_str)
            .unwrap_or_else(|e| panic!("Failed to apply filter \"{}\" - {}", filt_str, e));

        ctx.filter_mut(&filter);
        trace!("Applied \"{}\" filter", filt_str);
    }

    // apply NAV filter specs
    for filt_str in cli.nav_filters() {
        let filter = NavFilter::from_str(filt_str)
            .unwrap_or_else(|e| panic!("Failed to apply filter \"{}\" - {}", filt_str, e));

        ctx.nav_filter_mut(&filter);
        trace!("Applied \"{}\" filter", filt_str);
    }

    if cli.zero_repair() {
        info!("Repairing zero values..");
        ctx.repair_mut(Repair::Zero);
    }
}

/// Apply all preprocessing ops described [Cli] to mutable [Rinex].
pub fn rinex_preprocessing(rinex: &mut Rinex, cli: &Cli) {
    let mut gnss_filters = Vec::<&str>::new();

    if cli.gps_filter() {
        gnss_filters.push("!=GPS");
        info!("GPS filtered out");
    }

    if cli.glo_filter() {
        gnss_filters.push("!=GLO");
        info!("Glonass filtered out");
    }

    if cli.gal_filter() {
        gnss_filters.push("!=Gal");
        info!("Galileo filtered out");
    }

    if cli.bds_filter() {
        gnss_filters.push("!=BDS");
        info!("BeiDou filtered out");
    }

    if cli.bds_geo_filter() {
        gnss_filters.push(">C05;<C55");
        info!("BeiDou GEO filtered out");
    }

    if cli.sbas_filter() {
        gnss_filters.push("!=geo");
        info!("SBAS filtered out..");
    }

    if cli.qzss_filter() {
        gnss_filters.push("!=QZSS");
        info!("QZSS filtered out");
    }

    if cli.irnss_filter() {
        gnss_filters.push("!=INRSSN");
        info!("IRNSS/LNAV filtered out");
    }

    for filter in gnss_filters {
        let filter = QcFilter::from_str(filter).unwrap();
        rinex.filter_mut(&filter);
    }

    // apply other filter specs
    for filt_str in cli.preprocessing() {
        let filter = QcFilter::from_str(filt_str)
            .unwrap_or_else(|e| panic!("Failed to apply filter \"{}\" - {}", filt_str, e));

        rinex.filter_mut(&filter);
        trace!("Applied \"{}\" filter", filt_str);
    }

    if cli.zero_repair() {
        info!("Repairing zero values..");
        rinex.repair_mut(Repair::Zero);
    }
}
