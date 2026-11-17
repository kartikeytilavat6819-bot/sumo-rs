//! Port of microsim/MSVehicleControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::common::std_defs::{get_scaling_quota, SUMO_CONST_HALTING_SPEED};
use utils::common::sumo_vehicle_class::{DEFAULT_BIKETYPE_ID, DEFAULT_CONTAINERTYPE_ID, DEFAULT_PEDTYPE_ID, DEFAULT_RAILTYPE_ID, DEFAULT_TAXITYPE_ID, DEFAULT_VTYPE_ID, DEFAULT_VTYPES, SVC_BICYCLE, SVC_CONTAINER, SVC_IGNORING, SVC_NON_ROAD, SVC_PASSENGER, SVC_PEDESTRIAN, SVC_RAIL, SVC_RAIL_CLASSES, SVC_ROAD_MOTOR_CLASSES, SVC_TAXI};
use utils::common::time::{delta_t, stepfloor, steps2time, string2time, SUMOTIME_MAX};
use utils::common::to_string::f64_to_string;
use utils::options::options_cont::{get_float, get_string, is_set};

// VTYPEPARS_VEHICLECLASS_SET bit from utils/vehicle/SUMOVTypeParameter.h (not exported by the ported module yet)
const VTYPEPARS_VEHICLECLASS_SET: i64 = 1 << 7;


/// C++ `static_cast<MSBaseVehicle*>(SUMOVehicle*)` for calls that are virtual or only on MSBaseVehicle in the port.
fn as_base(v: Ptr<SUMOVehicle>) -> Ptr<MSBaseVehicle> {
    v.cast::<MSBaseVehicle>()
}

/// C++ `delete veh`: runs ~MSVehicle (microsim only; ~MEVehicle has no body, gUseMesoSim selects the class), then
/// ~MSBaseVehicle. The memory stays leaked (vehicles are Box::leak'ed by build_vehicle), only the destructor side effects run
fn delete_vehicle_object(veh: Ptr<SUMOVehicle>) {
    if !crate::microsim::ms_globals::use_meso_sim() {
        as_ms_vehicle(veh).m().drop_();
    }
    as_base(veh).m().drop_();
}

/// C++ `static_cast<MSVehicle*>(SUMOVehicle*)`
fn as_ms_vehicle(v: Ptr<SUMOVehicle>) -> Ptr<MSVehicle> {
    v.cast::<MSVehicle>()
}

/// C++ `MSVehicleControl::DeleteKeptVehicle`/`MSVehicleControl` copy and assignment are private and never defined
fn invalid_copy() -> ! {
    panic!("MSVehicleControl copy/assignment is private and never defined in C++")
}

fn max_of(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
fn min_of(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// C++ enum `MSVehicleControl::VehicleDefinitionSource`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSVehicleControl_VehicleDefinitionSource {
    ROUTEFILE = 0,
    STATE = 1,
    TRIGGER = 2,
    LIBSUMO = 3,
    OTHER = 4,
}

/// C++ typedef `std::map<std::string, MSVehicleType *> MSVehicleControl::VTypeDictType`
pub type MSVehicleControl_VTypeDictType = BTreeMap<String, Ptr<MSVehicleType>>;

/// C++ typedef `std::map<std::string, RandomDistributor<MSVehicleType *> *> MSVehicleControl::VTypeDistDictType`
pub type MSVehicleControl_VTypeDistDictType = BTreeMap<String, Ptr<RandomDistributor<Ptr<MSVehicleType>>>>;

/// C++ typedef `std::map<std::string, SUMOVehicle *> MSVehicleControl::VehicleDictType`
pub type MSVehicleControl_VehicleDictType = BTreeMap<String, Ptr<SUMOVehicle>>;

/// C++ typedef `std::map<std::string, SUMOVehicle *>::const_iterator MSVehicleControl::constVehIt`
pub type MSVehicleControl_constVehIt = Unmapped /* std::map<std::string, SUMOVehicle *>::const_iterator */;

/// @}
/// C++ class `MSVehicleControl::DeleteKeptVehicle` (microsim/MSVehicleControl.h)
#[repr(C)]
pub struct MSVehicleControl_DeleteKeptVehicle {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
    pub myVehicle: Ptr<SUMOVehicle>,
}

impl std::ops::Deref for MSVehicleControl_DeleteKeptVehicle {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSVehicleControl_DeleteKeptVehicle {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl Command for MSVehicleControl_DeleteKeptVehicle {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        MSVehicleControl_DeleteKeptVehicle::execute(self, currentTime)
    }
}

impl MSVehicleControl_DeleteKeptVehicle {
    /// C++: `void DeleteKeptVehicle::DeleteKeptVehicle(SUMOVehicle *)`
    // DeleteKeptVehicle(SUMOVehicle* veh) : myVehicle(veh) {}
    #[allow(unused_variables)]
    pub fn new(veh: Ptr<SUMOVehicle>) -> Self {
        MSVehicleControl_DeleteKeptVehicle {
            base: (),
            myVehicle: veh,
        }
    }

    /// C++: `void DeleteKeptVehicle::~DeleteKeptVehicle() noexcept` [virtual]
    // ~DeleteKeptVehicle() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is empty
    }

    /// C++: `SUMOTime DeleteKeptVehicle::execute(SUMOTime)` [virtual]
    // SUMOTime
    // MSVehicleControl::DeleteKeptVehicle::execute(SUMOTime /*currentTime*/) {
    //     MSNet::getInstance()->getVehicleControl().deleteVehicle(myVehicle, false, true);
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        MSNet::get_instance().m().get_vehicle_control().delete_vehicle(self.myVehicle, false, true);
        0
    }

    /// C++: `DeleteKeptVehicle & DeleteKeptVehicle::operator=(const DeleteKeptVehicle &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSVehicleControl_DeleteKeptVehicle) -> &mut MSVehicleControl_DeleteKeptVehicle {
        invalid_copy()
    }

}

/// The class responsible for building and deletion of vehicles
/// C++ class `MSVehicleControl` (microsim/MSVehicleControl.h)
pub struct MSVehicleControl {
    /// The number of build vehicles
    pub myLoadedVehNo: i32,
    /// The number of vehicles within the network (build and inserted but not removed)
    pub myRunningVehNo: i32,
    /// The number of removed vehicles
    pub myEndedVehNo: i32,
    /// The number of vehicles which were discarded while loading
    pub myDiscarded: i32,
    /// The number of collisions
    pub myCollisions: i32,
    /// The number of teleports due to collision
    pub myTeleportsCollision: i32,
    /// The number of teleports due to jam
    pub myTeleportsJam: i32,
    /// The number of teleports due to vehicles stuck on a minor road
    pub myTeleportsYield: i32,
    /// The number of teleports due to vehicles stuck on the wrong lane
    pub myTeleportsWrongLane: i32,
    /// The number of emergency stops
    pub myEmergencyStops: i32,
    /// The number of emergency stops
    pub myEmergencyBrakingCount: i32,
    /// The number of stopped vehicles
    pub myStoppedVehicles: i32,
    /// The aggregated time vehicles had to wait for departure (in seconds)
    pub myTotalDepartureDelay: f64,
    /// The aggregated time vehicles needed to accomplish their route (in seconds)
    pub myTotalTravelTime: f64,
    /// Dictionary of vehicles
    pub myVehicleDict: MSVehicleControl_VehicleDictType,
    /// Dictionary of vehicle types
    pub myVTypeDict: MSVehicleControl_VTypeDictType,
    /// A distribution of vehicle types (probability->vehicle type)
    pub myVTypeDistDict: MSVehicleControl_VTypeDistDictType,
    /// Inverse lookup from vehicle type to distributions it is a member of
    pub myVTypeToDist: BTreeMap<String, BTreeSet<String>>, // C++: std::map<std::string, std::set<std::string>>
    /// the default vehicle types which may still be replaced
    pub myReplaceableDefaultVTypes: BTreeSet<String>,
    /// the number of vehicles waiting for persons or containers contained in myWaiting which can only continue by being triggered
    pub myWaitingForTransportable: i32,
    /// The scaling factor (especially for inc-dua)
    pub myScale: f64,
    pub myKeepTime: SUMOTime,
    /// The maximum speed factor for all vehicles in the network
    pub myMaxSpeedFactor: f64,
    /// The minimum deceleration capability for all road vehicles in the network
    pub myMinDeceleration: f64,
    /// The minimum deceleration capability for all rail vehicles in the network
    pub myMinDecelerationRail: f64,
    /// The maximum minGap value encountered in the simulation
    pub myMaxMinGap: f64,
    /// List of vehicles which belong to public transport
    pub myPTVehicles: Vec<Ptr<SUMOVehicle>>,
    pub myPendingRemovals: Vec<Ptr<SUMOVehicle>>,
    /// virtual dispatch for `buildVehicle` (overridden by MEVehicleControl)
    pub vt: &'static MSVehicleControlVt,
}

/// Virtual method table of `MSVehicleControl` (C++ virtual `buildVehicle`)
pub struct MSVehicleControlVt {
    pub build_vehicle: fn(&mut MSVehicleControl, Ptr<SUMOVehicleParameter>, ConstMSRoutePtr, Ptr<MSVehicleType>, bool, MSVehicleControl_VehicleDefinitionSource, bool) -> Ptr<SUMOVehicle>,
}

/// Default `MSVehicleControl` vtable: the base class implementations
pub static DEFAULT_MS_VEHICLE_CONTROL_VT: MSVehicleControlVt = MSVehicleControlVt {
    build_vehicle: default_build_vehicle,
};

