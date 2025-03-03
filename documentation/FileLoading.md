File loading interface
======================

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
rinex-cli -q --fp data/ESBC00DNK_R_20201770000_01D_30S_MO.crx.gz
```

Example (2)

```bash
rinex-cli -q -d data/
```

## File naming conventions

The toolbox accepts files that do not follow standard naming conventions.

Note that working with files that do not following standard naming conventions will limit our 
smart file production capabilities. See [File Operations](./FileOperations) for more information.

## Tutorials

The output product you can generate, [now depends on the operation you will select
and the input you just loaded](./InputOutput.md)
