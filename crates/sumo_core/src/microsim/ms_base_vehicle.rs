//! Port of microsim/MSBaseVehicle.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::geom::POSITION_EPS;
use crate::microsim::ms_globals::MSGlobals;

/// C++ enum `MSBaseVehicle::RouteValidity`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSBaseVehicle_RouteValidity {
    ROUTE_VALID = 0,
    ROUTE_UNCHECKED = 1,
    ROUTE_INVALID = 2,
    ROUTE_START_INVALID_PERMISSIONS = 4,
    ROUTE_START_INVALID_LANE = 8,
}

/// C++ typedef `std::vector<std::pair<MSMoveReminder *, double>> MSBaseVehicle::MoveReminderCont`
pub type MSBaseVehicle_MoveReminderCont = Vec<(Ptr<MSMoveReminder>, f64)>;

/// Changes the wished vehicle speed / lanes
/// C++ class `MSBaseVehicle::BaseInfluencer` (microsim/MSBaseVehicle.h)
pub struct MSBaseVehicle_BaseInfluencer {
    /// dynamic impatience offset
    pub myExtraImpatience: f64,
}

impl MSBaseVehicle_BaseInfluencer {
    /// C++: `void BaseInfluencer::BaseInfluencer()`
    /// Constructor
    // MSBaseVehicle::BaseInfluencer::BaseInfluencer()
    // {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self { myExtraImpatience: 0.0 }
    }

    /// C++: `void BaseInfluencer::~BaseInfluencer() noexcept` [virtual]
    /// Destructor
    // virtual ~BaseInfluencer() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void BaseInfluencer::init()` [static]
    /// Static initalization
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn init() {
        // C++ declares init()/cleanup() but never defines them; nothing to do
    }

    /// C++: `void BaseInfluencer::cleanup()` [static]
    /// Static cleanup
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn cleanup() {
        // C++ declares init()/cleanup() but never defines them; nothing to do
    }

    /// C++: `double BaseInfluencer::getExtraImpatience() const` [const]
    /// return the current routing mode
    // double getExtraImpatience() const {
    //             return myExtraImpatience;
    //         }
    #[allow(unused_variables)]
    pub fn get_extra_impatience(&self) -> f64 {
        self.myExtraImpatience
    }

    /// C++: `void BaseInfluencer::setExtraImpatience(double)`
    /// Sets routing behavior
    // void setExtraImpatience(double value) {
    //             myExtraImpatience = value;
    //         }
    #[allow(unused_variables)]
    pub fn set_extra_impatience(&mut self, value: f64) {
        self.myExtraImpatience = value;
    }

}

/// The base class for microscopic and mesoscopic vehicles
/// C++ class `MSBaseVehicle` (microsim/MSBaseVehicle.h)
#[repr(C)]
pub struct MSBaseVehicle {
    /// C++ base class `SUMOVehicle`
    pub base: SUMOVehicle,
    /// This vehicle's parameter.
    pub myParameter: Ptr<SUMOVehicleParameter>,
    /// This vehicle's route.
    pub myRoute: ConstMSRoutePtr,
    /// This vehicle's type.
    pub myType: Ptr<MSVehicleType>,
    /// Iterator to current route-edge
    /// shortcut: MSRouteIterator is Unmapped (ms_route.rs), so the iterator is an index into myRoute's edges
    pub myCurrEdge: usize,
    /// A precomputed factor by which the driver wants to be faster than the speed limit
    pub myChosenSpeedFactor: f64,
    /// The vehicle's list of stops
    pub myStops: VecDeque<MSStop>,
    /// The list of stops that the vehicle has already reached
    pub myPastStops: StopParVector,
    /// Currently relevant move reminders
    pub myMoveReminders: MSBaseVehicle_MoveReminderCont,
    /// The devices this vehicle has
    pub myDevices: Vec<Ptr<MSVehicleDevice>>,
    /// The passengers this vehicle may have
    pub myPersonDevice: Ptr<MSDevice_Transportable>,
    /// The containers this vehicle may have
    pub myContainerDevice: Ptr<MSDevice_Transportable>,
    /// The emission parameters this vehicle may have
    pub myEnergyParams: Ptr<EnergyParams>,
    /// The real departure time
    pub myDeparture: SUMOTime,
    /// The real depart position
    pub myDepartPos: f64,
    /// The position on the destination lane where the vehicle stops
    pub myArrivalPos: f64,
    /// The destination lane where the vehicle stops
    pub myArrivalLane: i32,
    /// The number of reroutings
    pub myNumberReroutes: i32,
    /// The offset when adding route stops with 'until' on route replacement
    pub myStopUntilOffset: SUMOTime,
    /// A simple odometer to keep track of the length of the route already driven
    pub myOdometer: f64,
    /// status of the current vehicle route
    pub myRouteValidity: i32,
    /// memory for parking search
    pub myParkingMemory: Ptr<StoppingPlaceMemory>,
    pub myChargingMemory: Ptr<StoppingPlaceMemory>,
    pub myNumberParkingReroutes: i32,
    /// Whether this vehicle is registered as waiting for a person or container (for deadlock-recognition)
    pub myAmRegisteredAsWaiting: bool,
    pub myAmReversed: bool,
    /// routing mode (see TraCIConstants.h)
    pub myRoutingMode: i32,
    pub myNumericalID: SUMOTrafficObject_NumericalID,
    pub myRandomSeed: i64,
    pub myEdgeWeights: Ptr<MSEdgeWeightsStorage>,
}

impl std::ops::Deref for MSBaseVehicle {
    type Target = SUMOVehicle;
    fn deref(&self) -> &SUMOVehicle {
        &self.base
    }
}
impl std::ops::DerefMut for MSBaseVehicle {
    fn deref_mut(&mut self) -> &mut SUMOVehicle {
        &mut self.base
    }
}

// C++ static members of MSBaseVehicle (port as module-level statics, see AGENTS.md):
//   static const SUMOTime MSBaseVehicle::NOT_YET_DEPARTED;
//   static std::vector<MSTransportable *> MSBaseVehicle::myEmptyTransportableVector;
//   static NumericalID MSBaseVehicle::myCurrentNumericalIndex;
/// C++ `MSBaseVehicle::NOT_YET_DEPARTED`
pub const MS_BASE_VEHICLE_NOT_YET_DEPARTED: SUMOTime = SUMOTIME_MAX;
/// C++ `VEHPARS_FORCE_REROUTE` (utils/vehicle/SUMOVehicleParameter.h), not yet ported elsewhere
pub const VEHPARS_FORCE_REROUTE: i64 = 2 << 17;
/// C++ `MSBaseVehicle::myEmptyTransportableVector`
pub const MS_BASE_VEHICLE_EMPTY_TRANSPORTABLE_VECTOR: &Vec<Ptr<MSTransportable>> = &Vec::new();
/// C++ `VEHPARS_JUNCTIONMODEL_PARAMS_SET` (SUMOVehicleParameter.h)
pub const VEHPARS_JUNCTIONMODEL_PARAMS_SET: i64 = 2 << 28;
/// C++ `VEHPARS_CFMODEL_PARAMS_SET` (SUMOVehicleParameter.h)
pub const VEHPARS_CFMODEL_PARAMS_SET: i64 = 2 << 29;
/// C++ `VEHPARS_LINE_SET` (SUMOVehicleParameter.h), not yet ported elsewhere
pub const VEHPARS_LINE_SET: i64 = 2 << 14;
/// C++ `VEHPARS_SPEEDFACTOR_SET` (SUMOVehicleParameter.h), not yet ported elsewhere
pub const VEHPARS_SPEEDFACTOR_SET: i64 = 2 << 24;
/// C++ `VEHPARS_PARKING_BADGES_SET` (SUMOVehicleParameter.h), not yet ported elsewhere
pub const VEHPARS_PARKING_BADGES_SET: i64 = (2i64) << 30;
/// C++ `VEHPARS_INSERTION_CHECKS_SET` (SUMOVehicleParameter.h), not yet ported elsewhere
pub const VEHPARS_INSERTION_CHECKS_SET: i64 = (2i64) << 31;
/// C++ `STOP_INDEX_FIT` (SUMOVehicleParameter.h), not yet ported elsewhere
pub const STOP_INDEX_FIT: i32 = -2;
/// C++ `STOP_INDEX_END` (SUMOVehicleParameter.h)
const STOP_INDEX_END: i32 = -1;
/// C++ `STOP_INDEX_REPEAT` (SUMOVehicleParameter.h)
const STOP_INDEX_REPEAT: i32 = -3;
/// C++ `SPEED_EPS` (config.h.cmake), not yet ported elsewhere
pub const SPEED_EPS: f64 = 0.01;
thread_local! {
    /// C++ `MSBaseVehicle::myCurrentNumericalIndex`
    static MS_BASE_VEHICLE_CURRENT_NUMERICAL_INDEX: std::cell::Cell<SUMOTrafficObject_NumericalID> = const { std::cell::Cell::new(0) };
}
/// C++ `new StoppingPlaceMemory()` owned by the vehicle
// shortcut: the memory is leaked (never freed), because the vehicle destructor does not free it yet
fn new_stopping_place_memory() -> Ptr<StoppingPlaceMemory> {
    Ptr::from_mut(Box::leak(Box::new(StoppingPlaceMemory::new())))
}
/// C++ `RandHelper::getSeed()` (default RNG), not yet ported as a free function
fn rand_helper_get_seed() -> i32 {
    utils::common::rand_helper::with_default(|r| r.orig_seed)
}

impl MSBaseVehicle {
    // Downcast of this base to the concrete vehicle: micro sim is always MSVehicle, meso always MEVehicle.
    fn as_ms_vehicle(&self) -> &crate::microsim::ms_vehicle::MSVehicle {
        Ptr::new(self).cast::<crate::microsim::ms_vehicle::MSVehicle>().get().unwrap()
    }
    fn as_me_vehicle(&self) -> &crate::mesosim::me_vehicle::MEVehicle {
        Ptr::new(self).cast::<crate::mesosim::me_vehicle::MEVehicle>().get().unwrap()
    }

    /// C++: `double MSBaseVehicle::getPreviousSpeed() const` [const, virtual]
    /// Returns the vehicle's previous speed
    // double
    // MSBaseVehicle::getPreviousSpeed() const {
    //     throw ProcessError("getPreviousSpeed() is not available for non-MSVehicles.");
    // }
    #[allow(unused_variables)]
    pub fn get_previous_speed(&self) -> f64 {
        if MSGlobals::use_meso_sim() {
            panic!("getPreviousSpeed() is not available for non-MSVehicles.");
        }
        self.as_ms_vehicle().get_previous_speed()
    }

    /// C++: `void MSBaseVehicle::MSBaseVehicle(SUMOVehicleParameter *, ConstMSRoutePtr, MSVehicleType *, const double)`
    /// Constructor
    // MSBaseVehicle::MSBaseVehicle(SUMOVehicleParameter* pars, ConstMSRoutePtr route,
    //                              MSVehicleType* type, const double speedFactor) :
    //     SUMOVehicle(pars->id),
    //     myParameter(pars),
    //     myRoute(route),
    //     myType(type),
    //     myCurrEdge(route->begin()),
    //     myChosenSpeedFactor(pars->speedFactor < 0 ? speedFactor : pars->speedFactor),
    //     myMoveReminders(0),
    //     myPersonDevice(nullptr),
    //     myContainerDevice(nullptr),
    //     myEnergyParams(nullptr),
    //     myDeparture(NOT_YET_DEPARTED),
    //     myDepartPos(-1),
    //     myArrivalPos(-1),
    //     myArrivalLane(-1),
    //     myNumberReroutes(0),
    //     myStopUntilOffset(0),
    //     myOdometer(0.),
    //     myRouteValidity(ROUTE_UNCHECKED),
    //     myRoutingMode(libsumo::ROUTING_MODE_DEFAULT),
    //     myNumericalID(myCurrentNumericalIndex++),
    //     myRandomSeed(RandHelper::murmur3_32(pars->id, RandHelper::getSeed())),
    //     myEdgeWeights(nullptr)
    // #ifdef _DEBUG
    //     , myTraceMoveReminders(myShallTraceMoveReminders.count(pars->id) > 0)
    // #endif
    // {
    //     if ((*myRoute->begin())->isTazConnector() || myRoute->getLastEdge()->isTazConnector()) {
    //         pars->parametersSet |= VEHPARS_FORCE_REROUTE;
    //     }
    //     if ((pars->parametersSet & VEHPARS_FORCE_REROUTE) == 0) {
    //         setDepartAndArrivalEdge();
    //     }
    //     calculateArrivalParams(true);
    //     initTransientModelParams();
    // }
    #[allow(unused_variables)]
    pub fn new(pars: Ptr<SUMOVehicleParameter>, route: ConstMSRoutePtr, type_: Ptr<MSVehicleType>, speedFactor: f64) -> Self {
        let speed_factor = if pars.speedFactor < 0.0 { speedFactor } else { pars.speedFactor };
        let id = pars.id.clone();
        let first_edge = route.get_first_edge();
        let last_edge = route.get_last_edge();
        let mut this = MSBaseVehicle {
            base: SUMOVehicle::new(&id),
            myParameter: pars,
            myRoute: route.clone(),
            myType: type_,
            myCurrEdge: 0,
            myChosenSpeedFactor: speed_factor,
            myStops: VecDeque::new(),
            myPastStops: StopParVector::new(),
            myMoveReminders: MSBaseVehicle_MoveReminderCont::new(),
            myDevices: Vec::new(),
            myPersonDevice: Ptr::default(),
            myContainerDevice: Ptr::default(),
            myEnergyParams: Ptr::default(),
            myDeparture: MS_BASE_VEHICLE_NOT_YET_DEPARTED,
            myDepartPos: -1.0,
            myArrivalPos: -1.0,
            myArrivalLane: -1,
            myNumberReroutes: 0,
            myStopUntilOffset: 0,
            myOdometer: 0.0,
            myRouteValidity: MSBaseVehicle_RouteValidity::ROUTE_UNCHECKED as i32,
            myParkingMemory: Ptr::default(),
            myChargingMemory: Ptr::default(),
            myNumberParkingReroutes: 0,
            myAmRegisteredAsWaiting: false,
            myAmReversed: false,
            myRoutingMode: utils::traci::traci_constants::ROUTING_MODE_DEFAULT,
            myNumericalID: MS_BASE_VEHICLE_CURRENT_NUMERICAL_INDEX.with(|c| {
                let v = c.get();
                c.set(v + 1);
                v
            }),
            myRandomSeed: utils::common::rand_helper::murmur3_32(&id, rand_helper_get_seed()) as i64,
            myEdgeWeights: Ptr::default(),
        };
        // shortcut: _DEBUG-only move reminder tracing (myTraceMoveReminders) not ported
        if first_edge.is_taz_connector() || last_edge.is_taz_connector() {
            this.myParameter.m().parametersSet |= VEHPARS_FORCE_REROUTE;
        }
        if (this.myParameter.parametersSet & VEHPARS_FORCE_REROUTE) == 0 {
            this.set_depart_and_arrival_edge();
        }
        this.calculate_arrival_params(true);
        this.init_transient_model_params();
        this
    }

