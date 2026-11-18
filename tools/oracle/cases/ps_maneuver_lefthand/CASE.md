# ps_maneuver_lefthand

Parking on a left-hand-traffic net with `--parking.maneuver true` (base variant `nomaneuver` disables it).
Two vehicles park off-road on `A_0`/`B_0` with `parking="true"`, two more park without it; a through flow
shows the maneuver-related lane blocking.

Net: left-hand `A` → `B` → `C`, 300 m each, 1 lane (netconvert `--lefthand`). Sim: `--end 220`.

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ps_maneuver_lefthand; rm NEW_CASE`.
