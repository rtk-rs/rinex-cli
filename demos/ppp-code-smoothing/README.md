Code Smoothing + Static PPP
===========================

The [GNSS-RTK library](https://github.com/rtk-rs/gnss-rtk) supports pseudo range
code smoothing, currently limited when L1 + C1 and L2 + C2 are sampled.
We can use this to increase the accuracy of the PVT solutions.

PPP static surveying, without code smoothing:

```bash
rinex-cli \
    -P Gal \
    --fp data/CRNX/V3/AJAC00FRA_R_20242090000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/GRAS00FRA_R_20242090000_01D_EN.rnx.gz \
    ppp -c demos/ppp-code-smoothing/ppp-no-smoothing.json
```

Now let's compare the results when using PPP + code smoothing:

```bash
rinex-cli \
    -P Gal \
    --fp data/CRNX/V3/AJAC00FRA_R_20242090000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/GRAS00FRA_R_20242090000_01D_EN.rnx.gz \
    ppp -c examples/CONFIG/Static/PPP/basic.json
```
