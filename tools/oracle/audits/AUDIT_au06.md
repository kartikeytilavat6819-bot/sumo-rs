# AUDIT au06 — microsim/cfmodels (virtual dispatch, stale globals, unported code, nullptr)

Scope: `crates/sumo_core/src/microsim/cfmodels/` (Rust) vs `microsim/cfmodels/` + the MSGlobals/MSFrame/MSEdgeControl writers (C++, read-only).
Method: read both sides. No cargo run, no .rs edits. **Nothing below was reproduced by a simulation; all findings are static-read unless marked otherwise.**

## 1. Virtual-dispatch matrix

Rust dispatch: each model installs a `MSCFModelVt` (`&'static`) in its constructor; base helpers call `self.vt.*` (mscf_model.rs:224-288). Legend: `✓` C++ override exists **and** Rust vt slot present; `·` no C++ override (inherits base / vt default); `✗` C++ override exists, **no** Rust slot.

| model | fin | patch | free | follow | insFol | insStop | stop | intGap | maxNext | minNext | minNextEm | maxNSM | secGap | brake3 | maxLaneCF | startup | createVV | setters | get/setParam |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| ACC | · | · | · | ✓ | ✓ | · | ✓ | ✓ | · | · | · | · | ✓ | · | ✓ | · | ✓ | · | · |
| CACC | · | · | ✓ | ✓ | ✓ | · | ✓ | ✓ | · | · | · | · | ✓ | · | ✓ | · | ✓ | setHW ✓ | ✗ |
| CC | ✓ | · | ✓ | ✓ | ✓ | · | ✓ | ✓ | ✓ | ✓ | · | · | ✓ | · | · | · | ✓ | · | ✗ |
| Daniel1 | ✓ | · | · | ✓ | · | · | ✓ | · | · | · | · | · | · | · | · | · | · | imp/maxDec/hw ✓ | · |
| EIDM | ✓ | · | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | · | · | · | ✓ | ✓ | ✓ | ✓ | · | ✓ | · | · |
| IDM | ✓ | · | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | · | ✓ | · | · | ✓ | · | · | · | ✓ | · | · |
| Kerner | ✓ | · | · | ✓ | · | · | ✓ | · | · | · | · | · | · | · | · | · | ✓ | · | · |
| Krauss | · | ✓ | · | ✓ | · | · | ✓ | · | · | · | · | · | · | · | · | · | ✓ | inherits Orig1 ✓ | · |
| KraussOrig1 | · | ✓ | · | ✓ | · | · | ✓ | · | · | · | · | · | · | · | · | · | · | imp/maxDec/hw ✓ | · |
| KraussPS (: Krauss) | · | · | · | · | · | · | · | · | ✓ | · | · | · | · | · | · | · | · | · | · |
| KraussX (: Krauss) | · | ✓ | · | · | · | · | · | · | · | · | · | · | · | · | · | · | · | · | · |
| NaSch | · | ✓ | · | ✓ | ✓ | · | ✓ | · | · | · | ✓ | · | · | · | · | · | · | imp ✓ | · |
| PWag2009 | ✓ | ✓ | · | ✓ | · | · | ✓ | · | · | · | · | · | · | · | · | · | ✓ | imp ✓ | · |
| Rail | ✓ | · | ✓ | ✓ | · | · | ✓ | · | ✓ | ✓ | ✓ | · | · | · | · | ✓ | ✓ | · | · |
| SmartSK | ✓ | ✓ | · | ✓ | · | · | ✓ | · | · | · | · | · | · | · | · | · | ✓ | maxDec/imp/hw ✓ | · |
| W99 | · | · | · | ✓ | · | · | ✓ | ✓ | · | · | · | · | · | · | · | · | · | · | · |
| Wiedemann | ✓ | · | · | ✓ | · | · | ✓ | ✓ | · | · | · | · | ✓ | · | · | · | ✓ | · | · |

