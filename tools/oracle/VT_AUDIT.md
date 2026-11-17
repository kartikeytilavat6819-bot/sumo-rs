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
| MSCFModel_CACC | getParameter, setParameter | OPEN (no vt slot; see MSBaseVehicle row) |
| MSCFModel_CC | stopSpeed, finalizeSpeed, freeSpeed, followSpeed, insertionFollowSpeed, interactionGap, getSecureGap, maxNextSpeed, minNextSpeed, getModelID, duplicate, createVehicleVariables | PASS |
| MSCFModel_CC | getParameter, setParameter | OPEN (no vt slot) |
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

## Lane change models (`MSAbstractLaneChangeModelVt`)

| Class | Overrides | Status |
|---|---|---|
| MSLCM_LC2013 | inform, patchSpeed, changed, prepareStep, wantsChange, computeSpeedLat, getSafetyFactor, saveBlockerLength | PASS |
| MSLCM_LC2013_CC | none beyond LC2013 | MAIN (own table) |
| MSLCM_SL2015 | own table | PASS |
| MSLCM_DK2008 | wantsChange, inform, patchSpeed, changed, prepareStep | MAIN (the same five entries were missing on the pre-merge branch; I added them and main has its own) |

## Not audited in this pass (open)

| Family | Note |
|---|---|
| MSCFModel getParameter / setParameter for CACC and CC | No vt slot. MSBaseVehicle::get/set carFollowModel parameters is a `shortcut:` because MSBaseVehicle cannot downcast to MSVehicle. Needs the downcast, then a vt slot. |
| SUMORouteHandler / MSRouteHandler / MSStateHandler | Tables exist; overrides not compared. |
| NBTrafficLightDefinition family | Tables exist; overrides not compared. |
| SUMOVehicle / SUMOTrafficObject (MS_VEHICLE_VT, MS_VEHICLE_SUMO_VEHICLE_VT) | Present; overrides not compared. |
| MSLaneChanger / MSDevice_* not built for vehicles | Only tripinfo and FCD are built in `init_devices`; the rest are unreachable until that shortcut is lifted. |
