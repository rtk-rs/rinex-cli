#!/bin/sh

# Application   : Ephemeris radio broadcast
# Station       : MOJN (DNK) 
# Surveying     : 24hr
# Constellation : BeiDou
# Technique     : CPP (C2I+C7I)

# Preprocessing
# This will select BeiDou + Single frequency pseudo range
# L1 pseudo range selection (mask filter)
# PRN filter example
PIPELINE="BDS;C2I,C7I;>C01"

# Discard the first two hours of that day (another example)
TIMEFRAME=">=2020-06-25T01:00:00 GPST;<2020-06-25T12:00:00 GPST"

# SPP basic configuratio
RTK_CONF=examples/CONFIG/gpst_cpp.json

# Analysis + ppp solutions
#   -f: force new report synthesis
#   -o: custom name
./target/release/rinex-cli \
    -f \
    -P $PIPELINE \
    -P "$TIMEFRAME" \
    -o "Final-BDS-CPP" \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    ppp -c $RTK_CONF --static
