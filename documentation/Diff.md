File Operation: diff
====================

The `diff` opmode allows performing the A-B operation, on identical physics (usually "Observables")
sampled at the same time. It requires a secondary file, which serves as Reference (B) in the differential operation.

This is intended to be used on Observation RINEX.

Example (1): it is not possible to differentiate different file formats

```bash
rinex-cli \
    --fp data/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    diff data/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```

Example (2): differentiate two CRINEX together

```bash
rinex-cli \
    --fp data/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    diff data/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```
