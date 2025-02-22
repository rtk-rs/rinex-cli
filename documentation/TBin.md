File Operation: tbin
====================

Time binning consists in splitting one input product into a serie of products of the same kind,
of equal duration.

This operation is requested with `tbin` which requires the description of a duration.

Example: this modern RINEX spans 24h, we will split it into 6 output products of 4 hours each

```bash
rinex-cli \
    --fp data/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    tbin "24 h"
```
