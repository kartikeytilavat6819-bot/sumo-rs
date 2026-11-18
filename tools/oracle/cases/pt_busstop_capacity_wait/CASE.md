# pt_busstop_capacity_wait

Bus stop `bs0` has `personCapacity="2"`; six pedestrians want to board line **L1** there
but only a limited number can use the stop at once, so some are blocked and wait for a
later bus. `--stop-output` records `blockedDuration` and loaded/unloaded persons.

Plain-XML net (linear E0-E1-E2, sidewalks/crossings/walkingareas, `net.args`).

Sim: `--end 300`, plus `--stop-output` and `--personinfo-output` (`extra_outputs.txt`).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh pt_busstop_capacity_wait; rm NEW_CASE` (pristine C++ 1.28.0).
