# pt_flow_until_schedule

Bus line **L1** as a `<flow>` with `period="80"`; stops use absolute `until` plus `duration`
(a schedule). Three pedestrians use `personTrip modes="public"` from E0 to E2.

Plain-XML net (linear E0-E1-E2, sidewalks/crossings/walkingareas guessed, `net.args`),
bus stops `bs0`/`bs1` on the driving lanes in `pt_flow_until_schedule.add.xml`.

Sim: `--end 300`, plus `--stop-output` and `--personinfo-output` (`extra_outputs.txt`).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh pt_flow_until_schedule; rm NEW_CASE` (pristine C++ 1.28.0).
