# AUDIT au10 — MSGlobals, stand-in constants, event scheduling (classes 1–4)

Scope: `crates/sumo_core/src/microsim/{ms_net,ms_edge_control,ms_insertion_control,ms_vehicle_control,ms_event_control,ms_vehicle_transfer,ms_frame,ms_globals}.rs`, plus a crate-wide grep for `g_*` stubs, `G_*` consts, and `shortcut:` comments that stand in for an option or global.
C++ read-only: `C:\Users\karti\sumo-src\src`. Worktree: `.claude/worktrees/agent-au10` (branch `worktree-agent-au10`, base `93058c0`).
Method: static reading only. No cargo was run. Evidence tags: **V** = read both C++ and Rust bodies, **G** = grep-only, **U** = unverified (inferred, not traced).

---

## 1. `MSFrame::setMSGlobals` (C++ `MSFrame.cpp:1184-1291`) → Rust `ms_frame.rs:2167-2278`

Rust function is `MSFrame::set_ms_globals`, called once from `nl_builder.rs:685` (C++ `NLBuilder.cpp:365`). **Every C++ assignment has a Rust setter call with the same option.** The setter is correct. The problem is that many readers never read the setter's value (see §3 and findings F1–F8).

| C++ line | C++ assignment (option) | Rust line | Same option/value | Notes |
|---|---|---|---|---|
| 1187 | `gOmitEmptyEdgesOnDump = !netstate-dump.empty-edges` | 2171 | ✓ V | |
| 1189 | `gUsingInternalLanes = !no-internal-links` | 2173 | ✓ V | Readers stubbed: F2 |
| 1190-1191 | `gIgnoreJunctionBlocker = ignore-junction-blocker (<0 → max)` | 2174-2175 | ✓ V | |
| 1193 | `gTimeToGridlock = time-to-teleport (<0 → 0)` | 2177-2178 | ✓ V | |
| 1194 | `gTimeToImpatience = time-to-impatience` | 2179 | ✓ V | |
| 1195 | `gTimeToGridlockHighways = time-to-teleport.highways (<0 → 0)` | 2180-2181 | ✓ V | |
| 1196 | `gGridlockHighwaysSpeed = time-to-teleport.highways.min-speed` | 2182 | ✓ V | |
| 1197 | `gTimeToTeleportDisconnected = time-to-teleport.disconnected` | 2183 | ✓ V | |
| 1198 | `gTimeToTeleportBidi = time-to-teleport.bidi` | 2184 | ✓ V | |
| 1199 | `gTimeToTeleportRSDeadlock = time-to-teleport.railsignal-deadlock` | 2185 | ✓ V | |
| 1200 | `gRemoveGridlocked = time-to-teleport.remove` | 2186 | ✓ V | |
| 1201 | `gCheck4Accidents = !ignore-accidents` | 2187 | ✓ V | |
| 1202 | `gCheckRoutes = !ignore-route-errors` | 2188 | ✓ V | Readers stubbed: F6 |
| 1203 | `gEmergencyInsert = emergency-insert` | 2189 | ✓ V | |
| 1204 | `gWeightsSeparateTurns = weights.separate-turns` | 2190 | ✓ V | Reader stub: F14 |
| 1205 | `gStartupWaitThreshold = startup-wait-threshold` | 2191 | ✓ V | |
| 1206 | `gLaneChangeDuration = lanechange.duration` | 2192 | ✓ V | Reader stub: F11 |
| 1207 | `gLateralResolution = lateral-resolution` | 2193 | ✓ V | Reader stub: F3 |
| 1208 | `gSublane = (dur > 0 \|\| lat > 0)` | 2194 | ✓ V | Reader stub: F11 |
| 1209 | `gStateLoaded = isSet(load-state)` | 2195 | ✓ V (isSet semantics U) | `loadState` never sets it: F9 |
| 1210 | `gUseMesoSim = mesosim` | 2196 | ✓ V | |
| 1211 | `gMesoLimitedJunctionControl = meso-junction-control.limited` | 2197 | ✓ V | |
| 1212 | `gMesoInterpolatePos = meso-interpolate-pos` | 2198 | ✓ V | |
| 1213 | `gWaitingTimeMemory = waiting-time-memory` (default "100") | 2199 | ✓ V | Reader stub = 300 s: F7 |
| 1214 | `MSAbstractLaneChangeModel::initGlobalOptions(oc)` | 2200 | ✓ G | callee not traced |
| 1215 | `gOverheadWireSolver = overhead-wire.solver` | 2201 | ✓ V | |
| 1216 | `gOverheadWireRecuperation = overhead-wire.recuperation` | 2202 | ✓ V | |
| 1217 | `Circuit::enforceCurrentLimits(overhead-wire.substation-current-limits)` | 2203 | ✓ V | |
| 1218 | `gInsertionChecks = parseInsertionChecks(insertion-checks)` | 2204 | ✓ V | |
| 1219 | `gMaxRailSignalBlockLength = railsignal.max-block-length` | 2205 | ✓ V | |
| 1221 | `MSLane::initCollisionOptions(oc)` | 2206 | ✓ G | |
| 1223 | `DELTA_T = step-length` | 2207 | ✓ V | |
| 1225-1232 | `gSemiImplicitEulerUpdate = !step-method.ballistic`; auto-false if action step set and ballistic default | 2208-2215 | ✓ V | Readers stubbed: F1; auto-switch in vtype missing: F5 |
| 1233-1234 | `gActionStepLength = processActionStepLength(default.action-step-length)` | 2216-2217 | ✓ V | |
| 1236-1244 | `gDefaultEmergencyDecel`: "default"→-1, "decel"→-2, else float (`SUMOVTypeParameter.h:85-86`) | 2218-2228 | ✓ V (constants match) | Reader stub = -1 always: F1 |
| 1245 | `gNumSimThreads = threads` | 2229 | ✓ V | |
| 1246 | `gNumThreads = max(threads, device.rerouting.threads)` | 2230 | ✓ V | |
| 1248 | `gEmergencyDecelWarningThreshold` | 2231 | ✓ V | |
| 1249 | `gMinorPenalty = weights.minor-penalty` (default **1.5**, `MSFrame.cpp:684`) | 2232 | ✓ V | Reader const = 0: F2 |
| 1250 | `gTLSPenalty = weights.tls-penalty` | 2233 | ✓ V | |
| 1251 | `gTurnaroundPenalty = weights.turnaround-penalty` (default **5.0**, `MSFrame.cpp:690`) | 2234 | ✓ V | Reader const = 0: F2 |
| 1253 | `gModelParkingManoeuver = parking.maneuver` | 2235 | ✓ V | Readers hardcoded false: F4 |
| 1255 | `gStopTolerance = ride.stop-tolerance` | 2236 | ✓ V | Reader const = 10: low |
| 1256 | `gTLSYellowMinDecel = tls.yellow.min-decel` | 2237 | ✓ V | |
| 1257 | `gUseStopEnded` | 2238 | ✓ V | |
| 1258 | `gUseStopStarted` | 2239 | ✓ V | |
| 1259 | `gSlopeCentered` | 2240 | ✓ V | |
| 1262-1269 | railsignal default classes → `MSRailSignalControl::initSignalized` | 2241-2249 | ✓ V | |
| 1270-1273 | `gTaxiClasses = device.taxi.vclasses` | 2250-2255 | ✓ V value | i64 truncated to i32, see F15 |
| 1275-1285 | `parseDepartLane/Speed(default.departlane / default.departspeed)` | 2256-2276 | ✓ V | |
| 1287-1291 | `movereminder-output` (`#ifdef _DEBUG`) | 2277 shortcut | N/A (release build) | |

