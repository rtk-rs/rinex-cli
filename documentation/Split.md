File Operation: split
=====================

| Topics         | - Illustrate the `split` mode                                        |
|----------------|----------------------------------------------------------------------|
| Modes          | `split`                                                              |
| Difficulty     | <span style="color:gold"> &#9733;</span>&#9734;&#9734;&#9734;&#9734; |
| Constellations | Any                                                                  |
| Input          | RINEX, SP3                                                           |
| Output         | RINEX, SP3                                                           |

The `split` mode allows splitting an input product into two, at a specific point in time.

## Epoch description

Any valid `Epoch` description may apply:

- the most standard: `YYYY-MM-DDTHH:MM:SS TS` (example: `2020-01-01T00:00:00 UTC`)

## Example

Split signal observations at noon:

```bash
rinex-cli \
    --crx2rnx \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    split "2020-06-25T12:00:00 UTC"
```

## Output file name

The output file name follows standard naming conventions. Following previous example, we obtained:

```bash
"WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/ESBC00DNK_R_20201770000_01D_30S_MO.rnx" has been generated
"WORKSPACE/ESBC00DNK_R_20201770000_01D_30S_MO/ESBC01DNK_R_20201781200_01D_30S_MO.rnx" has been generated
```

## Advanced use

Any preprocessing pipeline may apply, so you can perform several tasks at once. 
For example, you can discard Constellations or SV your are not interested in, prior performing the temporal split.

In this example we split at noon C1C+C5Q GPS observations:

```bash
rinex-cli \
    -P GPS;C1C,C5Q \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    split "2020-06-25T12:00:00 UTC"
```

Other options exist, for example `--crx2rnx` to request seamless CRINEX decompression, while working with
CRINEX observations like in these examples. This would generate two readable RINEX files (decompressed) splitted at noon, 
for C1C+C5Q modulations of GPS vehicles only:

```bash
rinex-cli \
    -P GPS;C1C,C5Q \
    --crx2rnx \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    split 2020-06-25T12:00:00 UTC
```
