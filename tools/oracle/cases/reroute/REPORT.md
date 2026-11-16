# Rerouting device / rerouter test

Net: `tools/oracle/cases/real/ref_simple.net.xml`. Routes: `routes.rou` (10 vehicles, departs 0–20 s).
Additional files: `reroute.closing.add.xml` (variant A, closing rerouter only) and `reroute.add.xml`
(variant B, `device.rerouting.*` on the default vType + closing rerouter + destProbReroute rerouter).
Closed edge: `1117867029#1`. Run with `--end 200 --tripinfo-output --statistic-output`.

## Results

| Run | Ours | Reference (C++ sumo 1.28.0, WSL) |
|---|---|---|
| A: closing rerouter only | PANIC: `crates\sumo_core\src\utils\shapes\shape_handler.rs:88` `ShapeHandler::loadFiles: XMLSubSys::runParser not wired` | exit 0, 10 arrivals, `rerouteNo=0` for all |
| B: device attrs + 2 rerouters | PANIC: `crates\sumo_core\src\utils\vehicle\sumo_route_handler.rs:630` `Invalid parsing embedded VType` (also warns `Unknown car-following model rerouter`) | exit 0, 10 arrivals, `rerouteNo=0` for all, devices `tripinfo_v1 fcd_v1` (no rerouting device) |

Control (no `--additional-files`): ours runs, exit 0, output written.

## Findings

- Ours cannot load any `--additional-files`. Variant A panics in the shape loader before any rerouter is parsed. Variant B fails earlier, in the vType/route handler. The rerouting logic itself is not reached.
- Arrivals vs reference: not comparable (ours does not run). Reference: 10/10 arrived in both variants.
- Did vehicles avoid the closed edge: **no**, in the reference either. `rerouteNo=0` for every vehicle, and `routeLength` matches the no-rerouter control (v1: 16.02). Reference logs show failed reroute attempts (`not allowed on source edge` / `No route`), so vehicles kept their routes.
- SUMO 1.28 ignores `device.rerouting.*` given as attributes on `<vType>`. The full-variant reference has no rerouting device. The attribute form therefore does not test rerouting. The `<param key="device.rerouting.probability" value="1"/>` form is probably needed, but I did not test it.
- FCD check is inconclusive: `1117867029#1` is 0.20 m long, so 1 s FCD sampling never records it. Tripinfo is the reliable signal.
- `compare_sim` / `compare_stat` not run: ours produced no output.

## Verdict

Ours panics on additional-file loading (not rerouting-specific). Rerouting cannot be evaluated until the XML additional loader is wired. The reference shows no reroute happened, so the test case itself needs a `<param>`-based device before it measures anything.