**Other C++ writers of MSGlobals** (non-GUI):

| C++ site | Assignment | Rust site | Status |
|---|---|---|---|
| `NLBuilder.cpp:140` | `gUsingInternalLanes = false` | `nl_builder.rs:418` | ✓ value, branch condition U |
| `NLHandler.cpp:94` | `gLefthand = attr lefthand` | `nl_handler.rs:641` | ✓ value, default arg U |
| `MSEdgeControl.cpp:277/332` | `gComputeLC = true / false` around `changeLanes` | `ms_edge_control.rs:471/495` | ✓ V |
| `MSNet.cpp:266` | `gMesoNet = new MELoop` | `ms_net.rs:402-404` | ✓ intentional loud panic (meso not wired) |
| `MSNet.cpp:1095/1148` | `gClearState = true / false` | `ms_net.rs:1937/1980` | ✓ V |
| `MSNet.cpp:1236` | `gHaveEmissions = true` (when `full-output` set) | **none** | ✗ F8 |
| `MSNet.cpp:1876` | `gStateLoaded = true` (in `loadState`) | **none** (`load_state` panics, `ms_net.rs:2792`) | ✗ F9 |
| `MSVehicleType.cpp:475` | `gSemiImplicitEulerUpdate = false` (auto-switch to ballistic) | **none** (`ms_vehicle_type.rs:1572-1574` shortcut) | ✗ F5 |
| `GUILane.cpp:1607`, `GUINet.cpp:289` | GUI-only | n/a | n/a |

---

## 2. Stand-in `g_*` functions, `G_*` consts and stale "not ported" comments (whole crate)

"Returns" = what the Rust code computes. "Should read" = the real global, with its real default-run value.

### 2a. Local stubs / consts that shadow a global

