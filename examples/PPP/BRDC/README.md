BRDC Navigation examples
========================

BRDC (Broadcast) Navigation is separate from the true PPP application,
in the sense it does not use precise input products and only relies on RINEX data.

It is physically the same as radio based real-time navigation, except we're working here
with data buffered through RINEX files. Usually, some sort of recording device is used to capture
the RINEX on the field, and the solutions are solved when back to base.

BRDC Navigation is therefore the easiest navigation option we offer: it only requires two
RINEX files, the observations and the ephemeris. Both are mandatory, you cannot perform
post processed navigation without either one of them. There are two basic requirements
on the Ephemeris you provide:

- the Navigation file must must cover the observation time frame.
Any observation that is not covered by an Ephemeris message will not contribute to the solutions.

- it must cover the observed constellations. Any SV for without Ephemeris frame at a specific
point in time may not contribute to the solutions.

So really, the Navigation RINEX determines what solutions you will obtain.  
This application is flexible in how you load your data, you can for example provide
all your observations in a single RINEX, and provide Ephemeris on a constellation basis. 

## Examples

We divide our examples according to the navigation technique being used:

- [Single pseudo range code based navigation](./SPP)
- [Dual pseudo range code based navigation](./CPP)
- [Dual pseudo + phase range based navigation](./PPP)
