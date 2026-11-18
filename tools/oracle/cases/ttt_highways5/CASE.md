# ttt_highways5

Highway teleport settings on a fast (25 m/s) 1-lane net: regular gridlock teleporting is
disabled (`--time-to-teleport -1`), the highway threshold is 5 s and the minimum speed that
counts as a highway is lowered to 10 m/s (`--time-to-teleport.highways.min-speed 10`,
default 69/3.6). The existing `ttt_highways` case uses the opposite combination
(`-1` + `highways 20` at the default min-speed).

Net: `E0 n0->n1 -> E1 n1->n2`, 300 m each, 1 lane, speed 25.00 (copied from `ttt_highways`).

Sim: `--end 120 --time-to-teleport -1 --time-to-teleport.highways 5 --time-to-teleport.highways.min-speed 10`.
Compared: tripinfo, fcd and stat.
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh ttt_highways5; rm NEW_CASE`.
