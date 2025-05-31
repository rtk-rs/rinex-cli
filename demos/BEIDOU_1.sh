#! /bin/bash
CARGO=cargo
RINEX_CLI="./target/release/rinex-cli -q"

# BDS-Only
FILTER="BDS;>C11;<C30"

TESTFILE=MOJN00DNK_R_20201770000_01D
NAV_FILE=data/NAV/V3/"$TESTFILE"_MN.rnx.gz
OBS_FILE=data/CRNX/V3/"$TESTFILE"_30S_MO.crx.gz
SP3_FILE=data/SP3/D/Sta21114.sp3.gz

# start fresh
rm -rf WORKSPACE

# update
cargo build --all-features -r

# BDS Only
$RINEX_CLI \
    -P $FILTER \
    -o BDS_Only \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    ppp -c examples/CONFIG/gpst_cpp.json --static

# BDS Only
$RINEX_CLI \
    -P $FILTER \
    -o BDS_Only_Ultra \
    --fp $NAV_FILE \
    --fp $OBS_FILE \
    --fp $SP3_FILE \
    ppp -c examples/CONFIG/gpst_cpp.json --static
