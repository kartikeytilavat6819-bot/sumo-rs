# ps_lots_occupancy

Off-road parkingArea with five explicit `<space>` elements (`roadsideCapacity="0"`) and eight vehicles
wanting it, so some must wait or fail as lots fill. Tests lot assignment and full-lot behaviour.

Net: `E0`(1 lane) → `E1`(2 lanes) → `E2`(1 lane), 300 m each. Sim: `--end 300`.

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ps_lots_occupancy; rm NEW_CASE`.
