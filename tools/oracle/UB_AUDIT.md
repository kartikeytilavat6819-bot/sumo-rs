# UB audit: writes through `&self` (`Ptr::new(self).m()` and `*const Self as *mut Self`)

Rule: a `&self` method must not write a field that a `&self` method reads. The optimiser
assumes it cannot happen, so it can return stale values.

Verdict key:
- **UB-fixed**: the written field is a scalar and is now a `Cell`; reads use `.get()`, writes use `.set()`.
- **UB-deferred**: real UB, not fixed in this pass (non-`Copy` field, or a signature change that ripples).
- **legit**: no `&self` method reads the written field. Left as is (see caveat).

Line numbers are from HEAD before this pass.

## Original grep: `Ptr::new(self).m()` (21 live writes, plus 1 non-write call)

| File:line | Site | Verdict |
|---|---|---|
| `microsim/ms_edge.rs:2551` | `set_last_failed_insertion_time` (read by `get_last_failed_insertion_time`, `insert_vehicle`) | UB-fixed (`Cell<SUMOTime>`) |
| `microsim/ms_edge.rs:3242,3244` | `add_transportable` (read by `get_persons` / `get_sorted_*`) | UB-deferred: `myPersons`/`myContainers` are `BTreeSet`s; fixing needs `get_persons()` to return `Ref`, which ripples into ~12 callers |
| `microsim/ms_edge.rs:3289` | `mark_delayed` (read by `is_delayed`, `get_current_travel_time`) | UB-fixed (`Cell<bool>`) |
| `microsim/ms_edge.rs:3598` | `add_waiting` (read by `get_waiting_vehicle`) | UB-deferred: `Vec` field, needs `RefCell` |
| `microsim/ms_edge.rs:4016` | `invalidate_meso_cache` writes `myLastCacheUpdate` | legit: never read in the crate (only in commented C++) |
| `microsim/ms_lane.rs:2574` | `get_last_vehicle_information`: `myLeaderInfoTime` | UB-fixed (`Cell<SUMOTime>`) |
| `microsim/ms_lane.rs:2573` | same fn: `myLeaderInfo` (`MSLeaderInfo`) | UB-deferred: non-`Copy`, needs `RefCell` |
| `microsim/ms_lane.rs:2652` | `get_first_vehicle_information`: `myFollowerInfoTime` | UB-fixed (`Cell<SUMOTime>`) |
| `microsim/ms_lane.rs:2651` | same fn: `myFollowerInfo` (`MSLeaderInfo`) | UB-deferred: non-`Copy`, needs `RefCell` |
| `microsim/transportables/msp_model_striping.rs:2867` | `get_angle` cache `myAngle` (read in same fn) | UB-fixed (`Cell<f64>`) |
| `netbuild/nb_traffic_light_definition.rs:901` | `init_needs_cont_relation`: `myNeedsContRelationReady` | UB-fixed (`Cell<bool>`) |
| `netbuild/nb_traffic_light_definition.rs:896` | same fn: `myNeedsContRelation` (set) | UB-deferred: non-`Copy` |
| `netbuild/nb_traffic_light_definition.rs:936` | `init_extra_conflicts`: `myExtraConflictsReady` | UB-fixed (`Cell<bool>`) |
| `netbuild/nb_traffic_light_definition.rs:932` | same fn: `myExtraConflicts` (set) | UB-deferred: non-`Copy` |
| `netbuild/nb_own_tl_def.rs:1932` | `init_needs_cont_relation`: `myNeedsContRelationReady` | UB-fixed (`Cell<bool>`) |
| `netbuild/nb_own_tl_def.rs:1920,1928` | same fn: `myNeedsContRelation` clear/extend (set) | UB-deferred: non-`Copy` |
| `netbuild/nb_loaded_sumotl_def.rs:1883` | `Ptr::new(self).m().get_max_index()`: no field write. `get_max_index` takes `&mut self` | legit for field writes; aliasing caveat below |
| `utils/router/cch_graph.rs:364` | `invalidate_class_masks`: `myPrimedClasses` | UB-fixed (`Cell<SVCPermissions>`) |
| `utils/router/cch_graph.rs:363` | same fn: `myArcPerm.fill(0)` (`Vec`) | UB-deferred: needs `RefCell` |
| `utils/router/cch_graph.rs:411` | `prime_class_mask`: `myArcPerm[a] |= vClass` | UB-deferred: `Vec`, needs `RefCell` |
| `utils/router/cch_graph.rs:415` | `prime_class_mask`: `myPrimedClasses |= vClass` | UB-fixed (`Cell`) |

