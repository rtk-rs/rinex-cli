#! /bin/sh

CARGO=cargo
RINEX_CLI="./target/release/rinex-cli -q"
OUTPUT_DIR=/tmp/absolutetime

# MOJDNK 2020-06-25: GPST, GST, UTC
TEST_CONSTELLATIONS="GPS,GAL"

# a big notch @ 05:04:00 GPST when using GPS.
# unexplained, not investigated & out of scope here.
TIMEFRAME="<2020-06-25T05:00:00 GPST" 

TESTFILE=MOJN00DNK_R_20201770000_01D
NAV_FILE=data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz
OBS_FILE=data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz
WORKSPACE=WORKSPACE/MOJN00DNK_R_20201770000_01D_30S_MO

mkdir -p $OUTPUT_DIR

# update
cargo build --all-features -r

# start fresh
rm -rf WORKSPACE

# generate test data
$RINEX_CLI \
    -P $TEST_CONSTELLATIONS \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    cbin --tsbin

mkdir -p $OUTPUT_DIR/GPST
mkdir -p $OUTPUT_DIR/GST
mkdir -p $OUTPUT_DIR/UTC

#######################
# GPS Only
#######################
# GPST measurements T.GPST
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GPS_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/Static/gpst_cpp.json
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GPS_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/Static/gpst_cpp.json

# GPST measurements T.GST
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GPS_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/Static/gst_cpp.json
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GPS_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/Static/gst_cpp.json

# GPST measurements T.UTC
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GPS_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/Static/utc_cpp.json
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GPS_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/Static/utc_cpp.json

#######################
# GAL Only
#######################
# GPST measurements T.GPST
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GAL_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/Static/gpst_cpp.json
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GAL_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/Static/gpst_cpp.json

# GPST measurements T.GST
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GAL_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/Static/gst_cpp.json
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GAL_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/Static/gst_cpp.json

# GPST measurements T.UTC
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GAL_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/Static/utc_cpp.json
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GAL_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/Static/utc_cpp.json

#######################
# GPS+GAL
#######################
# GPST measurements T.GPST
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GPS+GAL \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/Static/gpst_cpp.json
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GPS+GAL \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/Static/gpst_cpp.json

# GPST measurements T.GST
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GPS+GAL \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/Static/gst_cpp.json
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GPS+GAL \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/Static/gst_cpp.json

# GPST measurements T.UTC
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GPS+GAL \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/Static/utc_cpp.json
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GPS+GAL \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/Static/utc_cpp.json