| Rust file:line | Symbol | Returns | Should read (C++ value, default run) | Status |
|---|---|---|---|---|
| `cfmodels/mscf_model.rs:34` | `g_semi_implicit_euler_update()` | `true` | `MSGlobals::semi_implicit_euler_update()` (false if `step-method.ballistic`) | ✗ F1 |
| `cfmodels/mscf_model.rs:37` | `g_compute_lc()` | `false` | `MSGlobals::compute_lc()` (true only inside `changeLanes`) | ✗ F1 |
| `cfmodels/mscf_model.rs:40` | `g_default_emergency_decel()` | `-1` | `MSGlobals::default_emergency_decel()` (-1 / -2 / float) | ✗ F1 |
| `microsim/ms_edge.rs:13` | `G_USING_INTERNAL_LANES` | `true` | `using_internal_lanes()` (false if `no-internal-links`) | ✗ F2 |
| `microsim/ms_edge.rs:14` | `G_USE_MESO_SIM` | `false` | `use_meso_sim()` (false) | ✓ equal |
| `microsim/ms_edge.rs:15` | `G_TLS_PENALTY` | `0` | `tls_penalty()` (0) | ✓ equal |
| `microsim/ms_edge.rs:16` | `G_MINOR_PENALTY` | `0` | `minor_penalty()` (**1.5**) | ✗ F2 |
| `microsim/ms_edge.rs:17` | `G_TURNAROUND_PENALTY` | `0` | `turnaround_penalty()` (**5.0**) | ✗ F2 |
| `microsim/ms_edge.rs:18` | `G_LANE_CHANGE_DURATION` | `0` | `lane_change_duration()` (0) | ✓ equal default |
| `microsim/ms_edge.rs:19` | `G_STOP_TOLERANCE` | `10` | `stop_tolerance()` (10) | ✓ equal default |
| `microsim/ms_lane.rs:401` | `g_lateral_resolution()` | `-1` | `lateral_resolution()` (-1 default) | ✗ F3 (wrong when option set) |
| `microsim/ms_lane.rs:409` | `g_emergency_insert()` | reads option | `emergency_insert()` | ✓ equal, duplicate source (F13) |
| `microsim/ms_lane.rs:414` | `g_check_routes()` | `!option` | `check_routes()` | ✓ equal, duplicate source (F13) |
| `microsim/output/ms_mean_data.rs:10` | `g_use_meso_sim()` | `false` | `use_meso_sim()` | ✓ equal |
| `microsim/output/ms_mean_data.rs:13` | `g_using_internal_lanes()` | `true` | `using_internal_lanes()` | ✗ F12 (U on C++ use site) |
| `router/ro_edge.rs:325` | `g_weights_random_factor()` | reads option per call | `weights.random-factor` | ✓ equal (not a global copy) |
| `microsim/devices/ms_device_taxi.rs:10` | `g_taxi_classes()` | wraps `std_defs` | `gTaxiClasses` | ✓ wired (not a stub) |
| `microsim/ms_leader_info.rs:10` | `g_lateral_resolution()` | `MSGlobals::lateral_resolution()` | ✓ wired (the earlier shadow is gone) |
| `microsim/ms_link.rs:15-93` | `g_compute_lc`, `g_sublane`, `g_ignore_junction_blocker`, `g_lefthand`, `g_use_meso_sim`, `g_using_internal_lanes` | `MSGlobals::*()` | ✓ wired |
| `utils/router/intermodal_network.rs:93` | `g_taxi_classes_perm()` | not traced | U |

### 2b. Inline `shortcut:` / `false` / `true` literals standing in for a global (not in the scope files, found by grep)

| Rust file:line | Literal / comment | Value used | Should be | Status |
|---|---|---|---|---|
| `microsim/ms_vehicle_transfer.rs:292-293` | `let gModelParkingManoeuver = false;` | false | `MSGlobals::model_parking_manoeuver()` (`parking.maneuver`) | ✗ F4 |
| `microsim/ms_vehicle_transfer.rs:304-305` | `let gLefthand = false;` | false | `MSGlobals::lefthand()` | ✗ F4 |
| `microsim/ms_vehicle_type.rs:1572-1574` | "cannot be written, flag not switched off" | flag untouched | `set_semi_implicit_euler_update(false)` | ✗ F5 |
| `microsim/ms_base_vehicle.rs:1503, 1558, 1685` | "`gCheckRoutes` not ported, treated as false" | false | `check_routes()` (**true** by default) | ✗ F6 |
| `libsumo/vehicle.rs:3333` | `if false /* gCheckRoutes not ported */` | false | `check_routes()` | ✗ F6 |
| `microsim/command_route_replacement.rs:94` | reads source option directly | equal | `check_routes()` | ✓ equal |
| `microsim/ms_vehicle.rs:5026-5027` | "`gWaitingTimeMemory` not ported, 300 s" | 300 000 ms | `waiting_time_memory()` (option default **100 s**, `MSFrame.cpp:483`) | ✗ F7 |
| `microsim/ms_vehicle.rs:3157` | "`gModelParkingManoeuver` not ported, always false" | false | `model_parking_manoeuver()` | ✗ F4 |
| `microsim/ms_vehicle.rs:375` | "`gUseMesoSim` not ported (always false)" | false | `use_meso_sim()` | ✓ equal |
| `microsim/ms_base_vehicle.rs:358` | "`gHaveEmissions` not ported" | never set | `have_emissions()` (set by `full-output`) | ✗ F8 |
| `microsim/output/ms_detector_control.rs:149` | "`gHaveEmissions |=` needs RTTI" | never set | set when emission detector exists | ✗ F8 |
| `microsim/ms_base_vehicle.rs:1375` | "`gUseMesoSim` not ported" | micro | `use_meso_sim()` | ✓ equal |
| `microsim/ms_base_vehicle.rs:1545` | "`gWeightsRandomFactor` not ported, assumed 1" | 1 | `weights.random-factor` | U |
| `microsim/ms_lane.rs:771` | "`gUsingInternalLanes` not ported, treated as false" (`is_normal() \|\| false`) | false | `using_internal_lanes()` (default true). C++ site U | ✗ F3b |
| `microsim/ms_lane_changer.rs:1212` | "`gLefthand` not ported; right-hand assumed" | false | `lefthand()` | ✗ F11 |
| `microsim/ms_lane_changer.rs:2297` | "`gLaneChangeDuration` not ported, defaults to 0" | 0 | `lane_change_duration()` | ✗ F11 (wrong if option set) |
| `microsim/ms_lane_changer.rs:2973` | "`gSublane` not ported, defaults false" | false | `sublane()` | ✗ F11 |
| `microsim/ms_lane_changer.rs:3456` | "`gLaneChangeDuration` and `gLateralResolution` default 0" | 0 | `lane_change_duration()`; `lateral_resolution()` (default **-1**, not 0) | ✗ F11 |
| `microsim/devices/ms_device_routing.rs:367` | "`gWeightsSeparateTurns` not ported, treated as 0" | 0 | `weights_separate_turns()` | ✗ F14 |
| `libsumo/induction_loop.rs:25,148` | "`gUseMesoSim` not ported" | micro | `use_meso_sim()` | ✓ equal |
| `microsim/traffic_lights/ms_delay_based_traffic_light_logic.rs:255` | "`gUseMesoSim` not ported" | micro | `use_meso_sim()` | ✓ equal |
| `traci_server/tra_ci_server.rs:1913` | warning skipped (`gUsingInternalLanes`/`gUseMesoSim` "not ported") | — | warning | ✗ low |
| `microsim/ms_net.rs:3060, 3102` | `if !v.is_null() && false /* per-vehicle weights not ported */` | false | `v.getWeightsStorage()` path | ✗ F27 (U) |
| `microsim/ms_net.rs:452` | `myVersion: ()` placeholder | unit | `MMVersion` | ✗ F10 |
| `microsim/ms_net.rs:868` | `VERSION_STRING not ported`, reported as 'unknown' | "unknown" | `VERSION_STRING` | ✗ F10 |
| `microsim/ms_frame.rs:2250-2255` | `gTaxiClasses` truncated to i32 | low bits | C++ type U | U (F15) |

