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
    "MET/V3/POTS00DEU_R_20232540000_01D_05M_MM.rnx.gz"
    "IONEX/V1/jplg0010.17i.gz"
    "CLK/V3/GRG0MGXFIN_20201770000_01D_30S_CLK.CLK.gz"
)
    
mkdir -p $LOCAL_DIR
cd $LOCAL_DIR

echo "Downloading test/example data..."

for file in "${FILES[@]}"; do
    file_name=$(basename $file)
    wget -q \
        --show-progress $GEORUST_URL/$file
done

echo "Test data downloaded!"

cd ..
ls -lah data/
