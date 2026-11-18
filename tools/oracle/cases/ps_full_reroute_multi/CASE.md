# ps_full_reroute_multi

Three capacity-1 parking areas (`pa0`, `pa1` on `E1`, `pa2` on `E0`) with a rerouter on `E0` offering two
`parkingAreaReroute` alternatives. Six vehicles compete for `pa0` so it fills up and later arrivals must be
rerouted to `pa1`/`pa2`. `--device.parking` is not a registered SUMO 1.28.0 option, so the alternative
mechanism (`parkingAreaReroute`) is exercised directly here.

Net: `E0`(1 lane) → `E1`(2 lanes), 400 m each. Sim: `--end 300`.

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ps_full_reroute_multi; rm NEW_CASE`.
