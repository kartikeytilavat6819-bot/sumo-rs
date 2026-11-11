//! Port of microsim/devices/MSTransportableDevice_FCDReplay.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A device which collects info on the vehicle trip (mainly on departure and arrival)
/// C++ class `MSTransportableDevice_FCDReplay` (microsim/devices/MSTransportableDevice_FCDReplay.h)
pub struct MSTransportableDevice_FCDReplay {
    /// C++ base class `MSTransportableDevice`
    pub base: MSTransportableDevice,
    pub myTrajectory: Ptr<MSDevice_FCDReplay_Trajectory>,
    pub myTrajectoryIndex: i32,
}

impl std::ops::Deref for MSTransportableDevice_FCDReplay {
    type Target = MSTransportableDevice;
    fn deref(&self) -> &MSTransportableDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSTransportableDevice_FCDReplay {
    fn deref_mut(&mut self) -> &mut MSTransportableDevice {
        &mut self.base
    }
}

// C++ static members of MSTransportableDevice_FCDReplay (port as module-level statics, see AGENTS.md):
//   static bool MSTransportableDevice_FCDReplay::myAmActive;

impl MSTransportableDevice_FCDReplay {
    /// C++: `void MSTransportableDevice_FCDReplay::insertOptions(OptionsCont &)` [static]
    /// Inserts MSTransportableDevice_FCDReplay-options
    // void
    // MSTransportableDevice_FCDReplay::insertOptions(OptionsCont& oc) {
    //     insertDefaultAssignmentOptions("fcd-replay", "FCD Replay Device", oc, true);
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        todo!("port MSTransportableDevice_FCDReplay::insertOptions")
    }

    /// C++: `void MSTransportableDevice_FCDReplay::buildDevices(MSTransportable &, std::vector<MSTransportableDevice *> &)` [static]
    /// Build devices for the given vehicle, if needed
    // void
    // MSTransportableDevice_FCDReplay::buildDevices(MSTransportable& t, std::vector<MSTransportableDevice*>& into) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     if (equippedByDefaultAssignmentOptions(oc, "fcd-replay", t, oc.isSet("device.fcd-replay.files"), true)) {
    //         MSTransportableDevice_FCDReplay* device = new MSTransportableDevice_FCDReplay(t, "fcdReplay_" + t.getID());
    //         into.push_back(device);
    //         if (!myAmActive) {
    //             MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(new MovePedestrians(), SIMSTEP + DELTA_T);
    //             myAmActive = true;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_devices(t: &mut MSTransportable, into: &mut Vec<Ptr<MSTransportableDevice>>) {
        todo!("port MSTransportableDevice_FCDReplay::buildDevices")
    }

    /// C++: `void MSTransportableDevice_FCDReplay::~MSTransportableDevice_FCDReplay() noexcept` [virtual]
    /// Destructor.
    // MSTransportableDevice_FCDReplay::~MSTransportableDevice_FCDReplay() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSTransportableDevice_FCDReplay::~MSTransportableDevice_FCDReplay")
    }

    /// C++: `const std::string MSTransportableDevice_FCDReplay::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "fcd-replay";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        todo!("port MSTransportableDevice_FCDReplay::deviceName")
    }

    /// C++: `void MSTransportableDevice_FCDReplay::setTrajectory(MSDevice_FCDReplay::Trajectory *const)`
    // void setTrajectory(MSDevice_FCDReplay::Trajectory* const t) {
    //         myTrajectory = t;
    //         myTrajectoryIndex = 1;
    //     }
    #[allow(unused_variables)]
    pub fn set_trajectory(&mut self, t: Ptr<MSDevice_FCDReplay_Trajectory>) {
        todo!("port MSTransportableDevice_FCDReplay::setTrajectory")
    }

    /// C++: `bool MSTransportableDevice_FCDReplay::move(SUMOTime)`
    // bool
    // MSTransportableDevice_FCDReplay::move(SUMOTime currentTime) {
    //     if (!myHolder.hasDeparted()) { // need to check this first for trajectories of size 1
    //         return false;
    //     }
    //     if (myTrajectory == nullptr || myTrajectoryIndex == (int)myTrajectory->size()) {
    //         // removing person
    //         return true;
    //     }
    //     MSPerson* person = dynamic_cast<MSPerson*>(&myHolder);
    //     const auto& te = myTrajectory->at(myTrajectoryIndex);
    //     if (person == nullptr || te.time > currentTime) {
    //         return false;
    //     }
    //     if (person->getCurrentStageType() == MSStageType::DRIVING) {
    //         if (person->getVehicle() == nullptr) {  // entering the vehicle
    //             const MSEdge* const edge = person->getEdge();
    //             for (const SUMOVehicle* v : edge->getVehicles()) {
    //                 if (v->getSpeed() == 0. && fabs(v->getPositionOnLane() - te.lanePos) < POSITION_EPS) {
    //                     v->getLane()->getVehiclesSecure();  // lock the lane
    //                     SUMOTime dummy = -1; // boarding- and loading-time are not considered
    //                     MSNet::getInstance()->getPersonControl().loadAnyWaiting(edge, const_cast<SUMOVehicle*>(v), dummy, dummy, person);
    //                     v->getLane()->releaseVehicles();  // unlock the lane
    //                 }
    //             }
    //         } else {
    //             SUMOVehicle* v = person->getVehicle();
    //             if (te.speed == 0. && fabs(v->getPositionOnLane() - te.lanePos) >= POSITION_EPS) {  // leaving the vehicle
    //                 v->getLane()->getVehiclesSecure();  // lock the lane
    //                 MSDevice_Transportable* transDev = static_cast<MSDevice_Transportable*>(v->getDevice(typeid(MSDevice_Transportable)));
    //                 transDev->removeTransportable(person);
    //                 person->proceed(MSNet::getInstance(), currentTime);
    //                 v->getLane()->releaseVehicles();  // unlock the lane
    //             }
    //         }
    //     }
    //     if (person->getCurrentStageType() == MSStageType::WALKING) {
    //         try {
    //             libsumo::Person::moveToXY(person->getID(), te.edgeOrLane, te.pos.x(), te.pos.y(), te.angle, 7, 0.1);
    //         } catch (const libsumo::TraCIException& e) {
    //             WRITE_WARNING(e.what());
    //         }
    //         MSStageWalking* walk = static_cast<MSStageWalking*>(person->getCurrentStage());
    //         if (myTrajectoryIndex > 0 && myTrajectory->at(myTrajectoryIndex - 1).edgeOrLane != te.edgeOrLane) {
    //             walk->moveToNextEdge(person, currentTime, 1, nullptr, true);
    //         }
    //     }
    //     // person->setPreviousSpeed(std::get<3>(p), std::numeric_limits<double>::min());
    //     myTrajectoryIndex++;
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn move_(&mut self, currentTime: SUMOTime) -> bool {
        todo!("port MSTransportableDevice_FCDReplay::move")
    }

    /// C++: `void MSTransportableDevice_FCDReplay::MSTransportableDevice_FCDReplay(MSTransportable &, const std::string &)` [private]
    /// Constructor
    // MSTransportableDevice_FCDReplay::MSTransportableDevice_FCDReplay(MSTransportable& holder, const std::string& id) :
    //     MSTransportableDevice(holder, id) {
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut MSTransportable, id: &str) -> Self {
        todo!("port MSTransportableDevice_FCDReplay::MSTransportableDevice_FCDReplay")
    }

    /// C++: `void MSTransportableDevice_FCDReplay::MSTransportableDevice_FCDReplay(const MSTransportableDevice_FCDReplay &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTransportableDevice_FCDReplay) -> Self {
        todo!("port MSTransportableDevice_FCDReplay::MSTransportableDevice_FCDReplay")
    }

    /// C++: `MSTransportableDevice_FCDReplay & MSTransportableDevice_FCDReplay::operator=(const MSTransportableDevice_FCDReplay &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTransportableDevice_FCDReplay) -> &mut MSTransportableDevice_FCDReplay {
        todo!("port MSTransportableDevice_FCDReplay::operator=")
    }

}

