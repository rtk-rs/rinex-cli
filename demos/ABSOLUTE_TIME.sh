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
NAV_FILE=data/NAV/V3/"$TESTFILE"_MN.rnx.gz
OBS_FILE=data/CRNX/V3/"$TESTFILE"_30S_MO.crx.gz

WORKSPACE=WORKSPACE/"$TESTFILE"_30S_MO

# start fresh
rm -rf WORKSPACE
rm -rf $OUTPUT_DIR

mkdir -p $OUTPUT_DIR
mkdir -p $OUTPUT_DIR/GPST
mkdir -p $OUTPUT_DIR/GST
mkdir -p $OUTPUT_DIR/UTC

# update
cargo build --all-features -r

#######################
# generate GST test data
#######################
$RINEX_CLI \
    -P $TEST_CONSTELLATIONS \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    cbin --timescale GST

cp $WORKSPACE/GPS/MOJN00DNK_R_20201770000_01D_30S_MO.crx \
    $WORKSPACE/GPS_IN_GST.crx

cp $WORKSPACE/GAL/MOJN00DNK_R_20201770000_01D_30S_MO.crx \
    $WORKSPACE/GAL_IN_GST.crx

#######################
# generate UTC test data
#######################
$RINEX_CLI \
    -P $TEST_CONSTELLATIONS \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    cbin --timescale UTC

cp $WORKSPACE/GPS/MOJN00DNK_R_20201772359_01D_30S_MO.crx \
    $WORKSPACE/GPS_IN_UTC.crx

cp $WORKSPACE/GAL/MOJN00DNK_R_20201772359_01D_30S_MO.crx \
    $WORKSPACE/GAL_IN_UTC.crx

#######################
# GPS Only (GPST)
#######################
# GPST measurements T.GPST
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GPST_GPS_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/gpst_cpp.json
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GPST_GPS_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/gpst_cpp.json

# GPST measurements T.GST
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GPST_GPS_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/gst_cpp.json
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GPST_GPS_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/gst_cpp.json

# GPST measurements T.UTC
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GPST_GPS_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/utc_cpp.json
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GPST_GPS_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/utc_cpp.json

#######################
# GPS Only (GST)
#######################
# GST measurements T.GPST
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GST_GPS_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_GST.crx \
    ppp -c examples/CONFIG/gpst_cpp.json
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GST_GPS_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_GST.crx \
    ppp --cggtts -c examples/CONFIG/gpst_cpp.json

# GST measurements T.GST
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GST_GPS_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_GST.crx \
    ppp -c examples/CONFIG/gst_cpp.json
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GST_GPS_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_GST.crx \
    ppp --cggtts -c examples/CONFIG/gst_cpp.json

# GST measurements T.UTC
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GST_GPS_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_GST.crx \
    ppp -c examples/CONFIG/utc_cpp.json
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GST_GPS_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_GST.crx \
    ppp --cggtts -c examples/CONFIG/utc_cpp.json

#######################
# GPS Only (UTC)
#######################
# UTC measurements T.GPST
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/UTC_GPS_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_UTC.crx \
    ppp -c examples/CONFIG/gpst_cpp.json
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/UTC_GPS_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_UTC.crx \
    ppp --cggtts -c examples/CONFIG/gpst_cpp.json

# UTC measurements T.GST
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/UTC_GPS_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_UTC.crx \
    ppp -c examples/CONFIG/gst_cpp.json
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/UTC_GPS_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_UTC.crx \
    ppp --cggtts -c examples/CONFIG/gst_cpp.json

# UTC measurements T.UTC
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/UTC_GPS_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_UTC.crx \
    ppp -c examples/CONFIG/utc_cpp.json
$RINEX_CLI \
    -P GPS \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GST_GPS_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_UTC.crx \
    ppp --cggtts -c examples/CONFIG/utc_cpp.json

#######################
# GAL Only (GPST)
#######################
# GPST measurements T.GPST
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GPST_GAL_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/gpst_cpp.json
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GPST_GAL_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/gpst_cpp.json

# GPST measurements T.GST
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GPST_GAL_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/gst_cpp.json
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GPST_GAL_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/gst_cpp.json

# GPST measurements T.UTC
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GPST_GAL_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/utc_cpp.json
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GPST_GAL_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/utc_cpp.json

#######################
# GAL Only (GST)
#######################
# GST measurements T.GPST
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GST_GAL_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GAL_IN_GST.crx \
    ppp -c examples/CONFIG/gpst_cpp.json
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GST_GAL_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GAL_IN_GST.crx \
    ppp --cggtts -c examples/CONFIG/gpst_cpp.json

# GST measurements T.GST
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GST_GAL_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GAL_IN_GST.crx \
    ppp -c examples/CONFIG/gst_cpp.json
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GST_GAL_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GAL_IN_GST.crx \
    ppp --cggtts -c examples/CONFIG/gst_cpp.json

