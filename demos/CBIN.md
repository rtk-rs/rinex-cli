File Operation: `cbin`
======================

| Topics         | - Illustrate the `cbin` mode                                          |
|----------------|-----------------------------------------------------------------------|
| Category       | `File Operation`                                                      |
| Modes          | `cbin`                                                                |
| Difficulty     | <span style="color:gold"> &#9733; &#9733;</span>&#9734;&#9734;&#9734; |
| Constellations | Any                                                                   |
| Input          | RINEX                                                                 |
| Output         | RINEX batch                                                           |

`cbin` allows splitting a Multi-GNSS context into individual Constellations and Timescales.
It is the mirror operation of `merge`, which can be used to create a Multi-GNSS context by stacking
single constellation files.

Example: split multi-GNSS observations into individual constellations:

```bash
rinex-cli \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    cbin

[2025-04-19T08:16:41Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GPS/ESBC00DNK_R_20201770000_01D_30S_MO.crx" has been generated
[2025-04-19T08:16:41Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GAL/ESBC00DNK_R_20201770017_01D_30S_MO.crx" has been generated
```

We can see that we synthesized output products for each constellations that were declared.  

This framework has precise knowledge of any SBAS vehicle if it is declared in the [core library](https://github.com/rtk-rs/gnss).
Therefore, you can see that some vehicles were recognized as `WASS` and `EGNOS` vehicles.

If you open the `GPS` observations, you can see that it remains expressed in `GPST`:

```bash
rinex-cli \
    --fp WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GPS/ESBC00DNK_R_20201770000_01D_30S_MO.crx
```

If you open the `Galileo` observations for example, you can see that they are also expressed in `GPST`:

```bash
rinex-cli \
    --fp WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GAL/ESBC00DNK_R_20201770000_01D_30S_MO.crx
```

That's because `cbin` will preserve the original Timescale by default. 

## Advanced use

Like any other operations, one can combine any preprocessing pipeline to `cbin` mode.
In this example, we retain GPS+GAL+BDS, so we will only obtain 3 output products:

```bash
rinex-cli \
    -P "GPS,GAL,BDS" \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    cbin
```

Any file operations option may apply. For example `--gzip` to request gzip compression
of the output products, or `--crx2rnx` to enforce readable RINEX:

```bash
rinex-cli \
    --crx2rnx \
    -P "GPS,GAL,BDS" \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    cbin \
    --gzip
```

Temporal shift & Timescale reexpression
=======================================

By default, the original timescale is preserved, as demonstrated in the basic examples in this page.

Now we move on to a *Navigation compatible* setup by stacking a NAV RINEX file along the OBS RINEX file.  

This one describes the behavior of the following `TimeScales`:
- GPST/UTC
- GST/UTC
- GST/GPST

This means we can express and translate to either one of them. If you would like to be able to handle `BDT` for example,
you would have to stack another NAV RINEX file.

Let's run the previous command line:

```bash
rinex-cli \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    cbin

[2025-04-19T08:22:23Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GPS/ESBC00DNK_R_20201770000_01D_30S_MO.crx" has been generated
[2025-04-19T08:22:23Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GAL/ESBC00DNK_R_20201770017_01D_30S_MO.crx" has been generated
[2025-04-19T08:22:23Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GPS/ESBC00DNK_R_20201770000_01D_MG.rnx" has been generated
[2025-04-19T08:22:23Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GAL/ESBC00DNK_R_20201770000_01D_ME.rnx" has been generated
```

Just like before, obtain similar `GPST` products. Now let's add the special `--tsbin` option:

```bash
rinex-cli \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    cbin --tsbin

[2025-04-19T08:22:23Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GPS/ESBC00DNK_R_20201770000_01D_30S_MO.crx" has been generated
[2025-04-19T08:22:23Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GAL/ESBC00DNK_R_20201770000_01D_30S_MO.crx" has been generated
[2025-04-19T08:22:23Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GPS/ESBC00DNK_R_20201770000_01D_MG.rnx" has been generated
[2025-04-19T08:22:23Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GAL/ESBC00DNK_R_20201770000_01D_ME.rnx" has been generated
```

We can see that the `GPS` product remains expressed in the native `GPST`:

```bash
rinex-cli \
    --fp WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GPS/ESBC00DNK_R_20201770017_01D_30S_MO.crx
```

But the `GAL` product is now expressed in `GST`:

```bash
rinex-cli \
    --fp WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GAL/ESBC00DNK_R_20201770017_01D_30S_MO.crx
```

:warning: remember that `SBAS` is expressed in `GPST` in RINEX by default. 

Prefered timescale synthesis
============================

Since `cbin` is a *File Operation*,  the `--timescale` option still applies (to similar NAV setups only). 

Instead of using `cbin --tsbin`, you can use `cbin --timescale` to express each product
in the desired `Timescale`, when that is feasible.

In the previous example, `GST` would apply:

```bash
rinex-cli \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    cbin --timescale GST

[2025-04-19T08:22:23Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GPS/ESBC00DNK_R_20201770000_01D_30S_MO.crx" has been generated
[2025-04-19T08:22:23Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GAL/ESBC00DNK_R_20201770017_01D_30S_MO.crx" has been generated
[2025-04-19T08:22:23Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GPS/ESBC00DNK_R_20201770000_01D_MG.rnx" has been generated
[2025-04-19T08:22:23Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GAL/ESBC00DNK_R_20201770000_01D_ME.rnx" has been generated
```

You can see that the `GAL` product remains in `GST` as previously obtained with `cbin --tsbin`:

```bash
rinex-cli \
    --fp WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GAL/ESBC00DNK_R_20201770000_01D_30S_MO.crx
```

But you can now see that the `GPS` product was transposed to `GST`:

```bash
rinex-cli \
    --fp WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GPS/ESBC00DNK_R_20201770000_01D_30S_MO.crx
```

## :warning: Prefered UTC Timescale

We support many timescales, `UTC` being one of them.  
Using these set of tools, you can forge `RINEX` files expressed in `UTC`, which is most likely unintended by the
Observation RINEX standards.

Yet, this particularly (since we sort of know what we're doing) has proved convenient in the testing of out framework. 

So in this example, we can transpose both the  `GPST` and `GST` observations to `UTC` timescale 

```bash
rinex-cli \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    cbin --timescale UTC

[2025-04-19T08:22:23Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GPS/ESBC00DNK_R_20201770000_01D_30S_MO.crx" has been generated
[2025-04-19T08:22:23Z INFO  rinex_cli::fops] "WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/GAL/ESBC00DNK_R_20201770017_01D_30S_MO.crx" has been generated
```

You can see that all output products were transposed to `UTC` timescale, and that you should use them with care.
