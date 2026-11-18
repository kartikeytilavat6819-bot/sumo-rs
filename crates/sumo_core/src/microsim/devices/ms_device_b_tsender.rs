//! Port of microsim/devices/MSDevice_BTsender.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::RefCell;

/// A BT sender
/// C++ class `MSDevice_BTsender` (microsim/devices/MSDevice_BTsender.h)
pub struct MSDevice_BTsender {
}

// C++ static members of MSDevice_BTsender (port as module-level statics, see AGENTS.md):
//   static int MSDevice_BTsender::sVehicles;

thread_local! {
    /// C++ `static std::map<std::string, MSDevice_BTsender::VehicleInformation*> sVehicles`
    pub(crate) static S_VEHICLES: RefCell<BTreeMap<String, Ptr<MSDevice_BTsender_VehicleInformation>>> = const { RefCell::new(BTreeMap::new()) };
}

fn vehicle_info(id: &str) -> Ptr<MSDevice_BTsender_VehicleInformation> {
    S_VEHICLES.with(|m| *m.borrow().get(id).expect("btsender: no VehicleInformation for vehicle"))
}

fn has_vehicle_info(id: &str) -> bool {
    S_VEHICLES.with(|m| m.borrow().contains_key(id))
}

impl MSDevice_BTsender {
    /// C++: `void MSDevice_BTsender::cleanup()` [static]
    /// removes remaining vehicleInformation in sVehicles
    // void
    // MSDevice_BTsender::cleanup() {
    //     std::map<std::string, MSDevice_BTsender::VehicleInformation*>::iterator i;
    //     for (i = sVehicles.begin(); i != sVehicles.end(); i++) {
    //         delete i->second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        S_VEHICLES.with(|m| {
            for (_, vi) in m.borrow().iter() {
                drop(unsafe { Box::from_raw((*vi).as_raw()) });
            }
            // C++ leaves the freed entries in the map; cleared here so later lookups cannot touch freed memory
            m.borrow_mut().clear();
        });
    }

    /// C++: `std::string MSDevice_BTsender::getLocation(const SUMOTrafficObject &)` [static]
    /// return either lane or edge id (depending on availability)
    // std::string
    // MSDevice_BTsender::getLocation(const SUMOTrafficObject& o) {
    //     return o.getLane() == nullptr ? o.getEdge()->getID() : o.getLane()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_location(o: &SUMOTrafficObject) -> String {
        if o.get_lane().is_null() {
            o.get_edge().m().get_id().to_string()
        } else {
            o.get_lane().m().get_id().to_string()
        }
    }

    /// C++: `void MSDevice_BTsender::~MSDevice_BTsender() noexcept`
    /// Destructor.
    // MSDevice_BTsender::~MSDevice_BTsender() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool MSDevice_BTsender::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)`
    /// Adds the vehicle to running vehicles if it (re-) enters the network
    // bool
    // MSDevice_BTsender::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (reason == MSMoveReminder::NOTIFICATION_DEPARTED && sVehicles.find(veh.getID()) == sVehicles.end()) {
    //         sVehicles[veh.getID()] = new VehicleInformation(veh.getID());
    //         sVehicles[veh.getID()]->route.push_back(veh.getEdge());
    //     }
    //     if (reason == MSMoveReminder::NOTIFICATION_TELEPORT && sVehicles.find(veh.getID()) != sVehicles.end()) {
    //         sVehicles[veh.getID()]->amOnNet = true;
    //     }
    //     if (reason == MSMoveReminder::NOTIFICATION_TELEPORT || reason == MSMoveReminder::NOTIFICATION_JUNCTION) {
    //         sVehicles[veh.getID()]->route.push_back(veh.getEdge());
    //     }
    //     sVehicles[veh.getID()]->updates.push_back(VehicleState(veh.getSpeed(), veh.getPosition(), getLocation(veh), veh.getPositionOnLane(), veh.getRoutePosition()));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        let id: String = veh.get_id().to_string();
        if reason == MSMoveReminder_Notification::NOTIFICATION_DEPARTED && !has_vehicle_info(&id) {
            let vi: &mut MSDevice_BTsender_VehicleInformation = Box::leak(Box::new(MSDevice_BTsender_VehicleInformation::new(&id)));
            S_VEHICLES.with(|m| m.borrow_mut().insert(id.clone(), Ptr::from_mut(vi)));
            vehicle_info(&id).m().route.push(veh.get_edge());
        }
        if reason == MSMoveReminder_Notification::NOTIFICATION_TELEPORT && has_vehicle_info(&id) {
            vehicle_info(&id).m().amOnNet = true;
        }
        if reason == MSMoveReminder_Notification::NOTIFICATION_TELEPORT || reason == MSMoveReminder_Notification::NOTIFICATION_JUNCTION {
            vehicle_info(&id).m().route.push(veh.get_edge());
        }
        let location: String = Self::get_location(veh);
        vehicle_info(&id).m().updates.push(MSDevice_BTsender_VehicleState::new(veh.get_speed(), &veh.get_position(0.0), &location, veh.get_position_on_lane(), veh.get_route_position()));
        true
    }

    /// C++: `bool MSDevice_BTsender::notifyMove(SUMOTrafficObject &, double, double, double)`
    /// Checks whether the reminder still has to be notified about the vehicle moves
    // bool
    // MSDevice_BTsender::notifyMove(SUMOTrafficObject& veh, double /* oldPos */, double newPos, double newSpeed) {
    //     if (sVehicles.find(veh.getID()) == sVehicles.end()) {
    //         WRITE_WARNINGF(TL("btsender: Can not update position of vehicle '%' which is not on the road."), veh.getID());
    //         return true;
    //     }
    //     sVehicles[veh.getID()]->updates.push_back(VehicleState(newSpeed, veh.getPosition(), getLocation(veh), newPos, veh.getRoutePosition()));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        let id: String = veh.get_id().to_string();
        if !has_vehicle_info(&id) {
            utils::common::msg_handler::write_warning(&utils::tlf!("btsender: Can not update position of vehicle '%' which is not on the road.", veh.get_id()));
            return true;
        }
        let location: String = Self::get_location(veh);
        vehicle_info(&id).m().updates.push(MSDevice_BTsender_VehicleState::new(newSpeed, &veh.get_position(0.0), &location, newPos, veh.get_route_position()));
        true
    }

    /// C++: `bool MSDevice_BTsender::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)`
    /// Moves (the known) vehicle from running to arrived vehicles' list
    // bool
    // MSDevice_BTsender::notifyLeave(SUMOTrafficObject& veh, double /* lastPos */, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (reason < MSMoveReminder::NOTIFICATION_TELEPORT) {
    //         return true;
    //     }
    //     if (sVehicles.find(veh.getID()) == sVehicles.end()) {
    //         WRITE_WARNINGF(TL("btsender: Can not update position of vehicle '%' which is not on the road."), veh.getID());
    //         return true;
    //     }
    //     sVehicles[veh.getID()]->updates.push_back(VehicleState(veh.getSpeed(), veh.getPosition(), getLocation(veh), veh.getPositionOnLane(), veh.getRoutePosition()));
    //     if (reason >= MSMoveReminder::NOTIFICATION_TELEPORT) {
    //         sVehicles[veh.getID()]->amOnNet = false;
    //     }
    //     if (reason >= MSMoveReminder::NOTIFICATION_ARRIVED) {
    //         sVehicles[veh.getID()]->amOnNet = false;
    //         sVehicles[veh.getID()]->haveArrived = true;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if reason < MSMoveReminder_Notification::NOTIFICATION_TELEPORT {
            return true;
        }
        let id: String = veh.get_id().to_string();
        if !has_vehicle_info(&id) {
            utils::common::msg_handler::write_warning(&utils::tlf!("btsender: Can not update position of vehicle '%' which is not on the road.", veh.get_id()));
            return true;
        }
        let location: String = Self::get_location(veh);
        vehicle_info(&id).m().updates.push(MSDevice_BTsender_VehicleState::new(veh.get_speed(), &veh.get_position(0.0), &location, veh.get_position_on_lane(), veh.get_route_position()));
        if reason >= MSMoveReminder_Notification::NOTIFICATION_TELEPORT {
            vehicle_info(&id).m().amOnNet = false;
        }
        if reason >= MSMoveReminder_Notification::NOTIFICATION_ARRIVED {
            vehicle_info(&id).m().amOnNet = false;
            vehicle_info(&id).m().haveArrived = true;
        }
        true
    }

    /// C++: `void MSDevice_BTsender::MSDevice_BTsender()` [protected]
    /// Constructor
    // MSDevice_BTsender() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSDevice_BTsender {}
    }

    /// C++: `void MSDevice_BTsender::MSDevice_BTsender(const MSDevice_BTsender &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_BTsender) -> Self {
        // C++ copy constructor is deleted ("Invalidated"), copying is a programming error
        panic!("MSDevice_BTsender copy constructor is invalidated in C++ and must not be used");
    }

    /// C++: `MSDevice_BTsender & MSDevice_BTsender::operator=(const MSDevice_BTsender &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_BTsender) -> &mut MSDevice_BTsender {
        // C++ assignment operator is deleted ("Invalidated"), assignment is a programming error
        panic!("MSDevice_BTsender assignment operator is invalidated in C++ and must not be used");
    }

}

