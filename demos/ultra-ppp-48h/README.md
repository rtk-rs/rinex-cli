48h Static Surveying using Ultra PPP technique
==============================================

Since `rinex-cli` has "no theoretical limitations" on the number of input
products, one consequence is we are not limited to survey a single RINEX.
In this example, we load two standard 24h observation datasets and survey for 2
entire days.

We host one complete dataset for that purpose, `data/CRNX/V3/AJAC00FRA_R_2024` 
DOY 209 and 210. Since both are standardized 24h RINEX files, sampling
and observed data remain valid during the midnight crossing.

In this example, we use Galileo E1+E5 (for which we host a complete example), but any supported constellation may apply.

```bash
rinex-cli \
    -P Gal \
    --fp data/CRNX/V3/AJAC00FRA_R_20242090000_01D_30S_MO.crx.gz \
    --fp data/CRNX/V3/AJAC00FRA_R_20242100000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/GRAS00FRA_R_20242090000_01D_EN.rnx.gz \
    --fp data/NAV/V3/GRAS00FRA_R_20242100000_01D_EN.rnx.gz \
    ppp -c examples/CONFIG/PPP/basic.json
```

To be continued
