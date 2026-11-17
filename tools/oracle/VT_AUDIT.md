# Virtual dispatch audit (bug class 1)

Scope: every Rust `*Vt` table under `crates/sumo_core` against the C++ overrides in
`C:\Users\karti\sumo-src\src`. A row is PASS when the Rust object is built with a table whose
entry points at the subclass method (not a base no-op or `DEFAULT_*` stub).

Skipped by instruction (owned by other agents): MS_PERSON_VT (ls07), ME_VEHICLE_VT (mc02),
MS_CONTAINER_VT (td01), MSMeanData_Emissions (em02), MSTransportableDevice / MSDevice_Transportable
(person devices, ls07).

Status key: PASS = table installed with the C++ overrides; FIXED = fixed in this branch;
MAIN = already wired on main (merged in, verified by grep of main); OPEN = gap left.

## CF models (`MSCFModelVt`)

| Class | Overrides checked | Status |
|---|---|---|
| MSCFModel_ACC | followSpeed, stopSpeed, getSecureGap, insertionFollowSpeed, interactionGap, maximumLaneSpeedCF, getModelID, duplicate, createVehicleVariables | PASS |
| MSCFModel_CACC | freeSpeed, followSpeed, stopSpeed, getSecureGap, insertionFollowSpeed, interactionGap, setHeadwayTime, maximumLaneSpeedCF, getModelID, duplicate, createVehicleVariables | PASS |
| MSCFModel_CACC | getParameter, setParameter | FIXED (MSCFModelVt slots added; MSBaseVehicle delegates via as_micro_vehicle) |
| MSCFModel_CC | stopSpeed, finalizeSpeed, freeSpeed, followSpeed, insertionFollowSpeed, interactionGap, getSecureGap, maxNextSpeed, minNextSpeed, getModelID, duplicate, createVehicleVariables | PASS |
| MSCFModel_CC | getParameter, setParameter | FIXED (same slots; CC setParameter already returned Result) |
| MSCFModel_Daniel1 | followSpeed, stopSpeed, getModelID, getImperfection, setMaxDecel, setImperfection, setHeadwayTime, duplicate, finalizeSpeed | PASS |
| MSCFModel_EIDM | insertionFollowSpeed, insertionStopSpeed, finalizeSpeed, followSpeed, stopSpeed, freeSpeed, interactionGap, getSecureGap, brakeGap, maxNextSafeMin, maximumLaneSpeedCF, getModelID, duplicate, createVehicleVariables | MAIN (brakeGap and maxNextSafeMin were missing; main wires them) |
| MSCFModel_IDM | finalizeSpeed, freeSpeed, followSpeed, stopSpeed, interactionGap, insertionFollowSpeed, insertionStopSpeed, getSecureGap, minNextSpeed, getModelID, duplicate, createVehicleVariables | PASS |
| MSCFModel_Kerner | stopSpeed, finalizeSpeed, followSpeed, getModelID, duplicate, createVehicleVariables | PASS |
| MSCFModel_Krauss | patchSpeedBeforeLC, stopSpeed, followSpeed, getModelID, duplicate, createVehicleVariables | PASS |
| MSCFModel_KraussPS | maxNextSpeed, getModelID, duplicate | PASS |
| MSCFModel_KraussOrig1 | patchSpeedBeforeLC, followSpeed, stopSpeed, getModelID, duplicate, getImperfection, setImperfection, setMaxDecel, setHeadwayTime | PASS |
| MSCFModel_KraussX | patchSpeedBeforeLC, getModelID, duplicate | PASS |
| MSCFModel_NaSch | followSpeed, insertionFollowSpeed, stopSpeed, patchSpeedBeforeLC, minNextSpeedEmergency, getModelID, getImperfection, setImperfection, duplicate | PASS |
| MSCFModel_PWag2009 | finalizeSpeed, patchSpeedBeforeLC, followSpeed, stopSpeed, getModelID, getImperfection, duplicate, createVehicleVariables | PASS |
| MSCFModel_Rail | followSpeed, getModelID, duplicate, createVehicleVariables, maxNextSpeed, minNextSpeed, minNextSpeedEmergency, finalizeSpeed, freeSpeed, startupDelayStopped, stopSpeed | PASS |
| MSCFModel_SmartSK | finalizeSpeed, followSpeed, stopSpeed, getModelID, getImperfection, setMaxDecel, setImperfection, setHeadwayTime, patchSpeedBeforeLC, duplicate, createVehicleVariables | PASS |
| MSCFModel_W99 | followSpeed, stopSpeed, interactionGap, getModelID, duplicate | PASS |
| MSCFModel_Wiedemann | finalizeSpeed, followSpeed, stopSpeed, interactionGap, getSecureGap, getModelID, duplicate, createVehicleVariables | PASS |