/// C++ class `MSTransportableDevice_BTsender` (microsim/devices/MSDevice_BTsender.h)
#[repr(C)]
pub struct MSTransportableDevice_BTsender {
    /// C++ base class `MSDevice_BTsender`
    pub base: MSDevice_BTsender,
    /// C++ base class `MSTransportableDevice`
    pub base2: MSTransportableDevice,
}

impl std::ops::Deref for MSTransportableDevice_BTsender {
    type Target = MSDevice_BTsender;
    fn deref(&self) -> &MSDevice_BTsender {
        &self.base
    }
}
impl std::ops::DerefMut for MSTransportableDevice_BTsender {
    fn deref_mut(&mut self) -> &mut MSDevice_BTsender {
        &mut self.base
    }
}

impl MSTransportableDevice_BTsender {
    /// C++: `void MSTransportableDevice_BTsender::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_BTsender-options
    // void
    // MSTransportableDevice_BTsender::insertOptions(OptionsCont& oc) {
    //     insertDefaultAssignmentOptions("btsender", "Communication", oc, true);
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        MSDevice::insert_default_assignment_options("btsender", "Communication", oc, true);
    }

    /// C++: `void MSTransportableDevice_BTsender::buildDevices(MSTransportable &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_devices(t: &mut MSTransportable, into: &mut Vec<Ptr<MSTransportableDevice>>) {
        let equipped = utils::options::options_cont::with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(oc, "btsender", t.get_id(), t.get_parameter(), t.get_vehicle_type().get_parameter(), false, true)
        });
        if equipped {
            let id = format!("btsender_{}", t.get_id());
            let device = Box::leak(Box::new(MSTransportableDevice_BTsender::new(t, &id)));
            into.push(Ptr::from_mut(&mut device.base2));
            MSDevice_BTreceiver::has_sending_persons();
        }
    }

    /// C++: `const std::string MSTransportableDevice_BTsender::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "btsender";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "btsender".to_string()
    }

    /// C++: `bool MSTransportableDevice_BTsender::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    // bool notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTsender::notifyEnter(veh, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        self.base.notify_enter(veh, reason, enteredLane)
    }

    /// C++: `bool MSTransportableDevice_BTsender::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    // bool notifyMove(SUMOTrafficObject& veh, double oldPos, double newPos, double newSpeed) {
    //         return MSDevice_BTsender::notifyMove(veh, oldPos, newPos, newSpeed);
    //     }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        self.base.notify_move(veh, oldPos, newPos, newSpeed)
    }

    /// C++: `bool MSTransportableDevice_BTsender::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    // bool notifyLeave(SUMOTrafficObject& veh, double lastPos, MSMoveReminder::Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTsender::notifyLeave(veh, lastPos, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        self.base.notify_leave(veh, lastPos, reason, enteredLane)
    }

    /// C++: `void MSTransportableDevice_BTsender::MSTransportableDevice_BTsender(MSTransportable &, const std::string &)` [private]
    /// Constructor
    // MSTransportableDevice_BTsender(MSTransportable& holder, const std::string& id) :
    //         MSTransportableDevice(holder, id) {
    //     }
    #[allow(unused_variables)]
    pub fn new(holder: &mut MSTransportable, id: &str) -> Self {
        MSTransportableDevice_BTsender {
            base: MSDevice_BTsender {},
            base2: MSTransportableDevice::new(holder, id),
        }
    }

}

