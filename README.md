RINEX-Cli
=========

[![Rust](https://github.com/rtk-rs/rinex-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/rtk-rs/rinex-cli/actions/workflows/rust.yml)
[![Rust](https://github.com/rtk-rs/rinex-cli/actions/workflows/daily.yml/badge.svg)](https://github.com/rtk-rs/rinex-cli/actions/workflows/daily.yml)
[![crates.io](https://img.shields.io/crates/v/rinex-cli.svg)](https://crates.io/crates/rinex-cli)

[![License](https://img.shields.io/badge/license-MPL_2.0-orange?style=for-the-badge&logo=mozilla)](https://github.com/rtk-rs/rinex-cli/blob/main/LICENSE)

`rinex-cli` is a command line tool to post process RINEX + SP3 data.

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

This will build and install the stripped binary to `${HOME}/.cargo/bin`, which
is usally defined in the ${PATH}. Because our examples span all applications, you should
activate `--all-features`:

```bash
cargo install --all-features --path .

rinex-cli --version

which rinex-cli
${HOME}/.cargo/bin/rinex-cli
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

:warning: All our examples and demos are expected to be execute at the base of this repo.

Once you have installed the tool, read the first few steps:

- [File loading interface](./documentation/FileLoading.md): learn how to load data into the toolbox
- [The Preprocessor documentation](./documentation/Preprocessor.md) will teach you
how design a filter and deploy up to complex processing pipelines
- [The Input / Output page](./documentation/InputOutput.md) summarizes the output you can
generate, based on your input products

Then, continue your learning journey with:

- [The Merge operation](./documentation/Merge.md): to merge RINEX files together,
which is particularly useful in Data production context & files management

- [The Split operation](./documentation/Split.md): demonstrates how to
split one RINEX file into two

- [Time Binning operation](./documentation/Tbin.md): allows creating a batch of RINEX
from a single one (basically, dividing into sub-parts)

- [The differentiation operation](./documentation/Diff.md): to create a special RINEX
by substracting one from the other

Dive into the world of precise navigation:

- [Introduction to the PPP (Post Processed Positioning) opmode](./documentation/PPP.md)

- [Focus on the PPP +cggtts special option](./documentation/CGGTTS.md)

- [Many examples](./examples/README.md) are provided, spanning many applications

Special Thanks
==============

These tools would not exist without the great libraries written by C. Rabotin, 
[check out his work](https://github.com/nyx-space).  

Some features would not exist without the invaluable help of J. Lesouple, through
our countless discussions. Check out his 
[PhD manuscript (french)](http://perso.recherche.enac.fr/~julien.lesouple/fr/publication/thesis/THESIS.pdf?fbclid=IwAR3WlHm0eP7ygRzywbL07Ig-JawvsdCEdvz1umJJaRRXVO265J9cp931YyI)

## Licensing

This application is part of the [RTK-rs framework](https://github.com/rtk-rs) which
is delivered under the [Mozilla V2 Public](https://www.mozilla.org/en-US/MPL/2.0) license.
