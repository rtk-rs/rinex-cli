#!/bin/sh

# Application   : Ultra PPP
# Station       : MOJN (DNK)
# Surveying     : 24hr
# Constellation : BeiDou
# Technique     : CPP (B2,B3)

# Preprocessing
PIPELINE="BDS;C2I,C6I;>C05"

# CPP basic configuration
RTK_CONF=examples/CONFIG/gpst_cpp.json

# Analysis + ppp solutions
#   -f: force new report synthesis
#   -o: custom name
./target/release/rinex-cli \
    -f \
    -P $PIPELINE \
    -o "Ultra-BDS-CPP" \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/SP3/D/Sta21114.sp3.gz \
    ppp -c $RTK_CONF --static