## Original grep: `*const Self as *mut Self` (2 sites)

| File:line | Site | Verdict |
|---|---|---|
| `netbuild/nb_algorithms_railway.rs:3116` | `Track::get_successors` cache (`svcSuccessors`) | UB-deferred: `BTreeMap` cache returned by reference; needs `RefCell` plus an owned return |
| `netbuild/nb_algorithms_railway.rs:3138` | `Track::get_via_successors` cache (`svcViaSuccessors`) | UB-deferred: same as above |

## Hits the original grep missed

- `microsim/ms_edge.rs:2109,2116` `insert_vehicle`: `me.m().myFailedInsertionMemory` (`BTreeSet`), read by `&self` in the same fn. UB-deferred (needs `RefCell`).
- `microsim/ms_edge.rs:3259-3261` `remove_transportable`: alias write to `myPersons`/`myContainers`. UB-deferred (see 3242).
- `microsim/ms_edge.rs:3614-3617` `remove_waiting`: alias write to `myWaiting`. UB-deferred (see 3598).
- `netbuild/nb_loaded_sumotl_def.rs:1275-1276,1316,1326`: alias writes to `myNeedsContRelation` / `myExtraConflicts` sets. UB-deferred.
- `netbuild/nb_loaded_sumotl_def.rs:1334-1335`: flag writes, UB-fixed (`Cell<bool>`).

## Alias-write families (`let this/me = Ptr::new(self)`), for the next pass

| File:line | Site | Verdict |
|---|---|---|
| `netbuild/nb_edge.rs:8114-8127` | `get_successors(&self)`: clears and refills `mySuccessors` via `this.m()`, then returns `&self.mySuccessors` | UB-deferred: `&self` write, returns reference into the same field. Needs `RefCell` plus an owned return (signature ripple) |
| `netbuild/nb_edge.rs:8154-8175` | `get_via_successors(&self)`: same pattern on `myViaSuccessors` | UB-deferred: same as above |
| `microsim/devices/ms_device_vehroutes.rs:411-437` | `notify_enter(&mut self)`: `Ptr::from_mut(self)` writes | legit: the method takes `&mut self`, so the writes are not through `&self` |

Not yet audited: `microsim/ms_lane.rs` 8227/8546/8694/8915, `microsim/ms_vehicle.rs` (`let me` sites), `router/ro_route_def.rs`, `netgen/ng_node.rs`, `netbuild/nb_node.rs`, `microsim/output/mse3_collector.rs` 204/209.

## Comment-only hits (no live code)

`utils/router/car_edge.rs` 96, 137; `flipped_node.rs` 78, 95; `flipped_edge.rs` 151; `reversed_edge.rs` 176; `rail_edge.rs` 394. These are `shortcut:` comments that will need `Cell`/`RefCell` when those routers are ported.

## Caveats

- Strictly, under the Rust aliasing model, any write through `&self` to a field without `UnsafeCell` is UB, even when no `&self` method reads it. The **legit** rows (`myLastCacheUpdate`, the `get_max_index` call) are therefore not fully safe. Converting `myLastCacheUpdate` to a `Cell` is a one-line change if we want to close it.
- `get_max_index` takes `&mut self`. Making it `&self` would remove the aliasing violation at 1883. Deferred: it changes a virtual-dispatch signature.

## Verification

- **Junction case** (`tools/oracle/cases/junction`, `ref_simple.net.xml` + `routes.rou`): base (HEAD), after-debug, after-release. `tripinfo.xml` and `fcd.xml` compared after dropping the `generated on` stamp and output-path lines. All three identical (tripinfo 30 lines, fcd 175 lines). Release == debug.
- The junction case does not trigger the UB, so identical output shows no regression, not that the UB is gone.
- **Scoreboard** (`run_all.sh` logic, run against HEAD binaries and the modified binaries, same cases and compare scripts): identical rows. PASS=6 FAIL=4 SKIP=0 both before and after. No PASS row lost. The four FAILs (junction sim, real_dense sim, roundabout sim, signal sim) already fail on HEAD. Note: `run_all.sh` itself hardcodes the main repo and `sumo-rs-target-main`, so it was run as a parameterised copy against each tree.
