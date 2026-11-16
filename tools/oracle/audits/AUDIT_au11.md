# AUDIT au11: netload/ + microsim/ms_route_handler.rs (SAX dispatch, registration, virtual dispatch, globals, null)

Branch `worktree-agent-au11` from `main` (93058c0). Read-only: no `.rs` edited, cargo not run.
C++ read from `C:\Users\karti\sumo-src\src`. Rust read from this worktree.
Verification: every finding below is from reading both sides. None was executed. "unverified" marks what I could not confirm by reading.

## 1. Element to handled? table

Start = SAX start-element dispatch, End = end-element dispatch.

| Element (C++ handler) | C++ ref | Rust start | Rust end | Note |
|---|---|---|---|---|
| net (NLHandler) | NLHandler.cpp:94-96 | yes nl_handler.rs:639 | yes (NET junction graph) | set_lefthand via ms_global! macro, not a shadow |
| edge, lane, neigh | NLHandler.cpp:99-110 | yes | yes | |
| junction, tlLogic, phase, condition, assignment, function, request | NLHandler.cpp:111-143 | yes | yes | |
| connection, conflict | NLHandler.cpp:127-131 | yes | n/a | |
| WAUT, WAUT_SWITCH, WAUT_JUNCTION | NLHandler.cpp:139-147 | yes | WAUT end yes | |
| e1Detector, inductionLoop | NLHandler.cpp:148-151 | yes | yes | |
| instantInductionLoop | NLHandler.cpp:166-168, 1008-1040 | yes | yes | unconditional broken=true reproduced (nl_handler.rs:1181) |
| e2Detector, laneAreaDetector | NLHandler.cpp:152-155 | yes | yes | |
| e3Detector, entryExitDetector, detEntry, detExit | NLHandler.cpp:156-165 | yes | yes | |
| **vss (child `step`)** | NLHandler.cpp:169-171; NLTriggerBuilder.cpp:137-139 | dispatched | n/a | **children NOT routed** (registerParent missing), F2 |
| **calibrator (child `flow`)** | NLHandler.cpp:172-174; NLTriggerBuilder.cpp:691-698 | dispatched | n/a | **children NOT routed**, F2 |
| rerouter (children `interval`, etc.) | NLHandler.cpp:175-177; NLTriggerBuilder.cpp:760 | yes | yes | routed via `myChildRerouter` (nl_handler.rs:632-637, 903-911) |
| busStop, trainStop, containerStop, chargingStation, parkingArea, parkingSpace, access | NLHandler.cpp:178-197 | yes | yes | chargingStation parkingArea patch missing, F10 |
| overheadWireSegment/Section, tractionSubstation, overheadWireClamp | NLHandler.cpp:198-209 | yes | n/a | |
| vTypeProbe, routeProbe | NLHandler.cpp:210-215 | yes | n/a | vTypeProbe never fires at runtime, F6 |
| meandataEdge, meandataLane | NLHandler.cpp:216-221 | yes | n/a | |
| timedEvent, vaporizer, location, taz, tazSource, tazSink, roundabout | NLHandler.cpp:222-242 | yes | n/a | |
| type, restriction, preference | NLHandler.cpp:243-283 | yes | n/a | |
| **stopOffset** | NLHandler.cpp:284-293 | **no-op** (nl_handler.rs:757-760) | n/a | silently dropped, F3 |
| railSignalConstraints | NLHandler.cpp:294-305 | yes | yes | dynamic_cast check missing, F7 |
| predecessor, foeInsertion, insertionPredecessor, insertionOrder, bidiPredecessor | NLHandler.cpp:306-312, 405-411 | yes | yes | same F7 |
| deadlock, routePart | NLHandler.cpp:313-318 | yes | n/a | |
| param | NLHandler.cpp:326-328 | yes (add_param) | n/a | |
| vehicle, person, container | SUMORouteHandler.cpp:96-98 | yes sumo_route_handler.rs:527 | yes | |
| flow, personFlow, containerFlow | SUMORouteHandler.cpp:109-135; MSRouteHandler.cpp:246 | yes | yes | |
| vType, vTypeDistribution, route, routeDistribution | SUMORouteHandler.cpp:145-167 | yes | yes | |
| **stop** (in route/vehicle/person) | SUMORouteHandler.cpp:170 | dispatched, **panics** (sumo_route_handler.rs:591-594 to ms_route_handler.rs:2373) | n/a | F1 |
| trip, personTrip, walk, ride, transport, tranship | SUMORouteHandler.cpp:173-210 | yes | yes | param stack not pushed for trip/walk/tranship/ride, F5 |
| interval | SUMORouteHandler.cpp:197 | yes | yes | |
| param | SUMORouteHandler.cpp:212 | yes | n/a | |
| lane, junction, connection (NLNetShapeHandler) | NLNetShapeHandler.cpp:53-59 | yes nl_net_shape_handler.rs:129-131 | n/a | all 3 present |

