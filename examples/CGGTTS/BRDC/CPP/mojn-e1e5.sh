#!/bin/sh

# Application   : CGGTTS solutions using radio navigation
# Station       : MOJN (DNK) 
# Surveying     : 24hr
# Constellation : Galileo
# Technique     : CPP (E1+E5)

# Preprocessing
# This will select Galileo + Single frequency pseudo range
# L1 pseudo range selection (mask filter)
# PRN filter example
PIPELINE="Gal;C1C,C5Q;>E01"

# Discard the first two hours of that day (another example)
TIMEFRAME=">=2020-06-25T01:00:00 GPST;<2020-06-25T23:30:00 GPST"

# CPP basic configuration
RTK_CONF=examples/CONFIG/Static/CPP/basic.json

# Analysis +cggtts solutions
#   -f: force new report synthesis
#   -o: custom name
./target/release/rinex-cli \
    -f \
    -P $PIPELINE \
    -P "$TIMEFRAME" \
    -o "BRDC-Gal-CPP" \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    ppp -c $RTK_CONF --cggtts