## Move reminders (`MSMoveReminderVt`)

| Class | Overrides | Status |
|---|---|---|
| MSDriveWay | notifyEnter, notifyLeave | MAIN (DRIVEWAY_MR_VT) |
| MSE2Collector | notifyEnter, notifyMove, notifyLeave | MAIN (MSE2_MSMOVE_REMINDER_VT) |
| MSE3EntryReminder / MSE3LeaveReminder | notifyEnter, notifyMove, notifyLeave | MAIN |
| MeanDataValues | notifyEnter, notifyMove, notifyLeave | MAIN |
| MSInductLoop | notifyEnter, notifyMove, notifyLeave | MAIN |
| MSInstantInductLoop | notifyEnter, notifyMove, notifyLeave | MAIN |
| MSTriggeredRerouter | notifyEnter, notifyMove, notifyLeave | MAIN |
| MSRouteProbe | notifyEnter | FIXED (table added on top of main; offset_of back-cast since base2 is not first) |
| MSCalibrator::VehicleRemover | notifyEnter, notifyLeave | FIXED (table added on top of main) |
| MSRailSignalConstraint::PassedTracker | notifyEnter | FIXED (table added on top of main) |
| MSDevice_Routing | notifyEnter, notifyStopEnded | FIXED (notifyStopEnded was missing: installed in MS_DEVICE_ROUTING_MSMOVE_REMINDER_IDLE_VT, set as `idle_vt`) |
| MSDevice_FCD | notifyEnter (returns false: reminder is dropped) | MAIN (MS_MOVE_REMINDER_VT, new_device) |
| MSDevice_Tripinfo | notifyEnter, notifyMove, notifyLeave | PASS |
| MSDevice_BTsender / BTreceiver / Routing / GLOSA / ElecHybrid / Friction / Example / Bluelight / Battery / SSM / StationFinder / Taxi / ToC / Vehroutes / Emissions | device overrides | MAIN (each installs its own table or MS_MOVE_REMINDER_VT) |

## Traffic light logics (`MSTrafficLightLogicVt`)

| Class | Overrides | Status |
|---|---|---|
| MSSimpleTrafficLightLogic | trySwitch, getPhase*, reset, offsets, changeStepAndDuration, saveState, mapTimeInCycle | PASS |
| MSActuatedTrafficLightLogic | init, trySwitch, getDetectorStates, ... | PASS; `init` now calls `init_base` (was a self-recursive vt call) |
| MSOffTrafficLightLogic | init, trySwitch and phase accessors | PASS |
| MSRailSignal | trySwitch, phase accessors, addLink, init, adaptLinkInformationFrom | PASS |
| MSDelayBasedTrafficLightLogic | init, trySwitch, getDetectorStates | FIXED on this branch before the merge; main also wires it, `init` calls `init_base` |
| MSRailCrossing | init, addLink, adaptLinkInformationFrom, trySwitch, getPhaseIndexAtTime, getOffsetFromIndex, getIndexFromOffset, changeStepAndDuration | MAIN |
| MSSOTLTrafficLightLogic | init, trySwitch | MAIN (MS_SOTL_TRAFFIC_LIGHT_LOGIC_VT); `init` calls `init_base` |
| MSSOTLHiLevelTrafficLightLogic | init | MAIN (only reached through Swarm / Deterministic tables) |
| MSSwarmTrafficLightLogic | init | MAIN |
| MSDeterministicHiLevelTrafficLightLogic | init | MAIN |
| MSSOTLPhase / PolicyBased / Wave | no own overrides; inherit SOTL table | MAIN |
| NEMALogic | init, trySwitch, getCurrentPhaseDef, resetLastSwitch, getDetectorStates | MAIN (NEMA_LOGIC_VT) |
| MSRailSignal (ext table) | getRivalVehicles, getPriorityVehicles, getBlockingDriveWays, getRequestedDriveWay | FIXED: base queries went to the base no-op (libsumo `trafficlight.getRivalVehicles`). Now `MSTrafficLightLogicExtVt` via `ext_vt` (base field, default installed in `new`) |
| MSDelayBasedTrafficLightLogic (ext table) | getDetectorState, getTLQueueLength | FIXED: GLOSA's `get_tl_queue_length` on the base type ignored the override |
| MSActuatedTrafficLightLogic | getMinDur, getMaxDur, getEarliestEnd, getLatestEnd, getDetectorState, activate/deactivateProgram, loadExtraState, get/setParameter | GAP, tl01 owns: overrides exist as inherent methods, not in any table |
| MSSimpleTrafficLightLogic, MSRailSignal, NEMALogic | get/setParameter | GAP, not fixed: `Parameterised` has no table, so base `set_parameter`/`get_parameter` do not reach overrides |
| NEMALogic | activateProgram, deactivateProgram | GAP, not fixed: needs base table slots (same pattern as the rows above) |

