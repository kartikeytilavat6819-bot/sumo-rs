# pt_tram_stops

Trams (`vClass="tram"`) on a dedicated three-edge track with two tram stops `ts0`/`ts1`.
Three scheduled T1 trams use `until` stop times; pedestrians use `personTrip modes="public"`
to board them. `--stop-output` captures the tram stop outcomes.

Plain-XML net (linear E0-E1-E2, sidewalks/crossings/walkingareas, `net.args`).

Sim: `--end 300`, plus `--stop-output` and `--personinfo-output` (`extra_outputs.txt`).

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh pt_tram_stops; rm NEW_CASE` (pristine C++ 1.28.0).
