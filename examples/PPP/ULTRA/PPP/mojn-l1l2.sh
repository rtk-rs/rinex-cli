#!/bin/sh

# Application   : Ultra PPP
# Station       : MOJN (DNK) 
# Surveying     : 24hr
# Constellation : GPS
# Technique     : PPP (L1+L2)

# Preprocessing
PIPELINE="GPS;C1C,L1C,C2W,L2W"
TIMEFRAME=">=2020-06-25T01:00:00 GPST;<2020-06-25T12:00:00 GPST"

RTK_CONF=examples/CONFIG/gpst_ppp.json

# Analysis + ppp solutions
#   -f: force new report synthesis
#   -o: custom name
./target/release/rinex-cli \
    -f \
    -q \
    -P $PIPELINE \
    -P "$TIMEFRAME" \
    -o "Ultra-GPS-PPP" \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/SP3/C/GRG0MGXFIN_20201770000_01D_15M_ORB.SP3.gz \
    --fp data/CLK/V3/GRG0MGXFIN_20201770000_01D_30S_CLK.CLK.gz \
    ppp -c $RTK_CONF --static
