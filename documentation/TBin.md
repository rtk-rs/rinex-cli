File Operation: tbin
====================

| Topics         | - Illustrate the `tbin` mode                                         |
|----------------|----------------------------------------------------------------------|
| Modes          | `tbin`                                                               |
| Difficulty     | <span style="color:gold"> &#9733;</span>&#9734;&#9734;&#9734;&#9734; |
| Constellations | Any                                                                  |
| Input          | RINEX                                                                |
| Output         | RINEX batch                                                          |

Time binning consists in creating a batch from one input product. The input product is splitted into
sub parts of equal durations. This operation is requested with `tbin` which requires the description of a duration.

Example (1): this modern RINEX spans 24h, we will split it into 4 files of 6 hour duration.

## Duration description

Any valid `Duration` description may apply. Example:

```bash
rinex-cli \
    --fp data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    tbin "6 h"
```

## :warning: RINEX compression

RINEX compression (to CRINEX) may still have a few issues from here & there. 
If you're facing formatting panics, we recommend you force `--crx2rnx` to make sure you format a readable RINEX.

## Advanced use

Any file operations option or preprocessing option may apply. So you can perform a complex time binning operation.

In this example, we split this 24h 30S observation RINEX into 4 but only keeping L1+GPS observations:

```bash
rinex-cli \
    -P "GPS;L1C,C1C" \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    tbin "6 h"
```

Force `gzip` compression of the output products with `--gzip`

```bash
rinex-cli \
    -P "GPS;L1C,C1C" \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    tbin "6 h" --gzip
```

Force readable RINEX using seamless decompression:

```bash
rinex-cli \
    --crx2rnx \
    -P "GPS;L1C,C1C" \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    tbin "6 h"
```
