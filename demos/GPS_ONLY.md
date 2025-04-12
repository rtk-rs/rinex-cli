GPS (Only) demonstrations
=========================

Topics:

1. demonstrates support for GPS vehicles
2. demonstrate support of GPST, GST and UTC prefered timescale settings

In these examples, we using a basic GPST RINEX file, which is the most common
format. In this setup, the most dummy PVT solution solver would work well
in GPST. 

GPST
====

GPST is the default prefered Timescale.
Let's deploy the solver and request for GPST solutions

```bash
rinex-cli \
    -P GPS,C1C,C2W \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    ppp -c examples/CONFIG/Static/gpst_cpp.json
```

`Static/gpst_cpp.json` emphasizes that we're surveing the laboratory geodetic marker
that provided this GPST RINEX, and that we want to express the solution in GPST.
Which is, as previously stated, the easiest case and would work (in that very setup)
without "much more".

`gnss_rtk` and this framework is now very advanced in its time management and handling,
and offers a high level of flexibility. If we look that `NAV/V3/MOJN00DNK_R_20201770000`
header, we see that this file describes the behavior of few timescales (for that day):

1. |GST - GPST|
2. |GST - UTC|
3. |GPST - UTC|

GST solutions
=============

Now we take advantage of (1) and request PVT solutions expressed in UTC timescale.

```bash
rinex-cli \
    -P GPS;C1C,C2W \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    ppp -c examples/CONFIG/Static/gst_cpp.json
```

That's it! all you need to do, is describe your prefered timescale in the configuration script. 
Obviously, you are limited by what your input data allows to do.

UTC Solutions
=============

Now we take advantage of (2) and (3) and request PVT solutions expressed in UTC timescale.

```bash
rinex-cli \
    -P GPS;C1C,C2W \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    ppp -c examples/CONFIG/Static/utc_cpp.json
```

:warning: RINEX V3 /V4
======================

RINEXv3 is not very precise because it only allows to describe an offset or a perturbation for a 24h timeframe. This means that (3) applies for that entire day, which is far from perfect. You should upgrade to RINEXv4 for better results and improved precision. You can check some of our [RINEX v4 demos](../)
