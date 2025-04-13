Time binning
============

Time binning consists in creating a batch from one input product. The input product is splitted into
sub parts of equal durations. This operation is requested with `tbin` which requires the description of a duration.

Example (1): this modern RINEX spans 24h, we will split it into 4 files of 6 hour duration.

```bash
rinex-cli \
    --fp data/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    tbin "6 h"
```

Example (2): like any file operation, use `--unzip` to silentely decompress the Gzip compression,
which may apply to `Example (1)`. Use `--crx2rnx` to silentely decompress CRINEX to readable RINEX:

```bash
rinex-cli \
    --crx2rnx \
    --fp data/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    tbin "6 h"
```
