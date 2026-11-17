# Mesoscopic (`--mesosim true`) port plan

Scoping pass, 2026-11-16, branch `worktree-agent-ms01` (base `d576c3e`). No code changed, no cargo run.
Counts are from grep and line counts, not from running the code.

Ground truth today: `--mesosim true` exits 1 at `crates/sumo_core/src/microsim/ms_net.rs:402-404`
("mesoscopic simulation is not wired into MSNet yet"). Nothing in the Rust meso files is reachable from a run.

## 1. Inventory: `crates/sumo_core/src/mesosim/`

Rust `todo!()`: 0 in every file. Rust `unimplemented!`: 0. Remaining markers are `// shortcut:` comments and deliberate panics.
C++ counts are `.h` + `.cpp` lines in `C:\Users\karti\sumo-src\src\mesosim\` (CMakeLists excluded).

| Rust file | exists | Rust lines | C++ source | C++ lines | live `shortcut:` | deliberate `panic!` | notes |
|---|---|---|---|---|---|---|---|
| `me_segment.rs` | yes | 2509 | MESegment.h/.cpp | 1709 | 4 | 2 (C++ private copy/assign) | DEBUG trace omitted (1011); `myVaporizationTarget` (1292, 1617) and `myDummyParent` (2260) statics not ported |
| `mel_segment.rs` | yes | 219 | MELSegment.h/.cpp | 249 | 0 | 3 (215 is a Rust-only dummy ctor with no C++ counterpart: check if used) | |
| `me_vehicle.rs` | yes | 1698 | MEVehicle.h/.cpp | 1168 | 13 | 1 (1605, C++ throws the same invalid-state error) | `getDevice(typeid)` lookups return null for taxi and tripinfo (537, 665, 786, 788); `MSStop.edge` is an Unmapped route iterator (640, 746, 841, 1648); `loadState` skips `addStops` (1634); traceMoveReminder is DEBUG-only (1273, 1338, 1351, 1355) |
| `me_vehicle_control.rs` | yes | 86 | MEVehicleControl.h/.cpp | 136 | 0 | 2 (C++ private copy/assign) | `buildVehicle` body present |
| `me_loop.rs` | yes | 888 | MELoop.h/.cpp | 628 | 1 (309, benign: C++ also never frees segments) | 2 (C++ private) | event queue ordering is determinism-critical (see WP3) |
| `me_induct_loop.rs` | yes | 158 | MEInductLoop.h/.cpp | 197 | 0 | 3 (C++ private ctors) | |
| `me_triggered_calibrator.rs` | yes | 503 | METriggeredCalibrator.h/.cpp | 360 | 1 (316, `MSRoute::dictionary` returns no Option) | 0 | |
| `me_type_handler.rs` | yes | 204 | METypeHandler.h/.cpp | 216 | 0 | 2 (C++ private) | |
| `mod.rs` | yes | 8 | n/a | n/a | n/a | n/a | all 8 modules declared |
| **total** | | **6273** | | **4663** | **19** | | |

Confidence: the bodies are present and the last commits ("Port me_triggered_calibrator", "Fix compile errors after
merging mesosim") suggest they compiled. No oracle has ever run them, because `MSNet` refuses meso. Treat them as
ported but unexecuted. Expect bugs on first run.

Stale scoreboard claims, corrected here:
- "eight `g_use_meso_sim() -> false` stubs": the current `ms_link.rs:88` and `ms_mean_data.rs:8` delegate to
  `MSGlobals::use_meso_sim()`, which is set from the option at `ms_frame.rs:2196` (C++ `MSFrame.cpp:1210`).
  `ms_mean_data.rs:28` is only a string literal in the file. The stubs are gone.
- The meso refusal itself is still correct and still true.

## 2. Microsim sites that branch on `use_meso_sim()` / `gMesoNet`

Status key: **P** = branch present, no shortcut marker at the site (body not read line by line in this pass).
**S** = meso branch missing or stubbed (`shortcut:`). **X** = panics when meso is on. **C** = only as a commented C++ block.

Core and refusal:

| File:line | Status | Meso behaviour (C++ reference) |
|---|---|---|
| `ms_net.rs:402-404` | **X** | refusal, the blocker. C++ `MSNet.cpp:266` creates `gMesoNet = new MELoop(meso-recheck)` |
| `ms_net.rs:1108-1109` | **X** | simulate step. C++ `MSNet.cpp:974`: `gMesoNet->simulate(myStep)`; also flush arrived meso vehicles (C++ ~999, comment only in Rust) |
| `ms_net.rs:1938-1939` | **X** | clearState. C++ `MSNet.cpp:1881-1884`: `gMesoNet->clearState()` and segment reset |
| `ms_net.rs:328, 1102, 1151, 3612` | P | gating (check4Accidents, `!use_meso_sim`) |
| `ms_net.rs:389, 535` | C | `MELoop` create and delete commented out |
| `ms_edge_control.rs:599-605` | **S** | `buildSegmentsFor` not called (C++ `MSEdgeControl.cpp:383`) |
| `ms_globals.rs:149` | P | flag. `ms_globals.rs:155` has the `gMesoNet` slot; check the getter/setter before WP4 |
| `ms_frame.rs:2196` | P | option -> flag |

Edges and lanes:

| File:line | Status | Meso behaviour |
|---|---|---|
| `ms_edge.rs:474-477` | P | TLS-penalty branch |
| `ms_edge.rs:855, 886, 914, 936, 958, 993` | **S** | six segment walks (vehicle lists, waiting seconds, length sums) |
| `ms_edge.rs:2021, 2045` | P | `MEVehicle` cast on insertion |
| `ms_edge.rs:2913, 3410, 4008` | **S** | updatePermissions, mean speed, meso positions |
| `ms_edge.rs:839-4008` (C) | C | ~9 commented C++ segment loops |
| `ms_lane.rs:4197` | **S** | segment speed update |
| `ms_link.rs:88-89, 1375, 1790, 1838` | P | |

Vehicles, stops, types:

| File:line | Status | Meso behaviour |
|---|---|---|
| `ms_base_vehicle.rs:1379` | **S** | meso stop-position branch skipped |
| `ms_base_vehicle.rs:1700, 2191, 2232, 3006, 3833, 4236, 5735` | P | |
| `ms_base_vehicle.rs:3670` | **S** | parking-area and meso segment lookup in stop checks |
| `ms_vehicle.rs:375` | P | |
| `ms_vehicle_control.rs:577` | P | |
| `ms_vehicle_type.rs:1582, 1588` | P | |
| `ms_stop.rs:549` (C), `588` | P | |

Devices:

| File | Status | Count |
|---|---|---|
| `devices/ms_device_tripinfo.rs` (1137-2036) | P | 9 sites |
| `devices/ms_device_vehroutes.rs` | P | 5 sites |
| `devices/ms_idling.rs` (320-533) | P, C at 239-252 | 14 sites |
| `devices/ms_device_taxi.rs` | P | 3 sites |
| `devices/ms_device_transportable.rs` | P | 4 sites |
| `devices/ms_device_fcd.rs:598`, `ms_device_glosa.rs:139`, `ms_device_ssm.rs:941`, `ms_device_bluelight.rs:88`, `ms_device_to_c.rs:275`, `ms_device_elec_hybrid.rs` | P | 1 each |
| `devices/ms_routing_engine.rs` | P | 2 |

Outputs and state:

| File:line | Status | Meso behaviour |
|---|---|---|
| `output/ms_induct_loop.rs:167, 176, 310-311, 495-496, 802-803` | **S** | MESegment lookup, `addDetector`, interpolated and extrapolated leave times; the `mesoLeaveTimes` cleanup uses `&self` and is not right |
| `output/ms_mean_data.rs:323, 791, 933, 1043` | **S** | meso branch of the lane/edge mean-data paths |
| `output/ms_mean_data.rs:1855` | P | |
| `output/ms_mean_data_net.rs:280, 299, 737, 752` | P | |
| `output/ms_route_probe.rs:72-73` | **S** | needs `getSegmentForEdge(edge)` without a position |
| `output/msfcd_export.rs:318-435` | P | ~11 sites |
| `output/msxml_raw_out.rs:86, 183, 208` | P | |
| `output/ms_queue_export.rs:187` | P | |
| `output/ms_stop_out.rs:351`, `output/mse3_collector.rs:483` | P | |
| `ms_state_handler.rs:222-261, 856-860` | P | |
| `ms_state_handler.rs:262-263` | **X** | save-state for meso panics |
| `ms_state_handler.rs:665-667` | **X** | bike-speed routing init and meso cached speeds panic |
| `ms_state_handler.rs:704-705` | **X** | meso segment state panics |

Traffic lights, triggers, libsumo:

| File:line | Status | Meso behaviour |
|---|---|---|
| `traffic_lights/ms_traffic_light_logic.rs:733` | **S** | `meso.tls.control` (`OVERRIDE_TLS_PENALTIES`) is read from logic params, which are not stored yet |
| `traffic_lights/ms_traffic_light_logic.rs:760` | **S** | `overrideTLSPenalty` needs `gMesoNet` |
| `traffic_lights/ms_drive_way.rs` (8 sites) | P | |
| `traffic_lights/ms_rail_signal_control.rs:459`, `ms_delay_based_traffic_light_logic.rs:274` | P | |
| `trigger/ms_triggered_rerouter.rs:168, 185` | **S** | the meso segment registration of rerouters |
| `trigger/ms_calibrator.rs:1110` | P | |
| `microsim/transportables/ms_stage_trip.rs:576` | P | |
| `libsumo/vehicle.rs:69-70, 287, 300, 639` | **S** | MEVehicle cannot be told apart from MSVehicle (no type tag on MSBaseVehicle) |
| `libsumo/induction_loop.rs` (10 sites) | P | |

Netload (builders):

| File:line | Status | Meso behaviour |
|---|---|---|
| `netload/nl_builder.rs:416` | P | option check |
| `netload/nl_builder.rs:686-687` | P | creates `MEVehicleControl` as the `MSVehicleControl` |
| `netload/nl_builder.rs:502, 524, 978` | P | |
| `netload/nl_detector_builder.rs:831` | P | builds `MEInductLoop` |
| `netload/nl_trigger_builder.rs:1577, 1591` | P | calibrator and jam threshold |
| `netload/nl_trigger_builder.rs:1821-1845` | P | `build_me_calibrator`: the segment length (C++ 1834) is passed as `length`; check the `gMesoNet` lookup |
| `netload/nl_edge_control_builder.rs:304`, `nl_handler.rs:1970, 2535` | P | |

Totals (from grep, approximate): about 190 matching lines in `crates/sumo_core/src`, of which roughly 150 are active
call sites and roughly 45 are S, X or C. Every P row is a claim that the site has no marker, not that its body was
checked.

## 3. Wiring needed in MSNet and NLBuilder

Do these in one commit (WP4), together with removing the refusal panic. Order matters.

1. `ms_globals.rs:155`: add a getter and setter for the `gMesoNet` pointer slot. Store it as a leaked `Box<MELoop>`.
2. `ms_net.rs` `MSNet::new` (about line 388-404): replace the panic with
   `MELoop::new(string2time(oc.getString("meso-recheck")))`, boxed and stored in the slot. Do it before the
   `MSEdgeControl` is built (C++ `MSNet.cpp:266`). Confirm the order with the real construction sequence,
   because `buildSegmentsFor` (step 4) needs the loop to exist.
3. `ms_net.rs` simulate (about 1108): replace the panic with the call to `simulate(myStep)` at the C++ position
   (`MSNet.cpp:974`). Add the flush of arrived meso vehicles (C++ about 999, with the micro vehicles removed by collision).
4. `ms_edge_control.rs:605`: call `build_segments_for(edge, oc)` per edge, where C++ does it (`MSEdgeControl.cpp:383`).
   Check the enclosing function in C++ first, because it might be the constructor or the build path.
5. `ms_net.rs` clearState (about 1938): call `clear_state()` and reset the segment walk (C++ 1881-1884).
6. `ms_net.rs` destructor (about 535): delete the loop.
7. `nl_builder.rs:686`: `MEVehicleControl` is already chosen. Check that `MSNet::new` gets it and that the `meso-*`
   option table (`getMesoType`, `ms_net.rs:775`) has every option the C++ reads. Options not registered would fail
   silently or with a default.
8. Remove the three panics (402-404, 1108-1109, 1938-1939) and the `shortcut:` at 403 in the same commit.

## 4. Work packages

Dependencies: WP1 and WP2 start in parallel. WP3 needs WP1 and WP2. WP4 (wiring) needs WP3 to compile. WP5-WP10
edit disjoint files, so they can start once WP2 is done and are tested once WP4 is in. Each package has one owner and
a disjoint file list.

| WP | Scope | Files owned | C++ (lines) | Rust now | Size | Depends on | Oracle proving it |
|---|---|---|---|---|---|---|---|
| **WP1** | Core segment, type handler | `mesosim/me_segment.rs`, `mesosim/mel_segment.rs`, `mesosim/me_type_handler.rs` | MESegment, MELSegment, METypeHandler (2174) | 2932 | S-M: resolve 4 shortcuts (add `myVaporizationTarget`, `myDummyParent` as statics), decide on mel_segment 215, unit tests for `has_space_for`, `compute_headway`, `get_tau_jj` from C++ values | none | unit tests now; `real --mesosim` after WP4 |
| **WP2** | Meso vehicle and control | `mesosim/me_vehicle.rs`, `mesosim/me_vehicle_control.rs` | MEVehicle, MEVehicleControl (1304) | 1784 | M: 13 shortcuts. `MSStop.edge` as a `usize` route index per the porting rules; device-by-type lookup needs WP6; `addStops` in loadState needs the route iterator | WP6 for `getDevice` (or a local shortcut) | unit tests; junction `--mesosim` tripinfo (WP4) |
| **WP3** | Meso loop, induction loop, triggered calibrator | `mesosim/me_loop.rs`, `mesosim/me_induct_loop.rs`, `mesosim/me_triggered_calibrator.rs` | MELoop, MEInductLoop, METriggeredCalibrator (1185) | 1549 | M-L. The leader event queue is C++ `std::priority_queue`; tie order depends on the heap algorithm. Port the same heap ops and add `// determinism:`. Fix `316` (route lookup) | WP1, WP2 | junction and real `--mesosim` tripinfo + FCD (WP4) |
| **WP4** | Wiring: MSNet, globals, edge-control build, option table | `microsim/ms_net.rs`, `microsim/ms_globals.rs`, `microsim/ms_edge_control.rs` (599-605) | MSNet.cpp (266, 974, 1881, 1884), MSEdgeControl.cpp (383) | 0 meso live | S (~80-120 lines): section 3 | WP3 compiles | **junction `--mesosim true`**: tripinfo count matches C++ (5 entries) and per-vehicle values within `acceptance.py` tolerance |
| **WP5** | Edge and lane meso walks | `microsim/ms_edge.rs`, `microsim/ms_lane.rs` | MSEdge.cpp (21 flag references), MSLane.cpp (2) | 9+1 S | M: six segment walks in `ms_edge.rs`, plus `updatePermissions`, mean speed, meso positions, and `ms_lane.rs:4197` | WP4 (for test) | junction and real meso FCD (lane-less rows) |
| **WP6** | Base vehicle, stops, types, links | `microsim/ms_base_vehicle.rs`, `ms_vehicle.rs`, `ms_vehicle_control.rs`, `ms_vehicle_type.rs`, `ms_stop.rs`, `ms_link.rs` | MSBaseVehicle.cpp (8 refs), MSStop, MSLink, MSVehicleType, MSVehicle, MSVehicleControl | 2 S + device lookup | M: 1379 (stop position), 3670 (stop checks), add typed `get_device_of_type` for taxi and tripinfo | WP4 for test; provides a dependency for WP2 and WP7 | junction and stress with stops `--mesosim` |
| **WP7** | Devices | `microsim/devices/*.rs` (about 14 files) | MSDevice_*.cpp meso branches | P, about 50 sites | L: review all P branches against C++ bodies (they may be stubs); most are unreachable today | WP6 for device lookup | new case `cases/meso_devices`: vehroutes, tripinfo, taxi, idling with `--mesosim true` |
| **WP8** | Outputs and detectors | `microsim/output/*.rs` | MSInductLoop, MSMeanData(_Net), MSRouteProbe, MSFCDExport, MSXMLRawOut, MSQueueExport, MSStopOut, MSE3Collector | 13 S, rest P | M-L: induction-loop meso paths and `mesoLeaveTimes` ownership (`&self` problem), mean-data meso branches, route probe | WP3 (MEInductLoop), WP4 | real `--mesosim`: stat and FCD diff, induction-loop output |
| **WP9** | State save/load | `microsim/ms_state_handler.rs` | MSStateHandler.cpp (13 meso refs), MEVehicle/MESegment save and load | 3 X | M: remove the three panics, port meso save/load (`MESegment::save_state` exists) | WP1, WP2, WP3 | new case `cases/meso_state`: saveState then loadState round trip, tripinfo unchanged |
| **WP10** | Libsumo, TLS penalties, rerouter | `libsumo/vehicle.rs`, `libsumo/induction_loop.rs`, `microsim/traffic_lights/ms_traffic_light_logic.rs`, `microsim/trigger/ms_triggered_rerouter.rs` | MSTrafficLightLogic.cpp, MSTriggeredRerouter.cpp, libsumo | 7 S + 2 S | M: TLS penalty override read from logic params; rerouter meso registration (needs WP5 `getSegmentForEdge`); libsumo type detection once WP2 exposes a tag | WP5, WP2 | `signal --mesosim` (TLS penalty); `reroute` with `--mesosim` |