Stale comments that are now wrong and should be deleted when the fix lands: `ms_edge.rs:10-12`, `cfmodels/mscf_model.rs:32-33`, `output/ms_mean_data.rs:7-9`, `ms_globals.rs` test comment, `libsumo/induction_loop.rs:25,148`.

---

## 3. Event scheduling audit (class 3)

### 3a. `MSNet::simulationStep` order (C++ `MSNet.cpp:808-954`) vs `ms_net.rs:1053-1170`

Order matches exactly: begin-of-timestep events → railSignal `updateSignals` → (STAGE_EVENTS collisions) → `check2Switch` → patchActiveLanes → planMovements → setJunctionApproaches → executeMovements → (STAGE_MOVEMENTS) → changeLanes → (STAGE_LANECHANGE) → `removePending` → `loadRoutes` → person/container `checkWaiting` → railSignal `resetWaitRelations` → `determineCandidates` → **insertion events** → `emitVehicles` → (STAGE_INSERTIONS) → `checkInsertions` → **end-of-timestep events** → postMoveStep. **V**

Differences (all already listed as shortcuts in Rust):
- `MSRoutingEngine::waitForAll()` (HAVE_FOX) not called: `ms_net.rs:1101`. Single-threaded, so harmless **G**.
- `processPendingRouting()` for persons/containers (HAVE_FOX, C++ 915-921) not called: `ms_net.rs:1142`. F26.
- TraCI load arguments early return (C++ 834-836) not honored: `ms_net.rs:1069`. F25.
- Meso branch panics (`ms_net.rs:1109`). Loud, intentional.
- `MSTriggeredRerouter` permissions applied after begin events via `apply_scheduled_permissions` (`ms_net.rs:1097`), not as scheduled events. F23.

### 3b. Event producers. C++ `addEvent` sites vs Rust