Summary: every C++ NLHandler/SUMORouteHandler/MSRouteHandler/NLNetShapeHandler tag has a Rust arm. The three defects are (a) the `stop` panic, (b) no-op stopOffset, (c) registerParent children not routed for VSS and calibrator. Rerouter children are the only routed case.

## 2. Findings (ranked)

| # | file:line (Rust) | class | C++ ref | What Rust does | Impact | Exact fix | Verified |
|---|---|---|---|---|---|---|---|
| F1 | `microsim/ms_route_handler.rs:2373` (reached via `sumo_route_handler.rs:591-594`, vtable `ms_route_handler.rs:151-152, 193`) | 4 (unconditional panic on a legal path) | `MSRouteHandler.cpp:1271` (parseStop in addStop); `SUMORouteHandler.cpp:170` | `let mut stop: SUMOVehicleParameter_Stop = panic!("...ctor not ported")`. Any `<stop>` in a route/vehicle/person aborts. | Loading any route file with `<stop>` panics. No oracle `*.rou.xml` under `tools/` contains a route `<stop>` (grep returned none), so the harness cannot catch it. | Port the `SUMOVehicleParameter::Stop()` default ctor (SUMOVehicleParameter.cpp) or check whether `SUMOVehicleParameter_Stop` already has `Default`/`new()`; replace the panic with `let mut stop = SUMOVehicleParameter_Stop::new();`. Add a route case with `<stop>`. | read-verified; not executed (cargo banned) |
| F2 | `netload/nl_trigger_builder.rs:230-231`, `:1595-1602` (also NLHandler dispatch `nl_handler.rs:674-681`) | 3 (registerParent child routing not ported) | `NLTriggerBuilder.cpp:137-139` (VSS), `:691-698` (calibrator, meso + micro); `GenericSAXHandler.cpp:163-182` (end handoff + registerParent) | Only `// shortcut` comments; `registerParent` is not called. Child events of `<variableSpeedSignal>` and `<calibrator>` go to NLHandler, then MSRouteHandler, which ignores STEP and FLOW (`MSRouteHandler` FLOW branch needs a vehicleParameter). | Inline `<step time=... speed=...>` in a VSS is dropped, so the VSS never changes speed. Inline `<flow>` in a calibrator is dropped, so it never injects vehicles. No warning. Also `MSLaneSpeedTrigger::my_end_element` (`ms_lane_speed_trigger.rs:490`) and calibrator end handling never see `</variableSpeedSignal>`/`</calibrator>`. Trigger still lazily inits at first execute, so only the inline children are lost. | Generalise the `myChildRerouter` hook (`nl_handler.rs:631-637, 902-911`) to `myChildTrigger: Ptr<SaxHandler>` set by `parse_and_build_lane_speed_trigger`/`parse_and_build_calibrator` when `file == ""`, cleared on the matching end tag, as C++ `GenericSAXHandler::endElement` does. Route `my_start_element`/`my_end_element` to it. | read-verified; not executed |
| F3 | `netload/nl_handler.rs:757-760` (helper `nl_edge_control_builder.rs:149` is never called from SAX) | 3 (ported, never called) | `NLHandler.cpp:284-293`; `SUMOVehicleClass.cpp:198-227` (`StopOffset(attrs, ok)`) | `SUMO_TAG_STOPOFFSET` arm is `let _ = attrs;` | Net `<stopOffset>` is ignored; vehicles stop at different positions than C++. Round trip netconvert to sim loses it. The writer side (`nw_writer_xml.rs:628-663`) does write them, so the data exists. | Port `StopOffset::from_attrs(attrs, &mut ok)` into `utils/common/sumo_vehicle_class.rs` (C++ 198-227). Replace the arm with: `let so = StopOffset::from_attrs(attrs, &mut ok); if !ok { write_error(&self.myEdgeControlBuilder.m().report_current_edge_or_lane()) } else { self.myEdgeControlBuilder.m().add_stop_offsets(&so) }`. | read-verified; not executed |
| F4 | `microsim/ms_route_handler.rs:84` (shortcut) | 4 (panic on legal input) | `MSRouteHandler.cpp:139, 145, 164, 166, 204, 210, 1345` (parseGeoEdges) | `parseGeoEdges` panics: MapMatcher not ported | Routes using `fromXY`/`fromLonLat`/`viaXY`/`viaLonLat`/`toXY`/`toLonLat` (and geo `<stop>` positions, `:1345`) abort. Reachability depends on whether those attributes are used. | Port MapMatcher, or until then make it a `ProcessError` with the C++ message instead of a panic so the run fails cleanly. | read-verified (shortcut text); reachability unverified |
| F5 | `microsim/ms_route_handler.rs:2815, 3021, 3286, 4349`; `sumo_route_handler.rs:593` (stack typed differently) | 3 (ported, state never pushed) | `MSRouteHandler.cpp:1230, 1604, 1686, 1833` (`myParamStack.push_back(stage)`) | Trip, walk, tranship and ride stages are not pushed onto the param stack (typed i32 in skeleton). | `<param>` children of `<trip>`/`<walk>`/`<tranship>`/`<ride>` are attached to whatever is on top of the stack, or dropped. Possible misattribution rather than an error. | Change `myParamStack` to `Vec<Ptr<Parameterised>>` in `sumo_route_handler.rs`, and push `Ptr::from_raw(addr_of_mut!((*stage.as_raw()).base2))` at each site, as C++ does. | read-verified; effect on output unverified |
| F6 | `netload/nl_detector_builder.rs:784` | 3 (ported, never registered) | `microsim/output/MSVTypeProbe.cpp:45` `getEndOfTimestepEvents()->addEvent(this)` | Shortcut: the probe is not added to the end-of-step events. | `<vTypeProbe>` (deprecated) writes no output. | Map `Command` for `MSVTypeProbe` base2, then `MSNet::get_instance().m().get_end_of_timestep_events().m().add_event(probe)` after construction. | read-verified |
| F7 | `netload/nl_handler.rs:358, 769, 1960` | 1/4 (missing type check, then cast to a different type) | `NLHandler.cpp:300-303`, `:1821-1824` (`dynamic_cast<MSRailSignal*>`, error "is not a rail signal") | `cast::<MSRailSignal>()` with no check. A non-rail TLS passes and is later used as `MSRailSignal`. | Invalid input causes type confusion (memory-unsafe) instead of the C++ error. | Before the cast, check the logic type (`TrafficLightType::RAIL_SIGNAL`) and throw `Traffic light '<id>' is not a rail signal`. | read-verified |
| F8 | `microsim/ms_route_handler.rs:2374-2375` | 4 (null handler) | `MSRouteHandler.cpp:1271` passes `MsgHandler::getErrorInstance()` | `parse_stop(..., Ptr::null())` | parseStop error messages are dropped. If `parse_stop` writes through `errorOutput` without a null check, that is a null deref (unverified). | Add a `MsgHandler::get_error_instance()` accessor and pass it, as C++ does. Check `parse_stop` for a null guard. | read-verified; deref unverified |
| F9 | `microsim/ms_route_handler.rs:1597`, `:4244`; C++ set at `MSRouteHandler.cpp:621` and count at `:1170` | 3 (state as i32) | `MSRouteHandler.cpp:621, 1170` (`mySkippedVehicles` set) | Stored as i32 (no set). Skipped ids are never recorded. | A reference to a previously skipped vehicle reports "unknown vehicle" as an error, where C++ skips silently. | Change to `HashSet<String>` (or `BTreeSet`; C++ is `std::set`, so iteration is not used). | read-verified |
| F10 | `microsim/ms_route_handler.rs:2385` | 3 (ported, skipped) | `MSRouteHandler.cpp` `dynamic_cast<MSChargingStation*>` parkingArea patch (line not located) | Patch skipped. | chargingStation stops on a parkingArea are not patched as in C++. | Port the downcast once `MSStoppingPlace` to `MSChargingStation` cast is available (per the shortcut). | read-verified; C++ line unverified |
| F11 | `netload/nl_handler.rs:52-59` (called at `:686, 692, 698`) | 4 (latent null deref) | `NLTriggerBuilder.cpp:825, 884` set `myCurrentStop = nullptr` before throwing | `addr_of_mut!((*p.as_raw()).base2)` on a null `p` is UB if reached. | Not reached today (the Rust path throws at `nl_trigger_builder.rs:1670`, so the push is skipped). A future caller would hit UB. | `if p.is_null() { return Ptr::null(); }` in both helpers. | read-verified |
| F12 | `microsim/ms_route_handler.rs:3821` | 4/1 (static_cast without checked downcast) | C++ `static_cast<MSStageMoving*>` on the copied walk | No checked downcast; `departPos` not reset. | Wrong departPos on copied walks (unverified effect). | Cast only after checking the stage type flag, as the AGENTS.md downcast rule says. | unverified |
| F13 | `netload/nl_detector_builder.rs:35` | 3 (duplicated logic) | `MSFrame::checkStepLengthMultiple` | Private copy in nl_detector_builder.rs. | Risk of drift; no behaviour change now. | Make the utils one `pub` and call it. | read-verified |
| F14 | `netload/nl_junction_control_builder.rs:347`; `microsim/ms_route_handler.rs:3504`; `netload/nl_trigger_builder.rs:22` | 4 (leak) | C++ `delete junction`, `delete s`, owner deletes at shutdown | Leaks | Memory only; no behaviour change. | Free through the concrete type once known. | read-verified |
| F15 | `netload/nl_handler.rs:16-24` | 2 (table scan) | C++ `toString(SumoXMLTag/Attr)` | Scans table; returns `""` for unknown. | Error message text may be blank for unknown tags/attrs. | Use the bijection once `sumo_xml_definitions` exposes `to_string`. | read-verified |

