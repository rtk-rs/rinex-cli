Input data & File loading interface
===================================

This page explains how to load data correctly into the toolbox.  

The toolbox behavior is 100% correlated to the input provided data.  
Some operations are only feasible if specific datasets were loaded, therefore

- It is vital to understand how you can load your data correctly into the toolbox
- master your dataset, advanced applications will require to truly understand the dataset

This page is focused on the command line interfacing, our tutorial serie will focus
on physical applications instead.

NB: the toolbox cannot operate without at least one input product.

## Command line interface

1. The `--fp` flag allows loading one file at a time. You can load as many as you need.
You need to provide a complete file path.

2. The `-d` flag allows loading a directory recursively. The default search depth is set to 5.
You can increase the default depth with the `--depth` option. Similarly, it is possible to load
as many directories as you need.

Example (1)

```bash

```

## File naming conventions

The toolbox can operate on files that do not follow standard naming conventions.  
But that will most likely limit your capabilities if you are interested in generating output products
and want to have the toolbox figure correct names for you. See the [File Production](#file-production) paragraph
for more information.

## File compression

The toolbox supports Gzip compressed files and CRINEX (compact RINEX) files natively.  
Your `Gzip` files must be terminated with `.gz` for the application to understand the format.

`Z` compression is not supported in Rust, so our toolbox has no means to understand this format natively.  
You'll have to uncompress them manually first.

## File production

The applications will generate standard names by default. For all file operations,
you have an interface to customize the attributes. These attributes describe your data production context,
for example Country and production Agency.

## Context compliancy

You can use our [Summary Report](./Qc-Summary) to understand what your context may and may not allow.  
The geodetic report will let you know.

For example, stacking OBS + NAV RINEX is the minimum setup for post processed navigation.  
Stacking SP3 is expected when targetting PPP solutions.   
Multi frequency OBS RINEX is also required for precise navigation.

## Process more than 24h of data

Although RINEX is supposed to describe a 24h time frame, it is not mandatory.   
This toolbox can process more than 24h of data in two scenarios:
- either by forwarding unusually long RINEX files
- or by stacking two files for example, to describe a two day course. See our [48h tutorial](https://github.com/georust/rinex/tree/main/tutorials/48H).

## Precise Products

SP3 and special Clock RINEX are supported natively by the toolbox. 

:warning: _in theory_ precise products should not be mixed. It is recommnded to use precise products
that were published by the same agency. But that may only truly apply to applications targetting the best precision.

Loading single files
====================

Use `--fp` to load your dataset, one file at a time. `--fp` does not have a shortened version,
for the reason that it is already quite short, and `-f` [has a total different meaning](./Report).

```bash 
rinex-cli \
    --fp test_resources/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```

Form a PPP compliant context with 3 files:

```bash 
rinex-cli \
  --fp test_resources/CRNX/V3/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz \
  --fp test_resources/NAV/V3/ESBC00DNK_R_20201770000_01D_MN.rnx.gz \
  --fp test_resources/SP3/GRG0MGXFIN_20201770000_01D_15M_ORB.SP3.gz
```

Recursive loader
================

`-d` is most convenient when loading many files.

Input / Output and Behavior
===========================

`rinex-cli` follows a few simple yet fundamental points:

1. the application will not deploy without at least 1 input product, whatever its kind
2. it will always synthesize a report, except when running a File Operation: because they generate data instead
3. Opmodes are exclusive: you can only perform one operation per run. To differentiate opmodes from other command line options,
opmodes are the only options that do not require a `--hyphen`, for example: `ppp` or `diff` are one of those
4. Command line orders never matter but:
  - we recommend passing the `-P` Filter Designer first
  - options that apply to the opmode need to be passed _after_ said opmode. For example, this is a basic `ppp` run:

```bash
rinex-cli --fp /tmp/obs.txt --fp /tmp/nav.txt ppp
``` 

If you want to apply a custom PPP setup with `-c` you need to define it _after_ `ppp`

```bash
rinex-cli --fp /tmp/obs.txt --fp /tmp/nav.txt ppp -c /tmp/preset.json
``` 

Otherwise, this is either invalid or is not the `-c` option that you intend:

```bash
rinex-cli --fp /tmp/obs.txt --fp /tmp/nav.txt -c /tmp/preset.json ppp 
``` 

## What's next

- [Define your workspace](./Workspace)
- [Report synthesis and analysis](./Report)
- [Resolve PVT solutions](./Positioning)
- [Resolve CGGTTS solutions](./CGGTTS)
- [Perform file operations](./FOPS)
