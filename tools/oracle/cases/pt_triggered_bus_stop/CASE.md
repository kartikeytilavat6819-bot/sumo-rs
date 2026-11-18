# pt_triggered_bus_stop

Buses wait at `bs0` with `<stop ... triggered="person" duration="10"/>`: they depart only
after a pedestrian boards, then dwell for `duration`. Pedestrians arrive in groups at
different times and ride to `bs1`. `--stop-output` records the triggered stop times.

Plain-XML net (linear E0-E1-E2, sidewalks/crossings/walkingareas, `net.args`).

Sim: `--end 300`, plus `--stop-output` and `--personinfo-output` (`extra_outputs.txt`).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh pt_triggered_bus_stop; rm NEW_CASE` (pristine C++ 1.28.0).