    /// C++: `void MSBaseVehicle::~MSBaseVehicle() noexcept` [virtual]
    /// Destructor
    // MSBaseVehicle::~MSBaseVehicle() {
    //     delete myEdgeWeights;
    //     if (myParameter->repetitionNumber == -1) {
    //         // this is not a flow (flows call checkDist in MSInsertionControl::determineCandidates)
    //         MSRoute::checkDist(myParameter->routeid);
    //     }
    //     for (MSVehicleDevice* dev : myDevices) {
    //         delete dev;
    //     }
    //     delete myEnergyParams;
    //     delete myParkingMemory;
    //     delete myChargingMemory;
    //     checkRouteRemoval();
    //     delete myParameter;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ frees owned devices/params; Rust ownership handles that.
        if self.myParameter.repetitionNumber == -1 {
            // this is not a flow (flows call checkDist in MSInsertionControl::determineCandidates)
            MSRoute::check_dist(&self.myParameter.routeid);
        }
        self.check_route_removal();
    }

    /// C++: `void MSBaseVehicle::initDevices()` [virtual]
    // void
    // MSBaseVehicle::initDevices() {
    //     MSDevice::buildVehicleDevices(*this, myDevices);
    //     for (MSVehicleDevice* dev : myDevices) {
    //         myMoveReminders.push_back(std::make_pair(dev, 0.));
    //     }
    //     if (MSGlobals::gHaveEmissions) {
    //         // ensure we have the emission parameters even if we don't have the device
    //         getEmissionParameters();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_devices(&mut self) {
        // ported: build vehicle devices and register them as move reminders
        let self_ptr = self as *mut MSBaseVehicle;
        let v: &mut SUMOVehicle = unsafe { &mut *(self_ptr as *mut SUMOVehicle) };
        let mut devices: Vec<Ptr<MSVehicleDevice>> = Vec::new();
        MSDevice::build_vehicle_devices(v, &mut devices);
        if !MSDevice::get_device_by_name(&devices, "rerouting").is_null() {
            // C++ records this in createDevice; the rerouter uses it to tell a vehicle with a routing device
            // from one without (see has_rerouting_device).
            self.myParameter.m().set_parameter("has.rerouting.device", "true");
        }
        self.myMoveReminders = devices.iter().map(|dev| (dev.cast::<MSMoveReminder>(), 0.0)).collect();
        self.myDevices = devices;
        if MSGlobals::have_emissions() {
            // ensure we have the emission parameters even if we don't have the device
            self.get_emission_parameters();
        }
    }

    /// C++: `bool MSBaseVehicle::isVehicle() const` [const, virtual]
    // bool isVehicle() const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn is_vehicle(&self) -> bool {
        true
    }

    /// C++: `void MSBaseVehicle::setID(const std::string &)` [virtual]
    /// set the id (inherited from Named but forbidden for vehicles)
    // void
    // MSBaseVehicle::setID(const std::string& /*newID*/) {
    //     throw ProcessError(TL("Changing a vehicle ID is not permitted"));
    // }
    #[allow(unused_variables)]
    pub fn set_id(&mut self, newID: &str) {
        panic!("Changing a vehicle ID is not permitted");
    }

    /// C++: `const SUMOVehicleParameter & MSBaseVehicle::getParameter() const` [const, virtual]
    /// Returns the vehicle's parameter (including departure definition)
    // const SUMOVehicleParameter&
    // MSBaseVehicle::getParameter() const {
    //     return *myParameter;
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self) -> &SUMOVehicleParameter {
        self.myParameter.get().unwrap()
    }

    /// C++: `std::string MSBaseVehicle::getPrefixedParameter(const std::string &, std::string &) const` [const]
    /// retrieve parameters of devices, models and the vehicle itself
    // std::string
    // MSBaseVehicle::getPrefixedParameter(const std::string& key, std::string& error) const {
    //     const MSVehicle* microVeh = dynamic_cast<const MSVehicle*>(this);
    //     if (StringUtils::startsWith(key, "device.")) {
    //         StringTokenizer tok(key, ".");
    //         if (tok.size() < 3) {
    //             error = TLF("Invalid device parameter '%' for vehicle '%'.", key, getID());
    //             return "";
    //         }
    //         try {
    //             return getDeviceParameter(tok.get(1), key.substr(tok.get(0).size() + tok.get(1).size() + 2));
    //         } catch (InvalidArgument& e) {
    //             error = TLF("Vehicle '%' does not support device parameter '%' (%).", getID(), key, e.what());
    //             return "";
    //         }
    //     } else if (StringUtils::startsWith(key, "laneChangeModel.")) {
    //         if (microVeh == nullptr) {
    //             error = TLF("Mesoscopic vehicle '%' does not support laneChangeModel parameters.", getID());
    //             return "";
    //         }
    //         const std::string attrName = key.substr(16);
    //         try {
    //             return microVeh->getLaneChangeModel().getParameter(attrName);
    //         } catch (InvalidArgument& e) {
    //             error = TLF("Vehicle '%' does not support laneChangeModel parameter '%' (%).", getID(), key, e.what());
    //             return "";
    //         }
    //     } else if (StringUtils::startsWith(key, "carFollowModel.")) {
    //         if (microVeh == nullptr) {
    //             error = TLF("Mesoscopic vehicle '%' does not support carFollowModel parameters.", getID());
    //             return "";
    //         }
    //         const std::string attrName = key.substr(15);
    //         try {
    //             return microVeh->getCarFollowModel().getParameter(microVeh, attrName);
    //         } catch (InvalidArgument& e) {
    //             error = TLF("Vehicle '%' does not support carFollowModel parameter '%' (%).", getID(), key, e.what());
    //             return "";
    //         }
    //     } else if (StringUtils::startsWith(key, "has.") && StringUtils::endsWith(key, ".device")) {
    //         StringTokenizer tok(key, ".");
    //         if (tok.size() != 3) {
    //             error = TL("Invalid check for device. Expected format is 'has.DEVICENAME.device'.");
    //             return "";
    //         }
    //         return hasDevice(tok.get(1)) ? "true" : "false";
    //         // parking related parameters start here
    //     } else if (key == "parking.rerouteCount") {
    //         return toString(getNumberParkingReroutes());
    //     } else if (StringUtils::startsWith(key, "parking.memory.")) {
    //         std::vector<std::string> values;
    //         if (getParkingMemory()) {
    //             if (key == "parking.memory.IDList") {
    //                 for (const auto& item : *getParkingMemory()) {
    //                     values.push_back(item.first->getID());
    //                 }
    //             } else if (key == "parking.memory.score") {
    //                 for (const auto& item : *getParkingMemory()) {
    //                     values.push_back(item.second.score);
    //                 }
    //             } else if (key == "parking.memory.blockedAtTime") {
    //                 for (const auto& item : *getParkingMemory()) {
    //                     values.push_back(toString(STEPS2TIME(item.second.blockedAtTime)));
    //                 }
    //             } else if (key == "parking.memory.blockedAtTimeLocal") {
    //                 for (const auto& item : *getParkingMemory()) {
    //                     values.push_back(toString(STEPS2TIME(item.second.blockedAtTimeLocal)));
    //                 }
    //             } else {
    //                 error = TLF("Unsupported parking parameter '%' for vehicle '%'.", key, getID());
    //             }
    //         }
    //         return toString(values);
    //     } else {
    //         // default: custom user parameter
    //         return getParameter().getParameter(key, "");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_prefixed_parameter(&self, key: &str, error: &mut String) -> String {
        let micro_veh: Option<Ptr<MSVehicle>> = self.as_micro_vehicle();
        if utils::common::string_utils::starts_with(key, "device.") {
            let tok = utils::common::string_tokenizer::StringTokenizer::with_token(key, ".", false).get_vector();
            if tok.len() < 3 {
                *error = utils::tlf!("Invalid device parameter '%' for vehicle '%'.", key, self.get_id());
                return String::new();
            }
            match self.try_get_device_parameter(&tok[1], &key[tok[0].len() + tok[1].len() + 2..]) {
                Ok(v) => return v,
                Err(e) => {
                    *error = utils::tlf!("Vehicle '%' does not support device parameter '%' (%).", self.get_id(), key, e);
                    return String::new();
                }
            }
        } else if utils::common::string_utils::starts_with(key, "laneChangeModel.") {
            let Some(microVeh) = micro_veh else {
                *error = utils::tlf!("Mesoscopic vehicle '%' does not support laneChangeModel parameters.", self.get_id());
                return String::new();
            };
            let attrName = &key[16..];
            return microVeh.m().get_lane_change_model().get_parameter(attrName);
        } else if utils::common::string_utils::starts_with(key, "carFollowModel.") {
            let Some(microVeh) = micro_veh else {
                *error = utils::tlf!("Mesoscopic vehicle '%' does not support carFollowModel parameters.", self.get_id());
                return String::new();
            };
            let attrName = &key[15..];
            return microVeh.get_car_follow_model().get_parameter(microVeh, attrName);
        } else if utils::common::string_utils::starts_with(key, "has.") && utils::common::string_utils::ends_with(key, ".device") {
            let tok = utils::common::string_tokenizer::StringTokenizer::with_token(key, ".", false).get_vector();
            if tok.len() != 3 {
                *error = "Invalid check for device. Expected format is 'has.DEVICENAME.device'.".to_string();
                return String::new();
            }
            return if self.has_device(&tok[1]) { "true".to_string() } else { "false".to_string() };
            // parking related parameters start here
        } else if key == "parking.rerouteCount" {
            return self.get_number_parking_reroutes().to_string();
        } else if utils::common::string_utils::starts_with(key, "parking.memory.") {
            let mut values: Vec<String> = Vec::new();
            if let Some(memory) = self.get_parking_memory().get() {
                if key == "parking.memory.IDList" {
                    for (item, _) in memory.myMap.iter() {
                        values.push(item.get_id().to_string());
                    }
                } else if key == "parking.memory.score" {
                    for (_, item) in memory.myMap.iter() {
                        values.push(item.score.clone());
                    }
                } else if key == "parking.memory.blockedAtTime" {
                    for (_, item) in memory.myMap.iter() {
                        values.push(utils::common::to_string::to_string(&utils::common::time::steps2time(item.blockedAtTime)));
                    }
                } else if key == "parking.memory.blockedAtTimeLocal" {
                    for (_, item) in memory.myMap.iter() {
                        values.push(utils::common::to_string::to_string(&utils::common::time::steps2time(item.blockedAtTimeLocal)));
                    }
                } else {
                    *error = utils::tlf!("Unsupported parking parameter '%' for vehicle '%'.", key, self.get_id());
                }
            }
            return utils::common::to_string::join_to_string(values.iter(), " ");
        } else {
            // default: custom user parameter
            return self.get_parameter().get_parameter(key, "");
        }
    }

    /// C++: `void MSBaseVehicle::replaceParameter(const SUMOVehicleParameter *)` [virtual]
    /// replace the vehicle parameter (deleting the old one)
    // void
    // MSBaseVehicle::replaceParameter(const SUMOVehicleParameter* newParameter) {
    //     delete myParameter;
    //     myParameter = newParameter;
    // }
    #[allow(unused_variables)]
    pub fn replace_parameter(&mut self, newParameter: Ptr<SUMOVehicleParameter>) {
        self.myParameter = newParameter;
    }

    /// Fallible form of `getDeviceParameter` (C++ throws InvalidArgument); the Err carries `what()`
    fn try_get_device_parameter(&self, deviceName: &str, key: &str) -> Result<String, String> {
        for dev in self.myDevices.iter() {
            if dev.base2.device_name() == deviceName {
                return Ok(MSDevice::get_parameter_of(*dev, key));
            }
        }
        Err(utils::tlf!("no device of type '%' exists", deviceName))
    }

    /// C++: `bool MSBaseVehicle::hasDevice(const std::string &) const` [const]
    /// check whether the vehicle is equipped with a device of the given name
    // bool
    // MSBaseVehicle::hasDevice(const std::string& deviceName) const {
    //     for (MSDevice* const dev : myDevices) {
    //         if (dev->deviceName() == deviceName) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    /// C++: `tObject.getDevice(typeid(MSDevice_Routing)) != nullptr`.
    ///
    /// The port has no type-keyed device lookup and `MSDevice::device_name` is pure virtual with no vtable,
    /// so ask the parameter that `createDevice` records for exactly this purpose (`has.rerouting.device`).
    pub fn has_rerouting_device(&self) -> bool {
        self.myParameter.get_parameter("has.rerouting.device", "false") == "true"
    }

    #[allow(unused_variables)]
    pub fn has_device(&self, deviceName: &str) -> bool {
        for dev in self.myDevices.iter() {
            if dev.base2.device_name() == deviceName {
                return true;
            }
        }
        false
    }

    /// C++: `void MSBaseVehicle::createDevice(const std::string &)`
    /// create device of the given type
    // void
    // MSBaseVehicle::createDevice(const std::string& deviceName) {
    //     if (!hasDevice(deviceName)) {
    //         if (deviceName == "rerouting") {
    //             ((SUMOVehicleParameter*)myParameter)->setParameter("has." + deviceName + ".device", "true");
    //             MSDevice_Routing::buildVehicleDevices(*this, myDevices);
    //             if (hasDeparted()) {
    //                 // vehicle already departed: disable pre-insertion rerouting and enable regular routing behavior
    //                 MSDevice_Routing* routingDevice = static_cast<MSDevice_Routing*>(getDevice(typeid(MSDevice_Routing)));
    //                 assert(routingDevice != 0);
    //                 routingDevice->notifyEnter(*this, MSMoveReminder::NOTIFICATION_DEPARTED);
    //             }
    //         } else {
    //             throw InvalidArgument(TLF("creating device of type '%' is not supported", deviceName));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn create_device(&mut self, deviceName: &str) {
        if !self.has_device(deviceName) {
            if deviceName == "rerouting" {
                self.myParameter.m().set_parameter(&format!("has.{}.device", deviceName), "true");
                // shortcut: MSDevice_Routing::buildVehicleDevices(*this, myDevices) not wired to the device vector yet
                if self.has_departed() {
                    // vehicle already departed: disable pre-insertion rerouting and enable regular routing behavior
                    // shortcut: getDevice(typeid(MSDevice_Routing)) + notifyEnter(*this, NOTIFICATION_DEPARTED) not wired yet
                }
            } else {
                panic!("{}", utils::tlf!("creating device of type '%' is not supported", deviceName));
            }
        }
    }

    /// C++: `std::string MSBaseVehicle::getDeviceParameter(const std::string &, const std::string &) const` [const]
    /// try to retrieve the given parameter from any of the vehicles devices, raise InvalidArgument if no device parameter by that name exists
    // std::string
    // MSBaseVehicle::getDeviceParameter(const std::string& deviceName, const std::string& key) const {
    //     for (MSVehicleDevice* const dev : myDevices) {
    //         if (dev->deviceName() == deviceName) {
    //             return dev->getParameter(key);
    //         }
    //     }
    //     throw InvalidArgument(TLF("no device of type '%' exists", deviceName));
    // }
    #[allow(unused_variables)]
    pub fn get_device_parameter(&self, deviceName: &str, key: &str) -> String {
        self.try_get_device_parameter(deviceName, key).unwrap_or_else(|e| panic!("{}", e))
    }

    /// C++: `void MSBaseVehicle::setDeviceParameter(const std::string &, const std::string &, const std::string &)`
    /// try to set the given parameter from any of the vehicles devices, raise InvalidArgument if no device parameter by that name exists
    // void
    // MSBaseVehicle::setDeviceParameter(const std::string& deviceName, const std::string& key, const std::string& value) {
    //     for (MSVehicleDevice* const dev : myDevices) {
    //         if (dev->deviceName() == deviceName) {
    //             dev->setParameter(key, value);
    //             return;
    //         }
    //     }
    //     throw InvalidArgument(TLF("no device of type '%' exists", deviceName));
    // }
    #[allow(unused_variables)]
    pub fn set_device_parameter(&mut self, deviceName: &str, key: &str, value: &str) {
        for dev in self.myDevices.iter_mut() {
            if dev.base2.device_name() == deviceName {
                MSDevice::set_parameter_of(*dev, key, value);
                return;
            }
        }
        panic!("{}", utils::tlf!("no device of type '%' exists", deviceName));
    }

    /// C++: `void MSBaseVehicle::setJunctionModelParameter(const std::string &, const std::string &)`
    /// set individual junction model parameter (not type related)
    // void
    // MSBaseVehicle::setJunctionModelParameter(const std::string& key, const std::string& value) {
    //     if (key == toString(SUMO_ATTR_JM_IGNORE_IDS) || key == toString(SUMO_ATTR_JM_IGNORE_TYPES)) {
    //         getParameter().parametersSet |= VEHPARS_JUNCTIONMODEL_PARAMS_SET;
    //         const_cast<SUMOVehicleParameter&>(getParameter()).set_parameter(key, value);
    //         // checked in MSLink::ignoreFoe
    //     } else {
    //         throw InvalidArgument(TLF("Vehicle '%' does not support junctionModel parameter '%'.", getID(), key));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_junction_model_parameter(&mut self, key: &str, value: &str) {
        // toString(SUMO_ATTR_JM_IGNORE_IDS / SUMO_ATTR_JM_IGNORE_TYPES)
        if key == "junctionModel.ignoreIDs" || key == "junctionModel.ignoreTypes" {
            self.myParameter.m().parametersSet |= VEHPARS_JUNCTIONMODEL_PARAMS_SET;
            self.myParameter.m().set_parameter(key, value);
            // checked in MSLink::ignoreFoe
        } else {
            panic!("{}", utils::tlf!("Vehicle '%' does not support junctionModel parameter '%'.", self.get_id(), key));
        }
    }

    /// C++: `void MSBaseVehicle::setCarFollowModelParameter(const std::string &, const std::string &)`
    /// set individual carFollow model parameters (not type related)
    // void
    // MSBaseVehicle::setCarFollowModelParameter(const std::string& key, const std::string& value) {
    //     // handle some generic params first and then delegate to the carFollowModel itself
    //     if (key == toString(SUMO_ATTR_CF_IGNORE_IDS) || key == toString(SUMO_ATTR_CF_IGNORE_TYPES)) {
    //         getParameter().parametersSet |= VEHPARS_CFMODEL_PARAMS_SET;
    //         const_cast<SUMOVehicleParameter&>(getParameter()).set_parameter(key, value);
    //         // checked in MSVehicle::planMove
    //     } else {
    //         MSVehicle* microVeh = dynamic_cast<MSVehicle*>(this);
    //         if (microVeh) {
    //             // remove 'carFollowModel.' prefix
    //             const std::string attrName = key.substr(15);
    //             microVeh->getCarFollowModel().set_parameter(microVeh, attrName, value);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_car_follow_model_parameter(&mut self, key: &str, value: &str) -> Result<(), InvalidArgument> {
        // handle some generic params first and then delegate to the carFollowModel itself
        // toString(SUMO_ATTR_CF_IGNORE_IDS / SUMO_ATTR_CF_IGNORE_TYPES)
        if key == "carFollowModel.ignoreIDs" || key == "carFollowModel.ignoreTypes" {
            self.myParameter.m().parametersSet |= VEHPARS_CFMODEL_PARAMS_SET;
            self.myParameter.m().set_parameter(key, value);
            // checked in MSVehicle::planMove
        } else if let Some(microVeh) = self.as_micro_vehicle() {
            // remove 'carFollowModel.' prefix
            let attrName = &key[15..];
            return microVeh.get_car_follow_model().set_parameter(microVeh, attrName, value);
        }
        Ok(())
    }

    /// C++ `dynamic_cast<MSVehicle*>(this)`: a checked downcast. MSVehicle is the only class that installs
    /// MS_VEHICLE_VT in its SUMOTrafficObject base, and MSBaseVehicle is its first (repr(C)) field.
    pub(crate) fn as_micro_vehicle(&self) -> Option<Ptr<MSVehicle>> {
        if std::ptr::eq(self.base.base.vt, &crate::microsim::ms_vehicle::MS_VEHICLE_VT) {
            Some(Ptr::from_raw(self as *const MSBaseVehicle as *mut MSVehicle))
        } else {
            None
        }
    }

    /// C++: `const MSRoute & MSBaseVehicle::getRoute() const` [const, virtual]
    /// Returns the current route
    // inline const MSRoute& getRoute() const {
    //         return *myRoute;
    //     }
    #[allow(unused_variables)]
    pub fn get_route(&self) -> &MSRoute {
        self.myRoute.as_ref()
    }

    /// C++: `ConstMSRoutePtr MSBaseVehicle::getRoutePtr() const` [const, virtual]
    /// Returns the current route
    // inline ConstMSRoutePtr getRoutePtr() const {
    //         return myRoute;
    //     }
    #[allow(unused_variables)]
    pub fn get_route_ptr(&self) -> ConstMSRoutePtr {
        self.myRoute.clone()
    }

    /// C++: `const MSVehicleType & MSBaseVehicle::getVehicleType() const` [const, virtual]
    /// Returns the vehicle's type definition
    // inline const MSVehicleType& getVehicleType() const {
    //         return *myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_type(&self) -> &MSVehicleType {
        self.myType.get().unwrap()
    }

    /// C++: `const SUMOVTypeParameter & MSBaseVehicle::getVTypeParameter() const` [const, virtual]
    /// Returns the vehicle's type parameter
    // inline const SUMOVTypeParameter& getVTypeParameter() const {
    //         return myType->getParameter();
    //     }
    #[allow(unused_variables)]
    pub fn get_v_type_parameter(&self) -> &SUMOVTypeParameter {
        self.myType.get_parameter()
    }

    /// C++: `SUMOVehicleClass MSBaseVehicle::getVClass() const` [const, virtual]
    /// Returns the vehicle's access class
    // inline SUMOVehicleClass getVClass() const {
    //         return myType->getParameter().vehicleClass;
    //     }
    #[allow(unused_variables)]
    pub fn get_v_class(&self) -> SUMOVehicleClass {
        self.myType.get_parameter().vehicleClass
    }

    /// C++: `bool MSBaseVehicle::ignoreTransientPermissions() const` [const, virtual]
    /// Returns whether this object is ignoring transient permission changes (during routing)
    // bool
    // MSBaseVehicle::ignoreTransientPermissions() const {
    //     return (getRoutingMode() & libsumo::ROUTING_MODE_IGNORE_TRANSIENT_PERMISSIONS) != 0;
    // }
    #[allow(unused_variables)]
    pub fn ignore_transient_permissions(&self) -> bool {
        (self.get_routing_mode() & utils::traci::traci_constants::ROUTING_MODE_IGNORE_TRANSIENT_PERMISSIONS) != 0
    }

    /// C++: `bool MSBaseVehicle::instantStopping() const` [const, virtual]
    /// whether instant stopping is permitted
    // virtual bool instantStopping() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn instant_stopping(&self) -> bool {
        false
    }

    /// C++: `double MSBaseVehicle::getMaxSpeed() const` [const, virtual]
    /// Returns the maximum speed (the minimum of desired and technical maximum speed)
    // double
    // MSBaseVehicle::getMaxSpeed() const {
    //     return MIN2(myType->getMaxSpeed(), myType->getDesiredMaxSpeed() * myChosenSpeedFactor);
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(&self) -> f64 {
        let a = self.myType.get_max_speed();
        let b = self.myType.get_desired_max_speed() * self.myChosenSpeedFactor;
        if a < b { a } else { b }
    }

    /// C++: `const MSEdge * MSBaseVehicle::succEdge(int) const` [const, virtual]
    /// Returns the nSuccs'th successor of edge the vehicle is currently at
    // const MSEdge*
    // MSBaseVehicle::succEdge(int nSuccs) const {
    //     if (myCurrEdge + nSuccs < myRoute->end() && std::distance(myCurrEdge, myRoute->begin()) <= nSuccs) {
    //         return *(myCurrEdge + nSuccs);
    //     } else {
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn succ_edge(&self, nSuccs: i32) -> Ptr<MSEdge> {
        // myCurrEdge + nSuccs < end() && distance(myCurrEdge, begin()) <= nSuccs (begin() is index 0)
        let cur = self.myCurrEdge as i64;
        let n = nSuccs as i64;
        if cur + n < self.myRoute.size() as i64 && -cur <= n {
            self.myRoute.index_op((cur + n) as i32)
        } else {
            Ptr::default()
        }
    }

    /// C++: `const MSEdge * MSBaseVehicle::getEdge() const` [const, virtual]
    /// Returns the edge the vehicle is currently at
    // const MSEdge*
    // MSBaseVehicle::getEdge() const {
    //     return *myCurrEdge;
    // }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> Ptr<MSEdge> {
        self.myRoute.index_op(self.myCurrEdge as i32)
    }

    /// C++: `const MSEdge * MSBaseVehicle::getCurrentEdge() const` [const, virtual]
    /// Returns the edge the vehicle is currently at (possibly an internal edge)
    // virtual const MSEdge* getCurrentEdge() const {
    //         return getEdge();
    //     }
    #[allow(unused_variables)]
    pub fn get_current_edge(&self) -> Ptr<MSEdge> {
        if MSGlobals::use_meso_sim() {
            return self.as_me_vehicle().get_current_edge();
        }
        self.as_ms_vehicle().get_current_edge()
    }

    /// C++: `const std::set<SUMOTrafficObject::NumericalID> MSBaseVehicle::getUpcomingEdgeIDs() const` [const, virtual]
    /// returns the numerical ids of edges to travel
    // const std::set<SUMOTrafficObject::NumericalID>
    // MSBaseVehicle::getUpcomingEdgeIDs() const {
    //     std::set<SUMOTrafficObject::NumericalID> result;
    //     for (auto e = myCurrEdge; e != myRoute->end(); ++e) {
    //         result.insert((*e)->getNumericalID());
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_upcoming_edge_i_ds(&self) -> BTreeSet<SUMOTrafficObject_NumericalID> {
        let mut result = BTreeSet::new();
        for e in (self.myCurrEdge as i32)..self.myRoute.size() {
            result.insert(self.myRoute.index_op(e).get_numerical_id() as SUMOTrafficObject_NumericalID);
        }
        result
    }

    /// C++: `bool MSBaseVehicle::stopsAt(MSStoppingPlace *) const` [const, virtual]
    /// Returns whether the vehicle stops at the given stopping place
    // bool
    // MSBaseVehicle::stopsAt(MSStoppingPlace* stop) const {
    //     if (stop == nullptr) {
    //         return false;
    //     }
    //     for (const MSStop& s : myStops) {
    //         if (s.busstop == stop
    //                 || s.containerstop == stop
    //                 || s.parkingarea == stop
    //                 || s.chargingStation == stop) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn stops_at(&self, stop: Ptr<MSStoppingPlace>) -> bool {
        if stop.is_null() {
            return false;
        }
        for s in self.myStops.iter() {
            if s.busstop == stop || s.containerstop == stop || s.parkingarea.get().map(|p| &p.base as *const MSStoppingPlace as *mut MSStoppingPlace) == Some(stop.as_raw()) || s.chargingStation == stop {
                return true;
            }
        }
        false
    }

    /// C++: `bool MSBaseVehicle::stopsAtEdge(const MSEdge *) const` [const, virtual]
    /// Returns whether the vehicle stops at the given edge
    // bool
    // MSBaseVehicle::stopsAtEdge(const MSEdge* edge) const {
    //     for (const MSStop& s : myStops) {
    //         if (&s.lane->getEdge() == edge) {
    //             return true;
    //         }
    //     }
    //     return myRoute->getLastEdge() == edge;
    // }
    #[allow(unused_variables)]
    pub fn stops_at_edge(&self, edge: Ptr<MSEdge>) -> bool {
        for s in self.myStops.iter() {
            if Ptr::new(s.lane.get_edge()) == edge {
                return true;
            }
        }
        self.myRoute.get_last_edge() == edge
    }

    /// C++: `const MSEdge * MSBaseVehicle::getNextEdgePtr() const` [const, virtual]
    /// returns the next edge (possibly an internal edge)
    // virtual const MSEdge* getNextEdgePtr() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_next_edge_ptr(&self) -> Ptr<MSEdge> {
        if MSGlobals::use_meso_sim() {
            return Ptr::default();
        }
        self.as_ms_vehicle().get_next_edge_ptr()
    }

    /// C++: `bool MSBaseVehicle::isOnRoad() const` [const, virtual]
    /// Returns the information whether the vehicle is on a road (is simulated)
    // virtual bool isOnRoad() const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn is_on_road(&self) -> bool {
        if MSGlobals::use_meso_sim() {
            return self.as_me_vehicle().is_on_road();
        }
        self.as_ms_vehicle().is_on_road()
    }

    /// C++: `bool MSBaseVehicle::isRemoteControlled() const` [const, virtual]
    /// Returns the information whether the vehicle is fully controlled via TraCI
    // virtual bool isRemoteControlled() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_remote_controlled(&self) -> bool {
        false
    }

    /// C++: `bool MSBaseVehicle::wasRemoteControlled(SUMOTime) const` [const, virtual]
    // virtual bool wasRemoteControlled(SUMOTime lookBack = DELTA_T) const {
    //         UNUSED_PARAMETER(lookBack);
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn was_remote_controlled(&self, lookBack: SUMOTime) -> bool {
        let _ = lookBack;
        false
    }

    /// C++: `bool MSBaseVehicle::isFrontOnLane(const MSLane *) const` [const, virtual]
    /// Returns the information whether the front of the vehicle is on the given lane
    // virtual bool isFrontOnLane(const MSLane*) const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn is_front_on_lane(&self, arg0: Ptr<MSLane>) -> bool {
        true
    }

    /// C++: `double MSBaseVehicle::getLateralPositionOnLane() const` [const, virtual]
    /// Get the vehicle's lateral position on the lane
    // virtual double getLateralPositionOnLane() const {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_lateral_position_on_lane(&self) -> f64 {
        if MSGlobals::use_meso_sim() {
            return 0.0;
        }
        self.as_ms_vehicle().get_lateral_position_on_lane()
    }

    /// C++: `double MSBaseVehicle::getRightSideOnEdge(const MSLane *) const` [const, virtual]
    /// Get the vehicle's lateral position on the edge of the given lane (or its current edge if lane == 0)
    // virtual double getRightSideOnEdge(const MSLane* lane = 0) const {
    //         UNUSED_PARAMETER(lane);
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_right_side_on_edge(&self, lane: Ptr<MSLane>) -> f64 {
        0.0
    }

    /// C++: `ConstMSEdgeVector::const_iterator MSBaseVehicle::getRerouteOrigin() const` [const, virtual]
    /// Returns the starting point for reroutes (usually the current edge)
    // virtual ConstMSEdgeVector::const_iterator getRerouteOrigin() const {
    //         return myCurrEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_reroute_origin(&self) -> usize /* ConstMSEdgeVector::const_iterator, as an index into myRoute */ {
        if MSGlobals::use_meso_sim() {
            return self.myCurrEdge;
        }
        self.as_ms_vehicle().get_reroute_origin()
    }

    /// C++: `const MSEdge * MSBaseVehicle::getRerouteDestination() const` [const, virtual]
    /// Returns the end point for reroutes (usually the last edge of the route)
    // virtual const MSEdge* getRerouteDestination() const {
    //         return myRoute->getLastEdge();
    //     }
    #[allow(unused_variables)]
    pub fn get_reroute_destination(&self) -> Ptr<MSEdge> {
        self.myRoute.get_last_edge()
    }

    /// C++: `double MSBaseVehicle::getTimeLossSeconds() const` [const, virtual]
    /// Returns the time loss in seconds
    // virtual double getTimeLossSeconds() const {
    //         // better timeLoss for meso?
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_time_loss_seconds(&self) -> f64 {
        if MSGlobals::use_meso_sim() {
            // better timeLoss for meso?
            return 0.0;
        }
        self.as_ms_vehicle().get_time_loss_seconds()
    }

    /// C++: `double MSBaseVehicle::getWaitingSeconds() const` [const]
    /// Returns the number of seconds waited (speed was lesser than 0.1m/s)
    // double getWaitingSeconds() const {
    //         return STEPS2TIME(getWaitingTime());
    //     }
    #[allow(unused_variables)]
    pub fn get_waiting_seconds(&self) -> f64 {
        utils::common::time::steps2time(self.get_waiting_time(false))
    }

    /// C++: `const MSRouteIterator & MSBaseVehicle::getCurrentRouteEdge() const` [const, virtual]
    /// Returns an iterator pointing to the current edge in this vehicles route
    // const MSRouteIterator& getCurrentRouteEdge() const {
    //         return myCurrEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_route_edge(&self) -> &usize /* MSRouteIterator, as an index into myRoute */ {
        &self.myCurrEdge
    }

    /// C++: `bool MSBaseVehicle::reroute(SUMOTime, const std::string &, SUMOAbstractRouter<MSEdge, SUMOVehicle> &, const bool, const bool, const bool, const MSEdge *)` [virtual]
    /// Performs a rerouting using the given router
    // bool
    // MSBaseVehicle::reroute(SUMOTime t, const std::string& info, SUMOAbstractRouter<MSEdge, SUMOVehicle>& router, const bool onInit, const bool withTaz, const bool silent, const MSEdge* sink) {
    //     // check whether to reroute
    //     const MSEdge* source = nullptr;
    //     if (onInit) {
    //         if (withTaz) {
    //             source = MSEdge::dictionary(myParameter->fromTaz + "-source");
    //             if (source == nullptr) {
    //                 source = myRoute->getEdges().front();
    //             }
    //         } else {
    //             source = myRoute->getEdges().front();
    //         }
    //     } else {
    //         source = *getRerouteOrigin();
    //     }
    //     if (sink == nullptr) {
    //         sink = withTaz ? MSEdge::dictionary(myParameter->toTaz + "-sink") : myRoute->getLastEdge();
    //         if (sink == nullptr) {
    //             sink = myRoute->getLastEdge();
    //         }
    //     }
    //     // on insertion, the new route is compared against the complete loaded route
    //     ConstMSEdgeVector oldEdgesRemaining(onInit ? myRoute->begin() : (source == *myCurrEdge ? myCurrEdge : myCurrEdge + 1), myRoute->end());
    //     ConstMSEdgeVector edges;
    //     if (source != sink || !sink->prohibits(this)) {
    //         edges.push_back(source);
    //     }
    //     std::vector<StopEdgeInfo> stops;
    //     std::set<int> jumps;
    //
    //
    //     double sourcePos = onInit ? 0 : getPositionOnLane();
    // #ifdef DEBUG_REROUTE
    //     if (DEBUG_COND) {
    //         std::cout << " curEdge=" << (*myCurrEdge)->getID() << " source=" << source->getID() << " sourcePos=" << sourcePos << " lane=" << Named::getIDSecure(getLane()) << " departPos=" << myParameter->departPos << " oldEdgesRemaining=" << toString(oldEdgesRemaining) << "\n";
    //     }
    // #endif
    //     if (myParameter->via.size() == 0) {
    //         double firstPos = INVALID_DOUBLE;
    //         double lastPos = INVALID_DOUBLE;
    //         stops = getStopEdges(firstPos, lastPos, jumps);
    //         if (stops.size() > 0) {
    //             if (MSGlobals::gUseMesoSim && isStopped()) {
    //                 sourcePos = getNextStop().pars.endPos;
    //             }
    //             if (/*!MSGlobals::gUseMesoSim &&*/ !isStopped() && myStops.front().pars.speed == 0) {
    //                 sourcePos += getBrakeGap();
    //             }
    //             // avoid superfluous waypoints for first and last edge
    //             const bool skipFirst = stops.front().edge == source && (source != getEdge() || sourcePos <= firstPos + NUMERICAL_EPS);
    // #ifdef DEBUG_REROUTE
    //             if (DEBUG_COND) {
    //                 std::cout << SIMTIME << " reroute " << info << " veh=" << getID() << " lane=" << Named::getIDSecure(getLane())
    //                           << " source=" << source->getID() << " sourcePos=" << sourcePos << " firstPos=" << firstPos << " arrivalPos=" << myArrivalPos << " lastPos=" << lastPos
    //                           << " route=" << toString(myRoute->getEdges()) << " skipFirst=" << skipFirst << "\n";
    //             }
    // #endif
    //             if (stops.size() == 1 && skipFirst) {
    //                 stops.clear();
    //             } else if (skipFirst) {
    //                 sourcePos = stops.front().pos;
    //                 stops.erase(stops.begin());
    //             }
    //         }
    //     } else {
    //         std::set<const MSEdge*> jumpEdges;
    //         std::map<const MSEdge*, StopEdgeInfo> stopsOnVia;
    //         for (const MSStop& stop : myStops) {
    //             if (stop.pars.jump >= 0) {
    //                 jumpEdges.insert(*stop.edge);
    //             }
    //             auto itsov = stopsOnVia.find(*stop.edge);
    //             if (itsov == stopsOnVia.end()) {
    //                 stopsOnVia.insert({*stop.edge, StopEdgeInfo(*stop.edge, stop.pars.priority, stop.getArrivalFallback(), stop.get_end_pos(*this))});
    //             } else {
    //                 itsov->second.priority = addStopPriority(itsov->second.priority, stop.pars.priority);
    //             }
    //         }
    //         // via takes precedence over stop edges
    //         // there is a consistency check in MSRouteHandler::addStop that warns when a stop edge is not part of the via edges
    //         for (std::vector<std::string>::const_iterator it = myParameter->via.begin(); it != myParameter->via.end(); ++it) {
    //             MSEdge* viaEdge = MSEdge::dictionary(*it);
    //             if ((viaEdge == source && it == myParameter->via.begin()) || (viaEdge == sink && myParameter->via.end() - it == 1)) {
    //                 continue;
    //             }
    //             assert(viaEdge != 0);
    //             if (!viaEdge->isTazConnector() && viaEdge->allowedLanes(getVClass()) == nullptr) {
    //                 throw ProcessError(TLF("Vehicle '%' is not allowed on any lane of via edge '%'.", getID(), viaEdge->getID()));
    //             }
    //             auto itsov = stopsOnVia.find(viaEdge);
    //             const double priority = (itsov == stopsOnVia.end() ? -1 : itsov->second.priority);
    //             const SUMOTime arrival = (itsov == stopsOnVia.end() ? -1 : itsov->second.arrival);
    //             const double pos = (itsov == stopsOnVia.end() ? 0 : itsov->second.pos);
    //             stops.push_back(StopEdgeInfo(viaEdge, priority, arrival, pos));
    //             // @todo determine whether the viaEdge is also used by a stop and then use the stop priority here
    //             if (jumpEdges.count(viaEdge) != 0) {
    //                 jumps.insert((int)stops.size());
    //             }
    //         }
    //     }
    //     if ((stops.size() == 0 && (source != sink || sourcePos > myArrivalPos))
    //             || ((stops.size() != 0) && (stops.back().edge != sink || myArrivalPos < stops.back().pos))) {
    //         stops.push_back(StopEdgeInfo(sink, -1, -1, myArrivalPos, true));
    //     }
    // #ifdef DEBUG_REROUTE
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " reroute veh=" << getID() << " onInit=" << onInit << " source=" << source->getID() << " sink=" << sink->getID() << " sourcePos=" << sourcePos << " arrivalPos=" << myArrivalPos << " stops:\n";
    //         for (auto item : stops) {
    //             std::cout << " e=" << item.edge->getID() << " pos=" << item.pos << " isSink=" << item.isSink << "\n";
    //         }
    //     }
    // #endif
    //     int stopIndex = -1;
    //     auto stopIt = myStops.begin();
    //     SUMOTime startTime = t;
    //     bool hasSkipped = false;
    //     const double origSourcePos = sourcePos;
    //     const MSEdge* origSource = source;
    //     const SUMOTime maxDelay = TIME2STEPS(getFloatParam(toString(SUMO_TAG_CLOSING_REROUTE) + ".maxDelay", false, MSTriggeredRerouter::DEFAULT_MAXDELAY, false));
    //     for (auto& stopEdgeInfo : stops) {
    //         const MSEdge* const stopEdge = stopEdgeInfo.edge;
    //         const double priority = stopEdgeInfo.priority;
    //         stopIndex++;
    //         ConstMSEdgeVector into;
    //         if (jumps.count(stopIndex) != 0) {
    //             edges.push_back(stopEdge);
    //             source = stopEdge;
    //             continue;
    //         }
    //         // !!! need to adapt t here
    //         router.compute(source, sourcePos, stopEdge, stopEdgeInfo.pos, this, t, into, silent || priority >= 0);
    // #ifdef DEBUG_REROUTE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " reroute veh=" << getID() << " source=" << source->getID() << " sourcePos=" << sourcePos << " target=" << stopEdgeInfo.edge->getID() << " targetPos=" << stopEdgeInfo.pos << " edges=" << toString(into) << "\n";
    //         }
    // #endif
    //         if (into.size() > 0) {
    //             // stopIt->edge may be invalid when trying to reroute for novel stops
    //             while (stopIt != myStops.end() && stopIt->lane->getNextNormal()->getID() != stopEdge->getID()) {
    //                 stopIt++;
    //             }
    //
    //             startTime += TIME2STEPS(router.recomputeCostsPos(into, this, sourcePos, stopEdgeInfo.pos, startTime));
    //             if (stopIt != myStops.end()) {
    //                 if (stopIt->pars.priority >= 0 && info != "device.rerouting") {
    //                     // consider skipping this stop if it cannot be reached in a timely manner
    //                     if (stopIt != myStops.end()) {
    //                         SUMOTime arrival = stopEdgeInfo.arrival;
    //                         if (arrival > 0) {
    //                             SUMOTime delay = startTime - arrival;
    //                             //std::cout << " t=" << time2string(t) << " veh=" << getID() << " info=" << info << " stopIndex=" << stopIndex
    //                             //   << " into=" << toString(into) << " sourcePos=" << sourcePos << " stopPos=" << stopPos
    //                             //   << " startTime=" << time2string(startTime) << " arrival=" << time2string(arrival) << " delay=" << time2string(delay) << "\n";
    //                             if (delay > 0) {
    //                                 if (delay > maxDelay) {
    //                                     stopEdgeInfo.skipped = true;
    //                                     stopEdgeInfo.delay = delay;
    //                                     hasSkipped = true;
    //                                     continue;
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //                 sourcePos = stopEdgeInfo.pos;
    //                 startTime += stopIt->getMinDuration(startTime);
    //             }
    //             edges.pop_back();
    //             edges.insert(edges.end(), into.begin(), into.end());
    //             if (edges.back()->isTazConnector()) {
    //                 edges.pop_back();
    //             }
    //             source = edges.back();
    //             stopEdgeInfo.routeIndex = (int)edges.size() - 1;
    //         } else {
    //             if (priority >= 0) {
    //                 stopEdgeInfo.skipped = true;
    //                 hasSkipped = true;
    //                 continue;
    //             } else if (stopEdgeInfo.isSink) {
    //                 //error = TLF("Vehicle '%' has no valid route from edge '%' to destination edge '%'.", getID(), source->getID(), stopEdge->getID());
    //                 edges.clear();
    //                 if (source == stopEdge && stops.size() == 1 && sourcePos > myArrivalPos) {
    //                     edges.push_back(stopEdge);
    //                     WRITE_WARNING(TLF("Vehicle '%' ignores arrivalPos % when driving from edge '%' to edge '%' with departPos %.",
    //                                 getID(), myArrivalPos, source->getID(), stopEdge->getID(), sourcePos));
    //                 }
    //             } else if (source == stopEdge && stopEdgeInfo.stopPar != nullptr && stopEdgeInfo.stopPar->endPos >= sourcePos) {
    //                 // special case: no failure on dynamically computed stop position
    //                 edges.clear();
    //             } else {
    //                 std::string error = TLF("Vehicle '%' has no valid route from edge '%' to stop edge '%'.", getID(), source->getID(), stopEdge->getID());
    //                 if (MSGlobals::gCheckRoutes || silent) {
    //                     throw ProcessError(error);
    //                 } else {
    //                     WRITE_WARNING(error);
    //                     edges.push_back(source);
    //                     source = stopEdge;
    //                 }
    //             }
    //         }
    //     }
    //     if (hasSkipped) {
    //         MSStopOptimizer opti(this, router, t, maxDelay);
    //         edges = opti.optimizeSkipped(origSource, origSourcePos, stops, edges);
    //         for (auto stop : stops) {
    //             if (stop.skipped || stop.origEdge != nullptr) {
    //                 const MSEdge* origEdge = stop.origEdge == nullptr ? stop.edge : stop.origEdge;
    //                 if (stop.delay > 0) {
    //                     WRITE_WARNING(TLF("Vehicle '%' skips stop on edge '%' with delay % at time %.", getID(), origEdge->getID(), time2string(stop.delay), time2string(SIMSTEP)));
    //                 } else if (stop.backtracked) {
    //                     WRITE_WARNING(TLF("Vehicle '%' skips stop on edge '%' with priority % at time %.", getID(), origEdge->getID(), stop.priority, time2string(SIMSTEP)));
    //                 } else {
    //                     WRITE_WARNING(TLF("Vehicle '%' skips unreachable stop on edge '%' with priority % at time %.", getID(), origEdge->getID(), stop.priority, time2string(SIMSTEP)));
    //                 }
    //             }
    //         }
    //     }
    //
    //     // router.setHint(myCurrEdge, myRoute->end(), this, t);
    //     if (edges.empty() && silent) {
    //         return false;
    //     }
    //     if (!edges.empty() && edges.front()->isTazConnector()) {
    //         edges.erase(edges.begin());
    //     }
    //     if (!edges.empty() && edges.back()->isTazConnector()) {
    //         edges.pop_back();
    //     }
    //     const double routeCost = router.recomputeCosts(edges, this, t);
    //     // trips have no loaded route to compare against when routed on insertion
    //     const double previousCost = onInit ? routeCost : router.recomputeCosts(oldEdgesRemaining, this, t);
    //     const double savings = previousCost - routeCost;
    //     bool savingsOk = onInit || info != "device.rerouting" || gWeightsRandomFactor != 1;
    //     if (!savingsOk) {
    //         MSDevice_Routing* routingDevice = static_cast<MSDevice_Routing*>(getDevice(typeid(MSDevice_Routing)));
    //         assert(routingDevice != 0);
    //         savingsOk = routingDevice->sufficientSaving(previousCost, routeCost);
    //         if (!savingsOk) {
    //             std::string dummyMsg;
    //             if (!hasValidRoute(dummyMsg, oldEdgesRemaining.begin(), oldEdgesRemaining.end(), true)) {
    //                 // the old route is prohibted (i.e. due to temporary permission changes)
    //                 savingsOk = true;
    //             }
    //         }
    //     }
    //     //if (getID() == "43") std::cout << SIMTIME << " pCost=" << previousCost << " cost=" << routeCost
    //     //    << " onInit=" << onInit
    //     //        << " prevEdges=" << toString(oldEdgesRemaining)
    //     //        << " newEdges=" << toString(edges)
    //     //        << "\n";
    //     if (savingsOk) {
    //         replaceRouteEdges(edges, routeCost, savings, info, onInit, false, hasSkipped);
    //     }
    //     // this must be called even if the route could not be replaced
    //     if (onInit) {
    //         if (edges.empty()) {
    //             if (MSGlobals::gCheckRoutes) {
    //                 throw ProcessError(TLF("Vehicle '%' has no valid route.", getID()));
    //             } else if (source->isTazConnector()) {
    //                 WRITE_WARNINGF(TL("Removing vehicle '%' which has no valid route."), getID());
    //                 MSNet::getInstance()->getInsertionControl().descheduleDeparture(this);
    //                 return false;
    //             }
    //         }
    //         setDepartAndArrivalEdge();
    //         calculateArrivalParams(onInit);
    //     }
    //     return !edges.empty();
    // }
    #[allow(unused_variables)]
    pub fn reroute(&mut self, t: SUMOTime, info: &str, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, onInit: bool, withTaz: bool, silent: bool, sink: Ptr<MSEdge>) -> bool {
        // check whether to reroute
        let mut source: Ptr<MSEdge>;
        if onInit {
            if withTaz {
                source = MSEdge::dictionary_2(&format!("{}-source", self.myParameter.fromTaz));
                if source.is_null() {
                    source = self.myRoute.get_first_edge();
                }
            } else {
                source = self.myRoute.get_first_edge();
            }
        } else {
            source = self.myRoute.index_op(self.base.get_reroute_origin() as i32);
        }
        let mut sink = sink;
        if sink.is_null() {
            sink = if withTaz { MSEdge::dictionary_2(&format!("{}-sink", self.myParameter.toTaz)) } else { self.myRoute.get_last_edge() };
            if sink.is_null() {
                sink = self.myRoute.get_last_edge();
            }
        }
        // on insertion, the new route is compared against the complete loaded route
        let oldStart = if onInit { 0 } else if source == self.get_edge() { self.myCurrEdge } else { self.myCurrEdge + 1 };
        let oldEdgesRemaining: ConstMSEdgeVector = self.myRoute.get_edges().get(oldStart..).unwrap_or(&[]).to_vec();
        let mut edges: ConstMSEdgeVector = Vec::new();
        if source != sink || !sink.prohibits(Ptr::from_mut(&mut self.base)) {
            edges.push(source);
        }
        let mut stops: Vec<MSBaseVehicle_StopEdgeInfo> = Vec::new();
        let mut jumps: BTreeSet<i32> = BTreeSet::new();

        let mut sourcePos = if onInit { 0.0 } else { self.get_position_on_lane() };
        if self.myParameter.via.is_empty() {
            let mut firstPos = utils::common::std_defs::INVALID_DOUBLE;
            let mut lastPos = utils::common::std_defs::INVALID_DOUBLE;
            stops = self.get_stop_edges(&mut firstPos, &mut lastPos, &mut jumps);
            if !stops.is_empty() {
                // shortcut: MSGlobals::gUseMesoSim not ported, so the meso stop-position branch is skipped
                if !self.is_stopped() && self.myStops.front().unwrap().pars.speed == 0.0 {
                    sourcePos += self.get_brake_gap(false);
                }
                // avoid superfluous waypoints for first and last edge
                let skipFirst = stops[0].edge == source && (source != self.get_edge() || sourcePos <= firstPos + utils::geom::NUMERICAL_EPS);
                if stops.len() == 1 && skipFirst {
                    stops.clear();
                } else if skipFirst {
                    sourcePos = stops[0].pos;
                    stops.remove(0);
                }
            }
        } else {
            let mut jumpEdges: BTreeSet<Ptr<MSEdge>> = BTreeSet::new();
            let mut stopsOnVia: BTreeMap<Ptr<MSEdge>, MSBaseVehicle_StopEdgeInfo> = BTreeMap::new();
            for stop in self.myStops.iter() {
                // shortcut: MSStop::edge is Unmapped, the stop edge is taken from its lane
                let stopEdge = Ptr::new(stop.lane.get_edge());
                if stop.pars.jump >= 0 {
                    jumpEdges.insert(stopEdge);
                }
                match stopsOnVia.get_mut(&stopEdge) {
                    None => {
                        stopsOnVia.insert(stopEdge, MSBaseVehicle_StopEdgeInfo::new(stopEdge, stop.pars.priority, stop.get_arrival_fallback(), stop.get_end_pos(&self.base), false));
                    }
                    Some(itsov) => {
                        itsov.priority = MSBaseVehicle::add_stop_priority(itsov.priority, stop.pars.priority);
                    }
                }
            }
            // via takes precedence over stop edges
            // there is a consistency check in MSRouteHandler::addStop that warns when a stop edge is not part of the via edges
            let viaCount = self.myParameter.via.len();
            for it in 0..viaCount {
                let viaEdge = MSEdge::dictionary_2(&self.myParameter.via[it]);
                if (viaEdge == source && it == 0) || (viaEdge == sink && it + 1 == viaCount) {
                    continue;
                }
                if !viaEdge.is_taz_connector() && viaEdge.allowed_lanes_2(self.get_v_class()).is_null() {
                    panic!("{}", utils::tlf!("Vehicle '%' is not allowed on any lane of via edge '%'.", self.get_id(), viaEdge.get_id()));
                }
                let (priority, arrival, pos) = match stopsOnVia.get(&viaEdge) {
                    None => (-1.0, -1, 0.0),
                    Some(itsov) => (itsov.priority, itsov.arrival, itsov.pos),
                };
                stops.push(MSBaseVehicle_StopEdgeInfo::new(viaEdge, priority, arrival, pos, false));
                // @todo determine whether the viaEdge is also used by a stop and then use the stop priority here
                if jumpEdges.contains(&viaEdge) {
                    jumps.insert(stops.len() as i32);
                }
            }
        }
        if (stops.is_empty() && (source != sink || sourcePos > self.myArrivalPos))
            || (!stops.is_empty() && (stops.last().unwrap().edge != sink || self.myArrivalPos < stops.last().unwrap().pos))
        {
            stops.push(MSBaseVehicle_StopEdgeInfo::new(sink, -1.0, -1, self.myArrivalPos, true));
        }

        let mut stopIndex: i32 = -1;
        let mut stopIt: usize = 0;
        let mut startTime = t;
        let mut hasSkipped = false;
        let origSourcePos = sourcePos;
        let origSource = source;
        let maxDelay = utils::common::time::time2steps(self.get_float_param("closingReroute.maxDelay", false, 7200.0, false));
        for idx in 0..stops.len() {
            let stopEdge = stops[idx].edge;
            let priority = stops[idx].priority;
            stopIndex += 1;
            let mut into: ConstMSEdgeVector = Vec::new();
            if jumps.contains(&stopIndex) {
                edges.push(stopEdge);
                source = stopEdge;
                continue;
            }
            // !!! need to adapt t here
            router.compute_2(source, sourcePos, stopEdge, stops[idx].pos, Ptr::from_mut(&mut self.base), t, &mut into, silent || priority >= 0.0);
            if !into.is_empty() {
                // stopIt->edge may be invalid when trying to reroute for novel stops
                while stopIt < self.myStops.len() && self.myStops[stopIt].lane.get_next_normal().get_id() != stopEdge.get_id() {
                    stopIt += 1;
                }

                startTime += utils::common::time::time2steps(router.recompute_costs_pos(&into, Ptr::from_mut(&mut self.base), sourcePos, stops[idx].pos, startTime, Ptr::default()));
                if stopIt < self.myStops.len() {
                    if self.myStops[stopIt].pars.priority >= 0.0 && info != "device.rerouting" {
                        // consider skipping this stop if it cannot be reached in a timely manner
                        let arrival = stops[idx].arrival;
                        if arrival > 0 {
                            let delay = startTime - arrival;
                            if delay > 0 && delay > maxDelay {
                                stops[idx].skipped = true;
                                stops[idx].delay = delay;
                                hasSkipped = true;
                                continue;
                            }
                        }
                    }
                    sourcePos = stops[idx].pos;
                    startTime += self.myStops[stopIt].get_min_duration(startTime);
                }
                edges.pop();
                edges.extend(into.iter().copied());
                if edges.last().is_some_and(|e| e.is_taz_connector()) {
                    edges.pop();
                }
                if let Some(last) = edges.last() {
                    source = *last;
                }
                stops[idx].routeIndex = edges.len() as i32 - 1;
            } else {
                if priority >= 0.0 {
                    stops[idx].skipped = true;
                    hasSkipped = true;
                    continue;
                } else if stops[idx].isSink {
                    edges.clear();
                    if onInit && source == stopEdge && stops.len() == 1 && sourcePos > self.myArrivalPos {
                        edges.push(stopEdge);
                        utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' ignores arrivalPos % when driving from edge '%' to edge '%' with departPos %.",
                            self.get_id(), self.myArrivalPos, source.get_id(), stopEdge.get_id(), sourcePos));
                    }
                } else if source == stopEdge && !stops[idx].stopPar.is_null() && stops[idx].stopPar.endPos >= sourcePos {
                    // special case: no failure on dynamically computed stop position
                    edges.clear();
                } else {
                    let error = utils::tlf!("Vehicle '%' has no valid route from edge '%' to stop edge '%'.", self.get_id(), source.get_id(), stopEdge.get_id());
                    if MSGlobals::check_routes() || silent {
                        panic!("{}", error);
                    } else {
                        utils::common::msg_handler::write_warning(&error);
                        edges.push(source);
                        source = stopEdge;
                    }
                }
            }
        }
        if hasSkipped {
            // shortcut: MSStopOptimizer::optimizeSkipped is not ported, skipped stops keep the edges computed above
            for stop in stops.iter() {
                if stop.skipped || !stop.origEdge.is_null() {
                    let origEdge = if stop.origEdge.is_null() { stop.edge } else { stop.origEdge };
                    if stop.delay > 0 {
                        utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' skips stop on edge '%' with delay % at time %.", self.get_id(), origEdge.get_id(), utils::common::time::time2string(stop.delay), utils::common::time::time2string(MSNet::get_instance().get_current_time_step())));
                    } else if stop.backtracked {
                        utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' skips stop on edge '%' with priority % at time %.", self.get_id(), origEdge.get_id(), stop.priority, utils::common::time::time2string(MSNet::get_instance().get_current_time_step())));
                    } else {
                        utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' skips unreachable stop on edge '%' with priority % at time %.", self.get_id(), origEdge.get_id(), stop.priority, utils::common::time::time2string(MSNet::get_instance().get_current_time_step())));
                    }
                }
            }
        }

        // router.setHint(myCurrEdge, myRoute->end(), this, t);
        if edges.is_empty() && silent {
            return false;
        }
        if !edges.is_empty() && edges[0].is_taz_connector() {
            edges.remove(0);
        }
        if !edges.is_empty() && edges.last().unwrap().is_taz_connector() {
            edges.pop();
        }
        let routeCost = router.recompute_costs(&edges, Ptr::from_mut(&mut self.base), t, Ptr::default());
        // trips have no loaded route to compare against when routed on insertion
        let previousCost = if onInit { routeCost } else { router.recompute_costs(&oldEdgesRemaining, Ptr::from_mut(&mut self.base), t, Ptr::default()) };
        let savings = previousCost - routeCost;
        // shortcut: gWeightsRandomFactor is not a ported global, read the option it is copied from on each use, upgrade when std_defs gains it
        let mut savingsOk = onInit || info != "device.rerouting" || utils::options::options_cont::get_float("weights.random-factor") != 1.0;
        if !savingsOk {
            // shortcut: MSDevice_Routing::sufficientSaving and hasValidRoute over an edge range need the
            // routing device and MSRouteIterator, neither reachable yet; the old route is treated as prohibited
            savingsOk = true;
        }
        if savingsOk {
            self.replace_route_edges(&mut edges, routeCost, savings, info, onInit, false, hasSkipped, Ptr::default());
        }
        // this must be called even if the route could not be replaced
        if onInit {
            if edges.is_empty() {
                if MSGlobals::check_routes() {
                    std::panic::panic_any(ProcessError::new(utils::tlf!("Vehicle '%' has no valid route.", self.get_id())));
                } else if source.is_taz_connector() {
                    utils::common::msg_handler::write_warning(&format!("Removing vehicle '{}' which has no valid route.", self.get_id()));
                    // shortcut: MSNet::getInstance()->getInsertionControl().descheduleDeparture(this) not ported
                    return false;
                }
            }
            self.set_depart_and_arrival_edge();
            self.calculate_arrival_params(onInit);
        }
        !edges.is_empty()
    }

    /// C++: `bool MSBaseVehicle::replaceRouteEdges(ConstMSEdgeVector &, double, double, const std::string &, bool, bool, bool, std::string *)` [virtual]
    /// Replaces the current route by the given edges
    // bool
    // MSBaseVehicle::replaceRouteEdges(ConstMSEdgeVector& edges, double cost, double savings, const std::string& info, bool onInit, bool check, bool removeStops, std::string* msgReturn) {
    //     if (edges.empty()) {
    //         WRITE_WARNINGF(TL("No route for vehicle '%' found."), getID());
    //         if (msgReturn != nullptr) {
    //             *msgReturn = "No route found";
    //         }
    //         return false;
    //     }
    //     // build a new id, first
    //     std::string id = getID();
    //     if (id[0] != '!') {
    //         id = "!" + id;
    //     }
    //     const std::string idSuffix = id + "!var#";
    //     int varIndex = 1;
    //     id = idSuffix + toString(varIndex);
    //     while (MSRoute::hasRoute(id)) {
    //         id = idSuffix + toString(++varIndex);
    //     }
    //     int oldSize = (int)edges.size();
    //     if (!onInit) {
    //         const MSEdge* const origin = *getRerouteOrigin();
    //         if (origin != *myCurrEdge && edges.front() == origin) {
    //             edges.insert(edges.begin(), *myCurrEdge);
    //             oldSize = (int)edges.size();
    //         }
    //         edges.insert(edges.begin(), myRoute->begin(), myCurrEdge);
    //     }
    //     if (edges == myRoute->getEdges() && haveValidStopEdges(true)) {
    //         // re-assign stop iterators when rerouting to a new parkingArea / insertStop
    //         return true;
    //     }
    //     const RGBColor& c = myRoute->getColor();
    //     MSRoute* newRoute = new MSRoute(id, edges, false, &c == &RGBColor::DEFAULT_COLOR ? nullptr : new RGBColor(c), StopParVector());
    //     newRoute->setCosts(cost);
    //     newRoute->setSavings(savings);
    //     ConstMSRoutePtr constRoute = std::shared_ptr<MSRoute>(newRoute);
    //     if (!MSRoute::dictionary(id, constRoute)) {
    //         delete newRoute;
    //         if (msgReturn != nullptr) {
    //             *msgReturn = "duplicate routeID '" + id + "'";
    //         }
    //         return false;
    //     }
    //
    //     std::string msg;
    //     if (check && !hasValidRoute(msg, constRoute)) {
    //         WRITE_WARNINGF(TL("Invalid route replacement for vehicle '%'. %"), getID(), msg);
    //         if (MSGlobals::gCheckRoutes) {
    //             if (msgReturn != nullptr) {
    //                 *msgReturn = msg;
    //             }
    //             return false;
    //         }
    //     }
    //     if (!replaceRoute(constRoute, info, onInit, (int)edges.size() - oldSize, false, removeStops, msgReturn)) {
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn replace_route_edges(&mut self, edges: &mut ConstMSEdgeVector, cost: f64, savings: f64, info: &str, onInit: bool, check: bool, removeStops: bool, msgReturn: Ptr<String>) -> bool {
        if edges.is_empty() {
            utils::common::msg_handler::write_warning(&utils::tlf!("No route for vehicle '%' found.", self.get_id()));
            if !msgReturn.is_null() {
                *msgReturn.m() = "No route found".to_string();
            }
            return false;
        }
        // build a new id, first
        let mut id = self.get_id().to_string();
        if !id.starts_with('!') {
            id = format!("!{}", id);
        }
        let idSuffix = format!("{}!var#", id);
        let mut varIndex = 1;
        id = format!("{}{}", idSuffix, varIndex);
        while MSRoute::has_route(&id) {
            varIndex += 1;
            id = format!("{}{}", idSuffix, varIndex);
        }
        let mut oldSize = edges.len() as i32;
        if !onInit {
            let origin = self.myRoute.index_op(self.base.get_reroute_origin() as i32);
            if origin != self.get_edge() && edges[0] == origin {
                edges.insert(0, self.get_edge());
                oldSize = edges.len() as i32;
            }
        }
        // edges.insert(edges.begin(), myRoute->begin(), myCurrEdge)
        let mut prefixed: ConstMSEdgeVector = self.myRoute.get_edges().get(..self.myCurrEdge).unwrap_or(&[]).to_vec();
        prefixed.append(edges);
        *edges = prefixed;
        if *edges == *self.myRoute.get_edges() && self.have_valid_stop_edges(true) {
            // re-assign stop iterators when rerouting to a new parkingArea / insertStop
            return true;
        }
        // shortcut: the route color is not copied (null means the default color)
        let mut newRoute = MSRoute::new(&id, edges, false, Ptr::default(), &StopParVector::new(), -1, -1);
        newRoute.set_costs(cost);
        newRoute.set_savings(savings);
        let constRoute: ConstMSRoutePtr = Rc::new(newRoute);
        if !MSRoute::dictionary(&id, constRoute.clone()) {
            if !msgReturn.is_null() {
                *msgReturn.m() = format!("duplicate routeID '{}'", id);
            }
            return false;
        }
        let mut msg = String::new();
        if check && !self.has_valid_route(&mut msg, constRoute.clone()) {
            utils::common::msg_handler::write_warning(&utils::tlf!("Invalid route replacement for vehicle '%'. %", self.get_id(), msg));
            if MSGlobals::check_routes() {
                if !msgReturn.is_null() {
                    *msgReturn.m() = msg;
                }
                return false;
            }
        }
        // C++ calls the *virtual* replaceRoute here, so MSVehicle::replaceRoute runs and refreshes the
        // best lanes afterwards. Calling MSBaseVehicle::replace_route directly skips that override, which
        // is why a rerouted vehicle kept its stale best lanes and stopped at the end of the old edge.
        // shortcut: the vehicle class is told apart by the use_meso_sim flag, not by a type tag; upgrade when MSBaseVehicle carries one
        let replaced = if MSGlobals::use_meso_sim() {
            Ptr::from_mut(self).cast::<crate::mesosim::me_vehicle::MEVehicle>().m().replace_route(constRoute, info, onInit, edges.len() as i32 - oldSize, false, removeStops, msgReturn)
        } else {
            Ptr::from_mut(self).cast::<crate::microsim::ms_vehicle::MSVehicle>().m().replace_route(constRoute, info, onInit, edges.len() as i32 - oldSize, false, removeStops, msgReturn)
        };
        if !replaced {
            return false;
        }
        true
    }

    /// C++: `bool MSBaseVehicle::replaceRoute(ConstMSRoutePtr, const std::string &, bool, int, bool, bool, std::string *)` [virtual]
    /// Replaces the current route by the given one
    // bool
    // MSBaseVehicle::replaceRoute(ConstMSRoutePtr newRoute, const std::string& info, bool onInit, int offset, bool addRouteStops, bool removeStops, std::string* msgReturn) {
    //     const ConstMSEdgeVector& edges = newRoute->getEdges();
    //     // rebuild in-vehicle route information
    //     if (onInit) {
    //         myCurrEdge = newRoute->begin();
    //     } else {
    //         MSRouteIterator newCurrEdge = std::find(edges.begin() + offset, edges.end(), *myCurrEdge);
    //         if (newCurrEdge == edges.end()) {
    //             if (msgReturn != nullptr) {
    //                 *msgReturn = TLF("current edge '%' not found in new route", (*myCurrEdge)->getID());
    //             }
    // #ifdef DEBUG_REPLACE_ROUTE
    //             if (DEBUG_COND) {
    //                 std::cout << "  newCurrEdge not found\n";
    //             }
    // #endif
    //             return false;
    //         }
    //         if (getLane() != nullptr) {
    //             if (getLane()->getEdge().isInternal() && (
    //                         (newCurrEdge + 1) == edges.end() || (*(newCurrEdge + 1)) != &(getLane()->getOutgoingViaLanes().front().first->getEdge()))) {
    //                 if (msgReturn != nullptr) {
    //                     *msgReturn = TL("Vehicle is on junction-internal edge leading elsewhere");
    //                 }
    // #ifdef DEBUG_REPLACE_ROUTE
    //                 if (DEBUG_COND) {
    //                     std::cout << "  Vehicle is on junction-internal edge leading elsewhere\n";
    //                 }
    // #endif
    //                 return false;
    //             } else if (getPositionOnLane() > getLane()->getLength()
    //                        && (myCurrEdge + 1) != myRoute->end()
    //                        && (newCurrEdge + 1) != edges.end()
    //                        && *(myCurrEdge + 1) != *(newCurrEdge + 1)) {
    //                 if (msgReturn != nullptr) {
    //                     *msgReturn = TL("Vehicle is moving past junction and committed to move to another successor edge");
    //                 }
    // #ifdef DEBUG_REPLACE_ROUTE
    //                 if (DEBUG_COND) {
    //                     std::cout << "  Vehicle is moving past junction and committed to move to another successor edge\n";
    //                 }
    // #endif
    //                 return false;
    //             }
    //         }
    //         myCurrEdge = newCurrEdge;
    //     }
    //     const bool stopsFromScratch = onInit && myRoute->getStops().empty();
    //     // assign new route
    //     checkRouteRemoval();
    //     myRoute = newRoute;
    //     // update arrival definition
    //     calculateArrivalParams(onInit);
    //     // save information that the vehicle was rerouted
    //     myNumberReroutes++;
    //     myStopUntilOffset += myRoute->getPeriod();
    //     MSNet::getInstance()->informVehicleStateListener(this, MSNet::VehicleState::NEWROUTE, info);
    //     if (!onInit && isRail() && MSRailSignalControl::hasInstance()) {
    //         // we need to update driveways (add/remove reminders) before the next call to MSRailSignalControl::updateSignals
    //         //
    //         // rerouting may be triggered through
    //         // - MoveReminders (executeMove->activateReminders)
    //         //   - rerouters
    //         //   - devices (MSDevice_Stationfinder)
    //         // - TraCI (changeTarget, replaceStop, ...
    //         // - events (MSDevice_Routing::myRerouteCommand, MSDevice_Taxi::triggerDispatch)
    //         //
    //         // Since activateReminders actively modifies reminders, adding/deleting reminders would create a mess
    //         // hence, we use an event to be safe for all case
    //
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(new WrappingCommand<MSBaseVehicle>(this,
    //                 &MSBaseVehicle::activateRemindersOnReroute), SIMSTEP);
    //     }
    // #ifdef DEBUG_REPLACE_ROUTE
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh=" << getID() << " replaceRoute info=" << info << " on " << (*myCurrEdge)->getID()
    //                   << " lane=" << Named::getIDSecure(getLane())
    //                   << " stopsFromScratch=" << stopsFromScratch
    //                   << "  newSize=" << newRoute->getEdges().size()
    //                   << " newIndex=" << (myCurrEdge - newRoute->begin())
    //                   << " edges=" << toString(newRoute->getEdges())
    //                   << "\n";
    //     }
    // #endif
    //     // remove past stops which are not on the route anymore
    //     for (StopParVector::iterator it = myPastStops.begin(); it != myPastStops.end();) {
    //         const MSEdge* stopEdge = (it->edge.empty()) ? &MSLane::dictionary(it->lane)->getEdge() : MSEdge::dictionary(it->edge);
    //         if (std::find(myRoute->begin(), myRoute->end(), stopEdge) == myRoute->end()) {
    //             it = myPastStops.erase(it);
    //         } else {
    //             ++it;
    //         }
    //     }
    //     // if we did not drive yet it may be best to simply reassign the stops from scratch
    //     if (stopsFromScratch) {
    //         myStops.clear();
    //         addStops(!MSGlobals::gCheckRoutes);
    //     } else {
    //         // recheck old stops
    //         MSRouteIterator searchStart = myCurrEdge;
    //         double lastPos = getPositionOnLane() + getBrakeGap();
    //         if (getLane() != nullptr && getLane()->isInternal()
    //                 && myStops.size() > 0 && !myStops.front().lane->isInternal()) {
    //             // searchStart is still incoming to the intersection so lastPos
    //             // relative to that edge must be adapted
    //             lastPos += (*myCurrEdge)->getLength();
    //         }
    //         int stopIndex = 0;
    //         for (std::list<MSStop>::iterator iter = myStops.begin(); iter != myStops.end();) {
    //             int lastToSkip = 0;
    //             double endPos = iter->getEndPos(*this);
    // #ifdef DEBUG_REPLACE_ROUTE
    //             if (DEBUG_COND) {
    //                 std::cout << "     stopEdge=" << iter->lane->getEdge().getID() << " start=" << (searchStart - myCurrEdge) << " endPos=" << endPos << " lastPos=" << lastPos << "\n";
    //             }
    // #endif
    //             if (*searchStart != &iter->lane->getEdge()
    //                     // loop required to reach upstream position
    //                     || endPos + NUMERICAL_EPS < lastPos
    //                     // loop requested at duplicate pos
    //                     || (endPos == lastPos && iter->pars.index == STOP_INDEX_REPEAT)
    //                     // index encodes a skip
    //                     || (iter->pars.index > (int)myPastStops.size() + stopIndex
    //                         // and we are not already past the skipped edge
    //                         && lastToSkip < (int)iter->skips.size()
    //                         && searchStart - myRoute->begin() <= iter->skips[lastToSkip])) {
    //                 if (searchStart != edges.end() && !iter->reached) {
    // #ifdef DEBUG_REPLACE_ROUTE
    //                     if (DEBUG_COND) {
    //                         std::cout << "        skip "  << (*searchStart)->getID() << "\n";
    //                     }
    // #endif
    //                     lastToSkip++;
    //                     searchStart++;
    //                 }
    //             }
    //             lastPos = endPos;
    //
    //             iter->edge = std::find(searchStart, edges.end(), &iter->lane->getEdge());
    // #ifdef DEBUG_REPLACE_ROUTE
    //             if (DEBUG_COND) {
    //                 std::cout << "        foundIndex=" << (iter->edge - myCurrEdge) << " end=" << (edges.end() - myCurrEdge) << "\n";
    //             }
    // #endif
    //             if (iter->edge == edges.end() && iter->pars.priority >= 0) {
    //                 const std::string oldEdge = iter->pars.edge;
    //                 const std::string oldName = iter->getStoppingPlaceName().first;
    //                 if (replaceWithAlternative(iter, searchStart, edges.end())) {
    //                     WRITE_WARNINGF(TL("Vehicle '%' replaced stop on edge '%' (named '%') and now stops at '%' instead; after rerouting (%) at time=%."),
    //                                    getID(), oldEdge, oldName, iter->getDescription(true), info, time2string(SIMSTEP));
    //                 }
    //             }
    //             if (iter->edge == edges.end()) {
    //                 if (!removeStops) {
    //                     WRITE_ERRORF(TL("Vehicle '%' could not assign stop '%' after rerouting (%) at time=%."), getID(), iter->getDescription(), info, time2string(SIMSTEP));
    //                 }
    //                 iter = myStops.erase(iter);
    //                 continue;
    //             } else {
    //                 setSkips(*iter, stopIndex);
    //                 searchStart = iter->edge;
    //             }
    //             ++iter;
    //             stopIndex++;
    //         }
    //         // add new stops
    //         if (addRouteStops) {
    //             for (StopParVector::const_iterator i = newRoute->getStops().begin(); i != newRoute->getStops().end(); ++i) {
    //                 std::string error;
    //                 addStop(*i, error, myParameter->depart + myStopUntilOffset);
    //                 if (error != "") {
    //                     WRITE_WARNING(error);
    //                 }
    //             }
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn replace_route(&mut self, newRoute: ConstMSRoutePtr, info: &str, onInit: bool, offset: i32, addRouteStops: bool, removeStops: bool, msgReturn: Ptr<String>) -> bool {
        // rebuild in-vehicle route information
        if onInit {
            self.myCurrEdge = 0;
        } else {
            let cur = self.get_edge();
            let start = offset.max(0) as usize;
            let newCurrEdge = match newRoute.get_edges().get(start..).and_then(|s| s.iter().position(|e| *e == cur)) {
                Some(p) => start + p,
                None => {
                    if !msgReturn.is_null() {
                        *msgReturn.m() = utils::tlf!("current edge '%' not found in new route", cur.get_id());
                    }
                    return false;
                }
            };
            let lane = self.get_lane();
            if !lane.is_null() {
                // shortcut: the junction-internal check via MSLane::getOutgoingViaLanes is not ported yet
                if self.get_position_on_lane() > lane.get_length()
                    && self.myCurrEdge + 1 != self.myRoute.size() as usize
                    && newCurrEdge + 1 != newRoute.get_edges().len()
                    && self.myRoute.index_op(self.myCurrEdge as i32 + 1) != newRoute.get_edges()[newCurrEdge + 1]
                {
                    if !msgReturn.is_null() {
                        *msgReturn.m() = "Vehicle is moving past junction and committed to move to another successor edge".to_string();
                    }
                    return false;
                }
            }
            self.myCurrEdge = newCurrEdge;
        }
        let stopsFromScratch = onInit && self.myRoute.get_stops().is_empty();
        // assign new route
        self.check_route_removal();
        self.myRoute = newRoute.clone();
        // update arrival definition
        self.calculate_arrival_params(onInit);
        // save information that the vehicle was rerouted
        self.myNumberReroutes += 1;
        self.myStopUntilOffset += self.myRoute.get_period();
        // shortcut: MSNet::informVehicleStateListener(NEWROUTE) is not ported
        // shortcut: MSRailSignalControl driveway update event (activateRemindersOnReroute) is not ported
        // remove past stops which are not on the route anymore
        let route = &self.myRoute;
        self.myPastStops.retain(|it| {
            let stopEdge = if it.edge.is_empty() {
                Ptr::new(MSLane::dictionary_2(&it.lane).get_edge())
            } else {
                MSEdge::dictionary_2(&it.edge)
            };
            route.contains(stopEdge)
        });
        // if we did not drive yet it may be best to simply reassign the stops from scratch
        if stopsFromScratch {
            self.myStops.clear();
            self.add_stops(true, Ptr::default(), true);
        } else {
            // shortcut: re-checking the old stops against the new route (searchStart over MSRouteIterator) is not ported
            if addRouteStops {
                for i in self.myRoute.clone().get_stops().iter() {
                    let mut error = String::new();
                    self.add_stop(i, &mut error, self.myParameter.depart + self.myStopUntilOffset, Ptr::default());
                    if error != "" {
                        utils::common::msg_handler::write_warning(&error);
                    }
                }
            }
        }
        true
    }

    /// C++: `double MSBaseVehicle::getAcceleration() const` [const, virtual]
    /// Returns the vehicle's acceleration
    // double
    // MSBaseVehicle::getAcceleration() const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_acceleration(&self) -> f64 {
        if MSGlobals::use_meso_sim() {
            return 0.0;
        }
        self.as_ms_vehicle().get_acceleration()
    }

    /// C++: `void MSBaseVehicle::onDepart()` [virtual]
    /// Called when the vehicle is inserted into the network
    // void
    // MSBaseVehicle::onDepart() {
    //     myDeparture = MSNet::getInstance()->getCurrentTimeStep();
    //     myDepartPos = getPositionOnLane();
    //     MSNet::getInstance()->getVehicleControl().vehicleDeparted(*this);
    // }
    #[allow(unused_variables)]
    pub fn on_depart(&mut self) {
        self.myDeparture = MSNet::get_instance().get_current_time_step();
        self.myDepartPos = self.get_position_on_lane();
        MSNet::get_instance().get_vehicle_control().vehicle_departed(self);
    }

    /// C++: `SUMOTime MSBaseVehicle::getDeparture() const` [const, virtual]
    /// Returns this vehicle's real departure time
    // inline SUMOTime getDeparture() const {
    //         return myDeparture;
    //     }
    #[allow(unused_variables)]
    pub fn get_departure(&self) -> SUMOTime {
        self.myDeparture
    }

    /// C++: `SUMOTime MSBaseVehicle::getDepartDelay() const` [const, virtual]
    /// Returns the depart delay
    // SUMOTime
    // MSBaseVehicle:: getDepartDelay() const {
    //     const SUMOTime dep = getParameter().depart;
    //     if (dep < 0) {
    //         return 0;
    //     }
    //     return hasDeparted() ? getDeparture() - dep : SIMSTEP - dep;
    // }
    #[allow(unused_variables)]
    pub fn get_depart_delay(&self) -> SUMOTime {
        let dep = self.get_parameter().depart;
        if dep < 0 {
            return 0;
        }
        if self.has_departed() {
            self.get_departure() - dep
        } else {
            MSNet::get_instance().get_current_time_step() - dep
        }
    }

    /// C++: `double MSBaseVehicle::getStopDelay() const` [const, virtual]
    /// Returns the estimated public transport stop (departure) delay in seconds
    // virtual double getStopDelay() const {
    //         /// @todo implement for meso
    //         return -1;
    //     }
    #[allow(unused_variables)]
    pub fn get_stop_delay(&self) -> f64 {
        // @todo implement for meso
        -1.0
    }

    /// C++: `double MSBaseVehicle::getStopArrivalDelay() const` [const, virtual]
    /// Returns the estimated public transport stop arrival delay in seconds
    // virtual double getStopArrivalDelay() const {
    //         /// @todo implement for meso
    //         return INVALID_DOUBLE;
    //     }
    #[allow(unused_variables)]
    pub fn get_stop_arrival_delay(&self) -> f64 {
        // @todo implement for meso
        utils::common::std_defs::INVALID_DOUBLE
    }

    /// C++: `std::pair<double, double> MSBaseVehicle::estimateTimeToNextStop() const` [const, virtual]
    /// return time (s) and distance to the next stop
    // virtual std::pair<double, double> estimateTimeToNextStop() const {
    //         return std::make_pair(-1, -1);
    //     }
    #[allow(unused_variables)]
    pub fn estimate_time_to_next_stop(&self) -> (f64, f64) {
        (-1.0, -1.0)
    }

    /// C++: `double MSBaseVehicle::getDepartPos() const` [const, virtual]
    /// Returns this vehicle's real departure position
    // inline double getDepartPos() const {
    //         return myDepartPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_depart_pos(&self) -> f64 {
        self.myDepartPos
    }

    /// C++: `double MSBaseVehicle::getArrivalPos() const` [const, virtual]
    /// Returns this vehicle's desired arrivalPos for its current route (may change on reroute)
    // virtual double getArrivalPos() const {
    //         return myArrivalPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self) -> f64 {
        self.myArrivalPos
    }

    /// C++: `int MSBaseVehicle::getArrivalLane() const` [const, virtual]
    // virtual int getArrivalLane() const {
    //         return myArrivalLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_lane(&self) -> i32 {
        self.myArrivalLane
    }

    /// C++: `void MSBaseVehicle::setArrivalPos(double)` [virtual]
    /// Sets this vehicle's desired arrivalPos for its current route
    // virtual void setArrivalPos(double arrivalPos) {
    //         myArrivalPos = arrivalPos;
    //     }
    #[allow(unused_variables)]
    pub fn set_arrival_pos(&mut self, arrivalPos: f64) {
        self.myArrivalPos = arrivalPos;
    }

    /// C++: `void MSBaseVehicle::onRemovalFromNet(const MSMoveReminder::Notification)` [virtual]
    /// Called when the vehicle is removed from the network.
    // virtual void onRemovalFromNet(const MSMoveReminder::Notification /*reason*/) {}
    #[allow(unused_variables)]
    pub fn on_removal_from_net(&mut self, arg0: MSMoveReminder_Notification) {
        // empty in C++
    }

    /// C++: `bool MSBaseVehicle::hasDeparted() const` [const, virtual]
    /// Returns whether this vehicle has already departed
    // inline bool hasDeparted() const {
    //         return myDeparture != NOT_YET_DEPARTED;
    //     }
    #[allow(unused_variables)]
    pub fn has_departed(&self) -> bool {
        self.myDeparture != MS_BASE_VEHICLE_NOT_YET_DEPARTED
    }

    /// C++: `bool MSBaseVehicle::hasArrived() const` [const, virtual]
    /// Returns whether this vehicle has already arrived (by default this is true if the vehicle has reached its final edge)
    // bool
    // MSBaseVehicle::hasArrived() const {
    //     return succEdge(1) == nullptr;
    // }
    #[allow(unused_variables)]
    pub fn has_arrived(&self) -> bool {
        self.succ_edge(1).is_null()
    }

    /// C++: `int MSBaseVehicle::getRoutePosition() const` [const, virtual]
    /// return index of edge within route
    // int
    // MSBaseVehicle::getRoutePosition() const {
    //     return (int) std::distance(myRoute->begin(), myCurrEdge);
    // }
    #[allow(unused_variables)]
    pub fn get_route_position(&self) -> i32 {
        self.myCurrEdge as i32
    }

    /// C++: `int MSBaseVehicle::getNumRemainingEdges() const` [const]
    /// return the number of edges remaining in the route (include the current)
    // int
    // MSBaseVehicle::getNumRemainingEdges() const {
    //     if (myParameter->arrivalEdge >= 0) {
    //         return myParameter->arrivalEdge - getRoutePosition() + 1;
    //     } else {
    //         return myRoute->size() - getRoutePosition();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_num_remaining_edges(&self) -> i32 {
        if self.myParameter.arrivalEdge >= 0 {
            self.myParameter.arrivalEdge - self.get_route_position() + 1
        } else {
            self.myRoute.size() - self.get_route_position()
        }
    }

    /// C++: `int MSBaseVehicle::getArrivalIndex() const` [const]
    // int getArrivalIndex() const {
    //         return myParameter->arrivalEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_index(&self) -> i32 {
        self.myParameter.arrivalEdge
    }

    /// C++: `void MSBaseVehicle::resetRoutePosition(int, DepartLaneDefinition)`
    /// reset index of edge within route
    // void
    // MSBaseVehicle::resetRoutePosition(int index, DepartLaneDefinition departLaneProcedure) {
    //     myCurrEdge = myRoute->begin() + index;
    //     const_cast<SUMOVehicleParameter*>(myParameter)->departLaneProcedure = departLaneProcedure;
    //     // !!! hack
    //     myArrivalPos = (*(myRoute->end() - 1))->getLanes()[0]->getLength();
    // }
    #[allow(unused_variables)]
    pub fn reset_route_position(&mut self, index: i32, departLaneProcedure: DepartLaneDefinition) {
        self.myCurrEdge = index as usize;
        self.myParameter.m().departLaneProcedure = departLaneProcedure;
        // !!! hack
        self.myArrivalPos = self.myRoute.get_last_edge().get_lanes()[0].get_length();
    }

    /// C++: `double MSBaseVehicle::getOdometer() const` [const, virtual]
    /// Returns the distance that was already driven by this vehicle
    // double
    // MSBaseVehicle::getOdometer() const {
    //     return -myDepartPos + myOdometer + (hasArrived() ? myArrivalPos : getPositionOnLane());
    // }
    #[allow(unused_variables)]
    pub fn get_odometer(&self) -> f64 {
        // hasArrived() is virtual: MSVehicle / MEVehicle override it
        let arrived = if MSGlobals::use_meso_sim() {
            Ptr::new(self).cast::<crate::mesosim::me_vehicle::MEVehicle>().get().unwrap().has_arrived()
        } else {
            Ptr::new(self).cast::<crate::microsim::ms_vehicle::MSVehicle>().get().unwrap().has_arrived()
        };
        -self.myDepartPos + self.myOdometer + (if arrived { self.myArrivalPos } else { self.get_position_on_lane() })
    }

    /// C++: `void MSBaseVehicle::addToOdometer(double)`
    /// Manipulate the odometer
    // void addToOdometer(double value) {
    //         myOdometer += value;
    //     }
    #[allow(unused_variables)]
    pub fn add_to_odometer(&mut self, value: f64) {
        self.myOdometer += value;
    }

    /// C++: `int MSBaseVehicle::getNumberReroutes() const` [const, virtual]
    /// Returns the number of new routes this vehicle got
    // inline int getNumberReroutes() const {
    //         return myNumberReroutes;
    //     }
    #[allow(unused_variables)]
    pub fn get_number_reroutes(&self) -> i32 {
        self.myNumberReroutes
    }

    /// C++: `double MSBaseVehicle::getImpatience() const` [const, virtual]
    /// Returns this vehicles impatience
    // double
    // MSBaseVehicle::getImpatience() const {
    //     return MAX2(0., MIN2(1., getVehicleType().getImpatience()
    //                          + (hasInfluencer() ? getBaseInfluencer()->getExtraImpatience() : 0)
    //                          + (MSGlobals::gTimeToImpatience > 0 ? (double)getWaitingTime() / (double)MSGlobals::gTimeToImpatience : 0.)));
    // }
    #[allow(unused_variables)]
    pub fn get_impatience(&self) -> f64 {
        let time_to_impatience: SUMOTime = crate::microsim::ms_globals::time_to_impatience();
        let waiting_term = if time_to_impatience > 0 { self.get_waiting_time(false) as f64 / time_to_impatience as f64 } else { 0.0 };
        // hasInfluencer() / getBaseInfluencer() are virtual: MSVehicle / MEVehicle override them
        let influence = if MSGlobals::use_meso_sim() {
            let veh = Ptr::new(self).cast::<crate::mesosim::me_vehicle::MEVehicle>().get().unwrap();
            if veh.has_influencer() { veh.get_base_influencer_2().get_extra_impatience() } else { 0.0 }
        } else {
            let veh = Ptr::new(self).cast::<crate::microsim::ms_vehicle::MSVehicle>().get().unwrap();
            if veh.has_influencer() { veh.get_base_influencer_2().get_extra_impatience() } else { 0.0 }
        };
        f64::max(0.0, f64::min(1.0, self.get_vehicle_type().get_impatience() + influence + waiting_term))
    }

    /// C++: `int MSBaseVehicle::getPersonNumber() const` [const, virtual]
    /// Returns the number of persons
    // int
    // MSBaseVehicle::getPersonNumber() const {
    //     int boarded = myPersonDevice == nullptr ? 0 : myPersonDevice->size();
    //     return boarded + myParameter->personNumber;
    // }
    #[allow(unused_variables)]
    pub fn get_person_number(&self) -> i32 {
        let boarded = if self.myPersonDevice.is_null() { 0 } else { self.myPersonDevice.size() };
        boarded + self.myParameter.personNumber
    }

    /// C++: `int MSBaseVehicle::getLeavingPersonNumber() const` [const]
    /// Returns the number of leaving persons
    // int
    // MSBaseVehicle::getLeavingPersonNumber() const {
    //     int leavingPersonNumber = 0;
    //     const std::vector<MSTransportable*>& persons = getPersons();
    //     for (std::vector<MSTransportable*>::const_iterator it_p = persons.begin(); it_p != persons.end(); ++it_p) {
    //         MSStageDriving* const stage = dynamic_cast<MSStageDriving*>((*it_p)->getCurrentStage());
    //         const MSStop* stop = &myStops.front();
    //         const MSVehicle* joinVeh = dynamic_cast<MSVehicle*>(MSNet::getInstance()->getVehicleControl().getVehicle((*stop).pars.join));
    //         if (stop && stage->canLeaveVehicle(*it_p, *this, *stop) && !MSDevice_Transportable::willTransferAtJoin(*it_p, joinVeh)) {
    //             leavingPersonNumber++;
    //         }
    //     }
    //     return leavingPersonNumber;
    // }
    #[allow(unused_variables)]
    pub fn get_leaving_person_number(&self) -> i32 {
        // shortcut: MSStageDriving::canLeaveVehicle and MSDevice_Transportable::willTransferAtJoin are not ported,
        // so every person currently in the vehicle is counted as leaving
        self.get_persons().len() as i32
    }

    /// C++: `std::vector<std::string> MSBaseVehicle::getPersonIDList() const` [const, virtual]
    /// Returns the list of persons
    // std::vector<std::string>
    // MSBaseVehicle::getPersonIDList() const {
    //     std::vector<std::string> ret;
    //     const std::vector<MSTransportable*>& persons = getPersons();
    //     for (std::vector<MSTransportable*>::const_iterator it_p = persons.begin(); it_p != persons.end(); ++it_p) {
    //         ret.push_back((*it_p)->getID());
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_person_id_list(&self) -> Vec<String> {
        self.get_persons().iter().map(|p| p.get_id().to_string()).collect()
    }

    /// C++: `int MSBaseVehicle::getContainerNumber() const` [const, virtual]
    /// Returns the number of containers
    // int
    // MSBaseVehicle::getContainerNumber() const {
    //     int loaded = myContainerDevice == nullptr ? 0 : myContainerDevice->size();
    //     return loaded + myParameter->containerNumber;
    // }
    #[allow(unused_variables)]
    pub fn get_container_number(&self) -> i32 {
        let loaded = if self.myContainerDevice.is_null() { 0 } else { self.myContainerDevice.size() };
        loaded + self.myParameter.containerNumber
    }

    /// C++: `const std::vector<MSVehicleDevice *> & MSBaseVehicle::getDevices() const` [const, virtual]
    /// Returns this vehicle's devices
    // inline const std::vector<MSVehicleDevice*>& getDevices() const {
    //         return myDevices;
    //     }
    #[allow(unused_variables)]
    pub fn get_devices(&self) -> &Vec<Ptr<MSVehicleDevice>> {
        &self.myDevices
    }

    /// C++: `bool MSBaseVehicle::allowsBoarding(const MSTransportable *) const` [const, virtual]
    /// whether the given transportable is allowed to board this vehicle
    // bool
    // MSBaseVehicle::allowsBoarding(const MSTransportable* t) const {
    //     if (t->isPerson() && getPersonNumber() >= getVehicleType().getPersonCapacity()) {
    //         return false;
    //     } else if (!t->isPerson() && getContainerNumber() >= getVehicleType().getContainerCapacity()) {
    //         return false;
    //     }
    //     if (isStopped() && myStops.begin()->pars.permitted.size() > 0
    //             && myStops.begin()->pars.permitted.count(t->getID()) == 0) {
    //         return false;
    //     }
    //     MSDevice_Taxi* taxiDevice = static_cast<MSDevice_Taxi*>(getDevice(typeid(MSDevice_Taxi)));
    //     if (taxiDevice != nullptr) {
    //         return taxiDevice->allowsBoarding(t);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn allows_boarding(&self, t: Ptr<MSTransportable>) -> bool {
        if t.is_person() && self.get_person_number() >= self.get_vehicle_type().get_person_capacity() {
            return false;
        } else if !t.is_person() && self.get_container_number() >= self.get_vehicle_type().get_container_capacity() {
            return false;
        }
        if self.is_stopped() && self.myStops.front().unwrap().pars.permitted.len() > 0 && !self.myStops.front().unwrap().pars.permitted.contains(t.get_id()) {
            return false;
        }
        // shortcut: MSDevice_Taxi::allowsBoarding (getDevice(typeid(MSDevice_Taxi))) is not reachable yet
        true
    }

    /// C++: `void MSBaseVehicle::addTransportable(MSTransportable *)` [virtual]
    /// Adds a person or container to this vehicle
    // void
    // MSBaseVehicle::addTransportable(MSTransportable* transportable) {
    //     if (transportable->isPerson()) {
    //         if (myPersonDevice == nullptr) {
    //             initTransportableDevice(true);
    //             if (myParameter->departProcedure == DepartDefinition::TRIGGERED && myParameter->depart == -1) {
    //                 const_cast<SUMOVehicleParameter*>(myParameter)->depart = MSNet::getInstance()->getCurrentTimeStep();
    //             }
    //         }
    //         myPersonDevice->addTransportable(transportable);
    //     } else {
    //         if (myContainerDevice == nullptr) {
    //             initTransportableDevice(false);
    //             if (myParameter->departProcedure == DepartDefinition::CONTAINER_TRIGGERED && myParameter->depart == -1) {
    //                 const_cast<SUMOVehicleParameter*>(myParameter)->depart = MSNet::getInstance()->getCurrentTimeStep();
    //             }
    //         }
    //         myContainerDevice->addTransportable(transportable);
    //     }
    //     if (myEnergyParams != nullptr) {
    //         myEnergyParams->setTransportableMass(myEnergyParams->getTransportableMass() + transportable->getVehicleType().getMass());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_transportable(&mut self, transportable: Ptr<MSTransportable>) {
        if transportable.is_person() {
            if self.myPersonDevice.is_null() {
                self.init_transportable_device(true);
                if self.myParameter.departProcedure == DepartDefinition::TRIGGERED && self.myParameter.depart == -1 {
                    self.myParameter.m().depart = MSNet::get_instance().get_current_time_step();
                }
            }
            self.myPersonDevice.m().add_transportable(transportable);
        } else {
            if self.myContainerDevice.is_null() {
                self.init_transportable_device(false);
                if self.myParameter.departProcedure == DepartDefinition::CONTAINER_TRIGGERED && self.myParameter.depart == -1 {
                    self.myParameter.m().depart = MSNet::get_instance().get_current_time_step();
                }
            }
            self.myContainerDevice.m().add_transportable(transportable);
        }
        if !self.myEnergyParams.is_null() {
            let mass = self.myEnergyParams.get_transportable_mass() + transportable.get_vehicle_type().get_mass();
            self.myEnergyParams.m().set_transportable_mass(mass);
        }
    }

    /// C++: `void MSBaseVehicle::initTransportableDevice(bool)`
    /// init device during state loading
    // void
    // MSBaseVehicle::initTransportableDevice(bool isPerson) {
    //     if (isPerson) {
    //         myPersonDevice = MSDevice_Transportable::buildVehicleDevices(*this, myDevices, false);
    //         myMoveReminders.insert(myMoveReminders.begin(), std::make_pair(myPersonDevice, 0.));
    //     } else {
    //         myContainerDevice = MSDevice_Transportable::buildVehicleDevices(*this, myDevices, true);
    //         myMoveReminders.insert(myMoveReminders.begin(), std::make_pair(myContainerDevice, 0.));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_transportable_device(&mut self, isPerson: bool) {
        // shortcut: MSDevice_Transportable::buildVehicleDevices is not wired to myDevices yet, so the device stays null
        let _ = isPerson;
    }

    /// C++: `void MSBaseVehicle::removeTransportable(MSTransportable *)` [virtual]
    /// removes a person or container
    // void
    // MSBaseVehicle::removeTransportable(MSTransportable* t) {
    //     // this might be called from the MSTransportable destructor so we cannot do a dynamic cast to determine the type
    //     if (myPersonDevice != nullptr) {
    //         myPersonDevice->removeTransportable(t);
    //     }
    //     if (myContainerDevice != nullptr) {
    //         myContainerDevice->removeTransportable(t);
    //     }
    //     if (myEnergyParams != nullptr) {
    //         myEnergyParams->setTransportableMass(myEnergyParams->getTransportableMass() - t->getVehicleType().getMass());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_transportable(&mut self, t: Ptr<MSTransportable>) {
        // this might be called from the MSTransportable destructor so we cannot do a dynamic cast to determine the type
        if !self.myPersonDevice.is_null() {
            self.myPersonDevice.m().remove_transportable(t);
        }
        if !self.myContainerDevice.is_null() {
            self.myContainerDevice.m().remove_transportable(t);
        }
        if !self.myEnergyParams.is_null() {
            let mass = self.myEnergyParams.get_transportable_mass() - t.get_vehicle_type().get_mass();
            self.myEnergyParams.m().set_transportable_mass(mass);
        }
    }

    /// C++: `void MSBaseVehicle::removeTransportableMass(MSTransportable *)` [virtual]
    /// removes a person or containers mass
    // void
    // MSBaseVehicle::removeTransportableMass(MSTransportable* t) {
    //     if (myEnergyParams != nullptr) {
    //         myEnergyParams->setTransportableMass(myEnergyParams->getTransportableMass() - t->getVehicleType().getMass());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_transportable_mass(&mut self, t: Ptr<MSTransportable>) {
        if !self.myEnergyParams.is_null() {
            let mass = self.myEnergyParams.get_transportable_mass() - t.get_vehicle_type().get_mass();
            self.myEnergyParams.m().set_transportable_mass(mass);
        }
    }

    /// C++: `const std::vector<MSTransportable *> & MSBaseVehicle::getPersons() const` [const, virtual]
    /// retrieve riding persons
    // const std::vector<MSTransportable*>&
    // MSBaseVehicle::getPersons() const {
    //     if (myPersonDevice == nullptr) {
    //         return myEmptyTransportableVector;
    //     } else {
    //         return myPersonDevice->getTransportables();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_persons(&self) -> &Vec<Ptr<MSTransportable>> {
        if self.myPersonDevice.is_null() {
            MS_BASE_VEHICLE_EMPTY_TRANSPORTABLE_VECTOR
        } else {
            self.myPersonDevice.get_transportables()
        }
    }

    /// C++: `const std::vector<MSTransportable *> & MSBaseVehicle::getContainers() const` [const, virtual]
    /// retrieve riding containers
    // const std::vector<MSTransportable*>&
    // MSBaseVehicle::getContainers() const {
    //     if (myContainerDevice == nullptr) {
    //         return myEmptyTransportableVector;
    //     } else {
    //         return myContainerDevice->getTransportables();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_containers(&self) -> &Vec<Ptr<MSTransportable>> {
        if self.myContainerDevice.is_null() {
            MS_BASE_VEHICLE_EMPTY_TRANSPORTABLE_VECTOR
        } else {
            self.myContainerDevice.get_transportables()
        }
    }

    /// C++: `bool MSBaseVehicle::isLineStop(double) const` [const, virtual]
    /// returns whether the vehicle serves a public transport line that serves the given stop
    // bool
    // MSBaseVehicle::isLineStop(double position) const {
    //     if (myParameter->line == "") {
    //         // not a public transport line
    //         return false;
    //     }
    //     for (const SUMOVehicleParameter::Stop& stop : myParameter->stops) {
    //         if (stop.startPos <= position && position <= stop.endPos) {
    //             return true;
    //         }
    //     }
    //     for (const SUMOVehicleParameter::Stop& stop : myRoute->getStops()) {
    //         if (stop.startPos <= position && position <= stop.endPos) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_line_stop(&self, position: f64) -> bool {
        if self.myParameter.line.is_empty() {
            // not a public transport line
            return false;
        }
        for stop in self.myParameter.stops.iter() {
            if stop.startPos <= position && position <= stop.endPos {
                return true;
            }
        }
        for stop in self.myRoute.get_stops().iter() {
            if stop.startPos <= position && position <= stop.endPos {
                return true;
            }
        }
        false
    }

    /// C++: `bool MSBaseVehicle::hasJump(const MSRouteIterator &) const` [const]
    /// check whether the vehicle has jump at the given part of its route
    // bool
    // MSBaseVehicle::hasJump(const MSRouteIterator& it) const {
    //     for (const MSStop& stop : myStops) {
    //         if (stop.edge == it && stop.pars.jump >= 0) {
    //             return true;
    //         } else if (stop.edge > it) {
    //             return false;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    /// `it` is an index into myRoute's edges (MSRouteIterator is Unmapped)
    pub fn has_jump(&self, it: usize) -> bool {
        // shortcut: MSStop::edge is Unmapped, so the stop's edge is taken from its lane and the
        // early exit on a later stop (stop.edge > it) is not applied
        let edge = self.myRoute.index_op(it as i32);
        for stop in self.myStops.iter() {
            if Ptr::new(stop.lane.get_edge()) == edge && stop.pars.jump >= 0 {
                return true;
            }
        }
        false
    }

    /// C++: `bool MSBaseVehicle::hasValidRoute(std::string &, ConstMSRoutePtr) const` [const, virtual]
    /// Validates the current or given route
    // bool
    // MSBaseVehicle::hasValidRoute(std::string& msg, ConstMSRoutePtr route) const {
    //     MSRouteIterator start = myCurrEdge;
    //     if (route == nullptr) {
    //         route = myRoute;
    //     } else {
    //         start = route->begin();
    //     }
    //     const bool checkJumps = route == myRoute;  // the edge iterators in the stops are invalid otherwise
    //     return hasValidRoute(msg, start, route->end(), checkJumps);
    // }
    #[allow(unused_variables)]
    pub fn has_valid_route(&self, msg: &mut String, route: ConstMSRoutePtr) -> bool {
        let checkJumps = Rc::ptr_eq(&route, &self.myRoute);
        // the edge iterators in the stops are invalid for other routes
        let start = if checkJumps { self.myCurrEdge } else { 0 };
        self.has_valid_route_2(msg, route.get_edges(), start, route.size() as usize, checkJumps)
    }

    /// C++: `bool MSBaseVehicle::hasValidRoute(std::string &, MSRouteIterator, MSRouteIterator, bool) const` [const]
    // bool
    // MSBaseVehicle::hasValidRoute(std::string& msg, MSRouteIterator start, MSRouteIterator last, bool checkJumps) const {
    //     MSRouteIterator lastValid = last - 1;
    //     // check connectivity, first
    //     for (MSRouteIterator e = start; e != lastValid; ++e) {
    //         const MSEdge& next = **(e + 1);
    //         if ((*e)->allowedLanes(next, myType->getVehicleClass()) == nullptr) {
    //             if (!checkJumps || !hasJump(e)) {
    //                 if (!ignoreTransientPermissions()
    //                         || (!next.hasTransientPermissions() && !(*e)->hasTransientPermissions())) {
    //                     msg = TLF("No connection between edge '%' and edge '%'.", (*e)->getID(), (*(e + 1))->getID());
    //                     return false;
    //                 }
    //             }
    //         }
    //     }
    //     // check usable lanes, then
    //     for (MSRouteIterator e = start; e != last; ++e) {
    //         if ((*e)->prohibits(this)) {
    //             msg = TLF("Edge '%' prohibits.", (*e)->getID());
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    /// `start`/`last` are indices into `edges`, the edges of the route being checked (MSRouteIterator is Unmapped)
    pub fn has_valid_route_2(&self, msg: &mut String, edges: &ConstMSEdgeVector, start: usize, last: usize, checkJumps: bool) -> bool {
        let vclass = self.myType.get_vehicle_class();
        // check connectivity, first
        let lastValid = last as i64 - 1;
        let mut e = start;
        // shortcut: guard against running past the edge list (C++ dereferences e + 1 unchecked)
        while (e as i64) != lastValid && e + 1 < last {
            let cur = edges[e];
            let next = edges[e + 1];
            if cur.allowed_lanes(&next, vclass, false).is_null() {
                if !checkJumps || !self.has_jump(e) {
                    if !self.ignore_transient_permissions() || (!next.has_transient_permissions() && !cur.has_transient_permissions()) {
                        *msg = utils::tlf!("No connection between edge '%' and edge '%'.", cur.get_id(), edges[e + 1].get_id());
                        return false;
                    }
                }
            }
            e += 1;
        }
        // check usable lanes, then
        for e in start..last {
            if edges[e].prohibits(Ptr::new(&self.base)) {
                *msg = utils::tlf!("Edge '%' prohibits.", edges[e].get_id());
                return false;
            }
        }
        true
    }

    /// C++: `bool MSBaseVehicle::hasValidRouteStart(std::string &)` [virtual]
    /// checks whether the vehicle can depart on the first edge
    // bool
    // MSBaseVehicle::hasValidRouteStart(std::string& msg) {
    //     if (!(*myCurrEdge)->isTazConnector()) {
    //         if (myParameter->departSpeedProcedure == DepartSpeedDefinition::GIVEN && myParameter->departSpeed > myType->getMaxSpeed() + SPEED_EPS) {
    //             msg = TLF("Departure speed for vehicle '%' is too high for the vehicle type '%'.", getID(), myType->getID());
    //             myRouteValidity |= ROUTE_START_INVALID_LANE;
    //             return false;
    //         }
    //     }
    //     if (myRoute->getEdges().size() > 0 && !(*myCurrEdge)->prohibits(this)) {
    //         myRouteValidity &= ~ROUTE_START_INVALID_PERMISSIONS;
    //         return true;
    //     } else {
    //         msg = TLF("Vehicle '%' is not allowed to depart on any lane of edge '%'.", getID(), (*myCurrEdge)->getID());
    //         myRouteValidity |= ROUTE_START_INVALID_PERMISSIONS;
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn has_valid_route_start(&mut self, msg: &mut String) -> bool {
        let cur = self.get_edge();
        if !cur.is_taz_connector() {
            if self.myParameter.departSpeedProcedure == DepartSpeedDefinition::GIVEN && self.myParameter.departSpeed > self.myType.get_max_speed() + SPEED_EPS {
                *msg = utils::tlf!("Departure speed for vehicle '%' is too high for the vehicle type '%'.", self.get_id(), self.myType.get_id());
                self.myRouteValidity |= MSBaseVehicle_RouteValidity::ROUTE_START_INVALID_LANE as i32;
                return false;
            }
        }
        if self.myRoute.get_edges().len() > 0 && !cur.prohibits(Ptr::new(&self.base)) {
            self.myRouteValidity &= !(MSBaseVehicle_RouteValidity::ROUTE_START_INVALID_PERMISSIONS as i32);
            true
        } else {
            *msg = utils::tlf!("Vehicle '%' is not allowed to depart on any lane of edge '%'.", self.get_id(), cur.get_id());
            self.myRouteValidity |= MSBaseVehicle_RouteValidity::ROUTE_START_INVALID_PERMISSIONS as i32;
            false
        }
    }

    /// C++: `int MSBaseVehicle::getRouteValidity(bool, bool, std::string *)` [virtual]
    /// check for route validity at first insertion attempt
    // int
    // MSBaseVehicle::getRouteValidity(bool update, bool silent, std::string* msgReturn) {
    //     if (!update) {
    //         return myRouteValidity;
    //     }
    //     // insertion check must be done in any case
    //     std::string msg;
    //     if (!hasValidRouteStart(msg)) {
    //         if (MSGlobals::gCheckRoutes) {
    //             throw ProcessError(msg);
    //         } else if (!silent) {
    //             // vehicle will be discarded
    //             WRITE_WARNING(msg);
    //         } else if (msgReturn != nullptr) {
    //             *msgReturn = msg;
    //         }
    //     }
    //     if ((MSGlobals::gCheckRoutes || myRoute->getFirstEdge()->isInternal())
    //             && (myRouteValidity & ROUTE_UNCHECKED) != 0
    //             // we could check after the first rerouting
    //             && (!myParameter->wasSet(VEHPARS_FORCE_REROUTE))) {
    //         if (!hasValidRoute(msg, myRoute)) {
    //             myRouteValidity |= ROUTE_INVALID;
    //             throw ProcessError(TLF("Vehicle '%' has no valid route. %", getID(), msg));
    //         }
    //     }
    //     myRouteValidity &= ~ROUTE_UNCHECKED;
    //     return myRouteValidity;
    // }
    #[allow(unused_variables)]
    pub fn get_route_validity(&mut self, update: bool, silent: bool, msgReturn: Ptr<String>) -> i32 {
        if !update {
            return self.myRouteValidity;
        }
        // insertion check must be done in any case
        let mut msg = String::new();
        if !SUMOVehicle::has_valid_route_start(self, &mut msg) {
            if MSGlobals::check_routes() {
                std::panic::panic_any(ProcessError::new(msg));
            } else if !silent {
                // vehicle will be discarded
                utils::common::msg_handler::write_warning(&msg);
            } else if !msgReturn.is_null() {
                *msgReturn.m() = msg.clone();
            }
        }
        if (MSGlobals::check_routes() || self.myRoute.get_first_edge().is_internal())
            && (self.myRouteValidity & MSBaseVehicle_RouteValidity::ROUTE_UNCHECKED as i32) != 0
            // we could check after the first rerouting
            && (self.myParameter.parametersSet & VEHPARS_FORCE_REROUTE) == 0
        {
            let route = self.myRoute.clone();
            if !self.has_valid_route(&mut msg, route) {
                self.myRouteValidity |= MSBaseVehicle_RouteValidity::ROUTE_INVALID as i32;
                panic!("{}", utils::tlf!("Vehicle '%' has no valid route. %", self.get_id(), msg));
            }
        }
        self.myRouteValidity &= !(MSBaseVehicle_RouteValidity::ROUTE_UNCHECKED as i32);
        self.myRouteValidity
    }

    /// C++: `bool MSBaseVehicle::hasReminder(MSMoveReminder *) const` [const]
    /// Checks whether the vehicle has the given MoveReminder
    // bool
    // MSBaseVehicle::hasReminder(MSMoveReminder* rem) const {
    //     for (auto item : myMoveReminders) {
    //         if (item.first == rem) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_reminder(&self, rem: Ptr<MSMoveReminder>) -> bool {
        self.myMoveReminders.iter().any(|item| item.0 == rem)
    }

    /// C++: `void MSBaseVehicle::addReminder(MSMoveReminder *, double)`
    /// Adds a MoveReminder dynamically
    // void
    // MSBaseVehicle::addReminder(MSMoveReminder* rem, double pos) {
    // #ifdef _DEBUG
    //     if (myTraceMoveReminders) {
    //         traceMoveReminder("add", rem, pos, true);
    //     }
    // #endif
    //     myMoveReminders.push_back(std::make_pair(rem, pos));
    // }
    #[allow(unused_variables)]
    pub fn add_reminder(&mut self, rem: Ptr<MSMoveReminder>, pos: f64) {
        self.myMoveReminders.push((rem, pos));
    }

    /// C++: `void MSBaseVehicle::removeReminder(MSMoveReminder *)`
    /// Removes a MoveReminder dynamically
    // void
    // MSBaseVehicle::removeReminder(MSMoveReminder* rem) {
    //     for (MoveReminderCont::iterator r = myMoveReminders.begin(); r != myMoveReminders.end(); ++r) {
    //         if (r->first == rem) {
    // #ifdef _DEBUG
    //             if (myTraceMoveReminders) {
    //                 traceMoveReminder("remove", rem, 0, false);
    //             }
    // #endif
    //             myMoveReminders.erase(r);
    //             return;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_reminder(&mut self, rem: Ptr<MSMoveReminder>) {
        if let Some(p) = self.myMoveReminders.iter().position(|r| r.0 == rem) {
            self.myMoveReminders.remove(p);
        }
    }

    /// C++: `void MSBaseVehicle::activateReminders(const MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// "Activates" all current move reminder
    // void
    // MSBaseVehicle::activateReminders(const MSMoveReminder::Notification reason, const MSLane* enteredLane) {
    //     // notifyEnter may cause new reminders to be added so we cannot use an iterator
    //     for (int i = 0; i < (int)myMoveReminders.size();) {
    //         MSMoveReminder* rem = myMoveReminders[i].first;
    //         const double remPos = myMoveReminders[i].second;
    //         // skip the reminder if it is a lane reminder but not for my lane (indicated by rem->second > 0.)
    //         if (rem->getLane() != nullptr && remPos > 0.) {
    // #ifdef _DEBUG
    //             if (myTraceMoveReminders) {
    //                 traceMoveReminder("notifyEnter_skipped", rem, remPos, true);
    //             }
    // #endif
    //             ++i;
    //         } else {
    //             if (rem->notifyEnter(*this, reason, enteredLane)) {
    // #ifdef _DEBUG
    //                 if (myTraceMoveReminders) {
    //                     traceMoveReminder("notifyEnter", rem, remPos, true);
    //                 }
    // #endif
    //                 ++i;
    //             } else {
    // #ifdef _DEBUG
    //                 if (myTraceMoveReminders) {
    //                     traceMoveReminder("notifyEnter", rem, remPos, false);
    //                 }
    // #endif
    //                 myMoveReminders.erase(myMoveReminders.begin() + i);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn activate_reminders(&mut self, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) {
        // notifyEnter may cause new reminders to be added so we cannot use an iterator
        let mut i = 0;
        while i < self.myMoveReminders.len() {
            let mut rem = self.myMoveReminders[i].0;
            let remPos = self.myMoveReminders[i].1;
            // skip the reminder if it is a lane reminder but not for my lane (indicated by rem->second > 0.)
            if !rem.get_lane().is_null() && remPos > 0.0 {
                i += 1;
            } else if rem.notify_enter(self, reason, enteredLane) {
                i += 1;
            } else {
                self.myMoveReminders.remove(i);
            }
        }
    }

    /// C++: `double MSBaseVehicle::getLength() const` [const, virtual]
    /// Returns the vehicle's length
    // inline double getLength() const {
    //         return myType->getLength();
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myType.get_length()
    }

    /// C++: `bool MSBaseVehicle::isRail() const` [const, virtual]
    // bool
    // MSBaseVehicle::isRail() const {
    //     return isRailway(getVClass()) || isRailway(getCurrentEdge()->getPermissions());
    // }
    #[allow(unused_variables)]
    pub fn is_rail(&self) -> bool {
        // getCurrentEdge() is virtual (MSVehicle returns the lane's edge): go through the SUMOVehicle vtable
        utils::common::sumo_vehicle_class::is_railway(self.get_v_class()) || utils::common::sumo_vehicle_class::is_railway(self.base.get_current_edge().get_permissions())
    }

    /// C++: `double MSBaseVehicle::getWidth() const` [const]
    /// Returns the vehicle's width
    // inline double getWidth() const {
    //         return myType->getWidth();
    //     }
    #[allow(unused_variables)]
    pub fn get_width(&self) -> f64 {
        self.myType.get_width()
    }

    /// C++: `double MSBaseVehicle::getChosenSpeedFactor() const` [const, virtual]
    /// Returns the precomputed factor by which the driver wants to be faster than the speed limit
    // inline double getChosenSpeedFactor() const {
    //         return myChosenSpeedFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_chosen_speed_factor(&self) -> f64 {
        self.myChosenSpeedFactor
    }

    /// C++: `double MSBaseVehicle::getDesiredMaxSpeed() const` [const]
    // inline double getDesiredMaxSpeed() const {
    //         return myType->getDesiredMaxSpeed() * myChosenSpeedFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_desired_max_speed(&self) -> f64 {
        self.myType.get_desired_max_speed() * self.myChosenSpeedFactor
    }

    /// C++: `void MSBaseVehicle::setChosenSpeedFactor(const double)` [virtual]
    /// Returns the precomputed factor by which the driver wants to be faster than the speed limit
    // inline void setChosenSpeedFactor(const double factor) {
    //         myChosenSpeedFactor = factor;
    //     }
    #[allow(unused_variables)]
    pub fn set_chosen_speed_factor(&mut self, factor: f64) {
        self.myChosenSpeedFactor = factor;
    }

    /// C++: `MSDevice * MSBaseVehicle::getDevice(const std::type_info &) const` [const, virtual]
    /// Returns a device of the given type if it exists, nullptr otherwise
    // MSDevice*
    // MSBaseVehicle::getDevice(const std::type_info& type) const {
    //     for (MSVehicleDevice* const dev : myDevices) {
    //         if (typeid(*dev) == type) {
    //             return dev;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_device(&self, type_: &Unmapped /* std::type_info */) -> Ptr<MSDevice> {
        // shortcut: typeid(*dev) == type needs a device-type key; no device matches until that is ported
        Ptr::default()
    }

    /// C++: `void MSBaseVehicle::replaceVehicleType(const MSVehicleType *)` [virtual]
    /// Replaces the current vehicle type by the one given
    // void
    // MSBaseVehicle::replaceVehicleType(const MSVehicleType* type) {
    //     assert(type != nullptr);
    //     // save old parameters before possible type deletion
    //     const double oldMu = myType->getSpeedFactor().getParameter(0);
    //     const double oldDev = myType->getSpeedFactor().getParameter(1);
    //     if (myType->isVehicleSpecific() && type != myType) {
    //         MSNet::getInstance()->getVehicleControl().removeVType(myType);
    //     }
    //     // adapt myChosenSpeedFactor to the new type
    //     if (oldDev == 0.) {
    //         // old type had speedDev 0, reroll
    //         myChosenSpeedFactor = type->computeChosenSpeedDeviation(getParameter().speedFactor, getRNG());
    //     } else {
    //         // map old speedFactor onto new distribution
    //         const double distPoint = (myChosenSpeedFactor - oldMu) / oldDev;
    //         const double newMu = type->getSpeedFactor().getParameter(0);
    //         const double newDev = type->getSpeedFactor().getParameter(1);
    //         myChosenSpeedFactor = newMu + distPoint * newDev;
    //         // respect distribution limits
    //         myChosenSpeedFactor = MIN2(myChosenSpeedFactor, type->getSpeedFactor().getMax());
    //         myChosenSpeedFactor = MAX2(myChosenSpeedFactor, type->getSpeedFactor().getMin());
    //     }
    //     myType = type;
    //     if (myEnergyParams != nullptr) {
    //         myEnergyParams->setSecondary(type->getEmissionParameters());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn replace_vehicle_type(&mut self, type_: Ptr<MSVehicleType>) {
        // shortcut: the speedFactor distribution (getSpeedFactor()) is Unmapped, so the speed factor
        // remapping onto the new type's distribution is not applied; the chosen factor is kept
        if self.myType.is_vehicle_specific() && type_ != self.myType {
            // shortcut: MSNet::getVehicleControl().removeVType(myType) is not ported
        }
        self.myType = type_;
        if !self.myEnergyParams.is_null() {
            // shortcut: EnergyParams::setSecondary(type->getEmissionParameters()) is not ported
        }
    }

    /// C++: `std::unique_ptr<MFXOptionalLock> MSBaseVehicle::getScopeLock()` [virtual]
    // virtual std::unique_ptr<MFXOptionalLock> getScopeLock() {
    //         return std::unique_ptr<MFXOptionalLock>(new MFXNoOpLock());
    //     }
    #[allow(unused_variables)]
    pub fn get_scope_lock(&mut self) -> Box<Unmapped /* MFXOptionalLock */> {
        // shortcut: MFXOptionalLock / MFXNoOpLock are not ported; a unit placeholder guard is returned
        Box::new(())
    }

    /// C++: `MSVehicleType & MSBaseVehicle::getSingularType()`
    /// Replaces the current vehicle type with a new one used by this vehicle only
    // MSVehicleType&
    // MSBaseVehicle::getSingularType() {
    //     if (myType->isVehicleSpecific()) {
    //         return *const_cast<MSVehicleType*>(myType);
    //     }
    //     MSVehicleType* type = myType->buildSingularType(myType->getID() + "@" + getID());
    //     replaceVehicleType(type);
    //     return *type;
    // }
    #[allow(unused_variables)]
    pub fn get_singular_type(&mut self) -> &mut MSVehicleType {
        if self.myType.is_vehicle_specific() {
            return self.myType.m();
        }
        let id = format!("{}@{}", self.myType.get_id(), self.get_id());
        let type_ = self.myType.build_singular_type(&id);
        // replaceVehicleType() is virtual: MSVehicle rebuilds myCFVariables for the new type
        if MSGlobals::use_meso_sim() {
            self.replace_vehicle_type(type_);
        } else {
            Ptr::from_mut(self).cast::<crate::microsim::ms_vehicle::MSVehicle>().m().replace_vehicle_type(type_);
        }
        type_.get_mut().unwrap()
    }

    /// C++: `void MSBaseVehicle::saveState(OutputDevice &)` [virtual]
    /// Saves the (common) state of a vehicle
    // void
    // MSBaseVehicle::saveState(OutputDevice& out) {
    //     // the parameters may hold the name of a vTypeDistribution but we are interested in the actual type
    //     const std::string& typeID = myParameter->vtypeid != getVehicleType().getID() ? getVehicleType().getID() : "";
    //     myParameter->write(out, OptionsCont::getOptions(), SUMO_TAG_VEHICLE, typeID);
    //     // params and stops must be written in child classes since they may wish to add additional attributes first
    //     out.writeAttr(SUMO_ATTR_ROUTE, myRoute->getID());
    //     std::ostringstream os;
    //     os << myOdometer << " " << myNumberReroutes;
    //     out.writeAttr(SUMO_ATTR_DISTANCE, os.str());
    //     if (myParameter->arrivalPosProcedure == ArrivalPosDefinition::RANDOM) {
    //         out.writeAttr(SUMO_ATTR_ARRIVALPOS_RANDOMIZED, myArrivalPos);
    //     }
    //     if (!myParameter->wasSet(VEHPARS_SPEEDFACTOR_SET)) {
    //         const int precision = out.getPrecision();
    //         out.setPrecision(MAX2(gPrecisionRandom, precision));
    //         out.writeAttr(SUMO_ATTR_SPEEDFACTOR, myChosenSpeedFactor);
    //         out.setPrecision(precision);
    //     }
    //     if (myParameter->wasSet(VEHPARS_FORCE_REROUTE)) {
    //         out.writeAttr(SUMO_ATTR_REROUTE, true);
    //     }
    //     if (!myParameter->wasSet(VEHPARS_LINE_SET) && myParameter->line != "") {
    //         // could be set from stop
    //         out.writeAttr(SUMO_ATTR_LINE, myParameter->line);
    //     }
    //     // here starts the vehicle internal part (see loading)
    //     // @note: remember to close the vehicle tag when calling this in a subclass!
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        // the parameters may hold the name of a vTypeDistribution but we are interested in the actual type
        let typeID = if self.myParameter.vtypeid != self.get_vehicle_type().get_id() { self.get_vehicle_type().get_id().to_string() } else { String::new() };
        utils::options::options_cont::with_options(|oc| self.myParameter.write(out, oc, SUMO_TAG_VEHICLE, &typeID));
        out.write_attr(SUMO_ATTR_ROUTE, self.myRoute.get_id());
        let os = format!("{} {}", self.myOdometer, self.myNumberReroutes);
        out.write_attr(SUMO_ATTR_DISTANCE, &os);
        if self.myParameter.arrivalPosProcedure == ArrivalPosDefinition::RANDOM {
            out.write_attr(SUMO_ATTR_ARRIVALPOS_RANDOMIZED, &self.myArrivalPos);
        }
        if !self.myParameter.was_set(VEHPARS_SPEEDFACTOR_SET) {
            let precision = out.get_precision();
            out.set_precision(std::cmp::max(utils::common::std_defs::g_precision_random(), precision));
            out.write_attr(SUMO_ATTR_SPEEDFACTOR, &self.myChosenSpeedFactor);
            out.set_precision(precision);
        }
        if self.myParameter.was_set(VEHPARS_FORCE_REROUTE) {
            out.write_attr(SUMO_ATTR_REROUTE, &true);
        }
        if !self.myParameter.was_set(VEHPARS_LINE_SET) && self.myParameter.line != "" {
            // could be set from stop
            out.write_attr(SUMO_ATTR_LINE, &self.myParameter.line);
        }
        // here starts the vehicle internal part (see loading)
        // @note: remember to close the vehicle tag when calling this in a subclass!
    }

    /// C++: `bool MSBaseVehicle::handleCollisionStop(MSStop &, const double)` [virtual]
    // bool
    // MSBaseVehicle::handleCollisionStop(MSStop& stop, const double distToStop) {
    //     UNUSED_PARAMETER(stop);
    //     UNUSED_PARAMETER(distToStop);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn handle_collision_stop(&mut self, stop: &mut MSStop, distToStop: f64) -> bool {
        let _ = (stop, distToStop);
        true
    }

    /// C++: `bool MSBaseVehicle::isStopped() const` [const, virtual]
    /// Returns whether the vehicle is at a stop
    // bool
    // MSBaseVehicle::isStopped() const {
    //     return !myStops.empty() && myStops.begin()->reached /*&& myState.mySpeed < SUMO_const_haltingSpeed @todo #1864#*/;
    // }
    #[allow(unused_variables)]
    pub fn is_stopped(&self) -> bool {
        !self.myStops.is_empty() && self.myStops.front().unwrap().reached
    }

    /// C++: `bool MSBaseVehicle::isParking() const` [const, virtual]
    /// Returns whether the vehicle is parking
    // bool
    // MSBaseVehicle::isParking() const {
    //     return (isStopped() && (myStops.begin()->pars.parking == ParkingType::OFFROAD)
    //             && (myStops.begin()->parkingarea == nullptr || !myStops.begin()->parkingarea->parkOnRoad())
    //             && (myStops.begin()->getSpeed() == 0 || getSpeed() < SUMO_const_haltingSpeed));
    // }
    #[allow(unused_variables)]
    pub fn is_parking(&self) -> bool {
        // C++ short-circuits on isStopped(); keep that order, the stop list may be empty
        if !self.is_stopped() {
            return false;
        }
        let front = self.myStops.front().unwrap();
        (front.pars.parking == ParkingType::OFFROAD)
            && (front.parkingarea.is_null() || !front.parkingarea.park_on_road())
            && (front.get_speed() == 0.0 || self.get_speed() < utils::common::std_defs::SUMO_CONST_HALTING_SPEED)
    }

    /// C++: `bool MSBaseVehicle::isJumping() const` [const]
    /// Returns whether the vehicle is perform a jump
    // bool
    // MSBaseVehicle::isJumping() const {
    //     return myPastStops.size() > 0 && myPastStops.back().jump >= 0 && getEdge()->getID() == myPastStops.back().edge && myPastStops.back().ended == SIMSTEP;
    // }
    #[allow(unused_variables)]
    pub fn is_jumping(&self) -> bool {
        match self.myPastStops.last() {
            Some(last) => last.jump >= 0 && self.get_edge().get_id() == last.edge && last.ended == MSNet::get_instance().get_current_time_step(),
            None => false,
        }
    }

    /// C++: `bool MSBaseVehicle::isReversed() const` [const]
    /// Returns whether the logical state of the vehicle is reversed - for drawing
    // inline bool isReversed() const {
    //         return myAmReversed;
    //     }
    #[allow(unused_variables)]
    pub fn is_reversed(&self) -> bool {
        self.myAmReversed
    }

    /// C++: `bool MSBaseVehicle::isStoppedTriggered() const` [const, virtual]
    /// Returns whether the vehicle is on a triggered stop
    // bool
    // MSBaseVehicle::isStoppedTriggered() const {
    //     return isStopped() && (myStops.begin()->triggered || myStops.begin()->containerTriggered || myStops.begin()->joinTriggered);
    // }
    #[allow(unused_variables)]
    pub fn is_stopped_triggered(&self) -> bool {
        self.is_stopped() && {
            let front = self.myStops.front().unwrap();
            front.triggered || front.containerTriggered || front.joinTriggered
        }
    }

    /// C++: `bool MSBaseVehicle::isStoppedParking() const` [const, virtual]
    /// Returns whether the vehicle is on a parking stop
    // bool
    // MSBaseVehicle::isStoppedParking() const {
    //     return isStopped() && (myStops.begin()->pars.parking == ParkingType::OFFROAD);
    // }
    #[allow(unused_variables)]
    pub fn is_stopped_parking(&self) -> bool {
        self.is_stopped() && self.myStops.front().unwrap().pars.parking == ParkingType::OFFROAD
    }

    /// C++: `bool MSBaseVehicle::isStoppedInRange(const double, const double, bool) const` [const, virtual]
    /// return whether the given position is within range of the current stop
    // bool
    // MSBaseVehicle::isStoppedInRange(const double pos, const double tolerance, bool checkFuture) const {
    //     if (isStopped() || (checkFuture && hasStops())) {
    //         const MSStop& stop = myStops.front();
    //         return stop.pars.startPos - tolerance <= pos && stop.pars.endPos + tolerance >= pos;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_stopped_in_range(&self, pos: f64, tolerance: f64, checkFuture: bool) -> bool {
        if self.is_stopped() || (checkFuture && self.has_stops()) {
            let stop = self.myStops.front().unwrap();
            return stop.pars.startPos - tolerance <= pos && stop.pars.endPos + tolerance >= pos;
        }
        false
    }

    /// C++: `bool MSBaseVehicle::hasStops() const` [const, virtual]
    /// Returns whether the vehicle has to stop somewhere
    // bool hasStops() const {
    //         return !myStops.empty();
    //     }
    #[allow(unused_variables)]
    pub fn has_stops(&self) -> bool {
        !self.myStops.is_empty()
    }

    /// C++: `void MSBaseVehicle::cleanupParkingReservation()`
    /// unregisters from a parking reservation when changing or skipping stops
    // void
    // MSBaseVehicle::cleanupParkingReservation() {
    //     if (hasStops() && myStops.front().parkingarea != nullptr && myStops.front().parkingarea->isReservable()) {
    //         myStops.front().parkingarea->removeSpaceReservation(this);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn cleanup_parking_reservation(&mut self) {
        if self.has_stops() && !self.myStops.front().unwrap().parkingarea.is_null() && self.myStops.front().unwrap().parkingarea.is_reservable() {
            let me = Ptr::from_mut(&mut self.base);
            self.myStops.front_mut().unwrap().parkingarea.m().remove_space_reservation(me);
        }
    }

    /// C++: `bool MSBaseVehicle::replaceParkingArea(MSParkingArea *, std::string &)` [virtual]
    /// replace the current parking area stop with a new stop with merge duration
    // bool
    // MSBaseVehicle::replaceParkingArea(MSParkingArea* parkingArea, std::string& errorMsg) {
    //     // Check if there is a parking area to be replaced
    //     if (parkingArea == 0) {
    //         errorMsg = "new parkingArea is NULL";
    //         return false;
    //     }
    //     if (myStops.size() == 0) {
    //         errorMsg = "vehicle has no stops";
    //         return false;
    //     }
    //     if (myStops.front().parkingarea == 0) {
    //         errorMsg = "first stop is not at parkingArea";
    //         return false;
    //     }
    //     MSStop& first = myStops.front();
    //     SUMOVehicleParameter::Stop& stopPar = const_cast<SUMOVehicleParameter::Stop&>(first.pars);
    //     std::string oldStopEdgeID = first.lane->getEdge().getID();
    //     // merge subsequent duplicate stops equals to parking area
    //     for (std::list<MSStop>::iterator iter = ++myStops.begin(); iter != myStops.end();) {
    //         if (iter->parkingarea == parkingArea) {
    //             stopPar.duration += iter->duration;
    //             myStops.erase(iter++);
    //         } else {
    //             break;
    //         }
    //     }
    //     stopPar.lane = parkingArea->getLane().getID();
    //     stopPar.parkingarea = parkingArea->getID();
    //     stopPar.startPos = parkingArea->getBeginLanePosition();
    //     stopPar.endPos = parkingArea->getEndLanePosition();
    //     stopPar.index = STOP_INDEX_FIT;
    //     first.edge = myRoute->end(); // will be patched in replaceRoute
    //     first.lane = &parkingArea->getLane();
    //     first.parkingarea = parkingArea;
    //
    //     // patch via edges
    //     std::string newStopEdgeID = parkingArea->getLane().getEdge().getID();
    //     if (myParameter->via.size() > 0 && myParameter->via.front() != newStopEdgeID) {
    //         if (myParameter->via.front() == oldStopEdgeID) {
    //             myParameter->via.erase(myParameter->via.begin());
    //         }
    //         myParameter->via.insert(myParameter->via.begin(), newStopEdgeID);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn replace_parking_area(&mut self, parkingArea: Ptr<MSParkingArea>, errorMsg: &mut String) -> bool {
        // Check if there is a parking area to be replaced
        if parkingArea.is_null() {
            *errorMsg = "new parkingArea is NULL".to_string();
            return false;
        }
        if self.myStops.is_empty() {
            *errorMsg = "vehicle has no stops".to_string();
            return false;
        }
        if self.myStops.front().unwrap().parkingarea.is_null() {
            *errorMsg = "first stop is not at parkingArea".to_string();
            return false;
        }
        let oldStopEdgeID = self.myStops.front().unwrap().lane.get_edge().get_id().to_string();
        // merge subsequent duplicate stops equals to parking area
        let mut idx = 1;
        while idx < self.myStops.len() {
            if self.myStops[idx].parkingarea == parkingArea {
                let dur = self.myStops[idx].duration;
                self.myStops.front_mut().unwrap().pars.duration += dur;
                self.myStops.remove(idx);
            } else {
                break;
            }
        }
        let first = self.myStops.front_mut().unwrap();
        let stopPar = &mut first.pars;
        stopPar.lane = parkingArea.get_lane().get_id().to_string();
        stopPar.parkingarea = parkingArea.get_id().to_string();
        stopPar.startPos = parkingArea.get_begin_lane_position();
        stopPar.endPos = parkingArea.get_end_lane_position();
        stopPar.index = STOP_INDEX_FIT;
        // first.edge = myRoute->end(); // will be patched in replaceRoute
        // shortcut: MSStop::edge is Unmapped, so the "end()" placeholder iterator is not set here (patched in replaceRoute)
        first.lane = Ptr::new(parkingArea.get_lane());
        first.parkingarea = parkingArea;
        // patch via edges
        let newStopEdgeID = parkingArea.get_lane().get_edge().get_id().to_string();
        if self.myParameter.via.len() > 0 && self.myParameter.via[0] != newStopEdgeID {
            if self.myParameter.via[0] == oldStopEdgeID {
                self.myParameter.m().via.remove(0);
            }
            self.myParameter.m().via.insert(0, newStopEdgeID);
        }
        true
    }

    /// C++: `MSParkingArea * MSBaseVehicle::getNextParkingArea()` [virtual]
    /// get the upcoming parking area stop or nullptr
    // MSParkingArea*
    // MSBaseVehicle::getNextParkingArea() {
    //     MSParkingArea* nextParkingArea = nullptr;
    //     if (!myStops.empty()) {
    //         SUMOVehicleParameter::Stop stopPar;
    //         MSStop stop = myStops.front();
    //         if (!stop.reached && stop.parkingarea != nullptr) {
    //             nextParkingArea = stop.parkingarea;
    //         }
    //     }
    //     return nextParkingArea;
    // }
    #[allow(unused_variables)]
    pub fn get_next_parking_area(&mut self) -> Ptr<MSParkingArea> {
        let mut nextParkingArea: Ptr<MSParkingArea> = Ptr::default();
        if let Some(stop) = self.myStops.front() {
            if !stop.reached && !stop.parkingarea.is_null() {
                nextParkingArea = stop.parkingarea;
            }
        }
        nextParkingArea
    }

    /// C++: `MSParkingArea * MSBaseVehicle::getCurrentParkingArea()`
    /// get the current parking area stop or nullptr
    // MSParkingArea*
    // MSBaseVehicle::getCurrentParkingArea() {
    //     MSParkingArea* currentParkingArea = nullptr;
    //     if (isParking()) {
    //         currentParkingArea = myStops.begin()->parkingarea;
    //     }
    //     return currentParkingArea;
    // }
    #[allow(unused_variables)]
    pub fn get_current_parking_area(&mut self) -> Ptr<MSParkingArea> {
        let mut currentParkingArea: Ptr<MSParkingArea> = Ptr::default();
        if self.is_parking() {
            currentParkingArea = self.myStops.front().unwrap().parkingarea;
        }
        currentParkingArea
    }

    /// C++: `const std::vector<std::string> & MSBaseVehicle::getParkingBadges() const` [const, virtual]
    /// get the valid parking access rights (vehicle settings override vehicle type settings)
    // const std::vector<std::string>&
    // MSBaseVehicle::getParkingBadges() const {
    //     if (myParameter->wasSet(VEHPARS_PARKING_BADGES_SET)) {
    //         return myParameter->parkingBadges;
    //     } else {
    //         return getVehicleType().getParkingBadges();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_parking_badges(&self) -> &Vec<String> {
        if self.myParameter.was_set(VEHPARS_PARKING_BADGES_SET) {
            &self.myParameter.parkingBadges
        } else {
            self.get_vehicle_type().get_parking_badges()
        }
    }

    /// C++: `double MSBaseVehicle::basePos(const MSEdge *) const` [const]
    /// departure position where the vehicle fits fully onto the edge (if possible)
    // double
    // MSBaseVehicle::basePos(const MSEdge* edge) const {
    //     double result = MIN2(getVehicleType().get_length() + POSITION_EPS, edge->getMinLength());
    //     if (hasStops()
    //             && myStops.front().edge == myRoute->begin()
    //             && (&myStops.front().lane->getEdge()) == *myStops.front().edge) {
    //         result = MIN2(result, MAX2(0.0, myStops.front().get_end_pos(*this)));
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn base_pos(&self, edge: Ptr<MSEdge>) -> f64 {
        let mut result = f64::min(self.get_vehicle_type().get_length() + utils::geom::POSITION_EPS, edge.get_min_length());
        // shortcut: myStops.front().edge == myRoute->begin() compares an Unmapped iterator; the lane edge is compared instead
        if self.has_stops() && self.myStops.front().unwrap().lane.get_edge() as *const MSEdge == edge.as_raw() as *const MSEdge {
            result = f64::min(result, f64::max(0.0, self.myStops.front().unwrap().get_end_pos(&self.base)));
        }
        result
    }

    /// C++: `bool MSBaseVehicle::addStop(const SUMOVehicleParameter::Stop &, std::string &, SUMOTime, MSRouteIterator *)` [virtual]
    /// Adds a stop
    // bool
    // MSBaseVehicle::addStop(const SUMOVehicleParameter::Stop& stopPar, std::string& errorMsg, SUMOTime untilOffset,
    //                        MSRouteIterator* searchStart) {
    //     MSStop stop(stopPar);
    //     if (stopPar.lane == "") {
    //         MSEdge* e = MSEdge::dictionary(stopPar.edge);
    //         stop.lane = e->getFirstAllowed(getVClass(), false, getRoutingMode());
    //         if (stop.lane == nullptr) {
    //             errorMsg = "Vehicle '" + myParameter->id + "' is not allowed to stop on any lane of edge '" + stopPar.edge + "'.";
    //             return false;
    //         }
    //     } else {
    //         stop.lane = MSLane::dictionary(stopPar.lane);
    //         if (stop.lane == nullptr) {
    //             // must be an opposite stop
    //             SUMOVehicleParameter::Stop tmp = stopPar;
    //             stop.lane = interpretOppositeStop(tmp);
    //             assert(stop.lane != nullptr);
    //         }
    //         if (!stop.lane->allowsVehicleClass(myType->getVehicleClass(), getRoutingMode())) {
    //             errorMsg = "Vehicle '" + myParameter->id + "' is not allowed to stop on lane '" + stopPar.lane + "'.";
    //             return false;
    //         }
    //     }
    //     if (MSGlobals::gUseMesoSim) {
    //         stop.segment = MSGlobals::gMesoNet->getSegmentForEdge(stop.lane->getEdge(), stop.get_end_pos(*this));
    //         if (stop.lane->isInternal()) {
    //             errorMsg = "Mesoscopic simulation does not allow stopping on internal edge '" + stopPar.edge + "' for vehicle '" + myParameter->id + "'.";
    //             return false;
    //         }
    //     }
    //     stop.init_pars(stopPar);
    //     if (stopPar.until != -1) {
    //         // !!! it would be much cleaner to invent a constructor for stops which takes "until" as an argument
    //         const_cast<SUMOVehicleParameter::Stop&>(stop.pars).until += untilOffset;
    //     }
    //     if (stopPar.arrival != -1) {
    //         const_cast<SUMOVehicleParameter::Stop&>(stop.pars).arrival += untilOffset;
    //     }
    //     std::string stopType = "stop";
    //     std::string stopID = "";
    //     double parkingLength = stop.pars.endPos - stop.pars.startPos;
    //     if (stop.busstop != nullptr) {
    //         stopType = toString(stop.busstop->getElement());
    //         stopID = stop.busstop->getID();
    //         parkingLength = stop.busstop->getParkingLength();
    //     } else if (stop.containerstop != nullptr) {
    //         stopType = "containerStop";
    //         stopID = stop.containerstop->getID();
    //         parkingLength = stop.containerstop->getParkingLength();
    //     } else if (stop.chargingStation != nullptr) {
    //         stopType = "chargingStation";
    //         stopID = stop.chargingStation->getID();
    //         parkingLength = stop.chargingStation->getParkingLength();
    //     } else if (stop.overheadWireSegment != nullptr) {
    //         stopType = "overheadWireSegment";
    //         stopID = stop.overheadWireSegment->getID();
    //         parkingLength = stop.overheadWireSegment->getParkingLength();
    //     } else if (stop.parkingarea != nullptr) {
    //         stopType = "parkingArea";
    //         stopID = stop.parkingarea->getID();
    //     }
    //     const std::string errorMsgStart = stopID == "" ? stopType : stopType + " '" + stopID + "'";
    //
    //     if (stop.pars.startPos < 0 || stop.pars.endPos > stop.lane->getLength()) {
    //         errorMsg = errorMsgStart + " for vehicle '" + myParameter->id + "' on lane '" + stop.lane->getID() + "' has an invalid position.";
    //         return false;
    //     }
    //     if (stopType != "stop" && stopType != "parkingArea" && myType->getLength() / 2. > parkingLength
    //             // do not warn for stops that fill the whole lane
    //             && parkingLength < stop.lane->getLength()
    //             // do not warn twice for the same stop
    //             && MSNet::getInstance()->warnOnce(stopType + ":" + stopID)) {
    //         errorMsg = errorMsgStart + " on lane '" + stop.lane->getID() + "' is too short for vehicle '" + myParameter->id + "'.";
    //     }
    //     if (stopType == "parkingArea" && !stop.parkingarea->accepts(this)) {
    //         // forbid access in case the parking requests other badges
    //         errorMsg = errorMsgStart + "on lane '" + stop.lane->getID() + "' forbids access because vehicle '" + myParameter->id + "' does not provide any valid badge.";
    //         return false;
    //     }
    //     const MSEdge* stopLaneEdge = &stop.lane->getEdge();
    //     const MSEdge* stopEdge;
    //     if (stopLaneEdge->getOppositeEdge() != nullptr && stopLaneEdge->getOppositeEdge()->getID() == stopPar.edge) {
    //         // stop lane is on the opposite side
    //         stopEdge = stopLaneEdge->getOppositeEdge();
    //         stop.isOpposite = true;
    //     } else {
    //         // if stop is on an internal edge the normal edge before the intersection is used
    //         stopEdge = stopLaneEdge->getNormalBefore();
    //     }
    //     MSRouteIterator succ = myCurrEdge + 1; // we're using the address but only within the scope of this function (and recursive calls)
    //     if (searchStart == nullptr) {
    //         searchStart = &myCurrEdge;
    //         if (stopLaneEdge->isNormal() && getLane() != nullptr && getLane()->isInternal()) {
    //             // already on the intersection but myCurrEdge is before it
    //             searchStart = &succ;
    //         }
    //     }
    // #ifdef DEBUG_ADD_STOP
    //     if (DEBUG_COND) {
    //         std::cout << "addStop desc=" << stop.getDescription() << " stopEdge=" << stopEdge->getID()
    //                   << " searchStart=" << ((*searchStart) == myRoute->end() ? "END" : (**searchStart)->getID())
    //                   << " index=" << (int)((*searchStart) - myRoute->begin()) << " route=" << toString(myRoute->getEdges())
    //                   << " stopParIndex=" << stopPar.index
    //                   << "\n";
    //     }
    // #endif
    //     stop.edge = std::find(*searchStart, myRoute->end(), stopEdge);
    //     MSRouteIterator prevStopEdge = myCurrEdge;
    //     const MSEdge* prevEdge = (getLane() == nullptr ? getEdge() : &getLane()->getEdge());
    //     double prevStopPos = getPositionOnLane();
    //     // where to insert the stop
    //     std::list<MSStop>::iterator iter = myStops.begin();
    //     if (stopPar.index == STOP_INDEX_END || stopPar.index >= static_cast<int>(myStops.size()) || stopPar.index == STOP_INDEX_REPEAT) {
    //         iter = myStops.end();
    //         if (myStops.size() > 0 && myStops.back().edge >= *searchStart) {
    //             prevStopEdge = myStops.back().edge;
    //             prevEdge = &myStops.back().lane->getEdge();
    //             prevStopPos = myStops.back().pars.endPos;
    //             stop.edge = std::find(prevStopEdge, myRoute->end(), stopEdge);
    //             if (prevStopEdge == stop.edge                // laneEdge check is insufficient for looped routes
    //                     && prevEdge == &stop.lane->getEdge() // route iterator check insufficient for internal lane stops
    //                     && (prevStopPos > stop.pars.endPos ||
    //                         (prevStopPos == stop.pars.endPos && stopPar.index == STOP_INDEX_REPEAT))) {
    //                 stop.edge = std::find(prevStopEdge + 1, myRoute->end(), stopEdge);
    //             }
    // #ifdef DEBUG_ADD_STOP
    //             if (DEBUG_COND) {
    //                 std::cout << " (@end) prevStopEdge=" << (*prevStopEdge)->getID() << " prevStopPos=" << prevStopPos << " index=" << (int)(prevStopEdge - myRoute->begin())
    //                           << " foundIndex=" << (stop.edge == myRoute->end() ? -1 : (int)(stop.edge - myRoute->begin())) << "\n";
    //             }
    // #endif
    //         }
    //         // skip a number of occurences of stopEdge in looped route
    //         int skipLooped = stopPar.index - static_cast<int>(myStops.size());
    //         for (int j = 0; j < skipLooped; j++) {
    //             auto nextIt = std::find(stop.edge + 1, myRoute->end(), stopEdge);
    //             if (nextIt == myRoute->end()) {
    //                 if (std::find(myRoute->begin(), stop.edge, stopEdge) != stop.edge) {
    //                     // only warn if the route loops over the stop edge at least once
    //                     errorMsg = errorMsgStart + " for vehicle '" + myParameter->id + "' could not skip " + toString(skipLooped) + " occurences of stop edge '" + stopEdge->getID() + "' in looped route.";
    //                 }
    //                 break;
    //             } else {
    //                 stop.edge = nextIt;
    //             }
    //         }
    //     } else {
    //         if (stopPar.index == STOP_INDEX_FIT) {
    //             while (iter != myStops.end() && (iter->edge < stop.edge ||
    //                                              (iter->pars.endPos < stop.pars.endPos && iter->edge == stop.edge) ||
    //                                              (stop.lane->getEdge().isInternal() && iter->edge == stop.edge))) {
    //                 prevStopEdge = iter->edge;
    //                 prevStopPos = iter->pars.endPos;
    //                 ++iter;
    //             }
    //         } else {
    //             int index = stopPar.index;
    //             while (index > 0) {
    //                 prevStopEdge = iter->edge;
    //                 prevStopPos = iter->pars.endPos;
    //                 ++iter;
    //                 --index;
    //             }
    // #ifdef DEBUG_ADD_STOP
    //             if (DEBUG_COND) {
    //                 std::cout << " (@fit) prevStopEdge=" << (*prevStopEdge)->getID() << " index=" << (int)(prevStopEdge - myRoute->begin()) << "\n";
    //             }
    // #endif
    //             stop.edge = std::find(prevStopEdge, myRoute->end(), stopEdge);
    //         }
    //     }
    //     const bool wasTooClose = errorMsg != "" && errorMsg.find("too close") != std::string::npos;
    //     if (stop.edge == myRoute->end()) {
    //         if (!wasTooClose) {
    //             errorMsg = errorMsgStart + " for vehicle '" + myParameter->id + "' on lane '" + stop.lane->getID() + "' is not downstream the current route.";
    //         }
    //         return false;
    //     }
    //
    //     const bool tooClose = (prevStopEdge == stop.edge && prevEdge == &stop.lane->getEdge() &&
    //                            prevStopPos + (iter == myStops.begin() && !instantStopping() ? getBrakeGap() : 0) > stop.pars.endPos + POSITION_EPS);
    //
    //     if (prevStopEdge > stop.edge ||
    //             // a collision-stop happens after vehicle movement and may move the
    //             // vehicle backwards on its lane (prevStopPos is the vehicle position)
    //             (tooClose && !stop.pars.collision)
    //             || (stop.lane->getEdge().isInternal() && stop.lane->getNextNormal() != *(stop.edge + 1))) {
    //         // check if the edge occurs again later in the route
    //         //std::cout << " could not add stop " << errorMsgStart << " prevStops=" << myStops.size() << " searchStart=" << (*searchStart - myRoute->begin()) << " route=" << toString(myRoute->getEdges())  << "\n";
    //         if (tooClose && prevStopPos <= stop.pars.endPos + POSITION_EPS) {
    //             errorMsg = errorMsgStart + " for vehicle '" + myParameter->id + "' on lane '" + stop.pars.lane + "' is too close to brake.";
    //         }
    //         MSRouteIterator next = stop.edge + 1;
    //         return addStop(stopPar, errorMsg, untilOffset, &next);
    //     }
    //     // David.C:
    //     //if (!stop.parking && (myCurrEdge == stop.edge && myState.myPos > stop.endPos - getCarFollowModel().brakeGap(myState.mySpeed))) {
    //     const double endPosOffset = stop.lane->getEdge().isInternal() ? (*stop.edge)->getLength() : 0;
    //     const double distToStop = stop.pars.endPos + endPosOffset - getPositionOnLane();
    //     if (stop.pars.collision && !handleCollisionStop(stop, distToStop)) {
    //         return false;
    //     }
    //     if (!hasDeparted() && myCurrEdge == stop.edge) {
    //         double pos = -1;
    //         if (myParameter->departPosProcedure == DepartPosDefinition::GIVEN) {
    //             pos = myParameter->departPos;
    //             if (pos < 0.) {
    //                 pos += (*myCurrEdge)->getLength();
    //             }
    //         }
    //         if (myParameter->departPosProcedure == DepartPosDefinition::BASE || myParameter->departPosProcedure == DepartPosDefinition::DEFAULT) {
    //             pos = MIN2(stop.pars.endPos + endPosOffset, basePos(*myCurrEdge));
    //         }
    //         if (pos > stop.pars.endPos + endPosOffset) {
    //             if (stop.edge != myRoute->end()) {
    //                 // check if the edge occurs again later in the route
    //                 MSRouteIterator next = stop.edge + 1;
    //                 return addStop(stopPar, errorMsg, untilOffset, &next);
    //             }
    //             errorMsg = errorMsgStart + " for vehicle '" + myParameter->id + "' on lane '" + stop.lane->getID() + "' is before departPos.";
    //             return false;
    //         }
    //     }
    //     if (iter != myStops.begin()) {
    //         std::list<MSStop>::iterator iter2 = iter;
    //         iter2--;
    //         if (stop.getUntil() >= 0 && iter2->getUntil() > stop.getUntil()
    //                 && (!MSGlobals::gUseStopEnded || iter2->pars.ended < 0 || stop.pars.ended >= 0)) {
    //             errorMsg = errorMsgStart + " for vehicle '" + myParameter->id + "' on lane '" + stop.lane->getID()
    //                        + "' set to end at " + time2string(stop.getUntil())
    //                        + " earlier than previous stop at " + time2string(iter2->getUntil()) + ".";
    //         }
    //         if (stop.pars.arrival >= 0 && iter2->pars.arrival > stop.pars.arrival) {
    //             errorMsg = errorMsgStart + " for vehicle '" + myParameter->id + "' on lane '" + stop.lane->getID()
    //                        + "' set to start at " + time2string(stop.pars.arrival)
    //                        + " earlier than previous stop end at " + time2string(iter2->getUntil()) + ".";
    //         }
    //         if (stop.pars.arrival >= 0 && iter2->pars.arrival > stop.pars.arrival) {
    //             errorMsg = errorMsgStart + " for vehicle '" + myParameter->id + "' on lane '" + stop.lane->getID()
    //                        + "' set to start at " + time2string(stop.pars.arrival)
    //                        + " earlier than previous stop arrival at " + time2string(iter2->pars.arrival) + ".";
    //         }
    //     } else {
    //         if (stop.getUntil() >= 0 && getParameter().depart > stop.getUntil()
    //                 && (!MSGlobals::gUseStopEnded || stop.pars.ended < 0)) {
    //             errorMsg = errorMsgStart + " for vehicle '" + myParameter->id + "' on lane '" + stop.lane->getID()
    //                        + "' set to end at " + time2string(stop.getUntil())
    //                        + " earlier than departure at " + time2string(getParameter().depart) + ".";
    //         }
    //     }
    //     if (stop.getUntil() >= 0 && stop.getArrival() > stop.getUntil() && errorMsg == "") {
    //         errorMsg = errorMsgStart + " for vehicle '" + myParameter->id + "' on lane '" + stop.lane->getID()
    //                    + "' set to end at " + time2string(stop.getUntil())
    //                    + " earlier than arrival at " + time2string(stop.getArrival()) + ".";
    //     }
    //     setSkips(stop, (int)myStops.size());
    //     myStops.insert(iter, stop);
    //     if (stopPar.tripId != "") {
    //         MSRailSignalConstraint::storeTripId(stopPar.tripId, getID());
    //     }
    // #ifdef DEBUG_ADD_STOP
    //     if (DEBUG_COND) {
    //         std::cout << " added stop " << errorMsgStart << " totalStops=" << myStops.size()
    //                   << " routeIndex=" << (stop.edge - myRoute->begin())
    //                   << " stopIndex=" << std::distance(myStops.begin(), iter)
    //                   //<< " searchStart=" << (*searchStart - myRoute->begin())
    //                   //<< " route=" << toString(myRoute->getEdges())
    //                   << "\n";
    //     }
    // #endif
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_stop(&mut self, stopPar: &SUMOVehicleParameter_Stop, errorMsg: &mut String, untilOffset: SUMOTime, searchStart: Ptr<MSRouteIterator>) -> bool {
        // shortcut: the parking-area access, too-short-stop warning, meso segment lookup, departPos-before-stop check and
        // stopPar.tripId registration are not ported, upgrade with the parking area and departPos port
        let mut stop = MSStop::new(stopPar);
        if stopPar.lane == "" {
            let e = MSEdge::dictionary_2(&stopPar.edge);
            stop.lane = e.get_first_allowed(self.get_v_class(), false, self.get_routing_mode());
            if stop.lane.is_null() {
                *errorMsg = format!("Vehicle '{}' is not allowed to stop on any lane of edge '{}'.", self.myParameter.id, stopPar.edge);
                return false;
            }
        } else {
            stop.lane = MSLane::dictionary_2(&stopPar.lane);
            if stop.lane.is_null() {
                // must be an opposite stop
                let mut tmp = stopPar.clone();
                stop.lane = Self::interpret_opposite_stop(&mut tmp);
            }
            if !stop.lane.allows_vehicle_class_2(self.myType.get_vehicle_class(), self.get_routing_mode()) {
                *errorMsg = format!("Vehicle '{}' is not allowed to stop on lane '{}'.", self.myParameter.id, stopPar.lane);
                return false;
            }
        }
        stop.init_pars(stopPar);
        if stopPar.until != -1 {
            stop.pars.until += untilOffset;
        }
        if stopPar.arrival != -1 {
            stop.pars.arrival += untilOffset;
        }
        let mut stopType = "stop".to_string();
        let mut stopID = String::new();
        if !stop.busstop.is_null() {
            stopType = if stop.busstop.get_element() == SUMO_TAG_TRAIN_STOP { "trainStop".to_string() } else { "busStop".to_string() };
            stopID = stop.busstop.get_id().to_string();
        } else if !stop.containerstop.is_null() {
            stopType = "containerStop".to_string();
            stopID = stop.containerstop.get_id().to_string();
        } else if !stop.chargingStation.is_null() {
            stopType = "chargingStation".to_string();
            stopID = stop.chargingStation.get_id().to_string();
        } else if !stop.overheadWireSegment.is_null() {
            stopType = "overheadWireSegment".to_string();
            stopID = stop.overheadWireSegment.get_id().to_string();
        } else if !stop.parkingarea.is_null() {
            stopType = "parkingArea".to_string();
            stopID = stop.parkingarea.get_id().to_string();
        }
        let errorMsgStart = if stopID == "" { stopType.clone() } else { format!("{} '{}'", stopType, stopID) };

        if stop.pars.startPos < 0.0 || stop.pars.endPos > stop.lane.get_length() {
            *errorMsg = format!("{} for vehicle '{}' on lane '{}' has an invalid position.", errorMsgStart, self.myParameter.id, stop.lane.get_id());
            return false;
        }
        if stopType == "parkingArea" && !stop.parkingarea.accepts(Ptr::from_mut(self).cast::<SUMOVehicle>()) {
            // forbid access in case the parking requests other badges
            *errorMsg = format!("{}on lane '{}' forbids access because vehicle '{}' does not provide any valid badge.", errorMsgStart, stop.lane.get_id(), self.myParameter.id);
            return false;
        }
        let stopLaneEdge: Ptr<MSEdge> = Ptr::new(stop.lane.get_edge());
        let stopEdge: Ptr<MSEdge>;
        let oppositeEdge: Ptr<MSEdge> = stopLaneEdge.get_opposite_edge();
        if !oppositeEdge.is_null() && oppositeEdge.get_id() == stopPar.edge {
            // stop lane is on the opposite side
            stopEdge = oppositeEdge;
            stop.isOpposite = true;
        } else {
            // if stop is on an internal edge the normal edge before the intersection is used
            stopEdge = stopLaneEdge.get_normal_before();
        }
        let edges: Vec<Ptr<MSEdge>> = self.myRoute.get_edges().to_vec();
        let nEdges = edges.len();
        let findEdge = |from: usize, to: usize, e: Ptr<MSEdge>| -> usize { (from..to).find(|&i| edges[i] == e).unwrap_or(to) };
        let succ: usize = self.myCurrEdge + 1;
        // we're using the address but only within the scope of this function (and recursive calls)
        let searchStartIdx: usize = match searchStart.get() {
            Some(s) => *s,
            None => {
                if stopLaneEdge.is_normal() && !self.get_lane().is_null() && self.get_lane().is_internal() {
                    // already on the intersection but myCurrEdge is before it
                    succ
                } else {
                    self.myCurrEdge
                }
            }
        };
        stop.edge = findEdge(searchStartIdx, nEdges, stopEdge);
        let mut prevStopEdge: usize = self.myCurrEdge;
        let mut prevEdge: Ptr<MSEdge> = if self.get_lane().is_null() { self.get_edge() } else { Ptr::new(self.get_lane().get_edge()) };
        let mut prevStopPos: f64 = self.get_position_on_lane();
        // where to insert the stop
        let mut iter: usize = 0;
        let nStops = self.myStops.len();
        if stopPar.index == STOP_INDEX_END || stopPar.index >= nStops as i32 || stopPar.index == STOP_INDEX_REPEAT {
            iter = nStops;
            if nStops > 0 && self.myStops.back().unwrap().edge >= searchStartIdx {
                prevStopEdge = self.myStops.back().unwrap().edge;
                prevEdge = Ptr::new(self.myStops.back().unwrap().lane.get_edge());
                prevStopPos = self.myStops.back().unwrap().pars.endPos;
                stop.edge = findEdge(prevStopEdge, nEdges, stopEdge);
                if prevStopEdge == stop.edge && prevEdge == Ptr::new(stop.lane.get_edge()) && (prevStopPos > stop.pars.endPos || (prevStopPos == stop.pars.endPos && stopPar.index == STOP_INDEX_REPEAT)) {
                    stop.edge = findEdge(prevStopEdge + 1, nEdges, stopEdge);
                }
            }
            // skip a number of occurences of stopEdge in looped route
            let skipLooped = stopPar.index - nStops as i32;
            for _j in 0..skipLooped {
                let nextIt = findEdge(stop.edge + 1, nEdges, stopEdge);
                if nextIt == nEdges {
                    if findEdge(0, stop.edge, stopEdge) != stop.edge {
                        // only warn if the route loops over the stop edge at least once
                        *errorMsg = format!("{} for vehicle '{}' could not skip {} occurences of stop edge '{}' in looped route.", errorMsgStart, self.myParameter.id, skipLooped, stopEdge.get_id());
                    }
                    break;
                } else {
                    stop.edge = nextIt;
                }
            }
        } else {
            if stopPar.index == STOP_INDEX_FIT {
                while iter < nStops && (self.myStops[iter].edge < stop.edge || (self.myStops[iter].pars.endPos < stop.pars.endPos && self.myStops[iter].edge == stop.edge) || (stop.lane.get_edge().is_internal() && self.myStops[iter].edge == stop.edge)) {
                    prevStopEdge = self.myStops[iter].edge;
                    prevStopPos = self.myStops[iter].pars.endPos;
                    iter += 1;
                }
            } else {
                let mut index = stopPar.index;
                while index > 0 {
                    prevStopEdge = self.myStops[iter].edge;
                    prevStopPos = self.myStops[iter].pars.endPos;
                    iter += 1;
                    index -= 1;
                }
                stop.edge = findEdge(prevStopEdge, nEdges, stopEdge);
            }
        }
        let wasTooClose = !errorMsg.is_empty() && errorMsg.contains("too close");
        if stop.edge == nEdges {
            if !wasTooClose {
                *errorMsg = format!("{} for vehicle '{}' on lane '{}' is not downstream the current route.", errorMsgStart, self.myParameter.id, stop.lane.get_id());
            }
            return false;
        }

        let tooClose = prevStopEdge == stop.edge && prevEdge == Ptr::new(stop.lane.get_edge()) && prevStopPos + (if iter == 0 && !self.instant_stopping() { self.get_brake_gap(false) } else { 0.0 }) > stop.pars.endPos + POSITION_EPS;

        if prevStopEdge > stop.edge
            // a collision-stop happens after vehicle movement and may move the
            // vehicle backwards on its lane (prevStopPos is the vehicle position)
            || (tooClose && !stop.pars.collision)
            || (stop.lane.get_edge().is_internal() && edges.get(stop.edge + 1).map_or(true, |next| stop.lane.get_next_normal() != *next))
        {
            // check if the edge occurs again later in the route
            if tooClose && prevStopPos <= stop.pars.endPos + POSITION_EPS {
                *errorMsg = format!("{} for vehicle '{}' on lane '{}' is too close to brake.", errorMsgStart, self.myParameter.id, stopPar.lane);
            }
            let mut next: usize = stop.edge + 1;
            return self.add_stop(stopPar, errorMsg, untilOffset, Ptr::from_mut(&mut next));
        }
        // shortcut: the departPos check for a stop on the current edge before departure is not ported
        let endPosOffset = if stop.lane.get_edge().is_internal() { edges[stop.edge].get_length() } else { 0.0 };
        let distToStop = stop.pars.endPos + endPosOffset - self.get_position_on_lane();
        if stop.pars.collision {
            // handleCollisionStop() is virtual: MSVehicle overrides it
            let handled = if MSGlobals::use_meso_sim() {
                self.handle_collision_stop(&mut stop, distToStop)
            } else {
                Ptr::from_mut(self).cast::<crate::microsim::ms_vehicle::MSVehicle>().m().handle_collision_stop(&mut stop, distToStop)
            };
            if !handled {
                return false;
            }
        }
        if iter > 0 {
            let iter2 = iter - 1;
            if stop.get_until() >= 0 && self.myStops[iter2].get_until() > stop.get_until() && (!use_stop_ended() || self.myStops[iter2].pars.ended < 0 || stop.pars.ended >= 0) {
                *errorMsg = format!("{} for vehicle '{}' on lane '{}' set to end at {} earlier than previous stop at {}.", errorMsgStart, self.myParameter.id, stop.lane.get_id(), time2string(stop.get_until()), time2string(self.myStops[iter2].get_until()));
            }
            if stop.pars.arrival >= 0 && self.myStops[iter2].pars.arrival > stop.pars.arrival {
                *errorMsg = format!("{} for vehicle '{}' on lane '{}' set to start at {} earlier than previous stop end at {}.", errorMsgStart, self.myParameter.id, stop.lane.get_id(), time2string(stop.pars.arrival), time2string(self.myStops[iter2].get_until()));
            }
            if stop.pars.arrival >= 0 && self.myStops[iter2].pars.arrival > stop.pars.arrival {
                *errorMsg = format!("{} for vehicle '{}' on lane '{}' set to start at {} earlier than previous stop arrival at {}.", errorMsgStart, self.myParameter.id, stop.lane.get_id(), time2string(stop.pars.arrival), time2string(self.myStops[iter2].pars.arrival));
            }
        } else {
            if stop.get_until() >= 0 && self.myParameter.depart > stop.get_until() && (!use_stop_ended() || stop.pars.ended < 0) {
                *errorMsg = format!("{} for vehicle '{}' on lane '{}' set to end at {} earlier than departure at {}.", errorMsgStart, self.myParameter.id, stop.lane.get_id(), time2string(stop.get_until()), time2string(self.myParameter.depart));
            }
        }
        if stop.get_until() >= 0 && stop.get_arrival() > stop.get_until() && *errorMsg == "" {
            *errorMsg = format!("{} for vehicle '{}' on lane '{}' set to end at {} earlier than arrival at {}.", errorMsgStart, self.myParameter.id, stop.lane.get_id(), time2string(stop.get_until()), time2string(stop.get_arrival()));
        }
        self.set_skips(&mut stop, nStops as i32);
        self.myStops.insert(iter, stop);
        true
    }

    /// C++: `void MSBaseVehicle::addStops(const bool, MSRouteIterator *, bool)`
    /// Adds stops to the built vehicle
    // void
    // MSBaseVehicle::addStops(const bool ignoreStopErrors, MSRouteIterator* searchStart, bool addRouteStops) {
    //     if (addRouteStops) {
    //         for (const SUMOVehicleParameter::Stop& stop : myRoute->getStops()) {
    //             std::string errorMsg;
    //             if (!addStop(stop, errorMsg, myParameter->depart, searchStart) && !ignoreStopErrors) {
    //                 throw ProcessError(errorMsg);
    //             }
    //             if (errorMsg != "") {
    //                 WRITE_WARNING(errorMsg);
    //             }
    //         }
    //     }
    //     const SUMOTime untilOffset = myParameter->repetitionOffset > 0 ? myParameter->repetitionsDone * myParameter->repetitionOffset : 0;
    //     for (const SUMOVehicleParameter::Stop& stop : myParameter->stops) {
    //         std::string errorMsg;
    //         if (!addStop(stop, errorMsg, untilOffset, searchStart) && !ignoreStopErrors) {
    //             throw ProcessError(errorMsg);
    //         }
    //         if (errorMsg != "") {
    //             WRITE_WARNING(errorMsg);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_stops(&mut self, ignoreStopErrors: bool, searchStart: Ptr<MSRouteIterator>, addRouteStops: bool) {
        if addRouteStops {
            for stop in self.myRoute.clone().get_stops().iter() {
                let mut errorMsg = String::new();
                if !self.add_stop(stop, &mut errorMsg, self.myParameter.depart, searchStart) && !ignoreStopErrors {
                    panic!("{}", errorMsg);
                }
                if errorMsg != "" {
                    utils::common::msg_handler::write_warning(&errorMsg);
                }
            }
        }
        let untilOffset = if self.myParameter.repetitionOffset > 0 { self.myParameter.repetitionsDone * self.myParameter.repetitionOffset } else { 0 };
        for stop in self.myParameter.get().unwrap().stops.iter() {
            let mut errorMsg = String::new();
            if !self.add_stop(stop, &mut errorMsg, untilOffset, searchStart) && !ignoreStopErrors {
                panic!("{}", errorMsg);
            }
            if errorMsg != "" {
                utils::common::msg_handler::write_warning(&errorMsg);
            }
        }
    }

    /// C++: `bool MSBaseVehicle::haveValidStopEdges(bool) const` [const]
    /// check whether all stop.edge MSRouteIterators are valid and in order
    // bool
    // MSBaseVehicle::haveValidStopEdges(bool silent) const {
    //     MSRouteIterator start = myCurrEdge;
    //     int i = 0;
    //     bool ok = true;
    //     for (const MSStop& stop : myStops) {
    //         MSRouteIterator it;
    //         if (stop.lane->isInternal()) {
    //             // find the normal predecessor and ensure that the next route edge
    //             // matches the successor of the internal edge successor
    //             it = std::find(start, myRoute->end(), stop.lane->getEdge().getNormalBefore());
    //             if (it != myRoute->end() && (
    //                         it + 1 == myRoute->end() || *(it + 1) != stop.lane->getEdge().getNormalSuccessor())) {
    //                 it = myRoute->end(); // signal failure
    //             }
    //         } else {
    //             it = std::find(start, myRoute->end(), &stop.lane->getEdge());
    //         }
    //         if (it == myRoute->end()) {
    //             if (!silent) {
    //                 WRITE_ERRORF("Stop % on edge '%' is not found after edge '%' (% after current) for vehicle '%' at time=%.",
    //                              i, stop.lane->getEdge().getID(), (*start)->getID(), toString(start - myCurrEdge), getID(), time2string(SIMSTEP));
    //             }
    //             ok = false;
    //         } else {
    //             MSRouteIterator it2;
    //             for (it2 = myRoute->begin(); it2 != myRoute->end(); it2++) {
    //                 if (it2 == stop.edge) {
    //                     break;
    //                 }
    //             }
    //             if (it2 == myRoute->end()) {
    //                 if (!silent) {
    //                     WRITE_ERRORF("Stop % on edge '%' used invalid route index for vehicle '%' at time=%.",
    //                                  i, stop.lane->getEdge().getID(), getID(), time2string(SIMSTEP));
    //                 }
    //                 ok = false;
    //             } else if (it2 < start) {
    //                 if (!silent) {
    //                     WRITE_ERRORF("Stop % on edge '%' used invalid (relative) route index % expected after % for vehicle '%' at time=%.",
    //                                  i, stop.lane->getEdge().getID(), toString(it2 - myCurrEdge), toString(start - myCurrEdge), getID(), time2string(SIMSTEP));
    //                 }
    //                 ok = false;
    //             } else {
    //                 start = stop.edge;
    //             }
    //         }
    //         i++;
    //     }
    //     return ok;
    // }
    #[allow(unused_variables)]
    pub fn have_valid_stop_edges(&self, silent: bool) -> bool {
        // shortcut: the internal-lane check (getNormalBefore / getNormalSuccessor) is not applied; only edge
        // presence after the current route index is verified
        let mut start = self.myCurrEdge;
        let mut i = 0;
        let mut ok = true;
        let edges = self.myRoute.get_edges();
        for stop in self.myStops.iter() {
            let stopEdge = Ptr::new(stop.lane.get_edge());
            let found = edges.get(start..).and_then(|s| s.iter().position(|e| *e == stopEdge)).map(|p| start + p);
            match found {
                None => {
                    if !silent {
                        utils::common::msg_handler::write_error(&utils::tlf!("Stop % on edge '%' is not found after edge '%' (% after current) for vehicle '%' at time=%.",
                            i, stop.lane.get_edge().get_id(), edges[start].get_id(), start as i32 - self.myCurrEdge as i32, self.get_id(),
                            utils::common::time::time2string(MSNet::get_instance().get_current_time_step())));
                    }
                    ok = false;
                }
                Some(it) => {
                    start = it;
                }
            }
            i += 1;
        }
        ok
    }

    /// C++: `std::vector<std::pair<int, double>> MSBaseVehicle::getStopIndices() const` [const, virtual]
    /// return list of route indices for the remaining stops
    // std::vector<std::pair<int, double> >
    // MSBaseVehicle::getStopIndices() const {
    //     std::vector<std::pair<int, double> > result;
    //     for (std::list<MSStop>::const_iterator iter = myStops.begin(); iter != myStops.end(); ++iter) {
    //         result.push_back(std::make_pair(
    //                              (int)(iter->edge - myRoute->begin()),
    //                              iter->getEndPos(*this)));
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_stop_indices(&self) -> Vec<(i32, f64)> {
        let mut result: Vec<(i32, f64)> = Vec::new();
        for stop in self.myStops.iter() {
            // shortcut: MSStop::edge is Unmapped, so the route index is found from the stop's lane edge
            let idx = self.myRoute.get_edges().iter().position(|e| *e == Ptr::new(stop.lane.get_edge())).unwrap_or(0) as i32;
            result.push((idx, stop.get_end_pos(&self.base)));
        }
        result
    }

    /// C++: `const std::list<MSStop> & MSBaseVehicle::getStops() const` [const, virtual]
    /// returns the list of stops not yet reached in the stop queue
    // inline const std::list<MSStop>& getStops() const {
    //         return myStops;
    //     }
    #[allow(unused_variables)]
    pub fn get_stops(&self) -> &VecDeque<MSStop> {
        &self.myStops
    }

    /// C++: `const StopParVector & MSBaseVehicle::getPastStops() const` [const]
    // inline const StopParVector& getPastStops() const {
    //         return myPastStops;
    //     }
    #[allow(unused_variables)]
    pub fn get_past_stops(&self) -> &StopParVector {
        &self.myPastStops
    }

    /// C++: `const MSStop & MSBaseVehicle::getNextStop() const` [const, virtual]
    /// returns the next imminent stop in the stop queue
    // const MSStop&
    // MSBaseVehicle::getNextStop() const {
    //     assert(myStops.size() > 0);
    //     return myStops.front();
    // }
    #[allow(unused_variables)]
    pub fn get_next_stop(&self) -> &MSStop {
        assert!(self.myStops.len() > 0);
        self.myStops.front().unwrap()
    }

    /// C++: `MSStop & MSBaseVehicle::getNextStopMutable()` [virtual]
    /// returns the next imminent stop in the stop queue
    // MSStop&
    // MSBaseVehicle::getNextStopMutable() {
    //     assert(myStops.size() > 0);
    //     return myStops.front();
    // }
    #[allow(unused_variables)]
    pub fn get_next_stop_mutable(&mut self) -> &mut MSStop {
        assert!(self.myStops.len() > 0);
        self.myStops.front_mut().unwrap()
    }

    /// C++: `SUMOTime MSBaseVehicle::getStopDuration() const` [const, virtual]
    /// get remaining stop duration or 0 if the vehicle isn't stopped
    // SUMOTime
    // MSBaseVehicle::getStopDuration() const {
    //     if (isStopped()) {
    //         return myStops.front().duration;
    //     } else {
    //         return 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_stop_duration(&self) -> SUMOTime {
        if self.is_stopped() {
            self.myStops.front().unwrap().duration
        } else {
            0
        }
    }

    /// C++: `MSStop & MSBaseVehicle::getStop(int)`
    /// returns the upcoming stop with the given index in the stop queue
    // MSStop&
    // MSBaseVehicle::getStop(int nextStopIndex) {
    //     if (nextStopIndex < 0 || (int)myStops.size() <= nextStopIndex) {
    //         throw InvalidArgument(TLF("Invalid stop index % (has % stops).", nextStopIndex, myStops.size()));
    //     }
    //     auto stopIt = myStops.begin();
    //     std::advance(stopIt, nextStopIndex);
    //     return *stopIt;
    // }
    #[allow(unused_variables)]
    pub fn get_stop(&mut self, nextStopIndex: i32) -> &mut MSStop {
        if nextStopIndex < 0 || self.myStops.len() as i32 <= nextStopIndex {
            panic!("{}", utils::tlf!("Invalid stop index % (has % stops).", nextStopIndex, self.myStops.len()));
        }
        &mut self.myStops[nextStopIndex as usize]
    }

    /// C++: `const SUMOVehicleParameter::Stop * MSBaseVehicle::getNextStopParameter() const` [const, virtual]
    /// return parameters for the next stop (SUMOVehicle Interface)
    // const SUMOVehicleParameter::Stop*
    // MSBaseVehicle::getNextStopParameter() const {
    //     if (hasStops()) {
    //         return &myStops.front().pars;
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_next_stop_parameter(&self) -> Ptr<SUMOVehicleParameter_Stop> {
        if self.has_stops() {
            Ptr::new(&self.myStops.front().unwrap().pars)
        } else {
            Ptr::default()
        }
    }

    /// C++: `bool MSBaseVehicle::addTraciStop(SUMOVehicleParameter::Stop, std::string &)` [virtual]
    /// schedule a new stop for the vehicle; each time a stop is reached, the vehicle will wait for the given duration before continuing on its route
    // bool
    // MSBaseVehicle::addTraciStop(SUMOVehicleParameter::Stop stop, std::string& errorMsg) {
    //     //if the stop exists update the duration
    //     for (std::list<MSStop>::iterator iter = myStops.begin(); iter != myStops.end(); iter++) {
    //         if (iter->lane->getID() == stop.lane && fabs(iter->pars.endPos - stop.endPos) < POSITION_EPS) {
    //             // update existing stop
    //             if (stop.duration == 0 && stop.until < 0 && !iter->reached) {
    //                 myStops.erase(iter);
    //             } else {
    //                 iter->duration = stop.duration;
    //                 iter->triggered = stop.triggered;
    //                 iter->containerTriggered = stop.containerTriggered;
    //                 const_cast<SUMOVehicleParameter::Stop&>(iter->pars).until = stop.until;
    //                 const_cast<SUMOVehicleParameter::Stop&>(iter->pars).parking = stop.parking;
    //             }
    //             return true;
    //         }
    //     }
    //     const bool result = addStop(stop, errorMsg);
    //     if (result) {
    //         /// XXX handle stops added out of order
    //         myParameter->stops.push_back(stop);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn add_traci_stop(&mut self, stop: SUMOVehicleParameter_Stop, errorMsg: &mut String) -> bool {
        // if the stop exists update the duration
        let mut i = 0;
        while i < self.myStops.len() {
            if self.myStops[i].lane.get_id() == stop.lane && (self.myStops[i].pars.endPos - stop.endPos).abs() < utils::geom::POSITION_EPS {
                // update existing stop
                if stop.duration == 0 && stop.until < 0 && !self.myStops[i].reached {
                    self.myStops.remove(i);
                } else {
                    let s = &mut self.myStops[i];
                    s.duration = stop.duration;
                    s.triggered = stop.triggered;
                    s.containerTriggered = stop.containerTriggered;
                    s.pars.until = stop.until;
                    s.pars.parking = stop.parking;
                }
                return true;
            }
            i += 1;
        }
        let result = self.add_stop(&stop, errorMsg, 0, Ptr::default());
        if result {
            // XXX handle stops added out of order
            self.myParameter.m().stops.push(stop);
        }
        result
    }

    /// C++: `bool MSBaseVehicle::resumeFromStopping()` [virtual, pure virtual]
    /// resumes a vehicle from stopping
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn resume_from_stopping(&mut self) -> bool {
        // pure virtual in C++: implemented by subclasses (MSVehicle)
        panic!("resumeFromStopping() is pure virtual in MSBaseVehicle")
    }

    /// C++: `void MSBaseVehicle::unregisterWaiting()` [virtual]
    /// mark vehicle as active
    // void
    // MSBaseVehicle::unregisterWaiting() {
    //     if (myAmRegisteredAsWaiting) {
    //         MSNet::getInstance()->getVehicleControl().unregisterOneWaiting();
    //         myAmRegisteredAsWaiting = false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn unregister_waiting(&mut self) {
        if self.myAmRegisteredAsWaiting {
            MSNet::get_instance().get_vehicle_control().unregister_one_waiting();
            self.myAmRegisteredAsWaiting = false;
        }
    }

    /// C++: `bool MSBaseVehicle::abortNextStop(int)` [virtual]
    /// deletes the next stop at the given index if it exists
    // bool
    // MSBaseVehicle::abortNextStop(int nextStopIndex) {
    //     if (hasStops() && nextStopIndex < (int)myStops.size()) {
    //         if (nextStopIndex == 0 && isStopped()) {
    //             resumeFromStopping();
    //         } else {
    //             auto stopIt = myStops.begin();
    //             std::advance(stopIt, nextStopIndex);
    //             if (stopIt->parkingarea != nullptr && stopIt->parkingarea->isReservable()) {
    //                 stopIt->parkingarea->removeSpaceReservation(this);
    //             }
    //             myStops.erase(stopIt);
    //         }
    //         if (!hasDeparted() && (int)myParameter->stops.size() > nextStopIndex) {
    //             // stops will be rebuilt from scratch on rerouting so we must patch the stops in myParameter
    //             auto stopIt2 = myParameter->stops.begin();
    //             std::advance(stopIt2, nextStopIndex);
    //             const_cast<SUMOVehicleParameter*>(myParameter)->stops.erase(stopIt2);
    //         }
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn abort_next_stop(&mut self, nextStopIndex: i32) -> bool {
        if self.has_stops() && (nextStopIndex as usize) < self.myStops.len() {
            if nextStopIndex == 0 && self.is_stopped() {
                // C++ resumeFromStopping() is virtual: dispatch to the MSVehicle / MEVehicle override
                if MSGlobals::use_meso_sim() {
                    Ptr::from_mut(self).cast::<crate::mesosim::me_vehicle::MEVehicle>().m().resume_from_stopping();
                } else {
                    Ptr::from_mut(self).cast::<crate::microsim::ms_vehicle::MSVehicle>().m().resume_from_stopping();
                }
            } else {
                let parkingarea = self.myStops[nextStopIndex as usize].parkingarea;
                if !parkingarea.is_null() && parkingarea.is_reservable() {
                    let me = Ptr::from_mut(&mut self.base);
                    parkingarea.m().remove_space_reservation(me);
                }
                self.myStops.remove(nextStopIndex as usize);
            }
            if !self.has_departed() && self.myParameter.stops.len() as i32 > nextStopIndex {
                // stops will be rebuilt from scratch on rerouting so we must patch the stops in myParameter
                self.myParameter.m().stops.remove(nextStopIndex as usize);
            }
            true
        } else {
            false
        }
    }

    /// C++: `bool MSBaseVehicle::replaceStop(int, SUMOVehicleParameter::Stop, const std::string &, bool, std::string &)`
    /// replace the next stop at the given index with the given stop parameters will wait for the given duration before continuing on its route The route between start other stops and destination will be kept unchanged and only the part around the replacement index will be adapted according to the new stop location
    // bool
    // MSBaseVehicle::replaceStop(int nextStopIndex, SUMOVehicleParameter::Stop stop, const std::string& info, bool teleport, std::string& errorMsg) {
    //     const int n = (int)myStops.size();
    //     if (nextStopIndex < 0 || nextStopIndex >= n) {
    //         errorMsg = TLF("invalid nextStopIndex % for % remaining stops", nextStopIndex, n);
    //         return false;
    //     }
    //     if (nextStopIndex == 0 && isStopped()) {
    //         errorMsg = TL("cannot replace reached stop");
    //         return false;
    //     }
    //     const SUMOTime t = MSNet::getInstance()->getCurrentTimeStep();
    //     MSLane* stopLane = MSLane::dictionary(stop.lane);
    //     MSEdge* stopEdge = &stopLane->getEdge();
    //
    //     auto itStop = myStops.begin();
    //     std::advance(itStop, nextStopIndex);
    //     MSStop& replacedStop = *itStop;
    //
    //     // check parking access rights
    //     if (stop.parkingarea != "") {
    //         MSParkingArea* pa = dynamic_cast<MSParkingArea*>(MSNet::getInstance()->getStoppingPlace(stop.parkingarea, SUMO_TAG_PARKING_AREA));
    //         if (pa != nullptr && !pa->accepts(this)) {
    //             errorMsg = TLF("vehicle '%' does not have the right badge to access parkingArea '%'", getID(), stop.parkingarea);
    //             return false;
    //         }
    //     }
    //
    //     if (replacedStop.lane == stopLane && replacedStop.pars.endPos == stop.endPos && !teleport) {
    //         // only replace stop attributes
    //         const_cast<SUMOVehicleParameter::Stop&>(replacedStop.pars) = stop;
    //         replacedStop.init_pars(stop);
    //         return true;
    //     }
    //
    //     if (!stopLane->allowsVehicleClass(getVClass(), myRoutingMode)) {
    //         errorMsg = TLF("disallowed stop lane '%'", stopLane->getID());
    //         return false;
    //     }
    //
    //     const ConstMSEdgeVector& oldEdges = getRoute().getEdges();
    //     std::vector<MSStop> stops(myStops.begin(), myStops.end());
    //     const int junctionOffset = getLane() != nullptr && getLane()->isInternal() ? 1 : 0;
    //     MSRouteIterator itStart = nextStopIndex == 0 ? getCurrentRouteEdge() + junctionOffset : stops[nextStopIndex - 1].edge;
    //     double startPos = nextStopIndex == 0 ? getPositionOnLane() : stops[nextStopIndex - 1].pars.endPos;
    //     MSRouteIterator itEnd = nextStopIndex == n - 1 ? oldEdges.end() - 1 : stops[nextStopIndex + 1].edge;
    //     auto endPos = nextStopIndex == n - 1 ? getArrivalPos() : stops[nextStopIndex + 1].pars.endPos;
    //     SUMOAbstractRouter<MSEdge, SUMOVehicle>& router = getRouterTT();
    //
    //     bool newDestination = nextStopIndex == n - 1 && stops[nextStopIndex].edge == oldEdges.end() - 1;
    //
    //     ConstMSEdgeVector toNewStop;
    //     if (!teleport) {
    //         router.compute(*itStart, startPos, stopEdge, stop.endPos, this, t, toNewStop, true);
    //         if (toNewStop.size() == 0) {
    //             errorMsg = TLF("no route found from edge '%' to stop edge '%'", (*itStart)->getID(), stopEdge->getID());
    //             return false;
    //         }
    //     }
    //
    //     ConstMSEdgeVector fromNewStop;
    //     if (!newDestination) {
    //         router.compute(stopEdge, stop.endPos, *itEnd, endPos, this, t, fromNewStop, true);
    //         if (fromNewStop.size() == 0) {
    //             errorMsg = TLF("no route found from stop edge '%' to edge '%'", stopEdge->getID(), (*itEnd)->getID());
    //             return false;
    //         }
    //     }
    //
    //     cleanupParkingReservation();
    //     const_cast<SUMOVehicleParameter::Stop&>(replacedStop.pars) = stop;
    //     replacedStop.init_pars(stop);
    //     replacedStop.edge = myRoute->end(); // will be patched in replaceRoute
    //     replacedStop.lane = stopLane;
    //     if (MSGlobals::gUseMesoSim) {
    //         replacedStop.segment = MSGlobals::gMesoNet->getSegmentForEdge(replacedStop.lane->getEdge(), replacedStop.get_end_pos(*this));
    //         if (replacedStop.lane->isInternal()) {
    //             errorMsg = TLF("Mesoscopic simulation does not allow stopping on internal edge '%' for vehicle '%'.", stop.edge, getID());
    //             return false;
    //         }
    //     }
    //
    //     ConstMSEdgeVector oldRemainingEdges(myCurrEdge, getRoute().end());
    //     ConstMSEdgeVector newEdges; // only remaining
    //     newEdges.insert(newEdges.end(), myCurrEdge, itStart);
    //     if (!teleport) {
    //         newEdges.insert(newEdges.end(), toNewStop.begin(), toNewStop.end() - 1);
    //     } else {
    //         newEdges.push_back(*itStart);
    //     }
    //     if (!newDestination) {
    //         newEdges.insert(newEdges.end(), fromNewStop.begin(), fromNewStop.end() - 1);
    //         newEdges.insert(newEdges.end(), itEnd, oldEdges.end());
    //     } else {
    //         newEdges.push_back(stopEdge);
    //     }
    //     //std::cout << SIMTIME << " replaceStop veh=" << getID()
    //     //    << " teleport=" << teleport
    //     //    << " busStop=" << stop.busstop
    //     //    << " oldEdges=" << oldRemainingEdges.size()
    //     //    << " newEdges=" << newEdges.size()
    //     //    << " toNewStop=" << toNewStop.size()
    //     //    << " fromNewStop=" << fromNewStop.size()
    //     //    << "\n";
    //
    //     const double routeCost = router.recomputeCosts(newEdges, this, t);
    //     const double previousCost = router.recomputeCosts(oldRemainingEdges, this, t);
    //     const double savings = previousCost - routeCost;
    //     if (!hasDeparted() && (int)myParameter->stops.size() > nextStopIndex) {
    //         // stops will be rebuilt from scratch so we must patch the stops in myParameter
    //         const_cast<SUMOVehicleParameter*>(myParameter)->stops[nextStopIndex] = stop;
    //     }
    //     if (teleport) {
    //         // let the vehicle jump rather than teleport
    //         // we add a jump-stop at the end of the edge (unless the vehicle is
    //         // already configure to jump before the replaced stop)
    //         if (!insertJump(nextStopIndex, itStart, errorMsg)) {
    //             return false;
    //         };
    //     }
    //     return replaceRouteEdges(newEdges, routeCost, savings, info, !hasDeparted(), false, false, &errorMsg);
    // }
    #[allow(unused_variables)]
    pub fn replace_stop(&mut self, nextStopIndex: i32, stop: SUMOVehicleParameter_Stop, info: &str, teleport: bool, errorMsg: &mut String) -> bool {
        // shortcut: route-iterator based stop replacement (MSRouteIterator is Unmapped) is not ported yet
        let n = self.myStops.len() as i32;
        if nextStopIndex < 0 || nextStopIndex >= n {
            *errorMsg = utils::tlf!("invalid nextStopIndex % for % remaining stops", nextStopIndex, n);
            return false;
        }
        *errorMsg = "replaceStop is not ported yet".to_string();
        let _ = (stop, info, teleport);
        false
    }

    /// C++: `bool MSBaseVehicle::rerouteBetweenStops(int, const std::string &, bool, std::string &)`
    /// reroute between stops nextStopIndex - 1 and nextStopIndex (defaults to current position / final edge) if the respective stops do not exist
    // bool
    // MSBaseVehicle::rerouteBetweenStops(int nextStopIndex, const std::string& info, bool teleport, std::string& errorMsg) {
    //     const int n = (int)myStops.size();
    //     if (nextStopIndex < 0 || nextStopIndex > n) {
    //         errorMsg = TLF("invalid nextStopIndex % for % remaining stops", nextStopIndex, n);
    //         return false;
    //     }
    //     if (nextStopIndex == 0 && isStopped()) {
    //         errorMsg = TL("cannot reroute towards reached stop");
    //         return false;
    //     }
    //     const SUMOTime t = MSNet::getInstance()->getCurrentTimeStep();
    //
    //     const ConstMSEdgeVector& oldEdges = getRoute().getEdges();
    //     std::vector<MSStop> stops(myStops.begin(), myStops.end());
    //     const int junctionOffset = getLane() != nullptr && getLane()->isInternal() ? 1 : 0;
    //     MSRouteIterator itStart = nextStopIndex == 0 ? getCurrentRouteEdge() + junctionOffset : stops[nextStopIndex - 1].edge;
    //     double startPos = nextStopIndex == 0 ? getPositionOnLane() : stops[nextStopIndex - 1].pars.endPos;
    //     MSRouteIterator itEnd = nextStopIndex == n ? oldEdges.end() - 1 : stops[nextStopIndex].edge;
    //     auto endPos = nextStopIndex == n ? getArrivalPos() : stops[nextStopIndex].pars.endPos;
    //     SUMOAbstractRouter<MSEdge, SUMOVehicle>& router = getRouterTT();
    //
    //     ConstMSEdgeVector newBetween;
    //     if (!teleport) {
    //         router.compute(*itStart, startPos, *itEnd, endPos, this, t, newBetween, true);
    //         if (newBetween.size() == 0) {
    //             errorMsg = TLF("no route found from edge '%' to stop edge '%'", (*itStart)->getID(), (*itEnd)->getID());
    //             return false;
    //         }
    //     }
    //
    //     ConstMSEdgeVector oldRemainingEdges(myCurrEdge, getRoute().end());
    //     ConstMSEdgeVector newEdges; // only remaining
    //     newEdges.insert(newEdges.end(), myCurrEdge, itStart);
    //     if (!teleport) {
    //         newEdges.insert(newEdges.end(), newBetween.begin(), newBetween.end() - 1);
    //     } else {
    //         newEdges.push_back(*itStart);
    //     }
    //     newEdges.insert(newEdges.end(), itEnd, oldEdges.end());
    //     //std::cout << SIMTIME << " rerouteBetweenStops veh=" << getID()
    //     //    << " oldEdges=" << oldRemainingEdges.size()
    //     //    << " newEdges=" << newEdges.size()
    //     //    << " toNewStop=" << toNewStop.size()
    //     //    << " fromNewStop=" << fromNewStop.size()
    //     //    << "\n";
    //
    //     const double routeCost = router.recomputeCosts(newEdges, this, t);
    //     const double previousCost = router.recomputeCosts(oldRemainingEdges, this, t);
    //     const double savings = previousCost - routeCost;
    //
    //     if (teleport) {
    //         // let the vehicle jump rather than teleport
    //         // we add a jump-stop at the end of the edge (unless the vehicle is
    //         // already configure to jump before the replaced stop)
    //         if (!insertJump(nextStopIndex, itStart, errorMsg)) {
    //             return false;
    //         };
    //     }
    //     return replaceRouteEdges(newEdges, routeCost, savings, info, !hasDeparted(), false, false, &errorMsg);
    // }
    #[allow(unused_variables)]
    pub fn reroute_between_stops(&mut self, nextStopIndex: i32, info: &str, teleport: bool, errorMsg: &mut String) -> bool {
        // shortcut: route-iterator based rerouting between stops (MSRouteIterator is Unmapped) is not ported yet
        let n = self.myStops.len() as i32;
        if nextStopIndex < 0 || nextStopIndex > n {
            *errorMsg = utils::tlf!("invalid nextStopIndex % for % remaining stops", nextStopIndex, n);
            return false;
        }
        if nextStopIndex == 0 && self.is_stopped() {
            *errorMsg = "cannot reroute towards reached stop".to_string();
            return false;
        }
        *errorMsg = "rerouteBetweenStops is not ported yet".to_string();
        let _ = (info, teleport);
        false
    }

    /// C++: `bool MSBaseVehicle::insertStop(int, SUMOVehicleParameter::Stop, const std::string &, bool, std::string &)`
    /// insert stop at the given index with the given stop parameters will wait for the given duration before continuing on its route The route will be adapted to pass the new stop edge but only from the previous stop (or start) to the new stop and only up to the next stop (or end).
    // bool
    // MSBaseVehicle::insertStop(int nextStopIndex, SUMOVehicleParameter::Stop stop, const std::string& info, bool teleport, std::string& errorMsg) {
    //     const int n = (int)myStops.size();
    //     if (nextStopIndex < 0 || nextStopIndex > n) {
    //         errorMsg = TLF("invalid nextStopIndex % for % remaining stops", nextStopIndex, n);
    //         return false;
    //     }
    //     if (nextStopIndex == 0 && isStopped()) {
    //         errorMsg = TL("cannot insert stop before the currently reached stop");
    //         return false;
    //     }
    //     const SUMOTime t = MSNet::getInstance()->getCurrentTimeStep();
    //     MSLane* stopLane = MSLane::dictionary(stop.lane);
    //     MSEdge* stopEdge = &stopLane->getEdge();
    //
    //     if (!stopLane->allowsVehicleClass(getVClass(), myRoutingMode)) {
    //         errorMsg = TLF("disallowed stop lane '%'", stopLane->getID());
    //         return false;
    //     }
    //
    //     // check parking access rights
    //     if (stop.parkingarea != "") {
    //         MSParkingArea* pa = dynamic_cast<MSParkingArea*>(MSNet::getInstance()->getStoppingPlace(stop.parkingarea, SUMO_TAG_PARKING_AREA));
    //         if (pa != nullptr && !pa->accepts(this)) {
    //             errorMsg = TLF("Vehicle '%' does not have the right badge to access parkingArea '%'.", getID(), stop.parkingarea);
    //             return false;
    //         }
    //     }
    //
    //     const ConstMSEdgeVector& oldEdges = getRoute().getEdges();
    //     std::vector<MSStop> stops(myStops.begin(), myStops.end());
    //     const int junctionOffset = ((getLane() != nullptr && getLane()->isInternal())
    //                                 || getRerouteOrigin() != getCurrentRouteEdge()) ? 1 : 0;
    //     MSRouteIterator itStart = nextStopIndex == 0 ? getCurrentRouteEdge() + junctionOffset : stops[nextStopIndex - 1].edge;
    //     double startPos = nextStopIndex == 0 ? getPositionOnLane() : stops[nextStopIndex - 1].pars.endPos;
    //     MSRouteIterator itEnd = nextStopIndex == n ? oldEdges.end() - 1 : stops[nextStopIndex].edge;
    //     auto endPos = nextStopIndex == n ? getArrivalPos() : stops[nextStopIndex].pars.endPos;
    //     SUMOAbstractRouter<MSEdge, SUMOVehicle>& router = getRouterTT();
    //
    //     const bool newDestination = nextStopIndex == n && stopEdge == oldEdges.back();
    //     const bool needLoop = nextStopIndex > 0 && *itStart == stopEdge && startPos >= stop.endPos;
    //     if (needLoop && startPos == stop.endPos) {
    //         // assume that two stops at the same position indicate a looped route
    //         startPos += POSITION_EPS;
    //         stop.index = STOP_INDEX_REPEAT;
    //     } else if (!hasDeparted()) {
    //         stop.index = STOP_INDEX_END;
    //     }
    //
    //     MSRouteIterator itSentintel = itEnd + 1;
    //     MSRouteIterator onRouteStart = itStart + (needLoop ? 1 : 0);
    //     const bool onRoute = std::find(onRouteStart, itSentintel, stopEdge) != itSentintel;
    //     if (onRoute) {
    //         if (!hasDeparted() && (int)myParameter->stops.size() >= nextStopIndex) {
    //             // stops will be rebuilt from scratch so we must patch the stops in myParameter
    //             auto it = myParameter->stops.begin() + nextStopIndex;
    //             const_cast<SUMOVehicleParameter*>(myParameter)->stops.insert(it, stop);
    //         }
    //         const bool ok = addStop(stop, errorMsg, 0, &onRouteStart);
    //         if (ok) {
    //             updateBestLanes(true, !hasDeparted() ? (*myCurrEdge)->getLanes().front() : 0);
    //         }
    //         return ok;
    //     }
    //
    //     ConstMSEdgeVector toNewStop;
    //     if (!teleport) {
    //         router.compute(*itStart, startPos, stopEdge, stop.endPos, this, t, toNewStop, true);
    //         if (toNewStop.size() == 0) {
    //             errorMsg = TLF("no route found from edge '%' to stop edge '%'", (*itStart)->getID(), stopEdge->getID());
    //             return false;
    //         }
    //     }
    //
    //     ConstMSEdgeVector fromNewStop;
    //     if (!newDestination) {
    //         router.compute(stopEdge, stop.endPos, *itEnd, endPos, this, t, fromNewStop, true);
    //         if (fromNewStop.size() == 0) {
    //             errorMsg = TLF("no route found from stop edge '%' to edge '%'", stopEdge->getID(), (*itEnd)->getID());
    //             return false;
    //         }
    //     }
    //
    //     auto itStop = myStops.begin();
    //     std::advance(itStop, nextStopIndex);
    //     MSStop newStop(stop);
    //     newStop.init_pars(stop);
    //     newStop.edge = myRoute->end(); // will be patched in replaceRoute
    //     newStop.lane = stopLane;
    //     if (MSGlobals::gUseMesoSim) {
    //         newStop.segment = MSGlobals::gMesoNet->getSegmentForEdge(newStop.lane->getEdge(), newStop.get_end_pos(*this));
    //         if (newStop.lane->isInternal()) {
    //             errorMsg = TLF("Mesoscopic simulation does not allow stopping on internal edge '%' for vehicle '%'.", stop.edge, getID());
    //             return false;
    //         }
    //     }
    //     myStops.insert(itStop, newStop);
    //
    //     ConstMSEdgeVector oldRemainingEdges(myCurrEdge, getRoute().end());
    //     ConstMSEdgeVector newEdges; // only remaining
    //     newEdges.insert(newEdges.end(), myCurrEdge, itStart);
    //     if (!teleport) {
    //         newEdges.insert(newEdges.end(), toNewStop.begin(), toNewStop.end() - 1);
    //     } else {
    //         newEdges.push_back(*itStart);
    //     }
    //     if (!newDestination) {
    //         newEdges.insert(newEdges.end(), fromNewStop.begin(), fromNewStop.end() - 1);
    //         newEdges.insert(newEdges.end(), itEnd, oldEdges.end());
    //     } else {
    //         newEdges.push_back(stopEdge);
    //     }
    //     //std::cout << SIMTIME << " insertStop veh=" << getID()
    //     //    << " teleport=" << teleport
    //     //    << " busStop=" << stop.busstop
    //     //    << " oldEdges=" << oldRemainingEdges.size()
    //     //    << " newEdges=" << newEdges.size()
    //     //    << " toNewStop=" << toNewStop.size()
    //     //    << " fromNewStop=" << fromNewStop.size()
    //     //    << "\n";
    //
    //     const double routeCost = router.recomputeCosts(newEdges, this, t);
    //     const double previousCost = router.recomputeCosts(oldRemainingEdges, this, t);
    //     const double savings = previousCost - routeCost;
    //
    //     if (!hasDeparted() && (int)myParameter->stops.size() >= nextStopIndex) {
    //         // stops will be rebuilt from scratch so we must patch the stops in myParameter
    //         auto it = myParameter->stops.begin() + nextStopIndex;
    //         const_cast<SUMOVehicleParameter*>(myParameter)->stops.insert(it, stop);
    //     }
    //     return replaceRouteEdges(newEdges, routeCost, savings, info, !hasDeparted(), false, false, &errorMsg);
    // }
    #[allow(unused_variables)]
    pub fn insert_stop(&mut self, nextStopIndex: i32, stop: SUMOVehicleParameter_Stop, info: &str, teleport: bool, errorMsg: &mut String) -> bool {
        // shortcut: insertStop relies on route-iterator based rerouting (MSRouteIterator is Unmapped), not ported yet
        *errorMsg = "insertStop is not ported yet".to_string();
        let _ = (nextStopIndex, stop, info, teleport);
        false
    }

    /// C++: `bool MSBaseVehicle::isSelected() const` [const, virtual]
    /// whether this vehicle is selected in the GUI
    // virtual bool isSelected() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_selected(&self) -> bool {
        false
    }

    /// C++: `void MSBaseVehicle::updateBestLanes(bool, const MSLane *)` [virtual]
    // virtual void updateBestLanes(bool forceRebuild = false, const MSLane* startLane = 0) {
    //         UNUSED_PARAMETER(forceRebuild);
    //         UNUSED_PARAMETER(startLane);
    //     }
    #[allow(unused_variables)]
    pub fn update_best_lanes(&mut self, forceRebuild: bool, startLane: Ptr<MSLane>) {
        // no-op in the base class; MSVehicle overrides it
        let _ = (forceRebuild, startLane);
    }

    /// C++: `int MSBaseVehicle::getRNGIndex() const` [const, virtual]
    /// Returns The index of the vehicle's associated RNG
    // int
    // MSBaseVehicle::getRNGIndex() const {
    //     const MSLane* const lane = getLane();
    //     if (lane == nullptr) {
    //         return getEdge()->getLanes()[0]->getRNGIndex();
    //     } else {
    //         return lane->getRNGIndex();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_rng_index(&self) -> i32 {
        let lane = self.get_lane();
        if lane.is_null() {
            self.get_edge().get_lanes()[0].get_rng_index()
        } else {
            lane.get_rng_index()
        }
    }

    /// C++: `SumoRNG * MSBaseVehicle::getRNG() const` [const, virtual]
    /// Returns The vehicle's associated RNG
    // SumoRNG*
    // MSBaseVehicle::getRNG() const {
    //     const MSLane* lane = getLane();
    //     if (lane == nullptr) {
    //         return getEdge()->getLanes()[0]->getRNG();
    //     } else {
    //         return lane->getRNG();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_rng(&self) -> Ptr<SumoRNG> {
        let lane = self.get_lane();
        if lane.is_null() {
            self.get_edge().get_lanes()[0].get_rng()
        } else {
            lane.get_rng()
        }
    }

    /// C++: `NumericalID MSBaseVehicle::getNumericalID() const` [const, virtual]
    // inline NumericalID getNumericalID() const {
    //         return myNumericalID;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> SUMOTrafficObject_NumericalID {
        self.myNumericalID
    }

    /// C++: `long long MSBaseVehicle::getRandomSeed() const` [const, virtual]
    /// return vehicle-specific random number
    // long long int getRandomSeed() const {
    //         return myRandomSeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_random_seed(&self) -> i64 {
        self.myRandomSeed
    }

    /// C++: `const MSDevice_Transportable * MSBaseVehicle::getPersonDevice() const` [const]
    // const MSDevice_Transportable* getPersonDevice() const {
    //         return myPersonDevice;
    //     }
    #[allow(unused_variables)]
    pub fn get_person_device(&self) -> Ptr<MSDevice_Transportable> {
        self.myPersonDevice
    }

    /// C++: `const MSDevice_Transportable * MSBaseVehicle::getContainerDevice() const` [const]
    // const MSDevice_Transportable* getContainerDevice() const {
    //         return myContainerDevice;
    //     }
    #[allow(unused_variables)]
    pub fn get_container_device(&self) -> Ptr<MSDevice_Transportable> {
        self.myContainerDevice
    }

    /// C++: `EnergyParams * MSBaseVehicle::getEmissionParameters() const` [const, virtual]
    /// retrieve parameters for the energy consumption model
    // EnergyParams*
    // MSBaseVehicle::getEmissionParameters() const {
    //     if (myEnergyParams == nullptr) {
    //         myEnergyParams = new EnergyParams(getVehicleType().get_emission_parameters());
    //         double tMass = 0;
    //         if (myPersonDevice != nullptr) {
    //             for (MSTransportable* t : myPersonDevice->getTransportables()) {
    //                 tMass += t->getVehicleType().getMass();
    //             }
    //         }
    //         if (myContainerDevice != nullptr) {
    //             for (MSTransportable* t : myContainerDevice->getTransportables()) {
    //                 tMass += t->getVehicleType().getMass();
    //             }
    //         }
    //         myEnergyParams->setTransportableMass(tMass);
    //     }
    //     return myEnergyParams;
    // }
    #[allow(unused_variables)]
    pub fn get_emission_parameters(&mut self) -> Ptr<EnergyParams> {
        if self.myEnergyParams.is_null() {
            let mut params = EnergyParams::new_2(self.get_vehicle_type().get_emission_parameters());
            let mut tMass = 0.0;
            if !self.myPersonDevice.is_null() {
                for t in self.myPersonDevice.get_transportables().iter() {
                    tMass += t.get_vehicle_type().get_mass();
                }
            }
            if !self.myContainerDevice.is_null() {
                for t in self.myContainerDevice.get_transportables().iter() {
                    tMass += t.get_vehicle_type().get_mass();
                }
            }
            params.set_transportable_mass(tMass);
            self.myEnergyParams = Ptr::from_mut(Box::leak(Box::new(params)));
        }
        self.myEnergyParams
    }

    /// C++: `double MSBaseVehicle::getEmissions() const` [const]
    /// Returns emissions of the current state The value is always per 1s, so multiply by step length if necessary.
    // template<PollutantsInterface::EmissionType ET>
    //     double getEmissions() const {
    //         if (isOnRoad() || isIdling()) {
    //             return PollutantsInterface::compute(myType->getEmissionClass(), ET, getSpeed(), getAcceleration(), getSlope(), getEmissionParameters());
    //         }
    //         return 0.;
    //     }
    #[allow(unused_variables)]
    pub fn get_emissions(&mut self, et: PollutantsInterface_EmissionType) -> f64 {
        if self.is_on_road() || self.is_idling() {
            // virtual calls in C++: read through SUMOTrafficObject so MSVehicle's acceleration is used, not the base 0
            let obj: &SUMOTrafficObject = self;
            PollutantsInterface::compute(obj.get_vehicle_type().get_emission_class(), et, obj.get_speed(), obj.get_acceleration(), obj.get_slope(), self.get_emission_parameters())
        } else {
            0.
        }
    }

    /// C++: `double MSBaseVehicle::getStateOfCharge() const` [const]
    /// Returns actual state of charge of battery (Wh) RICE_CHECK: This may be a misnomer, SOC is typically percentage of the maximum battery capacity.
    // double
    // MSBaseVehicle::getStateOfCharge() const {
    //     if (static_cast<MSDevice_Battery*>(getDevice(typeid(MSDevice_Battery))) != 0) {
    //         MSDevice_Battery* batteryOfVehicle = dynamic_cast<MSDevice_Battery*>(getDevice(typeid(MSDevice_Battery)));
    //         return batteryOfVehicle->getActualBatteryCapacity();
    //     } else {
    //         if (static_cast<MSDevice_ElecHybrid*>(getDevice(typeid(MSDevice_ElecHybrid))) != 0) {
    //             MSDevice_ElecHybrid* batteryOfVehicle = dynamic_cast<MSDevice_ElecHybrid*>(getDevice(typeid(MSDevice_ElecHybrid)));
    //             return batteryOfVehicle->getActualBatteryCapacity();
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_state_of_charge(&self) -> f64 {
        // shortcut: battery / elec-hybrid device lookup by typeid is not ported; device-less vehicles return -1
        -1.0
    }

    /// C++: `double MSBaseVehicle::getRelativeStateOfCharge() const` [const]
    /// Returns actual relative state of charge of battery (-)
    // double
    // MSBaseVehicle::getRelativeStateOfCharge() const {
    //     if (static_cast<MSDevice_Battery*>(getDevice(typeid(MSDevice_Battery))) != 0) {
    //         MSDevice_Battery* batteryOfVehicle = dynamic_cast<MSDevice_Battery*>(getDevice(typeid(MSDevice_Battery)));
    //         return batteryOfVehicle->getActualBatteryCapacity() / batteryOfVehicle->getMaximumBatteryCapacity();
    //     } else {
    //         if (static_cast<MSDevice_ElecHybrid*>(getDevice(typeid(MSDevice_ElecHybrid))) != 0) {
    //             MSDevice_ElecHybrid* batteryOfVehicle = dynamic_cast<MSDevice_ElecHybrid*>(getDevice(typeid(MSDevice_ElecHybrid)));
    //             return batteryOfVehicle->getActualBatteryCapacity() / batteryOfVehicle->getMaximumBatteryCapacity();
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_relative_state_of_charge(&self) -> f64 {
        // shortcut: battery device lookup by typeid is not ported; device-less vehicles return -1
        -1.0
    }

    /// C++: `double MSBaseVehicle::getChargedEnergy() const` [const]
    /// Returns the energy charged to the battery in the current time step (Wh)
    // double
    // MSBaseVehicle::getChargedEnergy() const {
    //     if (static_cast<MSDevice_Battery*>(getDevice(typeid(MSDevice_Battery))) != 0) {
    //         MSDevice_Battery* batteryOfVehicle = dynamic_cast<MSDevice_Battery*>(getDevice(typeid(MSDevice_Battery)));
    //         return batteryOfVehicle->getEnergyCharged();
    //     } else {
    //         if (static_cast<MSDevice_ElecHybrid*>(getDevice(typeid(MSDevice_ElecHybrid))) != 0) {
    //             MSDevice_ElecHybrid* batteryOfVehicle = dynamic_cast<MSDevice_ElecHybrid*>(getDevice(typeid(MSDevice_ElecHybrid)));
    //             return batteryOfVehicle->getEnergyCharged();
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_charged_energy(&self) -> f64 {
        // shortcut: battery device lookup by typeid is not ported; device-less vehicles return -1
        -1.0
    }

    /// C++: `double MSBaseVehicle::getMaxChargeRate() const` [const]
    /// Returns the maximum charge rate allowed by the battery in the current time step (W)
    // double
    // MSBaseVehicle::getMaxChargeRate() const {
    //     if (static_cast<MSDevice_Battery*>(getDevice(typeid(MSDevice_Battery))) != 0) {
    //         MSDevice_Battery* batteryOfVehicle = dynamic_cast<MSDevice_Battery*>(getDevice(typeid(MSDevice_Battery)));
    //         return batteryOfVehicle->getMaximumChargeRate();
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_max_charge_rate(&self) -> f64 {
        // shortcut: battery device lookup by typeid is not ported; device-less vehicles return -1
        -1.0
    }

    /// C++: `double MSBaseVehicle::getElecHybridCurrent() const` [const]
    /// Returns actual current (A) of ElecHybrid device RICE_CHECK: Is this the current consumed from the overhead wire or the current driving the powertrain of the vehicle? RICE_REV_JS: It is the current drawn from the overhead wire (value if the vehicle is not connected to overhead wire?)
    // double
    // MSBaseVehicle::getElecHybridCurrent() const {
    //     if (static_cast<MSDevice_ElecHybrid*>(getDevice(typeid(MSDevice_ElecHybrid))) != 0) {
    //         MSDevice_ElecHybrid* elecHybridDevice = dynamic_cast<MSDevice_ElecHybrid*>(getDevice(typeid(MSDevice_ElecHybrid)));
    //         return elecHybridDevice->getCurrentFromOverheadWire();
    //     }
    //
    //     return NAN;
    // }
    #[allow(unused_variables)]
    pub fn get_elec_hybrid_current(&self) -> f64 {
        // shortcut: elec-hybrid device lookup by typeid is not ported; device-less vehicles return 0
        0.0
    }

    /// C++: `double MSBaseVehicle::getHarmonoise_NoiseEmissions() const` [const]
    /// Returns noise emissions of the current state
    // double
    // MSBaseVehicle::getHarmonoise_NoiseEmissions() const {
    //     if (isOnRoad() || isIdling()) {
    //         return HelpersHarmonoise::computeNoise(myType->getEmissionClass(), getSpeed(), getAcceleration());
    //     } else {
    //         return 0.;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_harmonoise_noise_emissions(&self) -> f64 {
        if self.is_on_road() || self.is_idling() {
            // virtual getAcceleration(): the MSBaseVehicle stub returns 0, so go through the SUMOVehicle vtable
            crate::utils::emissions::helpers_harmonoise::HelpersHarmonoise::compute_noise(self.get_vehicle_type().get_emission_class(), self.get_speed(), self.base.get_acceleration())
        } else {
            0.
        }
    }

    /// C++: `BaseInfluencer & MSBaseVehicle::getBaseInfluencer()` [virtual, pure virtual]
    /// Returns the velocity/lane influencer
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_base_influencer(&mut self) -> &mut MSBaseVehicle_BaseInfluencer {
        panic!("getBaseInfluencer() is pure virtual in MSBaseVehicle")
    }

    /// C++: `const BaseInfluencer * MSBaseVehicle::getBaseInfluencer() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_base_influencer_2(&self) -> Ptr<MSBaseVehicle_BaseInfluencer> {
        panic!("getBaseInfluencer() is pure virtual in MSBaseVehicle")
    }

    /// C++: `bool MSBaseVehicle::hasInfluencer() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn has_influencer(&self) -> bool {
        false // shortcut: no myInfluencer field yet, so nothing can be attached; C++ returns myInfluencer != nullptr
    }

    /// C++: `int MSBaseVehicle::getRoutingMode() const` [const, virtual]
    /// return routing mode (configures router choice but also handling of transient permission changes)
    // int getRoutingMode() const {
    //         return myRoutingMode;
    //     }
    #[allow(unused_variables)]
    pub fn get_routing_mode(&self) -> i32 {
        self.myRoutingMode
    }

    /// C++: `void MSBaseVehicle::setRoutingMode(int)` [virtual]
    /// Sets routing behavior
    // void setRoutingMode(int value) {
    //         myRoutingMode = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_routing_mode(&mut self, value: i32) {
        self.myRoutingMode = value;
    }

    /// C++: `SUMOAbstractRouter<MSEdge, SUMOVehicle> & MSBaseVehicle::getRouterTT() const` [const, virtual]
    /// returns the router according to the current routing mode of the vehicle
    // SUMOAbstractRouter<MSEdge, SUMOVehicle>&
    // MSBaseVehicle::getRouterTT() const {
    //     if (myRoutingMode == libsumo::ROUTING_MODE_AGGREGATED) {
    //         return MSRoutingEngine::getRouterTT(getRNGIndex(), getVClass());
    //     } else {
    //         return MSNet::getInstance()->getRouterTT(getRNGIndex());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_router_tt(&self) -> &mut SUMOAbstractRouter<MSEdge, SUMOVehicle> {
        if self.myRoutingMode == utils::traci::traci_constants::ROUTING_MODE_AGGREGATED {
            MSRoutingEngine::get_router_tt(self.get_rng_index(), self.get_v_class(), &Default::default())
        } else {
            MSNet::get_instance().get_mut().unwrap().get_router_tt(self.get_rng_index(), &Default::default())
        }
    }

    /// C++: `const MSEdgeWeightsStorage & MSBaseVehicle::getWeightsStorage() const` [const]
    /// Returns the vehicle's internal edge travel times/efforts container
    // const MSEdgeWeightsStorage&
    // MSBaseVehicle::getWeightsStorage() const {
    //     return _getWeightsStorage();
    // }
    #[allow(unused_variables)]
    pub fn get_weights_storage(&self) -> &MSEdgeWeightsStorage {
        self.get_weights_storage_3()
    }

    /// C++: `MSEdgeWeightsStorage & MSBaseVehicle::getWeightsStorage()`
    // MSEdgeWeightsStorage&
    // MSBaseVehicle::getWeightsStorage() {
    //     return _getWeightsStorage();
    // }
    #[allow(unused_variables)]
    pub fn get_weights_storage_2(&mut self) -> &mut MSEdgeWeightsStorage {
        self.get_weights_storage_3()
    }

    /// C++: `std::pair<const MSVehicle *const, double> MSBaseVehicle::getLeader(double, bool) const` [const, virtual]
    /// Returns the leader of the vehicle looking for a fixed distance.
    // virtual std::pair<const MSVehicle* const, double> getLeader(double dist = 0, bool considerCrossingFoes = true) const {
    //         UNUSED_PARAMETER(dist);
    //         UNUSED_PARAMETER(considerCrossingFoes);
    //         WRITE_WARNING(TL("getLeader not yet implemented for meso"));
    //         return std::make_pair(nullptr, -1);
    //     }
    #[allow(unused_variables)]
    pub fn get_leader(&self, dist: f64, considerCrossingFoes: bool) -> (Ptr<MSVehicle>, f64) {
        // MSVehicle overrides this; the base (mesoscopic) version warns and reports no leader
        utils::common::msg_handler::write_warning("getLeader not yet implemented for mesoscopic vehicles");
        (Ptr::default(), -1.0)
    }

    /// C++: `std::pair<const MSVehicle *const, double> MSBaseVehicle::getFollower(double) const` [const, virtual]
    /// Returns the follower of the vehicle looking for a fixed distance.
    // virtual std::pair<const MSVehicle* const, double> getFollower(double dist = 0) const {
    //         UNUSED_PARAMETER(dist);
    //         WRITE_WARNING(TL("getFollower not yet implemented for meso"));
    //         return std::make_pair(nullptr, -1);
    //     }
    #[allow(unused_variables)]
    pub fn get_follower(&self, dist: f64) -> (Ptr<MSVehicle>, f64) {
        // MSVehicle overrides this; the base (mesoscopic) version warns and reports no follower
        utils::common::msg_handler::write_warning("getFollower not yet implemented for meso");
        (Ptr::default(), -1.0)
    }

    /// C++: `void MSBaseVehicle::calculateArrivalParams(bool)`
    /// (Re-)Calculates the arrival position and lane from the vehicle parameters
    // void
    // MSBaseVehicle::calculateArrivalParams(bool onInit) {
    //     if (myRoute->getLastEdge()->isTazConnector()) {
    //         return;
    //     }
    //     const int arrivalEdgeIndex = MIN2(myParameter->arrivalEdge, (int)myRoute->getEdges().size() - 1);
    //     if (arrivalEdgeIndex != myParameter->arrivalEdge) {
    //         if (!(onInit && myParameter->wasSet(VEHPARS_FORCE_REROUTE))) {
    //             WRITE_WARNINGF(TL("Vehicle '%' ignores attribute arrivalEdge=% after rerouting at time=% (routeLength=%)"),
    //                            getID(), myParameter->arrivalEdge, time2string(SIMSTEP), myRoute->getEdges().size() - 1);
    //         }
    //     }
    //     const MSEdge* arrivalEdge = myParameter->arrivalEdge >= 0 ? myRoute->getEdges()[arrivalEdgeIndex] : myRoute->getLastEdge();
    //     if (!onInit) {
    //         arrivalEdge = myRoute->getLastEdge();
    //         // ignore arrivalEdge parameter after rerouting
    //         const_cast<SUMOVehicleParameter*>(myParameter)->arrivalEdge = -1;
    //     }
    //     const std::vector<MSLane*>& lanes = arrivalEdge->getLanes();
    //     const double lastLaneLength = lanes[0]->getLength();
    //     switch (myParameter->arrivalPosProcedure) {
    //         case ArrivalPosDefinition::GIVEN:
    //             if (fabs(myParameter->arrivalPos) > lastLaneLength) {
    //                 WRITE_WARNINGF(TL("Vehicle '%' will not be able to arrive at the given position!"), getID());
    //             }
    //             // Maybe we should warn the user about invalid inputs!
    //             myArrivalPos = MIN2(myParameter->arrivalPos, lastLaneLength);
    //             if (myArrivalPos < 0) {
    //                 myArrivalPos = MAX2(myArrivalPos + lastLaneLength, 0.);
    //             }
    //             break;
    //         case ArrivalPosDefinition::RANDOM:
    //             myArrivalPos = RandHelper::rand(lastLaneLength);
    //             break;
    //         case ArrivalPosDefinition::CENTER:
    //             myArrivalPos = lastLaneLength / 2.;
    //             break;
    //         default:
    //             myArrivalPos = lastLaneLength;
    //             break;
    //     }
    //     if (myParameter->arrivalLaneProcedure == ArrivalLaneDefinition::GIVEN) {
    //         if (myParameter->arrivalLane >= (int)lanes.size() || !lanes[myParameter->arrivalLane]->allowsVehicleClass(myType->getVehicleClass())) {
    //             WRITE_WARNINGF(TL("Vehicle '%' will not be able to arrive at the given lane '%_%'!"), getID(), arrivalEdge->getID(), toString(myParameter->arrivalLane));
    //         }
    //         myArrivalLane = MIN2(myParameter->arrivalLane, (int)(lanes.size() - 1));
    //     } else if (myParameter->arrivalLaneProcedure == ArrivalLaneDefinition::FIRST_ALLOWED) {
    //         myArrivalLane = -1;
    //         for (MSLane* lane : lanes) {
    //             if (lane->allowsVehicleClass(myType->getVehicleClass())) {
    //                 myArrivalLane = lane->getIndex();
    //                 break;
    //             }
    //         }
    //         if (myArrivalLane == -1) {
    //             WRITE_WARNINGF(TL("Vehicle '%' has no usable arrivalLane on edge '%'."), getID(), arrivalEdge->getID());
    //             myArrivalLane = 0;
    //         }
    //     } else if (myParameter->arrivalLaneProcedure == ArrivalLaneDefinition::RANDOM) {
    //         // pick random lane among all usable lanes
    //         std::vector<MSLane*> usable;
    //         for (MSLane* lane : lanes) {
    //             if (lane->allowsVehicleClass(myType->getVehicleClass())) {
    //                 usable.push_back(lane);
    //             }
    //         }
    //         if (usable.empty()) {
    //             WRITE_WARNINGF(TL("Vehicle '%' has no usable arrivalLane on edge '%'."), getID(), arrivalEdge->getID());
    //             myArrivalLane = 0;
    //         } else {
    //             myArrivalLane = usable[RandHelper::rand(0, (int)usable.size())]->getIndex();
    //         }
    //     }
    //     if (myParameter->arrivalSpeedProcedure == ArrivalSpeedDefinition::GIVEN) {
    //         for (std::vector<MSLane*>::const_iterator l = lanes.begin(); l != lanes.end(); ++l) {
    //             if (myParameter->arrivalSpeed <= (*l)->getVehicleMaxSpeed(this)) {
    //                 return;
    //             }
    //         }
    //         WRITE_WARNINGF(TL("Vehicle '%' will not be able to arrive with the given speed!"), getID());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn calculate_arrival_params(&mut self, onInit: bool) {
        if self.myRoute.get_last_edge().is_taz_connector() {
            return;
        }
        let arrivalEdgeIndex = std::cmp::min(self.myParameter.arrivalEdge, self.myRoute.get_edges().len() as i32 - 1);
        if arrivalEdgeIndex != self.myParameter.arrivalEdge && !(onInit && (self.myParameter.parametersSet & VEHPARS_FORCE_REROUTE) != 0) {
            utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' ignores attribute arrivalEdge=% after rerouting at time=% (routeLength=%)",
                self.get_id(), self.myParameter.arrivalEdge, utils::common::time::time2string(MSNet::get_instance().get_current_time_step()), self.myRoute.get_edges().len() - 1));
        }
        let mut arrivalEdge = if self.myParameter.arrivalEdge >= 0 { self.myRoute.index_op(arrivalEdgeIndex) } else { self.myRoute.get_last_edge() };
        if !onInit {
            arrivalEdge = self.myRoute.get_last_edge();
            // ignore arrivalEdge parameter after rerouting
            self.myParameter.m().arrivalEdge = -1;
        }
        let lanes: Vec<Ptr<MSLane>> = arrivalEdge.get_lanes().clone();
        let lastLaneLength = lanes[0].get_length();
        match self.myParameter.arrivalPosProcedure {
            ArrivalPosDefinition::GIVEN => {
                if self.myParameter.arrivalPos.abs() > lastLaneLength {
                    utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' will not be able to arrive at the given position!", self.get_id()));
                }
                // Maybe we should warn the user about invalid inputs!
                self.myArrivalPos = f64::min(self.myParameter.arrivalPos, lastLaneLength);
                if self.myArrivalPos < 0.0 {
                    self.myArrivalPos = f64::max(self.myArrivalPos + lastLaneLength, 0.0);
                }
            }
            ArrivalPosDefinition::RANDOM => {
                self.myArrivalPos = utils::common::rand_helper::with_default(|r| r.rand_max(lastLaneLength));
            }
            ArrivalPosDefinition::CENTER => {
                self.myArrivalPos = lastLaneLength / 2.0;
            }
            _ => {
                self.myArrivalPos = lastLaneLength;
            }
        }
        let vclass = self.get_vehicle_type().get_vehicle_class();
        if self.myParameter.arrivalLaneProcedure == ArrivalLaneDefinition::GIVEN {
            let lane_idx = self.myParameter.arrivalLane;
            if lane_idx >= lanes.len() as i32 || !lanes[lane_idx as usize].allows_vehicle_class(vclass) {
                utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' will not be able to arrive at the given lane '%_%'!", self.get_id(), arrivalEdge.get_id(), lane_idx));
            }
            self.myArrivalLane = std::cmp::min(self.myParameter.arrivalLane, lanes.len() as i32 - 1);
        } else if self.myParameter.arrivalLaneProcedure == ArrivalLaneDefinition::FIRST_ALLOWED {
            self.myArrivalLane = -1;
            for lane in lanes.iter() {
                if lane.allows_vehicle_class(vclass) {
                    self.myArrivalLane = lane.get_index();
                    break;
                }
            }
            if self.myArrivalLane == -1 {
                utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' has no usable arrivalLane on edge '%'.", self.get_id(), arrivalEdge.get_id()));
                self.myArrivalLane = 0;
            }
        } else if self.myParameter.arrivalLaneProcedure == ArrivalLaneDefinition::RANDOM {
            // pick random lane among all usable lanes
            let usable: Vec<Ptr<MSLane>> = lanes.iter().copied().filter(|l| l.allows_vehicle_class(vclass)).collect();
            if usable.is_empty() {
                utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' has no usable arrivalLane on edge '%'.", self.get_id(), arrivalEdge.get_id()));
                self.myArrivalLane = 0;
            } else {
                let k = utils::common::rand_helper::with_default(|r| r.rand_int(usable.len() as i32)) as usize;
                self.myArrivalLane = usable[k].get_index();
            }
        }
        if self.myParameter.arrivalSpeedProcedure == ArrivalSpeedDefinition::GIVEN {
            for l in lanes.iter() {
                if self.myParameter.arrivalSpeed <= l.get_vehicle_max_speed(Ptr::new(&self.base.base)) {
                    return;
                }
            }
            utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' will not be able to arrive with the given speed!", self.get_id()));
        }
    }

    /// C++: `void MSBaseVehicle::setDepartAndArrivalEdge()`
    /// apply departEdge and arrivalEdge attributes
    // void
    // MSBaseVehicle::setDepartAndArrivalEdge() {
    //     SUMOVehicleParameter* pars = const_cast<SUMOVehicleParameter*>(myParameter);
    //     if (pars->departEdgeProcedure != RouteIndexDefinition::DEFAULT) {
    //         const int routeEdges = (int)myRoute->getEdges().size();
    //         if (pars->departEdgeProcedure == RouteIndexDefinition::RANDOM) {
    //             // write specific edge in vehroute output for reproducibility
    //             pars->departEdge = RandHelper::rand(0, routeEdges);
    //             pars->departEdgeProcedure = RouteIndexDefinition::GIVEN;
    //         }
    //         assert(pars->departEdge >= 0);
    //         if (pars->departEdge >= routeEdges) {
    //             WRITE_WARNINGF(TL("Ignoring departEdge % for vehicle '%' with % route edges"), toString(pars->departEdge), getID(), toString(routeEdges));
    //         } else {
    //             myCurrEdge = myRoute->begin() + pars->departEdge;
    //         }
    //     }
    //     if (pars->arrivalEdgeProcedure == RouteIndexDefinition::RANDOM) {
    //         const int routeEdges = (int)myRoute->getEdges().size();
    //         const int begin = (int)(myCurrEdge - myRoute->begin());
    //         // write specific edge in vehroute output for reproducibility
    //         pars->arrivalEdge = RandHelper::rand(begin, routeEdges);
    //         pars->arrivalEdgeProcedure = RouteIndexDefinition::GIVEN;
    //         assert(pars->arrivalEdge >= begin);
    //         assert(pars->arrivalEdge < routeEdges);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_depart_and_arrival_edge(&mut self) {
        if self.myParameter.departEdgeProcedure != RouteIndexDefinition::DEFAULT {
            let routeEdges = self.myRoute.get_edges().len() as i32;
            if self.myParameter.departEdgeProcedure == RouteIndexDefinition::RANDOM {
                // write specific edge in vehroute output for reproducibility
                let v = utils::common::rand_helper::with_default(|r| r.rand_int_range(0, routeEdges));
                self.myParameter.m().departEdge = v;
                self.myParameter.m().departEdgeProcedure = RouteIndexDefinition::GIVEN;
            }
            if self.myParameter.departEdge >= routeEdges {
                utils::common::msg_handler::write_warning(&utils::tlf!("Ignoring departEdge % for vehicle '%' with % route edges", self.myParameter.departEdge, self.get_id(), routeEdges));
            } else {
                self.myCurrEdge = self.myParameter.departEdge as usize;
            }
        }
        if self.myParameter.arrivalEdgeProcedure == RouteIndexDefinition::RANDOM {
            let routeEdges = self.myRoute.get_edges().len() as i32;
            let begin = self.myCurrEdge as i32;
            // write specific edge in vehroute output for reproducibility
            let v = utils::common::rand_helper::with_default(|r| r.rand_int_range(begin, routeEdges));
            self.myParameter.m().arrivalEdge = v;
            self.myParameter.m().arrivalEdgeProcedure = RouteIndexDefinition::GIVEN;
        }
    }

    /// C++: `int MSBaseVehicle::getDepartEdge() const` [const, virtual]
    // int
    // MSBaseVehicle::getDepartEdge() const {
    //     return myParameter->departEdge < myRoute->size() ? myParameter->departEdge : 0;
    // }
    #[allow(unused_variables)]
    pub fn get_depart_edge(&self) -> i32 {
        if (self.myParameter.departEdge as i32) < self.myRoute.size() { self.myParameter.departEdge as i32 } else { 0 }
    }

    /// C++: `int MSBaseVehicle::getInsertionChecks() const` [const]
    // int
    // MSBaseVehicle::getInsertionChecks() const {
    //     if (getParameter().was_set(VEHPARS_INSERTION_CHECKS_SET)) {
    //         return getParameter().insertionChecks;
    //     } else {
    //         return MSGlobals::gInsertionChecks;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_insertion_checks(&self) -> i32 {
        if self.get_parameter().was_set(VEHPARS_INSERTION_CHECKS_SET) {
            self.get_parameter().insertionChecks
        } else {
            MSGlobals::insertion_checks()
        }
    }

    /// C++: `MSLane * MSBaseVehicle::interpretOppositeStop(SUMOVehicleParameter::Stop &)` [static]
    /// interpret stop lane on opposite side of the road
    // MSLane*
    // MSBaseVehicle::interpretOppositeStop(SUMOVehicleParameter::Stop& stop) {
    //     const std::string edgeID = SUMOXMLDefinitions::getEdgeIDFromLane(stop.lane);
    //     const MSEdge* edge = MSEdge::dictionary(edgeID);
    //     if (edge == nullptr || edge->getOppositeEdge() == nullptr || stop.lane.find("_") == std::string::npos) {
    //         return nullptr;
    //     }
    //     const int laneIndex = SUMOXMLDefinitions::getIndexFromLane(stop.lane);
    //     if (laneIndex < (edge->getNumLanes() + edge->getOppositeEdge()->getNumLanes())) {
    //         const int oppositeIndex = edge->getOppositeEdge()->getNumLanes() + edge->getNumLanes() - 1 - laneIndex;
    //         stop.edge = edgeID;
    //         return edge->getOppositeEdge()->getLanes()[oppositeIndex];
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn interpret_opposite_stop(stop: &mut SUMOVehicleParameter_Stop) -> Ptr<MSLane> {
        let edgeID = utils::xml::sumo_xml_definitions::get_edge_id_from_lane(&stop.lane);
        let edge = MSEdge::dictionary_2(&edgeID);
        if edge.is_null() || edge.get_opposite_edge().is_null() || !stop.lane.contains('_') {
            return Ptr::default();
        }
        let laneIndex = utils::xml::sumo_xml_definitions::get_index_from_lane(&stop.lane).unwrap();
        if laneIndex < (edge.get_num_lanes() + edge.get_opposite_edge().get_num_lanes()) {
            let oppositeIndex = edge.get_opposite_edge().get_num_lanes() + edge.get_num_lanes() - 1 - laneIndex;
            stop.edge = edgeID;
            return edge.get_opposite_edge().get_lanes()[oppositeIndex as usize];
        }
        Ptr::default()
    }

    /// C++: `void MSBaseVehicle::rememberBlockedParkingArea(const MSStoppingPlace *, bool)` [virtual]
    // void
    // MSBaseVehicle::rememberBlockedParkingArea(const MSStoppingPlace* pa, bool local) {
    //     if (myParkingMemory == nullptr) {
    //         myParkingMemory = new StoppingPlaceMemory();
    //     }
    //     myParkingMemory->rememberBlockedStoppingPlace(pa, local);
    // }
    #[allow(unused_variables)]
    pub fn remember_blocked_parking_area(&mut self, pa: Ptr<MSStoppingPlace>, local: bool) {
        if self.myParkingMemory.is_null() {
            self.myParkingMemory = new_stopping_place_memory();
        }
        self.myParkingMemory.m().remember_blocked_stopping_place(pa, local);
    }

    /// C++: `SUMOTime MSBaseVehicle::sawBlockedParkingArea(const MSStoppingPlace *, bool) const` [const, virtual]
    // SUMOTime
    // MSBaseVehicle::sawBlockedParkingArea(const MSStoppingPlace* pa, bool local) const {
    //     if (myParkingMemory == nullptr) {
    //         return -1;
    //     }
    //     return myParkingMemory->sawBlockedStoppingPlace(pa, local);
    // }
    #[allow(unused_variables)]
    pub fn saw_blocked_parking_area(&self, pa: Ptr<MSStoppingPlace>, local: bool) -> SUMOTime {
        if self.myParkingMemory.is_null() {
            return -1;
        }
        self.myParkingMemory.saw_blocked_stopping_place(pa, local)
    }

    /// C++: `void MSBaseVehicle::rememberBlockedChargingStation(const MSStoppingPlace *, bool)` [virtual]
    // void MSBaseVehicle::rememberBlockedChargingStation(const MSStoppingPlace* cs, bool local) {
    //     if (myChargingMemory == nullptr) {
    //         myChargingMemory = new StoppingPlaceMemory();
    //     }
    //     myChargingMemory->rememberBlockedStoppingPlace(cs, local);
    // }
    #[allow(unused_variables)]
    pub fn remember_blocked_charging_station(&mut self, cs: Ptr<MSStoppingPlace>, local: bool) {
        if self.myChargingMemory.is_null() {
            self.myChargingMemory = new_stopping_place_memory();
        }
        self.myChargingMemory.m().remember_blocked_stopping_place(cs, local);
    }

    /// C++: `SUMOTime MSBaseVehicle::sawBlockedChargingStation(const MSStoppingPlace *, bool) const` [const, virtual]
    // SUMOTime
    // MSBaseVehicle::sawBlockedChargingStation(const MSStoppingPlace* cs, bool local) const {
    //     if (myChargingMemory == nullptr) {
    //         return -1;
    //     }
    //     return myChargingMemory->sawBlockedStoppingPlace(cs, local);
    // }
    #[allow(unused_variables)]
    pub fn saw_blocked_charging_station(&self, cs: Ptr<MSStoppingPlace>, local: bool) -> SUMOTime {
        if self.myChargingMemory.is_null() {
            return -1;
        }
        self.myChargingMemory.saw_blocked_stopping_place(cs, local)
    }

    /// C++: `void MSBaseVehicle::rememberParkingAreaScore(const MSStoppingPlace *, const std::string &)` [virtual]
    /// score only needed when running with gui
    // void
    // MSBaseVehicle::rememberParkingAreaScore(const MSStoppingPlace* pa, const std::string& score) {
    //     if (myParkingMemory == nullptr) {
    //         myParkingMemory = new StoppingPlaceMemory();
    //     }
    //     myParkingMemory->rememberStoppingPlaceScore(pa, score);
    // }
    #[allow(unused_variables)]
    pub fn remember_parking_area_score(&mut self, pa: Ptr<MSStoppingPlace>, score: &str) {
        if self.myParkingMemory.is_null() {
            self.myParkingMemory = new_stopping_place_memory();
        }
        self.myParkingMemory.m().remember_stopping_place_score(pa, score);
    }

    /// C++: `void MSBaseVehicle::resetParkingAreaScores()` [virtual]
    // void
    // MSBaseVehicle::resetParkingAreaScores() {
    //     if (myParkingMemory != nullptr) {
    //         myParkingMemory->resetStoppingPlaceScores();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_parking_area_scores(&mut self) {
        if !self.myParkingMemory.is_null() {
            self.myParkingMemory.m().reset_stopping_place_scores();
        }
    }

    /// C++: `void MSBaseVehicle::rememberChargingStationScore(const MSStoppingPlace *, const std::string &)` [virtual]
    // void
    // MSBaseVehicle::rememberChargingStationScore(const MSStoppingPlace* cs, const std::string& score) {
    //     if (myChargingMemory == nullptr) {
    //         myChargingMemory = new StoppingPlaceMemory();
    //     }
    //     myChargingMemory->rememberStoppingPlaceScore(cs, score);
    // }
    #[allow(unused_variables)]
    pub fn remember_charging_station_score(&mut self, cs: Ptr<MSStoppingPlace>, score: &str) {
        if self.myChargingMemory.is_null() {
            self.myChargingMemory = new_stopping_place_memory();
        }
        self.myChargingMemory.m().remember_stopping_place_score(cs, score);
    }

    /// C++: `void MSBaseVehicle::resetChargingStationScores()` [virtual]
    // void
    // MSBaseVehicle::resetChargingStationScores() {
    //     if (myChargingMemory != nullptr) {
    //         myChargingMemory->resetStoppingPlaceScores();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_charging_station_scores(&mut self) {
        if !self.myChargingMemory.is_null() {
            self.myChargingMemory.m().reset_stopping_place_scores();
        }
    }

    /// C++: `int MSBaseVehicle::getNumberParkingReroutes() const` [const, virtual]
    // int getNumberParkingReroutes() const {
    //         return myNumberParkingReroutes;
    //     }
    #[allow(unused_variables)]
    pub fn get_number_parking_reroutes(&self) -> i32 {
        self.myNumberParkingReroutes
    }

    /// C++: `void MSBaseVehicle::setNumberParkingReroutes(int)` [virtual]
    // void setNumberParkingReroutes(int value) {
    //         myNumberParkingReroutes = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_number_parking_reroutes(&mut self, value: i32) {
        self.myNumberParkingReroutes = value;
    }

    /// C++: `const StoppingPlaceMemory * MSBaseVehicle::getParkingMemory() const` [const]
    // const StoppingPlaceMemory* getParkingMemory() const {
    //         return myParkingMemory;
    //     }
    #[allow(unused_variables)]
    pub fn get_parking_memory(&self) -> Ptr<StoppingPlaceMemory> {
        self.myParkingMemory
    }

    /// C++: `const StoppingPlaceMemory * MSBaseVehicle::getChargingMemory() const` [const]
    // const StoppingPlaceMemory* getChargingMemory() const {
    //         return myChargingMemory;
    //     }
    #[allow(unused_variables)]
    pub fn get_charging_memory(&self) -> Ptr<StoppingPlaceMemory> {
        self.myChargingMemory
    }

    /// C++: `void MSBaseVehicle::resetApproachOnReroute()` [virtual, protected]
    /// reset rail signal approach information
    // virtual void resetApproachOnReroute() {}
    #[allow(unused_variables)]
    pub fn reset_approach_on_reroute(&mut self) {
        // no-op in the base class; MSVehicle overrides it
    }

    /// C++: `std::vector<StopEdgeInfo> MSBaseVehicle::getStopEdges(double &, double &, std::set<int> &) const` [const, protected]
    /// Returns the list of still pending stop edges also returns the first and last stop position
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_stop_edges(&self, firstPos: &mut f64, lastPos: &mut f64, jumps: &mut BTreeSet<i32>) -> Vec<MSBaseVehicle_StopEdgeInfo> {
        // shortcut: getStopEdges needs the StopEdgeInfo/MSStop edge iterators (Unmapped); not ported yet, so no stop edges are reported
        let _ = (firstPos, lastPos, jumps);
        Vec::new()
    }

    /// C++: `double MSBaseVehicle::addStopPriority(double, double)` [static, protected]
    // double
    // MSBaseVehicle::addStopPriority(double p1, double p2) {
    //     if (p1 < 0 || p2 < 0) {
    //         return p1;
    //     }
    //     return p1 + p2;
    // }
    #[allow(unused_variables)]
    pub fn add_stop_priority(p1: f64, p2: f64) -> f64 {
        if p1 < 0.0 || p2 < 0.0 {
            return p1;
        }
        p1 + p2
    }

    /// C++: `bool MSBaseVehicle::replaceWithAlternative(std::list<MSStop>::iterator, const MSRouteIterator, const MSRouteIterator)` [protected]
    /// replace stop with a same-name alternative that is on the route and return success
    // bool
    // MSBaseVehicle::replaceWithAlternative(std::list<MSStop>::iterator iter, const MSRouteIterator searchStart, const MSRouteIterator end) {
    //     std::pair<std::string, SumoXMLTag> nameTag = iter->getStoppingPlaceName();
    //     if (!nameTag.first.empty()) {
    //         const std::vector<MSStoppingPlace*>& alternatives = MSNet::getInstance()->getStoppingPlaceAlternatives(nameTag.first, nameTag.second);
    //         for (MSStoppingPlace* alt : alternatives) {
    //             //std::cout << SIMTIME << " veh=" << getID() << " name=" << nameTag.first << " alt=" << alt->getID() << "\n";
    //             if (&alt->getLane().getEdge() == &iter->lane->getEdge()
    //                     || !alt->getLane().allows_vehicle_class(getVClass())) {
    //                 continue;
    //             }
    //             iter->edge = std::find(searchStart, end, &alt->getLane().getEdge());
    //             if (iter->edge != end) {
    //                 iter->replaceStoppingPlace(alt);
    //                 return true;
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn replace_with_alternative(&mut self, iter: Unmapped /* std::list<MSStop>::iterator */, searchStart: MSRouteIterator, end: MSRouteIterator) -> bool {
        // shortcut: alternative-stop replacement needs MSRouteIterator and the stop list iterator (both Unmapped); not ported yet
        let _ = (iter, searchStart, end);
        false
    }

    /// C++: `MSEdgeWeightsStorage & MSBaseVehicle::_getWeightsStorage() const` [const, private]
    // MSEdgeWeightsStorage&
    // MSBaseVehicle::_getWeightsStorage() const {
    //     if (myEdgeWeights == nullptr) {
    //         myEdgeWeights = new MSEdgeWeightsStorage();
    //     }
    //     return *myEdgeWeights;
    // }
    #[allow(unused_variables)]
    pub fn get_weights_storage_3(&self) -> &mut MSEdgeWeightsStorage {
        // shortcut: MSEdgeWeightsStorage lazy allocation via the owned Ptr is not ported; returns the stored pointer
        self.myEdgeWeights.m()
    }

    /// C++: `void MSBaseVehicle::initTransientModelParams()` [private]
    /// init model parameters from generic params
    // void
    // MSBaseVehicle::initTransientModelParams() {
    //     /* Design idea for additional junction model parameters:
    //        We can distinguish between 3 levels of parameters
    //        1. typically shared by multiple vehicles -> vType parameter
    //        2. specific to one vehicle but stays constant throughout the simulation -> vehicle parameter
    //        3. specific to one vehicle and expected to change during simulation -> prefixed generic vehicle parameter
    //        */
    //     for (auto item : getParameter().getParametersMap()) {
    //         if (StringUtils::startsWith(item.first, "junctionModel.")) {
    //             setJunctionModelParameter(item.first, item.second);
    //         } else if (StringUtils::startsWith(item.first, "carFollowModel.")) {
    //             setCarFollowModelParameter(item.first, item.second);
    //         }
    //     }
    //     const std::string routingModeStr = getStringParam("device.rerouting.mode");
    //     try {
    //         int routingMode = StringUtils::toInt(routingModeStr);
    //         if (routingMode != libsumo::ROUTING_MODE_DEFAULT) {
    //             setRoutingMode(routingMode);
    //             if ((routingMode & (libsumo::ROUTING_MODE_AGGREGATED | libsumo::ROUTING_MODE_AGGREGATED_CUSTOM)) != 0) {
    //                 // vehicles wishes to use traffic-dependent routing weights on triggere rerouting
    //                 // (initWeightUpdate would be triggered anyway when equipped with device.rerouting)
    //                 MSRoutingEngine::initWeightUpdate();
    //                 MSRoutingEngine::initEdgeWeights(getVClass());
    //             }
    //         }
    //     } catch (NumberFormatException&) {
    //         // @todo interpret symbolic constants
    //         throw ProcessError(TLF("could not interpret routing.mode '%'", routingModeStr));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_transient_model_params(&mut self) {
        // shortcut: the junction-model / carFollow-model transient parameter setup (MSBaseVehicle.cpp:2725)
        // depends on SUMOVehicleParameter::getParameter and the model setters, which are not ported yet
        let _ = self.myParameter.parametersSet;
    }

    /// C++: `std::string MSBaseVehicle::getFlowID() const` [const, private]
    /// reconstruct flow id from vehicle id
    // std::string
    // MSBaseVehicle::getFlowID() const {
    //     return getID().substr(0, getID().rfind('.'));
    // }
    #[allow(unused_variables)]
    pub fn get_flow_id(&self) -> String {
        let id = self.get_id();
        match id.rfind('.') {
            Some(p) => id[..p].to_string(),
            None => id[..0].to_string(),
        }
    }

    /// C++: `void MSBaseVehicle::checkRouteRemoval()` [private]
    /// remove route at the end of the simulation
    // void
    // MSBaseVehicle::checkRouteRemoval() {
    //     // the check for an instance is needed for the unit tests which do not construct a network
    //     // TODO Optimize for speed and there should be a better way to check whether a vehicle is part of a flow
    //     if (MSNet::hasInstance() && !MSNet::getInstance()->hasFlow(getFlowID())) {
    //         myRoute->checkRemoval();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_route_removal(&mut self) {
        // shortcut: MSNet::hasInstance / hasFlow(getFlowID()) is not checked; MSRoute::checkRemoval is called directly
        self.myRoute.check_removal(false);
    }

    /// C++: `bool MSBaseVehicle::insertJump(int, MSRouteIterator, std::string &)` [private]
    /// helper function
    // bool
    // MSBaseVehicle::insertJump(int nextStopIndex, MSRouteIterator itStart, std::string& errorMsg) {
    //     bool needJump = true;
    //     if (nextStopIndex > 0) {
    //         auto itPriorStop = myStops.begin();
    //         std::advance(itPriorStop, nextStopIndex - 1);
    //         const MSStop& priorStop = *itPriorStop;
    //         if (priorStop.pars.jump >= 0) {
    //             needJump = false;
    //         }
    //     }
    //     if (needJump) {
    //         SUMOVehicleParameter::Stop jumpStopPars;
    //         jumpStopPars.endPos = (*itStart)->getLength();
    //         jumpStopPars.speed = 1000;
    //         jumpStopPars.jump = 0;
    //         jumpStopPars.edge = (*itStart)->getID();
    //         jumpStopPars.parametersSet = STOP_SPEED_SET | STOP_JUMP_SET;
    //         MSLane* jumpStopLane = nullptr;
    //         for (MSLane* cand : (*itStart)->getLanes()) {
    //             if (cand->allowsVehicleClass(getVClass())) {
    //                 jumpStopLane = cand;
    //                 break;
    //             }
    //         }
    //         if (jumpStopLane == nullptr) {
    //             errorMsg = TL("unable to replace stop with teleporting");
    //             return false;
    //         }
    //         auto itStop = myStops.begin();
    //         std::advance(itStop, nextStopIndex);
    //         MSStop jumpStop(jumpStopPars);
    //         jumpStop.init_pars(jumpStopPars);
    //         jumpStop.lane = jumpStopLane;
    //         jumpStop.edge = myRoute->end(); // will be patched in replaceRoute
    //         myStops.insert(itStop, jumpStop);
    //         if (!hasDeparted() && (int)myParameter->stops.size() > nextStopIndex) {
    //             // stops will be rebuilt from scratch so we must patch the stops in myParameter
    //             auto it = myParameter->stops.begin() + nextStopIndex;
    //             const_cast<SUMOVehicleParameter*>(myParameter)->stops.insert(it, jumpStopPars);
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn insert_jump(&mut self, nextStopIndex: i32, itStart: MSRouteIterator, errorMsg: &mut String) -> bool {
        // shortcut: jump-stop insertion needs route-iterator arithmetic (MSRouteIterator is Unmapped); not ported yet
        let _ = (nextStopIndex, itStart);
        *errorMsg = "insertJump is not ported yet".to_string();
        false
    }

    /// C++: `void MSBaseVehicle::setSkips(MSStop &, int)` [private]
    /// patch stop.pars.index to record the number of skipped candidate edges before stop.edge (in a looped route)
    // void
    // MSBaseVehicle::setSkips(MSStop& stop, int prevActiveStops) {
    //     if (hasDeparted() && stop.edge > myRoute->begin()) {
    //         // if the route is looped we must patch the index to ensure that state
    //         // loading (and vehroute-output) encode the correct number of skips
    //         stop.skips.clear();
    //         MSRouteIterator itPrev;
    //         double prevEndPos;
    //         if (prevActiveStops > 0) {
    //             assert((int)myStops.size() >= prevActiveStops);
    //             auto prevStopIt = myStops.begin();
    //             std::advance(prevStopIt, prevActiveStops - 1);
    //             const MSStop& prev = *prevStopIt;
    //             itPrev = prev.edge;
    //             prevEndPos = prev.pars.endPos;
    //         } else if (myPastStops.size() > 0) {
    //             itPrev = myRoute->begin() + myPastStops.back().routeIndex;
    //             prevEndPos = myPastStops.back().endPos;
    //         } else {
    //             itPrev = myRoute->begin() + myParameter->departEdge;
    //             prevEndPos = myDepartPos;
    //         }
    //         //auto itPrevOrig = itPrev;
    //         if (*itPrev == *stop.edge && prevEndPos > stop.pars.endPos) {
    //             itPrev++;
    //         }
    //         //std::cout << SIMTIME << " veh=" << getID() << " prevActive=" << prevActiveStops << " edge=" << (*stop.edge)->getID() << " routeIndex=" << (stop.edge - myRoute->begin()) << " prevIndex=" << (itPrev - myRoute->begin()) << "\n";
    //         while (itPrev < stop.edge) {
    //             if (*itPrev == *stop.edge) {
    //                 stop.skips.push_back((int)(itPrev - myRoute->begin()));
    //             }
    //             itPrev++;
    //         }
    //         int newIndex = STOP_INDEX_END;
    //         if (!stop.skips.empty()) {
    //             //if (getID() == "77_0_0") {
    //             //    std::cout << SIMTIME << " veh=" << getID() << " past=" << myPastStops.size() << " prevActive=" << prevActiveStops
    //             //        << " edge=" << (*stop.edge)->getID() << " routeIndex=" << (stop.edge - myRoute->begin())
    //             //        << " prevEdge=" << (*itPrevOrig)->getID()
    //             //        << " prevIndex=" << (itPrevOrig - myRoute->begin())
    //             //        << " skips=" << foundSkips << "\n";
    //             //}
    //             newIndex = (int)myPastStops.size() + prevActiveStops + (int)stop.skips.size();
    //         }
    //         const_cast<SUMOVehicleParameter::Stop&>(stop.pars).index = newIndex;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_skips(&mut self, stop: &mut MSStop, prevActiveStops: i32) {
        // shortcut: skip bookkeeping needs route-iterator positions (MSRouteIterator is Unmapped); not ported yet
        let _ = (stop, prevActiveStops);
    }

    /// C++: `SUMOTime MSBaseVehicle::activateRemindersOnReroute(SUMOTime)` [private]
    /// remove outdated driveways on reroute
    // SUMOTime
    // MSBaseVehicle::activateRemindersOnReroute(SUMOTime /*currentTime*/) {
    //     for (int i = 0; i < (int)myMoveReminders.size();) {
    //         auto rem = &myMoveReminders[i];
    //         if (rem->first->notifyReroute(*this)) {
    // #ifdef _DEBUG
    //             if (myTraceMoveReminders) {
    //                 traceMoveReminder("notifyReroute", rem->first, rem->second, true);
    //             }
    // #endif
    //             ++i;
    //         } else {
    // #ifdef _DEBUG
    //             if (myTraceMoveReminders) {
    //                 traceMoveReminder("notifyReroute", rem->first, rem->second, false);
    //             }
    // #endif
    //             myMoveReminders.erase(myMoveReminders.begin() + i);
    //         }
    //     }
    //     resetApproachOnReroute();
    //     // event only called once
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn activate_reminders_on_reroute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        let _ = currentTime;
        let mut i = 0;
        while i < self.myMoveReminders.len() {
            let mut rem = self.myMoveReminders[i].0;
            if rem.notify_reroute(self) {
                i += 1;
            } else {
                self.myMoveReminders.remove(i);
            }
        }
        // resetApproachOnReroute() is virtual: MSVehicle overrides it (MEVehicle does not)
        if MSGlobals::use_meso_sim() {
            self.reset_approach_on_reroute();
        } else {
            Ptr::from_mut(self).cast::<crate::microsim::ms_vehicle::MSVehicle>().m().reset_approach_on_reroute();
        }
        // event only called once
        0
    }

    /// C++: `MSBaseVehicle & MSBaseVehicle::operator=(const MSBaseVehicle &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSBaseVehicle) -> &mut MSBaseVehicle {
        // C++ declares operator= as deleted; assignment is not allowed
        panic!("MSBaseVehicle::operator= is deleted in C++")
    }

}

