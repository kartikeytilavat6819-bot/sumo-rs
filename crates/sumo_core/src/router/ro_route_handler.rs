//! Port of router/RORouteHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::sumo_vehicle_class::{SVC_BICYCLE, SVC_BUS, SVC_TAXI};
// shortcut: LINE_ANY belongs to SUMOVehicleParameter.h, not ported yet, move it there
const LINE_ANY: &str = "ANY";
/// C++ `SUMOXMLDefinitions::getEdgeIDFromLane`: the lane id without its "_<index>" suffix
fn edge_id_from_lane(lane: &str) -> String {
    match lane.rfind('_') {
        Some(p) => lane[..p].to_string(),
        None => lane.to_string(),
    }
}
use utils::common::sumo_vehicle_class::{DEFAULT_VTYPE_ID, SVC_IGNORING, SVC_PEDESTRIAN, SVC_PASSENGER};

// shortcut: VEHPARS_* and JUNCTION_TAZ_MISSING_HELP belong to SUMOVehicleParameter.h / MapMatcher.h, not ported yet, move them there
const VEHPARS_FROM_TAZ_SET: i64 = 2 << 15;
const VEHPARS_TO_TAZ_SET: i64 = 2 << 16;
const JUNCTION_TAZ_MISSING_HELP: &str = "\nSet option '--junction-taz' or load a TAZ-file";

/// C++ `delete p` for a pointer created with Box::into_raw; deleting nullptr is a no-op
fn delete_ptr<T>(p: Ptr<T>) {
    if !p.is_null() {
        unsafe { drop(Box::from_raw(p.as_raw())) };
    }
}

/// C++ `throw ProcessError(msg)`; caught by the catch_unwind emulation in myStartElement
fn throw_process_error(msg: String) -> ! {
    std::panic::panic_any(ProcessError::new(msg))
}

/// MsgHandler::getWarningInstance() / getErrorInstance()
// shortcut: the instances are kept in a locked static, the pointer stays valid because they are only dropped by cleanup_on_end
fn msg_instance(warning: bool) -> Ptr<MsgHandler> {
    let t = if warning { MsgType::Warning } else { MsgType::Error };
    Ptr::from_raw(utils::common::msg_handler::with_instance(t, |h| h as *mut MsgHandler))
}

/// C++ `toString(SumoXMLTag)`: the canonical name of a tag
// shortcut: scans the TAGS table instead of the SumoXMLTags bijection, upgrade when sumo_xml_definitions exposes toString
fn tag_to_string(tag: SumoXMLTag) -> String {
    utils::xml::sumo_xml_definitions::TAGS.iter().find(|(_, v)| *v == tag).map(|(s, _)| s.to_string()).unwrap_or_default()
}

/// C++ MapMatcher::parseGeoEdges (protected member of the MapMatcher base class)
// shortcut: parseGeoEdges belongs to the unported MapMatcher base class, geo-coordinate route parsing panics until it is ported
#[allow(unused_variables)]
fn parse_geo_edges(pv: PositionVector, geo: bool, vClass: SUMOVehicleClass, into: &mut ConstROEdgeVector, rid: &str, isFrom: bool, ok: &mut bool) {
    panic!("parseGeoEdges needs the unported MapMatcher base class");
}

/// Parser and container for routes during their loading
/// C++ class `RORouteHandler` (router/RORouteHandler.h)
#[repr(C)]
pub struct RORouteHandler {
    /// C++ base class `SUMORouteHandler`
    pub base: SUMORouteHandler,
    /// C++ base class `MapMatcher<ROEdge, ROLane, RONode>`
    pub base2: Unmapped /* MapMatcher<ROEdge, ROLane, RONode> */,
    /// The current route
    pub myNet: Ptr<RONet>,
    /// The current route
    pub myActiveRoute: ConstROEdgeVector,
    /// number of repetitions of the active route
    pub myActiveRouteRepeat: i32,
    pub myActiveRoutePeriod: SUMOTime,
    /// The plan of the current person
    pub myActivePlan: Ptr<Vec<Ptr<ROPerson_PlanItem>>>,
    /// The plan of the current container
    pub myActiveContainerPlan: Ptr<OutputDevice>,
    /// The number of stages in myActiveContainerPlan
    pub myActiveContainerPlanSize: i32,
    /// Information whether routes shall be repaired
    pub myTryRepair: bool,
    /// Information whether the "to" attribute is mandatory
    pub myEmptyDestinationsAllowed: bool,
    /// Depending on the "ignore-errors" option different outputs are used
    pub myErrorOutput: Ptr<MsgHandler>,
    /// The begin time
    pub myBegin: SUMOTime,
    /// whether to keep the vtype distribution in output
    pub myKeepVTypeDist: bool,
    /// whether input is read all at once (no sorting check is necessary)
    pub myUnsortedInput: bool,
    /// whether flows shall not be expanded
    pub myWriteFlows: bool,
    /// The currently parsed distribution of vehicle types (probability->vehicle type)
    pub myCurrentVTypeDistribution: Ptr<RandomDistributor<Ptr<SUMOVTypeParameter>>>,
    /// The id of the currently parsed vehicle type distribution
    pub myCurrentVTypeDistributionID: String,
    /// The currently parsed route alternatives
    pub myCurrentAlternatives: Ptr<RORouteDef>,
    /// cached options
    pub myUseTaz: bool,
    pub myWriteJunctions: bool,
    /// C++ std::set<std::string> mySkippedVehicles (skipped vehicle ids)
    pub mySkippedVehicleIDs: BTreeSet<String>,
}

impl std::ops::Deref for RORouteHandler {
    type Target = SUMORouteHandler;
    fn deref(&self) -> &SUMORouteHandler {
        &self.base
    }
}
impl std::ops::DerefMut for RORouteHandler {
    fn deref_mut(&mut self) -> &mut SUMORouteHandler {
        &mut self.base
    }
}

impl RORouteHandler {
    /// C++: `void RORouteHandler::RORouteHandler(RONet &, const std::string &, const bool, const bool, const bool, const bool)`
    /// standard constructor
    // RORouteHandler::RORouteHandler(RONet& net, const std::string& file,
    //                                const bool tryRepair,
    //                                const bool emptyDestinationsAllowed,
    //                                const bool ignoreErrors,
    //                                const bool checkSchema) :
    //     SUMORouteHandler(file, checkSchema ? "routes" : "", true),
    //     MapMatcher(OptionsCont::getOptions().getBool("mapmatch.junctions"),
    //                OptionsCont::getOptions().getBool("mapmatch.taz"),
    //                OptionsCont::getOptions().getFloat("mapmatch.distance"),
    //                ignoreErrors ? MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance()),
    //     myNet(net),
    //     myActiveRouteRepeat(0),
    //     myActiveRoutePeriod(0),
    //     myActivePlan(nullptr),
    //     myActiveContainerPlan(nullptr),
    //     myActiveContainerPlanSize(0),
    //     myTryRepair(tryRepair),
    //     myEmptyDestinationsAllowed(emptyDestinationsAllowed),
    //     myErrorOutput(ignoreErrors ? MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance()),
    //     myBegin(string2time(OptionsCont::getOptions().getString("begin"))),
    //     myKeepVTypeDist(OptionsCont::getOptions().getBool("keep-vtype-distributions")),
    //     myUnsortedInput(OptionsCont::getOptions().exists("unsorted-input") && OptionsCont::getOptions().getBool("unsorted-input")),
    //     myWriteFlows(OptionsCont::getOptions().exists("keep-flows") && OptionsCont::getOptions().getBool("keep-flows")),
    //     myCurrentVTypeDistribution(nullptr),
    //     myCurrentAlternatives(nullptr),
    //     myUseTaz(OptionsCont::getOptions().getBool("with-taz")),
    //     myWriteJunctions(OptionsCont::getOptions().exists("write-trips")
    //                      && OptionsCont::getOptions().getBool("write-trips")
    //                      && OptionsCont::getOptions().getBool("write-trips.junctions")) {
    //     myActiveRoute.reserve(100);
    // }
    #[allow(unused_variables)]
    pub fn new(net: &mut RONet, file: &str, tryRepair: bool, emptyDestinationsAllowed: bool, ignoreErrors: bool, checkSchema: bool) -> Self {
        let mut myActiveRoute: ConstROEdgeVector = Vec::new();
        myActiveRoute.reserve(100);
        RORouteHandler {
            base: SUMORouteHandler::new(file, if checkSchema { "routes" } else { "" }, true),
            base2: (),
            myNet: Ptr::from_mut(net),
            myActiveRoute,
            myActiveRouteRepeat: 0,
            myActiveRoutePeriod: 0,
            myActivePlan: Ptr::null(),
            myActiveContainerPlan: Ptr::null(),
            myActiveContainerPlanSize: 0,
            myTryRepair: tryRepair,
            myEmptyDestinationsAllowed: emptyDestinationsAllowed,
            myErrorOutput: msg_instance(ignoreErrors),
            myBegin: utils::common::time::string2time(&utils::options::options_cont::get_string("begin")).unwrap_or_else(|e| std::panic::panic_any(e)),
            myKeepVTypeDist: utils::options::options_cont::get_bool("keep-vtype-distributions"),
            myUnsortedInput: utils::options::options_cont::exists("unsorted-input") && utils::options::options_cont::get_bool("unsorted-input"),
            myWriteFlows: utils::options::options_cont::exists("keep-flows") && utils::options::options_cont::get_bool("keep-flows"),
            myCurrentVTypeDistribution: Ptr::null(),
            myCurrentVTypeDistributionID: String::new(),
            myCurrentAlternatives: Ptr::null(),
            myUseTaz: utils::options::options_cont::get_bool("with-taz"),
            myWriteJunctions: utils::options::options_cont::exists("write-trips")
                && utils::options::options_cont::get_bool("write-trips")
                && utils::options::options_cont::get_bool("write-trips.junctions"),
        mySkippedVehicleIDs: BTreeSet::new(),
        }
        // shortcut: MapMatcher base (base2) is Unmapped, so its mapmatch.* option reads are not stored; parseFromViaTo reads mapmatch.taz when needed
    }