/// C++ class `MSVehicleDevice_BTsender` (microsim/devices/MSDevice_BTsender.h)
#[repr(C)]
pub struct MSVehicleDevice_BTsender {
    /// C++ base class `MSDevice_BTsender`
    pub base: MSDevice_BTsender,
    /// C++ base class `MSVehicleDevice`
    pub base2: MSVehicleDevice,
}

impl std::ops::Deref for MSVehicleDevice_BTsender {
    type Target = MSDevice_BTsender;
    fn deref(&self) -> &MSDevice_BTsender {
        &self.base
    }
}
impl std::ops::DerefMut for MSVehicleDevice_BTsender {
    fn deref_mut(&mut self) -> &mut MSDevice_BTsender {
        &mut self.base
    }
}

impl MSVehicleDevice_BTsender {
    /// C++: `void MSVehicleDevice_BTsender::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_BTsender-options
    // void
    // MSVehicleDevice_BTsender::insertOptions(OptionsCont& oc) {
    //     insertDefaultAssignmentOptions("btsender", "Communication", oc);
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        MSDevice::insert_default_assignment_options("btsender", "Communication", oc, false);
    }

    /// C++: `void MSVehicleDevice_BTsender::buildVehicleDevices(SUMOVehicle &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        let equipped = utils::options::options_cont::with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(oc, "btsender", v.get_id(), v.get_parameter(), v.get_vehicle_type().get_parameter(), false, false)
        });
        if equipped {
            let id = format!("btsender_{}", v.get_id());
            let device = Box::leak(Box::new(MSVehicleDevice_BTsender::new(v, &id)));
            into.push(Ptr::from_mut(&mut device.base2));
        }
    }

    /// C++: `const std::string MSVehicleDevice_BTsender::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "btsender";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "btsender".to_string()
    }

    /// C++: `bool MSVehicleDevice_BTsender::notifyEnter(SUMOTrafficObject &, Notification, const MSLane *)` [virtual]
    // bool notifyEnter(SUMOTrafficObject& veh, Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTsender::notifyEnter(veh, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        self.base.notify_enter(veh, reason, enteredLane)
    }

    /// C++: `bool MSVehicleDevice_BTsender::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    // bool notifyMove(SUMOTrafficObject& veh, double oldPos, double newPos, double newSpeed) {
    //         return MSDevice_BTsender::notifyMove(veh, oldPos, newPos, newSpeed);
    //     }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        self.base.notify_move(veh, oldPos, newPos, newSpeed)
    }

    /// C++: `bool MSVehicleDevice_BTsender::notifyLeave(SUMOTrafficObject &, double, Notification, const MSLane *)` [virtual]
    // bool notifyLeave(SUMOTrafficObject& veh, double lastPos, Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTsender::notifyLeave(veh, lastPos, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        self.base.notify_leave(veh, lastPos, reason, enteredLane)
    }

    /// C++: `void MSVehicleDevice_BTsender::MSVehicleDevice_BTsender(SUMOVehicle &, const std::string &)` [private]
    /// Constructor
    // MSVehicleDevice_BTsender(SUMOVehicle& holder, const std::string& id) :
    //         MSVehicleDevice(holder, id) {
    //     }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str) -> Self {
        fn mr_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
            Ptr::from_mut(mr).cast::<MSVehicleDevice_BTsender>().m().notify_enter(veh, reason, enteredLane)
        }
        fn mr_notify_move(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
            Ptr::from_mut(mr).cast::<MSVehicleDevice_BTsender>().m().notify_move(veh, oldPos, newPos, newSpeed)
        }
        fn mr_notify_leave(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
            Ptr::from_mut(mr).cast::<MSVehicleDevice_BTsender>().m().notify_leave(veh, lastPos, reason, enteredLane)
        }
        static MS_MOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
            notify_enter: mr_notify_enter,
            notify_move: mr_notify_move,
            notify_leave: mr_notify_leave,
        };
        let mut this = MSVehicleDevice_BTsender {
            base: MSDevice_BTsender {},
            base2: MSVehicleDevice::new(holder, id),
        };
        this.base2.base.vt = &MS_MOVE_REMINDER_VT;
        this
    }

}

