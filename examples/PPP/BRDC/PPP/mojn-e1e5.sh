#!/bin/sh

# Application   : Ephemeris radio broadcast
# Station       : MOJN (DNK) 
# Surveying     : 24hr
# Constellation : Galileo
# Technique     : PPP (E1+E5)

# Preprocessing
# E1+E5 selection
# PRN filter example
PIPELINE="Gal;C1C,C5Q,L1C,L5Q;>E01"

# Discard the first two hours of that day (another example)
TIMEFRAME=">=2020-06-25T01:00:00 GPST;<2020-06-25T12:00:00 GPST"

# CPP basic configuration
RTK_CONF=examples/CONFIG/Static/gpst_ppp.json

# Analysis + ppp solutions
#   -f: force new report synthesis
#   -o: custom name
./target/release/rinex-cli \
    -f \
    -P $PIPELINE \
    -P "$TIMEFRAME" \
    -o "BRDC-Gal-PPP" \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    ppp -c $RTK_CONF
