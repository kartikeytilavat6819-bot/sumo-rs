# pt_personflow_public

A `personFlow` (period 20 s, begin 10, end 90) generates repeated pedestrians whose plan is a
`personTrip modes="public"` on a scheduled line L1. Exercises personFlow expansion combined
with intermodal public-transport routing.

Plain-XML net (linear E0-E1-E2, sidewalks/crossings/walkingareas, `net.args`).

Sim: `--end 300`, plus `--personinfo-output` (`extra_outputs.txt`).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh pt_personflow_public; rm NEW_CASE` (pristine C++ 1.28.0).
