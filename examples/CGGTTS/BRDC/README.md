BRDC + CGGTTS examples
======================

Like in simple [Post Processed Navigation](../../PPP), the solutions
rely on radio broadcast navigation messages and do not use high precision / post processed
products. Similarly:

* it is physically the same as solving in real-time
* you cannot expect results as good as the true PPP scenario

Read the [Broadcast Navigation page](../../PPP/BRDC) to understand the requirements
of this mode of navigation. As a rule of thumb, we always recommend getting familiar with
simple `ppp` first, prior adding the `cggtts` option.

## CGGTTS

CGGTTS solutions are text file that indicate the state of the local clock and the temporal
state of a constellation. They are solved with a specific algorithm and a fit operation
over the PPP solutions.

You can request CGGTTS solutions solviging by adding the `--cggtts` in the `ppp` opmode.

## Examples

We divide our examples according to the navigation technique being used:

- [Single pseudo range code based navigation](./SPP)
- [Dual pseudo range code based navigation](./CPP)
- [Dual pseudo range code and phase navigation](./PPP)
