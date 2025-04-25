#! /bin/sh

RINEX_CLI="./target/release/rinex-cli -f"
CONSTELLATION=Galileo

# The sampling was static. But data is somewhat low quality,
# especially using GPS, so we have to reduce the GDOP threshold
# that we typically use. One quick way for us to do that, is to switch
# to pedestrian roaming profile.
CONFIG=examples/CONFIG/Dynamic/pedestrian_ppp.json

$RINEX_CLI \
    -P $CONSTELLATION \
    --fp data/OBS/V3/GEOP092I.24o.gz \
    --fp data/NAV/V3/CORD00ARG_R_20240920000_01D_MN.rnx.gz \
    ppp -c $CONFIG
