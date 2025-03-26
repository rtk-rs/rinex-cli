#!/bin/sh

# Application   : CGGTTS solutions using radio navigation
# Station       : ESBJERG (DNK) 
# Surveying     : 24hr
# Constellation : GPS
# Technique     : SPP (L1)

# Preprocessing
# This will select GPS + Single frequency pseudo range
# L1 pseudo range selection (mask filter)
# PRN filter example
PIPELINE="GPS;C1C;>G01"

# Discard the first two hours of that day (another example)
TIMEFRAME=">=2020-06-25T00:00:00 GPST;<2020-06-25T02:00:00 GPST"

# SPP basic configuratio
RTK_CONF=examples/CONFIG/Static/SPP/basic.json

# Analysis +cggtts solutions
#   -f: force new report synthesis
#   -o: custom name
./target/release/rinex-cli \
    -q \
    -f \
    -P $PIPELINE \
    -P "$TIMEFRAME" \
    -o "BRDC-GPS-SPP" \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    ppp -c $RTK_CONF --cggtts
