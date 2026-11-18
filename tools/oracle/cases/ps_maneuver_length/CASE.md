# ps_maneuver_length

Off-road parkingArea with explicit `length="7"` and `angle="10"` attributes, exercised both without and
with `--parking.maneuver true` (the `maneuver` variant). Three vehicles park on `E1_0`, plus through traffic;
the maneuver option adds maneuvering time and lane blocking on arrival/departure.

Net: `E0`(1 lane) → `E1`(2 lanes) → `E2`(1 lane), 300 m each. Sim: `--end 220`.

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ps_maneuver_length; rm NEW_CASE`.
