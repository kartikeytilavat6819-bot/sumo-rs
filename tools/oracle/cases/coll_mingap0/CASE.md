# coll_mingap0

Forced insertion collision with `--collision.mingap-factor 0`, so a collision is only reported
when vehicles physically overlap (gap based on minGap is not subtracted).

Net: `E0 n0->n1 -> E1 n1->n2`, 400 m each, 1 lane (copied from the `collision` case).
`ram` is inserted at `departPos=97` at t=5 overlapping `lead` (stopped with front at 100,
back at 95). With the default factor 1 the reported gap is `-4.50`; with factor 0 it is
`-2.00`. `--insertion-checks none` allows the overlapping insertion.

Sim: `--end 150 --insertion-checks none --collision.action warn --collision.mingap-factor 0`.
Compared: tripinfo, fcd, stat and `collision` output (the `gap` attribute is the key signal).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh coll_mingap0; rm NEW_CASE`.
