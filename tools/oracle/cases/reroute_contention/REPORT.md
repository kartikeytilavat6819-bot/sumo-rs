# reroute_contention — many vehicles reroute at once and compete for the bypass

Gap it fills: every other reroute case has one rerouting vehicle. Here 12 vehicles reroute off `AB` in
the same window and share the bypass `AC` + `CB`, queueing and merging back at `nB` before `OUT`.

## Net

Same as `reroute_bypass` (`bypass.nod/edg/con.xml`, `ref_simple.net.xml`, built from plain XML via `net.args`).
`n0 → nA` is `IN`; `nA → nB` is `AB` (200 m) or `nA → nC → nB` is `AC` + `CB` (141 + 141 m); then `OUT`.
All edges have one lane at 13.89 m/s.

## Case

- `routes.rou`: a `<flow id="f">` of 12 vehicles, `period=1.50`, `begin=0`, `end=17`, route `r0` = `LEAD IN AB OUT`.
  SUMO rounds departures to whole seconds, so the C++ departures are 0, 2, 3, 5, 6, 8, 9, 11, 14, 16, 18, 20.
- Routing device by `<param>` on `DEFAULT_VEHTYPE`: probability 1, period 1000, pre-period 0. Period is not 1
  (see `reroute_bypass_p1`).
- `closing.add.xml`: closing rerouter on `IN` closing `AB`, interval **8 → 200**. At t=8, `f.0` is on `IN`
  and `f.1`…`f.4` are on `LEAD` or just behind it. Later vehicles enter `IN` inside the interval.
- `net.args`: repo-relative `--node-files/--edge-files/--connection-files`.
- `sim.args`: `--end 200 --additional-files tools/oracle/cases/reroute_contention/closing.add.xml`.
- Variant `ref_adapt` (`sim_adapt.args`): same, plus `--device.rerouting.adaptation-steps 3
  --device.rerouting.adaptation-interval 2`. Both are command-line device options (`MSDevice_Routing.cpp:63-69`).

## Proof of contention (C++ 1.28.0, `ref.*`)

| check | result |
|---|---|
| vehicles with `rerouteNo>=1` | **12 of 12** (all `rerouteNo=1`) |
| route after closure | `LEAD IN AC CB OUT`, `routeLength=777.08` |
| first step with two vehicles on AC/CB | t=28: `f.0` and `f.1` both on `AC` |
| steps with ≥2 distinct vehicles on AC/CB | 56 |
| max vehicles on AC/CB at one step | 11 (t=50) |
| queueing evidence | `f.9` slowed to 1.37 m/s on `AC` (t=62); `f.1` slowed to 2.49 m/s on `CB` (t=41) |
| arrivals | 61, 71, 73, 75, 78, 88, 89, 93, 95, 101, 102, 104 |

The criterion (≥3 rerouted, ≥2 on AC/CB at the same step) holds on the first timing, so the timings were not adjusted.

Adaptation variant (`ref_adapt`): arrivals and routes are identical to `ref`, 12/12 `rerouteNo=1`. This is
expected: with `AB` closed, the choice at `nA` is forced to `AC`/`CB`, so edge weights can change the route cost
but not the path. The variant exercises the adaptation code path, not a route decision.

## Verdict (ours)

- **`reroute_contention` net: PASS.** `compare_net.py` matches the C++ netconvert output.
- **`reroute_contention/ref`: PASS.** tripinfo, FCD and statistics all match C++ exactly.
- **`reroute_contention/ref_adapt`: PASS.** tripinfo, FCD and statistics all match C++ exactly.

So the contention scenario reproduces byte-for-byte on the comparators. No divergence to report on the sim.

## Finding: inline `<route>` inside `<flow>` panics (not in the case)

The first version of this case used `<flow ...><route edges="LEAD IN AB OUT"/></flow>`. C++ accepts it. Our
`sumo` panics before the first step:

```
Error: The route '' for flow 'f' is not known.
```

The same flow with a named route (`<route id="r0" .../>` plus `<flow route="r0" .../>`) loads and runs (12
vehicles). So the defect is that our flow loader drops an inline route child and keeps an empty route id.
The inline form on `<vehicle>` works (`reroute_bypass`). This case uses the named form so that it can test
contention. The inline-flow defect is not a row on the scoreboard, and no case covers it yet.

## Not covered

- Merging at `nB` from `CB` with a pre-closure vehicle still on `AB`: the closure lands before any vehicle
  reaches `nA` on `AB`, so `AB` never carries traffic after t=8 and there is no merge conflict at `nB`. Testing
  that needs a closure after some vehicle has already passed `nA`.
- Adaptation changes weights only, not paths; a real decision test needs an open alternative with a measured
  cost difference.
- Rounded departures (SUMO steps are 1 s): flow `period=1.50` is recorded as 1–2 s spacing.
