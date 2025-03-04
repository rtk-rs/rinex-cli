#!/bin/sh

# Application   : Ephemeris radio broadcast
# Station       : ESBJERG (DNK) 
# Surveying     : 24hr
# Constellation : GPS
# Technique     : SPP

# Preprocessing
# This will select GPS + Single frequency pseudo range
# Customize this filter to select the signal to be used.
# PRN>04 is another filter example.
PIPELINE="GPS;C1C;>G04"

# Discard the first two hours of that day (another example)
TIMEFRAME=">=2020-06-25T01:00:00 GPST"

# SPP basic configuratio
RTK_CONF=examples/CONFIG/SPP/basic.json

# Analysis + ppp solutions
#   -f: force new report synthesis
#   -o: custom name
./target/release/rinex-cli \
    -f \
    -P $PIPELINE \
    -P "$TIMEFRAME" \
    -o "BRDC-GPS-SPP" \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    ppp -c $RTK_CONF
