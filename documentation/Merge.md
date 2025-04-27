File Operation: merge
=====================

| Topics         | - `merge` mode documentation                                         |
|----------------|----------------------------------------------------------------------|
| Modes          | `merge`                                                              |
| Difficulty     | <span style="color:gold"> &#9733;</span>&#9734;&#9734;&#9734;&#9734; |
| Constellations | Any                                                                  |
| Input          | RINEX, SP3                                                           |
| Output         | RINEX, SP3                                                           |

File merging consists in creating one output product from two products of the same kind.  
This operation is requested with `merge` which requires a secondary input file.

## Input Products

You cannot merge files of different kinds together.  
For example, attempting to merge Navigation RINEX into Observation is an invalid operation:

```bash
rinex-cli \
    --fp data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    merge data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```

On the other hand, merging two NAV files together is a valid operation:

```bash
rinex-cli \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    merge data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```

Each file merged with this toolkit will describe the version of this frawork and
the date and time of the merge operation, in a standardized comment:

```bash
rs-rinex v0.17.1    FILE MERGE          20250413 102952 UTC COMMENT
```

## :warning: CRINEX compression

CRINEX compression may still have a few issues, especially in V2 format.
We recommend you use the `--crx2rnx` to make sure you ouput in RINEX format,
in case the CRINEX formatter is in failure.

## Output file name

The output file is created within the workspace, and is named after the original file that we used in the merging operation.

For example:

```bash
./target/release/rinex-cli \
    --crx2rnx \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    merge data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz

cat WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/ESBC00DNK_R_20201770000_01D_30S_MO.rnx
```

## Advanced use

Other preprocessing operations may apply along `merge` operation,
to perform complex tasks all at once.

In particular, any preprocessing pipeline may apply, for example to retain
the Constellation and signals you are interested in. The resulting merged file
will only contain those:

```bash
rinex-cli \
    --crx2rnx \
    -P "GPS;C1C,C5Q" \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    merge data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz
```

You can see that the resulting RINEX only describes GPS and C1C+C5Q signals.

The `--crx2rnx` would force readable RINEX output (when coming from compressed RINEX), 
while the `--rnx2crx` would force compressed RINEX output (when coming from readable RINEX).

The `--gzip` option will force Gzip compression, which is very useful when creating compressed RINEX in particular,
to reduce the size of the output product.

```bash
rinex-cli \
    -P GPS;C1C,C5Q \
    --crx2rnx \
    --gzip
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    merge data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz
```