Notes on the matrix:
- EIDM `maxNextSafeMin` (C++ EIDM.h:201, returns 0) is reached: EIDM_VT sets `max_next_safe_min: eidm_max_next_safe_min` (mscf_model_eidm.rs:33), and the MSVehicle caller uses the dispatching `cfModel.max_next_safe_min(...)` (ms_vehicle.rs:10904, 10906). C++ caller is MSVehicle.cpp:3870/3872. **Clean.**
- EIDM 3-arg `brakeGap` (EIDM.h:183) → Rust slot `brake_gap_2` = `eidm_brake_gap`. **Clean.**
- Base helpers → vt, all checked: `finalize_speed_base` → min/max/emergency/patch/startup (mscf_model.rs:447-456); 4-arg `stop_speed` → `stop_speed_2` (604); `insertion_stop_speed_base` → `stop_speed` (628); `interaction_gap_base` → `max_next_speed` (744); `max_next_safe_min_base` → `max_next_speed` (946); `brake_gap` 1-arg → `brake_gap_2` (998); `get_secure_gap_base` → `brake_gap_2` (1182-1189); `follow_speed_transient_base` → `brake_gap_2` (695). **Clean.**
- Qualified C++ calls (`MSCFModel::x(...)` inside overrides: CACC freeSpeed/getSecureGap, CC freeSpeed/minNextSpeed, IDM/Kerner/PWag/SmartSK finalizeSpeed, IDM insertionStopSpeed) map to the Rust `*_base` functions. **Clean.**
- `getSpeedAfterMaxDecel` is non-virtual in C++ (MSCFModel.h:447); Rail hides it (Rail.h:78). Rust mirrors this with an inherent method on `MSCFModel_Rail`; base callers take the base version, as in C++. **Clean.**
- `followSpeedTransient`, `distAfterTime`, `applyStartupDelay`, `getCurrentAccel`, `interpolateProfile`, `minNextSpeedEmergency` (base): C++ has no overrides beyond NaSch/Rail on `minNextSpeedEmergency`. Non-slot Rust fns are therefore faithful. **Clean.**
- **Not in any vtable:** `getParameter` / `setParameter` (C++ MSCFModel.h:707, 720; overridden in CC.cpp:760/1054 and CACC.cpp:259/271). See finding 2.

## 2. Findings (ranked)

| rank | file:line | class | C++ ref | what Rust does | impact | exact fix |
|---|---|---|---|---|---|---|
| 1 (HIGH) | `mscf_model.rs:34-42` (shims) + 31 call sites: `mscf_model.rs` 341, 589, 627, 691, 962, 983, 1017, 1117, 1184, 1551, 1715, 1728, 1800, 1836, 2044, 2048, 2052, 2062, 2200, 2215; `mscf_model_krauss.rs` 185, 261; `mscf_model_krauss_orig1.rs` 128, 150, 271; `mscf_model_krauss_x.rs` 143, 160; `mscf_model_rail.rs` 324, 470, 549; `mscf_model_w99.rs` 307 | base + Krauss, KraussOrig1, KraussX, Rail, W99 | MSGlobals.h:53, 130, 143; MSGlobals.cpp:63, 69, 78; MSFrame.cpp:1227-1243 (writes `gSemiImplicitEulerUpdate`, `gDefaultEmergencyDecel`); MSEdgeControl.cpp:277, 332 (writes `gComputeLC`); MSCFModel.cpp:60 (ctor), 185 (getSecureGap), 987, 996 | Three local `fn g_*()` shadows return **constants** and ignore the real flags: `g_semi_implicit_euler_update()`=true, `g_compute_lc()`=false, `g_default_emergency_decel()`=-1. The real `MSGlobals` statics exist (`ms_globals.rs:115` `semi_implicit_euler_update`, `:173` `default_emergency_decel`, `compute_lc` via `ms_global!`) and MSFrame / ms_edge_control.rs:471/495 already write them. The shim comment at `mscf_model.rs:32-33` ("ms_globals.rs is empty") is stale. This is the handover's bug class 1, which was fixed in ms_vehicle/MSLCM but not here. | (a) `--step-method.ballistic` (and `actionStepLength` without integration method) makes C++ take the ballistic branch; Rust still takes Euler in all 31 sites. Krauss, the default model, is affected. (b) While `gComputeLC` is true (lane-change execution), C++ `getSecureGap` (MSCFModel.cpp:185) and the EIDM-style gap branches differ from Rust. (c) `--default.emergency-decel decel` (C++ value -2) is ignored: every vType without explicit `emergencyDecel` gets the -1 default, changing `myEmergencyDecel`. **Unverified in a run.** | Delete `mscf_model.rs:32-42` (the three shims). Replace each call with `MSGlobals::semi_implicit_euler_update()`, `MSGlobals::compute_lc()`, `MSGlobals::default_emergency_decel()` (all generated by the existing `ms_global!` / `ms_global_f64!` macros). Add `use crate::microsim::ms_globals::MSGlobals;` to the six files if it is not in the prelude. Fix the stale comment. Extend the `ms_globals.rs:224` shadow test to scan `cfmodels/` for `fn g_`, since it currently misses this. |
| 2 (MED) | `ms_base_vehicle.rs:507` and `:725` (outside scope; the cause is missing vtable slots in `mscf_model.rs:78-104`) | CC, CACC | MSCFModel.h:707 (virtual `getParameter`), :720 (virtual `setParameter`, throws at :724 "not supported by carFollowModel"); MSCFModel_CC.cpp:760, :1054; MSCFModel_CACC.cpp:259, :271; MSBaseVehicle.cpp:2718, :2864 | No `get_parameter` / `set_parameter` slot in `MSCFModelVt`, no CC/CACC override, no base port. `ms_base_vehicle.rs` skips the delegation with `// shortcut:` notes (`:507` "getCarFollowModel().getParameter ... not reachable", `:725` "dynamic_cast<MSVehicle*>(this) has no downcast yet"). | TraCI `vehicle.getParameter/setParameter(id, "carFollowModel.<x>")` never reaches CC/CACC handlers, and base's "not supported" error is never raised. Silent divergence for TraCI clients that tune CC/CACC. | Add `get_parameter: fn(&MSCFModel, Ptr<MSVehicle>, &str) -> String` and `set_parameter: fn(&mut MSCFModel, Ptr<MSVehicle>, &str, &str)` to `MSCFModelVt`. Port the base bodies from MSCFModel.h:707-724 as the defaults. Add CC and CACC overrides (CC.cpp:760/1054, CACC.cpp:259/271). Then remove the shortcuts at `ms_base_vehicle.rs:507/725`; the downcast there is a separate blocker (outside this scope). |
| 3 (LOW, latent) | `mscf_model_rail.rs:462-463` `min_next_speed`: `veh.get_slope()` | Rail | MSCFModel_Rail.cpp:334-336 (`veh->getSlope()` unconditional) | Same unconditional deref as C++. Callers that pass a null `veh` into a Rail object are **not** present: the only null-`veh` calls in cfmodels are `mscf_model_acc.rs:340` (ACC, which does not call Rail code) and `mscf_model_idm.rs:182` (IDM `get_secure_gap`, which ignores `pred`/`veh`). C++ `Krauss.cpp:150` calls `minNextSpeedEmergency(speed)` with no `veh`, but Krauss's base does not dereference it. | None today. A future null-`veh` caller on a Rail object would segfault in both languages. | No change needed. Optional: `debug_assert!(!veh.is_null())` at `mscf_model_rail.rs:462`. |
| 4 (INFO) | `mscf_model.rs:32-33` comment | all | — | Comment says `ms_globals.rs` is empty. It is not. | Misleads reviewers into thinking the shims are a temporary fallback. | Covered by fix 1. |

