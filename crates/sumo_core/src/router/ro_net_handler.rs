//! Port of router/RONetHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use utils::xml::generic_sax_handler::GenericSAXHandler;
use crate::prelude::*;
use utils::common::msg_handler::{write_message, write_warning};
use utils::common::string_bijection::StringBijection;
use utils::common::sumo_vehicle_class::{SVC_ALL, SVC_PEDESTRIAN};

/// The handler that parses a SUMO-network for its usage in a router
/// C++ class `RONetHandler` (router/RONetHandler.h)
#[repr(C)]
pub struct RONetHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// The net to store the information into
    pub myNet: Ptr<RONet>,
    /// the loaded network version
    pub myNetworkVersion: (i32, f64),
    /// The object used to build of edges of the desired type
    pub myEdgeBuilder: Ptr<ROAbstractEdgeBuilder>,
    /// whether to ignore junction internal edges
    pub myIgnoreInternal: bool,
    /// The name of the edge/node that is currently processed
    pub myCurrentName: String,
    /// The id of the currently processed edge type
    pub myCurrentTypeID: String,
    /// The currently built edge
    pub myCurrentEdge: Ptr<ROEdge>,
    /// The currently built stopping place
    pub myCurrentStoppingPlace: Ptr<SUMOVehicleParameter_Stop>,
    /// temporary data for checking node initialisation after network parsing is finished
    pub myUnseenNodeIDs: BTreeSet<String>,
    /// time penalty for passing a minor link
    pub myMinorPenalty: f64,
    pub myTLSPenalty: f64,
    pub myTurnaroundPenalty: f64,
    /// temporary storage for bidi attributes (to be resolved after loading all edges)
    pub myBidiEdges: BTreeMap<Ptr<ROEdge>, String>,
    pub myRerouterID: String,
    pub myIntervalBegin: SUMOTime,
    pub myIntervalEnd: SUMOTime,
}

impl std::ops::Deref for RONetHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for RONetHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl utils::xml::generic_sax_handler::SaxHandler for RONetHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        RONetHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        RONetHandler::my_end_element(self, element);
        Ok(())
    }
}

/// C++ `throw` inside these void parsers: the Rust parsers return (), so the exception becomes a panic with the same message
// shortcut: C++ exceptions should reach the SAX driver as ProcessError results, once the SUMOSAXHandler base is ported
fn throw_error(e: ProcessError) -> ! {
    panic!("{}", e)
}

/// C++ `StringBijection::get` as used for enum attributes; an unknown value reports and clears ok
// shortcut: enum-valued attributes are parsed with the StringBijection tables here, SUMOSAXAttributes has no enum FromAttr yet
fn parse_enum<T: Copy + Ord>(value: &str, bijection: &utils::xml::string_bijection::StringBijection<T>, ok: &mut bool, default_value: T) -> T {
    match bijection.get(value) {
        Ok(v) => v,
        Err(e) => {
            utils::common::msg_handler::write_error(&format!("{e}"));
            *ok = false;
            default_value
        }
    }
}

/// C++ `attrs.getOpt<E>(attr, ...)` for an enum attribute with a default
fn get_opt_enum<T: Copy + Ord>(attrs: &SUMOSAXAttributes, attr: i32, ok: &mut bool, default_value: T, bijection: &utils::xml::string_bijection::StringBijection<T>) -> T {
    match attrs.get_string_opt(attr) {
        None => default_value,
        Some(s) => parse_enum(&s, bijection, ok, default_value),
    }
}

/// C++ `new SUMOVehicleParameter::Stop()` (default member values as in SUMOVehicleParameter.h)
fn new_stop() -> SUMOVehicleParameter_Stop {
    SUMOVehicleParameter_Stop {
        base: Parameterised::new(),
        edge: String::new(),
        lane: String::new(),
        busstop: String::new(),
        containerstop: String::new(),
        parkingarea: String::new(),
        chargingStation: String::new(),
        overheadWireSegment: String::new(),
        startPos: 0.,
        endPos: 0.,
        arrival: -1,
        duration: -1,
        durationDist: (),
        until: -1,
        extension: -1,
        waitUntil: -1,
        triggered: false,
        containerTriggered: false,
        joinTriggered: false,
        parking: ParkingType::ONROAD,
        awaitedPersons: BTreeSet::new(),
        permitted: BTreeSet::new(),
        awaitedContainers: BTreeSet::new(),
        friendlyPos: false,
        actType: String::new(),
        tripId: String::new(),
        line: String::new(),
        split: String::new(),
        join: String::new(),
        speed: 0.,
        posLat: utils::common::std_defs::INVALID_DOUBLE,
        onDemand: false,
        priority: -1.,
        jump: -1,
        jumpUntil: -1,
        started: -1,
        ended: -1,
        accessPos: Vec::new(),
        index: 0,
        routeIndex: 0,
        parametersSet: 0,
        collision: false,
        breakDown: false,
    }
}

impl RONetHandler {
    /// C++: `void RONetHandler::RONetHandler(RONet &, ROAbstractEdgeBuilder &, const bool, const double, double, double)`
    /// Constructor
    // RONetHandler::RONetHandler(RONet& net, ROAbstractEdgeBuilder& eb, const bool ignoreInternal, const double minorPenalty, double tlsPenalty, double turnaroundPenalty) :
    //     SUMOSAXHandler("sumo-network"),
    //     myNet(net),
    //     myNetworkVersion(0, 0),
    //     myEdgeBuilder(eb), myIgnoreInternal(ignoreInternal),
    //     myCurrentName(), myCurrentEdge(nullptr), myCurrentStoppingPlace(nullptr),
    //     myMinorPenalty(minorPenalty),
    //     myTLSPenalty(tlsPenalty),
    //     myTurnaroundPenalty(turnaroundPenalty)
    // {}
    #[allow(unused_variables)]
    pub fn new(net: &mut RONet, eb: &mut ROAbstractEdgeBuilder, ignoreInternal: bool, minorPenalty: f64, tlsPenalty: f64, turnaroundPenalty: f64) -> Self {
        RONetHandler {
            base: utils::xml::sumo_sax_handler::new("sumo-network", ""),
            myNet: Ptr::from_mut(net),
            myNetworkVersion: (0, 0.),
            myEdgeBuilder: Ptr::from_mut(eb),
            myIgnoreInternal: ignoreInternal,
            myCurrentName: String::new(),
            myCurrentTypeID: String::new(),
            myCurrentEdge: Ptr::null(),
            myCurrentStoppingPlace: Ptr::null(),
            myUnseenNodeIDs: BTreeSet::new(),
            myMinorPenalty: minorPenalty,
            myTLSPenalty: tlsPenalty,
            myTurnaroundPenalty: turnaroundPenalty,
            myBidiEdges: BTreeMap::new(),
            myRerouterID: String::new(),
            // C++ leaves these uninitialised; they are set by the interval element before use
            myIntervalBegin: 0,
            myIntervalEnd: 0,
        }
    }

