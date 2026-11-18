# pt_modes_walk_public

Pedestrians with plain `<walk>` stages alongside `personTrip modes="public"` riders on a
scheduled line L1. A second variant sets `--persontrip.walk-opposite-factor 3.0` to scale the
routing cost of walking against the direction of traffic on access walks to the stops.

Plain-XML net (linear E0-E1-E2, sidewalks/crossings/walkingareas, `net.args`).

Sim: `--end 300`, `sim_opp.args` adds `--persontrip.walk-opposite-factor 3.0`;
both with `--personinfo-output` (`extra_outputs.txt`).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh pt_modes_walk_public; rm NEW_CASE` (pristine C++ 1.28.0).
