#!/bin/sh

# Application   : Final CGGTTS
# Station       : ESBJERG (DNK) 
# Surveying     : 24hr
# Constellation : GPS
# Technique     : SPP (L5)

# Preprocessing
# This will select GPS + Single frequency pseudo range
# L1 pseudo range selection (mask filter)
# PRN filter example
PIPELINE="GPS;C5Q;>G01"

# Discard the first two hours of that day (another example)
TIMEFRAME=">=2020-06-25T01:00:00 GPST;<2020-06-25T09:30:00 GPST"

# SPP basic configuratio
RTK_CONF=examples/CONFIG/gpst_spp.json

# Analysis + ppp solutions
#   -f: force new report synthesis
#   -o: custom name
./target/release/rinex-cli \
    -q \
    -f \
    -P $PIPELINE \
    -P "$TIMEFRAME" \
    -o "Final-GPS-SPP" \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/SP3/C/GRG0MGXFIN_20201770000_01D_15M_ORB.SP3.gz \
    ppp --static --cggtts -c $RTK_CONF
