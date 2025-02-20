#! /bin/bash

# Downloads some of the georust/rinex test_data
# that is used for thorough library testing and validation
# which spans all formats and revisions
LOCAL_DIR=data
GEORUST_URL=https://github.com/georust/rinex/tree/main/test_resources

FILES=(
    "CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz"
    "CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz"
    "NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz"
    "NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz"
)
    
mkdir -p $LOCAL_DIR

echo "Downloading test/example data..."

for file in "${FILES[@]}"; do
    file_name=$(basename $file)
    wget -q \
        --show-progress $GEORUST_URL/$file \
        -o $LOCAL_DIR/$file_name
done

echo "Test data downloaded!"
ls -lah $LOCAL_DIR/*
