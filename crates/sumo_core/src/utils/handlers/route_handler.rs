//! Port of utils/handlers/RouteHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::handlers::common_handler::{attr_str, tag_str};
use utils::common::msg_handler::write_warning;
use utils::common::std_defs::INVALID_DOUBLE;

// shortcut: NamespaceIDs is not ported (utils/xml/mod.rs); lists copied from NamespaceIDs.cpp without the GNE_* netedit tags, upgrade when NamespaceIDs lands
const NS_ROUTES: [SumoXMLTag; 2] = [SUMO_TAG_ROUTE, SUMO_TAG_ROUTE_DISTRIBUTION];
const NS_VEHICLES: [SumoXMLTag; 3] = [SUMO_TAG_TRIP, SUMO_TAG_FLOW, SUMO_TAG_VEHICLE];
const NS_PERSONS: [SumoXMLTag; 2] = [SUMO_TAG_PERSON, SUMO_TAG_PERSONFLOW];
const NS_CONTAINERS: [SumoXMLTag; 2] = [SUMO_TAG_CONTAINER, SUMO_TAG_CONTAINERFLOW];

// SUMOVehicleParameter.h Stop::parametersSet flags (C++ macros)
const STOP_START_SET: i32 = 1;
const STOP_END_SET: i32 = 2;
const STOP_DURATION_SET: i32 = 2 << 1;
const STOP_UNTIL_SET: i32 = 2 << 2;
const STOP_EXTENSION_SET: i32 = 2 << 3;
const STOP_TRIGGER_SET: i32 = 2 << 4;
const STOP_PARKING_SET: i32 = 2 << 5;
const STOP_EXPECTED_SET: i32 = 2 << 6;
const STOP_CONTAINER_TRIGGER_SET: i32 = 2 << 7;
const STOP_EXPECTED_CONTAINERS_SET: i32 = 2 << 8;
const STOP_TRIP_ID_SET: i32 = 2 << 9;
const STOP_LINE_SET: i32 = 2 << 10;
const STOP_SPEED_SET: i32 = 2 << 11;
const STOP_SPLIT_SET: i32 = 2 << 12;
const STOP_JOIN_SET: i32 = 2 << 13;
const STOP_ARRIVAL_SET: i32 = 2 << 14;
const STOP_PERMITTED_SET: i32 = 2 << 15;
const STOP_ENDED_SET: i32 = 2 << 16;
const STOP_STARTED_SET: i32 = 2 << 17;
const STOP_POSLAT_SET: i32 = 2 << 18;
const STOP_ONDEMAND_SET: i32 = 2 << 19;
const STOP_JUMP_SET: i32 = 2 << 20;
const STOP_JUMP_UNTIL_SET: i32 = 2 << 21;
const STOP_INDEX_END: i32 = -1;
const STOP_INDEX_FIT: i32 = -2;

/// C++ `throw` of a ProcessError: aborts parsing like the uncaught C++ exception
fn throw_error(e: ProcessError) -> ! {
    panic!("{}", e)
}

/// unwrap a CommonXMLStructure getter; the C++ getters throw when the attribute is missing
fn or_throw<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| throw_error(e))
}

fn str_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> String {
    or_throw(obj.get_string_attribute(attr)).clone()
}
fn int_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> i32 {
    or_throw(obj.get_int_attribute(attr))
}
fn double_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> f64 {
    or_throw(obj.get_double_attribute(attr))
}
fn time_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> SUMOTime {
    or_throw(obj.get_time_attribute(attr))
}
fn color_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> RGBColor {
    *or_throw(obj.get_color_attribute(attr))
}
fn str_list_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> Vec<String> {
    or_throw(obj.get_string_list_attribute(attr)).clone()
}
fn double_list_attr(obj: Ptr<CommonXMLStructure_SumoBaseObject>, attr: SumoXMLAttr) -> Vec<f64> {
    or_throw(obj.get_double_list_attribute(attr)).clone()
}
/// C++ `getVehicleParameter()` (SUMOVehicleParameter&) of a SumoBaseObject
fn vehicle_parameter_of(obj: Ptr<CommonXMLStructure_SumoBaseObject>) -> &'static SUMOVehicleParameter {
    or_throw(obj.get().unwrap().get_vehicle_parameter()).downcast_ref::<SUMOVehicleParameter>().expect("SumoBaseObject does not hold a SUMOVehicleParameter")
}
/// C++ `getVehicleTypeParameter()` (SUMOVTypeParameter&) of a SumoBaseObject
fn vehicle_type_parameter_of(obj: Ptr<CommonXMLStructure_SumoBaseObject>) -> &'static SUMOVTypeParameter {
    or_throw(obj.get().unwrap().get_vehicle_type_parameter()).downcast_ref::<SUMOVTypeParameter>().expect("SumoBaseObject does not hold a SUMOVTypeParameter")
}
/// C++ `SUMOVehicleParameter::Stop stop;` (default member values of utils/vehicle/SUMOVehicleParameter.h)
// shortcut: Stop has no Default impl yet (libsumo/helper.rs and ro_net_handler.rs build it the same way), move this into sumo_vehicle_parameter.rs
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
        posLat: INVALID_DOUBLE,
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

/// C++ `new`-allocated parameter that the caller `delete`s after the copy went into the XML structure:
/// the Box takes over the allocation, so dropping it later is the C++ `delete`
fn take_parameter<T>(p: Ptr<T>) -> Box<T> {
    // SAFETY: the SUMOVehicleParserHelper parse functions hand out Box::leak/Box::into_raw allocations and nothing else owns them
    unsafe { Box::from_raw(p.as_raw()) }
}

/// C++ class `RouteHandler` (utils/handlers/RouteHandler.h)
pub struct RouteHandler {
    /// C++ base class `CommonHandler`
    pub base: CommonHandler,
    /// enable or disable hardFail (stop parsing if parameter aren't correct)
    pub myHardFail: bool,
    /// The default value for flow begins
    pub myFlowBeginDefault: SUMOTime,
    /// The default value for flow ends
    pub myFlowEndDefault: SUMOTime,
}

impl std::ops::Deref for RouteHandler {
    type Target = CommonHandler;
    fn deref(&self) -> &CommonHandler {
        &self.base
    }
}
impl std::ops::DerefMut for RouteHandler {
    fn deref_mut(&mut self) -> &mut CommonHandler {
        &mut self.base
    }
}

impl RouteHandler {
    /// C++: `void RouteHandler::RouteHandler(FileBucket *, const bool)`
    /// Constructor
    // RouteHandler::RouteHandler(FileBucket* fileBucket, const bool hardFail) :
    //     CommonHandler(fileBucket),
    //     myHardFail(hardFail),
    //     myFlowBeginDefault(string2time(OptionsCont::getOptions().getString("begin"))),
    //     myFlowEndDefault(string2time(OptionsCont::getOptions().getString("end"))) {
    // }
    #[allow(unused_variables)]
    pub fn new(fileBucket: Ptr<Unmapped /* FileBucket */>, hardFail: bool) -> Self {
        Self {
            base: CommonHandler::new(fileBucket),
            myHardFail: hardFail,
            myFlowBeginDefault: or_throw(string2time(&utils::options::options_cont::get_string("begin"))),
            myFlowEndDefault: or_throw(string2time(&utils::options::options_cont::get_string("end"))),
        }
    }

