File Operation: split
=====================

The `split` opmode allows splitting an input product into two, a specific point in time.

The Epoch description must be valid, the expected format is `YYYY-MM-DDTHH:MM:SS TS`,
for example `2020-01-01T00:00:00 UTC` is a valid description.

This example will divide this file in two at noon:

```bash
rinex-cli \
    --fp data/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    split 2020-06-25T12:00:00 UTC
```
