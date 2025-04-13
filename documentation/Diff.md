=====================

| Topics         | - Illustrate the `diff` mode                                         |
|----------------|----------------------------------------------------------------------|
| Modes          | `diff`                                                               |
| Difficulty     | <span style="color:gold"> &#9733;</span>&#9734;&#9734;&#9734;&#9734; |
| Constellations | Any                                                                  |
| Input          | Observation RINEX                                                    |
| Output         | Observation RINEX                                                    |

The `diff` mode allows differentiating signal observations, provided in Observation RINEX format, per frequency and signal modulations, and formatting this exotic output still in standard RINEX. Thefore, like `merge` and other operations, `diff` requires two input products (Observation RINEX here).

The differentiation is applied in very precise manner, where only identical physics (also referred to as _Observables_ in RINEX 
terminology) are differentiated to one another.  For example, if `C1C` is measured on both sites, `C1C(a-b) = C1C(a) - C1C(b)` is
formatted. Any observation that is not made on both site is not formatted.

The output product is still valid RINEX yet represents something that is illegal as per the RINEX standards.

## Input product and command line

`diff` requires that both input format do match. For example, this operation is invalid:

```bash
rinex-cli \
    --fp data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    diff data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```

`diff` only applies to Observation RINEX, for example, this operation is invalid:

```bash
rinex-cli \
    --fp data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    diff data/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz
```

`diff` applies to Observation and compressed Observation RINEX, and any other File Operations option we support still applies.

```bash
rinex-cli \
    data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    diff data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```

## :warning: RINEX compression

RINEX compression stil has a few issues here & there. We recommed you
force `--crx2rnx` to remain in readable RINEX, especially when facing formatting issues.

## Advanced use

Any preprocessing pipeline applies. Let say you are only interested in `L1C(b-a)` and `L5Q(b-a), you can restrict to that:

```bash
rinex-cli \
    -P "GPS;L1C,L5Q" \
    data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    diff data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```
 
`--crx2rnx` for seamless decompression obviously still applies:

```bash
rinex-cli \
    --crx2rnx \
    -P L1C \
    data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    diff data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```

`--rnx2crx` for seamless compression obviously still applies.

Any file operation options applies, because `diff` is a file operation:

- `--gzip`: force gzip compression, which is useful when coming from readable data
- any production setup customization 

another example:

```bash
./target/release/rinex-cli \
    -P "GPS;L1C,L5Q" \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    diff data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz --gzip
```
