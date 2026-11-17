# reroute_bypass — mid-route detour through a real junction structure

## Net

`n0 → nA` (edge `IN`), then a choice at junction `nA`:
- direct `AB` (`nA → nB`, 200 m), or
- bypass `AC` + `CB` (`nA → nC → nB`, 141 + 141 m).

Then `OUT` (`nB → nD`). Lead-in `LEAD` (`nS → n0`, 100 m) so the vehicle enters `IN` after the closure starts.
Built from plain XML with `--node-files/--edge-files/--connection-files`.

## Case

- One vehicle `v0`, route `LEAD IN AB OUT`, departs at t=0.
- Routing device by `<param>` form (`routes.rou`): probability 1, pre-period 0, **period 1000**.
- Closing rerouter on `IN` (`closing.add.xml`), `closingReroute id="AB"`, interval 2–200. The vehicle enters `IN`
  at about t=8, while the interval is active, so the closure lands mid-route.

## Deviation from the brief: `period` is 1000, not 1

With `device.rerouting.period=1`, C++ 1.28 reverts the closure detour. The periodic device reroute replans
without the rerouter's prohibition and sends `v0` back onto `AB`. The C++ reference is `rerouteNo=2`, route
`LEAD IN AB OUT`, arrival 53.00, so it never detours. A case with that config cannot test a detour. I kept the
main case on `period=1000`, where C++ detours (see below). The `period=1` configuration is its own case,
`reroute_bypass_p1`, and it FAILs against us. See that case's `CASE.md`.

## Proof (C++ 1.28.0, `ref.*`)

| check | result |
|---|---|
| `v0` `rerouteNo` | **1** |
| route before closure | `LEAD IN AB OUT` |
| route after closure | `LEAD IN AC CB OUT`, `routeLength=777.08` (direct was 695.01) |
| FCD lanes | `LEAD_0 → IN_0 → AC_0 → CB_0 → OUT_0` (no `AB_0`) |
| arrival | 61.00 |

Closure-only check: with no routing device, C++ also detours through `AC`/`CB` (`rerouteNo=1`), so the closing
rerouter alone is enough and the device is not the cause of the detour.

## Verdict

- **Net:** our `netconvert` matches C++ netconvert: `compare_net.py` gives `SUMMARY: 0 differences (42 elements)`.
  `run_all.sh` lists `net SKIP` for this case because the net stage only regenerates from `input.osm`.
- **Sim:** `reroute_bypass/ref` **PASS**: tripinfo MATCH, FCD MATCH, statistics MATCH (ours detours to `AC`/`CB`
  exactly as C++ does).
- **run_all.sh:** PASS=27 FAIL=0 SKIP=9 (the 26 prior rows plus this one). The `reroute_bypass_p1` row is a FAIL
  row, see below.

## Finding: periodic device reroute diverges under a closure (`reroute_bypass_p1`)

At `period=1`, C++ keeps `AB` (`rerouteNo=2`, arrival 53.00). Ours keeps the bypass (arrival 61.00). The first
difference is FCD `t=20`, `v0` `x`: ref 286.55, ours 286.38. That is before the decision point. The gap appears
as soon as the periodic device reroutes, so it is a real port divergence in device rerouting under a closure,
not a timing artefact. Not investigated further in this session.
