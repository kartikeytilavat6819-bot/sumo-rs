//! Port of microsim/MSRouteHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
const VTYPEPARS_VEHICLECLASS_SET: i64 = 1 << 7;
use utils::common::sumo_vehicle_class::{get_vehicle_class_names, DEFAULT_CONTAINER_TRANSHIP_SPEED, DEFAULT_VTYPE_ID, SVC_BICYCLE, SVC_IGNORING, SVC_PASSENGER, SVC_PEDESTRIAN};

/// C++ enum `MSRouteHandler::ObjectTypeEnum`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSRouteHandler_ObjectTypeEnum {
    PERSON = 0,
    CONTAINER = 1,
    UNDEFINED = 2,
}

// shortcut: VEHPARS_* and JUNCTION_TAZ_MISSING_HELP belong to SUMOVehicleParameter.h / MapMatcher.h, not ported yet, move them there
const VEHPARS_FROM_TAZ_SET: i64 = 2 << 15;
const VEHPARS_TO_TAZ_SET: i64 = 2 << 16;
const VEHPARS_ROUTE_SET: i64 = 2 << 10;
const VEHPARS_FORCE_REROUTE: i64 = 2 << 17;
const VEHPARS_ARRIVALPOS_SET: i64 = 2 << 12;
const VTYPEPARS_MAXSPEED_SET: i64 = 1 << 2;
const JUNCTION_TAZ_MISSING_HELP: &str = "\nSet option '--junction-taz' or load a TAZ-file";
// shortcut: STOP_INDEX_* belong to SUMOVehicleParameter.h, not ported yet, move them there
const STOP_INDEX_REPEAT: i32 = -3;
const STOP_INDEX_END: i32 = -1;
// shortcut: MSPModel constants belong to MSPModel.h, not ported yet, move them there
const UNSPECIFIED_POS_LAT: f64 = f64::MAX;
const RANDOM_POS_LAT: f64 = -f64::MAX;

// shortcut: these constants belong to SUMOVehicleParameter.h / MSStage.h / SUMOVehicleParameter.h, not ported yet, move them there
const STOP_END_SET: i32 = 2;
const VEHPARS_DEPARTPOS_SET: i64 = 2 << 2;
const ARRIVALPOS_UNSPECIFIED: f64 = f64::INFINITY;
const MIN_STOP_LENGTH: f64 = 2.0 * utils::geom::POSITION_EPS;

/// C++ `new MSStageWaiting(...)` as a plan entry (Vec<Ptr<MSStage>>), pointing at its MSStage base
fn stage_waiting_ptr(stage: MSStageWaiting) -> Ptr<MSStage> {
    let raw = Box::into_raw(Box::new(stage));
    Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) })
}

/// C++ implicit `MSStage*` -> `Parameterised*` conversion (MSStage derives from Parameterised)
fn stage_as_parameterised(stage: Ptr<MSStage>) -> Ptr<Parameterised> {
    Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*stage.as_raw()).base) })
}

/// C++ `delete p` for a pointer created with Box::into_raw; deleting nullptr is a no-op
fn delete_ptr<T>(p: Ptr<T>) {
    if !p.is_null() {
        unsafe { drop(Box::from_raw(p.as_raw())) };
    }
}

/// C++ `throw ProcessError(msg)`; caught by MSRouteHandler::myStartElement
fn throw_process_error(msg: String) -> ! {
    std::panic::panic_any(ProcessError::new(msg))
}

/// C++ `throw ProcessError()`
fn throw_process_error_default() -> ! {
    std::panic::panic_any(ProcessError::default_msg())
}

