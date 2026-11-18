# ps_duration_until_triggered

One run contrasting the three ways to end a stop: `duration` (relative), `until` (absolute time) and a
`triggered="person"` stop that waits for person `pT`. Plus an unstopped control vehicle.

Net: `E0` → `E1` → `E2`, 300 m each, 1 lane. Sim: `--end 200`.

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ps_duration_until_triggered; rm NEW_CASE`.
