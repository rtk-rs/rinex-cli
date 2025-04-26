#! /bin/sh

RINEX_CLI="./target/release/rinex-cli -f"
CONSTELLATION=Galileo
CONFIG=/tmp/config.json

# Static sampling with poor clock quality and poor receiver quality.
rm -f $CONFIG
echo '
{
    "method": "CPP",
    "timescale": "GPST",
    "solver": {
        "postfit_denoising": 1000,
        "max_gdop": 5.0
    }
}' >> $CONFIG

$RINEX_CLI \
    -P $CONSTELLATION \
    --fp data/OBS/V3/GEOP092I.24o.gz \
    --fp data/NAV/V3/CORD00ARG_R_20240920000_01D_MN.rnx.gz \
    ppp -c $CONFIG
