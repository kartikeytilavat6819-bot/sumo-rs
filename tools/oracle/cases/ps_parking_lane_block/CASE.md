# ps_parking_lane_block

Dedicated `parking="true"` lane stops with no parkingArea at all: two on the two lanes of `E1` (one with
`duration`, one with `until`) and one on `E0`. A through flow shows how parked vehicles behave on a
two-lane edge.

Net: `E0`(1 lane) → `E1`(2 lanes) → `E2`(1 lane), 300 m each. Sim: `--end 220`.

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ps_parking_lane_block; rm NEW_CASE`.