## Routers (`SUMOAbstractRouter` slots: `myCompute`, `myProhibit`, `mySupportsProhibitions`, `myReset`, `mySetBulkMode`)

| Class | Overrides | Status |
|---|---|---|
| DijkstraRouter | compute | FIXED: the three MARouter constructions never installed compute (base compute panicked); now `.with_compute()` |
| CHRouter | compute, prohibit, supportsProhibitions, reset | FIXED: reset slot added (`ch_reset`); `clone()` now installs its slots (cloned routers panicked on compute) |
| CHRouterWrapper | compute, prohibit, supportsProhibitions | FIXED: `clone()` installs slots |
| AStarRouter | compute | FIXED: `clone()` installs compute |
| RailwayRouter | compute, setBulkMode | FIXED: setBulkMode slot added (`railway_set_bulk_mode`) |
| IntermodalRouter, PedestrianRouter | none virtual on base | explicit base calls renamed to `set_bulk_mode_base` |
| CCHRouter | reset, setBulkMode | GAP, not wired: no slot installed (`CCHRouter::new` has no callers outside its file). Explicit base call renamed to `set_bulk_mode_base`, so a future slot cannot recurse |
| AFRouter | reset, setBulkMode | GAP, not wired: no slot installed (no constructor callers outside its file) |
| Node2EdgeRouter (derives AStarRouter) | reset, setBulkMode | GAP, not wired: `Node2EdgeRouter::new` never installs compute, reached only from AF builder |
| MARouter `router->reset(&defaultVehicle)` | CH reset via base | GAP: MARouter only builds Dijkstra (CH/astar/CCH panic with shortcut), so no-op is correct today |

## Lane change models (`MSAbstractLaneChangeModelVt`)

| Class | Overrides | Status |
|---|---|---|
| MSLCM_LC2013 | inform, patchSpeed, changed, prepareStep, wantsChange, computeSpeedLat, getSafetyFactor, saveBlockerLength | PASS |
| MSLCM_LC2013_CC | none beyond LC2013 | MAIN (own table) |
| MSLCM_SL2015 | own table | PASS |
| MSLCM_DK2008 | wantsChange, inform, patchSpeed, changed, prepareStep | MAIN (the same five entries were missing on the pre-merge branch; I added them and main has its own) |

## SUMOVehicle / SUMOTrafficObject (`SUMOTrafficObjectVt`, `SUMOVehicleVt`)

| Table | Status |
|---|---|
| MS_VEHICLE_VT (MSVehicle) | PASS: every field of the struct is set; no default fallthrough, no panicking default |
| MS_VEHICLE_SUMO_VEHICLE_VT (MSVehicle) | PASS: every field set |
| ME_VEHICLE_VT (MEVehicle, owned by mc02) | GAP, report only: 17 entries fall through to `DEFAULT_SUMO_TRAFFIC_OBJECT_VT` panics: get_parameter, get_numerical_id, get_random_seed, get_vehicle_type, get_v_type_parameter, replace_vehicle_type, get_rng, get_rng_index, is_stopped, get_route_position, get_upcoming_edge_i_ds, replace_route, get_routing_mode, set_routing_mode, get_previous_speed, is_selected, get_device. C++ may inherit some non-virtually from MSBaseVehicle; mc02 to confirm each |
| ME_SUMO_VEHICLE_VT (MEVehicle, owned by mc02) | Partial: 5 overrides; the rest default. Not compared (mc02) |
| MSPerson (MS_PERSON_VT) | Not compared (ls07 owns it) |

## Route handlers (`SUMORouteHandlerVt`)

| Table | Status |
|---|---|
| MS_ROUTE_HANDLER_VT (MSRouteHandler) | PASS: all 25 slots set; closeVehicle/addTransportable/getVTypeParameter overrides present |
| MS_STATE_HANDLER_VT (MSStateHandler) | PASS: its only override in C++ is closeVehicle; inherits the MS table |
| RO_ROUTE_HANDLER_VT (RORouteHandler) | PASS: all 25 slots; addTransportable stays on the base no-op, which matches C++ `SUMORouteHandler::addTransportable` (empty body). checkLastDepart override is an inherent method, and no base-class code calls it, so no dispatch gap |

