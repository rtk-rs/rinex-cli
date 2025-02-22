File Operation: merge
=====================

File merging consists in creating one output product from two input of the same kind.

This operation is requested with `merge` which requires a secondary input file, this one is to serve as the reference point in the Merge(A, B) operation.

Example (1): attempting to merge Navigation RINEX into Observation is an invalid operation

```bash
rinex-cli \
    --fp data/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    merge data/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```

Example (2): merging two CRINEX files together.

```bash
rinex-cli \
    --fp data/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    merge data/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```