# GST measurements T.UTC
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GST_GAL_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GAL_IN_GST.crx \
    ppp -c examples/CONFIG/utc_cpp.json
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GST_GAL_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GAL_IN_GST.crx \
    ppp --cggtts -c examples/CONFIG/utc_cpp.json

#######################
# GAL Only (UTC)
#######################
# UTC measurements T.GPST
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/UTC_GAL_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GAL_IN_UTC.crx \
    ppp -c examples/CONFIG/gpst_cpp.json
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/UTC_GAL_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GAL_IN_UTC.crx \
    ppp --cggtts -c examples/CONFIG/gpst_cpp.json

# UTC measurements T.GST
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/UTC_GAL_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GAL_IN_UTC.crx \
    ppp -c examples/CONFIG/gst_cpp.json
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/UTC_GAL_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GAL_IN_UTC.crx \
    ppp --cggtts -c examples/CONFIG/gst_cpp.json

# UTC measurements T.UTC
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/UTC_GAL_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GAL_IN_UTC.crx \
    ppp -c examples/CONFIG/utc_cpp.json
$RINEX_CLI \
    -P Gal \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/UTC_GAL_Only \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GAL_IN_UTC.crx \
    ppp --cggtts -c examples/CONFIG/utc_cpp.json

#######################
# GPS+GAL (GPST)
#######################
# GPST measurements T.GPST
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GPST_GPS+GAL \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/gpst_cpp.json
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GPST_GPS+GAL \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/gpst_cpp.json

# GPST measurements T.GST
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GPST_GPS+GAL \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/gst_cpp.json
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GPST_GPS+GAL \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/gst_cpp.json

# GPST measurements T.UTC
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GPST_GPS+GAL \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/utc_cpp.json
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GPST_GPS+GAL \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp --cggtts -c examples/CONFIG/utc_cpp.json

#######################
# GPS+GAL (GST)
#######################
# GST measurements T.GPST
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GST_GPS+GAL \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_GST.crx \
    --fp $WORKSPACE/GAL_IN_GST.crx \
    ppp -c examples/CONFIG/gpst_cpp.json
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/GST_GPS+GAL \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_GST.crx \
    --fp $WORKSPACE/GAL_IN_GST.crx \
    ppp --cggtts -c examples/CONFIG/gpst_cpp.json

# GST measurements T.GST
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GST_GPS+GAL \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_GST.crx \
    --fp $WORKSPACE/GAL_IN_GST.crx \
    ppp -c examples/CONFIG/gst_cpp.json
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/GST_GPS+GAL \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_GST.crx \
    --fp $WORKSPACE/GAL_IN_GST.crx \
    ppp --cggtts -c examples/CONFIG/gst_cpp.json

# GST measurements T.UTC
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GST_GPS+GAL \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_GST.crx \
    --fp $WORKSPACE/GAL_IN_GST.crx \
    ppp -c examples/CONFIG/utc_cpp.json
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/GST_GPS+GAL \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_GST.crx \
    --fp $WORKSPACE/GAL_IN_GST.crx \
    ppp --cggtts -c examples/CONFIG/utc_cpp.json

#######################
# GPS+GAL (UTC)
#######################
# UTC measurements T.GPST
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/UTC_GPS+GAL \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_UTC.crx \
    --fp $WORKSPACE/GAL_IN_UTC.crx \
    ppp -c examples/CONFIG/gpst_cpp.json
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GPST/UTC_GPS+GAL \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_UTC.crx \
    --fp $WORKSPACE/GAL_IN_UTC.crx \
    ppp --cggtts -c examples/CONFIG/gpst_cpp.json

# UTC measurements T.GST
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/UTC_GPS+GAL \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_UTC.crx \
    --fp $WORKSPACE/GAL_IN_UTC.crx \
    ppp -c examples/CONFIG/gst_cpp.json
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/GST/UTC_GPS+GAL \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_UTC.crx \
    --fp $WORKSPACE/GAL_IN_UTC.crx \
    ppp --cggtts -c examples/CONFIG/gst_cpp.json

# UTC measurements T.UTC
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/UTC_GPS+GAL \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_UTC.crx \
    --fp $WORKSPACE/GAL_IN_UTC.crx \
    ppp -c examples/CONFIG/utc_cpp.json
$RINEX_CLI \
    -P "GPS,Gal" \
    -P "$TIMEFRAME" \
    -o $OUTPUT_DIR/UTC/UTC_GPS+GAL \
    --fp $NAV_FILE \
    --fp $WORKSPACE/GPS_IN_UTC.crx \
    --fp $WORKSPACE/GAL_IN_UTC.crx \
    ppp --cggtts -c examples/CONFIG/utc_cpp.json
