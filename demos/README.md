Demos
=====

Serie of demonstrations using `rinex-cli` and our [GNSS framework](https://github.com/rtk-rs)

- [PPP surveying (from scratch) without apriori knowledge](./SURVEY.md)
- [PPP using 10' of smartphone recording (urban environment)](./ANDROID_RINEX.sh)
- [Roaming PPP (pedestrian profile)](./PPP_PEDESTRIAN.md)
- [Zero Repair (-z) to obtain valid PVT solutions](./ZERO_REPAIR_PPP.md)
- [Two 24H RINEX (=48h) surveying using static PPP technique](./STATIC_PPP_48H.md)
- [Postfit Denoising filter for improved PVT solutions](./STATIC_POSTFIT_DENOISING.md)
- [Code Smoothing combined to static PPP](./PPP_CODE_SMOOTHING.md)

File Operations
===============

- [Constellation binning and Timescale binning (`cbin`)](./CBIN.md)

Timescales & Constellations Demos
=================================

Demonstrate Timescale support, absolute time correctness and constellations support

- [GPS (only) to GPST/UTC and GST timescales (RINEXv3)](./GPS_ONLY.md)
- [GAL (only) to GPST/UTC and GST timescale (RINEXv3)](./GAL_ONLY.md)
- [GAL (only) to GPST/UTC and GST timescale (RINEXv3)](./BDS_ONLY.md)

All these scripts apply to `CGGTTS` solutions, by simply adding the `--cggtts` option.

Multi - GNSS examples
=====================

Demonstrate (modern) multi GNSS navigation scenarios. Combining
several different constellations to enhance the total precision while
preserving correct absolute time.

- [GPS+GAL to GPST/UTC/GST timescales (RINEXv3)](./GPSGAL_DUAL.md)

All these scripts apply to `CGGTTS` solutions, by simply adding the `--cggtts` option.

Triple Constellation examples
=============================

TODO

By RINEX types
==============

- [NAV to CSV extraction](./NAV_CSV.md)


RINEX V4 examples
=================

TODO
