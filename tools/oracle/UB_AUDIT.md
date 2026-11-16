# UB audit: writes through `&self` (`Ptr::new(self).m()` and `*const Self as *mut Self`)

Rule: a `&self` method must not write a field that a `&self` method reads. The optimiser
assumes it cannot happen, so it can return stale values.

Verdicts:
- **fixed**: `Cell` for scalars (`.get()` / `.set()`), `RefCell` for collections (`.borrow()` / `.borrow_mut()`). Returned references become owned clones where a `RefCell` cannot lend one.
- **deferred**: real UB, not fixed. The reason is given.
- **legit**: no `&self` method reads the written field, or the method takes `&mut self`.

Line numbers are from the pre-audit HEAD (`a4e7db0`).

## Status

| Commit | Batch | Sites |
|---|---|---|
| `47bb353` (merged `12ee4fb`) | scalar `Cell`s | `ms_edge` 2551, 3289; `ms_lane` 2574, 2652 (times); `msp_model_striping` 2867; `nb_traffic_light_definition` 901, 936; `nb_own_tl_def` 1932; `nb_loaded_sumotl_def` 1334-1335; `cch_graph` 364, 415 |
| `d72636c` | `NBEdge` successor caches | `nb_edge` 8114-8175 (`get_successors`, `get_via_successors`) |
| `c5f0652` | railway `svc*` caches | `nb_algorithms_railway` 3116, 3138 (both `*const Self as *mut Self` casts removed) |
| `0723062` | `MSEdge` waiting list and failed-insertion memory, CCH arc permissions | `ms_edge` 2109, 2116, 3614-3617, 3646, 3685; `cch_graph` 323, 363, 411 |
| `923d923` | traffic-light need-cont-relation and extra-conflict sets | `nb_traffic_light_definition` 896, 932; `nb_own_tl_def` 1920, 1928; `nb_loaded_tl_def` 459, 487; `nb_loaded_sumotl_def` 1275-1276, 1316, 1326 |

## Deferred

| Site | Why |
|---|---|
| `microsim/ms_edge.rs` 3247, 3249 (`add_transportable`), 3259-3261 (`remove_transportable`) | `myPersons` / `myContainers` need `RefCell`, which changes `get_persons()` to return `Ref`. That ripples into callers in `ms_link.rs` (3807, 3814), `ms_lane.rs` (3908, 8086, 9010), `ms_vehicle.rs` (13562, 13574), and `libsumo`/output files. Those are owned by other agents, so this is a separate step to revert alone. |
| `microsim/ms_lane.rs` 2573, 2651 (`MSLeaderInfo` leader/follower caches) | `ms_lane.rs` is owned by another agent (MSGlobals sweep and arrival-speed work). The fix is `RefCell<MSLeaderInfo>`; it is a few lines and ready to apply once the file is free. |
| `microsim/ms_lane.rs` `let this = Ptr::new(self)` alias sites (8227, 8546, 8694, 8915) | Not audited. `ms_lane.rs` is owned by another agent. |
| `microsim/ms_vehicle.rs` `let me = Ptr::new(self)` sites (8644, 13388, 14539, 14772, 14896, 15283, 15424, 15498) | Not audited. `ms_vehicle.rs` is owned by another agent. |
| `microsim/ms_base_vehicle.rs`, `microsim/cfmodels/mscf_model*`, `microsim/output/ms_mean_data.rs`, `microsim/ms_link.rs` | Not audited. Owned by other agents this round. |
| `netbuild/nb_node.rs` 2283, 2585; `netgen/ng_node.rs` 277 (`let me/this = Ptr::new(self)`) | Not audited: needs a check of each method's receiver and writes. |
| `router/ro_route_def.rs` 235, 501 (`Ptr::new(self).get_mut()`) | Not audited. Needs a check whether the method is `&self`. |
| `microsim/output/mse3_collector.rs` 204, 209 | Not audited. |

## Legit (no `&self` write to a read field)

| Site | Why |
|---|---|
| `microsim/ms_edge.rs` 4023 `invalidate_meso_cache`: `myLastCacheUpdate` | Never read in the crate (only in commented C++). Caveat below. |
| `netbuild/nb_loaded_sumotl_def.rs` 1883: `Ptr::new(self).m().get_max_index()` | No field write. `get_max_index` takes `&mut self`. Caveat below. |
| `microsim/devices/ms_device_vehroutes.rs` 421-437 `notify_enter` | Takes `&mut self`. |
| `netbuild/nb_edge.rs` 4107 `compute_edge2_edges` | Takes `&mut self`. |

Comment-only (no live code): `utils/router/car_edge.rs` 96, 137; `flipped_node.rs` 78, 95; `flipped_edge.rs` 151; `reversed_edge.rs` 176; `rail_edge.rs` 394. These will need `Cell`/`RefCell` when those routers are ported.

## Caveats

- Under the Rust aliasing model, a write through `&self` to a field without `UnsafeCell` is UB even when nothing reads it. The two legit `&self` rows (`myLastCacheUpdate`, `get_max_index`) are not fully safe. Converting `myLastCacheUpdate` to `Cell` is one line. `get_max_index` needs `&self` in its trait, which changes a virtual signature.
- The junction, roundabout, and real cases don't exercise these caches in a way that would show stale values, so identical output shows no regression, not that the UB is gone.

## Verification (batch 4, `923d923`, and the same for every batch above)

- Cases: `junction`, `roundabout`, `real`. Release and debug run with the same command line as `run_all.sh` (including `sim.args`). `tripinfo.xml`, `fcd.xml`, and exit code compared after dropping the `generated on` stamp and output-path lines.
- Release == debug on all three cases for every batch. Output identical to the pre-audit baseline on all three.
- Scoreboard (`run_all.sh` logic, run from a copy because `run_all.sh` hardcodes the main checkout and `sumo-rs-target-main`): PASS=8 FAIL=2 SKIP=0 before and after every batch. The two FAILs (`real_dense` sim, `signal` sim) fail on the baseline too. No PASS row lost.
- Baseline is the current `wave2` (`12ee4fb`), which includes the junction fix `0868319`. `main` does not contain the UB commits; `wave2` is `main` plus them.
