# pt_missed_bus_wait

Pedestrians arrive at `bs0` shortly *after* a bus has left and must wait for the next one.
Three L1 buses stop at `bs0` at absolute times 20/80/140; four persons use `<ride>` stages
with a requested `depart` time placed just after each departure, so waiting time accumulates.

Plain-XML net (linear E0-E1-E2, sidewalks/crossings/walkingareas, `net.args`).

Sim: `--end 300`, plus `--personinfo-output` and `--stop-output` (`extra_outputs.txt`).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh pt_missed_bus_wait; rm NEW_CASE` (pristine C++ 1.28.0).