/// C++ class `MSBaseVehicle::StopEdgeInfo` (microsim/MSBaseVehicle.h)
pub struct MSBaseVehicle_StopEdgeInfo {
    pub edge: Ptr<MSEdge>,
    pub pos: f64,
    pub priority: f64,
    pub arrival: SUMOTime,
    pub isSink: bool,
    pub stopPar: Ptr<SUMOVehicleParameter_Stop>,
    /// values set during routing and used during optimization
    pub routeIndex: i32,
    pub skipped: bool,
    pub backtracked: bool,
    pub delay: SUMOTime,
    /// optional info about stopping place
    pub nameTag: (String, SumoXMLTag),
    /// set when replacing stop with an alternative
    pub origEdge: Ptr<MSEdge>,
}

impl MSBaseVehicle_StopEdgeInfo {
    /// C++: `void StopEdgeInfo::StopEdgeInfo(const MSEdge *, double, SUMOTime, double, bool)`
    // StopEdgeInfo(const MSEdge* _edge, double _priority, SUMOTime _arrival, double _pos, bool _isSink = false):
    //             edge(_edge), pos(_pos),
    //             priority(_priority),
    //             arrival(_arrival),
    //             isSink(_isSink) {}
    #[allow(unused_variables)]
    pub fn new(_edge: Ptr<MSEdge>, _priority: f64, _arrival: SUMOTime, _pos: f64, _isSink: bool) -> Self {
        Self {
            edge: _edge,
            pos: _pos,
            priority: _priority,
            arrival: _arrival,
            isSink: _isSink,
            stopPar: Ptr::default(),
            routeIndex: 0,
            skipped: false,
            backtracked: false,
            delay: 0,
            nameTag: (String::new(), SumoXMLTag::default()),
            origEdge: Ptr::default(),
        }
    }

    /// C++: `bool StopEdgeInfo::operator==(const StopEdgeInfo &) const` [const]
    // bool operator==(const StopEdgeInfo& o) const {
    //             return edge == o.edge;
    //         }
    #[allow(unused_variables)]
    pub fn eq_op(&self, o: &MSBaseVehicle_StopEdgeInfo) -> bool {
        self.edge == o.edge
    }

    /// C++: `bool StopEdgeInfo::operator!=(const StopEdgeInfo &) const` [const]
    // bool operator!=(const StopEdgeInfo& o) const {
    //             return !(*this == o);
    //         }
    #[allow(unused_variables)]
    pub fn ne_op(&self, o: &MSBaseVehicle_StopEdgeInfo) -> bool {
        !(self.eq_op(o))
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_checks_fallback_is_option_default_all_not_none() {
        // the fallback of get_insertion_checks is MSGlobals::insertion_checks(), which MSFrame sets from the "insertion-checks" option (default "all")
        MSGlobals::set_insertion_checks(SUMOVehicleParameter::parse_insertion_checks("all"));
        assert_eq!(MSGlobals::insertion_checks(), InsertionCheck::ALL as i32);
        assert_ne!(MSGlobals::insertion_checks(), InsertionCheck::NONE as i32);
    }
}
