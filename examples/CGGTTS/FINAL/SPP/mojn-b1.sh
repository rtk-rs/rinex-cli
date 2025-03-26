#!/bin/sh

# Application   : Final CGGTTS
# Station       : MOJN (DNK) 
# Surveying     : 24hr
# Constellation : BeiDou
# Technique     : SPP (B1)

# Preprocessing
# This will select BeiDou + Single frequency pseudo range
# L1 pseudo range selection (mask filter)
# PRN filter example
PIPELINE="BDS;C2I;>E01"

# Discard the first two hours of that day (another example)
TIMEFRAME=">=2020-06-25T01:00:00 GPST;<2020-06-25T09:30:00 GPST"

# SPP basic configuratio
RTK_CONF=examples/CONFIG/Static/SPP/basic.json

# Analysis + ppp solutions
#   -f: force new report synthesis
#   -o: custom name
./target/release/rinex-cli \
    -q \
    -f \
    -P $PIPELINE \
    -P "$TIMEFRAME" \
    -o "Final-BDS-SPP" \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/SP3/D/Sta21114.sp3.gz \
    ppp --cggtts -c $RTK_CONF
