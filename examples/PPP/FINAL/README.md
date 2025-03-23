Final PPP (+3 weeks)
====================

As oppoposed to the [BRDC (Broadcast) examples](../BRDC) which
is identical to real time navigation, here we truly place ourselves
in a post processing scenario, where a lot of time has passed since signal observation.

Public laboratories like IGS and JPL provide data to permit the post processing. 
The highest precision product is called `Final` and is published 3 weeks after
any point in time.

This toolbox adapts to the input products, we switch to "this mode" by simply
providing one of those files. The command lines are 99% identical to the BRDC case.
From the user perspective, this scenario is a little more complex because it requires
one more input. Yet it is easier to deploy than RTK which requires much more input.

Notes on Clock profiles
=======================

The toolbox accepts clock profiles (also refered to High Precision Clock products)
in the form of both SP3 or special RINEX. If you provide both, that will work.
SP3 allows describing everything in a single file, which makes it easier to deploy,
we have examples of that scenario as well.

## Examples

We divide our examples according to the navigation technique being used:

- [Final Single pseudo range code based navigation](./SPP)
- [Final Dual pseudo range code based navigation](./CPP)
