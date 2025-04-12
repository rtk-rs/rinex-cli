Demos
=====

Serie of demonstrations using `rinex-cli` and our [GNSS framework](https://github.com/rtk-rs)

- [Zero Repair (-z) to obtain valid PVT solutions](./ZERO_REPAIR_PPP.md)
- [Two 24H RINEX (=48h) surveying using static PPP technique](./STATIC_PPP_48H.md)
- [Pedestrian profile (roaming) PPP](./PPP_ROAMING_PEDESTRIAN.md)
- [Postfit Denoising filter for improved PVT solutions](./STATIC_POSTFIT_DENOISING.md)
- [Code Smoothing combined to static PPP](./PPP_CODE_SMOOTHING.md)

Timescales & Constellations Demos
=================================

Demonstrate Timescale support, absolute time correctness and constellations support

- [GPS (only) to UTC timescale](./GPS_ONLY_UTC.md)
- [GPS (only) to GST timescale (RINEXv3)](./GPS_ONLY_GST.md)
- [GAL (only) to GPS timescale](./GAL_ONLY_GPST.md)
- [GAL (only) to UTC timescale (RINEXv3)](./GAL_ONLY_UTC.md)
- [GAL (only) to GST timescale (RINEXv3)](./GPS_ONLY_GST.md)
- [BDS (only) to GPS timescale](./BDS_ONLY_GPST.md)
- [BDS (only) to UTC timescale (RINEXv3)](./BDS_ONLY_UTC.md)
- [BDS (only) to GST timescale (RINEXv3)](./BDS_ONLY_GST.md)

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

RINEX V4 examples
=================

TODO
