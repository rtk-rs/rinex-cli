Input / Output
==============

This table summarizes the output product you can generate,
based on the selected opmode and the input product.

`opmode` is the operation you requested. This toolbox
supports one operation per run (to simplify internal logic),
it will then adapt to the provided context.

Quality Control (QC) is the default mode, and synthesizes a report.


| Input                | Opmode              | Second argument   | Application                            | Output product(s)  |
|----------------------------------------------------------------|----------------------------------------|--------------------|
| RINEX                | None                | None              | RINEX QC                               | report.html        |
| SP3                  | None                | None              | SP3 QC                                 | report.html        |
| RINEX + SP3          | None                | None              | Post processed, High precision geodesy | report.html        |

`merge` I/O: 

| Input                | Opmode              | Second argument                        | Application        | Output product(s)  |
|-------------------------------------------------------------------------------------|--------------------|--------------------|
| RINEX                | merge               | RINEX File                             | File management    | Merged RINEX       |
| RINEX                | merge               | RINEX File `+rnx2crx`                  | File management    | Merged CRINEX      |
| CRINEX               | merge               | RINEX File                             | File management    | Merged CRINEX      |
| CRINEX               | merge               | RINEX File `+crx2rnx`                  | File management    | Merged RINEX       |

`filegen` I/O:

| Input                | Opmode              | Second argument                        | Application        | Output product(s)   |
|-------------------------------------------------------------------------------------|--------------------|---------------------|
| RINEX                | filegen             | None                                   | File management    | RINEX               |
| RINEX                | filegen             | `+rnx2crx`                             | File management    | CRINEX              |
| CRINEX               | filegen             | `+crx2rnx`                             | File management    | RINEX               |
| RINEX /CRINEX        | filegen             | `+csv`                                 | File management    | Observations to CSV |
| NAV RINEX            | filegen             | `+csv`                                 | File management    | Ephemeris to CSV    |

`tbin` I/O:

| Input                | Opmode              | Second argument                        | Application        | Output product(s)   |
|-------------------------------------------------------------------------------------|--------------------|---------------------|
| RINEX                | tbin                | Duration                               | File management    | RINEX Batch         |
| RINEX                | tbin                | Duration  `+rnx2crx`                   | File management    | CRINEX Batch        |
| CRINEX               | tbin                | Duration  `+crx2rnx`                   | File management    | RINEX Batch         |

`diff` I/O:

| Input                | Opmode              | Second argument                        | Application        | Output product(s)   |
|-------------------------------------------------------------------------------------|--------------------|---------------------|
| RINEX                | diff                | Reference RINEX File                   | File management    | Special RINEX(A-B)  |
| CRINEX               | diff                | Reference RINEX File                   | File management    | Special CRINEX(A-B) |
| CRINEX               | diff                | Reference RINEX File `+crx2rnx`        | File management    | Special RINEX(A-B)  |
| RINEX                | diff                | Reference RINEX File `+rnx2crx`        | File management    | Special CRINEX(A-B) |

All `File Mangement` application [accept many options](./FileProduction.md) that will let you customize your production context.


## Tutorials

Continue with reading the documentation of the [opmode(s) you are interested in](../README.md),
follow our [examples](../examples/README.md) that span many applications.