## NBTrafficLightDefinition (`NBTrafficLightDefinitionVt`)

| Class | Overrides | Status |
|---|---|---|
| NBOwnTLDef | remapRemoved, replaceRemoved, collectLinks, myCompute, initNeedsContRelation, getLayout | FIXED: initNeedsContRelation and getLayout added to the vt; the rest were already in it |
| NBLoadedTLDef | remapRemoved, replaceRemoved, collectLinks, myCompute, initNeedsContRelation, setParticipantsInformation | FIXED: initNeedsContRelation and setParticipantsInformation added |
| NBLoadedSUMOTLDef | setProgramID, remapRemoved, replaceRemoved, shiftTLConnectionLaneIndex, removeConnection, setType, extraConflict, usingSignalGroups, finalChecks, collectLinks, collectEdges, myCompute, amInvalid, initNeedsContRelation, getMaxValidIndex | FIXED: 11 non-pure virtuals added to the vt. Before this, base calls ignored these overrides (bug class 1) |

The 13 new slots (`set_program_id`, `shift_tl_connection_lane_index`, `remove_connection`, `set_type`, `extra_conflict`, `using_signal_groups`, `final_checks`, `collect_edges`, `am_invalid`, `init_needs_cont_relation`, `get_max_valid_index`, `set_participants_information`, `get_layout`) default to the `*_base` bodies, so behaviour is unchanged for classes that do not override them. Subclass bodies that call the base explicitly now use the `*_base` names.

## Pass 2 (pv01): detector outputs, stopping places, devices, router hooks

| Family | Overrides checked | Status |
|---|---|---|
| MSDetectorFileOutput | clearState: MSE2, MSE3, MSInductLoop, MSRouteProbe | FIXED: `clear_state` slot added to MSDetectorFileOutputVt. MSDetectorControl::clearState reached only the base no-op before |
| MSMeanData | detectorUpdate: MSMeanData_Harmonoise | FIXED: `detector_update` slot in MSMeanDataVt; base body is `detector_update_base`. Harmonoise was unreachable from both MSDetectorControl::updateDetectors and the detector vt |
| MSMeanData | resetOnly, Net, Amitran | NOTE: C++ resetOnly is non-virtual and Net/Amitran declare it without a definition. The table entry is base-only, which matches the C++ call path |
| MSStoppingPlace | enter, leaveFrom, getLastFreePos, accepts: MSParkingArea | FIXED: MSStoppingPlaceVt; MSParkingArea installs its own table. get_stopping_position and the rerouters (alternative.get_last_free_pos, cand.accepts) now dispatch |
| MSStoppingPlace | finishedLoading, addAccess | GAP (report only): C++ overrides exist only in GUIBusStop (GUI, not ported). The base body is the port of the non-GUI behaviour |
| MSStoppingPlace | MSChargingStation, MSOverheadWire | PASS: no C++ override, so the base table is correct |
| MSDevice | getParameter / setParameter / generateOutput | PASS: dispatch by deviceName covers every C++ override (13 get, 12 set, 11 generateOutput). deviceName literals match the match arms |
| MSDevice | saveState / loadState | GAP (sl02 owns state save): dispatched as inherent methods, not by name. Not changed here |
| SUMOAbstractRouter | supportsProhibitions: CHRouter, CHRouterWrapper return false; CCHRouter returns true | FIXED: `mySupportsProhibitions` hook. The base returned true for every router, so ro_route_def took the prohibition path for CH |
| SUMOAbstractRouter | reset, clone, compute, prohibit, setBulkMode | NOT RE-AUDITED in this pass: they dispatch through the myCompute / myProhibit / myOperation hooks; per-router coverage was not checked |

## Not audited in this pass (open)

| Family | Note |
|---|---|
| MSLaneChanger / MSDevice_* not built for vehicles | Only tripinfo and FCD are built in `init_devices`; the rest are unreachable until that shortcut is lifted. |
| ms_base_vehicle `laneChangeModel.` get | Now delegates to the micro vehicle. The C++ try/catch that turns an InvalidArgument into an error message is not mirrored, because the Rust `get_parameter` has no error channel. |
| CACC vtype TraCI probe | No case probes `getParameter(carFollowModel.*)` or `setParameter` on CACC/CC. `cases/vtypes` runs CACC but does not call them. A probe needs a new scenario plus reference regeneration from pristine. |
