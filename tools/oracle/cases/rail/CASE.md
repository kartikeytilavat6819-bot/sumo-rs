# rail

Hand-written railway net: single-track section r1..r2 with a passing loop (j1..j2, main + loop),
`rail_signal` nodes r1 and r2, dead ends w and e. Every edge is `allow="rail"`, `spreadType="center"`
(required for bidi rail pairing; without it the driveways never conflict and no train is held).

Two trains in opposite directions: `trainEast` departs w at t=0, `trainWest` departs e at t=3.

C++ reference (sumo 1.28.0, WSL):
- `trainEast` departs at 0 and clears the section at 42 s.
- `trainWest` is held at its departure for 39 s (`departDelay 39`) and departs at 42 s.
- Held trains are not inserted, so they never appear in the FCD: the FCD has no speed-0 row.
  A train that is in the network is never held by these signals, so the hold is only visible as
  departure delay in tripinfo.

Files: `net.args` and `sim.args` use repo-root relative paths. `net_ref.sh` and `sim_ref.sh` regenerate
the references in WSL.
