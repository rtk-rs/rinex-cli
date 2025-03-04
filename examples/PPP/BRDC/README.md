BRDC Navigation examples
========================

BRDC (Broadcast) Navigation is separate from the true PPP application,
in the sense it does not use precise input products and only relies on RINEX data.

Because of that, 

- It can resolve right away (at sampling time)
- It is physically the same as real-time navigation
- Yet, the RINEX snapshot
will allow to capture the signal conditions and resolve the "real-time" solution afterwards.
Usually, some sort of recording device is used to capture the RINEX on the field, and the 
solutions will be resolved when back to base.

The minimum required input for all these examples are the most simplistic navigation input:
one RINEX observation, one Navigation RINEX. Both are mandatory, you cannot operate with
just one or the other, both must be combined. Once we have said that, there are a few
requirements to respect with your Navigation RINEX:

- it must cover the observation time frame
- it must cover the observed constellations

- [esbjergdnk-gps.sh](./SPP/esbjerg-dnk.sh) is the static surveying 
of the ESBJERG (DNK) GNSS station, using a single GPS pseudo range code