## 3. Checked, clean (no finding)

- **Stale globals / constants:** `INVALID_SPEED` (MSCFModel.h:33) is `299792458 + 1` with no parentheses. Every C++ use is a comparison, assignment, return or default argument (MSLane.cpp:979, 1176, 1468; MSLCM_LC2013.cpp:470-471; MSCFModel.cpp:583), so precedence is harmless. Rust uses `299792459.0`, which is the correct value. `EMERGENCY_DECEL_AMPLIFIER` = 1.2 (MSCFModel.h:35), matched in Rust. `VTYPEPARS_DEFAULT_EMERGENCYDECEL_{DEFAULT,DECEL}` = -1/-2 (SUMOVTypeParameter.h:85-86).
- **Ported but never called:** `get_max_accel_profile` / `get_des_accel_profile` have no Rust callers. C++'s only callers are `MSVehicleType::setMaxAccelProfile/setDesAccelProfile` (MSVehicleType.cpp:544-613), and those bodies are entirely inside `/* */`. So the Rust no-ops in `ms_vehicle_type.rs:704-757` are faithful. `interpolate_profile` has no callers on either side. `free_speed_2` (static) and `brake_gap_euler` are called (eidm, LC). **Clean.**
- **C++-legal nullptr:** `ACC::insertionFollowSpeed` passes `pred=nullptr` into `followSpeed` → `applyHeadwayAndSpeedDifferencePerceptionErrors`. C++ only uses `pred` as a map key (`getPerceivedHeadway(gap, pred)`, `getPerceivedSpeedDifference(…, pred)`), and Rust does the same through `Ptr<()>` (`ms_driver_state.rs:582, 640`). The Rust ACC path (`mscf_model_acc.rs:340`) is faithful. IDM `get_secure_gap` (`mscf_model_idm.rs:373`) does not touch `pred`. **Clean.**
- **Overrides missing from a vtable:** none. Every C++ override in the 17 concrete classes has a Rust slot, except finding 2. KraussPS / KraussX correctly spread `..KRAUSS_VT`, and Krauss spreads `..KRAUSS_ORIG1_VT` (`mscf_model_krauss.rs:311`), so inherited Krauss overrides carry over. **Clean.**

## 4. Unverified

- Whether `ms_vehicle_type.rs`'s equivalent of MSVehicleType.cpp:351 (`getDefaultEmergencyDecel(..., MSGlobals::gDefaultEmergencyDecel)`) reads the real global or a shadow. Not inspected in this pass; the same shadow pattern may be there.
- Whether `ms_base_vehicle.rs:507/725` are the only callers that would need finding 2's slots.
- Runtime effect of finding 1 (no run, per brief).
