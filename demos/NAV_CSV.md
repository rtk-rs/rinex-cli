NAV to CSV extraction with `filegen`
====================================

| Topics         | - Illustrate the `filegen` mode                                        |
|----------------|-----------------------------------------------------------------------|
| Category       | `File Operation`                                                      |
| Mode           | `filegen`                                                             |
| Difficulty     | <span style="color:gold"> &#9733</span>&#9734;&#9734;&#9734;&#9734;   |
| Constellations | Any                                                                   |
| Input          | NAV RINEX                                                             |
| Output         | NAV RINEX                                                             |

`rinex-cli` offers means to extract data, from all supported products.    
We support CSV extraction, std i/o prints and file synthesis. All of this is happening
after potential pre-processing, which allows data customization and reformating.  

CSV is most useful to extract data easily and route it to other tools.   
In the future, this toolbox may support other formats like parquet as well.

In this demo we focus on NAV RINEX data, which contains a lot of information:

- Ephemeris data for navigation purposes
- Constellation timescale corrections
- Ionospheric corrections
- Earth orientation for accurate compensation (only on V4).

Getting started
===============

`filegen` basically means you want to synthesize data, from input products.

```bash
rinex-cli --filegen --help
```

It is a file operation, so it supports all the shared options between those, in particular:

- production context customizations
- gzip compression
- CSV and other reformatting, we will focus here
- CRINEX compression, when compatible, may applies to all operations supported by this toolbox.
It is not a `filegen` option.

When selecting `filegen` only (and possibly context customizations), the tool will synthesize the
same format:

```C
# Synthesize NAV RINEX
rinex-cli --fp data/NAV/V2/cbw10010.21n.gz --filegen

# Apply some input customization before synthesis
rinex-cli \
    -P ">G09" \ # discard G01..G09
    --fp data/NAV/V2/cbw10010.21n.gz \
    --filegen 

# Apply input & output customization to the RINEX synthesis
rinex-cli \
    -P ">G09" \ # discard G01..G09
    --fp data/NAV/V2/cbw10010.21n.gz \
    --filegen \
        --short \ # restrict to V2 name
        --agency "CWB" # rename CBW agency to CWB
```

CSV output
==========

We can then decompose the file to CSV adding `--csv`. The output stream is file type dependent

```C
# Apply some input customization before synthesis
rinex-cli \
    -P ">G09" \ # discard G01..G09
    --fp data/NAV/V2/cbw10010.21n.gz \
    --filegen --csv
```

NAV V3
======

When working with NAV/OBS/METEO V3 files and newer revisions, the file is named according
to the newer standards. It defines the sampling rate (for observations) and the country code.
V3 filename synthesis is our default option. It can obviously only work with V3 input. When working
with V2 files like before, you will have to specify `--short` to restrict to a shortened V2 filename
(that can always be synthesized correctly).

When that applies, you can then redefine other V3 specific attributes, like the country code:

```C
rinex-cli \
    -P ">G09" \ # discard G01..G09
    --fp data/NAV/V3/AMEL00NLD_R_20210010000_01D_MN.rnx \
    --filegen --country NDL # The Nederlands

2025-05-22T19:43:28Z INFO  rinex_cli::cli::workspace] session workspace is "WORKSPACE/AMEL00NLD_R_20210010000_01D_MN"
[2025-05-22T19:43:28Z DEBUG rinex_cli::fops] ProductionAttributes { name: "AMEL", year: 2020, doy: 1, v3_details: Some(DetailedProductionAttributes { country: "NDL", batch: 0, data_src: Receiver, ppu: Daily, ffu: None, hh: 23, mm: 59 }), region: None }
[2025-05-22T19:43:28Z INFO  rinex_cli::fops::filegen] Broadcast Navigation (BRDC) RINEX "WORKSPACE/AMEL00NLD_R_20210010000_01D_MN/BRDC/AMEL00NDL_R_20200012359_01D_MN.rnx" has been generated
```


NAV V4
======

When working with V4, you get a higher refresh rate on all those parameters.

TODO : add meaningful example.
