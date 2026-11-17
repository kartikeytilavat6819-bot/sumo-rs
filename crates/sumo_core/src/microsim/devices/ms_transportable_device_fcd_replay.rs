//! Port of microsim/devices/MSTransportableDevice_FCDReplay.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use utils::common::time::delta_t;
use utils::geom::POSITION_EPS;

/// A device which collects info on the vehicle trip (mainly on departure and arrival)
/// C++ class `MSTransportableDevice_FCDReplay` (microsim/devices/MSTransportableDevice_FCDReplay.h)
#[repr(C)]
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
static MY_AM_ACTIVE: AtomicBool = AtomicBool::new(false);

impl MSTransportableDevice_FCDReplay {
    /// C++: `void MSTransportableDevice_FCDReplay::insertOptions(OptionsCont &)` [static]
    /// Inserts MSTransportableDevice_FCDReplay-options
    // void
    // MSTransportableDevice_FCDReplay::insertOptions(OptionsCont& oc) {
    //     insertDefaultAssignmentOptions("fcd-replay", "FCD Replay Device", oc, true);
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        MSDevice::insert_default_assignment_options("fcd-replay", "FCD Replay Device", oc, true);
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
        let outputOptionSet = utils::options::options_cont::is_set("device.fcd-replay.files");
        let equipped = utils::options::options_cont::with_options(|oc| {
            MSDevice::equipped_by_default_assignment_options(oc, "fcd-replay", t.get_id(), t.get_parameter(), t.get_vehicle_type().get_parameter(), outputOptionSet, true)
        });
        if equipped {
            let id = format!("fcdReplay_{}", t.get_id());
            // shortcut: device is leaked; the person's device list will own it once person device ownership is ported, upgrade then
            let device: &mut MSTransportableDevice_FCDReplay = Box::leak(Box::new(MSTransportableDevice_FCDReplay::new(t, &id)));
            into.push(Ptr::from_mut(&mut device.base));
            if !MY_AM_ACTIVE.load(Ordering::Relaxed) {
                // shortcut: the command is leaked and owned by the begin-of-timestep event control for the rest of the run
                let cmd: &mut dyn Command = Box::leak(Box::new(MSTransportableDevice_FCDReplay_MovePedestrians::new()));
                let execTime = MSNet::get_instance().m().get_current_time_step() + delta_t();
                MSNet::get_instance().m().get_begin_of_timestep_events().m().add_event(Ptr::from_mut(cmd), execTime);
                MY_AM_ACTIVE.store(true, Ordering::Relaxed);
            }
        }
    }

    /// C++: `void MSTransportableDevice_FCDReplay::~MSTransportableDevice_FCDReplay() noexcept` [virtual]
    /// Destructor.
    // MSTransportableDevice_FCDReplay::~MSTransportableDevice_FCDReplay() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; nothing to release.
    }

    /// C++: `const std::string MSTransportableDevice_FCDReplay::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "fcd-replay";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "fcd-replay".to_string()
    }

    /// C++: `void MSTransportableDevice_FCDReplay::setTrajectory(MSDevice_FCDReplay::Trajectory *const)`
    // void setTrajectory(MSDevice_FCDReplay::Trajectory* const t) {
    //         myTrajectory = t;
    //         myTrajectoryIndex = 1;
    //     }
    #[allow(unused_variables)]
    pub fn set_trajectory(&mut self, t: Ptr<MSDevice_FCDReplay_Trajectory>) {
        self.myTrajectory = t;
        self.myTrajectoryIndex = 1;
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
        let holder = self.myHolder;
        if !holder.m().has_departed() { // need to check this first for trajectories of size 1
            return false;
        }
        if self.myTrajectory.is_null() || self.myTrajectoryIndex == self.myTrajectory.m().len() as i32 {
            // removing person
            return true;
        }
        let idx = self.myTrajectoryIndex as usize;
        let (teTime, teEdge, tePos, teAngle, teLanePos, teSpeed) = {
            let te = &self.myTrajectory.m()[idx];
            (te.time, te.edgeOrLane.clone(), te.pos, te.angle, te.lanePos, te.speed)
        };
        // dynamic_cast<MSPerson*>: the person check is the is_person() type flag
        if !holder.m().is_person() || teTime > currentTime {
            return false;
        }
        if matches!(holder.m().get_current_stage_type(), MSStageType::DRIVING) {
            if holder.m().get_vehicle().is_null() { // entering the vehicle
                let edge = holder.m().get_edge();
                for v in edge.m().get_vehicles() {
                    if v.m().get_speed() == 0. && (v.m().get_position_on_lane() - teLanePos).abs() < POSITION_EPS {
                        let lane = v.m().get_lane();
                        lane.m().get_vehicles_secure(); // lock the lane
                        let mut dummy: SUMOTime = -1; // boarding- and loading-time are not considered
                        let mut dummy2: SUMOTime = -1;
                        MSNet::get_instance().m().get_person_control().load_any_waiting(edge, v, &mut dummy, &mut dummy2, holder);
                        lane.m().release_vehicles(); // unlock the lane
                    }
                }
            } else {
                let v = holder.m().get_vehicle();
                if teSpeed == 0. && (v.m().get_position_on_lane() - teLanePos).abs() >= POSITION_EPS { // leaving the vehicle
                    let lane = v.m().get_lane();
                    lane.m().get_vehicles_secure(); // lock the lane
                    // getDevice(typeid(MSDevice_Transportable)): the first person or container device
                    let transDev = v.m().get_devices().iter().copied().find(|d| { let n = d.base2.device_name(); n == "person" || n == "container" }).unwrap_or_default();
                    transDev.cast::<MSDevice_Transportable>().m().remove_transportable(holder);
                    holder.m().proceed(MSNet::get_instance(), currentTime, false);
                    lane.m().release_vehicles(); // unlock the lane
                }
            }
        }
        if matches!(holder.m().get_current_stage_type(), MSStageType::WALKING) {
            let pid = holder.m().get_id().to_string();
            // C++ catch (libsumo::TraCIException& e) { WRITE_WARNING(e.what()); }: TraCI errors travel as String panic payloads
            let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                crate::libsumo::person::Person::move_to_xy(&pid, &teEdge, tePos.x(), tePos.y(), teAngle, 7, 0.1);
            }));
            if let Err(payload) = res {
                match payload.downcast_ref::<String>() {
                    Some(msg) => utils::common::msg_handler::write_warning(msg),
                    None => std::panic::resume_unwind(payload),
                }
            }
            let walk: Ptr<MSStageWalking> = holder.m().get_current_stage().cast::<MSStageWalking>();
            if self.myTrajectoryIndex > 0 && self.myTrajectory.m()[(self.myTrajectoryIndex - 1) as usize].edgeOrLane != teEdge {
                walk.m().move_to_next_edge(holder, currentTime, 1, Ptr::null(), true);
            }
        }
        // person->setPreviousSpeed(...) is commented out in C++
        self.myTrajectoryIndex += 1;
        false
    }

    /// C++: `void MSTransportableDevice_FCDReplay::MSTransportableDevice_FCDReplay(MSTransportable &, const std::string &)` [private]
    /// Constructor
    // MSTransportableDevice_FCDReplay::MSTransportableDevice_FCDReplay(MSTransportable& holder, const std::string& id) :
    //     MSTransportableDevice(holder, id) {
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut MSTransportable, id: &str) -> Self {
        let mut this = Self::new_device(holder, id);
        // C++ deviceName() is "fcd-replay", which the id prefix "fcdReplay_" does not give
        this.base.base.name = "fcd-replay".to_string();
        this
    }

    fn new_device(holder: &mut MSTransportable, id: &str) -> Self {
        MSTransportableDevice_FCDReplay {
            base: MSTransportableDevice::new(holder, id),
            myTrajectory: Ptr::null(),
            myTrajectoryIndex: 0,
        }
    }

    /// C++: `void MSTransportableDevice_FCDReplay::MSTransportableDevice_FCDReplay(const MSTransportableDevice_FCDReplay &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTransportableDevice_FCDReplay) -> Self {
        panic!("MSTransportableDevice_FCDReplay copy constructor is private and invalidated in C++")
    }

    /// C++: `MSTransportableDevice_FCDReplay & MSTransportableDevice_FCDReplay::operator=(const MSTransportableDevice_FCDReplay &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTransportableDevice_FCDReplay) -> &mut MSTransportableDevice_FCDReplay {
        panic!("MSTransportableDevice_FCDReplay assignment operator is private and invalidated in C++")
    }

}

