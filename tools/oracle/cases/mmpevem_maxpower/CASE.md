# Case `mmpevem_maxpower`: MMPEVEM/unknown battery EV with maximumPower

`MMPEVEM/unknown` vType with `maximumPower="55000"` (consumed by HelpersMMPEVEM::compute) and the standard
electric-vehicle drag parameters, equipped with a battery device. One vehicle charges at a station and one
just drives, so both the consumption model and the charging path are exercised under MMPEVEM. Existing
`ev_mmpevem` does not set `maximumPower`; `emis_mmpevem_params` sets it but has no battery device.
Outputs: battery and chargingstations.
