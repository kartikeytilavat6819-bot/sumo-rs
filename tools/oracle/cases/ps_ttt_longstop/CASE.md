# ps_ttt_longstop

`--time-to-teleport 20` while vehicles stop for a long time on a single-lane edge. A 150 s lane stop on
`E0` blocks a following flow (which teleports after the threshold); a `parking="true"` stop on `E1` is also
included to contrast parked (non-blocking) behaviour.

Net: `E0` → `E1` → `E2`, 300 m each, 1 lane. Sim: `--end 200 --time-to-teleport 20`.

Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ps_ttt_longstop; rm NEW_CASE`.