impl MSVehicleControl {
    /// C++: `void MSVehicleControl::MSVehicleControl()`
    /// Constructor
    // MSVehicleControl::MSVehicleControl() :
    //     myLoadedVehNo(0),
    //     myRunningVehNo(0),
    //     myEndedVehNo(0),
    //     myDiscarded(0),
    //     myCollisions(0),
    //     myTeleportsCollision(0),
    //     myTeleportsJam(0),
    //     myTeleportsYield(0),
    //     myTeleportsWrongLane(0),
    //     myEmergencyStops(0),
    //     myEmergencyBrakingCount(0),
    //     myStoppedVehicles(0),
    //     myTotalDepartureDelay(0),
    //     myTotalTravelTime(0),
    //     myWaitingForTransportable(0),
    //     myMaxSpeedFactor(1),
    //     myMinDeceleration(SUMOVTypeParameter::getDefaultDecel(SVC_IGNORING)),
    //     myMinDecelerationRail(SUMOVTypeParameter::getDefaultDecel(SVC_RAIL)),
    //     myMaxMinGap(0),
    //     myPendingRemovals(MSGlobals::gNumSimThreads > 1) {
    //
    //     initDefaultTypes();
    //     myScale = OptionsCont::getOptions().getFloat("scale");
    //     myKeepTime = string2time(OptionsCont::getOptions().getString("keep-after-arrival"));
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        let mut s = MSVehicleControl {
            myLoadedVehNo: 0,
            myRunningVehNo: 0,
            myEndedVehNo: 0,
            myDiscarded: 0,
            myCollisions: 0,
            myTeleportsCollision: 0,
            myTeleportsJam: 0,
            myTeleportsYield: 0,
            myTeleportsWrongLane: 0,
            myEmergencyStops: 0,
            myEmergencyBrakingCount: 0,
            myStoppedVehicles: 0,
            myTotalDepartureDelay: 0.,
            myTotalTravelTime: 0.,
            myVehicleDict: BTreeMap::new(),
            myVTypeDict: BTreeMap::new(),
            myVTypeDistDict: BTreeMap::new(),
            myVTypeToDist: BTreeMap::new(),
            myReplaceableDefaultVTypes: BTreeSet::new(),
            myWaitingForTransportable: 0,
            myScale: 0.,
            myKeepTime: 0,
            myMaxSpeedFactor: 1.,
            myMinDeceleration: SUMOVTypeParameter::get_default_decel(SVC_IGNORING),
            myMinDecelerationRail: SUMOVTypeParameter::get_default_decel(SVC_RAIL),
            myMaxMinGap: 0.,
            // shortcut: C++ myPendingRemovals(gNumSimThreads > 1) is a locking container flag; a plain Vec is used
            myPTVehicles: Vec::new(),
            myPendingRemovals: Vec::new(),
            vt: &DEFAULT_MS_VEHICLE_CONTROL_VT,
        };

        s.init_default_types();
        s.myScale = get_float("scale") as f64;
        s.myKeepTime = string2time(&get_string("keep-after-arrival")).unwrap_or_else(|e| panic!("{}", e));
        s
    }

    /// C++: `void MSVehicleControl::~MSVehicleControl()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        self.clear_state(false);
    }

    /// C++: `SUMOVehicle * MSVehicleControl::buildVehicle(SUMOVehicleParameter *, ConstMSRoutePtr, MSVehicleType *, const bool, const VehicleDefinitionSource, bool)` [virtual]
    /// Builds a vehicle, increases the number of built vehicles
    // SUMOVehicle*
    // MSVehicleControl::buildVehicle(SUMOVehicleParameter* defs,
    //                                ConstMSRoutePtr route, MSVehicleType* type,
    //                                const bool ignoreStopErrors, const VehicleDefinitionSource source, bool addRouteStops) {
    //     const double speedFactor = type->computeChosenSpeedDeviation(defs->speedFactor, source == VehicleDefinitionSource::ROUTEFILE ? MSRouteHandler::getParsingRNG() : nullptr);
    //     MSVehicle* built = new MSVehicle(defs, route, type, speedFactor);
    //     initVehicle(built, ignoreStopErrors, addRouteStops, source);
    //     return built;
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle(&mut self, defs: Ptr<SUMOVehicleParameter>, route: ConstMSRoutePtr, type_: Ptr<MSVehicleType>, ignoreStopErrors: bool, source: MSVehicleControl_VehicleDefinitionSource, addRouteStops: bool) -> Ptr<SUMOVehicle> {
        (self.vt.build_vehicle)(self, defs, route, type_, ignoreStopErrors, source, addRouteStops)
    }
}

/// C++ body of `MSVehicleControl::buildVehicle` (the base implementation of the virtual)
#[allow(unused_variables)]
fn default_build_vehicle(vc: &mut MSVehicleControl, defs: Ptr<SUMOVehicleParameter>, route: ConstMSRoutePtr, type_: Ptr<MSVehicleType>, ignoreStopErrors: bool, source: MSVehicleControl_VehicleDefinitionSource, addRouteStops: bool) -> Ptr<SUMOVehicle> {
        let rng = if source == MSVehicleControl_VehicleDefinitionSource::ROUTEFILE {
            SUMORouteHandler::get_parsing_rng()
        } else {
            Ptr::null()
        };
        let speedFactor = type_.compute_chosen_speed_deviation(defs.speedFactor, rng, -1.);
        let built: Ptr<MSVehicle> = Ptr::from_mut(Box::leak(Box::new(MSVehicle::new(defs, route, type_, speedFactor))));
        vc.init_vehicle(Ptr::from_mut(&mut built.m().base), ignoreStopErrors, addRouteStops, source);
        Ptr::from_mut(&mut built.m().base.base)
}

impl MSVehicleControl {
    /// C++: `bool MSVehicleControl::addVehicle(const std::string &, SUMOVehicle *)` [virtual]
    /// Tries to insert the vehicle into the internal vehicle container
    // bool
    // MSVehicleControl::addVehicle(const std::string& id, SUMOVehicle* v) {
    //     VehicleDictType::iterator it = myVehicleDict.find(id);
    //     if (it == myVehicleDict.end()) {
    //         // id not in myVehicleDict.
    //         myVehicleDict[id] = v;
    //         handleTriggeredDepart(v, true);
    //         const SUMOVehicleParameter& pars = v->getParameter();
    //         if (v->getVClass() != SVC_TAXI && pars.line != "" && pars.repetitionNumber < 0) {
    //             myPTVehicles.push_back(v);
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add_vehicle(&mut self, id: &str, v: Ptr<SUMOVehicle>) -> bool {
        if !self.myVehicleDict.contains_key(id) {
            // id not in myVehicleDict.
            self.myVehicleDict.insert(id.to_string(), v);
            self.handle_triggered_depart(v, true);
            let bv = as_base(v);
            let pars = bv.get_parameter();
            if bv.get_v_class() != SVC_TAXI && !pars.line.is_empty() && pars.repetitionNumber < 0 {
                self.myPTVehicles.push(v);
            }
            return true;
        }
        false
    }

