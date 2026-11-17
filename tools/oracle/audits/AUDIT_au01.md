# AUDIT au01: ms_vehicle.rs virtual dispatch and nullptr derefs

Scope: `crates/sumo_core/src/microsim/ms_vehicle.rs` (grep + ~80-line windows). C++ read-only at `sumo-src\src`.
Method: read both sides. No cargo run. No .rs edited.
Base: `main` at 93058c0.

## Findings, ranked by impact

| # | file:line | class | C++ ref | what Rust does | impact | exact proposed fix |
|---|---|---|---|---|---|---|
| 1 | ms_vehicle.rs:10102 | 1 (no virtual dispatch) | MSVehicle.cpp:5034-5035 (`dev->notifyParking()`, virtual); MSMoveReminder.h:197 (`virtual notifyParking() {}`); MSDevice_Battery.cpp:598-602 (override) | `dev.m().base.notify_parking()` calls the field `base: MSMoveReminder` directly. `ms_move_reminder.rs:147-149` is an empty body and `MSMoveReminder.vt` has no `notify_parking` slot. The battery override at `devices/ms_device_battery.rs:467` is never reached. | certain for any parked vehicle with a battery device: the parking charge (`notifyMove` at C++ 600) and the `myConsum = 0` reset never run. Output of battery consumption and charge diverges. | (a) add a `notify_parking` slot to the `MSMoveReminder` vtable in `ms_move_reminder.rs`, dispatching `(self.vt.notify_parking)(self)` as `notify_move` does at line 128; (b) wire `ms_device_battery.rs:467` into the battery's vtable, and make its body call `notify_move(myHolder, pos, pos, speed)` then set `myConsum = 0` as in C++ 600-601; (c) change line 10102 to `dev.m().notify_parking();`. |
| 2 | ms_vehicle.rs:2750-2751, root cause ms_base_vehicle.rs:2907-2910 | 2 (stub standing in for a lookup) | MSVehicle.cpp:1084-1085 (`getDevice(typeid(MSDevice_DriverState))`, `getDevice(typeid(MSDevice_Friction))`); MSBaseVehicle.cpp:1246-1253 (`typeid(*dev) == type` loop) | `MSBaseVehicle::get_device` is `Ptr::default()` with a shortcut. So `myDriverState` and `myFrictionDevice` are always null. The comment at line 2750 says "the first device is taken", which is false. Driver-state update at ms_vehicle.rs:3130-3132 is skipped. | certain whenever `device.driverstate` or `device.friction` is enabled: driver state and friction are never updated. Also a misleading shortcut comment. (Known, HANDOVER item 7, re-confirmed here.) | Typed key: change `get_device` to look up by `MSDevice::get_device_name(dev.base2.get_id())` (the id-prefix match already used at ms_vehicle.rs:13836), e.g. `get_device_by_name("driverstate")` / `"friction"` (names UNVERIFIED, confirm against `MSDevice::buildVehicleDevices`). Update lines 2750-2751 to use it and fix the comment. |
| 3 | ms_vehicle.rs:7106 (used at 7176, 7258) | 2 (hardcoded value standing in for a lookup) | MSVehicle.cpp:6743, 6823 (`getDevice(typeid(MSDevice_ElecHybrid)) != nullptr`) | `hasElecHybrid = !self.get_device(&()).is_null()`. `get_device` always returns null, so this is always `false`. The overhead-wire preference is never applied. | certain for vehicles with an elec-hybrid device: `updateBestLanes` continuation choice differs from C++. Not reachable without elec-hybrid devices. | Same fix as #2: `hasElecHybrid = self.get_device_by_name("elecHybrid").is_some()` (name UNVERIFIED). |
| 4 | ms_vehicle.rs:3645 | 2 (shortcut, in-file) | MSVehicle.cpp (`dynamic_cast<MSDevice_ElecHybrid*>(getDevice(typeid(...)))` at 4745 per grep; commented C++ block at Rust 3406-3421) | Elec-hybrid acceleration limit is skipped, labelled `shortcut:`. | certain for vehicles with an elec-hybrid device: acceleration is not power-capped. | Same device lookup as #2/#3, then port the block at Rust 3406-3421 (C++ MSVehicle.cpp lines are in the comment block). |
| 5 | ms_vehicle.rs:1240 | 4 (possible nullptr deref / panic) | C++ equivalent UNVERIFIED (not read) | `currentEdge.get_lanes().last().unwrap()` panics if the edge has zero lanes. Reached only when `destinationLaneIndex >= lanes.len()`. | none in practice: a valid network edge always has a lane. Would panic, not silently diverge, on a degenerate network. | Guard: `if let Some(last) = currentEdge.get_lanes().last() { if !last.get_opposite().is_null() { ... } }`. Mark UNVERIFIED until C++ is read. |
| 6 | ms_vehicle.rs:2064, 2068, 2084, 2182, 2186 | 4 (unwrap on Ptr::get) | n/a (vtable adapters) | `Ptr::get().unwrap()` on the `SUMOVehicle`/`SUMOTrafficObject` pointer handed to a vtable slot. Panics on null. | none: the vtable is only invoked on live `MSVehicle` objects, and C++ would be UB on a null `this` too. | None needed. Noted only. |

## Class-1 checks that came back clean

- `replace_route` (ms_vehicle.rs:2883): `self.base.replace_route` is a non-virtual call matching C++ `MSBaseVehicle::replaceRoute` (MSVehicle.cpp:1140). The subclass override path is wired through vtable slot `replace_route` (ms_vehicle.rs:2043). OK.
- `get_reroute_origin` (ms_vehicle.rs:4997): wired through vtable slot (2547). OK.
- `notify_move`, `notify_idle`, `notify_enter` (ms_vehicle.rs:2942, 2980, 2986, 8480, 10095, 10099): dispatch through `MSMoveReminder` vtable (`ms_move_reminder.rs:111, 128`). OK. Only `notify_parking` lacks a slot (#1).
- `changedToOpposite` (ms_vehicle.rs:3754): non-virtual in C++ (MSAbstractLaneChangeModel.h:398). Direct call is correct.
- `getDevice(typeid)` is non-virtual on MSBaseVehicle (MSBaseVehicle.h:617) but overridden on MSTransportable. Not an MSVehicle problem.

## Not verified

- The C++ counterpart of ms_vehicle.rs:1240 was not read.
- The device names "driverstate", "friction", "elecHybrid" are assumed from naming; check `MSDevice::buildVehicleDevices` before implementing #2-#4.
- The other items from the HANDOVER list (`MSLaneChanger::change`, `VehicleStateListener`, rerouter `MSMoveReminder`, `device_name` pure virtual) were not re-audited.

## Count per class (this file)

- Class 1 (virtual dispatch): 1 new (#1).
- Class 2 (stub / hardcoded standing in for lookup): 3 (#2, #3, #4).
- Class 3 (ported, never called): 0 checked.
- Class 4 (nullptr deref): 2 (#5 possible, #6 none).