/// C++ class `MSTransportableDevice_FCDReplay::MovePedestrians` (microsim/devices/MSTransportableDevice_FCDReplay.h)
#[repr(C)]
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
        // priority = 1 is provided by the Command impl below
        MSTransportableDevice_FCDReplay_MovePedestrians { base: () }
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
        let mut net = MSNet::get_instance();
        let c = net.m().get_person_control();
        let mut devices: Vec<Ptr<MSTransportableDevice_FCDReplay>> = Vec::new();
        for (_, t) in c.loaded_begin() {
            let device = t.get_device("fcd-replay");
            if !device.is_null() {
                devices.push(device.cast::<MSTransportableDevice_FCDReplay>());
            }
        }
        for d in devices {
            if d.m().move_(currentTime) {
                d.m().get_holder().remove_stage(0, false);
            }
        }
        delta_t()
    }

    /// C++: `MovePedestrians & MovePedestrians::operator=(const MovePedestrians &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTransportableDevice_FCDReplay_MovePedestrians) -> &mut MSTransportableDevice_FCDReplay_MovePedestrians {
        panic!("MovePedestrians assignment operator is private and invalidated in C++")
    }

}

impl Command for MSTransportableDevice_FCDReplay_MovePedestrians {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        MSTransportableDevice_FCDReplay_MovePedestrians::execute(self, currentTime)
    }

    fn priority(&self) -> i32 {
        // C++ ctor: higher than default command priority of 0
        1
    }
}

