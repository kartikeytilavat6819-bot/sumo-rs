//! Port of router/RONet.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::multimap<const SUMOTime, const std::string> RONet::ContainerMap`
pub type RONet_ContainerMap = BTreeMap<SUMOTime, Vec<String>>;

/// C++ typedef `std::map<const ROLane *, RouterProhibition> RONet::LaneProhibitions`
pub type RONet_LaneProhibitions = BTreeMap<Ptr<ROLane>, RouterProhibition>;

/// C++ typedef `std::map<const ROEdge *, RouterProhibition> RONet::Prohibitions`
pub type RONet_Prohibitions = BTreeMap<Ptr<ROEdge>, RouterProhibition>;

/// C++ typedef `MapMatcher<ROEdge, ROLane, RONode> ROMapMatcher`
pub type ROMapMatcher = crate::utils::map_matcher::MapMatcher<ROEdge, ROLane, RONode>;

/// C++ typedef `std::map<const SUMOTime, std::vector<RORoutable *>> RONet::RoutablesMap`
pub type RONet_RoutablesMap = BTreeMap<SUMOTime, Vec<Ptr<RORoutable>>>;

/// C++ typedef `std::map<std::string, RandomDistributor<SUMOVTypeParameter *> *> RONet::VTypeDistDictType`
pub type RONet_VTypeDistDictType = BTreeMap<String, Ptr<RandomDistributor<Ptr<SUMOVTypeParameter>>>>;

/// The router's network representation.
/// C++ class `RONet` (router/RONet.h)
pub struct RONet {
    /// Known vehicle ids and their departure
    pub myVehIDs: BTreeMap<String, SUMOTime>,
    /// Known person ids
    pub myPersonIDs: BTreeSet<String>,
    /// Known nodes
    pub myNodes: BTreeMap<String, Ptr<RONode>>,
    /// Known edges
    pub myEdges: BTreeMap<String, Ptr<ROEdge>>,
    /// Known bus / train / container stops and parking areas
    pub myStoppingPlaces: BTreeMap<SumoXMLTag, BTreeMap<String, Ptr<SUMOVehicleParameter_Stop>>>,
    /// Known vehicle types
    pub myVehicleTypes: BTreeMap<String, Ptr<SUMOVTypeParameter>>,
    /// A distribution of vehicle types (probability->vehicle type)
    pub myVTypeDistDict: RONet_VTypeDistDictType,
    /// Whether the default vehicle type was already used or can still be replaced
    pub myDefaultVTypeMayBeDeleted: bool,
    /// Whether the default pedestrian type was already used or can still be replaced
    pub myDefaultPedTypeMayBeDeleted: bool,
    /// Whether the default bicycle type was already used or can still be replaced
    pub myDefaultBikeTypeMayBeDeleted: bool,
    /// Whether the default taxi type was already used or can still be replaced
    pub myDefaultTaxiTypeMayBeDeleted: bool,
    /// Whether the default rail type was already used or can still be replaced
    pub myDefaultRailTypeMayBeDeleted: bool,
    /// Known routes
    pub myRoutes: BTreeMap<String, Ptr<RORouteDef>>,
    /// Known routables
    pub myRoutables: RONet_RoutablesMap,
    /// Known flows
    pub myFlows: BTreeMap<String, Ptr<SUMOVehicleParameter>>,
    /// whether any flows are still active
    pub myHaveActiveFlows: bool,
    pub myContainers: RONet_ContainerMap,
    /// vehicles to keep for public transport routing
    pub myPTVehicles: Vec<Ptr<RORoutable>>,
    /// Departure times for randomized flows
    pub myDepartures: BTreeMap<String, Vec<SUMOTime>>,
    /// traffic assignment zones with sources and sinks
    pub myDistricts: BTreeMap<String, (Vec<String>, Vec<String>)>, // C++: std::map<std::string, std::pair<std::vector<std::string>, std::vector<std::string>>>
    /// The file to write the computed routes into
    pub myRoutesOutput: Ptr<OutputDevice>,
    /// The file to write the computed route alternatives into
    pub myRouteAlternativesOutput: Ptr<OutputDevice>,
    /// The file to write the vehicle types into
    pub myTypesOutput: Ptr<OutputDevice>,
    /// The number of read routes
    pub myReadRouteNo: i32,
    /// The number of discarded routes
    pub myDiscardedRouteNo: i32,
    /// The number of written routes
    pub myWrittenRouteNo: i32,
    /// Whether traffic was excluded by the begin time or processed for output.
    pub myHaveSkippedRoutes: bool,
    pub myHaveProcessedRoutes: bool,
    /// Whether the network contains edges which not all vehicles may pass
    pub myHavePermissions: bool,
    /// The vehicle class specific speed restrictions
    pub mySpeedRestrictions: BTreeMap<String, BTreeMap<SUMOVehicleClass, f64>>, // C++: std::map<std::string, std::map<SUMOVehicleClass, double>>
    /// whether parameter-based access restrictions are configured
    pub myHaveParamRestrictions: bool,
    /// Preferences for routing
    pub myVClassPreferences: BTreeMap<SUMOVehicleClass, BTreeMap<String, f64>>, // C++: std::map<SUMOVehicleClass, std::map<std::string, double>>
    pub myVTypePreferences: BTreeMap<String, BTreeMap<String, f64>>, // C++: std::map<std::string, std::map<std::string, double>>
    /// The number of internal edges in the dictionary
    pub myNumInternalEdges: i32,
    /// handler for ignorable error messages
    pub myErrorHandler: Ptr<MsgHandler>,
    /// whether to keep the vtype distribution in output
    pub myKeepVTypeDist: bool,
    /// whether to calculate routes for public transport
    pub myDoPTRouting: bool,
    /// whether to preserve flows
    pub myKeepFlows: bool,
    /// whether the network contains bidirectional railway edges
    pub myHasBidiEdges: bool,
    /// the maximum traveltime beyond which routing is considered a failure
    pub myMaxTraveltime: f64,
    /// temporary edge closing (rerouters)
    pub myProhibitions: RONet_Prohibitions,
    /// temporary lane closing (rerouters)
    pub myLaneProhibitions: RONet_LaneProhibitions,
    pub myLaneProhibitionTimes: BTreeMap<RONet_TimeKey, BTreeSet<Ptr<ROLane>>>, // C++: std::map<double, std::set<const ROLane *>>
}

// C++ static members of RONet (port as module-level statics, see AGENTS.md):
//   static RONet * RONet::myInstance;


/// C++ `ROEdge` and `RONode` deletion (`delete edge`) for objects that were rejected by the net
fn delete_edge(edge: Ptr<ROEdge>) {
    edge.m().drop_();
    drop(unsafe { Box::from_raw(edge.as_raw()) });
}

/// C++ `SUMOVTypeParameter::parametersSet` bit VTYPEPARS_VEHICLECLASS_SET (SUMOVTypeParameter.h)
// shortcut: the VTYPEPARS_* flags are not ported as Rust constants yet, this copies the C++ value
const VTYPEPARS_VEHICLECLASS_SET: i64 = 1 << 7;

/// C++ `OptionsCont::getOptions().exists(name)` (global options)
fn oexists(name: &str) -> bool {
    utils::options::options_cont::exists(name)
}

/// C++ `OptionsCont::getOptions().getBool(name)` (global options)
fn get_bool_global(name: &str) -> bool {
    utils::options::options_cont::get_bool(name)
}

/// C++ `OptionsCont::getOptions().getString(name)` (global options)
fn get_string_global(name: &str) -> String {
    utils::options::options_cont::get_string(name)
}

/// C++ `options.exists(name)` for an explicit options object
fn oexists_in(o: &OptionsCont, name: &str) -> bool {
    o.exists(name)
}

/// C++ `options.getBool(name)` for an explicit options object
fn ob(o: &OptionsCont, name: &str) -> bool {
    o.get_bool(name).unwrap_or_else(|e| panic!("{e}"))
}

/// C++ `options.getInt(name)` for an explicit options object
fn oint(o: &OptionsCont, name: &str) -> i32 {
    o.get_int(name).unwrap_or_else(|e| panic!("{e}"))
}

/// C++ `options.getFloat(name)` for an explicit options object
fn ofloat(o: &OptionsCont, name: &str) -> f64 {
    o.get_float(name).unwrap_or_else(|e| panic!("{e}"))
}

/// C++ `options.getString(name)` for an explicit options object
fn ostring(o: &OptionsCont, name: &str) -> String {
    o.get_string(name).unwrap_or_else(|e| panic!("{e}"))
}

/// C++ `options.isSet(name)` for an explicit options object
fn oset(o: &OptionsCont, name: &str) -> bool {
    o.is_set(name, false).unwrap_or_else(|e| panic!("{e}"))
}

/// C++ `MsgHandler::getWarningInstance()` / `getErrorInstance()`
// shortcut: the instances are kept in a locked static, the pointer stays valid because they are only dropped by cleanup_on_end
fn msg_instance(warning: bool) -> Ptr<MsgHandler> {
    let t = if warning { MsgType::Warning } else { MsgType::Error };
    Ptr::from_raw(utils::common::msg_handler::with_instance(t, |h| h as *mut MsgHandler))
}

/// C++ `OutputDevice::getDevice(name)`; the shared device stays alive in the output registry
fn get_device_ptr(name: &str) -> Ptr<OutputDevice> {
    let rc = OutputDevice::get_device(name, true).unwrap_or_else(|e| panic!("{e}"));
    Ptr::from_raw(rc.as_ptr())
}

/// C++ `min(a, b)` for SUMOTime
fn min2i(a: SUMOTime, b: SUMOTime) -> SUMOTime {
    if a < b { a } else { b }
}

/// C++ `RouterProhibition` copy (the struct is not Clone in Rust yet)
fn copy_prohibition(p: &RouterProhibition) -> RouterProhibition {
    RouterProhibition {
        begin: p.begin,
        end: p.end,
        permissions: p.permissions,
    }
}

/// C++ implicit copy constructor `SUMOVehicleParameter::Stop(const Stop&)`
// shortcut: duplicates copy_stop of microsim/ms_insertion_control.rs (private there), merge when that is made public
fn copy_stop(s: &SUMOVehicleParameter_Stop) -> SUMOVehicleParameter_Stop {
    SUMOVehicleParameter_Stop {
        base: s.base.clone(),
        edge: s.edge.clone(),
        lane: s.lane.clone(),
        busstop: s.busstop.clone(),
        containerstop: s.containerstop.clone(),
        parkingarea: s.parkingarea.clone(),
        chargingStation: s.chargingStation.clone(),
        overheadWireSegment: s.overheadWireSegment.clone(),
        startPos: s.startPos,
        endPos: s.endPos,
        arrival: s.arrival,
        duration: s.duration,
        durationDist: s.durationDist.clone(),
        until: s.until,
        extension: s.extension,
        waitUntil: s.waitUntil,
        triggered: s.triggered,
        containerTriggered: s.containerTriggered,
        joinTriggered: s.joinTriggered,
        parking: s.parking,
        awaitedPersons: s.awaitedPersons.clone(),
        permitted: s.permitted.clone(),
        awaitedContainers: s.awaitedContainers.clone(),
        friendlyPos: s.friendlyPos,
        actType: s.actType.clone(),
        tripId: s.tripId.clone(),
        line: s.line.clone(),
        split: s.split.clone(),
        join: s.join.clone(),
        speed: s.speed,
        posLat: s.posLat,
        onDemand: s.onDemand,
        priority: s.priority,
        jump: s.jump,
        jumpUntil: s.jumpUntil,
        started: s.started,
        ended: s.ended,
        accessPos: s.accessPos.clone(),
        index: s.index,
        routeIndex: s.routeIndex,
        parametersSet: s.parametersSet,
        collision: s.collision,
        breakDown: s.breakDown,
    }
}

