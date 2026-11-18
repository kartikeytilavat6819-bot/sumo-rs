# ps_stop_output

`--stop-output` on parking-related stops: a parking-area stop with and without `parking="true"`, a
`parking="true"` lane stop, and an `until` lane stop. Tests that stop output records parking stops, not
only ordinary lane stops. Compared via `ref.stop.xml` (extra_outputs.txt).

Net: `E0` → `E1` → `E2`, 300 m each, 1 lane. Sim: `--end 220`.

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ps_stop_output; rm NEW_CASE`.