/// C++ `e.what()` of a caught exception (ProcessError, or a plain panic message)
fn exception_what(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(e) = payload.downcast_ref::<ProcessError>() {
        e.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else {
        String::new()
    }
}

/// C++ `MSRouteHandler` lookup `MSRoute::dictionary(id, &myParsingRNG)` that yields nullptr when unknown
fn lookup_route(id: &str) -> Option<ConstMSRoutePtr> {
    if MSRoute::has_route(id) { Some(MSRoute::dictionary_3(id, SUMORouteHandler::get_parsing_rng())) } else { None }
}

/// C++ MapMatcher::parseGeoEdges (protected member of the MapMatcher base class)
// shortcut: parseGeoEdges belongs to the unported MapMatcher base class, geo-coordinate route parsing panics until it is ported
#[allow(unused_variables)]
fn parse_geo_edges(pv: PositionVector, geo: bool, vClass: SUMOVehicleClass, into: &mut ConstMSEdgeVector, rid: &str, isFrom: bool, ok: &mut bool) {
    panic!("parseGeoEdges needs the unported MapMatcher base class");
}

/// C++ `toString(SumoXMLTag)`: the canonical name of a tag
// shortcut: scans the TAGS table instead of the SumoXMLTags bijection, upgrade when sumo_xml_definitions exposes toString
fn tag_to_string(tag: SumoXMLTag) -> String {
    utils::xml::sumo_xml_definitions::TAGS.iter().find(|(_, v)| *v == tag).map(|(s, _)| s.to_string()).unwrap_or_default()
}

/// Parser and container for routes during their loading
/// C++ class `MSRouteHandler` (microsim/MSRouteHandler.h)
#[repr(C)]
pub struct MSRouteHandler {
    /// C++ base class `SUMORouteHandler`
    pub base: SUMORouteHandler,
    /// C++ base class `MapMatcher<MSEdge, MSLane, MSJunction>`
    pub base2: Unmapped /* MapMatcher<MSEdge, MSLane, MSJunction> */,
    /// The current route
    pub myActiveRoute: ConstMSEdgeVector,
    /// number of repetitions of the active route
    pub myActiveRouteRepeat: i32,
    pub myActiveRoutePeriod: SUMOTime,
    /// whether the active route is stored indefinitely (used by state loader)
    pub myActiveRoutePermanent: bool,
    /// The time at which this route was replaced (from vehroute-output)
    pub myActiveRouteReplacedAtTime: SUMOTime,
    /// The index at which this route was replaced (from vehroute-output)
    pub myActiveRouteReplacedIndex: i32,
    /// The type of the current object
    pub myActiveType: MSRouteHandler_ObjectTypeEnum,
    /// The name of the current object type
    pub myActiveTypeName: String,
    /// Whether an object with 'via'-attribute is being parsed
    pub myHaveVia: bool,
    /// The plan of the current transportable (person or container)
    pub myActiveTransportablePlan: Ptr<MSTransportable_MSTransportablePlan>,
    /// Information whether vehicles shall be directly added to the network or kept within the buffer
    pub myAddVehiclesDirectly: bool,
    /// The currently parsed distribution of vehicle types (probability->vehicle type)
    pub myCurrentVTypeDistribution: Ptr<RandomDistributor<Ptr<MSVehicleType>>>,
    /// The id of the currently parsed vehicle type distribution
    pub myCurrentVTypeDistributionID: String,
    /// The currently parsed distribution of routes (probability->route)
    pub myCurrentRouteDistribution: Ptr<RandomDistributor<ConstMSRoutePtr>>,
    /// The id of the currently parsed route distribution
    pub myCurrentRouteDistributionID: String,
    /// whether a state file is being loaded
    pub myAmLoadingState: bool,
    /// prefix when copying vehicles with --scale
    pub myScaleSuffix: String,
    /// whether loaded rerouting events shall be replayed
    pub myReplayRerouting: bool,
    /// whether we are loading a personFlow that is starting triggered in a vehicle flow
    pub myStartTriggeredInFlow: bool,
}

impl std::ops::Deref for MSRouteHandler {
    type Target = SUMORouteHandler;
    fn deref(&self) -> &SUMORouteHandler {
        &self.base
    }
}
impl std::ops::DerefMut for MSRouteHandler {
    fn deref_mut(&mut self) -> &mut SUMORouteHandler {
        &mut self.base
    }
}

impl MSRouteHandler {
    /// C++: `void MSRouteHandler::MSRouteHandler(const std::string &, bool)`
    /// standard constructor
    // MSRouteHandler::MSRouteHandler(const std::string& file, bool addVehiclesDirectly) :
    //     SUMORouteHandler(file, addVehiclesDirectly ? "" : "routes", true),
    //     MapMatcher(OptionsCont::getOptions().getBool("mapmatch.junctions"),
    //                OptionsCont::getOptions().getBool("mapmatch.taz"),
    //                OptionsCont::getOptions().getFloat("mapmatch.distance"),
    //                MsgHandler::getErrorInstance()),
    //     myActiveRouteRepeat(0),
    //     myActiveRoutePeriod(0),
    //     myActiveRoutePermanent(false),
    //     myActiveType(ObjectTypeEnum::UNDEFINED),
    //     myHaveVia(false),
    //     myActiveTransportablePlan(nullptr),
    //     myAddVehiclesDirectly(addVehiclesDirectly),
    //     myCurrentVTypeDistribution(nullptr),
    //     myCurrentRouteDistribution(nullptr),
    //     myAmLoadingState(false),
    //     myScaleSuffix(OptionsCont::getOptions().getString("scale-suffix")),
    //     myReplayRerouting(OptionsCont::getOptions().getBool("replay-rerouting")),
    //     myStartTriggeredInFlow(false) {
    //     myActiveRoute.reserve(100);
    // }
    #[allow(unused_variables)]
    pub fn new(file: &str, addVehiclesDirectly: bool) -> Self {
        // shortcut: MapMatcher<MSEdge, MSLane, MSJunction> is not ported yet (base2 is a unit placeholder), its mapmatch.* options are not read, upgrade when utils MapMatcher exists
        MSRouteHandler {
            base: SUMORouteHandler::new(file, if addVehiclesDirectly { "" } else { "routes" }, true),
            base2: (),
            myActiveRoute: Vec::with_capacity(100),
            myActiveRouteRepeat: 0,
            myActiveRoutePeriod: 0,
            myActiveRoutePermanent: false,
            myActiveRouteReplacedAtTime: 0,
            myActiveRouteReplacedIndex: 0,
            myActiveType: MSRouteHandler_ObjectTypeEnum::UNDEFINED,
            myActiveTypeName: String::new(),
            myHaveVia: false,
            myActiveTransportablePlan: Ptr::null(),
            myAddVehiclesDirectly: addVehiclesDirectly,
            myCurrentVTypeDistribution: Ptr::null(),
            myCurrentVTypeDistributionID: String::new(),
            myCurrentRouteDistribution: Ptr::null(),
            myCurrentRouteDistributionID: String::new(),
            myAmLoadingState: false,
            myScaleSuffix: utils::options::options_cont::get_string("scale-suffix"),
            myReplayRerouting: utils::options::options_cont::get_bool("replay-rerouting"),
            myStartTriggeredInFlow: false,
        }
    }

    /// C++: `void MSRouteHandler::~MSRouteHandler() noexcept` [virtual]
    /// standard destructor
    // MSRouteHandler::~MSRouteHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body is empty; members are released by their own destructors
    }

    /// C++: `void MSRouteHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // MSRouteHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     try {
    //         if (myActiveTransportablePlan != nullptr && myActiveTransportablePlan->empty() && myVehicleParameter->departProcedure == DepartDefinition::TRIGGERED
    //                 && element != SUMO_TAG_RIDE && element != SUMO_TAG_TRANSPORT && element != SUMO_TAG_PARAM) {
    //             const std::string mode = myActiveType == ObjectTypeEnum::PERSON ? "ride" : "transport";
    //             throw ProcessError("Triggered departure for " + myActiveTypeName + " '" + myVehicleParameter->id + "' requires starting with a " + mode + ".");
    //         }
    //         if (myVehicleParameter == nullptr) {
    //             myActiveTypeName = toString((SumoXMLTag)element);
    //             myHaveVia = false;
    //         }
    //         SUMORouteHandler::myStartElement(element, attrs);
    //         switch (element) {
    //             case SUMO_TAG_PERSONFLOW:
    //                 addTransportable(attrs, true);
    //                 break;
    //             case SUMO_TAG_CONTAINERFLOW:
    //                 addTransportable(attrs, false);
    //                 break;
    //             case SUMO_TAG_FLOW:
    //                 if (myVehicleParameter) {
    //                     parseFromViaTo((SumoXMLTag)element, attrs);
    //                 }
    //                 break;
    //             case SUMO_TAG_TRIP:
    //                 parseFromViaTo((SumoXMLTag)element, attrs);
    //                 break;
    //             default:
    //                 break;
    //         }
    //     } catch (ProcessError&) {
    //         deleteActivePlanAndVehicleParameter();
    //         throw;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        // C++ `throw ProcessError` is modelled as panic_any(ProcessError); the catch/rethrow pair
        // is catch_unwind + resume_unwind (also runs for non-ProcessError panics, which is harmless here)
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !self.myActiveTransportablePlan.is_null()
                && self.myActiveTransportablePlan.is_empty()
                && self.myVehicleParameter.departProcedure == DepartDefinition::TRIGGERED
                && element != SUMO_TAG_RIDE
                && element != SUMO_TAG_TRANSPORT
                && element != SUMO_TAG_PARAM
            {
                let mode = if self.myActiveType == MSRouteHandler_ObjectTypeEnum::PERSON { "ride" } else { "transport" };
                std::panic::panic_any(ProcessError::new(format!(
                    "Triggered departure for {} '{}' requires starting with a {}.",
                    self.myActiveTypeName, self.myVehicleParameter.id, mode
                )));
            }
            if self.myVehicleParameter.is_null() {
                self.myActiveTypeName = tag_to_string(element);
                self.myHaveVia = false;
            }
            self.base.my_start_element(element, attrs);
            match element {
                SUMO_TAG_PERSONFLOW => self.add_transportable(attrs, true),
                SUMO_TAG_CONTAINERFLOW => self.add_transportable(attrs, false),
                SUMO_TAG_FLOW => {
                    if !self.myVehicleParameter.is_null() {
                        self.parse_from_via_to(element, attrs);
                    }
                }
                SUMO_TAG_TRIP => self.parse_from_via_to(element, attrs),
                _ => {}
            }
        }));
        if let Err(payload) = result {
            self.delete_active_plan_and_vehicle_parameter();
            std::panic::resume_unwind(payload);
        }
    }

    /// C++: `void MSRouteHandler::parseFromViaTo(SumoXMLTag, const SUMOSAXAttributes &)` [protected]
    /// Called for parsing from and to and the corresponding taz attributes
    // void
    // MSRouteHandler::parseFromViaTo(SumoXMLTag tag, const SUMOSAXAttributes& attrs) {
    //     const std::string element = toString(tag);
    //     myActiveRoute.clear();
    //     bool useTaz = OptionsCont::getOptions().getBool("with-taz");
    //     if (useTaz && !myVehicleParameter->wasSet(VEHPARS_FROM_TAZ_SET) && !myVehicleParameter->wasSet(VEHPARS_TO_TAZ_SET)) {
    //         WRITE_WARNINGF(TL("Taz usage was requested but no taz present in % '%'!"), element, myVehicleParameter->id);
    //         useTaz = false;
    //     }
    //     bool ok = true;
    //     const std::string rid = "for " + element + " '" + myVehicleParameter->id + "'";
    //     SUMOVehicleClass vClass = SVC_PASSENGER;
    //     auto& vc = MSNet::getInstance()->getVehicleControl();
    //     if (!vc.getVTypeDistribution(myVehicleParameter->vtypeid)) {
    //         MSVehicleType* const type = vc.getVType(myVehicleParameter->vtypeid, &myParsingRNG);
    //         if (type != nullptr) {
    //             vClass = type->getParameter().vehicleClass;
    //         }
    //     }
    //     // from-attributes
    //     if ((useTaz || (!attrs.hasAttribute(SUMO_ATTR_FROM) && !attrs.hasAttribute(SUMO_ATTR_FROMXY) && !attrs.hasAttribute(SUMO_ATTR_FROMLONLAT))) &&
    //             (attrs.hasAttribute(SUMO_ATTR_FROM_TAZ) || attrs.hasAttribute(SUMO_ATTR_FROM_JUNCTION))) {
    //         const bool useJunction = attrs.hasAttribute(SUMO_ATTR_FROM_JUNCTION);
    //         const std::string tazType = useJunction ? "junction" : "taz";
    //         const std::string tazID = attrs.get<std::string>(useJunction ? SUMO_ATTR_FROM_JUNCTION : SUMO_ATTR_FROM_TAZ, myVehicleParameter->id.c_str(), ok);
    //         const MSEdge* fromTaz = MSEdge::dictionary(tazID + "-source");
    //         if (fromTaz == nullptr) {
    //             throw ProcessError("Source " + tazType + " '" + tazID + "' not known " + rid + "!"
    //                                + (useJunction ? JUNCTION_TAZ_MISSING_HELP : ""));
    //         } else if (fromTaz->getNumSuccessors() == 0 && tag != SUMO_TAG_PERSON) {
    //             throw ProcessError("Source " + tazType + " '" + tazID + "' has no outgoing edges " + rid + "!");
    //         } else {
    //             myActiveRoute.push_back(fromTaz);
    //         }
    //     } else if (attrs.hasAttribute(SUMO_ATTR_FROMXY)) {
    //         parseGeoEdges(attrs.get<PositionVector>(SUMO_ATTR_FROMXY, myVehicleParameter->id.c_str(), ok), false, vClass, myActiveRoute, rid, true, ok);
    //         if (myMapMatchTAZ && ok) {
    //             myVehicleParameter->fromTaz = myActiveRoute.back()->getID();
    //             myVehicleParameter->parametersSet |= VEHPARS_FROM_TAZ_SET;
    //         }
    //     } else if (attrs.hasAttribute(SUMO_ATTR_FROMLONLAT)) {
    //         parseGeoEdges(attrs.get<PositionVector>(SUMO_ATTR_FROMLONLAT, myVehicleParameter->id.c_str(), ok), true, vClass, myActiveRoute, rid, true, ok);
    //         if (myMapMatchTAZ && ok) {
    //             myVehicleParameter->fromTaz = myActiveRoute.back()->getID();
    //             myVehicleParameter->parametersSet |= VEHPARS_FROM_TAZ_SET;
    //         }
    //     } else {
    //         MSEdge::parseEdgesList(attrs.getOpt<std::string>(SUMO_ATTR_FROM, myVehicleParameter->id.c_str(), ok),
    //                                myActiveRoute, rid);
    //     }
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //
    //     // via-attributes
    //     if (!attrs.hasAttribute(SUMO_ATTR_VIA) && !attrs.hasAttribute(SUMO_ATTR_ROUTE)) {
    //         myInsertStopEdgesAt = (int)myActiveRoute.size();
    //     }
    //     ConstMSEdgeVector viaEdges;
    //     if (attrs.hasAttribute(SUMO_ATTR_VIAXY)) {
    //         parseGeoEdges(attrs.get<PositionVector>(SUMO_ATTR_VIAXY, myVehicleParameter->id.c_str(), ok), false, vClass, viaEdges, rid, false, ok);
    //     } else if (attrs.hasAttribute(SUMO_ATTR_VIALONLAT)) {
    //         parseGeoEdges(attrs.get<PositionVector>(SUMO_ATTR_VIALONLAT, myVehicleParameter->id.c_str(), ok), true, vClass, viaEdges, rid, false, ok);
    //     } else if (attrs.hasAttribute(SUMO_ATTR_VIAJUNCTIONS)) {
    //         for (std::string junctionID : attrs.get<std::vector<std::string> >(SUMO_ATTR_VIAJUNCTIONS, myVehicleParameter->id.c_str(), ok)) {
    //             const MSEdge* viaSink = MSEdge::dictionary(junctionID + "-sink");
    //             if (viaSink == nullptr) {
    //                 throw ProcessError("Junction-taz '" + junctionID + "' not found." + JUNCTION_TAZ_MISSING_HELP);
    //             } else {
    //                 viaEdges.push_back(viaSink);
    //             }
    //         }
    //     } else {
    //         MSEdge::parseEdgesList(attrs.getOpt<std::string>(SUMO_ATTR_VIA, myVehicleParameter->id.c_str(), ok),
    //                                viaEdges, rid);
    //     }
    //     if (!viaEdges.empty()) {
    //         myHaveVia = true;
    //     }
    //     for (const MSEdge* e : viaEdges) {
    //         myActiveRoute.push_back(e);
    //         myVehicleParameter->via.push_back(e->getID());
    //     }
    //
    //     // to-attributes
    //     if ((useTaz || (!attrs.hasAttribute(SUMO_ATTR_TO) && !attrs.hasAttribute(SUMO_ATTR_TOXY) && !attrs.hasAttribute(SUMO_ATTR_TOLONLAT))) &&
    //             (attrs.hasAttribute(SUMO_ATTR_TO_TAZ) || attrs.hasAttribute(SUMO_ATTR_TO_JUNCTION))) {
    //         const bool useJunction = attrs.hasAttribute(SUMO_ATTR_TO_JUNCTION);
    //         const std::string tazType = useJunction ? "junction" : "taz";
    //         const std::string tazID = attrs.get<std::string>(useJunction ? SUMO_ATTR_TO_JUNCTION : SUMO_ATTR_TO_TAZ, myVehicleParameter->id.c_str(), ok, true);
    //         const MSEdge* toTaz = MSEdge::dictionary(tazID + "-sink");
    //         if (toTaz == nullptr) {
    //             throw ProcessError("Sink " + tazType + " '" + tazID + "' not known " + rid + "!"
    //                                + (useJunction ? JUNCTION_TAZ_MISSING_HELP : ""));
    //         } else if (toTaz->getNumPredecessors() == 0 && tag != SUMO_TAG_PERSON) {
    //             throw ProcessError("Sink " + tazType + " '" + tazID + "' has no incoming edges " + rid + "!");
    //         } else {
    //             myActiveRoute.push_back(toTaz);
    //         }
    //     } else if (attrs.hasAttribute(SUMO_ATTR_TOXY)) {
    //         parseGeoEdges(attrs.get<PositionVector>(SUMO_ATTR_TOXY, myVehicleParameter->id.c_str(), ok, true), false, vClass, myActiveRoute, rid, false, ok);
    //         if (myMapMatchTAZ && ok) {
    //             myVehicleParameter->toTaz = myActiveRoute.back()->getID();
    //             myVehicleParameter->parametersSet |= VEHPARS_TO_TAZ_SET;
    //         }
    //     } else if (attrs.hasAttribute(SUMO_ATTR_TOLONLAT)) {
    //         parseGeoEdges(attrs.get<PositionVector>(SUMO_ATTR_TOLONLAT, myVehicleParameter->id.c_str(), ok, true), true, vClass, myActiveRoute, rid, false, ok);
    //         if (myMapMatchTAZ && ok) {
    //             myVehicleParameter->toTaz = myActiveRoute.back()->getID();
    //             myVehicleParameter->parametersSet |= VEHPARS_TO_TAZ_SET;
    //         }
    //     } else {
    //         MSEdge::parseEdgesList(attrs.getOpt<std::string>(SUMO_ATTR_TO, myVehicleParameter->id.c_str(), ok, "", true),
    //                                myActiveRoute, rid);
    //     }
    //     myActiveRouteID = "!" + myVehicleParameter->id;
    //     if (myVehicleParameter->routeid == "") {
    //         myVehicleParameter->routeid = myActiveRouteID;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_from_via_to(&mut self, tag: SumoXMLTag, attrs: &SUMOSAXAttributes) {
        let element = tag_to_string(tag);
        self.myActiveRoute.clear();
        let mut useTaz = utils::options::options_cont::get_bool("with-taz");
        // MapMatcher member copied from the mapmatch.taz option
        let myMapMatchTAZ = utils::options::options_cont::get_bool("mapmatch.taz");
        if useTaz && !self.myVehicleParameter.was_set(VEHPARS_FROM_TAZ_SET) && !self.myVehicleParameter.was_set(VEHPARS_TO_TAZ_SET) {
            write_warningf!("Taz usage was requested but no taz present in % '%'!", element, self.myVehicleParameter.id);
            useTaz = false;
        }
        let mut ok = true;
        let rid = format!("for {} '{}'", element, self.myVehicleParameter.id);
        let mut vClass = SVC_PASSENGER;
        let net = MSNet::get_instance();
        let vc = net.m().get_vehicle_control();
        if vc.get_v_type_distribution(&self.myVehicleParameter.vtypeid).is_null() {
            let type_ = vc.get_v_type(&self.myVehicleParameter.vtypeid, SUMORouteHandler::get_parsing_rng(), false);
            if !type_.is_null() {
                vClass = type_.get_parameter().vehicleClass;
            }
        }
        // from-attributes
        if (useTaz || (!attrs.has_attribute(SUMO_ATTR_FROM) && !attrs.has_attribute(SUMO_ATTR_FROMXY) && !attrs.has_attribute(SUMO_ATTR_FROMLONLAT)))
            && (attrs.has_attribute(SUMO_ATTR_FROM_TAZ) || attrs.has_attribute(SUMO_ATTR_FROM_JUNCTION))
        {
            let useJunction = attrs.has_attribute(SUMO_ATTR_FROM_JUNCTION);
            let tazType = if useJunction { "junction" } else { "taz" };
            let tazID: String = attrs.get(if useJunction { SUMO_ATTR_FROM_JUNCTION } else { SUMO_ATTR_FROM_TAZ }, &self.myVehicleParameter.id, &mut ok, true);
            let fromTaz = MSEdge::dictionary_2(&format!("{tazID}-source"));
            if fromTaz.is_null() {
                throw_process_error(format!(
                    "Source {} '{}' not known {}!{}",
                    tazType,
                    tazID,
                    rid,
                    if useJunction { JUNCTION_TAZ_MISSING_HELP } else { "" }
                ));
            } else if fromTaz.get_num_successors() == 0 && tag != SUMO_TAG_PERSON {
                throw_process_error(format!("Source {} '{}' has no outgoing edges {}!", tazType, tazID, rid));
            } else {
                self.myActiveRoute.push(fromTaz);
            }
        } else if attrs.has_attribute(SUMO_ATTR_FROMXY) {
            let pv: PositionVector = attrs.get(SUMO_ATTR_FROMXY, &self.myVehicleParameter.id, &mut ok, true);
            parse_geo_edges(pv, false, vClass, &mut self.myActiveRoute, &rid, true, &mut ok);
            if myMapMatchTAZ && ok {
                let taz = self.myActiveRoute.last().unwrap().get_id().to_string();
                self.myVehicleParameter.m().fromTaz = taz;
                self.myVehicleParameter.m().parametersSet |= VEHPARS_FROM_TAZ_SET;
            }
        } else if attrs.has_attribute(SUMO_ATTR_FROMLONLAT) {
            let pv: PositionVector = attrs.get(SUMO_ATTR_FROMLONLAT, &self.myVehicleParameter.id, &mut ok, true);
            parse_geo_edges(pv, true, vClass, &mut self.myActiveRoute, &rid, true, &mut ok);
            if myMapMatchTAZ && ok {
                let taz = self.myActiveRoute.last().unwrap().get_id().to_string();
                self.myVehicleParameter.m().fromTaz = taz;
                self.myVehicleParameter.m().parametersSet |= VEHPARS_FROM_TAZ_SET;
            }
        } else {
            let desc: String = attrs.get_opt(SUMO_ATTR_FROM, &self.myVehicleParameter.id, &mut ok, String::new(), true);
            MSEdge::parse_edges_list(&desc, &mut self.myActiveRoute, &rid);
        }
        if !ok {
            throw_process_error_default();
        }

        // via-attributes
        if !attrs.has_attribute(SUMO_ATTR_VIA) && !attrs.has_attribute(SUMO_ATTR_ROUTE) {
            self.myInsertStopEdgesAt = self.myActiveRoute.len() as i32;
        }
        let mut viaEdges: ConstMSEdgeVector = Vec::new();
        if attrs.has_attribute(SUMO_ATTR_VIAXY) {
            let pv: PositionVector = attrs.get(SUMO_ATTR_VIAXY, &self.myVehicleParameter.id, &mut ok, true);
            parse_geo_edges(pv, false, vClass, &mut viaEdges, &rid, false, &mut ok);
        } else if attrs.has_attribute(SUMO_ATTR_VIALONLAT) {
            let pv: PositionVector = attrs.get(SUMO_ATTR_VIALONLAT, &self.myVehicleParameter.id, &mut ok, true);
            parse_geo_edges(pv, true, vClass, &mut viaEdges, &rid, false, &mut ok);
        } else if attrs.has_attribute(SUMO_ATTR_VIAJUNCTIONS) {
            let junctionIDs: Vec<String> = attrs.get(SUMO_ATTR_VIAJUNCTIONS, &self.myVehicleParameter.id, &mut ok, true);
            for junctionID in junctionIDs {
                let viaSink = MSEdge::dictionary_2(&format!("{junctionID}-sink"));
                if viaSink.is_null() {
                    throw_process_error(format!("Junction-taz '{}' not found.{}", junctionID, JUNCTION_TAZ_MISSING_HELP));
                } else {
                    viaEdges.push(viaSink);
                }
            }
        } else {
            let desc: String = attrs.get_opt(SUMO_ATTR_VIA, &self.myVehicleParameter.id, &mut ok, String::new(), true);
            MSEdge::parse_edges_list(&desc, &mut viaEdges, &rid);
        }
        if !viaEdges.is_empty() {
            self.myHaveVia = true;
        }
        for e in viaEdges.iter() {
            self.myActiveRoute.push(*e);
            let eid = e.get_id().to_string();
            self.myVehicleParameter.m().via.push(eid);
        }

        // to-attributes
        if (useTaz || (!attrs.has_attribute(SUMO_ATTR_TO) && !attrs.has_attribute(SUMO_ATTR_TOXY) && !attrs.has_attribute(SUMO_ATTR_TOLONLAT)))
            && (attrs.has_attribute(SUMO_ATTR_TO_TAZ) || attrs.has_attribute(SUMO_ATTR_TO_JUNCTION))
        {
            let useJunction = attrs.has_attribute(SUMO_ATTR_TO_JUNCTION);
            let tazType = if useJunction { "junction" } else { "taz" };
            let tazID: String = attrs.get(if useJunction { SUMO_ATTR_TO_JUNCTION } else { SUMO_ATTR_TO_TAZ }, &self.myVehicleParameter.id, &mut ok, true);
            let toTaz = MSEdge::dictionary_2(&format!("{tazID}-sink"));
            if toTaz.is_null() {
                throw_process_error(format!(
                    "Sink {} '{}' not known {}!{}",
                    tazType,
                    tazID,
                    rid,
                    if useJunction { JUNCTION_TAZ_MISSING_HELP } else { "" }
                ));
            } else if toTaz.get_num_predecessors() == 0 && tag != SUMO_TAG_PERSON {
                throw_process_error(format!("Sink {} '{}' has no incoming edges {}!", tazType, tazID, rid));
            } else {
                self.myActiveRoute.push(toTaz);
            }
        } else if attrs.has_attribute(SUMO_ATTR_TOXY) {
            let pv: PositionVector = attrs.get(SUMO_ATTR_TOXY, &self.myVehicleParameter.id, &mut ok, true);
            parse_geo_edges(pv, false, vClass, &mut self.myActiveRoute, &rid, false, &mut ok);
            if myMapMatchTAZ && ok {
                let taz = self.myActiveRoute.last().unwrap().get_id().to_string();
                self.myVehicleParameter.m().toTaz = taz;
                self.myVehicleParameter.m().parametersSet |= VEHPARS_TO_TAZ_SET;
            }
        } else if attrs.has_attribute(SUMO_ATTR_TOLONLAT) {
            let pv: PositionVector = attrs.get(SUMO_ATTR_TOLONLAT, &self.myVehicleParameter.id, &mut ok, true);
            parse_geo_edges(pv, true, vClass, &mut self.myActiveRoute, &rid, false, &mut ok);
            if myMapMatchTAZ && ok {
                let taz = self.myActiveRoute.last().unwrap().get_id().to_string();
                self.myVehicleParameter.m().toTaz = taz;
                self.myVehicleParameter.m().parametersSet |= VEHPARS_TO_TAZ_SET;
            }
        } else {
            let desc: String = attrs.get_opt(SUMO_ATTR_TO, &self.myVehicleParameter.id, &mut ok, String::new(), true);
            MSEdge::parse_edges_list(&desc, &mut self.myActiveRoute, &rid);
        }
        self.myActiveRouteID = format!("!{}", self.myVehicleParameter.id);
        if self.myVehicleParameter.routeid.is_empty() {
            let routeID = self.myActiveRouteID.clone();
            self.myVehicleParameter.m().routeid = routeID;
        }
    }

    /// C++: `void MSRouteHandler::openVehicleTypeDistribution(const SUMOSAXAttributes &)` [virtual, protected]
    /// opens a type distribution for reading
    // void
    // MSRouteHandler::openVehicleTypeDistribution(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     myCurrentVTypeDistributionID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (ok) {
    //         myCurrentVTypeDistribution = new RandomDistributor<MSVehicleType*>();
    //         if (attrs.hasAttribute(SUMO_ATTR_VTYPES)) {
    //             std::vector<double> probs;
    //             if (attrs.hasAttribute(SUMO_ATTR_PROBS)) {
    //                 StringTokenizer st(attrs.get<std::string>(SUMO_ATTR_PROBS, myCurrentVTypeDistributionID.c_str(), ok));
    //                 while (st.hasNext()) {
    //                     probs.push_back(StringUtils::toDoubleSecure(st.next(), 1.0));
    //                 }
    //             }
    //             const std::string vTypes = attrs.get<std::string>(SUMO_ATTR_VTYPES, myCurrentVTypeDistributionID.c_str(), ok);
    //             StringTokenizer st(vTypes);
    //             int probIndex = 0;
    //             while (st.hasNext()) {
    //                 const std::string& vtypeID = st.next();
    //                 const RandomDistributor<MSVehicleType*>* const dist = MSNet::getInstance()->getVehicleControl().getVTypeDistribution(vtypeID);
    //                 if (dist != nullptr) {
    //                     const double distProb = ((int)probs.size() > probIndex ? probs[probIndex] : 1.) / dist->getOverallProb();
    //                     std::vector<double>::const_iterator probIt = dist->getProbs().begin();
    //                     for (MSVehicleType* const type : dist->getVals()) {
    //                         myCurrentVTypeDistribution->add(type, distProb * *probIt);
    //                         probIt++;
    //                     }
    //                 } else {
    //                     MSVehicleType* const type = MSNet::getInstance()->getVehicleControl().getVType(vtypeID, &myParsingRNG);
    //                     if (type == nullptr) {
    //                         throw ProcessError("Unknown vtype '" + vtypeID + "' in distribution '" + myCurrentVTypeDistributionID + "'.");
    //                     }
    //                     const double prob = ((int)probs.size() > probIndex ? probs[probIndex] : type->getDefaultProbability());
    //                     myCurrentVTypeDistribution->add(type, prob);
    //                 }
    //                 probIndex++;
    //             }
    //             if (probs.size() > 0 && probIndex != (int)probs.size()) {
    //                 WRITE_WARNING("Got " + toString(probs.size()) + " probabilities for " + toString(probIndex) +
    //                               " types in vTypeDistribution '" + myCurrentVTypeDistributionID + "'");
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn open_vehicle_type_distribution(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        self.myCurrentVTypeDistributionID = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
        if ok {
            self.myCurrentVTypeDistribution = Ptr::from_raw(Box::into_raw(Box::new(RandomDistributor::default())));
            if attrs.has_attribute(SUMO_ATTR_VTYPES) {
                let mut probs: Vec<f64> = Vec::new();
                if attrs.has_attribute(SUMO_ATTR_PROBS) {
                    let probsStr: String = attrs.get(SUMO_ATTR_PROBS, &self.myCurrentVTypeDistributionID, &mut ok, true);
                    let mut st = StringTokenizer::new(&probsStr);
                    while st.has_next() {
                        let next = st.next().unwrap();
                        probs.push(utils::common::string_utils::to_double_secure(&next, 1.0).unwrap_or(1.0));
                    }
                }
                let vTypes: String = attrs.get(SUMO_ATTR_VTYPES, &self.myCurrentVTypeDistributionID, &mut ok, true);
                let mut st = StringTokenizer::new(&vTypes);
                let mut probIndex: i32 = 0;
                while st.has_next() {
                    let vtypeID = st.next().unwrap();
                    let net = MSNet::get_instance();
                    let vc = net.m().get_vehicle_control();
                    let dist = vc.get_v_type_distribution(&vtypeID);
                    if !dist.is_null() {
                        let distProb = (if (probs.len() as i32) > probIndex { probs[probIndex as usize] } else { 1.0 }) / dist.get_overall_prob();
                        let distProbs = dist.get_probs();
                        for (i, type_) in dist.get_vals().iter().enumerate() {
                            self.myCurrentVTypeDistribution.m().add(*type_, distProb * distProbs[i], false);
                        }
                    } else {
                        let type_ = vc.get_v_type(&vtypeID, SUMORouteHandler::get_parsing_rng(), false);
                        if type_.is_null() {
                            throw_process_error(format!(
                                "Unknown vtype '{}' in distribution '{}'.",
                                vtypeID, self.myCurrentVTypeDistributionID
                            ));
                        }
                        let prob = if (probs.len() as i32) > probIndex { probs[probIndex as usize] } else { type_.get_default_probability() };
                        self.myCurrentVTypeDistribution.m().add(type_, prob, false);
                    }
                    probIndex += 1;
                }
                if !probs.is_empty() && probIndex != probs.len() as i32 {
                    utils::common::msg_handler::write_warning(&format!(
                        "Got {} probabilities for {} types in vTypeDistribution '{}'",
                        probs.len(),
                        probIndex,
                        self.myCurrentVTypeDistributionID
                    ));
                }
            }
        }
    }

    /// C++: `void MSRouteHandler::closeVehicleTypeDistribution()` [virtual, protected]
    /// closes (ends) the building of a distribution
    // void
    // MSRouteHandler::closeVehicleTypeDistribution() {
    //     if (myCurrentVTypeDistribution != nullptr) {
    //         if (state_loaded() && MSNet::getInstance()->getVehicleControl().hasVTypeDistribution(myCurrentVTypeDistributionID)) {
    //             myCurrentVTypeDistribution = nullptr;
    //             delete myCurrentVTypeDistribution;
    //             return;
    //         }
    //         if (myCurrentVTypeDistribution->getOverallProb() == 0) {
    //             delete myCurrentVTypeDistribution;
    //             throw ProcessError(TLF("Vehicle type distribution '%' is empty.", myCurrentVTypeDistributionID));
    //         }
    //         if (!MSNet::getInstance()->getVehicleControl().addVTypeDistribution(myCurrentVTypeDistributionID, myCurrentVTypeDistribution)) {
    //             delete myCurrentVTypeDistribution;
    //             throw ProcessError(TLF("Another vehicle type (or distribution) with the id '%' exists.", myCurrentVTypeDistributionID));
    //         }
    //         myCurrentVTypeDistribution = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_vehicle_type_distribution(&mut self) {
        if !self.myCurrentVTypeDistribution.is_null() {
            let net = MSNet::get_instance();
            if state_loaded() && net.m().get_vehicle_control().has_v_type_distribution(&self.myCurrentVTypeDistributionID) {
                // C++ nulls the pointer and then deletes nullptr, so the distribution is not freed here either
                self.myCurrentVTypeDistribution = Ptr::null();
                return;
            }
            if self.myCurrentVTypeDistribution.m().get_overall_prob() == 0.0 {
                let dist = self.myCurrentVTypeDistribution;
                unsafe { drop(Box::from_raw(dist.as_raw())) };
                throw_process_error(format!("Vehicle type distribution '{}' is empty.", self.myCurrentVTypeDistributionID));
            }
            if !net.m().get_vehicle_control().add_v_type_distribution(&self.myCurrentVTypeDistributionID, self.myCurrentVTypeDistribution) {
                let dist = self.myCurrentVTypeDistribution;
                unsafe { drop(Box::from_raw(dist.as_raw())) };
                throw_process_error(format!(
                    "Another vehicle type (or distribution) with the id '{}' exists.",
                    self.myCurrentVTypeDistributionID
                ));
            }
            self.myCurrentVTypeDistribution = Ptr::null();
        }
    }

    /// C++: `void MSRouteHandler::openRoute(const SUMOSAXAttributes &)` [virtual, protected]
    /// opens a route for reading
    // void
    // MSRouteHandler::openRoute(const SUMOSAXAttributes& attrs) {
    //     myActiveRoute.clear();
    //     myInsertStopEdgesAt = -1;
    //     // check whether the id is really necessary
    //     std::string rid;
    //     if (myCurrentRouteDistribution != nullptr) {
    //         myActiveRouteID = myCurrentRouteDistributionID + "#" + toString(myCurrentRouteDistribution->getProbs().size()); // !!! document this
    //         rid =  "distribution '" + myCurrentRouteDistributionID + "'";
    //     } else if (myVehicleParameter != nullptr) {
    //         // ok, a vehicle is wrapping the route,
    //         //  we may use this vehicle's id as default
    //         myActiveRouteID = "!" + myVehicleParameter->id; // !!! document this
    //         if (attrs.hasAttribute(SUMO_ATTR_ID)) {
    //             WRITE_WARNINGF(TL("Ids of internal routes are ignored (vehicle '%')."), myVehicleParameter->id);
    //         }
    //     } else {
    //         bool ok = true;
    //         myActiveRouteID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok, false);
    //         if (!ok) {
    //             return;
    //         }
    //         rid = "'" + myActiveRouteID + "'";
    //     }
    //     if (myVehicleParameter != nullptr) { // have to do this here for nested route distributions
    //         rid =  "for vehicle '" + myVehicleParameter->id + "'";
    //     }
    //     bool ok = true;
    //     if (attrs.hasAttribute(SUMO_ATTR_EDGES)) {
    //         MSEdge::parseEdgesList(attrs.get<std::string>(SUMO_ATTR_EDGES, myActiveRouteID.c_str(), ok), myActiveRoute, rid);
    //     }
    //     myActiveRouteRefID = attrs.getOpt<std::string>(SUMO_ATTR_REFID, myActiveRouteID.c_str(), ok, "");
    //     if (myActiveRouteRefID != "" && MSRoute::dictionary(myActiveRouteRefID, &myParsingRNG) == nullptr) {
    //         throw ProcessError(TLF("Invalid reference to route '%' in route %.", myActiveRouteRefID, rid));
    //     }
    //     myActiveRouteProbability = attrs.getOpt<double>(SUMO_ATTR_PROB, myActiveRouteID.c_str(), ok, DEFAULT_VEH_PROB);
    //     myActiveRouteColor = attrs.hasAttribute(SUMO_ATTR_COLOR) ? new RGBColor(attrs.get<RGBColor>(SUMO_ATTR_COLOR, myActiveRouteID.c_str(), ok)) : nullptr;
    //     myActiveRouteRepeat = attrs.getOpt<int>(SUMO_ATTR_REPEAT, myActiveRouteID.c_str(), ok, 0);
    //     myActiveRouteReplacedAtTime = attrs.getOptSUMOTimeReporting(SUMO_ATTR_REPLACED_AT_TIME, myActiveRouteID.c_str(), ok, -1);
    //     myActiveRouteReplacedIndex = attrs.getOpt<int>(SUMO_ATTR_REPLACED_ON_INDEX, myActiveRouteID.c_str(), ok, 0);
    //     myActiveRoutePeriod = attrs.getOptSUMOTimeReporting(SUMO_ATTR_CYCLETIME, myActiveRouteID.c_str(), ok,
    //                           // handle obsolete attribute name
    //                           attrs.getOptSUMOTimeReporting(SUMO_ATTR_PERIOD, myActiveRouteID.c_str(), ok, 0));
    //     myActiveRoutePermanent = attrs.getOpt<bool>(SUMO_ATTR_STATE, myActiveRouteID.c_str(), ok, false);
    //     if (attrs.hasAttribute(SUMO_ATTR_PERIOD)) {
    //         WRITE_WARNING(TL("Attribute 'period' is deprecated for route. Use 'cycleTime' instead."));
    //     }
    //     myCurrentCosts = attrs.getOpt<double>(SUMO_ATTR_COST, myActiveRouteID.c_str(), ok, -1);
    //     if (ok && myCurrentCosts != -1 && myCurrentCosts < 0) {
    //         WRITE_WARNING(TLF("Invalid cost for route '%'.", myActiveRouteID));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn open_route(&mut self, attrs: &SUMOSAXAttributes) {
        self.myActiveRoute.clear();
        self.myInsertStopEdgesAt = -1;
        // check whether the id is really necessary
        let mut rid: String;
        if !self.myCurrentRouteDistribution.is_null() {
            self.myActiveRouteID = format!("{}#{}", self.myCurrentRouteDistributionID, self.myCurrentRouteDistribution.get_probs().len());
            rid = format!("distribution '{}'", self.myCurrentRouteDistributionID);
        } else if !self.myVehicleParameter.is_null() {
            // ok, a vehicle is wrapping the route,
            //  we may use this vehicle's id as default
            self.myActiveRouteID = format!("!{}", self.myVehicleParameter.id);
            if attrs.has_attribute(SUMO_ATTR_ID) {
                write_warningf!("Ids of internal routes are ignored (vehicle '%').", self.myVehicleParameter.id);
            }
            rid = String::new();
        } else {
            let mut ok = true;
            self.myActiveRouteID = attrs.get(SUMO_ATTR_ID, "", &mut ok, false);
            if !ok {
                return;
            }
            rid = format!("'{}'", self.myActiveRouteID);
        }
        if !self.myVehicleParameter.is_null() {
            // have to do this here for nested route distributions
            rid = format!("for vehicle '{}'", self.myVehicleParameter.id);
        }
        let mut ok = true;
        if attrs.has_attribute(SUMO_ATTR_EDGES) {
            let edges: String = attrs.get(SUMO_ATTR_EDGES, &self.myActiveRouteID, &mut ok, true);
            MSEdge::parse_edges_list(&edges, &mut self.myActiveRoute, &rid);
        }
        self.myActiveRouteRefID = attrs.get_opt(SUMO_ATTR_REFID, &self.myActiveRouteID, &mut ok, String::new(), true);
        if self.myActiveRouteRefID != "" && !MSRoute::has_route(&self.myActiveRouteRefID) {
            throw_process_error(format!("Invalid reference to route '{}' in route {}.", self.myActiveRouteRefID, rid));
        }
        self.myActiveRouteProbability = attrs.get_opt(SUMO_ATTR_PROB, &self.myActiveRouteID, &mut ok, DEFAULT_VEH_PROB, true);
        self.myActiveRouteColor = if attrs.has_attribute(SUMO_ATTR_COLOR) {
            let color: RGBColor = attrs.get(SUMO_ATTR_COLOR, &self.myActiveRouteID, &mut ok, true);
            Ptr::from_raw(Box::into_raw(Box::new(color)))
        } else {
            Ptr::null()
        };
        self.myActiveRouteRepeat = attrs.get_opt(SUMO_ATTR_REPEAT, &self.myActiveRouteID, &mut ok, 0, true);
        self.myActiveRouteReplacedAtTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_REPLACED_AT_TIME, &self.myActiveRouteID, &mut ok, -1, true);
        self.myActiveRouteReplacedIndex = attrs.get_opt(SUMO_ATTR_REPLACED_ON_INDEX, &self.myActiveRouteID, &mut ok, 0, true);
        // handle obsolete attribute name
        let obsoletePeriod = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_PERIOD, &self.myActiveRouteID, &mut ok, 0, true);
        self.myActiveRoutePeriod = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_CYCLETIME, &self.myActiveRouteID, &mut ok, obsoletePeriod, true);
        self.myActiveRoutePermanent = attrs.get_opt(SUMO_ATTR_STATE, &self.myActiveRouteID, &mut ok, false, true);
        if attrs.has_attribute(SUMO_ATTR_PERIOD) {
            utils::common::msg_handler::write_warning("Attribute 'period' is deprecated for route. Use 'cycleTime' instead.");
        }
        self.myCurrentCosts = attrs.get_opt(SUMO_ATTR_COST, &self.myActiveRouteID, &mut ok, -1.0, true);
        if ok && self.myCurrentCosts != -1.0 && self.myCurrentCosts < 0.0 {
            utils::common::msg_handler::write_warning(&format!("Invalid cost for route '{}'.", self.myActiveRouteID));
        }
    }

    /// C++: `void MSRouteHandler::openFlow(const SUMOSAXAttributes &)` [virtual, protected]
    /// opens a flow for reading
    // void
    // MSRouteHandler::openFlow(const SUMOSAXAttributes& /*attrs*/) {
    //     // Currently unused
    // }
    #[allow(unused_variables)]
    pub fn open_flow(&mut self, attrs: &SUMOSAXAttributes) {
        // C++ body is empty (currently unused)
    }

    /// C++: `void MSRouteHandler::openRouteFlow(const SUMOSAXAttributes &)` [virtual, protected]
    /// opens a route flow for reading
    // void
    // MSRouteHandler::openRouteFlow(const SUMOSAXAttributes& /*attrs*/) {
    //     // Currently unused
    // }
    #[allow(unused_variables)]
    pub fn open_route_flow(&mut self, attrs: &SUMOSAXAttributes) {
        // C++ body is empty (currently unused)
    }

    /// C++: `void MSRouteHandler::openTrip(const SUMOSAXAttributes &)` [virtual, protected]
    /// opens a trip for reading
    // void
    // MSRouteHandler::openTrip(const SUMOSAXAttributes& /*attrs*/) {
    //     // Currently unsued
    // }
    #[allow(unused_variables)]
    pub fn open_trip(&mut self, attrs: &SUMOSAXAttributes) {
        // C++ body is empty (currently unused)
    }

    /// C++: `void MSRouteHandler::closeRoute(const bool)` [virtual, protected]
    /// closes (ends) the building of a route.
    // void
    // MSRouteHandler::closeRoute(const bool mayBeDisconnected) {
    //     std::string type = "vehicle";
    //     if (mayBeDisconnected) {
    //         if (myVehicleParameter->repetitionNumber >= 0) {
    //             type = "flow";
    //         } else {
    //             type = "trip";
    //         }
    //     }
    //
    //     try {
    //         const bool mustReroute = myActiveRoute.size() == 0 && myActiveRouteStops.size() != 0;
    //         if (mustReroute) {
    //             // implicit route from stops
    //             for (const SUMOVehicleParameter::Stop& stop : myActiveRouteStops) {
    //                 myActiveRoute.push_back(MSEdge::dictionary(stop.edge));
    //             }
    //         }
    //         if (myActiveRoute.size() == 0) {
    //             delete myActiveRouteColor;
    //             myActiveRouteColor = nullptr;
    //             if (myActiveRouteRefID != "" && myCurrentRouteDistribution != nullptr) {
    //                 ConstMSRoutePtr route = MSRoute::dictionary(myActiveRouteRefID, &myParsingRNG);
    //                 if (route != nullptr) {
    //                     myCurrentRouteDistribution->add(route, myActiveRouteProbability);
    //                 }
    //                 myActiveRouteID = "";
    //                 myActiveRouteRefID = "";
    //                 return;
    //             }
    //             if (myVehicleParameter != nullptr) {
    //                 throw ProcessError("The route for " + type + " '" + myVehicleParameter->id + "' has no edges.");
    //             } else {
    //                 throw ProcessError(TLF("Route '%' has no edges.", myActiveRouteID));
    //             }
    //         }
    //         if (myActiveRoute.size() == 1 && myActiveRoute.front()->isTazConnector()) {
    //             throw ProcessError("The routing information for " + type + " '" + myVehicleParameter->id + "' is insufficient.");
    //         }
    //         if (myActiveRouteRepeat > 0) {
    //             // duplicate route
    //             ConstMSEdgeVector tmpEdges = myActiveRoute;
    //             auto tmpStops = myActiveRouteStops;
    //
    //             if (check_routes()) {
    //                 SUMOVehicleClass vClass = SVC_IGNORING;
    //                 std::string errSuffix = ".";
    //                 if (myVehicleParameter != nullptr) {
    //                     MSVehicleControl& vehControl = MSNet::getInstance()->getVehicleControl();
    //                     MSVehicleType* vtype = vehControl.getVType(myVehicleParameter->vtypeid, &myParsingRNG);
    //                     if (vtype != nullptr) {
    //                         vClass = vtype->getVehicleClass();
    //                         errSuffix = TLF(" for vehicle '%' with vClass %.", myVehicleParameter->id, getVehicleClassNames(vClass));
    //                     }
    //                 }
    //                 if (myActiveRoute.size() > 0 && !myActiveRoute.back()->isConnectedTo(*myActiveRoute.front(), vClass)) {
    //                     if (tmpStops.size() == 0 || tmpStops.back().jump < 0) {
    //                         throw ProcessError(TLF("Disconnected route '%' when repeating. Last edge '%' is not connected to first edge '%'%",
    //                                                myActiveRouteID, myActiveRoute.back()->getID(), myActiveRoute.front()->getID(), errSuffix));
    //                     }
    //                 }
    //             }
    //             for (int i = 0; i < myActiveRouteRepeat; i++) {
    //                 myActiveRoute.insert(myActiveRoute.begin(), tmpEdges.begin(), tmpEdges.end());
    //                 for (SUMOVehicleParameter::Stop stop : tmpStops) {
    //                     if (stop.until > 0) {
    //                         if (myActiveRoutePeriod <= 0) {
    //                             const std::string description = myVehicleParameter != nullptr
    //                                                             ?  "for " + type + " '" + myVehicleParameter->id + "'"
    //                                                             :  "'" + myActiveRouteID + "'";
    //                             throw ProcessError(TLF("Cannot repeat stops with 'until' in route % because no cycleTime is defined.", description));
    //                         }
    //                         stop.until += myActiveRoutePeriod * (i + 1);
    //                     }
    //                     if (stop.arrival > 0) {
    //                         if (myActiveRoutePeriod <= 0) {
    //                             const std::string description = myVehicleParameter != nullptr
    //                                                             ?  "for " + type + " '" + myVehicleParameter->id + "'"
    //                                                             :  "'" + myActiveRouteID + "'";
    //                             throw ProcessError(TLF("Cannot repeat stops with 'arrival' in route % because no cycleTime is defined.", description));
    //                         }
    //                         stop.arrival += myActiveRoutePeriod * (i + 1);
    //                     }
    //                     stop.index = STOP_INDEX_REPEAT;
    //                     myActiveRouteStops.push_back(stop);
    //                 }
    //             }
    //             if (myActiveRouteStops.size() > 0) {
    //                 // never jump on the last stop of a repeating route
    //                 myActiveRouteStops.back().jump = -1;
    //             }
    //         }
    //         MSRoute* const route = new MSRoute(myActiveRouteID, myActiveRoute,
    //                                            myAmLoadingState ? myActiveRoutePermanent : myVehicleParameter == nullptr,
    //                                            myActiveRouteColor, myActiveRouteStops,
    //                                            myActiveRouteReplacedAtTime, myActiveRouteReplacedIndex);
    //         route->setPeriod(myActiveRoutePeriod);
    //         route->setCosts(myCurrentCosts);
    //         route->setReroute(mustReroute);
    //         myActiveRoute.clear();
    //         ConstMSRoutePtr constRoute = std::shared_ptr<const MSRoute>(route);
    //         if (!MSRoute::dictionary(myActiveRouteID, constRoute)) {
    //             if (!state_loaded()) {
    //                 if (myVehicleParameter != nullptr) {
    //                     if (MSNet::getInstance()->getVehicleControl().getVehicle(myVehicleParameter->id) == nullptr) {
    //                         throw ProcessError("Another route for " + type + " '" + myVehicleParameter->id + "' exists.");
    //                     } else {
    //                         throw ProcessError(TLF("A vehicle with id '%' already exists.", myVehicleParameter->id));
    //                     }
    //                 } else {
    //                     throw ProcessError(TLF("Another route (or distribution) with the id '%' exists.", myActiveRouteID));
    //                 }
    //             }
    //         } else {
    //             if (myCurrentRouteDistribution != nullptr) {
    //                 myCurrentRouteDistribution->add(constRoute, myActiveRouteProbability);
    //             }
    //         }
    //         myActiveRouteID = "";
    //         myActiveRouteColor = nullptr;
    //         myActiveRouteStops.clear();
    //         myActiveRouteRepeat = 0;
    //     } catch (ProcessError&) {
    //         deleteActivePlanAndVehicleParameter();
    //         throw;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_route(&mut self, mayBeDisconnected: bool) {
        let mut typeName = "vehicle";
        if mayBeDisconnected {
            if self.myVehicleParameter.repetitionNumber >= 0 {
                typeName = "flow";
            } else {
                typeName = "trip";
            }
        }

        // C++ try/catch(ProcessError&): deleteActivePlanAndVehicleParameter, then rethrow
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mustReroute = self.myActiveRoute.len() == 0 && self.myActiveRouteStops.len() != 0;
            if mustReroute {
                // implicit route from stops
                let stopEdges: Vec<Ptr<MSEdge>> = self.myActiveRouteStops.iter().map(|stop| MSEdge::dictionary_2(&stop.edge)).collect();
                self.myActiveRoute.extend(stopEdges);
            }
            if self.myActiveRoute.len() == 0 {
                delete_ptr(self.myActiveRouteColor);
                self.myActiveRouteColor = Ptr::null();
                if !self.myActiveRouteRefID.is_empty() && !self.myCurrentRouteDistribution.is_null() {
                    if MSRoute::has_route(&self.myActiveRouteRefID) {
                        let route = MSRoute::dictionary_3(&self.myActiveRouteRefID, SUMORouteHandler::get_parsing_rng());
                        self.myCurrentRouteDistribution.m().add(route, self.myActiveRouteProbability, false);
                    }
                    self.myActiveRouteID = String::new();
                    self.myActiveRouteRefID = String::new();
                    return;
                }
                if !self.myVehicleParameter.is_null() {
                    throw_process_error(format!("The route for {} '{}' has no edges.", typeName, self.myVehicleParameter.id));
                } else {
                    throw_process_error(format!("Route '{}' has no edges.", self.myActiveRouteID));
                }
            }
            if self.myActiveRoute.len() == 1 && self.myActiveRoute[0].is_taz_connector() {
                throw_process_error(format!("The routing information for {} '{}' is insufficient.", typeName, self.myVehicleParameter.id));
            }
            if self.myActiveRouteRepeat > 0 {
                // duplicate route
                let tmpEdges: ConstMSEdgeVector = self.myActiveRoute.clone();
                let tmpStops: StopParVector = self.myActiveRouteStops.clone();

                if check_routes() {
                    let mut vClass = SVC_IGNORING;
                    let mut errSuffix = ".".to_string();
                    if !self.myVehicleParameter.is_null() {
                        let net = MSNet::get_instance();
                        let vehControl = net.m().get_vehicle_control();
                        let vtype = vehControl.get_v_type(&self.myVehicleParameter.vtypeid, SUMORouteHandler::get_parsing_rng(), false);
                        if !vtype.is_null() {
                            vClass = vtype.get_vehicle_class();
                            errSuffix = format!(
                                " for vehicle '{}' with vClass {}.",
                                self.myVehicleParameter.id,
                                utils::common::sumo_vehicle_class::get_vehicle_class_names(vClass, false)
                            );
                        }
                    }
                    if self.myActiveRoute.len() > 0 {
                        let front = self.myActiveRoute[0];
                        let back = *self.myActiveRoute.last().unwrap();
                        if !back.is_connected_to(&front, vClass, false) && (tmpStops.is_empty() || tmpStops.last().unwrap().jump < 0) {
                            throw_process_error(format!(
                                "Disconnected route '{}' when repeating. Last edge '{}' is not connected to first edge '{}'{}",
                                self.myActiveRouteID,
                                back.get_id(),
                                front.get_id(),
                                errSuffix
                            ));
                        }
                    }
                }
                for i in 0..self.myActiveRouteRepeat {
                    self.myActiveRoute.splice(0..0, tmpEdges.iter().copied());
                    for stop in tmpStops.iter() {
                        let mut stop = stop.clone();
                        if stop.until > 0 {
                            if self.myActiveRoutePeriod <= 0 {
                                let description = if !self.myVehicleParameter.is_null() {
                                    format!("for {} '{}'", typeName, self.myVehicleParameter.id)
                                } else {
                                    format!("'{}'", self.myActiveRouteID)
                                };
                                throw_process_error(format!(
                                    "Cannot repeat stops with 'until' in route {} because no cycleTime is defined.",
                                    description
                                ));
                            }
                            stop.until += self.myActiveRoutePeriod * (i + 1) as i64;
                        }
                        if stop.arrival > 0 {
                            if self.myActiveRoutePeriod <= 0 {
                                let description = if !self.myVehicleParameter.is_null() {
                                    format!("for {} '{}'", typeName, self.myVehicleParameter.id)
                                } else {
                                    format!("'{}'", self.myActiveRouteID)
                                };
                                throw_process_error(format!(
                                    "Cannot repeat stops with 'arrival' in route {} because no cycleTime is defined.",
                                    description
                                ));
                            }
                            stop.arrival += self.myActiveRoutePeriod * (i + 1) as i64;
                        }
                        stop.index = STOP_INDEX_REPEAT;
                        self.myActiveRouteStops.push(stop);
                    }
                }
                if self.myActiveRouteStops.len() > 0 {
                    // never jump on the last stop of a repeating route
                    self.myActiveRouteStops.last_mut().unwrap().jump = -1;
                }
            }
            let mut route = MSRoute::new(
                &self.myActiveRouteID,
                &self.myActiveRoute,
                if self.myAmLoadingState { self.myActiveRoutePermanent } else { self.myVehicleParameter.is_null() },
                self.myActiveRouteColor,
                &self.myActiveRouteStops,
                self.myActiveRouteReplacedAtTime,
                self.myActiveRouteReplacedIndex,
            );
            route.set_period(self.myActiveRoutePeriod);
            route.set_costs(self.myCurrentCosts);
            route.set_reroute(mustReroute);
            self.myActiveRoute.clear();
            let constRoute: ConstMSRoutePtr = Rc::new(route);
            if !MSRoute::dictionary(&self.myActiveRouteID, constRoute.clone()) {
                if !state_loaded() {
                    if !self.myVehicleParameter.is_null() {
                        let net = MSNet::get_instance();
                        if net.m().get_vehicle_control().get_vehicle(&self.myVehicleParameter.id).is_null() {
                            throw_process_error(format!("Another route for {} '{}' exists.", typeName, self.myVehicleParameter.id));
                        } else {
                            throw_process_error(format!("A vehicle with id '{}' already exists.", self.myVehicleParameter.id));
                        }
                    } else {
                        throw_process_error(format!("Another route (or distribution) with the id '{}' exists.", self.myActiveRouteID));
                    }
                }
            } else if !self.myCurrentRouteDistribution.is_null() {
                self.myCurrentRouteDistribution.m().add(constRoute, self.myActiveRouteProbability, false);
            }
            self.myActiveRouteID = String::new();
            self.myActiveRouteColor = Ptr::null();
            self.myActiveRouteStops.clear();
            self.myActiveRouteRepeat = 0;
        }));
        if let Err(payload) = result {
            self.delete_active_plan_and_vehicle_parameter();
            std::panic::resume_unwind(payload);
        }
    }

    /// C++: `void MSRouteHandler::openRouteDistribution(const SUMOSAXAttributes &)` [virtual, protected]
    /// opens a route distribution for reading
    // void
    // MSRouteHandler::openRouteDistribution(const SUMOSAXAttributes& attrs) {
    //     // check whether the id is really necessary
    //     if (myVehicleParameter != nullptr) {
    //         // ok, a vehicle is wrapping the route,
    //         //  we may use this vehicle's id as default
    //         myCurrentRouteDistributionID = "!" + myVehicleParameter->id; // !!! document this
    //         // we have to record this or we cannot remove the distribution later
    //         myVehicleParameter->routeid = myCurrentRouteDistributionID;
    //     } else {
    //         bool ok = true;
    //         myCurrentRouteDistributionID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //         if (!ok) {
    //             return;
    //         }
    //     }
    //     myCurrentRouteDistribution = new RandomDistributor<ConstMSRoutePtr>();
    //     std::vector<double> probs;
    //     if (attrs.hasAttribute(SUMO_ATTR_PROBS)) {
    //         bool ok = true;
    //         StringTokenizer st(attrs.get<std::string>(SUMO_ATTR_PROBS, myCurrentRouteDistributionID.c_str(), ok));
    //         while (st.hasNext()) {
    //             probs.push_back(StringUtils::toDoubleSecure(st.next(), 1.0));
    //         }
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_ROUTES)) {
    //         bool ok = true;
    //         StringTokenizer st(attrs.get<std::string>(SUMO_ATTR_ROUTES, myCurrentRouteDistributionID.c_str(), ok));
    //         int probIndex = 0;
    //         while (st.hasNext()) {
    //             std::string routeID = st.next();
    //             ConstMSRoutePtr route = MSRoute::dictionary(routeID, &myParsingRNG);
    //             if (route == nullptr) {
    //                 throw ProcessError("Unknown route '" + routeID + "' in distribution '" + myCurrentRouteDistributionID + "'.");
    //             }
    //             const double prob = ((int)probs.size() > probIndex ? probs[probIndex] : 1.0);
    //             myCurrentRouteDistribution->add(route, prob, false);
    //             probIndex++;
    //         }
    //         if (probs.size() > 0 && probIndex != (int)probs.size()) {
    //             WRITE_WARNING("Got " + toString(probs.size()) + " probabilities for " + toString(probIndex) +
    //                           " routes in routeDistribution '" + myCurrentRouteDistributionID + "'");
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn open_route_distribution(&mut self, attrs: &SUMOSAXAttributes) {
        // check whether the id is really necessary
        if !self.myVehicleParameter.is_null() {
            // ok, a vehicle is wrapping the route,
            //  we may use this vehicle's id as default
            self.myCurrentRouteDistributionID = format!("!{}", self.myVehicleParameter.id);
            // we have to record this or we cannot remove the distribution later
            self.myVehicleParameter.m().routeid = self.myCurrentRouteDistributionID.clone();
        } else {
            let mut ok = true;
            self.myCurrentRouteDistributionID = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
            if !ok {
                return;
            }
        }
        self.myCurrentRouteDistribution = Ptr::from_raw(Box::into_raw(Box::new(RandomDistributor::default())));
        let mut probs: Vec<f64> = Vec::new();
        if attrs.has_attribute(SUMO_ATTR_PROBS) {
            let mut ok = true;
            let probsStr: String = attrs.get(SUMO_ATTR_PROBS, &self.myCurrentRouteDistributionID, &mut ok, true);
            let mut st = StringTokenizer::new(&probsStr);
            while st.has_next() {
                let next = st.next().unwrap();
                probs.push(utils::common::string_utils::to_double_secure(&next, 1.0).unwrap_or(1.0));
            }
        }
        if attrs.has_attribute(SUMO_ATTR_ROUTES) {
            let mut ok = true;
            let routesStr: String = attrs.get(SUMO_ATTR_ROUTES, &self.myCurrentRouteDistributionID, &mut ok, true);
            let mut st = StringTokenizer::new(&routesStr);
            let mut probIndex: i32 = 0;
            while st.has_next() {
                let routeID = st.next().unwrap();
                if !MSRoute::has_route(&routeID) {
                    throw_process_error(format!("Unknown route '{}' in distribution '{}'.", routeID, self.myCurrentRouteDistributionID));
                }
                let route = MSRoute::dictionary_3(&routeID, SUMORouteHandler::get_parsing_rng());
                let prob = if (probs.len() as i32) > probIndex { probs[probIndex as usize] } else { 1.0 };
                self.myCurrentRouteDistribution.m().add(route, prob, false);
                probIndex += 1;
            }
            if !probs.is_empty() && probIndex != probs.len() as i32 {
                utils::common::msg_handler::write_warning(&format!(
                    "Got {} probabilities for {} routes in routeDistribution '{}'",
                    probs.len(),
                    probIndex,
                    self.myCurrentRouteDistributionID
                ));
            }
        }
    }

    /// C++: `void MSRouteHandler::closeRouteDistribution()` [virtual, protected]
    /// closes (ends) the building of a distribution
    // void
    // MSRouteHandler::closeRouteDistribution() {
    //     if (myCurrentRouteDistribution != nullptr) {
    //         const bool haveSameID = MSRoute::dictionary(myCurrentRouteDistributionID, &myParsingRNG) != nullptr;
    //         if (state_loaded() && haveSameID) {
    //             delete myCurrentRouteDistribution;
    //             myCurrentRouteDistribution = nullptr;
    //             return;
    //         }
    //         if (haveSameID) {
    //             delete myCurrentRouteDistribution;
    //             throw ProcessError(TLF("Another route (or distribution) with the id '%' exists.", myCurrentRouteDistributionID));
    //         }
    //         if (myCurrentRouteDistribution->getOverallProb() == 0) {
    //             delete myCurrentRouteDistribution;
    //             throw ProcessError(TLF("Route distribution '%' is empty.", myCurrentRouteDistributionID));
    //         }
    //         MSRoute::dictionary(myCurrentRouteDistributionID, myCurrentRouteDistribution, myVehicleParameter == nullptr);
    //         myCurrentRouteDistribution = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_route_distribution(&mut self) {
        if !self.myCurrentRouteDistribution.is_null() {
            let haveSameID = MSRoute::has_route(&self.myCurrentRouteDistributionID);
            if state_loaded() && haveSameID {
                delete_ptr(self.myCurrentRouteDistribution);
                self.myCurrentRouteDistribution = Ptr::null();
                return;
            }
            if haveSameID {
                delete_ptr(self.myCurrentRouteDistribution);
                throw_process_error(format!("Another route (or distribution) with the id '{}' exists.", self.myCurrentRouteDistributionID));
            }
            if self.myCurrentRouteDistribution.m().get_overall_prob() == 0.0 {
                delete_ptr(self.myCurrentRouteDistribution);
                throw_process_error(format!("Route distribution '{}' is empty.", self.myCurrentRouteDistributionID));
            }
            MSRoute::dictionary_2(&self.myCurrentRouteDistributionID, self.myCurrentRouteDistribution, self.myVehicleParameter.is_null());
            self.myCurrentRouteDistribution = Ptr::null();
        }
    }

    /// C++: `void MSRouteHandler::closeVehicle()` [virtual, protected]
    /// Ends the processing of a vehicle (note: is virtual because is reimplemented in MSStateHandler)
    // void
    // MSRouteHandler::closeVehicle() {
    //     // get nested route
    //     const std::string embeddedRouteID = "!" + myVehicleParameter->id;
    //     ConstMSRoutePtr route = nullptr;
    //     if (myReplayRerouting) {
    //         RandomDistributor<ConstMSRoutePtr>* rDist = MSRoute::distDictionary(embeddedRouteID);
    //         if (rDist != nullptr && rDist->getVals().size() > 0) {
    //             route = rDist->getVals().front();
    //         }
    //     }
    //     if (route == nullptr) {
    //         route = MSRoute::dictionary(embeddedRouteID, &myParsingRNG);
    //     }
    //     MSVehicleControl& vehControl = MSNet::getInstance()->getVehicleControl();
    //     if (myVehicleParameter->departProcedure == DepartDefinition::GIVEN) {
    //         // let's check whether this vehicle had to depart before the simulation starts
    //         if (!(myAddVehiclesDirectly || checkLastDepart()) || (myVehicleParameter->depart <= MSNet::getInstance()->getStateLoaderTime() && !myAmLoadingState)) {
    //             mySkippedVehicles.insert(myVehicleParameter->id);
    //             return;
    //         }
    //     }
    //
    //     // get the vehicle's type
    //     MSVehicleType* vtype = nullptr;
    //
    //     try {
    //         if (myVehicleParameter->vtypeid != "") {
    //             vtype = vehControl.getVType(myVehicleParameter->vtypeid, &myParsingRNG);
    //             if (vtype == nullptr) {
    //                 throw ProcessError("The vehicle type '" + myVehicleParameter->vtypeid + "' for vehicle '" + myVehicleParameter->id + "' is not known.");
    //             }
    //             if (vtype->getVehicleClass() == SVC_PEDESTRIAN) {
    //                 WRITE_WARNINGF(TL("Vehicle type '%' with vClass=pedestrian should only be used for persons and not for vehicle '%'."), vtype->getID(), myVehicleParameter->id);
    //             }
    //         } else {
    //             // there should be one (at least the default one)
    //             vtype = vehControl.getVType(DEFAULT_VTYPE_ID, &myParsingRNG);
    //         }
    //         if (myVehicleParameter->wasSet(VEHPARS_ROUTE_SET)) {
    //             // if the route id was given, prefer that one
    //             if (route != nullptr && !myAmLoadingState) {
    //                 WRITE_WARNINGF(TL("Ignoring child element 'route' for vehicle '%' because attribute 'route' is set."), myVehicleParameter->id);
    //             }
    //             route = MSRoute::dictionary(myVehicleParameter->routeid, &myParsingRNG);
    //         }
    //         if (route == nullptr) {
    //             // nothing found? -> error
    //             if (myVehicleParameter->routeid != "") {
    //                 throw ProcessError("The route '" + myVehicleParameter->routeid + "' for vehicle '" + myVehicleParameter->id + "' is not known.");
    //             } else {
    //                 throw ProcessError(TLF("Vehicle '%' has no route.", myVehicleParameter->id));
    //             }
    //         }
    //         myActiveRouteID = "";
    //
    //     } catch (ProcessError&) {
    //         deleteActivePlanAndVehicleParameter();
    //         throw;
    //     }
    //     if (route->mustReroute()) {
    //         myVehicleParameter->parametersSet |= VEHPARS_FORCE_REROUTE;
    //         if (myVehicleParameter->stops.size() > 0) {
    //             route = addVehicleStopsToImplicitRoute(route, false);
    //         }
    //     }
    //     if (myVehicleParameter->departEdgeProcedure != RouteIndexDefinition::DEFAULT) {
    //         if ((myVehicleParameter->parametersSet & VEHPARS_FORCE_REROUTE) == 0 &&
    //                 myVehicleParameter->departEdgeProcedure == RouteIndexDefinition::GIVEN &&
    //                 myVehicleParameter->departEdge >= (int)route->getEdges().size()) {
    //             throw ProcessError("Vehicle '" + myVehicleParameter->id + "' has invalid departEdge index "
    //                                + toString(myVehicleParameter->departEdge) + " for route with " + toString(route->getEdges().size()) + " edges.");
    //         }
    //     }
    //     if (myVehicleParameter->arrivalEdgeProcedure != RouteIndexDefinition::DEFAULT) {
    //         if ((myVehicleParameter->parametersSet & VEHPARS_FORCE_REROUTE) == 0 &&
    //                 myVehicleParameter->arrivalEdgeProcedure == RouteIndexDefinition::GIVEN &&
    //                 myVehicleParameter->arrivalEdge >= (int)route->getEdges().size()) {
    //             throw ProcessError("Vehicle '" + myVehicleParameter->id + "' has invalid arrivalEdge index "
    //                                + toString(myVehicleParameter->arrivalEdge) + " for route with " + toString(route->getEdges().size()) + " edges.");
    //         }
    //     }
    //
    //     // try to build the vehicle
    //     SUMOVehicle* vehicle = nullptr;
    //     if (vehControl.getVehicle(myVehicleParameter->id) == nullptr) {
    //         MSVehicleControl::VehicleDefinitionSource source = (myAmLoadingState) ? MSVehicleControl::VehicleDefinitionSource::STATE : MSVehicleControl::VehicleDefinitionSource::ROUTEFILE;
    //         try {
    //             vehicle = vehControl.buildVehicle(myVehicleParameter, route, vtype, !check_routes(), source, !myAmLoadingState);
    //         } catch (const ProcessError& e) {
    //             myVehicleParameter = nullptr;
    //             if (!check_routes()) {
    //                 WRITE_WARNING(e.what());
    //                 vehControl.deleteVehicle(vehicle, true);
    //                 myVehicleParameter = nullptr;
    //                 vehicle = nullptr;
    //                 return;
    //             } else {
    //                 throw;
    //             }
    //         }
    //         const SUMOTime origDepart = myVehicleParameter->depart;
    //         // maybe we do not want this vehicle to be inserted due to scaling
    //         int quota = myAmLoadingState ? 1 : vehControl.getQuota(vehControl.getScale() * vtype->getParameter().scale);
    //         if (quota > 0) {
    //             registerLastDepart();
    //             myVehicleParameter->depart += MSNet::getInstance()->getInsertionControl().computeRandomDepartOffset();
    //             vehControl.addVehicle(myVehicleParameter->id, vehicle);
    //             if (myReplayRerouting) {
    //                 RandomDistributor<ConstMSRoutePtr>* rDist = MSRoute::distDictionary(embeddedRouteID);
    //                 if (rDist != nullptr) {
    //                     for (int i = 0; i < (int)rDist->getVals().size() - 1; i++) {
    //                         SUMOTime replacedAt = rDist->getVals()[i]->getReplacedTime();
    //                         auto* cmd = new Command_RouteReplacement(vehicle->getID(), rDist->getVals()[i + 1]);
    //                         if (replacedAt >= 0 && replacedAt == myVehicleParameter->depart) {
    //                             // routing in the insertion step happens *after* insertion
    //                             MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(cmd, replacedAt);
    //                         } else {
    //                             MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(cmd, replacedAt);
    //                         }
    //                     }
    //                 }
    //             }
    //             int offset = 0;
    //             for (int i = 1; i < quota; i++) {
    //                 if (vehicle->getParameter().departProcedure == DepartDefinition::GIVEN || vehicle->getParameter().departProcedure == DepartDefinition::BEGIN) {
    //                     MSNet::getInstance()->getInsertionControl().add(vehicle);
    //                 }
    //                 SUMOVehicleParameter* newPars = new SUMOVehicleParameter(*myVehicleParameter);
    //                 newPars->id = myVehicleParameter->id + myScaleSuffix + toString(i + offset);
    //                 while (vehControl.getVehicle(newPars->id) != nullptr) {
    //                     offset += 1;
    //                     newPars->id = myVehicleParameter->id + myScaleSuffix + toString(i + offset);
    //                 }
    //                 newPars->depart = origDepart + MSNet::getInstance()->getInsertionControl().computeRandomDepartOffset();
    //                 if (vehControl.hasVTypeDistribution(myVehicleParameter->vtypeid)) {
    //                     // resample type
    //                     vtype = vehControl.getVType(myVehicleParameter->vtypeid, &myParsingRNG);
    //                 }
    //                 vehicle = vehControl.buildVehicle(newPars, route, vtype, !check_routes(), source, !myAmLoadingState);
    //                 vehControl.addVehicle(newPars->id, vehicle);
    //             }
    //             myVehicleParameter = nullptr;
    //         } else {
    //             vehControl.deleteVehicle(vehicle, true);
    //             myVehicleParameter = nullptr;
    //             vehicle = nullptr;
    //         }
    //     } else {
    //         // strange: another vehicle with the same id already exists
    //         if (!state_loaded()) {
    //             // and was not loaded while loading a simulation state
    //             // -> error
    //             std::string veh_id = myVehicleParameter->id;
    //             deleteActivePlanAndVehicleParameter();
    //             std::string scaleWarning = "";
    //             if (vehControl.getScale() * vtype->getParameter().scale > 1 && veh_id.find(myScaleSuffix) != std::string::npos) {
    //                 scaleWarning = "\n   (Possibly duplicate id due to using option --scale. Set option --scale-suffix to prevent this)";
    //             }
    //             throw ProcessError("Another vehicle with the id '" + veh_id + "' exists." + scaleWarning);
    //         } else {
    //             // ok, it seems to be loaded previously while loading a simulation state
    //             vehicle = nullptr;
    //         }
    //     }
    //     // check whether the vehicle shall be added directly to the network or
    //     //  shall stay in the internal buffer
    //     if (vehicle != nullptr) {
    //         if (vehicle->getParameter().departProcedure == DepartDefinition::GIVEN || vehicle->getParameter().departProcedure == DepartDefinition::BEGIN) {
    //             MSNet::getInstance()->getInsertionControl().add(vehicle);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_vehicle(&mut self) {
        // get nested route
        let embeddedRouteID = format!("!{}", self.myVehicleParameter.id);
        let mut route: Option<ConstMSRoutePtr> = None;
        if self.myReplayRerouting {
            let rDist = MSRoute::dist_dictionary(&embeddedRouteID);
            if !rDist.is_null() && rDist.get_vals().len() > 0 {
                route = Some(rDist.get_vals()[0].clone());
            }
        }
        if route.is_none() {
            route = lookup_route(&embeddedRouteID);
        }
        let net = MSNet::get_instance();
        let vehControl = net.m().get_vehicle_control();
        if self.myVehicleParameter.departProcedure == DepartDefinition::GIVEN {
            // let's check whether this vehicle had to depart before the simulation starts
            if !(self.myAddVehiclesDirectly || self.check_last_depart())
                || (self.myVehicleParameter.depart <= net.m().get_state_loader_time() && !self.myAmLoadingState)
            {
                // shortcut: mySkippedVehicles is an i32 in the skeleton (C++ std::set of ids), skipped ids are not recorded until SUMORouteHandler is fixed
                return;
            }
        }

        // get the vehicle's type
        // C++ try/catch(ProcessError&): deleteActivePlanAndVehicleParameter, then rethrow
        let checked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut vtype: Ptr<MSVehicleType> = Ptr::null();
            if !self.myVehicleParameter.vtypeid.is_empty() {
                vtype = vehControl.get_v_type(&self.myVehicleParameter.vtypeid, SUMORouteHandler::get_parsing_rng(), false);
                if vtype.is_null() {
                    throw_process_error(format!(
                        "The vehicle type '{}' for vehicle '{}' is not known.",
                        self.myVehicleParameter.vtypeid, self.myVehicleParameter.id
                    ));
                }
                if vtype.get_vehicle_class() == SVC_PEDESTRIAN {
                    let vtypeID = vtype.get_id().to_string();
                    write_warningf!(
                        "Vehicle type '%' with vClass=pedestrian should only be used for persons and not for vehicle '%'.",
                        vtypeID,
                        self.myVehicleParameter.id
                    );
                }
            } else {
                // there should be one (at least the default one)
                vtype = vehControl.get_v_type(DEFAULT_VTYPE_ID, SUMORouteHandler::get_parsing_rng(), false);
            }
            if self.myVehicleParameter.was_set(VEHPARS_ROUTE_SET) {
                // if the route id was given, prefer that one
                if route.is_some() && !self.myAmLoadingState {
                    write_warningf!(
                        "Ignoring child element 'route' for vehicle '%' because attribute 'route' is set.",
                        self.myVehicleParameter.id
                    );
                }
                route = lookup_route(&self.myVehicleParameter.routeid);
            }
            let route = match route.take() {
                Some(r) => r,
                None => {
                    // nothing found? -> error
                    if !self.myVehicleParameter.routeid.is_empty() {
                        throw_process_error(format!(
                            "The route '{}' for vehicle '{}' is not known.",
                            self.myVehicleParameter.routeid, self.myVehicleParameter.id
                        ));
                    } else {
                        throw_process_error(format!("Vehicle '{}' has no route.", self.myVehicleParameter.id));
                    }
                }
            };
            self.myActiveRouteID = String::new();
            (vtype, route)
        }));
        let (mut vtype, mut route) = match checked {
            Ok(v) => v,
            Err(payload) => {
                self.delete_active_plan_and_vehicle_parameter();
                std::panic::resume_unwind(payload);
            }
        };

        if route.must_reroute() {
            self.myVehicleParameter.m().parametersSet |= VEHPARS_FORCE_REROUTE;
            if self.myVehicleParameter.stops.len() > 0 {
                route = self.add_vehicle_stops_to_implicit_route(route, false);
            }
        }
        if self.myVehicleParameter.departEdgeProcedure != RouteIndexDefinition::DEFAULT {
            if (self.myVehicleParameter.parametersSet & VEHPARS_FORCE_REROUTE) == 0
                && self.myVehicleParameter.departEdgeProcedure == RouteIndexDefinition::GIVEN
                && self.myVehicleParameter.departEdge >= route.get_edges().len() as i32
            {
                throw_process_error(format!(
                    "Vehicle '{}' has invalid departEdge index {} for route with {} edges.",
                    self.myVehicleParameter.id,
                    self.myVehicleParameter.departEdge,
                    route.get_edges().len()
                ));
            }
        }
        if self.myVehicleParameter.arrivalEdgeProcedure != RouteIndexDefinition::DEFAULT {
            if (self.myVehicleParameter.parametersSet & VEHPARS_FORCE_REROUTE) == 0
                && self.myVehicleParameter.arrivalEdgeProcedure == RouteIndexDefinition::GIVEN
                && self.myVehicleParameter.arrivalEdge >= route.get_edges().len() as i32
            {
                throw_process_error(format!(
                    "Vehicle '{}' has invalid arrivalEdge index {} for route with {} edges.",
                    self.myVehicleParameter.id,
                    self.myVehicleParameter.arrivalEdge,
                    route.get_edges().len()
                ));
            }
        }

        // try to build the vehicle
        let mut vehicle: Ptr<SUMOVehicle> = Ptr::null();
        if vehControl.get_vehicle(&self.myVehicleParameter.id).is_null() {
            let source = if self.myAmLoadingState {
                MSVehicleControl_VehicleDefinitionSource::STATE
            } else {
                MSVehicleControl_VehicleDefinitionSource::ROUTEFILE
            };
            let built = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                vehControl.build_vehicle(
                    self.myVehicleParameter,
                    route.clone(),
                    vtype,
                    !check_routes(),
                    source,
                    !self.myAmLoadingState,
                )
            }));
            match built {
                Ok(v) => vehicle = v,
                Err(payload) => {
                    self.myVehicleParameter = Ptr::null();
                    if !check_routes() {
                        utils::common::msg_handler::write_warning(&exception_what(&payload));
                        vehControl.delete_vehicle(vehicle, true, false);
                        self.myVehicleParameter = Ptr::null();
                        return;
                    } else {
                        std::panic::resume_unwind(payload);
                    }
                }
            }
            let origDepart = self.myVehicleParameter.depart;
            // maybe we do not want this vehicle to be inserted due to scaling
            let quota = if self.myAmLoadingState {
                1
            } else {
                vehControl.get_quota(vehControl.get_scale() * vtype.get_parameter().scale, 0)
            };
            if quota > 0 {
                self.register_last_depart();
                let offset = net.m().get_insertion_control().compute_random_depart_offset();
                self.myVehicleParameter.m().depart += offset;
                vehControl.add_vehicle(&self.myVehicleParameter.id, vehicle);
                if self.myReplayRerouting {
                    let rDist = MSRoute::dist_dictionary(&embeddedRouteID);
                    if !rDist.is_null() {
                        let vals = rDist.get_vals();
                        for i in 0..(vals.len() as i32 - 1) {
                            let i = i as usize;
                            let replacedAt = vals[i].get_replaced_time();
                            let cmd = Box::into_raw(Box::new(Command_RouteReplacement::new(vehicle.get_id(), vals[i + 1].clone())));
                            let cmd: Ptr<dyn Command> = Ptr::from_raw(cmd as *mut dyn Command);
                            if replacedAt >= 0 && replacedAt == self.myVehicleParameter.depart {
                                // routing in the insertion step happens *after* insertion
                                net.m().get_end_of_timestep_events().m().add_event(cmd, replacedAt);
                            } else {
                                net.m().get_begin_of_timestep_events().m().add_event(cmd, replacedAt);
                            }
                        }
                    }
                }
                let mut offset: i32 = 0;
                for i in 1..quota {
                    let dp = vehicle.get_parameter().departProcedure;
                    if dp == DepartDefinition::GIVEN || dp == DepartDefinition::BEGIN {
                        net.m().get_insertion_control().add(vehicle);
                    }
                    let newPars = Ptr::from_raw(Box::into_raw(Box::new((*self.myVehicleParameter).clone())));
                    newPars.m().id = format!("{}{}{}", self.myVehicleParameter.id, self.myScaleSuffix, i + offset);
                    while !vehControl.get_vehicle(&newPars.m().id).is_null() {
                        offset += 1;
                        newPars.m().id = format!("{}{}{}", self.myVehicleParameter.id, self.myScaleSuffix, i + offset);
                    }
                    let depart = origDepart + net.m().get_insertion_control().compute_random_depart_offset();
                    newPars.m().depart = depart;
                    if vehControl.has_v_type_distribution(&self.myVehicleParameter.vtypeid) {
                        // resample type
                        vtype = vehControl.get_v_type(&self.myVehicleParameter.vtypeid, SUMORouteHandler::get_parsing_rng(), false);
                    }
                    vehicle = vehControl.build_vehicle(newPars, route.clone(), vtype, !check_routes(), source, !self.myAmLoadingState);
                    let newId = newPars.m().id.clone();
                    vehControl.add_vehicle(&newId, vehicle);
                }
                self.myVehicleParameter = Ptr::null();
            } else {
                vehControl.delete_vehicle(vehicle, true, false);
                self.myVehicleParameter = Ptr::null();
                vehicle = Ptr::null();
            }
        } else {
            // strange: another vehicle with the same id already exists
            if !state_loaded() {
                // and was not loaded while loading a simulation state -> error
                let veh_id = self.myVehicleParameter.id.clone();
                self.delete_active_plan_and_vehicle_parameter();
                let mut scaleWarning = String::new();
                if vehControl.get_scale() * vtype.get_parameter().scale > 1.0 && veh_id.contains(&self.myScaleSuffix) {
                    scaleWarning = "\n   (Possibly duplicate id due to using option --scale. Set option --scale-suffix to prevent this)".to_string();
                }
                throw_process_error(format!("Another vehicle with the id '{}' exists.{}", veh_id, scaleWarning));
            } else {
                // ok, it seems to be loaded previously while loading a simulation state
                vehicle = Ptr::null();
            }
        }

        // check whether the vehicle shall be added directly to the network or
        //  shall stay in the internal buffer
        if !vehicle.is_null() {
            let dp = vehicle.get_parameter().departProcedure;
            if dp == DepartDefinition::GIVEN || dp == DepartDefinition::BEGIN {
                net.m().get_insertion_control().add(vehicle);
            }
        }
    }

    /// C++: `void MSRouteHandler::closeVType()` [virtual, protected]
    /// Ends the processing of a vehicle type
    // void
    // MSRouteHandler::closeVType() {
    //     MSVehicleType* vehType = nullptr;
    //     if (myCurrentVTypeRef.empty()) {
    //         vehType = MSVehicleType::build(*myCurrentVType, getFileName());
    //         vehType->check();
    //         if (!MSNet::getInstance()->getVehicleControl().addVType(vehType)) {
    //             const std::string id = vehType->getID();
    //             delete vehType;
    //             if (!state_loaded()) {
    //                 throw ProcessError(TLF("Another vehicle type (or distribution) with the id '%' exists.", id));
    //             }
    //         }
    //     }
    //     if (myCurrentVTypeDistribution != nullptr) {
    //         if (!myCurrentVTypeRef.empty()) {
    //             const RandomDistributor<MSVehicleType*>* const dist = MSNet::getInstance()->getVehicleControl().getVTypeDistribution(myCurrentVTypeRef);
    //             if (dist != nullptr) {
    //                 const double distProb = (myCurrentVTypeProbability >= 0 ? myCurrentVTypeProbability : 1) / dist->getOverallProb();
    //                 std::vector<double>::const_iterator probIt = dist->getProbs().begin();
    //                 for (MSVehicleType* const type : dist->getVals()) {
    //                     myCurrentVTypeDistribution->add(type, distProb * *probIt);
    //                     probIt++;
    //                 }
    //                 return;
    //             } else {
    //                 vehType = MSNet::getInstance()->getVehicleControl().getVType(myCurrentVTypeRef);
    //             }
    //         }
    //         double probability = myCurrentVTypeProbability >= 0 ? myCurrentVTypeProbability : vehType->getDefaultProbability();
    //         myCurrentVTypeDistribution->add(vehType, probability);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_v_type(&mut self) {
        let mut vehType: Ptr<MSVehicleType> = Ptr::null();
        let net = MSNet::get_instance();
        if self.myCurrentVTypeRef.is_empty() {
            // shortcut: SUMOSAXHandler base is Unmapped so getFileName() is unreachable, upgrade when SUMORouteHandler.base is ported
            let fileName: String = panic!("SUMOSAXHandler::getFileName not ported");
            vehType = MSVehicleType::build(self.myCurrentVType.m(), &fileName);
            vehType.m().check();
            if !net.m().get_vehicle_control().add_v_type(vehType) {
                let id = vehType.get_id().to_string();
                delete_ptr(vehType);
                if !state_loaded() {
                    throw_process_error(format!("Another vehicle type (or distribution) with the id '{}' exists.", id));
                }
            }
        }
        if !self.myCurrentVTypeDistribution.is_null() {
            if !self.myCurrentVTypeRef.is_empty() {
                let dist = net.m().get_vehicle_control().get_v_type_distribution(&self.myCurrentVTypeRef);
                if !dist.is_null() {
                    let distProb = (if self.myCurrentVTypeProbability >= 0.0 { self.myCurrentVTypeProbability } else { 1.0 }) / dist.get_overall_prob();
                    let distProbs = dist.get_probs();
                    for (i, type_) in dist.get_vals().iter().enumerate() {
                        self.myCurrentVTypeDistribution.m().add(*type_, distProb * distProbs[i], false);
                    }
                    return;
                } else {
                    vehType = net.m().get_vehicle_control().get_v_type(&self.myCurrentVTypeRef, Ptr::null(), false);
                }
            }
            let probability = if self.myCurrentVTypeProbability >= 0.0 { self.myCurrentVTypeProbability } else { vehType.get_default_probability() };
            self.myCurrentVTypeDistribution.m().add(vehType, probability, false);
        }
    }

    /// C++: `void MSRouteHandler::closePerson()` [virtual, protected]
    /// Ends the processing of a person
    // void
    // MSRouteHandler::closePerson() {
    //     closeTransportable();
    // }
    #[allow(unused_variables)]
    pub fn close_person(&mut self) {
        self.close_transportable();
    }

    /// C++: `void MSRouteHandler::closePersonFlow()` [virtual, protected]
    /// Ends the processing of a personFlow
    // void
    // MSRouteHandler::closePersonFlow() {
    //     closeTransportableFlow();
    // }
    #[allow(unused_variables)]
    pub fn close_person_flow(&mut self) {
        self.close_transportable_flow();
    }

    /// C++: `void MSRouteHandler::closeContainer()` [virtual, protected]
    /// Ends the processing of a container
    // void
    // MSRouteHandler::closeContainer() {
    //     closeTransportable();
    // }
    #[allow(unused_variables)]
    pub fn close_container(&mut self) {
        self.close_transportable();
    }

    /// C++: `void MSRouteHandler::closeContainerFlow()` [virtual, protected]
    /// Ends the processing of a containerFlow
    // void
    // MSRouteHandler::closeContainerFlow() {
    //     closeTransportableFlow();
    // }
    #[allow(unused_variables)]
    pub fn close_container_flow(&mut self) {
        self.close_transportable_flow();
    }

    /// C++: `void MSRouteHandler::closeFlow()` [virtual, protected]
    /// Ends the processing of a flow
    // void
    // MSRouteHandler::closeFlow() {
    //     myInsertStopEdgesAt = -1;
    //     if (myVehicleParameter->repetitionNumber == 0) {
    //         delete myVehicleParameter;
    //         myVehicleParameter = nullptr;
    //         return;
    //     }
    //     // let's check whether vehicles had to depart before the simulation starts
    //     myVehicleParameter->repetitionsDone = 0;
    //     if (myVehicleParameter->repetitionProbability < 0) {
    //         const SUMOTime offsetToBegin = string2time(OptionsCont::getOptions().getString("begin")) - myVehicleParameter->depart;
    //         while (myVehicleParameter->repetitionTotalOffset < offsetToBegin) {
    //             myVehicleParameter->incrementFlow(1, &myParsingRNG);
    //             if (myVehicleParameter->repetitionsDone == myVehicleParameter->repetitionNumber) {
    //                 delete myVehicleParameter;
    //                 myVehicleParameter = nullptr;
    //                 return;
    //             }
    //         }
    //     }
    //     if (MSNet::getInstance()->getVehicleControl().getVType(myVehicleParameter->vtypeid, &myParsingRNG) == nullptr) {
    //         throw ProcessError("The vehicle type '" + myVehicleParameter->vtypeid + "' for flow '" + myVehicleParameter->id + "' is not known.");
    //     }
    //     if (myVehicleParameter->routeid[0] == '!' && MSRoute::dictionary(myVehicleParameter->routeid, &myParsingRNG) == nullptr) {
    //         myVehicleParameter->parametersSet |= VEHPARS_FORCE_REROUTE;
    //         closeRoute(true);
    //     }
    //     ConstMSRoutePtr route = MSRoute::dictionary(myVehicleParameter->routeid, &myParsingRNG);
    //     if (route == nullptr) {
    //         throw ProcessError("The route '" + myVehicleParameter->routeid + "' for flow '" + myVehicleParameter->id + "' is not known.");
    //     }
    //     if (route->mustReroute()) {
    //         myVehicleParameter->parametersSet |= VEHPARS_FORCE_REROUTE;
    //         if (myVehicleParameter->stops.size() > 0) {
    //             route = addVehicleStopsToImplicitRoute(route, true);
    //             myVehicleParameter->routeid = route->getID();
    //         }
    //     }
    //     if (myVehicleParameter->departEdgeProcedure != RouteIndexDefinition::DEFAULT) {
    //         if ((myVehicleParameter->parametersSet & VEHPARS_FORCE_REROUTE) == 0 &&
    //                 myVehicleParameter->departEdgeProcedure == RouteIndexDefinition::GIVEN &&
    //                 myVehicleParameter->departEdge >= (int)route->getEdges().size()) {
    //             throw ProcessError("Flow '" + myVehicleParameter->id + "' has invalid departEdge index "
    //                                + toString(myVehicleParameter->departEdge) + " for route with " + toString(route->getEdges().size()) + " edges.");
    //         }
    //     }
    //     if (myVehicleParameter->arrivalEdgeProcedure != RouteIndexDefinition::DEFAULT) {
    //         if ((myVehicleParameter->parametersSet & VEHPARS_FORCE_REROUTE) == 0 &&
    //                 myVehicleParameter->arrivalEdgeProcedure == RouteIndexDefinition::GIVEN &&
    //                 myVehicleParameter->arrivalEdge >= (int)route->getEdges().size()) {
    //             throw ProcessError("Flow '" + myVehicleParameter->id + "' has invalid arrivalEdge index "
    //                                + toString(myVehicleParameter->arrivalEdge) + " for route with " + toString(route->getEdges().size()) + " edges.");
    //         }
    //     }
    //     myActiveRouteID = "";
    //
    //     // check whether the vehicle shall be added directly to the network or
    //     //  shall stay in the internal buffer
    //     if (myAddVehiclesDirectly || checkLastDepart()) {
    //         if (MSNet::getInstance()->getInsertionControl().addFlow(myVehicleParameter)) {
    //             registerLastDepart();
    //         } else {
    //             if (state_loaded()) {
    //                 delete myVehicleParameter;
    //             } else {
    //                 throw ProcessError(TLF("Another flow with the id '%' exists.", myVehicleParameter->id));
    //             }
    //         }
    //     }
    //     myVehicleParameter = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn close_flow(&mut self) {
        self.myInsertStopEdgesAt = -1;
        if self.myVehicleParameter.repetitionNumber == 0 {
            delete_ptr(self.myVehicleParameter);
            self.myVehicleParameter = Ptr::null();
            return;
        }
        // let's check whether vehicles had to depart before the simulation starts
        self.myVehicleParameter.m().repetitionsDone = 0;
        if self.myVehicleParameter.repetitionProbability < 0.0 {
            let begin = utils::common::time::string2time(&utils::options::options_cont::get_string("begin"))
                .unwrap_or_else(|e| std::panic::panic_any(e));
            let offsetToBegin = begin - self.myVehicleParameter.depart;
            while self.myVehicleParameter.repetitionTotalOffset < offsetToBegin {
                self.myVehicleParameter.m().increment_flow(1.0, SUMORouteHandler::get_parsing_rng());
                if self.myVehicleParameter.repetitionsDone == self.myVehicleParameter.repetitionNumber {
                    delete_ptr(self.myVehicleParameter);
                    self.myVehicleParameter = Ptr::null();
                    return;
                }
            }
        }
        let net = MSNet::get_instance();
        if net.m().get_vehicle_control().get_v_type(&self.myVehicleParameter.vtypeid, SUMORouteHandler::get_parsing_rng(), false).is_null() {
            throw_process_error(format!(
                "The vehicle type '{}' for flow '{}' is not known.",
                self.myVehicleParameter.vtypeid, self.myVehicleParameter.id
            ));
        }
        if self.myVehicleParameter.routeid.starts_with('!') && !MSRoute::has_route(&self.myVehicleParameter.routeid) {
            self.myVehicleParameter.m().parametersSet |= VEHPARS_FORCE_REROUTE;
            self.close_route(true);
        }
        let mut route = match lookup_route(&self.myVehicleParameter.routeid) {
            Some(r) => r,
            None => throw_process_error(format!(
                "The route '{}' for flow '{}' is not known.",
                self.myVehicleParameter.routeid, self.myVehicleParameter.id
            )),
        };
        if route.must_reroute() {
            self.myVehicleParameter.m().parametersSet |= VEHPARS_FORCE_REROUTE;
            if self.myVehicleParameter.stops.len() > 0 {
                route = self.add_vehicle_stops_to_implicit_route(route, true);
                self.myVehicleParameter.m().routeid = route.get_id().to_string();
            }
        }
        if self.myVehicleParameter.departEdgeProcedure != RouteIndexDefinition::DEFAULT {
            if (self.myVehicleParameter.parametersSet & VEHPARS_FORCE_REROUTE) == 0
                && self.myVehicleParameter.departEdgeProcedure == RouteIndexDefinition::GIVEN
                && self.myVehicleParameter.departEdge >= route.get_edges().len() as i32
            {
                throw_process_error(format!(
                    "Flow '{}' has invalid departEdge index {} for route with {} edges.",
                    self.myVehicleParameter.id,
                    self.myVehicleParameter.departEdge,
                    route.get_edges().len()
                ));
            }
        }
        if self.myVehicleParameter.arrivalEdgeProcedure != RouteIndexDefinition::DEFAULT {
            if (self.myVehicleParameter.parametersSet & VEHPARS_FORCE_REROUTE) == 0
                && self.myVehicleParameter.arrivalEdgeProcedure == RouteIndexDefinition::GIVEN
                && self.myVehicleParameter.arrivalEdge >= route.get_edges().len() as i32
            {
                throw_process_error(format!(
                    "Flow '{}' has invalid arrivalEdge index {} for route with {} edges.",
                    self.myVehicleParameter.id,
                    self.myVehicleParameter.arrivalEdge,
                    route.get_edges().len()
                ));
            }
        }
        self.myActiveRouteID = String::new();

        // check whether the vehicle shall be added directly to the network or
        //  shall stay in the internal buffer
        if self.myAddVehiclesDirectly || self.check_last_depart() {
            if net.m().get_insertion_control().add_flow(self.myVehicleParameter, -1) {
                self.register_last_depart();
            } else if state_loaded() {
                delete_ptr(self.myVehicleParameter);
            } else {
                throw_process_error(format!("Another flow with the id '{}' exists.", self.myVehicleParameter.id));
            }
        }
        self.myVehicleParameter = Ptr::null();
    }

    /// C++: `void MSRouteHandler::closeTrip()` [virtual, protected]
    /// Ends the processing of a trip
    // void
    // MSRouteHandler::closeTrip() {
    //     myVehicleParameter->parametersSet |= VEHPARS_FORCE_REROUTE;
    //     closeRoute(true);
    //     closeVehicle();
    // }
    #[allow(unused_variables)]
    pub fn close_trip(&mut self) {
        self.myVehicleParameter.m().parametersSet |= VEHPARS_FORCE_REROUTE;
        self.close_route(true);
        self.close_vehicle();
    }

    /// C++: `MSStoppingPlace * MSRouteHandler::retrieveStoppingPlace(const SUMOSAXAttributes &, const std::string &, SUMOVehicleParameter::Stop *)` [protected]
    /// Parse destination stop
    // MSStoppingPlace*
    // MSRouteHandler::retrieveStoppingPlace(const SUMOSAXAttributes& attrs, const std::string& errorSuffix, SUMOVehicleParameter::Stop* stopParam) {
    //     MSStoppingPlace* toStop = nullptr;
    //     SumoXMLTag tag = SUMO_TAG_NOTHING;
    //     std::string id;
    //     bool ok = SUMOVehicleParserHelper::parseStopTag(attrs, stopParam, tag, id);
    //     if (tag != SUMO_TAG_NOTHING) {
    //         toStop = MSNet::getInstance()->getStoppingPlace(id, tag);
    //         if (toStop == nullptr) {
    //             ok = false;
    //             WRITE_ERROR(TLF("The % '%' is not known%.", toString(tag), id, errorSuffix));
    //         }
    //     }
    //     if (!ok && check_routes()) {
    //         throw ProcessError(TLF("Invalid stop definition%.", errorSuffix));
    //     }
    //     return toStop;
    // }
    #[allow(unused_variables)]
    pub fn retrieve_stopping_place(&mut self, attrs: &SUMOSAXAttributes, errorSuffix: &str, stopParam: Ptr<SUMOVehicleParameter_Stop>) -> Ptr<MSStoppingPlace> {
        let mut toStop: Ptr<MSStoppingPlace> = Ptr::null();
        let mut tag: SumoXMLTag = SUMO_TAG_NOTHING;
        let mut id = String::new();
        let mut ok = SUMOVehicleParserHelper::parse_stop_tag(attrs, stopParam, &mut tag, &mut id);
        if tag != SUMO_TAG_NOTHING {
            let net = MSNet::get_instance();
            toStop = net.m().get_stopping_place(&id, tag);
            if toStop.is_null() {
                ok = false;
                write_errorf!("The % '%' is not known%.", tag_to_string(tag), id, errorSuffix);
            }
        }
        if !ok && check_routes() {
            throw_process_error(format!("Invalid stop definition{}.", errorSuffix));
        }
        toStop
    }

    /// C++: `Parameterised * MSRouteHandler::addStop(const SUMOSAXAttributes &)` [virtual, protected]
    /// Processing of a stop
    // Parameterised*
    // MSRouteHandler::addStop(const SUMOSAXAttributes& attrs) {
    //     Parameterised* result = nullptr;
    //     try {
    //         std::string errorSuffix;
    //         if (myActiveType == ObjectTypeEnum::PERSON) {
    //             errorSuffix = " in person '" + myVehicleParameter->id + "'";
    //         } else if (myActiveType == ObjectTypeEnum::CONTAINER) {
    //             errorSuffix = " in container '" + myVehicleParameter->id + "'";
    //         } else if (myVehicleParameter != nullptr) {
    //             errorSuffix = " in vehicle '" + myVehicleParameter->id + "'";
    //         } else {
    //             errorSuffix = " in route '" + myActiveRouteID + "'";
    //         }
    //         SUMOVehicleParameter::Stop stop;
    //         bool ok = parseStop(stop, attrs, errorSuffix, MsgHandler::getErrorInstance());
    //         if (!ok) {
    //             if (check_routes()) {
    //                 throw ProcessError();
    //             }
    //             return result;
    //         }
    //         const MSEdge* edge = nullptr;
    //         // patch chargingStation stop on a parkingArea
    //         if (stop.chargingStation != "") {
    //             const MSChargingStation* cs = dynamic_cast<MSChargingStation*>(MSNet::getInstance()->getStoppingPlace(stop.chargingStation, SUMO_TAG_CHARGING_STATION));
    //             const MSParkingArea* pa = cs->getParkingArea();
    //             if (pa != nullptr) {
    //                 stop.parkingarea = pa->getID();
    //                 stop.parking = ParkingType::OFFROAD;
    //             }
    //         }
    //         MSStoppingPlace* toStop = retrieveStoppingPlace(attrs, errorSuffix, &stop);
    //         // if one of the previous stops is defined
    //         if (toStop != nullptr) {
    //             const MSLane& l = toStop->getLane();
    //             stop.lane = l.getID();
    //             if ((stop.parametersSet & STOP_END_SET) == 0) {
    //                 stop.endPos = toStop->getEndLanePosition();
    //             } else {
    //                 stop.endPos = attrs.get<double>(SUMO_ATTR_ENDPOS, nullptr, ok);
    //             }
    //             stop.startPos = toStop->getBeginLanePosition();
    //             edge = &l.getEdge();
    //         } else {
    //             // no, the lane and the position should be given directly
    //             // get the lane
    //             stop.lane = attrs.getOpt<std::string>(SUMO_ATTR_LANE, nullptr, ok, "");
    //             stop.edge = attrs.getOpt<std::string>(SUMO_ATTR_EDGE, nullptr, ok, "");
    //             if (ok && stop.edge != "") { // edge is given directly
    //                 edge = MSEdge::dictionary(stop.edge);
    //                 if (edge == nullptr || (edge->isInternal() && !using_internal_lanes())) {
    //                     throw ProcessError(TLF("The edge '%' for a stop is not known%.", stop.edge, errorSuffix));
    //                 }
    //             } else if (ok && stop.lane != "") { // lane is given directly
    //                 MSLane* stopLane = MSLane::dictionary(stop.lane);
    //                 if (stopLane == nullptr) {
    //                     // check for opposite-direction stop
    //                     stopLane = MSBaseVehicle::interpretOppositeStop(stop);
    //                     if (stopLane != nullptr) {
    //                         edge = MSEdge::dictionary(stop.edge);
    //                     }
    //                 } else {
    //                     edge = &stopLane->getEdge();
    //                 }
    //                 if (stopLane == nullptr || (stopLane->isInternal() && !using_internal_lanes())) {
    //                     throw ProcessError(TLF("The lane '%' for a stop is not known%.", stop.lane, errorSuffix));
    //                 }
    //             } else if (ok && ((attrs.hasAttribute(SUMO_ATTR_X) && attrs.hasAttribute(SUMO_ATTR_Y))
    //                               || (attrs.hasAttribute(SUMO_ATTR_LON) && attrs.hasAttribute(SUMO_ATTR_LAT)))) {
    //                 Position pos;
    //                 bool geo = false;
    //                 if (attrs.hasAttribute(SUMO_ATTR_X) && attrs.hasAttribute(SUMO_ATTR_Y)) {
    //                     pos = Position(attrs.get<double>(SUMO_ATTR_X, myVehicleParameter->id.c_str(), ok), attrs.get<double>(SUMO_ATTR_Y, myVehicleParameter->id.c_str(), ok));
    //                 } else {
    //                     pos = Position(attrs.get<double>(SUMO_ATTR_LON, myVehicleParameter->id.c_str(), ok), attrs.get<double>(SUMO_ATTR_LAT, myVehicleParameter->id.c_str(), ok));
    //                     geo = true;
    //                 }
    //                 PositionVector positions;
    //                 positions.push_back(pos);
    //                 ConstMSEdgeVector geoEdges;
    //                 SUMOVehicleClass vClass = SVC_PASSENGER;
    //                 auto& vc = MSNet::getInstance()->getVehicleControl();
    //                 if (!vc.getVTypeDistribution(myVehicleParameter->vtypeid)) {
    //                     MSVehicleType* const type = vc.getVType(myVehicleParameter->vtypeid, &myParsingRNG);
    //                     if (type != nullptr) {
    //                         vClass = type->getParameter().vehicleClass;
    //                     }
    //                 }
    //                 parseGeoEdges(positions, geo, vClass, geoEdges, myVehicleParameter->id, true, ok, true);
    //                 if (ok) {
    //                     edge = geoEdges.front();
    //                     if (geo) {
    //                         GeoConvHelper::getFinal().x2cartesian_const(pos);
    //                     }
    //                     stop.parametersSet |= STOP_END_SET;
    //                     stop.endPos = edge->getLanes()[0]->getShape().nearest_offset_to_point2D(pos, false);
    //                 } else {
    //                     throw ProcessError(TLF("Could not map stop position '%' to the network%.", pos, errorSuffix));
    //                 }
    //             } else {
    //                 if (myActiveTransportablePlan && !myActiveTransportablePlan->empty()) { // use end of movement before
    //                     toStop = myActiveTransportablePlan->back()->getDestinationStop();
    //                     if (toStop != nullptr) { // use end of movement before definied as a stopping place
    //                         edge = &toStop->getLane().getEdge();
    //                         stop.lane = toStop->getLane().getID();
    //                         stop.endPos = toStop->getEndLanePosition();
    //                         stop.startPos = toStop->getBeginLanePosition();
    //                     } else { // use end of movement before definied as lane/edge
    //                         edge = myActiveTransportablePlan->back()->getDestination();
    //                         stop.lane = edge->getLanes()[0]->getID();
    //                         stop.endPos = myActiveTransportablePlan->back()->unspecifiedArrivalPos() ?
    //                                       MSStage::ARRIVALPOS_UNSPECIFIED : myActiveTransportablePlan->back()->getArrivalPos();
    //                         stop.startPos = MAX2(0., stop.endPos - MIN_STOP_LENGTH);
    //                     }
    //                 } else {
    //                     const std::string msg = TLF("A stop must be placed on a busStop, a chargingStation, an overheadWireSegment, a containerStop, a parkingArea, an edge or a lane%.", errorSuffix);
    //                     if (check_routes()) {
    //                         throw ProcessError(msg);
    //                     } else {
    //                         WRITE_WARNING(msg);
    //                         return result;
    //                     }
    //                 }
    //             }
    //             stop.endPos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, nullptr, ok, edge->getLength());
    //             if (attrs.hasAttribute(SUMO_ATTR_POSITION)) {
    //                 WRITE_WARNINGF(TL("Deprecated attribute 'pos' in description of stop%."), errorSuffix);
    //                 stop.endPos = attrs.getOpt<double>(SUMO_ATTR_POSITION, nullptr, ok, stop.endPos);
    //             }
    //             if (stop.endPos < 0) {
    //                 stop.endPos += edge->getLength();
    //             }
    //             stop.startPos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, nullptr, ok, MAX2(0., stop.endPos - MIN_STOP_LENGTH));
    //             if (!myAmLoadingState) {
    //                 const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, nullptr, ok, !attrs.hasAttribute(SUMO_ATTR_STARTPOS) && !attrs.hasAttribute(SUMO_ATTR_ENDPOS))
    //                                          || !check_routes();
    //                 if (!ok || (checkStopPos(stop.startPos, stop.endPos, edge->getLength(), 0, friendlyPos) != StopPos::STOPPOS_VALID)) {
    //                     throw ProcessError(TLF("Invalid start or end position for stop on %'%.",
    //                                            stop.lane != "" ? ("lane '" + stop.lane) : ("edge '" + stop.edge), errorSuffix));
    //                 }
    //             }
    //         }
    //         stop.edge = edge->getID();
    //         if (myActiveTransportablePlan) {
    //             if (myActiveTransportablePlan->empty()) {
    //                 double departPos = toStop == nullptr || myVehicleParameter->wasSet(VEHPARS_DEPARTPOS_SET)
    //                                    ? myVehicleParameter->departPos
    //                                    : (toStop->getBeginLanePosition() + toStop->getEndLanePosition()) / 2;
    //                 myActiveTransportablePlan->push_back(new MSStageWaiting(
    //                         edge, toStop, -1, myVehicleParameter->depart, departPos, "start", true));
    //             } else if (myActiveTransportablePlan->back()->getDestination() != edge) {
    //                 if (myActiveTransportablePlan->back()->getDestination()->isTazConnector()) {
    //                     myActiveTransportablePlan->back()->setDestination(edge, toStop);
    //                 } else if (myActiveTransportablePlan->back()->getJumpDuration() < 0) {
    //                     throw ProcessError(TLF("Disconnected plan for % '%' (%!=%).", myActiveTypeName, myVehicleParameter->id,
    //                                            edge->getID(), myActiveTransportablePlan->back()->getDestination()->getID()));
    //                 }
    //             }
    //             // transporting veh stops somewhere
    //             else if (myActiveTransportablePlan->back()->getStageType() == MSStageType::WAITING
    //                      && (attrs.hasAttribute(SUMO_ATTR_ENDPOS) || attrs.hasAttribute(SUMO_ATTR_STARTPOS))) {
    //                 const double start = SUMOVehicleParameter::interpretEdgePos(stop.startPos, edge->getLength(), SUMO_ATTR_STARTPOS, "stopping at " + edge->getID());
    //                 const double end = SUMOVehicleParameter::interpretEdgePos(stop.endPos, edge->getLength(), SUMO_ATTR_ENDPOS, "stopping at " + edge->getID());
    //                 const double prevAr = myActiveTransportablePlan->back()->getArrivalPos();
    //                 if (start > prevAr + NUMERICAL_EPS || end < prevAr - NUMERICAL_EPS) {
    //                     WRITE_WARNINGF(TL("Disconnected plan for % '%' (stop range %-% does not cover previous arrival position %)."),
    //                                    myActiveTypeName, myVehicleParameter->id, toString(start), toString(end), toString(prevAr));
    //                 }
    //             }
    //             std::string actType = attrs.getOpt<std::string>(SUMO_ATTR_ACTTYPE, nullptr, ok, "");
    //             double pos = (stop.startPos + stop.endPos) / 2.;
    //             if (!myActiveTransportablePlan->empty() && myActiveTransportablePlan->back()->getJumpDuration() < 0) {
    //                 pos = myActiveTransportablePlan->back()->unspecifiedArrivalPos() ?
    //                       MSStage::ARRIVALPOS_UNSPECIFIED : myActiveTransportablePlan->back()->getArrivalPos();
    //             }
    //             myActiveTransportablePlan->push_back(new MSStageWaiting(edge, toStop, stop.duration, stop.until, pos, actType, false, stop.jump));
    //             result = myActiveTransportablePlan->back();
    //
    //         } else if (myVehicleParameter != nullptr) {
    //             myVehicleParameter->stops.push_back(stop);
    //             result = &myVehicleParameter->stops.back();
    //         } else {
    //             myActiveRouteStops.push_back(stop);
    //             result = &myActiveRouteStops.back();
    //         }
    //         if (myInsertStopEdgesAt >= 0) {
    //             //std::cout << " myInsertStopEdgesAt=" << myInsertStopEdgesAt << " edge=" << edge->getID() << " myRoute=" << toString(myActiveRoute) << "\n";
    //             if (edge->isInternal()) {
    //                 if (myInsertStopEdgesAt > 0 && *(myActiveRoute.begin() + (myInsertStopEdgesAt - 1)) != edge->getNormalBefore()) {
    //                     myActiveRoute.insert(myActiveRoute.begin() + myInsertStopEdgesAt, edge->getNormalBefore());
    //                     myInsertStopEdgesAt++;
    //                 }
    //                 myActiveRoute.insert(myActiveRoute.begin() + myInsertStopEdgesAt, edge->getNormalSuccessor());
    //                 myInsertStopEdgesAt++;
    //             } else {
    //                 myActiveRoute.insert(myActiveRoute.begin() + myInsertStopEdgesAt, edge);
    //                 myInsertStopEdgesAt++;
    //             }
    //         } else if (myHaveVia) {
    //             // vias were loaded, check for consistency
    //             if (std::find(myActiveRoute.begin(), myActiveRoute.end(), edge) == myActiveRoute.end()) {
    //                 WRITE_WARNINGF(TL("Stop edge '%' missing in attribute 'via' for % '%'."),
    //                                edge->getID(), myActiveTypeName, myVehicleParameter->id);
    //             }
    //         }
    //     } catch (ProcessError&) {
    //         deleteActivePlanAndVehicleParameter();
    //         throw;
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn add_stop(&mut self, attrs: &SUMOSAXAttributes) -> Ptr<Parameterised> {
        // C++ try/catch(ProcessError&): deleteActivePlanAndVehicleParameter, then rethrow
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Ptr<Parameterised> {
            let mut result: Ptr<Parameterised> = Ptr::null();
            let errorSuffix: String = if self.myActiveType == MSRouteHandler_ObjectTypeEnum::PERSON {
                format!(" in person '{}'", self.myVehicleParameter.id)
            } else if self.myActiveType == MSRouteHandler_ObjectTypeEnum::CONTAINER {
                format!(" in container '{}'", self.myVehicleParameter.id)
            } else if !self.myVehicleParameter.is_null() {
                format!(" in vehicle '{}'", self.myVehicleParameter.id)
            } else {
                format!(" in route '{}'", self.myActiveRouteID)
            };
            // shortcut: SUMOVehicleParameter::Stop default ctor is not ported, upgrade when SUMOVehicleParameter_Stop gets a new()
            let mut stop: SUMOVehicleParameter_Stop = panic!("SUMOVehicleParameter::Stop() ctor not ported");
            let ok = self.parse_stop(&mut stop, attrs, errorSuffix.clone(), Ptr::null());
            // shortcut: MsgHandler::getErrorInstance() has no accessor yet, parseStop reports to a null handler until it does
            if !ok {
                if check_routes() {
                    throw_process_error_default();
                }
                return result;
            }
            let mut edge: Ptr<MSEdge> = Ptr::null();
            // patch chargingStation stop on a parkingArea
            if !stop.chargingStation.is_empty() {
                // shortcut: dynamic_cast<MSChargingStation*> needs a downcast from MSStoppingPlace, the parkingArea patch is skipped until it exists
            }
            let mut toStop = self.retrieve_stopping_place(attrs, &errorSuffix, Ptr::from_mut(&mut stop));
            let mut ok = true;
            // if one of the previous stops is defined
            if !toStop.is_null() {
                let l = toStop.get_lane();
                stop.lane = l.get_id().to_string();
                if (stop.parametersSet & STOP_END_SET) == 0 {
                    stop.endPos = toStop.get_end_lane_position();
                } else {
                    stop.endPos = attrs.get(SUMO_ATTR_ENDPOS, "", &mut ok, true);
                }
                stop.startPos = toStop.get_begin_lane_position();
                edge = Ptr::from_mut(l.get_edge());
            } else {
                // no, the lane and the position should be given directly
                // get the lane
                stop.lane = attrs.get_opt(SUMO_ATTR_LANE, "", &mut ok, String::new(), true);
                stop.edge = attrs.get_opt(SUMO_ATTR_EDGE, "", &mut ok, String::new(), true);
                if ok && stop.edge != "" {
                    // edge is given directly
                    edge = MSEdge::dictionary_2(&stop.edge);
                    if edge.is_null() || (edge.is_internal() && !using_internal_lanes()) {
                        throw_process_error(format!("The edge '{}' for a stop is not known{}.", stop.edge, errorSuffix));
                    }
                } else if ok && stop.lane != "" {
                    // lane is given directly
                    let mut stopLane = MSLane::dictionary_2(&stop.lane);
                    if stopLane.is_null() {
                        // check for opposite-direction stop
                        stopLane = MSBaseVehicle::interpret_opposite_stop(&mut stop);
                        if !stopLane.is_null() {
                            edge = MSEdge::dictionary_2(&stop.edge);
                        }
                    } else {
                        edge = Ptr::from_mut(stopLane.m().get_edge());
                    }
                    if stopLane.is_null() || (stopLane.is_internal() && !using_internal_lanes()) {
                        throw_process_error(format!("The lane '{}' for a stop is not known{}.", stop.lane, errorSuffix));
                    }
                } else if ok && ((attrs.has_attribute(SUMO_ATTR_X) && attrs.has_attribute(SUMO_ATTR_Y)) || (attrs.has_attribute(SUMO_ATTR_LON) && attrs.has_attribute(SUMO_ATTR_LAT))) {
                    let mut pos: Position;
                    let mut geo = false;
                    if attrs.has_attribute(SUMO_ATTR_X) && attrs.has_attribute(SUMO_ATTR_Y) {
                        let x: f64 = attrs.get(SUMO_ATTR_X, &self.myVehicleParameter.id, &mut ok, true);
                        let y: f64 = attrs.get(SUMO_ATTR_Y, &self.myVehicleParameter.id, &mut ok, true);
                        pos = Position::new(x, y);
                    } else {
                        let lon: f64 = attrs.get(SUMO_ATTR_LON, &self.myVehicleParameter.id, &mut ok, true);
                        let lat: f64 = attrs.get(SUMO_ATTR_LAT, &self.myVehicleParameter.id, &mut ok, true);
                        pos = Position::new(lon, lat);
                        geo = true;
                    }
                    let positions = PositionVector(vec![pos]);
                    let mut geoEdges: ConstMSEdgeVector = Vec::new();
                    let mut vClass = SVC_PASSENGER;
                    let net = MSNet::get_instance();
                    let vc = net.m().get_vehicle_control();
                    if vc.get_v_type_distribution(&self.myVehicleParameter.vtypeid).is_null() {
                        let type_ = vc.get_v_type(&self.myVehicleParameter.vtypeid, SUMORouteHandler::get_parsing_rng(), false);
                        if !type_.is_null() {
                            vClass = type_.get_parameter().vehicleClass;
                        }
                    }
                    parse_geo_edges(positions, geo, vClass, &mut geoEdges, &self.myVehicleParameter.id, true, &mut ok);
                    if ok {
                        edge = geoEdges[0];
                        if geo {
                            utils::geom::geo_conv_helper::GeoConvHelper::with_final(|g| g.x2cartesian_const(&mut pos));
                        }
                        stop.parametersSet |= STOP_END_SET;
                        stop.endPos = edge.get_lanes()[0].get_shape(false).nearest_offset_to_point2d(&pos, false);
                    } else {
                        throw_process_error(format!("Could not map stop position '{}' to the network{}.", pos, errorSuffix));
                    }
                } else {
                    if !self.myActiveTransportablePlan.is_null() && !self.myActiveTransportablePlan.is_empty() {
                        // use end of movement before
                        toStop = self.myActiveTransportablePlan.last().unwrap().get_destination_stop();
                        if !toStop.is_null() {
                            // use end of movement before definied as a stopping place
                            edge = Ptr::from_mut(toStop.get_lane().get_edge());
                            stop.lane = toStop.get_lane().get_id().to_string();
                            stop.endPos = toStop.get_end_lane_position();
                            stop.startPos = toStop.get_begin_lane_position();
                        } else {
                            // use end of movement before definied as lane/edge
                            edge = self.myActiveTransportablePlan.last().unwrap().get_destination();
                            stop.lane = edge.get_lanes()[0].get_id().to_string();
                            stop.endPos = if self.myActiveTransportablePlan.last().unwrap().unspecified_arrival_pos() {
                                ARRIVALPOS_UNSPECIFIED
                            } else {
                                self.myActiveTransportablePlan.last().unwrap().get_arrival_pos()
                            };
                            stop.startPos = f64::max(0.0, stop.endPos - MIN_STOP_LENGTH);
                        }
                    } else {
                        let msg = format!(
                            "A stop must be placed on a busStop, a chargingStation, an overheadWireSegment, a containerStop, a parkingArea, an edge or a lane{}.",
                            errorSuffix
                        );
                        if check_routes() {
                            throw_process_error(msg);
                        } else {
                            utils::common::msg_handler::write_warning(&msg);
                            return result;
                        }
                    }
                }
                stop.endPos = attrs.get_opt(SUMO_ATTR_ENDPOS, "", &mut ok, edge.get_length(), true);
                if attrs.has_attribute(SUMO_ATTR_POSITION) {
                    write_warningf!("Deprecated attribute 'pos' in description of stop%.", errorSuffix);
                    stop.endPos = attrs.get_opt(SUMO_ATTR_POSITION, "", &mut ok, stop.endPos, true);
                }
                if stop.endPos < 0.0 {
                    stop.endPos += edge.get_length();
                }
                stop.startPos = attrs.get_opt(SUMO_ATTR_STARTPOS, "", &mut ok, f64::max(0.0, stop.endPos - MIN_STOP_LENGTH), true);
                if !self.myAmLoadingState {
                    let friendlyPos = attrs.get_opt(
                        SUMO_ATTR_FRIENDLY_POS,
                        "",
                        &mut ok,
                        !attrs.has_attribute(SUMO_ATTR_STARTPOS) && !attrs.has_attribute(SUMO_ATTR_ENDPOS) || !check_routes(),
                        true,
                    );
                    if !ok || SUMORouteHandler::check_stop_pos(&mut stop.startPos, &mut stop.endPos, edge.get_length(), 0.0, friendlyPos) != SUMORouteHandler_StopPos::STOPPOS_VALID {
                        let what = if stop.lane != "" { format!("lane '{}", stop.lane) } else { format!("edge '{}", stop.edge) };
                        throw_process_error(format!("Invalid start or end position for stop on {}'{}.", what, errorSuffix));
                    }
                }
            }
            stop.edge = edge.get_id().to_string();
            if !self.myActiveTransportablePlan.is_null() {
                if self.myActiveTransportablePlan.is_empty() {
                    let departPos = if toStop.is_null() || self.myVehicleParameter.was_set(VEHPARS_DEPARTPOS_SET) {
                        self.myVehicleParameter.departPos
                    } else {
                        (toStop.get_begin_lane_position() + toStop.get_end_lane_position()) / 2.0
                    };
                    let stage = MSStageWaiting::new(edge, toStop, -1, self.myVehicleParameter.depart, departPos, "start", true, -1);
                    self.myActiveTransportablePlan.m().push(stage_waiting_ptr(stage));
                } else if self.myActiveTransportablePlan.last().unwrap().get_destination() != edge {
                    if self.myActiveTransportablePlan.last().unwrap().get_destination().is_taz_connector() {
                        self.myActiveTransportablePlan.last().unwrap().m().set_destination(edge, toStop);
                    } else if self.myActiveTransportablePlan.last().unwrap().get_jump_duration() < 0 {
                        throw_process_error(format!(
                            "Disconnected plan for {} '{}' ({}!={}).",
                            self.myActiveTypeName,
                            self.myVehicleParameter.id,
                            edge.get_id(),
                            self.myActiveTransportablePlan.last().unwrap().get_destination().get_id()
                        ));
                    }
                }
                // transporting veh stops somewhere
                else if self.myActiveTransportablePlan.last().unwrap().get_stage_type() == MSStageType::WAITING
                    && (attrs.has_attribute(SUMO_ATTR_ENDPOS) || attrs.has_attribute(SUMO_ATTR_STARTPOS))
                {
                    let edgeLength = edge.get_length();
                    let start = SUMOVehicleParameter::interpret_edge_pos(stop.startPos, edgeLength, SUMO_ATTR_STARTPOS, &format!("stopping at {}", edge.get_id()), false);
                    let end = SUMOVehicleParameter::interpret_edge_pos(stop.endPos, edgeLength, SUMO_ATTR_ENDPOS, &format!("stopping at {}", edge.get_id()), false);
                    let prevAr = self.myActiveTransportablePlan.last().unwrap().get_arrival_pos();
                    if start > prevAr + utils::geom::NUMERICAL_EPS || end < prevAr - utils::geom::NUMERICAL_EPS {
                        write_warningf!(
                            "Disconnected plan for % '%' (stop range %-% does not cover previous arrival position %).",
                            self.myActiveTypeName,
                            self.myVehicleParameter.id,
                            start,
                            end,
                            prevAr
                        );
                    }
                }
                let actType: String = attrs.get_opt(SUMO_ATTR_ACTTYPE, "", &mut ok, String::new(), true);
                let mut pos = (stop.startPos + stop.endPos) / 2.0;
                if !self.myActiveTransportablePlan.is_empty() && self.myActiveTransportablePlan.last().unwrap().get_jump_duration() < 0 {
                    pos = if self.myActiveTransportablePlan.last().unwrap().unspecified_arrival_pos() {
                        ARRIVALPOS_UNSPECIFIED
                    } else {
                        self.myActiveTransportablePlan.last().unwrap().get_arrival_pos()
                    };
                }
                let stage = MSStageWaiting::new(edge, toStop, stop.duration, stop.until, pos, &actType, false, stop.jump);
                self.myActiveTransportablePlan.m().push(stage_waiting_ptr(stage));
                result = stage_as_parameterised(*self.myActiveTransportablePlan.last().unwrap());
            } else if !self.myVehicleParameter.is_null() {
                self.myVehicleParameter.m().stops.push(stop);
                result = Ptr::from_mut(&mut self.myVehicleParameter.m().stops.last_mut().unwrap().base);
            } else {
                self.myActiveRouteStops.push(stop);
                result = Ptr::from_mut(&mut self.myActiveRouteStops.last_mut().unwrap().base);
            }
            if self.myInsertStopEdgesAt >= 0 {
                if edge.is_internal() {
                    if self.myInsertStopEdgesAt > 0 && self.myActiveRoute[(self.myInsertStopEdgesAt - 1) as usize] != edge.get_normal_before() {
                        self.myActiveRoute.insert(self.myInsertStopEdgesAt as usize, edge.get_normal_before());
                        self.myInsertStopEdgesAt += 1;
                    }
                    self.myActiveRoute.insert(self.myInsertStopEdgesAt as usize, edge.get_normal_successor());
                    self.myInsertStopEdgesAt += 1;
                } else {
                    self.myActiveRoute.insert(self.myInsertStopEdgesAt as usize, edge);
                    self.myInsertStopEdgesAt += 1;
                }
            } else if self.myHaveVia {
                // vias were loaded, check for consistency
                if !self.myActiveRoute.contains(&edge) {
                    write_warningf!(
                        "Stop edge '%' missing in attribute 'via' for % '%'.",
                        edge.get_id(),
                        self.myActiveTypeName,
                        self.myVehicleParameter.id
                    );
                }
            }
            result
        }));
        match result {
            Ok(r) => r,
            Err(payload) => {
                self.delete_active_plan_and_vehicle_parameter();
                std::panic::resume_unwind(payload);
            }
        }
    }

    /// C++: `void MSRouteHandler::addPersonTrip(const SUMOSAXAttributes &)` [virtual, protected]
    /// add a routing request for a walking or intermodal person
    // void
    // MSRouteHandler::addPersonTrip(const SUMOSAXAttributes& attrs) {
    //     if (myVehicleParameter == nullptr) {
    //         throw ProcessError(TL("Cannot define person stage without person."));
    //     }
    //     try {
    //         myActiveRoute.clear();
    //         bool ok = true;
    //         const char* const id = myVehicleParameter->id.c_str();
    //         const MSEdge* from = nullptr;
    //         const MSEdge* to = nullptr;
    //         parseFromViaTo(SUMO_TAG_PERSON, attrs);
    //         myInsertStopEdgesAt = -1;
    //         if (attrs.hasAttribute(SUMO_ATTR_FROM) || attrs.hasAttribute(SUMO_ATTR_FROM_JUNCTION) || attrs.hasAttribute(SUMO_ATTR_FROM_TAZ)
    //                 || attrs.hasAttribute(SUMO_ATTR_FROMXY) || attrs.hasAttribute(SUMO_ATTR_FROMLONLAT)) {
    //             from = myActiveRoute.front();
    //         } else if (myActiveTransportablePlan->empty()) {
    //             throw ProcessError(TLF("Start edge not defined for person '%'.", myVehicleParameter->id));
    //         } else {
    //             from = myActiveTransportablePlan->back()->getDestination();
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_TO) || attrs.hasAttribute(SUMO_ATTR_TO_JUNCTION) || attrs.hasAttribute(SUMO_ATTR_TO_TAZ)
    //                 || attrs.hasAttribute(SUMO_ATTR_TOXY) || attrs.hasAttribute(SUMO_ATTR_TOLONLAT)) {
    //             to = myActiveRoute.back();
    //         } // else, to may also be derived from stopping place
    //
    //         const SUMOTime duration = attrs.getOptSUMOTimeReporting(SUMO_ATTR_DURATION, id, ok, -1);
    //         if (attrs.hasAttribute(SUMO_ATTR_DURATION) && duration <= 0) {
    //             throw ProcessError(TLF("Non-positive walking duration for '%'.", myVehicleParameter->id));
    //         }
    //
    //         double departPos = 0;
    //         double arrivalPos = 0;
    //         MSStoppingPlace* stoppingPlace = nullptr;
    //         parseWalkPositions(attrs, myVehicleParameter->id, from, to, departPos, arrivalPos, stoppingPlace, nullptr, ok);
    //
    //         SVCPermissions modeSet = 0;
    //         if (attrs.hasAttribute(SUMO_ATTR_MODES)) {
    //             const std::string modes = attrs.getOpt<std::string>(SUMO_ATTR_MODES, id, ok, "");
    //             std::string errorMsg;
    //             // try to parse person modes
    //             if (!SUMOVehicleParameter::parsePersonModes(modes, "person", id, modeSet, errorMsg)) {
    //                 throw InvalidArgument(errorMsg);
    //             }
    //         } else {
    //             modeSet = myVehicleParameter->modes;
    //         }
    //         const std::string group = attrs.getOpt<std::string>(SUMO_ATTR_GROUP, id, ok, OptionsCont::getOptions().getString("persontrip.default.group"));
    //         MSVehicleControl& vehControl = MSNet::getInstance()->getVehicleControl();
    //         const std::string types = attrs.getOpt<std::string>(SUMO_ATTR_VTYPES, id, ok, myVehicleParameter->vTypes);
    //         for (StringTokenizer st(types); st.hasNext();) {
    //             const std::string vtypeid = st.next();
    //             const MSVehicleType* const vType = vehControl.getVType(vtypeid);
    //             if (vType == nullptr) {
    //                 throw InvalidArgument("The vehicle type '" + vtypeid + "' in a trip for person '" + myVehicleParameter->id + "' is not known.");
    //             }
    //             modeSet |= (vType->getVehicleClass() == SVC_BICYCLE) ? SVC_BICYCLE : SVC_PASSENGER;
    //         }
    //         const double speed = attrs.getOpt<double>(SUMO_ATTR_SPEED, id, ok, -1.);
    //         if (attrs.hasAttribute(SUMO_ATTR_SPEED) && speed <= 0) {
    //             throw ProcessError(TLF("Non-positive walking speed for '%'.", myVehicleParameter->id));
    //         }
    //         const double walkFactor = attrs.getOpt<double>(SUMO_ATTR_WALKFACTOR, id, ok, OptionsCont::getOptions().getFloat("persontrip.walkfactor"));
    //         const double departPosLat = interpretDepartPosLat(attrs.getOpt<std::string>(SUMO_ATTR_DEPARTPOS_LAT, nullptr, ok, ""), -1, "personTrip");
    //         if (ok) {
    //             if (myActiveTransportablePlan->empty()) {
    //                 double initialDepartPos = myVehicleParameter->departPos;
    //                 if (myVehicleParameter->departPosProcedure == DepartPosDefinition::RANDOM) {
    //                     initialDepartPos = RandHelper::rand(from->getLength(), &myParsingRNG);
    //                 }
    //                 myActiveTransportablePlan->push_back(new MSStageWaiting(from, nullptr, -1, myVehicleParameter->depart, initialDepartPos, "start", true));
    //             }
    //             myVehicleParameter->parametersSet |= VEHPARS_FORCE_REROUTE;
    //             MSStoppingPlace* fromStop = myActiveTransportablePlan->empty() ? nullptr : myActiveTransportablePlan->back()->getDestinationStop();
    //             myActiveTransportablePlan->push_back(new MSStageTrip(from, fromStop, to == nullptr ? &stoppingPlace->getLane().getEdge() : to,
    //                                                  stoppingPlace, duration, modeSet, types, speed, walkFactor, group,
    //                                                  departPosLat, attrs.hasAttribute(SUMO_ATTR_ARRIVALPOS), arrivalPos));
    //             myParamStack.push_back(myActiveTransportablePlan->back());
    //             if (attrs.hasAttribute(SUMO_ATTR_ARRIVALPOS)) {
    //                 myActiveTransportablePlan->back()->markSet(VEHPARS_ARRIVALPOS_SET);
    //             }
    //         }
    //         myActiveRoute.clear();
    //     } catch (ProcessError&) {
    //         deleteActivePlanAndVehicleParameter();
    //         throw;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_person_trip(&mut self, attrs: &SUMOSAXAttributes) {
        if self.myVehicleParameter.is_null() {
            std::panic::panic_any(ProcessError::new("Cannot define person stage without person."));
        }
        // C++ try/catch(ProcessError&): deleteActivePlanAndVehicleParameter, then rethrow
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.myActiveRoute.clear();
            let mut ok = true;
            let id = self.myVehicleParameter.id.clone();
            let mut from: Ptr<MSEdge> = Ptr::null();
            let mut to: Ptr<MSEdge> = Ptr::null();
            self.parse_from_via_to(SUMO_TAG_PERSON, attrs);
            self.myInsertStopEdgesAt = -1;
            if attrs.has_attribute(SUMO_ATTR_FROM)
                || attrs.has_attribute(SUMO_ATTR_FROM_JUNCTION)
                || attrs.has_attribute(SUMO_ATTR_FROM_TAZ)
                || attrs.has_attribute(SUMO_ATTR_FROMXY)
                || attrs.has_attribute(SUMO_ATTR_FROMLONLAT)
            {
                from = self.myActiveRoute[0];
            } else if self.myActiveTransportablePlan.is_empty() {
                throw_process_error(format!("Start edge not defined for person '{}'.", self.myVehicleParameter.id));
            } else {
                from = self.myActiveTransportablePlan.last().unwrap().get_destination();
            }
            if attrs.has_attribute(SUMO_ATTR_TO)
                || attrs.has_attribute(SUMO_ATTR_TO_JUNCTION)
                || attrs.has_attribute(SUMO_ATTR_TO_TAZ)
                || attrs.has_attribute(SUMO_ATTR_TOXY)
                || attrs.has_attribute(SUMO_ATTR_TOLONLAT)
            {
                to = *self.myActiveRoute.last().unwrap();
            } // else, to may also be derived from stopping place

            let duration: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_DURATION, &id, &mut ok, -1, true);
            if attrs.has_attribute(SUMO_ATTR_DURATION) && duration <= 0 {
                throw_process_error(format!("Non-positive walking duration for '{}'.", self.myVehicleParameter.id));
            }

            let mut departPos: f64 = 0.0;
            let mut arrivalPos: f64 = 0.0;
            let mut stoppingPlace: Ptr<MSStoppingPlace> = Ptr::null();
            self.parse_walk_positions(attrs, &id, from, &to, &mut departPos, &mut arrivalPos, &mut stoppingPlace, Ptr::null(), &mut ok);

            let mut modeSet: SVCPermissions = 0;
            if attrs.has_attribute(SUMO_ATTR_MODES) {
                let modes: String = attrs.get_opt(SUMO_ATTR_MODES, &id, &mut ok, String::new(), true);
                let mut errorMsg = String::new();
                // try to parse person modes
                if !SUMOVehicleParameter::parse_person_modes(&modes, "person", &id, &mut modeSet, &mut errorMsg) {
                    std::panic::panic_any(ProcessError::InvalidArgument(errorMsg));
                }
            } else {
                modeSet = self.myVehicleParameter.modes;
            }
            let group: String = attrs.get_opt(SUMO_ATTR_GROUP, &id, &mut ok, utils::options::options_cont::get_string("persontrip.default.group"), true);
            let net = MSNet::get_instance();
            let vehControl = net.m().get_vehicle_control();
            let types: String = attrs.get_opt(SUMO_ATTR_VTYPES, &id, &mut ok, self.myVehicleParameter.vTypes.clone(), true);
            let mut st = StringTokenizer::new(&types);
            while st.has_next() {
                let vtypeid = st.next().unwrap();
                let vType = vehControl.get_v_type(&vtypeid, Ptr::null(), false);
                if vType.is_null() {
                    std::panic::panic_any(ProcessError::InvalidArgument(format!(
                        "The vehicle type '{}' in a trip for person '{}' is not known.",
                        vtypeid, id
                    )));
                }
                modeSet |= if vType.get_vehicle_class() == SVC_BICYCLE { SVC_BICYCLE } else { SVC_PASSENGER };
            }
            let speed: f64 = attrs.get_opt(SUMO_ATTR_SPEED, &id, &mut ok, -1.0, true);
            if attrs.has_attribute(SUMO_ATTR_SPEED) && speed <= 0.0 {
                throw_process_error(format!("Non-positive walking speed for '{}'.", self.myVehicleParameter.id));
            }
            let walkFactor: f64 = attrs.get_opt(SUMO_ATTR_WALKFACTOR, &id, &mut ok, utils::options::options_cont::get_float("persontrip.walkfactor"), true);
            let departPosLatStr: String = attrs.get_opt(SUMO_ATTR_DEPARTPOS_LAT, "", &mut ok, String::new(), true);
            let departPosLat = self.interpret_depart_pos_lat(&departPosLatStr, -1, "personTrip");
            if ok {
                if self.myActiveTransportablePlan.is_empty() {
                    let mut initialDepartPos = self.myVehicleParameter.departPos;
                    if self.myVehicleParameter.departPosProcedure == DepartPosDefinition::RANDOM {
                        initialDepartPos = SUMORouteHandler::get_parsing_rng().m().rand_max(from.get_length());
                    }
                    let stage = MSStageWaiting::new(from, Ptr::null(), -1, self.myVehicleParameter.depart, initialDepartPos, "start", true, -1);
                    self.myActiveTransportablePlan.m().push(stage_waiting_ptr(stage));
                }
                self.myVehicleParameter.m().parametersSet |= VEHPARS_FORCE_REROUTE;
                let fromStop = if self.myActiveTransportablePlan.is_empty() {
                    Ptr::null()
                } else {
                    self.myActiveTransportablePlan.last().unwrap().get_destination_stop()
                };
                let toEdge = if to.is_null() { Ptr::from_mut(stoppingPlace.get_lane().get_edge()) } else { to };
                let trip = MSStageTrip::new(
                    from,
                    fromStop,
                    toEdge,
                    stoppingPlace,
                    duration,
                    modeSet,
                    &types,
                    speed,
                    walkFactor,
                    &group,
                    departPosLat,
                    attrs.has_attribute(SUMO_ATTR_ARRIVALPOS),
                    arrivalPos,
                );
                let raw = Box::into_raw(Box::new(trip));
                self.myActiveTransportablePlan.m().push(Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) }));
                // shortcut: myParamStack is an i32 in the skeleton (C++ vector of MSStage*), the trip stage is not pushed onto it until SUMORouteHandler is fixed
                if attrs.has_attribute(SUMO_ATTR_ARRIVALPOS) {
                    self.myActiveTransportablePlan.last().unwrap().m().mark_set(VEHPARS_ARRIVALPOS_SET as i32);
                }
            }
            self.myActiveRoute.clear();
        }));
        if let Err(payload) = result {
            self.delete_active_plan_and_vehicle_parameter();
            std::panic::resume_unwind(payload);
        }
    }

    /// C++: `void MSRouteHandler::addWalk(const SUMOSAXAttributes &)` [virtual, protected]
    /// add a fully specified walk
    // void
    // MSRouteHandler::addWalk(const SUMOSAXAttributes& attrs) {
    //     if (myVehicleParameter == nullptr) {
    //         throw ProcessError(TL("Cannot define person stage without person."));
    //     }
    //     myActiveRouteID = "";
    //     if (attrs.hasAttribute(SUMO_ATTR_EDGES) || attrs.hasAttribute(SUMO_ATTR_ROUTE)) {
    //         try {
    //             myActiveRoute.clear();
    //             bool ok = true;
    //             const SUMOTime duration = attrs.getOptSUMOTimeReporting(SUMO_ATTR_DURATION, nullptr, ok, -1);
    //             if (attrs.hasAttribute(SUMO_ATTR_DURATION) && duration <= 0) {
    //                 throw ProcessError(TLF("Non-positive walking duration for '%'.", myVehicleParameter->id));
    //             }
    //             double speed = -1; // default to vType speed
    //             if (attrs.hasAttribute(SUMO_ATTR_SPEED)) {
    //                 speed = attrs.get<double>(SUMO_ATTR_SPEED, nullptr, ok);
    //                 if (speed <= 0) {
    //                     throw ProcessError(TLF("Non-positive walking speed for '%'.", myVehicleParameter->id));
    //                 }
    //             }
    //             double departPos = 0;
    //             double arrivalPos = 0;
    //             MSStoppingPlace* bs = nullptr;
    //             if (attrs.hasAttribute(SUMO_ATTR_ROUTE)) {
    //                 myActiveRouteID = attrs.get<std::string>(SUMO_ATTR_ROUTE, myVehicleParameter->id.c_str(), ok);
    //                 ConstMSRoutePtr route = MSRoute::dictionary(myActiveRouteID, &myParsingRNG);
    //                 if (route == nullptr) {
    //                     throw ProcessError("The route '" + myActiveRouteID + "' for walk of person '" + myVehicleParameter->id + "' is not known.");
    //                 }
    //                 myActiveRoute = route->getEdges();
    //             } else {
    //                 MSEdge::parseEdgesList(attrs.get<std::string>(SUMO_ATTR_EDGES, myVehicleParameter->id.c_str(), ok), myActiveRoute, myActiveRouteID);
    //             }
    //             if (myActiveTransportablePlan->empty()) {
    //                 double initialDepartPos = myVehicleParameter->departPos;
    //                 if (myVehicleParameter->departPosProcedure == DepartPosDefinition::RANDOM) {
    //                     initialDepartPos = RandHelper::rand(myActiveRoute.front()->getLength(), &myParsingRNG);
    //                 }
    //                 myActiveTransportablePlan->push_back(new MSStageWaiting(myActiveRoute.front(), nullptr, -1, myVehicleParameter->depart, initialDepartPos, "start", true));
    //             }
    //             parseWalkPositions(attrs, myVehicleParameter->id, myActiveRoute.front(), myActiveRoute.back(), departPos, arrivalPos, bs, myActiveTransportablePlan->back(), ok);
    //             if (myActiveRoute.empty()) {
    //                 throw ProcessError(TLF("No edges to walk for person '%'.", myVehicleParameter->id));
    //             }
    //             if (myActiveTransportablePlan->back()->getDestination() != myActiveRoute.front() &&
    //                     myActiveTransportablePlan->back()->getDestination()->getToJunction() != myActiveRoute.front()->getFromJunction() &&
    //                     myActiveTransportablePlan->back()->getDestination()->getToJunction() != myActiveRoute.front()->getToJunction()) {
    //                 if (myActiveTransportablePlan->back()->getDestinationStop() == nullptr || myActiveTransportablePlan->back()->getDestinationStop()->getAccessPos(myActiveRoute.front()) < 0.) {
    //                     throw ProcessError("Disconnected plan for person '" + myVehicleParameter->id + "' (" + myActiveRoute.front()->getID() + " not connected to " + myActiveTransportablePlan->back()->getDestination()->getID() + ").");
    //                 }
    //             }
    //             const int departLane = attrs.getOpt<int>(SUMO_ATTR_DEPARTLANE, nullptr, ok, -1);
    //             const double departPosLat = interpretDepartPosLat(attrs.getOpt<std::string>(SUMO_ATTR_DEPARTPOS_LAT, nullptr, ok, ""), departLane, "walk");
    //             MSStageWalking* stage = new MSStageWalking(myVehicleParameter->id, myActiveRoute, bs, duration, speed, departPos, arrivalPos, departPosLat, departLane, myActiveRouteID);
    //             stage->setDeparted(attrs.getOptSUMOTimeReporting(SUMO_ATTR_STARTED, nullptr, ok, -1));
    //             stage->setEnded(attrs.getOptSUMOTimeReporting(SUMO_ATTR_ENDED, nullptr, ok, -1));
    //             stage->setTotalWaitingTime(attrs.getOptSUMOTimeReporting(SUMO_ATTR_WAITINGTIME, nullptr, ok, 0));
    //             if (attrs.hasAttribute(SUMO_ATTR_EXITTIMES) && OptionsCont::getOptions().getBool("vehroute-output.exit-times")) {
    //                 std::vector<SUMOTime>* exitTimes = new std::vector<SUMOTime>();
    //                 for (const std::string& tStr : attrs.get<std::vector<std::string> >(SUMO_ATTR_EXITTIMES, nullptr, ok)) {
    //                     exitTimes->push_back(string2time(tStr));
    //                 }
    //                 stage->setExitTimes(exitTimes);
    //             }
    //             if (attrs.hasAttribute(SUMO_ATTR_ARRIVALPOS)) {
    //                 stage->markSet(VEHPARS_ARRIVALPOS_SET);
    //             }
    //             myActiveTransportablePlan->push_back(stage);
    //             myParamStack.push_back(stage);
    //             myActiveRoute.clear();
    //         } catch (ProcessError&) {
    //             deleteActivePlanAndVehicleParameter();
    //             throw;
    //         }
    //     } else { // parse walks from->to as person trips
    //         addPersonTrip(attrs);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_walk(&mut self, attrs: &SUMOSAXAttributes) {
        if self.myVehicleParameter.is_null() {
            std::panic::panic_any(ProcessError::new("Cannot define person stage without person."));
        }
        self.myActiveRouteID = String::new();
        if attrs.has_attribute(SUMO_ATTR_EDGES) || attrs.has_attribute(SUMO_ATTR_ROUTE) {
            // C++ try/catch(ProcessError&): deleteActivePlanAndVehicleParameter, then rethrow
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.myActiveRoute.clear();
                let mut ok = true;
                let duration: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_DURATION, "", &mut ok, -1, true);
                if attrs.has_attribute(SUMO_ATTR_DURATION) && duration <= 0 {
                    throw_process_error(format!("Non-positive walking duration for '{}'.", self.myVehicleParameter.id));
                }
                let mut speed: f64 = -1.0; // default to vType speed
                if attrs.has_attribute(SUMO_ATTR_SPEED) {
                    speed = attrs.get(SUMO_ATTR_SPEED, "", &mut ok, true);
                    if speed <= 0.0 {
                        throw_process_error(format!("Non-positive walking speed for '{}'.", self.myVehicleParameter.id));
                    }
                }
                let mut departPos: f64 = 0.0;
                let mut arrivalPos: f64 = 0.0;
                let mut bs: Ptr<MSStoppingPlace> = Ptr::null();
                if attrs.has_attribute(SUMO_ATTR_ROUTE) {
                    self.myActiveRouteID = attrs.get(SUMO_ATTR_ROUTE, &self.myVehicleParameter.id, &mut ok, true);
                    if !MSRoute::has_route(&self.myActiveRouteID) {
                        throw_process_error(format!(
                            "The route '{}' for walk of person '{}' is not known.",
                            self.myActiveRouteID, self.myVehicleParameter.id
                        ));
                    }
                    let route = MSRoute::dictionary_3(&self.myActiveRouteID, SUMORouteHandler::get_parsing_rng());
                    self.myActiveRoute = route.get_edges().clone();
                } else {
                    let edges: String = attrs.get(SUMO_ATTR_EDGES, &self.myVehicleParameter.id, &mut ok, true);
                    let routeID = self.myActiveRouteID.clone();
                    MSEdge::parse_edges_list(&edges, &mut self.myActiveRoute, &routeID);
                }
                if self.myActiveTransportablePlan.is_empty() {
                    let mut initialDepartPos = self.myVehicleParameter.departPos;
                    if self.myVehicleParameter.departPosProcedure == DepartPosDefinition::RANDOM {
                        initialDepartPos = SUMORouteHandler::get_parsing_rng().m().rand_max(self.myActiveRoute[0].get_length());
                    }
                    let stage = MSStageWaiting::new(
                        self.myActiveRoute[0],
                        Ptr::null(),
                        -1,
                        self.myVehicleParameter.depart,
                        initialDepartPos,
                        "start",
                        true,
                        -1,
                    );
                    self.myActiveTransportablePlan.m().push(stage_waiting_ptr(stage));
                }
                let planBack = *self.myActiveTransportablePlan.last().unwrap();
                let id = self.myVehicleParameter.id.clone();
                let front = self.myActiveRoute.first().copied().unwrap_or(Ptr::null());
                let back = self.myActiveRoute.last().copied().unwrap_or(Ptr::null());
                self.parse_walk_positions(attrs, &id, front, &back, &mut departPos, &mut arrivalPos, &mut bs, planBack, &mut ok);
                if self.myActiveRoute.is_empty() {
                    throw_process_error(format!("No edges to walk for person '{}'.", self.myVehicleParameter.id));
                }
                let front = self.myActiveRoute[0];
                if planBack.get_destination() != front
                    && planBack.get_destination().get_to_junction() != front.get_from_junction()
                    && planBack.get_destination().get_to_junction() != front.get_to_junction()
                {
                    if planBack.get_destination_stop().is_null() || planBack.get_destination_stop().get_access_pos(front, Ptr::null()) < 0.0 {
                        throw_process_error(format!(
                            "Disconnected plan for person '{}' ({} not connected to {}).",
                            self.myVehicleParameter.id,
                            front.get_id(),
                            planBack.get_destination().get_id()
                        ));
                    }
                }
                let departLane: i32 = attrs.get_opt(SUMO_ATTR_DEPARTLANE, "", &mut ok, -1, true);
                let departPosLatStr: String = attrs.get_opt(SUMO_ATTR_DEPARTPOS_LAT, "", &mut ok, String::new(), true);
                let departPosLat = self.interpret_depart_pos_lat(&departPosLatStr, departLane, "walk");
                let mut stage = MSStageWalking::new(
                    &self.myVehicleParameter.id,
                    &self.myActiveRoute,
                    bs,
                    duration,
                    speed,
                    departPos,
                    arrivalPos,
                    departPosLat,
                    departLane,
                    &self.myActiveRouteID,
                );
                stage.set_departed(attrs.get_opt_sumo_time_reporting(SUMO_ATTR_STARTED, "", &mut ok, -1, true));
                stage.set_ended(attrs.get_opt_sumo_time_reporting(SUMO_ATTR_ENDED, "", &mut ok, -1, true));
                stage.set_total_waiting_time(attrs.get_opt_sumo_time_reporting(SUMO_ATTR_WAITINGTIME, "", &mut ok, 0, true));
                if attrs.has_attribute(SUMO_ATTR_EXITTIMES) && utils::options::options_cont::get_bool("vehroute-output.exit-times") {
                    let mut exitTimes: Vec<SUMOTime> = Vec::new();
                    let tStrs: Vec<String> = attrs.get(SUMO_ATTR_EXITTIMES, "", &mut ok, true);
                    for tStr in tStrs.iter() {
                        exitTimes.push(
                            utils::common::time::string2time(tStr).unwrap_or_else(|e| std::panic::panic_any(e)),
                        );
                    }
                    stage.set_exit_times(Ptr::from_raw(Box::into_raw(Box::new(exitTimes))));
                }
                if attrs.has_attribute(SUMO_ATTR_ARRIVALPOS) {
                    stage.mark_set(VEHPARS_ARRIVALPOS_SET as i32);
                }
                let raw = Box::into_raw(Box::new(stage));
                self.myActiveTransportablePlan.m().push(Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base.base) }));
                // shortcut: myParamStack is an i32 in the skeleton (C++ vector of MSStage*), the walk stage is not pushed onto it until SUMORouteHandler is fixed
                self.myActiveRoute.clear();
            }));
            if let Err(payload) = result {
                self.delete_active_plan_and_vehicle_parameter();
                std::panic::resume_unwind(payload);
            }
        } else {
            // parse walks from->to as person trips
            self.add_person_trip(attrs);
        }
    }

    /// C++: `void MSRouteHandler::addTransportable(const SUMOSAXAttributes &, const bool)` [virtual, protected]
    /// Processing of a person
    // void
    // MSRouteHandler::addTransportable(const SUMOSAXAttributes& /*attrs*/, const bool isPerson) {
    //     myActiveType = isPerson ? ObjectTypeEnum::PERSON : ObjectTypeEnum::CONTAINER;
    //     if (!MSNet::getInstance()->getVehicleControl().hasVType(myVehicleParameter->vtypeid)) {
    //         const std::string error = TLF("The type '%' for % '%' is not known.", myVehicleParameter->vtypeid, myActiveTypeName, myVehicleParameter->id);
    //         deleteActivePlanAndVehicleParameter();
    //         throw ProcessError(error);
    //     }
    //     myActiveTransportablePlan = new MSTransportable::MSTransportablePlan();
    // }
    #[allow(unused_variables)]
    pub fn add_transportable(&mut self, attrs: &SUMOSAXAttributes, isPerson: bool) {
        self.myActiveType = if isPerson {
            MSRouteHandler_ObjectTypeEnum::PERSON
        } else {
            MSRouteHandler_ObjectTypeEnum::CONTAINER
        };
        let net = MSNet::get_instance();
        if !net.m().get_vehicle_control().has_v_type(&self.myVehicleParameter.vtypeid) {
            let error = format!(
                "The type '{}' for {} '{}' is not known.",
                self.myVehicleParameter.vtypeid, self.myActiveTypeName, self.myVehicleParameter.id
            );
            self.delete_active_plan_and_vehicle_parameter();
            throw_process_error(error);
        }
        self.myActiveTransportablePlan = Ptr::from_raw(Box::into_raw(Box::new(Vec::new())));
    }

    /// C++: `void MSRouteHandler::addRide(const SUMOSAXAttributes &)` [virtual, protected]
    /// Processing of a ride
    // void
    // MSRouteHandler::addRide(const SUMOSAXAttributes& attrs) {
    //     addRideOrTransport(attrs, SUMO_TAG_RIDE);
    // }
    #[allow(unused_variables)]
    pub fn add_ride(&mut self, attrs: &SUMOSAXAttributes) {
        self.add_ride_or_transport(attrs, SUMO_TAG_RIDE);
    }

    /// C++: `void MSRouteHandler::addTranship(const SUMOSAXAttributes &)` [virtual, protected]
    /// Processing of a tranship
    // void
    // MSRouteHandler::addTranship(const SUMOSAXAttributes& attrs) {
    //     try {
    //         myActiveRoute.clear();
    //         const std::string cid = myVehicleParameter->id;
    //         bool ok = true;
    //         const MSEdge* from = nullptr;
    //         const MSEdge* to = nullptr;
    //         MSStoppingPlace* cs = nullptr;
    //
    //         double speed;
    //         const MSVehicleType* vtype = MSNet::getInstance()->getVehicleControl().getVType(myVehicleParameter->vtypeid);
    //         if (attrs.hasAttribute(SUMO_ATTR_SPEED)) { // speed is explicitly set
    //             speed = attrs.getOpt<double>(SUMO_ATTR_SPEED, nullptr, ok, -1);
    //             if (!ok) {
    //                 throw ProcessError(TLF("Could not read tranship speed for container '%'.", cid));
    //             }
    //         } else if (vtype != nullptr && vtype->wasSet(VTYPEPARS_MAXSPEED_SET)) { // speed is set by vtype
    //             speed = vtype->getMaxSpeed();
    //         } else { // default speed value
    //             speed = DEFAULT_CONTAINER_TRANSHIP_SPEED;
    //         }
    //         if (speed <= 0) {
    //             throw ProcessError(TLF("Non-positive tranship speed for container '%'.", cid));
    //         }
    //         // values from preceding stage:
    //         const MSEdge* preEdge = nullptr;
    //         double prePos = 0;
    //         if (!myActiveTransportablePlan->empty()) {
    //             preEdge = myActiveTransportablePlan->back()->getDestination();
    //             prePos = myActiveTransportablePlan->back()->getArrivalPos();
    //         }
    //         // set depart position as given attribute value, arrival position of preceding stage or default (=0)
    //         double departPos = attrs.getOpt<double>(SUMO_ATTR_DEPARTPOS, cid.c_str(), ok, prePos);
    //
    //         if (attrs.hasAttribute(SUMO_ATTR_EDGES)) {
    //             MSEdge::parseEdgesList(attrs.get<std::string>(SUMO_ATTR_EDGES, cid.c_str(), ok), myActiveRoute, myActiveRouteID);
    //         } else {
    //             // set 'from':
    //             if (attrs.hasAttribute(SUMO_ATTR_FROM)) {
    //                 const std::string fromID = attrs.get<std::string>(SUMO_ATTR_FROM, cid.c_str(), ok);
    //                 from = MSEdge::dictionary(fromID);
    //                 if (from == nullptr) {
    //                     throw ProcessError("The from edge '" + fromID + "' within a tranship of container '" + cid + "' is not known.");
    //                 }
    //                 if (preEdge != nullptr && preEdge != from) {
    //                     throw ProcessError("Disconnected plan for container '" + cid + "' (" + from->getID() + "!=" + preEdge->getID() + ").");
    //                 }
    //             } else if (preEdge == nullptr) {
    //                 throw ProcessError(TLF("The start edge for container '%' is not known.", cid));
    //             } else {
    //                 from = preEdge;
    //             }
    //             // set 'to':
    //             if (attrs.hasAttribute(SUMO_ATTR_TO)) {
    //                 const std::string toID = attrs.get<std::string>(SUMO_ATTR_TO, cid.c_str(), ok);
    //                 to = MSEdge::dictionary(toID);
    //                 if (to == nullptr) {
    //                     throw ProcessError("The to edge '" + toID + "' within a tranship of container '" + cid + "' is not known.");
    //                 }
    //             } else {
    //                 const std::string description = "container '" + cid + "' transhipping from edge '" + from->getID() + "'";
    //                 cs = retrieveStoppingPlace(attrs, " " + description);
    //                 if (cs != nullptr) {
    //                     to = &cs->getLane().getEdge();
    //                 } else {
    //                     throw ProcessError(TLF("Inconsistent tranship for container '%', needs either: 'edges', 'to', 'containerStop' (or any other stopping place)", cid));
    //                 }
    //             }
    //             myActiveRoute.push_back(from);
    //             myActiveRoute.push_back(to);
    //         }
    //         if (myActiveRoute.empty()) {
    //             throw ProcessError(TLF("No edges to tranship container '%'.", cid));
    //         }
    //         if (preEdge == nullptr) { // additional 'stop' to start the container plan
    //             myActiveTransportablePlan->push_back(new MSStageWaiting(
    //                     myActiveRoute.front(), nullptr, -1, myVehicleParameter->depart, departPos, "start", true));
    //         }
    //         double arrivalPos = attrs.getOpt<double>(SUMO_ATTR_ARRIVALPOS, cid.c_str(), ok,
    //                             cs == nullptr ? myActiveRoute.back()->getLength() : cs->getEndLanePosition());
    //         myActiveTransportablePlan->push_back(new MSStageTranship(myActiveRoute, cs, speed, departPos, arrivalPos));
    //         myParamStack.push_back(myActiveTransportablePlan->back());
    //         myActiveRoute.clear();
    //     } catch (ProcessError&) {
    //         deleteActivePlanAndVehicleParameter();
    //         throw;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_tranship(&mut self, attrs: &SUMOSAXAttributes) {
        // C++ try/catch(ProcessError&): deleteActivePlanAndVehicleParameter, then rethrow
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.myActiveRoute.clear();
            let cid = self.myVehicleParameter.id.clone();
            let mut ok = true;
            let mut from: Ptr<MSEdge> = Ptr::null();
            let mut to: Ptr<MSEdge> = Ptr::null();
            let mut cs: Ptr<MSStoppingPlace> = Ptr::null();

            let speed: f64;
            let net = MSNet::get_instance();
            let vtype = net.m().get_vehicle_control().get_v_type(&self.myVehicleParameter.vtypeid, Ptr::null(), false);
            if attrs.has_attribute(SUMO_ATTR_SPEED) {
                // speed is explicitly set
                speed = attrs.get_opt(SUMO_ATTR_SPEED, "", &mut ok, -1.0, true);
                if !ok {
                    throw_process_error(format!("Could not read tranship speed for container '{}'.", cid));
                }
            } else if !vtype.is_null() && vtype.was_set(VTYPEPARS_MAXSPEED_SET) {
                // speed is set by vtype
                speed = vtype.get_max_speed();
            } else {
                // default speed value
                speed = DEFAULT_CONTAINER_TRANSHIP_SPEED;
            }
            if speed <= 0.0 {
                throw_process_error(format!("Non-positive tranship speed for container '{}'.", cid));
            }
            // values from preceding stage:
            let mut preEdge: Ptr<MSEdge> = Ptr::null();
            let mut prePos: f64 = 0.0;
            if !self.myActiveTransportablePlan.is_empty() {
                preEdge = self.myActiveTransportablePlan.last().unwrap().get_destination();
                prePos = self.myActiveTransportablePlan.last().unwrap().get_arrival_pos();
            }
            // set depart position as given attribute value, arrival position of preceding stage or default (=0)
            let departPos: f64 = attrs.get_opt(SUMO_ATTR_DEPARTPOS, &cid, &mut ok, prePos, true);

            if attrs.has_attribute(SUMO_ATTR_EDGES) {
                let edges: String = attrs.get(SUMO_ATTR_EDGES, &cid, &mut ok, true);
                let routeID = self.myActiveRouteID.clone();
                MSEdge::parse_edges_list(&edges, &mut self.myActiveRoute, &routeID);
            } else {
                // set 'from':
                if attrs.has_attribute(SUMO_ATTR_FROM) {
                    let fromID: String = attrs.get(SUMO_ATTR_FROM, &cid, &mut ok, true);
                    from = MSEdge::dictionary_2(&fromID);
                    if from.is_null() {
                        throw_process_error(format!(
                            "The from edge '{}' within a tranship of container '{}' is not known.",
                            fromID, cid
                        ));
                    }
                    if !preEdge.is_null() && preEdge != from {
                        throw_process_error(format!(
                            "Disconnected plan for container '{}' ({}!={}).",
                            cid,
                            from.get_id(),
                            preEdge.get_id()
                        ));
                    }
                } else if preEdge.is_null() {
                    throw_process_error(format!("The start edge for container '{}' is not known.", cid));
                } else {
                    from = preEdge;
                }
                // set 'to':
                if attrs.has_attribute(SUMO_ATTR_TO) {
                    let toID: String = attrs.get(SUMO_ATTR_TO, &cid, &mut ok, true);
                    to = MSEdge::dictionary_2(&toID);
                    if to.is_null() {
                        throw_process_error(format!(
                            "The to edge '{}' within a tranship of container '{}' is not known.",
                            toID, cid
                        ));
                    }
                } else {
                    let description = format!("container '{}' transhipping from edge '{}'", cid, from.get_id());
                    cs = self.retrieve_stopping_place(attrs, &format!(" {}", description), Ptr::null());
                    if !cs.is_null() {
                        to = Ptr::from_mut(cs.get_lane().get_edge());
                    } else {
                        throw_process_error(format!(
                            "Inconsistent tranship for container '{}', needs either: 'edges', 'to', 'containerStop' (or any other stopping place)",
                            cid
                        ));
                    }
                }
                self.myActiveRoute.push(from);
                self.myActiveRoute.push(to);
            }
            if self.myActiveRoute.is_empty() {
                throw_process_error(format!("No edges to tranship container '{}'.", cid));
            }
            if preEdge.is_null() {
                // additional 'stop' to start the container plan
                let stage = MSStageWaiting::new(
                    self.myActiveRoute[0],
                    Ptr::null(),
                    -1,
                    self.myVehicleParameter.depart,
                    departPos,
                    "start",
                    true,
                    -1,
                );
                self.myActiveTransportablePlan.m().push(stage_waiting_ptr(stage));
            }
            let arrivalDefault = if cs.is_null() {
                self.myActiveRoute.last().unwrap().get_length()
            } else {
                cs.get_end_lane_position()
            };
            let arrivalPos: f64 = attrs.get_opt(SUMO_ATTR_ARRIVALPOS, &cid, &mut ok, arrivalDefault, true);
            let stage = MSStageTranship::new(&self.myActiveRoute, cs, speed, departPos, arrivalPos);
            let raw = Box::into_raw(Box::new(stage));
            self.myActiveTransportablePlan.m().push(Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base.base) }));
            // shortcut: myParamStack is an i32 in the skeleton (C++ vector of MSStage*), the tranship stage is not pushed onto it until SUMORouteHandler is fixed
            self.myActiveRoute.clear();
        }));
        if let Err(payload) = result {
            self.delete_active_plan_and_vehicle_parameter();
            std::panic::resume_unwind(payload);
        }
    }

    /// C++: `void MSRouteHandler::addTransport(const SUMOSAXAttributes &)` [virtual, protected]
    /// Processing of a transport
    // void
    // MSRouteHandler::addTransport(const SUMOSAXAttributes& attrs) {
    //     addRideOrTransport(attrs, SUMO_TAG_TRANSPORT);
    // }
    #[allow(unused_variables)]
    pub fn add_transport(&mut self, attrs: &SUMOSAXAttributes) {
        self.add_ride_or_transport(attrs, SUMO_TAG_TRANSPORT);
    }

    /// C++: `void MSRouteHandler::parseWalkPositions(const SUMOSAXAttributes &, const std::string &, const MSEdge *, const MSEdge *&, double &, double &, MSStoppingPlace *&, const MSStage *const, bool &)` [protected]
    /// @ brief parse depart- and arrival positions of a walk
    // void
    // MSRouteHandler::parseWalkPositions(const SUMOSAXAttributes& attrs, const std::string& personID,
    //                                    const MSEdge* fromEdge, const MSEdge*& toEdge,
    //                                    double& departPos, double& arrivalPos, MSStoppingPlace*& bs,
    //                                    const MSStage* const lastStage, bool& ok) {
    //     try {
    //         const std::string description = "person '" + personID + "' walking from edge '" + fromEdge->getID() + "'";
    //
    //         if (attrs.hasAttribute(SUMO_ATTR_DEPARTPOS)) {
    //             WRITE_WARNING(TL("The attribute departPos is no longer supported for walks, please use the person attribute, the arrivalPos of the previous step or explicit stops."));
    //         }
    //         departPos = 0.;
    //         if (lastStage != nullptr) {
    //             if (lastStage->getDestinationStop() != nullptr) {
    //                 departPos = lastStage->getDestinationStop()->getAccessPos(fromEdge, &myParsingRNG);
    //             } else if (lastStage->getDestination() == fromEdge) {
    //                 departPos = lastStage->getArrivalPos();
    //             } else if (lastStage->getDestination()->getToJunction() == fromEdge->getToJunction()) {
    //                 departPos = fromEdge->getLength();
    //             }
    //         }
    //
    //         bs = retrieveStoppingPlace(attrs, " " + description);
    //         if (bs != nullptr) {
    //             arrivalPos = bs->getAccessPos(toEdge != nullptr ? toEdge : &bs->getLane().getEdge());
    //             if (arrivalPos < 0) {
    //                 throw ProcessError("Bus stop '" + bs->getID() + "' is not connected to arrival edge '" + toEdge->getID() + "' for " + description + ".");
    //             }
    //             if (attrs.hasAttribute(SUMO_ATTR_ARRIVALPOS)) {
    //                 const double length = toEdge != nullptr ? toEdge->getLength() : bs->getLane().getLength();
    //                 const double arrPos = SUMOVehicleParserHelper::parseWalkPos(SUMO_ATTR_ARRIVALPOS, myHardFail, description, length,
    //                                       attrs.get<std::string>(SUMO_ATTR_ARRIVALPOS, description.c_str(), ok), &myParsingRNG);
    //                 if (arrPos >= bs->getBeginLanePosition() && arrPos < bs->getEndLanePosition()) {
    //                     arrivalPos = arrPos;
    //                 } else {
    //                     WRITE_WARNINGF(TL("Ignoring arrivalPos for % because it is outside the given stop '%'."), description, toString(SUMO_ATTR_BUS_STOP));
    //                     arrivalPos = bs->getAccessPos(&bs->getLane().getEdge());
    //                 }
    //             }
    //         } else {
    //             if (toEdge == nullptr) {
    //                 throw ProcessError(TLF("No destination edge for %.", description));
    //             }
    //             if (attrs.hasAttribute(SUMO_ATTR_ARRIVALPOS)) {
    //                 arrivalPos = SUMOVehicleParserHelper::parseWalkPos(SUMO_ATTR_ARRIVALPOS, myHardFail, description, toEdge->getLength(),
    //                              attrs.get<std::string>(SUMO_ATTR_ARRIVALPOS, description.c_str(), ok), &myParsingRNG);
    //             } else {
    //                 arrivalPos = toEdge->getLength() / 2.;
    //             }
    //         }
    //     } catch (ProcessError&) {
    //         deleteActivePlanAndVehicleParameter();
    //         throw;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_walk_positions(&mut self, attrs: &SUMOSAXAttributes, personID: &str, fromEdge: Ptr<MSEdge>, toEdge: &Ptr<MSEdge>, departPos: &mut f64, arrivalPos: &mut f64, bs: &mut Ptr<MSStoppingPlace>, lastStage: Ptr<MSStage>, ok: &mut bool) {
        // C++ try/catch(ProcessError&): deleteActivePlanAndVehicleParameter, then rethrow
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let rng = SUMORouteHandler::get_parsing_rng();
            let description = format!("person '{}' walking from edge '{}'", personID, fromEdge.get_id());

            if attrs.has_attribute(SUMO_ATTR_DEPARTPOS) {
                utils::common::msg_handler::write_warning("The attribute departPos is no longer supported for walks, please use the person attribute, the arrivalPos of the previous step or explicit stops.");
            }
            *departPos = 0.0;
            if !lastStage.is_null() {
                if !lastStage.get_destination_stop().is_null() {
                    *departPos = lastStage.get_destination_stop().get_access_pos(fromEdge, rng);
                } else if lastStage.get_destination() == fromEdge {
                    *departPos = lastStage.get_arrival_pos();
                } else if lastStage.get_destination().get_to_junction() == fromEdge.get_to_junction() {
                    *departPos = fromEdge.get_length();
                }
            }

            *bs = self.retrieve_stopping_place(attrs, &format!(" {}", description), Ptr::null());
            if !bs.is_null() {
                let stopEdge = if !toEdge.is_null() { *toEdge } else { Ptr::from_mut(bs.get_lane().get_edge()) };
                *arrivalPos = bs.get_access_pos(stopEdge, rng);
                if *arrivalPos < 0.0 {
                    throw_process_error(format!(
                        "Bus stop '{}' is not connected to arrival edge '{}' for {}.",
                        bs.get_id(),
                        toEdge.get_id(),
                        description
                    ));
                }
                if attrs.has_attribute(SUMO_ATTR_ARRIVALPOS) {
                    let length = if !toEdge.is_null() { toEdge.get_length() } else { bs.get_lane().get_length() };
                    let arrStr: String = attrs.get(SUMO_ATTR_ARRIVALPOS, &description, ok, true);
                    let arrPos = SUMOVehicleParserHelper::parse_walk_pos(SUMO_ATTR_ARRIVALPOS, self.myHardFail, &description, length, &arrStr, rng);
                    if arrPos >= bs.get_begin_lane_position() && arrPos < bs.get_end_lane_position() {
                        *arrivalPos = arrPos;
                    } else {
                        // toString(SUMO_ATTR_BUS_STOP) is "busStop"
                        write_warningf!(
                            "Ignoring arrivalPos for % because it is outside the given stop '%'.",
                            description,
                            "busStop"
                        );
                        *arrivalPos = bs.get_access_pos(Ptr::from_mut(bs.get_lane().get_edge()), rng);
                    }
                }
            } else {
                if toEdge.is_null() {
                    throw_process_error(format!("No destination edge for {}.", description));
                }
                if attrs.has_attribute(SUMO_ATTR_ARRIVALPOS) {
                    let arrStr: String = attrs.get(SUMO_ATTR_ARRIVALPOS, &description, ok, true);
                    *arrivalPos = SUMOVehicleParserHelper::parse_walk_pos(SUMO_ATTR_ARRIVALPOS, self.myHardFail, &description, toEdge.get_length(), &arrStr, rng);
                } else {
                    *arrivalPos = toEdge.get_length() / 2.0;
                }
            }
        }));
        if let Err(payload) = result {
            self.delete_active_plan_and_vehicle_parameter();
            std::panic::resume_unwind(payload);
        }
    }

    /// C++: `void MSRouteHandler::initLaneTree(NamedRTree *)` [virtual, protected]
    // void
    // MSRouteHandler::initLaneTree(NamedRTree* tree) {
    //     for (const auto& edge : MSEdge::getAllEdges()) {
    //         if (edge->isNormal() || using_internal_lanes()) {
    //             for (MSLane* lane : edge->getLanes()) {
    //                 Boundary b = lane->getShape().getBoxBoundary();
    //                 const float cmin[2] = {(float) b.xmin(), (float) b.ymin()};
    //                 const float cmax[2] = {(float) b.xmax(), (float) b.ymax()};
    //                 tree->Insert(cmin, cmax, lane);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_lane_tree(&mut self, tree: Ptr<Unmapped /* NamedRTree */>) {
        // shortcut: NamedRTree is not ported, lanes are not inserted into the tree until it is (the tree only serves map matching)
        let _ = tree;
    }

    /// C++: `MSEdge * MSRouteHandler::retrieveEdge(const std::string &)` [virtual, protected]
    // MSEdge*
    // MSRouteHandler::retrieveEdge(const std::string& id) {
    //     return MSEdge::dictionary(id);
    // }
    #[allow(unused_variables)]
    pub fn retrieve_edge(&mut self, id: &str) -> Ptr<MSEdge> {
        MSEdge::dictionary_2(id)
    }

    /// C++: `SumoRNG * MSRouteHandler::getRNG()` [virtual, protected]
    // SumoRNG*
    // MSRouteHandler::getRNG() {
    //     return &myParsingRNG;
    // }
    #[allow(unused_variables)]
    pub fn get_rng(&mut self) -> Ptr<SumoRNG> {
        SUMORouteHandler::get_parsing_rng()
    }

    /// C++: `const SUMOVTypeParameter * MSRouteHandler::getVTypeParameter(const std::string &)` [virtual, protected]
    // const SUMOVTypeParameter*
    // MSRouteHandler::getVTypeParameter(const std::string& refid) {
    //     const auto t = MSNet::getInstance()->getVehicleControl().getVType(refid);
    //     return t != nullptr ? &t->getParameter() : nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_v_type_parameter(&mut self, refid: &str) -> Ptr<SUMOVTypeParameter> {
        let net = MSNet::get_instance();
        let t = net.m().get_vehicle_control().get_v_type(refid, Ptr::null(), false);
        if !t.is_null() {
            Ptr::new(t.get_parameter())
        } else {
            Ptr::null()
        }
    }

    /// C++: `void MSRouteHandler::deleteActivePlanAndVehicleParameter()` [private]
    /// delete already created MSTransportablePlans if error occurs before handing over responsibility to a MSTransportable.
    // void
    // MSRouteHandler::deleteActivePlanAndVehicleParameter() {
    //     if (myActiveTransportablePlan != nullptr) {
    //         for (MSStage* const s : *myActiveTransportablePlan) {
    //             delete s;
    //         }
    //         delete myActiveTransportablePlan;
    //     }
    //     delete myVehicleParameter;
    //     resetActivePlanAndVehicleParameter();
    // }
    #[allow(unused_variables)]
    pub fn delete_active_plan_and_vehicle_parameter(&mut self) {
        let plan = self.myActiveTransportablePlan;
        if !plan.is_null() {
            // shortcut: C++ `delete s` on each stage is a virtual delete, stages are leaked until the concrete stage type is known here
            delete_ptr(plan);
        }
        delete_ptr(self.myVehicleParameter);
        self.reset_active_plan_and_vehicle_parameter();
    }

    /// C++: `void MSRouteHandler::resetActivePlanAndVehicleParameter()` [private]
    /// reset MSTransportablePlans after transportable tag closes
    // void
    // MSRouteHandler::resetActivePlanAndVehicleParameter() {
    //     myVehicleParameter = nullptr;
    //     myActiveTransportablePlan = nullptr;
    //     myActiveType = ObjectTypeEnum::UNDEFINED;
    // }
    #[allow(unused_variables)]
    pub fn reset_active_plan_and_vehicle_parameter(&mut self) {
        self.myVehicleParameter = Ptr::null();
        self.myActiveTransportablePlan = Ptr::null();
        self.myActiveType = MSRouteHandler_ObjectTypeEnum::UNDEFINED;
    }

    /// C++: `void MSRouteHandler::closeTransportableFlow()` [private]
    /// ends the flow of a transportable
    // void
    // MSRouteHandler::closeTransportableFlow() {
    //     try {
    //         const std::string fid = myVehicleParameter->id;
    //         if (myActiveTransportablePlan->size() == 0) {
    //             throw ProcessError(myActiveTypeName + "Flow '" + fid + "' has no plan.");
    //         }
    //         // let's check whether this transportable (person/container) had to depart before the simulation starts
    //         if (!(myAddVehiclesDirectly || checkLastDepart())
    //                 || (myVehicleParameter->depart <= MSNet::getInstance()->getStateLoaderTime() && !myAmLoadingState)) {
    //             deleteActivePlanAndVehicleParameter();
    //             return;
    //         }
    //         // instantiate all persons/containers of this flow
    //         int i = 0;
    //         registerLastDepart();
    //         std::string baseID = myVehicleParameter->id;
    //         if (myVehicleParameter->repetitionProbability > 0) {
    //             if (myVehicleParameter->repetitionEnd == SUMOTime_MAX) {
    //                 throw ProcessError("probabilistic " + myActiveTypeName + "Flow '" + fid + "' must specify end time");
    //             } else {
    //                 for (SUMOTime t = myVehicleParameter->depart; t < myVehicleParameter->repetitionEnd; t += TIME2STEPS(1)) {
    //                     if (RandHelper::rand(&myParsingRNG) < myVehicleParameter->repetitionProbability) {
    //                         // type existence has been checked on opening
    //                         MSVehicleType* const type = MSNet::getInstance()->getVehicleControl().getVType(myVehicleParameter->vtypeid, &myParsingRNG);
    //                         addFlowTransportable(t, type, baseID, i++);
    //                     }
    //                 }
    //             }
    //         } else {
    //             SUMOTime depart = myVehicleParameter->depart;
    //             if (myVehicleParameter->repetitionOffset < 0) {
    //                 // poisson: randomize first depart
    //                 myVehicleParameter->incrementFlow(1, &myParsingRNG);
    //             }
    //             for (; i < myVehicleParameter->repetitionNumber && (myVehicleParameter->repetitionNumber != std::numeric_limits<long long int>::max()
    //                     || depart + myVehicleParameter->repetitionTotalOffset <= myVehicleParameter->repetitionEnd); i++) {
    //                 // type existence has been checked on opening
    //                 MSVehicleType* const type = MSNet::getInstance()->getVehicleControl().getVType(myVehicleParameter->vtypeid, &myParsingRNG);
    //                 addFlowTransportable(depart + myVehicleParameter->repetitionTotalOffset, type, baseID, i);
    //                 myVehicleParameter->incrementFlow(1, &myParsingRNG);
    //             }
    //         }
    //         resetActivePlanAndVehicleParameter();
    //     } catch (ProcessError&) {
    //         deleteActivePlanAndVehicleParameter();
    //         throw;
    //     }
    //     myStartTriggeredInFlow = false;
    // }
    #[allow(unused_variables)]
    pub fn close_transportable_flow(&mut self) {
        // C++ try/catch(ProcessError&): deleteActivePlanAndVehicleParameter, then rethrow
        // the closure returns true for the early `return` of the C++ body (which skips resetting myStartTriggeredInFlow)
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> bool {
            let fid = self.myVehicleParameter.id.clone();
            if self.myActiveTransportablePlan.is_empty() {
                throw_process_error(format!("{}Flow '{}' has no plan.", self.myActiveTypeName, fid));
            }
            let net = MSNet::get_instance();
            // let's check whether this transportable (person/container) had to depart before the simulation starts
            if !(self.myAddVehiclesDirectly || self.check_last_depart())
                || (self.myVehicleParameter.depart <= net.m().get_state_loader_time() && !self.myAmLoadingState)
            {
                self.delete_active_plan_and_vehicle_parameter();
                return true;
            }
            // instantiate all persons/containers of this flow
            let mut i: i32 = 0;
            self.register_last_depart();
            let baseID = self.myVehicleParameter.id.clone();
            if self.myVehicleParameter.repetitionProbability > 0.0 {
                if self.myVehicleParameter.repetitionEnd == i64::MAX {
                    throw_process_error(format!("probabilistic {}Flow '{}' must specify end time", self.myActiveTypeName, fid));
                } else {
                    let mut t = self.myVehicleParameter.depart;
                    while t < self.myVehicleParameter.repetitionEnd {
                        if SUMORouteHandler::get_parsing_rng().m().rand() < self.myVehicleParameter.repetitionProbability {
                            // type existence has been checked on opening
                            let type_ = net.m().get_vehicle_control().get_v_type(&self.myVehicleParameter.vtypeid, SUMORouteHandler::get_parsing_rng(), false);
                            self.add_flow_transportable(t, type_, &baseID, i);
                            i += 1;
                        }
                        t += utils::common::time::time2steps(1.0);
                    }
                }
            } else {
                let depart = self.myVehicleParameter.depart;
                if self.myVehicleParameter.repetitionOffset < 0 {
                    // poisson: randomize first depart
                    self.myVehicleParameter.m().increment_flow(1.0, SUMORouteHandler::get_parsing_rng());
                }
                while (i as i64) < self.myVehicleParameter.repetitionNumber
                    && (self.myVehicleParameter.repetitionNumber != i64::MAX
                        || depart + self.myVehicleParameter.repetitionTotalOffset <= self.myVehicleParameter.repetitionEnd)
                {
                    // type existence has been checked on opening
                    let type_ = net.m().get_vehicle_control().get_v_type(&self.myVehicleParameter.vtypeid, SUMORouteHandler::get_parsing_rng(), false);
                    self.add_flow_transportable(depart + self.myVehicleParameter.repetitionTotalOffset, type_, &baseID, i);
                    self.myVehicleParameter.m().increment_flow(1.0, SUMORouteHandler::get_parsing_rng());
                    i += 1;
                }
            }
            self.reset_active_plan_and_vehicle_parameter();
            false
        }));
        match result {
            Ok(true) => return,
            Ok(false) => {}
            Err(payload) => {
                self.delete_active_plan_and_vehicle_parameter();
                std::panic::resume_unwind(payload);
            }
        }
        self.myStartTriggeredInFlow = false;
    }

    /// C++: `void MSRouteHandler::closeTransportable()` [private]
    /// ends the processing of a transportable (as person or container)
    // void
    // MSRouteHandler::closeTransportable() {
    //     try {
    //         if (myActiveTransportablePlan->size() == 0) {
    //             std::string error = myActiveTypeName + " '" + myVehicleParameter->id + "' has no plan.";
    //             error[0] = (char)::toupper((char)error[0]);
    //             throw ProcessError(error);
    //         }
    //         // let's check whether this transportable had to depart before the simulation starts
    //         if (!(myAddVehiclesDirectly || checkLastDepart())
    //                 || (myVehicleParameter->depart <= MSNet::getInstance()->getStateLoaderTime() && !myAmLoadingState)) {
    //             deleteActivePlanAndVehicleParameter();
    //             return;
    //         }
    //         // type existence has been checked on opening
    //         MSVehicleType* type = MSNet::getInstance()->getVehicleControl().getVType(myVehicleParameter->vtypeid, &myParsingRNG);
    //         if (myActiveType == ObjectTypeEnum::PERSON
    //                 && type->getVehicleClass() != SVC_PEDESTRIAN
    //                 && !type->getParameter().wasSet(VTYPEPARS_VEHICLECLASS_SET)) {
    //             WRITE_WARNINGF(TL("Person '%' receives type '%' which implicitly uses unsuitable vClass '%'."), myVehicleParameter->id, type->getID(), toString(type->getVehicleClass()));
    //         }
    //         int created = addFlowTransportable(myVehicleParameter->depart, type, myVehicleParameter->id, -1);
    //         registerLastDepart();
    //         if (created > 0) {
    //             resetActivePlanAndVehicleParameter();
    //         } else {
    //             deleteActivePlanAndVehicleParameter();
    //         }
    //     } catch (ProcessError&) {
    //         deleteActivePlanAndVehicleParameter();
    //         throw;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_transportable(&mut self) {
        // C++ try/catch(ProcessError&): deleteActivePlanAndVehicleParameter, then rethrow
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if self.myActiveTransportablePlan.is_empty() {
                let mut error = format!("{} '{}' has no plan.", self.myActiveTypeName, self.myVehicleParameter.id);
                // error[0] = toupper(error[0])
                let first = error[..1].to_uppercase();
                error.replace_range(..1, &first);
                throw_process_error(error);
            }
            let net = MSNet::get_instance();
            // let's check whether this transportable had to depart before the simulation starts
            if !(self.myAddVehiclesDirectly || self.check_last_depart())
                || (self.myVehicleParameter.depart <= net.m().get_state_loader_time() && !self.myAmLoadingState)
            {
                self.delete_active_plan_and_vehicle_parameter();
                return;
            }
            // type existence has been checked on opening
            let type_ = net.m().get_vehicle_control().get_v_type(&self.myVehicleParameter.vtypeid, SUMORouteHandler::get_parsing_rng(), false);
            if self.myActiveType == MSRouteHandler_ObjectTypeEnum::PERSON
                && type_.get_vehicle_class() != SVC_PEDESTRIAN
                && !type_.get_parameter().was_set(VTYPEPARS_VEHICLECLASS_SET)
            {
                write_warningf!(
                    "Person '%' receives type '%' which implicitly uses unsuitable vClass '%'.",
                    self.myVehicleParameter.id,
                    type_.get_id().to_string(),
                    get_vehicle_class_names(type_.get_vehicle_class(), false)
                );
            }
            let id = self.myVehicleParameter.id.clone();
            let created = self.add_flow_transportable(self.myVehicleParameter.depart, type_, &id, -1);
            self.register_last_depart();
            if created > 0 {
                self.reset_active_plan_and_vehicle_parameter();
            } else {
                self.delete_active_plan_and_vehicle_parameter();
            }
        }));
        if let Err(payload) = result {
            self.delete_active_plan_and_vehicle_parameter();
            std::panic::resume_unwind(payload);
        }
    }

    /// C++: `int MSRouteHandler::addFlowTransportable(SUMOTime, MSVehicleType *, const std::string &, int)` [private]
    /// delete already created MSTransportablePlans if error occurs before handing over responsibility to a MSTransportable.
    // int
    // MSRouteHandler::addFlowTransportable(SUMOTime depart, MSVehicleType* type, const std::string& baseID, int i) {
    //     try {
    //         int numCreated = 0;
    //         MSNet* const net = MSNet::getInstance();
    //         MSTransportableControl& tc = myActiveType == ObjectTypeEnum::PERSON ? net->getPersonControl() : net->getContainerControl();
    //         const MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //         //MSTransportableControl& pc = net->getPersonControl();
    //         const int quota = vc.getQuota(vc.getScale() * type->getParameter().scale, tc.getLoadedNumber());
    //         if (quota == 0) {
    //             tc.addDiscarded();
    //         }
    //         for (int j = 0; j < quota; j++) {
    //             if (i > 0 || j > 0) {
    //                 // copy parameter and plan because the transportable takes over responsibility
    //                 SUMOVehicleParameter* copyParam = new SUMOVehicleParameter();
    //                 *copyParam = *myVehicleParameter;
    //                 myVehicleParameter = copyParam;
    //                 MSTransportable::MSTransportablePlan* copyPlan = new MSTransportable::MSTransportablePlan();
    //                 MSStage* lastStage = nullptr;
    //                 for (MSStage* const s : *myActiveTransportablePlan) {
    //                     copyPlan->push_back(s->clone());
    //                     if (lastStage != nullptr && s->getStageType() == MSStageType::WALKING && lastStage->getDestinationStop() != nullptr) {
    //                         MSStageMoving* walk = static_cast<MSStageMoving*>(copyPlan->back());
    //                         walk->setDepartPos(lastStage->getDestinationStop()->getAccessPos(walk->getEdge(), &myParsingRNG));
    //                     }
    //                     lastStage = s;
    //                 }
    //                 myActiveTransportablePlan = copyPlan;
    //                 if (myVehicleParameter->departPosProcedure == DepartPosDefinition::RANDOM) {
    //                     const double initialDepartPos = RandHelper::rand(myActiveTransportablePlan->front()->getDestination()->getLength(), &myParsingRNG);
    //                     myActiveTransportablePlan->front()->setArrivalPos(initialDepartPos);
    //                 }
    //             }
    //             myVehicleParameter->id = (baseID
    //                                       + (i >= 0 ? "." + toString(i) : "")
    //                                       + (j > 0 ? "." + toString(j) : ""));
    //             if (state_loaded() && tc.get(myVehicleParameter->id) != nullptr) {
    //                 return numCreated;
    //             }
    //             myVehicleParameter->depart = depart += net->getInsertionControl().computeRandomDepartOffset();
    //             MSTransportable* transportable = myActiveType == ObjectTypeEnum::PERSON ?
    //                                              tc.buildPerson(myVehicleParameter, type, myActiveTransportablePlan, &myParsingRNG) :
    //                                              tc.buildContainer(myVehicleParameter, type, myActiveTransportablePlan);
    //             numCreated++;
    //             if (!tc.add(transportable)) {
    //                 std::string error = "Another " + myActiveTypeName + " with the id '" + myVehicleParameter->id + "' exists.";
    //                 delete transportable;
    //                 resetActivePlanAndVehicleParameter();
    //                 throw ProcessError(error);
    //             } else if ((net->hasPersons() && net->getPersonControl().get(myVehicleParameter->id) != nullptr)
    //                        && (net->hasContainers() && net->getContainerControl().get(myVehicleParameter->id) != nullptr)) {
    //                 WRITE_WARNINGF(TL("There exists a person and a container with the same id '%'. Starting with SUMO 1.9.0 this is an error."), myVehicleParameter->id);
    //             }
    //         }
    //         return numCreated;
    //     } catch (ProcessError&) {
    //         deleteActivePlanAndVehicleParameter();
    //         throw;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_flow_transportable(&mut self, depart: SUMOTime, type_: Ptr<MSVehicleType>, baseID: &str, i: i32) -> i32 {
        // C++ try/catch(ProcessError&): deleteActivePlanAndVehicleParameter, then rethrow
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> i32 {
            let mut numCreated: i32 = 0;
            let mut depart = depart;
            let net = MSNet::get_instance();
            let isPerson = self.myActiveType == MSRouteHandler_ObjectTypeEnum::PERSON;
            let tc: &mut MSTransportableControl = if isPerson {
                net.m().get_person_control()
            } else {
                net.m().get_container_control()
            };
            let vc = net.m().get_vehicle_control();
            let quota = vc.get_quota(vc.get_scale() * type_.get_parameter().scale, tc.get_loaded_number());
            if quota == 0 {
                tc.add_discarded();
            }
            for j in 0..quota {
                if i > 0 || j > 0 {
                    // copy parameter and plan because the transportable takes over responsibility
                    self.myVehicleParameter = Ptr::from_raw(Box::into_raw(Box::new((*self.myVehicleParameter).clone())));
                    let copyPlan = Ptr::from_raw(Box::into_raw(Box::new(Vec::<Ptr<MSStage>>::new())));
                    let mut lastStage: Ptr<MSStage> = Ptr::null();
                    for s in self.myActiveTransportablePlan.iter() {
                        let cloned = s.m().clone();
                        copyPlan.m().push(cloned);
                        if !lastStage.is_null()
                            && s.get_stage_type() == MSStageType::WALKING
                            && !lastStage.get_destination_stop().is_null()
                        {
                            // shortcut: C++ static_cast<MSStageMoving*> of the copied walk needs a checked downcast from MSStage, its departPos is not reset until it exists
                        }
                        lastStage = *s;
                    }
                    self.myActiveTransportablePlan = copyPlan;
                    if self.myVehicleParameter.departPosProcedure == DepartPosDefinition::RANDOM {
                        let length = self.myActiveTransportablePlan.first().unwrap().get_destination().get_length();
                        let initialDepartPos = SUMORouteHandler::get_parsing_rng().m().rand_max(length);
                        self.myActiveTransportablePlan.first().unwrap().m().set_arrival_pos(initialDepartPos);
                    }
                }
                let idStr = format!(
                    "{}{}{}",
                    baseID,
                    if i >= 0 { format!(".{}", i) } else { String::new() },
                    if j > 0 { format!(".{}", j) } else { String::new() }
                );
                self.myVehicleParameter.m().id = idStr.clone();
                if state_loaded() && !tc.get(&idStr).is_null() {
                    return numCreated;
                }
                depart += net.m().get_insertion_control().compute_random_depart_offset();
                self.myVehicleParameter.m().depart = depart;
                let transportable = if isPerson {
                    tc.build_person(self.myVehicleParameter, type_, self.myActiveTransportablePlan, SUMORouteHandler::get_parsing_rng())
                } else {
                    tc.build_container(self.myVehicleParameter, type_, self.myActiveTransportablePlan)
                };
                numCreated += 1;
                if !tc.add(transportable) {
                    let error = format!("Another {} with the id '{}' exists.", self.myActiveTypeName, idStr);
                    delete_ptr(transportable);
                    self.reset_active_plan_and_vehicle_parameter();
                    throw_process_error(error);
                } else if net.m().has_persons()
                    && !net.m().get_person_control().get(&idStr).is_null()
                    && net.m().has_containers()
                    && !net.m().get_container_control().get(&idStr).is_null()
                {
                    write_warningf!(
                        "There exists a person and a container with the same id '%'. Starting with SUMO 1.9.0 this is an error.",
                        idStr
                    );
                }
            }
            numCreated
        }));
        match result {
            Ok(n) => n,
            Err(payload) => {
                self.delete_active_plan_and_vehicle_parameter();
                std::panic::resume_unwind(payload);
            }
        }
    }

    /// C++: `double MSRouteHandler::interpretDepartPosLat(const std::string &, int, const std::string &)` [private]
    // double
    // MSRouteHandler::interpretDepartPosLat(const std::string& value, int departLane, const std::string& element) {
    //     double pos = MSPModel::UNSPECIFIED_POS_LAT;
    //     if (value == "") {
    //         return pos;
    //     }
    //     std::string error;
    //     DepartPosLatDefinition dpd;
    //     if (SUMOVehicleParameter::parseDepartPosLat(value, element, myVehicleParameter->id, pos, dpd, error)) {
    //         if (dpd != DepartPosLatDefinition::GIVEN) {
    //             const MSLane* lane = MSStageMoving::checkDepartLane(myActiveRoute.front(), SVC_IGNORING, departLane, myVehicleParameter->id);
    //             if (lane == nullptr) {
    //                 throw ProcessError(TLF("Could not find departure lane for walk of person '%' when interpreting departPosLat", myVehicleParameter->id));
    //             }
    //             const double usableWidth = lane->getWidth() - 0.5;
    //             switch (dpd) {
    //                 case DepartPosLatDefinition::RIGHT:
    //                     pos = -usableWidth / 2;
    //                     break;
    //                 case DepartPosLatDefinition::LEFT:
    //                     pos = usableWidth / 2;
    //                     break;
    //                 case DepartPosLatDefinition::CENTER:
    //                     pos = 0;
    //                     break;
    //                 case DepartPosLatDefinition::RANDOM:
    //                 case DepartPosLatDefinition::FREE:
    //                 case DepartPosLatDefinition::RANDOM_FREE:
    //                     /// @note must be randomized for every person individually when loading a personFlow
    //                     pos = MSPModel::RANDOM_POS_LAT;
    //                     break;
    //                 default:
    //                     break;
    //             }
    //         }
    //     } else {
    //         throw ProcessError(error);
    //     }
    //     return pos;
    // }
    #[allow(unused_variables)]
    pub fn interpret_depart_pos_lat(&mut self, value: &str, departLane: i32, element: &str) -> f64 {
        let mut pos = UNSPECIFIED_POS_LAT;
        if value == "" {
            return pos;
        }
        let mut error = String::new();
        let mut dpd = DepartPosLatDefinition::DEFAULT;
        if SUMOVehicleParameter::parse_depart_pos_lat(value, element, &self.myVehicleParameter.id, &mut pos, &mut dpd, &mut error) {
            if dpd != DepartPosLatDefinition::GIVEN {
                let lane = MSStageMoving::check_depart_lane(self.myActiveRoute[0], SVC_IGNORING, departLane, &self.myVehicleParameter.id);
                if lane.is_null() {
                    throw_process_error(format!(
                        "Could not find departure lane for walk of person '{}' when interpreting departPosLat",
                        self.myVehicleParameter.id
                    ));
                }
                let usableWidth = lane.get_width() - 0.5;
                match dpd {
                    DepartPosLatDefinition::RIGHT => pos = -usableWidth / 2.0,
                    DepartPosLatDefinition::LEFT => pos = usableWidth / 2.0,
                    DepartPosLatDefinition::CENTER => pos = 0.0,
                    // @note must be randomized for every person individually when loading a personFlow
                    DepartPosLatDefinition::RANDOM | DepartPosLatDefinition::FREE | DepartPosLatDefinition::RANDOM_FREE => {
                        pos = RANDOM_POS_LAT
                    }
                    _ => {}
                }
            }
        } else {
            throw_process_error(error);
        }
        pos
    }

    /// C++: `ConstMSRoutePtr MSRouteHandler::addVehicleStopsToImplicitRoute(ConstMSRoutePtr, bool)` [private]
    /// adapt implicit route (edges derived from stops) to additional vehicle-stops
    // ConstMSRoutePtr
    // MSRouteHandler::addVehicleStopsToImplicitRoute(ConstMSRoutePtr route, bool isPermanent) {
    //     // the route was defined without edges and its current edges were
    //     // derived from route-stops.
    //     // We may need to add additional edges for the vehicle-stops
    //     assert(myVehicleParameter->wasSet(VEHPARS_ROUTE_SET));
    //     assert(route->getStops().size() > 0);
    //     ConstMSEdgeVector edges = route->getEdges();
    //     for (SUMOVehicleParameter::Stop stop : myVehicleParameter->stops) {
    //         MSEdge* stopEdge = MSEdge::dictionary(stop.edge);
    //         if (stop.index == 0) {
    //             if (edges.front() != stopEdge ||
    //                     route->getStops().front().endPos < stop.endPos) {
    //                 edges.insert(edges.begin(), stopEdge);
    //             }
    //         } else if (stop.index == STOP_INDEX_END) {
    //             if (edges.back() != stopEdge ||
    //                     route->getStops().back().endPos > stop.endPos) {
    //                 edges.push_back(stopEdge);
    //             }
    //         } else {
    //             WRITE_WARNINGF(TL("Could not merge vehicle stops for vehicle '%' into implicitly defined route '%'"), myVehicleParameter->id, route->getID());
    //         }
    //     }
    //     ConstMSRoutePtr newRoute = std::make_shared<MSRoute>("!" + myVehicleParameter->id, edges,
    //                                isPermanent, new RGBColor(route->getColor()), route->getStops());
    //     if (!MSRoute::dictionary(newRoute->getID(), newRoute)) {
    //         throw ProcessError("Could not adapt implicit route for " + std::string(isPermanent ? "flow" : "vehicle") + "  '" + myVehicleParameter->id + "'");
    //     }
    //     return newRoute;
    // }
    #[allow(unused_variables)]
    pub fn add_vehicle_stops_to_implicit_route(&mut self, route: ConstMSRoutePtr, isPermanent: bool) -> ConstMSRoutePtr {
        // the route was defined without edges and its current edges were
        // derived from route-stops.
        // We may need to add additional edges for the vehicle-stops
        let mut edges: ConstMSEdgeVector = route.get_edges().clone();
        let routeStops: &StopParVector = route.get_stops();
        for stop in self.myVehicleParameter.stops.iter() {
            let stopEdge = MSEdge::dictionary_2(&stop.edge);
            if stop.index == 0 {
                if edges.first().copied() != Some(stopEdge) || routeStops.first().unwrap().endPos < stop.endPos {
                    edges.insert(0, stopEdge);
                }
            } else if stop.index == STOP_INDEX_END {
                if edges.last().copied() != Some(stopEdge) || routeStops.last().unwrap().endPos > stop.endPos {
                    edges.push(stopEdge);
                }
            } else {
                write_warningf!(
                    "Could not merge vehicle stops for vehicle '%' into implicitly defined route '%'",
                    self.myVehicleParameter.id,
                    route.get_id()
                );
            }
        }
        let color = Ptr::from_raw(Box::into_raw(Box::new(route.get_color().clone())));
        let newRoute: ConstMSRoutePtr = Rc::new(MSRoute::new(
            &format!("!{}", self.myVehicleParameter.id),
            &edges,
            isPermanent,
            color,
            routeStops,
            -1,
            0,
        ));
        if !MSRoute::dictionary(newRoute.get_id(), newRoute.clone()) {
            throw_process_error(format!(
                "Could not adapt implicit route for {}  '{}'",
                if isPermanent { "flow" } else { "vehicle" },
                self.myVehicleParameter.id
            ));
        }
        newRoute
    }

    /// C++: `void MSRouteHandler::MSRouteHandler(const MSRouteHandler &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &MSRouteHandler) -> Self {
        // C++ copy constructor is deleted ("Invalidated"), copying is a programming error
        panic!("MSRouteHandler copy constructor is invalidated in C++ and must not be used");
    }

    /// C++: `MSRouteHandler & MSRouteHandler::operator=(const MSRouteHandler &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSRouteHandler) -> &mut MSRouteHandler {
        // C++ assignment operator is deleted ("Invalidated"), assignment is a programming error
        panic!("MSRouteHandler assignment operator is invalidated in C++ and must not be used");
    }

    /// C++: `void MSRouteHandler::addRideOrTransport(const SUMOSAXAttributes &, const SumoXMLTag)` [private]
    /// Processing of a transport
    // void
    // MSRouteHandler::addRideOrTransport(const SUMOSAXAttributes& attrs, const SumoXMLTag modeTag) {
    //     if (myVehicleParameter == nullptr) {
    //         throw ProcessError(TLF("Cannot define % stage without %.", toString(modeTag), toString(modeTag)));
    //     }
    //     try {
    //         const std::string mode = modeTag == SUMO_TAG_RIDE ? "ride" : "transport";
    //         std::string agent = "person";
    //         std::string stop = "bus stop";
    //         if (myActiveType == ObjectTypeEnum::CONTAINER) {
    //             agent = "container";
    //             stop = "container stop";
    //         }
    //
    //         if (!((myActiveType == ObjectTypeEnum::PERSON && modeTag == SUMO_TAG_RIDE) ||
    //                 (myActiveType == ObjectTypeEnum::CONTAINER && modeTag == SUMO_TAG_TRANSPORT))) {
    //             throw ProcessError("Found " + mode + " inside " + agent + " element");
    //         }
    //         const std::string aid = myVehicleParameter->id;
    //         bool ok = true;
    //         const MSEdge* from = nullptr;
    //         const std::string desc = attrs.getOpt<std::string>(SUMO_ATTR_LINES, aid.c_str(), ok, LINE_ANY);
    //         StringTokenizer st(desc);
    //         MSStoppingPlace* s = retrieveStoppingPlace(attrs, " in " + agent + " '" + aid + "'");
    //         MSEdge* to = nullptr;
    //         if (s != nullptr) {
    //             to = &s->getLane().getEdge();
    //         }
    //         double arrivalPos = attrs.getOpt<double>(SUMO_ATTR_ARRIVALPOS, aid.c_str(), ok,
    //                             s == nullptr ? std::numeric_limits<double>::infinity() : s->getEndLanePosition());
    //
    //         const SUMOVehicleParameter* startVeh = nullptr;
    //         const MSEdge* startVehFrom = nullptr;
    //         if (myActiveTransportablePlan->empty() && myVehicleParameter->departProcedure == DepartDefinition::TRIGGERED) {
    //             if (st.size() != 1) {
    //                 throw ProcessError("Triggered departure for " + agent + " '" + aid + "' requires a unique lines value.");
    //             }
    //             // agent starts
    //             MSVehicleControl& vehControl = MSNet::getInstance()->getVehicleControl();
    //             const std::string vehID = st.front();
    //             SUMOVehicle* sVeh = vehControl.getVehicle(vehID);
    //             if (sVeh == nullptr) {
    //                 if (MSNet::getInstance()->hasFlow(vehID)) {
    //                     startVeh = MSNet::getInstance()->getInsertionControl().getFlowPars(vehID);
    //                     if (startVeh != nullptr) {
    //                         ConstMSRoutePtr const route = MSRoute::dictionary(startVeh->routeid);
    //                         startVehFrom = route->getEdges().front();
    //                         // flows are inserted at the end of the time step so we
    //                         // do delay the pedestrian event by one time step
    //                         myVehicleParameter->depart = startVeh->depart + DELTA_T;
    //                         myStartTriggeredInFlow = true;
    //                     }
    //                 }
    //             } else {
    //                 startVeh = &sVeh->getParameter();
    //                 startVehFrom = sVeh->getRoute().getEdges().front();
    //                 myVehicleParameter->depart = startVeh->depart;
    //             }
    //             if (startVeh == nullptr) {
    //                 if (mySkippedVehicles.count(vehID) == 0) {
    //                     throw ProcessError("Unknown vehicle '" + vehID + "' in triggered departure for " + agent + " '" + aid + "'.");
    //                 }
    //                 // we cannot simply throw here because we need to parse the rest of the person (just to discard it)
    //                 from = MSEdge::getAllEdges().front();  // a dummy edge to keep parsing active
    //             } else if (startVeh->departProcedure == DepartDefinition::TRIGGERED) {
    //                 throw ProcessError("Cannot use triggered vehicle '" + vehID + "' in triggered departure for " + agent + " '" + aid + "'.");
    //             }
    //         }
    //
    //         if (attrs.hasAttribute(SUMO_ATTR_FROM)) {
    //             const std::string fromID = attrs.get<std::string>(SUMO_ATTR_FROM, aid.c_str(), ok);
    //             from = MSEdge::dictionary(fromID);
    //             if (from == nullptr) {
    //                 throw ProcessError("The from edge '" + fromID + "' within a " + mode + " of " + agent + " '" + aid + "' is not known.");
    //             }
    //             if (!myActiveTransportablePlan->empty() && myActiveTransportablePlan->back()->getDestination() != from) {
    //                 const bool stopWithAccess = (myActiveTransportablePlan->back()->getDestinationStop() != nullptr
    //                                              && &myActiveTransportablePlan->back()->getDestinationStop()->getLane().getEdge() == from);
    //                 const bool transferAtJunction = (from->getFromJunction() == myActiveTransportablePlan->back()->getDestination()->getFromJunction()
    //                                                  || from->getFromJunction() == myActiveTransportablePlan->back()->getDestination()->getToJunction());
    //                 if (!(stopWithAccess || transferAtJunction)) {
    //                     throw ProcessError("Disconnected plan for " + agent + " '" + aid +
    //                                        "' (edge '" + fromID + "' != edge '" + myActiveTransportablePlan->back()->getDestination()->getID() + "').");
    //                 }
    //             }
    //             if (startVeh != nullptr && startVehFrom != from) {
    //                 throw ProcessError("Disconnected plan for triggered " + agent + " '" + aid +
    //                                    "' (edge '" + fromID + "' != edge '" + startVehFrom->getID() + "').");
    //             }
    //         } else if (startVeh != nullptr) {
    //             from = startVehFrom;
    //         }
    //         if (myActiveTransportablePlan->empty()) {
    //             if (from == nullptr) {
    //                 throw ProcessError("The start edge for " + agent + " '" + aid + "' is not known.");
    //             } else {
    //                 myActiveTransportablePlan->push_back(new MSStageWaiting(
    //                         from, nullptr, -1, myVehicleParameter->depart, myVehicleParameter->departPos, "start", true));
    //             }
    //         }
    //         // given attribute may override given stopping place due access requirements
    //         if (to == nullptr || attrs.hasAttribute(SUMO_ATTR_TO)) {
    //             const std::string toID = attrs.get<std::string>(SUMO_ATTR_TO, aid.c_str(), ok);
    //             to = MSEdge::dictionary(toID);
    //             if (to == nullptr) {
    //                 throw ProcessError("The to edge '" + toID + "' within a " + mode + " of " + agent + " '" + aid + "' is not known.");
    //             }
    //         }
    //         const std::string group = attrs.getOpt<std::string>(SUMO_ATTR_GROUP, aid.c_str(), ok, OptionsCont::getOptions().getString("persontrip.default.group"));
    //         const std::string intendedVeh = attrs.getOpt<std::string>(SUMO_ATTR_INTENDED, nullptr, ok, "");
    //         const SUMOTime intendedDepart = attrs.getOptSUMOTimeReporting(SUMO_ATTR_DEPART, nullptr, ok, -1);
    //         arrivalPos = SUMOVehicleParameter::interpretEdgePos(arrivalPos, to->getLength(), SUMO_ATTR_ARRIVALPOS, agent + " '" + aid + "' takes a " + mode + " to edge '" + to->getID() + "'");
    //         MSStageDriving* stage = new MSStageDriving(from, to, s, arrivalPos, 0.0, st.getVector(), group, intendedVeh, intendedDepart);
    //         // for loading from saved state
    //         stage->setDeparted(attrs.getOptSUMOTimeReporting(SUMO_ATTR_STARTED, nullptr, ok, -1));
    //         stage->setEnded(attrs.getOptSUMOTimeReporting(SUMO_ATTR_ENDED, nullptr, ok, -1));
    //         stage->setVehicleID(attrs.getOpt<std::string>(SUMO_ATTR_VEHICLE, nullptr, ok, ""));
    //         stage->setVehicleDistance(attrs.getOpt<double>(SUMO_ATTR_ROUTELENGTH, nullptr, ok, -1));
    //         myActiveTransportablePlan->push_back(stage);
    //         myParamStack.push_back(stage);
    //     } catch (ProcessError&) {
    //         deleteActivePlanAndVehicleParameter();
    //         throw;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_ride_or_transport(&mut self, attrs: &SUMOSAXAttributes, modeTag: SumoXMLTag) {
        if self.myVehicleParameter.is_null() {
            throw_process_error(format!("Cannot define {} stage without {}.", tag_to_string(modeTag), tag_to_string(modeTag)));
        }
        // C++ try/catch(ProcessError&): deleteActivePlanAndVehicleParameter, then rethrow
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mode = if modeTag == SUMO_TAG_RIDE { "ride" } else { "transport" };
            let mut agent = "person";
            if self.myActiveType == MSRouteHandler_ObjectTypeEnum::CONTAINER {
                agent = "container";
            }
            if !((self.myActiveType == MSRouteHandler_ObjectTypeEnum::PERSON && modeTag == SUMO_TAG_RIDE)
                || (self.myActiveType == MSRouteHandler_ObjectTypeEnum::CONTAINER && modeTag == SUMO_TAG_TRANSPORT))
            {
                throw_process_error(format!("Found {} inside {} element", mode, agent));
            }
            let aid = self.myVehicleParameter.id.clone();
            let mut ok = true;
            let mut from: Ptr<MSEdge> = Ptr::null();
            let desc: String = attrs.get_opt(SUMO_ATTR_LINES, &aid, &mut ok, "ANY".to_string(), true); // LINE_ANY
            let mut st = StringTokenizer::new(&desc);
            let s = self.retrieve_stopping_place(attrs, &format!(" in {} '{}'", agent, aid), Ptr::null());
            let mut to: Ptr<MSEdge> = Ptr::null();
            if !s.is_null() {
                to = Ptr::from_mut(s.get_lane().get_edge());
            }
            let mut arrivalPos: f64 = attrs.get_opt(
                SUMO_ATTR_ARRIVALPOS,
                &aid,
                &mut ok,
                if s.is_null() { f64::INFINITY } else { s.get_end_lane_position() },
                true,
            );

            let mut startVeh: Option<Ptr<SUMOVehicleParameter>> = None;
            let mut startVehFrom: Ptr<MSEdge> = Ptr::null();
            if self.myActiveTransportablePlan.is_empty() && self.myVehicleParameter.departProcedure == DepartDefinition::TRIGGERED {
                if st.size() != 1 {
                    throw_process_error(format!(
                        "Triggered departure for {} '{}' requires a unique lines value.",
                        agent, aid
                    ));
                }
                // agent starts
                let net = MSNet::get_instance();
                let vehID = st.front().unwrap();
                let sVeh = net.m().get_vehicle_control().get_vehicle(&vehID);
                if sVeh.is_null() {
                    if net.m().has_flow(&vehID) {
                        let pars = net.m().get_insertion_control().get_flow_pars(&vehID);
                        if !pars.is_null() {
                            let route = MSRoute::dictionary_3(&pars.routeid, SUMORouteHandler::get_parsing_rng());
                            startVehFrom = route.get_edges()[0];
                            startVeh = Some(pars);
                            // flows are inserted at the end of the time step so we
                            // do delay the pedestrian event by one time step
                            self.myVehicleParameter.m().depart = pars.depart + utils::common::time::delta_t();
                            self.myStartTriggeredInFlow = true;
                        }
                    }
                } else {
                    startVeh = Some(Ptr::new(sVeh.get_parameter()));
                    startVehFrom = sVeh.get_route().get_edges()[0];
                    self.myVehicleParameter.m().depart = sVeh.get_parameter().depart;
                }
                match startVeh {
                    None => {
                        // shortcut: mySkippedVehicles is i32 in the skeleton, vehicles skipped earlier cannot be recognised here, so this always reports the unknown vehicle
                        // we cannot simply throw here because we need to parse the rest of the person (just to discard it)
                        // C++ uses a dummy edge (MSEdge::getAllEdges().front()) to keep parsing active in that case
                        throw_process_error(format!(
                            "Unknown vehicle '{}' in triggered departure for {} '{}'.",
                            vehID, agent, aid
                        ));
                    }
                    Some(sv) => {
                        if sv.departProcedure == DepartDefinition::TRIGGERED {
                            throw_process_error(format!(
                                "Cannot use triggered vehicle '{}' in triggered departure for {} '{}'.",
                                vehID, agent, aid
                            ));
                        }
                    }
                }
            }

            if attrs.has_attribute(SUMO_ATTR_FROM) {
                let fromID: String = attrs.get(SUMO_ATTR_FROM, &aid, &mut ok, true);
                from = MSEdge::dictionary_2(&fromID);
                if from.is_null() {
                    throw_process_error(format!(
                        "The from edge '{}' within a {} of {} '{}' is not known.",
                        fromID, mode, agent, aid
                    ));
                }
                if !self.myActiveTransportablePlan.is_empty() && self.myActiveTransportablePlan.last().unwrap().get_destination() != from {
                    let back = *self.myActiveTransportablePlan.last().unwrap();
                    let stopWithAccess = !back.get_destination_stop().is_null()
                        && Ptr::from_mut(back.get_destination_stop().get_lane().get_edge()) == from;
                    let transferAtJunction = from.get_from_junction() == back.get_destination().get_from_junction()
                        || from.get_from_junction() == back.get_destination().get_to_junction();
                    if !(stopWithAccess || transferAtJunction) {
                        throw_process_error(format!(
                            "Disconnected plan for {} '{}' (edge '{}' != edge '{}').",
                            agent,
                            aid,
                            fromID,
                            back.get_destination().get_id()
                        ));
                    }
                }
                if let Some(_) = startVeh {
                    if startVehFrom != from {
                        throw_process_error(format!(
                            "Disconnected plan for triggered {} '{}' (edge '{}' != edge '{}').",
                            agent,
                            aid,
                            fromID,
                            startVehFrom.get_id()
                        ));
                    }
                }
            } else if startVeh.is_some() {
                from = startVehFrom;
            }
            if self.myActiveTransportablePlan.is_empty() {
                if from.is_null() {
                    throw_process_error(format!("The start edge for {} '{}' is not known.", agent, aid));
                } else {
                    let stage = MSStageWaiting::new(
                        from,
                        Ptr::null(),
                        -1,
                        self.myVehicleParameter.depart,
                        self.myVehicleParameter.departPos,
                        "start",
                        true,
                        -1,
                    );
                    self.myActiveTransportablePlan.m().push(stage_waiting_ptr(stage));
                }
            }
            // given attribute may override given stopping place due access requirements
            if to.is_null() || attrs.has_attribute(SUMO_ATTR_TO) {
                let toID: String = attrs.get(SUMO_ATTR_TO, &aid, &mut ok, true);
                to = MSEdge::dictionary_2(&toID);
                if to.is_null() {
                    throw_process_error(format!(
                        "The to edge '{}' within a {} of {} '{}' is not known.",
                        toID, mode, agent, aid
                    ));
                }
            }
            let group: String = attrs.get_opt(SUMO_ATTR_GROUP, &aid, &mut ok, utils::options::options_cont::get_string("persontrip.default.group"), true);
            let intendedVeh: String = attrs.get_opt(SUMO_ATTR_INTENDED, "", &mut ok, String::new(), true);
            let intendedDepart: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_DEPART, "", &mut ok, -1, true);
            arrivalPos = SUMOVehicleParameter::interpret_edge_pos(
                arrivalPos,
                to.get_length(),
                SUMO_ATTR_ARRIVALPOS,
                &format!("{} '{}' takes a {} to edge '{}'", agent, aid, mode, to.get_id()),
                false,
            );
            let lines = st.get_vector();
            let mut stage = MSStageDriving::new(from, to, s, arrivalPos, 0.0, &lines, &group, &intendedVeh, intendedDepart);
            // for loading from saved state
            stage.set_departed(attrs.get_opt_sumo_time_reporting(SUMO_ATTR_STARTED, "", &mut ok, -1, true));
            stage.set_ended(attrs.get_opt_sumo_time_reporting(SUMO_ATTR_ENDED, "", &mut ok, -1, true));
            stage.set_vehicle_id(attrs.get_opt(SUMO_ATTR_VEHICLE, "", &mut ok, String::new(), true));
            stage.set_vehicle_distance(attrs.get_opt(SUMO_ATTR_ROUTELENGTH, "", &mut ok, -1.0, true));
            let raw = Box::into_raw(Box::new(stage));
            self.myActiveTransportablePlan.m().push(Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) }));
            // shortcut: myParamStack is an i32 in the skeleton (C++ vector of MSStage*), the stage is not pushed onto it until SUMORouteHandler is fixed
        }));
        if let Err(payload) = result {
            self.delete_active_plan_and_vehicle_parameter();
            std::panic::resume_unwind(payload);
        }
    }

}


impl utils::xml::generic_sax_handler::SaxHandler for MSRouteHandler {
    fn handler(&mut self) -> &mut utils::xml::generic_sax_handler::GenericSAXHandler { self.base.handler() }
}