    /// C++: `void RORouteHandler::~RORouteHandler() noexcept` [virtual]
    /// standard destructor
    // RORouteHandler::~RORouteHandler() {
    //     delete myCurrentAlternatives;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body only deletes myCurrentAlternatives (its destructor is empty)
        delete_ptr(self.myCurrentAlternatives);
        self.myCurrentAlternatives = Ptr::null();
    }

    /// C++: `bool RORouteHandler::checkLastDepart()` [virtual]
    /// Checks whether the route file is sorted by departure time if needed
    // bool
    // RORouteHandler::checkLastDepart() {
    //     if (!myUnsortedInput) {
    //         return SUMORouteHandler::checkLastDepart();
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn check_last_depart(&mut self) -> bool {
        if !self.myUnsortedInput {
            return self.base.check_last_depart();
        }
        true
    }

    /// C++: `void RORouteHandler::deleteActivePlanAndVehicleParameter()` [protected]
    // void
    // RORouteHandler::deleteActivePlanAndVehicleParameter() {
    //     if (myActivePlan != nullptr) {
    //         for (ROPerson::PlanItem* const it : *myActivePlan) {
    //             delete it;
    //         }
    //         delete myActivePlan;
    //         myActivePlan = nullptr;
    //     }
    //     delete myActiveContainerPlan;
    //     myActiveContainerPlan = nullptr;
    //     delete myVehicleParameter;
    //     myVehicleParameter = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn delete_active_plan_and_vehicle_parameter(&mut self) {
        if !self.myActivePlan.is_null() {
            for it in self.myActivePlan.m().iter() {
                // shortcut: PlanItem subclasses are freed through the base type pointer, same layout as the C++ base-pointer delete
                delete_ptr(*it);
            }
            delete_ptr(self.myActivePlan);
            self.myActivePlan = Ptr::null();
        }
        delete_ptr(self.myActiveContainerPlan);
        self.myActiveContainerPlan = Ptr::null();
        delete_ptr(self.myVehicleParameter);
        self.myVehicleParameter = Ptr::null();
    }

    /// C++: `void RORouteHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // RORouteHandler::myStartElement(int element,
    //                                const SUMOSAXAttributes& attrs) {
    //     try {
    //         if (myActivePlan != nullptr && myActivePlan->empty() && myVehicleParameter->departProcedure == DepartDefinition::TRIGGERED && element != SUMO_TAG_RIDE && element != SUMO_TAG_PARAM) {
    //             throw ProcessError(TLF("Triggered departure for person '%' requires starting with a ride.", myVehicleParameter->id));
    //         } else if (myActiveContainerPlan != nullptr && myActiveContainerPlanSize == 0 && myVehicleParameter->departProcedure == DepartDefinition::TRIGGERED && element != SUMO_TAG_TRANSPORT && element != SUMO_TAG_PARAM) {
    //             throw ProcessError(TLF("Triggered departure for container '%' requires starting with a transport.", myVehicleParameter->id));
    //         }
    //         SUMORouteHandler::myStartElement(element, attrs);
    //         bool ok = true;
    //         switch (element) {
    //             case SUMO_TAG_PERSON:
    //             case SUMO_TAG_PERSONFLOW: {
    //                 myActivePlan = new std::vector<ROPerson::PlanItem*>();
    //                 break;
    //             }
    //             case SUMO_TAG_RIDE:
    //                 break; // handled in addRide, called from SUMORouteHandler::myStartElement
    //             case SUMO_TAG_CONTAINER:
    //             case SUMO_TAG_CONTAINERFLOW:
    //                 myActiveContainerPlan = new OutputDevice_String(1);
    //                 myActiveContainerPlanSize = 0;
    //                 myActiveContainerPlan->openTag((SumoXMLTag)element);
    //                 (*myActiveContainerPlan) << attrs;
    //                 break;
    //             case SUMO_TAG_TRANSPORT:
    //             case SUMO_TAG_TRANSHIP:
    //                 if (myActiveContainerPlan == nullptr) {
    //                     throw ProcessError(TLF("Found % outside container element", toString((SumoXMLTag)element)));
    //                 }
    //                 // copy container elements
    //                 myActiveContainerPlan->openTag((SumoXMLTag)element);
    //                 (*myActiveContainerPlan) << attrs;
    //                 myActiveContainerPlan->closeTag();
    //                 myActiveContainerPlanSize++;
    //                 break;
    //             case SUMO_TAG_FLOW:
    //                 myActiveRouteProbability = DEFAULT_VEH_PROB;
    //                 parseFromViaTo((SumoXMLTag)element, attrs, ok);
    //                 break;
    //             case SUMO_TAG_TRIP:
    //                 myActiveRouteProbability = DEFAULT_VEH_PROB;
    //                 parseFromViaTo((SumoXMLTag)element, attrs, ok);
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
        // C++ throw ProcessError is panic_any(ProcessError); the try/catch is catch_unwind + resume_unwind
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if !self.myActivePlan.is_null()
                && self.myActivePlan.is_empty()
                && self.myVehicleParameter.departProcedure == DepartDefinition::TRIGGERED
                && element != SUMO_TAG_RIDE
                && element != SUMO_TAG_PARAM
            {
                throw_process_error(format!("Triggered departure for person '{}' requires starting with a ride.", self.myVehicleParameter.id));
            } else if !self.myActiveContainerPlan.is_null()
                && self.myActiveContainerPlanSize == 0
                && self.myVehicleParameter.departProcedure == DepartDefinition::TRIGGERED
                && element != SUMO_TAG_TRANSPORT
                && element != SUMO_TAG_PARAM
            {
                throw_process_error(format!("Triggered departure for container '{}' requires starting with a transport.", self.myVehicleParameter.id));
            }
            self.base.my_start_element(element, attrs);
            let mut ok = true;
            match element {
                SUMO_TAG_PERSON | SUMO_TAG_PERSONFLOW => {
                    self.myActivePlan = Ptr::from_raw(Box::into_raw(Box::new(Vec::new())));
                }
                SUMO_TAG_RIDE => {
                    // handled in addRide, called from SUMORouteHandler::myStartElement
                }
                SUMO_TAG_CONTAINER | SUMO_TAG_CONTAINERFLOW => {
                    self.myActiveContainerPlan = Ptr::from_raw(Box::into_raw(Box::new(OutputDevice::new_string(1))));
                    self.myActiveContainerPlanSize = 0;
                    self.myActiveContainerPlan.m().open_tag_id(element);
                    self.myActiveContainerPlan.m().write(&attrs.serialize());
                }
                SUMO_TAG_TRANSPORT | SUMO_TAG_TRANSHIP => {
                    if self.myActiveContainerPlan.is_null() {
                        throw_process_error(format!("Found {} outside container element", tag_to_string(element)));
                    }
                    // copy container elements
                    self.myActiveContainerPlan.m().open_tag_id(element);
                    self.myActiveContainerPlan.m().write(&attrs.serialize());
                    self.myActiveContainerPlan.m().close_tag("");
                    self.myActiveContainerPlanSize += 1;
                }
                SUMO_TAG_FLOW | SUMO_TAG_TRIP => {
                    self.myActiveRouteProbability = DEFAULT_VEH_PROB;
                    self.parse_from_via_to(element, attrs, &mut ok);
                }
                _ => {}
            }
        }));
        if let Err(payload) = result {
            self.delete_active_plan_and_vehicle_parameter();
            std::panic::resume_unwind(payload);
        }
    }

    /// C++: `void RORouteHandler::parseFromViaTo(SumoXMLTag, const SUMOSAXAttributes &, bool &)` [protected]
    /// Called for parsing from and to and the corresponding taz attributes
    // void
    // RORouteHandler::parseFromViaTo(SumoXMLTag tag, const SUMOSAXAttributes& attrs, bool& ok) {
    //     const std::string element = toString(tag);
    //     myActiveRoute.clear();
    //     bool useTaz = myUseTaz;
    //     if (myUseTaz && !myVehicleParameter->wasSet(VEHPARS_FROM_TAZ_SET) && !myVehicleParameter->wasSet(VEHPARS_TO_TAZ_SET)) {
    //         WRITE_WARNINGF(TL("Taz usage was requested but no taz present in % '%'!"), element, myVehicleParameter->id);
    //         useTaz = false;
    //     }
    //     SUMOVehicleClass vClass = SVC_PASSENGER;
    //     if (!myNet.getVTypeDistribution(myVehicleParameter->vtypeid)) {
    //         SUMOVTypeParameter* type = myNet.getVehicleTypeSecure(myVehicleParameter->vtypeid);
    //         if (type != nullptr) {
    //             vClass = type->vehicleClass;
    //         }
    //     }
    //     // from-attributes
    //     const std::string rid = "for " + element + " '" + myVehicleParameter->id + "'";
    //     if ((useTaz || (!attrs.hasAttribute(SUMO_ATTR_FROM) && !attrs.hasAttribute(SUMO_ATTR_FROMXY) && !attrs.hasAttribute(SUMO_ATTR_FROMLONLAT))) &&
    //             (attrs.hasAttribute(SUMO_ATTR_FROM_TAZ) || attrs.hasAttribute(SUMO_ATTR_FROM_JUNCTION))) {
    //         const bool useJunction = attrs.hasAttribute(SUMO_ATTR_FROM_JUNCTION);
    //         const std::string tazType = useJunction ? "junction" : "taz";
    //         const std::string tazID = attrs.get<std::string>(useJunction ? SUMO_ATTR_FROM_JUNCTION : SUMO_ATTR_FROM_TAZ, myVehicleParameter->id.c_str(), ok, true);
    //         const ROEdge* fromTaz = myNet.getEdge(tazID + "-source");
    //         if (fromTaz == nullptr) {
    //             myErrorOutput->inform("Source " + tazType + " '" + tazID + "' not known for " + element + " '" + myVehicleParameter->id + "'!"
    //                                   + (useJunction ? JUNCTION_TAZ_MISSING_HELP : ""));
    //             ok = false;
    //         } else if (fromTaz->getNumSuccessors() == 0 && tag != SUMO_TAG_PERSON) {
    //             myErrorOutput->inform("Source " + tazType + " '" + tazID + "' has no outgoing edges for " + element + " '" + myVehicleParameter->id + "'!");
    //             ok = false;
    //         } else {
    //             myActiveRoute.push_back(fromTaz);
    //             if (useJunction && tag != SUMO_TAG_PERSON && !myWriteJunctions) {
    //                 myVehicleParameter->fromTaz = tazID;
    //                 myVehicleParameter->parametersSet |= VEHPARS_FROM_TAZ_SET;
    //             }
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
    //         parseEdges(attrs.getOpt<std::string>(SUMO_ATTR_FROM, myVehicleParameter->id.c_str(), ok), myActiveRoute, rid, ok);
    //     }
    //     if (!attrs.hasAttribute(SUMO_ATTR_VIA) && !attrs.hasAttribute(SUMO_ATTR_VIALONLAT) && !attrs.hasAttribute(SUMO_ATTR_VIAXY)) {
    //         myInsertStopEdgesAt = (int)myActiveRoute.size();
    //     }
    //
    //     // via-attributes
    //     ConstROEdgeVector viaEdges;
    //     if (attrs.hasAttribute(SUMO_ATTR_VIAXY)) {
    //         parseGeoEdges(attrs.get<PositionVector>(SUMO_ATTR_VIAXY, myVehicleParameter->id.c_str(), ok), false, vClass, viaEdges, rid, false, ok);
    //     } else if (attrs.hasAttribute(SUMO_ATTR_VIALONLAT)) {
    //         parseGeoEdges(attrs.get<PositionVector>(SUMO_ATTR_VIALONLAT, myVehicleParameter->id.c_str(), ok), true, vClass, viaEdges, rid, false, ok);
    //     } else if (attrs.hasAttribute(SUMO_ATTR_VIAJUNCTIONS)) {
    //         for (std::string junctionID : attrs.get<std::vector<std::string> >(SUMO_ATTR_VIAJUNCTIONS, myVehicleParameter->id.c_str(), ok)) {
    //             const ROEdge* viaSink = myNet.getEdge(junctionID + "-sink");
    //             if (viaSink == nullptr) {
    //                 myErrorOutput->inform("Junction-taz '" + junctionID + "' not found." + JUNCTION_TAZ_MISSING_HELP);
    //                 ok = false;
    //             } else {
    //                 viaEdges.push_back(viaSink);
    //             }
    //         }
    //     } else {
    //         parseEdges(attrs.getOpt<std::string>(SUMO_ATTR_VIA, myVehicleParameter->id.c_str(), ok, "", true), viaEdges, rid, ok);
    //     }
    //     for (const ROEdge* e : viaEdges) {
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
    //         const ROEdge* toTaz = myNet.getEdge(tazID + "-sink");
    //         if (toTaz == nullptr) {
    //             myErrorOutput->inform("Sink " + tazType + " '" + tazID + "' not known for " + element + " '" + myVehicleParameter->id + "'!"
    //                                   + (useJunction ? JUNCTION_TAZ_MISSING_HELP : ""));
    //             ok = false;
    //         } else if (toTaz->getNumPredecessors() == 0 && tag != SUMO_TAG_PERSON) {
    //             myErrorOutput->inform("Sink " + tazType + " '" + tazID + "' has no incoming edges for " + element + " '" + myVehicleParameter->id + "'!");
    //             ok = false;
    //         } else {
    //             myActiveRoute.push_back(toTaz);
    //             if (useJunction && tag != SUMO_TAG_PERSON && !myWriteJunctions) {
    //                 myVehicleParameter->toTaz = tazID;
    //                 myVehicleParameter->parametersSet |= VEHPARS_TO_TAZ_SET;
    //             }
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
    //         parseEdges(attrs.getOpt<std::string>(SUMO_ATTR_TO, myVehicleParameter->id.c_str(), ok, "", true), myActiveRoute, rid, ok);
    //     }
    //     myActiveRouteID = "!" + myVehicleParameter->id;
    //     if (myVehicleParameter->routeid == "") {
    //         myVehicleParameter->routeid = myActiveRouteID;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_from_via_to(&mut self, tag: SumoXMLTag, attrs: &SUMOSAXAttributes, ok: &mut bool) {
        let element = tag_to_string(tag);
        self.myActiveRoute.clear();
        let mut useTaz = self.myUseTaz;
        if self.myUseTaz && !self.myVehicleParameter.was_set(VEHPARS_FROM_TAZ_SET) && !self.myVehicleParameter.was_set(VEHPARS_TO_TAZ_SET) {
            write_warningf!("Taz usage was requested but no taz present in % '%'!", element, self.myVehicleParameter.id);
            useTaz = false;
        }
        // MapMatcher member, copied from the mapmatch.taz option
        let myMapMatchTAZ = utils::options::options_cont::get_bool("mapmatch.taz");
        let mut vClass: SUMOVehicleClass = SVC_PASSENGER;
        let net = self.myNet;
        if net.m().get_v_type_distribution(&self.myVehicleParameter.vtypeid).is_null() {
            let type_ = net.m().get_vehicle_type_secure(&self.myVehicleParameter.vtypeid);
            if !type_.is_null() {
                vClass = type_.vehicleClass;
            }
        }
        // from-attributes
        let rid = format!("for {} '{}'", element, self.myVehicleParameter.id);
        if (useTaz || (!attrs.has_attribute(SUMO_ATTR_FROM) && !attrs.has_attribute(SUMO_ATTR_FROMXY) && !attrs.has_attribute(SUMO_ATTR_FROMLONLAT)))
            && (attrs.has_attribute(SUMO_ATTR_FROM_TAZ) || attrs.has_attribute(SUMO_ATTR_FROM_JUNCTION))
        {
            let useJunction = attrs.has_attribute(SUMO_ATTR_FROM_JUNCTION);
            let tazType = if useJunction { "junction" } else { "taz" };
            let tazID: String = attrs.get(if useJunction { SUMO_ATTR_FROM_JUNCTION } else { SUMO_ATTR_FROM_TAZ }, &self.myVehicleParameter.id, ok, true);
            let fromTaz = self.myNet.get_edge(&format!("{}-source", tazID));
            if fromTaz.is_null() {
                let msg = format!(
                    "Source {} '{}' not known for {} '{}'!{}",
                    tazType,
                    tazID,
                    element,
                    self.myVehicleParameter.id,
                    if useJunction { JUNCTION_TAZ_MISSING_HELP } else { "" }
                );
                self.myErrorOutput.m().inform(&msg, true);
                *ok = false;
            } else if fromTaz.get_num_successors() == 0 && tag != SUMO_TAG_PERSON {
                let msg = format!("Source {} '{}' has no outgoing edges for {} '{}'!", tazType, tazID, element, self.myVehicleParameter.id);
                self.myErrorOutput.m().inform(&msg, true);
                *ok = false;
            } else {
                self.myActiveRoute.push(fromTaz);
                if useJunction && tag != SUMO_TAG_PERSON && !self.myWriteJunctions {
                    self.myVehicleParameter.m().fromTaz = tazID;
                    self.myVehicleParameter.m().parametersSet |= VEHPARS_FROM_TAZ_SET;
                }
            }
        } else if attrs.has_attribute(SUMO_ATTR_FROMXY) {
            let pv: PositionVector = attrs.get(SUMO_ATTR_FROMXY, &self.myVehicleParameter.id, ok, true);
            parse_geo_edges(pv, false, vClass, &mut self.myActiveRoute, &rid, true, ok);
            if myMapMatchTAZ && *ok {
                self.myVehicleParameter.m().fromTaz = self.myActiveRoute.last().unwrap().get_id().to_string();
                self.myVehicleParameter.m().parametersSet |= VEHPARS_FROM_TAZ_SET;
            }
        } else if attrs.has_attribute(SUMO_ATTR_FROMLONLAT) {
            let pv: PositionVector = attrs.get(SUMO_ATTR_FROMLONLAT, &self.myVehicleParameter.id, ok, true);
            parse_geo_edges(pv, true, vClass, &mut self.myActiveRoute, &rid, true, ok);
            if myMapMatchTAZ && *ok {
                self.myVehicleParameter.m().fromTaz = self.myActiveRoute.last().unwrap().get_id().to_string();
                self.myVehicleParameter.m().parametersSet |= VEHPARS_FROM_TAZ_SET;
            }
        } else {
            let desc: String = attrs.get_opt(SUMO_ATTR_FROM, &self.myVehicleParameter.id, ok, String::new(), true);
            let mut route = std::mem::take(&mut self.myActiveRoute);
            self.parse_edges(&desc, &mut route, &rid, ok);
            self.myActiveRoute = route;
        }
        if !attrs.has_attribute(SUMO_ATTR_VIA) && !attrs.has_attribute(SUMO_ATTR_VIALONLAT) && !attrs.has_attribute(SUMO_ATTR_VIAXY) {
            self.myInsertStopEdgesAt = self.myActiveRoute.len() as i32;
        }

        // via-attributes
        let mut viaEdges: ConstROEdgeVector = Vec::new();
        if attrs.has_attribute(SUMO_ATTR_VIAXY) {
            let pv: PositionVector = attrs.get(SUMO_ATTR_VIAXY, &self.myVehicleParameter.id, ok, true);
            parse_geo_edges(pv, false, vClass, &mut viaEdges, &rid, false, ok);
        } else if attrs.has_attribute(SUMO_ATTR_VIALONLAT) {
            let pv: PositionVector = attrs.get(SUMO_ATTR_VIALONLAT, &self.myVehicleParameter.id, ok, true);
            parse_geo_edges(pv, true, vClass, &mut viaEdges, &rid, false, ok);
        } else if attrs.has_attribute(SUMO_ATTR_VIAJUNCTIONS) {
            let junctionIDs: Vec<String> = attrs.get(SUMO_ATTR_VIAJUNCTIONS, &self.myVehicleParameter.id, ok, true);
            for junctionID in junctionIDs {
                let viaSink = self.myNet.get_edge(&format!("{}-sink", junctionID));
                if viaSink.is_null() {
                    let msg = format!("Junction-taz '{}' not found.{}", junctionID, JUNCTION_TAZ_MISSING_HELP);
                    self.myErrorOutput.m().inform(&msg, true);
                    *ok = false;
                } else {
                    viaEdges.push(viaSink);
                }
            }
        } else {
            let desc: String = attrs.get_opt(SUMO_ATTR_VIA, &self.myVehicleParameter.id, ok, String::new(), true);
            self.parse_edges(&desc, &mut viaEdges, &rid, ok);
        }
        for e in viaEdges.iter() {
            self.myActiveRoute.push(*e);
            self.myVehicleParameter.m().via.push(e.get_id().to_string());
        }

        // to-attributes
        if (useTaz || (!attrs.has_attribute(SUMO_ATTR_TO) && !attrs.has_attribute(SUMO_ATTR_TOXY) && !attrs.has_attribute(SUMO_ATTR_TOLONLAT)))
            && (attrs.has_attribute(SUMO_ATTR_TO_TAZ) || attrs.has_attribute(SUMO_ATTR_TO_JUNCTION))
        {
            let useJunction = attrs.has_attribute(SUMO_ATTR_TO_JUNCTION);
            let tazType = if useJunction { "junction" } else { "taz" };
            let tazID: String = attrs.get(if useJunction { SUMO_ATTR_TO_JUNCTION } else { SUMO_ATTR_TO_TAZ }, &self.myVehicleParameter.id, ok, true);
            let toTaz = self.myNet.get_edge(&format!("{}-sink", tazID));
            if toTaz.is_null() {
                let msg = format!(
                    "Sink {} '{}' not known for {} '{}'!{}",
                    tazType,
                    tazID,
                    element,
                    self.myVehicleParameter.id,
                    if useJunction { JUNCTION_TAZ_MISSING_HELP } else { "" }
                );
                self.myErrorOutput.m().inform(&msg, true);
                *ok = false;
            } else if toTaz.get_num_predecessors() == 0 && tag != SUMO_TAG_PERSON {
                let msg = format!("Sink {} '{}' has no incoming edges for {} '{}'!", tazType, tazID, element, self.myVehicleParameter.id);
                self.myErrorOutput.m().inform(&msg, true);
                *ok = false;
            } else {
                self.myActiveRoute.push(toTaz);
                if useJunction && tag != SUMO_TAG_PERSON && !self.myWriteJunctions {
                    self.myVehicleParameter.m().toTaz = tazID;
                    self.myVehicleParameter.m().parametersSet |= VEHPARS_TO_TAZ_SET;
                }
            }
        } else if attrs.has_attribute(SUMO_ATTR_TOXY) {
            let pv: PositionVector = attrs.get(SUMO_ATTR_TOXY, &self.myVehicleParameter.id, ok, true);
            parse_geo_edges(pv, false, vClass, &mut self.myActiveRoute, &rid, false, ok);
            if myMapMatchTAZ && *ok {
                self.myVehicleParameter.m().toTaz = self.myActiveRoute.last().unwrap().get_id().to_string();
                self.myVehicleParameter.m().parametersSet |= VEHPARS_TO_TAZ_SET;
            }
        } else if attrs.has_attribute(SUMO_ATTR_TOLONLAT) {
            let pv: PositionVector = attrs.get(SUMO_ATTR_TOLONLAT, &self.myVehicleParameter.id, ok, true);
            parse_geo_edges(pv, true, vClass, &mut self.myActiveRoute, &rid, false, ok);
            if myMapMatchTAZ && *ok {
                self.myVehicleParameter.m().toTaz = self.myActiveRoute.last().unwrap().get_id().to_string();
                self.myVehicleParameter.m().parametersSet |= VEHPARS_TO_TAZ_SET;
            }
        } else {
            let desc: String = attrs.get_opt(SUMO_ATTR_TO, &self.myVehicleParameter.id, ok, String::new(), true);
            let mut route = std::mem::take(&mut self.myActiveRoute);
            self.parse_edges(&desc, &mut route, &rid, ok);
            self.myActiveRoute = route;
        }
        self.myActiveRouteID = format!("!{}", self.myVehicleParameter.id);
        if self.myVehicleParameter.routeid == "" {
            self.myVehicleParameter.m().routeid = self.myActiveRouteID.clone();
        }
    }

    /// C++: `void RORouteHandler::openVehicleTypeDistribution(const SUMOSAXAttributes &)` [virtual, protected]
    /// opens a type distribution for reading
    // void
    // RORouteHandler::openVehicleTypeDistribution(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     myCurrentVTypeDistributionID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (ok) {
    //         myCurrentVTypeDistribution = new RandomDistributor<SUMOVTypeParameter*>();
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
    //                 const std::string typeID = st.next();
    //                 const RandomDistributor<SUMOVTypeParameter*>* const dist = myNet.getVTypeDistribution(typeID);
    //                 if (dist != nullptr) {
    //                     const double distProb = ((int)probs.size() > probIndex ? probs[probIndex] : 1.) / dist->getOverallProb();
    //                     std::vector<double>::const_iterator probIt = dist->getProbs().begin();
    //                     for (SUMOVTypeParameter* const type : dist->getVals()) {
    //                         myCurrentVTypeDistribution->add(type, distProb * *probIt);
    //                         probIt++;
    //                     }
    //                 } else {
    //                     SUMOVTypeParameter* const type = myNet.getVehicleTypeSecure(typeID);
    //                     if (type == nullptr) {
    //                         myErrorOutput->inform("Unknown vehicle type '" + typeID + "' in distribution '" + myCurrentVTypeDistributionID + "'.");
    //                     } else {
    //                         const double prob = ((int)probs.size() > probIndex ? probs[probIndex] : type->defaultProbability);
    //                         myCurrentVTypeDistribution->add(type, prob);
    //                     }
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
                        // C++ toDoubleSecure throws on a malformed value
                        probs.push(utils::common::string_utils::to_double_secure(&next, 1.0).unwrap_or_else(|e| std::panic::panic_any(e)));
                    }
                }
                let vTypes: String = attrs.get(SUMO_ATTR_VTYPES, &self.myCurrentVTypeDistributionID, &mut ok, true);
                let mut st = StringTokenizer::new(&vTypes);
                let mut probIndex: i32 = 0;
                while st.has_next() {
                    let typeID = st.next().unwrap();
                    let dist = self.myNet.m().get_v_type_distribution(&typeID);
                    if !dist.is_null() {
                        let distProb = (if (probs.len() as i32) > probIndex { probs[probIndex as usize] } else { 1.0 }) / dist.get_overall_prob();
                        let distProbs = dist.get_probs();
                        for (i, type_) in dist.get_vals().iter().enumerate() {
                            self.myCurrentVTypeDistribution.m().add(*type_, distProb * distProbs[i], true);
                        }
                    } else {
                        let type_ = self.myNet.m().get_vehicle_type_secure(&typeID);
                        if type_.is_null() {
                            let msg = format!("Unknown vehicle type '{}' in distribution '{}'.", typeID, self.myCurrentVTypeDistributionID);
                            self.myErrorOutput.m().inform(&msg, true);
                        } else {
                            let prob = if (probs.len() as i32) > probIndex { probs[probIndex as usize] } else { type_.defaultProbability };
                            self.myCurrentVTypeDistribution.m().add(type_, prob, true);
                        }
                    }
                    probIndex += 1;
                }
                if probs.len() > 0 && probIndex != (probs.len() as i32) {
                    write_warning(&format!(
                        "Got {} probabilities for {} types in vTypeDistribution '{}'",
                        probs.len(),
                        probIndex,
                        self.myCurrentVTypeDistributionID
                    ));
                }
            }
        }
    }

    /// C++: `void RORouteHandler::closeVehicleTypeDistribution()` [virtual, protected]
    /// closes (ends) the building of a distribution
    // void
    // RORouteHandler::closeVehicleTypeDistribution() {
    //     if (myCurrentVTypeDistribution != nullptr) {
    //         if (myCurrentVTypeDistribution->getOverallProb() == 0) {
    //             delete myCurrentVTypeDistribution;
    //             myErrorOutput->inform("Vehicle type distribution '" + myCurrentVTypeDistributionID + "' is empty.");
    //         } else if (!myNet.addVTypeDistribution(myCurrentVTypeDistributionID, myCurrentVTypeDistribution)) {
    //             delete myCurrentVTypeDistribution;
    //             myErrorOutput->inform("Another vehicle type (or distribution) with the id '" + myCurrentVTypeDistributionID + "' exists.");
    //         }
    //         myCurrentVTypeDistribution = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_vehicle_type_distribution(&mut self) {
        if !self.myCurrentVTypeDistribution.is_null() {
            if self.myCurrentVTypeDistribution.get_overall_prob() == 0. {
                delete_ptr(self.myCurrentVTypeDistribution);
                let msg = format!("Vehicle type distribution '{}' is empty.", self.myCurrentVTypeDistributionID);
                self.myErrorOutput.m().inform(&msg, true);
            } else if !self.myNet.m().add_v_type_distribution(&self.myCurrentVTypeDistributionID, self.myCurrentVTypeDistribution) {
                delete_ptr(self.myCurrentVTypeDistribution);
                let msg = format!("Another vehicle type (or distribution) with the id '{}' exists.", self.myCurrentVTypeDistributionID);
                self.myErrorOutput.m().inform(&msg, true);
            }
            self.myCurrentVTypeDistribution = Ptr::null();
        }
    }

    /// C++: `void RORouteHandler::openRoute(const SUMOSAXAttributes &)` [virtual, protected]
    /// opens a route for reading
    // void
    // RORouteHandler::openRoute(const SUMOSAXAttributes& attrs) {
    //     myActiveRoute.clear();
    //     myInsertStopEdgesAt = -1;
    //     // check whether the id is really necessary
    //     std::string rid;
    //     if (myCurrentAlternatives != nullptr) {
    //         myActiveRouteID = myCurrentAlternatives->getID();
    //         rid =  "distribution '" + myCurrentAlternatives->getID() + "'";
    //     } else if (myVehicleParameter != nullptr) {
    //         // ok, a vehicle is wrapping the route,
    //         //  we may use this vehicle's id as default
    //         myVehicleParameter->routeid = myActiveRouteID = "!" + myVehicleParameter->id; // !!! document this
    //         if (attrs.hasAttribute(SUMO_ATTR_ID)) {
    //             WRITE_WARNINGF(TL("Ids of internal routes are ignored (vehicle '%')."), myVehicleParameter->id);
    //         }
    //     } else {
    //         bool ok = true;
    //         myActiveRouteID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
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
    //         parseEdges(attrs.get<std::string>(SUMO_ATTR_EDGES, myActiveRouteID.c_str(), ok), myActiveRoute, rid, ok);
    //     }
    //     myActiveRouteRefID = attrs.getOpt<std::string>(SUMO_ATTR_REFID, myActiveRouteID.c_str(), ok, "");
    //     if (myActiveRouteRefID != "" && myNet.getRouteDef(myActiveRouteRefID) == nullptr) {
    //         myErrorOutput->inform("Invalid reference to route '" + myActiveRouteRefID + "' in route " + rid + ".");
    //     }
    //     if (myCurrentAlternatives != nullptr && !attrs.hasAttribute(SUMO_ATTR_PROB)) {
    //         WRITE_WARNINGF(TL("No probability for route %, using default."), rid);
    //     }
    //     myActiveRouteProbability = attrs.getOpt<double>(SUMO_ATTR_PROB, myActiveRouteID.c_str(), ok, DEFAULT_VEH_PROB);
    //     if (ok && myActiveRouteProbability < 0) {
    //         myErrorOutput->inform("Invalid probability for route '" + myActiveRouteID + "'.");
    //     }
    //     myActiveRouteColor = attrs.hasAttribute(SUMO_ATTR_COLOR) ? new RGBColor(attrs.get<RGBColor>(SUMO_ATTR_COLOR, myActiveRouteID.c_str(), ok)) : nullptr;
    //     ok = true;
    //     myActiveRouteRepeat = attrs.getOpt<int>(SUMO_ATTR_REPEAT, myActiveRouteID.c_str(), ok, 0);
    //     myActiveRoutePeriod = attrs.getOptSUMOTimeReporting(SUMO_ATTR_CYCLETIME, myActiveRouteID.c_str(), ok, 0);
    //     if (myActiveRouteRepeat > 0) {
    //         SUMOVehicleClass vClass = SVC_IGNORING;
    //         if (myVehicleParameter != nullptr) {
    //             SUMOVTypeParameter* type = myNet.getVehicleTypeSecure(myVehicleParameter->vtypeid);
    //             if (type != nullptr) {
    //                 vClass = type->vehicleClass;
    //             }
    //         }
    //         if (myActiveRoute.size() > 0 && !myActiveRoute.back()->isConnectedTo(*myActiveRoute.front(), vClass)) {
    //             myErrorOutput->inform("Disconnected route " + rid + " when repeating.");
    //         }
    //     }
    //     myCurrentCosts = attrs.getOpt<double>(SUMO_ATTR_COST, myActiveRouteID.c_str(), ok, -1);
    //     if (ok && myCurrentCosts != -1 && myCurrentCosts < 0) {
    //         myErrorOutput->inform("Invalid cost for route '" + myActiveRouteID + "'.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn open_route(&mut self, attrs: &SUMOSAXAttributes) {
        self.myActiveRoute.clear();
        self.myInsertStopEdgesAt = -1;
        // check whether the id is really necessary
        let mut rid = String::new();
        if !self.myCurrentAlternatives.is_null() {
            self.myActiveRouteID = self.myCurrentAlternatives.get_id().to_string();
            rid = format!("distribution '{}'", self.myCurrentAlternatives.get_id());
        } else if !self.myVehicleParameter.is_null() {
            // ok, a vehicle is wrapping the route, we may use this vehicle's id as default
            let routeid = format!("!{}", self.myVehicleParameter.id);
            self.myVehicleParameter.m().routeid = routeid.clone();
            self.myActiveRouteID = routeid;
            if attrs.has_attribute(SUMO_ATTR_ID) {
                write_warningf!("Ids of internal routes are ignored (vehicle '%').", self.myVehicleParameter.id);
            }
        } else {
            let mut ok = true;
            self.myActiveRouteID = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
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
            let desc: String = attrs.get(SUMO_ATTR_EDGES, &self.myActiveRouteID, &mut ok, true);
            let mut route = std::mem::take(&mut self.myActiveRoute);
            self.parse_edges(&desc, &mut route, &rid, &mut ok);
            self.myActiveRoute = route;
        }
        self.myActiveRouteRefID = attrs.get_opt(SUMO_ATTR_REFID, &self.myActiveRouteID, &mut ok, String::new(), true);
        if self.myActiveRouteRefID != "" && self.myNet.get_route_def(&self.myActiveRouteRefID).is_null() {
            let msg = format!("Invalid reference to route '{}' in route {}.", self.myActiveRouteRefID, rid);
            self.myErrorOutput.m().inform(&msg, true);
        }
        if !self.myCurrentAlternatives.is_null() && !attrs.has_attribute(SUMO_ATTR_PROB) {
            write_warningf!("No probability for route %, using default.", rid);
        }
        let prob: f64 = attrs.get_opt(SUMO_ATTR_PROB, &self.myActiveRouteID, &mut ok, DEFAULT_VEH_PROB, true);
        self.myActiveRouteProbability = prob;
        if ok && self.myActiveRouteProbability < 0. {
            let msg = format!("Invalid probability for route '{}'.", self.myActiveRouteID);
            self.myErrorOutput.m().inform(&msg, true);
        }
        self.myActiveRouteColor = if attrs.has_attribute(SUMO_ATTR_COLOR) {
            let color: RGBColor = attrs.get(SUMO_ATTR_COLOR, &self.myActiveRouteID, &mut ok, true);
            Ptr::from_raw(Box::into_raw(Box::new(color)))
        } else {
            Ptr::null()
        };
        ok = true;
        self.myActiveRouteRepeat = attrs.get_opt(SUMO_ATTR_REPEAT, &self.myActiveRouteID, &mut ok, 0, true);
        self.myActiveRoutePeriod = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_CYCLETIME, &self.myActiveRouteID, &mut ok, 0, true);
        if self.myActiveRouteRepeat > 0 {
            let mut vClass: SUMOVehicleClass = SVC_IGNORING;
            if !self.myVehicleParameter.is_null() {
                let type_ = self.myNet.m().get_vehicle_type_secure(&self.myVehicleParameter.vtypeid);
                if !type_.is_null() {
                    vClass = type_.vehicleClass;
                }
            }
            if self.myActiveRoute.len() > 0 && !self.myActiveRoute.last().unwrap().is_connected_to(&self.myActiveRoute[0], vClass, false) {
                let msg = format!("Disconnected route {} when repeating.", rid);
                self.myErrorOutput.m().inform(&msg, true);
            }
        }
        self.myCurrentCosts = attrs.get_opt(SUMO_ATTR_COST, &self.myActiveRouteID, &mut ok, -1., true);
        if ok && self.myCurrentCosts != -1. && self.myCurrentCosts < 0. {
            let msg = format!("Invalid cost for route '{}'.", self.myActiveRouteID);
            self.myErrorOutput.m().inform(&msg, true);
        }
    }

    /// C++: `void RORouteHandler::openFlow(const SUMOSAXAttributes &)` [virtual, protected]
    /// opens a flow for reading
    // void
    // RORouteHandler::openFlow(const SUMOSAXAttributes& /*attrs*/) {
    //     // currently unused
    // }
    #[allow(unused_variables)]
    pub fn open_flow(&mut self, attrs: &SUMOSAXAttributes) {
        // C++ body is empty (currently unused)
    }

    /// C++: `void RORouteHandler::openRouteFlow(const SUMOSAXAttributes &)` [virtual, protected]
    /// opens a route flow for reading
    // void
    // RORouteHandler::openRouteFlow(const SUMOSAXAttributes& /*attrs*/) {
    //     // currently unused
    // }
    #[allow(unused_variables)]
    pub fn open_route_flow(&mut self, attrs: &SUMOSAXAttributes) {
        // C++ body is empty (currently unused)
    }

    /// C++: `void RORouteHandler::openTrip(const SUMOSAXAttributes &)` [virtual, protected]
    /// opens a trip for reading
    // void
    // RORouteHandler::openTrip(const SUMOSAXAttributes& /*attrs*/) {
    //     // currently unused
    // }
    #[allow(unused_variables)]
    pub fn open_trip(&mut self, attrs: &SUMOSAXAttributes) {
        // C++ body is empty (currently unused)
    }

    /// C++: `void RORouteHandler::closeRoute(const bool)` [virtual, protected]
    /// closes (ends) the building of a route.
    // void
    // RORouteHandler::closeRoute(const bool mayBeDisconnected) {
    //     const bool mustReroute = myActiveRoute.size() == 0 && myActiveRouteStops.size() != 0;
    //     if (mustReroute) {
    //         // implicit route from stops
    //         for (const SUMOVehicleParameter::Stop& stop : myActiveRouteStops) {
    //             ROEdge* edge = myNet.getEdge(stop.edge);
    //             myActiveRoute.push_back(edge);
    //         }
    //     }
    //     if (myActiveRoute.size() == 0) {
    //         if (myActiveRouteRefID != "" && myCurrentAlternatives != nullptr) {
    //             myCurrentAlternatives->addAlternativeDef(myNet.getRouteDef(myActiveRouteRefID));
    //             myActiveRouteID = "";
    //             myActiveRouteRefID = "";
    //             return;
    //         }
    //         if (myVehicleParameter != nullptr) {
    //             myErrorOutput->inform("The route for vehicle '" + myVehicleParameter->id + "' has no edges.");
    //         } else {
    //             myErrorOutput->inform("Route '" + myActiveRouteID + "' has no edges.");
    //         }
    //         myActiveRouteID = "";
    //         myActiveRouteStops.clear();
    //         return;
    //     }
    //     if (myActiveRoute.size() == 1 && myActiveRoute.front()->isTazConnector()) {
    //         myErrorOutput->inform("The routing information for vehicle '" + myVehicleParameter->id + "' is insufficient.");
    //         myActiveRouteID = "";
    //         myActiveRouteStops.clear();
    //         return;
    //     }
    //     if (!mayBeDisconnected && OptionsCont::getOptions().exists("no-internal-links") && !OptionsCont::getOptions().getBool("no-internal-links")) {
    //         // fix internal edges which did not get parsed
    //         const ROEdge* last = nullptr;
    //         ConstROEdgeVector fullRoute;
    //         for (const ROEdge* roe : myActiveRoute) {
    //             if (last != nullptr) {
    //                 for (const ROEdge* intern : last->getSuccessors()) {
    //                     if (intern->isInternal() && intern->getSuccessors().size() == 1 && intern->getSuccessors().front() == roe) {
    //                         fullRoute.push_back(intern);
    //                     }
    //                 }
    //             }
    //             fullRoute.push_back(roe);
    //             last = roe;
    //         }
    //         myActiveRoute = fullRoute;
    //     }
    //     if (myActiveRouteRepeat > 0) {
    //         // duplicate route
    //         ConstROEdgeVector tmpEdges = myActiveRoute;
    //         auto tmpStops = myActiveRouteStops;
    //         for (int i = 0; i < myActiveRouteRepeat; i++) {
    //             myActiveRoute.insert(myActiveRoute.begin(), tmpEdges.begin(), tmpEdges.end());
    //             for (SUMOVehicleParameter::Stop stop : tmpStops) {
    //                 if (stop.until > 0) {
    //                     if (myActiveRoutePeriod <= 0) {
    //                         const std::string description = myVehicleParameter != nullptr
    //                                                         ?  "for vehicle '" + myVehicleParameter->id + "'"
    //                                                         :  "'" + myActiveRouteID + "'";
    //                         throw ProcessError(TLF("Cannot repeat stops with 'until' in route % because no cycleTime is defined.", description));
    //                     }
    //                     stop.until += myActiveRoutePeriod * (i + 1);
    //                     stop.arrival += myActiveRoutePeriod * (i + 1);
    //                 }
    //                 myActiveRouteStops.push_back(stop);
    //             }
    //         }
    //     }
    //     std::shared_ptr<RORoute> route = std::make_shared<RORoute>(myActiveRouteID, myCurrentCosts, myActiveRouteProbability, myActiveRoute,
    //                                      myActiveRouteColor, myActiveRouteStops);
    //     myActiveRoute.clear();
    //     if (myCurrentAlternatives == nullptr) {
    //         if (myNet.getRouteDef(myActiveRouteID) != nullptr) {
    //             if (myVehicleParameter != nullptr) {
    //                 myErrorOutput->inform("Another route for vehicle '" + myVehicleParameter->id + "' exists.");
    //             } else {
    //                 myErrorOutput->inform("Another route (or distribution) with the id '" + myActiveRouteID + "' exists.");
    //             }
    //             myActiveRouteID = "";
    //             myActiveRouteStops.clear();
    //             return;
    //         } else {
    //             myCurrentAlternatives = new RORouteDef(myActiveRouteID, 0, mayBeDisconnected || myTryRepair, mayBeDisconnected);
    //             myCurrentAlternatives->addLoadedAlternative(route);
    //             myNet.addRouteDef(myCurrentAlternatives);
    //             myCurrentAlternatives = nullptr;
    //         }
    //     } else {
    //         myCurrentAlternatives->addLoadedAlternative(route);
    //     }
    //     myActiveRouteID = "";
    //     myActiveRouteStops.clear();
    // }
    #[allow(unused_variables)]
    pub fn close_route(&mut self, mayBeDisconnected: bool) {
        let mustReroute = self.myActiveRoute.len() == 0 && self.myActiveRouteStops.len() != 0;
        if mustReroute {
            // implicit route from stops
            for stop in self.myActiveRouteStops.iter() {
                let edge = self.myNet.get_edge(&stop.edge);
                self.myActiveRoute.push(edge);
            }
        }
        if self.myActiveRoute.len() == 0 {
            if self.myActiveRouteRefID != "" && !self.myCurrentAlternatives.is_null() {
                let def = self.myNet.get_route_def(&self.myActiveRouteRefID);
                self.myCurrentAlternatives.m().add_alternative_def(def);
                self.myActiveRouteID = String::new();
                self.myActiveRouteRefID = String::new();
                return;
            }
            if !self.myVehicleParameter.is_null() {
                let msg = format!("The route for vehicle '{}' has no edges.", self.myVehicleParameter.id);
                self.myErrorOutput.m().inform(&msg, true);
            } else {
                let msg = format!("Route '{}' has no edges.", self.myActiveRouteID);
                self.myErrorOutput.m().inform(&msg, true);
            }
            self.myActiveRouteID = String::new();
            self.myActiveRouteStops.clear();
            return;
        }
        if self.myActiveRoute.len() == 1 && self.myActiveRoute[0].is_taz_connector() {
            let msg = format!("The routing information for vehicle '{}' is insufficient.", self.myVehicleParameter.id);
            self.myErrorOutput.m().inform(&msg, true);
            self.myActiveRouteID = String::new();
            self.myActiveRouteStops.clear();
            return;
        }
        if !mayBeDisconnected && utils::options::options_cont::exists("no-internal-links") && !utils::options::options_cont::get_bool("no-internal-links") {
            // fix internal edges which did not get parsed
            let mut last: Ptr<ROEdge> = Ptr::null();
            let mut fullRoute: ConstROEdgeVector = Vec::new();
            for roe in self.myActiveRoute.iter() {
                if !last.is_null() {
                    for intern in last.get_successors(SVC_IGNORING).iter() {
                        if intern.is_internal() && intern.get_successors(SVC_IGNORING).len() == 1 && intern.get_successors(SVC_IGNORING)[0] == *roe {
                            fullRoute.push(*intern);
                        }
                    }
                }
                fullRoute.push(*roe);
                last = *roe;
            }
            self.myActiveRoute = fullRoute;
        }
        if self.myActiveRouteRepeat > 0 {
            // duplicate route
            let tmpEdges: ConstROEdgeVector = self.myActiveRoute.clone();
            let tmpStops = self.myActiveRouteStops.clone();
            for i in 0..self.myActiveRouteRepeat {
                self.myActiveRoute.splice(0..0, tmpEdges.iter().cloned());
                for stop in tmpStops.iter() {
                    let mut stop = stop.clone();
                    if stop.until > 0 {
                        if self.myActiveRoutePeriod <= 0 {
                            let description = if !self.myVehicleParameter.is_null() {
                                format!("for vehicle '{}'", self.myVehicleParameter.id)
                            } else {
                                format!("'{}'", self.myActiveRouteID)
                            };
                            throw_process_error(format!(
                                "Cannot repeat stops with 'until' in route {} because no cycleTime is defined.",
                                description
                            ));
                        }
                        stop.until += self.myActiveRoutePeriod * ((i + 1) as SUMOTime);
                        stop.arrival += self.myActiveRoutePeriod * ((i + 1) as SUMOTime);
                    }
                    self.myActiveRouteStops.push(stop);
                }
            }
        }
        let route: Rc<RORoute> = Rc::new(RORoute::new(
            &self.myActiveRouteID,
            self.myCurrentCosts,
            self.myActiveRouteProbability,
            &self.myActiveRoute,
            self.myActiveRouteColor,
            &self.myActiveRouteStops,
        ));
        self.myActiveRoute.clear();
        if self.myCurrentAlternatives.is_null() {
            if !self.myNet.get_route_def(&self.myActiveRouteID).is_null() {
                if !self.myVehicleParameter.is_null() {
                    let msg = format!("Another route for vehicle '{}' exists.", self.myVehicleParameter.id);
                    self.myErrorOutput.m().inform(&msg, true);
                } else {
                    let msg = format!("Another route (or distribution) with the id '{}' exists.", self.myActiveRouteID);
                    self.myErrorOutput.m().inform(&msg, true);
                }
                self.myActiveRouteID = String::new();
                self.myActiveRouteStops.clear();
                return;
            } else {
                let def: Ptr<RORouteDef> = Ptr::from_raw(Box::into_raw(Box::new(RORouteDef::new(
                    &self.myActiveRouteID,
                    0,
                    mayBeDisconnected || self.myTryRepair,
                    mayBeDisconnected,
                ))));
                self.myCurrentAlternatives = def;
                def.m().add_loaded_alternative(route);
                self.myNet.m().add_route_def(def);
                self.myCurrentAlternatives = Ptr::null();
            }
        } else {
            self.myCurrentAlternatives.m().add_loaded_alternative(route);
        }
        self.myActiveRouteID = String::new();
        self.myActiveRouteStops.clear();
    }

    /// C++: `void RORouteHandler::openRouteDistribution(const SUMOSAXAttributes &)` [virtual, protected]
    /// opens a route distribution for reading
    // void
    // RORouteHandler::openRouteDistribution(const SUMOSAXAttributes& attrs) {
    //     // check whether the id is really necessary
    //     bool ok = true;
    //     std::string id;
    //     if (myVehicleParameter != nullptr) {
    //         // ok, a vehicle is wrapping the route,
    //         //  we may use this vehicle's id as default
    //         myVehicleParameter->routeid = id = "!" + myVehicleParameter->id; // !!! document this
    //         if (attrs.hasAttribute(SUMO_ATTR_ID)) {
    //             WRITE_WARNINGF(TL("Ids of internal route distributions are ignored (vehicle '%')."), myVehicleParameter->id);
    //         }
    //     } else {
    //         id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //         if (!ok) {
    //             return;
    //         }
    //     }
    //     // try to get the index of the last element
    //     int index = attrs.getOpt<int>(SUMO_ATTR_LAST, id.c_str(), ok, 0);
    //     if (ok && index < 0) {
    //         myErrorOutput->inform("Negative index of a route alternative (id='" + id + "').");
    //         return;
    //     }
    //     // build the alternative cont
    //     myCurrentAlternatives = new RORouteDef(id, index, myTryRepair, false);
    //     if (attrs.hasAttribute(SUMO_ATTR_ROUTES)) {
    //         ok = true;
    //         StringTokenizer st(attrs.get<std::string>(SUMO_ATTR_ROUTES, id.c_str(), ok));
    //         while (st.hasNext()) {
    //             const std::string routeID = st.next();
    //             const RORouteDef* route = myNet.getRouteDef(routeID);
    //             if (route == nullptr) {
    //                 myErrorOutput->inform("Unknown route '" + routeID + "' in distribution '" + id + "'.");
    //             } else {
    //                 myCurrentAlternatives->addAlternativeDef(route);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn open_route_distribution(&mut self, attrs: &SUMOSAXAttributes) {
        // check whether the id is really necessary
        let mut ok = true;
        let mut id: String = String::new();
        if !self.myVehicleParameter.is_null() {
            // ok, a vehicle is wrapping the route, we may use this vehicle's id as default
            id = format!("!{}", self.myVehicleParameter.id);
            self.myVehicleParameter.m().routeid = id.clone();
            if attrs.has_attribute(SUMO_ATTR_ID) {
                write_warningf!("Ids of internal route distributions are ignored (vehicle '%').", self.myVehicleParameter.id);
            }
        } else {
            id = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
            if !ok {
                return;
            }
        }
        // try to get the index of the last element
        let index: i32 = attrs.get_opt(SUMO_ATTR_LAST, &id, &mut ok, 0, true);
        if ok && index < 0 {
            let msg = format!("Negative index of a route alternative (id='{}').", id);
            self.myErrorOutput.m().inform(&msg, true);
            return;
        }
        // build the alternative cont
        self.myCurrentAlternatives = Ptr::from_raw(Box::into_raw(Box::new(RORouteDef::new(&id, index, self.myTryRepair, false))));
        if attrs.has_attribute(SUMO_ATTR_ROUTES) {
            ok = true;
            let routes: String = attrs.get(SUMO_ATTR_ROUTES, &id, &mut ok, true);
            let mut st = StringTokenizer::new(&routes);
            while st.has_next() {
                let routeID = st.next().unwrap();
                let route = self.myNet.get_route_def(&routeID);
                if route.is_null() {
                    let msg = format!("Unknown route '{}' in distribution '{}'.", routeID, id);
                    self.myErrorOutput.m().inform(&msg, true);
                } else {
                    self.myCurrentAlternatives.m().add_alternative_def(route);
                }
            }
        }
    }

    /// C++: `void RORouteHandler::closeRouteDistribution()` [virtual, protected]
    /// closes (ends) the building of a distribution
    // void
    // RORouteHandler::closeRouteDistribution() {
    //     if (myCurrentAlternatives != nullptr) {
    //         if (myCurrentAlternatives->getOverallProb() == 0) {
    //             myErrorOutput->inform("Route distribution '" + myCurrentAlternatives->getID() + "' is empty.");
    //             delete myCurrentAlternatives;
    //         } else if (!myNet.addRouteDef(myCurrentAlternatives)) {
    //             myErrorOutput->inform("Another route (or distribution) with the id '" + myCurrentAlternatives->getID() + "' exists.");
    //             delete myCurrentAlternatives;
    //         } else {
    //             if (myVehicleParameter != nullptr
    //                     && (myUseTaz || OptionsCont::getOptions().getBool("junction-taz"))
    //                     && (myVehicleParameter->wasSet(VEHPARS_FROM_TAZ_SET) ||
    //                         myVehicleParameter->wasSet(VEHPARS_TO_TAZ_SET))) {
    //                 // we are loading a rou.alt.xml, permit rerouting between taz
    //                 bool ok = true;
    //                 ConstROEdgeVector edges;
    //                 if (myVehicleParameter->fromTaz != "") {
    //                     const std::string tazID = myVehicleParameter->fromTaz;
    //                     const ROEdge* fromTaz = myNet.getEdge(tazID + "-source");
    //                     if (fromTaz == nullptr) {
    //                         myErrorOutput->inform("Source taz '" + tazID + "' not known for vehicle '" + myVehicleParameter->id + "'!");
    //                         ok = false;
    //                     } else if (fromTaz->getNumSuccessors() == 0) {
    //                         myErrorOutput->inform("Source taz '" + tazID + "' has no outgoing edges for vehicle '" + myVehicleParameter->id + "'!");
    //                         ok = false;
    //                     } else {
    //                         edges.push_back(fromTaz);
    //                     }
    //                 } else {
    //                     edges.push_back(myCurrentAlternatives->getOrigin());
    //                 }
    //                 if (myVehicleParameter->toTaz != "") {
    //                     const std::string tazID = myVehicleParameter->toTaz;
    //                     const ROEdge* toTaz = myNet.getEdge(tazID + "-sink");
    //                     if (toTaz == nullptr) {
    //                         myErrorOutput->inform("Sink taz '" + tazID + "' not known for vehicle '" + myVehicleParameter->id + "'!");
    //                         ok = false;
    //                     } else if (toTaz->getNumPredecessors() == 0) {
    //                         myErrorOutput->inform("Sink taz '" + tazID + "' has no incoming edges for vehicle '" + myVehicleParameter->id + "'!");
    //                         ok = false;
    //                     } else {
    //                         edges.push_back(toTaz);
    //                     }
    //                 } else {
    //                     edges.push_back(myCurrentAlternatives->getDestination());
    //                 }
    //                 if (ok) {
    //                     // negative probability indicates that this route should not be written
    //                     myCurrentAlternatives->addLoadedAlternative(std::make_shared<RORoute>(myCurrentAlternatives->getID(), 0, -1, edges, nullptr, myActiveRouteStops));
    //                 }
    //             }
    //         }
    //         myCurrentAlternatives = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_route_distribution(&mut self) {
        if !self.myCurrentAlternatives.is_null() {
            if self.myCurrentAlternatives.get_overall_prob() == 0. {
                let msg = format!("Route distribution '{}' is empty.", self.myCurrentAlternatives.get_id());
                self.myErrorOutput.m().inform(&msg, true);
                delete_ptr(self.myCurrentAlternatives);
            } else if !self.myNet.m().add_route_def(self.myCurrentAlternatives) {
                let msg = format!("Another route (or distribution) with the id '{}' exists.", self.myCurrentAlternatives.get_id());
                self.myErrorOutput.m().inform(&msg, true);
                delete_ptr(self.myCurrentAlternatives);
            } else {
                if !self.myVehicleParameter.is_null()
                    && (self.myUseTaz || utils::options::options_cont::get_bool("junction-taz"))
                    && (self.myVehicleParameter.was_set(VEHPARS_FROM_TAZ_SET) || self.myVehicleParameter.was_set(VEHPARS_TO_TAZ_SET))
                {
                    // we are loading a rou.alt.xml, permit rerouting between taz
                    let mut ok = true;
                    let mut edges: ConstROEdgeVector = Vec::new();
                    if self.myVehicleParameter.fromTaz != "" {
                        let tazID = self.myVehicleParameter.fromTaz.clone();
                        let fromTaz = self.myNet.get_edge(&format!("{}-source", tazID));
                        if fromTaz.is_null() {
                            let msg = format!("Source taz '{}' not known for vehicle '{}'!", tazID, self.myVehicleParameter.id);
                            self.myErrorOutput.m().inform(&msg, true);
                            ok = false;
                        } else if fromTaz.get_num_successors() == 0 {
                            let msg = format!("Source taz '{}' has no outgoing edges for vehicle '{}'!", tazID, self.myVehicleParameter.id);
                            self.myErrorOutput.m().inform(&msg, true);
                            ok = false;
                        } else {
                            edges.push(fromTaz);
                        }
                    } else {
                        edges.push(self.myCurrentAlternatives.get_origin());
                    }
                    if self.myVehicleParameter.toTaz != "" {
                        let tazID = self.myVehicleParameter.toTaz.clone();
                        let toTaz = self.myNet.get_edge(&format!("{}-sink", tazID));
                        if toTaz.is_null() {
                            let msg = format!("Sink taz '{}' not known for vehicle '{}'!", tazID, self.myVehicleParameter.id);
                            self.myErrorOutput.m().inform(&msg, true);
                            ok = false;
                        } else if toTaz.get_num_predecessors() == 0 {
                            let msg = format!("Sink taz '{}' has no incoming edges for vehicle '{}'!", tazID, self.myVehicleParameter.id);
                            self.myErrorOutput.m().inform(&msg, true);
                            ok = false;
                        } else {
                            edges.push(toTaz);
                        }
                    } else {
                        edges.push(self.myCurrentAlternatives.get_destination());
                    }
                    if ok {
                        // negative probability indicates that this route should not be written
                        let route: Rc<RORoute> = Rc::new(RORoute::new(
                            self.myCurrentAlternatives.get_id(),
                            0.,
                            -1.,
                            &edges,
                            Ptr::null(),
                            &self.myActiveRouteStops,
                        ));
                        self.myCurrentAlternatives.m().add_loaded_alternative(route);
                    }
                }
            }
            self.myCurrentAlternatives = Ptr::null();
        }
    }

    /// C++: `void RORouteHandler::closeVehicle()` [virtual, protected]
    /// Ends the processing of a vehicle
    // void
    // RORouteHandler::closeVehicle() {
    //     checkLastDepart();
    //     // get the vehicle id
    //     if (myVehicleParameter->departProcedure == DepartDefinition::GIVEN && myVehicleParameter->depart < myBegin) {
    //         mySkippedVehicles.insert(myVehicleParameter->id);
    //         myNet.routeSkipped();
    //         return;
    //     }
    //     // get vehicle type
    //     SUMOVTypeParameter* type = myNet.getVehicleTypeSecure(myVehicleParameter->vtypeid);
    //     if (type == nullptr) {
    //         myErrorOutput->inform("The vehicle type '" + myVehicleParameter->vtypeid + "' for vehicle '" + myVehicleParameter->id + "' is not known.");
    //         type = myNet.getVehicleTypeSecure(DEFAULT_VTYPE_ID);
    //     } else {
    //         if (!myKeepVTypeDist) {
    //             // fix the type id in case we used a distribution
    //             myVehicleParameter->vtypeid = type->id;
    //         }
    //     }
    //     if (type->vehicleClass == SVC_PEDESTRIAN) {
    //         WRITE_WARNINGF(TL("Vehicle type '%' with vClass=pedestrian should only be used for persons and not for vehicle '%'."), type->id, myVehicleParameter->id);
    //     }
    //     // get the route
    //     RORouteDef* route = myNet.getRouteDef(myVehicleParameter->routeid);
    //     if (route == nullptr) {
    //         myErrorOutput->inform("The route of the vehicle '" + myVehicleParameter->id + "' is not known.");
    //         return;
    //     }
    //     if (MsgHandler::getErrorInstance()->wasInformed()) {
    //         return;
    //     }
    //     const bool needCopy = route->getID()[0] != '!';
    //     if (needCopy) {
    //         route = route->copy("!" + myVehicleParameter->id, myVehicleParameter->depart);
    //     }
    //     // build the vehicle
    //     ROVehicle* veh = new ROVehicle(*myVehicleParameter, route, type, &myNet, myErrorOutput);
    //     if (myNet.addVehicle(myVehicleParameter->id, veh)) {
    //         registerLastDepart();
    //     } else if (needCopy) {
    //         delete route;
    //     }
    //     delete myVehicleParameter;
    //     myVehicleParameter = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn close_vehicle(&mut self) {
        self.check_last_depart();
        // get the vehicle id
        if self.myVehicleParameter.departProcedure == DepartDefinition::GIVEN && self.myVehicleParameter.depart < self.myBegin {
            self.mySkippedVehicleIDs.insert(self.myVehicleParameter.id.clone());
            self.myNet.m().route_skipped();
            return;
        }
        // get vehicle type
        let mut type_ = self.myNet.m().get_vehicle_type_secure(&self.myVehicleParameter.vtypeid);
        if type_.is_null() {
            let msg = format!("The vehicle type '{}' for vehicle '{}' is not known.", self.myVehicleParameter.vtypeid, self.myVehicleParameter.id);
            self.myErrorOutput.m().inform(&msg, true);
            type_ = self.myNet.m().get_vehicle_type_secure(utils::common::sumo_vehicle_class::DEFAULT_VTYPE_ID);
        } else {
            if !self.myKeepVTypeDist {
                // fix the type id in case we used a distribution
                self.myVehicleParameter.m().vtypeid = type_.id.clone();
            }
        }
        if type_.vehicleClass == utils::common::sumo_vehicle_class::SVC_PEDESTRIAN {
            write_warningf!(
                "Vehicle type '%' with vClass=pedestrian should only be used for persons and not for vehicle '%'.",
                type_.id,
                self.myVehicleParameter.id
            );
        }
        // get the route
        let mut route = self.myNet.m().get_route_def(&self.myVehicleParameter.routeid);
        if route.is_null() {
            let msg = format!("The route of the vehicle '{}' is not known.", self.myVehicleParameter.id);
            self.myErrorOutput.m().inform(&msg, true);
            return;
        }
        if msg_instance(false).was_informed() {
            return;
        }
        let needCopy = !route.get_id().starts_with('!');
        if needCopy {
            route = route.copy(&format!("!{}", self.myVehicleParameter.id), self.myVehicleParameter.depart);
        }
        // build the vehicle
        let veh: Ptr<ROVehicle> = Ptr::from_raw(Box::into_raw(Box::new(ROVehicle::new(
            &self.myVehicleParameter,
            route,
            type_,
            self.myNet,
            self.myErrorOutput,
        ))));
        if self.myNet.m().add_vehicle(&self.myVehicleParameter.id, veh) {
            self.register_last_depart();
        } else if needCopy {
            delete_ptr(route);
        }
        delete_ptr(self.myVehicleParameter);
        self.myVehicleParameter = Ptr::null();
    }

    /// C++: `void RORouteHandler::closeVType()` [virtual, protected]
    /// Ends the processing of a vehicle type
    // void
    // RORouteHandler::closeVType() {
    //     if (myCurrentVTypeRef.empty()) {
    //         myNet.addVehicleType(myCurrentVType);
    //         if (OptionsCont::getOptions().isSet("restriction-params")) {
    //             const std::vector<std::string> paramKeys = OptionsCont::getOptions().getStringVector("restriction-params");
    //             myCurrentVType->cacheParamRestrictions(paramKeys);
    //         }
    //     }
    //     if (myCurrentVTypeDistribution != nullptr) {
    //         if (myCurrentVTypeRef.empty()) {
    //             myCurrentVTypeDistribution->add(myCurrentVType, myCurrentVType->defaultProbability);
    //         } else {
    //             const RandomDistributor<SUMOVTypeParameter*>* const dist = myNet.getVTypeDistribution(myCurrentVTypeRef);
    //             if (dist != nullptr) {
    //                 const double distProb = (myCurrentVTypeProbability >= 0 ? myCurrentVTypeProbability : 1) / dist->getOverallProb();
    //                 std::vector<double>::const_iterator probIt = dist->getProbs().begin();
    //                 for (SUMOVTypeParameter* const type : dist->getVals()) {
    //                     myCurrentVTypeDistribution->add(type, distProb * *probIt);
    //                     probIt++;
    //                 }
    //             } else {
    //                 SUMOVTypeParameter* const type = myNet.getVehicleTypeSecure(myCurrentVTypeRef);
    //                 double probability = myCurrentVTypeProbability >= 0 ? myCurrentVTypeProbability : type->defaultProbability;
    //                 myCurrentVTypeDistribution->add(type, probability);
    //             }
    //         }
    //     }
    //     myCurrentVType = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn close_v_type(&mut self) {
        if self.myCurrentVTypeRef.is_empty() {
            self.myNet.m().add_vehicle_type(self.myCurrentVType);
            if utils::options::options_cont::is_set("restriction-params") {
                let paramKeys = utils::options::options_cont::get_string_vector("restriction-params");
                self.myCurrentVType.m().cache_param_restrictions(&paramKeys);
            }
        }
        if !self.myCurrentVTypeDistribution.is_null() {
            if self.myCurrentVTypeRef.is_empty() {
                let defaultProb = self.myCurrentVType.defaultProbability;
                self.myCurrentVTypeDistribution.m().add(self.myCurrentVType, defaultProb, true);
            } else {
                let dist = self.myNet.m().get_v_type_distribution(&self.myCurrentVTypeRef);
                if !dist.is_null() {
                    let distProb = (if self.myCurrentVTypeProbability >= 0. { self.myCurrentVTypeProbability } else { 1. }) / dist.get_overall_prob();
                    let distProbs = dist.get_probs();
                    for (i, type_) in dist.get_vals().iter().enumerate() {
                        self.myCurrentVTypeDistribution.m().add(*type_, distProb * distProbs[i], true);
                    }
                } else {
                    let type_ = self.myNet.m().get_vehicle_type_secure(&self.myCurrentVTypeRef);
                    let probability = if self.myCurrentVTypeProbability >= 0. { self.myCurrentVTypeProbability } else { type_.defaultProbability };
                    self.myCurrentVTypeDistribution.m().add(type_, probability, true);
                }
            }
        }
        self.myCurrentVType = Ptr::null();
    }

    /// C++: `void RORouteHandler::closePerson()` [virtual, protected]
    /// Ends the processing of a person
    // void
    // RORouteHandler::closePerson() {
    //     SUMOVTypeParameter* type = myNet.getVehicleTypeSecure(myVehicleParameter->vtypeid);
    //     if (type == nullptr) {
    //         myErrorOutput->inform("The vehicle type '" + myVehicleParameter->vtypeid + "' for person '" + myVehicleParameter->id + "' is not known.");
    //         type = myNet.getVehicleTypeSecure(DEFAULT_PEDTYPE_ID);
    //     }
    //     if (myActivePlan == nullptr || myActivePlan->empty()) {
    //         WRITE_WARNINGF(TL("Discarding person '%' because her plan is empty"), myVehicleParameter->id);
    //     } else if (myVehicleParameter->departProcedure != DepartDefinition::GIVEN || myVehicleParameter->depart >= myBegin) {
    //         ROPerson* person = new ROPerson(*myVehicleParameter, type);
    //         for (ROPerson::PlanItem* item : *myActivePlan) {
    //             person->getPlan().push_back(item);
    //         }
    //         myActivePlan->clear();
    //         if (myNet.addPerson(person)) {
    //             checkLastDepart();
    //             registerLastDepart();
    //         }
    //     } else {
    //         myNet.routeSkipped();
    //     }
    //     deleteActivePlanAndVehicleParameter();
    // }
    #[allow(unused_variables)]
    pub fn close_person(&mut self) {
        let mut type_ = self.myNet.m().get_vehicle_type_secure(&self.myVehicleParameter.vtypeid);
        if type_.is_null() {
            let msg = format!("The vehicle type '{}' for person '{}' is not known.", self.myVehicleParameter.vtypeid, self.myVehicleParameter.id);
            self.myErrorOutput.m().inform(&msg, true);
            type_ = self.myNet.m().get_vehicle_type_secure(utils::common::sumo_vehicle_class::DEFAULT_PEDTYPE_ID);
        }
        if self.myActivePlan.is_null() || self.myActivePlan.is_empty() {
            write_warningf!("Discarding person '%' because her plan is empty", self.myVehicleParameter.id);
        } else if self.myVehicleParameter.departProcedure != DepartDefinition::GIVEN || self.myVehicleParameter.depart >= self.myBegin {
            let person: Ptr<ROPerson> = Ptr::from_raw(Box::into_raw(Box::new(ROPerson::new(&self.myVehicleParameter, type_))));
            for item in self.myActivePlan.m().iter() {
                person.m().get_plan().push(*item);
            }
            self.myActivePlan.m().clear();
            if self.myNet.m().add_person(person) {
                self.check_last_depart();
                self.register_last_depart();
            }
        } else {
            self.myNet.m().route_skipped();
        }
        self.delete_active_plan_and_vehicle_parameter();
    }

    /// C++: `void RORouteHandler::closePersonFlow()` [virtual, protected]
    /// Ends the processing of a personFlow
    // void
    // RORouteHandler::closePersonFlow() {
    //     std::string typeID = DEFAULT_PEDTYPE_ID;
    //     if (myNet.getVehicleTypeSecure(myVehicleParameter->vtypeid) == nullptr) {
    //         myErrorOutput->inform("The vehicle type '" + myVehicleParameter->vtypeid + "' for personFlow '" + myVehicleParameter->id + "' is not known.");
    //     } else {
    //         typeID = myVehicleParameter->vtypeid;
    //     }
    //     if (myActivePlan == nullptr || myActivePlan->empty()) {
    //         WRITE_WARNINGF(TL("Discarding personFlow '%' because their plan is empty"), myVehicleParameter->id);
    //     } else {
    //         checkLastDepart();
    //         // instantiate all persons of this flow
    //         int i = 0;
    //         std::string baseID = myVehicleParameter->id;
    //         if (myWriteFlows) {
    //             addFlowPerson(typeID, myVehicleParameter->depart, baseID, i);
    //         } else if (myVehicleParameter->repetitionProbability > 0) {
    //             if (myVehicleParameter->repetitionEnd == SUMOTime_MAX) {
    //                 throw ProcessError(TLF("probabilistic personFlow '%' must specify end time", myVehicleParameter->id));
    //             } else {
    //                 for (SUMOTime t = myVehicleParameter->depart; t < myVehicleParameter->repetitionEnd; t += TIME2STEPS(1)) {
    //                     if (RandHelper::rand() < myVehicleParameter->repetitionProbability) {
    //                         addFlowPerson(typeID, t, baseID, i++);
    //                     }
    //                 }
    //             }
    //         } else {
    //             SUMOTime depart = myVehicleParameter->depart;
    //             // uniform sampling of departures from range is equivalent to poisson flow (encoded by negative offset)
    //             if (OptionsCont::getOptions().getBool("randomize-flows") && myVehicleParameter->repetitionOffset >= 0) {
    //                 std::vector<SUMOTime> departures;
    //                 const SUMOTime range = myVehicleParameter->repetitionNumber * myVehicleParameter->repetitionOffset;
    //                 for (int j = 0; j < myVehicleParameter->repetitionNumber; ++j) {
    //                     departures.push_back(depart + RandHelper::rand(range));
    //                 }
    //                 std::sort(departures.begin(), departures.end());
    //                 std::reverse(departures.begin(), departures.end());
    //                 for (; i < myVehicleParameter->repetitionNumber; i++) {
    //                     addFlowPerson(typeID, departures[i], baseID, i);
    //                     depart += myVehicleParameter->repetitionOffset;
    //                 }
    //             } else {
    //                 const bool triggered = myVehicleParameter->departProcedure == DepartDefinition::TRIGGERED;
    //                 if (myVehicleParameter->repetitionOffset < 0) {
    //                     // poisson: randomize first depart
    //                     myVehicleParameter->incrementFlow(1);
    //                 }
    //                 for (; i < myVehicleParameter->repetitionNumber && (triggered || depart + myVehicleParameter->repetitionTotalOffset <= myVehicleParameter->repetitionEnd); i++) {
    //                     addFlowPerson(typeID, depart + myVehicleParameter->repetitionTotalOffset, baseID, i);
    //                     if (myVehicleParameter->departProcedure != DepartDefinition::TRIGGERED) {
    //                         myVehicleParameter->incrementFlow(1);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     deleteActivePlanAndVehicleParameter();
    // }
    #[allow(unused_variables)]
    pub fn close_person_flow(&mut self) {
        let mut typeID: String = utils::common::sumo_vehicle_class::DEFAULT_PEDTYPE_ID.to_string();
        if self.myNet.m().get_vehicle_type_secure(&self.myVehicleParameter.vtypeid).is_null() {
            let msg = format!("The vehicle type '{}' for personFlow '{}' is not known.", self.myVehicleParameter.vtypeid, self.myVehicleParameter.id);
            self.myErrorOutput.m().inform(&msg, true);
        } else {
            typeID = self.myVehicleParameter.vtypeid.clone();
        }
        if self.myActivePlan.is_null() || self.myActivePlan.is_empty() {
            write_warningf!("Discarding personFlow '%' because their plan is empty", self.myVehicleParameter.id);
        } else {
            self.check_last_depart();
            // instantiate all persons of this flow
            let mut i: i32 = 0;
            let baseID = self.myVehicleParameter.id.clone();
            if self.myWriteFlows {
                self.add_flow_person(&typeID, self.myVehicleParameter.depart, &baseID, i);
            } else if self.myVehicleParameter.repetitionProbability > 0. {
                if self.myVehicleParameter.repetitionEnd == utils::common::time::SUMOTIME_MAX {
                    throw_process_error(format!("probabilistic personFlow '{}' must specify end time", self.myVehicleParameter.id));
                } else {
                    let mut t = self.myVehicleParameter.depart;
                    while t < self.myVehicleParameter.repetitionEnd {
                        if utils::common::rand_helper::rand() < self.myVehicleParameter.repetitionProbability {
                            self.add_flow_person(&typeID, t, &baseID, i);
                            i += 1;
                        }
                        t += utils::common::time::time2steps(1.);
                    }
                }
            } else {
                let mut depart = self.myVehicleParameter.depart;
                // uniform sampling of departures from range is equivalent to poisson flow (encoded by negative offset)
                if utils::options::options_cont::get_bool("randomize-flows") && self.myVehicleParameter.repetitionOffset >= 0 {
                    let mut departures: Vec<SUMOTime> = Vec::new();
                    let range: SUMOTime = self.myVehicleParameter.repetitionNumber * self.myVehicleParameter.repetitionOffset;
                    for _j in 0..self.myVehicleParameter.repetitionNumber {
                        departures.push(depart + utils::common::rand_helper::with_default(|r| r.rand_i64(range)));
                    }
                    departures.sort();
                    departures.reverse();
                    while i64::from(i) < self.myVehicleParameter.repetitionNumber {
                        self.add_flow_person(&typeID, departures[i as usize], &baseID, i);
                        depart += self.myVehicleParameter.repetitionOffset;
                        i += 1;
                    }
                } else {
                    let triggered = self.myVehicleParameter.departProcedure == DepartDefinition::TRIGGERED;
                    if self.myVehicleParameter.repetitionOffset < 0 {
                        // poisson: randomize first depart
                        self.myVehicleParameter.m().increment_flow(1., Ptr::null());
                    }
                    while i64::from(i) < self.myVehicleParameter.repetitionNumber
                        && (triggered || depart + self.myVehicleParameter.repetitionTotalOffset <= self.myVehicleParameter.repetitionEnd)
                    {
                        self.add_flow_person(&typeID, depart + self.myVehicleParameter.repetitionTotalOffset, &baseID, i);
                        if self.myVehicleParameter.departProcedure != DepartDefinition::TRIGGERED {
                            self.myVehicleParameter.m().increment_flow(1., Ptr::null());
                        }
                        i += 1;
                    }
                }
            }
        }
        self.delete_active_plan_and_vehicle_parameter();
    }

    /// C++: `void RORouteHandler::closeContainer()` [virtual, protected]
    /// Ends the processing of a container
    // void
    // RORouteHandler::closeContainer() {
    //     myActiveContainerPlan->closeTag();
    //     if (myActiveContainerPlanSize > 0) {
    //         myNet.addContainer(myVehicleParameter->depart, myActiveContainerPlan->getString());
    //         checkLastDepart();
    //         registerLastDepart();
    //     } else {
    //         WRITE_WARNINGF(TL("Discarding container '%' because its plan is empty"), myVehicleParameter->id);
    //     }
    //     delete myVehicleParameter;
    //     myVehicleParameter = nullptr;
    //     delete myActiveContainerPlan;
    //     myActiveContainerPlan = nullptr;
    //     myActiveContainerPlanSize = 0;
    // }
    #[allow(unused_variables)]
    pub fn close_container(&mut self) {
        self.myActiveContainerPlan.m().close_tag("");
        if self.myActiveContainerPlanSize > 0 {
            let desc = self.myActiveContainerPlan.get_string();
            self.myNet.m().add_container(self.myVehicleParameter.depart, desc);
            self.check_last_depart();
            self.register_last_depart();
        } else {
            write_warningf!("Discarding container '%' because its plan is empty", self.myVehicleParameter.id);
        }
        delete_ptr(self.myVehicleParameter);
        self.myVehicleParameter = Ptr::null();
        delete_ptr(self.myActiveContainerPlan);
        self.myActiveContainerPlan = Ptr::null();
        self.myActiveContainerPlanSize = 0;
    }

    /// C++: `void RORouteHandler::closeContainerFlow()` [virtual, protected]
    /// Ends the processing of a containerFlow
    // void RORouteHandler::closeContainerFlow() {
    //     myActiveContainerPlan->closeTag();
    //     if (myActiveContainerPlanSize > 0) {
    //         myNet.addContainer(myVehicleParameter->depart, myActiveContainerPlan->getString());
    //         checkLastDepart();
    //         registerLastDepart();
    //     } else {
    //         WRITE_WARNINGF(TL("Discarding containerFlow '%' because its plan is empty"), myVehicleParameter->id);
    //     }
    //     delete myVehicleParameter;
    //     myVehicleParameter = nullptr;
    //     delete myActiveContainerPlan;
    //     myActiveContainerPlan = nullptr;
    //     myActiveContainerPlanSize = 0;
    // }
    #[allow(unused_variables)]
    pub fn close_container_flow(&mut self) {
        self.myActiveContainerPlan.m().close_tag("");
        if self.myActiveContainerPlanSize > 0 {
            let desc = self.myActiveContainerPlan.get_string();
            self.myNet.m().add_container(self.myVehicleParameter.depart, desc);
            self.check_last_depart();
            self.register_last_depart();
        } else {
            write_warningf!("Discarding containerFlow '%' because its plan is empty", self.myVehicleParameter.id);
        }
        delete_ptr(self.myVehicleParameter);
        self.myVehicleParameter = Ptr::null();
        delete_ptr(self.myActiveContainerPlan);
        self.myActiveContainerPlan = Ptr::null();
        self.myActiveContainerPlanSize = 0;
    }

    /// C++: `void RORouteHandler::closeFlow()` [virtual, protected]
    /// Ends the processing of a flow
    // void
    // RORouteHandler::closeFlow() {
    //     checkLastDepart();
    //     // @todo: consider myScale?
    //     if (myVehicleParameter->repetitionNumber == 0) {
    //         delete myVehicleParameter;
    //         myVehicleParameter = nullptr;
    //         return;
    //     }
    //     // let's check whether vehicles had to depart before the simulation starts
    //     myVehicleParameter->repetitionsDone = 0;
    //     const SUMOTime offsetToBegin = myBegin - myVehicleParameter->depart;
    //     while (myVehicleParameter->repetitionTotalOffset < offsetToBegin) {
    //         myVehicleParameter->incrementFlow(1);
    //         if (myVehicleParameter->repetitionsDone == myVehicleParameter->repetitionNumber) {
    //             myNet.routeSkipped();
    //             delete myVehicleParameter;
    //             myVehicleParameter = nullptr;
    //             return;
    //         }
    //     }
    //     if (myNet.getVehicleTypeSecure(myVehicleParameter->vtypeid) == nullptr) {
    //         myErrorOutput->inform("The vehicle type '" + myVehicleParameter->vtypeid + "' for flow '" + myVehicleParameter->id + "' is not known.");
    //     }
    //     if (myVehicleParameter->routeid[0] == '!' && myNet.getRouteDef(myVehicleParameter->routeid) == nullptr) {
    //         closeRoute(true);
    //     }
    //     if (myNet.getRouteDef(myVehicleParameter->routeid) == nullptr) {
    //         myErrorOutput->inform("The route '" + myVehicleParameter->routeid + "' for flow '" + myVehicleParameter->id + "' is not known.");
    //         delete myVehicleParameter;
    //         myVehicleParameter = nullptr;
    //         return;
    //     }
    //     myActiveRouteID = "";
    //     if (!MsgHandler::getErrorInstance()->wasInformed()) {
    //         if (myNet.addFlow(myVehicleParameter, OptionsCont::getOptions().getBool("randomize-flows"))) {
    //             registerLastDepart();
    //         } else {
    //             myErrorOutput->inform("Another flow with the id '" + myVehicleParameter->id + "' exists.");
    //             delete myVehicleParameter;
    //         }
    //     } else {
    //         delete myVehicleParameter;
    //     }
    //     myVehicleParameter = nullptr;
    //     myInsertStopEdgesAt = -1;
    // }
    #[allow(unused_variables)]
    pub fn close_flow(&mut self) {
        self.check_last_depart();
        // @todo: consider myScale?
        if self.myVehicleParameter.repetitionNumber == 0 {
            delete_ptr(self.myVehicleParameter);
            self.myVehicleParameter = Ptr::null();
            return;
        }
        // let's check whether vehicles had to depart before the simulation starts
        self.myVehicleParameter.m().repetitionsDone = 0;
        let offsetToBegin = self.myBegin - self.myVehicleParameter.depart;
        while self.myVehicleParameter.repetitionTotalOffset < offsetToBegin {
            self.myVehicleParameter.m().increment_flow(1., Ptr::null());
            if self.myVehicleParameter.repetitionsDone == self.myVehicleParameter.repetitionNumber {
                self.myNet.m().route_skipped();
                delete_ptr(self.myVehicleParameter);
                self.myVehicleParameter = Ptr::null();
                return;
            }
        }
        if self.myNet.m().get_vehicle_type_secure(&self.myVehicleParameter.vtypeid).is_null() {
            let msg = format!("The vehicle type '{}' for flow '{}' is not known.", self.myVehicleParameter.vtypeid, self.myVehicleParameter.id);
            self.myErrorOutput.m().inform(&msg, true);
        }
        if self.myVehicleParameter.routeid.starts_with('!') && self.myNet.m().get_route_def(&self.myVehicleParameter.routeid).is_null() {
            self.close_route(true);
        }
        if self.myNet.m().get_route_def(&self.myVehicleParameter.routeid).is_null() {
            let msg = format!("The route '{}' for flow '{}' is not known.", self.myVehicleParameter.routeid, self.myVehicleParameter.id);
            self.myErrorOutput.m().inform(&msg, true);
            delete_ptr(self.myVehicleParameter);
            self.myVehicleParameter = Ptr::null();
            return;
        }
        self.myActiveRouteID = String::new();
        if !msg_instance(false).was_informed() {
            if self.myNet.m().add_flow(self.myVehicleParameter, utils::options::options_cont::get_bool("randomize-flows")) {
                self.register_last_depart();
            } else {
                let msg = format!("Another flow with the id '{}' exists.", self.myVehicleParameter.id);
                self.myErrorOutput.m().inform(&msg, true);
                delete_ptr(self.myVehicleParameter);
            }
        } else {
            delete_ptr(self.myVehicleParameter);
        }
        self.myVehicleParameter = Ptr::null();
        self.myInsertStopEdgesAt = -1;
    }

    /// C++: `void RORouteHandler::closeTrip()` [virtual, protected]
    /// Ends the processing of a trip
    // void
    // RORouteHandler::closeTrip() {
    //     closeRoute(true);
    //     closeVehicle();
    // }
    #[allow(unused_variables)]
    pub fn close_trip(&mut self) {
        self.close_route(true);
        self.close_vehicle();
    }

    /// C++: `const SUMOVehicleParameter::Stop * RORouteHandler::retrieveStoppingPlace(const SUMOSAXAttributes &, const std::string &, std::string &, const SUMOVehicleParameter::Stop *)` [protected]
    /// retrieve stopping place element
    // const SUMOVehicleParameter::Stop*
    // RORouteHandler::retrieveStoppingPlace(const SUMOSAXAttributes& attrs, const std::string& errorSuffix, std::string& id, const SUMOVehicleParameter::Stop* stopParam) {
    //     const SUMOVehicleParameter::Stop* toStop = nullptr;
    //     SumoXMLTag tag = SUMO_TAG_NOTHING;
    //     SUMOVehicleParserHelper::parseStopTag(attrs, stopParam, tag, id);
    //     if (tag != SUMO_TAG_NOTHING) {
    //         toStop = myNet.getStoppingPlace(id, tag);
    //         if (toStop == nullptr) {
    //             myErrorOutput->inform(TLF("The % '%' is not known%", toString(tag), id, errorSuffix));
    //         }
    //     }
    //     return toStop;
    // }
    #[allow(unused_variables)]
    pub fn retrieve_stopping_place(&mut self, attrs: &SUMOSAXAttributes, errorSuffix: &str, id: &mut String, stopParam: Ptr<SUMOVehicleParameter_Stop>) -> Ptr<SUMOVehicleParameter_Stop> {
        let mut toStop: Ptr<SUMOVehicleParameter_Stop> = Ptr::null();
        let mut tag: SumoXMLTag = SUMO_TAG_NOTHING;
        SUMOVehicleParserHelper::parse_stop_tag(attrs, stopParam, &mut tag, id);
        if tag != SUMO_TAG_NOTHING {
            toStop = self.myNet.get_stopping_place(id.as_str(), tag);
            if toStop.is_null() {
                let msg = format!("The {} '{}' is not known{}", tag_to_string(tag), id, errorSuffix);
                self.myErrorOutput.m().inform(&msg, true);
            }
        }
        toStop
    }

    /// C++: `Parameterised * RORouteHandler::addStop(const SUMOSAXAttributes &)` [virtual, protected]
    /// Processing of a stop
    // Parameterised*
    // RORouteHandler::addStop(const SUMOSAXAttributes& attrs) {
    //     Parameterised* result = nullptr;
    //     if (myActiveContainerPlan != nullptr) {
    //         myActiveContainerPlan->openTag(SUMO_TAG_STOP);
    //         (*myActiveContainerPlan) << attrs;
    //         myActiveContainerPlan->closeTag();
    //         myActiveContainerPlanSize++;
    //         return result;
    //     }
    //     std::string errorSuffix;
    //     if (myActivePlan != nullptr) {
    //         errorSuffix = " in person '" + myVehicleParameter->id + "'.";
    //     } else if (myActiveContainerPlan != nullptr) {
    //         errorSuffix = " in container '" + myVehicleParameter->id + "'.";
    //     } else if (myVehicleParameter != nullptr) {
    //         errorSuffix = " in vehicle '" + myVehicleParameter->id + "'.";
    //     } else {
    //         errorSuffix = " in route '" + myActiveRouteID + "'.";
    //     }
    //     SUMOVehicleParameter::Stop stop;
    //     bool ok = parseStop(stop, attrs, errorSuffix, myErrorOutput);
    //     if (!ok) {
    //         return result;
    //     }
    //     // try to parse the assigned bus stop
    //     const ROEdge* edge = nullptr;
    //     std::string stoppingPlaceID;
    //     const SUMOVehicleParameter::Stop* stoppingPlace = retrieveStoppingPlace(attrs, errorSuffix, stoppingPlaceID, &stop);
    //     bool hasPos = false;
    //     if (stoppingPlace != nullptr) {
    //         stop.lane = stoppingPlace->lane;
    //         stop.endPos = stoppingPlace->endPos;
    //         stop.startPos = stoppingPlace->startPos;
    //         edge = myNet.getEdge(SUMOXMLDefinitions::getEdgeIDFromLane(stop.lane));
    //     } else {
    //         // no, the lane and the position should be given
    //         stop.lane = attrs.getOpt<std::string>(SUMO_ATTR_LANE, nullptr, ok, "");
    //         stop.edge = attrs.getOpt<std::string>(SUMO_ATTR_EDGE, nullptr, ok, "");
    //         if (ok && stop.edge != "") {
    //             edge = myNet.getEdge(stop.edge);
    //             if (edge == nullptr) {
    //                 myErrorOutput->inform("The edge '" + stop.edge + "' for a stop is not known" + errorSuffix);
    //                 return result;
    //             }
    //         } else if (ok && stop.lane != "") {
    //             edge = myNet.getEdge(SUMOXMLDefinitions::getEdgeIDFromLane(stop.lane));
    //             if (edge == nullptr) {
    //                 myErrorOutput->inform("The lane '" + stop.lane + "' for a stop is not known" + errorSuffix);
    //                 return result;
    //             }
    //         } else if (ok && ((attrs.hasAttribute(SUMO_ATTR_X) && attrs.hasAttribute(SUMO_ATTR_Y))
    //                           || (attrs.hasAttribute(SUMO_ATTR_LON) && attrs.hasAttribute(SUMO_ATTR_LAT)))) {
    //             Position pos;
    //             bool geo = false;
    //             if (attrs.hasAttribute(SUMO_ATTR_X) && attrs.hasAttribute(SUMO_ATTR_Y)) {
    //                 pos = Position(attrs.get<double>(SUMO_ATTR_X, myVehicleParameter->id.c_str(), ok), attrs.get<double>(SUMO_ATTR_Y, myVehicleParameter->id.c_str(), ok));
    //             } else {
    //                 pos = Position(attrs.get<double>(SUMO_ATTR_LON, myVehicleParameter->id.c_str(), ok), attrs.get<double>(SUMO_ATTR_LAT, myVehicleParameter->id.c_str(), ok));
    //                 geo = true;
    //             }
    //             PositionVector positions;
    //             positions.push_back(pos);
    //             ConstROEdgeVector geoEdges;
    //             SUMOVehicleClass vClass = SVC_PASSENGER;
    //             if (!myNet.getVTypeDistribution(myVehicleParameter->vtypeid)) {
    //                 SUMOVTypeParameter* type = myNet.getVehicleTypeSecure(myVehicleParameter->vtypeid);
    //                 if (type != nullptr) {
    //                     vClass = type->vehicleClass;
    //                 }
    //             }
    //             parseGeoEdges(positions, geo, vClass, geoEdges, myVehicleParameter->id, true, ok, true);
    //             if (ok) {
    //                 edge = geoEdges.front();
    //                 hasPos = true;
    //                 if (geo) {
    //                     GeoConvHelper::getFinal().x2cartesian_const(pos);
    //                 }
    //                 stop.parametersSet |= STOP_END_SET;
    //                 stop.endPos = edge->getLanes()[0]->getShape().nearest_offset_to_point2D(pos, false);
    //             } else {
    //                 return result;
    //             }
    //         } else if (!ok || (stop.lane == "" && stop.edge == "")) {
    //             myErrorOutput->inform("A stop must be placed on a bus stop, a container stop, a parking area, an edge or a lane" + errorSuffix);
    //             return result;
    //         }
    //         if (!hasPos) {
    //             stop.endPos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, nullptr, ok, edge->getLength());
    //         }
    //         stop.startPos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, nullptr, ok, stop.endPos - 2 * POSITION_EPS);
    //         const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, nullptr, ok, !attrs.hasAttribute(SUMO_ATTR_STARTPOS) && !attrs.hasAttribute(SUMO_ATTR_ENDPOS));
    //         const double endPosOffset = edge->isInternal() ? edge->getNormalBefore()->getLength() : 0;
    //         if (!ok || (checkStopPos(stop.startPos, stop.endPos, edge->getLength() + endPosOffset, POSITION_EPS, friendlyPos) != SUMORouteHandler::StopPos::STOPPOS_VALID)) {
    //             myErrorOutput->inform("Invalid start or end position for stop" + errorSuffix);
    //             return result;
    //         }
    //     }
    //     stop.edge = edge->getID();
    //     if (myActivePlan != nullptr) {
    //         ROPerson::addStop(*myActivePlan, stop, edge);
    //         result = myActivePlan->back()->getStopParameters();
    //     } else if (myVehicleParameter != nullptr) {
    //         myVehicleParameter->stops.push_back(stop);
    //         result = &myVehicleParameter->stops.back();
    //     } else {
    //         myActiveRouteStops.push_back(stop);
    //         result = &myActiveRouteStops.back();
    //     }
    //     if (myInsertStopEdgesAt >= 0) {
    //         myActiveRoute.insert(myActiveRoute.begin() + myInsertStopEdgesAt, edge);
    //         myInsertStopEdgesAt++;
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn add_stop(&mut self, attrs: &SUMOSAXAttributes) -> Ptr<Parameterised> {
        let mut result: Ptr<Parameterised> = Ptr::null();
        if !self.myActiveContainerPlan.is_null() {
            self.myActiveContainerPlan.m().open_tag_id(SUMO_TAG_STOP);
            self.myActiveContainerPlan.m().write(&attrs.serialize());
            self.myActiveContainerPlan.m().close_tag("");
            self.myActiveContainerPlanSize += 1;
            return result;
        }
        let errorSuffix: String = if !self.myActivePlan.is_null() {
            format!(" in person '{}'.", self.myVehicleParameter.id)
        } else if !self.myActiveContainerPlan.is_null() {
            format!(" in container '{}'.", self.myVehicleParameter.id)
        } else if !self.myVehicleParameter.is_null() {
            format!(" in vehicle '{}'.", self.myVehicleParameter.id)
        } else {
            format!(" in route '{}'.", self.myActiveRouteID)
        };
        let mut stop = SUMOVehicleParameter_Stop::default();
        let eo = self.myErrorOutput;
        let parsed = self.parse_stop(&mut stop, attrs, errorSuffix.clone(), eo);
        let mut ok = parsed;
        if !ok {
            return result;
        }
        // try to parse the assigned bus stop
        let mut edge: Ptr<ROEdge> = Ptr::null();
        let mut stoppingPlaceID = String::new();
        let stoppingPlace = self.retrieve_stopping_place(attrs, &errorSuffix, &mut stoppingPlaceID, Ptr::from_mut(&mut stop));
        let mut hasPos = false;
        if !stoppingPlace.is_null() {
            stop.lane = stoppingPlace.lane.clone();
            stop.endPos = stoppingPlace.endPos;
            stop.startPos = stoppingPlace.startPos;
            edge = self.myNet.get_edge(&edge_id_from_lane(&stop.lane));
        } else {
            // no, the lane and the position should be given
            stop.lane = attrs.get_opt(SUMO_ATTR_LANE, "", &mut ok, String::new(), true);
            stop.edge = attrs.get_opt(SUMO_ATTR_EDGE, "", &mut ok, String::new(), true);
            if ok && stop.edge != "" {
                edge = self.myNet.get_edge(&stop.edge);
                if edge.is_null() {
                    let msg = format!("The edge '{}' for a stop is not known{}", stop.edge, errorSuffix);
                    self.myErrorOutput.m().inform(&msg, true);
                    return result;
                }
            } else if ok && stop.lane != "" {
                edge = self.myNet.get_edge(&edge_id_from_lane(&stop.lane));
                if edge.is_null() {
                    let msg = format!("The lane '{}' for a stop is not known{}", stop.lane, errorSuffix);
                    self.myErrorOutput.m().inform(&msg, true);
                    return result;
                }
            } else if ok && ((attrs.has_attribute(SUMO_ATTR_X) && attrs.has_attribute(SUMO_ATTR_Y)) || (attrs.has_attribute(SUMO_ATTR_LON) && attrs.has_attribute(SUMO_ATTR_LAT))) {
                // shortcut: stop positions given as x/y or lon/lat need PositionVector::nearest_offset_to_point2D and GeoConvHelper, not ported yet
                panic!("addStop: x/y and lon/lat stop positions need the unported nearest_offset_to_point2D and GeoConvHelper");
            } else if !ok || (stop.lane == "" && stop.edge == "") {
                let msg = format!("A stop must be placed on a bus stop, a container stop, a parking area, an edge or a lane{}", errorSuffix);
                self.myErrorOutput.m().inform(&msg, true);
                return result;
            }
            if !hasPos {
                stop.endPos = attrs.get_opt(SUMO_ATTR_ENDPOS, "", &mut ok, edge.get_length(), true);
            }
            stop.startPos = attrs.get_opt(SUMO_ATTR_STARTPOS, "", &mut ok, stop.endPos - 2. * utils::geom::POSITION_EPS, true);
            let friendlyPos: bool = attrs.get_opt(
                SUMO_ATTR_FRIENDLY_POS,
                "",
                &mut ok,
                !attrs.has_attribute(SUMO_ATTR_STARTPOS) && !attrs.has_attribute(SUMO_ATTR_ENDPOS),
                true,
            );
            let endPosOffset = if edge.is_internal() { edge.get_normal_before().get_length() } else { 0. };
            if !ok || (SUMORouteHandler::check_stop_pos(&mut stop.startPos, &mut stop.endPos, edge.get_length() + endPosOffset, utils::geom::POSITION_EPS, friendlyPos) != SUMORouteHandler_StopPos::STOPPOS_VALID) {
                let msg = format!("Invalid start or end position for stop{}", errorSuffix);
                self.myErrorOutput.m().inform(&msg, true);
                return result;
            }
        }
        stop.edge = edge.get_id().to_string();
        if !self.myActivePlan.is_null() {
            ROPerson::add_stop(self.myActivePlan.m(), &stop, edge);
            let back = *self.myActivePlan.m().last().unwrap();
            result = back.m().get_stop_parameters().cast::<Parameterised>();
        } else if !self.myVehicleParameter.is_null() {
            self.myVehicleParameter.m().stops.push(stop);
            result = Ptr::from_mut(self.myVehicleParameter.m().stops.last_mut().unwrap()).cast::<Parameterised>();
        } else {
            self.myActiveRouteStops.push(stop);
            result = Ptr::from_mut(self.myActiveRouteStops.last_mut().unwrap()).cast::<Parameterised>();
        }
        if self.myInsertStopEdgesAt >= 0 {
            let pos = self.myInsertStopEdgesAt as usize;
            self.myActiveRoute.insert(pos, edge);
            self.myInsertStopEdgesAt += 1;
        }
        result
    }

    /// C++: `void RORouteHandler::addFlowPerson(const std::string &, SUMOTime, const std::string &, int)` [protected]
    /// Processing of a person from a personFlow
    // void
    // RORouteHandler::addFlowPerson(const std::string& typeID, SUMOTime depart, const std::string& baseID, int i) {
    //     if (myVehicleParameter->departProcedure == DepartDefinition::GIVEN && depart < myBegin) {
    //         myNet.routeSkipped();
    //         return;
    //     }
    //     SUMOVehicleParameter pars = *myVehicleParameter;
    //     pars.id = baseID + "." + toString(i);
    //     pars.depart = depart;
    //     const SUMOVTypeParameter* const type = myNet.getVehicleTypeSecure(typeID);
    //     if (!myKeepVTypeDist) {
    //         pars.vtypeid = type->id;
    //     }
    //     ROPerson* person = new ROPerson(pars, type);
    //     for (ROPerson::PlanItem* item : *myActivePlan) {
    //         person->getPlan().push_back(item->clone());
    //     }
    //     if (myNet.addPerson(person)) {
    //         if (i == 0) {
    //             registerLastDepart();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_flow_person(&mut self, typeID: &str, depart: SUMOTime, baseID: &str, i: i32) {
        if self.myVehicleParameter.departProcedure == DepartDefinition::GIVEN && depart < self.myBegin {
            self.myNet.m().route_skipped();
            return;
        }
        let mut pars: SUMOVehicleParameter = (*self.myVehicleParameter).clone();
        pars.id = format!("{}.{}", baseID, i);
        pars.depart = depart;
        let type_ = self.myNet.m().get_vehicle_type_secure(typeID);
        if !self.myKeepVTypeDist {
            pars.vtypeid = type_.id.clone();
        }
        let person: Ptr<ROPerson> = Ptr::from_raw(Box::into_raw(Box::new(ROPerson::new(&pars, type_))));
        for item in self.myActivePlan.m().iter() {
            let planItem: Ptr<ROPerson_PlanItem> = *item;
            person.m().get_plan().push(ROPerson_PlanItem::clone(planItem.get().unwrap()));
        }
        if self.myNet.m().add_person(person) {
            if i == 0 {
                self.register_last_depart();
            }
        }
    }

    /// C++: `void RORouteHandler::addRide(const SUMOSAXAttributes &)` [virtual, protected]
    /// Processing of a ride
    // void
    // RORouteHandler::addRide(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::vector<ROPerson::PlanItem*>& plan = *myActivePlan;
    //     const std::string pid = myVehicleParameter->id;
    //
    //     const ROEdge* from = nullptr;
    //     const ROEdge* to = nullptr;
    //     parseFromViaTo(SUMO_TAG_PERSON, attrs, ok);
    //     if (attrs.hasAttribute(SUMO_ATTR_FROM) || attrs.hasAttribute(SUMO_ATTR_FROM_JUNCTION) || attrs.hasAttribute(SUMO_ATTR_FROM_TAZ)
    //             || attrs.hasAttribute(SUMO_ATTR_FROMLONLAT) || attrs.hasAttribute(SUMO_ATTR_FROMXY)) {
    //         if (ok) {
    //             from = myActiveRoute.front();
    //         }
    //     } else if (plan.empty()) {
    //         myErrorOutput->inform("The start edge for person '" + pid + "' is not known.");
    //         return;
    //     }
    //     std::string stoppingPlaceID;
    //     const SUMOVehicleParameter::Stop* stop = retrieveStoppingPlace(attrs, " for ride of person '" + myVehicleParameter->id + "'", stoppingPlaceID);
    //     if (stop != nullptr) {
    //         to = myNet.getEdge(SUMOXMLDefinitions::getEdgeIDFromLane(stop->lane));
    //     } else {
    //         if (attrs.hasAttribute(SUMO_ATTR_TO) || attrs.hasAttribute(SUMO_ATTR_TO_JUNCTION) || attrs.hasAttribute(SUMO_ATTR_TO_TAZ)
    //                 || attrs.hasAttribute(SUMO_ATTR_TOLONLAT) || attrs.hasAttribute(SUMO_ATTR_TOXY)) {
    //             to = myActiveRoute.back();
    //         } else {
    //             myErrorOutput->inform("The to edge is missing within a ride of '" + myVehicleParameter->id + "'.");
    //             return;
    //         }
    //     }
    //     double arrivalPos = attrs.getOpt<double>(SUMO_ATTR_ARRIVALPOS, myVehicleParameter->id.c_str(), ok,
    //                         stop == nullptr ? std::numeric_limits<double>::infinity() : stop->endPos);
    //     const std::string lines = attrs.getOpt<std::string>(SUMO_ATTR_LINES, pid.c_str(), ok, LINE_ANY);
    //     const std::string group = attrs.getOpt<std::string>(SUMO_ATTR_GROUP, pid.c_str(), ok, "");
    //
    //     if (plan.empty() && myVehicleParameter->departProcedure == DepartDefinition::TRIGGERED) {
    //         StringTokenizer st(lines);
    //         if (st.size() != 1 || st.get(0) == LINE_ANY) {
    //             myErrorOutput->inform("Triggered departure for person '" + pid + "' requires a unique lines value.");
    //             return;
    //         }
    //         const std::string vehID = st.front();
    //         if (myNet.knowsVehicle(vehID)) {
    //             const SUMOTime vehDepart = myNet.getDeparture(vehID);
    //             if (vehDepart == -1) {
    //                 myErrorOutput->inform("Cannot use triggered vehicle '" + vehID + "' in triggered departure for person '" + pid + "'.");
    //                 return;
    //             }
    //             myVehicleParameter->depart = vehDepart + 1; // write person after vehicle
    //         } else {
    //             if (mySkippedVehicles.count(vehID) == 0) {
    //                 myErrorOutput->inform("Unknown vehicle '" + vehID + "' in triggered departure for person '" + pid + "'.");
    //                 return;
    //             }
    //             myVehicleParameter->departProcedure = DepartDefinition::GIVEN; // make sure the person gets skipped due to depart time
    //         }
    //     }
    //     ROPerson::addRide(plan, from, to, lines, arrivalPos, stoppingPlaceID, group);
    // }
    #[allow(unused_variables)]
    pub fn add_ride(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let planPtr = self.myActivePlan;
        let pid = self.myVehicleParameter.id.clone();

        let mut from: Ptr<ROEdge> = Ptr::null();
        let mut to: Ptr<ROEdge> = Ptr::null();
        self.parse_from_via_to(SUMO_TAG_PERSON, attrs, &mut ok);
        if attrs.has_attribute(SUMO_ATTR_FROM) || attrs.has_attribute(SUMO_ATTR_FROM_JUNCTION) || attrs.has_attribute(SUMO_ATTR_FROM_TAZ) || attrs.has_attribute(SUMO_ATTR_FROMLONLAT) || attrs.has_attribute(SUMO_ATTR_FROMXY) {
            if ok {
                from = self.myActiveRoute[0];
            }
        } else if planPtr.m().is_empty() {
            let msg = format!("The start edge for person '{}' is not known.", pid);
            self.myErrorOutput.m().inform(&msg, true);
            return;
        }
        let mut stoppingPlaceID = String::new();
        let errorSuffix = format!(" for ride of person '{}'", self.myVehicleParameter.id);
        let stop = self.retrieve_stopping_place(attrs, &errorSuffix, &mut stoppingPlaceID, Ptr::null());
        if !stop.is_null() {
            to = self.myNet.get_edge(&edge_id_from_lane(&stop.lane));
        } else {
            if attrs.has_attribute(SUMO_ATTR_TO) || attrs.has_attribute(SUMO_ATTR_TO_JUNCTION) || attrs.has_attribute(SUMO_ATTR_TO_TAZ) || attrs.has_attribute(SUMO_ATTR_TOLONLAT) || attrs.has_attribute(SUMO_ATTR_TOXY) {
                to = *self.myActiveRoute.last().unwrap();
            } else {
                let msg = format!("The to edge is missing within a ride of '{}'.", self.myVehicleParameter.id);
                self.myErrorOutput.m().inform(&msg, true);
                return;
            }
        }
        let arrivalPosDefault: f64 = if stop.is_null() { f64::INFINITY } else { stop.endPos };
        let arrivalPos: f64 = attrs.get_opt(SUMO_ATTR_ARRIVALPOS, &pid, &mut ok, arrivalPosDefault, true);
        let lines: String = attrs.get_opt(SUMO_ATTR_LINES, &pid, &mut ok, LINE_ANY.to_string(), true);
        let group: String = attrs.get_opt(SUMO_ATTR_GROUP, &pid, &mut ok, String::new(), true);

        if planPtr.m().is_empty() && self.myVehicleParameter.departProcedure == DepartDefinition::TRIGGERED {
            let mut st = StringTokenizer::new(&lines);
            let mut tokens: Vec<String> = Vec::new();
            while st.has_next() {
                tokens.push(st.next().unwrap());
            }
            if tokens.len() != 1 || tokens[0] == LINE_ANY {
                let msg = format!("Triggered departure for person '{}' requires a unique lines value.", pid);
                self.myErrorOutput.m().inform(&msg, true);
                return;
            }
            let vehID = tokens[0].clone();
            if self.myNet.knows_vehicle(&vehID) {
                let vehDepart = self.myNet.get_departure(&vehID);
                if vehDepart == -1 {
                    let msg = format!("Cannot use triggered vehicle '{}' in triggered departure for person '{}'.", vehID, pid);
                    self.myErrorOutput.m().inform(&msg, true);
                    return;
                }
                self.myVehicleParameter.m().depart = vehDepart + 1; // write person after vehicle
            } else {
                if !self.mySkippedVehicleIDs.contains(&vehID) {
                    let msg = format!("Unknown vehicle '{}' in triggered departure for person '{}'.", vehID, pid);
                    self.myErrorOutput.m().inform(&msg, true);
                    return;
                }
                // make sure the person gets skipped due to depart time
                self.myVehicleParameter.m().departProcedure = DepartDefinition::GIVEN;
            }
        }
        ROPerson::add_ride(planPtr.m(), from, to, &lines, arrivalPos, &stoppingPlaceID, &group);
    }

    /// C++: `void RORouteHandler::addTransport(const SUMOSAXAttributes &)` [virtual, protected]
    /// Processing of a transport
    // void
    // RORouteHandler::addTransport(const SUMOSAXAttributes& attrs) {
    //     if (myActiveContainerPlan != nullptr && myActiveContainerPlanSize == 0 && myVehicleParameter->departProcedure == DepartDefinition::TRIGGERED) {
    //         bool ok = true;
    //         const std::string pid = myVehicleParameter->id;
    //         const std::string desc = attrs.get<std::string>(SUMO_ATTR_LINES, pid.c_str(), ok);
    //         StringTokenizer st(desc);
    //         if (st.size() != 1) {
    //             throw ProcessError(TLF("Triggered departure for container '%' requires a unique lines value.", pid));
    //         }
    //         const std::string vehID = st.front();
    //         if (!myNet.knowsVehicle(vehID)) {
    //             if (mySkippedVehicles.count(vehID) == 0) {
    //                 throw ProcessError("Unknown vehicle '" + vehID + "' in triggered departure for container '" + pid + "'.");
    //             }
    //             return;
    //         }
    //         SUMOTime vehDepart = myNet.getDeparture(vehID);
    //         if (vehDepart == -1) {
    //             throw ProcessError("Cannot use triggered vehicle '" + vehID + "' in triggered departure for container '" + pid + "'.");
    //         }
    //         myVehicleParameter->depart = vehDepart + 1; // write container after vehicle
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_transport(&mut self, attrs: &SUMOSAXAttributes) {
        if !self.myActiveContainerPlan.is_null() && self.myActiveContainerPlanSize == 0 && self.myVehicleParameter.departProcedure == DepartDefinition::TRIGGERED {
            let mut ok = true;
            let pid = self.myVehicleParameter.id.clone();
            let desc: String = attrs.get(SUMO_ATTR_LINES, &pid, &mut ok, true);
            let mut st = StringTokenizer::new(&desc);
            let mut tokens: Vec<String> = Vec::new();
            while st.has_next() {
                tokens.push(st.next().unwrap());
            }
            if tokens.len() != 1 {
                throw_process_error(format!("Triggered departure for container '{}' requires a unique lines value.", pid));
            }
            let vehID = tokens[0].clone();
            if !self.myNet.knows_vehicle(&vehID) {
                if !self.mySkippedVehicleIDs.contains(&vehID) {
                    throw_process_error(format!("Unknown vehicle '{}' in triggered departure for container '{}'.", vehID, pid));
                }
                return;
            }
            let vehDepart = self.myNet.get_departure(&vehID);
            if vehDepart == -1 {
                throw_process_error(format!("Cannot use triggered vehicle '{}' in triggered departure for container '{}'.", vehID, pid));
            }
            self.myVehicleParameter.m().depart = vehDepart + 1; // write container after vehicle
        }
    }

    /// C++: `void RORouteHandler::addTranship(const SUMOSAXAttributes &)` [virtual, protected]
    /// Processing of a tranship
    // void
    // RORouteHandler::addTranship(const SUMOSAXAttributes& /*attrs*/) {
    // }
    #[allow(unused_variables)]
    pub fn add_tranship(&mut self, attrs: &SUMOSAXAttributes) {
        // C++ body is empty
    }

    /// C++: `void RORouteHandler::parseEdges(const std::string &, ConstROEdgeVector &, const std::string &, bool &)` [protected]
    /// Parse edges from strings
    // void
    // RORouteHandler::parseEdges(const std::string& desc, ConstROEdgeVector& into,
    //                            const std::string& rid, bool& ok) {
    //     for (StringTokenizer st(desc); st.hasNext();) {
    //         const std::string id = st.next();
    //         const ROEdge* edge = myNet.getEdge(id);
    //         if (edge == nullptr) {
    //             myErrorOutput->inform("The edge '" + id + "' within the route " + rid + " is not known.");
    //             ok = false;
    //         } else {
    //             into.push_back(edge);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_edges(&mut self, desc: &str, into: &mut ConstROEdgeVector, rid: &str, ok: &mut bool) {
        let mut st = StringTokenizer::new(desc);
        while st.has_next() {
            let id = st.next().unwrap();
            let edge = self.myNet.get_edge(&id);
            if edge.is_null() {
                let msg = format!("The edge '{}' within the route {} is not known.", id, rid);
                self.myErrorOutput.m().inform(&msg, true);
                *ok = false;
            } else {
                into.push(edge);
            }
        }
    }

    /// C++: `void RORouteHandler::addPersonTrip(const SUMOSAXAttributes &)` [virtual, protected]
    /// add a routing request for a walking or intermodal person
    // void
    // RORouteHandler::addPersonTrip(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const char* const id = myVehicleParameter->id.c_str();
    //     assert(!attrs.hasAttribute(SUMO_ATTR_EDGES));
    //     const ROEdge* from = nullptr;
    //     const ROEdge* to = nullptr;
    //     parseFromViaTo(SUMO_TAG_PERSON, attrs, ok);
    //     myInsertStopEdgesAt = -1;
    //     if (attrs.hasAttribute(SUMO_ATTR_FROM) || attrs.hasAttribute(SUMO_ATTR_FROM_JUNCTION) || attrs.hasAttribute(SUMO_ATTR_FROM_TAZ)
    //             || attrs.hasAttribute(SUMO_ATTR_FROMLONLAT) || attrs.hasAttribute(SUMO_ATTR_FROMXY)) {
    //         if (ok) {
    //             from = myActiveRoute.front();
    //         }
    //     } else if (myActivePlan->empty()) {
    //         throw ProcessError(TLF("Start edge not defined for person '%'.", myVehicleParameter->id));
    //     } else {
    //         from = myActivePlan->back()->getDestination();
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_TO) || attrs.hasAttribute(SUMO_ATTR_TO_JUNCTION) || attrs.hasAttribute(SUMO_ATTR_TO_TAZ)
    //             || attrs.hasAttribute(SUMO_ATTR_TOLONLAT) || attrs.hasAttribute(SUMO_ATTR_TOXY)) {
    //         to = myActiveRoute.back();
    //     } // else, to may also be derived from stopping place
    //
    //     const SUMOTime duration = attrs.getOptSUMOTimeReporting(SUMO_ATTR_DURATION, id, ok, -1);
    //     if (attrs.hasAttribute(SUMO_ATTR_DURATION) && duration <= 0) {
    //         throw ProcessError(TLF("Non-positive walking duration for '%'.", myVehicleParameter->id));
    //     }
    //
    //     double departPos = 0;
    //     double arrivalPos = std::numeric_limits<double>::infinity();
    //     std::string busStopID;
    //     const ROPerson::PlanItem* const lastStage = myActivePlan->empty() ? nullptr : myActivePlan->back();
    //     parseWalkPositions(attrs, myVehicleParameter->id, from, to, departPos, arrivalPos, busStopID, lastStage, ok);
    //
    //     const std::string modes = attrs.getOpt<std::string>(SUMO_ATTR_MODES, id, ok, "");
    //     const std::string group = attrs.getOpt<std::string>(SUMO_ATTR_GROUP, id, ok, "");
    //     SVCPermissions modeSet = 0;
    //     for (StringTokenizer st(modes); st.hasNext();) {
    //         const std::string mode = st.next();
    //         if (mode == "car") {
    //             modeSet |= SVC_PASSENGER;
    //         } else if (mode == "taxi") {
    //             modeSet |= SVC_TAXI;
    //         } else if (mode == "bicycle") {
    //             modeSet |= SVC_BICYCLE;
    //         } else if (mode == "public") {
    //             modeSet |= SVC_BUS;
    //         } else {
    //             throw InvalidArgument("Unknown person mode '" + mode + "'.");
    //         }
    //     }
    //     const std::string types = attrs.getOpt<std::string>(SUMO_ATTR_VTYPES, id, ok, "");
    //     double walkFactor = attrs.getOpt<double>(SUMO_ATTR_WALKFACTOR, id, ok, OptionsCont::getOptions().getFloat("persontrip.walkfactor"));
    //     if (ok) {
    //         const std::string originStopID = myActivePlan->empty() ?  "" : myActivePlan->back()->getStopDest();
    //         ROPerson::addTrip(*myActivePlan, myVehicleParameter->id, from, to, modeSet, types,
    //                           departPos, originStopID, arrivalPos, busStopID, walkFactor, group);
    //         myParamStack.push_back(myActivePlan->back());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_person_trip(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let id = self.myVehicleParameter.id.clone();
        let mut from: Ptr<ROEdge> = Ptr::null();
        let mut to: Ptr<ROEdge> = Ptr::null();
        self.parse_from_via_to(SUMO_TAG_PERSON, attrs, &mut ok);
        self.myInsertStopEdgesAt = -1;
        let planPtr = self.myActivePlan;
        if attrs.has_attribute(SUMO_ATTR_FROM) || attrs.has_attribute(SUMO_ATTR_FROM_JUNCTION) || attrs.has_attribute(SUMO_ATTR_FROM_TAZ) || attrs.has_attribute(SUMO_ATTR_FROMLONLAT) || attrs.has_attribute(SUMO_ATTR_FROMXY) {
            if ok {
                from = self.myActiveRoute[0];
            }
        } else if planPtr.m().is_empty() {
            throw_process_error(format!("Start edge not defined for person '{}'.", self.myVehicleParameter.id));
        } else {
            from = planPtr.m().last().unwrap().get_destination();
        }
        if attrs.has_attribute(SUMO_ATTR_TO) || attrs.has_attribute(SUMO_ATTR_TO_JUNCTION) || attrs.has_attribute(SUMO_ATTR_TO_TAZ) || attrs.has_attribute(SUMO_ATTR_TOLONLAT) || attrs.has_attribute(SUMO_ATTR_TOXY) {
            to = *self.myActiveRoute.last().unwrap();
        } // else, to may also be derived from stopping place

        let duration: SUMOTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_DURATION, &id, &mut ok, -1, true);
        if attrs.has_attribute(SUMO_ATTR_DURATION) && duration <= 0 {
            throw_process_error(format!("Non-positive walking duration for '{}'.", self.myVehicleParameter.id));
        }

        let mut departPos: f64 = 0.;
        let mut arrivalPos: f64 = f64::INFINITY;
        let mut busStopID = String::new();
        let lastStage: Ptr<ROPerson_PlanItem> = if planPtr.m().is_empty() { Ptr::null() } else { *planPtr.m().last().unwrap() };
        self.parse_walk_positions(attrs, &id, from, &mut to, &mut departPos, &mut arrivalPos, &mut busStopID, lastStage, &mut ok);

        let modes: String = attrs.get_opt(SUMO_ATTR_MODES, &id, &mut ok, String::new(), true);
        let group: String = attrs.get_opt(SUMO_ATTR_GROUP, &id, &mut ok, String::new(), true);
        let mut modeSet: SVCPermissions = 0;
        let mut st = StringTokenizer::new(&modes);
        while st.has_next() {
            let mode = st.next().unwrap();
            if mode == "car" {
                modeSet |= SVC_PASSENGER;
            } else if mode == "taxi" {
                modeSet |= SVC_TAXI;
            } else if mode == "bicycle" {
                modeSet |= SVC_BICYCLE;
            } else if mode == "public" {
                modeSet |= SVC_BUS;
            } else {
                std::panic::panic_any(ProcessError::InvalidArgument(format!("Unknown person mode '{}'.", mode)));
            }
        }
        let types: String = attrs.get_opt(SUMO_ATTR_VTYPES, &id, &mut ok, String::new(), true);
        let walkFactor: f64 = attrs.get_opt(SUMO_ATTR_WALKFACTOR, &id, &mut ok, utils::options::options_cont::get_float("persontrip.walkfactor"), true);
        if ok {
            let originStopID: String = if planPtr.m().is_empty() { String::new() } else { planPtr.m().last().unwrap().get_stop_dest().to_string() };
            ROPerson::add_trip(planPtr.m(), &id, from, to, modeSet, &types, departPos, &originStopID, arrivalPos, &busStopID, walkFactor, &group);
            // shortcut: myParamStack is an i32 counter in the skeleton, C++ pushes the new plan item pointer onto it
        }
    }

    /// C++: `void RORouteHandler::addWalk(const SUMOSAXAttributes &)` [virtual, protected]
    /// add a fully specified walk
    // void
    // RORouteHandler::addWalk(const SUMOSAXAttributes& attrs) {
    //     // parse walks from->to as person trips
    //     if (attrs.hasAttribute(SUMO_ATTR_EDGES) || attrs.hasAttribute(SUMO_ATTR_ROUTE)) {
    //         // XXX allow --repair?
    //         bool ok = true;
    //         if (attrs.hasAttribute(SUMO_ATTR_ROUTE)) {
    //             const std::string routeID = attrs.get<std::string>(SUMO_ATTR_ROUTE, myVehicleParameter->id.c_str(), ok);
    //             RORouteDef* routeDef = myNet.getRouteDef(routeID);
    //             const std::shared_ptr<const RORoute> route = routeDef != nullptr ? routeDef->getFirstRoute() : nullptr;
    //             if (route == nullptr) {
    //                 throw ProcessError("The route '" + routeID + "' for walk of person '" + myVehicleParameter->id + "' is not known.");
    //             }
    //             myActiveRoute = route->getEdgeVector();
    //         } else {
    //             myActiveRoute.clear();
    //             parseEdges(attrs.get<std::string>(SUMO_ATTR_EDGES, myVehicleParameter->id.c_str(), ok), myActiveRoute, " walk for person '" + myVehicleParameter->id + "'", ok);
    //         }
    //         const char* const objId = myVehicleParameter->id.c_str();
    //         const double duration = attrs.getOpt<double>(SUMO_ATTR_DURATION, objId, ok, -1);
    //         if (attrs.hasAttribute(SUMO_ATTR_DURATION) && duration <= 0) {
    //             throw ProcessError(TLF("Non-positive walking duration for '%'.", myVehicleParameter->id));
    //         }
    //         const double speed = attrs.getOpt<double>(SUMO_ATTR_SPEED, objId, ok, -1.);
    //         if (attrs.hasAttribute(SUMO_ATTR_SPEED) && speed <= 0) {
    //             throw ProcessError(TLF("Non-positive walking speed for '%'.", myVehicleParameter->id));
    //         }
    //         double departPos = 0.;
    //         double arrivalPos = std::numeric_limits<double>::infinity();
    //         if (attrs.hasAttribute(SUMO_ATTR_DEPARTPOS)) {
    //             WRITE_WARNING(TL("The attribute departPos is no longer supported for walks, please use the person attribute, the arrivalPos of the previous step or explicit stops."));
    //         }
    //         if (attrs.hasAttribute(SUMO_ATTR_ARRIVALPOS)) {
    //             arrivalPos = SUMOVehicleParserHelper::parseWalkPos(SUMO_ATTR_ARRIVALPOS, myHardFail, objId, myActiveRoute.back()->getLength(), attrs.get<std::string>(SUMO_ATTR_ARRIVALPOS, objId, ok));
    //         }
    //         std::string stoppingPlaceID;
    //         const std::string errorSuffix = " for walk of person '" + myVehicleParameter->id + "'";
    //         retrieveStoppingPlace(attrs, errorSuffix, stoppingPlaceID);
    //         if (ok) {
    //             ROPerson::addWalk(*myActivePlan, myActiveRoute, duration, speed, departPos, arrivalPos, stoppingPlaceID);
    //             myParamStack.push_back(myActivePlan->back());
    //         }
    //     } else {
    //         addPersonTrip(attrs);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_walk(&mut self, attrs: &SUMOSAXAttributes) {
        // parse walks from->to as person trips
        if attrs.has_attribute(SUMO_ATTR_EDGES) || attrs.has_attribute(SUMO_ATTR_ROUTE) {
            // XXX allow --repair?
            let mut ok = true;
            if attrs.has_attribute(SUMO_ATTR_ROUTE) {
                let routeID: String = attrs.get(SUMO_ATTR_ROUTE, &self.myVehicleParameter.id, &mut ok, true);
                let routeDef = self.myNet.get_route_def(&routeID);
                let route: Option<Rc<RORoute>> = if !routeDef.is_null() { Some(routeDef.get_first_route()) } else { None };
                match route {
                    None => throw_process_error(format!(
                        "The route '{}' for walk of person '{}' is not known.",
                        routeID, self.myVehicleParameter.id
                    )),
                    Some(r) => {
                        self.myActiveRoute = r.get_edge_vector().clone();
                    }
                }
            } else {
                self.myActiveRoute.clear();
                let desc: String = attrs.get(SUMO_ATTR_EDGES, &self.myVehicleParameter.id, &mut ok, true);
                let rid = format!(" walk for person '{}'", self.myVehicleParameter.id);
                let mut route = std::mem::take(&mut self.myActiveRoute);
                self.parse_edges(&desc, &mut route, &rid, &mut ok);
                self.myActiveRoute = route;
            }
            let objId = self.myVehicleParameter.id.clone();
            let duration: f64 = attrs.get_opt(SUMO_ATTR_DURATION, &objId, &mut ok, -1., true);
            if attrs.has_attribute(SUMO_ATTR_DURATION) && duration <= 0. {
                throw_process_error(format!("Non-positive walking duration for '{}'.", self.myVehicleParameter.id));
            }
            let speed: f64 = attrs.get_opt(SUMO_ATTR_SPEED, &objId, &mut ok, -1., true);
            if attrs.has_attribute(SUMO_ATTR_SPEED) && speed <= 0. {
                throw_process_error(format!("Non-positive walking speed for '{}'.", self.myVehicleParameter.id));
            }
            let mut departPos: f64 = 0.;
            let mut arrivalPos: f64 = f64::INFINITY;
            if attrs.has_attribute(SUMO_ATTR_DEPARTPOS) {
                write_warning("The attribute departPos is no longer supported for walks, please use the person attribute, the arrivalPos of the previous step or explicit stops.");
            }
            if attrs.has_attribute(SUMO_ATTR_ARRIVALPOS) {
                let lastLength = self.myActiveRoute.last().unwrap().get_length();
                let val: String = attrs.get(SUMO_ATTR_ARRIVALPOS, &objId, &mut ok, true);
                arrivalPos = SUMOVehicleParserHelper::parse_walk_pos(SUMO_ATTR_ARRIVALPOS, self.myHardFail, &objId, lastLength, &val, SUMORouteHandler::get_parsing_rng());
            }
            let mut stoppingPlaceID = String::new();
            let errorSuffix = format!(" for walk of person '{}'", self.myVehicleParameter.id);
            self.retrieve_stopping_place(attrs, &errorSuffix, &mut stoppingPlaceID, Ptr::null());
            if ok {
                let planPtr = self.myActivePlan;
                ROPerson::add_walk(planPtr.m(), &self.myActiveRoute, duration, speed, departPos, arrivalPos, &stoppingPlaceID);
                // shortcut: myParamStack is an i32 counter in the skeleton, C++ pushes the new plan item pointer onto it
            }
        } else {
            self.add_person_trip(attrs);
        }
    }

    /// C++: `void RORouteHandler::parseWalkPositions(const SUMOSAXAttributes &, const std::string &, const ROEdge *, const ROEdge *&, double &, double &, std::string &, const ROPerson::PlanItem *const, bool &)` [protected]
    /// @ brief parse depart- and arrival positions of a walk
    // void
    // RORouteHandler::parseWalkPositions(const SUMOSAXAttributes& attrs, const std::string& personID,
    //                                    const ROEdge* /*fromEdge*/, const ROEdge*& toEdge,
    //                                    double& departPos, double& arrivalPos, std::string& busStopID,
    //                                    const ROPerson::PlanItem* const lastStage, bool& ok) {
    //     const std::string description = "walk or personTrip of '" + personID + "'.";
    //     if (attrs.hasAttribute(SUMO_ATTR_DEPARTPOS)) {
    //         WRITE_WARNING(TL("The attribute departPos is no longer supported for walks, please use the person attribute, the arrivalPos of the previous step or explicit stops."));
    //     }
    //     departPos = myVehicleParameter->departPos;
    //     if (lastStage != nullptr) {
    //         departPos = lastStage->getDestinationPos();
    //     }
    //
    //     busStopID = attrs.getOpt<std::string>(SUMO_ATTR_BUS_STOP, nullptr, ok, "");
    //
    //     const SUMOVehicleParameter::Stop* bs = retrieveStoppingPlace(attrs, description, busStopID);
    //     if (bs != nullptr) {
    //         toEdge = myNet.getEdge(SUMOXMLDefinitions::getEdgeIDFromLane(bs->lane));
    //         arrivalPos = (bs->startPos + bs->endPos) / 2;
    //     }
    //     if (toEdge != nullptr) {
    //         if (attrs.hasAttribute(SUMO_ATTR_ARRIVALPOS)) {
    //             arrivalPos = SUMOVehicleParserHelper::parseWalkPos(SUMO_ATTR_ARRIVALPOS,
    //                          myHardFail, description, toEdge->getLength(),
    //                          attrs.get<std::string>(SUMO_ATTR_ARRIVALPOS, description.c_str(), ok));
    //         }
    //     } else {
    //         throw ProcessError(TLF("No destination edge for %.", description));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_walk_positions(&mut self, attrs: &SUMOSAXAttributes, personID: &str, fromEdge: Ptr<ROEdge>, toEdge: &mut Ptr<ROEdge>, departPos: &mut f64, arrivalPos: &mut f64, busStopID: &mut String, lastStage: Ptr<ROPerson_PlanItem>, ok: &mut bool) {
        let description = format!("walk or personTrip of '{}'.", personID);
        if attrs.has_attribute(SUMO_ATTR_DEPARTPOS) {
            write_warning("The attribute departPos is no longer supported for walks, please use the person attribute, the arrivalPos of the previous step or explicit stops.");
        }
        *departPos = self.myVehicleParameter.departPos;
        if !lastStage.is_null() {
            *departPos = lastStage.get_destination_pos();
        }

        *busStopID = attrs.get_opt(SUMO_ATTR_BUS_STOP, "", ok, String::new(), true);

        let bs = self.retrieve_stopping_place(attrs, &description, busStopID, Ptr::null());
        if !bs.is_null() {
            *toEdge = self.myNet.get_edge(&edge_id_from_lane(&bs.lane));
            *arrivalPos = (bs.startPos + bs.endPos) / 2.;
        }
        if !toEdge.is_null() {
            if attrs.has_attribute(SUMO_ATTR_ARRIVALPOS) {
                let val: String = attrs.get(SUMO_ATTR_ARRIVALPOS, &description, ok, true);
                *arrivalPos = SUMOVehicleParserHelper::parse_walk_pos(
                    SUMO_ATTR_ARRIVALPOS,
                    self.myHardFail,
                    &description,
                    toEdge.get_length(),
                    &val,
                    SUMORouteHandler::get_parsing_rng(),
                );
            }
        } else {
            throw_process_error(format!("No destination edge for {}.", description));
        }
    }

    /// C++: `void RORouteHandler::initLaneTree(NamedRTree *)` [virtual, protected]
    // void
    // RORouteHandler::initLaneTree(NamedRTree* tree) {
    //     for (const auto& edgeItem : myNet.getEdgeMap()) {
    //         for (ROLane* lane : edgeItem.second->getLanes()) {
    //             Boundary b = lane->getShape().getBoxBoundary();
    //             const float cmin[2] = {(float) b.xmin(), (float) b.ymin()};
    //             const float cmax[2] = {(float) b.xmax(), (float) b.ymax()};
    //             tree->Insert(cmin, cmax, lane);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_lane_tree(&mut self, tree: Ptr<Unmapped /* NamedRTree */>) {
        // shortcut: NamedRTree is not ported, lanes cannot be inserted into the tree yet, upgrade when the R-tree type is mapped
        let _ = tree;
        panic!("RORouteHandler::initLaneTree needs the unported NamedRTree");
    }

    /// C++: `ROEdge * RORouteHandler::retrieveEdge(const std::string &)` [virtual, protected]
    // ROEdge*
    // RORouteHandler::retrieveEdge(const std::string& id) {
    //     return myNet.getEdge(id);
    // }
    #[allow(unused_variables)]
    pub fn retrieve_edge(&mut self, id: &str) -> Ptr<ROEdge> {
        self.myNet.get_edge(id)
    }

    /// C++: `const SUMOVTypeParameter * RORouteHandler::getVTypeParameter(const std::string &)` [virtual, protected]
    // const SUMOVTypeParameter*
    // RORouteHandler::getVTypeParameter(const std::string& refid) {
    //     if (refid == "") {
    //         return nullptr;
    //     }
    //     if (!myNet.knowsVType(refid)) {
    //         myErrorOutput->inform(TLF("Unknown vehicle type '%'.", refid));
    //         return nullptr;
    //     }
    //     return myNet.getVehicleTypeSecure(refid);
    // }
    #[allow(unused_variables)]
    pub fn get_v_type_parameter(&mut self, refid: &str) -> Ptr<SUMOVTypeParameter> {
        if refid == "" {
            return Ptr::null();
        }
        if !self.myNet.knows_v_type(refid) {
            let msg = format!("Unknown vehicle type '{}'.", refid);
            self.myErrorOutput.m().inform(&msg, true);
            return Ptr::null();
        }
        self.myNet.m().get_vehicle_type_secure(refid)
    }

    /// C++: `void RORouteHandler::RORouteHandler(const RORouteHandler &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &RORouteHandler) -> Self {
        // C++ copy constructor is private and has no definition, so it can never be called
        panic!("RORouteHandler copy constructor is private and undefined in C++");
    }

    /// C++: `RORouteHandler & RORouteHandler::operator=(const RORouteHandler &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &RORouteHandler) -> &mut RORouteHandler {
        // C++ assignment operator is private and has no definition, so it can never be called
        panic!("RORouteHandler assignment operator is private and undefined in C++");
    }

}

