# pt_lines_choice

Two bus lines **L1** (departs 0/60/120) and **L2** (departs 30/90/150) share bus stops
`bs0`/`bs1` (`lines="L1 L2"`). Pedestrians use `personTrip modes="public"` with an explicit
`lines="L1"`, `lines="L2"`, or no `lines` (any) attribute, arriving at different times so
they board (or wait for) different lines.

Plain-XML net (linear E0-E1-E2, sidewalks/crossings/walkingareas, `net.args`).

Sim: `--end 300`, plus `--personinfo-output` (`extra_outputs.txt`).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh pt_lines_choice; rm NEW_CASE` (pristine C++ 1.28.0).
