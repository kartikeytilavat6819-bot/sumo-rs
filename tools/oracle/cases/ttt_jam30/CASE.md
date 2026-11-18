# ttt_jam30

Jam teleporting with a 30 s threshold (the existing `teleport` / `jt_jam_teleport` cases use 20 s).
`b0` stops for 1000 s at `E1_0 endPos=390` on a 1-lane road; a queue of six followers builds
behind it and the front of the queue is teleported after waiting longer than 30 s.

Net: `E0 n0->n1 -> E1 n1->n2`, 400 m each, 1 lane (copied from the `collision` case).

Sim: `--end 120 --time-to-teleport 30`.
Compared: tripinfo, fcd and stat (the `teleports` section's `total`/`jam` counters).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ttt_jam30; rm NEW_CASE`.
