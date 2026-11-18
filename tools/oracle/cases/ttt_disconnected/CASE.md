# ttt_disconnected

Vehicles with a disconnected route (`E1 E0`: E1 ends at n2, E0 starts at n0, so E0 is not a
successor of E1) are loaded thanks to `--ignore-route-errors` and then teleported after
waiting 5 s (`--time-to-teleport.disconnected 5`). The C++ warning names the reason
`", disconnected"`.

Net: `E0 n0->n1 -> E1 n1->n2`, 400 m each, 1 lane (copied from the `collision` case).

Sim: `--end 100 --ignore-route-errors true --time-to-teleport.disconnected 5`.
Compared: tripinfo, fcd and stat (`teleports total`, which counts disconnected teleports).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ttt_disconnected; rm NEW_CASE`.
