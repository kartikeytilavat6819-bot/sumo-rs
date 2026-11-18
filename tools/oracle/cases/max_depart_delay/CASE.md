# max_depart_delay

`--max-depart-delay 10` skips vehicles that cannot be inserted within 10 s of their
departure. `blocker` stops for 500 s with its back at 95 m, occupying `E0_0` at x=95..100;
`wait1` and `wait2` both try to depart there (default insertion checks) and are skipped
after waiting 10 s.

Net: `E0 n0->n1 -> E1 n1->n2`, 400 m each, 1 lane (copied from `collision`).

Sim: `--end 100 --max-depart-delay 10`.
Compared: tripinfo (skipped vehicles have no arrival), fcd and stat (`vehicles loaded/inserted`).
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh max_depart_delay; rm NEW_CASE`.
