File Operation: `diff`
======================

| Topics         | - Illustrate the `diff` mode                                         |
|----------------|----------------------------------------------------------------------|
| Category       | `File Operation`                                                     |
| Modes          | `diff`                                                               |
| Difficulty     | <span style="color:gold"> &#9733;</span>&#9734;&#9734;&#9734;&#9734; |
| Constellations | Any                                                                  |
| Input          | Observation RINEX                                                    |
| Output         | Observation RINEX                                                    |

The `diff` mode allows differentiating signal observations, provided in Observation RINEX files.  
Like `merge` and other similar operations, `diff` requires a first input product and a secondary (as reference).

The differentiation is applied in very precise manner, by only substracting identical signals, physics and modulations.  
Those are described by _Observables_ in RINEX terminology. In other words, we only substract identical Observables to each other.

For example, if `C1C` was measured on both sites, `C1C(a-b) = C1C(a) - C1C(b)` is formatted. 
Any observation that was made only on one site is left out.

The output product is still valid RINEX yet represents something that is illegal as per the RINEX standards, so you should
use it with care.

This operation was designed to output `RINEX` to `CSV` for precise clock comparison, by means of dual Pseudo Range
or dual Phase Range observations, by two devices that share the same sampling clock.

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
