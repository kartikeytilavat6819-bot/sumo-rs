# ttt_disconnected_remove

Disconnected-route vehicles (`E1 E0`, loaded via `--ignore-route-errors`) are **removed**
instead of teleported after 5 s (`--time-to-teleport.disconnected 5` +
`--time-to-teleport.remove true`). The C++ warning still names the `disconnected` reason
and the stat `teleports` counter still increments, but the vehicles vanish instead of
appearing later on E0 — the counterpart of `ttt_disconnected` (which teleports).

Net: `E0 n0->n1 -> E1 n1->n2`, 400 m each, 1 lane (copied from `collision`).

Sim: `--end 100 --ignore-route-errors true --time-to-teleport.disconnected 5 --time-to-teleport.remove true`.
Compared: tripinfo, fcd and stat.
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ttt_disconnected_remove; rm NEW_CASE`.
