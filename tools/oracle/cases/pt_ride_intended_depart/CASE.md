# pt_ride_intended_depart

Pedestrians use explicit `<ride>` stages with the `intended` (specific vehicle id) and
`depart` (requested departure time) attributes. Four L1 buses depart at 0/40/80/120 and
serve `bsStart` (E0_1) and `bsEnd` (E2_1). Persons either pin a vehicle, pin only a
departure time, or let the router choose.

Plain-XML net (linear E0-E1-E2, sidewalks/crossings/walkingareas, `net.args`).

Sim: `--end 300`, plus `--personinfo-output` (`extra_outputs.txt`).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh pt_ride_intended_depart; rm NEW_CASE` (pristine C++ 1.28.0).
