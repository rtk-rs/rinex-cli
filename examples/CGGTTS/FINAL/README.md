Final CGGTTS (+3 weeks)
=======================

As oppoposed to the [BRDC (Broadcast) examples](../BRDC) which
is identical to real time navigation, here we truly place ourselves
in a post processing scenario, where a lot of time has passed since signal observation.

As always, it is vital to read [the `ppp` equivalent](../../PPP/FINAL)
prior running the `cggtts` option.

Considering that CGGTTS is a purely static application targetting ultra high precision,
this usecase is the typical CGGTTS workflow.

This toolbox adapts to the input products, we switch to "this mode" by simply
providing one of those files. The command lines are 99% identical to the BRDC case.
From the user perspective, this scenario is a little more complex because it requires
one more input. Yet it is easier to deploy than RTK which requires much more input.

## Examples

We divide our examples according to the navigation technique being used:

- [Final solutions using Single pseudo range code](./SPP)
- [Final solutions using Dual pseudo range code](./CPP)