    /// C++: `void RouteHandler::~RouteHandler() noexcept` [virtual]
    /// Destructor
    // RouteHandler::~RouteHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor has an empty body
    }

    /// C++: `bool RouteHandler::beginParseAttributes(SumoXMLTag, const SUMOSAXAttributes &)`
    /// begin parse attributes
    // bool
    // RouteHandler::beginParseAttributes(SumoXMLTag tag, const SUMOSAXAttributes& attrs) {
    //     // open SUMOBaseOBject
    //     myCommonXMLStructure.openSUMOBaseOBject();
    //     // check tag
    //     try {
    //         switch (tag) {
    //             // vTypes
    //             case SUMO_TAG_VTYPE:
    //                 // continue depeding if we're parsing a vType or a reference
    //                 if ((myCommonXMLStructure.getSumoBaseObjectRoot()->getTag() == SUMO_TAG_VTYPE_DISTRIBUTION) &&
    //                         attrs.hasAttribute(SUMO_ATTR_REFID)) {
    //                     parseVTypeRef(attrs);
    //                 } else {
    //                     parseVType(attrs);
    //                 }
    //                 break;
    //             case SUMO_TAG_VTYPE_DISTRIBUTION:
    //                 parseVTypeDistribution(attrs);
    //                 break;
    //             // routes
    //             case SUMO_TAG_ROUTE: {
    //                 // continue depeding if we're parsing a basic route, an embedded route, or a distribution ref
    //                 const auto parentTag = myCommonXMLStructure.getSumoBaseObjectRoot()->getTag();
    //                 // this is temporal, until #16476
    //                 if (parentTag != SUMO_TAG_ERROR) {
    //                     if ((parentTag == SUMO_TAG_VEHICLE) || (parentTag == SUMO_TAG_FLOW)) {
    //                         parseRouteEmbedded(attrs);
    //                     } else if ((parentTag == SUMO_TAG_ROUTE_DISTRIBUTION) && attrs.hasAttribute(SUMO_ATTR_REFID)) {
    //                         parseRouteRef(attrs);
    //                     } else {
    //                         parseRoute(attrs);
    //                     }
    //                 }
    //                 break;
    //             }
    //             case SUMO_TAG_ROUTE_DISTRIBUTION:
    //                 parseRouteDistribution(attrs);
    //                 break;
    //             // vehicles
    //             case SUMO_TAG_TRIP:
    //                 parseTrip(attrs);
    //                 break;
    //             case SUMO_TAG_VEHICLE:
    //                 parseVehicle(attrs);
    //                 break;
    //             // flows
    //             case SUMO_TAG_FLOW:
    //                 parseFlow(attrs);
    //                 break;
    //             // stop
    //             case SUMO_TAG_STOP:
    //                 parseStop(attrs);
    //                 break;
    //             // persons
    //             case SUMO_TAG_PERSON:
    //                 parsePerson(attrs);
    //                 break;
    //             case SUMO_TAG_PERSONFLOW:
    //                 parsePersonFlow(attrs);
    //                 break;
    //             // person plans
    //             case SUMO_TAG_PERSONTRIP:
    //                 parsePersonTrip(attrs);
    //                 break;
    //             case SUMO_TAG_RIDE:
    //                 parseRide(attrs);
    //                 break;
    //             case SUMO_TAG_WALK:
    //                 parseWalk(attrs);
    //                 break;
    //             // container
    //             case SUMO_TAG_CONTAINER:
    //                 parseContainer(attrs);
    //                 break;
    //             case SUMO_TAG_CONTAINERFLOW:
    //                 parseContainerFlow(attrs);
    //                 break;
    //             // container plans
    //             case SUMO_TAG_TRANSPORT:
    //                 parseTransport(attrs);
    //                 break;
    //             case SUMO_TAG_TRANSHIP:
    //                 parseTranship(attrs);
    //                 break;
    //             // parameters
    //             case SUMO_TAG_PARAM:
    //                 parseParameters(attrs);
    //                 break;
    //             // other
    //             case SUMO_TAG_INTERVAL: {
    //                 parseInterval(attrs);
    //                 break;
    //             }
    //             default:
    //                 // get vehicle type Base object
    //                 const auto vTypeObject = myCommonXMLStructure.getCurrentSumoBaseObject()->getParentSumoBaseObject();
    //                 // parse embedded car following model information
    //                 if (vTypeObject && (vTypeObject->getTag() == SUMO_TAG_VTYPE)) {
    //                     // nested CFM attributes
    //                     return parseNestedCFM(tag, attrs, vTypeObject);
    //                 } else {
    //                     // tag cannot be parsed in routeHandler
    //                     myCommonXMLStructure.abortSUMOBaseOBject();
    //                     return false;
    //                 }
    //         }
    //     } catch (InvalidArgument& e) {
    //         WRITE_ERROR(e.what());
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn begin_parse_attributes(&mut self, tag: SumoXMLTag, attrs: &SUMOSAXAttributes) -> bool {
        // open SUMOBaseOBject
        self.myCommonXMLStructure.open_sumo_base_object();
        // check tag
        // shortcut: C++ try/catch(InvalidArgument) around the switch is not reproduced: attribute getters here return Result and or_throw panics instead, upgrade when InvalidArgument is raised by the port
        match tag {
            // vTypes
            SUMO_TAG_VTYPE => {
                // continue depeding if we're parsing a vType or a reference
                if self.myCommonXMLStructure.get_sumo_base_object_root().get_tag() == SUMO_TAG_VTYPE_DISTRIBUTION && attrs.has_attribute(SUMO_ATTR_REFID) {
                    self.parse_v_type_ref(attrs);
                } else {
                    self.parse_v_type(attrs);
                }
            }
            SUMO_TAG_VTYPE_DISTRIBUTION => {
                self.parse_v_type_distribution(attrs);
            }
            // routes
            SUMO_TAG_ROUTE => {
                // continue depeding if we're parsing a basic route, an embedded route, or a distribution ref
                let parentTag = self.myCommonXMLStructure.get_sumo_base_object_root().get_tag();
                // this is temporal, until #16476
                if parentTag != SUMO_TAG_ERROR {
                    if parentTag == SUMO_TAG_VEHICLE || parentTag == SUMO_TAG_FLOW {
                        self.parse_route_embedded(attrs);
                    } else if parentTag == SUMO_TAG_ROUTE_DISTRIBUTION && attrs.has_attribute(SUMO_ATTR_REFID) {
                        self.parse_route_ref(attrs);
                    } else {
                        self.parse_route(attrs);
                    }
                }
            }
            SUMO_TAG_ROUTE_DISTRIBUTION => {
                self.parse_route_distribution(attrs);
            }
            // vehicles
            SUMO_TAG_TRIP => {
                self.parse_trip(attrs);
            }
            SUMO_TAG_VEHICLE => {
                self.parse_vehicle(attrs);
            }
            // flows
            SUMO_TAG_FLOW => {
                self.parse_flow(attrs);
            }
            // stop
            SUMO_TAG_STOP => {
                self.parse_stop(attrs);
            }
            // persons
            SUMO_TAG_PERSON => {
                self.parse_person(attrs);
            }
            SUMO_TAG_PERSONFLOW => {
                self.parse_person_flow(attrs);
            }
            // person plans
            SUMO_TAG_PERSONTRIP => {
                self.parse_person_trip(attrs);
            }
            SUMO_TAG_RIDE => {
                self.parse_ride(attrs);
            }
            SUMO_TAG_WALK => {
                self.parse_walk(attrs);
            }
            // container
            SUMO_TAG_CONTAINER => {
                self.parse_container(attrs);
            }
            SUMO_TAG_CONTAINERFLOW => {
                self.parse_container_flow(attrs);
            }
            // container plans
            SUMO_TAG_TRANSPORT => {
                self.parse_transport(attrs);
            }
            SUMO_TAG_TRANSHIP => {
                self.parse_tranship(attrs);
            }
            // parameters
            SUMO_TAG_PARAM => {
                self.parse_parameters(attrs);
            }
            // other
            SUMO_TAG_INTERVAL => {
                self.parse_interval(attrs);
            }
            _ => {
                // get vehicle type Base object
                let vTypeObject = self.myCommonXMLStructure.get_current_sumo_base_object().get_parent_sumo_base_object();
                // parse embedded car following model information
                if !vTypeObject.is_null() && vTypeObject.get_tag() == SUMO_TAG_VTYPE {
                    // nested CFM attributes
                    return self.parse_nested_cfm(tag, attrs, vTypeObject);
                } else {
                    // tag cannot be parsed in routeHandler
                    self.myCommonXMLStructure.abort_sumo_base_object();
                    return false;
                }
            }
        }
        true
    }

    /// C++: `void RouteHandler::endParseAttributes()`
    /// end parse attributes
    // void
    // RouteHandler::endParseAttributes() {
    //     // get last inserted object
    //     CommonXMLStructure::SumoBaseObject* obj = myCommonXMLStructure.getCurrentSumoBaseObject();
    //     // check tag
    //     if (obj) {
    //         // close SUMOBaseOBject
    //         myCommonXMLStructure.closeSUMOBaseOBject();
    //         switch (obj->getTag()) {
    //             case SUMO_TAG_ROUTE:
    //                 // special case, because embedded and distribution routes are created within other elements
    //                 if (obj->getParentSumoBaseObject() == nullptr) {
    //                     // parse object and all their childrens
    //                     parseSumoBaseObject(obj);
    //                     // delete object (and all of their childrens)
    //                     delete obj;
    //                 }
    //                 break;
    //             case SUMO_TAG_ROUTE_DISTRIBUTION:
    //                 // parse object and all their childrens
    //                 parseSumoBaseObject(obj);
    //                 // delete object (and all of their childrens)
    //                 delete obj;
    //                 break;
    //             case SUMO_TAG_VTYPE_DISTRIBUTION:
    //                 // parse object and all their childrens
    //                 parseSumoBaseObject(obj);
    //                 // delete object (and all of their childrens)
    //                 delete obj;
    //                 break;
    //             case SUMO_TAG_VTYPE:
    //                 // special case, because embedded and distribution routes are created within other elements
    //                 if (obj->getParentSumoBaseObject() == nullptr) {
    //                     // parse object and all their childrens
    //                     parseSumoBaseObject(obj);
    //                     // delete object (and all of their childrens)
    //                     delete obj;
    //                 }
    //                 break;
    //             case SUMO_TAG_TRIP:
    //             case SUMO_TAG_VEHICLE:
    //             case SUMO_TAG_FLOW:
    //             case SUMO_TAG_PERSON:
    //             case SUMO_TAG_PERSONFLOW:
    //             case SUMO_TAG_CONTAINER:
    //             case SUMO_TAG_CONTAINERFLOW:
    //                 // parse object and all their childrens
    //                 parseSumoBaseObject(obj);
    //                 // delete object (and all of their childrens)
    //                 delete obj;
    //                 break;
    //             default:
    //                 break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn end_parse_attributes(&mut self) {
        // get last inserted object
        let obj = self.myCommonXMLStructure.get_current_sumo_base_object();
        // check tag
        if !obj.is_null() {
            // close SUMOBaseOBject
            self.myCommonXMLStructure.close_sumo_base_object();
            // shortcut: C++ `delete obj` after each parseSumoBaseObject is not reproduced: close_sumo_base_object keeps the closed root allocated, so the object lives until the structure is cleared
            match obj.get_tag() {
                SUMO_TAG_ROUTE => {
                    // special case, because embedded and distribution routes are created within other elements
                    if obj.get_parent_sumo_base_object().is_null() {
                        // parse object and all their childrens
                        self.parse_sumo_base_object(obj);
                    }
                }
                SUMO_TAG_ROUTE_DISTRIBUTION => {
                    // parse object and all their childrens
                    self.parse_sumo_base_object(obj);
                }
                SUMO_TAG_VTYPE_DISTRIBUTION => {
                    // parse object and all their childrens
                    self.parse_sumo_base_object(obj);
                }
                SUMO_TAG_VTYPE => {
                    // special case, because embedded and distribution routes are created within other elements
                    if obj.get_parent_sumo_base_object().is_null() {
                        // parse object and all their childrens
                        self.parse_sumo_base_object(obj);
                    }
                }
                SUMO_TAG_TRIP | SUMO_TAG_VEHICLE | SUMO_TAG_FLOW | SUMO_TAG_PERSON | SUMO_TAG_PERSONFLOW | SUMO_TAG_CONTAINER | SUMO_TAG_CONTAINERFLOW => {
                    // parse object and all their childrens
                    self.parse_sumo_base_object(obj);
                }
                _ => {}
            }
        }
    }

    /// C++: `void RouteHandler::parseSumoBaseObject(CommonXMLStructure::SumoBaseObject *)`
    /// parse SumoBaseObject (it's called recursivelly)
    // void
    // RouteHandler::parseSumoBaseObject(CommonXMLStructure::SumoBaseObject* obj) {
    //     // check if loading was aborted
    //     if (!myAbortLoading) {
    //         // switch tag
    //         switch (obj->getTag()) {
    //             // vTypes
    //             case SUMO_TAG_VTYPE:
    //                 // check if parse vType or Ref
    //                 if (obj->hasStringAttribute(SUMO_ATTR_REFID)) {
    //                     if (buildVTypeRef(obj,
    //                                       obj->getStringAttribute(SUMO_ATTR_REFID),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_PROB))) {
    //                         obj->markAsCreated();
    //                     }
    //                 } else {
    //                     if (buildVType(obj,
    //                                    obj->getVehicleTypeParameter())) {
    //                         obj->markAsCreated();
    //                     }
    //                 }
    //                 break;
    //             case SUMO_TAG_VTYPE_DISTRIBUTION:
    //                 if (buildVTypeDistribution(obj,
    //                                            obj->getStringAttribute(SUMO_ATTR_ID),
    //                                            obj->getIntAttribute(SUMO_ATTR_DETERMINISTIC))) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             // route
    //             case SUMO_TAG_ROUTE:
    //                 // embedded routes are created in build<Vehicle/Flow>EmbeddedRoute
    //                 if (obj->hasStringAttribute(SUMO_ATTR_REFID)) {
    //                     if (buildRouteRef(obj,
    //                                       obj->getStringAttribute(SUMO_ATTR_REFID),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_PROB))) {
    //                         obj->markAsCreated();
    //                     }
    //                 } else if (obj->hasStringAttribute(SUMO_ATTR_ID)) {
    //                     if (buildRoute(obj,
    //                                    obj->getStringAttribute(SUMO_ATTR_ID),
    //                                    obj->getVClass(),
    //                                    obj->getStringListAttribute(SUMO_ATTR_EDGES),
    //                                    obj->getColorAttribute(SUMO_ATTR_COLOR),
    //                                    obj->getIntAttribute(SUMO_ATTR_REPEAT),
    //                                    obj->getTimeAttribute(SUMO_ATTR_CYCLETIME),
    //                                    obj->getDoubleAttribute(SUMO_ATTR_PROB),
    //                                    obj->getParameters())) {
    //                         obj->markAsCreated();
    //                     }
    //                 }
    //                 break;
    //             case SUMO_TAG_ROUTE_DISTRIBUTION:
    //                 if (buildRouteDistribution(obj,
    //                                            obj->getStringAttribute(SUMO_ATTR_ID))) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             // vehicles
    //             case SUMO_TAG_TRIP:
    //                 if (checkVehicleParents(obj)) {
    //                     if (isOverFromToEdges(obj)) {
    //                         // build trip with from-to edges
    //                         if (buildTrip(obj,
    //                                       obj->getVehicleParameter(),
    //                                       obj->hasStringAttribute(SUMO_ATTR_FROM) ? obj->getStringAttribute(SUMO_ATTR_FROM) : "",
    //                                       obj->hasStringAttribute(SUMO_ATTR_TO) ? obj->getStringAttribute(SUMO_ATTR_TO) : "")) {
    //                             obj->markAsCreated();
    //                         }
    //                     } else if (isOverFromToJunctions(obj)) {
    //                         // build trip with from-to junctions
    //                         if (buildTripJunctions(obj,
    //                                                obj->getVehicleParameter(),
    //                                                obj->getStringAttribute(SUMO_ATTR_FROM_JUNCTION),
    //                                                obj->getStringAttribute(SUMO_ATTR_TO_JUNCTION))) {
    //                             obj->markAsCreated();
    //                         }
    //                     } else if (isOverFromToTAZs(obj)) {
    //                         // build trip with from-to TAZs
    //                         if (buildTripTAZs(obj,
    //                                           obj->getVehicleParameter(),
    //                                           obj->getStringAttribute(SUMO_ATTR_FROM_TAZ),
    //                                           obj->getStringAttribute(SUMO_ATTR_TO_TAZ))) {
    //                             obj->markAsCreated();
    //                         }
    //                     }
    //                 }
    //                 break;
    //             case SUMO_TAG_VEHICLE:
    //                 if (checkVehicleParents(obj)) {
    //                     if (obj->hasStringAttribute(SUMO_ATTR_ROUTE)) {
    //                         // build vehicle over route
    //                         if (buildVehicleOverRoute(obj,
    //                                                   obj->getVehicleParameter())) {
    //                             obj->markAsCreated();
    //                         }
    //                     } else {
    //                         const auto embeddedRoute = getEmbeddedRoute(obj);
    //                         if (embeddedRoute) {
    //                             // build vehicle with embedded route
    //                             if (buildVehicleEmbeddedRoute(obj,
    //                                                           obj->getVehicleParameter(),
    //                                                           embeddedRoute->getStringListAttribute(SUMO_ATTR_EDGES),
    //                                                           embeddedRoute->getColorAttribute(SUMO_ATTR_COLOR),
    //                                                           embeddedRoute->getIntAttribute(SUMO_ATTR_REPEAT),
    //                                                           embeddedRoute->getTimeAttribute(SUMO_ATTR_CYCLETIME),
    //                                                           embeddedRoute->getParameters())) {
    //                                 obj->markAsCreated();
    //                             }
    //                         }
    //                     }
    //                 }
    //                 break;
    //             // flows
    //             case SUMO_TAG_FLOW:
    //                 if (checkVehicleParents(obj)) {
    //                     if (obj->hasStringAttribute(SUMO_ATTR_ROUTE)) {
    //                         // build flow over route
    //                         if (buildFlowOverRoute(obj,
    //                                                obj->getVehicleParameter())) {
    //                             obj->markAsCreated();
    //                         }
    //                     } else if (isOverFromToEdges(obj)) {
    //                         // build flow with from-to edges
    //                         if (buildFlow(obj,
    //                                       obj->getVehicleParameter(),
    //                                       obj->getStringAttribute(SUMO_ATTR_FROM),
    //                                       obj->getStringAttribute(SUMO_ATTR_TO))) {
    //                             obj->markAsCreated();
    //                         }
    //                     } else if (isOverFromToJunctions(obj)) {
    //                         // build flow with from-to junctions
    //                         if (buildFlowJunctions(obj,
    //                                                obj->getVehicleParameter(),
    //                                                obj->getStringAttribute(SUMO_ATTR_FROM_JUNCTION),
    //                                                obj->getStringAttribute(SUMO_ATTR_TO_JUNCTION))) {
    //                             obj->markAsCreated();
    //                         }
    //                     } else if (isOverFromToTAZs(obj)) {
    //                         // build flow with from-to TAZs
    //                         if (buildFlowTAZs(obj,
    //                                           obj->getVehicleParameter(),
    //                                           obj->getStringAttribute(SUMO_ATTR_FROM_TAZ),
    //                                           obj->getStringAttribute(SUMO_ATTR_TO_TAZ))) {
    //                             obj->markAsCreated();
    //                         }
    //
    //                     } else {
    //                         const auto embeddedRoute = getEmbeddedRoute(obj);
    //                         if (embeddedRoute) {
    //                             // build flow with embedded route
    //                             if (buildFlowEmbeddedRoute(obj,
    //                                                        obj->getVehicleParameter(),
    //                                                        embeddedRoute->getStringListAttribute(SUMO_ATTR_EDGES),
    //                                                        embeddedRoute->getColorAttribute(SUMO_ATTR_COLOR),
    //                                                        embeddedRoute->getIntAttribute(SUMO_ATTR_REPEAT),
    //                                                        embeddedRoute->getTimeAttribute(SUMO_ATTR_CYCLETIME),
    //                                                        embeddedRoute->getParameters())) {
    //                                 obj->markAsCreated();
    //                             }
    //                         }
    //                     }
    //                 }
    //                 break;
    //             // persons
    //             case SUMO_TAG_PERSON:
    //                 if (buildPerson(obj,
    //                                 obj->getVehicleParameter())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_PERSONFLOW:
    //                 if (buildPersonFlow(obj,
    //                                     obj->getVehicleParameter())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             // person plans
    //             case SUMO_TAG_PERSONTRIP:
    //                 if (checkPersonPlanParents(obj)) {
    //                     if (buildPersonTrip(obj,
    //                                         obj->getPlanParameters(),
    //                                         obj->getDoubleAttribute(SUMO_ATTR_ARRIVALPOS),
    //                                         obj->getStringListAttribute(SUMO_ATTR_VTYPES),
    //                                         obj->getStringListAttribute(SUMO_ATTR_MODES),
    //                                         obj->getStringListAttribute(SUMO_ATTR_LINES),
    //                                         obj->getDoubleAttribute(SUMO_ATTR_WALKFACTOR),
    //                                         obj->getStringAttribute(SUMO_ATTR_GROUP))) {
    //                         obj->markAsCreated();
    //                     }
    //                 }
    //                 break;
    //             case SUMO_TAG_RIDE:
    //                 if (checkPersonPlanParents(obj)) {
    //                     if (buildRide(obj,
    //                                   obj->getPlanParameters(),
    //                                   obj->getDoubleAttribute(SUMO_ATTR_ARRIVALPOS),
    //                                   obj->getStringListAttribute(SUMO_ATTR_LINES),
    //                                   obj->getStringAttribute(SUMO_ATTR_GROUP))) {
    //                         obj->markAsCreated();
    //                     }
    //                 }
    //                 break;
    //             case SUMO_TAG_WALK:
    //                 if (checkPersonPlanParents(obj)) {
    //                     if (buildWalk(obj,
    //                                   obj->getPlanParameters(),
    //                                   obj->getDoubleAttribute(SUMO_ATTR_ARRIVALPOS),
    //                                   obj->getDoubleAttribute(SUMO_ATTR_SPEED),
    //                                   obj->getTimeAttribute(SUMO_ATTR_DURATION))) {
    //                         obj->markAsCreated();
    //                     }
    //                 }
    //                 break;
    //             // container
    //             case SUMO_TAG_CONTAINER:
    //                 if (buildContainer(obj,
    //                                    obj->getVehicleParameter())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             case SUMO_TAG_CONTAINERFLOW:
    //                 if (buildContainerFlow(obj,
    //                                        obj->getVehicleParameter())) {
    //                     obj->markAsCreated();
    //                 }
    //                 break;
    //             // container plans
    //             case SUMO_TAG_TRANSPORT:
    //                 if (checkContainerPlanParents(obj)) {
    //                     if (buildTransport(obj,
    //                                        obj->getPlanParameters(),
    //                                        obj->getDoubleAttribute(SUMO_ATTR_ARRIVALPOS),
    //                                        obj->getStringListAttribute(SUMO_ATTR_LINES),
    //                                        obj->getStringAttribute(SUMO_ATTR_GROUP))) {
    //                         obj->markAsCreated();
    //                     }
    //                 }
    //                 break;
    //             case SUMO_TAG_TRANSHIP:
    //                 if (checkContainerPlanParents(obj)) {
    //                     if (buildTranship(obj,
    //                                       obj->getPlanParameters(),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_ARRIVALPOS),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_DEPARTPOS),
    //                                       obj->getDoubleAttribute(SUMO_ATTR_SPEED),
    //                                       obj->getTimeAttribute(SUMO_ATTR_DURATION))) {
    //                         obj->markAsCreated();
    //                     }
    //                 }
    //                 break;
    //             // stops
    //             case SUMO_TAG_STOP:
    //                 if (checkStopParents(obj)) {
    //                     if (buildStop(obj,
    //                                   obj->getPlanParameters(),
    //                                   obj->getStopParameter())) {
    //                         obj->markAsCreated();
    //                     }
    //                 }
    //                 break;
    //             default:
    //                 break;
    //         }
    //         // now iterate over childrens
    //         for (const auto& child : obj->getSumoBaseObjectChildren()) {
    //             // call this function recursively
    //             parseSumoBaseObject(child);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_sumo_base_object(&mut self, obj: Ptr<CommonXMLStructure_SumoBaseObject>) {
        // check if loading was aborted
        if !self.myAbortLoading {
            // switch tag
            match obj.get_tag() {
                // vTypes
                SUMO_TAG_VTYPE => {
                    // check if parse vType or Ref
                    if obj.has_string_attribute(SUMO_ATTR_REFID) {
                        if self.build_v_type_ref(obj, &str_attr(obj, SUMO_ATTR_REFID), double_attr(obj, SUMO_ATTR_PROB)) {
                            obj.m().mark_as_created();
                        }
                    } else if self.build_v_type(obj, vehicle_type_parameter_of(obj)) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_VTYPE_DISTRIBUTION => {
                    if self.build_v_type_distribution(obj, &str_attr(obj, SUMO_ATTR_ID), int_attr(obj, SUMO_ATTR_DETERMINISTIC)) {
                        obj.m().mark_as_created();
                    }
                }
                // route
                SUMO_TAG_ROUTE => {
                    // embedded routes are created in build<Vehicle/Flow>EmbeddedRoute
                    if obj.has_string_attribute(SUMO_ATTR_REFID) {
                        if self.build_route_ref(obj, &str_attr(obj, SUMO_ATTR_REFID), double_attr(obj, SUMO_ATTR_PROB)) {
                            obj.m().mark_as_created();
                        }
                    } else if obj.has_string_attribute(SUMO_ATTR_ID) {
                        if self.build_route(
                            obj,
                            &str_attr(obj, SUMO_ATTR_ID),
                            obj.get_vclass(),
                            &str_list_attr(obj, SUMO_ATTR_EDGES),
                            &color_attr(obj, SUMO_ATTR_COLOR),
                            int_attr(obj, SUMO_ATTR_REPEAT),
                            time_attr(obj, SUMO_ATTR_CYCLETIME),
                            double_attr(obj, SUMO_ATTR_PROB),
                            obj.get_parameters(),
                        ) {
                            obj.m().mark_as_created();
                        }
                    }
                }
                SUMO_TAG_ROUTE_DISTRIBUTION => {
                    if self.build_route_distribution(obj, &str_attr(obj, SUMO_ATTR_ID)) {
                        obj.m().mark_as_created();
                    }
                }
                // vehicles
                SUMO_TAG_TRIP => {
                    if self.check_vehicle_parents(obj) {
                        if self.is_over_from_to_edges(obj) {
                            // build trip with from-to edges
                            let from = if obj.has_string_attribute(SUMO_ATTR_FROM) { str_attr(obj, SUMO_ATTR_FROM) } else { String::new() };
                            let to = if obj.has_string_attribute(SUMO_ATTR_TO) { str_attr(obj, SUMO_ATTR_TO) } else { String::new() };
                            if self.build_trip(obj, vehicle_parameter_of(obj), &from, &to) {
                                obj.m().mark_as_created();
                            }
                        } else if self.is_over_from_to_junctions(obj) {
                            // build trip with from-to junctions
                            if self.build_trip_junctions(obj, vehicle_parameter_of(obj), &str_attr(obj, SUMO_ATTR_FROM_JUNCTION), &str_attr(obj, SUMO_ATTR_TO_JUNCTION)) {
                                obj.m().mark_as_created();
                            }
                        } else if self.is_over_from_to_ta_zs(obj) {
                            // build trip with from-to TAZs
                            if self.build_trip_ta_zs(obj, vehicle_parameter_of(obj), &str_attr(obj, SUMO_ATTR_FROM_TAZ), &str_attr(obj, SUMO_ATTR_TO_TAZ)) {
                                obj.m().mark_as_created();
                            }
                        }
                    }
                }
                SUMO_TAG_VEHICLE => {
                    if self.check_vehicle_parents(obj) {
                        if obj.has_string_attribute(SUMO_ATTR_ROUTE) {
                            // build vehicle over route
                            if self.build_vehicle_over_route(obj, vehicle_parameter_of(obj)) {
                                obj.m().mark_as_created();
                            }
                        } else {
                            let embeddedRoute = self.get_embedded_route(obj);
                            if !embeddedRoute.is_null() {
                                // build vehicle with embedded route
                                if self.build_vehicle_embedded_route(
                                    obj,
                                    vehicle_parameter_of(obj),
                                    &str_list_attr(embeddedRoute, SUMO_ATTR_EDGES),
                                    &color_attr(embeddedRoute, SUMO_ATTR_COLOR),
                                    int_attr(embeddedRoute, SUMO_ATTR_REPEAT),
                                    time_attr(embeddedRoute, SUMO_ATTR_CYCLETIME),
                                    embeddedRoute.get_parameters(),
                                ) {
                                    obj.m().mark_as_created();
                                }
                            }
                        }
                    }
                }
                // flows
                SUMO_TAG_FLOW => {
                    if self.check_vehicle_parents(obj) {
                        if obj.has_string_attribute(SUMO_ATTR_ROUTE) {
                            // build flow over route
                            if self.build_flow_over_route(obj, vehicle_parameter_of(obj)) {
                                obj.m().mark_as_created();
                            }
                        } else if self.is_over_from_to_edges(obj) {
                            // build flow with from-to edges
                            if self.build_flow(obj, vehicle_parameter_of(obj), &str_attr(obj, SUMO_ATTR_FROM), &str_attr(obj, SUMO_ATTR_TO)) {
                                obj.m().mark_as_created();
                            }
                        } else if self.is_over_from_to_junctions(obj) {
                            // build flow with from-to junctions
                            if self.build_flow_junctions(obj, vehicle_parameter_of(obj), &str_attr(obj, SUMO_ATTR_FROM_JUNCTION), &str_attr(obj, SUMO_ATTR_TO_JUNCTION)) {
                                obj.m().mark_as_created();
                            }
                        } else if self.is_over_from_to_ta_zs(obj) {
                            // build flow with from-to TAZs
                            if self.build_flow_ta_zs(obj, vehicle_parameter_of(obj), &str_attr(obj, SUMO_ATTR_FROM_TAZ), &str_attr(obj, SUMO_ATTR_TO_TAZ)) {
                                obj.m().mark_as_created();
                            }
                        } else {
                            let embeddedRoute = self.get_embedded_route(obj);
                            if !embeddedRoute.is_null() {
                                // build flow with embedded route
                                if self.build_flow_embedded_route(
                                    obj,
                                    vehicle_parameter_of(obj),
                                    &str_list_attr(embeddedRoute, SUMO_ATTR_EDGES),
                                    &color_attr(embeddedRoute, SUMO_ATTR_COLOR),
                                    int_attr(embeddedRoute, SUMO_ATTR_REPEAT),
                                    time_attr(embeddedRoute, SUMO_ATTR_CYCLETIME),
                                    embeddedRoute.get_parameters(),
                                ) {
                                    obj.m().mark_as_created();
                                }
                            }
                        }
                    }
                }
                // persons
                SUMO_TAG_PERSON => {
                    if self.build_person(obj, vehicle_parameter_of(obj)) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_PERSONFLOW => {
                    if self.build_person_flow(obj, vehicle_parameter_of(obj)) {
                        obj.m().mark_as_created();
                    }
                }
                // person plans
                SUMO_TAG_PERSONTRIP => {
                    if self.check_person_plan_parents(obj) {
                        if self.build_person_trip(
                            obj,
                            obj.get_plan_parameters(),
                            double_attr(obj, SUMO_ATTR_ARRIVALPOS),
                            &str_list_attr(obj, SUMO_ATTR_VTYPES),
                            &str_list_attr(obj, SUMO_ATTR_MODES),
                            &str_list_attr(obj, SUMO_ATTR_LINES),
                            double_attr(obj, SUMO_ATTR_WALKFACTOR),
                            &str_attr(obj, SUMO_ATTR_GROUP),
                        ) {
                            obj.m().mark_as_created();
                        }
                    }
                }
                SUMO_TAG_RIDE => {
                    if self.check_person_plan_parents(obj) {
                        if self.build_ride(obj, obj.get_plan_parameters(), double_attr(obj, SUMO_ATTR_ARRIVALPOS), &str_list_attr(obj, SUMO_ATTR_LINES), &str_attr(obj, SUMO_ATTR_GROUP)) {
                            obj.m().mark_as_created();
                        }
                    }
                }
                SUMO_TAG_WALK => {
                    if self.check_person_plan_parents(obj) {
                        if self.build_walk(obj, obj.get_plan_parameters(), double_attr(obj, SUMO_ATTR_ARRIVALPOS), double_attr(obj, SUMO_ATTR_SPEED), time_attr(obj, SUMO_ATTR_DURATION)) {
                            obj.m().mark_as_created();
                        }
                    }
                }
                // container
                SUMO_TAG_CONTAINER => {
                    if self.build_container(obj, vehicle_parameter_of(obj)) {
                        obj.m().mark_as_created();
                    }
                }
                SUMO_TAG_CONTAINERFLOW => {
                    if self.build_container_flow(obj, vehicle_parameter_of(obj)) {
                        obj.m().mark_as_created();
                    }
                }
                // container plans
                SUMO_TAG_TRANSPORT => {
                    if self.check_container_plan_parents(obj) {
                        if self.build_transport(obj, obj.get_plan_parameters(), double_attr(obj, SUMO_ATTR_ARRIVALPOS), &str_list_attr(obj, SUMO_ATTR_LINES), &str_attr(obj, SUMO_ATTR_GROUP)) {
                            obj.m().mark_as_created();
                        }
                    }
                }
                SUMO_TAG_TRANSHIP => {
                    if self.check_container_plan_parents(obj) {
                        if self.build_tranship(
                            obj,
                            obj.get_plan_parameters(),
                            double_attr(obj, SUMO_ATTR_ARRIVALPOS),
                            double_attr(obj, SUMO_ATTR_DEPARTPOS),
                            double_attr(obj, SUMO_ATTR_SPEED),
                            time_attr(obj, SUMO_ATTR_DURATION),
                        ) {
                            obj.m().mark_as_created();
                        }
                    }
                }
                // stops
                SUMO_TAG_STOP => {
                    if self.check_stop_parents(obj) {
                        if self.build_stop(obj, obj.get_plan_parameters(), or_throw(obj.get_stop_parameter()).downcast_ref::<SUMOVehicleParameter_Stop>().expect("SumoBaseObject does not hold a Stop")) {
                            obj.m().mark_as_created();
                        }
                    }
                }
                _ => {}
            }
            // now iterate over childrens
            for child in obj.get_sumo_base_object_children() {
                // call this function recursively
                self.parse_sumo_base_object(Ptr::from_box(child));
            }
        }
    }

    /// C++: `bool RouteHandler::buildVType(const CommonXMLStructure::SumoBaseObject *, const SUMOVTypeParameter &)` [virtual, pure virtual]
    /// build vType
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_v_type(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, vTypeParameter: &SUMOVTypeParameter) -> bool {
        panic!("pure virtual RouteHandler::buildVType")
    }

    /// C++: `bool RouteHandler::buildVTypeRef(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double)` [virtual, pure virtual]
    /// build vType ref
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_v_type_ref(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, vTypeID: &str, probability: f64) -> bool {
        panic!("pure virtual RouteHandler::buildVTypeRef")
    }

    /// C++: `bool RouteHandler::buildVTypeDistribution(const CommonXMLStructure::SumoBaseObject *, const std::string &, const int)` [virtual, pure virtual]
    /// build vType distribution
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_v_type_distribution(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, deterministic: i32) -> bool {
        panic!("pure virtual RouteHandler::buildVTypeDistribution")
    }

    /// C++: `bool RouteHandler::buildRoute(const CommonXMLStructure::SumoBaseObject *, const std::string &, SUMOVehicleClass, const std::vector<std::string> &, const RGBColor &, const int, const SUMOTime, const double, const Parameterised::Map &)` [virtual, pure virtual]
    /// build route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_route(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str, vClass: SUMOVehicleClass, edgeIDs: &Vec<String>, color: &RGBColor, repeat: i32, cycleTime: SUMOTime, probability: f64, routeParameters: &ParameterisedMap) -> bool {
        panic!("pure virtual RouteHandler::buildRoute")
    }

    /// C++: `bool RouteHandler::buildRouteRef(const CommonXMLStructure::SumoBaseObject *, const std::string &, const double)` [virtual, pure virtual]
    /// build route ref
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_route_ref(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, routeID: &str, probability: f64) -> bool {
        panic!("pure virtual RouteHandler::buildRouteRef")
    }

    /// C++: `bool RouteHandler::buildRouteDistribution(const CommonXMLStructure::SumoBaseObject *, const std::string &)` [virtual, pure virtual]
    /// build route distribution
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_route_distribution(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, id: &str) -> bool {
        panic!("pure virtual RouteHandler::buildRouteDistribution")
    }

    /// C++: `bool RouteHandler::buildVehicleOverRoute(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &)` [virtual, pure virtual]
    /// build a vehicle over an existent route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_vehicle_over_route(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, vehicleParameters: &SUMOVehicleParameter) -> bool {
        panic!("pure virtual RouteHandler::buildVehicleOverRoute")
    }

    /// C++: `bool RouteHandler::buildVehicleEmbeddedRoute(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &, const std::vector<std::string> &, const RGBColor &, const int, const SUMOTime, const Parameterised::Map &)` [virtual, pure virtual]
    /// build a vehicle with an embedded route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_vehicle_embedded_route(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, vehicleParameters: &SUMOVehicleParameter, edgeIDs: &Vec<String>, color: &RGBColor, repeat: i32, cycleTime: SUMOTime, routeParameters: &ParameterisedMap) -> bool {
        panic!("pure virtual RouteHandler::buildVehicleEmbeddedRoute")
    }

    /// C++: `bool RouteHandler::buildFlowOverRoute(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &)` [virtual, pure virtual]
    /// build a flow over an existent route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_flow_over_route(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, vehicleParameters: &SUMOVehicleParameter) -> bool {
        panic!("pure virtual RouteHandler::buildFlowOverRoute")
    }

    /// C++: `bool RouteHandler::buildFlowEmbeddedRoute(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &, const std::vector<std::string> &, const RGBColor &, const int, const SUMOTime, const Parameterised::Map &)` [virtual, pure virtual]
    /// build a flow with an embedded route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_flow_embedded_route(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, vehicleParameters: &SUMOVehicleParameter, edgeIDs: &Vec<String>, color: &RGBColor, repeat: i32, cycleTime: SUMOTime, routeParameters: &ParameterisedMap) -> bool {
        panic!("pure virtual RouteHandler::buildFlowEmbeddedRoute")
    }

    /// C++: `bool RouteHandler::buildTrip(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &, const std::string &, const std::string &)` [virtual, pure virtual]
    /// build trip (from-to edges)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_trip(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, vehicleParameters: &SUMOVehicleParameter, fromEdgeID: &str, toEdgeID: &str) -> bool {
        panic!("pure virtual RouteHandler::buildTrip")
    }

    /// C++: `bool RouteHandler::buildTripJunctions(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &, const std::string &, const std::string &)` [virtual, pure virtual]
    /// build trip (from-to junctions)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_trip_junctions(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, vehicleParameters: &SUMOVehicleParameter, fromJunctionID: &str, toJunctionID: &str) -> bool {
        panic!("pure virtual RouteHandler::buildTripJunctions")
    }

    /// C++: `bool RouteHandler::buildTripTAZs(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &, const std::string &, const std::string &)` [virtual, pure virtual]
    /// build trip (from-to TAZs)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_trip_ta_zs(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, vehicleParameters: &SUMOVehicleParameter, fromTazID: &str, toTazID: &str) -> bool {
        panic!("pure virtual RouteHandler::buildTripTAZs")
    }

    /// C++: `bool RouteHandler::buildFlow(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &, const std::string &, const std::string &)` [virtual, pure virtual]
    /// build flow (from-to edges)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_flow(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, vehicleParameters: &SUMOVehicleParameter, fromEdgeID: &str, toEdgeID: &str) -> bool {
        panic!("pure virtual RouteHandler::buildFlow")
    }

    /// C++: `bool RouteHandler::buildFlowJunctions(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &, const std::string &, const std::string &)` [virtual, pure virtual]
    /// build flow (from-to junctions)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_flow_junctions(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, vehicleParameters: &SUMOVehicleParameter, fromJunctionID: &str, toJunctionID: &str) -> bool {
        panic!("pure virtual RouteHandler::buildFlowJunctions")
    }

    /// C++: `bool RouteHandler::buildFlowTAZs(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &, const std::string &, const std::string &)` [virtual, pure virtual]
    /// build flow (from-to TAZs)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_flow_ta_zs(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, vehicleParameters: &SUMOVehicleParameter, fromTAZID: &str, toTAZID: &str) -> bool {
        panic!("pure virtual RouteHandler::buildFlowTAZs")
    }

    /// C++: `bool RouteHandler::buildPerson(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &)` [virtual, pure virtual]
    /// build person
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_person(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, personParameters: &SUMOVehicleParameter) -> bool {
        panic!("pure virtual RouteHandler::buildPerson")
    }

    /// C++: `bool RouteHandler::buildPersonFlow(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &)` [virtual, pure virtual]
    /// build person flow
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_person_flow(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, personFlowParameters: &SUMOVehicleParameter) -> bool {
        panic!("pure virtual RouteHandler::buildPersonFlow")
    }

    /// C++: `bool RouteHandler::buildPersonTrip(const CommonXMLStructure::SumoBaseObject *, const CommonXMLStructure::PlanParameters &, const double, const std::vector<std::string> &, const std::vector<std::string> &, const std::vector<std::string> &, const double, const std::string &)` [virtual, pure virtual]
    /// build person trip
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_person_trip(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, planParameters: &CommonXMLStructure_PlanParameters, arrivalPos: f64, types: &Vec<String>, modes: &Vec<String>, lines: &Vec<String>, walkFactor: f64, group: &str) -> bool {
        panic!("pure virtual RouteHandler::buildPersonTrip")
    }

    /// C++: `bool RouteHandler::buildWalk(const CommonXMLStructure::SumoBaseObject *, const CommonXMLStructure::PlanParameters &, const double, const double, const SUMOTime)` [virtual, pure virtual]
    /// build walk
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_walk(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, planParameters: &CommonXMLStructure_PlanParameters, arrivalPos: f64, speed: f64, duration: SUMOTime) -> bool {
        panic!("pure virtual RouteHandler::buildWalk")
    }

    /// C++: `bool RouteHandler::buildRide(const CommonXMLStructure::SumoBaseObject *, const CommonXMLStructure::PlanParameters &, const double, const std::vector<std::string> &, const std::string &)` [virtual, pure virtual]
    /// build ride
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_ride(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, planParameters: &CommonXMLStructure_PlanParameters, arrivalPos: f64, lines: &Vec<String>, group: &str) -> bool {
        panic!("pure virtual RouteHandler::buildRide")
    }

    /// C++: `bool RouteHandler::buildContainer(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &)` [virtual, pure virtual]
    /// build container
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_container(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, containerParameters: &SUMOVehicleParameter) -> bool {
        panic!("pure virtual RouteHandler::buildContainer")
    }

    /// C++: `bool RouteHandler::buildContainerFlow(const CommonXMLStructure::SumoBaseObject *, const SUMOVehicleParameter &)` [virtual, pure virtual]
    /// build container flow
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_container_flow(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, containerFlowParameters: &SUMOVehicleParameter) -> bool {
        panic!("pure virtual RouteHandler::buildContainerFlow")
    }

    /// C++: `bool RouteHandler::buildTransport(const CommonXMLStructure::SumoBaseObject *, const CommonXMLStructure::PlanParameters &, const double, const std::vector<std::string> &, const std::string &)` [virtual, pure virtual]
    /// build transport
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_transport(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, planParameters: &CommonXMLStructure_PlanParameters, arrivalPos: f64, lines: &Vec<String>, group: &str) -> bool {
        panic!("pure virtual RouteHandler::buildTransport")
    }

    /// C++: `bool RouteHandler::buildTranship(const CommonXMLStructure::SumoBaseObject *, const CommonXMLStructure::PlanParameters &, const double, const double, const double, const SUMOTime)` [virtual, pure virtual]
    /// build tranship
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_tranship(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, planParameters: &CommonXMLStructure_PlanParameters, arrivalPosition: f64, departPosition: f64, speed: f64, duration: SUMOTime) -> bool {
        panic!("pure virtual RouteHandler::buildTranship")
    }

    /// C++: `bool RouteHandler::buildStop(const CommonXMLStructure::SumoBaseObject *, const CommonXMLStructure::PlanParameters &, const SUMOVehicleParameter::Stop &)` [virtual, pure virtual]
    /// build stop
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn build_stop(&mut self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>, planParameters: &CommonXMLStructure_PlanParameters, stopParameters: &SUMOVehicleParameter_Stop) -> bool {
        panic!("pure virtual RouteHandler::buildStop")
    }

    /// C++: `void RouteHandler::parseVType(const SUMOSAXAttributes &)` [private]
    /// parse vType
    // void
    // RouteHandler::parseVType(const SUMOSAXAttributes& attrs) {
    //     // parse vehicleType
    //     SUMOVTypeParameter* vehicleTypeParameter = SUMOVehicleParserHelper::beginVTypeParsing(attrs, myHardFail, myFileBucket->getFilename(), nullptr);
    //     if (vehicleTypeParameter) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_VTYPE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setVehicleTypeParameter(vehicleTypeParameter);
    //         // delete vehicleType parameter (because in XMLStructure we have a copy)
    //         delete vehicleTypeParameter;
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_v_type(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseVType")
    }

    /// C++: `void RouteHandler::parseVTypeRef(const SUMOSAXAttributes &)` [private]
    /// parse vType reference
    // void
    // RouteHandler::parseVTypeRef(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // special case for ID
    //     const std::string refId = attrs.get<std::string>(SUMO_ATTR_REFID, "", parsedOk);
    //     const double probability = attrs.getOpt<double>(SUMO_ATTR_PROB, refId.c_str(), parsedOk, INVALID_DOUBLE);
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_VTYPE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_REFID, refId);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_PROB, probability);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_v_type_ref(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseVTypeRef")
    }

    /// C++: `void RouteHandler::parseVTypeDistribution(const SUMOSAXAttributes &)` [private]
    /// parse vType distribution
    // void
    // RouteHandler::parseVTypeDistribution(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     // optional attributes
    //     const int deterministic = attrs.getOpt<int>(SUMO_ATTR_DETERMINISTIC, id.c_str(), parsedOk, -1);
    //     std::vector<std::string> vTypes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VTYPES, id.c_str(), parsedOk);
    //     std::vector<double> probabilities = attrs.getOpt<std::vector<double> >(SUMO_ATTR_PROBS, id.c_str(), parsedOk);
    //     // adjust sizes and probabilities
    //     adjustTypesAndProbabilities(vTypes, probabilities);
    //     // continue if all was parsed ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_VTYPE_DISTRIBUTION);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addIntAttribute(SUMO_ATTR_DETERMINISTIC, deterministic);
    //         // add references
    //         for (int i = 0; i < (int)vTypes.size(); i++) {
    //             // open SUMOBaseOBject
    //             myCommonXMLStructure.openSUMOBaseOBject();
    //             // set tag
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_VTYPE);
    //             // add all attributes
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_REFID, vTypes.at(i));
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_PROB, probabilities.at(i));
    //             // close SUMOBaseOBject
    //             myCommonXMLStructure.closeSUMOBaseOBject();
    //         }
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_v_type_distribution(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseVTypeDistribution")
    }

    /// C++: `void RouteHandler::parseRoute(const SUMOSAXAttributes &)` [private]
    /// parse route
    // void
    // RouteHandler::parseRoute(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     const std::vector<std::string> edges = attrs.get<std::vector<std::string> >(SUMO_ATTR_EDGES, id.c_str(), parsedOk);
    //     // optional attributes
    //     SUMOVehicleClass vClass = SUMOVehicleParserHelper::parseVehicleClass(attrs, id);
    //     const RGBColor color = attrs.getOpt<RGBColor>(SUMO_ATTR_COLOR, id.c_str(), parsedOk, RGBColor::INVISIBLE);
    //     const int repeat = attrs.getOpt<int>(SUMO_ATTR_REPEAT, id.c_str(), parsedOk, 0);
    //     const SUMOTime cycleTime = attrs.getOptSUMOTimeReporting(SUMO_ATTR_CYCLETIME, id.c_str(), parsedOk, 0);
    //     const double probability = attrs.getOpt<double>(SUMO_ATTR_PROB, id.c_str(), parsedOk, 1.0);
    //     // check attributes
    //     if (!checkNegative(SUMO_TAG_ROUTE, id, SUMO_ATTR_CYCLETIME, cycleTime, true)) {
    //         parsedOk = false;
    //     }
    //     if (!checkNegative(SUMO_TAG_ROUTE, id, SUMO_ATTR_REPEAT, repeat, true)) {
    //         parsedOk = false;
    //     }
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ROUTE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setVClass(vClass);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_EDGES, edges);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addColorAttribute(SUMO_ATTR_COLOR, color);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addIntAttribute(SUMO_ATTR_REPEAT, repeat);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_CYCLETIME, cycleTime);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_PROB, probability);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_route(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseRoute")
    }

    /// C++: `void RouteHandler::parseRouteRef(const SUMOSAXAttributes &)` [private]
    /// parse route reference
    // void
    // RouteHandler::parseRouteRef(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // special case for ID
    //     const std::string refId = attrs.get<std::string>(SUMO_ATTR_REFID, "", parsedOk);
    //     const double probability = attrs.getOpt<double>(SUMO_ATTR_PROB, refId.c_str(), parsedOk, INVALID_DOUBLE);
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ROUTE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_REFID, refId);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_PROB, probability);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_route_ref(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseRouteRef")
    }

    /// C++: `void RouteHandler::parseRouteEmbedded(const SUMOSAXAttributes &)` [private]
    /// parse embedded route
    // void
    // RouteHandler::parseRouteEmbedded(const SUMOSAXAttributes& attrs) {
    //     // first check if this is an embedded route
    //     if (attrs.hasAttribute(SUMO_ATTR_ID)) {
    //         writeError(TL("an embedded route cannot have their own ID"));
    //     } else {
    //         // declare Ok Flag
    //         bool parsedOk = true;
    //         // special case for ID
    //         const std::string vehicleID = myCommonXMLStructure.getCurrentSumoBaseObject()->getParentSumoBaseObject()->getVehicleParameter().id;
    //         // needed attributes
    //         const std::vector<std::string> edges = attrs.get<std::vector<std::string> >(SUMO_ATTR_EDGES, vehicleID.c_str(), parsedOk);
    //         // optional attributes
    //         SUMOVehicleClass vClass = SUMOVehicleParserHelper::parseVehicleClass(attrs, vehicleID);
    //         const RGBColor color = attrs.getOpt<RGBColor>(SUMO_ATTR_COLOR, vehicleID.c_str(), parsedOk, RGBColor::INVISIBLE);
    //         const int repeat = attrs.getOpt<int>(SUMO_ATTR_REPEAT, vehicleID.c_str(), parsedOk, 0);
    //         const SUMOTime cycleTime = attrs.getOptSUMOTimeReporting(SUMO_ATTR_CYCLETIME, vehicleID.c_str(), parsedOk, 0);
    //         // check attributes
    //         if (!checkNegative(SUMO_TAG_ROUTE, vehicleID, SUMO_ATTR_CYCLETIME, cycleTime, true)) {
    //             parsedOk = false;
    //         }
    //         if (!checkNegative(SUMO_TAG_ROUTE, vehicleID, SUMO_ATTR_REPEAT, repeat, true)) {
    //             parsedOk = false;
    //         }
    //         if (parsedOk) {
    //             // set tag
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ROUTE);
    //             // add all attributes
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setVClass(vClass);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_EDGES, edges);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addColorAttribute(SUMO_ATTR_COLOR, color);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addIntAttribute(SUMO_ATTR_REPEAT, repeat);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_CYCLETIME, cycleTime);
    //         } else {
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_route_embedded(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseRouteEmbedded")
    }

    /// C++: `void RouteHandler::parseRouteDistribution(const SUMOSAXAttributes &)` [private]
    /// parse route distribution
    // void
    // RouteHandler::parseRouteDistribution(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // needed attributes
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, "", parsedOk);
    //     // optional attributes
    //     std::vector<std::string> routes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_ROUTES, id.c_str(), parsedOk);
    //     std::vector<double> probabilities = attrs.getOpt<std::vector<double> >(SUMO_ATTR_PROBS, id.c_str(), parsedOk);
    //     // adjust sizes and probabilities
    //     adjustTypesAndProbabilities(routes, probabilities);
    //     // continue if all was parsed ok
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ROUTE_DISTRIBUTION);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ID, id);
    //         // add references
    //         for (int i = 0; i < (int)routes.size(); i++) {
    //             // open SUMOBaseOBject
    //             myCommonXMLStructure.openSUMOBaseOBject();
    //             // set tag
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ROUTE);
    //             // add all attributes
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_REFID, routes.at(i));
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_PROB, probabilities.at(i));
    //             // close SUMOBaseOBject
    //             myCommonXMLStructure.closeSUMOBaseOBject();
    //         }
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_route_distribution(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseRouteDistribution")
    }

    /// C++: `void RouteHandler::parseTrip(const SUMOSAXAttributes &)` [private]
    /// parse trip
    // void
    // RouteHandler::parseTrip(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // parse vehicle
    //     SUMOVehicleParameter* tripParameter = SUMOVehicleParserHelper::parseVehicleAttributes(SUMO_TAG_TRIP, attrs, myHardFail);
    //     if (tripParameter) {
    //         // check from/to edge/junction
    //         if ((attrs.hasAttribute(SUMO_ATTR_FROM) + attrs.hasAttribute(SUMO_ATTR_FROM_JUNCTION) + attrs.hasAttribute(SUMO_ATTR_FROM_TAZ)) > 1) {
    //             writeError(TL("Attributes 'from', 'fromJunction' and 'fromTaz' cannot be defined together"));
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //         } else if ((attrs.hasAttribute(SUMO_ATTR_TO) + attrs.hasAttribute(SUMO_ATTR_TO_JUNCTION) + attrs.hasAttribute(SUMO_ATTR_TO_TAZ)) > 1) {
    //             writeError(TL("Attributes 'to', 'toJunction' and 'toTaz' cannot be defined together"));
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //         } else if (attrs.hasAttribute(SUMO_ATTR_FROM_JUNCTION) && attrs.hasAttribute(SUMO_ATTR_TO_JUNCTION)) {
    //             // from-to attributes
    //             const std::string fromJunction = attrs.get<std::string>(SUMO_ATTR_FROM_JUNCTION, tripParameter->id.c_str(), parsedOk);
    //             const std::string toJunction = attrs.get<std::string>(SUMO_ATTR_TO_JUNCTION, tripParameter->id.c_str(), parsedOk);
    //             if (parsedOk) {
    //                 // set tag
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_TRIP);
    //                 // set vehicle parameters
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setVehicleParameter(tripParameter);
    //                 // add other attributes
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FROM_JUNCTION, fromJunction);
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_TO_JUNCTION, toJunction);
    //             } else {
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //             }
    //         } else if (attrs.hasAttribute(SUMO_ATTR_FROM_TAZ) && attrs.hasAttribute(SUMO_ATTR_TO_TAZ)) {
    //             // from-to attributes
    //             const std::string fromJunction = attrs.get<std::string>(SUMO_ATTR_FROM_TAZ, tripParameter->id.c_str(), parsedOk);
    //             const std::string toJunction = attrs.get<std::string>(SUMO_ATTR_TO_TAZ, tripParameter->id.c_str(), parsedOk);
    //             if (parsedOk) {
    //                 // set tag
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_TRIP);
    //                 // set vehicle parameters
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setVehicleParameter(tripParameter);
    //                 // add other attributes
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FROM_TAZ, fromJunction);
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_TO_TAZ, toJunction);
    //             } else {
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //             }
    //         } else {
    //             // from-to attributes
    //             const std::string from = attrs.getOpt<std::string>(SUMO_ATTR_FROM, tripParameter->id.c_str(), parsedOk, "");
    //             const std::string to = attrs.getOpt<std::string>(SUMO_ATTR_TO, tripParameter->id.c_str(), parsedOk, "");
    //             // optional attributes
    //             const std::vector<std::string> via = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VIA, tripParameter->id.c_str(), parsedOk);
    //             if (parsedOk) {
    //                 // set tag
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_TRIP);
    //                 // set vehicle parameters
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setVehicleParameter(tripParameter);
    //                 // add other attributes
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FROM, from);
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_TO, to);
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_VIA, via);
    //             } else {
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //             }
    //         }
    //         // delete trip parameter (because in XMLStructure we have a copy)
    //         delete tripParameter;
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_trip(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseTrip")
    }

    /// C++: `void RouteHandler::parseVehicle(const SUMOSAXAttributes &)` [private]
    /// parse vehicle (including vehicles over routes and vehicles with embedded routes)
    // void
    // RouteHandler::parseVehicle(const SUMOSAXAttributes& attrs) {
    //     // first parse vehicle
    //     SUMOVehicleParameter* vehicleParameter = SUMOVehicleParserHelper::parseVehicleAttributes(SUMO_TAG_VEHICLE, attrs, myHardFail);
    //     if (vehicleParameter) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_VEHICLE);
    //         // set vehicle parameters
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setVehicleParameter(vehicleParameter);
    //         // delete vehicle parameter (because in XMLStructure we have a copy)
    //         delete vehicleParameter;
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_vehicle(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseVehicle")
    }

    /// C++: `void RouteHandler::parseFlow(const SUMOSAXAttributes &)` [private]
    /// parse flow (including flows, flows over routes and flows with embedded routes)
    // void
    // RouteHandler::parseFlow(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // first parse flow
    //     SUMOVehicleParameter* flowParameter = SUMOVehicleParserHelper::parseFlowAttributes(SUMO_TAG_FLOW, attrs, myHardFail, true, myFlowBeginDefault, myFlowEndDefault);
    //     if (flowParameter) {
    //         // set vehicle parameters
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setVehicleParameter(flowParameter);
    //         // check from/to edge/junction
    //         if ((attrs.hasAttribute(SUMO_ATTR_FROM) + attrs.hasAttribute(SUMO_ATTR_FROM_JUNCTION) + attrs.hasAttribute(SUMO_ATTR_FROM_TAZ)) > 1) {
    //             writeError(TL("Attributes 'from', 'fromJunction' and 'fromTaz' cannot be defined together"));
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //         } else if ((attrs.hasAttribute(SUMO_ATTR_TO) + attrs.hasAttribute(SUMO_ATTR_TO_JUNCTION) + attrs.hasAttribute(SUMO_ATTR_TO_TAZ)) > 1) {
    //             writeError(TL("Attributes 'to', 'toJunction' and 'toTaz' cannot be defined together"));
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //         } else if (attrs.hasAttribute(SUMO_ATTR_FROM) && attrs.hasAttribute(SUMO_ATTR_TO)) {
    //             // from-to attributes
    //             const std::string from = attrs.get<std::string>(SUMO_ATTR_FROM, flowParameter->id.c_str(), parsedOk);
    //             const std::string to = attrs.get<std::string>(SUMO_ATTR_TO, flowParameter->id.c_str(), parsedOk);
    //             // optional attributes
    //             const std::vector<std::string> via = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VIA, flowParameter->id.c_str(), parsedOk);
    //             if (parsedOk) {
    //                 // set tag
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_FLOW);
    //                 // add other attributes
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FROM, from);
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_TO, to);
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_VIA, via);
    //             } else {
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //             }
    //         } else if (attrs.hasAttribute(SUMO_ATTR_FROM_JUNCTION) && attrs.hasAttribute(SUMO_ATTR_TO_JUNCTION)) {
    //             // from-to attributes
    //             const std::string fromJunction = attrs.get<std::string>(SUMO_ATTR_FROM_JUNCTION, flowParameter->id.c_str(), parsedOk);
    //             const std::string toJunction = attrs.get<std::string>(SUMO_ATTR_TO_JUNCTION, flowParameter->id.c_str(), parsedOk);
    //             if (parsedOk) {
    //                 // set tag
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_FLOW);
    //                 // add other attributes
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FROM_JUNCTION, fromJunction);
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_TO_JUNCTION, toJunction);
    //             } else {
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //             }
    //         } else if (attrs.hasAttribute(SUMO_ATTR_FROM_TAZ) && attrs.hasAttribute(SUMO_ATTR_TO_TAZ)) {
    //             // from-to attributes
    //             const std::string fromJunction = attrs.get<std::string>(SUMO_ATTR_FROM_TAZ, flowParameter->id.c_str(), parsedOk);
    //             const std::string toJunction = attrs.get<std::string>(SUMO_ATTR_TO_TAZ, flowParameter->id.c_str(), parsedOk);
    //             if (parsedOk) {
    //                 // set tag
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_FLOW);
    //                 // add other attributes
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_FROM_TAZ, fromJunction);
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_TO_TAZ, toJunction);
    //             } else {
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //             }
    //         } else if (attrs.hasAttribute(SUMO_ATTR_ROUTE)) {
    //             // from-to attributes
    //             const std::string route = attrs.get<std::string>(SUMO_ATTR_ROUTE, flowParameter->id.c_str(), parsedOk);
    //             if (parsedOk) {
    //                 // set tag
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_FLOW);
    //                 // add other attributes
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_ROUTE, route);
    //             } else {
    //                 myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //             }
    //         } else {
    //             // set tag
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_FLOW);
    //         }
    //         // delete flow parameter (because in XMLStructure we have a copy)
    //         delete flowParameter;
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_flow(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseFlow")
    }

    /// C++: `void RouteHandler::parseStop(const SUMOSAXAttributes &)` [private]
    /// parse stop
    // void
    // RouteHandler::parseStop(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // declare stop
    //     SUMOVehicleParameter::Stop stop;
    //     // plan parameters
    //     const auto planParameters = CommonXMLStructure::PlanParameters(myCommonXMLStructure.getCurrentSumoBaseObject(), attrs, parsedOk);
    //     // get parents
    //     std::vector<SumoXMLTag> stopParents;
    //     stopParents.insert(stopParents.end(), NamespaceIDs::routes.begin(), NamespaceIDs::routes.end());
    //     stopParents.insert(stopParents.end(), NamespaceIDs::vehicles.begin(), NamespaceIDs::vehicles.end());
    //     stopParents.insert(stopParents.end(), NamespaceIDs::persons.begin(), NamespaceIDs::persons.end());
    //     stopParents.insert(stopParents.end(), NamespaceIDs::containers.begin(), NamespaceIDs::containers.end());
    //     //  check parents
    //     checkParsedParent(SUMO_TAG_STOP, stopParents, parsedOk);
    //     // parse stop
    //     if (parsedOk && parseStopParameters(stop, attrs)) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_STOP);
    //         // add stop attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setPlanParameters(planParameters);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setStopParameter(stop);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_stop(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseStop")
    }

    /// C++: `void RouteHandler::parsePerson(const SUMOSAXAttributes &)` [private]
    /// parse person
    // void
    // RouteHandler::parsePerson(const SUMOSAXAttributes& attrs) {
    //     // first parse vehicle
    //     SUMOVehicleParameter* personParameter = SUMOVehicleParserHelper::parseVehicleAttributes(SUMO_TAG_PERSON, attrs, myHardFail);
    //     if (personParameter) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_PERSON);
    //         // set vehicle parameter
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setVehicleParameter(personParameter);
    //         // delete person parameter (because in XMLStructure we have a copy)
    //         delete personParameter;
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_person(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parsePerson")
    }

    /// C++: `void RouteHandler::parsePersonFlow(const SUMOSAXAttributes &)` [private]
    /// parse person flow
    // void
    // RouteHandler::parsePersonFlow(const SUMOSAXAttributes& attrs) {
    //     // first parse flow
    //     SUMOVehicleParameter* personFlowParameter = SUMOVehicleParserHelper::parseFlowAttributes(SUMO_TAG_PERSONFLOW, attrs, myHardFail, true, myFlowBeginDefault, myFlowEndDefault);
    //     if (personFlowParameter) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_PERSONFLOW);
    //         // set vehicle parameter
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setVehicleParameter(personFlowParameter);
    //         // delete person flow parameter (because in XMLStructure we have a copy)
    //         delete personFlowParameter;
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_person_flow(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parsePersonFlow")
    }

    /// C++: `void RouteHandler::parsePersonTrip(const SUMOSAXAttributes &)` [private]
    /// parse person trip
    // void
    // RouteHandler::parsePersonTrip(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // plan parameters
    //     const auto planParameters = CommonXMLStructure::PlanParameters(myCommonXMLStructure.getCurrentSumoBaseObject(), attrs, parsedOk);
    //     // optional attributes
    //     const std::vector<std::string> via = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VIA, "", parsedOk);
    //     const std::vector<std::string> vTypes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_VTYPES, "", parsedOk);
    //     const std::vector<std::string> lines = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_LINES, "", parsedOk);
    //     std::vector<std::string> modes = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_MODES, "", parsedOk);
    //     const double departPos = attrs.getOpt<double>(SUMO_ATTR_DEPARTPOS, "", parsedOk, -1);
    //     const double arrivalPos = attrs.getOpt<double>(SUMO_ATTR_ARRIVALPOS, "", parsedOk, -1);
    //     const double walkFactor = attrs.getOpt<double>(SUMO_ATTR_WALKFACTOR, "", parsedOk, 0);
    //     const std::string group = attrs.getOpt<std::string>(SUMO_ATTR_GROUP, "", parsedOk, "");
    //     // check modes
    //     SVCPermissions dummyModeSet;
    //     std::string dummyError;
    //     if (!SUMOVehicleParameter::parsePersonModes(toString(modes), toString(SUMO_TAG_PERSONTRIP), "", dummyModeSet, dummyError)) {
    //         WRITE_WARNING(dummyError);
    //         modes.clear();
    //     }
    //     // check parents
    //     checkParsedParent(SUMO_TAG_PERSONTRIP, NamespaceIDs::persons, parsedOk);
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_PERSONTRIP);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setPlanParameters(planParameters);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_VTYPES, vTypes);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_MODES, modes);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_LINES, lines);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_DEPARTPOS, departPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ARRIVALPOS, arrivalPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_WALKFACTOR, walkFactor);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_GROUP, group);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_person_trip(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parsePersonTrip")
    }

    /// C++: `void RouteHandler::parseWalk(const SUMOSAXAttributes &)` [private]
    /// parse walk
    // void
    // RouteHandler::parseWalk(const SUMOSAXAttributes& attrs) {
    //     if (attrs.hasAttribute(SUMO_ATTR_SPEED) && attrs.hasAttribute(SUMO_ATTR_DURATION)) {
    //         writeError(TL("Speed and duration attributes cannot be defined together in walks"));
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     } else {
    //         // declare Ok Flag
    //         bool parsedOk = true;
    //         // plan parameters
    //         const auto planParameters = CommonXMLStructure::PlanParameters(myCommonXMLStructure.getCurrentSumoBaseObject(), attrs, parsedOk);
    //         // optional attributes
    //         const double departPos = attrs.getOpt<double>(SUMO_ATTR_DEPARTPOS, "", parsedOk, -1);
    //         const double arrivalPos = attrs.getOpt<double>(SUMO_ATTR_ARRIVALPOS, "", parsedOk, -1);
    //         const double speed = attrs.getOpt<double>(SUMO_ATTR_SPEED, "", parsedOk, 1.39);
    //         const SUMOTime duration = attrs.getOptSUMOTimeReporting(SUMO_ATTR_DURATION, "", parsedOk, 0);
    //         // check parents
    //         checkParsedParent(SUMO_TAG_WALK, NamespaceIDs::persons, parsedOk);
    //         if (parsedOk) {
    //             // set tag
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_WALK);
    //             // add all attributes
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setPlanParameters(planParameters);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_DEPARTPOS, departPos);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ARRIVALPOS, arrivalPos);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_SPEED, speed);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_DURATION, duration);
    //         } else {
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_walk(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseWalk")
    }

    /// C++: `void RouteHandler::parseRide(const SUMOSAXAttributes &)` [private]
    /// parse ride
    // void
    // RouteHandler::parseRide(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // plan parameters
    //     const auto planParameters = CommonXMLStructure::PlanParameters(myCommonXMLStructure.getCurrentSumoBaseObject(), attrs, parsedOk);
    //     // optional attributes
    //     const std::vector<std::string> lines = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_LINES, "", parsedOk);
    //     const double arrivalPos = attrs.getOpt<double>(SUMO_ATTR_ARRIVALPOS, "", parsedOk, -1);
    //     const std::string group = attrs.getOpt<std::string>(SUMO_ATTR_GROUP, "", parsedOk, "");
    //     // check parents
    //     checkParsedParent(SUMO_TAG_RIDE, NamespaceIDs::persons, parsedOk);
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_RIDE);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setPlanParameters(planParameters);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_LINES, lines);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ARRIVALPOS, arrivalPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_GROUP, group);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_ride(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseRide")
    }

    /// C++: `void RouteHandler::parseContainer(const SUMOSAXAttributes &)` [private]
    /// parse container
    // void
    // RouteHandler::parseContainer(const SUMOSAXAttributes& attrs) {
    //     // first parse container
    //     SUMOVehicleParameter* containerParameter = SUMOVehicleParserHelper::parseVehicleAttributes(SUMO_TAG_CONTAINER, attrs, myHardFail);
    //     if (containerParameter) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_CONTAINER);
    //         // set vehicle parameter
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setVehicleParameter(containerParameter);
    //         // delete container parameter (because in XMLStructure we have a copy)
    //         delete containerParameter;
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_container(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseContainer")
    }

    /// C++: `void RouteHandler::parseContainerFlow(const SUMOSAXAttributes &)` [private]
    /// parse container flow
    // void
    // RouteHandler::parseContainerFlow(const SUMOSAXAttributes& attrs) {
    //     // first parse flow
    //     SUMOVehicleParameter* containerFlowParameter = SUMOVehicleParserHelper::parseFlowAttributes(SUMO_TAG_CONTAINERFLOW, attrs, myHardFail, true, myFlowBeginDefault, myFlowEndDefault);
    //     if (containerFlowParameter) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_CONTAINERFLOW);
    //         // set vehicle parameter
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setVehicleParameter(containerFlowParameter);
    //         // delete container flow parameter (because in XMLStructure we have a copy)
    //         delete containerFlowParameter;
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_container_flow(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseContainerFlow")
    }

    /// C++: `void RouteHandler::parseTransport(const SUMOSAXAttributes &)` [private]
    /// parse transport
    // void
    // RouteHandler::parseTransport(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // plan parameters
    //     const auto planParameters = CommonXMLStructure::PlanParameters(myCommonXMLStructure.getCurrentSumoBaseObject(), attrs, parsedOk);
    //     // optional attributes
    //     const std::vector<std::string> lines = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_LINES, "", parsedOk);
    //     const double arrivalPos = attrs.getOpt<double>(SUMO_ATTR_ARRIVALPOS, "", parsedOk, -1);
    //     const std::string group = attrs.getOpt<std::string>(SUMO_ATTR_GROUP, "", parsedOk, "");
    //     // check parents
    //     checkParsedParent(SUMO_TAG_TRANSPORT, NamespaceIDs::containers, parsedOk);
    //     if (parsedOk) {
    //         // set tag
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_TRANSPORT);
    //         // add all attributes
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setPlanParameters(planParameters);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringListAttribute(SUMO_ATTR_LINES, lines);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ARRIVALPOS, arrivalPos);
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->addStringAttribute(SUMO_ATTR_GROUP, group);
    //     } else {
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_transport(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseTransport")
    }

    /// C++: `void RouteHandler::parseTranship(const SUMOSAXAttributes &)` [private]
    /// parse tranship
    // void
    // RouteHandler::parseTranship(const SUMOSAXAttributes& attrs) {
    //     if (attrs.hasAttribute(SUMO_ATTR_SPEED) && attrs.hasAttribute(SUMO_ATTR_DURATION)) {
    //         writeError(TL("Speed and duration attributes cannot be defined together in tranships"));
    //         myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //     } else {
    //         // declare Ok Flag
    //         bool parsedOk = true;
    //         // plan parameters
    //         const auto planParameters = CommonXMLStructure::PlanParameters(myCommonXMLStructure.getCurrentSumoBaseObject(), attrs, parsedOk);
    //         // optional attributes
    //         const double arrivalPos = attrs.getOpt<double>(SUMO_ATTR_ARRIVALPOS, "", parsedOk, -1);
    //         const double departPos = attrs.getOpt<double>(SUMO_ATTR_DEPARTPOS, "", parsedOk, -1);
    //         const double speed = attrs.getOpt<double>(SUMO_ATTR_SPEED, "", parsedOk, 1.39);
    //         const SUMOTime duration = attrs.getOptSUMOTimeReporting(SUMO_ATTR_DURATION, "", parsedOk, 0);
    //         // check parents
    //         checkParsedParent(SUMO_TAG_TRANSHIP, NamespaceIDs::containers, parsedOk);
    //         if (parsedOk) {
    //             // set tag
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_TRANSHIP);
    //             // add all attributes
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setPlanParameters(planParameters);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_ARRIVALPOS, arrivalPos);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_DEPARTPOS, departPos);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addDoubleAttribute(SUMO_ATTR_SPEED, speed);
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->addTimeAttribute(SUMO_ATTR_DURATION, duration);
    //         } else {
    //             myCommonXMLStructure.getCurrentSumoBaseObject()->setTag(SUMO_TAG_ERROR);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_tranship(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseTranship")
    }

    /// C++: `void RouteHandler::parseInterval(const SUMOSAXAttributes &)` [private]
    /// parse interval
    // void
    // RouteHandler::parseInterval(const SUMOSAXAttributes& attrs) {
    //     // declare Ok Flag
    //     bool parsedOk = true;
    //     // just parse begin and end default
    //     myFlowBeginDefault = attrs.getSUMOTimeReporting(SUMO_ATTR_BEGIN, nullptr, parsedOk);
    //     myFlowEndDefault = attrs.getSUMOTimeReporting(SUMO_ATTR_END, nullptr, parsedOk);
    // }
    #[allow(unused_variables)]
    pub fn parse_interval(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port RouteHandler::parseInterval")
    }

    /// C++: `bool RouteHandler::parseNestedCFM(const SumoXMLTag, const SUMOSAXAttributes &, CommonXMLStructure::SumoBaseObject *)` [private]
    /// parse nested CarFollowingModel
    // bool
    // RouteHandler::parseNestedCFM(const SumoXMLTag tag, const SUMOSAXAttributes& attrs, CommonXMLStructure::SumoBaseObject* vTypeObject) {
    //     // write warning info
    //     WRITE_WARNINGF(TL("Defining car-following parameters in a nested element is deprecated in vType '%', use attributes instead!"), vTypeObject->getStringAttribute(SUMO_ATTR_ID));
    //     // get vType to modify it
    //     auto vType = vTypeObject->getVehicleTypeParameter();
    //     // parse nested CFM attributes
    //     if (SUMOVehicleParserHelper::parseCFMParams(&vType, tag, attrs, true)) {
    //         vTypeObject->setVehicleTypeParameter(&vType);
    //         return true;
    //     } else if (myHardFail) {
    //         throw ProcessError(TL("Invalid parsing embedded VType"));
    //     } else {
    //         return writeError(TL("Invalid parsing embedded VType"));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_nested_cfm(&mut self, tag: SumoXMLTag, attrs: &SUMOSAXAttributes, vTypeObject: Ptr<CommonXMLStructure_SumoBaseObject>) -> bool {
        todo!("port RouteHandler::parseNestedCFM")
    }

    /// C++: `bool RouteHandler::parseStopParameters(SUMOVehicleParameter::Stop &, const SUMOSAXAttributes &)` [private]
    /// parse stop parameters
    // bool
    // RouteHandler::parseStopParameters(SUMOVehicleParameter::Stop& stop, const SUMOSAXAttributes& attrs) {
    //     // check stop parameters
    //     if (attrs.hasAttribute(SUMO_ATTR_ARRIVAL)) {
    //         stop.parametersSet |= STOP_ARRIVAL_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_DURATION)) {
    //         stop.parametersSet |= STOP_DURATION_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_UNTIL)) {
    //         stop.parametersSet |= STOP_UNTIL_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_STARTED)) {
    //         stop.parametersSet |= STOP_STARTED_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_ENDED)) {
    //         stop.parametersSet |= STOP_ENDED_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_EXTENSION)) {
    //         stop.parametersSet |= STOP_EXTENSION_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_ENDPOS)) {
    //         stop.parametersSet |= STOP_END_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_STARTPOS)) {
    //         stop.parametersSet |= STOP_START_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_POSITION_LAT)) {
    //         stop.parametersSet |= STOP_POSLAT_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_TRIGGERED)) {
    //         stop.parametersSet |= STOP_TRIGGER_SET;
    //     }
    //     // legacy attribute
    //     if (attrs.hasAttribute(SUMO_ATTR_CONTAINER_TRIGGERED)) {
    //         if (attrs.hasAttribute(SUMO_ATTR_SPEED)) {
    //             WRITE_WARNING(TLF("Attributes '%' and '%' cannot be loaded simultaneously. Ignoring '%'", toString(SUMO_ATTR_SPEED), toString(SUMO_ATTR_CONTAINER_TRIGGERED), toString(SUMO_ATTR_CONTAINER_TRIGGERED)));
    //         } else {
    //             stop.parametersSet |= STOP_TRIGGER_SET;
    //         }
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_PARKING)) {
    //         stop.parametersSet |= STOP_PARKING_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_EXPECTED)) {
    //         stop.parametersSet |= STOP_EXPECTED_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_PERMITTED)) {
    //         stop.parametersSet |= STOP_PERMITTED_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_EXPECTED_CONTAINERS)) {
    //         stop.parametersSet |= STOP_EXPECTED_CONTAINERS_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_TRIP_ID)) {
    //         stop.parametersSet |= STOP_TRIP_ID_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_SPLIT)) {
    //         stop.parametersSet |= STOP_SPLIT_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_JOIN)) {
    //         stop.parametersSet |= STOP_JOIN_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_LINE)) {
    //         stop.parametersSet |= STOP_LINE_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_SPEED)) {
    //         stop.parametersSet |= STOP_SPEED_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_ONDEMAND)) {
    //         stop.parametersSet |= STOP_ONDEMAND_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_JUMP)) {
    //         stop.parametersSet |= STOP_JUMP_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_JUMP_UNTIL)) {
    //         stop.parametersSet |= STOP_JUMP_UNTIL_SET;
    //     }
    //     // get parameters
    //     bool ok = true;
    //     // edge/lane
    //     stop.edge = attrs.getOpt<std::string>(SUMO_ATTR_EDGE, nullptr, ok, "");
    //     stop.lane = attrs.getOpt<std::string>(SUMO_ATTR_LANE, nullptr, ok, stop.busstop);
    //     // check errors
    //     if (!stop.edge.empty() && !stop.lane.empty()) {
    //         return writeError(TL("A stop must be defined either with an edge or with an lane, not both"));
    //     }
    //     // stopping places
    //     stop.busstop = attrs.getOpt<std::string>(SUMO_ATTR_BUS_STOP, nullptr, ok, "");
    //     if (stop.busstop.empty()) {
    //         stop.busstop = attrs.getOpt<std::string>(SUMO_ATTR_TRAIN_STOP, nullptr, ok, stop.busstop);
    //     }
    //     stop.chargingStation = attrs.getOpt<std::string>(SUMO_ATTR_CHARGING_STATION, nullptr, ok, "");
    //     stop.overheadWireSegment = attrs.getOpt<std::string>(SUMO_ATTR_OVERHEAD_WIRE_SEGMENT, nullptr, ok, "");
    //     stop.containerstop = attrs.getOpt<std::string>(SUMO_ATTR_CONTAINER_STOP, nullptr, ok, "");
    //     stop.parkingarea = attrs.getOpt<std::string>(SUMO_ATTR_PARKING_AREA, nullptr, ok, "");
    //     //check stopping places
    //     const int numStoppingPlaces = !stop.busstop.empty() + !stop.chargingStation.empty() + !stop.overheadWireSegment.empty() +
    //                                   !stop.containerstop.empty() + !stop.parkingarea.empty();
    //     if (numStoppingPlaces > 1) {
    //         return writeError(TL("A stop must be defined only in a StoppingPlace"));
    //     } else if ((numStoppingPlaces == 0) && stop.edge.empty() && stop.lane.empty()) {
    //         return writeError(TL("A stop must be defined in an edge, a lane, or in a StoppingPlace"));
    //     }
    //     // declare error suffix
    //     std::string errorSuffix;
    //     if (stop.busstop != "") {
    //         errorSuffix = " at '" + stop.busstop + "'" + errorSuffix;
    //     } else if (stop.chargingStation != "") {
    //         errorSuffix = " at '" + stop.chargingStation + "'" + errorSuffix;
    //     } else if (stop.overheadWireSegment != "") {
    //         errorSuffix = " at '" + stop.overheadWireSegment + "'" + errorSuffix;
    //     } else if (stop.containerstop != "") {
    //         errorSuffix = " at '" + stop.containerstop + "'" + errorSuffix;
    //     } else if (stop.parkingarea != "") {
    //         errorSuffix = " at '" + stop.parkingarea + "'" + errorSuffix;
    //     } else if (stop.edge != "") {
    //         errorSuffix = " at '" + stop.edge + "'" + errorSuffix;
    //     } else {
    //         errorSuffix = " on lane '" + stop.lane + "'" + errorSuffix;
    //     }
    //     // speed for counting as stopped
    //     stop.speed = attrs.getOpt<double>(SUMO_ATTR_SPEED, nullptr, ok, 0);
    //     if (stop.speed < 0) {
    //         return writeError("Speed cannot be negative for stop" + errorSuffix);
    //     }
    //     // get the standing duration
    //     bool expectTrigger = !attrs.hasAttribute(SUMO_ATTR_DURATION) && !attrs.hasAttribute(SUMO_ATTR_UNTIL) && !attrs.hasAttribute(SUMO_ATTR_SPEED);
    //     std::vector<std::string> triggers = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_TRIGGERED, nullptr, ok);
    //     // legacy
    //     if (attrs.getOpt<bool>(SUMO_ATTR_CONTAINER_TRIGGERED, nullptr, ok, false) && !attrs.hasAttribute(SUMO_ATTR_SPEED)) {
    //         triggers.push_back(toString(SUMO_TAG_CONTAINER));
    //     };
    //     SUMOVehicleParameter::parseStopTriggers(triggers, expectTrigger, stop);
    //     stop.startPos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, nullptr, ok, 0);
    //     stop.endPos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, nullptr, ok, 0);
    //     stop.friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, nullptr, ok, false);
    //     stop.arrival = attrs.getOptSUMOTimeReporting(SUMO_ATTR_ARRIVAL, nullptr, ok, -1);
    //     stop.duration = attrs.getOptSUMOTimeReporting(SUMO_ATTR_DURATION, nullptr, ok, -1);
    //     stop.until = attrs.getOptSUMOTimeReporting(SUMO_ATTR_UNTIL, nullptr, ok, -1);
    //     if (!expectTrigger && (!ok || (stop.duration < 0 && stop.until < 0 && stop.speed == 0))) {
    //         return writeError("Invalid duration or end time is given for a stop" + errorSuffix);
    //     }
    //     stop.extension = attrs.getOptSUMOTimeReporting(SUMO_ATTR_EXTENSION, nullptr, ok, -1);
    //     const bool defaultParking = (stop.triggered || stop.containerTriggered || stop.parkingarea != "");
    //     stop.parking = attrs.getOpt<ParkingType>(SUMO_ATTR_PARKING, nullptr, ok, defaultParking ? ParkingType::OFFROAD : ParkingType::ONROAD);
    //     if ((stop.parkingarea != "") && (stop.parking == ParkingType::ONROAD)) {
    //         WRITE_WARNING("Stop at parkingarea overrides attribute 'parking' for stop" + errorSuffix);
    //         stop.parking = ParkingType::OFFROAD;
    //     }
    //     if (!ok) {
    //         return writeError("Invalid bool for 'triggered', 'containerTriggered' or 'parking' for stop" + errorSuffix);
    //     }
    //     // expected persons
    //     const std::vector<std::string>& expected = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_EXPECTED, nullptr, ok);
    //     stop.awaitedPersons.insert(expected.begin(), expected.end());
    //     if (stop.awaitedPersons.size() > 0 && (stop.parametersSet & STOP_TRIGGER_SET) == 0) {
    //         stop.triggered = true;
    //         if ((stop.parametersSet & STOP_PARKING_SET) == 0) {
    //             stop.parking = ParkingType::OFFROAD;
    //         }
    //     }
    //     // permitted transportables
    //     const std::vector<std::string>& permitted = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_PERMITTED, nullptr, ok);
    //     stop.permitted.insert(permitted.begin(), permitted.end());
    //     // expected containers
    //     const std::vector<std::string>& expectedContainers = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_EXPECTED_CONTAINERS, nullptr, ok);
    //     stop.awaitedContainers.insert(expectedContainers.begin(), expectedContainers.end());
    //     if (stop.awaitedContainers.size() > 0 && (stop.parametersSet & STOP_CONTAINER_TRIGGER_SET) == 0) {
    //         stop.containerTriggered = true;
    //         if ((stop.parametersSet & STOP_PARKING_SET) == 0) {
    //             stop.parking = ParkingType::OFFROAD;
    //         }
    //     }
    //     // public transport trip id
    //     stop.tripId = attrs.getOpt<std::string>(SUMO_ATTR_TRIP_ID, nullptr, ok, "");
    //     stop.split = attrs.getOpt<std::string>(SUMO_ATTR_SPLIT, nullptr, ok, "");
    //     stop.join = attrs.getOpt<std::string>(SUMO_ATTR_JOIN, nullptr, ok, "");
    //     stop.line = attrs.getOpt<std::string>(SUMO_ATTR_LINE, nullptr, ok, "");
    //     // index
    //     const std::string idx = attrs.getOpt<std::string>(SUMO_ATTR_INDEX, nullptr, ok, "end");
    //     if (idx == "end") {
    //         stop.index = STOP_INDEX_END;
    //     } else if (idx == "fit") {
    //         stop.index = STOP_INDEX_FIT;
    //     } else {
    //         stop.index = attrs.get<int>(SUMO_ATTR_INDEX, nullptr, ok);
    //         if (!ok || stop.index < 0) {
    //             return writeError("Invalid 'index' for stop" + errorSuffix);
    //         }
    //     }
    //     stop.started = attrs.getOptSUMOTimeReporting(SUMO_ATTR_STARTED, nullptr, ok, -1);
    //     stop.ended = attrs.getOptSUMOTimeReporting(SUMO_ATTR_ENDED, nullptr, ok, -1);
    //     stop.posLat = attrs.getOpt<double>(SUMO_ATTR_POSITION_LAT, nullptr, ok, INVALID_DOUBLE);
    //     stop.actType = attrs.getOpt<std::string>(SUMO_ATTR_ACTTYPE, nullptr, ok, "");
    //     stop.onDemand = attrs.getOpt<bool>(SUMO_ATTR_ONDEMAND, nullptr, ok, false);
    //     stop.jump = attrs.getOptSUMOTimeReporting(SUMO_ATTR_JUMP, nullptr, ok, -1);
    //     stop.jumpUntil = attrs.getOptSUMOTimeReporting(SUMO_ATTR_JUMP_UNTIL, nullptr, ok, -1);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_stop_parameters(&mut self, stop: &mut SUMOVehicleParameter_Stop, attrs: &SUMOSAXAttributes) -> bool {
        todo!("port RouteHandler::parseStopParameters")
    }

    /// C++: `bool RouteHandler::isOverFromToEdges(const CommonXMLStructure::SumoBaseObject *) const` [const, private]
    /// check if element is defined over from-to edges
    // bool
    // RouteHandler::isOverFromToEdges(const CommonXMLStructure::SumoBaseObject* sumoBaseObject) const {
    //     return sumoBaseObject->hasStringAttribute(SUMO_ATTR_FROM) && sumoBaseObject->hasStringAttribute(SUMO_ATTR_TO);
    // }
    #[allow(unused_variables)]
    pub fn is_over_from_to_edges(&self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>) -> bool {
        todo!("port RouteHandler::isOverFromToEdges")
    }

    /// C++: `bool RouteHandler::isOverFromToJunctions(const CommonXMLStructure::SumoBaseObject *) const` [const, private]
    /// check if element is defined over from-to junctions
    // bool
    // RouteHandler::isOverFromToJunctions(const CommonXMLStructure::SumoBaseObject* sumoBaseObject) const {
    //     return sumoBaseObject->hasStringAttribute(SUMO_ATTR_FROM_JUNCTION) && sumoBaseObject->hasStringAttribute(SUMO_ATTR_TO_JUNCTION);
    // }
    #[allow(unused_variables)]
    pub fn is_over_from_to_junctions(&self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>) -> bool {
        todo!("port RouteHandler::isOverFromToJunctions")
    }

    /// C++: `bool RouteHandler::isOverFromToTAZs(const CommonXMLStructure::SumoBaseObject *) const` [const, private]
    /// check if element is defined over from-to junctions
    // bool
    // RouteHandler::isOverFromToTAZs(const CommonXMLStructure::SumoBaseObject* sumoBaseObject) const {
    //     return sumoBaseObject->hasStringAttribute(SUMO_ATTR_FROM_TAZ) && sumoBaseObject->hasStringAttribute(SUMO_ATTR_TO_TAZ);
    // }
    #[allow(unused_variables)]
    pub fn is_over_from_to_ta_zs(&self, sumoBaseObject: Ptr<CommonXMLStructure_SumoBaseObject>) -> bool {
        todo!("port RouteHandler::isOverFromToTAZs")
    }

    /// C++: `void RouteHandler::adjustTypesAndProbabilities(std::vector<std::string> &, std::vector<double> &)` [private]
    /// adjust types and probabilities
    // void
    // RouteHandler::adjustTypesAndProbabilities(std::vector<std::string>& vTypes, std::vector<double>& probabilities) {
    //     // add more probabilities
    //     for (size_t i = probabilities.size(); i < vTypes.size(); i++) {
    //         probabilities.push_back(INVALID_DOUBLE);
    //     }
    //     // reduce number of probabilities
    //     while (vTypes.size() < probabilities.size()) {
    //         probabilities.pop_back();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn adjust_types_and_probabilities(&mut self, vTypes: &mut Vec<String>, probabilities: &mut Vec<f64>) {
        todo!("port RouteHandler::adjustTypesAndProbabilities")
    }

    /// C++: `void RouteHandler::RouteHandler()` [private]
    /// invalidate default onstructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        todo!("port RouteHandler::RouteHandler")
    }

    /// C++: `void RouteHandler::RouteHandler(const RouteHandler &)` [private]
    /// invalidate copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(s: &RouteHandler) -> Self {
        todo!("port RouteHandler::RouteHandler")
    }

    /// C++: `RouteHandler & RouteHandler::operator=(const RouteHandler &)` [private]
    /// invalidate assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &RouteHandler) -> &mut RouteHandler {
        todo!("port RouteHandler::operator=")
    }

}