/// C++ class `MSTransportableDevice_FCDReplay::MovePedestrians` (microsim/devices/MSTransportableDevice_FCDReplay.h)
pub struct MSTransportableDevice_FCDReplay_MovePedestrians {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
}

impl std::ops::Deref for MSTransportableDevice_FCDReplay_MovePedestrians {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSTransportableDevice_FCDReplay_MovePedestrians {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl MSTransportableDevice_FCDReplay_MovePedestrians {
    /// C++: `void MovePedestrians::MovePedestrians()`
    // MSTransportableDevice_FCDReplay::MovePedestrians::MovePedestrians() {
    //     // higher than default command priority of 0
    //     priority = 1;
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port MovePedestrians::MovePedestrians")
    }

    /// C++: `SUMOTime MovePedestrians::execute(SUMOTime)` [virtual]
    // SUMOTime
    // MSTransportableDevice_FCDReplay::MovePedestrians::execute(SUMOTime currentTime) {
    //     MSTransportableControl& c = MSNet::getInstance()->getPersonControl();
    //     std::vector<MSTransportableDevice_FCDReplay*> devices;
    //     for (MSTransportableControl::constVehIt i = c.loadedBegin(); i != c.loadedEnd(); ++i) {
    //         MSTransportableDevice_FCDReplay* device = static_cast<MSTransportableDevice_FCDReplay*>(i->second->getDevice(typeid(MSTransportableDevice_FCDReplay)));
    //         if (device != nullptr) {
    //             devices.push_back(device);
    //         }
    //     }
    //     for (MSTransportableDevice_FCDReplay* d : devices) {
    //         if (d->move(currentTime)) {
    //             d->getHolder().removeStage(0, false);
    //         }
    //     }
    //     return DELTA_T;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port MovePedestrians::execute")
    }

    /// C++: `MovePedestrians & MovePedestrians::operator=(const MovePedestrians &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTransportableDevice_FCDReplay_MovePedestrians) -> &mut MSTransportableDevice_FCDReplay_MovePedestrians {
        todo!("port MovePedestrians::operator=")
    }

}