/// Stores the information of a vehicle
/// C++ class `MSDevice_BTsender::VehicleInformation` (microsim/devices/MSDevice_BTsender.h)
#[repr(C)]
pub struct MSDevice_BTsender_VehicleInformation {
    /// C++ base class `Named`
    pub base: Named,
    /// List of position updates during last step
    pub updates: Vec<MSDevice_BTsender_VehicleState>,
    /// Whether the vehicle is within the simulated network
    pub amOnNet: bool,
    /// Whether the vehicle was removed from the simulation
    pub haveArrived: bool,
    /// List of edges travelled
    pub route: ConstMSEdgeVector,
}

impl std::ops::Deref for MSDevice_BTsender_VehicleInformation {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_BTsender_VehicleInformation {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl MSDevice_BTsender_VehicleInformation {
    /// C++: `void VehicleInformation::VehicleInformation(const std::string &)`
    /// Constructor
    // VehicleInformation(const std::string& id) : Named(id), amOnNet(true), haveArrived(false)  {}
    #[allow(unused_variables)]
    pub fn new(id: &str) -> Self {
        MSDevice_BTsender_VehicleInformation {
            base: Named::new(id),
            updates: Vec::new(),
            amOnNet: true,
            haveArrived: false,
            route: Vec::new(),
        }
    }

