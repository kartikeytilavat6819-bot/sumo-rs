# ignore_route_errors

A disconnected route (`E1 E0`, E1 ends at n2 while E0 starts at n0) is loaded thanks to
`--ignore-route-errors true`. Unlike `ttt_disconnected`, no disconnected teleport is
configured, so with the default 300 s jam threshold the vehicles simply reach the end of
E1 and remain unfinished within the 100 s horizon — this isolates the effect of the flag
itself.

Net: `E0 n0->n1 -> E1 n1->n2`, 400 m each, 1 lane (copied from `collision`).

Sim: `--end 100 --ignore-route-errors true`.
Compared: tripinfo, fcd and stat (`teleports total=0`, `vehicles running`).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ignore_route_errors; rm NEW_CASE`.
