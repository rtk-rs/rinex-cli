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
    ppp -c examples/CONFIG/gpst_cpp.json --static
```

`--static` because in this particular case, we are surveying the laboratory reference position.
We offer many configurations preset, this one says we want to express the solutions in `GPST`.

`gnss_rtk` and this framework is now very advanced in its time management and handling,
and offers a high level of flexibility. If we look that `NAV/V3/MOJN00DNK_R_20201770000`
header, we see that this file describes the behavior of few timescales (for that day):

1. |GST - GPST|
2. |GST - UTC|
3. |GPST - UTC|

GST solutions
=============

When `GST` solutions are requested, the framework will take advantage of (1)

```bash
rinex-cli \
    -P GPS;C1C,C2W \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    ppp -c examples/CONFIG/gst_cpp.json --static
```

All you have to do is select the desired timescale.

UTC Solutions
=============

Same applies to UTC in this example (taking advantage of (3):

```bash
rinex-cli \
    -P GPS;C1C,C2W \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    ppp -c examples/CONFIG/utc_cpp.json --static
```

:warning: RINEX V3 /V4
======================

RINEXv3 is not very precise because it only allows to describe an offset or a perturbation for a 24h timeframe. This means that (3) applies for that entire day, which is far from perfect. You should upgrade to RINEXv4 for better results and improved precision. You can check some of our [RINEX v4 demos](../)