    /// C++: `void VehicleInformation::~VehicleInformation() noexcept` [virtual]
    /// Destructor
    // virtual ~VehicleInformation() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `Boundary VehicleInformation::getBoxBoundary() const` [const]
    /// Returns the boundary of passed positions
    // Boundary getBoxBoundary() const {
    //             Boundary ret;
    //             for (std::vector<VehicleState>::const_iterator i = updates.begin(); i != updates.end(); ++i) {
    //                 ret.add((*i).position);
    //             }
    //             return ret;
    //         }
    #[allow(unused_variables)]
    pub fn get_box_boundary(&self) -> Boundary {
        let mut ret = Boundary::default();
        for i in self.updates.iter() {
            ret.add_pos(&i.position);
        }
        ret
    }

}

/// A single movement state of the vehicle
/// C++ class `MSDevice_BTsender::VehicleState` (microsim/devices/MSDevice_BTsender.h)
pub struct MSDevice_BTsender_VehicleState {
    /// The speed of the vehicle
    pub speed: f64,
    /// The position of the vehicle
    pub position: Position,
    /// The lane the vehicle was at
    pub laneID: String,
    /// The position at the lane of the vehicle
    pub lanePos: f64,
    /// The position in the route of the vehicle
    pub routePos: i32,
}

impl MSDevice_BTsender_VehicleState {
    /// C++: `void VehicleState::VehicleState(const double, const Position &, const std::string &, const double, const int)`
    /// Constructor
    // VehicleState(const double _speed, const Position& _position, const std::string& _laneID,
    //                      const double _lanePos, const int _routePos)
    //             : speed(_speed), position(_position), laneID(_laneID), lanePos(_lanePos), routePos(_routePos) {}
    #[allow(unused_variables)]
    pub fn new(_speed: f64, _position: &Position, _laneID: &str, _lanePos: f64, _routePos: i32) -> Self {
        MSDevice_BTsender_VehicleState {
            speed: _speed,
            position: _position.clone(),
            laneID: _laneID.to_string(),
            lanePos: _lanePos,
            routePos: _routePos,
        }
    }

    /// C++: `void VehicleState::~VehicleState() noexcept`
    /// Destructor
    // ~VehicleState() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

}

