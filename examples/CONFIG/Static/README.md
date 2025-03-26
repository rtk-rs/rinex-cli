Configuration presets
=====================

## Timescale

The selected timescale will have PVT solutions expressed with respect to that timescale.

## SPP presets

In SPP, we navigate using a single signal in sight. 
With this framework, it [does not have to L1, as demonstrated in this example]().

SPP is dedicated to reduced navigation capabilities and will exhibit poorer results
than others. Yet, it is possible to obtain very good results (~ 1m) in static applications,
and if you have high quality sampling.

Because SPP is very limited, it does not make sense to compensate many perturbations. 
If you compare these presets to others, they will appear much more simple. It is one
of the reasons why we recommend starting with `SPP` navigation technique/strategy, before
diving into CPP and later on, PPP.

As learning experience, you can activate advanced compensations while still using the
SPP technique, to see that it has no impact, because your accuracy is mostly limited by the
ionospheric bias.

## CPP preset

Dual pseudo range navigation technique. Gives much higher accuracy as comparsed to SPP.

## PPP preset

Dual pseudo range + phase range navigation technique. Gives the highest accuracy.