    /// C++: `void RONetHandler::~RONetHandler()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has no body
    }

    /// C++: `const int & RONetHandler::getBidiMap() const` [const]
    /// retrieve mapping of edges to bidi edges (must be resolved after loading network)
    // const std::map<ROEdge*, std::string>& getBidiMap() const {
    //         return myBidiEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi_map(&self) -> &BTreeMap<Ptr<ROEdge>, String> {
        &self.myBidiEdges
    }

    /// C++: `void RONetHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // RONetHandler::myStartElement(int element,
    //                              const SUMOSAXAttributes& attrs) {
    //     switch (element) {
    //         case SUMO_TAG_LOCATION:
    //             setLocation(attrs);
    //             break;
    //         case SUMO_TAG_NET: {
    //             bool ok;
    //             myNetworkVersion = StringUtils::toVersion(attrs.get<std::string>(SUMO_ATTR_VERSION, nullptr, ok, false));
    //             break;
    //         }
    //         case SUMO_TAG_EDGE:
    //             // in the first step, we do need the name to allocate the edge
    //             // in the second, we need it to know to which edge we have to add
    //             //  the following edges to
    //             parseEdge(attrs);
    //             break;
    //         case SUMO_TAG_LANE:
    //             parseLane(attrs);
    //             break;
    //         case SUMO_TAG_JUNCTION:
    //             parseJunction(attrs);
    //             break;
    //         case SUMO_TAG_CONNECTION:
    //             parseConnection(attrs);
    //             break;
    //         case SUMO_TAG_BUS_STOP:
    //         case SUMO_TAG_TRAIN_STOP:
    //         case SUMO_TAG_CONTAINER_STOP:
    //         case SUMO_TAG_PARKING_AREA:
    //         case SUMO_TAG_CHARGING_STATION:
    //         case SUMO_TAG_OVERHEAD_WIRE_SEGMENT:
    //             parseStoppingPlace(attrs, (SumoXMLTag)element);
    //             break;
    //         case SUMO_TAG_ACCESS:
    //             parseAccess(attrs);
    //             break;
    //         case SUMO_TAG_TAZ:
    //             parseDistrict(attrs);
    //             break;
    //         case SUMO_TAG_TAZSOURCE:
    //             parseDistrictEdge(attrs, true);
    //             break;
    //         case SUMO_TAG_TAZSINK:
    //             parseDistrictEdge(attrs, false);
    //             break;
    //         case SUMO_TAG_TYPE: {
    //             bool ok = true;
    //             myCurrentTypeID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //             break;
    //         }
    //         case SUMO_TAG_RESTRICTION: {
    //             bool ok = true;
    //             const SUMOVehicleClass svc = getVehicleClassID(attrs.get<std::string>(SUMO_ATTR_VCLASS, myCurrentTypeID.c_str(), ok));
    //             const double speed = attrs.get<double>(SUMO_ATTR_SPEED, myCurrentTypeID.c_str(), ok);
    //             if (ok) {
    //                 myNet.addSpeedRestriction(myCurrentTypeID, svc, speed);
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_PREFERENCE: {
    //             bool ok = true;
    //             const std::string routingType = attrs.get<std::string>(SUMO_ATTR_ROUTINGTYPE, nullptr, ok);
    //             const double prio = attrs.get<double>(SUMO_ATTR_PRIORITY, routingType.c_str(), ok);
    //             if (prio <= 0) {
    //                 throw InvalidArgument("In preference for routingType '" + routingType + "', priority must be positve");
    //             }
    //             if (attrs.hasAttribute(SUMO_ATTR_VCLASSES)) {
    //                 StringTokenizer st(attrs.get<std::string>(SUMO_ATTR_VCLASSES, routingType.c_str(), ok));
    //                 for (std::string className : st.getVector()) {
    //                     myNet.addPreference(routingType, getVehicleClassID(className), prio);
    //                 }
    //             } else if (!attrs.hasAttribute(SUMO_ATTR_VTYPES)) {
    //                 // general preferenze applying to all types and vClasses
    //                 myNet.addPreference(routingType, "", prio);
    //             }
    //             if (attrs.hasAttribute(SUMO_ATTR_VTYPES)) {
    //                 StringTokenizer st(attrs.get<std::string>(SUMO_ATTR_VTYPES, routingType.c_str(), ok));
    //                 for (std::string typeName : st.getVector()) {
    //                     myNet.addPreference(routingType, typeName, prio);
    //                 }
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_REROUTER: {
    //             bool ok = true;
    //             myRerouterID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //             break;
    //         }
    //         case SUMO_TAG_INTERVAL:
    //             if (myRerouterID != "") {
    //                 bool ok = true;
    //                 myIntervalBegin = attrs.getOptSUMOTimeReporting(SUMO_ATTR_BEGIN, myRerouterID.c_str(), ok, -1);
    //                 myIntervalEnd = attrs.getOptSUMOTimeReporting(SUMO_ATTR_END, myRerouterID.c_str(), ok, SUMOTime_MAX);
    //                 if (myIntervalEnd <= myIntervalBegin) {
    //                     throw ProcessError(TLF("Invalid rerouter interval from % to %", time2string(myIntervalBegin), time2string(myIntervalEnd)));
    //                 }
    //             }
    //             break;
    //         case SUMO_TAG_CLOSING_REROUTE: {
    //             const std::string& closed_id = attrs.getStringSecure(SUMO_ATTR_ID, "");
    //             ROEdge* closedEdge = myNet.getEdge(closed_id);
    //             if (closedEdge == nullptr) {
    //                 throw ProcessError(TLF("rerouter '%': Edge '%' to close is not known.", myRerouterID, closed_id));
    //             }
    //             bool ok;
    //             const std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, myRerouterID.c_str(), ok, "", false);
    //             const std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, myRerouterID.c_str(), ok, "");
    //             RouterProhibition prohibition;
    //             prohibition.permissions = parseVehicleClasses(allow, disallow);
    //             prohibition.begin = STEPS2TIME(myIntervalBegin);
    //             prohibition.end = STEPS2TIME(attrs.getOptSUMOTimeReporting(SUMO_ATTR_UNTIL, nullptr, ok, myIntervalEnd));
    //             myNet.addProhibition(closedEdge, prohibition);
    //             break;
    //         }
    //         case SUMO_TAG_CLOSING_LANE_REROUTE: {
    //             const std::string& closed_id = attrs.getStringSecure(SUMO_ATTR_ID, "");
    //             ROLane* closedLane = myNet.getLane(closed_id);
    //             if (closedLane == nullptr) {
    //                 throw ProcessError(TLF("rerouter '%': Lane '%' to close is not known.", myRerouterID, closed_id));
    //             }
    //             bool ok;
    //             const std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, myRerouterID.c_str(), ok, "", false);
    //             const std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, myRerouterID.c_str(), ok, "");
    //             RouterProhibition prohibition;
    //             prohibition.permissions = parseVehicleClasses(allow, disallow);
    //             prohibition.begin = STEPS2TIME(myIntervalBegin);
    //             prohibition.end = STEPS2TIME(attrs.getOptSUMOTimeReporting(SUMO_ATTR_UNTIL, nullptr, ok, myIntervalEnd));
    //             myNet.addLaneProhibition(closedLane, prohibition);
    //             break;
    //         }
    //         case SUMO_TAG_PARAM:
    //             addParam(attrs);
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        match element {
            SUMO_TAG_LOCATION => {
                self.set_location(attrs);
            }
            SUMO_TAG_NET => {
                // C++ StringUtils::toVersion throws on malformed input
                self.myNetworkVersion = utils::common::string_utils::to_version(&attrs.get::<String>(SUMO_ATTR_VERSION, "", &mut ok, false)).unwrap_or_else(|e| throw_error(e));
            }
            // in the first step, we do need the name to allocate the edge
            // in the second, we need it to know to which edge we have to add
            //  the following edges to
            SUMO_TAG_EDGE => {
                self.parse_edge(attrs);
            }
            SUMO_TAG_LANE => {
                self.parse_lane(attrs);
            }
            SUMO_TAG_JUNCTION => {
                self.parse_junction(attrs);
            }
            SUMO_TAG_CONNECTION => {
                self.parse_connection(attrs);
            }
            SUMO_TAG_BUS_STOP | SUMO_TAG_TRAIN_STOP | SUMO_TAG_CONTAINER_STOP | SUMO_TAG_PARKING_AREA | SUMO_TAG_CHARGING_STATION | SUMO_TAG_OVERHEAD_WIRE_SEGMENT => {
                self.parse_stopping_place(attrs, element);
            }
            SUMO_TAG_ACCESS => {
                self.parse_access(attrs);
            }
            SUMO_TAG_TAZ => {
                self.parse_district(attrs);
            }
            SUMO_TAG_TAZSOURCE => {
                self.parse_district_edge(attrs, true);
            }
            SUMO_TAG_TAZSINK => {
                self.parse_district_edge(attrs, false);
            }
            SUMO_TAG_TYPE => {
                self.myCurrentTypeID = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
            }
            SUMO_TAG_RESTRICTION => {
                let tid = self.myCurrentTypeID.clone();
                let svc = utils::common::sumo_vehicle_class::get_vehicle_class_id(&attrs.get::<String>(SUMO_ATTR_VCLASS, &tid, &mut ok, true)).unwrap_or_else(|e| throw_error(e));
                let speed = attrs.get::<f64>(SUMO_ATTR_SPEED, &tid, &mut ok, true);
                if ok {
                    self.myNet.m().add_speed_restriction(&tid, svc, speed);
                }
            }
            SUMO_TAG_PREFERENCE => {
                let routingType = attrs.get::<String>(SUMO_ATTR_ROUTINGTYPE, "", &mut ok, true);
                let prio = attrs.get::<f64>(SUMO_ATTR_PRIORITY, &routingType, &mut ok, true);
                if prio <= 0. {
                    throw_error(ProcessError::InvalidArgument(format!("In preference for routingType '{}', priority must be positve", routingType)));
                }
                if attrs.has_attribute(SUMO_ATTR_VCLASSES) {
                    let mut st = utils::common::string_tokenizer::StringTokenizer::new(&attrs.get::<String>(SUMO_ATTR_VCLASSES, &routingType, &mut ok, true));
                    for className in st.get_vector() {
                        let svc = utils::common::sumo_vehicle_class::get_vehicle_class_id(&className).unwrap_or_else(|e| throw_error(e));
                        self.myNet.m().add_preference(&routingType, svc, prio);
                    }
                } else if !attrs.has_attribute(SUMO_ATTR_VTYPES) {
                    // general preferenze applying to all types and vClasses
                    self.myNet.m().add_preference_2(&routingType, String::new(), prio);
                }
                if attrs.has_attribute(SUMO_ATTR_VTYPES) {
                    let mut st = utils::common::string_tokenizer::StringTokenizer::new(&attrs.get::<String>(SUMO_ATTR_VTYPES, &routingType, &mut ok, true));
                    for typeName in st.get_vector() {
                        self.myNet.m().add_preference_2(&routingType, typeName, prio);
                    }
                }
            }
            SUMO_TAG_REROUTER => {
                self.myRerouterID = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
            }
            SUMO_TAG_INTERVAL => {
                if !self.myRerouterID.is_empty() {
                    let rid = self.myRerouterID.clone();
                    self.myIntervalBegin = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_BEGIN, &rid, &mut ok, -1, true);
                    self.myIntervalEnd = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_END, &rid, &mut ok, utils::common::time::SUMOTIME_MAX, true);
                    if self.myIntervalEnd <= self.myIntervalBegin {
                        throw_error(ProcessError::Process(format!("Invalid rerouter interval from {} to {}", utils::common::time::time2string(self.myIntervalBegin), utils::common::time::time2string(self.myIntervalEnd))));
                    }
                }
            }
            SUMO_TAG_CLOSING_REROUTE => {
                let closed_id = attrs.get_string_secure(SUMO_ATTR_ID, "");
                let closedEdge = self.myNet.get_edge(&closed_id);
                if closedEdge.is_null() {
                    throw_error(ProcessError::Process(format!("rerouter '{}': Edge '{}' to close is not known.", self.myRerouterID, closed_id)));
                }
                let rid = self.myRerouterID.clone();
                let allow = attrs.get_opt::<String>(SUMO_ATTR_ALLOW, &rid, &mut ok, String::new(), false);
                let disallow = attrs.get_opt::<String>(SUMO_ATTR_DISALLOW, &rid, &mut ok, String::new(), true);
                let mut prohibition = RouterProhibition {
                    begin: 0.,
                    end: 0.,
                    permissions: utils::common::sumo_vehicle_class::parse_vehicle_classes_allow_disallow(&allow, &disallow, utils::common::std_defs::NETWORK_VERSION),
                };
                prohibition.begin = utils::common::time::steps2time(self.myIntervalBegin);
                prohibition.end = utils::common::time::steps2time(attrs.get_opt_sumo_time_reporting(SUMO_ATTR_UNTIL, "", &mut ok, self.myIntervalEnd, true));
                self.myNet.m().add_prohibition(closedEdge, &prohibition);
            }
            SUMO_TAG_CLOSING_LANE_REROUTE => {
                let closed_id = attrs.get_string_secure(SUMO_ATTR_ID, "");
                let closedLane = self.myNet.get_lane(&closed_id);
                if closedLane.is_null() {
                    throw_error(ProcessError::Process(format!("rerouter '{}': Lane '{}' to close is not known.", self.myRerouterID, closed_id)));
                }
                let rid = self.myRerouterID.clone();
                let allow = attrs.get_opt::<String>(SUMO_ATTR_ALLOW, &rid, &mut ok, String::new(), false);
                let disallow = attrs.get_opt::<String>(SUMO_ATTR_DISALLOW, &rid, &mut ok, String::new(), true);
                let mut prohibition = RouterProhibition {
                    begin: 0.,
                    end: 0.,
                    permissions: utils::common::sumo_vehicle_class::parse_vehicle_classes_allow_disallow(&allow, &disallow, utils::common::std_defs::NETWORK_VERSION),
                };
                prohibition.begin = utils::common::time::steps2time(self.myIntervalBegin);
                prohibition.end = utils::common::time::steps2time(attrs.get_opt_sumo_time_reporting(SUMO_ATTR_UNTIL, "", &mut ok, self.myIntervalEnd, true));
                self.myNet.m().add_lane_prohibition(closedLane, &prohibition);
            }
            SUMO_TAG_PARAM => {
                self.add_param(attrs);
            }
            _ => {}
        }
    }

    /// C++: `void RONetHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // RONetHandler::myEndElement(int element) {
    //     switch (element) {
    //         case SUMO_TAG_REROUTER:
    //             myRerouterID = "";
    //             break;
    //         case SUMO_TAG_NET:
    //             // build junction graph
    //             for (std::set<std::string>::const_iterator it = myUnseenNodeIDs.begin(); it != myUnseenNodeIDs.end(); ++it) {
    //                 WRITE_ERRORF(TL("Unknown node '%'."), *it);
    //             }
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        match element {
            SUMO_TAG_REROUTER => {
                self.myRerouterID = String::new();
            }
            SUMO_TAG_NET => {
                // build junction graph
                for id in self.myUnseenNodeIDs.iter() {
                    write_errorf!("Unknown node '%'.", id);
                }
            }
            _ => {}
        }
    }

    /// C++: `void RONetHandler::addParam(const SUMOSAXAttributes &)` [protected]
    /// assign arbitrary vehicle parameters
    // void
    // RONetHandler::addParam(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string key = attrs.get<std::string>(SUMO_ATTR_KEY, nullptr, ok);
    //     // circumventing empty string test
    //     const std::string val = attrs.hasAttribute(SUMO_ATTR_VALUE) ? attrs.getString(SUMO_ATTR_VALUE) : "";
    //     // add parameter in current created element, or in myLoadedParameterised
    //     if (myCurrentEdge != nullptr) {
    //         myCurrentEdge->setParameter(key, val);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_param(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let key = attrs.get::<String>(SUMO_ATTR_KEY, "", &mut ok, true);
        // circumventing empty string test
        let val = if attrs.has_attribute(SUMO_ATTR_VALUE) { attrs.get_string(SUMO_ATTR_VALUE) } else { String::new() };
        // add parameter in current created element, or in myLoadedParameterised
        if !self.myCurrentEdge.is_null() {
            self.myCurrentEdge.m().base2.set_parameter(&key, &val);
        }
    }

    /// C++: `void RONetHandler::parseEdge(const SUMOSAXAttributes &)` [protected]
    /// Parses and builds an edge
    // void
    // RONetHandler::parseEdge(const SUMOSAXAttributes& attrs) {
    //     // get the id, report an error if not given or empty...
    //     bool ok = true;
    //     myCurrentName = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //     const SumoXMLEdgeFunc func = attrs.getOpt<SumoXMLEdgeFunc>(SUMO_ATTR_FUNCTION, myCurrentName.c_str(), ok, SumoXMLEdgeFunc::NORMAL);
    //     if (!ok) {
    //         return;
    //     }
    //     // get the edge
    //     std::string from;
    //     std::string to;
    //     int priority;
    //     myCurrentEdge = nullptr;
    //     if (func == SumoXMLEdgeFunc::INTERNAL || func == SumoXMLEdgeFunc::CROSSING || func == SumoXMLEdgeFunc::WALKINGAREA) {
    //         assert(myCurrentName[0] == ':');
    //         const std::string junctionID = SUMOXMLDefinitions::getJunctionIDFromInternalEdge(myCurrentName);
    //         from = junctionID;
    //         to = junctionID;
    //         priority = -1;
    //     } else {
    //         from = attrs.get<std::string>(SUMO_ATTR_FROM, myCurrentName.c_str(), ok);
    //         to = attrs.get<std::string>(SUMO_ATTR_TO, myCurrentName.c_str(), ok);
    //         priority = attrs.get<int>(SUMO_ATTR_PRIORITY, myCurrentName.c_str(), ok);
    //         if (!ok) {
    //             return;
    //         }
    //     }
    //     RONode* fromNode = myNet.getNode(from);
    //     if (fromNode == nullptr) {
    //         myUnseenNodeIDs.insert(from);
    //         fromNode = new RONode(from);
    //         myNet.addNode(fromNode);
    //     }
    //     RONode* toNode = myNet.getNode(to);
    //     if (toNode == nullptr) {
    //         myUnseenNodeIDs.insert(to);
    //         toNode = new RONode(to);
    //         myNet.addNode(toNode);
    //     }
    //     const std::string type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, myCurrentName.c_str(), ok, "");
    //     const std::string routingType = attrs.getOpt<std::string>(SUMO_ATTR_ROUTINGTYPE, myCurrentName.c_str(), ok, "");
    //     // build the edge
    //     myCurrentEdge = myEdgeBuilder.buildEdge(myCurrentName, fromNode, toNode, priority, type, routingType);
    //     myCurrentEdge->setFunction(func);
    //
    //     if (myNet.addEdge(myCurrentEdge)) {
    //         fromNode->addOutgoing(myCurrentEdge);
    //         toNode->addIncoming(myCurrentEdge);
    //         const std::string bidi = attrs.getOpt<std::string>(SUMO_ATTR_BIDI, myCurrentName.c_str(), ok, "");
    //         if (bidi != "") {
    //             myBidiEdges[myCurrentEdge] = bidi;
    //         }
    //     } else {
    //         myCurrentEdge = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_edge(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        self.myCurrentName = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            throw_error(ProcessError::Process(String::new()));
        }
        let name = self.myCurrentName.clone();
        let func: SumoXMLEdgeFunc = get_opt_enum(attrs, SUMO_ATTR_FUNCTION, &mut ok, SumoXMLEdgeFunc::NORMAL, &*EdgeFunctions);
        if !ok {
            return;
        }
        // get the edge
        let from: String;
        let to: String;
        let priority: i32;
        self.myCurrentEdge = Ptr::null();
        if func == SumoXMLEdgeFunc::INTERNAL || func == SumoXMLEdgeFunc::CROSSING || func == SumoXMLEdgeFunc::WALKINGAREA {
            // C++ asserts that the name starts with ':'
            let junctionID = utils::xml::sumo_xml_definitions::get_junction_id_from_internal_edge(&name);
            from = junctionID.clone();
            to = junctionID;
            priority = -1;
        } else {
            from = attrs.get::<String>(SUMO_ATTR_FROM, &name, &mut ok, true);
            to = attrs.get::<String>(SUMO_ATTR_TO, &name, &mut ok, true);
            priority = attrs.get::<i32>(SUMO_ATTR_PRIORITY, &name, &mut ok, true);
            if !ok {
                return;
            }
        }
        let mut fromNode = self.myNet.get_node(&from);
        if fromNode.is_null() {
            self.myUnseenNodeIDs.insert(from.clone());
            fromNode = Ptr::from_raw(Box::into_raw(Box::new(RONode::new(&from))));
            self.myNet.m().add_node(fromNode);
        }
        let mut toNode = self.myNet.get_node(&to);
        if toNode.is_null() {
            self.myUnseenNodeIDs.insert(to.clone());
            toNode = Ptr::from_raw(Box::into_raw(Box::new(RONode::new(&to))));
            self.myNet.m().add_node(toNode);
        }
        let type_ = attrs.get_opt::<String>(SUMO_ATTR_TYPE, &name, &mut ok, String::new(), true);
        let routingType = attrs.get_opt::<String>(SUMO_ATTR_ROUTINGTYPE, &name, &mut ok, String::new(), true);
        // build the edge
        self.myCurrentEdge = self.myEdgeBuilder.m().build_edge(&name, fromNode, toNode, priority, &type_, &routingType);
        self.myCurrentEdge.m().set_function(func);

        if self.myNet.m().add_edge(self.myCurrentEdge) {
            fromNode.m().add_outgoing(self.myCurrentEdge);
            toNode.m().add_incoming(self.myCurrentEdge);
            let bidi = attrs.get_opt::<String>(SUMO_ATTR_BIDI, &name, &mut ok, String::new(), true);
            if bidi != "" {
                self.myBidiEdges.insert(self.myCurrentEdge, bidi);
            }
        } else {
            self.myCurrentEdge = Ptr::null();
        }
    }

    /// C++: `void RONetHandler::parseLane(const SUMOSAXAttributes &)` [virtual, protected]
    /// Parses and builds a lane
    // void
    // RONetHandler::parseLane(const SUMOSAXAttributes& attrs) {
    //     if (myCurrentEdge == nullptr) {
    //         // was an internal edge to skip or an error occurred
    //         return;
    //     }
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     // get the speed
    //     double maxSpeed = attrs.get<double>(SUMO_ATTR_SPEED, id.c_str(), ok);
    //     double length = attrs.get<double>(SUMO_ATTR_LENGTH, id.c_str(), ok);
    //     std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, id.c_str(), ok, "");
    //     std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, id.c_str(), ok, "");
    //     const PositionVector shape = attrs.get<PositionVector>(SUMO_ATTR_SHAPE, id.c_str(), ok);
    //     if (!ok) {
    //         return;
    //     }
    //     if (shape.size() < 2 && myCurrentEdge->getFunction() != SumoXMLEdgeFunc::WALKINGAREA) {
    //         WRITE_ERRORF(TL("Ignoring lane '%' with broken shape."), id);
    //         return;
    //     }
    //     // get the length
    //     // get the vehicle classes
    //     SVCPermissions permissions = parseVehicleClasses(allow, disallow, myNetworkVersion);
    //     if (permissions != SVCAll) {
    //         myNet.setPermissionsFound();
    //     }
    //     // add when both values are valid
    //     if (maxSpeed > 0 && length > 0 && id.length() > 0) {
    //         myCurrentEdge->addLane(new ROLane(id, myCurrentEdge, length, maxSpeed, permissions, shape));
    //     } else {
    //         WRITE_WARNING("Ignoring lane '" + id + "' with speed " + toString(maxSpeed) + " and length " + toString(length));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_lane(&mut self, attrs: &SUMOSAXAttributes) {
        if self.myCurrentEdge.is_null() {
            // was an internal edge to skip or an error occurred
            return;
        }
        let mut ok = true;
        // get the id, report an error if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            return;
        }
        // get the speed
        let maxSpeed = attrs.get::<f64>(SUMO_ATTR_SPEED, &id, &mut ok, true);
        let length = attrs.get::<f64>(SUMO_ATTR_LENGTH, &id, &mut ok, true);
        let allow = attrs.get_opt::<String>(SUMO_ATTR_ALLOW, &id, &mut ok, String::new(), true);
        let disallow = attrs.get_opt::<String>(SUMO_ATTR_DISALLOW, &id, &mut ok, String::new(), true);
        let shape = attrs.get::<PositionVector>(SUMO_ATTR_SHAPE, &id, &mut ok, true);
        if !ok {
            return;
        }
        if shape.0.len() < 2 && self.myCurrentEdge.get_function() != SumoXMLEdgeFunc::WALKINGAREA {
            write_errorf!("Ignoring lane '%' with broken shape.", id);
            return;
        }
        // get the length
        // get the vehicle classes
        let permissions = utils::common::sumo_vehicle_class::parse_vehicle_classes_allow_disallow(&allow, &disallow, self.myNetworkVersion);
        if permissions != SVC_ALL {
            self.myNet.m().set_permissions_found();
        }
        // add when both values are valid
        if maxSpeed > 0. && length > 0. && id.len() > 0 {
            let lane = ROLane::new(&id, self.myCurrentEdge, length, maxSpeed, permissions, &shape);
            self.myCurrentEdge.m().add_lane(Ptr::from_raw(Box::into_raw(Box::new(lane))));
        } else {
            let precision = utils::common::std_defs::g_precision();
            write_warning(&format!("Ignoring lane '{}' with speed {} and length {}", id, utils::common::to_string::f64_to_string_prec(maxSpeed, precision), utils::common::to_string::f64_to_string_prec(length, precision)));
        }
    }

    /// C++: `void RONetHandler::parseJunction(const SUMOSAXAttributes &)` [protected]
    /// Parses a junction's position
    // void
    // RONetHandler::parseJunction(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     const SumoXMLNodeType type = attrs.get<SumoXMLNodeType>(SUMO_ATTR_TYPE, id.c_str(), ok);
    //     if (type == SumoXMLNodeType::INTERNAL) {
    //         return;
    //     }
    //     myUnseenNodeIDs.erase(id);
    //     // get the position of the node
    //     const double x = attrs.get<double>(SUMO_ATTR_X, id.c_str(), ok);
    //     const double y = attrs.get<double>(SUMO_ATTR_Y, id.c_str(), ok);
    //     const double z = attrs.getOpt<double>(SUMO_ATTR_Z, id.c_str(), ok, 0.);
    //     if (!ok) {
    //         return;
    //     }
    //     RONode* n = myNet.getNode(id);
    //     if (n == nullptr) {
    //         WRITE_WARNINGF(TL("Skipping isolated junction '%'."), id);
    //     } else {
    //         n->setPosition(Position(x, y, z));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_junction(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        // get the id, report an error if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        let typeName = attrs.get::<String>(SUMO_ATTR_TYPE, &id, &mut ok, true);
        let type_ = parse_enum(&typeName, &*NodeTypes, &mut ok, SumoXMLNodeType::INTERNAL);
        if type_ == SumoXMLNodeType::INTERNAL {
            return;
        }
        self.myUnseenNodeIDs.remove(&id);
        // get the position of the node
        let x = attrs.get::<f64>(SUMO_ATTR_X, &id, &mut ok, true);
        let y = attrs.get::<f64>(SUMO_ATTR_Y, &id, &mut ok, true);
        let z = attrs.get_opt::<f64>(SUMO_ATTR_Z, &id, &mut ok, 0., true);
        if !ok {
            return;
        }
        let n = self.myNet.get_node(&id);
        if n.is_null() {
            write_warningf!("Skipping isolated junction '%'.", id);
        } else {
            n.m().set_position(&Position::new3(x, y, z));
        }
    }

    /// C++: `void RONetHandler::parseConnection(const SUMOSAXAttributes &)` [protected]
    /// Parses a connection Called on the occurrence of a "connection" element
    // void
    // RONetHandler::parseConnection(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::string fromID = attrs.get<std::string>(SUMO_ATTR_FROM, nullptr, ok);
    //     std::string toID = attrs.get<std::string>(SUMO_ATTR_TO, nullptr, ok);
    //     const int fromLane = attrs.get<int>(SUMO_ATTR_FROM_LANE, nullptr, ok);
    //     const int toLane = attrs.get<int>(SUMO_ATTR_TO_LANE, nullptr, ok);
    //     std::string dir = attrs.get<std::string>(SUMO_ATTR_DIR, nullptr, ok);
    //     std::string viaID = attrs.getOpt<std::string>(SUMO_ATTR_VIA, nullptr, ok, "");
    //     std::string tlID = attrs.getOpt<std::string>(SUMO_ATTR_TLID, nullptr, ok, "");
    //     ROEdge* from = myNet.getEdge(fromID);
    //     ROEdge* to = myNet.getEdge(toID);
    //     if (from == nullptr) {
    //         throw ProcessError(TLF("unknown from-edge '%' in connection", fromID));
    //     }
    //     if (to == nullptr) {
    //         throw ProcessError(TLF("unknown to-edge '%' in connection", toID));
    //     }
    //     if ((int)from->getLanes().size() <= fromLane) {
    //         throw ProcessError("invalid fromLane '" + toString(fromLane) + "' in connection from '" + fromID + "'.");
    //     }
    //     if ((int)to->getLanes().size() <= toLane) {
    //         throw ProcessError("invalid toLane '" + toString(toLane) + "' in connection to '" + toID + "'.");
    //     }
    //     if (myIgnoreInternal || viaID == "") {
    //         std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, nullptr, ok, "");
    //         std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, nullptr, ok, "");
    //         ROEdge* dummyVia = nullptr;
    //         SVCPermissions permissions;
    //         if (allow == "" && disallow == "") {
    //             permissions = SVC_UNSPECIFIED;
    //         } else {
    //             myNet.setPermissionsFound();
    //             // dummyVia is only needed to hold permissions
    //             permissions = parseVehicleClasses(allow, disallow);
    //             dummyVia = new ROEdge("dummyVia_" + from->getLanes()[fromLane]->getID() + "->" + to->getLanes()[toLane]->getID(),
    //                                   from->getToJunction(), from->getToJunction(), permissions);
    //             myNet.addEdge(dummyVia);
    //         }
    //         from->getLanes()[fromLane]->addOutgoingLane(to->getLanes()[toLane], dummyVia);
    //         from->addSuccessor(to, nullptr, dir);
    //     } else {
    //         ROEdge* const via = myNet.getEdge(SUMOXMLDefinitions::getEdgeIDFromLane(viaID));
    //         if (via == nullptr) {
    //             throw ProcessError(TLF("unknown via-edge '%' in connection", viaID));
    //         }
    //         from->getLanes()[fromLane]->addOutgoingLane(to->getLanes()[toLane], via);
    //         from->addSuccessor(to, via, dir);
    //         via->addSuccessor(to, nullptr, dir);
    //         LinkState state = SUMOXMLDefinitions::LinkStates.get(attrs.get<std::string>(SUMO_ATTR_STATE, nullptr, ok));
    //         if (state == LINKSTATE_MINOR || state == LINKSTATE_EQUAL || state == LINKSTATE_STOP || state == LINKSTATE_ALLWAY_STOP) {
    //             via->setTimePenalty(myMinorPenalty);
    //         }
    //         if (dir == toString(LinkDirection::TURN) || dir == toString(LinkDirection::TURN_LEFTHAND)) {
    //             via->setTimePenalty(myTurnaroundPenalty);
    //         }
    //         if (tlID != "") {
    //             via->setTimePenalty(myTLSPenalty);
    //         }
    //     }
    //     if (to->isCrossing()) {
    //         to->setTimePenalty(myTLSPenalty);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_connection(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let fromID = attrs.get::<String>(SUMO_ATTR_FROM, "", &mut ok, true);
        let toID = attrs.get::<String>(SUMO_ATTR_TO, "", &mut ok, true);
        let fromLane = attrs.get::<i32>(SUMO_ATTR_FROM_LANE, "", &mut ok, true);
        let toLane = attrs.get::<i32>(SUMO_ATTR_TO_LANE, "", &mut ok, true);
        let dir = attrs.get::<String>(SUMO_ATTR_DIR, "", &mut ok, true);
        let viaID = attrs.get_opt::<String>(SUMO_ATTR_VIA, "", &mut ok, String::new(), true);
        let tlID = attrs.get_opt::<String>(SUMO_ATTR_TLID, "", &mut ok, String::new(), true);
        let from = self.myNet.get_edge(&fromID);
        let to = self.myNet.get_edge(&toID);
        if from.is_null() {
            throw_error(ProcessError::Process(format!("unknown from-edge '{}' in connection", fromID)));
        }
        if to.is_null() {
            throw_error(ProcessError::Process(format!("unknown to-edge '{}' in connection", toID)));
        }
        if (from.get_lanes().len() as i32) <= fromLane {
            throw_error(ProcessError::Process(format!("invalid fromLane '{}' in connection from '{}'.", fromLane, fromID)));
        }
        if (to.get_lanes().len() as i32) <= toLane {
            throw_error(ProcessError::Process(format!("invalid toLane '{}' in connection to '{}'.", toLane, toID)));
        }
        if self.myIgnoreInternal || viaID == "" {
            let allow = attrs.get_opt::<String>(SUMO_ATTR_ALLOW, "", &mut ok, String::new(), true);
            let disallow = attrs.get_opt::<String>(SUMO_ATTR_DISALLOW, "", &mut ok, String::new(), true);
            let mut dummyVia: Ptr<ROEdge> = Ptr::null();
            if allow == "" && disallow == "" {
                // permissions are unspecified, no dummy edge is needed
            } else {
                self.myNet.m().set_permissions_found();
                // dummyVia is only needed to hold permissions
                let permissions = utils::common::sumo_vehicle_class::parse_vehicle_classes_allow_disallow(&allow, &disallow, utils::common::std_defs::NETWORK_VERSION);
                let fromLaneID = from.get_lanes()[fromLane as usize].base.get_id().to_string();
                let toLaneID = to.get_lanes()[toLane as usize].base.get_id().to_string();
                dummyVia = Ptr::from_raw(Box::into_raw(Box::new(ROEdge::new_2(&format!("dummyVia_{}->{}", fromLaneID, toLaneID), from.get_to_junction(), from.get_to_junction(), permissions))));
                self.myNet.m().add_edge(dummyVia);
            }
            from.get_lanes()[fromLane as usize].m().add_outgoing_lane(to.get_lanes()[toLane as usize], dummyVia);
            from.m().add_successor(to, Ptr::null(), dir.clone());
        } else {
            let via = self.myNet.get_edge(&utils::xml::sumo_xml_definitions::get_edge_id_from_lane(&viaID));
            if via.is_null() {
                throw_error(ProcessError::Process(format!("unknown via-edge '{}' in connection", viaID)));
            }
            from.get_lanes()[fromLane as usize].m().add_outgoing_lane(to.get_lanes()[toLane as usize], via);
            from.m().add_successor(to, via, dir.clone());
            via.m().add_successor(to, Ptr::null(), dir.clone());
            let stateName = attrs.get::<String>(SUMO_ATTR_STATE, "", &mut ok, true);
            let state = parse_enum(&stateName, &*LinkStates, &mut ok, LINKSTATE_DEADEND);
            if state == LINKSTATE_MINOR || state == LINKSTATE_EQUAL || state == LINKSTATE_STOP || state == LINKSTATE_ALLWAY_STOP {
                via.m().set_time_penalty(self.myMinorPenalty);
            }
            let turn = LinkDirections.get_string(LinkDirection::TURN);
            let turnLefthand = LinkDirections.get_string(LinkDirection::TURN_LEFTHAND);
            if dir == turn || dir == turnLefthand {
                via.m().set_time_penalty(self.myTurnaroundPenalty);
            }
            if tlID != "" {
                via.m().set_time_penalty(self.myTLSPenalty);
            }
        }
        if to.is_crossing() {
            to.m().set_time_penalty(self.myTLSPenalty);
        }
    }

    /// C++: `void RONetHandler::parseStoppingPlace(const SUMOSAXAttributes &, const SumoXMLTag)` [protected]
    /// Parses a stopping place Called on the occurrence of a "busStop", "trainStop" or "containerStop" element
    // void
    // RONetHandler::parseStoppingPlace(const SUMOSAXAttributes& attrs, const SumoXMLTag element) {
    //     bool ok = true;
    //     myCurrentStoppingPlace = new SUMOVehicleParameter::Stop();
    //     // get the id, throw if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, toString(element).c_str(), ok);
    //     // get the lane
    //     myCurrentStoppingPlace->lane = attrs.get<std::string>(SUMO_ATTR_LANE, toString(element).c_str(), ok);
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //     const ROEdge* edge = myNet.getEdgeForLaneID(myCurrentStoppingPlace->lane);
    //     if (edge == nullptr) {
    //         throw InvalidArgument("Unknown lane '" + myCurrentStoppingPlace->lane + "' for " + toString(element) + " '" + id + "'.");
    //     }
    //     // get the positions
    //     myCurrentStoppingPlace->startPos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, id.c_str(), ok, 0.);
    //     myCurrentStoppingPlace->endPos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, id.c_str(), ok, edge->getLength());
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), ok, false);
    //     if (!ok || (SUMORouteHandler::checkStopPos(myCurrentStoppingPlace->startPos, myCurrentStoppingPlace->endPos, edge->getLength(), POSITION_EPS, friendlyPos) != SUMORouteHandler::StopPos::STOPPOS_VALID)) {
    //         throw InvalidArgument("Invalid position for " + toString(element) + " '" + id + "'.");
    //     }
    //     // this is a hack: the busstop attribute is meant to hold the id within the simulation context but this is not used within the router context
    //     myCurrentStoppingPlace->busstop = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok, "");
    //     // this is a hack: the actType is not used when using this to encode a stopping place
    //     myCurrentStoppingPlace->actType = toString(element);
    //     myNet.addStoppingPlace(id, element, myCurrentStoppingPlace);
    // }
    #[allow(unused_variables)]
    pub fn parse_stopping_place(&mut self, attrs: &SUMOSAXAttributes, element: SumoXMLTag) {
        let mut ok = true;
        let tagName = utils::xml::sumo_xml_definitions::Tags.get_string(element).to_string();
        let stopPtr = Ptr::from_raw(Box::into_raw(Box::new(new_stop())));
        self.myCurrentStoppingPlace = stopPtr;
        // get the id, throw if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, &tagName, &mut ok, true);
        // get the lane
        stopPtr.m().lane = attrs.get::<String>(SUMO_ATTR_LANE, &tagName, &mut ok, true);
        if !ok {
            throw_error(ProcessError::Process(String::new()));
        }
        let edge = self.myNet.get_edge_for_lane_id(&stopPtr.lane);
        if edge.is_null() {
            throw_error(ProcessError::InvalidArgument(format!("Unknown lane '{}' for {} '{}'.", stopPtr.lane, tagName, id)));
        }
        // get the positions
        stopPtr.m().startPos = attrs.get_opt::<f64>(SUMO_ATTR_STARTPOS, &id, &mut ok, 0., true);
        stopPtr.m().endPos = attrs.get_opt::<f64>(SUMO_ATTR_ENDPOS, &id, &mut ok, edge.get_length(), true);
        let friendlyPos = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut ok, false, true);
        if !ok || crate::utils::vehicle::sumo_route_handler::SUMORouteHandler::check_stop_pos(&mut stopPtr.m().startPos, &mut stopPtr.m().endPos, edge.get_length(), utils::geom::POSITION_EPS, friendlyPos) != crate::utils::vehicle::sumo_route_handler::SUMORouteHandler_StopPos::STOPPOS_VALID {
            throw_error(ProcessError::InvalidArgument(format!("Invalid position for {} '{}'.", tagName, id)));
        }
        // this is a hack: the busstop attribute is meant to hold the id within the simulation context but this is not used within the router context
        stopPtr.m().busstop = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);
        // this is a hack: the actType is not used when using this to encode a stopping place
        stopPtr.m().actType = tagName.clone();
        self.myNet.m().add_stopping_place(&id, element, stopPtr);
    }

    /// C++: `void RONetHandler::parseAccess(const SUMOSAXAttributes &)` [protected]
    /// Parses an access point to a train stop Called on the occurrence of an "access" element
    // void
    // RONetHandler::parseAccess(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string lane = attrs.get<std::string>(SUMO_ATTR_LANE, "access", ok);
    //     const ROEdge* edge = myNet.getEdgeForLaneID(lane);
    //     if (edge == nullptr) {
    //         throw InvalidArgument("Unknown lane '" + lane + "' for access.");
    //     }
    //     if ((edge->getPermissions() & SVC_PEDESTRIAN) == 0) {
    //         WRITE_WARNINGF(TL("Ignoring invalid access from non-pedestrian edge '%'."), edge->getID());
    //         return;
    //     }
    //     const bool random = attrs.getOpt<std::string>(SUMO_ATTR_POSITION, "access", ok) == "random";
    //     double pos = random ? edge->getLength() / 2. : attrs.getOpt<double>(SUMO_ATTR_POSITION, "access", ok, 0.);
    //     double length = attrs.getOpt<double>(SUMO_ATTR_LENGTH, "access", ok, -1);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, "access", ok, false);
    //     if (!ok || (SUMORouteHandler::checkStopPos(pos, pos, edge->getLength(), 0., friendlyPos) != SUMORouteHandler::StopPos::STOPPOS_VALID)) {
    //         throw InvalidArgument("Invalid position " + toString(pos) + " for access on lane '" + lane + "'.");
    //     }
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //     if (length < 0) {
    //         const Position accPos = myNet.getLane(lane)->geometryPositionAtOffset(pos);
    //         const double stopCenter = (myCurrentStoppingPlace->startPos + myCurrentStoppingPlace->endPos) / 2;
    //         const Position stopPos = myNet.getLane(myCurrentStoppingPlace->lane)->geometryPositionAtOffset(stopCenter);
    //         length  = accPos.distanceTo(stopPos);
    //     }
    //     myCurrentStoppingPlace->accessPos.push_back(std::make_tuple(lane, pos, length));
    // }
    #[allow(unused_variables)]
    pub fn parse_access(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let lane = attrs.get::<String>(SUMO_ATTR_LANE, "access", &mut ok, true);
        let edge = self.myNet.get_edge_for_lane_id(&lane);
        if edge.is_null() {
            throw_error(ProcessError::InvalidArgument(format!("Unknown lane '{}' for access.", lane)));
        }
        if (edge.get_permissions() & SVC_PEDESTRIAN) == 0 {
            write_warningf!("Ignoring invalid access from non-pedestrian edge '%'.", edge.get_id());
            return;
        }
        let random = attrs.get_opt::<String>(SUMO_ATTR_POSITION, "access", &mut ok, String::new(), true) == "random";
        let mut pos = if random { edge.get_length() / 2. } else { attrs.get_opt::<f64>(SUMO_ATTR_POSITION, "access", &mut ok, 0., true) };
        let mut length = attrs.get_opt::<f64>(SUMO_ATTR_LENGTH, "access", &mut ok, -1., true);
        let friendlyPos = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, "access", &mut ok, false, true);
        if !ok || !check_stop_pos_aliased(&mut pos, edge.get_length(), 0., friendlyPos) {
            throw_error(ProcessError::InvalidArgument(format!("Invalid position {} for access on lane '{}'.", utils::common::to_string::f64_to_string_prec(pos, utils::common::std_defs::g_precision()), lane)));
        }
        if !ok {
            throw_error(ProcessError::Process(String::new()));
        }
        let stop = self.myCurrentStoppingPlace;
        if length < 0. {
            let accPos = self.myNet.get_lane(&lane).geometry_position_at_offset(pos, 0.);
            let stopCenter = (stop.startPos + stop.endPos) / 2.;
            let stopPos = self.myNet.get_lane(&stop.lane).geometry_position_at_offset(stopCenter, 0.);
            length = accPos.distance_to(&stopPos);
        }
        // shortcut: SUMOVehicleParameter_Stop::accessPos is still Unmapped in the port, so the access entry (lane, pos, length) is not stored
        let _ = (lane, pos, length);
    }
}

