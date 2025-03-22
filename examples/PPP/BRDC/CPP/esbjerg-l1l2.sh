#!/bin/sh

# Application   : Ephemeris radio broadcast
# Station       : ESBJERG (DNK) 
# Surveying     : 24hr
# Constellation : GPS
# Technique     : CPP (L1+L2)

# Preprocessing
# This will select GPS (L1+L2) pseudo range (mask filter)
# PRN filter example
PIPELINE="GPS;C1C,C2W;>G01"

# Discard the first two hours of that day (another example)
TIMEFRAME=">=2020-06-25T01:00:00 GPST;<2020-06-25T06:05:00 GPST"

# CPP basic configuratio
RTK_CONF=examples/CONFIG/CPP/basic.json

# Analysis + ppp solutions
#   -f: force new report synthesis
#   -o: custom name
./target/release/rinex-cli \
    -f \
    -q \
    -P $PIPELINE \
    -P "$TIMEFRAME" \
    -o "BRDC-GPS-CPP" \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    ppp -c $RTK_CONF
