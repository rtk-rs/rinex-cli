File Operation: cbin
====================

| Topics         | - Illustrate the `cbin` mode                                         |
|----------------|----------------------------------------------------------------------|
| Modes          | `cbin`                                                               |
| Difficulty     | <span style="color:gold"> &#9733;</span>&#9734;&#9734;&#9734;&#9734; |
| Constellations | Any                                                                  |
| Input          | RINEX                                                                |
| Output         | RINEX batch                                                          |

`cbin` allows splitting a Multi-GNSS setup into individual Constellations and Timescales.
It is the mirror operation of merging, which allows creating a complex Multi-GNSS setup by stacking individual files.

Example: split multi-GNSS observations into individual components:

```bash
rinex-cli \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    cbin

[2025-04-19T08:16:41Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GPS/ESBC00DNK_R_20201770000_01D_30S_MO.crx" has been generated
[2025-04-19T08:16:41Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GAL/ESBC00DNK_R_20201770017_01D_30S_MO.crx" has been generated
```

We can see that we synthesized output products for each constellations that were declared.  

If you open the Galileo observations for example, you can see that they remain expressed in `GPST`:

```bash
rinex-cli \
    --fp WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GAL/ESBC00DNK_R_20201770017_01D_30S_MO.crx
```

Performing temporal shift and timescale re-expression is a little more advanced and
described at the bottom of this page.

## Advanced use

Any file operations option or preprocessing option may apply. In this example, we only generate
a GAL+GPS+BDS batch because all other components were discarded prior `cbin` operation:

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

Temporal shift & Timescale reexpression
=======================================

By default, the original timescale is preserved, as demonstrated in the basic examples in this page.

If your dataset is now, Navigation compatible, you can decide to not only split into individual GNSS systems, but decide to re-express all measurements into the related timescale (_when feasible_).

For this we augment the previous example with a NAV RINEX and add the `--tsbin` option to `cbin`:

```bash
rinex-cli \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    cbin --tsbin
```
