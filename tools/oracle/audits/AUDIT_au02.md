# AUDIT au02 — `microsim/ms_base_vehicle.rs` virtual dispatch, globals, dead code, nullptr

Scope: `crates/sumo_core/src/microsim/ms_base_vehicle.rs` (grep + targeted windows). No cargo run, no .rs edits.
C++ refs: `src/microsim/MSBaseVehicle.{h,cpp}`, `MSVehicle.{h,cpp}`, `mesosim/MEVehicle.{h,cpp}`, `utils/vehicle/SUMOVehicle.h`.

## How dispatch works in this port (read before the table)

- `MSBaseVehicle` has an inherent method for each base virtual. A bare `self.foo()` inside `ms_base_vehicle.rs`
  resolves to that inherent method, never to the MSVehicle/MEVehicle override.
- `SUMOVehicle` (`utils/vehicle/sumo_vehicle.rs`) carries a function-pointer table `self.vt`. Only the entries in
  that table dispatch. Entries are populated in `ms_vehicle.rs` (~L2536-2600). `self.base.foo()` reaches them.
- Working pattern (verified): `self.base.get_reroute_origin()` (L1344, L1657); `self.get_brake_gap(false)` (L1377,
  no inherent on MSBaseVehicle, so Deref to SUMOVehicle's vt); `SUMOVehicle::has_valid_route_start(self, ..)` (L2703).
- Not dispatched (no vt entry, base body runs): `has_arrived`, `has_influencer`, `get_base_influencer*`,
  `instant_stopping`, `handle_collision_stop`, `resume_from_stopping`, `replace_vehicle_type`,
  `reset_approach_on_reroute`, `update_best_lanes`.

## Findings (ranked by impact)

| # | file:line | class | C++ ref | what Rust does | impact | exact fix |
|---|---|---|---|---|---|---|
| 1 | ms_base_vehicle.rs:4049-4050 (`abort_next_stop`) → L3998-4000 | 1 missing virtual dispatch (pure virtual) | MSBaseVehicle.cpp:2029-2032 `resumeFromStopping()` → MSVehicle.cpp:7470 (MSVehicle.cpp:9244 in Rust) | Calls base `resume_from_stopping`, which `panic!`s. Nothing reaches MSVehicle. | **certain**: `abortNextStop(0)` on any stopped MSVehicle panics (TraCI path) | Add `resume_from_stopping: fn(&mut SUMOVehicle) -> bool` to the vt (sumo_vehicle.rs:11-96), set it to the MSVehicle fn in ms_vehicle.rs vt (~L2536), set MEVehicle's entry, and call `self.base.resume_from_stopping()` at L4050. Verified: L3998 panics; ms_vehicle.rs:9244 has the real body. |
| 2 | ms_base_vehicle.rs:2182 (`get_odometer`) → inherent `has_arrived` L2114 | 1 missing virtual dispatch | MSBaseVehicle.cpp:871 `hasArrived()` → MSVehicle.cpp:1124 `hasArrivedInternal(false)` (arrivalEdge, stops, and position conditions, MSVehicle.cpp:1130-1134) | Base returns `succ_edge(1).is_null()` only. Equivalent only at route end. | **certain** for MSVehicle on last edge before `arrivalPos` (odometer uses `myArrivalPos` too early). Also MEVehicle overrides (MEVehicle.h:151). | Add vt entry `has_arrived: fn(&SUMOVehicle) -> bool`; set MSVehicle/MEVehicle entries; call `self.base.has_arrived()` at L2182. |
| 3 | ms_base_vehicle.rs:2983 (`get_singular_type`) → L2943 `replace_vehicle_type` | 1 missing virtual dispatch | MSBaseVehicle.cpp:2804 → MSVehicle.cpp:5041 (also rebuilds `myCFVariables`, L5043-5044) | Base only. `myCFVariables` stays stale for the old type. Base also drops speedFactor and energy params (shortcuts L2944-2951). | **certain** for MSVehicle with a singular type (TraCI setType / vehicle-specific types): car-following state is from the old type. | Add vt entry `replace_vehicle_type`; call `self.base.replace_vehicle_type(type_)`. ms_vehicle.rs:10115 already has the override body. |
| 4 | ms_base_vehicle.rs:3642-3679 (`add_stop`) | 2 stale/missing call (no dispatch) | MSBaseVehicle.cpp:1645-1647 `handleCollisionStop()` (virtual → MSVehicle.cpp:7443); L1628 `instantStopping()` (virtual → MSVehicle.cpp:8474) | `add_stop` is a truncated port. It never calls `handle_collision_stop` or the `tooClose` / brake-gap checks. The shortcut at L3643-3644 lists access/meso/insertion checks but not collision handling. | **certain** for stops with `collision="true"`: C++ handles the collision, Rust silently adds the stop. | Port L1645-1647 into `add_stop` after the position check. Add vt entry `handle_collision_stop: fn(&mut SUMOVehicle, &mut MSStop, f64) -> bool` and call `self.base.handle_collision_stop(..)`. Document any remaining gap in a shortcut comment. |
| 5 | ms_base_vehicle.rs:2216-2217 (`get_impatience`) → inherent `has_influencer` L4716 (returns `false`), `get_base_influencer_2` L4709 | 1 missing virtual dispatch + hardcoded value | MSBaseVehicle.cpp:1240 `hasInfluencer()` / `getBaseInfluencer()` → MSVehicle.h:1700-1706 (`myInfluencer != nullptr`) | Always `false`, so extra impatience from a TraCI influencer is ignored. Enabling the naive fix would hit the panic in `get_base_influencer*` (L4703/4710). | **certain** for vehicles with an influencer; impatience and `waiting_term` differ. | Add vt entries `has_influencer` and `get_base_influencer_2` (the latter must return the MSVehicle influencer, not panic). Call `self.base.has_influencer()`. Do not enable one without the other. |
| 6 | ms_base_vehicle.rs:2853 (`is_rail`) → inherent `get_current_edge` L852 → `get_edge()` | 1 missing virtual dispatch | MSBaseVehicle.cpp:1109 `getCurrentEdge()` → MSVehicle.cpp:8368 (`myLane ? &myLane->getEdge() : getEdge()`) | Uses the route edge. On an internal lane the lane's edge differs, so the rail-permission check reads the wrong edge. | **possible**: rail-class vehicles crossing junction internal lanes. | `self.base.get_current_edge()` at L2853. vt entry `get_current_edge` already exists (ms_vehicle.rs:2540, verified). |
| 7 | ms_base_vehicle.rs:5541 (`reset_approach` call site) → inherent L5249 (empty body) | 1 missing virtual dispatch | MSBaseVehicle.cpp:1791 → MSVehicle.cpp:8449 (removes approach entries for diverged `myLFLinkLanes`) | Base is `{}`. Stale approach entries survive a reroute. | **possible** (depends on `myLFLinkLanes` state at reroute time; unverified runtime). | Add vt entry `reset_approach_on_reroute`; call `self.base.reset_approach_on_reroute()`. ms_vehicle.rs:16248 has the body. |
| 8 | ms_base_vehicle.rs:1690-1696 (`replace_route` dispatch) | 1 unsafe dispatch | MSBaseVehicle.cpp:605 virtual `replaceRoute`; MEVehicle.h:356 overrides it too | `Ptr::cast` is unchecked (utils/common/ptr.rs:56) and `veh.get().is_some()` is true for any non-null pointer. An **MEVehicle** is therefore routed into `MSVehicle::replace_route` (UB on the wrong layout). The comment at L1687-1689 claims the MSVehicle path is correct, but it does not check the type. | **possible**: needs a meso vehicle reroute. Meso is not ported (HANDOVER), but `me_vehicle.rs:1201` exists. (`Ptr::get()` semantics unverified.) | Replace the cast with a type tag or a vt entry `replace_route` set by both MSVehicle and MEVehicle. Do not rely on an unchecked `cast`. |
| 9 | ms_base_vehicle.rs:1503, 1558, 1685 (shortcut comments) | 2 stale shortcut: global "not ported" | MSGlobals::gCheckRoutes (C++ MSBaseVehicle.cpp:1622 and others) | Comments say `gCheckRoutes` is "not ported, treated as false". `MSGlobals::check_routes()` is ported (called at L2704; `impl MSGlobals` at ms_globals.rs:61+). Branches still use `false`. | **possible**: with `ignore-route-errors` unset, the C++ check branches are skipped. | Replace the literal `false` with `MSGlobals::check_routes()` at each site (L1503, L1558, L1685). Check C++ 1620-1625 first (unverified). |
| 10 | ms_base_vehicle.rs:358 (`shortcut: MSGlobals::gHaveEmissions is not ported`) | 2 stale shortcut | MSBaseVehicle.cpp (emission params forced when gHaveEmissions) | Comment says `gHaveEmissions` not ported. `ms_globals.rs:57` lists it, so it is ported. The branch around L361 may still be skipped. | **possible** if emissions are on (unverified: I did not read L355-365). | Read L355-365 and use `MSGlobals::have_emissions()` if the branch is gated on it. |
| 11 | ms_base_vehicle.rs:1545 (`gWeightsRandomFactor ... assumed 1`) | 2 hardcoded constant | MSEdgeWeightsStorage global (C++ `gWeightsRandomFactor`) | Constant `1`. Option `weights.random-factor` is ignored here. | **possible** (unverified: whether that option reaches this path). | Read the option via `utils::options::options_cont::get_float("weights.random-factor")` if the C++ source does, after checking the C++ site. |
| 12 | ms_base_vehicle.rs:1375 (`gUseMesoSim not ported`) | 2 stale shortcut | MSGlobals::gUseMesoSim (C++ MSBaseVehicle.cpp:318) | Branch skipped. `use_meso_sim` is ported; the skipped branch only fires in meso runs. | **none** for microsim-only runs. | Replace with `MSGlobals::use_meso_sim()` when meso lands. |
| 13 | ms_base_vehicle.rs:4439-4444, 4348 (`update_best_lanes`) | 3 ported, never called | MSBaseVehicle.cpp:2347 (insertStop path) → MSVehicle.cpp:6023 | Only referenced from commented C++ and the insertStop shortcut (L4422). Base body is never reached. | **none** now (insertStop errors out at L4423). Becomes a missing-dispatch bug once insertStop is ported. | When insertStop lands, add vt entry `update_best_lanes` and call via `self.base`. |
| 14 | ms_base_vehicle.rs:2646 (`has_valid_route_start` inherent) | 3 ported, never called directly | MSBaseVehicle.cpp:984 / MSVehicle.cpp:1091 | Inherent base is not called. The live path is `SUMOVehicle::has_valid_route_start` via vt (L2703), which correctly reaches MSVehicle (ms_vehicle.rs:2552). | **none** (verified dispatch is correct). | No fix; consider removing the inherent duplicate later. |

## Verified correct (no finding)

- `replace_route_edges` → `replace_route` (L1690-1695): MSVehicle branch dispatches (see #8 for the MEVehicle caveat).
- `get_reroute_origin` (L1344, L1657): `self.base.get_reroute_origin()` via vt. Known fix holds.
- `get_brake_gap` (L1377): reaches MSVehicle via SUMOVehicle vt (ms_vehicle.rs:2600).
- `get_route_validity` → `has_valid_route_start`: vt dispatch to MSVehicle (ms_vehicle.rs:2552).
- Null-lane in `replace_route`: `lane.is_null()` guard at L1902 covers the C++ `getLane()->getLength()` deref.

## Nullptr (C++-legal null, Rust dereferences unconditionally)

- None found in scope. Unguarded `.unwrap()` on `myStops.front()` (L1376, L3084, L3123, L3136, L3152, L3866) is guarded by `is_stopped()` / `has_stops()` at the call sites, and C++ front() on empty is UB too (not a nullptr case). Not a finding.

## Out of scope (noticed, not audited)

- `libsumo/vehicle.rs:3333`: `if false /* gCheckRoutes not ported */` (same stale global as #9).
- `microsim/ms_lane.rs:414`: local `fn g_check_routes()` re-reads the option on each call instead of `MSGlobals`. Matches the HANDOVER class-1 pattern; no current divergence found.

## Counts

- 14 rows: certain 5 (#1-#5), possible 6 (#6-#11), none 3 (#12-#14). #13 counted under none.
- Unverified: #8 (`Ptr::get` semantics), #10 (L355-365 body), #11 (C++ site), #7 (runtime state).
