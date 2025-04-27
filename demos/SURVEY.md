Post Processed Positioning without apriori knowledge
====================================================

| Topics         | - Illustrate one deployment aspect of `ppp` mode                      |
|----------------|-----------------------------------------------------------------------|
| Category       | `Post Processed Navigation`                                           |
| Modes          | `cbin`                                                                |
| Difficulty     | <span style="color:gold"> &#9733;&#9733;&#9733;</span>&#9734;&#9734;  |
| Constellations | Any                                                                   |
| Input          | RINEX                                                                 |
| Output         | PVT Solutions                                                         |

As already demonstrated, `ppp` allows post processed positioning with high accuracy,
using input RINEX, Clock RINEX and/or SP3 files. 

One particular fundamental aspect of navigation, is that it is always differential, ideally
very locally. In the navigation process, we need a reference position (origin) from which we estimate
a correction to the truth position. Most of our RINEX files were provided by laboratories and tend
to describe the position of a geodetic marker. In static positioning, that is the position
we survey. We usually consider this position was surveyed using professional techniques and equipments,
so we use it as a comparison point for our own performance.

In such case, this framework picks up that position and use it as initial preset. It is particularly well suited for
all scenarios

- in absolute (PPP) navigation, we are surveying this position and try to obtain better. So it's the best initial guess
we can think of
- in differential (RTK) navigation, assuming this is a base station, the rover is supposed to be in nearby area, so it's still
far better than simply initializing with a null value.

In this example, we will use one of those profesionnal RINEX files but we will remove the geodetic marker, forcing
the solver to guess the initial position.

Before getting started, let's do a basic `ppp` run that will serve as reference point:

```bash
rinex-cli \
    -P Gal \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    ppp -c examples/CONFIG/Static/gpst_cpp.json
```

When deploying, the framework gives all meaningful information. In particular that both RINEX files were
correctly loaded, that the observation source is reference, and that a geodetic marker was picked up:

```bash
[2025-04-26T19:47:13Z DEBUG rinex_cli] Primary: "MOJN00DNK_R_20201770000_01D_30S_MO"
    Observation: ["data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz"]
    Broadcast Navigation (BRDC): ["data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz"]
[2025-04-26T19:47:13Z INFO  rinex_cli] reference point identified: 3.62843E3km, 5.62059E2km, 5.19787E3km (lat=54.94432°, long=8.80538°)
```

Surveyed position:   

<div align="center">
    <a href=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/survey-map.png>
        <img src=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/survey-map.png alt="Plot">
    </a>
</div>

We can see that a reference point is being reported. All projects are reported with respect of that position.

## PPP from scratch with initial guess

As of today, `rinex-cli` does not offer micro patching options so we will have to cheat manually.
Using the following sequence, we unzip the observations and remove the geodetic marker:

```bash 
gzip -d data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx.gz
sed -i '/APPROX POSITION XYZ/d' data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx
```

If you load the file we just patched, you can see that the summary report will not report a geodetic marker:

```bash
rinex-cli --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx
```

Now we can run a new `ppp` session. This framework is powerful enough to determine it needs to do an initial guess,
and we will see that the initial guess is good enough to obtain similar results:

```bash
rinex-cli \
    -P Gal \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    ppp -c examples/CONFIG/Static/gpst_cpp.json
```

The frameworks lets us know once again, now with a ""warning"" to emphasize that the 
initial phase is more complicated:

```bash
[2025-04-26T19:58:37Z DEBUG rinex_cli] Primary: "MOJN00DNK_R_20201770000_01D_30S_MO"
    Observation: ["data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx"]
    Broadcast Navigation (BRDC): ["data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz"]
[2025-04-26T19:58:37Z WARN  rinex_cli] no reference point identifed
```

<div align="center">
    <a href=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/absolute-map.png>
        <img src=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/absolute-map.png alt="Plot">
    </a>
</div>

We can see that no reference point is being reported, and we can only project the absolute PVT solutions.
The final results look fairly similar. 

Resolved Coordinates comparison
===============================

Comparing coordinates resolved in both runs:

<div align="center">
    <p>
        Latitude coordinates, resolved with a reference point:
    </p>
    <a href=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/survey-latitude.png>
        <img src=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/survey-latitude.png alt="Plot">
    </a>
</div>

<div align="center">
    <p>
        Latitude coordinates, resolved without a reference point:
    </p>
    <a href=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/absolute-latitude.png>
        <img src=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/absolute-latitude.png alt="Plot">
    </a>
</div>

<div align="center">
    <p>
        Longitude coordinates, resolved with a reference point:
    </p>
    <a href=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/survey-longitude.png>
        <img src=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/survey-longitude.png alt="Plot">
    </a>
</div>
<div align="center">
    <p>
        Longitude coordinates, resolved without a reference point:
    </p>
    <a href=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/absolute-longitude.png>
        <img src=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/absolute-longitude.png alt="Plot">
    </a>
</div>

Absolute altitude comparison
============================

Comparing absolute altitude (above mean sea level) resolved from both runs:

<div align="center">
    <p>
        Absolute altitude, with a reference point:
    </p>
    <a href=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/survey-altitude.png>
        <img src=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/survey-altitude.png alt="Plot">
    </a>
</div>
<div align="center">
    <p>
        Absolute altitude, without a reference point:
    </p>
    <a href=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/absolute-altitude.png>
        <img src=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/absolute-altitude.png alt="Plot">
    </a>
</div>

Absolute clock offset comparison
================================

Comparing absolute clock offset (to `GPST` in this example) from both runs:

<div align="center">
    <p>
        Offset to GPST with a reference point:
    </p>
    <a href=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/survey-clock.png>
        <img src=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/survey-clock.png alt="Plot">
    </a>
</div>
<div align="center">
    <p>
        Offset to GPST without a reference point:
    </p>
    <a href=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/absolute-clock.png>
        <img src=https://github.com/rtk-rs/rinex-cli/blob/develop/plots/survey-demo/absolute-clock.png alt="Plot">
    </a>
</div>

There is no reason that a geometric reference point interferes with the resolved absolute time.  
We can see that it is verified.

:warning: CGGTTS mode
=====================

As stated elsewhere, `cggtts` mode option requires the definition of a reference point. 
So it is normal that this command line does not work:

```bash
rinex-cli \
    -P Gal \
    --fp data/CRNX/V3/MOJN00DNK_R_20201770000_01D_30S_MO.crx \
    --fp data/NAV/V3/MOJN00DNK_R_20201770000_01D_MN.rnx.gz \
    ppp -c examples/CONFIG/Static/gpst_cpp.json --cggtts
```
