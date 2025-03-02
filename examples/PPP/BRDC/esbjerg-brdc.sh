#!/bin/sh

# static + "real-time" navigation example
# station: ESBJERG (DNK) 
# date: 2020/06/25
# timeframe: 24hr
# Radio based ephemeris

# preprocessing
# This will select Galileo + Dual frequency pseudo range
# a PRN>05 example filter
PIPELINE="Gal;C1C,C5Q;E>05"

# Discard the first two hours of that day (example)
TIMEFRAME=">2020-06-25T01:00:00 GPST"

# NB: it is important that your -P ops correspond
# to your navigation technique.
# It is not possible to deploy CPP or PPP technique
# if you only kept E1/L1 obviously.
RTK_CONF=examples/CONFIG/SPP/basic.json

# Analysis + ppp solutions
#   -f: force new report synthesis
#   -o: custom name
rinex-cli \
    -f \
    -P $PIPELINE \
    -P "$TIMEFRAME" \
    -o "BRDC-GalE1E5" \
    --fp $DATA_DIR/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp $DATA_DIR/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    ppp -c $RTK_CONF
