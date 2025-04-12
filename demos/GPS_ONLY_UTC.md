GPS (Only) and UTC timescale
============================

_Objective_: demonstrate support of both GPST and UTC timescales.

In these examples, we using a basic GPST RINEX file, which is the most common
format. In this setup, the most dummy PVT solution solver would work well
in GPST. 

Let's deploy the solver and request for GPST solutions

```bash
rinex-cli \
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

For the first two, you will have to switch to one of your [GST (Galileo System Time) demo](../gst). We will now use (3) to express the PVT solutions in UTC timescale:

```bash
rinex-cli \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    ppp -c examples/CONFIG/Static/utc_cpp.json
```

That's it! all you need to do, is describe your prefered timescale in the configuration script. Obviously, in similar use cases, you are limited by your input data and what it allows to do.

:warning: accurate timescale transposition is only feasible if the `Modeling:sv_clock_bias` is compensated for (obviously):

```json
{
    timescale: "UTC",
    [..]
    "modeling": Modeling {
        [..]
        "sv_clock_bias": true,
        [..]
    },
    [..]
}
```

:warning: RINEX V3 /V4
======================

RINEXv3 is not very precise because it only allows to describe an offset or a perturbation for a 24h timeframe. This means that (3) applies for that entire day, which is far from perfect. You should upgrade to RINEXv4 for better results and improved precision. You can check some of our [RINEX v4 demos](../)

