Zero repair (-z) to obtain valid PVT solutions
==============================================

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
	ppp -c examples/CONFIG/Static/gpst_spp.json

[...]
panic: Physical non sense - rx=2024-05-07T12:30:30 GPST prior tx=2024-05-07T12:30:30.000123693 GPST
```

Or any dual signal technique involving `C5X`, for example `C1C+C5X CPP`:

```bash
rinex-cli \
	-P GPS \
	-P C1C,C5X \
	--fp data/CRNX/V3/NYA100NOR_S_20241280000_01D_30S_MO.crx.gz \
	--fp data/NAV/V3/NYA100NOR_S_20241280000_01D_GN.rnx.gz \
	ppp -c examples/CONFIG/Static/gpst_cpp.json

[...]
panic: Physical non sense - rx=2024-05-07T12:30:30 GPST prior tx=2024-05-07T12:30:30.000171848 GPST
```

You can also see that `C1C SPP` gives correct results, because that signal was correctly encoded:

```bash
rinex-cli \
	-P GPS \
	-P C1C \
	--fp data/CRNX/V3/NYA100NOR_S_20241280000_01D_30S_MO.crx.gz \
	--fp data/NAV/V3/NYA100NOR_S_20241280000_01D_GN.rnx.gz \
	ppp -c examples/CONFIG/Static/gpst_spp.json
```

To fix that, simply request the zero repair operation with `-z`. 
We now obtain valid PVT solutions. The `C5X` signal is quite often lost, so we also add a narrow timeframe
to illustrate we can now obtain valid solutions (other problem that is out of scope here): 

```bash
rinex-cli \
    -z \
	-P GPS \
	-P C1C,C5X \
	-P ">2024-05-07T12:30:00 GPST" \
	-P "<2024-05-07T13:15:00 GPST" \
	--fp data/CRNX/V3/NYA100NOR_S_20241280000_01D_30S_MO.crx.gz \
	--fp data/NAV/V3/NYA100NOR_S_20241280000_01D_GN.rnx.gz \
	ppp -c examples/CONFIG/Static/gpst_cpp.json
```
