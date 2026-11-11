//! Port of microsim/devices/MSDevice_BTsender.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A BT sender
/// C++ class `MSDevice_BTsender` (microsim/devices/MSDevice_BTsender.h)
pub struct MSDevice_BTsender {
}

// C++ static members of MSDevice_BTsender (port as module-level statics, see AGENTS.md):
//   static int MSDevice_BTsender::sVehicles;

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
        todo!("port MSDevice_BTsender::cleanup")
    }

    /// C++: `std::string MSDevice_BTsender::getLocation(const SUMOTrafficObject &)` [static]
    /// return either lane or edge id (depending on availability)
    // std::string
    // MSDevice_BTsender::getLocation(const SUMOTrafficObject& o) {
    //     return o.getLane() == nullptr ? o.getEdge()->getID() : o.getLane()->getID();
    // }
    #[allow(unused_variables)]
    pub fn get_location(o: &SUMOTrafficObject) -> String {
        todo!("port MSDevice_BTsender::getLocation")
    }

    /// C++: `void MSDevice_BTsender::~MSDevice_BTsender() noexcept`
    /// Destructor.
    // MSDevice_BTsender::~MSDevice_BTsender() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSDevice_BTsender::~MSDevice_BTsender")
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
        todo!("port MSDevice_BTsender::notifyEnter")
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
        todo!("port MSDevice_BTsender::notifyMove")
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
        todo!("port MSDevice_BTsender::notifyLeave")
    }

    /// C++: `void MSDevice_BTsender::MSDevice_BTsender()` [protected]
    /// Constructor
    // MSDevice_BTsender() { }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port MSDevice_BTsender::MSDevice_BTsender")
    }

    /// C++: `void MSDevice_BTsender::MSDevice_BTsender(const MSDevice_BTsender &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_BTsender) -> Self {
        todo!("port MSDevice_BTsender::MSDevice_BTsender")
    }

    /// C++: `MSDevice_BTsender & MSDevice_BTsender::operator=(const MSDevice_BTsender &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_BTsender) -> &mut MSDevice_BTsender {
        todo!("port MSDevice_BTsender::operator=")
    }

}

/// C++ class `MSTransportableDevice_BTsender` (microsim/devices/MSDevice_BTsender.h)
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
        todo!("port MSTransportableDevice_BTsender::insertOptions")
    }

    /// C++: `void MSTransportableDevice_BTsender::buildDevices(MSTransportable &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_devices(t: &mut MSTransportable, into: &mut i32) {
        todo!("port MSTransportableDevice_BTsender::buildDevices")
    }

    /// C++: `const std::string MSTransportableDevice_BTsender::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "btsender";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSTransportableDevice_BTsender::deviceName")
    }

    /// C++: `bool MSTransportableDevice_BTsender::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    // bool notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTsender::notifyEnter(veh, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSTransportableDevice_BTsender::notifyEnter")
    }

    /// C++: `bool MSTransportableDevice_BTsender::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    // bool notifyMove(SUMOTrafficObject& veh, double oldPos, double newPos, double newSpeed) {
    //         return MSDevice_BTsender::notifyMove(veh, oldPos, newPos, newSpeed);
    //     }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        todo!("port MSTransportableDevice_BTsender::notifyMove")
    }

    /// C++: `bool MSTransportableDevice_BTsender::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    // bool notifyLeave(SUMOTrafficObject& veh, double lastPos, MSMoveReminder::Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTsender::notifyLeave(veh, lastPos, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSTransportableDevice_BTsender::notifyLeave")
    }

    /// C++: `void MSTransportableDevice_BTsender::MSTransportableDevice_BTsender(MSTransportable &, const std::string &)` [private]
    /// Constructor
    // MSTransportableDevice_BTsender(MSTransportable& holder, const std::string& id) :
    //         MSTransportableDevice(holder, id) {
    //     }
    #[allow(unused_variables)]
    pub fn new(holder: &mut MSTransportable, id: &str) -> Self {
        todo!("port MSTransportableDevice_BTsender::MSTransportableDevice_BTsender")
    }

}

/// C++ class `MSVehicleDevice_BTsender` (microsim/devices/MSDevice_BTsender.h)
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
        todo!("port MSVehicleDevice_BTsender::insertOptions")
    }

    /// C++: `void MSVehicleDevice_BTsender::buildVehicleDevices(SUMOVehicle &, int &)` [static]
    /// Build devices for the given vehicle, if needed
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut i32) {
        todo!("port MSVehicleDevice_BTsender::buildVehicleDevices")
    }

    /// C++: `const std::string MSVehicleDevice_BTsender::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "btsender";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSVehicleDevice_BTsender::deviceName")
    }

    /// C++: `bool MSVehicleDevice_BTsender::notifyEnter(SUMOTrafficObject &, Notification, const MSLane *)` [virtual]
    // bool notifyEnter(SUMOTrafficObject& veh, Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTsender::notifyEnter(veh, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSVehicleDevice_BTsender::notifyEnter")
    }

    /// C++: `bool MSVehicleDevice_BTsender::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    // bool notifyMove(SUMOTrafficObject& veh, double oldPos, double newPos, double newSpeed) {
    //         return MSDevice_BTsender::notifyMove(veh, oldPos, newPos, newSpeed);
    //     }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        todo!("port MSVehicleDevice_BTsender::notifyMove")
    }

    /// C++: `bool MSVehicleDevice_BTsender::notifyLeave(SUMOTrafficObject &, double, Notification, const MSLane *)` [virtual]
    // bool notifyLeave(SUMOTrafficObject& veh, double lastPos, Notification reason, const MSLane* enteredLane = 0) {
    //         return MSDevice_BTsender::notifyLeave(veh, lastPos, reason, enteredLane);
    //     }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        todo!("port MSVehicleDevice_BTsender::notifyLeave")
    }

    /// C++: `void MSVehicleDevice_BTsender::MSVehicleDevice_BTsender(SUMOVehicle &, const std::string &)` [private]
    /// Constructor
    // MSVehicleDevice_BTsender(SUMOVehicle& holder, const std::string& id) :
    //         MSVehicleDevice(holder, id) {
    //     }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str) -> Self {
        todo!("port MSVehicleDevice_BTsender::MSVehicleDevice_BTsender")
    }

}

/// Stores the information of a vehicle
/// C++ class `MSDevice_BTsender::VehicleInformation` (microsim/devices/MSDevice_BTsender.h)
pub struct MSDevice_BTsender_VehicleInformation {
    /// C++ base class `Named`
    pub base: Named,
    /// List of position updates during last step
    pub updates: i32,
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
        todo!("port VehicleInformation::VehicleInformation")
    }

    /// C++: `void VehicleInformation::~VehicleInformation() noexcept` [virtual]
    /// Destructor
    // virtual ~VehicleInformation() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port VehicleInformation::~VehicleInformation")
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
        todo!("port VehicleInformation::getBoxBoundary")
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
        todo!("port VehicleState::VehicleState")
    }

    /// C++: `void VehicleState::~VehicleState() noexcept`
    /// Destructor
    // ~VehicleState() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port VehicleState::~VehicleState")
    }

}

