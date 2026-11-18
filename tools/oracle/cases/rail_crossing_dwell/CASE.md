# rail_crossing_dwell

Rail crossing at `rc` with road arms north/south. Unlike `rail_crossing_road`, the
eastbound train `t1` (60 m long) stops with its front 30 m past the crossing
(`stop lane="rc_re_0" endPos="30" duration="30"`), so its body straddles the crossing
and holds it for the whole 30 s dwell. Four cars arrive from both road directions
during and after the dwell, plus a second train from the east at t=50.

Sim: `--end 200 --time-to-teleport -1`.
