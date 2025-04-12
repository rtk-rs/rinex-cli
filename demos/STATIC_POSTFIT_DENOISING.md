Static PPP + Post fit denoising filter
======================================

All our static configuration presets activate the post-fit denoising filter
to enhance the accuracy of the X, Y, Z and derivative component of the PVT solutions.

This is requested by specifying a denoising factor in the `solver:postfit_denoising` field of the configuration preset:

```json
{
    [...]
    "solver": {
        "postfit_denoising": 1000,
        [..]
    }
    [..]
}
```

In this example, we use Galileo E1+E5 and CPP navigation technique, first without this option:

```bash
cat /tmp/gpst_no_postfit_no_smoothing_cpp.json

{
    "method": "CPP",
    "timescale": "GPST"
}

rinex-cli \
    -P Gal;C1C,C5Q
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    ppp -c /tmp/gpst_no_postfit_no_smoothing_cpp.json
```

Then we activate the denoising filter:

```bash
cat /tmp/gpst_postfit_no_smoothing_cpp.json

{
    "method": "CPP",
    "timescale": "GPST",
    "solver": {
        "postfit_denoising": 1000
    }
}

rinex-cli \
    -P Gal;C1C,C5Q
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    ppp -c /tmp/gpst_postfit_no_smooting_cpp.json
```

Finally, for ultimate performance, we activate the phase/code smoothing filter, with a 10 sample window length, combined to the denoising filter (with x1000 denoising factor):

```bash
cat /tmp/gpst_cpp.json

{
    "method": "CPP",
    "timescale": "GPST",
    "code_smoothing": 10,
    "solver": {
        "postfit_denoising": 1000
    }
}

rinex-cli \
    -P Gal;C1C,C5Q
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    ppp -c /tmp/gpst_cpp.json
```