Total remaining size: roughly 2,500-3,500 changed lines plus about 800 lines of unit tests, based on the
S/M/L estimates above. This is a rough estimate. The unknown is how many P-marked bodies turn out to be stubs (WP7).

## 5. Oracle

C++ reference for `real` with meso was generated in WSL on 2026-11-16 into
`C:\Users\karti\AppData\Local\Temp\opencode\ms01\` (scratch, not committed):

```
SUMO_HOME=$HOME/sumo-src $HOME/sumo-src/bin/sumo --net-file ref_simple.net.xml --route-files routes.rou \
  --tripinfo-output ... --fcd-output ... --statistic-output ... --mesosim true --no-step-log true
```

Exit 0. Shape: `meso.tripinfo.xml` has 9 tripinfo entries (same as `ref.tripinfo.xml`), `meso.fcd.xml` has 47 timesteps
(same as `ref.fcd.xml`), `meso.stat.xml` is 1967 bytes. The C++ run confirms the output has the same shape as the
micro reference, so the comparison harness can reuse the existing tripinfo, FCD and statistics comparators.

To add to the harness (later, not in this pass): `cases/real/sim_meso.args` containing `--mesosim true`, and
`ref_meso.{tripinfo,fcd,stat}.xml` generated by `make_sim_refs.sh`. Same for junction.

Open checks before WP3 is trusted: FCD rows for meso vehicles have no lane (the comparator must treat missing lane
as equal), and the C++ `meso-*` defaults must match `ms_net.rs:775-794`.

## 6. Risks

- The meso Rust code has never run. Expect a first-run bug in nearly every package. Plan a fixer pass after WP4.
- `std::priority_queue` tie order (WP3) can change which vehicle leaves first. Measure, don't assume.
- `MSGlobals::gMesoNet` is read by many microsim files. Changing its type or lifetime affects all of them.
- The `mel_segment.rs:215` dummy constructor and the `ms_induct_loop.rs` `&self` cleanup are suspected wrong. Check before relying on them.
- No meso case exists on the scoreboard. Until WP4 lands, every meso change is measured only by unit tests.
