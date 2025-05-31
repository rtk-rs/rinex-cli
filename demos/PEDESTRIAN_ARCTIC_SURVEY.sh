#! /bin/bash

###############################
# Roaming survey in Spitzberg, 
# arctic/glacier environment
###############################
RINEX_CLI="./target/release/rinex-cli -f"
CONSTELLATION="GPS,Gal"

$RINEX_CLI \
    -P $CONSTELLATION \
    --fp data/OBS/V3/2024_09_20_10_17_06.obs.gz \
    --fp data/NAV/V3/2024_09_20_10_17_06.nav \
    ppp -c examples/CONFIG/gpst_cpp.json