## 3. Checked and OK

- Virtual dispatch: NLHandler `my_start_element`/`my_end_element` chain to `MSRouteHandler` then `SUMORouteHandler` (`sumo_route_handler.rs:740`). C++ defines no `MSRouteHandler::myEndElement` (only `NLHandler.cpp:437`, `MSCalibrator.cpp:271`, `MSStateHandler.cpp:518` call it), so the chain matches. `SUMORouteHandler::add_stop` goes through the vtable `vt.add_stop` and is wired to `ms_add_stop` (`ms_route_handler.rs:193`). Unverified: whether every other `vt.*` entry is wired for MS, because the defaults at `sumo_route_handler.rs:199-203` panic.
- Rerouter child routing (`myChildRerouter`) is implemented: it forwards start, end, and clears on `</rerouter>`. This matches `GenericSAXHandler::endElement`.
- Junction-logic constructors (`nl_junction_control_builder.rs:862-981`) cover C++ `NLJunctionControlBuilder.cpp:277-335` and call `schedule_initial_switch` for each simple, actuated, nema, delay-based, SOTL-policy, and wave type. Unverified: the rail types (no `schedule` in C++ for them).
- `addInstantE1Detector`: `myCurrentIsBroken = true` is unconditional in both C++ (`NLHandler.cpp:1039`) and Rust (`nl_handler.rs:1181`). Faithful, including the pop-skip quirk.
- Lane-speed trigger `add_event` for begin/end of time-step: `nl_trigger_builder.rs:158-161` matches `NLTriggerBuilder.cpp:101-103`.
- Detector registration with `get_detector_control().add*` is wired for all E1, E2, E3, route-probe, and instant cases (`nl_detector_builder.rs:267-1086`).
- Class 2 (stale globals/hardcoded constants): not systematically checked. I found no new instance in the paths read.

## 4. Not checked (honest gaps)

- `nl_handler.rs` past line 1018 was only read at the dispatch level. Bodies of `add_connection`, `add_conflict`, `add_deadlock`, `add_deadlock_route`, `begin_e3_detector`, `add_edge_lane_mean_data` were not read side by side.
- `ms_route_handler.rs` bodies beyond the lines cited were not read.
- `NLEdgeControlBuilder`, `NLJunctionControlBuilder` builders other than the spots above.
- Whether `SUMOVehicleParameter_Stop` has a `Default` impl (needed to fix F1 exactly).

## 5. Top 3

1. F1 `<stop>` in route files panics (`ms_route_handler.rs:2373`). Not caught by any oracle case.
2. F2 registerParent children of `<variableSpeedSignal>` and `<calibrator>` are silently dropped (`nl_trigger_builder.rs:230, 1595-1602`).
3. F3 net `<stopOffset>` is a no-op (`nl_handler.rs:757-760`), with the StopOffset port and the edge helper already present.
