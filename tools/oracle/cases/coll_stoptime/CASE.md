# coll_stoptime

Forced insertion collision with `--collision.action teleport` and a 5 s
`--collision.stoptime`: after the collision is detected at t=5 the collider stops for
5 s before being teleported (t=10). The `collision` output records the detection time.

Net: `E0 n0->n1 -> E1 n1->n2`, 400 m each, 1 lane (copied from the `collision` case).
Same demand as the `collision` case (`lead` stopped with front at 100, `ram` inserted at
departPos 97 at t=5 with `--insertion-checks none`, `chase` following).

Sim: `--end 150 --insertion-checks none --collision.action teleport --collision.stoptime 5`.
Compared: tripinfo, fcd, stat and collision output.
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh coll_stoptime; rm NEW_CASE`.
