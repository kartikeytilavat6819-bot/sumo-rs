# ps_multistop

Vehicles with three consecutive stops spanning different edge types: a lane stop on `E0`, a parkingArea
stop on `E1` and a lane stop on `E2`. One vehicle mixes `until` with a parkingArea stop. Control vehicle
has no stops.

Net: `E0` → `E1` → `E2`, 300 m each, 1 lane. Sim: `--end 260`.

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ps_multistop; rm NEW_CASE`.
