File Operation: split
=====================

| Topics         | - Illustrate the `split` mode                                 |
|----------------|---------------------------------------------------------------|
| CLI Modes      | `split`                                                       |
| Difficulty     | <span style='color:gold'> &#9733;&#9734;&#9734;&#9734;&#9734; |
| Constellations | Any                                                           |
| Input          | RINEX, SP3                                                    |
| Output         | RINEX, SP3, CSV                                               |

The `split` mode allows splitting an input product into two, at a specific point in time.

## Epoch description

Any valid `Epoch` description may apply:

- the most standard: `YYYY-MM-DDTHH:MM:SS TS` (example: `2020-01-01T00:00:00 UTC`)

## Example

Split signal observations at noon:

```bash
rinex-cli \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    split "2020-06-25T12:00:00 UTC"
```

## Output format

By default, the tool generates (preserves) the input product format. But any file operations
allows changing the format, for example by selecting `--csv`. In this example, we split a CRINEX file
into two CSV files:

```bash
rinex-cli \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    split "2020-06-25T12:00:00 UTC" --csv
```

## Advanced use

Any preprocessing pipeline may apply, so you can perform several tasks at once. 
For example, you can discard Constellations or SV your are not interested in, prior performing the temporal split.

```bash
rinex-cli \
    -P GPS;C1C,C5Q \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    split 2020-06-25T12:00:00 UTC
```

Any valid `-P` pipeline applies here.

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
