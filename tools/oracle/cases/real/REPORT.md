# real / reroute (variant `ref_reroute` + `sim_reroute.args`)

## Design

- Additional file: `reroute.add.xml` (`<param>` form of the rerouting device; closure window `begin=2 end=11`).
- Rerouter on `397795464#1`, closing `397795464#3` (`closingReroute`).
- Vehicle: `v0`, departs 0.00 on `397795464#0 … #3`, reference arrival 10.00, route 110.91 m.
  It is on `#1` at t≈2–5 s, so the closure is applied mid-route, not after arrival
  (`routes.rou` vehicles finish in ~2 s on the short routes, so those were rejected).
- Flags: `--additional-files tools/oracle/cases/real/reroute.add.xml`; C++ reference made with the
  exact `run_all.sh` flags (`--tripinfo-output --fcd-output --statistic-output --no-step-log true` + args).

## Candidate scan (C++, 1.28.0)

Every downstream (trigger, closure) pair on the six routes was run through C++ with `begin=1`.
Only two gave `rerouteNo > 0` without "No route for vehicle":

| trigger | closure | result |
|---|---|---|
| `397795464#1` | `397795464#3` | `v0` rerouted, arrival 10.00 → 8.00 (**chosen**) |
| `682360554#1` | `1117867029#0` | `v2` rerouted, arrival 17.00 → 13.00 |

Rejected: `397795464#1 → #2` (no connection `#1`→`#3`; `v0` removed with "No route"), and most others
(no alternative connection at all).

## Caveat — what this case really exercises

The closed edge `397795464#3` is `v0`'s **destination** edge. SUMO logs
`Cannot keep destination edge '397795464#3' for vehicle 'v0' due to closed edges. Terminating route.`
and `v0` ends on `#1` at `arrivalPos 79.06` (route length 85.19 m), not on a detour. The case therefore proves
that a mid-route closure of the destination triggers a reroute and the same early termination on both sides.
It does **not** prove that a vehicle detours around a closed edge on the Times Square net: the net has no
alternative connection for the routes checked. That needs a network with a bypass (the 4-edge toy net does have one).

## Verdict

| comparator | result |
|---|---|
| tripinfo (`rerouteNo`, arrivals, devices) | MATCH |
| FCD (per-step lanes, so the route taken) | MATCH |
| statistics | MATCH, 0 differences in 0 groups |

C++ `rerouteNo`: **1** (`v0`); all other vehicles 0. Our `rerouteNo` is 1 as well.

`run_all.sh`: `real/ref_reroute sim PASS` (rest unchanged: `PASS=25 FAIL=1 SKIP=8`; the single FAIL is the known
`reroute/ref_device` one-step offset). `depart` variants: 15/15.