/// C++ `SUMORouteHandler::checkStopPos(pos, pos, ...)` with both reference parameters bound to the same
/// variable (RONetHandler::parseAccess): every write to startPos or endPos lands in `pos`; true iff STOPPOS_VALID
fn check_stop_pos_aliased(pos: &mut f64, laneLength: f64, minLength: f64, friendlyPos: bool) -> bool {
    if minLength > laneLength {
        return false;
    }
    if *pos < 0. {
        *pos += laneLength;
    }
    if *pos < 0. {
        *pos += laneLength;
    }
    if (*pos < minLength) || (*pos > laneLength) {
        if !friendlyPos {
            return false;
        }
        if *pos < minLength {
            *pos = minLength;
        }
        if *pos > laneLength {
            *pos = laneLength;
        }
    }
    if (*pos < 0.) || (*pos > (*pos - minLength)) {
        if !friendlyPos {
            return false;
        }
        if *pos < 0. {
            *pos = 0.;
        }
        if *pos > (*pos - minLength) {
            *pos = *pos - minLength;
        }
    }
    true
}

impl RONetHandler {

    /// C++: `void RONetHandler::parseDistrict(const SUMOSAXAttributes &)` [protected]
    /// Parses a district and creates a pseudo edge for it
    // void
    // RONetHandler::parseDistrict(const SUMOSAXAttributes& attrs) {
    //     myCurrentEdge = nullptr;
    //     bool ok = true;
    //     myCurrentName = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     ROEdge* const sink = myEdgeBuilder.buildEdge(myCurrentName + "-sink", nullptr, nullptr, 0, "", "");
    //     ROEdge* const source = myEdgeBuilder.buildEdge(myCurrentName + "-source", nullptr, nullptr, 0, "", "");
    //     myNet.addDistrict(myCurrentName, source, sink);
    //     if (attrs.hasAttribute(SUMO_ATTR_EDGES)) {
    //         const std::vector<std::string>& desc = attrs.get<std::vector<std::string> >(SUMO_ATTR_EDGES, myCurrentName.c_str(), ok);
    //         for (const std::string& eID : desc) {
    //             myNet.addDistrictEdge(myCurrentName, eID, true);
    //             myNet.addDistrictEdge(myCurrentName, eID, false);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_district(&mut self, attrs: &SUMOSAXAttributes) {
        self.myCurrentEdge = Ptr::null();
        let mut ok = true;
        self.myCurrentName = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            return;
        }
        let name = self.myCurrentName.clone();
        let sink = self.myEdgeBuilder.m().build_edge(&format!("{}-sink", name), Ptr::null(), Ptr::null(), 0, "", "");
        let source = self.myEdgeBuilder.m().build_edge(&format!("{}-source", name), Ptr::null(), Ptr::null(), 0, "", "");
        self.myNet.m().add_district(name.clone(), source, sink);
        if attrs.has_attribute(SUMO_ATTR_EDGES) {
            let desc = attrs.get::<Vec<String>>(SUMO_ATTR_EDGES, &name, &mut ok, true);
            for eID in desc.iter() {
                self.myNet.m().add_district_edge(name.clone(), eID.clone(), true);
                self.myNet.m().add_district_edge(name.clone(), eID.clone(), false);
            }
        }
    }

    /// C++: `void RONetHandler::parseDistrictEdge(const SUMOSAXAttributes &, bool)` [protected]
    /// Parses a district edge and connects it to the district
    // void
    // RONetHandler::parseDistrictEdge(const SUMOSAXAttributes& attrs, bool isSource) {
    //     bool ok = true;
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, myCurrentName.c_str(), ok);
    //     myNet.addDistrictEdge(myCurrentName, id, isSource);
    // }
    #[allow(unused_variables)]
    pub fn parse_district_edge(&mut self, attrs: &SUMOSAXAttributes, isSource: bool) {
        let mut ok = true;
        let name = self.myCurrentName.clone();
        let id = attrs.get::<String>(SUMO_ATTR_ID, &name, &mut ok, true);
        self.myNet.m().add_district_edge(name, id, isSource);
    }

    /// C++: `void RONetHandler::setLocation(const SUMOSAXAttributes &)` [protected]
    /// Parses network location description
    // void
    // RONetHandler::setLocation(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     PositionVector s = attrs.get<PositionVector>(SUMO_ATTR_NET_OFFSET, nullptr, ok);
    //     Boundary convBoundary = attrs.get<Boundary>(SUMO_ATTR_CONV_BOUNDARY, nullptr, ok);
    //     Boundary origBoundary = attrs.get<Boundary>(SUMO_ATTR_ORIG_BOUNDARY, nullptr, ok);
    //     std::string proj = attrs.get<std::string>(SUMO_ATTR_ORIG_PROJ, nullptr, ok);
    //     if (ok) {
    //         Position networkOffset = s[0];
    //         GeoConvHelper::init(proj, networkOffset, origBoundary, convBoundary);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_location(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let s = attrs.get::<PositionVector>(SUMO_ATTR_NET_OFFSET, "", &mut ok, true);
        let convBoundary = attrs.get::<Boundary>(SUMO_ATTR_CONV_BOUNDARY, "", &mut ok, true);
        let origBoundary = attrs.get::<Boundary>(SUMO_ATTR_ORIG_BOUNDARY, "", &mut ok, true);
        let proj = attrs.get::<String>(SUMO_ATTR_ORIG_PROJ, "", &mut ok, true);
        if ok {
            let networkOffset = *s.at(0);
            GeoConvHelper::init(&proj, networkOffset, origBoundary, convBoundary, 1.0);
        }
    }

    /// C++: `void RONetHandler::RONetHandler(const RONetHandler &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &RONetHandler) -> Self {
        // C++ declares this copy constructor private and never defines it, so it can not be called
        panic!("RONetHandler copy constructor is private and not defined in C++");
    }

    /// C++: `RONetHandler & RONetHandler::operator=(const RONetHandler &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RONetHandler) -> &mut RONetHandler {
        // C++ declares this assignment operator private and never defines it, so it can not be called
        panic!("RONetHandler assignment operator is private and not defined in C++");
    }

}

