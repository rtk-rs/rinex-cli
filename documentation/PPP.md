Post Processed Positioning
==========================

`rinex-cli` offers RINEX and SP3 exploitation for high precision post processed navigation.

This page describes the basic of the `ppp` mode, which combines
your RINEX files and deploys a position solver. It is dedicated to post-processing
navigation, because we only accept text files as input. This mode is kind of like
an equivalent or modern replacement of some of the options you might find in the RTKLib toolkit.

You can summon `ppp` after a preprocessing pipeline. This is how we can
focus on specific constellations or signals.

RINEX-Cli behavior
==================

The tool's behavior is dictated by the input products.  
If you only provide RINEX products, you place yourself in the ""real-time"" scenario,
that we detail in a [dedicated chapter of our examples](../examples/PPP/BRDC).

If you stack SP3 or special RINEX products, you will place yourself in a post-processing scenario,
which will give the best results you can obtain without access to a reference station.
This is typically the task required when setting up and defining new reference station.

Other key elements defined by your input products:

- the time frame: we will consume all signal observations. If you may reduce the time frame
with a filter, this will reduce the observations we may consume

- the navigation RINEX should cover your signal observations. Any signal observation that
is not covered by a navigation message cannot serve in the PPP process. It is identical
to discarding it with a filter

- We have no limitations on the number of input RINEX. If you take standard RINEX publications
and stack two of them, you will then survey for 48 hours.

- Observed signals: available signal observations must be compatible with the navigation technique being
used. The navigation technique is defined in the `GNSS-RTK` configuration script, which is optionnally
passed with `-c` after `ppp` mode selection (see our example scripts). If you select a technique
using a combination of signals yet only provide one signal, the algorithm will not be able to deploy.

Examples
========

We host many [Navigation examples](../PPP) spanning all use cases.
We recommend reading this page prior getting started.

What next ?
===========

- [CGGTTS solutions solving](./CGGTTS.md)
