# Audit summary: au01 to au12

Base: `main` f69d911. Source: the 12 reports `tools/oracle/audits/AUDIT_au01.md` to `AUDIT_au12.md` in this folder (copied byte-for-byte from `worktree-agent-au01` to `worktree-agent-au12`). No cargo run, no `.rs` edits.

## Counts

- Raw rows across the 12 reports: 149.
- Deduplicated findings: 133 (R01 to R133).
- Certain on a plain car-only run: **1** (R01).
- UNASSIGNED: **54** (30 possible, 24 none; grouped by file at the end).

## Legend

- **Class**: 1 missing dispatch, 2 stale global (hardcoded value or stub standing in for a global, option, or lookup), 3 never called (ported but unwired), 4 null deref or panic. A `(+4)` marks a panic on top of the main class.
- **Impact on a plain car-only run**:
  - `certain`: fires on a default car run with no options, devices, TraCI, or special network features.
  - `possible`: needs an option, device, TraCI call, special TLS/junction/net type, or is unverified.
  - `none`: rail, persons, meso, or latent; no effect on a car-only run.
- **Status**: the owner from the assignment list, with the source report row in brackets. Dedup rule: a row goes to the owner of an identical defect. Where a row is not covered by any listed assignment, it is `UNASSIGNED`.

## Findings table

Order: certain first, then `possible` grouped by feature, then `none` grouped by feature.

