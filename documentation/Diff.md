FIle (A) - (B) special operation
================================

Amongst many file operations, `rinex-cli` allows differentiating two files of the same kind.  

The differentiation is applied is a very precise manner, where only identical physics (also referred to as _Observables_ in RINEX 
terminology) are differentiated to one another.

This operation may apply to many exotic applications.

Example (1): `diff` can only apply if both input products format do match, the following is an incorrect operation.

```bash
rinex-cli \
    --fp data/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
    diff data/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```

Example (2): Generate a CRINEX that equals `CRINEX{ RINEX(A-B) }`

```bash
rinex-cli \
    --fp data/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    diff data/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```

Like any file operations, use `--unzip` to silentely decompress, and `--crx2rnx` to silentely decompress to RINEX.  
This this third example, we now obtain readable RINEX:

```bash
rinex-cli \
    --crx2rnx \
    --fp data/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    diff data/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```
