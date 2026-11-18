# ttt_remove10

Gridlocked vehicles are *removed* instead of teleported (`--time-to-teleport.remove true`)
with a 10 s threshold. The existing `cv_teleport_remove` case pairs removal with
`--ignore-route-errors`; this one removes vehicles stuck in a plain jam.

Net/demand: same 1-lane jam as `ttt_jam30` (`b0` stopped at `E1_0 endPos=390`, six followers).

Sim: `--end 120 --time-to-teleport 10 --time-to-teleport.remove true`.
Compared: tripinfo, fcd and stat (`vehicles` / `teleports` counters — removal does not
produce a teleport in tripinfo, so the two cases diverge in `arrival` / stat).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ttt_remove10; rm NEW_CASE`.
