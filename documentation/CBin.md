File Operation: cbin
====================

| Topics         | - Illustrate the `cbin` mode                                         |
|----------------|----------------------------------------------------------------------|
| Modes          | `cbin`                                                               |
| Difficulty     | <span style="color:gold"> &#9733;</span>&#9734;&#9734;&#9734;&#9734; |
| Constellations | Any                                                                  |
| Input          | RINEX                                                                |
| Output         | RINEX batch                                                          |

`cbin` allows splitting a complete setup into individual Constellations and timescales.  
It allows performing the mirror operation of merging individual constellations into a single modern multi-GNSS RINEX file.

Split this modern Multi-GNSS navigation file into all its individual components:

```bash
rinex-cli \
    --fp data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    cbin
```

## Advanced use

Any file operations option or preprocessing option may apply. In this example, we only generate
a GAL+GPS+BDS batch because all other components were discarded:

```bash
rinex-cli \
    -P "GPS,GAL,BDS" \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    cbin
```

Any file operations option may apply.

Force `gzip` compression of the output products with `--gzip`

```bash
rinex-cli \
    -P "GPS,GAL,BDS" \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    cbin --gzip
```

Force readable RINEX using seamless decompression:

```bash
rinex-cli \
    --crx2rnx \
    -P "GPS,GAL,BDS" \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    cbin --gzip
```