| C++ producer (`file:line`) | Queue | Rust status |
|---|---|---|
| `MSVehicleControl.cpp:196` DeleteKeptVehicle, `SIMSTEP + keep` | end | ✓ `ms_vehicle_control.rs:386`, `execute` matches C++ 615-617 **V** |
| `MSBaseVehicle.cpp:683` `activateRemindersOnReroute`, `SIMSTEP` | begin | ✗ never scheduled. Function exists (`ms_base_vehicle.rs:5530`), no caller. F19 **G** |
| `MSDevice_Routing.cpp:195` `myRerouteCommand`, `execTime` | insertion | ✗ file is a skeleton. F20 **G** |
| `MSDevice_Routing.cpp:259` `myRerouteCommand`, `start` | begin | ✗ F20 **G** |
| `MSDevice_Routing.cpp:432` `myRerouteCommand`, `lastRouting + preInsertionPeriod` | insertion | ✗ F20 **G** |
| `MSTransportableDevice_Routing.cpp:81, 137` | insertion | ✗ not ported. F21 **G** |
| `MSDevice_StationFinder.cpp:277, 740` | begin | ✗ not ported. F21 **G** |
| `MSDevice_ToC.cpp:499, 506, 511, 567, 633` | begin | ✗ not ported. F21 **G** |
| `MSStageDriving.cpp:103` `BookReservation` | begin | ✗ no call found **G** U |
| `MSTransportable.cpp:309` `myAbortCommand`, timeout | begin | ✗ no call found **G** U |
| `MSDevice_Taxi.cpp:182` dispatch | end | ✓ `ms_device_taxi.rs:1822` |
| `MSDevice_FCDReplay.cpp:118, 120` | begin | ✓ `ms_device_fcd_replay.rs:851, 854` |
| `MSDevice_BTreceiver.cpp:122` | end | ✓ `ms_device_b_treceiver.rs:118` |
| `MSRoutingEngine.cpp:115` | end | ✓ `ms_routing_engine.rs:160` |
| `MSParkingArea.cpp:359, 385, 394, 403` `myUpdateEvent` | end | ◐ one site only, `ms_parking_area.rs:42` U |
| `MSChargingStation.cpp:162, 172, 186` `myTotalPowerCheckEvent` | end | ◐ one site only, `ms_charging_station.rs:82` U |
| `MSRouteHandler.cpp:719, 721` | end / begin | ✓ `ms_route_handler.rs:1749, 1751` |
| `MSCalibrator.cpp:163` | end | ✓ `ms_calibrator.rs:1240` |
| `MSTriggeredRerouter.cpp:369, 399` `setPermissions` | begin | ✗ replaced by `apply_scheduled_permissions`. F23 |
| `MSLaneSpeedTrigger.cpp:91, 96` | begin | ✓ `ms_lane_speed_trigger.rs:559, 564` |
| `MSTLLogicControl.cpp:761` | begin | ✓ `mstl_logic_control.rs:454` |
| `MSTrafficLightLogic.cpp:131` | begin | ✓ `ms_traffic_light_logic.rs:207` |
| `MSActuatedTrafficLightLogic.cpp:795, 810` | begin | ✓ `ms_actuated_traffic_light_logic.rs:1309, 1623` |
| `MSSimpleTrafficLightLogic.cpp:67, 307` | begin | ✓ `ms_simple_traffic_light_logic.rs:169, 470` (307 mapping U) |
| `MSPerson.cpp:83, 160` `ProceedCmd` | begin | ✓ `ms_person.rs:694, 839` |
| `MSPModel_NonInteracting.cpp:63, 73` | begin | ✓ `msp_model_non_interacting.rs:215, 241` |
| `MSPModel_Striping.cpp:169, 195` `MovePedestrians` | begin | ◐ one site, `msp_model_striping.rs:351` U |
| `MSVTypeProbe.cpp:45` | end | ✓ `msv_type_probe.rs:65` |
| `Command_SaveTLSProgram.cpp:37`, `Command_SaveTLSState.cpp:41`, `Command_SaveTLSSwitches.cpp:41`, `Command_SaveTLSSwitchStates.cpp:38`, `Command_SaveTLCoupledDet` | end | ✗ self-registration skipped (shortcut in each file). F24 |
| `MSOverheadWire.cpp:670` `addSolvingCircuitToEndOfTimestepEvents` | end | ◐ `ms_overhead_wire.rs:1191` exists; caller in `ms_device_elec_hybrid.rs:839` U |
| `MSEventControl.cpp` `execute` / heap / `clearState` | — | ✓ libstdc++ heap port, `ms_event_control.rs:26-87`, `execute` and `clear_state` match. No defect **V** |
| `MSVehicleTransfer.cpp checkInsertions` | — | ◐ parking-branch flags wrong (F4). Sort is `std::sort` vs stable `sort_by` (determinism comment present) |
| `MSInsertionControl.cpp` | — | no `addEvent` in C++. The `static_cast<MSDevice_Routing>` at `ms_insertion_control.rs:954` is a shortcut (F21) |

---

## 4. Findings (ranked)

Rank 1 = highest. "Tag" = V/G/U as defined above.