| id | file:line | class | impact | feature | status |
|---|---|---|---|---|---|
| R01 | ms_base_vehicle.rs:2182 (get_odometer, calls base has_arrived at L2114) | 1 | certain | odometer (TraCI distance readers); any car on last edge before arrivalPos | w01 (au02#2) |
| R02 | output/ms_induct_loop.rs:131-162 (registered :159) | 1 | possible | detectors: E1 induction loop counts nothing | tl01 (au09#1) |
| R03 | output/mse2_collector.rs:322,431,2860-2865 | 1 | possible | detectors: E2 area detector never tracks vehicles | tl01 (au09#2) |
| R04 | netload/nl_detector_builder.rs:910-912; output/mse3_collector.rs:202-213 (build_reminders has 0 callers) | 3 | possible | detectors: E3 entry/exit reminders never built | tl01 (au09#4) |
| R05 | output/ms_instant_induct_loop.rs:60-65 (registered nl_detector_builder.rs:857) | 1 | possible | detectors: instant E1 | tl01 (au09#5) |
| R06 | output/ms_mean_data.rs:1251-1253,1813,1827-1832; ms_move_reminder.rs:299-301 | 1 | possible | MeanData output: edge/lane data zero or empty | w03 (au09#3) |
| R07 | traffic_lights/ms_simple_traffic_light_logic.rs:132-134 (vt only for STATIC); no vt in delay_based, nema_controller, ms_rail_crossing, mssotl_*, ms_swarm, ms_deterministic_hi_level | 1 | possible | TLS types other than static: panic at first switch | tl01 (au08#1) |
| R08 | traffic_lights/ms_traffic_light_logic.rs:1383-1386 (get_tl_queue_length) | 1 | possible | TLS: GLOSA queue term is 0 for delay_based | tl01 (au08#3) |
| R09 | ms_move_reminder.rs:33-37; enter/move/leave overrides in ~18 device ctors | 1 | possible | devices (all but tripinfo): enter/move/leave overrides dead (fcd, vehroutes, ssm, taxi, glosa, friction, ...) | w02 (au07#2) |
| R10 | ms_move_reminder.rs:138-157; callers ms_vehicle.rs:2980,2986,9282,10102,10404 | 1 | possible | devices: battery parking charge skipped; tripinfo waitingTime and waitingCount stay 0 | w02 (au07#3; au01#1) |
| R11 | ms_base_vehicle.rs:564-565,593,666-667; ms_device_tripinfo.rs:337-344 | 1 (+4) | possible | devices: TraCI has/get/setDeviceParameter panic once a device exists; tripinfo pending output | w02 (au07#1; au10 F17) |
| R12 | ms_base_vehicle.rs:343-363 (init_devices builds 3 of 19) | 3 | possible | devices: vehroute, emission, per-vehicle device params ignored | w02 (au07#4) |
| R13 | ms_vehicle.rs:2750-2751, 7778-7781; ms_base_vehicle.rs:2907-2910 (get_device stub) | 2 | possible | devices: driverstate and friction never set | w02 (au07#5; au01#2) |
| R14 | devices/ms_device_fcd.rs:515-523 | 3 | possible | devices: fcd-output.filter-shapes misses back-position test | w02 (au07#8) |
| R15 | ms_base_vehicle.rs:343-344 (unsafe &mut alias) | 4 | possible (UB, no observed effect) | devices: init_devices runs for every vehicle | w02 (au07#9) |
| R16 | devices/ms_device_b_tsender.rs:259-370; ms_device_b_treceiver.rs:851-991; ms_device_tripinfo.rs:2067-2075 | 1 | possible (unverified) | devices: BT and tripinfo hooks reached via base dispatch | w02 (au09#15) |
| R17 | lcmodels/mslcm_dk2008.rs:96-107 | 1 | possible | lane change: DK2008 vtypes panic on patch_speed (no vt installed) | w04 (au05#1) |
| R18 | ms_lane_changer.rs:2338-2346 (called at 1431, 3477) | 1 | possible | lane change: opposite-direction overtakes with lateral-resolution > 0 | w04 (au05#2) |
| R19 | ms_lane_changer.rs:2153-2156 (checkChangeBeforeCommitting) | 3 | possible | lane change: CC platoons; the check never runs | w04 (au05#4) |
| R20 | ms_lane_changer.rs:3456-3458 | 2 | possible | lane change: opposite stop time is 0 with lanechange.duration or lateral-resolution set | w04 (au05#5; au10 F11) |
| R21 | ms_lane_changer.rs:2973-2979 | 2 | possible | lane change: sublane term missing from overtaking time | w04 (au05#6; au10 F11) |
| R22 | ms_lane_changer.rs:2297-2299 | 3 | possible | lane change: duration block skipped when lanechange.duration > 0 | w04 (au05#7; au10 F11) |
| R23 | ms_lane_changer.rs:1212 (lefthand = false) | 2 | possible | lane change: lefthand networks change on the wrong side | sg01 (au10 F11) |
| R24 | ms_lane.rs:401-403 (callers 1068,1117,3515,5321-5328) | 2 | possible | sublane / lateral-resolution > 0 disabled | sg01 (au03#1; au05#3) |
| R25 | ms_edge.rs:3663 | 2 | possible | stopping: stopped-in-range check passes checkFuture=true (C++ false) | sg01 (au03#2) |
| R26 | ms_edge.rs:13 (G_USING_INTERNAL_LANES = true) | 2 | possible | no-internal-links ignored | sg01 (au03#4; au10 F2) |
| R27 | ms_edge.rs:18 (G_LANE_CHANGE_DURATION = 0) | 2 | possible | lanechange.duration ignored | sg01 (au03#5) |
| R28 | ms_edge.rs:19 (G_STOP_TOLERANCE = 10) | 2 | possible | ride.stop-tolerance ignored | sg01 (au03#6) |
| R29 | ms_edge.rs:14-17 (USE_MESO, TLS, MINOR, TURNAROUND penalty consts) | 2 | possible | routing costs: minor 1.5 and turnaround 5.0 penalties read as 0. au03 says meso-only, au10 says default run: audits disagree | sg01 (au03#7; au10 F2) |
| R30 | cfmodels/mscf_model.rs:34-42 (about 31 call sites) | 2 | possible | all CF models: compute_lc is false during lane changes; ballistic and default.emergencydecel options ignored. Candidate to upgrade to certain (see notes) | sg01 (au06#1; au10 F1) |
| R31 | ms_vehicle_transfer.rs:292-293,304-305; ms_vehicle.rs:3157 | 2 | possible | parking.maneuver ignored; lefthand on parked re-entry | sg01 (au10 F4) |
| R32 | ms_vehicle_type.rs:1566-1580 | 2 | possible | action-step vtypes stay semi-implicit, no ballistic switch | sg01 (au10 F5) |
| R33 | output/ms_mean_data.rs:10-15 (use_meso_sim and using_internal_lanes stubs) | 2 | possible | MeanData: no-internal-links ignored | sg01 (au09#11; au10 F12) |
| R34 | ms_vehicle.rs:5026-5027 | 2 | possible | accumulated waiting time uses 300 s, not the option | sg01 (au10 F7) |
| R35 | ms_net.rs write_output (gHaveEmissions never set); output/ms_detector_control.rs:149 | 2 | possible | full-output emissions and emission detectors not enabled | sg01 (au10 F8) |
| R36 | ms_lane.rs:771-772 (is_normal() or false) | 2 | possible (unverified) | bidi-length warning suppressed for internal lanes | sg01 (au10 F3b) |
| R37 | devices/ms_device_routing.rs:367 | 2 | possible | weights.separate-turns: junction travel times not recorded | sg01 (au10 F14) |
| R38 | libsumo/vehicle.rs:3333 (if false, gCheckRoutes) | 2 | possible | TraCI setRoute: route-error path skipped | sg01 (au10 F6) |
| R39 | devices/ms_device_ssm.rs:2194 (via ms_junction.rs:159) | 1 | possible | SSM device: foe internal lanes never collected at logic junctions | w05 (au04#2) |
| R40 | devices/ms_device_ssm.rs:2146,6168; netload/nl_net_shape_handler.rs:90,274; ms_junction.rs:205 | 1 | possible | SSM internal lanes empty at logic junctions; alt-network shape matching | w05 (au04#3) |
| R41 | ms_link.rs:49 (zipper_foe_as_vehicle stub) -> ms_link.rs:3011 | 2 | possible | ZIPPER junctions: ego never yields to vehicle foes | w05 (au04#4) |
| R42 | ms_route_handler.rs:2373-2375 | 4 | possible | route files with a `<stop>` element: panic | rh01 (au11#1) |
| R43 | netload/nl_trigger_builder.rs:230-231,1595-1602 | 3 | possible | inline `<step>` in variableSpeedSignal and inline `<flow>` in calibrator dropped | w06 (au11#2) |
| R44 | netload/nl_handler.rs:757-760 | 3 | possible | net `<stopOffset>` ignored | w06 (au11#3) |
| R45 | ms_route_handler.rs:84 (parseGeoEdges, MapMatcher) | 4 | possible | routes with geo attributes: panic | UNASSIGNED (au11#4) |
| R46 | ms_route_handler.rs:1597,4244 (mySkippedVehicles) | 3 | possible | references to skipped vehicles report errors | UNASSIGNED (au11#9) |
| R47 | ms_route_handler.rs:2385 | 3 | possible | chargingStation stops on parkingArea not patched | UNASSIGNED (au11#10) |
| R48 | ms_route_handler.rs:2815,3021,3286,4349 | 3 | possible | param children of trip/walk/tranship/ride misattributed | UNASSIGNED (au11#5) |
| R49 | netload/nl_handler.rs:358,769,1960 | 1 (+4) | possible | rail-signal TLS: unchecked cast to MSRailSignal | UNASSIGNED (au11#7) |
| R50 | ms_route_handler.rs:3821 | 4 | possible (unverified) | copied walk: departPos not reset | UNASSIGNED (au11#12) |
| R51 | netload/nl_detector_builder.rs:784 vs output/msv_type_probe.rs:62-65 | 3 | possible | vTypeProbe output. DISPUTED: au11#6 says never registered, au09#12 says registered by msv_type_probe.rs | UNASSIGNED (au11#6; au09#12) |
| R52 | trigger/ms_calibrator.rs:1138-1147,314-364; netload/nl_trigger_builder.rs:1808-1811 | 3 | possible | calibrator: never init()ed, XML not parsed; inert | UNASSIGNED (au09#7) |
| R53 | trigger/ms_calibrator.rs:213-217,371-373,1565-1571; ms_move_reminder.rs:72-74 | 3 | possible | calibrator VehicleRemover and lane data never added to a lane | UNASSIGNED (au09#8) |
| R54 | output/ms_route_probe.rs:59-80 | 1 | possible | routeProbe writes no routes | UNASSIGNED (au09#6) |
| R55 | trigger/ms_overhead_wire.rs:1191-1196,1250 | 3 | possible | EV overhead wire: circuit never solved, charging state stuck | UNASSIGNED (au09#9) |
| R56 | ms_vehicle.rs:7106,7176,7258 (hasElecHybrid) | 2 | possible | EV: elec-hybrid overhead-wire lane choice (get_device stub) | UNASSIGNED (au01#3) |
| R57 | ms_vehicle.rs:3645 (C++ block at 3406-3421) | 2 | possible | EV: elec-hybrid acceleration cap skipped | UNASSIGNED (au01#4) |
| R58 | ms_lane.rs:3406-3407,9128-9129 | 2 | possible | taxi vehicles teleported off last edge (get_device stub) | UNASSIGNED (au03#10) |
| R59 | ms_lane.rs:9134-9135 | 3 | possible (unverified) | rerouter move-reminder identity match | UNASSIGNED (au03#11) |
| R60 | ms_link.rs:75 (ego_rand) | 2 | possible | ignore-foe-prob > 0: wrong RNG stream | UNASSIGNED (au04#6) |
| R61 | ms_internal_junction.rs:139-140 | 4 | possible | malformed net (internal junction with no incoming lanes): panic, not a clean error | UNASSIGNED (au04#5) |
| R62 | ms_edge.rs:365-368 | 4 | possible | lateral-resolution > 0 and edge drop: wrong-type free (UB) | UNASSIGNED (au03#3) |
| R63 | cfmodels/mscf_model.rs:78-104 | 1 | possible | TraCI get/setParameter on CC and CACC not dispatched | UNASSIGNED (au06#2) |
| R64 | ms_base_vehicle.rs:1777 (commented), 5530 | 3 | possible | reroute does not re-activate move reminders (rerouter, station finder) | UNASSIGNED (au10 F19) |
| R65 | ms_device_routing.rs (whole file); ms_insertion_control.rs:954 | 3 | possible | periodic and pre-insertion reroutes never fire | UNASSIGNED (au10 F20) |
| R66 | ms_vehicle_control.rs:477-486 | 1 | possible | generateOutput only dispatches for tripinfo; other devices lose output | UNASSIGNED (au10 F16; root = w02 vtable) |
| R67 | ms_net.rs:2870-2873 | 1 | possible | TraCI VehicleStateListener: only some listeners get callbacks | UNASSIGNED (au10 F18) |
| R68 | ms_net.rs:1749-1762,2179,2201 | 3 | possible | netstate, emission, battery, and other output files silently missing | UNASSIGNED (au10 F22) |
| R69 | ms_net.rs:1097 (apply_scheduled_permissions) | 3 | possible (unverified) | triggered-rerouter permissions applied out of order | UNASSIGNED (au10 F23) |
| R70 | actions/command_save_tls_program.rs:46; command_save_tls_state.rs:45; command_save_tls_switches.rs:42; command_save_tls_switch_states.rs:44; command_save_tl_coupled_det.rs:45 | 3 | possible | TLS save outputs never written | UNASSIGNED (au10 F24) |
| R71 | ms_net.rs:614 | 4 | possible | weights.separate-turns without internal links: panic | UNASSIGNED (au10 F30) |
| R72 | ms_net.rs:452,868 | 2 | possible | output header version reads "unknown" | UNASSIGNED (au10 F10) |
| R73 | ms_net.rs:3060,3102 | 2 | possible (unverified) | per-vehicle weights storage ignored in routing costs | UNASSIGNED (au10 F27) |
| R74 | ms_frame.rs:2250-2255 | 2 | possible (unverified) | taxi class mask truncated to i32 | UNASSIGNED (au10 F15) |
| R75 | trigger/ms_stopping_place_rerouter.rs:898,963 | 1 | possible | parking-area alternatives: base last-free-pos used, override skipped | w08 (au12#10) |
| R76 | trigger/ms_triggered_rerouter.rs:1860,2220 | 1 | possible (unverified) | raw cast to MSParkingArea without type check | w08 (au12#15) |
| R77 | ms_base_vehicle.rs:4049-4050 (-> L3998) | 1 (+4) | possible | TraCI abortNextStop on a stopped vehicle: panic | w01 (au02#1) |
| R78 | ms_base_vehicle.rs:2983 (replace_vehicle_type) | 1 | possible | TraCI setType: car-following state from old type | w01 (au02#3) |
| R79 | ms_base_vehicle.rs:3642-3679 (add_stop) | 3 | possible | stops with collision="true" added without collision handling | w01 (au02#4) |
| R80 | ms_base_vehicle.rs:2216-2217 | 1 | possible | TraCI influencer ignored in impatience and waiting term | w01 (au02#5) |
| R81 | ms_base_vehicle.rs:2853 | 1 | possible | rail class on internal lanes reads route edge | w01 (au02#6) |
| R82 | ms_base_vehicle.rs:5541 | 1 | possible | reroute: approach entries not reset | w01 (au02#7) |
| R83 | ms_base_vehicle.rs:1690-1696 | 1 (+4) | possible | meso vehicle reroute: unchecked Ptr::cast to MSVehicle | w01 (au02#8) |
| R84 | ms_base_vehicle.rs:1503,1558,1685 | 2 | possible | gCheckRoutes treated as false (C++ default true) on route errors | w01 (au02#9; au10 F6) |
| R85 | ms_base_vehicle.rs:358 | 2 | possible | gHaveEmissions: emission params not forced | w01 (au02#10; au07#6; au10 F8) |
| R86 | ms_base_vehicle.rs:1545 | 2 | possible (unverified) | weights.random-factor fixed at 1 | w01 (au02#11) |
| R87 | ms_junction.rs:250 (get_logic; callers ms_drive_way.rs:735,737,755,756,3347) | 1 | none | rail driveways and RoW logic junctions (rail only) | w05 (au04#1) |
| R88 | ms_traffic_light_logic.rs:1426-1465 (rival, priority, blocking, requested) | 1 | none | rail signals; TraCI getRivalVehicles and getPriorityVehicles, incl. subscriptions (w07 scope) | tl01 (au08#2) |
| R89 | ms_traffic_light_logic.rs:911-935 | 1 | none (headless) | actuated and NEMA program switch: GUI detector visibility | tl01 (au08#4) |
| R90 | ms_traffic_light_logic.rs:1115-1154 | 1 | none | actuated min/max/earliest/latest durations (no caller found) | tl01 (au08#5) |
| R91 | ms_traffic_light_logic.rs:1372-1375 | 1 | none | get_detector_state (no caller found) | tl01 (au08#6) |
| R92 | ms_simple_traffic_light_logic.rs:166,456; ms_actuated_traffic_light_logic.rs:1306,1620 | 4 | none (latent) | null mySwitchCommand deschedule | tl01 (au08#7) |
| R93 | transportables/ms_stage.rs:228-231,237-240,285-288,294-297,303-306,581-584,590-593,599-602,607-610 | 1 | none | persons: base panic on any person plan | w08 (au12#1) |
| R94 | ms_transportable.rs:987,997 | 1 | none | persons: never board a vehicle | w08 (au12#2) |
| R95 | ms_transportable.rs:585 | 1 | none | persons: current lane null for moving stages | w08 (au12#3) |
| R96 | ms_transportable.rs:1055,286-287,1262,1529 | 1 | none | persons: vehicle never released; panic at 1529 | w08 (au12#4) |
| R97 | ms_transportable.rs:673 | 1 | none | persons: stage speed 0 | w08 (au12#5) |
| R98 | ms_transportable.rs:807 | 1 | none | persons: route position 0 | w08 (au12#6) |
| R99 | ms_transportable.rs:760,1179; ms_stage_trip.rs:650 | 1 | none | persons: edge list reduced to destination | w08 (au12#7) |
| R100 | ms_transportable.rs:616 | 1 | none (unverified) | persons: edge position | w08 (au12#8) |
| R101 | ms_stage_driving.rs:663,1200,1343; ms_person.rs:326-345; ms_transportable.rs:1526,1531 | 4 | none (unverified) | persons: unguarded stop get_lane | w08 (au12#12-14) |
| R102 | ms_stopping_place.rs:444-449 (get_stopping_position) | 1 | none | no callers (latent) | w08 (au12#11) |
| R103 | ms_base_vehicle.rs:2393 (initTransportableDevice) | 3 | none | persons: transportable device not wired | UNASSIGNED (au07#7) |
| R104 | ms_junction_control.rs:76-78 | 4 | none | unchecked cast (builder guarantees RoW) | UNASSIGNED (au04#8) |
| R105 | ms_internal_junction.rs:148-161 | 1 | none | node-type approximation of dynamic_cast | UNASSIGNED (au04#9) |
| R106 | ms_internal_junction.rs:160 | 4 | none | unguarded viaLink (same as C++) | UNASSIGNED (au04#10) |
| R107 | ms_logic_junction.rs:72,101 | 3 | none | never reached; another agent is editing this file | UNASSIGNED (au04#11) |
| R108 | ms_junction.rs:238-243 (passed_junction) | 3 | none | empty body, no callers | UNASSIGNED (au04#12) |
| R109 | ms_link.rs:61 (MSPMODEL_SAFETY_GAP) | 2 | none | matches C++ value; no action | UNASSIGNED (au04#7) |
| R110 | ms_edge.rs:499,524,534 | 4 | none | unguarded deref, same as C++; gated by constants | UNASSIGNED (au03#12) |
| R111 | ms_vehicle.rs:1240 | 4 | none | unwrap on zero-lane edge (degenerate network) | UNASSIGNED (au01#5) |
| R112 | ms_vehicle.rs:2064,2068,2084,2182,2186 | 4 | none | Ptr::get unwrap in vtable adapters; no action | UNASSIGNED (au01#6) |
| R113 | mscf_model_rail.rs:462-463 | 4 | none | unguarded veh.get_slope (no null caller found); no action | UNASSIGNED (au06#3) |
| R114 | lcmodels/mslcm_sl2015.rs:4522-4525; ms_abstract_lane_change_model.rs:3340-3367 | 4 | none | unguarded msg (same as C++ assert); no action | w04 (au05#9) |
| R115 | ms_lane.rs:409-416 (g_emergency_insert, g_check_routes) | 2 | none | duplicate option readers, values equal today | sg01 (au03#8; au10 F13) |
| R116 | ms_edge.rs:10-12; cfmodels/mscf_model.rs:32-33; output/ms_mean_data.rs:7-9; ms_edge.rs:365-366,669-670 | — (comment) | none | stale comments say MSGlobals or shims are not ported | sg01 (au03#9; au06#4; au05#8; au09#11) |
| R117 | ms_net.rs:2791-2792 (gStateLoaded) | 2 | none | never set; loadState panics today | sg01 (au10 F9) |
| R118 | ms_net.rs:1142 (processPendingRouting) | 3 | none | persons and containers: pending routing not run | UNASSIGNED (au10 F26) |
| R119 | ms_net.rs:1069,1792,1821,1937 | 3 | none | TraCI load: early return and SIMSTATE_LOADING not reported | UNASSIGNED (au10 F25) |
| R120 | ms_edge_control.rs:102,241,350,469 | 3 | none | serial execution; determinism note only | UNASSIGNED (au10 F28) |
| R121 | ms_edge_control.rs:605 | 3 | none | meso segments not built (meso is refused) | UNASSIGNED (au10 F29) |
| R122 | output/mse2_collector.rs:2861-2863; output/ms_route_probe.rs:60; output/mse3_collector.rs:154; output/ms_induct_loop.rs:138 | 4 | none | invalid input: panic instead of ProcessError | UNASSIGNED (au09#14) |
| R123 | trigger/ms_calibrator.rs:59; output/ms_instant_induct_loop.rs:61; output/mse3_collector.rs:201 | — (comment) | none | stale comments | UNASSIGNED (au09#12, #13) |
| R124 | trigger/ms_calibrator.rs:1586 | 4 | none (latent) | unchecked index in VehicleRemover | UNASSIGNED (au09#10) |
| R125 | ms_transportable_routing.rs:81,137; ms_stage_driving.rs:103; ms_transportable.rs:309; ms_device_station_finder.rs:277,740; ms_device_to_c.rs:499-633 | 3 | none | addEvent never scheduled (persons, or devices not built on a car run) | UNASSIGNED (au10 F21) |
| R126 | netload/nl_detector_builder.rs:35 | 3 | none | duplicate of MSFrame::checkStepLengthMultiple | UNASSIGNED (au11#13) |
| R127 | netload/nl_junction_control_builder.rs:347; ms_route_handler.rs:3504; netload/nl_trigger_builder.rs:22 | 4 | none | memory leaks | UNASSIGNED (au11#14) |
| R128 | ms_route_handler.rs:2374-2375 | 4 | none | null error handler passed to parse_stop | UNASSIGNED (au11#8) |
| R129 | netload/nl_handler.rs:52-59 | 4 | none (latent) | addr_of on a possibly null pointer | UNASSIGNED (au11#11) |
| R130 | netload/nl_handler.rs:16-24 | 2 | none | toString returns "" for unknown tags and attrs | UNASSIGNED (au11#15) |
| R131 | ms_base_vehicle.rs:1375 (gUseMesoSim) | 2 | none | meso only | w01 (au02#12) |
| R132 | ms_base_vehicle.rs:4439-4444, 4348 (update_best_lanes) | 3 | none | never called; insertStop not ported | w01 (au02#13) |
| R133 | ms_base_vehicle.rs:2646 (inherent has_valid_route_start) | 3 | none | unused duplicate; live path is correct | w01 (au02#14) |

## Assignment gaps and conflicts

- **rh01, "route-file flow inline route"**: no audit row in these 12 reports. Only its stop panic (R42) is covered.
- **tl01, "actuated/NEMA option panic"**: no audit row. The NEMA vtable gap is R07.
- **fx03 (LinkBits)**: no audit row.
- **w07 (TraCI subscriptions)**: only mentioned in R88 (au08#2). No dedicated row.
- **au12**: numbering skips row 9 in the source report. Not a missing finding; the report has 14 rows.
- **Disputed (R51)**: au09 says vTypeProbe is registered (msv_type_probe.rs:62-65), au11 says it never fires (nl_detector_builder.rs:784). One needs checking against the source.
- **Disputed (R29)**: au03 says penalties matter only with meso; au10 says they matter on the default run. Not resolved here.
- **Upgrade candidate (R30)**: lane-change steps set compute_lc, which the stub reads as false. On a default run this may be certain. Runtime unverified, so kept as possible.
- **R20, R21, R22 (au05 #5-7)** and **R22 vs au10 F11**: same defects, one row each.
- **R18 and R19**: depend on lateral-resolution and CC; the default run is unaffected.

## UNASSIGNED rows grouped by file (54)

- **ms_vehicle.rs** (4): R56, R57, R111, R112
- **ms_lane.rs** (2): R58, R59
- **ms_edge.rs** (2): R62, R110
- **ms_link.rs** (2): R60, R109
- **ms_junction_control.rs** (1): R104
- **ms_internal_junction.rs** (3): R61, R105, R106
- **ms_logic_junction.rs** (1): R107 (another agent is editing this file)
- **ms_junction.rs** (1): R108
- **cfmodels/mscf_model.rs** (1): R63
- **cfmodels/mscf_model_rail.rs** (1): R113
- **ms_base_vehicle.rs** (2): R64, R103
- **ms_device_routing.rs / ms_insertion_control.rs** (1 row, 2 files): R65
- **ms_transportable_routing.rs, ms_stage_driving.rs, ms_transportable.rs, ms_device_station_finder.rs, ms_device_to_c.rs** (1 row, 5 files): R125
- **ms_vehicle_control.rs** (1): R66
- **ms_route_handler.rs** (7): R45, R46, R47, R48, R50, R127, R128
- **nl_handler.rs** (3): R49, R129, R130
- **nl_junction_control_builder.rs** (1): R127
- **nl_trigger_builder.rs** (2): R52 (line 1808), R127 (line 22)
- **nl_detector_builder.rs** (2): R51 (disputed), R126
- **output/msv_type_probe.rs** (1): R51 (disputed companion)
- **trigger/ms_calibrator.rs** (4): R52, R53, R123, R124
- **ms_move_reminder.rs** (1): R53 (lines 72-74)
- **output/ms_route_probe.rs** (2): R54, R122
- **output/mse2_collector.rs** (1): R122
- **output/mse3_collector.rs** (1): R122, R123 (2 rows)
- **output/ms_induct_loop.rs** (1): R122
- **output/ms_instant_induct_loop.rs** (1): R123
- **trigger/ms_overhead_wire.rs** (1): R55
- **ms_net.rs** (8): R67, R68, R69, R71, R72, R73, R118, R119
- **ms_edge_control.rs** (2): R120, R121
- **ms_frame.rs** (1): R74
- **actions/command_save_tls_*.rs** (5 files, 1 row): R70

Rows are unique across the list: 54 rows in total. Some rows span several files, so the per-file counts above add up to more than 54.
