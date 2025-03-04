CPP
===

The CPP navigation technique uses two pseudo range codes to cancel ionospheric bias.  
Once two signals (two carrier frequencies) have been sampled for a single SV, any ionosphere modeling
and other cancellation option is automatically disabled: we always prefer phyisical cancellation.

- ESBJERG (Denmark) static station surveying
  - [Using L1+L2](./esbjerg-l1l2.sh)
  - [Using L2+L5](./esbjerg-l2l5.sh)
  - [Using High Precision L2+L5 codes](./esbjerg-l2w.sh)

- MOJN (Denmark) static station surveying
  - [Using E1+E5](./mojn-e1e5.sh)
  - [Mixed E1+E5 (Galileo) B1+B2I (BeiDou) example](./mojn-gal-bds.sh)
