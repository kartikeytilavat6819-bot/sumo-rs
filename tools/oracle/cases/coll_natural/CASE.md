# coll_natural

A *natural* rear-end collision (no overlapping insertion). `lead` brakes to a stop at
`E0_0 endPos=120`; `follow` has weak brakes (`decel=0.5`), short `tau=0.5`, `sigma=1` and a
2 s `actionStepLength`, so it cannot stop in time and hits the leader. `--seed 42` makes
the stochastic behaviour deterministic.

Net: `E0 n0->n1 -> E1 n1->n2`, 400 m each, 1 lane (copied from the `collision` case).

Sim: `--end 120 --collision.action warn --seed 42`.
Compared: tripinfo, fcd, stat (`safety collisions`) and collision output.
Refs: `touch NEW_CASE; bash tools/oracle/make_sim_refs.sh coll_natural; rm NEW_CASE`.
