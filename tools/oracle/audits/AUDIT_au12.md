# AUDIT au12: transportables, MSStage/MSStoppingPlace virtual dispatch

Scope: `crates/sumo_core/src/microsim/transportables/`, `ms_stop*.rs`, `ms_parking_area.rs`, `ms_stopping_place*.rs`.
C++ reference: `C:\Users\karti\sumo-src\src\microsim\...`. No cargo run, no `.rs` edits.
Base commit: `93058c0` (branch `worktree-agent-au12`).

## Scope of impact

- **Person / container / public transport only:** every row in the stage table (1-9). Oracle refs in `tools/oracle/cases/*/ref.stat.xml` all show `persons loaded="0"`, so no current case exercises these paths (the scan covered the refs I read, not all 19 cases).
- **Car-only default runs:** no finding in the default car path. `ms_vehicle.rs:8553` and `:9264` call `enter` / `leave_from` on `stop.parkingarea`, which is typed `Ptr<MSParkingArea>` (`ms_stop.rs:24`), so overrides dispatch correctly.
- **Car with a non-default config:** row 10 (stopping-place rerouter with parking-area alternatives).

## Findings (ranked)

Legend: **V** = verified by reading both sides. **U** = unverified.

| # | file:line | class | C++ ref | what Rust does | impact | exact fix | status |
|---|---|---|---|---|---|---|---|
| 1 | `transportables/ms_stage.rs:228-231, 237-240, 285-288, 294-297, 303-306, 581-584, 590-593, 599-602, 607-610` | 1 virtual dispatch | `MSStage.h:122,125,148,151,154,230,236,245,247` (pure virtual) | Base `MSStage` method is an inherent `panic!`. Callers hold `Ptr<MSStage>` (`ms_transportable.rs:389,395`), so the base is always reached. | **Crash on any person plan.** `ms_transportable.rs:395` `next.m().proceed(...)` panics on the first stage advance. `:639,651` (position/angle), `:718,728` (summary/description), `:929` (trip output), `:972,1745` (route output) also panic. Person-only. | Add a dispatcher on `myType` in `ms_stage.rs`. Each method: `match self.myType { WAITING_FOR_DEPART \| WAITING => Ptr::from_mut(self).cast::<MSStageWaiting>().m().X(..), WALKING \| ACCESS => ...MSStageWalking, DRIVING => ...MSStageDriving, TRIP => ...MSStageTrip, TRANSHIP => ...MSStageTranship }`. Confirm the ACCESS-to-class mapping in `MSStageWalking.cpp` before writing it. | V (crash path); ACCESS mapping U |
| 2 | `transportables/ms_transportable.rs:987, 997`; caller `ms_transportable_control.rs:518, 590` | 1 virtual dispatch | `MSStage.h:184,187` (virtual); `MSStageDriving.h:129,132` override | `is_waiting_for` and `is_waiting4_vehicle` on `Ptr<MSStage>` return the base `false`. | **Silent.** Persons never board a vehicle (`ms_transportable_control.rs:518,590`). Person-only, public-transport scenarios. | Same dispatcher as row 1, for `is_waiting_for` and `is_waiting4_vehicle`, with `DRIVING => MSStageDriving`. | V |
| 3 | `transportables/ms_transportable.rs:585` | 1 virtual dispatch | `MSStage.h:128` (virtual base returns `nullptr`); `MSStageMoving.h:63`, `MSStageDriving.h:90` override | `get_current_stage().get_lane()` returns `Ptr::null()` for moving and driving stages. | **Silent.** Lane lookup for a person in a walking or driving stage returns null. Person-only. | Dispatcher on `MSStageMoving` / `MSStageDriving` for `get_lane`. | V |
| 4 | `transportables/ms_transportable.rs:1055`; also `:286-287`, `:1262`, `:1529` | 1 virtual dispatch | `MSStage.h:197` (virtual base returns `nullptr`); `MSStageDriving.h:137` override | `get_vehicle` on `Ptr<MSStage>` always returns null. | **Silent** at `:286` and `:1055`: vehicle is never released. **Panic** at `:1529` (`assert!(!stage.get_vehicle().is_null())`) if reached for a driving stage. Person-only. | Dispatcher for `get_vehicle` (`DRIVING => MSStageDriving`). | V |
| 5 | `transportables/ms_transportable.rs:673` | 1 virtual dispatch | `MSStage.h:202` (virtual base returns 0); `MSStageMoving.h:95`, `MSStageDriving.h:144` override | `get_speed` on `Ptr<MSStage>` returns 0. | **Silent.** Person-only. | Dispatcher for `get_speed` (`WALKING`/`ACCESS` via `MSStageMoving`, `DRIVING` via `MSStageDriving`). | V |
| 6 | `transportables/ms_transportable.rs:807` | 1 virtual dispatch | `MSStage.h:208` (virtual base returns 0); `MSStageWalking.h:85` override | `get_route_position` on `Ptr<MSStage>` returns 0. | **Silent.** Person-only. | Dispatcher for `get_route_position` (`WALKING => MSStageWalking`). | V |
| 7 | `transportables/ms_transportable.rs:760, 1179`; `ms_stage_trip.rs:650` | 1 virtual dispatch | `MSStage.h:205` (virtual base returns `[destination]`); `MSStageMoving.h:69`, `MSStageDriving.h:146` override | `get_edges` on `Ptr<MSStage>` returns only `[destination]`. | **Silent.** `ms_transportable.rs:1179` (last-edge lookup) and `ms_stage_trip.rs:650` (previous-stage edges) get one edge where C++ returns the full route. Person-only. | Dispatcher for `get_edges` (moving types). | V |
| 8 | `transportables/ms_transportable.rs:616` (`get_current_stage().get_edge_pos(now)`) | 1 virtual dispatch | `MSStage.h:115` (virtual); overrides `MSStageMoving.h:72`, `MSStageDriving.h:85`, `MSStageTrip.h:66` | Base `get_edge_pos` is in `ms_stage.rs` and I did not read that body. | Probably **silent** for moving, driving and trip stages. Person-only. | Read `ms_stage.rs` `get_edge_pos`. Add dispatch for `MOVING`, `DRIVING`, `TRIP`. | U |
| 10 | `trigger/ms_stopping_place_rerouter.rs:898, 963` (`alternative.get_last_free_pos(...)`) | 1 virtual dispatch | `MSStoppingPlace.h:158` (virtual); `MSParkingArea.h:225` override | `alternative` is `Ptr<MSStoppingPlace>` (`:883`). Calls `get_last_free_pos` on the base, so the parking-area override is skipped whenever the alternative is a parking area. | **Silent wrong stop end position** for parking-area alternatives in the stopping-place rerouter. This is a car scenario, but only with a rerouter and parking-area alternatives. | Add `fn stop_last_free_pos(sp: Ptr<MSStoppingPlace>, veh, brake) -> f64` in `ms_stopping_place.rs`: if the stop's element tag is `SUMO_TAG_PARKING_AREA`, cast to `MSParkingArea` and call its override; else the base. Use it at `:898` and `:963`. Check that `MSStoppingPlace` stores its element tag (C++ ctor takes a `SumoXMLTag`). | V (mechanism); tag field U |
| 11 | `ms_stopping_place.rs:444-449` (`get_stopping_position`) | 1 virtual dispatch | `MSStoppingPlace.h:202` (`getLastFreePos()`, non-virtual, calls the virtual `getLastFreePos(veh)`) | `self.get_last_free_pos` on `&MSStoppingPlace`, so a parking-area `self` gets the base. | **None today:** no caller found (grep for `get_stopping_position(` returns only the definition). Latent. | Delete if still unused, or apply the row-10 helper. | V |
| 12 | `ms_stage_driving.rs:663, 1200, 1343` (`myOriginStop` / `myDestinationStop.get_lane()`) | 4 nullptr | `MSStageDriving.cpp` (guard not checked) | `get_lane()` on a possibly-null stop, with no guard at the call site I read. | If the C++ path is guarded and the Rust is not: **panic** in a stop-less driving stage. Person-only. | Read `MSStageDriving.cpp` around the matching lines. If C++ guards, add `if !stop.is_null()` before `get_lane()`. | U |
| 13 | `ms_person.rs:326-345` (`prevStop.get_lane()`) | 4 nullptr | `MSPerson.cpp` (guard not checked) | `prevStop.get_lane()` unguarded in the train/tranship exit path. | Same as row 12. Person-only, train scenarios. | Check the C++ guard in `MSPerson.cpp` and mirror it. | U |
| 14 | `ms_transportable.rs:1526, 1531` (`orig.get_lane()`, `replacement.get_lane()`) | 4 nullptr | `MSTransportable.cpp` (guard not checked) | Unguarded `get_lane()` on route-replacement stops. | Person-only, route replacement. | Check the C++ guard and mirror it. | U |
| 15 | `trigger/ms_triggered_rerouter.rs:1860, 2220` (`Ptr::from_raw(... as *mut MSParkingArea)`) | 1 / 4 (cast) | `MSTriggeredRerouter.cpp` (`dynamic_cast` or type check not verified) | Raw pointer cast from `MSStoppingPlace` to `MSParkingArea` without a type check. | If the id is not a parking area: **wrong-type call**. Rerouter, outside scope. | Verify the type with the element tag before the cast (see row 10). | U |