| # | file:line (Rust) | Class | C++ ref | What Rust does | Impact | Exact fix | Tag |
|---|---|---|---|---|---|---|---|
| F1 | `cfmodels/mscf_model.rs:34-42` (used at ~25 sites: mscf_model 341, 589-2052; krauss, krauss_x, krauss_orig1, w99, rail; ms_mean_data import) | 2 | `MSCFModel.cpp:88..1031`, `MSCFModel_Krauss.cpp:145,156`, `MSGlobals.cpp:63,69` | Stubs return `true` / `false` / `-1` regardless of options | `step-method.ballistic` ignored by every car-following model; lane-change branches (`gComputeLC`) wrong during changeLanes; `default.emergencydecel=decel` or float ignored. Probably changes every run that sets those options | Replace the three fn bodies with `crate::microsim::ms_globals::{semi_implicit_euler_update, compute_lc, default_emergency_decel}()`, or delete them and import `MSGlobals` at call sites. Note `ms_vehicle.rs:30` already reads the real flag, so the two disagree today | V |
| F2 | `microsim/ms_edge.rs:13-19` (used 485-541, 668, 674, 703, 2665, 3355) | 2 | `MSEdge.cpp:163-178, 265, 281, 1009, 1417`; defaults `MSFrame.cpp:684,690` | Consts: internal lanes `true` (ignores `no-internal-links`), `G_MINOR_PENALTY=0`, `G_TURNAROUND_PENALTY=0` | Default run uses penalty **1.5 s** (minor) and **5.0 s** (turnaround) in C++ but **0** here. Internal-lane travel times and route costs differ in every run with internal lanes. `no-internal-links` ignored | Delete the 7 consts. Use `MSGlobals::using_internal_lanes()`, `minor_penalty()`, `turnaround_penalty()`, `tls_penalty()`, `lane_change_duration()`, `stop_tolerance()`, `use_meso_sim()` | V |
| F3 | `microsim/ms_lane.rs:401-403` (used 1068, 1117, 3515, 5321-5328) | 2 | `MSLane.cpp` sublane paths; `MSGlobals.cpp` `gLateralResolution` | Returns `-1` always | With `lateral-resolution` set, sublane offsets and lane-side math use -1 (disabled) instead of the option. Sublane runs are wrong | Delete `g_lateral_resolution`, call `MSGlobals::lateral_resolution()` (use `crate::microsim::ms_globals::lateral_resolution()`) | V |
| F3b | `microsim/ms_lane.rs:771-772` | 2 | `MSLane.cpp:352` `isNormal() \|\| gUsingInternalLanes` (C++ line for this warning U) | `self.is_normal() \|\| false` | Bidi-length warning suppressed for internal lanes | Replace `false` with `crate::microsim::ms_globals::using_internal_lanes()` after confirming the C++ line | U |
| F4 | `microsim/ms_vehicle_transfer.rs:292-293, 304-305` (also `ms_vehicle.rs:3157`) | 2 | `MSVehicleTransfer.cpp:140` (`gModelParkingManoeuver`), `:150` (`gLefthand`) | Both hardcoded `false` | `parking.maneuver=true` ignored (no exit manoeuvre). Lefthand networks signal the wrong blinker when a parked vehicle re-enters | Replace with `crate::microsim::ms_globals::model_parking_manoeuver()` and `crate::microsim::ms_globals::lefthand()`. Fix `ms_vehicle.rs:3157` the same way | V |
| F5 | `microsim/ms_vehicle_type.rs:1566-1580` | 2 | `MSVehicleType.cpp:468-481` | Warns "Setting it now to avoid collisions" but never clears the flag (shortcut at 1574) | Action-step-length vtypes stay semi-implicit. C++ switches to ballistic, which is the collision-avoidance mode | In the `is_default("step-method.ballistic")` branch add `crate::microsim::ms_globals::set_semi_implicit_euler_update(false);`. Only effective after F1 | V |
| F6 | `microsim/ms_base_vehicle.rs:1503, 1558, 1685`; `libsumo/vehicle.rs:3333` | 2 | `MSBaseVehicle.cpp:468, 534, 598, 709, 1011, 1020`; `Command_RouteReplacement.cpp:53`; `MSRouteHandler.cpp:448-1392` | `gCheckRoutes` treated as `false` (C++ default `true`) | In default runs, route-error handling takes the "ignore" branch: invalid stops and routes are kept or the error path is skipped. TraCI `setRoute` error path also skipped | Replace each `false` with `crate::microsim::ms_globals::check_routes()`. In `vehicle.rs:3333` use `if check_routes() { panic!(...) }` | V |
| F7 | `microsim/ms_vehicle.rs:5026-5027` | 2 | `MSVehicle.cpp` `getWaitingTime(accumulated)`; `MSFrame.cpp:483` (default "100") | Hardcodes 300 000 ms | Accumulated waiting time uses 300 s window instead of the option's 100 s. Already listed in HANDOVER class 2. Affects accumulated waiting-time outputs | Replace with `self.myWaitingTimeCollector.cumulated_waiting_time(crate::microsim::ms_globals::waiting_time_memory())` | V (C++ caller U) |
| F8 | `microsim/ms_base_vehicle.rs:358`; `output/ms_detector_control.rs:149`; `ms_net.rs` (`write_output`, C++ 1234-1238) | 2 | `MSNet.cpp:1235-1236`; `MSBaseVehicle.cpp:180` | `gHaveEmissions` never set anywhere in Rust | Emission parameters are not forced on vehicles, and emission output is missing for `full-output` / emission detectors | In `write_output`, when `full-output` is set, call `MSGlobals::set_have_emissions(true)`. In `ms_base_vehicle.rs:358` read `have_emissions()`. Add the detector case once RTTI is replaced by a type tag | V |
| F9 | `ms_net.rs:2791-2792` (`load_state` panics); `ms_insertion_control.rs:497` | 2 | `MSNet.cpp:1876`; `MSInsertionControl.cpp:296` | `gStateLoaded` is set only from the option, never by loadState | If loadState is later ported, `MSInsertionControl` takes the wrong branch (throws instead of break). Low now because loadState panics | When porting loadState, add `MSGlobals::set_state_loaded(true);` after the load | V |
| F10 | `ms_net.rs:452, 868` | 2 | `MSNet.cpp` `VERSION_STRING` use; `MSNet` `myVersion` (`MMVersion`) | `myVersion: ()`; version reported as 'unknown' | Output headers and network-version checks differ from C++ (HANDOVER: version string lacks `v…-0000000000`) | Port `VERSION_STRING` and `MMVersion`, then set `myVersion` in `close_building`, and read `get_network_version` from it | V |
| F11 | `microsim/ms_lane_changer.rs:1212, 2297, 2973, 3456` | 2 | `MSLaneChanger.cpp:1369` (`gLefthand`); `MSLaneChangerSublane`; `MSLCM*` use `gLaneChangeDuration`, `gSublane`, `gLateralResolution` | `lefthand` = false; duration and sublane = 0/false; lateral term = 0 (C++ default -1) | Lefthand networks change lanes on the wrong side. `lanechange.duration` and `lateral-resolution` ignored in lane-change | Replace with `lefthand()`, `lane_change_duration()`, `sublane()`, `lateral_resolution()` | V (the shortcut comments) |
| F12 | `microsim/output/ms_mean_data.rs:10-15` | 2 | `MSMeanData.cpp` (`gUsingInternalLanes` use U) | `g_using_internal_lanes()` returns true; `g_use_meso_sim()` returns false | `no-internal-links` runs still count internal lanes in mean data. Affects edge/lane data output | Replace both fns with the MSGlobals readers | U |
| F13 | `microsim/ms_lane.rs:409-416` | 2 | `MSGlobals::gEmergencyInsert`, `gCheckRoutes` | Reads options directly | Equal values, but a second source of truth. Breaks if a future writer changes `MSGlobals` | Replace with `MSGlobals::emergency_insert()`, `MSGlobals::check_routes()` | V |
| F14 | `microsim/devices/ms_device_routing.rs:367` | 2 | `MSDevice_Routing.cpp` junction travel-time recording with `gWeightsSeparateTurns` | Treated as 0, so the recording is skipped | Junction travel-time records missing when `weights.separate-turns > 0` | Replace the 0 with `MSGlobals::weights_separate_turns()` and remove the skip | G |
| F15 | `microsim/ms_frame.rs:2250-2255` | 2 | `gTaxiClasses` type U (`ms_device_taxi.rs` comment says `int`, the ms_frame comment says `SVCPermissions`) | i64 truncated to i32 | Only matters if the vehicle class mask exceeds 32 bits | Check the C++ declaration in `StdDefs.h`, then widen `std_defs` global to i64 if needed | U |
| F16 | `microsim/ms_vehicle_control.rs:477-486` (`remove_pending`) | 1 | `MSVehicleControl.cpp:168-171` (`dev->generateOutput`, virtual) | Only `tripinfo` dispatches to its subclass; every other device calls base `MSDevice::generate_output` (empty body, `ms_device.rs:308`) | At arrival, `generateOutput` is never run for vehroutes, emissions, battery, elecHybrid, taxi, SSM, GLOSA, bluelight, station finder. Their tripinfo attributes and per-vehicle outputs are silently missing | Add a name-keyed dispatch in one place (e.g. `MSDevice::generate_output` matching `get_device_name(id)` and casting), then call it from `remove_pending` and from `ms_device_tripinfo.rs:344` | V |
| F17 | `microsim/devices/ms_device_tripinfo.rs:337-344` | 1 | `MSDevice_Tripinfo.cpp` pending-output loop, `typeid`/`name` check | `dev.base2.device_name()` on the base struct is pure virtual and **panics** (`ms_device.rs:258-260`). Reachable when the loop runs with tripinfo output set and the holder has any device (tripinfo itself is one) | Potential panic at tripinfo write time. Reachability U: depends on whether `pendingOutput` is non-empty at write time in real runs | Replace `dev.base2.device_name()` with `MSDevice::get_device_name(dev.base2.get_id())`, the same as `ms_vehicle_control.rs:479` | V (code), U (reachability) |
| F18 | `microsim/ms_net.rs:2870-2873` (`inform_vehicle_state_listener`) | 1 | `MSNet.cpp:1402-1410`; `VehicleStateListener` virtual | Identity-matched overrides only. "other overrides still hit the base no-op" (own comment) | TraCI listeners other than Helper/TraCIServer get no callbacks (e.g. `getLoadedNumber`, HANDOVER class 4) | Make listeners `Ptr<dyn MSNet_VehicleStateListener>` and call trait methods | V |
| F19 | `microsim/ms_base_vehicle.rs:1777` (commented), `:5530` (uncalled) | 3 | `MSBaseVehicle.cpp:683-684` | `replaceRoute` does not schedule `activateRemindersOnReroute(SIMSTEP)` on begin-of-timestep | After TraCI/rerouter route replacement, move reminders (rerouters, station finder) are not re-activated for the new route | In `replace_route` (non-stopsFromScratch path), add a begin-of-timestep `Command` wrapping `activate_reminders_on_reroute` at `SIMSTEP` | G |
| F20 | `microsim/devices/ms_device_routing.rs` (whole file), `ms_insertion_control.rs:954` | 3 | `MSDevice_Routing.cpp:195, 259, 432` | No event scheduled; the insertion-time static_cast is a shortcut | Periodic and pre-insertion reroutes never fire. Matches HANDOVER's `reroute/ref_device` one-step offset and the "periodic device reroute" branch `worktree-agent-pr01` | Port the three `addEvent` calls with a `Command` wrapper and wire `ms_insertion_control.rs:954` | G |
| F21 | `microsim/devices/ms_transportable_device_routing.rs`, `ms_stage_driving.rs`, `ms_transportable.rs`, `ms_device_station_finder.rs`, `ms_device_to_c.rs` | 3 | `MSTransportableDevice_Routing.cpp:81, 137`; `MSStageDriving.cpp:103`; `MSTransportable.cpp:309`; `MSDevice_StationFinder.cpp:277, 740`; `MSDevice_ToC.cpp:499-633` | Not scheduled (files unported or skeleton) | Charging rescue, ToC/MRM, ride reservations and transportable timeouts never fire | Port each file's `addEvent` calls with the matching `Command` | G (U for transportable and stage-driving) |
| F22 | `microsim/ms_net.rs:1749-1762`, `:2179`, `:2201` | 3 | `MSNet.cpp:1164-1236` (`writeOutput`), `:765-807` (`closeSimulation`) | netstate, emission, battery, chargingstations, elechybrid, full, queue, amitran, vtk, SSM, ToC outputs skipped; `MSQueueExport::finish` and `MSStopOut::generateOutputForUnfinished` skipped | Output files for those options are **silently** missing (no error) | Port the exports, or make the option produce an error instead of nothing | G |
| F23 | `microsim/trigger/ms_triggered_rerouter.rs` (`apply_scheduled_permissions`), `ms_net.rs:1097` | 3 | `MSTriggeredRerouter.cpp:369, 399` | Permission changes applied after the begin-event queue drains, not at their scheduled slot | Within a step, begin events that read permissions (tls switches, device begin events) can see the wrong permissions. Ordering-dependent | Replace with real `WrappingCommand`-style begin events; retire the workaround | U |
| F24 | `actions/command_save_tls_program.rs:46`, `command_save_tls_state.rs:45`, `command_save_tls_switches.rs:42`, `command_save_tls_switch_states.rs:44`, `command_save_tl_coupled_det.rs:45` | 3 | `Command_SaveTLS*.cpp:37-41` (`getEndOfTimestepEvents()->addEvent(this)`) | Self-registration skipped (shortcut in each) | TLS state/switch/program output files never written | Register the boxed command with `end_of_timestep_events` as the constructor does in C++ | G |
| F25 | `microsim/ms_net.rs:1937-1940` (`clear_state`) and the `load_state`/TraCI load path (`ms_net.rs:1069, 1792, 1821`) | 3 | `MSNet.cpp:834-836`, `:1094-1148` | TraCI load-args early return and SIMSTATE_LOADING never reported | TraCI load requests do not end the step; load state never reported | Port `TraCIServer::getLoadArgs` and the early return | G |
| F26 | `ms_net.rs:1142` | 3 | `MSNet.cpp:915-921` (`processPendingRouting`, HAVE_FOX) | Not called | Persons/containers whose routing is deferred are never routed (if the pending queue is populated) | Call `processPendingRouting` on both controls at the same point | U |
| F27 | `ms_net.rs:3060, 3102` | 2 | `MSNet.cpp:155-180` (`getTravelTime`/`getEffort` use `v->getWeightsStorage()`) | `&& false` shortcut | Per-vehicle weight storage ignored in routing costs | Remove the `&& false` and port `getWeightsStorage` | U |
| F28 | `microsim/ms_edge_control.rs:102, 241, 350, 469` | 3 | `MSEdgeControl.cpp` threaded planning/executing | Serial execution | Results should match C++ (determinism) but thread-ordering effects are not tested | No change needed for correctness; note in HANDOVER | G |
| F29 | `microsim/ms_edge_control.rs:605` | 3 | `MSEdgeControl::buildSegments` (meso) | Meso segments not built | Only matters with meso (already refused) | None until meso is wired | G |
| F30 | `ms_net.rs:614` | 2 | `MSNet.cpp` `weights.separate-turns` check | Panics when option set without internal links | C++ behaviour U | Compare exact C++ error path, then use `ProcessError` | U |

**Class 4 (legal nullptr deref):** no new unguarded null deref found in the scope files. `ms_vehicle_transfer.rs:288` (`lane.m()` after `get_mutable_lane()`) mirrors the unguarded C++ deref. `ms_vehicle_control.rs:482` casts only after a name check. Not exhaustively checked.

**Class 1 (virtual dispatch):** `MSEventControl` `Command::execute` and `shift_time` dispatch correctly through `dyn Command`. The gaps are F16, F17, F18.

---

## 5. Notes for the lead

- F1 is a precondition for F5 to have any effect, and it is the highest-leverage fix in the crate (about 25 call sites).
- Fixing F2, F3, F4, F6, F7 is mechanical: replace a literal with a `MSGlobals::*()` call. Each change is one line plus deleting a stub.
- Stale comments claim `MSGlobals` is "not ported" in 10+ files. They are wrong. Grep `not ported` near `MSGlobals` before trusting them.
- Nothing in this audit was compiled or run. Tags U and G mean the claim should be confirmed by a test run before fixing.

Commit: see git log for `AUDIT au10`.
