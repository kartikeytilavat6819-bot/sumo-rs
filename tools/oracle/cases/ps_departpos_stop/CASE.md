# ps_departpos_stop

`departPos="stop"` inserts a vehicle at the end position of its first stop when that stop is on the
departure edge; without a stop on the first edge it falls back to `base`. One vehicle also uses a
parkingArea stop to show that only lane stops on the departure edge count.

Net: `E0` → `E1` → `E2`, 300 m each, 1 lane. Sim: `--end 200`.

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ps_departpos_stop; rm NEW_CASE`.
