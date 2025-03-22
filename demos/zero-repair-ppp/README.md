Zero repair (-z) and PPP opmode
===============================

One application of the `-z` zero repair flag is to correct invalid observations
produced incorrectly by low quality setups or receivers.

The `CRNX/V3/NYA100NOR_S_2024128` daily observations are incorrect and encoded
zeros from time to time in the `C5X` pseudo range code. This is illegal. 
Any `ppp` solving attempt will rapidly panic with physical errors.

One example would be `C5X SPP`:

```bash
rinex-cli \
	-P GPS \
	-P C5X \
	--fp data/CRNX/V3/NYA100NOR_S_20241280000_01D_30S_MO.crx.gz \
	--fp data/NAV/V3/NYA100NOR_S_20241280000_01D_GN.rnx.gz \
	ppp -c examples/CONFIG/SPP/basic.json
```

Or any dual signal technique involving `C5X`, for example `C1C+C5X CPP`:

```bash
rinex-cli \
	-P GPS \
	-P C1C,C5X \
	--fp data/CRNX/V3/NYA100NOR_S_20241280000_01D_30S_MO.crx.gz \
	--fp data/NAV/V3/NYA100NOR_S_20241280000_01D_GN.rnx.gz \
	ppp -c examples/CONFIG/CPP/basic.json
```

You can also see that `C1C SPP` gives correct results, because that signal is correctly encoded:

```bash
rinex-cli \
	-P GPS \
	-P C1C \
	--fp data/CRNX/V3/NYA100NOR_S_20241280000_01D_30S_MO.crx.gz \
	--fp data/NAV/V3/NYA100NOR_S_20241280000_01D_GN.rnx.gz \
	ppp -c examples/CONFIG/SPP/basic.json
```

To fix that, we can request the zero repair preprocessing operation, with `-z`, and obtain
PVT solutions. The `C5X` signal is quite often lost, so we also add a narrow timeframe
to illustrate we can now obtain valid solutions (other problem that is out of scope here): 

```bash
rinex-cli \
	-P GPS \
	-P C1C,C5X \
	-P ">2024-05-07T12:30:00 GPST" \
	-P "<2024-05-07T16:20:00 GPST" \
	--fp data/CRNX/V3/NYA100NOR_S_20241280000_01D_30S_MO.crx.gz \
	--fp data/NAV/V3/NYA100NOR_S_20241280000_01D_GN.rnx.gz \
	ppp -c examples/CONFIG/CPP/basic.json
```