    /// C++: `SUMOVehicle * MSVehicleControl::getVehicle(const std::string &) const` [const]
    /// Returns the vehicle with the given id
    // SUMOVehicle*
    // MSVehicleControl::getVehicle(const std::string& id) const {
    //     VehicleDictType::const_iterator it = myVehicleDict.find(id);
    //     if (it == myVehicleDict.end()) {
    //         return nullptr;
    //     }
    //     return it->second;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle(&self, id: &str) -> Ptr<SUMOVehicle> {
        self.myVehicleDict.get(id).copied().unwrap_or(Ptr::null())
    }

    /// C++: `void MSVehicleControl::deleteVehicle(SUMOVehicle *, bool, bool)` [virtual]
    /// Deletes the vehicle
    // void
    // MSVehicleControl::deleteVehicle(SUMOVehicle* veh, bool discard, bool wasKept) {
    //     if (!wasKept) {
    //         myEndedVehNo++;
    //         if (discard) {
    //             myDiscarded++;
    //         }
    //     }
    //     if (veh != nullptr) {
    //         myVehicleDict.erase(veh->getID());
    //     }
    //     auto ptVehIt = std::find(myPTVehicles.begin(), myPTVehicles.end(), veh);
    //     if (ptVehIt != myPTVehicles.end()) {
    //         myPTVehicles.erase(ptVehIt);
    //     }
    //     delete veh;
    // }
    #[allow(unused_variables)]
    pub fn delete_vehicle(&mut self, veh: Ptr<SUMOVehicle>, discard: bool, wasKept: bool) {
        if !wasKept {
            self.myEndedVehNo += 1;
            if discard {
                self.myDiscarded += 1;
            }
        }
        if !veh.is_null() {
            self.myVehicleDict.remove(&veh.get_id().to_string());
        }
        if let Some(ptVehIt) = self.myPTVehicles.iter().position(|p| *p == veh) {
            self.myPTVehicles.remove(ptVehIt);
        }
        if !veh.is_null() {
            delete_vehicle_object(veh);
        }
    }

    /// C++: `void MSVehicleControl::deleteKeptVehicle(SUMOVehicle *)`
    /// when a vehicle is kept after arrival, schedule later deletion *
    // void
    // MSVehicleControl::deleteKeptVehicle(SUMOVehicle* veh) {
    //     myEndedVehNo++;
    //     MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(new DeleteKeptVehicle(veh), SIMSTEP + myKeepTime);
    // }
    #[allow(unused_variables)]
    pub fn delete_kept_vehicle(&mut self, veh: Ptr<SUMOVehicle>) {
        self.myEndedVehNo += 1;
        let execTimeStep = MSNet::get_instance().m().get_current_time_step() + self.myKeepTime;
        let event = MSVehicleControl_DeleteKeptVehicle::new(veh);
        let cmd: *mut dyn Command = Box::into_raw(Box::new(event));
        MSNet::get_instance().m().get_end_of_timestep_events().m().add_event(Ptr::from_raw(cmd), execTimeStep);
    }

    /// C++: `void MSVehicleControl::fixVehicleCounts()`
    // void fixVehicleCounts() {
    //         myLoadedVehNo++;
    //         myEndedVehNo++;
    //         myDiscarded++;
    //     }
    #[allow(unused_variables)]
    pub fn fix_vehicle_counts(&mut self) {
        self.myLoadedVehNo += 1;
        self.myEndedVehNo += 1;
        self.myDiscarded += 1;
    }

    /// C++: `void MSVehicleControl::scheduleVehicleRemoval(SUMOVehicle *, bool)`
    /// Removes a vehicle after it has ended
    // void
    // MSVehicleControl::scheduleVehicleRemoval(SUMOVehicle* veh, bool checkDuplicate) {
    //     assert(myRunningVehNo > 0);
    //     if (!checkDuplicate || !isPendingRemoval(veh)) {
    //         myPendingRemovals.push_back(veh);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn schedule_vehicle_removal(&mut self, veh: Ptr<SUMOVehicle>, checkDuplicate: bool) {
        debug_assert!(self.myRunningVehNo > 0);
        if !checkDuplicate || !self.is_pending_removal(veh) {
            self.myPendingRemovals.push(veh);
        }
    }

    /// C++: `void MSVehicleControl::removePending()`
    /// Removes a vehicle after it has ended
    // void
    // MSVehicleControl::removePending() {
    //     OutputDevice* const tripinfoOut = OptionsCont::getOptions().isSet("tripinfo-output") ? &OutputDevice::getDeviceByOption("tripinfo-output") : nullptr;
    // #ifdef HAVE_FOX
    //     std::vector<SUMOVehicle*>& vehs = myPendingRemovals.getContainer();
    // #else
    //     std::vector<SUMOVehicle*>& vehs = myPendingRemovals;
    // #endif
    //     std::sort(vehs.begin(), vehs.end(), ComparatorNumericalIdLess());
    //     for (SUMOVehicle* const veh : vehs) {
    //         myTotalTravelTime += STEPS2TIME(MSNet::getInstance()->getCurrentTimeStep() - veh->getDeparture());
    //         myRunningVehNo--;
    //         MSNet::getInstance()->informVehicleStateListener(veh, MSNet::VehicleState::ARRIVED);
    //         // vehicle is equipped with tripinfo device (not all vehicles are)
    //         const bool hasTripinfo = veh->getDevice(typeid(MSDevice_Tripinfo)) != nullptr;
    //         for (MSVehicleDevice* const dev : veh->getDevices()) {
    //             dev->generateOutput(hasTripinfo ? tripinfoOut : nullptr);
    //         }
    //         if (tripinfoOut != nullptr && hasTripinfo) {
    //             // close tag after tripinfo (possibly including emissions from another device) have been written
    //             tripinfoOut->closeTag();
    //         }
    //         if (myKeepTime == 0) {
    //             deleteVehicle(veh);
    //         } else {
    //             deleteKeptVehicle(veh);
    //         }
    //     }
    //     vehs.clear();
    //     if (tripinfoOut != nullptr) {
    //         // there seem to be people who think reading an unfinished xml is a good idea ;-)
    //         tripinfoOut->flush();
    //     }
    // #ifdef HAVE_FOX
    //     myPendingRemovals.unlock();
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn remove_pending(&mut self) {
        let tripinfoOut: Ptr<OutputDevice> = if is_set("tripinfo-output") {
            match OutputDevice::get_device_by_option("tripinfo-output") {
                Ok(dev) => Ptr::from_raw(dev.as_ptr()),
                Err(e) => panic!("{}", e),
            }
        } else {
            Ptr::null()
        };
        // std::sort with ComparatorNumericalIdLess; numerical ids are unique so ties cannot occur
        let mut vehs = std::mem::take(&mut self.myPendingRemovals);
        vehs.sort_by_key(|v| as_base(*v).get_numerical_id());
        for veh in vehs.iter().copied() {
            self.myTotalTravelTime += steps2time(MSNet::get_instance().m().get_current_time_step() - veh.get_departure());
            self.myRunningVehNo -= 1;
            MSNet::get_instance().m().inform_vehicle_state_listener(veh, MSNet_VehicleState::ARRIVED, "");
            // vehicle is equipped with tripinfo device (not all vehicles are)
            let hasTripinfo = veh.get_devices().iter().any(|d| MSDevice::get_device_name(d.base2.get_id()) == "tripinfo");
            for dev in veh.get_devices().iter() {
                MSDevice::generate_output_of(*dev, if hasTripinfo { tripinfoOut } else { Ptr::null() });
            }
            if !tripinfoOut.is_null() && hasTripinfo {
                // close tag after tripinfo (possibly including emissions from another device) have been written
                tripinfoOut.m().close_tag("");
            }
            if self.myKeepTime == 0 {
                self.delete_vehicle(veh, false, false);
            } else {
                self.delete_kept_vehicle(veh);
            }
        }
        if !tripinfoOut.is_null() {
            // there seem to be people who think reading an unfinished xml is a good idea ;-)
            tripinfoOut.m().flush();
        }
    }

    /// C++: `constVehIt MSVehicleControl::loadedVehBegin() const` [const]
    /// Returns the begin of the internal vehicle map
    // constVehIt loadedVehBegin() const {
    //         return myVehicleDict.begin();
    //     }
    #[allow(unused_variables)]
    pub fn loaded_veh_begin(&self) -> usize {
        // C++ map const_iterator is an index into myVehicleDict (id order, like the std::map), as for route iterators (AGENTS.md)
        0
    }

    /// C++: `constVehIt MSVehicleControl::loadedVehEnd() const` [const]
    /// Returns the end of the internal vehicle map
    // constVehIt loadedVehEnd() const {
    //         return myVehicleDict.end();
    //     }
    #[allow(unused_variables)]
    pub fn loaded_veh_end(&self) -> usize {
        // see loadedVehBegin; the end position is the number of loaded vehicles
        self.myVehicleDict.len()
    }

    /// C++: `void MSVehicleControl::handleTriggeredDepart(SUMOVehicle *, bool)`
    /// register / unregister depart-triggered vehicles with edges
    // void
    // MSVehicleControl::handleTriggeredDepart(SUMOVehicle* v, bool add) {
    //     const SUMOVehicleParameter& pars = v->getParameter();
    //     if (pars.departProcedure == DepartDefinition::TRIGGERED || pars.departProcedure == DepartDefinition::CONTAINER_TRIGGERED || pars.departProcedure == DepartDefinition::SPLIT) {
    //         const MSEdge* const firstEdge = v->getRoute().getEdges()[pars.departEdge];
    //         if (add) {
    //             if (!MSGlobals::gUseMesoSim) {
    //                 // position will be checked against person position later
    //                 static_cast<MSVehicle*>(v)->setTentativeLaneAndPosition(nullptr, v->getParameter().departPos);
    //             }
    //             if (firstEdge->isTazConnector()) {
    //                 for (MSEdge* out : firstEdge->getSuccessors()) {
    //                     out->addWaiting(v);
    //                 }
    //             } else {
    //                 firstEdge->addWaiting(v);
    //             }
    //             registerOneWaiting();
    //         } else {
    //             if (firstEdge->isTazConnector()) {
    //                 for (MSEdge* out : firstEdge->getSuccessors()) {
    //                     out->removeWaiting(v);
    //                 }
    //             } else {
    //                 firstEdge->removeWaiting(v);
    //             }
    //             unregisterOneWaiting();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn handle_triggered_depart(&mut self, v: Ptr<SUMOVehicle>, add: bool) {
        let bv = as_base(v);
        let pars = bv.get_parameter();
        if pars.departProcedure == DepartDefinition::TRIGGERED || pars.departProcedure == DepartDefinition::CONTAINER_TRIGGERED || pars.departProcedure == DepartDefinition::SPLIT {
            let firstEdge: Ptr<MSEdge> = v.get_route().get_edges()[pars.departEdge as usize];
            if add {
                if !MSGlobals::use_meso_sim() {
                    // position will be checked against person position later
                    as_ms_vehicle(v).m().set_tentative_lane_and_position(Ptr::null(), pars.departPos, 0.);
                }
                if firstEdge.is_taz_connector() {
                    for out in firstEdge.get_successors(SVC_IGNORING).iter() {
                        out.add_waiting(v);
                    }
                } else {
                    firstEdge.add_waiting(v);
                }
                self.register_one_waiting();
            } else {
                if firstEdge.is_taz_connector() {
                    for out in firstEdge.get_successors(SVC_IGNORING).iter() {
                        out.remove_waiting(v);
                    }
                } else {
                    firstEdge.remove_waiting(v);
                }
                self.unregister_one_waiting();
            }
        }
    }

    /// C++: `void MSVehicleControl::vehicleDeparted(const SUMOVehicle &)`
    /// Informs this control about a vehicle's departure
    // void
    // MSVehicleControl::vehicleDeparted(const SUMOVehicle& v) {
    //     ++myRunningVehNo;
    //     myTotalDepartureDelay += STEPS2TIME(v.getDeparture() - STEPFLOOR(v.getParameter().depart));
    //     MSNet::getInstance()->informVehicleStateListener(&v, MSNet::VehicleState::DEPARTED);
    //     myMaxSpeedFactor = MAX2(myMaxSpeedFactor, v.getChosenSpeedFactor());
    //     myMaxMinGap = MAX2(myMaxMinGap, v.getVehicleType().getMinGap());
    //     const double maxDecel = v.getVehicleType().getCarFollowModel().getMaxDecel();
    //     if ((v.getVClass() & (SVC_PEDESTRIAN | SVC_NON_ROAD)) == 0) {
    //         // only  worry about deceleration of road users
    //         myMinDeceleration = MIN2(myMinDeceleration, maxDecel);
    //     } else if ((v.getVClass() & SVC_RAIL_CLASSES) != 0) {
    //         myMinDecelerationRail = MIN2(myMinDecelerationRail, maxDecel);
    //         if ((v.getEdge()->getPermissions() & SVC_ROAD_MOTOR_CLASSES) != 0) {
    //             // shared rail/road
    //             myMinDeceleration = MIN2(myMinDeceleration, maxDecel);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn vehicle_departed(&mut self, v: &SUMOVehicle) {
        let bv: Ptr<MSBaseVehicle> = Ptr::new(v).cast::<MSBaseVehicle>();
        self.myRunningVehNo += 1;
        self.myTotalDepartureDelay += steps2time(v.get_departure() - stepfloor(bv.get_parameter().depart));
        MSNet::get_instance().m().inform_vehicle_state_listener(Ptr::new(v), MSNet_VehicleState::DEPARTED, "");
        self.myMaxSpeedFactor = max_of(self.myMaxSpeedFactor, bv.get_chosen_speed_factor());
        self.myMaxMinGap = max_of(self.myMaxMinGap, bv.get_vehicle_type().get_min_gap());
        let maxDecel = bv.get_vehicle_type().get_car_follow_model().get_max_decel();
        if (bv.get_v_class() & (SVC_PEDESTRIAN | SVC_NON_ROAD)) == 0 {
            // only  worry about deceleration of road users
            self.myMinDeceleration = min_of(self.myMinDeceleration, maxDecel);
        } else if (bv.get_v_class() & SVC_RAIL_CLASSES) != 0 {
            self.myMinDecelerationRail = min_of(self.myMinDecelerationRail, maxDecel);
            if (bv.get_edge().get_permissions() & SVC_ROAD_MOTOR_CLASSES) != 0 {
                // shared rail/road
                self.myMinDeceleration = min_of(self.myMinDeceleration, maxDecel);
            }
        }
    }

    /// C++: `int MSVehicleControl::getLoadedVehicleNo() const` [const]
    /// Returns the number of build vehicles
    // int getLoadedVehicleNo() const {
    //         return myLoadedVehNo;
    //     }
    #[allow(unused_variables)]
    pub fn get_loaded_vehicle_no(&self) -> i32 {
        self.myLoadedVehNo
    }

    /// C++: `int MSVehicleControl::getHaltingVehicleNo() const` [const, virtual]
    /// Returns the number of halting vehicles
    // int
    // MSVehicleControl::getHaltingVehicleNo() const {
    //     int result = 0;
    //     for (MSVehicleControl::constVehIt it = loadedVehBegin(); it != loadedVehEnd(); ++it) {
    //         const SUMOVehicle* veh = it->second;
    //         if ((veh->isOnRoad() || veh->isRemoteControlled()) && veh->getSpeed() < SUMO_const_haltingSpeed)  {
    //             result++;
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_halting_vehicle_no(&self) -> i32 {
        let mut result = 0;
        for veh in self.myVehicleDict.values() {
            let mv = as_ms_vehicle(*veh);
            if (mv.is_on_road() || mv.is_remote_controlled()) && mv.get_speed() < SUMO_CONST_HALTING_SPEED {
                result += 1;
            }
        }
        result
    }

    /// C++: `std::pair<double, double> MSVehicleControl::getVehicleMeanSpeeds() const` [const, virtual]
    /// get current absolute and relative mean vehicle speed in the network
    // std::pair<double, double>
    // MSVehicleControl::getVehicleMeanSpeeds() const {
    //     double speedSum = 0;
    //     double relSpeedSum = 0;
    //     int count = 0;
    //     for (MSVehicleControl::constVehIt it = loadedVehBegin(); it != loadedVehEnd(); ++it) {
    //         const SUMOVehicle* veh = it->second;
    //         if ((veh->isOnRoad() || veh->isRemoteControlled()) && !veh->isStopped()) {
    //             count++;
    //             speedSum += veh->getSpeed();
    //             relSpeedSum += veh->getEdge()->getSpeedLimit() > 0 ? veh->getSpeed() / veh->getEdge()->getSpeedLimit() : 0;
    //         }
    //     }
    //     if (count > 0) {
    //         return std::make_pair(speedSum / count, relSpeedSum / count);
    //     } else {
    //         return std::make_pair(-1, -1);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_mean_speeds(&self) -> (f64, f64) {
        let mut speedSum = 0.;
        let mut relSpeedSum = 0.;
        let mut count = 0;
        for veh in self.myVehicleDict.values() {
            let mv = as_ms_vehicle(*veh);
            if (mv.is_on_road() || mv.is_remote_controlled()) && !mv.base.is_stopped() {
                count += 1;
                speedSum += mv.get_speed();
                let limit = mv.base.get_edge().get_speed_limit();
                relSpeedSum += if limit > 0. { mv.get_speed() / limit } else { 0. };
            }
        }
        if count > 0 {
            (speedSum / count as f64, relSpeedSum / count as f64)
        } else {
            (-1., -1.)
        }
    }

    /// C++: `double MSVehicleControl::getVehicleMeanSpeed() const` [const]
    // double getVehicleMeanSpeed() const {
    //         return getVehicleMeanSpeeds().first;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_mean_speed(&self) -> f64 {
        self.get_vehicle_mean_speeds().0
    }

    /// C++: `double MSVehicleControl::getVehicleMeanSpeedRelative() const` [const]
    // double getVehicleMeanSpeedRelative() const {
    //         return getVehicleMeanSpeeds().second;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_mean_speed_relative(&self) -> f64 {
        self.get_vehicle_mean_speeds().1
    }

    /// C++: `int MSVehicleControl::getEndedVehicleNo() const` [const]
    /// Returns the number of removed vehicles
    // int getEndedVehicleNo() const {
    //         return myEndedVehNo;
    //     }
    #[allow(unused_variables)]
    pub fn get_ended_vehicle_no(&self) -> i32 {
        self.myEndedVehNo
    }

    /// C++: `int MSVehicleControl::getArrivedVehicleNo() const` [const]
    /// Returns the number of arrived vehicles
    // int getArrivedVehicleNo() const {
    //         return myEndedVehNo - myDiscarded;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrived_vehicle_no(&self) -> i32 {
        self.myEndedVehNo - self.myDiscarded
    }

    /// C++: `int MSVehicleControl::getDiscardedVehicleNo() const` [const]
    /// Returns the number of discarded vehicles
    // int getDiscardedVehicleNo() const {
    //         return myDiscarded;
    //     }
    #[allow(unused_variables)]
    pub fn get_discarded_vehicle_no(&self) -> i32 {
        self.myDiscarded
    }

    /// C++: `int MSVehicleControl::getRunningVehicleNo() const` [const]
    /// Returns the number of build and inserted, but not yet deleted vehicles
    // int getRunningVehicleNo() const {
    //         return myRunningVehNo;
    //     }
    #[allow(unused_variables)]
    pub fn get_running_vehicle_no(&self) -> i32 {
        self.myRunningVehNo
    }

    /// C++: `int MSVehicleControl::getDepartedVehicleNo() const` [const]
    /// Returns the number of inserted vehicles
    // int getDepartedVehicleNo() const {
    //         return myRunningVehNo + myEndedVehNo - myDiscarded;
    //     }
    #[allow(unused_variables)]
    pub fn get_departed_vehicle_no(&self) -> i32 {
        self.myRunningVehNo + self.myEndedVehNo - self.myDiscarded
    }

    /// C++: `int MSVehicleControl::getQuota(double, int) const` [const]
    /// Returns the number of instances of the current vehicle that shall be emitted considering that "frac" of all vehicles shall be emitted overall if a negative fraction is given the demand scaling factor is used (--scale) if a negative loaded number is is given, myLoadedVehNo is used
    // int
    // MSVehicleControl::getQuota(double frac, int loaded) const {
    //     frac = frac < 0 ? myScale : frac;
    //     const int origLoaded = (loaded < 1
    //                             // the vehicle in question has already been loaded, hence  the '-1'
    //                             ? frac > 1. ? (int)(myLoadedVehNo / frac) : myLoadedVehNo - 1
    //                             // given transportable number reflects only previously loaded
    //                             : frac > 1. ? (int)(loaded / frac) : loaded);
    //     return getScalingQuota(frac, origLoaded);
    // }
    #[allow(unused_variables)]
    pub fn get_quota(&self, frac: f64, loaded: i32) -> i32 {
        let frac = if frac < 0. { self.myScale } else { frac };
        let origLoaded: i32 = if loaded < 1 {
            // the vehicle in question has already been loaded, hence  the '-1'
            if frac > 1. { (self.myLoadedVehNo as f64 / frac) as i32 } else { self.myLoadedVehNo - 1 }
        } else {
            // given transportable number reflects only previously loaded
            if frac > 1. { (loaded as f64 / frac) as i32 } else { loaded }
        };
        get_scaling_quota(frac, origLoaded as i64)
    }

    /// C++: `int MSVehicleControl::getActiveVehicleCount() const` [const]
    /// Returns the number of build vehicles that have not been removed or need to wait for a passenger or a container
    // int getActiveVehicleCount() const {
    //         return myLoadedVehNo - (myWaitingForTransportable + myEndedVehNo);
    //     }
    #[allow(unused_variables)]
    pub fn get_active_vehicle_count(&self) -> i32 {
        self.myLoadedVehNo - (self.myWaitingForTransportable + self.myEndedVehNo)
    }

    /// C++: `int MSVehicleControl::getTriggeredVehicleCount() const` [const]
    /// return the number of vehicles that are waiting for a transportable or a join
    // int getTriggeredVehicleCount() const {
    //         return myWaitingForTransportable;
    //     }
    #[allow(unused_variables)]
    pub fn get_triggered_vehicle_count(&self) -> i32 {
        self.myWaitingForTransportable
    }

    /// C++: `int MSVehicleControl::getCollisionCount() const` [const]
    /// return the number of collisions
    // int getCollisionCount() const {
    //         return myCollisions;
    //     }
    #[allow(unused_variables)]
    pub fn get_collision_count(&self) -> i32 {
        self.myCollisions
    }

    /// C++: `int MSVehicleControl::getTeleportsCollisions() const` [const]
    /// return the number of collisions
    // int getTeleportsCollisions() const {
    //         return myTeleportsCollision;
    //     }
    #[allow(unused_variables)]
    pub fn get_teleports_collisions(&self) -> i32 {
        self.myTeleportsCollision
    }

    /// C++: `int MSVehicleControl::getTeleportsJam() const` [const]
    /// return the number of teleports due to jamming
    // int getTeleportsJam() const {
    //         return myTeleportsJam;
    //     }
    #[allow(unused_variables)]
    pub fn get_teleports_jam(&self) -> i32 {
        self.myTeleportsJam
    }

    /// C++: `int MSVehicleControl::getTeleportsYield() const` [const]
    /// return the number of teleports due to vehicles stuck on a minor road
    // int getTeleportsYield() const {
    //         return myTeleportsYield;
    //     }
    #[allow(unused_variables)]
    pub fn get_teleports_yield(&self) -> i32 {
        self.myTeleportsYield
    }

    /// C++: `int MSVehicleControl::getTeleportsWrongLane() const` [const]
    /// return the number of teleports due to vehicles stuck on the wrong lane
    // int getTeleportsWrongLane() const {
    //         return myTeleportsWrongLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_teleports_wrong_lane(&self) -> i32 {
        self.myTeleportsWrongLane
    }

    /// C++: `int MSVehicleControl::getTeleportCount() const` [const]
    /// return the number of teleports (including collisions)
    // int
    // MSVehicleControl::getTeleportCount() const {
    //     return myTeleportsCollision + myTeleportsJam + myTeleportsYield + myTeleportsWrongLane;
    // }
    #[allow(unused_variables)]
    pub fn get_teleport_count(&self) -> i32 {
        self.myTeleportsCollision + self.myTeleportsJam + self.myTeleportsYield + self.myTeleportsWrongLane
    }

    /// C++: `int MSVehicleControl::getEmergencyStops() const` [const]
    /// return the number of emergency stops
    // int getEmergencyStops() const {
    //         return myEmergencyStops;
    //     }
    #[allow(unused_variables)]
    pub fn get_emergency_stops(&self) -> i32 {
        self.myEmergencyStops
    }

    /// C++: `int MSVehicleControl::getEmergencyBrakingCount() const` [const]
    /// return the number of emergency stops
    // int getEmergencyBrakingCount() const {
    //         return myEmergencyBrakingCount;
    //     }
    #[allow(unused_variables)]
    pub fn get_emergency_braking_count(&self) -> i32 {
        self.myEmergencyBrakingCount
    }

    /// C++: `int MSVehicleControl::getStoppedVehiclesCount() const` [const]
    /// return the number of vehicles that are currently stopped
    // int getStoppedVehiclesCount() const {
    //         return myStoppedVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn get_stopped_vehicles_count(&self) -> i32 {
        self.myStoppedVehicles
    }

    /// C++: `double MSVehicleControl::getTotalDepartureDelay() const` [const]
    /// Returns the total departure delay
    // double getTotalDepartureDelay() const {
    //         return myTotalDepartureDelay;
    //     }
    #[allow(unused_variables)]
    pub fn get_total_departure_delay(&self) -> f64 {
        self.myTotalDepartureDelay
    }

    /// C++: `double MSVehicleControl::getTotalTravelTime() const` [const]
    /// Returns the total travel time
    // double getTotalTravelTime() const {
    //         return myTotalTravelTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_total_travel_time(&self) -> f64 {
        self.myTotalTravelTime
    }

    /// C++: `bool MSVehicleControl::addVType(MSVehicleType *)`
    /// Adds a vehicle type
    // bool
    // MSVehicleControl::addVType(MSVehicleType* vehType) {
    //     if (checkVType(vehType->getID())) {
    //         myVTypeDict[vehType->getID()] = vehType;
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add_v_type(&mut self, vehType: Ptr<MSVehicleType>) -> bool {
        if self.check_v_type(vehType.get_id()) {
            self.myVTypeDict.insert(vehType.get_id().to_string(), vehType);
            return true;
        }
        false
    }

    /// C++: `void MSVehicleControl::removeVType(const MSVehicleType *)`
    // void
    // MSVehicleControl::removeVType(const MSVehicleType* vehType) {
    //     assert(vehType != nullptr);
    //     assert(myVTypeDict.find(vehType->getID()) != myVTypeDict.end());
    //     myVTypeDict.erase(vehType->getID());
    //     if (myVTypeToDist.find(vehType->getID()) != myVTypeToDist.end()) {
    //         myVTypeToDist.erase(vehType->getID());
    //     }
    //     delete vehType;
    // }
    #[allow(unused_variables)]
    pub fn remove_v_type(&mut self, vehType: Ptr<MSVehicleType>) {
        let id = vehType.get_id().to_string();
        self.myVTypeDict.remove(&id);
        if self.myVTypeToDist.contains_key(&id) {
            self.myVTypeToDist.remove(&id);
        }
        // shortcut: C++ `delete vehType` is not done, types are Box::leak'ed by MSVehicleType::build, upgrade when type ownership is Box-based
    }

    /// C++: `bool MSVehicleControl::addVTypeDistribution(const std::string &, RandomDistributor<MSVehicleType *> *)`
    /// Adds a vehicle type distribution
    // bool
    // MSVehicleControl::addVTypeDistribution(const std::string& id, RandomDistributor<MSVehicleType*>* vehTypeDistribution) {
    //     if (checkVType(id)) {
    //         myVTypeDistDict[id] = vehTypeDistribution;
    //         std::vector<MSVehicleType*> vehTypes = vehTypeDistribution->getVals();
    //         for (auto vehType : vehTypes) {
    //             if (myVTypeToDist.find(vehType->getID()) != myVTypeToDist.end()) {
    //                 myVTypeToDist[vehType->getID()].insert(id);
    //             } else {
    //                 myVTypeToDist[vehType->getID()] = { id };
    //             }
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add_v_type_distribution(&mut self, id: &str, vehTypeDistribution: Ptr<RandomDistributor<Ptr<MSVehicleType>>>) -> bool {
        if self.check_v_type(id) {
            self.myVTypeDistDict.insert(id.to_string(), vehTypeDistribution);
            let vehTypes: Vec<Ptr<MSVehicleType>> = vehTypeDistribution.get_vals().to_vec();
            for vehType in vehTypes {
                self.myVTypeToDist.entry(vehType.get_id().to_string()).or_default().insert(id.to_string());
            }
            return true;
        }
        false
    }

    /// C++: `bool MSVehicleControl::hasVType(const std::string &) const` [const]
    /// Asks for existence of a vehicle type
    // bool
    // MSVehicleControl::hasVType(const std::string& id) const {
    //     return myVTypeDict.count(id) > 0 || myVTypeDistDict.count(id) > 0;
    // }
    #[allow(unused_variables)]
    pub fn has_v_type(&self, id: &str) -> bool {
        self.myVTypeDict.contains_key(id) || self.myVTypeDistDict.contains_key(id)
    }

    /// C++: `bool MSVehicleControl::hasVTypeDistribution(const std::string &) const` [const]
    /// Asks for a vehicle type distribution
    // bool
    // MSVehicleControl::hasVTypeDistribution(const std::string& id) const {
    //     return myVTypeDistDict.count(id) > 0;
    // }
    #[allow(unused_variables)]
    pub fn has_v_type_distribution(&self, id: &str) -> bool {
        self.myVTypeDistDict.contains_key(id)
    }

    /// C++: `MSVehicleType * MSVehicleControl::getVType(const std::string &, SumoRNG *, bool)`
    /// Returns the named vehicle type or a sample from the named distribution
    // MSVehicleType*
    // MSVehicleControl::getVType(const std::string& id, SumoRNG* rng, bool readOnly) {
    //     VTypeDictType::iterator it = myVTypeDict.find(id);
    //     if (it == myVTypeDict.end()) {
    //         VTypeDistDictType::iterator it2 = myVTypeDistDict.find(id);
    //         if (it2 == myVTypeDistDict.end()) {
    //             return nullptr;
    //         }
    //         return it2->second->get(rng);
    //     }
    //     if (!readOnly && myReplaceableDefaultVTypes.erase(id) > 0) {
    //         it->second->check();
    //     }
    //     return it->second;
    // }
    #[allow(unused_variables)]
    pub fn get_v_type(&mut self, id: &str, rng: Ptr<SumoRNG>, readOnly: bool) -> Ptr<MSVehicleType> {
        match self.myVTypeDict.get(id).copied() {
            None => match self.myVTypeDistDict.get(id).copied() {
                None => Ptr::null(),
                Some(dist) => dist
                    .m()
                    .get(if rng.is_null() { None } else { Some(rng.m()) })
                    .unwrap_or_else(|e| panic!("{}", e)),
            },
            Some(vt) => {
                if !readOnly && self.myReplaceableDefaultVTypes.remove(id) {
                    vt.m().check();
                }
                vt
            }
        }
    }

    /// C++: `void MSVehicleControl::insertVTypeIDs(std::vector<std::string> &) const` [const]
    /// Inserts ids of all known vehicle types and vehicle type distributions to the given vector
    // void
    // MSVehicleControl::insertVTypeIDs(std::vector<std::string>& into) const {
    //     into.reserve(into.size() + myVTypeDict.size() + myVTypeDistDict.size());
    //     for (VTypeDictType::const_iterator i = myVTypeDict.begin(); i != myVTypeDict.end(); ++i) {
    //         into.push_back((*i).first);
    //     }
    //     for (VTypeDistDictType::const_iterator i = myVTypeDistDict.begin(); i != myVTypeDistDict.end(); ++i) {
    //         into.push_back((*i).first);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn insert_v_type_i_ds(&self, into: &mut Vec<String>) {
        into.reserve(into.len() + self.myVTypeDict.len() + self.myVTypeDistDict.len());
        for id in self.myVTypeDict.keys() {
            into.push(id.clone());
        }
        for id in self.myVTypeDistDict.keys() {
            into.push(id.clone());
        }
    }

    /// C++: `const std::set<std::string> MSVehicleControl::getVTypeDistributionMembership(const std::string &) const` [const]
    /// Return the distribution IDs the vehicle type is a member of
    // const std::set<std::string>
    // MSVehicleControl::getVTypeDistributionMembership(const std::string& id) const {
    //     std::map<std::string, std::set<std::string>>::const_iterator it = myVTypeToDist.find(id);
    //     if (it == myVTypeToDist.end()) {
    //         return std::set<std::string>();
    //     }
    //     return it->second;
    // }
    #[allow(unused_variables)]
    pub fn get_v_type_distribution_membership(&self, id: &str) -> BTreeSet<String> {
        self.myVTypeToDist.get(id).cloned().unwrap_or_default()
    }

    /// C++: `const RandomDistributor<MSVehicleType *> * MSVehicleControl::getVTypeDistribution(const std::string &) const` [const]
    /// return the vehicle type distribution with the given id
    // const RandomDistributor<MSVehicleType*>*
    // MSVehicleControl::getVTypeDistribution(const std::string& typeDistID) const {
    //     const auto it = myVTypeDistDict.find(typeDistID);
    //     if (it != myVTypeDistDict.end()) {
    //         return it->second;
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_v_type_distribution(&self, typeDistID: &str) -> Ptr<RandomDistributor<Ptr<MSVehicleType>>> {
        self.myVTypeDistDict.get(typeDistID).copied().unwrap_or(Ptr::null())
    }

    /// C++: `const std::vector<MSVehicleType *> MSVehicleControl::getPedestrianTypes() const` [const]
    /// Return all pedestrian vehicle types.
    // const std::vector<MSVehicleType*>
    // MSVehicleControl::getPedestrianTypes(void) const {
    //     std::vector<MSVehicleType*> pedestrianTypes;
    //     for (auto const& e : myVTypeDict)
    //         if (e.second->getVehicleClass() == SUMOVehicleClass::SVC_PEDESTRIAN) {
    //             pedestrianTypes.push_back(e.second);
    //         }
    //     return pedestrianTypes;
    // }
    #[allow(unused_variables)]
    pub fn get_pedestrian_types(&self) -> Vec<Ptr<MSVehicleType>> {
        self.myVTypeDict
            .values()
            .filter(|t| t.get_vehicle_class() == SVC_PEDESTRIAN)
            .copied()
            .collect()
    }

    /// C++: `void MSVehicleControl::registerOneWaiting()`
    /// increases the count of vehicles waiting for a transport to allow recognition of person / container related deadlocks
    // void registerOneWaiting() {
    //         myWaitingForTransportable++;
    //     }
    #[allow(unused_variables)]
    pub fn register_one_waiting(&mut self) {
        self.myWaitingForTransportable += 1;
    }

    /// C++: `void MSVehicleControl::unregisterOneWaiting()`
    /// decreases the count of vehicles waiting for a transport to allow recognition of person / container related deadlocks
    // void unregisterOneWaiting() {
    //         myWaitingForTransportable--;
    //     }
    #[allow(unused_variables)]
    pub fn unregister_one_waiting(&mut self) {
        self.myWaitingForTransportable -= 1;
    }

    /// C++: `void MSVehicleControl::countCollision(bool)`
    /// registers one collision-related teleport
    // void countCollision(bool teleport) {
    //         myCollisions++;
    //         if (teleport) {
    //             myTeleportsCollision++;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn count_collision(&mut self, teleport: bool) {
        self.myCollisions += 1;
        if teleport {
            self.myTeleportsCollision += 1;
        }
    }

    /// C++: `void MSVehicleControl::registerTeleportJam()`
    /// register one non-collision-related teleport
    // void registerTeleportJam() {
    //         myTeleportsJam++;
    //     }
    #[allow(unused_variables)]
    pub fn register_teleport_jam(&mut self) {
        self.myTeleportsJam += 1;
    }

    /// C++: `void MSVehicleControl::registerTeleportYield()`
    /// register one non-collision-related teleport
    // void registerTeleportYield() {
    //         myTeleportsYield++;
    //     }
    #[allow(unused_variables)]
    pub fn register_teleport_yield(&mut self) {
        self.myTeleportsYield += 1;
    }

    /// C++: `void MSVehicleControl::registerTeleportWrongLane()`
    /// register one non-collision-related teleport
    // void registerTeleportWrongLane() {
    //         myTeleportsWrongLane++;
    //     }
    #[allow(unused_variables)]
    pub fn register_teleport_wrong_lane(&mut self) {
        self.myTeleportsWrongLane += 1;
    }

    /// C++: `void MSVehicleControl::registerEmergencyStop()`
    /// register emergency stop
    // void registerEmergencyStop() {
    //         myEmergencyStops++;
    //     }
    #[allow(unused_variables)]
    pub fn register_emergency_stop(&mut self) {
        self.myEmergencyStops += 1;
    }

    /// C++: `void MSVehicleControl::registerEmergencyBraking()`
    /// register emergency stop
    // void registerEmergencyBraking() {
    //         myEmergencyBrakingCount++;
    //     }
    #[allow(unused_variables)]
    pub fn register_emergency_braking(&mut self) {
        self.myEmergencyBrakingCount += 1;
    }

    /// C++: `void MSVehicleControl::registerStopStarted()`
    /// register emergency stop
    // void registerStopStarted() {
    //         myStoppedVehicles++;
    //     }
    #[allow(unused_variables)]
    pub fn register_stop_started(&mut self) {
        self.myStoppedVehicles += 1;
    }

    /// C++: `void MSVehicleControl::registerStopEnded()`
    /// register emergency stop
    // void registerStopEnded() {
    //         myStoppedVehicles--;
    //     }
    #[allow(unused_variables)]
    pub fn register_stop_ended(&mut self) {
        self.myStoppedVehicles -= 1;
    }

    /// C++: `void MSVehicleControl::setState(int, int, int, double, double, double, double)`
    /// Sets the current state variables as loaded from the stream
    // void
    // MSVehicleControl::setState(int runningVehNo, int loadedVehNo, int endedVehNo, double totalDepartureDelay, double totalTravelTime, double maxSpeedFactor, double minDecel) {
    //     myRunningVehNo = runningVehNo;
    //     myLoadedVehNo = loadedVehNo;
    //     myEndedVehNo = endedVehNo;
    //     myTotalDepartureDelay = totalDepartureDelay;
    //     myTotalTravelTime = totalTravelTime;
    //     myMaxSpeedFactor = maxSpeedFactor;
    //     myMinDeceleration = minDecel;
    // }
    #[allow(unused_variables)]
    pub fn set_state(&mut self, runningVehNo: i32, loadedVehNo: i32, endedVehNo: i32, totalDepartureDelay: f64, totalTravelTime: f64, maxSpeedFactor: f64, minDecel: f64) {
        self.myRunningVehNo = runningVehNo;
        self.myLoadedVehNo = loadedVehNo;
        self.myEndedVehNo = endedVehNo;
        self.myTotalDepartureDelay = totalDepartureDelay;
        self.myTotalTravelTime = totalTravelTime;
        self.myMaxSpeedFactor = maxSpeedFactor;
        self.myMinDeceleration = minDecel;
    }

    /// C++: `void MSVehicleControl::saveState(OutputDevice &)`
    /// Saves the current state into the given stream
    // void
    // MSVehicleControl::saveState(OutputDevice& out) {
    //     out.openTag(SUMO_TAG_DELAY);
    //     out.writeAttr(SUMO_ATTR_NUMBER, myRunningVehNo);
    //     out.writeAttr(SUMO_ATTR_BEGIN, myLoadedVehNo);
    //     out.writeAttr(SUMO_ATTR_END, myEndedVehNo);
    //     out.writeAttr(SUMO_ATTR_DEPART, myTotalDepartureDelay);
    //     out.writeAttr(SUMO_ATTR_TIME, myTotalTravelTime);
    //     out.writeAttr(SUMO_ATTR_SPEEDFACTOR, myMaxSpeedFactor);
    //     out.writeAttr(SUMO_ATTR_DECEL, myMinDeceleration);
    //     const SUMOTime loaderTime = MSNet::getInstance()->getLoaderTime();
    //     if (loaderTime > 0 && loaderTime != SUMOTime_MAX) {
    //         out.writeAttr(SUMO_ATTR_LOADERTIME, MSNet::getInstance()->getLoaderTime());
    //     }
    //     out.closeTag();
    //     // save vehicle types
    //     for (const auto& item : myVTypeDict) {
    //         if (myReplaceableDefaultVTypes.count(item.first) == 0) {
    //             item.second->getParameter().write(out);
    //         }
    //     }
    //     for (const auto& item : myVTypeDistDict) {
    //         out.openTag(SUMO_TAG_VTYPE_DISTRIBUTION).writeAttr(SUMO_ATTR_ID, item.first);
    //         out.writeAttr(SUMO_ATTR_VTYPES, item.second->getVals());
    //         out.writeAttr(SUMO_ATTR_PROBS, item.second->getProbs());
    //         out.closeTag();
    //     }
    //     std::vector<SUMOVehicle*> sortedVehs;
    //     for (const auto& item : myVehicleDict) {
    //         sortedVehs.push_back(item.second);
    //     }
    //     std::sort(sortedVehs.begin(), sortedVehs.end(), ComparatorNumericalIdLess());
    //     for (SUMOVehicle* veh : sortedVehs) {
    //         veh->saveState(out);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        out.open_tag_id(SUMO_TAG_DELAY);
        out.write_attr(SUMO_ATTR_NUMBER, &self.myRunningVehNo);
        out.write_attr(SUMO_ATTR_BEGIN, &self.myLoadedVehNo);
        out.write_attr(SUMO_ATTR_END, &self.myEndedVehNo);
        out.write_attr(SUMO_ATTR_DEPART, &self.myTotalDepartureDelay);
        out.write_attr(SUMO_ATTR_TIME, &self.myTotalTravelTime);
        out.write_attr(SUMO_ATTR_SPEEDFACTOR, &self.myMaxSpeedFactor);
        out.write_attr(SUMO_ATTR_DECEL, &self.myMinDeceleration);
        let loaderTime = MSNet::get_instance().m().get_loader_time();
        if loaderTime > 0 && loaderTime != SUMOTIME_MAX {
            out.write_attr(SUMO_ATTR_LOADERTIME, &MSNet::get_instance().m().get_loader_time());
        }
        out.close_tag("");
        // save vehicle types
        for (id, item) in self.myVTypeDict.iter() {
            if !self.myReplaceableDefaultVTypes.contains(id) {
                item.get_parameter().write(out);
            }
        }
        for (id, item) in self.myVTypeDistDict.iter() {
            out.open_tag_id(SUMO_TAG_VTYPE_DISTRIBUTION).write_attr(SUMO_ATTR_ID, id.as_str());
            let vals: Vec<String> = item.get_vals().iter().map(|t| t.get_id().to_string()).collect();
            out.write_attr(SUMO_ATTR_VTYPES, vals.join(" ").as_str());
            let probs: Vec<String> = item.get_probs().iter().map(|p| f64_to_string(*p)).collect();
            out.write_attr(SUMO_ATTR_PROBS, probs.join(" ").as_str());
            out.close_tag("");
        }
        let mut sortedVehs: Vec<Ptr<SUMOVehicle>> = self.myVehicleDict.values().copied().collect();
        // determinism: ComparatorNumericalIdLess; numerical ids are unique
        sortedVehs.sort_by_key(|v| as_base(*v).get_numerical_id());
        for veh in sortedVehs {
            veh.m().save_state(out);
        }
    }

    /// C++: `void MSVehicleControl::clearState(const bool)`
    /// Remove all vehicles before quick-loading state
    // void
    // MSVehicleControl::clearState(const bool reinit) {
    //     for (const auto& item : myVehicleDict) {
    //         delete item.second;
    //     }
    //     myVehicleDict.clear();
    //     // delete vehicle type distributions
    //     for (const auto& item : myVTypeDistDict) {
    //         delete item.second;
    //     }
    //     myVTypeDistDict.clear();
    //     // delete vehicle types
    //     for (const auto& item : myVTypeDict) {
    //         delete item.second;
    //     }
    //     myVTypeDict.clear();
    //     myPendingRemovals.clear(); // could be leftovers from MSVehicleTransfer::checkInsertions (teleport beyond arrival)
    //     if (reinit) {
    //         initDefaultTypes();
    //     }
    //     myLoadedVehNo = 0;
    //     myRunningVehNo = 0;
    //     myEndedVehNo = 0;
    //     myDiscarded = 0;
    //     myCollisions = 0;
    //     myTeleportsCollision = 0;
    //     myTeleportsJam = 0;
    //     myTeleportsYield = 0;
    //     myTeleportsWrongLane = 0;
    //     myEmergencyStops = 0;
    //     myEmergencyBrakingCount = 0;
    //     myStoppedVehicles = 0;
    //     myTotalDepartureDelay = 0;
    //     myTotalTravelTime = 0;
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self, reinit: bool) {
        // shortcut: the vehicle destructors (delete_vehicle_object) are not run here: C++ runs ~MSTransportable first
        // (MSTransportableControl::clearState removes riders from their vehicles), which the Rust transportable clearState
        // does not do yet, so ~MSDevice_Transportable would warn about riders C++ already removed; upgrade with that port
        self.myVehicleDict.clear();
        self.myVTypeDistDict.clear();
        self.myVTypeDict.clear();
        self.myPendingRemovals.clear(); // could be leftovers from MSVehicleTransfer::checkInsertions (teleport beyond arrival)
        if reinit {
            self.init_default_types();
        }
        self.myLoadedVehNo = 0;
        self.myRunningVehNo = 0;
        self.myEndedVehNo = 0;
        self.myDiscarded = 0;
        self.myCollisions = 0;
        self.myTeleportsCollision = 0;
        self.myTeleportsJam = 0;
        self.myTeleportsYield = 0;
        self.myTeleportsWrongLane = 0;
        self.myEmergencyStops = 0;
        self.myEmergencyBrakingCount = 0;
        self.myStoppedVehicles = 0;
        self.myTotalDepartureDelay = 0.;
        self.myTotalTravelTime = 0.;
    }

    /// C++: `void MSVehicleControl::discountStateRemoved(int)`
    /// discount vehicles that were removed during state loading
    // void discountStateRemoved(int n) {
    //         myRunningVehNo -= n;
    //         myDiscarded += n;
    //         myEndedVehNo += n;
    //     }
    #[allow(unused_variables)]
    pub fn discount_state_removed(&mut self, n: i32) {
        self.myRunningVehNo -= n;
        self.myDiscarded += n;
        self.myEndedVehNo += n;
    }

    /// C++: `void MSVehicleControl::discountRoutingVehicle()`
    // void discountRoutingVehicle() {
    //         myLoadedVehNo--;
    //         myEndedVehNo--;
    //         myDiscarded--;
    //     }
    #[allow(unused_variables)]
    pub fn discount_routing_vehicle(&mut self) {
        self.myLoadedVehNo -= 1;
        self.myEndedVehNo -= 1;
        self.myDiscarded -= 1;
    }

    /// C++: `void MSVehicleControl::abortWaiting()`
    /// informes about all waiting vehicles (deletion in destructor)
    // void
    // MSVehicleControl::abortWaiting() {
    //     for (VehicleDictType::iterator i = myVehicleDict.begin(); i != myVehicleDict.end(); ++i) {
    //         SUMOVehicle* veh = i->second;
    //         std::string waitReason;
    //         if (veh->isStoppedTriggered()) {
    //             const MSStop& stop = veh->getNextStop();
    //             if (stop.triggered) {
    //                 waitReason = "for a person that will never come";
    //             } else if (stop.containerTriggered) {
    //                 waitReason = "for a container that will never come";
    //             } else if (stop.joinTriggered) {
    //                 if (stop.pars.join != "") {
    //                     waitReason = "to be joined to vehicle '" + stop.pars.join + "'";
    //                 } else {
    //                     waitReason = "for a joining vehicle that will never come";
    //                 }
    //             } else {
    //                 waitReason = "for an unknown trigger";
    //             }
    //         } else if (!veh->hasDeparted()) {
    //             if (veh->getParameter().departProcedure == DepartDefinition::SPLIT) {
    //                 waitReason = "for a train from which to split";
    //             } else if (veh->getParameter().departProcedure == DepartDefinition::TRIGGERED) {
    //                 waitReason = "at insertion for a person that will never come";
    //             } else if (veh->getParameter().departProcedure == DepartDefinition::CONTAINER_TRIGGERED) {
    //                 waitReason = "at insertion for a container that will never come";
    //             } else {
    //                 waitReason = "for an unknown departure trigger";
    //             }
    //         } else {
    //             waitReason = "for an unknown reason";
    //         }
    //         WRITE_WARNINGF(TL("Vehicle '%' aborted waiting %."), i->first, waitReason);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn abort_waiting(&mut self) {
        for (id, veh) in self.myVehicleDict.iter() {
            let mut waitReason = String::new();
            if veh.is_stopped_triggered() {
                let stop = veh.get_next_stop();
                if stop.triggered {
                    waitReason = "for a person that will never come".to_string();
                } else if stop.containerTriggered {
                    waitReason = "for a container that will never come".to_string();
                } else if stop.joinTriggered {
                    if !stop.pars.join.is_empty() {
                        waitReason = format!("to be joined to vehicle '{}'", stop.pars.join);
                    } else {
                        waitReason = "for a joining vehicle that will never come".to_string();
                    }
                } else {
                    waitReason = "for an unknown trigger".to_string();
                }
            } else if !veh.has_departed() {
                let departProcedure = as_base(*veh).get_parameter().departProcedure;
                if departProcedure == DepartDefinition::SPLIT {
                    waitReason = "for a train from which to split".to_string();
                } else if departProcedure == DepartDefinition::TRIGGERED {
                    waitReason = "at insertion for a person that will never come".to_string();
                } else if departProcedure == DepartDefinition::CONTAINER_TRIGGERED {
                    waitReason = "at insertion for a container that will never come".to_string();
                } else {
                    waitReason = "for an unknown departure trigger".to_string();
                }
            } else {
                waitReason = "for an unknown reason".to_string();
            }
            utils::write_warningf!("Vehicle '%' aborted waiting %.", id.as_str(), waitReason.as_str());
        }
    }

    /// C++: `double MSVehicleControl::getMaxSpeedFactor() const` [const]
    /// return the maximum speed factor for all vehicles that ever entered the network
    // double getMaxSpeedFactor() const {
    //         return myMaxSpeedFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_speed_factor(&self) -> f64 {
        self.myMaxSpeedFactor
    }

    /// C++: `double MSVehicleControl::getMinDeceleration() const` [const]
    /// return the minimum deceleration capability for all road vehicles that ever entered the network
    // double getMinDeceleration() const {
    //         return myMinDeceleration;
    //     }
    #[allow(unused_variables)]
    pub fn get_min_deceleration(&self) -> f64 {
        self.myMinDeceleration
    }

    /// C++: `double MSVehicleControl::getMinDecelerationRail() const` [const]
    /// return the minimum deceleration capability for all ral vehicles that ever entered the network
    // double getMinDecelerationRail() const {
    //         return myMinDecelerationRail;
    //     }
    #[allow(unused_variables)]
    pub fn get_min_deceleration_rail(&self) -> f64 {
        self.myMinDecelerationRail
    }

    /// C++: `double MSVehicleControl::getMaxMinGap() const` [const]
    /// return the maximum minGap for all vehicles that ever entered the network
    // double getMaxMinGap() const {
    //         return myMaxMinGap;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_min_gap(&self) -> f64 {
        self.myMaxMinGap
    }

    /// C++: `void MSVehicleControl::adaptIntermodalRouter(MSTransportableRouter &) const` [const]
    // void
    // MSVehicleControl::adaptIntermodalRouter(MSTransportableRouter& router) const {
    //     for (const SUMOVehicle* const veh : myPTVehicles) {
    //         // add single vehicles with line attribute which are not part of a flow
    //         ConstMSRoutePtr const route = MSRoute::dictionary(veh->getParameter().routeid);
    //         router.getNetwork()->addSchedule(veh->getParameter(), route == nullptr ? nullptr : &route->getStops());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn adapt_intermodal_router(&self, router: &mut MSTransportableRouter) {
        for veh in self.myPTVehicles.iter() {
            // add single vehicles with line attribute which are not part of a flow
            let bv = as_base(*veh);
            let pars = bv.get_parameter();
            let route: ConstMSRoutePtr = MSRoute::dictionary_3(&pars.routeid, Ptr::null());
            // an unknown route id yields MSRoute's null-route sentinel (C++ nullptr)
            let stops: Ptr<StopParVector> = if crate::microsim::ms_route::is_null_route(&route) { Ptr::null() } else { Ptr::new(route.get_stops()) };
            router.get_network().m().add_schedule(pars, stops);
        }
    }

    /// C++: `void MSVehicleControl::setScale(double)`
    /// sets the demand scaling factor
    // void setScale(double scale) {
    //         myScale = scale;
    //     }
    #[allow(unused_variables)]
    pub fn set_scale(&mut self, scale: f64) {
        self.myScale = scale;
    }

    /// C++: `double MSVehicleControl::getScale() const` [const]
    /// sets the demand scaling factor
    // double getScale() const {
    //         return myScale;
    //     }
    #[allow(unused_variables)]
    pub fn get_scale(&self) -> f64 {
        self.myScale
    }

    /// C++: `void MSVehicleControl::secureVehicles()` [virtual]
    /// lock access to vehicle removal/additions for thread synchronization
    // virtual void secureVehicles() {}
    #[allow(unused_variables)]
    pub fn secure_vehicles(&mut self) {
        // C++ virtual body is empty (locking is handled by subclasses)
    }

    /// C++: `void MSVehicleControl::releaseVehicles()` [virtual]
    /// unlock access to vehicle removal/additions for thread synchronization
    // virtual void releaseVehicles() {}
    #[allow(unused_variables)]
    pub fn release_vehicles(&mut self) {
        // C++ virtual body is empty (locking is handled by subclasses)
    }

    /// C++: `void MSVehicleControl::initDefaultTypes()` [private]
    /// create default types
    // void
    // MSVehicleControl::initDefaultTypes() {
    //     SUMOVTypeParameter defType(DEFAULT_VTYPE_ID, SVC_PASSENGER);
    //     myVTypeDict[DEFAULT_VTYPE_ID] = MSVehicleType::build(defType);
    //
    //     SUMOVTypeParameter defPedType(DEFAULT_PEDTYPE_ID, SVC_PEDESTRIAN);
    //     defPedType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //     myVTypeDict[DEFAULT_PEDTYPE_ID] = MSVehicleType::build(defPedType);
    //
    //     SUMOVTypeParameter defBikeType(DEFAULT_BIKETYPE_ID, SVC_BICYCLE);
    //     defBikeType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //     myVTypeDict[DEFAULT_BIKETYPE_ID] = MSVehicleType::build(defBikeType);
    //
    //     SUMOVTypeParameter defTaxiType(DEFAULT_TAXITYPE_ID, SVC_TAXI);
    //     defTaxiType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //     myVTypeDict[DEFAULT_TAXITYPE_ID] = MSVehicleType::build(defTaxiType);
    //
    //     SUMOVTypeParameter defRailType(DEFAULT_RAILTYPE_ID, SVC_RAIL);
    //     defRailType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //     myVTypeDict[DEFAULT_RAILTYPE_ID] = MSVehicleType::build(defRailType);
    //
    //     SUMOVTypeParameter defContainerType(DEFAULT_CONTAINERTYPE_ID, SVC_CONTAINER);
    //     defContainerType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //     myVTypeDict[DEFAULT_CONTAINERTYPE_ID] = MSVehicleType::build(defContainerType);
    //
    //     myReplaceableDefaultVTypes = DEFAULT_VTYPES;
    // }
    #[allow(unused_variables)]
    pub fn init_default_types(&mut self) {
        let mut defType = SUMOVTypeParameter::new(DEFAULT_VTYPE_ID, SVC_PASSENGER);
        self.myVTypeDict.insert(DEFAULT_VTYPE_ID.to_string(), MSVehicleType::build(&mut defType, ""));

        let mut defPedType = SUMOVTypeParameter::new(DEFAULT_PEDTYPE_ID, SVC_PEDESTRIAN);
        defPedType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
        self.myVTypeDict.insert(DEFAULT_PEDTYPE_ID.to_string(), MSVehicleType::build(&mut defPedType, ""));

        let mut defBikeType = SUMOVTypeParameter::new(DEFAULT_BIKETYPE_ID, SVC_BICYCLE);
        defBikeType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
        self.myVTypeDict.insert(DEFAULT_BIKETYPE_ID.to_string(), MSVehicleType::build(&mut defBikeType, ""));

        let mut defTaxiType = SUMOVTypeParameter::new(DEFAULT_TAXITYPE_ID, SVC_TAXI);
        defTaxiType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
        self.myVTypeDict.insert(DEFAULT_TAXITYPE_ID.to_string(), MSVehicleType::build(&mut defTaxiType, ""));

        let mut defRailType = SUMOVTypeParameter::new(DEFAULT_RAILTYPE_ID, SVC_RAIL);
        defRailType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
        self.myVTypeDict.insert(DEFAULT_RAILTYPE_ID.to_string(), MSVehicleType::build(&mut defRailType, ""));

        let mut defContainerType = SUMOVTypeParameter::new(DEFAULT_CONTAINERTYPE_ID, SVC_CONTAINER);
        defContainerType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
        self.myVTypeDict.insert(DEFAULT_CONTAINERTYPE_ID.to_string(), MSVehicleType::build(&mut defContainerType, ""));

        self.myReplaceableDefaultVTypes = DEFAULT_VTYPES.iter().map(|s| s.to_string()).collect();
    }

    /// C++: `bool MSVehicleControl::checkVType(const std::string &)` [private]
    /// Checks whether the vehicle type (distribution) may be added
    // bool
    // MSVehicleControl::checkVType(const std::string& id) {
    //     if (myReplaceableDefaultVTypes.erase(id) > 0) {
    //         delete myVTypeDict[id];
    //         myVTypeDict.erase(myVTypeDict.find(id));
    //     } else {
    //         if (myVTypeDict.find(id) != myVTypeDict.end() || myVTypeDistDict.find(id) != myVTypeDistDict.end()) {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn check_v_type(&mut self, id: &str) -> bool {
        if self.myReplaceableDefaultVTypes.remove(id) {
            // shortcut: C++ `delete myVTypeDict[id]` is not done, see clearState
            self.myVTypeDict.remove(id);
        } else if self.myVTypeDict.contains_key(id) || self.myVTypeDistDict.contains_key(id) {
            return false;
        }
        true
    }

    /// C++: `bool MSVehicleControl::isPendingRemoval(SUMOVehicle *)` [private]
    /// whether the given vehicle is scheduled for removal
    // bool
    // MSVehicleControl::isPendingRemoval(SUMOVehicle* veh) {
    // #ifdef HAVE_FOX
    //     return myPendingRemovals.contains(veh);
    // #else
    //     return std::find(myPendingRemovals.begin(), myPendingRemovals.end(), veh) == myPendingRemovals.end();
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn is_pending_removal(&mut self, veh: Ptr<SUMOVehicle>) -> bool {
        self.myPendingRemovals.contains(&veh)
    }

    /// C++: `void MSVehicleControl::initVehicle(MSBaseVehicle *, const bool, bool, const VehicleDefinitionSource)` [protected]
    // void
    // MSVehicleControl::initVehicle(MSBaseVehicle* built, const bool ignoreStopErrors, bool addRouteStops, const VehicleDefinitionSource source) {
    //     myLoadedVehNo++;
    //     try {
    //         built->initDevices();
    //         if (source != VehicleDefinitionSource::STATE) {
    //             built->addStops(ignoreStopErrors, nullptr, addRouteStops);
    //         }
    //     } catch (ProcessError&) {
    //         delete built;
    //         throw;
    //     }
    //     MSNet::getInstance()->informVehicleStateListener(built, MSNet::VehicleState::BUILT);
    // }
    #[allow(unused_variables)]
    pub fn init_vehicle(&mut self, built: Ptr<MSBaseVehicle>, ignoreStopErrors: bool, addRouteStops: bool, source: MSVehicleControl_VehicleDefinitionSource) {
        self.myLoadedVehNo += 1;
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            // C++ calls the virtual initDevices(); MEVehicle does not override it, so meso vehicles get MSBaseVehicle's
            if crate::microsim::ms_globals::use_meso_sim() {
                built.m().init_devices();
            } else {
                built.cast::<MSVehicle>().m().init_devices();
            }
            if source != MSVehicleControl_VehicleDefinitionSource::STATE {
                built.m().add_stops(ignoreStopErrors, Ptr::null(), addRouteStops);
            }
        }));
        if let Err(e) = res {
            // C++ catches ProcessError to `delete built` (destructor side effects; the memory stays leaked) and rethrows
            delete_vehicle_object(Ptr::from_mut(&mut built.m().base));
            std::panic::resume_unwind(e);
        }
        MSNet::get_instance().m().inform_vehicle_state_listener(Ptr::from_mut(&mut built.m().base), MSNet_VehicleState::BUILT, "");
    }

    /// C++: `void MSVehicleControl::MSVehicleControl(const MSVehicleControl &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &MSVehicleControl) -> Self {
        invalid_copy()
    }

    /// C++: `MSVehicleControl & MSVehicleControl::operator=(const MSVehicleControl &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSVehicleControl) -> &mut MSVehicleControl {
        invalid_copy()
    }

}

