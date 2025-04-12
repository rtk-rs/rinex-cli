GPS + Gal mixed scenario
========================

`gnss_rtk` allows mixed constellation (modern) setups and garantees correctness of the temporal solution. For this, you will need a navigation RINEX file that desribes the offset between all constellations involved.

Using our `V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz`, we have a definitions for that day of

1. |GST - GPST|
2. |GST - UTC|
3. |GPST - UTC|

that means we can navigate and resolve correct X, Y, Z for any constellations described, but that may only apply only in GPST, GST and UTC timescales for the temporal solution.

GPST solution
=============

In this example, we remain in GPST (default option) and we use a Gal+GPS navigation context:

- `L1+L2` pseudo range is selected for `GPS`
- `E1+E5` pseudo range is selected for `Gal`
- (1) is used to express all solutions in GPST. If (1) would not exist,
even though `GPST` is the default prefered timescale, the `Galileo` selection would corrupt the temporal solution

```bash
rinex-cli \
    -P GPS,Gal \
    -P C1C,C2W,C5Q \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    ppp -c examples/CONFIG/Static/gpst_cpp.json
```

In the logs, you will see lines like these that address the `Gal` vehicles only:

```bash
2020-06-25T23:59:00 GPST(E01) - |GST - GPST| 2.70249422843626 ns correction
```

that means that the vehicles that belong to `GST` are correctly switched to `GPST` while GPST are naturally preserved.

UTC solution
============

Now we take advantage of (2) + (3) and switch to `UTC` temporal solutions.

```bash
rinex-cli \
    -P GPS,Gal \
    -P C1C,C2W,C5Q \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    ppp -c examples/CONFIG/Static/utc_cpp.json
```

All you have to do is select a valid prefered timescale, using the configuration setup. 
Once again, your input data limitates what is feasible. In this example, it is not possible to obtain valid solutions for `BDT` timescale for example.

In the logs, you can now see lines like these:

```
[2025-04-12T15:35:02Z DEBUG gnss_rtk::pool::prefit] 2020-06-25T00:00:00 GPST - E09: |GPST-UTC| 0 ns correction
[2025-04-12T15:35:02Z DEBUG gnss_rtk::pool::prefit] 2020-06-25T00:00:00 GPST - G13: |GPST-UTC| 0 ns correction
```

which mean that all contributors are switch to `UTC` timescale internally, in order to obtain valid `UTC` solutions.

:warning: RINEX V3 /V4
======================

RINEXv3 is not very precise because it only allows to describe an offset or a perturbation for a 24h timeframe. This means that (3) applies for that entire day, which is far from perfect. You should upgrade to RINEXv4 for better results and improved precision. You can check some of our [RINEX v4 demos](../)
