Filegen
=======

The `filegen` opmode signifies we want to output text data, after a possible
preprocessing pipeline or patching operation.

For example `-z` (zero repair) would patch an invalid Observation file and
format the patched/corrected version.

It also offers easy options to rework and reformat an existing file, for example
reworking the satellite content: 

```bash
rinex-cli \ 
    -P GPS,Gal \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    filegen
```

this example would preserve GPS and Galileo (only).

Any valid preprocessing pipeline may apply.

## Output format

When no extra option is passed (like `--csv`) the toolbox will match the input format(s).  
For example, if one Observation RINEX was loaded, we will format an Observation RINEX.
If both Observation and Navigation RINEX were loaded, we will format both.

## Gzip compression

Request gzip compression by adding the `--gzip` flag to any `filegen` pipeline:

```bash
rinex-cli \ 
    -P GPS,Gal \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    filegen --gzip
```

## CSV output

Request `CSV` output (instead of native) by adding the special `--csv` option.  
This allows converting your RINEX and/or SP3 data to CSV.  
This is particularly useful to take advantage of our parser, and export to third party tools.

```bash
rinex-cli \ 
    -P GPS,Gal \
    --fp data/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
    filegen --csv
```

Once again, the content you will obtain depends on the input products

| Input Product           | CSV                                                              |
|-------------------------|------------------------------------------------------------------|
| Observation RINEX       | Extracted signals, per date, time, SV, constellation and physics |
| Navigation RINEX        | Extracted and interpreted Navigation messages                    |
| OBS+NAV RINEX           | Joint file with orbital attitude resolved @ sampling epoch       |