## Verified OK (no finding)

- `ms_stage.rs:219-221` `get_direction` returns 0. C++ `MSPModel::UNDEFINED_DIRECTION(0)`, `MSPModel.cpp:41`. Correct.
- `ms_stage.rs` `ARRIVALPOS_UNSPECIFIED = inf`, `ROADSIDE_OFFSET = 3`. Match `MSStage.cpp:38-39`.
- `MSStageType` values 0-6 match `MSStage.h:56-62`.
- `ms_stopping_place.rs:129` `myParkingFactor`: `parkingLength <= 0 ? 1 : (endPos-begPos)/parkingLength`. Matches `MSStoppingPlace.cpp:58`.
- `ms_stopping_place.rs:293-343` `get_last_free_pos` (base) follows the C++ body. The sort uses the same pair key.
- `ms_stop.rs:163-` `get_end_pos`: `busstop` / `containerstop` / `chargingStation` / `overheadWireSegment` are base-typed, and C++ has no non-GUI override of `getLastFreePos` for them (only `GUIBusStop` overrides `finishedLoading` / `addAccess`, GUI-only). Equivalent.
- `ms_stopping_place.rs:350` `accepts` returns true. Current callers are typed `Ptr<MSParkingArea>` (`ms_device_station_finder.rs:1186`, `ms_charging_station.rs:374`).
- `ms_vehicle.rs:8553, 9264` `parkingarea` calls are typed `Ptr<MSParkingArea>`, so overrides dispatch correctly.

## Class 3 (ported, never called)

- `MSStoppingPlace::finishedLoading` is called from `netload/nl_trigger_builder.rs:747`. Connected.
- `get_stopping_position` (row 11): no caller.
- Pedestrian model init (`MSPModel` build / `MSNet` wiring) was **not checked** (out of time budget). Unverified.

## Class 2 (stale globals / hardcoded constants)

- No stale global found in scope. The `ms_stage.rs` hardcodes match C++ (see Verified OK).
- `ms_stage.rs:220` comment says the 0 is `UNDEFINED_DIRECTION`. That is correct but should use `MSPMODEL_UNDEFINED_DIRECTION` (defined `msp_model.rs:10`) to avoid drift.

## Summary

- 14 findings: 9 verified (rows 1-7, 10, 11), 5 unverified (rows 8, 12-15). Row 1 also has an unverified ACCESS mapping.
- Rows 1-8 are person-only. Row 10 needs a rerouter with parking-area alternatives. Row 11 is latent. Rows 12-15 are unverified.
- Pedestrian model init was not checked.
