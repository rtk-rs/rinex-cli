Ultra PPP (+3 weeks)
====================

As oppoposed to the [BRDC (Broadcast) examples](../BRDC) which
is identical to real time navigation, here we truly place ourselves
in a post processing scenario, where a lot of time has passed since signal observation.

Unlike our [PPP (Final) examples](../FINAL), here we will introduce a special
Clock RINEX, which is just like the SP3 published up to 3 weeks after sampling,
but with higher resolution on the clock states.

This toolbox once again adapts to the provided input products. Clock RINEX
is always prefered over SP3 for temporal states.

Notes on time frame
===================

Both BRDC and FINAL requirements still apply. 
The Clock RINEX should cover the observed period.

Examples
========

We divide our examples according to the navigation technique being used:

- [Ultra - Dual pseudo range code based navigation](./CPP)
- [Ultra - Dual pseudo range code + phase navigation](./PPP)
