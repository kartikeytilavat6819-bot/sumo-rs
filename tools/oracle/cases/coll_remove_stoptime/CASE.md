# coll_remove_stoptime

Forced insertion collision with `--collision.action remove --collision.stoptime 5`: both
participants stop for 5 s after the t=5 collision and are then removed (unlike
`coll_stoptime`, where the collider is teleported). The collision output records the
detection, and `chase` is unaffected.

Net/demand: same 1-lane overlap as the `collision` / `coll_stoptime` cases.

Sim: `--end 150 --insertion-checks none --collision.action remove --collision.stoptime 5`.
Compared: tripinfo, fcd, stat (`teleports total=0` for removal) and collision output.
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh coll_remove_stoptime; rm NEW_CASE`.