/// C++ implicit copy constructor `SUMOVehicleParameter(const SUMOVehicleParameter&)`
// shortcut: duplicates copy_vehicle_parameter of microsim/ms_insertion_control.rs (private there), merge when that is made public
fn copy_vehicle_parameter(p: &SUMOVehicleParameter) -> SUMOVehicleParameter {
    SUMOVehicleParameter {
        base: p.base.clone(),
        tag: p.tag,
        id: p.id.clone(),
        routeid: p.routeid.clone(),
        vtypeid: p.vtypeid.clone(),
        color: p.color,
        depart: p.depart,
        departProcedure: p.departProcedure,
        departLane: p.departLane,
        departLaneProcedure: p.departLaneProcedure,
        departPos: p.departPos,
        departPosProcedure: p.departPosProcedure,
        departPosLat: p.departPosLat,
        departPosLatProcedure: p.departPosLatProcedure,
        departSpeed: p.departSpeed,
        departSpeedProcedure: p.departSpeedProcedure,
        departEdge: p.departEdge,
        departEdgeProcedure: p.departEdgeProcedure,
        arrivalLane: p.arrivalLane,
        arrivalLaneProcedure: p.arrivalLaneProcedure,
        arrivalPos: p.arrivalPos,
        arrivalPosProcedure: p.arrivalPosProcedure,
        arrivalPosLat: p.arrivalPosLat,
        arrivalPosLatProcedure: p.arrivalPosLatProcedure,
        arrivalSpeed: p.arrivalSpeed,
        arrivalSpeedProcedure: p.arrivalSpeedProcedure,
        arrivalEdge: p.arrivalEdge,
        arrivalEdgeProcedure: p.arrivalEdgeProcedure,
        repetitionNumber: p.repetitionNumber,
        repetitionsDone: p.repetitionsDone,
        repetitionOffset: p.repetitionOffset,
        repetitionTotalOffset: p.repetitionTotalOffset,
        repetitionProbability: p.repetitionProbability,
        poissonRate: p.poissonRate,
        repetitionEnd: p.repetitionEnd,
        line: p.line.clone(),
        fromTaz: p.fromTaz.clone(),
        toTaz: p.toTaz.clone(),
        stops: p.stops.iter().map(copy_stop).collect(),
        via: p.via.clone(),
        parkingBadges: p.parkingBadges.clone(),
        modes: p.modes,
        vTypes: p.vTypes.clone(),
        personNumber: p.personNumber,
        containerNumber: p.containerNumber,
        speedFactor: p.speedFactor,
        calibratorSpeed: p.calibratorSpeed,
        insertionChecks: p.insertionChecks,
        parametersSet: p.parametersSet,
    }
}

/// C++ `std::map<double, ...>` key ordering for the lane prohibition timeline (times are finite)
#[derive(Clone, Copy, PartialEq)]
pub struct RONet_TimeKey(pub f64);
impl Eq for RONet_TimeKey {}
impl PartialOrd for RONet_TimeKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RONet_TimeKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal)
    }
}

// C++ static member RONet::myInstance
static mut MY_INSTANCE: Ptr<RONet> = Ptr::null();


impl RONet {
    /// Makes this net the singleton returned by get_instance (C++ myInstance = this in the constructor)
    pub fn register_instance(this: Ptr<RONet>) {
        if !unsafe { MY_INSTANCE }.is_null() {
            panic!("A network was already constructed.");
        }
        unsafe { MY_INSTANCE = this; }
    }

    /// C++: `void RONet::RONet()`
    /// Constructor
    // RONet::RONet() :
    //     myVehicleTypes(), myDefaultVTypeMayBeDeleted(true),
    //     myDefaultPedTypeMayBeDeleted(true),
    //     myDefaultBikeTypeMayBeDeleted(true),
    //     myDefaultTaxiTypeMayBeDeleted(true),
    //     myDefaultRailTypeMayBeDeleted(true),
    //     myHaveActiveFlows(true),
    //     myRoutesOutput(nullptr), myRouteAlternativesOutput(nullptr), myTypesOutput(nullptr),
    //     myReadRouteNo(0), myDiscardedRouteNo(0), myWrittenRouteNo(0),
    //     myHavePermissions(false),
    //     myHaveParamRestrictions(false),
    //     myNumInternalEdges(0),
    //     myErrorHandler(OptionsCont::getOptions().exists("ignore-errors")
    //                    && OptionsCont::getOptions().getBool("ignore-errors") ? MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance()),
    //     myKeepVTypeDist(OptionsCont::getOptions().exists("keep-vtype-distributions")
    //                     && OptionsCont::getOptions().getBool("keep-vtype-distributions")),
    //     myDoPTRouting(!OptionsCont::getOptions().exists("ptline-routing")
    //                   || OptionsCont::getOptions().getBool("ptline-routing")),
    //     myKeepFlows(OptionsCont::getOptions().exists("keep-flows")
    //                 && OptionsCont::getOptions().getBool("keep-flows")),
    //     myHasBidiEdges(false),
    //     myMaxTraveltime(OptionsCont::getOptions().exists("max-traveltime") ? STEPS2TIME(string2time(OptionsCont::getOptions().getString("max-traveltime"))) : -1) {
    //     if (myInstance != nullptr) {
    //         throw ProcessError(TL("A network was already constructed."));
    //     }
    //     SUMOVTypeParameter* type = new SUMOVTypeParameter(DEFAULT_VTYPE_ID, SVC_PASSENGER);
    //     type->onlyReferenced = true;
    //     myVehicleTypes.add(type->id, type);
    //
    //     SUMOVTypeParameter* defPedType = new SUMOVTypeParameter(DEFAULT_PEDTYPE_ID, SVC_PEDESTRIAN);
    //     defPedType->onlyReferenced = true;
    //     defPedType->parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //     myVehicleTypes.add(defPedType->id, defPedType);
    //
    //     SUMOVTypeParameter* defBikeType = new SUMOVTypeParameter(DEFAULT_BIKETYPE_ID, SVC_BICYCLE);
    //     defBikeType->onlyReferenced = true;
    //     defBikeType->parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //     myVehicleTypes.add(defBikeType->id, defBikeType);
    //
    //     SUMOVTypeParameter* defTaxiType = new SUMOVTypeParameter(DEFAULT_TAXITYPE_ID, SVC_TAXI);
    //     defTaxiType->onlyReferenced = true;
    //     defTaxiType->parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //     myVehicleTypes.add(defTaxiType->id, defTaxiType);
    //
    //     SUMOVTypeParameter* defRailType = new SUMOVTypeParameter(DEFAULT_RAILTYPE_ID, SVC_RAIL);
    //     defRailType->onlyReferenced = true;
    //     defRailType->parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
    //     myVehicleTypes.add(defRailType->id, defRailType);
    //
    //     myInstance = this;
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        let mut ty = SUMOVTypeParameter::new(DEFAULT_VTYPE_ID, SVC_PASSENGER);
        ty.onlyReferenced = true;
        let mut vehicleTypes: BTreeMap<String, Ptr<SUMOVTypeParameter>> = BTreeMap::new();
        vehicleTypes.insert(ty.id.clone(), Ptr::from_raw(Box::into_raw(Box::new(ty))));

        let mut defPedType = SUMOVTypeParameter::new(DEFAULT_PEDTYPE_ID, SVC_PEDESTRIAN);
        defPedType.onlyReferenced = true;
        defPedType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
        vehicleTypes.insert(defPedType.id.clone(), Ptr::from_raw(Box::into_raw(Box::new(defPedType))));

        let mut defBikeType = SUMOVTypeParameter::new(DEFAULT_BIKETYPE_ID, SVC_BICYCLE);
        defBikeType.onlyReferenced = true;
        defBikeType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
        vehicleTypes.insert(defBikeType.id.clone(), Ptr::from_raw(Box::into_raw(Box::new(defBikeType))));

        let mut defTaxiType = SUMOVTypeParameter::new(DEFAULT_TAXITYPE_ID, SVC_TAXI);
        defTaxiType.onlyReferenced = true;
        defTaxiType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
        vehicleTypes.insert(defTaxiType.id.clone(), Ptr::from_raw(Box::into_raw(Box::new(defTaxiType))));

        let mut defRailType = SUMOVTypeParameter::new(DEFAULT_RAILTYPE_ID, SVC_RAIL);
        defRailType.onlyReferenced = true;
        defRailType.parametersSet |= VTYPEPARS_VEHICLECLASS_SET;
        vehicleTypes.insert(defRailType.id.clone(), Ptr::from_raw(Box::into_raw(Box::new(defRailType))));

