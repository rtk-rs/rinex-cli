Pedestrian roaming application
==============================

| Topics         | Post Processed Navigation                                             |
|----------------|-----------------------------------------------------------------------|
| Category       | `Navigation`                                                          |
| Modes          | `ppp`                                                                 |
| Difficulty     | <span style="color:gold"> &#9733;&#9733;</span>&#9734;&#9734;&#9734;  |
| Constellations | GPS, Galileo                                                          |
| Input          | RINEX                                                                 |
| Output         | PVT Solutions                                                         |

As already demonstrated, `ppp` allows post processed positioning with high accuracy of a static target,
using RINEX, Clock RINEX and/or SP3 input.

Whether the target is moving or not is application dependent. In this example, the receiver is carried
in a backpack and the user is walking while carrying the receiver. We will switch to the `Pedestrian` profile.  
We host a complete data set which is compatible with real-time navigation (also referred to as BRDC navigation),
and it is compatible with GPS and Galileo.

Defining the user profile
=========================

:warning: this framework uses `Static` as the default profile. To correctly describe this use case,
we need to use a custom preset:

```bash
echo '
{
    "method": "CPP",
    "timescale": "GPST",
    "user": {
        "profile": "pedestrian"
    }
}' >> /tmp/pedestrian.json
```

Now let's use it:

```bash
rinex-cli \
    -P GPS \
    --fp data/OBS/V3/2024_09_20_10_17_06.obs.gz \
    --fp data/NAV/V3/2024_09_20_10_17_06.nav \
    ppp -c /tmp/pedestrian.json
```

We can see that the dilution of precision is rather good for a moving target (good receiver quality). `solver:max_gdop=5` is a good choice in this example. You can reduce it to `3` and still obtain quite a lot of solutions, but gaps will be be frequent.

We can see that the clock drift is about 10ms/s, we can emphasize it by adjusting our user profile:

```bash
echo '
{
    "method": "CPP",
    "timescale": "GPST",
    "user": {
        "profile": "pedestrian",
        "clock_sigma": 10.0
    }
}' >> /tmp/pedestrian.json
```

GPS + Galileo mix
=================

We can check that the data set is compatible with Galileo as well:

```bash
rinex-cli \
    -P Galileo \
    --fp data/OBS/V3/2024_09_20_10_17_06.obs.gz \
    --fp data/NAV/V3/2024_09_20_10_17_06.nav \
    ppp -c /tmp/pedestrian.json
```

So it is possible to deploy the Galileo + GPS mixed scenario, to improve the overall accuracy:

```bash
rinex-cli \
    -P GPS,Gal \
    --fp data/OBS/V3/2024_09_20_10_17_06.obs.gz \
    --fp data/NAV/V3/2024_09_20_10_17_06.nav \
    ppp -c /tmp/pedestrian.json
```
