RINEX-Cli
=========

[![Rust](https://github.com/rtk-rs/rinex-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/rtk-rs/rinex-cli/actions/workflows/rust.yml)
[![Rust](https://github.com/rtk-rs/rinex-cli/actions/workflows/daily.yml/badge.svg)](https://github.com/rtk-rs/rinex-cli/actions/workflows/daily.yml)
[![crates.io](https://img.shields.io/crates/v/rinex-cli.svg)](https://crates.io/crates/rinex-cli)

[![License](https://img.shields.io/badge/license-MPL_2.0-orange?style=for-the-badge&logo=mozilla)](https://github.com/rtk-rs/rinex-cli/blob/main/LICENSE)

`rinex-cli` is a command line tool to post process RINEX + SP3 data.

## Licensing

This application is part of the [RTK-rs framework](https://github.com/rtk-rs) which
is delivered under the [Mozilla V2 Public](https://www.mozilla.org/en-US/MPL/2.0) license.

## Download the tool

You can download the latest version from [the release portal](https://github.com/rtk-rs/rinex-cli/releases)

## Install from Cargo

You can directly install the tool from Cargo with internet access:

```bash
cargo install rinex-cli
```

## Build from sources

Download the version you are interested in:

```bash
git clone https://github.com/rtk-rs/rinex-cli
```

And build it using cargo:

```bash
cargo build --all-features -r
```

## File formats & revisions

`rinex-cli` supports 

- all formats & revisions supported by [the RINEX parser](https://github.com/georust/tree/main/rinex)
- all revisions supported by [the SP3 parser](https://github.com/georust/tree/main/sp3)

Summary:

| Format                 | File name restrictions            |    Support                         |
|------------------------|-----------------------------------|------------------------------------|
| RINEX                  | :heavy_minus_sign:                | :heavy_check_mark:                 |
| CRINEX                 | :heavy_minus_sign:                | :heavy_check_mark:                 | 
| gzip compressed RINEX  | Name must end with `.gz`          | :heavy_check_mark:                 | 
| gzip compressed CRINEX | Name must end with `.gz`          | :heavy_check_mark:                 | 
| .Z compressed RINEX    | Not supported                     | Not supported                      |
| DORIS RINEX            | :heavy_minus_sign:                | :construction: Work in progress    |
| gzip compressed DORIS  | Name must end with `.gz`          | 
| .Z compressed DORIS    | Not supported                     | Not supported                      |
| SP3                    | :heavy_minus_sign:                | :heavy_check_mark:                 | 
| gzip compressed SP3    | Name must end with `.gz`          | :heavy_check_mark:                 | 
| .Z compressed SP3      | Not supported                     | Not supported                      |
| BINEX                  | :heavy_minus_sign:                | :heavy_minus_sign:                 |
| UBX                    | :heavy_minus_sign:                | :heavy_minus_sign:                 |

:heavy_minus_sign: No restrictions: file names do not have to follow naming conventions.  

## Documentation

Once you have installed the tool, read

- [The file loading documentation](./documentation/Input.md) that will teach you
how to load data into the toolbox
- [The Preprocessor documentation](./documentation/Preprocessor.md) that will teach you
how design a filter
- [The File operations documentation](./documentation/FileOperations.md) that will teach you
how to deploy the file operations
- [The SPP navigation documentation](./documentation/SPP.md) that will teach you
how to deploy the navigation solver in basic setup
- [The CGGTTS documentation](./documentation/CGGTTS.md) that will teach you
how to deploy the CGGTTS solver in basic setup

## Examples

We provide a set of [(command line) examples](./examples) written in Shell,
that cover most applications.

Special Thanks
==============

These tools would not exist without the great libraries written by C. Rabotin, 
[check out his work](https://github.com/nyx-space).  

Some features would not exist without the invaluable help of J. Lesouple, through
our countless discussions. Check out his 
[PhD manuscript (french)](http://perso.recherche.enac.fr/~julien.lesouple/fr/publication/thesis/THESIS.pdf?fbclid=IwAR3WlHm0eP7ygRzywbL07Ig-JawvsdCEdvz1umJJaRRXVO265J9cp931YyI)
