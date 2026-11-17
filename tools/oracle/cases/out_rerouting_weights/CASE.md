# out_rerouting_weights

Rerouting device with period + adaptation (steps 6, interval 5): `--device.rerouting.output` (edge weights per adaptation) and vehroute output with exit-times, route-length and cost.

Sim: `--end 300 --device.rerouting.probability 1 --device.rerouting.period 15 --device.rerouting.adaptation-steps 6 --device.rerouting.adaptation-interval 5 --vehroute-output.exit-times true --vehroute-output.route-length true --vehroute-output.cost true`.

Compared by run_all: tripinfo, fcd, stat and `ref.<suffix>.xml` for rerouting, vehroute (compare_xml.py, exact).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh out_rerouting_weights; rm NEW_CASE` (pristine C++ 1.28.0, WSL).

Gate 2026-11-17 (cv01, fc4c07bf): FAIL — first weights diff: edge traveltime ref 0.85 vs ours 0.91; vehicle fC.3 arrival 53 vs 66 (fcd x at t=51); vehroute writes an extra departLane attribute.
