Post Processed Positioning
==========================

`rinex-cli` offers RINEX and SP3 exploitation
for high precision post processed navigation.

This mode is summoned with the `ppp` option, it accepts
a custom configuration script and deploys after a possible preprocessing pipeline.

Examples
========

We host many [Navigation examples](../PPP) spanning all use cases.
We recommend reading this page prior getting started.

RINEX-Cli behavior
==================

The tool's behavior is dictated by the input products.  
If you only provide RINEX products, you place yourself in the real-time/BRDC scenario. 
The tool will consume all observations remaining after the preprocessing pipeline. 
If you stacked many RINEX (for example, 2x24h=48h surveying), will we consume all of them in chronological order.
But you have to be aware of data gaps.