        // shortcut: C++ sets myInstance = this here; Rust returns by value, so the owner must call RONet::register_instance once the net is at its final address
        let ignoreErrors = oexists(&"ignore-errors") && get_bool_global("ignore-errors");
        RONet {
            myVehIDs: BTreeMap::new(),
            myPersonIDs: BTreeSet::new(),
            myNodes: BTreeMap::new(),
            myEdges: BTreeMap::new(),
            myStoppingPlaces: BTreeMap::new(),
            myVehicleTypes: vehicleTypes,
            myVTypeDistDict: BTreeMap::new(),
            myDefaultVTypeMayBeDeleted: true,
            myDefaultPedTypeMayBeDeleted: true,
            myDefaultBikeTypeMayBeDeleted: true,
            myDefaultTaxiTypeMayBeDeleted: true,
            myDefaultRailTypeMayBeDeleted: true,
            myRoutes: BTreeMap::new(),
            myRoutables: BTreeMap::new(),
            myFlows: BTreeMap::new(),
            myHaveActiveFlows: true,
            myContainers: BTreeMap::new(),
            myPTVehicles: Vec::new(),
            myDepartures: BTreeMap::new(),
            myDistricts: BTreeMap::new(),
            myRoutesOutput: Ptr::null(),
            myRouteAlternativesOutput: Ptr::null(),
            myTypesOutput: Ptr::null(),
            myReadRouteNo: 0,
            myDiscardedRouteNo: 0,
            myWrittenRouteNo: 0,
            myHaveSkippedRoutes: false,
            myHaveProcessedRoutes: false,
            myHavePermissions: false,
            mySpeedRestrictions: BTreeMap::new(),
            myHaveParamRestrictions: false,
            myVClassPreferences: BTreeMap::new(),
            myVTypePreferences: BTreeMap::new(),
            myNumInternalEdges: 0,
            myErrorHandler: msg_instance(ignoreErrors),
            myKeepVTypeDist: oexists(&"keep-vtype-distributions") && get_bool_global("keep-vtype-distributions"),
            myDoPTRouting: !oexists(&"ptline-routing") || get_bool_global("ptline-routing"),
            myKeepFlows: oexists(&"keep-flows") && get_bool_global("keep-flows"),
            myHasBidiEdges: false,
            myMaxTraveltime: if oexists(&"max-traveltime") {
                utils::common::time::steps2time(utils::common::time::string2time(&get_string_global("max-traveltime")).unwrap_or_else(|e| panic!("{e}")))
            } else {
                -1.
            },
            myProhibitions: BTreeMap::new(),
            myLaneProhibitions: BTreeMap::new(),
            myLaneProhibitionTimes: BTreeMap::new(),
        }
    }

    /// C++: `RONet * RONet::getInstance()` [static]
    /// Returns the pointer to the unique instance of RONet (singleton).
    // RONet*
    // RONet::getInstance(void) {
    //     if (myInstance != nullptr) {
    //         return myInstance;
    //     }
    //     throw ProcessError(TL("A network was not yet constructed."));
    // }
    #[allow(unused_variables)]
    pub fn get_instance() -> Ptr<RONet> {
        let inst = unsafe { MY_INSTANCE };
        if inst.is_null() {
            panic!("A network was not yet constructed.");
        }
        inst
    }

    /// C++: `void RONet::~RONet()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is implicit (members are destroyed by the owner), nothing to do here
    }

    /// C++: `void RONet::addSpeedRestriction(const std::string &, const SUMOVehicleClass, const double)`
    /// Adds a restriction for an edge type
    // void
    // RONet::addSpeedRestriction(const std::string& id, const SUMOVehicleClass svc, const double speed) {
    //     mySpeedRestrictions[id][svc] = speed;
    // }
    #[allow(unused_variables)]
    pub fn add_speed_restriction(&mut self, id: &str, svc: SUMOVehicleClass, speed: f64) {
        self.mySpeedRestrictions.entry(id.to_string()).or_default().insert(svc, speed);
    }

    /// C++: `const std::map<SUMOVehicleClass, double> * RONet::getRestrictions(const std::string &) const` [const]
    /// Returns the restrictions for an edge type If no restrictions are present, 0 is returned.
    // const std::map<SUMOVehicleClass, double>*
    // RONet::getRestrictions(const std::string& id) const {
    //     std::map<std::string, std::map<SUMOVehicleClass, double> >::const_iterator i = mySpeedRestrictions.find(id);
    //     if (i == mySpeedRestrictions.end()) {
    //         return nullptr;
    //     }
    //     return &i->second;
    // }
    #[allow(unused_variables)]
    pub fn get_restrictions(&self, id: &str) -> Ptr<BTreeMap<SUMOVehicleClass, f64>> {
        match self.mySpeedRestrictions.get(id) {
            Some(m) => Ptr::new(m),
            None => Ptr::null(),
        }
    }

    /// C++: `bool RONet::hasSpeedRestrictions() const` [const]
    // bool hasSpeedRestrictions() const {
    //         return !mySpeedRestrictions.empty();
    //     }
    #[allow(unused_variables)]
    pub fn has_speed_restrictions(&self) -> bool {
        !self.mySpeedRestrictions.is_empty()
    }

    /// C++: `bool RONet::hasParamRestrictions() const` [const]
    // bool hasParamRestrictions() const {
    //         return myHaveParamRestrictions;
    //     }
    #[allow(unused_variables)]
    pub fn has_param_restrictions(&self) -> bool {
        self.myHaveParamRestrictions
    }

    /// C++: `void RONet::setParamRestrictions()`
    // void setParamRestrictions() {
    //         myHaveParamRestrictions = true;
    //     }
    #[allow(unused_variables)]
    pub fn set_param_restrictions(&mut self) {
        self.myHaveParamRestrictions = true;
    }

    /// C++: `double RONet::getPreference(const std::string &, const SUMOVTypeParameter &) const` [const]
    /// retriefe edge type specific routing preference
    // double
    // RONet::getPreference(const std::string& routingType, const SUMOVTypeParameter& pars) const {
    //     if (gRoutingPreferences) {
    //         auto it = myVTypePreferences.find(pars.id);
    //         if (it != myVTypePreferences.end()) {
    //             auto it2 = it->second.find(routingType);
    //             if (it2 != it->second.end()) {
    //                 return it2->second;
    //             }
    //         }
    //         auto it3 = myVClassPreferences.find(pars.vehicleClass);
    //         if (it3 != myVClassPreferences.end()) {
    //             auto it4 = it3->second.find(routingType);
    //             if (it4 != it3->second.end()) {
    //                 return it4->second;
    //             }
    //         }
    //         // fallback to generel preferences
    //         it = myVTypePreferences.find("");
    //         if (it != myVTypePreferences.end()) {
    //             auto it2 = it->second.find(routingType);
    //             if (it2 != it->second.end()) {
    //                 return it2->second;
    //             }
    //         }
    //     }
    //     return 1;
    // }
    #[allow(unused_variables)]
    pub fn get_preference(&self, routingType: &str, pars: &SUMOVTypeParameter) -> f64 {
        if utils::common::std_defs::g_routing_preferences() {
            if let Some(m) = self.myVTypePreferences.get(&pars.id) {
                if let Some(v) = m.get(routingType) {
                    return *v;
                }
            }
            if let Some(m) = self.myVClassPreferences.get(&pars.vehicleClass) {
                if let Some(v) = m.get(routingType) {
                    return *v;
                }
            }
            // fallback to generel preferences
            if let Some(m) = self.myVTypePreferences.get("") {
                if let Some(v) = m.get(routingType) {
                    return *v;
                }
            }
        }
        1.
    }

    /// C++: `void RONet::addPreference(const std::string &, SUMOVehicleClass, double)`
    /// add edge type specific routing preference
    // void
    // RONet::addPreference(const std::string& routingType, SUMOVehicleClass svc, double prio) {
    //     myVClassPreferences[svc][routingType] = prio;
    //     gRoutingPreferences = true;
    // }
    #[allow(unused_variables)]
    pub fn add_preference(&mut self, routingType: &str, svc: SUMOVehicleClass, prio: f64) {
        self.myVClassPreferences.entry(svc).or_default().insert(routingType.to_string(), prio);
        utils::common::std_defs::set_g_routing_preferences(true);
    }

    /// C++: `void RONet::addPreference(const std::string &, std::string, double)`
    /// add edge type specific routing preference
    // void
    // RONet::addPreference(const std::string& routingType, std::string vType, double prio) {
    //     myVTypePreferences[vType][routingType] = prio;
    //     gRoutingPreferences = true;
    // }
    #[allow(unused_variables)]
    pub fn add_preference_2(&mut self, routingType: &str, vType: String, prio: f64) {
        self.myVTypePreferences.entry(vType).or_default().insert(routingType.to_string(), prio);
        utils::common::std_defs::set_g_routing_preferences(true);
    }

    /// C++: `bool RONet::addEdge(ROEdge *)` [virtual]
    // bool
    // RONet::addEdge(ROEdge* edge) {
    //     if (!myEdges.add(edge->getID(), edge)) {
    //         WRITE_ERRORF(TL("The edge '%' occurs at least twice."), edge->getID());
    //         delete edge;
    //         return false;
    //     }
    //     if (edge->isInternal()) {
    //         myNumInternalEdges += 1;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_edge(&mut self, edge: Ptr<ROEdge>) -> bool {
        let id = edge.get_id().to_string();
        if self.myEdges.contains_key(&id) {
            write_errorf!("The edge '%' occurs at least twice.", id);
            delete_edge(edge);
            return false;
        }
        if edge.is_internal() {
            self.myNumInternalEdges += 1;
        }
        self.myEdges.insert(id, edge);
        true
    }

    /// C++: `bool RONet::addDistrict(const std::string, ROEdge *, ROEdge *)`
    // bool
    // RONet::addDistrict(const std::string id, ROEdge* source, ROEdge* sink) {
    //     if (myDistricts.count(id) > 0) {
    //         WRITE_ERRORF(TL("The TAZ '%' occurs at least twice."), id);
    //         delete source;
    //         delete sink;
    //         return false;
    //     }
    //     sink->setFunction(SumoXMLEdgeFunc::CONNECTOR);
    //     if (!addEdge(sink)) {
    //         return false;
    //     }
    //     source->setFunction(SumoXMLEdgeFunc::CONNECTOR);
    //     if (!addEdge(source)) {
    //         return false;
    //     }
    //     sink->setOtherTazConnector(source);
    //     source->setOtherTazConnector(sink);
    //     myDistricts[id] = std::make_pair(std::vector<std::string>(), std::vector<std::string>());
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_district(&mut self, id: String, source: Ptr<ROEdge>, sink: Ptr<ROEdge>) -> bool {
        if self.myDistricts.contains_key(&id) {
            write_errorf!("The TAZ '%' occurs at least twice.", id);
            delete_edge(source);
            delete_edge(sink);
            return false;
        }
        sink.m().set_function(SumoXMLEdgeFunc::CONNECTOR);
        if !self.add_edge(sink) {
            return false;
        }
        source.m().set_function(SumoXMLEdgeFunc::CONNECTOR);
        if !self.add_edge(source) {
            return false;
        }
        sink.m().set_other_taz_connector(source);
        source.m().set_other_taz_connector(sink);
        self.myDistricts.insert(id, (Vec::new(), Vec::new()));
        true
    }

    /// C++: `bool RONet::addDistrictEdge(const std::string, const std::string, const bool)`
    // bool
    // RONet::addDistrictEdge(const std::string tazID, const std::string edgeID, const bool isSource) {
    //     if (myDistricts.count(tazID) == 0) {
    //         WRITE_ERRORF(TL("The TAZ '%' is unknown."), tazID);
    //         return false;
    //     }
    //     ROEdge* edge = getEdge(edgeID);
    //     if (edge == nullptr) {
    //         WRITE_ERRORF(TL("The edge '%' for TAZ '%' is unknown."), edgeID, tazID);
    //         return false;
    //     }
    //     if (isSource) {
    //         getEdge(tazID + "-source")->addSuccessor(edge);
    //         myDistricts[tazID].first.push_back(edgeID);
    //     } else {
    //         edge->addSuccessor(getEdge(tazID + "-sink"));
    //         myDistricts[tazID].second.push_back(edgeID);
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_district_edge(&mut self, tazID: String, edgeID: String, isSource: bool) -> bool {
        if !self.myDistricts.contains_key(&tazID) {
            write_errorf!("The TAZ '%' is unknown.", tazID);
            return false;
        }
        let edge = self.get_edge(&edgeID);
        if edge.is_null() {
            write_errorf!("The edge '%' for TAZ '%' is unknown.", edgeID, tazID);
            return false;
        }
        if isSource {
            let src = self.get_edge(&format!("{}-source", tazID));
            src.m().add_successor(edge, Ptr::null(), String::new());
            self.myDistricts.get_mut(&tazID).expect("checked above").0.push(edgeID);
        } else {
            let sink = self.get_edge(&format!("{}-sink", tazID));
            edge.m().add_successor(sink, Ptr::null(), String::new());
            self.myDistricts.get_mut(&tazID).expect("checked above").1.push(edgeID);
        }
        true
    }

    /// C++: `void RONet::addJunctionTaz(ROAbstractEdgeBuilder &)`
    /// add a taz for every junction unless a taz with the same id already exists
    // void
    // RONet::addJunctionTaz(ROAbstractEdgeBuilder& eb) {
    //     for (auto item : myNodes) {
    //         const std::string tazID = item.first;
    //         if (myDistricts.count(tazID) != 0) {
    //             WRITE_WARNINGF(TL("A TAZ with id '%' already exists. Not building junction TAZ."), tazID);
    //             continue;
    //         }
    //         const std::string sourceID = tazID + "-source";
    //         const std::string sinkID = tazID + "-sink";
    //         // sink must be added before source
    //         ROEdge* sink = eb.buildEdge(sinkID, nullptr, nullptr, 0, "", "");
    //         ROEdge* source = eb.buildEdge(sourceID, nullptr, nullptr, 0, "", "");
    //         sink->setOtherTazConnector(source);
    //         source->setOtherTazConnector(sink);
    //         if (!addDistrict(tazID, source, sink)) {
    //             continue;
    //         }
    //         auto& district = myDistricts[tazID];
    //         const RONode* junction = item.second;
    //         for (const ROEdge* edge : junction->getIncoming()) {
    //             if (!edge->isInternal()) {
    //                 const_cast<ROEdge*>(edge)->addSuccessor(sink);
    //                 district.second.push_back(edge->getID());
    //             }
    //         }
    //         for (const ROEdge* edge : junction->getOutgoing()) {
    //             if (!edge->isInternal()) {
    //                 source->addSuccessor(const_cast<ROEdge*>(edge));
    //                 district.first.push_back(edge->getID());
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_junction_taz(&mut self, eb: &mut ROAbstractEdgeBuilder) {
        let nodes: Vec<(String, Ptr<RONode>)> = self.myNodes.iter().map(|(k, v)| (k.clone(), *v)).collect();
        for (tazID, junction) in nodes {
            if self.myDistricts.contains_key(&tazID) {
                write_warningf!("A TAZ with id '%' already exists. Not building junction TAZ.", tazID);
                continue;
            }
            let sourceID = format!("{}-source", tazID);
            let sinkID = format!("{}-sink", tazID);
            // sink must be added before source
            let sink = eb.build_edge(&sinkID, Ptr::null(), Ptr::null(), 0, "", "");
            let source = eb.build_edge(&sourceID, Ptr::null(), Ptr::null(), 0, "", "");
            sink.m().set_other_taz_connector(source);
            source.m().set_other_taz_connector(sink);
            if !self.add_district(tazID.clone(), source, sink) {
                continue;
            }
            for edge in junction.get_incoming().iter() {
                if !edge.is_internal() {
                    edge.m().add_successor(sink, Ptr::null(), String::new());
                    self.myDistricts.get_mut(&tazID).expect("added above").1.push(edge.get_id().to_string());
                }
            }
            for edge in junction.get_outgoing().iter() {
                if !edge.is_internal() {
                    source.m().add_successor(*edge, Ptr::null(), String::new());
                    self.myDistricts.get_mut(&tazID).expect("added above").0.push(edge.get_id().to_string());
                }
            }
        }
    }

    /// C++: `void RONet::setBidiEdges(const std::map<ROEdge *, std::string> &)`
    /// add a taz for every junction unless a taz with the same id already exists
    // void
    // RONet::setBidiEdges(const std::map<ROEdge*, std::string>& bidiMap) {
    //     for (const auto& item : bidiMap) {
    //         ROEdge* bidi = myEdges.get(item.second);
    //         if (bidi == nullptr) {
    //             WRITE_ERRORF(TL("The bidi edge '%' is not known."), item.second);
    //         }
    //         item.first->setBidiEdge(bidi);
    //         myHasBidiEdges = true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_bidi_edges(&mut self, bidiMap: &BTreeMap<Ptr<ROEdge>, String>) {
        for (edge, bidiName) in bidiMap.iter() {
            let bidi = self.get_edge(bidiName);
            if bidi.is_null() {
                write_errorf!("The bidi edge '%' is not known.", bidiName);
            }
            edge.m().set_bidi_edge(bidi);
            self.myHasBidiEdges = true;
        }
    }

    /// C++: `const std::map<std::string, std::pair<std::vector<std::string>, std::vector<std::string>>> & RONet::getDistricts() const` [const]
    /// Retrieves all TAZ (districts) from the network
    // const std::map<std::string, std::pair<std::vector<std::string>, std::vector<std::string> > >& getDistricts() const {
    //         return myDistricts;
    //     }
    #[allow(unused_variables)]
    pub fn get_districts(&self) -> &BTreeMap<String, (Vec<String>, Vec<String>)> {
        &self.myDistricts
    }

    /// C++: `ROEdge * RONet::getEdge(const std::string &) const` [const]
    /// Retrieves an edge from the network
    // ROEdge* getEdge(const std::string& name) const {
    //         return myEdges.get(name);
    //     }
    #[allow(unused_variables)]
    pub fn get_edge(&self, name: &str) -> Ptr<ROEdge> {
        self.myEdges.get(name).copied().unwrap_or(Ptr::null())
    }

    /// C++: `ROEdge * RONet::getEdgeForLaneID(const std::string &) const` [const]
    /// Retrieves an edge from the network when the lane id is given
    // ROEdge*
    // RONet::getEdgeForLaneID(const std::string& laneID) const {
    //     return getEdge(SUMOXMLDefinitions::getEdgeIDFromLane(laneID));
    // }
    #[allow(unused_variables)]
    pub fn get_edge_for_lane_id(&self, laneID: &str) -> Ptr<ROEdge> {
        self.get_edge(&utils::xml::sumo_xml_definitions::get_edge_id_from_lane(laneID))
    }

    /// C++: `ROLane * RONet::getLane(const std::string &) const` [const]
    /// Retrieves a lane rom the network given its id
    // ROLane*
    // RONet::getLane(const std::string& laneID) const {
    //     int laneIndex = SUMOXMLDefinitions::getIndexFromLane(laneID);
    //     return getEdgeForLaneID(laneID)->getLanes()[laneIndex];
    // }
    #[allow(unused_variables)]
    pub fn get_lane(&self, laneID: &str) -> Ptr<ROLane> {
        let laneIndex = utils::xml::sumo_xml_definitions::get_index_from_lane(laneID).unwrap_or_else(|e| panic!("{e}"));
        self.get_edge_for_lane_id(laneID).get_lanes()[laneIndex as usize]
    }

    /// C++: `void RONet::addNode(RONode *)`
    // void
    // RONet::addNode(RONode* node) {
    //     if (!myNodes.add(node->getID(), node)) {
    //         WRITE_ERRORF(TL("The node '%' occurs at least twice."), node->getID());
    //         delete node;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_node(&mut self, node: Ptr<RONode>) {
        let id = node.get_id().to_string();
        if self.myNodes.contains_key(&id) {
            write_errorf!("The node '%' occurs at least twice.", id);
            node.m().drop_();
            drop(unsafe { Box::from_raw(node.as_raw()) });
        } else {
            self.myNodes.insert(id, node);
        }
    }

    /// C++: `RONode * RONet::getNode(const std::string &) const` [const]
    /// Retrieves an node from the network
    // RONode* getNode(const std::string& id) const {
    //         return myNodes.get(id);
    //     }
    #[allow(unused_variables)]
    pub fn get_node(&self, id: &str) -> Ptr<RONode> {
        self.myNodes.get(id).copied().unwrap_or(Ptr::null())
    }

    /// C++: `void RONet::addStoppingPlace(const std::string &, const SumoXMLTag, SUMOVehicleParameter::Stop *)`
    // void
    // RONet::addStoppingPlace(const std::string& id, const SumoXMLTag category, SUMOVehicleParameter::Stop* stop) {
    //     if (!myStoppingPlaces[category == SUMO_TAG_TRAIN_STOP ? SUMO_TAG_BUS_STOP : category].add(id, stop)) {
    //         WRITE_ERRORF(TL("The % '%' occurs at least twice."), toString(category), id);
    //         delete stop;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_stopping_place(&mut self, id: &str, category: SumoXMLTag, stop: Ptr<SUMOVehicleParameter_Stop>) {
        // C++ maps train stops onto the bus stop category
        let cat = if category == SUMO_TAG_TRAIN_STOP { SUMO_TAG_BUS_STOP } else { category };
        let places = self.myStoppingPlaces.entry(cat).or_default();
        if places.contains_key(id) {
            write_errorf!("The % '%' occurs at least twice.", utils::xml::sumo_xml_definitions::Tags.get_string(category), id);
            drop(unsafe { Box::from_raw(stop.as_raw()) });
        } else {
            places.insert(id.to_string(), stop);
        }
    }

    /// C++: `const SUMOVehicleParameter::Stop * RONet::getStoppingPlace(const std::string &, const SumoXMLTag) const` [const]
    /// Retrieves a stopping place from the network
    // const SUMOVehicleParameter::Stop* getStoppingPlace(const std::string& id, const SumoXMLTag category) const {
    //         if (myStoppingPlaces.count(category) > 0) {
    //             return myStoppingPlaces.find(category)->second.get(id);
    //         }
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn get_stopping_place(&self, id: &str, category: SumoXMLTag) -> Ptr<SUMOVehicleParameter_Stop> {
        self.myStoppingPlaces.get(&category).and_then(|m| m.get(id)).copied().unwrap_or(Ptr::null())
    }

    /// C++: `const std::string RONet::getStoppingPlaceName(const std::string &) const` [const]
    /// return the name for the given stopping place id
    // const std::string
    // RONet::getStoppingPlaceName(const std::string& id) const {
    //     for (const auto& mapItem : myStoppingPlaces) {
    //         SUMOVehicleParameter::Stop* stop = mapItem.second.get(id);
    //         if (stop != nullptr) {
    //             // see RONetHandler::parseStoppingPlace
    //             return stop->busstop;
    //         }
    //     }
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_place_name(&self, id: &str) -> String {
        for (_, places) in self.myStoppingPlaces.iter() {
            if let Some(stop) = places.get(id) {
                // see RONetHandler::parseStoppingPlace
                return stop.busstop.clone();
            }
        }
        String::new()
    }

    /// C++: `const std::string RONet::getStoppingPlaceElement(const std::string &) const` [const]
    /// return the element name for the given stopping place id
    // const std::string
    // RONet::getStoppingPlaceElement(const std::string& id) const {
    //     for (const auto& mapItem : myStoppingPlaces) {
    //         SUMOVehicleParameter::Stop* stop = mapItem.second.get(id);
    //         if (stop != nullptr) {
    //             // see RONetHandler::parseStoppingPlace
    //             return stop->actType;
    //         }
    //     }
    //     return toString(SUMO_TAG_BUS_STOP);
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_place_element(&self, id: &str) -> String {
        for (_, places) in self.myStoppingPlaces.iter() {
            if let Some(stop) = places.get(id) {
                // see RONetHandler::parseStoppingPlace
                return stop.actType.clone();
            }
        }
        utils::xml::sumo_xml_definitions::Tags.get_string(SUMO_TAG_BUS_STOP).to_string()
    }

    /// C++: `bool RONet::checkVType(const std::string &)`
    /// Checks whether the vehicle type (distribution) may be added
    // bool
    // RONet::checkVType(const std::string& id) {
    //     if (id == DEFAULT_VTYPE_ID) {
    //         if (myDefaultVTypeMayBeDeleted) {
    //             myVehicleTypes.remove(id);
    //             myDefaultVTypeMayBeDeleted = false;
    //         } else {
    //             return false;
    //         }
    //     } else if (id == DEFAULT_PEDTYPE_ID) {
    //         if (myDefaultPedTypeMayBeDeleted) {
    //             myVehicleTypes.remove(id);
    //             myDefaultPedTypeMayBeDeleted = false;
    //         } else {
    //             return false;
    //         }
    //     } else if (id == DEFAULT_BIKETYPE_ID) {
    //         if (myDefaultBikeTypeMayBeDeleted) {
    //             myVehicleTypes.remove(id);
    //             myDefaultBikeTypeMayBeDeleted = false;
    //         } else {
    //             return false;
    //         }
    //     } else if (id == DEFAULT_TAXITYPE_ID) {
    //         if (myDefaultTaxiTypeMayBeDeleted) {
    //             myVehicleTypes.remove(id);
    //             myDefaultTaxiTypeMayBeDeleted = false;
    //         } else {
    //             return false;
    //         }
    //     } else if (id == DEFAULT_RAILTYPE_ID) {
    //         if (myDefaultRailTypeMayBeDeleted) {
    //             myVehicleTypes.remove(id);
    //             myDefaultRailTypeMayBeDeleted = false;
    //         } else {
    //             return false;
    //         }
    //     } else {
    //         if (myVehicleTypes.get(id) != 0 || myVTypeDistDict.find(id) != myVTypeDistDict.end()) {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn check_v_type(&mut self, id: &str) -> bool {
        if id == DEFAULT_VTYPE_ID {
            if self.myDefaultVTypeMayBeDeleted {
                self.myVehicleTypes.remove(id);
                self.myDefaultVTypeMayBeDeleted = false;
            } else {
                return false;
            }
        } else if id == DEFAULT_PEDTYPE_ID {
            if self.myDefaultPedTypeMayBeDeleted {
                self.myVehicleTypes.remove(id);
                self.myDefaultPedTypeMayBeDeleted = false;
            } else {
                return false;
            }
        } else if id == DEFAULT_BIKETYPE_ID {
            if self.myDefaultBikeTypeMayBeDeleted {
                self.myVehicleTypes.remove(id);
                self.myDefaultBikeTypeMayBeDeleted = false;
            } else {
                return false;
            }
        } else if id == DEFAULT_TAXITYPE_ID {
            if self.myDefaultTaxiTypeMayBeDeleted {
                self.myVehicleTypes.remove(id);
                self.myDefaultTaxiTypeMayBeDeleted = false;
            } else {
                return false;
            }
        } else if id == DEFAULT_RAILTYPE_ID {
            if self.myDefaultRailTypeMayBeDeleted {
                self.myVehicleTypes.remove(id);
                self.myDefaultRailTypeMayBeDeleted = false;
            } else {
                return false;
            }
        } else if self.myVehicleTypes.contains_key(id) || self.myVTypeDistDict.contains_key(id) {
            return false;
        }
        true
    }

    /// C++: `bool RONet::addVehicleType(SUMOVTypeParameter *)` [virtual]
    /// Adds a read vehicle type definition to the network
    // bool
    // RONet::addVehicleType(SUMOVTypeParameter* type) {
    //     if (checkVType(type->id)) {
    //         myVehicleTypes.add(type->id, type);
    //     } else {
    //         WRITE_ERRORF(TL("The vehicle type '%' occurs at least twice."), type->id);
    //         delete type;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_vehicle_type(&mut self, type_: Ptr<SUMOVTypeParameter>) -> bool {
        let id = type_.id.clone();
        if self.check_v_type(&id) {
            self.myVehicleTypes.insert(id, type_);
        } else {
            write_errorf!("The vehicle type '%' occurs at least twice.", id);
            drop(unsafe { Box::from_raw(type_.as_raw()) });
            return false;
        }
        true
    }

    /// C++: `bool RONet::addVTypeDistribution(const std::string &, RandomDistributor<SUMOVTypeParameter *> *)`
    /// Adds a vehicle type distribution
    // bool
    // RONet::addVTypeDistribution(const std::string& id, RandomDistributor<SUMOVTypeParameter*>* vehTypeDistribution) {
    //     if (checkVType(id)) {
    //         myVTypeDistDict[id] = vehTypeDistribution;
    //         return true;
    //     }
    //     delete vehTypeDistribution;
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add_v_type_distribution(&mut self, id: &str, vehTypeDistribution: Ptr<RandomDistributor<Ptr<SUMOVTypeParameter>>>) -> bool {
        if self.check_v_type(id) {
            self.myVTypeDistDict.insert(id.to_string(), vehTypeDistribution);
            return true;
        }
        drop(unsafe { Box::from_raw(vehTypeDistribution.as_raw()) });
        false
    }

    /// C++: `const RandomDistributor<SUMOVTypeParameter *> * RONet::getVTypeDistribution(const std::string &)`
    /// Retrieves the named vehicle type distribution
    // const RandomDistributor<SUMOVTypeParameter*>* getVTypeDistribution(const std::string& id) {
    //         const auto it = myVTypeDistDict.find(id);
    //         return it != myVTypeDistDict.end() ? it ->second : nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_v_type_distribution(&mut self, id: &str) -> Ptr<RandomDistributor<Ptr<SUMOVTypeParameter>>> {
        self.myVTypeDistDict.get(id).copied().unwrap_or(Ptr::null())
    }

    /// C++: `SUMOVTypeParameter * RONet::getVehicleTypeSecure(const std::string &)`
    /// Retrieves the named vehicle type
    // SUMOVTypeParameter*
    // RONet::getVehicleTypeSecure(const std::string& id) {
    //     // check whether the type was already known
    //     SUMOVTypeParameter* type = myVehicleTypes.get(id);
    //     if (id == DEFAULT_VTYPE_ID) {
    //         myDefaultVTypeMayBeDeleted = false;
    //     } else if (id == DEFAULT_PEDTYPE_ID) {
    //         myDefaultPedTypeMayBeDeleted = false;
    //     } else if (id == DEFAULT_BIKETYPE_ID) {
    //         myDefaultBikeTypeMayBeDeleted = false;
    //     } else if (id == DEFAULT_TAXITYPE_ID) {
    //         myDefaultTaxiTypeMayBeDeleted = false;
    //     } else if (id == DEFAULT_RAILTYPE_ID) {
    //         myDefaultRailTypeMayBeDeleted = false;
    //     }
    //     if (type != nullptr) {
    //         return type;
    //     }
    //     VTypeDistDictType::iterator it2 = myVTypeDistDict.find(id);
    //     if (it2 != myVTypeDistDict.end()) {
    //         return it2->second->get();
    //     }
    //     if (id == "") {
    //         // ok, no vehicle type or an unknown type was given within the user input
    //         //  return the default type
    //         myDefaultVTypeMayBeDeleted = false;
    //         return myVehicleTypes.get(DEFAULT_VTYPE_ID);
    //     }
    //     return type;
    // }
    #[allow(unused_variables)]
    pub fn get_vehicle_type_secure(&mut self, id: &str) -> Ptr<SUMOVTypeParameter> {
        // check whether the type was already known
        let ty = self.myVehicleTypes.get(id).copied().unwrap_or(Ptr::null());
        if id == DEFAULT_VTYPE_ID {
            self.myDefaultVTypeMayBeDeleted = false;
        } else if id == DEFAULT_PEDTYPE_ID {
            self.myDefaultPedTypeMayBeDeleted = false;
        } else if id == DEFAULT_BIKETYPE_ID {
            self.myDefaultBikeTypeMayBeDeleted = false;
        } else if id == DEFAULT_TAXITYPE_ID {
            self.myDefaultTaxiTypeMayBeDeleted = false;
        } else if id == DEFAULT_RAILTYPE_ID {
            self.myDefaultRailTypeMayBeDeleted = false;
        }
        if !ty.is_null() {
            return ty;
        }
        if let Some(dist) = self.myVTypeDistDict.get(id).copied() {
            return (*dist).get(None).unwrap_or_else(|e| panic!("{e}"));  // explicit deref: Ptr::get would shadow RandomDistributor::get
        }
        if id == "" {
            // ok, no vehicle type or an unknown type was given within the user input
            //  return the default type
            self.myDefaultVTypeMayBeDeleted = false;
            return self.myVehicleTypes.get(DEFAULT_VTYPE_ID).copied().unwrap_or(Ptr::null());
        }
        ty
    }

    /// C++: `bool RONet::addRouteDef(RORouteDef *)`
    // bool
    // RONet::addRouteDef(RORouteDef* def) {
    //     return myRoutes.add(def->getID(), def);
    // }
    #[allow(unused_variables)]
    pub fn add_route_def(&mut self, def: Ptr<RORouteDef>) -> bool {
        let id = def.get_id().to_string();
        if self.myRoutes.contains_key(&id) {
            return false;
        }
        self.myRoutes.insert(id, def);
        true
    }

    /// C++: `RORouteDef * RONet::getRouteDef(const std::string &) const` [const]
    /// Returns the named route definition
    // RORouteDef* getRouteDef(const std::string& name) const {
    //         return myRoutes.get(name);
    //     }
    #[allow(unused_variables)]
    pub fn get_route_def(&self, name: &str) -> Ptr<RORouteDef> {
        self.myRoutes.get(name).copied().unwrap_or(Ptr::null())
    }

    /// C++: `bool RONet::addVehicle(const std::string &, ROVehicle *)` [virtual]
    // bool
    // RONet::addVehicle(const std::string& id, ROVehicle* veh) {
    //     if (myVehIDs.find(id) == myVehIDs.end()) {
    //         myVehIDs[id] = veh->getParameter().departProcedure == DepartDefinition::TRIGGERED ? -1 : veh->getDepartureTime();
    //
    //         if (veh->isPublicTransport()) {
    //             if (!veh->isPartOfFlow()) {
    //                 myPTVehicles.push_back(veh);
    //             }
    //             if (!myDoPTRouting) {
    //                 return true;
    //             }
    //         }
    //         myRoutables[veh->getDepart()].push_back(veh);
    //         return true;
    //     }
    //     WRITE_ERRORF(TL("Another vehicle with the id '%' exists."), id);
    //     delete veh;
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add_vehicle(&mut self, id: &str, veh: Ptr<ROVehicle>) -> bool {
        if !self.myVehIDs.contains_key(id) {
            let dep = if veh.myParameter.departProcedure == DepartDefinition::TRIGGERED { -1 } else { veh.get_departure_time() };
            self.myVehIDs.insert(id.to_string(), dep);

            if veh.is_public_transport() {
                if !veh.is_part_of_flow() {
                    self.myPTVehicles.push(Ptr::new(&veh.base));
                }
                if !self.myDoPTRouting {
                    return true;
                }
            }
            self.myRoutables.entry(veh.get_depart()).or_default().push(Ptr::new(&veh.base));
            return true;
        }
        write_errorf!("Another vehicle with the id '%' exists.", id);
        drop(unsafe { Box::from_raw(veh.as_raw()) });
        false
    }

    /// C++: `bool RONet::knowsVehicle(const std::string &) const` [const]
    /// returns whether a vehicle with the given id was already loaded
    // bool
    // RONet::knowsVehicle(const std::string& id) const {
    //     return myVehIDs.find(id) != myVehIDs.end();
    // }
    #[allow(unused_variables)]
    pub fn knows_vehicle(&self, id: &str) -> bool {
        self.myVehIDs.contains_key(id)
    }

    /// C++: `bool RONet::knowsVType(const std::string &) const` [const]
    /// returns whether a vehicle type or distribution with the given id was already loaded
    // bool
    // RONet::knowsVType(const std::string& id) const {
    //     return myVehicleTypes.get(id) != nullptr || myVTypeDistDict.find(id) != myVTypeDistDict.end();
    // }
    #[allow(unused_variables)]
    pub fn knows_v_type(&self, id: &str) -> bool {
        self.myVehicleTypes.contains_key(id) || self.myVTypeDistDict.contains_key(id)
    }

    /// C++: `SUMOTime RONet::getDeparture(const std::string &) const` [const]
    /// returns departure time for the given vehicle id
    // SUMOTime
    // RONet::getDeparture(const std::string& vehID) const {
    //     auto it = myVehIDs.find(vehID);
    //     if (it != myVehIDs.end()) {
    //         return it->second;
    //     } else {
    //         throw ProcessError(TLF("Requesting departure time for unknown vehicle '%'", vehID));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_departure(&self, vehID: &str) -> SUMOTime {
        match self.myVehIDs.get(vehID) {
            Some(d) => *d,
            None => panic!("Requesting departure time for unknown vehicle '{}'", vehID),
        }
    }

    /// C++: `bool RONet::addFlow(SUMOVehicleParameter *, const bool)`
    // bool
    // RONet::addFlow(SUMOVehicleParameter* flow, const bool randomize) {
    //     if (randomize && flow->repetitionOffset >= 0) {
    //         myDepartures[flow->id].reserve(flow->repetitionNumber);
    //         for (int i = 0; i < flow->repetitionNumber; ++i) {
    //             myDepartures[flow->id].push_back(flow->depart + RandHelper::rand(flow->repetitionNumber * flow->repetitionOffset));
    //         }
    //         std::sort(myDepartures[flow->id].begin(), myDepartures[flow->id].end());
    //         std::reverse(myDepartures[flow->id].begin(), myDepartures[flow->id].end());
    //     } else if (flow->repetitionOffset < 0) {
    //         // init poisson flow (but only the timing)
    //         flow->incrementFlow(1);
    //         flow->repetitionsDone--;
    //     }
    //     const bool added = myFlows.add(flow->id, flow);
    //     if (added) {
    //         myHaveActiveFlows = true;
    //     }
    //     return added;
    // }
    #[allow(unused_variables)]
    pub fn add_flow(&mut self, flow: Ptr<SUMOVehicleParameter>, randomize: bool) -> bool {
        let id = flow.id.clone();
        if randomize && flow.repetitionOffset >= 0 {
            let maxV = (flow.repetitionNumber * flow.repetitionOffset) as f64;
            let mut deps: Vec<SUMOTime> = Vec::new();
            for _ in 0..flow.repetitionNumber {
                let r = utils::common::rand_helper::with_default(|rng| rng.rand_max(maxV));
                deps.push((flow.depart as f64 + r) as SUMOTime);
            }
            let v = self.myDepartures.entry(id.clone()).or_default();
            v.extend(deps);
            v.sort();
            v.reverse();
        } else if flow.repetitionOffset < 0 {
            // init poisson flow (but only the timing)
            utils::common::rand_helper::with_default(|rng| flow.m().increment_flow(1., Ptr::from_mut(rng)));
            flow.m().repetitionsDone -= 1;
        }
        let added = !self.myFlows.contains_key(&id);
        if added {
            self.myFlows.insert(id, flow);
            self.myHaveActiveFlows = true;
        }
        added
    }

    /// C++: `bool RONet::addPerson(ROPerson *)`
    // bool
    // RONet::addPerson(ROPerson* person) {
    //     if (myPersonIDs.count(person->getID()) == 0) {
    //         myPersonIDs.insert(person->getID());
    //         myRoutables[person->getDepart()].push_back(person);
    //         return true;
    //     }
    //     WRITE_ERRORF(TL("Another person with the id '%' exists."), person->getID());
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn add_person(&mut self, person: Ptr<ROPerson>) -> bool {
        let id = person.get_id().to_string();
        if !self.myPersonIDs.contains(&id) {
            self.myPersonIDs.insert(id);
            self.myRoutables.entry(person.get_depart()).or_default().push(Ptr::new(&person.base));
            return true;
        }
        write_errorf!("Another person with the id '%' exists.", id);
        false
    }

    /// C++: `void RONet::addContainer(const SUMOTime, const std::string)`
    // void
    // RONet::addContainer(const SUMOTime depart, const std::string desc) {
    //     myContainers.insert(std::pair<const SUMOTime, const std::string>(depart, desc));
    // }
    #[allow(unused_variables)]
    pub fn add_container(&mut self, depart: SUMOTime, desc: String) {
        self.myContainers.entry(depart).or_default().push(desc);
    }

    /// C++: `SUMOTime RONet::saveAndRemoveRoutesUntil(OptionsCont &, const RORouterProvider &, SUMOTime)`
    /// Computes routes described by their definitions and saves them
    // SUMOTime
    // RONet::saveAndRemoveRoutesUntil(OptionsCont& options, const RORouterProvider& provider,
    //                                 SUMOTime time) {
    //     MsgHandler* mh = (options.getBool("ignore-errors") ?
    //                       MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance());
    //     if (myHaveActiveFlows) {
    //         checkFlows(time, mh);
    //     }
    //     SUMOTime lastTime = -1;
    //     const bool removeLoops = options.getBool("remove-loops");
    // #ifdef HAVE_FOX
    //     const int maxNumThreads = options.getInt("routing-threads");
    // #endif
    //     if (myRoutables.size() != 0) {
    //         if (options.getBool("bulk-routing")) {
    // #ifdef HAVE_FOX
    //             while ((int)myThreadPool.size() < maxNumThreads) {
    //                 new WorkerThread(myThreadPool, provider);
    //             }
    // #endif
    //             createBulkRouteRequests(provider, time, removeLoops);
    //         } else {
    //             for (RoutablesMap::const_iterator i = myRoutables.begin(); i != myRoutables.end(); ++i) {
    //                 if (i->first >= time) {
    //                     break;
    //                 }
    //                 for (RORoutable* const routable : i->second) {
    // #ifdef HAVE_FOX
    //                     // add task
    //                     if (maxNumThreads > 0) {
    //                         const int numThreads = (int)myThreadPool.size();
    //                         if (numThreads == 0) {
    //                             // This is the very first routing. Since at least the CHRouter needs initialization
    //                             // before it gets cloned, we do not do this in parallel
    //                             routable->computeRoute(provider, removeLoops, myErrorHandler);
    //                             new WorkerThread(myThreadPool, provider);
    //                         } else {
    //                             // add thread if necessary
    //                             if (numThreads < maxNumThreads && myThreadPool.isFull()) {
    //                                 new WorkerThread(myThreadPool, provider);
    //                             }
    //                             myThreadPool.add(new RoutingTask(routable, removeLoops, myErrorHandler));
    //                         }
    //                         continue;
    //                     }
    // #endif
    //                     routable->computeRoute(provider, removeLoops, myErrorHandler);
    //                 }
    //             }
    //         }
    // #ifdef HAVE_FOX
    //         myThreadPool.waitAll();
    // #endif
    //     }
    //     const double scale = options.exists("scale-suffix") ? options.getFloat("scale") : 1;
    //     // write all vehicles (and additional structures)
    //     while (myRoutables.size() != 0 || myContainers.size() != 0) {
    //         // get the next vehicle, person or container
    //         RoutablesMap::iterator routables = myRoutables.begin();
    //         const SUMOTime routableTime = routables == myRoutables.end() ? SUMOTime_MAX : routables->first;
    //         ContainerMap::iterator container = myContainers.begin();
    //         const SUMOTime containerTime = container == myContainers.end() ? SUMOTime_MAX : container->first;
    //         // check whether it shall not yet be computed
    //         if (routableTime >= time && containerTime >= time) {
    //             lastTime = MIN2(routableTime, containerTime);
    //             break;
    //         }
    //         const SUMOTime minTime = MIN2(routableTime, containerTime);
    //         myHaveProcessedRoutes = true;
    //         if (routableTime == minTime) {
    //             // check whether to print the output
    //             if (lastTime != routableTime && lastTime != -1) {
    //                 // report writing progress
    //                 if (options.getInt("stats-period") >= 0 && ((int)routableTime % options.getInt("stats-period")) == 0) {
    //                     WRITE_MESSAGE("Read: " + toString(myVehIDs.size()) + ",  Discarded: " + toString(myDiscardedRouteNo) + ",  Written: " + toString(myWrittenRouteNo));
    //                 }
    //             }
    //             lastTime = routableTime;
    //             for (const RORoutable* const r : routables->second) {
    //                 // ok, check whether it has been routed
    //                 if (r->getRoutingSuccess()) {
    //                     // write the route
    //                     int quota = getScalingQuota(scale, myWrittenRouteNo);
    //                     r->write(myRoutesOutput, myRouteAlternativesOutput, myTypesOutput, options, quota);
    //                     myWrittenRouteNo++;
    //                 } else {
    //                     myDiscardedRouteNo++;
    //                 }
    //                 // we need to keep individual public transport vehicles but not the flows
    //                 if (!r->isPublicTransport() || r->isPartOfFlow()) {
    //                     // delete routes and the vehicle
    //                     const ROVehicle* const veh = dynamic_cast<const ROVehicle*>(r);
    //                     if (veh != nullptr && veh->getRouteDefinition()->getID()[0] == '!') {
    //                         if (r->isPartOfFlow() || !myRoutes.remove(veh->getRouteDefinition()->getID())) {
    //                             delete veh->getRouteDefinition();
    //                         }
    //                     }
    //                     delete r;
    //                 }
    //             }
    //             myRoutables.erase(routables);
    //         }
    //         if (containerTime == minTime) {
    //             myRoutesOutput->writePreformattedTag(container->second);
    //             if (myRouteAlternativesOutput != nullptr) {
    //                 myRouteAlternativesOutput->writePreformattedTag(container->second);
    //             }
    //             myContainers.erase(container);
    //         }
    //     }
    //     return lastTime;
    // }
    #[allow(unused_variables)]
    pub fn save_and_remove_routes_until(&mut self, options: &mut OptionsCont, provider: &RORouterProvider, time: SUMOTime) -> SUMOTime {
        let mh = msg_instance(ob(options, "ignore-errors"));
        if self.myHaveActiveFlows {
            self.check_flows(time, mh);
        }
        let mut lastTime: SUMOTime = -1;
        let removeLoops = ob(options, "remove-loops");
        // shortcut: the HAVE_FOX thread pool is not ported, routing is sequential as in a build without Fox
        if !self.myRoutables.is_empty() {
            if ob(options, "bulk-routing") {
                self.create_bulk_route_requests(provider, time, removeLoops);
            } else {
                let keys: Vec<SUMOTime> = self.myRoutables.keys().copied().collect();
                for k in keys {
                    if k >= time {
                        break;
                    }
                    let list = self.myRoutables.get(&k).cloned().unwrap_or_default();
                    for routable in list {
                        routable.m().compute_route(provider, removeLoops, self.myErrorHandler);
                    }
                }
            }
        }
        let scale = if oexists_in(options, "scale-suffix") { ofloat(options, "scale") } else { 1. };
        // write all vehicles (and additional structures)
        while !self.myRoutables.is_empty() || !self.myContainers.is_empty() {
            // get the next vehicle, person or container
            let routableTime = self.myRoutables.keys().next().copied().unwrap_or(SUMOTIME_MAX);
            let containerTime = self.myContainers.keys().next().copied().unwrap_or(SUMOTIME_MAX);
            // check whether it shall not yet be computed
            if routableTime >= time && containerTime >= time {
                lastTime = min2i(routableTime, containerTime);
                break;
            }
            let minTime = min2i(routableTime, containerTime);
            self.myHaveProcessedRoutes = true;
            if routableTime == minTime {
                // check whether to print the output
                if lastTime != routableTime && lastTime != -1 {
                    // report writing progress
                    let period = oint(options, "stats-period");
                    // shortcut: C++ takes % with the period unguarded, a period of 0 is skipped here instead of being undefined
                    if period >= 0 && period != 0 && ((routableTime as i32) % period) == 0 {
                        utils::common::msg_handler::write_message(&format!("Read: {},  Discarded: {},  Written: {}", self.myVehIDs.len(), self.myDiscardedRouteNo, self.myWrittenRouteNo));
                    }
                }
                lastTime = routableTime;
                let list = self.myRoutables.get(&routableTime).cloned().unwrap_or_default();
                for r in list.iter() {
                    // ok, check whether it has been routed
                    if r.get_routing_success() {
                        // write the route
                        let quota = utils::common::std_defs::get_scaling_quota(scale, self.myWrittenRouteNo as i64);
                        r.write(self.myRoutesOutput, self.myRouteAlternativesOutput, self.myTypesOutput, options, quota);
                        self.myWrittenRouteNo += 1;
                    } else {
                        self.myDiscardedRouteNo += 1;
                    }
                    // shortcut: C++ deletes the routable and its '!' route definition here after a dynamic_cast to ROVehicle; RORoutable has no type tag, so they are not freed (memory only, no output effect)
                }
                self.myRoutables.remove(&routableTime);
            }
            if containerTime == minTime {
                let desc = {
                    let v = self.myContainers.get_mut(&containerTime).expect("container time exists");
                    v.remove(0)
                };
                if self.myContainers.get(&containerTime).map_or(false, |v| v.is_empty()) {
                    self.myContainers.remove(&containerTime);
                }
                self.myRoutesOutput.m().write_preformatted_tag(&desc);
                if !self.myRouteAlternativesOutput.is_null() {
                    self.myRouteAlternativesOutput.m().write_preformatted_tag(&desc);
                }
            }
        }
        lastTime
    }

    /// C++: `bool RONet::furtherStored()`
    /// Returns the information whether further vehicles, persons or containers are stored
    // bool
    // RONet::furtherStored() {
    //     return myRoutables.size() > 0 || (myFlows.size() > 0 && myHaveActiveFlows) || myContainers.size() > 0;
    // }
    #[allow(unused_variables)]
    pub fn further_stored(&mut self) -> bool {
        !self.myRoutables.is_empty() || (!self.myFlows.is_empty() && self.myHaveActiveFlows) || !self.myContainers.is_empty()
    }

    /// C++: `void RONet::routeSkipped()`
    /// Record routes excluded by the begin time.
    // void routeSkipped() {
    //         myHaveSkippedRoutes = true;
    //     }
    #[allow(unused_variables)]
    pub fn route_skipped(&mut self) {
        self.myHaveSkippedRoutes = true;
    }

    /// C++: `bool RONet::haveSkippedRoutes() const` [const]
    // bool haveSkippedRoutes() const {
    //         return myHaveSkippedRoutes;
    //     }
    #[allow(unused_variables)]
    pub fn have_skipped_routes(&self) -> bool {
        self.myHaveSkippedRoutes
    }

    /// C++: `bool RONet::haveProcessedRoutes() const` [const]
    // bool haveProcessedRoutes() const {
    //         return myHaveProcessedRoutes;
    //     }
    #[allow(unused_variables)]
    pub fn have_processed_routes(&self) -> bool {
        self.myHaveProcessedRoutes
    }

    /// C++: `void RONet::openOutput(const OptionsCont &)`
    /// Opens the output for computed routes
    // void
    // RONet::openOutput(const OptionsCont& options) {
    //     if (options.isSet("output-file") && options.getString("output-file") != "") {
    //         myRoutesOutput = &OutputDevice::getDevice(options.getString("output-file"));
    //         if (myRoutesOutput->isNull()) {
    //             myRoutesOutput = nullptr;
    //         } else {
    //             myRoutesOutput->writeXMLHeader("routes", "routes_file.xsd");
    //         }
    //     }
    //     if (options.exists("alternatives-output") && options.isSet("alternatives-output")
    //             && !(options.exists("write-trips") && options.getBool("write-trips"))) {
    //         myRouteAlternativesOutput = &OutputDevice::getDevice(options.getString("alternatives-output"));
    //         if (myRouteAlternativesOutput->isNull()) {
    //             myRouteAlternativesOutput = nullptr;
    //         } else {
    //             myRouteAlternativesOutput->writeXMLHeader("routes", "routes_file.xsd");
    //         }
    //     }
    //     if (options.isSet("vtype-output")) {
    //         myTypesOutput = &OutputDevice::getDevice(options.getString("vtype-output"));
    //         myTypesOutput->writeXMLHeader("routes", "routes_file.xsd");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn open_output(&mut self, options: &OptionsCont) {
        if oset(options, "output-file") && ostring(options, "output-file") != "" {
            self.myRoutesOutput = get_device_ptr(&ostring(options, "output-file"));
            if self.myRoutesOutput.m().is_null() {
                self.myRoutesOutput = Ptr::null();
            } else {
                self.myRoutesOutput.m().write_xml_header("routes", "routes_file.xsd", BTreeMap::new(), true);
            }
        }
        if oexists_in(options, "alternatives-output") && oset(options, "alternatives-output") && !(oexists_in(options, "write-trips") && ob(options, "write-trips")) {
            self.myRouteAlternativesOutput = get_device_ptr(&ostring(options, "alternatives-output"));
            if self.myRouteAlternativesOutput.m().is_null() {
                self.myRouteAlternativesOutput = Ptr::null();
            } else {
                self.myRouteAlternativesOutput.m().write_xml_header("routes", "routes_file.xsd", BTreeMap::new(), true);
            }
        }
        if oset(options, "vtype-output") {
            self.myTypesOutput = get_device_ptr(&ostring(options, "vtype-output"));
            self.myTypesOutput.m().write_xml_header("routes", "routes_file.xsd", BTreeMap::new(), true);
        }
    }

    /// C++: `void RONet::writeIntermodal(const OptionsCont &, ROIntermodalRouter &) const` [const]
    /// Writes the intermodal network and weights if requested
    // void
    // RONet::writeIntermodal(const OptionsCont& options, ROIntermodalRouter& router) const {
    //     if (options.exists("intermodal-network-output") && options.isSet("intermodal-network-output")) {
    //         OutputDevice::createDeviceByOption("intermodal-network-output", "intermodal");
    //         router.writeNetwork(OutputDevice::getDevice(options.getString("intermodal-network-output")));
    //     }
    //     if (options.exists("intermodal-weight-output") && options.isSet("intermodal-weight-output")) {
    //         OutputDevice::createDeviceByOption("intermodal-weight-output", "weights", "meandata_file.xsd");
    //         OutputDevice& dev = OutputDevice::getDeviceByOption("intermodal-weight-output");
    //         dev.openTag(SUMO_TAG_INTERVAL);
    //         dev.writeAttr(SUMO_ATTR_ID, "intermodalweights");
    //         dev.writeAttr(SUMO_ATTR_BEGIN, 0);
    //         dev.writeAttr(SUMO_ATTR_END, SUMOTime_MAX);
    //         router.writeWeights(dev);
    //         dev.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_intermodal(&self, options: &OptionsCont) {
        // shortcut: ROIntermodalRouter is not ported, so the network and weight files are not written; only the device creation of the option is done
        if oexists_in(options, "intermodal-network-output") && oset(options, "intermodal-network-output") {
            let _ = OutputDevice::create_device_by_option("intermodal-network-output", "intermodal", "", 0);
        }
        if oexists_in(options, "intermodal-weight-output") && oset(options, "intermodal-weight-output") {
            let _ = OutputDevice::create_device_by_option("intermodal-weight-output", "weights", "meandata_file.xsd", 0);
        }
    }

    /// C++: `void RONet::cleanup()`
    /// closes the file output for computed routes and deletes associated threads if necessary
    // void
    // RONet::cleanup() {
    //     // end writing
    //     if (myRoutesOutput != nullptr) {
    //         myRoutesOutput->close();
    //     }
    //     // only if opened
    //     if (myRouteAlternativesOutput != nullptr) {
    //         myRouteAlternativesOutput->close();
    //     }
    //     // only if opened
    //     if (myTypesOutput != nullptr) {
    //         myTypesOutput->close();
    //     }
    //     RouteCostCalculator<RORoute, ROEdge, ROVehicle>::cleanup();
    // #ifdef HAVE_FOX
    //     if (myThreadPool.size() > 0) {
    //         myThreadPool.clear();
    //     }
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn cleanup(&mut self) {
        // end writing, only if opened
        for out in [self.myRoutesOutput, self.myRouteAlternativesOutput, self.myTypesOutput] {
            if !out.is_null() {
                let _ = out.m().close();
            }
        }
        RouteCostCalculator::<RORoute, ROEdge, ROVehicle>::cleanup();
    }

    /// C++: `int RONet::getEdgeNumber() const` [const]
    /// Returns the total number of edges the network contains including internal edges
    // int
    // RONet::getEdgeNumber() const {
    //     return myEdges.size();
    // }
    #[allow(unused_variables)]
    pub fn get_edge_number(&self) -> i32 {
        self.myEdges.len() as i32
    }

    /// C++: `int RONet::getInternalEdgeNumber() const` [const]
    /// Returns the number of internal edges the network contains
    // int
    // RONet::getInternalEdgeNumber() const {
    //     return myNumInternalEdges;
    // }
    #[allow(unused_variables)]
    pub fn get_internal_edge_number(&self) -> i32 {
        self.myNumInternalEdges
    }

    /// C++: `const NamedObjectCont<ROEdge *> & RONet::getEdgeMap() const` [const]
    // const NamedObjectCont<ROEdge*>& getEdgeMap() const {
    //         return myEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge_map(&self) -> &BTreeMap<String, Ptr<ROEdge>> {
        &self.myEdges
    }

    /// C++: `void RONet::adaptIntermodalRouter(ROIntermodalRouter &)` [static]
    // void
    // RONet::adaptIntermodalRouter(ROIntermodalRouter& router) {
    //     double taxiWait = STEPS2TIME(string2time(OptionsCont::getOptions().getString("persontrip.taxi.waiting-time")));
    //     for (const auto& stopType : myInstance->myStoppingPlaces) {
    //         // add access to all stopping places
    //         const SumoXMLTag element = stopType.first;
    //         for (const auto& stop : stopType.second) {
    //             router.getNetwork()->addAccess(stop.first, myInstance->getEdgeForLaneID(stop.second->lane),
    //                                            stop.second->startPos, stop.second->endPos, 0., element, false, taxiWait);
    //             // add access to all public transport stops
    //             if (element == SUMO_TAG_BUS_STOP) {
    //                 for (const auto& a : stop.second->accessPos) {
    //                     router.getNetwork()->addAccess(stop.first, myInstance->getEdgeForLaneID(std::get<0>(a)),
    //                                                    std::get<1>(a), std::get<1>(a), std::get<2>(a), SUMO_TAG_BUS_STOP, true, taxiWait);
    //                 }
    //             }
    //         }
    //     }
    //     // fill the public transport router with pre-parsed public transport lines
    //     for (const auto& i : myInstance->myFlows) {
    //         if (i.second->line != "") {
    //             const RORouteDef* const route = myInstance->getRouteDef(i.second->routeid);
    //             const StopParVector* addStops = nullptr;
    //             if (route != nullptr && route->getFirstRoute() != nullptr) {
    //                 addStops = &route->getFirstRoute()->getStops();
    //             }
    //             router.getNetwork()->addSchedule(*i.second, addStops);
    //         }
    //     }
    //     for (const RORoutable* const veh : myInstance->myPTVehicles) {
    //         // add single vehicles with line attribute which are not part of a flow
    //         // no need to add route stops here, they have been added to the vehicle before
    //         router.getNetwork()->addSchedule(veh->getParameter());
    //     }
    //     // add access to transfer from walking to taxi-use
    //     if ((router.getCarWalkTransfer() & ModeChangeOptions::TAXI_PICKUP_ANYWHERE) != 0) {
    //         for (const ROEdge* edge : ROEdge::getAllEdges()) {
    //             if (!edge->isTazConnector() && (edge->getPermissions() & SVC_PEDESTRIAN) != 0 && (edge->getPermissions() & SVC_TAXI) != 0) {
    //                 router.getNetwork()->addCarAccess(edge, SVC_TAXI, taxiWait);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn adapt_intermodal_router() {
        // shortcut: needs ROIntermodalRouter (access, schedules and car-access setup) which is not ported yet
    }

    /// C++: `bool RONet::hasPermissions() const` [const]
    // bool
    // RONet::hasPermissions() const {
    //     return myHavePermissions;
    // }
    #[allow(unused_variables)]
    pub fn has_permissions(&self) -> bool {
        self.myHavePermissions
    }

    /// C++: `void RONet::setPermissionsFound()`
    // void
    // RONet::setPermissionsFound() {
    //     myHavePermissions = true;
    // }
    #[allow(unused_variables)]
    pub fn set_permissions_found(&mut self) {
        self.myHavePermissions = true;
    }

    /// C++: `bool RONet::hasBidiEdges() const` [const]
    /// return whether the network contains bidirectional rail edges
    // bool hasBidiEdges() const {
    //         return myHasBidiEdges;
    //     }
    #[allow(unused_variables)]
    pub fn has_bidi_edges(&self) -> bool {
        self.myHasBidiEdges
    }

    /// C++: `bool RONet::hasLoadedEffort() const` [const]
    /// whether efforts were loaded from file
    // bool
    // RONet::hasLoadedEffort() const {
    //     for (const auto& item : myEdges) {
    //         if (item.second->hasStoredEffort()) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_loaded_effort(&self) -> bool {
        self.myEdges.values().any(|e| e.has_stored_effort())
    }

    /// C++: `double RONet::getMaxTraveltime() const` [const]
    // double getMaxTraveltime() const {
    //         return myMaxTraveltime;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_traveltime(&self) -> f64 {
        self.myMaxTraveltime
    }

    /// C++: `bool RONet::hasProhibitions() const` [const]
    // bool hasProhibitions() const {
    //         return !myProhibitions.empty() || !myLaneProhibitions.empty();
    //     }
    #[allow(unused_variables)]
    pub fn has_prohibitions(&self) -> bool {
        !self.myProhibitions.is_empty() || !self.myLaneProhibitions.is_empty()
    }

    /// C++: `const Prohibitions & RONet::getProhibitions() const` [const]
    // const Prohibitions& getProhibitions() const {
    //         return myProhibitions;
    //     }
    #[allow(unused_variables)]
    pub fn get_prohibitions(&self) -> &RONet_Prohibitions {
        &self.myProhibitions
    }

    /// C++: `void RONet::updateLaneProhibitions(SUMOTime)`
    // void
    // RONet::updateLaneProhibitions(SUMOTime begin) {
    //     const double beginS = STEPS2TIME(begin);
    //     while (myLaneProhibitionTimes.size() > 0 && myLaneProhibitionTimes.begin()->first <= beginS) {
    //         const double t = myLaneProhibitionTimes.begin()->first;
    //         for (const ROLane* const lane : myLaneProhibitionTimes.begin()->second) {
    //             const SVCPermissions orig = lane->getPermissions();
    //             assert(myLaneProhibitions.count(lane) != 0);
    //             RouterProhibition& rp = myLaneProhibitions[lane];
    //             const_cast<ROLane*>(lane)->setPermissions(rp.permissions);
    //             lane->getEdge().resetSuccessors();
    //             for (ROEdge* pred : lane->getEdge().getPredecessors()) {
    //                 pred->resetSuccessors();
    //             }
    //             if (t == rp.begin) {
    //                 // schedule restoration of original permissions. This works
    //                 // without a stack because there is at most one prohibition per lane
    //                 myLaneProhibitionTimes[rp.end].insert(lane);
    //                 rp.permissions = orig;
    //             }
    //         }
    //         myLaneProhibitionTimes.erase(myLaneProhibitionTimes.begin());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_lane_prohibitions(&mut self, begin: SUMOTime) {
        let beginS = utils::common::time::steps2time(begin);
        while !self.myLaneProhibitionTimes.is_empty() && self.myLaneProhibitionTimes.keys().next().expect("non-empty").0 <= beginS {
            let (tkey, lanes) = {
                let (k, v) = self.myLaneProhibitionTimes.iter().next().expect("non-empty");
                (*k, v.clone())
            };
            let t = tkey.0;
            for lane in lanes.iter() {
                let orig = lane.get_permissions();
                let rp = self.myLaneProhibitions.get_mut(lane).expect("lane has a prohibition");
                let rpBegin = rp.begin;
                let rpEnd = rp.end;
                lane.m().set_permissions(rp.permissions);
                let edge = lane.get_edge();
                edge.reset_successors();
                for pred in edge.get_predecessors().clone() {
                    pred.m().reset_successors();
                }
                if t == rpBegin {
                    // schedule restoration of original permissions. This works
                    // without a stack because there is at most one prohibition per lane
                    self.myLaneProhibitionTimes.entry(RONet_TimeKey(rpEnd)).or_default().insert(*lane);
                    rp.permissions = orig;
                }
            }
            self.myLaneProhibitionTimes.remove(&tkey);
        }
    }

    /// C++: `void RONet::addProhibition(const ROEdge *, const RouterProhibition &)`
    // void
    // RONet::addProhibition(const ROEdge* edge, const RouterProhibition& prohibition) {
    //     if (myProhibitions.count(edge) != 0) {
    //         throw ProcessError(TLF("Already loaded prohibition for edge '%'. (Only one prohibition per edge is supported)", edge->getID()));
    //     }
    //     myProhibitions[edge] = prohibition;
    //     myHavePermissions = true;
    // }
    #[allow(unused_variables)]
    pub fn add_prohibition(&mut self, edge: Ptr<ROEdge>, prohibition: &RouterProhibition) {
        if self.myProhibitions.contains_key(&edge) {
            panic!("Already loaded prohibition for edge '{}'. (Only one prohibition per edge is supported)", edge.get_id());
        }
        self.myProhibitions.insert(edge, copy_prohibition(prohibition));
        self.myHavePermissions = true;
    }

    /// C++: `void RONet::addLaneProhibition(const ROLane *, const RouterProhibition &)`
    // void
    // RONet::addLaneProhibition(const ROLane* lane, const RouterProhibition& prohibition) {
    //     if (myLaneProhibitions.count(lane) != 0) {
    //         throw ProcessError(TLF("Already loaded prohibition for lane '%'. (Only one prohibition per lane is supported)", lane->getID()));
    //     }
    //     assert(prohibition.end > prohibition.begin);
    //     myLaneProhibitions[lane] = prohibition;
    //     myLaneProhibitionTimes[prohibition.begin].insert(lane);
    //     myHavePermissions = true;
    // }
    #[allow(unused_variables)]
    pub fn add_lane_prohibition(&mut self, lane: Ptr<ROLane>, prohibition: &RouterProhibition) {
        if self.myLaneProhibitions.contains_key(&lane) {
            panic!("Already loaded prohibition for lane '{}'. (Only one prohibition per lane is supported)", lane.get_id());
        }
        debug_assert!(prohibition.end > prohibition.begin);
        self.myLaneProhibitions.insert(lane, copy_prohibition(prohibition));
        self.myLaneProhibitionTimes.entry(RONet_TimeKey(prohibition.begin)).or_default().insert(lane);
        self.myHavePermissions = true;
    }

    /// C++: `OutputDevice * RONet::getRouteOutput(const bool)`
    // OutputDevice* getRouteOutput(const bool alternative = false) {
    //         if (alternative) {
    //             return myRouteAlternativesOutput;
    //         }
    //         return myRoutesOutput;
    //     }
    #[allow(unused_variables)]
    pub fn get_route_output(&mut self, alternative: bool) -> Ptr<OutputDevice> {
        if alternative {
            return self.myRouteAlternativesOutput;
        }
        self.myRoutesOutput
    }

    /// C++: `void RONet::checkFlows(SUMOTime, MsgHandler *)` [private]
    // void
    // RONet::checkFlows(SUMOTime time, MsgHandler* errorHandler) {
    //     myHaveActiveFlows = false;
    //     for (const auto& i : myFlows) {
    //         SUMOVehicleParameter* const pars = i.second;
    //         if (pars->line != "" && !myDoPTRouting) {
    //             continue;
    //         }
    //         if (myKeepFlows) {
    //             if (pars->repetitionsDone < pars->repetitionNumber) {
    //                 // each each flow only once
    //                 pars->repetitionsDone = pars->repetitionNumber;
    //                 const SUMOVTypeParameter* type = getVehicleTypeSecure(pars->vtypeid);
    //                 if (type == nullptr) {
    //                     type = getVehicleTypeSecure(DEFAULT_VTYPE_ID);
    //                 } else {
    //                     auto dist = getVTypeDistribution(pars->vtypeid);
    //                     if (dist != nullptr) {
    //                         WRITE_WARNINGF("Keeping flow '%' with a vTypeDistribution can lead to invalid routes if the distribution contains different vClasses", pars->id);
    //                     }
    //                 }
    //                 RORouteDef* route = getRouteDef(pars->routeid)->copy(pars->routeid, pars->depart);
    //                 ROVehicle* veh = new ROVehicle(*pars, route, type, this, errorHandler);
    //                 addVehicle(pars->id, veh);
    //             }
    //         } else if (pars->repetitionProbability > 0) {
    //             if (pars->repetitionEnd > pars->depart && pars->repetitionsDone < pars->repetitionNumber) {
    //                 myHaveActiveFlows = true;
    //             }
    //             const SUMOTime origDepart = pars->depart;
    //             while (pars->depart < time && pars->repetitionsDone < pars->repetitionNumber) {
    //                 if (pars->repetitionEnd <= pars->depart) {
    //                     break;
    //                 }
    //                 // only call rand if all other conditions are met
    //                 if (RandHelper::rand() < (pars->repetitionProbability * TS)) {
    //                     SUMOVehicleParameter* newPars = new SUMOVehicleParameter(*pars);
    //                     newPars->id = pars->id + "." + toString(pars->repetitionsDone);
    //                     newPars->depart = pars->depart;
    //                     for (StopParVector::iterator stop = newPars->stops.begin(); stop != newPars->stops.end(); ++stop) {
    //                         if (stop->until >= 0) {
    //                             stop->until += pars->depart - origDepart;
    //                         }
    //                         if (stop->arrival >= 0) {
    //                             stop->arrival += pars->depart - origDepart;
    //                         }
    //                     }
    //                     pars->repetitionsDone++;
    //                     // try to build the vehicle
    //                     const SUMOVTypeParameter* type = getVehicleTypeSecure(pars->vtypeid);
    //                     if (type == nullptr) {
    //                         type = getVehicleTypeSecure(DEFAULT_VTYPE_ID);
    //                     } else if (!myKeepVTypeDist) {
    //                         // fix the type id in case we used a distribution
    //                         newPars->vtypeid = type->id;
    //                     }
    //                     const SUMOTime stopOffset = pars->routeid[0] == '!' ? pars->depart - origDepart : pars->depart;
    //                     RORouteDef* route = getRouteDef(pars->routeid)->copy("!" + newPars->id, stopOffset);
    //                     ROVehicle* veh = new ROVehicle(*newPars, route, type, this, errorHandler);
    //                     addVehicle(newPars->id, veh);
    //                     delete newPars;
    //                 }
    //                 pars->depart += DELTA_T;
    //             }
    //         } else {
    //             SUMOTime depart = static_cast<SUMOTime>(pars->depart + pars->repetitionTotalOffset);
    //             while (pars->repetitionsDone < pars->repetitionNumber && pars->repetitionEnd >= depart) {
    //                 myHaveActiveFlows = true;
    //                 depart = static_cast<SUMOTime>(pars->depart + pars->repetitionTotalOffset);
    //                 if (myDepartures.find(pars->id) != myDepartures.end()) {
    //                     depart = myDepartures[pars->id].back();
    //                 }
    //                 if (depart >= time + DELTA_T) {
    //                     break;
    //                 }
    //                 if (myDepartures.find(pars->id) != myDepartures.end()) {
    //                     myDepartures[pars->id].pop_back();
    //                 }
    //                 SUMOVehicleParameter* newPars = new SUMOVehicleParameter(*pars);
    //                 newPars->id = pars->id + "." + toString(pars->repetitionsDone);
    //                 newPars->depart = depart;
    //                 for (StopParVector::iterator stop = newPars->stops.begin(); stop != newPars->stops.end(); ++stop) {
    //                     if (stop->until >= 0) {
    //                         stop->until += depart - pars->depart;
    //                     }
    //                     if (stop->arrival >= 0) {
    //                         stop->arrival += depart - pars->depart;
    //                     }
    //                 }
    //                 pars->incrementFlow(1);
    //                 // try to build the vehicle
    //                 const SUMOVTypeParameter* type = getVehicleTypeSecure(pars->vtypeid);
    //                 if (type == nullptr) {
    //                     type = getVehicleTypeSecure(DEFAULT_VTYPE_ID);
    //                 } else if (!myKeepVTypeDist) {
    //                     // fix the type id in case we used a distribution
    //                     newPars->vtypeid = type->id;
    //                 }
    //                 const SUMOTime stopOffset = pars->routeid[0] == '!' ? depart - pars->depart : depart;
    //                 RORouteDef* route = getRouteDef(pars->routeid)->copy("!" + newPars->id, stopOffset);
    //                 ROVehicle* veh = new ROVehicle(*newPars, route, type, this, errorHandler);
    //                 addVehicle(newPars->id, veh);
    //                 delete newPars;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_flows(&mut self, time: SUMOTime, errorHandler: Ptr<MsgHandler>) {
        self.myHaveActiveFlows = false;
        let flows: Vec<Ptr<SUMOVehicleParameter>> = self.myFlows.values().copied().collect();
        for pars in flows {
            if pars.line != "" && !self.myDoPTRouting {
                continue;
            }
            if self.myKeepFlows {
                if pars.repetitionsDone < pars.repetitionNumber {
                    // each each flow only once
                    pars.m().repetitionsDone = pars.repetitionNumber;
                    let mut ty = self.get_vehicle_type_secure(&pars.vtypeid);
                    if ty.is_null() {
                        ty = self.get_vehicle_type_secure(DEFAULT_VTYPE_ID);
                    } else {
                        let dist = self.get_v_type_distribution(&pars.vtypeid);
                        if !dist.is_null() {
                            write_warningf!("Keeping flow '%' with a vTypeDistribution can lead to invalid routes if the distribution contains different vClasses", pars.id);
                        }
                    }
                    let route = self.get_route_def(&pars.routeid).copy(&pars.routeid, pars.depart);
                    let veh = Ptr::from_raw(Box::into_raw(Box::new(ROVehicle::new(&pars, route, ty, Ptr::from_mut(self), errorHandler))));
                    self.add_vehicle(&pars.id, veh);
                }
            } else if pars.repetitionProbability > 0. {
                if pars.repetitionEnd > pars.depart && pars.repetitionsDone < pars.repetitionNumber {
                    self.myHaveActiveFlows = true;
                }
                let origDepart = pars.depart;
                while pars.depart < time && pars.repetitionsDone < pars.repetitionNumber {
                    if pars.repetitionEnd <= pars.depart {
                        break;
                    }
                    // only call rand if all other conditions are met
                    if utils::common::rand_helper::rand() < (pars.repetitionProbability * ts()) {
                        let mut newPars = copy_vehicle_parameter(&pars);
                        newPars.id = format!("{}.{}", pars.id, pars.repetitionsDone);
                        newPars.depart = pars.depart;
                        for stop in newPars.stops.iter_mut() {
                            if stop.until >= 0 {
                                stop.until += pars.depart - origDepart;
                            }
                            if stop.arrival >= 0 {
                                stop.arrival += pars.depart - origDepart;
                            }
                        }
                        pars.m().repetitionsDone += 1;
                        // try to build the vehicle
                        let mut ty = self.get_vehicle_type_secure(&pars.vtypeid);
                        if ty.is_null() {
                            ty = self.get_vehicle_type_secure(DEFAULT_VTYPE_ID);
                        } else if !self.myKeepVTypeDist {
                            // fix the type id in case we used a distribution
                            newPars.vtypeid = ty.id.clone();
                        }
                        let stopOffset = if pars.routeid.starts_with('!') { pars.depart - origDepart } else { pars.depart };
                        let route = self.get_route_def(&pars.routeid).copy(&format!("!{}", newPars.id), stopOffset);
                        let veh = Ptr::from_raw(Box::into_raw(Box::new(ROVehicle::new(&newPars, route, ty, Ptr::from_mut(self), errorHandler))));
                        self.add_vehicle(&newPars.id, veh);
                    }
                    pars.m().depart += utils::common::time::delta_t();
                }
            } else {
                let mut depart: SUMOTime = pars.depart + pars.repetitionTotalOffset;
                while pars.repetitionsDone < pars.repetitionNumber && pars.repetitionEnd >= depart {
                    self.myHaveActiveFlows = true;
                    depart = pars.depart + pars.repetitionTotalOffset;
                    if let Some(last) = self.myDepartures.get(&pars.id).and_then(|d| d.last().copied()) {
                        depart = last;
                    }
                    if depart >= time + utils::common::time::delta_t() {
                        break;
                    }
                    if let Some(d) = self.myDepartures.get_mut(&pars.id) {
                        d.pop();
                    }
                    let mut newPars = copy_vehicle_parameter(&pars);
                    newPars.id = format!("{}.{}", pars.id, pars.repetitionsDone);
                    newPars.depart = depart;
                    for stop in newPars.stops.iter_mut() {
                        if stop.until >= 0 {
                            stop.until += depart - pars.depart;
                        }
                        if stop.arrival >= 0 {
                            stop.arrival += depart - pars.depart;
                        }
                    }
                    utils::common::rand_helper::with_default(|rng| pars.m().increment_flow(1., Ptr::from_mut(rng)));
                    // try to build the vehicle
                    let mut ty = self.get_vehicle_type_secure(&pars.vtypeid);
                    if ty.is_null() {
                        ty = self.get_vehicle_type_secure(DEFAULT_VTYPE_ID);
                    } else if !self.myKeepVTypeDist {
                        // fix the type id in case we used a distribution
                        newPars.vtypeid = ty.id.clone();
                    }
                    let stopOffset = if pars.routeid.starts_with('!') { depart - pars.depart } else { depart };
                    let route = self.get_route_def(&pars.routeid).copy(&format!("!{}", newPars.id), stopOffset);
                    let veh = Ptr::from_raw(Box::into_raw(Box::new(ROVehicle::new(&newPars, route, ty, Ptr::from_mut(self), errorHandler))));
                    self.add_vehicle(&newPars.id, veh);
                }
            }
        }
    }

    /// C++: `void RONet::createBulkRouteRequests(const RORouterProvider &, const SUMOTime, const bool)` [private]
    // void
    // RONet::createBulkRouteRequests(const RORouterProvider& provider, const SUMOTime time, const bool removeLoops) {
    //     std::map<const int, std::vector<RORoutable*> > bulkVehs;
    //     int numBulked = 0;
    //     for (RoutablesMap::const_iterator i = myRoutables.begin(); i != myRoutables.end(); ++i) {
    //         if (i->first >= time) {
    //             break;
    //         }
    //         for (RORoutable* const routable : i->second) {
    //             const ROEdge* const depEdge = routable->getDepartEdge();
    //             bulkVehs[depEdge->getNumericalID()].push_back(routable);
    //             RORoutable* const first = bulkVehs[depEdge->getNumericalID()].front();
    //             numBulked++;
    //             if (first->getMaxSpeed() != routable->getMaxSpeed()) {
    //                 WRITE_WARNINGF(TL("Bulking different maximum speeds ('%' and '%') may lead to suboptimal routes."), first->getID(), routable->getID());
    //             }
    //             if (first->getVClass() != routable->getVClass()) {
    //                 WRITE_WARNINGF(TL("Bulking different vehicle classes ('%' and '%') may lead to invalid routes."), first->getID(), routable->getID());
    //             }
    //         }
    //     }
    // #ifdef HAVE_FOX
    //     int workerIndex = 0;
    // #endif
    //     if ((int)bulkVehs.size() < numBulked) {
    //         WRITE_MESSAGE(TLF("Using bulk-mode for % entities from % origins", numBulked, bulkVehs.size()));
    //     }
    //     for (std::map<const int, std::vector<RORoutable*> >::const_iterator i = bulkVehs.begin(); i != bulkVehs.end(); ++i) {
    // #ifdef HAVE_FOX
    //         if (myThreadPool.size() > 0) {
    //             bool bulk = true;
    //             for (RORoutable* const r : i->second) {
    //                 myThreadPool.add(new RoutingTask(r, removeLoops, myErrorHandler), workerIndex);
    //                 if (bulk) {
    //                     myThreadPool.add(new BulkmodeTask(true), workerIndex);
    //                     bulk = false;
    //                 }
    //             }
    //             myThreadPool.add(new BulkmodeTask(false), workerIndex);
    //             workerIndex++;
    //             if (workerIndex == (int)myThreadPool.size()) {
    //                 workerIndex = 0;
    //             }
    //             continue;
    //         }
    // #endif
    //         for (RORoutable* const r : i->second) {
    //             r->computeRoute(provider, removeLoops, myErrorHandler);
    //             provider.setBulkMode(true);
    //         }
    //         provider.setBulkMode(false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn create_bulk_route_requests(&mut self, provider: &RORouterProvider, time: SUMOTime, removeLoops: bool) {
        let mut bulkVehs: BTreeMap<i32, Vec<Ptr<RORoutable>>> = BTreeMap::new();
        let mut numBulked: usize = 0;
        let keys: Vec<SUMOTime> = self.myRoutables.keys().copied().collect();
        for k in keys {
            if k >= time {
                break;
            }
            let list = self.myRoutables.get(&k).cloned().unwrap_or_default();
            for routable in list {
                let depEdge = routable.get_depart_edge();
                let nid = depEdge.get_numerical_id();
                bulkVehs.entry(nid).or_default().push(routable);
                let first = bulkVehs[&nid][0];
                numBulked += 1;
                if first.get_max_speed() != routable.get_max_speed() {
                    write_warningf!("Bulking different maximum speeds ('%' and '%') may lead to suboptimal routes.", first.get_id(), routable.get_id());
                }
                if first.get_v_class() != routable.get_v_class() {
                    write_warningf!("Bulking different vehicle classes ('%' and '%') may lead to invalid routes.", first.get_id(), routable.get_id());
                }
            }
        }
        // shortcut: the HAVE_FOX worker-thread bulk path is not ported, routing is sequential
        if bulkVehs.len() < numBulked {
            utils::common::msg_handler::write_message(&format!("Using bulk-mode for {} entities from {} origins", numBulked, bulkVehs.len()));
        }
        for (_, list) in bulkVehs.iter() {
            for r in list.iter() {
                r.m().compute_route(provider, removeLoops, self.myErrorHandler);
                provider.set_bulk_mode(true);
            }
            provider.set_bulk_mode(false);
        }
    }

    /// C++: `void RONet::RONet(const RONet &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &RONet) -> Self {
        // C++ declares this copy constructor private and never defines it, so it can not be called
        panic!("RONet copy constructor is private and not defined in C++");
    }

    /// C++: `RONet & RONet::operator=(const RONet &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RONet) -> &mut RONet {
        // C++ declares this assignment operator private and never defines it, so it can not be called
        panic!("RONet assignment operator is private and not defined in C++");
    }

}

