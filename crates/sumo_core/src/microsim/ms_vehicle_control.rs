//! Port of microsim/MSVehicleControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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

impl MSVehicleControl_DeleteKeptVehicle {
    /// C++: `void DeleteKeptVehicle::DeleteKeptVehicle(SUMOVehicle *)`
    // DeleteKeptVehicle(SUMOVehicle* veh) : myVehicle(veh) {}
    #[allow(unused_variables)]
    pub fn new(veh: Ptr<SUMOVehicle>) -> Self {
        todo!("port DeleteKeptVehicle::DeleteKeptVehicle")
    }

    /// C++: `void DeleteKeptVehicle::~DeleteKeptVehicle() noexcept` [virtual]
    // ~DeleteKeptVehicle() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port DeleteKeptVehicle::~DeleteKeptVehicle")
    }

    /// C++: `SUMOTime DeleteKeptVehicle::execute(SUMOTime)` [virtual]
    // SUMOTime
    // MSVehicleControl::DeleteKeptVehicle::execute(SUMOTime /*currentTime*/) {
    //     MSNet::getInstance()->getVehicleControl().deleteVehicle(myVehicle, false, true);
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        todo!("port DeleteKeptVehicle::execute")
    }

    /// C++: `DeleteKeptVehicle & DeleteKeptVehicle::operator=(const DeleteKeptVehicle &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSVehicleControl_DeleteKeptVehicle) -> &mut MSVehicleControl_DeleteKeptVehicle {
        todo!("port DeleteKeptVehicle::operator=")
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
    pub myVTypeDistDict: RONet_VTypeDistDictType,
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
}

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
        todo!("port MSVehicleControl::MSVehicleControl")
    }

    /// C++: `void MSVehicleControl::~MSVehicleControl()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSVehicleControl::~MSVehicleControl")
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
        todo!("port MSVehicleControl::buildVehicle")
    }

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
        todo!("port MSVehicleControl::addVehicle")
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
        todo!("port MSVehicleControl::getVehicle")
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
    pub fn delete_vehicle(&mut self, v: Ptr<SUMOVehicle>, discard: bool, wasKept: bool) {
        todo!("port MSVehicleControl::deleteVehicle")
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
        todo!("port MSVehicleControl::deleteKeptVehicle")
    }

    /// C++: `void MSVehicleControl::fixVehicleCounts()`
    // void fixVehicleCounts() {
    //         myLoadedVehNo++;
    //         myEndedVehNo++;
    //         myDiscarded++;
    //     }
    #[allow(unused_variables)]
    pub fn fix_vehicle_counts(&mut self) {
        todo!("port MSVehicleControl::fixVehicleCounts")
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
        todo!("port MSVehicleControl::scheduleVehicleRemoval")
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
        todo!("port MSVehicleControl::removePending")
    }

    /// C++: `constVehIt MSVehicleControl::loadedVehBegin() const` [const]
    /// Returns the begin of the internal vehicle map
    // constVehIt loadedVehBegin() const {
    //         return myVehicleDict.begin();
    //     }
    #[allow(unused_variables)]
    pub fn loaded_veh_begin(&self) -> MSVehicleControl_constVehIt {
        todo!("port MSVehicleControl::loadedVehBegin")
    }

    /// C++: `constVehIt MSVehicleControl::loadedVehEnd() const` [const]
    /// Returns the end of the internal vehicle map
    // constVehIt loadedVehEnd() const {
    //         return myVehicleDict.end();
    //     }
    #[allow(unused_variables)]
    pub fn loaded_veh_end(&self) -> MSVehicleControl_constVehIt {
        todo!("port MSVehicleControl::loadedVehEnd")
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
        todo!("port MSVehicleControl::handleTriggeredDepart")
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
        todo!("port MSVehicleControl::vehicleDeparted")
    }

    /// C++: `int MSVehicleControl::getLoadedVehicleNo() const` [const]
    /// Returns the number of build vehicles
    // int getLoadedVehicleNo() const {
    //         return myLoadedVehNo;
    //     }
    #[allow(unused_variables)]
    pub fn get_loaded_vehicle_no(&self) -> i32 {
        todo!("port MSVehicleControl::getLoadedVehicleNo")
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
        todo!("port MSVehicleControl::getHaltingVehicleNo")
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
        todo!("port MSVehicleControl::getVehicleMeanSpeeds")
    }

    /// C++: `double MSVehicleControl::getVehicleMeanSpeed() const` [const]
    // double getVehicleMeanSpeed() const {
    //         return getVehicleMeanSpeeds().first;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_mean_speed(&self) -> f64 {
        todo!("port MSVehicleControl::getVehicleMeanSpeed")
    }

    /// C++: `double MSVehicleControl::getVehicleMeanSpeedRelative() const` [const]
    // double getVehicleMeanSpeedRelative() const {
    //         return getVehicleMeanSpeeds().second;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_mean_speed_relative(&self) -> f64 {
        todo!("port MSVehicleControl::getVehicleMeanSpeedRelative")
    }

    /// C++: `int MSVehicleControl::getEndedVehicleNo() const` [const]
    /// Returns the number of removed vehicles
    // int getEndedVehicleNo() const {
    //         return myEndedVehNo;
    //     }
    #[allow(unused_variables)]
    pub fn get_ended_vehicle_no(&self) -> i32 {
        todo!("port MSVehicleControl::getEndedVehicleNo")
    }

    /// C++: `int MSVehicleControl::getArrivedVehicleNo() const` [const]
    /// Returns the number of arrived vehicles
    // int getArrivedVehicleNo() const {
    //         return myEndedVehNo - myDiscarded;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrived_vehicle_no(&self) -> i32 {
        todo!("port MSVehicleControl::getArrivedVehicleNo")
    }

    /// C++: `int MSVehicleControl::getDiscardedVehicleNo() const` [const]
    /// Returns the number of discarded vehicles
    // int getDiscardedVehicleNo() const {
    //         return myDiscarded;
    //     }
    #[allow(unused_variables)]
    pub fn get_discarded_vehicle_no(&self) -> i32 {
        todo!("port MSVehicleControl::getDiscardedVehicleNo")
    }

    /// C++: `int MSVehicleControl::getRunningVehicleNo() const` [const]
    /// Returns the number of build and inserted, but not yet deleted vehicles
    // int getRunningVehicleNo() const {
    //         return myRunningVehNo;
    //     }
    #[allow(unused_variables)]
    pub fn get_running_vehicle_no(&self) -> i32 {
        todo!("port MSVehicleControl::getRunningVehicleNo")
    }

    /// C++: `int MSVehicleControl::getDepartedVehicleNo() const` [const]
    /// Returns the number of inserted vehicles
    // int getDepartedVehicleNo() const {
    //         return myRunningVehNo + myEndedVehNo - myDiscarded;
    //     }
    #[allow(unused_variables)]
    pub fn get_departed_vehicle_no(&self) -> i32 {
        todo!("port MSVehicleControl::getDepartedVehicleNo")
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
        todo!("port MSVehicleControl::getQuota")
    }

    /// C++: `int MSVehicleControl::getActiveVehicleCount() const` [const]
    /// Returns the number of build vehicles that have not been removed or need to wait for a passenger or a container
    // int getActiveVehicleCount() const {
    //         return myLoadedVehNo - (myWaitingForTransportable + myEndedVehNo);
    //     }
    #[allow(unused_variables)]
    pub fn get_active_vehicle_count(&self) -> i32 {
        todo!("port MSVehicleControl::getActiveVehicleCount")
    }

    /// C++: `int MSVehicleControl::getTriggeredVehicleCount() const` [const]
    /// return the number of vehicles that are waiting for a transportable or a join
    // int getTriggeredVehicleCount() const {
    //         return myWaitingForTransportable;
    //     }
    #[allow(unused_variables)]
    pub fn get_triggered_vehicle_count(&self) -> i32 {
        todo!("port MSVehicleControl::getTriggeredVehicleCount")
    }

    /// C++: `int MSVehicleControl::getCollisionCount() const` [const]
    /// return the number of collisions
    // int getCollisionCount() const {
    //         return myCollisions;
    //     }
    #[allow(unused_variables)]
    pub fn get_collision_count(&self) -> i32 {
        todo!("port MSVehicleControl::getCollisionCount")
    }

    /// C++: `int MSVehicleControl::getTeleportsCollisions() const` [const]
    /// return the number of collisions
    // int getTeleportsCollisions() const {
    //         return myTeleportsCollision;
    //     }
    #[allow(unused_variables)]
    pub fn get_teleports_collisions(&self) -> i32 {
        todo!("port MSVehicleControl::getTeleportsCollisions")
    }

    /// C++: `int MSVehicleControl::getTeleportsJam() const` [const]
    /// return the number of teleports due to jamming
    // int getTeleportsJam() const {
    //         return myTeleportsJam;
    //     }
    #[allow(unused_variables)]
    pub fn get_teleports_jam(&self) -> i32 {
        todo!("port MSVehicleControl::getTeleportsJam")
    }

    /// C++: `int MSVehicleControl::getTeleportsYield() const` [const]
    /// return the number of teleports due to vehicles stuck on a minor road
    // int getTeleportsYield() const {
    //         return myTeleportsYield;
    //     }
    #[allow(unused_variables)]
    pub fn get_teleports_yield(&self) -> i32 {
        todo!("port MSVehicleControl::getTeleportsYield")
    }

    /// C++: `int MSVehicleControl::getTeleportsWrongLane() const` [const]
    /// return the number of teleports due to vehicles stuck on the wrong lane
    // int getTeleportsWrongLane() const {
    //         return myTeleportsWrongLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_teleports_wrong_lane(&self) -> i32 {
        todo!("port MSVehicleControl::getTeleportsWrongLane")
    }

    /// C++: `int MSVehicleControl::getTeleportCount() const` [const]
    /// return the number of teleports (including collisions)
    // int
    // MSVehicleControl::getTeleportCount() const {
    //     return myTeleportsCollision + myTeleportsJam + myTeleportsYield + myTeleportsWrongLane;
    // }
    #[allow(unused_variables)]
    pub fn get_teleport_count(&self) -> i32 {
        todo!("port MSVehicleControl::getTeleportCount")
    }

    /// C++: `int MSVehicleControl::getEmergencyStops() const` [const]
    /// return the number of emergency stops
    // int getEmergencyStops() const {
    //         return myEmergencyStops;
    //     }
    #[allow(unused_variables)]
    pub fn get_emergency_stops(&self) -> i32 {
        todo!("port MSVehicleControl::getEmergencyStops")
    }

    /// C++: `int MSVehicleControl::getEmergencyBrakingCount() const` [const]
    /// return the number of emergency stops
    // int getEmergencyBrakingCount() const {
    //         return myEmergencyBrakingCount;
    //     }
    #[allow(unused_variables)]
    pub fn get_emergency_braking_count(&self) -> i32 {
        todo!("port MSVehicleControl::getEmergencyBrakingCount")
    }

    /// C++: `int MSVehicleControl::getStoppedVehiclesCount() const` [const]
    /// return the number of vehicles that are currently stopped
    // int getStoppedVehiclesCount() const {
    //         return myStoppedVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn get_stopped_vehicles_count(&self) -> i32 {
        todo!("port MSVehicleControl::getStoppedVehiclesCount")
    }

    /// C++: `double MSVehicleControl::getTotalDepartureDelay() const` [const]
    /// Returns the total departure delay
    // double getTotalDepartureDelay() const {
    //         return myTotalDepartureDelay;
    //     }
    #[allow(unused_variables)]
    pub fn get_total_departure_delay(&self) -> f64 {
        todo!("port MSVehicleControl::getTotalDepartureDelay")
    }

    /// C++: `double MSVehicleControl::getTotalTravelTime() const` [const]
    /// Returns the total travel time
    // double getTotalTravelTime() const {
    //         return myTotalTravelTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_total_travel_time(&self) -> f64 {
        todo!("port MSVehicleControl::getTotalTravelTime")
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
        todo!("port MSVehicleControl::addVType")
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
        todo!("port MSVehicleControl::removeVType")
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
        todo!("port MSVehicleControl::addVTypeDistribution")
    }

    /// C++: `bool MSVehicleControl::hasVType(const std::string &) const` [const]
    /// Asks for existence of a vehicle type
    // bool
    // MSVehicleControl::hasVType(const std::string& id) const {
    //     return myVTypeDict.count(id) > 0 || myVTypeDistDict.count(id) > 0;
    // }
    #[allow(unused_variables)]
    pub fn has_v_type(&self, id: &str) -> bool {
        todo!("port MSVehicleControl::hasVType")
    }

    /// C++: `bool MSVehicleControl::hasVTypeDistribution(const std::string &) const` [const]
    /// Asks for a vehicle type distribution
    // bool
    // MSVehicleControl::hasVTypeDistribution(const std::string& id) const {
    //     return myVTypeDistDict.count(id) > 0;
    // }
    #[allow(unused_variables)]
    pub fn has_v_type_distribution(&self, id: &str) -> bool {
        todo!("port MSVehicleControl::hasVTypeDistribution")
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
        todo!("port MSVehicleControl::getVType")
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
        todo!("port MSVehicleControl::insertVTypeIDs")
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
        todo!("port MSVehicleControl::getVTypeDistributionMembership")
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
        todo!("port MSVehicleControl::getVTypeDistribution")
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
        todo!("port MSVehicleControl::getPedestrianTypes")
    }

    /// C++: `void MSVehicleControl::registerOneWaiting()`
    /// increases the count of vehicles waiting for a transport to allow recognition of person / container related deadlocks
    // void registerOneWaiting() {
    //         myWaitingForTransportable++;
    //     }
    #[allow(unused_variables)]
    pub fn register_one_waiting(&mut self) {
        todo!("port MSVehicleControl::registerOneWaiting")
    }

    /// C++: `void MSVehicleControl::unregisterOneWaiting()`
    /// decreases the count of vehicles waiting for a transport to allow recognition of person / container related deadlocks
    // void unregisterOneWaiting() {
    //         myWaitingForTransportable--;
    //     }
    #[allow(unused_variables)]
    pub fn unregister_one_waiting(&mut self) {
        todo!("port MSVehicleControl::unregisterOneWaiting")
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
        todo!("port MSVehicleControl::countCollision")
    }

    /// C++: `void MSVehicleControl::registerTeleportJam()`
    /// register one non-collision-related teleport
    // void registerTeleportJam() {
    //         myTeleportsJam++;
    //     }
    #[allow(unused_variables)]
    pub fn register_teleport_jam(&mut self) {
        todo!("port MSVehicleControl::registerTeleportJam")
    }

    /// C++: `void MSVehicleControl::registerTeleportYield()`
    /// register one non-collision-related teleport
    // void registerTeleportYield() {
    //         myTeleportsYield++;
    //     }
    #[allow(unused_variables)]
    pub fn register_teleport_yield(&mut self) {
        todo!("port MSVehicleControl::registerTeleportYield")
    }

    /// C++: `void MSVehicleControl::registerTeleportWrongLane()`
    /// register one non-collision-related teleport
    // void registerTeleportWrongLane() {
    //         myTeleportsWrongLane++;
    //     }
    #[allow(unused_variables)]
    pub fn register_teleport_wrong_lane(&mut self) {
        todo!("port MSVehicleControl::registerTeleportWrongLane")
    }

    /// C++: `void MSVehicleControl::registerEmergencyStop()`
    /// register emergency stop
    // void registerEmergencyStop() {
    //         myEmergencyStops++;
    //     }
    #[allow(unused_variables)]
    pub fn register_emergency_stop(&mut self) {
        todo!("port MSVehicleControl::registerEmergencyStop")
    }

    /// C++: `void MSVehicleControl::registerEmergencyBraking()`
    /// register emergency stop
    // void registerEmergencyBraking() {
    //         myEmergencyBrakingCount++;
    //     }
    #[allow(unused_variables)]
    pub fn register_emergency_braking(&mut self) {
        todo!("port MSVehicleControl::registerEmergencyBraking")
    }

    /// C++: `void MSVehicleControl::registerStopStarted()`
    /// register emergency stop
    // void registerStopStarted() {
    //         myStoppedVehicles++;
    //     }
    #[allow(unused_variables)]
    pub fn register_stop_started(&mut self) {
        todo!("port MSVehicleControl::registerStopStarted")
    }

    /// C++: `void MSVehicleControl::registerStopEnded()`
    /// register emergency stop
    // void registerStopEnded() {
    //         myStoppedVehicles--;
    //     }
    #[allow(unused_variables)]
    pub fn register_stop_ended(&mut self) {
        todo!("port MSVehicleControl::registerStopEnded")
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
        todo!("port MSVehicleControl::setState")
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
        todo!("port MSVehicleControl::saveState")
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
        todo!("port MSVehicleControl::clearState")
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
        todo!("port MSVehicleControl::discountStateRemoved")
    }

    /// C++: `void MSVehicleControl::discountRoutingVehicle()`
    // void discountRoutingVehicle() {
    //         myLoadedVehNo--;
    //         myEndedVehNo--;
    //         myDiscarded--;
    //     }
    #[allow(unused_variables)]
    pub fn discount_routing_vehicle(&mut self) {
        todo!("port MSVehicleControl::discountRoutingVehicle")
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
        todo!("port MSVehicleControl::abortWaiting")
    }

    /// C++: `double MSVehicleControl::getMaxSpeedFactor() const` [const]
    /// return the maximum speed factor for all vehicles that ever entered the network
    // double getMaxSpeedFactor() const {
    //         return myMaxSpeedFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_speed_factor(&self) -> f64 {
        todo!("port MSVehicleControl::getMaxSpeedFactor")
    }

    /// C++: `double MSVehicleControl::getMinDeceleration() const` [const]
    /// return the minimum deceleration capability for all road vehicles that ever entered the network
    // double getMinDeceleration() const {
    //         return myMinDeceleration;
    //     }
    #[allow(unused_variables)]
    pub fn get_min_deceleration(&self) -> f64 {
        todo!("port MSVehicleControl::getMinDeceleration")
    }

    /// C++: `double MSVehicleControl::getMinDecelerationRail() const` [const]
    /// return the minimum deceleration capability for all ral vehicles that ever entered the network
    // double getMinDecelerationRail() const {
    //         return myMinDecelerationRail;
    //     }
    #[allow(unused_variables)]
    pub fn get_min_deceleration_rail(&self) -> f64 {
        todo!("port MSVehicleControl::getMinDecelerationRail")
    }

    /// C++: `double MSVehicleControl::getMaxMinGap() const` [const]
    /// return the maximum minGap for all vehicles that ever entered the network
    // double getMaxMinGap() const {
    //         return myMaxMinGap;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_min_gap(&self) -> f64 {
        todo!("port MSVehicleControl::getMaxMinGap")
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
        todo!("port MSVehicleControl::adaptIntermodalRouter")
    }

    /// C++: `void MSVehicleControl::setScale(double)`
    /// sets the demand scaling factor
    // void setScale(double scale) {
    //         myScale = scale;
    //     }
    #[allow(unused_variables)]
    pub fn set_scale(&mut self, scale: f64) {
        todo!("port MSVehicleControl::setScale")
    }

    /// C++: `double MSVehicleControl::getScale() const` [const]
    /// sets the demand scaling factor
    // double getScale() const {
    //         return myScale;
    //     }
    #[allow(unused_variables)]
    pub fn get_scale(&self) -> f64 {
        todo!("port MSVehicleControl::getScale")
    }

    /// C++: `void MSVehicleControl::secureVehicles()` [virtual]
    /// lock access to vehicle removal/additions for thread synchronization
    // virtual void secureVehicles() {}
    #[allow(unused_variables)]
    pub fn secure_vehicles(&mut self) {
        todo!("port MSVehicleControl::secureVehicles")
    }

    /// C++: `void MSVehicleControl::releaseVehicles()` [virtual]
    /// unlock access to vehicle removal/additions for thread synchronization
    // virtual void releaseVehicles() {}
    #[allow(unused_variables)]
    pub fn release_vehicles(&mut self) {
        todo!("port MSVehicleControl::releaseVehicles")
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
        todo!("port MSVehicleControl::initDefaultTypes")
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
        todo!("port MSVehicleControl::checkVType")
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
        todo!("port MSVehicleControl::isPendingRemoval")
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
        todo!("port MSVehicleControl::initVehicle")
    }

    /// C++: `void MSVehicleControl::MSVehicleControl(const MSVehicleControl &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &MSVehicleControl) -> Self {
        todo!("port MSVehicleControl::MSVehicleControl")
    }

    /// C++: `MSVehicleControl & MSVehicleControl::operator=(const MSVehicleControl &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSVehicleControl) -> &mut MSVehicleControl {
        todo!("port MSVehicleControl::operator=")
    }

}

