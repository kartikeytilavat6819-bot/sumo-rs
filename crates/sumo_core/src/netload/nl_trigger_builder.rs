//! Port of netload/NLTriggerBuilder.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{write_error, write_message, write_warning};

/// C++ `throw InvalidArgument(msg)`
fn throw_invalid_argument(msg: String) -> ! {
    std::panic::panic_any(InvalidArgument(msg))
}

/// C++ `throw ProcessError(msg)`
fn throw_process_error(msg: String) -> ! {
    std::panic::panic_any(ProcessError::new(msg))
}

/// C++ `StringUtils::toDouble` / `toInt` / `string2time` throw on bad input; the Rust helpers return Result
fn or_throw<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| panic!("{}", e))
}

/// C++ `new X(...)` whose owner (the net or the event/detector containers) keeps it alive for the whole run.
// shortcut: the C++ owner deletes these at shutdown; Rust leaks the box so the Ptr stays valid, upgrade with the owning containers' Drop
fn leak_ptr<T>(b: Box<T>) -> Ptr<T> {
    Ptr::from_mut(Box::leak(b))
}

/// C++ `WrappingCommand<MSEdge>` for `&MSEdge::incVaporization` / `&MSEdge::decVaporization`
struct EdgeVaporizationCommand {
    edge: Ptr<MSEdge>,
    inc: bool,
}

impl crate::microsim::ms_event_control::Command for EdgeVaporizationCommand {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        if self.inc {
            self.edge.m().inc_vaporization(currentTime)
        } else {
            self.edge.m().dec_vaporization(currentTime)
        }
    }
}

/// Builds trigger objects for microsim
/// C++ class `NLTriggerBuilder` (netload/NLTriggerBuilder.h)
pub struct NLTriggerBuilder {
    /// The parent handler to set for subhandlers
    pub myHandler: Ptr<NLHandler>,
    /// definition of the currently parsed parking area
    pub myParkingArea: Ptr<MSParkingArea>,
    /// The currently parsed stop to add access points to
    pub myCurrentStop: Ptr<MSStoppingPlace>,
    pub myHaveWarnedAboutEigen: bool,
    pub myParkingAreaCapacitySet: bool,
    /// C++ `registerParent(SUMO_TAG_REROUTER, myHandler)`: the rerouter that receives the elements until its closing tag
    pub myChildRerouter: Ptr<MSTriggeredRerouter>,
}

impl NLTriggerBuilder {
    /// C++: `void NLTriggerBuilder::NLTriggerBuilder()`
    /// Constructor
    // NLTriggerBuilder::NLTriggerBuilder()
    //     : myHandler(nullptr), myParkingArea(nullptr), myCurrentStop(nullptr) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        NLTriggerBuilder {
            myHandler: Ptr::null(),
            myParkingArea: Ptr::null(),
            myCurrentStop: Ptr::null(),
            myHaveWarnedAboutEigen: false,
            myParkingAreaCapacitySet: false,
            myChildRerouter: Ptr::null(),
        }
    }

    /// C++: `void NLTriggerBuilder::~NLTriggerBuilder() noexcept` [virtual]
    /// Destructor
    // NLTriggerBuilder::~NLTriggerBuilder() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {}

    /// C++: `void NLTriggerBuilder::setHandler(NLHandler *)`
    /// Sets the parent handler to use for nested parsing
    // void
    // NLTriggerBuilder::setHandler(NLHandler* handler) {
    //     myHandler = handler;
    // }
    #[allow(unused_variables)]
    pub fn set_handler(&mut self, handler: Ptr<NLHandler>) {
        self.myHandler = handler;
    }

    /// C++: `void NLTriggerBuilder::buildVaporizer(const SUMOSAXAttributes &)`
    /// Builds a vaporization
    // void
    // NLTriggerBuilder::buildVaporizer(const SUMOSAXAttributes& attrs) {
    //     WRITE_WARNING(TL("Vaporizers are deprecated. Use rerouters instead."));
    //     bool ok = true;
    //     // get the id, throw if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     MSEdge* e = MSEdge::dictionary(id);
    //     if (e == nullptr) {
    //         WRITE_ERRORF(TL("Unknown edge ('%') referenced in a vaporizer."), id);
    //         return;
    //     }
    //     SUMOTime begin = attrs.getSUMOTimeReporting(SUMO_ATTR_BEGIN, nullptr, ok);
    //     SUMOTime end = attrs.getSUMOTimeReporting(SUMO_ATTR_END, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     if (begin < 0) {
    //         WRITE_ERRORF(TL("A vaporization begin time is negative (edge id='%')."), id);
    //         return;
    //     }
    //     if (begin >= end) {
    //         WRITE_ERRORF(TL("A vaporization ends before it starts (edge id='%')."), id);
    //         return;
    //     }
    //     if (end >= string2time(OptionsCont::getOptions().getString("begin"))) {
    //         Command* cb = new WrappingCommand< MSEdge >(e, &MSEdge::incVaporization);
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(cb, begin);
    //         Command* ce = new WrappingCommand< MSEdge >(e, &MSEdge::decVaporization);
    //         MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(ce, end);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_vaporizer(&mut self, attrs: &SUMOSAXAttributes) {
        write_warning("Vaporizers are deprecated. Use rerouters instead.");
        let mut ok = true;
        // get the id, throw if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            return;
        }
        let e = MSEdge::dictionary_2(&id);
        if e.is_null() {
            write_error(&utils::tlf!("Unknown edge ('%') referenced in a vaporizer.", id));
            return;
        }
        let begin = attrs.get_sumo_time_reporting(SUMO_ATTR_BEGIN, "", &mut ok, true);
        let end = attrs.get_sumo_time_reporting(SUMO_ATTR_END, "", &mut ok, true);
        if !ok {
            return;
        }
        if begin < 0 {
            write_error(&utils::tlf!("A vaporization begin time is negative (edge id='%').", id));
            return;
        }
        if begin >= end {
            write_error(&utils::tlf!("A vaporization ends before it starts (edge id='%').", id));
            return;
        }
        if end >= or_throw(utils::common::time::string2time(&utils::options::options_cont::get_string("begin"))) {
            let cb: Ptr<dyn crate::microsim::ms_event_control::Command> =
                Ptr::from_mut(Box::leak(Box::new(EdgeVaporizationCommand { edge: e, inc: true })) as &mut dyn crate::microsim::ms_event_control::Command);
            MSNet::get_instance().m().get_begin_of_timestep_events().m().add_event(cb, begin);
            let ce: Ptr<dyn crate::microsim::ms_event_control::Command> =
                Ptr::from_mut(Box::leak(Box::new(EdgeVaporizationCommand { edge: e, inc: false })) as &mut dyn crate::microsim::ms_event_control::Command);
            MSNet::get_instance().m().get_begin_of_timestep_events().m().add_event(ce, end);
        }
    }

    /// C++: `void NLTriggerBuilder::parseAndBuildLaneSpeedTrigger(MSNet &, const SUMOSAXAttributes &, const std::string &)`
    /// Parses his values and builds a lane speed trigger
    // void
    // NLTriggerBuilder::parseAndBuildLaneSpeedTrigger(MSNet& net, const SUMOSAXAttributes& attrs,
    //         const std::string& base) {
    //     // get the id, throw if not given or empty...
    //     bool ok = true;
    //     // get the id, throw if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     // get the file name to read further definitions from
    //     std::string file = getFileName(attrs, base, true);
    //     std::string objectid = attrs.get<std::string>(SUMO_ATTR_LANES, id.c_str(), ok);
    //     std::vector<MSLane*> lanes;
    //     for (const std::string& laneID : attrs.get<std::vector<std::string> >(SUMO_ATTR_LANES, id.c_str(), ok)) {
    //         MSLane* lane = MSLane::dictionary(laneID);
    //         if (lane == nullptr) {
    //             throw InvalidArgument("The lane '" + laneID + "' to use within MSLaneSpeedTrigger '" + id + "' is not known.");
    //         }
    //         lanes.push_back(lane);
    //     }
    //     if (!ok) {
    //         throw InvalidArgument("The lanes to use within MSLaneSpeedTrigger '" + id + "' are not known.");
    //     }
    //     if (lanes.size() == 0) {
    //         throw InvalidArgument("No lane defined for MSLaneSpeedTrigger '" + id + "'.");
    //     }
    //     try {
    //         MSLaneSpeedTrigger* trigger = buildLaneSpeedTrigger(net, id, lanes, file);
    //         if (file == "") {
    //             trigger->registerParent(SUMO_TAG_VSS, myHandler);
    //         }
    //     } catch (ProcessError& e) {
    //         throw InvalidArgument(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_and_build_lane_speed_trigger(&mut self, net: &mut MSNet, attrs: &SUMOSAXAttributes, base: &str) {
        let mut ok = true;
        // get the id, throw if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            return;
        }
        // get the file name to read further definitions from
        let file = self.get_file_name(attrs, base, true);
        let _objectid = attrs.get::<String>(SUMO_ATTR_LANES, &id, &mut ok, true);
        let mut lanes: Vec<Ptr<MSLane>> = Vec::new();
        for laneID in attrs.get::<Vec<String>>(SUMO_ATTR_LANES, &id, &mut ok, true) {
            let lane = MSLane::dictionary_2(&laneID);
            if lane.is_null() {
                throw_invalid_argument(format!("The lane '{}' to use within MSLaneSpeedTrigger '{}' is not known.", laneID, id));
            }
            lanes.push(lane);
        }
        if !ok {
            throw_invalid_argument(format!("The lanes to use within MSLaneSpeedTrigger '{}' are not known.", id));
        }
        if lanes.is_empty() {
            throw_invalid_argument(format!("No lane defined for MSLaneSpeedTrigger '{}'.", id));
        }
        // C++ wraps the build in try/catch(ProcessError) and rethrows InvalidArgument; build_lane_speed_trigger already throws InvalidArgument
        let _trigger = self.build_lane_speed_trigger(net, &id, &lanes, &file);
        if file == "" {
            // shortcut: trigger->registerParent(SUMO_TAG_VSS, myHandler) is not ported (no registerParent in sumo_core), upgrade when MSTrigger::registerParent lands
        }
    }

    /// C++: `void NLTriggerBuilder::parseAndBuildRerouter(MSNet &, const SUMOSAXAttributes &)`
    /// Parses his values and builds a rerouter
    // void
    // NLTriggerBuilder::parseAndBuildRerouter(MSNet& net, const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     // get the id, throw if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //     if (MSTriggeredRerouter::getInstances().count(id) > 0) {
    //         throw InvalidArgument("Could not build rerouter '" + id + "'; probably declared twice.");
    //     }
    //     MSEdgeVector edges;
    //     for (const std::string& edgeID : attrs.get<std::vector<std::string> >(SUMO_ATTR_EDGES, id.c_str(), ok)) {
    //         MSEdge* edge = MSEdge::dictionary(edgeID);
    //         if (edge == nullptr) {
    //             throw InvalidArgument("The edge '" + edgeID + "' to use within rerouter '" + id + "' is not known.");
    //         }
    //         edges.push_back(edge);
    //     }
    //     if (!ok) {
    //         throw InvalidArgument("The edge to use within rerouter '" + id + "' is not known.");
    //     }
    //     if (edges.size() == 0) {
    //         throw InvalidArgument("No edges found for rerouter '" + id + "'.");
    //     }
    //     const double prob = attrs.getOpt<double>(SUMO_ATTR_PROB, id.c_str(), ok, 1);
    //     const bool off = attrs.getOpt<bool>(SUMO_ATTR_OFF, id.c_str(), ok, false);
    //     const bool optional = attrs.getOpt<bool>(SUMO_ATTR_OPTIONAL, id.c_str(), ok, false);
    //     const SUMOTime timeThreshold = TIME2STEPS(attrs.getOpt<double>(SUMO_ATTR_HALTING_TIME_THRESHOLD, id.c_str(), ok, 0));
    //     const std::string vTypes = attrs.getOpt<std::string>(SUMO_ATTR_VTYPES, id.c_str(), ok, "");
    //     const std::string pos = attrs.getOpt<std::string>(SUMO_ATTR_POSITION, id.c_str(), ok, "");
    //     const double radius = attrs.getOpt<double>(SUMO_ATTR_RADIUS, id.c_str(), ok, std::numeric_limits<double>::max());
    //     if (attrs.hasAttribute(SUMO_ATTR_RADIUS) && !attrs.hasAttribute(SUMO_ATTR_POSITION)) {
    //         WRITE_WARNINGF(TL("It is strongly advisable to give an explicit position when using radius in the definition of rerouter '%'."), id)
    //     }
    //     Position p = Position::INVALID;
    //     if (pos != "") {
    //         const std::vector<std::string> posSplit = StringTokenizer(pos, ",").getVector();
    //         if (posSplit.size() == 1) {
    //             double lanePos = StringUtils::toDouble(pos);
    //             if (lanePos < 0) {
    //                 lanePos += edges.front()->getLanes()[0]->getLength();
    //             }
    //             p = edges.front()->getLanes()[0]->geometryPositionAtOffset(lanePos);
    //         } else if (posSplit.size() == 2) {
    //             p = Position(StringUtils::toDouble(posSplit[0]), StringUtils::toDouble(posSplit[1]));
    //         } else if (posSplit.size() == 3) {
    //             p = Position(StringUtils::toDouble(posSplit[0]), StringUtils::toDouble(posSplit[1]), StringUtils::toDouble(posSplit[2]));
    //         } else {
    //             throw InvalidArgument("Invalid position for rerouter '" + id + "'.");
    //         }
    //     }
    //     if (!ok) {
    //         throw InvalidArgument("Could not parse rerouter '" + id + "'.");
    //     }
    //     MSTriggeredRerouter* trigger = buildRerouter(net, id, edges, prob, off, optional, timeThreshold, vTypes, p, radius);
    //     // read in the trigger description
    //     trigger->registerParent(SUMO_TAG_REROUTER, myHandler);
    // }
    #[allow(unused_variables)]
    pub fn parse_and_build_rerouter(&mut self, net: &mut MSNet, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        // get the id, throw if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            throw_process_error(String::new());
        }
        if MSTriggeredRerouter::get_instances().contains_key(&id) {
            throw_invalid_argument(format!("Could not build rerouter '{}'; probably declared twice.", id));
        }
        let mut edges: MSEdgeVector = Vec::new();
        for edgeID in attrs.get::<Vec<String>>(SUMO_ATTR_EDGES, &id, &mut ok, true) {
            let edge = MSEdge::dictionary_2(&edgeID);
            if edge.is_null() {
                throw_invalid_argument(format!("The edge '{}' to use within rerouter '{}' is not known.", edgeID, id));
            }
            edges.push(edge);
        }
        if !ok {
            throw_invalid_argument(format!("The edge to use within rerouter '{}' is not known.", id));
        }
        if edges.is_empty() {
            throw_invalid_argument(format!("No edges found for rerouter '{}'.", id));
        }
        let prob = attrs.get_opt::<f64>(SUMO_ATTR_PROB, &id, &mut ok, 1., true);
        let off = attrs.get_opt::<bool>(SUMO_ATTR_OFF, &id, &mut ok, false, true);
        let optional = attrs.get_opt::<bool>(SUMO_ATTR_OPTIONAL, &id, &mut ok, false, true);
        let timeThreshold: SUMOTime = utils::common::time::time2steps(attrs.get_opt::<f64>(SUMO_ATTR_HALTING_TIME_THRESHOLD, &id, &mut ok, 0., true));
        let vTypes = attrs.get_opt::<String>(SUMO_ATTR_VTYPES, &id, &mut ok, String::new(), true);
        let pos = attrs.get_opt::<String>(SUMO_ATTR_POSITION, &id, &mut ok, String::new(), true);
        let radius = attrs.get_opt::<f64>(SUMO_ATTR_RADIUS, &id, &mut ok, f64::MAX, true);
        if attrs.has_attribute(SUMO_ATTR_RADIUS) && !attrs.has_attribute(SUMO_ATTR_POSITION) {
            write_warning(&utils::tlf!("It is strongly advisable to give an explicit position when using radius in the definition of rerouter '%'.", id));
        }
        let mut p = Position::INVALID;
        if pos != "" {
            let posSplit = utils::common::string_tokenizer::StringTokenizer::with_token(&pos, ",", false).get_vector();
            if posSplit.len() == 1 {
                let mut lanePos = or_throw(utils::common::string_utils::to_double(&pos));
                if lanePos < 0. {
                    lanePos += edges[0].m().get_lanes()[0].m().get_length();
                }
                p = edges[0].m().get_lanes()[0].m().geometry_position_at_offset(lanePos, 0.);
            } else if posSplit.len() == 2 {
                p = Position::new(
                    or_throw(utils::common::string_utils::to_double(&posSplit[0])),
                    or_throw(utils::common::string_utils::to_double(&posSplit[1])),
                );
            } else if posSplit.len() == 3 {
                p = Position::new3(
                    or_throw(utils::common::string_utils::to_double(&posSplit[0])),
                    or_throw(utils::common::string_utils::to_double(&posSplit[1])),
                    or_throw(utils::common::string_utils::to_double(&posSplit[2])),
                );
            } else {
                throw_invalid_argument(format!("Invalid position for rerouter '{}'.", id));
            }
        }
        if !ok {
            throw_invalid_argument(format!("Could not parse rerouter '{}'.", id));
        }
        let trigger = self.build_rerouter(net, &id, &mut edges, prob, off, optional, timeThreshold, &vTypes, &p, radius);
        // read in the trigger description: NLHandler forwards every element to the rerouter until </rerouter> (XMLSubSys::setHandler emulation)
        self.myChildRerouter = trigger;
    }

    /// C++: `void NLTriggerBuilder::parseAndBuildStoppingPlace(MSNet &, const SUMOSAXAttributes &, const SumoXMLTag)`
    /// Parses the values and builds a stopping places for busses, trains or container vehicles
    // void
    // NLTriggerBuilder::parseAndBuildStoppingPlace(MSNet& net, const SUMOSAXAttributes& attrs, const SumoXMLTag element) {
    //     bool ok = true;
    //     // get the id, throw if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //
    //     //get the name, leave blank if not given
    //     const std::string ptStopName = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok, "");
    //
    //     //get the color, use default if not given
    //     // default color, copy from GUIVisualizationStoppingPlaceSettings::busStopColor / containerStopColor
    //     RGBColor color = attrs.getOpt<RGBColor>(SUMO_ATTR_COLOR, id.c_str(), ok, RGBColor::INVISIBLE);
    //
    //     MSLane* lane = getLane(attrs, toString(element), id);
    //     // get the positions
    //     double frompos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, id.c_str(), ok, 0.);
    //     double topos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, id.c_str(), ok, lane->getLength());
    //     double angle = attrs.getOpt<double>(SUMO_ATTR_ANGLE, id.c_str(), ok, 90);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), ok, false);
    //     if (!ok || (myHandler->checkStopPos(frompos, topos, lane->getLength(), POSITION_EPS, friendlyPos) != SUMORouteHandler::StopPos::STOPPOS_VALID)) {
    //         throw InvalidArgument("Invalid position for " + toString(element) + " '" + id + "'.");
    //     }
    //     const std::vector<std::string>& lines = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_LINES, id.c_str(), ok);
    //     int defaultCapacity;
    //     SumoXMLAttr capacityAttr;
    //     if (element != SUMO_TAG_CONTAINER_STOP) {
    //         defaultCapacity = MAX2(MSStoppingPlace::getDefaultTransportablesAbreast(topos - frompos, element) * 3, 6);
    //         capacityAttr = SUMO_ATTR_PERSON_CAPACITY;
    //     } else {
    //         defaultCapacity = MSStoppingPlace::getDefaultTransportablesAbreast(topos - frompos, element);
    //         capacityAttr = SUMO_ATTR_CONTAINER_CAPACITY;
    //     }
    //     const int transportableCapacity = attrs.getOpt<int>(capacityAttr, id.c_str(), ok, defaultCapacity);
    //     const double parkingLength = attrs.getOpt<double>(SUMO_ATTR_PARKING_LENGTH, id.c_str(), ok, 0);
    //     // build the bus stop
    //     buildStoppingPlace(net, id, lines, lane, frompos, topos, element, ptStopName, transportableCapacity, parkingLength, color, angle);
    // }
    #[allow(unused_variables)]
    pub fn parse_and_build_stopping_place(&mut self, net: &mut MSNet, attrs: &SUMOSAXAttributes, element: SumoXMLTag) {
        let mut ok = true;
        // get the id, throw if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            throw_process_error(String::new());
        }
        //get the name, leave blank if not given
        let ptStopName = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);
        //get the color, use default if not given
        // default color, copy from GUIVisualizationStoppingPlaceSettings::busStopColor / containerStopColor
        let mut color = attrs.get_opt::<RGBColor>(SUMO_ATTR_COLOR, &id, &mut ok, RGBColor::INVISIBLE, true);
        let lane = self.get_lane(attrs, Tags.get_string(element), &id);
        // get the positions
        let mut frompos = attrs.get_opt::<f64>(SUMO_ATTR_STARTPOS, &id, &mut ok, 0., true);
        let mut topos = attrs.get_opt::<f64>(SUMO_ATTR_ENDPOS, &id, &mut ok, lane.m().get_length(), true);
        let angle = attrs.get_opt::<f64>(SUMO_ATTR_ANGLE, &id, &mut ok, 90., true);
        let friendlyPos = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut ok, false, true);
        if !ok
            || SUMORouteHandler::check_stop_pos(&mut frompos, &mut topos, lane.m().get_length(), utils::geom::POSITION_EPS, friendlyPos)
                != SUMORouteHandler_StopPos::STOPPOS_VALID
        {
            throw_invalid_argument(format!("Invalid position for {} '{}'.", Tags.get_string(element), id));
        }
        let lines = attrs.get_opt::<Vec<String>>(SUMO_ATTR_LINES, &id, &mut ok, Vec::new(), true);
        let defaultCapacity: i32;
        let capacityAttr: SumoXMLAttr;
        if element != SUMO_TAG_CONTAINER_STOP {
            let abreast = MSStoppingPlace::get_default_transportables_abreast(topos - frompos, element) * 3;
            defaultCapacity = if abreast > 6 { abreast } else { 6 }; // MAX2(..., 6)
            capacityAttr = SUMO_ATTR_PERSON_CAPACITY;
        } else {
            defaultCapacity = MSStoppingPlace::get_default_transportables_abreast(topos - frompos, element);
            capacityAttr = SUMO_ATTR_CONTAINER_CAPACITY;
        }
        let transportableCapacity = attrs.get_opt::<i32>(capacityAttr, &id, &mut ok, defaultCapacity, true);
        let parkingLength = attrs.get_opt::<f64>(SUMO_ATTR_PARKING_LENGTH, &id, &mut ok, 0., true);
        // build the bus stop
        self.build_stopping_place(net, id, lines, lane, frompos, topos, element, ptStopName, transportableCapacity, parkingLength, &mut color, angle);
    }

    /// C++: `void NLTriggerBuilder::beginParkingArea(MSNet &, const std::string &, const std::vector<std::string> &, const std::vector<std::string> &, MSLane *, double, double, unsigned int, double, double, double, const std::string &, bool, const std::string &, bool, bool)` [virtual]
    /// Begin a parking area
    // void
    // NLTriggerBuilder::beginParkingArea(MSNet& net, const std::string& id,
    //                                    const std::vector<std::string>& lines,
    //                                    const std::vector<std::string>& badges,
    //                                    MSLane* lane, double frompos, double topos,
    //                                    unsigned int capacity,
    //                                    double width, double length, double angle, const std::string& name,
    //                                    bool onRoad,
    //                                    const std::string& departPos,
    //                                    bool lefthand, bool reservable) {
    //     // Close previous parking area if there are no lots inside
    //     MSParkingArea* stop = new MSParkingArea(id, lines, badges, *lane, frompos, topos, capacity, width, length, angle, name, onRoad, departPos, lefthand, reservable);
    //     if (!net.addStoppingPlace(SUMO_TAG_PARKING_AREA, stop)) {
    //         delete stop;
    //         throw InvalidArgument("Could not build parking area '" + id + "'; probably declared twice.");
    //     } else {
    //         myParkingArea = stop;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn begin_parking_area(&mut self, net: &mut MSNet, id: &str, lines: &Vec<String>, badges: &Vec<String>, lane: Ptr<MSLane>, frompos: f64, topos: f64, capacity: u32, width: f64, length: f64, angle: f64, name: &str, onRoad: bool, departPos: &str, lefthand: bool, reservable: bool) {
        // Close previous parking area if there are no lots inside
        let mut stop = Box::new(MSParkingArea::new(id, lines, badges, lane.m(), frompos, topos, capacity as i32, width, length, angle, name, onRoad, departPos, lefthand, reservable));
        let stopPtr = Ptr::from_mut(&mut *stop);
        if !net.add_stopping_place(SUMO_TAG_PARKING_AREA, stopPtr.cast::<MSStoppingPlace>()) {
            // C++ deletes the stop here (the box is dropped on the throw)
            throw_invalid_argument(format!("Could not build parking area '{}'; probably declared twice.", id));
        } else {
            self.myParkingArea = leak_ptr(stop);
        }
    }

    /// C++: `void NLTriggerBuilder::addLotEntry(double, double, double, double, double, double, double)`
    /// Add a lot entry to current parking area
    // void
    // NLTriggerBuilder::addLotEntry(double x, double y, double z,
    //                               double width, double length,
    //                               double angle, double slope) {
    //     if (myParkingArea != nullptr) {
    //         if (!myParkingArea->parkOnRoad()) {
    //             myParkingArea->addLotEntry(x, y, z, width, length, angle, slope);
    //             myParkingAreaCapacitySet = true;
    //         } else {
    //             throw InvalidArgument("Cannot not add lot entry to on-road parking area.");
    //         }
    //     } else {
    //         throw InvalidArgument("Could not add lot entry outside a parking area.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_lot_entry(&mut self, x: f64, y: f64, z: f64, width: f64, length: f64, angle: f64, slope: f64) {
        if !self.myParkingArea.is_null() {
            if !self.myParkingArea.m().park_on_road() {
                self.myParkingArea.m().add_lot_entry(x, y, z, width, length, angle, slope);
                self.myParkingAreaCapacitySet = true;
            } else {
                throw_invalid_argument("Cannot not add lot entry to on-road parking area.".to_string());
            }
        } else {
            throw_invalid_argument("Could not add lot entry outside a parking area.".to_string());
        }
    }

    /// C++: `void NLTriggerBuilder::addAccess(MSNet &, const SUMOSAXAttributes &)`
    /// Parses the values and adds an access point to the currently parsed stopping place
    // void
    // NLTriggerBuilder::addAccess(MSNet& /* net */, const SUMOSAXAttributes& attrs) {
    //     if (myCurrentStop == nullptr) {
    //         throw InvalidArgument("Could not add access outside a stopping place.");
    //     }
    //     // get the lane
    //     MSLane* lane = getLane(attrs, "access", myCurrentStop->getID());
    //     if (!lane->allowsVehicleClass(SVC_PEDESTRIAN)) {
    //         WRITE_WARNINGF(TL("Ignoring invalid access from non-pedestrian lane '%' in busStop '%'."), lane->getID(), myCurrentStop->getID());
    //         return;
    //     }
    //     // get the positions
    //     bool ok = true;
    //     const std::string accessPos = attrs.getOpt<std::string>(SUMO_ATTR_POSITION, "access", ok);
    //     const bool random = accessPos == "random";
    //     MSStoppingPlace::AccessExit exit = MSStoppingPlace::AccessExit::PLATFORM;
    //     if (accessPos == "doors") {
    //         exit = MSStoppingPlace::AccessExit::DOORS;
    //     } else if (accessPos == "carriage") {
    //         exit = MSStoppingPlace::AccessExit::CARRIAGE;
    //     }
    //     double startPos = random || exit != MSStoppingPlace::AccessExit::PLATFORM ? 0. : attrs.getOpt<double>(SUMO_ATTR_POSITION, "access", ok, 0);
    //     double endPos = random || exit != MSStoppingPlace::AccessExit::PLATFORM ? lane->getLength() : startPos;
    //     const double length = attrs.getOpt<double>(SUMO_ATTR_LENGTH, "access", ok, -1);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, "access", ok, false);
    //     if (!ok || (myHandler->checkStopPos(startPos, endPos, lane->getLength(), 0, friendlyPos) != SUMORouteHandler::StopPos::STOPPOS_VALID)) {
    //         throw InvalidArgument("Invalid position " + attrs.getString(SUMO_ATTR_POSITION) + " for access on lane '" + lane->getID() + "' in stop '" + myCurrentStop->getID() + "'.");
    //     }
    //     // add bus stop access
    //     if (!myCurrentStop->addAccess(lane, startPos, endPos, length, exit)) {
    //         throw InvalidArgument("Duplicate access on lane '" + lane->getID() + "' for stop '" + myCurrentStop->getID() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_access(&mut self, net: &mut MSNet, attrs: &SUMOSAXAttributes) {
        if self.myCurrentStop.is_null() {
            throw_invalid_argument("Could not add access outside a stopping place.".to_string());
        }
        // get the lane
        let stop = self.myCurrentStop;
        let lane = self.get_lane(attrs, "access", stop.m().get_id());
        if !lane.m().allows_vehicle_class(SVC_PEDESTRIAN) {
            write_warning(&utils::tlf!("Ignoring invalid access from non-pedestrian lane '%' in busStop '%'.", lane.m().get_id(), stop.m().get_id()));
            return;
        }
        // get the positions
        let mut ok = true;
        let accessPos = attrs.get_opt::<String>(SUMO_ATTR_POSITION, "access", &mut ok, String::new(), true);
        let random = accessPos == "random";
        let mut exit = MSStoppingPlace_AccessExit::PLATFORM;
        if accessPos == "doors" {
            exit = MSStoppingPlace_AccessExit::DOORS;
        } else if accessPos == "carriage" {
            exit = MSStoppingPlace_AccessExit::CARRIAGE;
        }
        let mut startPos = if random || exit != MSStoppingPlace_AccessExit::PLATFORM { 0. } else { attrs.get_opt::<f64>(SUMO_ATTR_POSITION, "access", &mut ok, 0., true) };
        let mut endPos = if random || exit != MSStoppingPlace_AccessExit::PLATFORM { lane.m().get_length() } else { startPos };
        let length = attrs.get_opt::<f64>(SUMO_ATTR_LENGTH, "access", &mut ok, -1., true);
        let friendlyPos = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, "access", &mut ok, false, true);
        if !ok
            || SUMORouteHandler::check_stop_pos(&mut startPos, &mut endPos, lane.m().get_length(), 0., friendlyPos) != SUMORouteHandler_StopPos::STOPPOS_VALID
        {
            throw_invalid_argument(format!(
                "Invalid position {} for access on lane '{}' in stop '{}'.",
                attrs.get_string(SUMO_ATTR_POSITION),
                lane.m().get_id(),
                stop.m().get_id()
            ));
        }
        // add bus stop access
        if !stop.m().add_access(lane, startPos, endPos, length, exit) {
            throw_invalid_argument(format!("Duplicate access on lane '{}' for stop '{}'", lane.m().get_id(), stop.m().get_id()));
        }
    }

    /// C++: `void NLTriggerBuilder::parseAndBeginParkingArea(MSNet &, const SUMOSAXAttributes &)`
    /// Parses his values and builds a parking area
    // void
    // NLTriggerBuilder::parseAndBeginParkingArea(MSNet& net, const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     // get the id, throw if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //     // get the lane
    //     MSLane* lane = getLane(attrs, "parkingArea", id);
    //     // get the positions
    //     double frompos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, id.c_str(), ok, 0);
    //     double topos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, id.c_str(), ok, lane->getLength());
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), ok, false);
    //     unsigned int capacity = attrs.getOpt<int>(SUMO_ATTR_ROADSIDE_CAPACITY, id.c_str(), ok, 0);
    //     myParkingAreaCapacitySet = attrs.hasAttribute(SUMO_ATTR_ROADSIDE_CAPACITY);
    //     bool onRoad = attrs.getOpt<bool>(SUMO_ATTR_ONROAD, id.c_str(), ok, false);
    //     double width = attrs.getOpt<double>(SUMO_ATTR_WIDTH, id.c_str(), ok, 0);
    //     double length = attrs.getOpt<double>(SUMO_ATTR_LENGTH, id.c_str(), ok, 0);
    //     double angle = attrs.getOpt<double>(SUMO_ATTR_ANGLE, id.c_str(), ok, 0);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok);
    //     const std::string departPos = attrs.getOpt<std::string>(SUMO_ATTR_DEPARTPOS, id.c_str(), ok);
    //     bool lefthand = attrs.getOpt<bool>(SUMO_ATTR_LEFTHAND, id.c_str(), ok, false);
    //     const std::vector<std::string>& acceptedBadges = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_ACCEPTED_BADGES, id.c_str(), ok);
    //     const bool reservable = attrs.getOpt<bool>(SUMO_ATTR_RESERVABLE, id.c_str(), ok, false);
    //
    //     if (!ok || (myHandler->checkStopPos(frompos, topos, lane->getLength(), POSITION_EPS, friendlyPos) != SUMORouteHandler::StopPos::STOPPOS_VALID)) {
    //         throw InvalidArgument("Invalid position for parking area '" + id + "'.");
    //     }
    //     const std::vector<std::string>& lines = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_LINES, id.c_str(), ok);
    //     // build the parking area
    //     beginParkingArea(net, id, lines, acceptedBadges, lane, frompos, topos, capacity, width, length, angle, name, onRoad, departPos, lefthand, reservable);
    // }
    #[allow(unused_variables)]
    pub fn parse_and_begin_parking_area(&mut self, net: &mut MSNet, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        // get the id, throw if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            throw_process_error(String::new());
        }
        // get the lane
        let lane = self.get_lane(attrs, "parkingArea", &id);
        // get the positions
        let mut frompos = attrs.get_opt::<f64>(SUMO_ATTR_STARTPOS, &id, &mut ok, 0., true);
        let mut topos = attrs.get_opt::<f64>(SUMO_ATTR_ENDPOS, &id, &mut ok, lane.m().get_length(), true);
        let friendlyPos = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut ok, false, true);
        let capacity = attrs.get_opt::<i32>(SUMO_ATTR_ROADSIDE_CAPACITY, &id, &mut ok, 0, true) as u32;
        self.myParkingAreaCapacitySet = attrs.has_attribute(SUMO_ATTR_ROADSIDE_CAPACITY);
        let onRoad = attrs.get_opt::<bool>(SUMO_ATTR_ONROAD, &id, &mut ok, false, true);
        let width = attrs.get_opt::<f64>(SUMO_ATTR_WIDTH, &id, &mut ok, 0., true);
        let length = attrs.get_opt::<f64>(SUMO_ATTR_LENGTH, &id, &mut ok, 0., true);
        let angle = attrs.get_opt::<f64>(SUMO_ATTR_ANGLE, &id, &mut ok, 0., true);
        let name = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);
        let departPos = attrs.get_opt::<String>(SUMO_ATTR_DEPARTPOS, &id, &mut ok, String::new(), true);
        let lefthand = attrs.get_opt::<bool>(SUMO_ATTR_LEFTHAND, &id, &mut ok, false, true);
        let acceptedBadges = attrs.get_opt::<Vec<String>>(SUMO_ATTR_ACCEPTED_BADGES, &id, &mut ok, Vec::new(), true);
        let reservable = attrs.get_opt::<bool>(SUMO_ATTR_RESERVABLE, &id, &mut ok, false, true);

        if !ok
            || SUMORouteHandler::check_stop_pos(&mut frompos, &mut topos, lane.m().get_length(), utils::geom::POSITION_EPS, friendlyPos)
                != SUMORouteHandler_StopPos::STOPPOS_VALID
        {
            throw_invalid_argument(format!("Invalid position for parking area '{}'.", id));
        }
        let lines = attrs.get_opt::<Vec<String>>(SUMO_ATTR_LINES, &id, &mut ok, Vec::new(), true);
        // build the parking area
        self.begin_parking_area(net, &id, &lines, &acceptedBadges, lane, frompos, topos, capacity, width, length, angle, &name, onRoad, &departPos, lefthand, reservable);
    }

    /// C++: `void NLTriggerBuilder::parseAndAddLotEntry(const SUMOSAXAttributes &)`
    /// Parses his values and adds a lot entry to current parking area
    // void
    // NLTriggerBuilder::parseAndAddLotEntry(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     // Check for open parking area
    //     if (myParkingArea == nullptr) {
    //         throw ProcessError();
    //     }
    //     // get the positions
    //     double x = attrs.get<double>(SUMO_ATTR_X, "", ok);
    //     if (!ok) {
    //         throw InvalidArgument("Invalid x position for lot entry.");
    //     }
    //     double y = attrs.get<double>(SUMO_ATTR_Y, "", ok);
    //     if (!ok) {
    //         throw InvalidArgument("Invalid y position for lot entry.");
    //     }
    //     double z = attrs.getOpt<double>(SUMO_ATTR_Z, "", ok, 0.);
    //     double width = attrs.getOpt<double>(SUMO_ATTR_WIDTH, "", ok, myParkingArea->getWidth());
    //     double length = attrs.getOpt<double>(SUMO_ATTR_LENGTH, "", ok, myParkingArea->getLength());
    //     double angle = attrs.getOpt<double>(SUMO_ATTR_ANGLE, "", ok, myParkingArea->getAngle());
    //     double slope = attrs.getOpt<double>(SUMO_ATTR_SLOPE, "", ok, 0.);
    //     // add the lot entry
    //     addLotEntry(x, y, z, width, length, angle, slope);
    // }
    #[allow(unused_variables)]
    pub fn parse_and_add_lot_entry(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        // Check for open parking area
        if self.myParkingArea.is_null() {
            throw_process_error(String::new());
        }
        // get the positions
        let x = attrs.get::<f64>(SUMO_ATTR_X, "", &mut ok, true);
        if !ok {
            throw_invalid_argument("Invalid x position for lot entry.".to_string());
        }
        let y = attrs.get::<f64>(SUMO_ATTR_Y, "", &mut ok, true);
        if !ok {
            throw_invalid_argument("Invalid y position for lot entry.".to_string());
        }
        let z = attrs.get_opt::<f64>(SUMO_ATTR_Z, "", &mut ok, 0., true);
        let width = attrs.get_opt::<f64>(SUMO_ATTR_WIDTH, "", &mut ok, self.myParkingArea.m().get_width(), true);
        let length = attrs.get_opt::<f64>(SUMO_ATTR_LENGTH, "", &mut ok, self.myParkingArea.m().get_length(), true);
        let angle = attrs.get_opt::<f64>(SUMO_ATTR_ANGLE, "", &mut ok, self.myParkingArea.m().get_angle(), true);
        let slope = attrs.get_opt::<f64>(SUMO_ATTR_SLOPE, "", &mut ok, 0., true);
        // add the lot entry
        self.add_lot_entry(x, y, z, width, length, angle, slope);
    }

    /// C++: `void NLTriggerBuilder::endParkingArea()` [virtual]
    /// End a parking area
    // void
    // NLTriggerBuilder::endParkingArea() {
    //     if (myParkingArea != nullptr) {
    //         myParkingArea = nullptr;
    //         myParkingAreaCapacitySet = false;
    //     } else {
    //         throw InvalidArgument("Could not end a parking area that is not opened.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn end_parking_area(&mut self) {
        if !self.myParkingArea.is_null() {
            self.myParkingArea = Ptr::null();
            self.myParkingAreaCapacitySet = false;
        } else {
            throw_invalid_argument("Could not end a parking area that is not opened.".to_string());
        }
    }

    /// C++: `void NLTriggerBuilder::endStoppingPlace()` [virtual]
    /// End a stopping place
    // void
    // NLTriggerBuilder::endStoppingPlace() {
    //     if (myCurrentStop != nullptr) {
    //         myCurrentStop->finishedLoading();
    //         myCurrentStop = nullptr;
    //     } else {
    //         throw InvalidArgument("Could not end a stopping place that is not opened.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn end_stopping_place(&mut self) {
        if !self.myCurrentStop.is_null() {
            self.myCurrentStop.m().finished_loading();
            self.myCurrentStop = Ptr::null();
        } else {
            throw_invalid_argument("Could not end a stopping place that is not opened.".to_string());
        }
    }

    /// C++: `void NLTriggerBuilder::parseAndBuildChargingStation(MSNet &, const SUMOSAXAttributes &)`
    /// Parses his values and builds a charging station
    // void
    // NLTriggerBuilder::parseAndBuildChargingStation(MSNet& net, const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //
    //     // get the id, throw if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //
    //     MSLane* const lane = getLane(attrs, "chargingStation", id);
    //     double frompos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, id.c_str(), ok, 0);
    //     double topos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, id.c_str(), ok, lane->getLength());
    //     const double chargingPower = attrs.getOpt<double>(SUMO_ATTR_CHARGINGPOWER, id.c_str(), ok, 22000);
    //     const double totalPower = attrs.getOpt<double>(SUMO_ATTR_TOTALPOWER, id.c_str(), ok, -1);
    //     const double efficiency = attrs.getOpt<double>(SUMO_ATTR_EFFICIENCY, id.c_str(), ok, 0.95);
    //     const bool chargeInTransit = attrs.getOpt<bool>(SUMO_ATTR_CHARGEINTRANSIT, id.c_str(), ok, 0);
    //     const SUMOTime chargeDelay = attrs.getOptSUMOTimeReporting(SUMO_ATTR_CHARGEDELAY, id.c_str(), ok, 0);
    //     const std::string chargeType = attrs.getOpt<std::string>(SUMO_ATTR_CHARGETYPE, id.c_str(), ok, "normal");
    //     const SUMOTime waitingTime = attrs.getOptSUMOTimeReporting(SUMO_ATTR_WAITINGTIME, id.c_str(), ok, 900);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), ok, false);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok, "");
    //     MSParkingArea* parkingArea = getParkingArea(attrs, "parkingArea", id);
    //
    //     // check charge type
    //     if ((chargeType != "normal") && (chargeType != "battery-exchange") && (chargeType != "fuel")) {
    //         throw InvalidArgument("The chargeType to use within MSLaneSpeedTrigger '" + id + "' is invalid.");
    //     }
    //
    //     if (!ok || (myHandler->checkStopPos(frompos, topos, lane->getLength(), POSITION_EPS, friendlyPos) != SUMORouteHandler::StopPos::STOPPOS_VALID)) {
    //         throw InvalidArgument("Invalid position for charging station '" + id + "'.");
    //     }
    //
    //     buildChargingStation(net, id, lane, frompos, topos, name, chargingPower, totalPower, efficiency, chargeInTransit, chargeDelay, chargeType, waitingTime, parkingArea);
    // }
    #[allow(unused_variables)]
    pub fn parse_and_build_charging_station(&mut self, net: &mut MSNet, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        // get the id, throw if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            throw_process_error(String::new());
        }
        let lane = self.get_lane(attrs, "chargingStation", &id);
        let mut frompos = attrs.get_opt::<f64>(SUMO_ATTR_STARTPOS, &id, &mut ok, 0., true);
        let mut topos = attrs.get_opt::<f64>(SUMO_ATTR_ENDPOS, &id, &mut ok, lane.m().get_length(), true);
        let chargingPower = attrs.get_opt::<f64>(SUMO_ATTR_CHARGINGPOWER, &id, &mut ok, 22000., true);
        let totalPower = attrs.get_opt::<f64>(SUMO_ATTR_TOTALPOWER, &id, &mut ok, -1., true);
        let efficiency = attrs.get_opt::<f64>(SUMO_ATTR_EFFICIENCY, &id, &mut ok, 0.95, true);
        let chargeInTransit = attrs.get_opt::<bool>(SUMO_ATTR_CHARGEINTRANSIT, &id, &mut ok, false, true);
        let chargeDelay = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_CHARGEDELAY, &id, &mut ok, 0, true);
        let chargeType = attrs.get_opt::<String>(SUMO_ATTR_CHARGETYPE, &id, &mut ok, "normal".to_string(), true);
        let waitingTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_WAITINGTIME, &id, &mut ok, 900, true);
        let friendlyPos = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut ok, false, true);
        let name = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);
        let parkingArea = self.get_parking_area(attrs, "parkingArea", &id);

        // check charge type
        if chargeType != "normal" && chargeType != "battery-exchange" && chargeType != "fuel" {
            throw_invalid_argument(format!("The chargeType to use within MSLaneSpeedTrigger '{}' is invalid.", id));
        }

        if !ok
            || SUMORouteHandler::check_stop_pos(&mut frompos, &mut topos, lane.m().get_length(), utils::geom::POSITION_EPS, friendlyPos)
                != SUMORouteHandler_StopPos::STOPPOS_VALID
        {
            throw_invalid_argument(format!("Invalid position for charging station '{}'.", id));
        }

        self.build_charging_station(net, &id, lane, frompos, topos, &name, chargingPower, totalPower, efficiency, chargeInTransit, chargeDelay, chargeType, waitingTime, parkingArea);
    }

    /// C++: `void NLTriggerBuilder::parseAndBuildOverheadWireSegment(MSNet &, const SUMOSAXAttributes &)`
    /// Parses its values and builds an overhead wire segment
    // void
    // NLTriggerBuilder::parseAndBuildOverheadWireSegment(MSNet& net, const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //
    //     // get the id, throw if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, 0, ok);
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //
    //     /* The following call may either throw InvalidArgument exeption or return NULL:
    //         NULL is returned in case when the overhead wire segment should be built over an already
    //         ignored internal lane of an intersection, the exeption is thrown in case that
    //         the overhead wire segment references a non-existent lane. */
    //     MSLane* const lane = getLane(attrs, "overheadWireSegment", id);
    //     if (lane == nullptr) {
    //         WRITE_MESSAGEF(TL("The overheadWireSegment '%' was not created as it is attached to internal lane. It will be build automatically."), id);
    //         return;
    //     }
    //
    //     if (lane->isInternal()) {
    //         WRITE_MESSAGEF(TL("The overheadWireSegment '%' not built as it is attached to internal lane. It will be build automatically."), id);
    //         return;
    //     }
    //
    //     double frompos = attrs.getOpt<double>(SUMO_ATTR_STARTPOS, id.c_str(), ok, 0);
    //     double topos = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, id.c_str(), ok, lane->getLength());
    //     const bool voltageSource = attrs.getOpt<bool>(SUMO_ATTR_VOLTAGESOURCE, id.c_str(), ok, false);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), ok, false);
    //
    //     if (!ok || myHandler->checkStopPos(frompos, topos, lane->getLength(), POSITION_EPS, friendlyPos) != SUMORouteHandler::StopPos::STOPPOS_VALID) {
    //         frompos = 0;
    //         topos = lane->getLength();
    //         WRITE_MESSAGEF(TL("The overheadWireSegment '%' has wrong position. Automatically set from 0 to the length of the lane."), id);
    //         //throw InvalidArgument("Invalid position for overheadWireSegment'" + id + "'.");
    //     }
    //
    //     buildOverheadWireSegment(net, id, lane, frompos, topos, voltageSource);
    // #ifndef HAVE_EIGEN
    //     if (MSGlobals::gOverheadWireSolver && !myHaveWarnedAboutEigen) {
    //         myHaveWarnedAboutEigen = true;
    //         WRITE_WARNING(TL("Overhead wire solver (Eigen) not compiled in, expect errors in overhead wire simulation"))
    //     }
    // #endif // !HAVE_EIGEN
    // }
    #[allow(unused_variables)]
    pub fn parse_and_build_overhead_wire_segment(&mut self, net: &mut MSNet, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        // get the id, throw if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            throw_process_error(String::new());
        }
        /* The following call may either throw InvalidArgument exeption or return NULL:
            NULL is returned in case when the overhead wire segment should be built over an already
            ignored internal lane of an intersection, the exeption is thrown in case that
            the overhead wire segment references a non-existent lane. */
        let lane = self.get_lane(attrs, "overheadWireSegment", &id);
        if lane.is_null() {
            write_message(&utils::tlf!("The overheadWireSegment '%' was not created as it is attached to internal lane. It will be build automatically.", id));
            return;
        }
        if lane.m().is_internal() {
            write_message(&utils::tlf!("The overheadWireSegment '%' not built as it is attached to internal lane. It will be build automatically.", id));
            return;
        }
        let mut frompos = attrs.get_opt::<f64>(SUMO_ATTR_STARTPOS, &id, &mut ok, 0., true);
        let mut topos = attrs.get_opt::<f64>(SUMO_ATTR_ENDPOS, &id, &mut ok, lane.m().get_length(), true);
        let voltageSource = attrs.get_opt::<bool>(SUMO_ATTR_VOLTAGESOURCE, &id, &mut ok, false, true);
        let friendlyPos = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut ok, false, true);

        if !ok
            || SUMORouteHandler::check_stop_pos(&mut frompos, &mut topos, lane.m().get_length(), utils::geom::POSITION_EPS, friendlyPos)
                != SUMORouteHandler_StopPos::STOPPOS_VALID
        {
            frompos = 0.;
            topos = lane.m().get_length();
            write_message(&utils::tlf!("The overheadWireSegment '%' has wrong position. Automatically set from 0 to the length of the lane.", id));
            //throw InvalidArgument("Invalid position for overheadWireSegment'" + id + "'.");
        }

        self.build_overhead_wire_segment(net, &id, lane, frompos, topos, voltageSource);
        // #ifndef HAVE_EIGEN warning block omitted: the Rust port ships the overhead wire circuit solver, i.e. HAVE_EIGEN is treated as defined
    }

    /// C++: `void NLTriggerBuilder::parseAndBuildOverheadWireSection(MSNet &, const SUMOSAXAttributes &)`
    /// Parses its values and builds an overhead wire section
    // void
    // NLTriggerBuilder::parseAndBuildOverheadWireSection(MSNet& net, const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::string substationId = attrs.get<std::string>(SUMO_ATTR_SUBSTATIONID, 0, ok);
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //
    //     MSTractionSubstation* substation = MSNet::getInstance()->findTractionSubstation(substationId);
    //     if (substation == nullptr) {
    //         throw InvalidArgument("Traction substation '" + substationId + "' refereced by an OverheadWire Section is not known.");
    //     } else if (substation->isAnySectionPreviouslyDefined()) {
    //         throw InvalidArgument("Traction substation '" + substationId + "' refereced by an OverheadWire Section is probably referenced twice (a known limitation of the actual version of overhead wire simulation).");
    //     }
    //
    //     // @todo This may be a relict of older approach to processing the attributes ...
    //     std::string segmentStrings = attrs.get<std::string>(SUMO_ATTR_OVERHEAD_WIRE_SEGMENTS, substationId.c_str(), ok);
    //     if (!ok) {
    //         throw InvalidArgument("Segments referenced by Traction substation '" + substationId + "' are not declared .");
    //     }
    //
    //     // process forbidden internal lanes
    //     const std::vector<std::string>& forbiddenInnerLanesIDs = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_OVERHEAD_WIRE_FORBIDDEN, substationId.c_str(), ok);
    //     /// @todo for cycle abbreviation?
    //     for (const std::string& laneID : forbiddenInnerLanesIDs) {
    //         MSLane* lane = MSLane::dictionary(laneID);
    //         if (lane != nullptr) {
    //             substation->addForbiddenLane(lane);
    //         } else {
    //             throw InvalidArgument("Unknown lane '" + laneID + "' in Traction substation '" + substationId + "'.");
    //         }
    //     }
    //
    //     // @todo Check this as well ...
    //     // Original version from 2018
    //     // std::vector<std::string> segmentIDs;
    //     // SUMOSAXAttributes::parseStringVector(segmentStrings, segmentIDs);
    //     const std::vector<std::string>& segmentIDs = attrs.get<std::vector<std::string> >(SUMO_ATTR_OVERHEAD_WIRE_SEGMENTS, substationId.c_str(), ok);
    //     std::vector<MSOverheadWire*> segments;
    //
    //     // ----------------------------------------------
    //     // Add overhead wire segments over internal lanes
    //     // ----------------------------------------------
    //
    //     // Adding internal overhead wire segments (segments on neighboring inner lanes if a connection between two regular lane with overhead wire segment exists)
    //     for (const std::string& segmentID : segmentIDs) {
    //         const MSLane* connection = nullptr;
    //         MSOverheadWire* ovrhdSegment = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace(segmentID, SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //         std::string neigboringOvrhdSegmentID;
    //         MSOverheadWire* neigboringOvrhdSegment;
    //         MSTractionSubstation* neigboringOvrhdSegmentTractionSubstation;
    //         if (ovrhdSegment == nullptr) {
    //             throw InvalidArgument("The OverheadWireSegment with id='" + segmentID + "' referenced by OverheadWireSegment for substation '" + substationId + "' was not defined.");
    //         }
    //
    //         MSTractionSubstation* ts = ovrhdSegment->getTractionSubstation();
    //         if (!(ts == substation || ts == nullptr)) {
    //             std::string tsName = ts->getID();
    //             throw InvalidArgument("The OverheadWireSegment '" + segmentID + "' referenced by OverheadWireSegment for substation '" + substationId + "' is already assigned to substation '" + tsName + "'.");
    //         }
    //         ovrhdSegment->setTractionSubstation(substation);
    //
    //         const MSLane* lane = &(ovrhdSegment->getLane());
    //
    //         /* in version before SUMO 1.0.1 the function getOutgoingLanes() returning MSLane* exists,
    //         in new version of SUMO the funciton getOutgoingViaLanes() returning MSLane* and MSEdge* pair exists */
    //         const std::vector<std::pair<const MSLane*, const MSEdge*> > outgoingLanesAndEdges = lane->getOutgoingViaLanes();
    //         std::vector<const MSLane*> neigboringInnerLanes;
    //         neigboringInnerLanes.reserve(outgoingLanesAndEdges.size());
    //         for (size_t it = 0; it < outgoingLanesAndEdges.size(); ++it) {
    //             neigboringInnerLanes.push_back(outgoingLanesAndEdges[it].first);
    //         }
    //
    //         // Check if an outgoing lane has an overhead wire segment. If not, do nothing, otherwise find connnecting internal lanes and
    //         // add overhead wire segments over all detected internal lanes
    //         for (std::vector<const MSLane*>::iterator it = neigboringInnerLanes.begin(); it != neigboringInnerLanes.end(); ++it) {
    //             // If the overhead wire segment is over the outgoing (not internal) lane
    //             neigboringOvrhdSegmentID = MSNet::getInstance()->getStoppingPlaceID(*it, NUMERICAL_EPS, SUMO_TAG_OVERHEAD_WIRE_SEGMENT);
    //             if (neigboringOvrhdSegmentID != "") {
    //                 neigboringOvrhdSegment = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace(neigboringOvrhdSegmentID, SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //                 neigboringOvrhdSegmentTractionSubstation = neigboringOvrhdSegment->getTractionSubstation();
    //             } else {
    //                 neigboringOvrhdSegment = nullptr;
    //                 neigboringOvrhdSegmentTractionSubstation = nullptr;
    //             }
    //
    //             if (neigboringOvrhdSegmentTractionSubstation == substation && !(*it)->isInternal()) {
    //                 connection = lane->getInternalFollowingLane(*it);
    //                 if (connection != nullptr) {
    //                     //is connection forbidden?
    //                     if (!(substation->isForbidden(connection) || substation->isForbidden(lane->getInternalFollowingLane(connection)) || substation->isForbidden(connection->getInternalFollowingLane(*it)))) {
    //                         buildInnerOverheadWireSegments(net, connection, lane->getInternalFollowingLane(connection), connection->getInternalFollowingLane(*it));
    //                     }
    //                 }
    //             }
    //         }
    //
    //         // Check if an incoming lane has an overhead wire segment. If not, do nothing, otherwise find connnecting internal lanes and
    //         // add overhead wire segments over all detected internal lanes
    //         neigboringInnerLanes = lane->getNormalIncomingLanes();
    //         for (std::vector<const MSLane*>::iterator it = neigboringInnerLanes.begin(); it != neigboringInnerLanes.end(); ++it) {
    //             // If the overhead wire segment is over the incoming (not internal) lane
    //             neigboringOvrhdSegmentID = MSNet::getInstance()->getStoppingPlaceID(*it, (*it)->getLength() - NUMERICAL_EPS, SUMO_TAG_OVERHEAD_WIRE_SEGMENT);
    //             if (neigboringOvrhdSegmentID != "") {
    //                 neigboringOvrhdSegment = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace(neigboringOvrhdSegmentID, SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //                 neigboringOvrhdSegmentTractionSubstation = neigboringOvrhdSegment->getTractionSubstation();
    //             } else {
    //                 neigboringOvrhdSegment = nullptr;
    //                 neigboringOvrhdSegmentTractionSubstation = nullptr;
    //             }
    //
    //             if (neigboringOvrhdSegmentTractionSubstation == substation && !(*it)->isInternal()) {
    //                 connection = (*it)->getInternalFollowingLane(lane);
    //                 if (connection != nullptr) {
    //                     //is connection forbidden?
    //                     if (!(substation->isForbidden(connection) || substation->isForbidden((*it)->getInternalFollowingLane(connection)) || substation->isForbidden(connection->getInternalFollowingLane(lane)))) {
    //                         buildInnerOverheadWireSegments(net, connection, (*it)->getInternalFollowingLane(connection), connection->getInternalFollowingLane(lane));
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //
    //     // ----- *** adding segments into the electric circuit*** -----
    //
    //     // setting nullptr for substation (a fragment from old version of adding segments into the circuit)
    //     for (const std::string& segmentID : segmentIDs) {
    //         MSOverheadWire* ovrhdSegment = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace(segmentID, SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //         ovrhdSegment->setTractionSubstation(nullptr);
    //     }
    //
    //     for (const std::string& segmentID : segmentIDs) {
    //         if (segmentID == "") {
    //             continue;
    //         }
    //         MSOverheadWire* ovrhdSegment = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace(segmentID, SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //         substation->addOverheadWireSegmentToCircuit(ovrhdSegment);
    //         segments.push_back(ovrhdSegment);
    //     }
    //
    //     // adding overhead wire clamp
    //     std::string clampsString = attrs.getOpt<std::string>(SUMO_ATTR_OVERHEAD_WIRE_CLAMPS, nullptr, ok, "");
    //     if (clampsString != "" && MSGlobals::gOverheadWireSolver) {
    // #ifdef HAVE_EIGEN
    //         const std::vector<std::string>& clampIDs = attrs.get<std::vector<std::string> >(SUMO_ATTR_OVERHEAD_WIRE_CLAMPS, nullptr, ok);
    //         for (const std::string& clampID : clampIDs) {
    //             MSTractionSubstation::OverheadWireClamp* clamp = substation->findClamp(clampID);
    //             if (clamp != nullptr) {
    //                 if (clamp->start->getTractionSubstation() == substation && clamp->end->getTractionSubstation() == substation) {
    //                     substation->addOverheadWireClampToCircuit(clamp->id, clamp->start, clamp->end);
    //                     buildOverheadWireClamp(net, clamp->id, const_cast<MSLane*>(&clamp->start->getLane()), const_cast<MSLane*>(&clamp->end->getLane()));
    //                     clamp->usage = true;
    //                 } else {
    //                     if (clamp->start->getTractionSubstation() != substation) {
    //                         WRITE_WARNINGF(TL("A connecting overhead wire start segment '%' defined for overhead wire clamp '%' is not assigned to the traction substation '%'."), clamp->start->getID(), clampID, substationId);
    //                     } else {
    //                         WRITE_WARNINGF(TL("A connecting overhead wire end segment '%' defined for overhead wire clamp '%' is not assigned to the traction substation '%'."), clamp->end->getID(), clampID, substationId);
    //                     }
    //                 }
    //             } else {
    //                 WRITE_WARNINGF(TL("The overhead wire clamp '%' defined in an overhead wire section was not assigned to the substation '%'. Please define proper <overheadWireClamp .../> in additional files before defining overhead wire section."), clampID, substationId);
    //             }
    //         }
    // #else
    //         WRITE_WARNING(TL("Overhead circuit solver requested, but solver support (Eigen) not compiled in."));
    // #endif
    //     }
    //
    //     if (segments.size() == 0) {
    //         throw InvalidArgument("No segments found for overHeadWireSection '" + substationId + "'.");
    //     } else if (MSGlobals::gOverheadWireSolver) {
    // #ifdef HAVE_EIGEN
    //         // check that the electric circuit makes sense
    //         segments[0]->getCircuit()->checkCircuit(substationId);
    // #else
    //         WRITE_WARNING(TL("Cannot check circuit, overhead circuit solver support (Eigen) not compiled in."));
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_and_build_overhead_wire_section(&mut self, net: &mut MSNet, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let substationId = attrs.get::<String>(SUMO_ATTR_SUBSTATIONID, "", &mut ok, true);
        if !ok {
            throw_process_error(String::new());
        }

        let substation = net.find_traction_substation(&substationId);
        if substation.is_null() {
            throw_invalid_argument(format!("Traction substation '{}' refereced by an OverheadWire Section is not known.", substationId));
        } else if substation.m().is_any_section_previously_defined() {
            throw_invalid_argument(format!(
                "Traction substation '{}' refereced by an OverheadWire Section is probably referenced twice (a known limitation of the actual version of overhead wire simulation).",
                substationId
            ));
        }

        // @todo This may be a relict of older approach to processing the attributes ...
        let _segmentStrings = attrs.get::<String>(SUMO_ATTR_OVERHEAD_WIRE_SEGMENTS, &substationId, &mut ok, true);
        if !ok {
            throw_invalid_argument(format!("Segments referenced by Traction substation '{}' are not declared .", substationId));
        }

        // process forbidden internal lanes
        let forbiddenInnerLanesIDs = attrs.get_opt::<Vec<String>>(SUMO_ATTR_OVERHEAD_WIRE_FORBIDDEN, &substationId, &mut ok, Vec::new(), true);
        // @todo for cycle abbreviation?
        for laneID in &forbiddenInnerLanesIDs {
            let lane = MSLane::dictionary_2(laneID);
            if !lane.is_null() {
                substation.m().add_forbidden_lane(lane);
            } else {
                throw_invalid_argument(format!("Unknown lane '{}' in Traction substation '{}'.", laneID, substationId));
            }
        }

        // @todo Check this as well ...
        // Original version from 2018
        // std::vector<std::string> segmentIDs;
        // SUMOSAXAttributes::parseStringVector(segmentStrings, segmentIDs);
        let segmentIDs = attrs.get::<Vec<String>>(SUMO_ATTR_OVERHEAD_WIRE_SEGMENTS, &substationId, &mut ok, true);
        let mut segments: Vec<Ptr<MSOverheadWire>> = Vec::new();
        let tagSegment = SUMO_TAG_OVERHEAD_WIRE_SEGMENT;

        // ----------------------------------------------
        // Add overhead wire segments over internal lanes
        // ----------------------------------------------

        // Adding internal overhead wire segments (segments on neighboring inner lanes if a connection between two regular lane with overhead wire segment exists)
        for segmentID in &segmentIDs {
            let mut connection: Ptr<MSLane> = Ptr::null();
            let ovrhdSegment = net.get_stopping_place(segmentID, tagSegment).cast::<MSOverheadWire>();
            if ovrhdSegment.is_null() {
                throw_invalid_argument(format!(
                    "The OverheadWireSegment with id='{}' referenced by OverheadWireSegment for substation '{}' was not defined.",
                    segmentID, substationId
                ));
            }

            let ts = ovrhdSegment.m().get_traction_substation();
            if !(ts == substation || ts.is_null()) {
                let tsName = ts.m().get_id().to_string();
                throw_invalid_argument(format!(
                    "The OverheadWireSegment '{}' referenced by OverheadWireSegment for substation '{}' is already assigned to substation '{}'.",
                    segmentID, substationId, tsName
                ));
            }
            ovrhdSegment.m().set_traction_substation(substation);

            let lane = Ptr::new(ovrhdSegment.m().get_lane());

            // in new version of SUMO the function getOutgoingViaLanes() returning MSLane* and MSEdge* pair exists
            let outgoingLanesAndEdges = lane.m().get_outgoing_via_lanes();
            let mut neigboringInnerLanes: Vec<Ptr<MSLane>> = Vec::with_capacity(outgoingLanesAndEdges.len());
            for it in 0..outgoingLanesAndEdges.len() {
                neigboringInnerLanes.push(outgoingLanesAndEdges[it].0);
            }

            // Check if an outgoing lane has an overhead wire segment. If not, do nothing, otherwise find connnecting internal lanes and
            // add overhead wire segments over all detected internal lanes
            for it in &neigboringInnerLanes {
                // If the overhead wire segment is over the outgoing (not internal) lane
                let neigboringOvrhdSegmentID = net.get_stopping_place_id(*it, utils::geom::NUMERICAL_EPS, tagSegment);
                let neigboringOvrhdSegmentTractionSubstation: Ptr<MSTractionSubstation> = if neigboringOvrhdSegmentID != "" {
                    let neigboringOvrhdSegment = net.get_stopping_place(&neigboringOvrhdSegmentID, tagSegment).cast::<MSOverheadWire>();
                    neigboringOvrhdSegment.m().get_traction_substation()
                } else {
                    Ptr::null()
                };

                if neigboringOvrhdSegmentTractionSubstation == substation && !it.m().is_internal() {
                    connection = lane.m().get_internal_following_lane(*it);
                    if !connection.is_null() {
                        // is connection forbidden?
                        if !(substation.m().is_forbidden(connection)
                            || substation.m().is_forbidden(lane.m().get_internal_following_lane(connection))
                            || substation.m().is_forbidden(connection.m().get_internal_following_lane(*it)))
                        {
                            self.build_inner_overhead_wire_segments(net, connection, lane.m().get_internal_following_lane(connection), connection.m().get_internal_following_lane(*it));
                        }
                    }
                }
            }

            // Check if an incoming lane has an overhead wire segment. If not, do nothing, otherwise find connnecting internal lanes and
            // add overhead wire segments over all detected internal lanes
            neigboringInnerLanes = lane.m().get_normal_incoming_lanes();
            for it in &neigboringInnerLanes {
                // If the overhead wire segment is over the incoming (not internal) lane
                let neigboringOvrhdSegmentID = net.get_stopping_place_id(*it, it.m().get_length() - utils::geom::NUMERICAL_EPS, tagSegment);
                let neigboringOvrhdSegmentTractionSubstation: Ptr<MSTractionSubstation> = if neigboringOvrhdSegmentID != "" {
                    let neigboringOvrhdSegment = net.get_stopping_place(&neigboringOvrhdSegmentID, tagSegment).cast::<MSOverheadWire>();
                    neigboringOvrhdSegment.m().get_traction_substation()
                } else {
                    Ptr::null()
                };

                if neigboringOvrhdSegmentTractionSubstation == substation && !it.m().is_internal() {
                    connection = it.m().get_internal_following_lane(lane);
                    if !connection.is_null() {
                        // is connection forbidden?
                        if !(substation.m().is_forbidden(connection)
                            || substation.m().is_forbidden(it.m().get_internal_following_lane(connection))
                            || substation.m().is_forbidden(connection.m().get_internal_following_lane(lane)))
                        {
                            self.build_inner_overhead_wire_segments(net, connection, it.m().get_internal_following_lane(connection), connection.m().get_internal_following_lane(lane));
                        }
                    }
                }
            }
        }

        // ----- *** adding segments into the electric circuit*** -----

        // setting nullptr for substation (a fragment from old version of adding segments into the circuit)
        for segmentID in &segmentIDs {
            let ovrhdSegment = net.get_stopping_place(segmentID, tagSegment).cast::<MSOverheadWire>();
            ovrhdSegment.m().set_traction_substation(Ptr::null());
        }

        for segmentID in &segmentIDs {
            if segmentID == "" {
                continue;
            }
            let ovrhdSegment = net.get_stopping_place(segmentID, tagSegment).cast::<MSOverheadWire>();
            substation.m().add_overhead_wire_segment_to_circuit(ovrhdSegment);
            segments.push(ovrhdSegment);
        }

        // adding overhead wire clamp
        let clampsString = attrs.get_opt::<String>(SUMO_ATTR_OVERHEAD_WIRE_CLAMPS, "", &mut ok, String::new(), true);
        if clampsString != "" && crate::microsim::ms_globals::overhead_wire_solver() {
            let clampIDs = attrs.get::<Vec<String>>(SUMO_ATTR_OVERHEAD_WIRE_CLAMPS, "", &mut ok, true);
            for clampID in &clampIDs {
                let clamp = substation.m().find_clamp(clampID.clone());
                if !clamp.is_null() {
                    let clampRef = clamp.m();
                    if clampRef.start.m().get_traction_substation() == substation && clampRef.end.m().get_traction_substation() == substation {
                        let (startLane, endLane) = (Ptr::new(clampRef.start.m().get_lane()), Ptr::new(clampRef.end.m().get_lane()));
                        let clampStart = clampRef.start;
                        let clampEnd = clampRef.end;
                        let clampId = clampRef.id.clone();
                        substation.m().add_overhead_wire_clamp_to_circuit(clampId.clone(), clampStart, clampEnd);
                        self.build_overhead_wire_clamp(net, &clampId, startLane, endLane);
                        clampRef.usage = true;
                    } else {
                        if clampRef.start.m().get_traction_substation() != substation {
                            write_warning(&utils::tlf!(
                                "A connecting overhead wire start segment '%' defined for overhead wire clamp '%' is not assigned to the traction substation '%'.",
                                clampRef.start.m().get_id(),
                                clampID,
                                substationId
                            ));
                        } else {
                            write_warning(&utils::tlf!(
                                "A connecting overhead wire end segment '%' defined for overhead wire clamp '%' is not assigned to the traction substation '%'.",
                                clampRef.end.m().get_id(),
                                clampID,
                                substationId
                            ));
                        }
                    }
                } else {
                    write_warning(&utils::tlf!(
                        "The overhead wire clamp '%' defined in an overhead wire section was not assigned to the substation '%'. Please define proper <overheadWireClamp .../> in additional files before defining overhead wire section.",
                        clampID,
                        substationId
                    ));
                }
            }
        }

        if segments.is_empty() {
            throw_invalid_argument(format!("No segments found for overHeadWireSection '{}'.", substationId));
        } else if crate::microsim::ms_globals::overhead_wire_solver() {
            // check that the electric circuit makes sense
            segments[0].m().get_circuit().m().check_circuit(&substationId);
        }
    }

    /// C++: `void NLTriggerBuilder::parseAndBuildTractionSubstation(MSNet &, const SUMOSAXAttributes &)`
    /// Parses its values and builds a traction substation
    // void
    // NLTriggerBuilder::parseAndBuildTractionSubstation(MSNet& net, const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //
    //     // get the id, throw if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, 0, ok);
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //
    //     // RICE_TODO Limits are fixed, change them to some predefined constants ...
    //     const double voltage = attrs.getOpt<double>(SUMO_ATTR_VOLTAGE, id.c_str(), ok, 600);
    //     const double currentLimit = attrs.getOpt<double>(SUMO_ATTR_CURRENTLIMIT, id.c_str(), ok, 400);
    //     buildTractionSubstation(net, id, voltage, currentLimit);
    // }
    #[allow(unused_variables)]
    pub fn parse_and_build_traction_substation(&mut self, net: &mut MSNet, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        // get the id, throw if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            throw_process_error(String::new());
        }
        // RICE_TODO Limits are fixed, change them to some predefined constants ...
        let voltage = attrs.get_opt::<f64>(SUMO_ATTR_VOLTAGE, &id, &mut ok, 600., true);
        let currentLimit = attrs.get_opt::<f64>(SUMO_ATTR_CURRENTLIMIT, &id, &mut ok, 400., true);
        self.build_traction_substation(net, id, voltage, currentLimit);
    }

    /// C++: `void NLTriggerBuilder::parseAndBuildOverheadWireClamp(MSNet &, const SUMOSAXAttributes &)`
    /// Parses its values and builds an overhead wire clamp
    // void
    // NLTriggerBuilder::parseAndBuildOverheadWireClamp(MSNet& /*net*/, const SUMOSAXAttributes& attrs) {
    //     if (MSGlobals::gOverheadWireSolver) {
    // #ifdef HAVE_EIGEN
    //         bool ok = true;
    //         std::string id = attrs.get<std::string>(SUMO_ATTR_ID, 0, ok);
    //         if (!ok) {
    //             throw ProcessError();
    //         }
    //
    //         std::string substationId = attrs.get<std::string>(SUMO_ATTR_SUBSTATIONID, 0, ok);
    //         if (!ok) {
    //             throw ProcessError();
    //         }
    //         MSTractionSubstation* substation = MSNet::getInstance()->findTractionSubstation(substationId);
    //         if (substation == nullptr) {
    //             throw InvalidArgument("Traction substation '" + substationId + "' using within an overheadWireClamp '" + id + "' is not known.");
    //         }
    //
    //         std::string overhead_fromItsStart = attrs.get<std::string>(SUMO_ATTR_OVERHEAD_WIRE_CLAMP_START, 0, ok);
    //         if (!ok) {
    //             throw ProcessError();
    //         }
    //         MSOverheadWire* ovrhdSegment_fromItsStart = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace(overhead_fromItsStart, SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //         if (ovrhdSegment_fromItsStart == nullptr) {
    //             throw InvalidArgument("The overheadWireSegment '" + overhead_fromItsStart + "' to use within overheadWireClamp '" + id + "' is not known.");
    //         }
    //         /*if (ovrhdSegment_fromItsStart->getTractionSubstation() != substation) {
    //             throw InvalidArgument("The overheadWireSegment '" + overhead_fromItsStart + "' to use within overheadWireClamp is assign to a different overhead wire section or substation.");
    //         }
    //         */
    //         std::string overhead_fromItsEnd = attrs.get<std::string>(SUMO_ATTR_OVERHEAD_WIRE_CLAMP_END, 0, ok);
    //         if (!ok) {
    //             throw ProcessError();
    //         }
    //         MSOverheadWire* ovrhdSegment_fromItsEnd = dynamic_cast<MSOverheadWire*>(MSNet::getInstance()->getStoppingPlace(overhead_fromItsEnd, SUMO_TAG_OVERHEAD_WIRE_SEGMENT));
    //         if (ovrhdSegment_fromItsEnd == nullptr) {
    //             throw InvalidArgument("The overheadWireSegment '" + overhead_fromItsEnd + "' to use within overheadWireClamp '" + id + "' is not known.");
    //         }
    //         /*
    //         if (ovrhdSegment_fromItsEnd->getTractionSubstation() != substation) {
    //             throw InvalidArgument("The overheadWireSegment '" + overhead_fromItsEnd + "' to use within overheadWireClamp is assign to a different overhead wire section or substation.");
    //         }
    //         */
    //         if (substation->findClamp(id) == nullptr) {
    //             substation->addClamp(id, ovrhdSegment_fromItsStart, ovrhdSegment_fromItsEnd);
    //         } else {
    //             WRITE_ERROR("The overhead wire clamp '" + id + "' is probably declared twice.")
    //         }
    // #else
    //         UNUSED_PARAMETER(attrs);
    //         WRITE_WARNING(TL("Not building overhead wire clamps, overhead wire solver support (Eigen) not compiled in."));
    // #endif
    //     } else {
    //         WRITE_WARNING(TL("Ignoring overhead wire clamps, they make no sense when overhead wire circuit solver is off."));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_and_build_overhead_wire_clamp(&mut self, net: &mut MSNet, attrs: &SUMOSAXAttributes) {
        if crate::microsim::ms_globals::overhead_wire_solver() {
            // #ifdef HAVE_EIGEN branch: the Rust port ships the overhead wire circuit solver
            let mut ok = true;
            let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
            if !ok {
                throw_process_error(String::new());
            }

            let substationId = attrs.get::<String>(SUMO_ATTR_SUBSTATIONID, "", &mut ok, true);
            if !ok {
                throw_process_error(String::new());
            }
            let substation = net.find_traction_substation(&substationId);
            if substation.is_null() {
                throw_invalid_argument(format!("Traction substation '{}' using within an overheadWireClamp '{}' is not known.", substationId, id));
            }

            let overhead_fromItsStart = attrs.get::<String>(SUMO_ATTR_OVERHEAD_WIRE_CLAMP_START, "", &mut ok, true);
            if !ok {
                throw_process_error(String::new());
            }
            let ovrhdSegment_fromItsStart = net.get_stopping_place(&overhead_fromItsStart, SUMO_TAG_OVERHEAD_WIRE_SEGMENT).cast::<MSOverheadWire>();
            if ovrhdSegment_fromItsStart.is_null() {
                throw_invalid_argument(format!(
                    "The overheadWireSegment '{}' to use within overheadWireClamp '{}' is not known.",
                    overhead_fromItsStart, id
                ));
            }

            let overhead_fromItsEnd = attrs.get::<String>(SUMO_ATTR_OVERHEAD_WIRE_CLAMP_END, "", &mut ok, true);
            if !ok {
                throw_process_error(String::new());
            }
            let ovrhdSegment_fromItsEnd = net.get_stopping_place(&overhead_fromItsEnd, SUMO_TAG_OVERHEAD_WIRE_SEGMENT).cast::<MSOverheadWire>();
            if ovrhdSegment_fromItsEnd.is_null() {
                throw_invalid_argument(format!("The overheadWireSegment '{}' to use within overheadWireClamp '{}' is not known.", overhead_fromItsEnd, id));
            }

            if substation.m().find_clamp(id.clone()).is_null() {
                substation.m().add_clamp(&id, ovrhdSegment_fromItsStart, ovrhdSegment_fromItsEnd);
            } else {
                write_error(&format!("The overhead wire clamp '{}' is probably declared twice.", id));
            }
        } else {
            write_warning("Ignoring overhead wire clamps, they make no sense when overhead wire circuit solver is off.");
        }
    }

    /// C++: `void NLTriggerBuilder::parseAndBuildCalibrator(MSNet &, const SUMOSAXAttributes &, const std::string &)`
    /// Parses his values and builds a mesoscopic or microscopic calibrator
    // void
    // NLTriggerBuilder::parseAndBuildCalibrator(MSNet& net, const SUMOSAXAttributes& attrs,
    //         const std::string& base) {
    //     bool ok = true;
    //     // get the id, throw if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         throw ProcessError();
    //     }
    //     MSLane* lane = nullptr;
    //     MSEdge* edge = nullptr;
    //     MSJunction* node = nullptr;
    //     if (attrs.hasAttribute(SUMO_ATTR_NODE)) {
    //         if (attrs.hasAttribute(SUMO_ATTR_LANE) || attrs.hasAttribute(SUMO_ATTR_EDGE)) {
    //             throw InvalidArgument("The node calibrator '" + id + "' cannot define an edge or lane as well.");
    //         }
    //         const std::string nodeID = attrs.get<std::string>(SUMO_ATTR_NODE, id.c_str(), ok);
    //         node = net.getJunctionControl().get(nodeID);
    //         if (node == nullptr) {
    //             throw InvalidArgument("The node " + nodeID + " to use within the calibrator '" + id + "' is not known.");
    //         }
    //     } else {
    //         if (attrs.hasAttribute(SUMO_ATTR_EDGE)) {
    //             const std::string edgeID = attrs.get<std::string>(SUMO_ATTR_EDGE, id.c_str(), ok);
    //             edge = MSEdge::dictionary(edgeID);
    //             if (edge == nullptr) {
    //                 throw InvalidArgument("The edge " + edgeID + " to use within the calibrator '" + id + "' is not known.");
    //             }
    //             if (attrs.hasAttribute(SUMO_ATTR_LANE)) {
    //                 lane = getLane(attrs, "calibrator", id);
    //                 if (&lane->getEdge() != edge) {
    //                     throw InvalidArgument("The edge " + edgeID + " to use within the calibrator '" + id
    //                                           + "' does not match the calibrator lane '" + lane->getID() + ".");
    //                 }
    //             }
    //         } else {
    //             lane = getLane(attrs, "calibrator", id);
    //             edge = &lane->getEdge();
    //         }
    //     }
    //     const double pos = node != nullptr ? 0 : getPosition(attrs, lane, "calibrator", id, edge);
    //     SUMOTime period = attrs.getOptPeriod(id.c_str(), ok, DELTA_T); // !!! no error handling
    //     if (period > TIME2STEPS(1)) {
    //         WRITE_WARNINGF("Reducing period to 1 for calibrator '%'", id);
    //         period = TIME2STEPS(1);
    //     }
    //     const std::string vTypes = attrs.getOpt<std::string>(SUMO_ATTR_VTYPES, id.c_str(), ok, "");
    //     const std::string file = getFileName(attrs, base, true);
    //     const std::string outfile = attrs.getOpt<std::string>(SUMO_ATTR_OUTPUT, id.c_str(), ok, "");
    //     const std::string routeProbe = attrs.getOpt<std::string>(SUMO_ATTR_ROUTEPROBE, id.c_str(), ok, "");
    //     // differing defaults for backward compatibility, values are dimensionless
    //     const double invalidJamThreshold = attrs.getOpt<double>(SUMO_ATTR_JAM_DIST_THRESHOLD, id.c_str(), ok, MSGlobals::gUseMesoSim ? 0.8 : 0.5);
    //     const bool local = attrs.getOpt<bool>(SUMO_ATTR_LOCAL, id.c_str(), ok, false);
    //     MSRouteProbe* probe = nullptr;
    //     if (routeProbe != "") {
    //         probe = dynamic_cast<MSRouteProbe*>(net.getDetectorControl().getTypedDetectors(SUMO_TAG_ROUTEPROBE).get(routeProbe));
    //         if (probe == nullptr) {
    //             throw InvalidArgument("The routeProbe '" + routeProbe + "' to use within the calibrator '" + id + "' is not known.");
    //         }
    //     }
    //     if (MSGlobals::gUseMesoSim) {
    //         if (lane != nullptr && edge->getLanes().size() > 1) {
    //             WRITE_WARNING("Meso calibrator '" + id
    //                           + "' defined for lane '" + lane->getID()
    //                           + "' will collect data for all lanes of edge '" + edge->getID() + "'.");
    //         }
    //         METriggeredCalibrator* trigger = buildMECalibrator(id, edge, pos, file, outfile, period, probe, invalidJamThreshold, vTypes);
    //         if (file == "") {
    //             trigger->registerParent(SUMO_TAG_CALIBRATOR, myHandler);
    //         }
    //     } else {
    //         MSCalibrator* trigger = buildCalibrator(id, edge, lane, node, pos, file, outfile, period, probe, invalidJamThreshold, vTypes, local);
    //         if (file == "") {
    //             trigger->registerParent(SUMO_TAG_CALIBRATOR, myHandler);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_and_build_calibrator(&mut self, net: &mut MSNet, attrs: &SUMOSAXAttributes, base: &str) {
        let mut ok = true;
        // get the id, throw if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            throw_process_error(String::new());
        }
        let mut lane: Ptr<MSLane> = Ptr::null();
        let mut edge: Ptr<MSEdge> = Ptr::null();
        let mut node: Ptr<MSJunction> = Ptr::null();
        if attrs.has_attribute(SUMO_ATTR_NODE) {
            if attrs.has_attribute(SUMO_ATTR_LANE) || attrs.has_attribute(SUMO_ATTR_EDGE) {
                throw_invalid_argument(format!("The node calibrator '{}' cannot define an edge or lane as well.", id));
            }
            let nodeID = attrs.get::<String>(SUMO_ATTR_NODE, &id, &mut ok, true);
            // C++ NamedObjectCont::get returns nullptr when the id is unknown
            node = net.get_junction_control().base.get(&nodeID).copied().unwrap_or(Ptr::null());
            if node.is_null() {
                throw_invalid_argument(format!("The node {} to use within the calibrator '{}' is not known.", nodeID, id));
            }
        } else {
            if attrs.has_attribute(SUMO_ATTR_EDGE) {
                let edgeID = attrs.get::<String>(SUMO_ATTR_EDGE, &id, &mut ok, true);
                edge = MSEdge::dictionary_2(&edgeID);
                if edge.is_null() {
                    throw_invalid_argument(format!("The edge {} to use within the calibrator '{}' is not known.", edgeID, id));
                }
                if attrs.has_attribute(SUMO_ATTR_LANE) {
                    lane = self.get_lane(attrs, "calibrator", &id);
                    if Ptr::from_mut(lane.m().get_edge()) != edge {
                        throw_invalid_argument(format!(
                            "The edge {} to use within the calibrator '{}' does not match the calibrator lane '{}.",
                            edgeID,
                            id,
                            lane.m().get_id()
                        ));
                    }
                }
            } else {
                lane = self.get_lane(attrs, "calibrator", &id);
                edge = Ptr::from_mut(lane.m().get_edge());
            }
        }
        let pos = if !node.is_null() { 0. } else { self.get_position(attrs, lane, "calibrator", &id, edge) };
        let mut period = attrs.get_opt_period(&id, &mut ok, utils::common::time::delta_t(), true); // !!! no error handling
        if period > utils::common::time::time2steps(1.) {
            write_warning(&utils::tlf!("Reducing period to 1 for calibrator '%'", id));
            period = utils::common::time::time2steps(1.);
        }
        let vTypes = attrs.get_opt::<String>(SUMO_ATTR_VTYPES, &id, &mut ok, String::new(), true);
        let file = self.get_file_name(attrs, base, true);
        let outfile = attrs.get_opt::<String>(SUMO_ATTR_OUTPUT, &id, &mut ok, String::new(), true);
        let routeProbe = attrs.get_opt::<String>(SUMO_ATTR_ROUTEPROBE, &id, &mut ok, String::new(), true);
        // differing defaults for backward compatibility, values are dimensionless
        let invalidJamThreshold = attrs.get_opt::<f64>(SUMO_ATTR_JAM_DIST_THRESHOLD, &id, &mut ok, if crate::microsim::ms_globals::use_meso_sim() { 0.8 } else { 0.5 }, true);
        let local = attrs.get_opt::<bool>(SUMO_ATTR_LOCAL, &id, &mut ok, false, true);
        let mut probe: Ptr<MSRouteProbe> = Ptr::null();
        if routeProbe != "" {
            probe = net
                .get_detector_control()
                .get_typed_detectors(SUMO_TAG_ROUTEPROBE)
                .get(&routeProbe)
                .map(|d| d.cast::<MSRouteProbe>())
                .unwrap_or(Ptr::null());
            if probe.is_null() {
                throw_invalid_argument(format!("The routeProbe '{}' to use within the calibrator '{}' is not known.", routeProbe, id));
            }
        }
        if crate::microsim::ms_globals::use_meso_sim() {
            if !lane.is_null() && edge.m().get_lanes().len() > 1 {
                write_warning(&format!(
                    "Meso calibrator '{}' defined for lane '{}' will collect data for all lanes of edge '{}'.",
                    id,
                    lane.m().get_id(),
                    edge.m().get_id()
                ));
            }
            let _trigger = self.build_me_calibrator(&id, edge, pos, &file, &outfile, period, probe, invalidJamThreshold, &vTypes);
            if file == "" {
                // shortcut: trigger->registerParent(SUMO_TAG_CALIBRATOR, myHandler) is not ported (no registerParent in sumo_core), upgrade when MSTrigger::registerParent lands
            }
        } else {
            let _trigger = self.build_calibrator(&id, edge, lane, node, pos, &file, &outfile, period, probe, invalidJamThreshold, &vTypes, local);
            if file == "" {
                // shortcut: trigger->registerParent(SUMO_TAG_CALIBRATOR, myHandler) is not ported (no registerParent in sumo_core), upgrade when MSTrigger::registerParent lands
            }
        }
    }

    /// C++: `void NLTriggerBuilder::updateParkingAreaDefaultCapacity()`
    /// updates the parkingArea default capacity
    // void
    // NLTriggerBuilder::updateParkingAreaDefaultCapacity() {
    //     if (myParkingArea != nullptr && !myParkingAreaCapacitySet) {
    //         myParkingArea->setRoadsideCapacity(1);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_parking_area_default_capacity(&mut self) {
        if !self.myParkingArea.is_null() && !self.myParkingAreaCapacitySet {
            self.myParkingArea.m().set_roadside_capacity(1);
        }
    }

    /// C++: `MSStoppingPlace * NLTriggerBuilder::getCurrentStop()`
    // MSStoppingPlace*
    // NLTriggerBuilder::getCurrentStop() {
    //     return myParkingArea == nullptr ? myCurrentStop : myParkingArea;
    // }
    #[allow(unused_variables)]
    pub fn get_current_stop(&mut self) -> Ptr<MSStoppingPlace> {
        if self.myParkingArea.is_null() {
            self.myCurrentStop
        } else {
            self.myParkingArea.cast::<MSStoppingPlace>()
        }
    }

    /// C++: `MSLaneSpeedTrigger * NLTriggerBuilder::buildLaneSpeedTrigger(MSNet &, const std::string &, const std::vector<MSLane *> &, const std::string &)` [virtual, protected]
    /// Builds a lane speed trigger
    // MSLaneSpeedTrigger*
    // NLTriggerBuilder::buildLaneSpeedTrigger(MSNet& /*net*/, const std::string& id,
    //                                         const std::vector<MSLane*>& destLanes,
    //                                         const std::string& file) {
    //     return new MSLaneSpeedTrigger(id, destLanes, file);
    // }
    #[allow(unused_variables)]
    pub fn build_lane_speed_trigger(&mut self, net: &mut MSNet, id: &str, destLanes: &Vec<Ptr<MSLane>>, file: &str) -> Ptr<MSLaneSpeedTrigger> {
        // return new MSLaneSpeedTrigger(id, destLanes, file); (a ProcessError from the constructor propagates as InvalidArgument)
        match MSLaneSpeedTrigger::new(id, destLanes, file) {
            Ok(trigger) => leak_ptr(trigger),
            Err(e) => throw_invalid_argument(e.to_string()),
        }
    }

    /// C++: `void NLTriggerBuilder::buildStoppingPlace(MSNet &, std::string, std::vector<std::string>, MSLane *, double, double, const SumoXMLTag, std::string, int, double, RGBColor &, double)` [virtual, protected]
    /// Builds a stopping place
    // void
    // NLTriggerBuilder::buildStoppingPlace(MSNet& net, std::string id, std::vector<std::string> lines, MSLane* lane,
    //                                      double frompos, double topos, const SumoXMLTag element, std::string ptStopName,
    //                                      int personCapacity, double parkingLength, RGBColor& color, double angle) {
    //     myCurrentStop = new MSStoppingPlace(id, element, lines, *lane, frompos, topos, ptStopName, personCapacity, parkingLength, color, angle);
    //     if (!net.addStoppingPlace(element, myCurrentStop)) {
    //         delete myCurrentStop;
    //         myCurrentStop = nullptr;
    //         throw InvalidArgument("Could not build " + toString(element) + " '" + id + "'; probably declared twice.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_stopping_place(&mut self, net: &mut MSNet, id: String, lines: Vec<String>, lane: Ptr<MSLane>, frompos: f64, topos: f64, element: SumoXMLTag, string: String, personCapacity: i32, parkingLength: f64, color: &mut RGBColor, angle: f64) {
        let mut stop = Box::new(MSStoppingPlace::new(&id, element, &lines, lane.m(), frompos, topos, string, personCapacity, parkingLength, color, angle));
        self.myCurrentStop = Ptr::from_mut(&mut *stop);
        if !net.add_stopping_place(element, self.myCurrentStop) {
            self.myCurrentStop = Ptr::null();
            throw_invalid_argument(format!("Could not build {} '{}'; probably declared twice.", Tags.get_string(element), id));
        }
        leak_ptr(stop);
    }

    /// C++: `void NLTriggerBuilder::buildChargingStation(MSNet &, const std::string &, MSLane *, double, double, const std::string &, double, double, double, bool, SUMOTime, std::string, SUMOTime, MSParkingArea *)` [virtual, protected]
    /// Builds a charging station
    // void
    // NLTriggerBuilder::buildChargingStation(MSNet& net, const std::string& id, MSLane* lane, double frompos, double topos,
    //                                        const std::string& name, double chargingPower, double totalPower, double efficiency,
    //                                        bool chargeInTransit, SUMOTime chargeDelay, std::string chargeType, SUMOTime waitingTime, MSParkingArea* parkingArea) {
    //     MSChargingStation* chargingStation = (parkingArea == nullptr) ? new MSChargingStation(id, *lane, frompos, topos, name, chargingPower, totalPower, efficiency,
    //                                          chargeInTransit, chargeDelay, chargeType, waitingTime) : new MSChargingStation(id, parkingArea, name, chargingPower, totalPower, efficiency,
    //                                                  chargeInTransit, chargeDelay, chargeType, waitingTime);
    //     if (!net.addStoppingPlace(SUMO_TAG_CHARGING_STATION, chargingStation)) {
    //         delete chargingStation;
    //         throw InvalidArgument("Could not build charging station '" + id + "'; probably declared twice.");
    //     }
    //     myCurrentStop = chargingStation;
    // }
    #[allow(unused_variables)]
    pub fn build_charging_station(&mut self, net: &mut MSNet, id: &str, lane: Ptr<MSLane>, frompos: f64, topos: f64, name: &str, chargingPower: f64, totalPower: f64, efficiency: f64, chargeInTransit: bool, chargeDelay: SUMOTime, chargeType: String, waitingTime: SUMOTime, parkingArea: Ptr<MSParkingArea>) {
        let chargingStation: Box<MSChargingStation> = if parkingArea.is_null() {
            Box::new(MSChargingStation::new(id, lane.m(), frompos, topos, name, chargingPower, totalPower, efficiency, chargeInTransit, chargeDelay, &chargeType, waitingTime))
        } else {
            Box::new(MSChargingStation::new_2(id, parkingArea, name, chargingPower, totalPower, efficiency, chargeInTransit, chargeDelay, &chargeType, waitingTime))
        };
        let stopPtr = Ptr::from_mut(Box::leak(chargingStation));
        if !net.add_stopping_place(SUMO_TAG_CHARGING_STATION, stopPtr.cast::<MSStoppingPlace>()) {
            throw_invalid_argument(format!("Could not build charging station '{}'; probably declared twice.", id));
        }
        self.myCurrentStop = stopPtr.cast::<MSStoppingPlace>();
    }

    /// C++: `void NLTriggerBuilder::buildOverheadWireSegment(MSNet &, const std::string &, MSLane *, double, double, bool)` [virtual, protected]
    /// Builds an overhead wire segment
    // void
    // NLTriggerBuilder::buildOverheadWireSegment(MSNet& net, const std::string& id, MSLane* lane, double frompos, double topos,
    //         bool voltageSource) {
    //     MSOverheadWire* overheadWireSegment = new MSOverheadWire(id, *lane, frompos, topos, voltageSource);
    //     if (!net.addStoppingPlace(SUMO_TAG_OVERHEAD_WIRE_SEGMENT, overheadWireSegment)) {
    //         delete overheadWireSegment;
    //         throw InvalidArgument("Could not build overheadWireSegment '" + id + "'; probably declared twice.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_overhead_wire_segment(&mut self, net: &mut MSNet, id: &str, lane: Ptr<MSLane>, frompos: f64, topos: f64, voltageSource: bool) {
        let overheadWireSegment = Box::new(MSOverheadWire::new(id, lane.m(), frompos, topos, voltageSource));
        let segPtr = Ptr::from_mut(Box::leak(overheadWireSegment));
        if !net.add_stopping_place(SUMO_TAG_OVERHEAD_WIRE_SEGMENT, segPtr.cast::<MSStoppingPlace>()) {
            throw_invalid_argument(format!("Could not build overheadWireSegment '{}'; probably declared twice.", id));
        }
    }

    /// C++: `void NLTriggerBuilder::buildInnerOverheadWireSegments(MSNet &, const MSLane *, const MSLane *, const MSLane *)` [protected]
    /// Builds an overhead wire inner segments
    // void
    // NLTriggerBuilder::buildInnerOverheadWireSegments(MSNet& net, const MSLane* connection, const MSLane* frontConnection, const MSLane* behindConnection) {
    //     if (frontConnection == NULL && behindConnection == NULL) {
    //         buildOverheadWireSegment(net, "ovrhd_inner_" + connection->getID(), const_cast<MSLane*>(connection), 0, connection->getLength(), false);
    //     } else if (frontConnection != NULL && behindConnection == NULL) {
    //         buildOverheadWireSegment(net, "ovrhd_inner_" + frontConnection->getID(), const_cast<MSLane*>(frontConnection), 0, frontConnection->getLength(), false);
    //         buildOverheadWireSegment(net, "ovrhd_inner_" + connection->getID(), const_cast<MSLane*>(connection), 0, connection->getLength(), false);
    //     } else if (frontConnection == NULL && behindConnection != NULL) {
    //         buildOverheadWireSegment(net, "ovrhd_inner_" + behindConnection->getID(), const_cast<MSLane*>(behindConnection), 0, behindConnection->getLength(), false);
    //         buildOverheadWireSegment(net, "ovrhd_inner_" + connection->getID(), const_cast<MSLane*>(connection), 0, connection->getLength(), false);
    //     } else if (frontConnection != NULL && behindConnection != NULL) {
    //         buildOverheadWireSegment(net, "ovrhd_inner_" + frontConnection->getID(), const_cast<MSLane*>(frontConnection), 0, frontConnection->getLength(), false);
    //         buildOverheadWireSegment(net, "ovrhd_inner_" + behindConnection->getID(), const_cast<MSLane*>(behindConnection), 0, behindConnection->getLength(), false);
    //         buildOverheadWireSegment(net, "ovrhd_inner_" + connection->getID(), const_cast<MSLane*>(connection), 0, connection->getLength(), false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_inner_overhead_wire_segments(&mut self, net: &mut MSNet, connection: Ptr<MSLane>, frontConnection: Ptr<MSLane>, behindConnection: Ptr<MSLane>) {
        if frontConnection.is_null() && behindConnection.is_null() {
            self.build_overhead_wire_segment(net, &format!("ovrhd_inner_{}", connection.m().get_id()), connection, 0., connection.m().get_length(), false);
        } else if !frontConnection.is_null() && behindConnection.is_null() {
            self.build_overhead_wire_segment(net, &format!("ovrhd_inner_{}", frontConnection.m().get_id()), frontConnection, 0., frontConnection.m().get_length(), false);
            self.build_overhead_wire_segment(net, &format!("ovrhd_inner_{}", connection.m().get_id()), connection, 0., connection.m().get_length(), false);
        } else if frontConnection.is_null() && !behindConnection.is_null() {
            self.build_overhead_wire_segment(net, &format!("ovrhd_inner_{}", behindConnection.m().get_id()), behindConnection, 0., behindConnection.m().get_length(), false);
            self.build_overhead_wire_segment(net, &format!("ovrhd_inner_{}", connection.m().get_id()), connection, 0., connection.m().get_length(), false);
        } else if !frontConnection.is_null() && !behindConnection.is_null() {
            self.build_overhead_wire_segment(net, &format!("ovrhd_inner_{}", frontConnection.m().get_id()), frontConnection, 0., frontConnection.m().get_length(), false);
            self.build_overhead_wire_segment(net, &format!("ovrhd_inner_{}", behindConnection.m().get_id()), behindConnection, 0., behindConnection.m().get_length(), false);
            self.build_overhead_wire_segment(net, &format!("ovrhd_inner_{}", connection.m().get_id()), connection, 0., connection.m().get_length(), false);
        }
    }

    /// C++: `void NLTriggerBuilder::buildTractionSubstation(MSNet &, std::string, double, double)` [protected]
    /// Builds a traction substation
    // void
    // NLTriggerBuilder::buildTractionSubstation(MSNet& net, std::string id, double voltage, double currentLimit) {
    //     MSTractionSubstation* myTractionSubstation = new MSTractionSubstation(id, voltage, currentLimit);
    //     if (!net.addTractionSubstation(myTractionSubstation)) {
    //         delete myTractionSubstation;
    //         throw InvalidArgument("Could not build traction substation '" + id + "'; probably declared twice.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_traction_substation(&mut self, net: &mut MSNet, id: String, voltage: f64, currentLimit: f64) {
        let myTractionSubstation = Box::new(MSTractionSubstation::new(&id, voltage, currentLimit));
        let substPtr = Ptr::from_mut(Box::leak(myTractionSubstation));
        if !net.add_traction_substation(substPtr) {
            throw_invalid_argument(format!("Could not build traction substation '{}'; probably declared twice.", id));
        }
    }

    /// C++: `void NLTriggerBuilder::buildOverheadWireClamp(MSNet &, const std::string &, MSLane *, MSLane *)` [virtual, protected]
    // void
    // NLTriggerBuilder::buildOverheadWireClamp(MSNet& /*net*/, const std::string& /*id*/, MSLane* /*lane_start*/, MSLane* /*lane_end*/) {
    // }
    #[allow(unused_variables)]
    pub fn build_overhead_wire_clamp(&mut self, net: &mut MSNet, id: &str, lane_start: Ptr<MSLane>, lane_end: Ptr<MSLane>) {
        // C++ body is empty (the clamp is registered by the caller)
    }

    /// C++: `MSCalibrator * NLTriggerBuilder::buildCalibrator(const std::string &, MSEdge *, MSLane *, MSJunction *, double, const std::string &, const std::string &, const SUMOTime, const MSRouteProbe *, const double, const std::string &, const bool)` [virtual, protected]
    /// builds a microscopic calibrator
    // MSCalibrator*
    // NLTriggerBuilder::buildCalibrator(const std::string& id,
    //                                   MSEdge* edge,
    //                                   MSLane* lane,
    //                                   MSJunction* node,
    //                                   double pos,
    //                                   const std::string& file,
    //                                   const std::string& outfile,
    //                                   const SUMOTime freq,
    //                                   const MSRouteProbe* probe,
    //                                   const double invalidJamThreshold,
    //                                   const std::string& vTypes,
    //                                   const bool local) {
    //     return new MSCalibrator(id, edge, lane, node, pos, file, outfile, freq,
    //                             edge == nullptr ? 0. : edge->getLength(),
    //                             probe, invalidJamThreshold, vTypes, local, true);
    // }
    #[allow(unused_variables)]
    pub fn build_calibrator(&mut self, id: &str, edge: Ptr<MSEdge>, lane: Ptr<MSLane>, node: Ptr<MSJunction>, pos: f64, file: &str, outfile: &str, freq: SUMOTime, probe: Ptr<MSRouteProbe>, invalidJamThreshold: f64, vTypes: &str, local: bool) -> Ptr<MSCalibrator> {
        // return new MSCalibrator(id, edge, lane, node, pos, file, outfile, freq, edge == nullptr ? 0. : edge->getLength(), probe, invalidJamThreshold, vTypes, local, true);
        let length = if edge.is_null() { 0. } else { edge.m().get_length() };
        leak_ptr(MSCalibrator::new(id, edge, lane, node, pos, file, outfile, freq, length, probe, invalidJamThreshold, vTypes, local, true))
    }

    /// C++: `METriggeredCalibrator * NLTriggerBuilder::buildMECalibrator(const std::string &, MSEdge *, double, const std::string &, const std::string &, const SUMOTime, MSRouteProbe *, const double, const std::string &)` [virtual, protected]
    /// builds a mesoscopic calibrator
    // METriggeredCalibrator*
    // NLTriggerBuilder::buildMECalibrator(const std::string& id,
    //                                     MSEdge* edge,
    //                                     double pos,
    //                                     const std::string& file,
    //                                     const std::string& outfile,
    //                                     const SUMOTime freq,
    //                                     MSRouteProbe* probe,
    //                                     const double invalidJamThreshold,
    //                                     const std::string& vTypes) {
    //     return new METriggeredCalibrator(id, edge, pos, file, outfile, freq,
    //                                      edge == nullptr ? 0. : MSGlobals::gMesoNet->getSegmentForEdge(*edge, pos)->getLength(),
    //                                      probe, invalidJamThreshold, vTypes);
    // }
    #[allow(unused_variables)]
    pub fn build_me_calibrator(&mut self, id: &str, edge: Ptr<MSEdge>, pos: f64, file: &str, outfile: &str, freq: SUMOTime, probe: Ptr<MSRouteProbe>, invalidJamThreshold: f64, vTypes: &str) -> Ptr<METriggeredCalibrator> {
        // return new METriggeredCalibrator(..., edge == nullptr ? 0. : MSGlobals::gMesoNet->getSegmentForEdge(*edge, pos)->getLength(), ...);
        let length = if edge.is_null() {
            0.
        } else {
            crate::microsim::ms_globals::meso_net().m().get_segment_for_edge(edge.m(), pos).m().get_length()
        };
        leak_ptr(METriggeredCalibrator::new(id, edge, pos, file, outfile, freq, length, probe, invalidJamThreshold, vTypes))
    }

    /// C++: `MSTriggeredRerouter * NLTriggerBuilder::buildRerouter(MSNet &, const std::string &, MSEdgeVector &, double, bool, bool, SUMOTime, const std::string &, const Position &, const double)` [virtual, protected]
    /// builds an rerouter
    // MSTriggeredRerouter*
    // NLTriggerBuilder::buildRerouter(MSNet&, const std::string& id,
    //                                 MSEdgeVector& edges, double prob, bool off, bool optional,
    //                                 SUMOTime timeThreshold, const std::string& vTypes, const Position& pos, const double radius) {
    //     return new MSTriggeredRerouter(id, edges, prob, off, optional, timeThreshold, vTypes, pos, radius);
    // }
    #[allow(unused_variables)]
    pub fn build_rerouter(&mut self, net: &mut MSNet, id: &str, edges: &mut MSEdgeVector, prob: f64, off: bool, optional: bool, timeThreshold: SUMOTime, vTypes: &str, pos: &Position, radius: f64) -> Ptr<MSTriggeredRerouter> {
        // return new MSTriggeredRerouter(id, edges, prob, off, optional, timeThreshold, vTypes, pos, radius);
        let r = leak_ptr(Box::new(MSTriggeredRerouter::new(id, edges, prob, off, optional, timeThreshold, vTypes, pos, radius)));
        // C++ registers `this` with myInstances and with every lane's move reminders inside the
        // constructor. The object only has its final address after placement, so register here.
        r.m().register_at(r);
        r
    }

    /// C++: `std::string NLTriggerBuilder::getFileName(const SUMOSAXAttributes &, const std::string &, const bool)` [protected]
    /// Helper method to obtain the filename
    // std::string
    // NLTriggerBuilder::getFileName(const SUMOSAXAttributes& attrs,
    //                               const std::string& base,
    //                               const bool allowEmpty) {
    //     // get the file name to read further definitions from
    //     bool ok = true;
    //     std::string file = attrs.getOpt<std::string>(SUMO_ATTR_FILE, nullptr, ok, "");
    //     if (file == "") {
    //         if (allowEmpty) {
    //             return file;
    //         }
    //         throw InvalidArgument("No filename given.");
    //     }
    //     // check whether absolute or relative filenames are given
    //     if (!FileHelpers::isAbsolute(file)) {
    //         return FileHelpers::getConfigurationRelative(base, file);
    //     }
    //     return file;
    // }
    #[allow(unused_variables)]
    pub fn get_file_name(&mut self, attrs: &SUMOSAXAttributes, base: &str, allowEmpty: bool) -> String {
        // get the file name to read further definitions from
        let mut ok = true;
        let file = attrs.get_opt::<String>(SUMO_ATTR_FILE, "", &mut ok, String::new(), true);
        if file == "" {
            if allowEmpty {
                return file;
            }
            throw_invalid_argument("No filename given.".to_string());
        }
        // check whether absolute or relative filenames are given
        if !utils::common::file_helpers::is_absolute(&file) {
            return utils::common::file_helpers::get_configuration_relative(base, &file);
        }
        file
    }

    /// C++: `MSLane * NLTriggerBuilder::getLane(const SUMOSAXAttributes &, const std::string &, const std::string &)` [protected]
    /// Returns the lane defined by attribute "lane"
    // MSLane*
    // NLTriggerBuilder::getLane(const SUMOSAXAttributes& attrs,
    //                           const std::string& tt,
    //                           const std::string& tid) {
    //     bool ok = true;
    //     std::string objectid = attrs.get<std::string>(SUMO_ATTR_LANE, tid.c_str(), ok);
    //     MSLane* lane = MSLane::dictionary(objectid);
    //     if (lane == nullptr) {
    //         // Either a lane that is non-existent/broken, or a lane that is internal and has been ignored.
    //         // We assume that internal lane names start with ':'.
    //         if (objectid[0] == ':' && !MSGlobals::gUsingInternalLanes) {
    //             return nullptr;
    //         }
    //         // Throw the exception only in case that the lane really does not exist in the network file
    //         // or it is broken.
    //         throw InvalidArgument("The lane " + objectid + " to use within the " + tt + " '" + tid + "' is not known.");
    //     }
    //     return lane;
    // }
    #[allow(unused_variables)]
    pub fn get_lane(&mut self, attrs: &SUMOSAXAttributes, tt: &str, tid: &str) -> Ptr<MSLane> {
        let mut ok = true;
        let objectid = attrs.get::<String>(SUMO_ATTR_LANE, tid, &mut ok, true);
        let lane = MSLane::dictionary_2(&objectid);
        if lane.is_null() {
            // Either a lane that is non-existent/broken, or a lane that is internal and has been ignored.
            // We assume that internal lane names start with ':'.
            if objectid.starts_with(':') && !crate::microsim::ms_globals::using_internal_lanes() {
                return Ptr::null();
            }
            // Throw the exception only in case that the lane really does not exist in the network file
            // or it is broken.
            throw_invalid_argument(format!("The lane {} to use within the {} '{}' is not known.", objectid, tt, tid));
        }
        lane
    }

    /// C++: `MSParkingArea * NLTriggerBuilder::getParkingArea(const SUMOSAXAttributes &, const std::string &, const std::string &)` [protected]
    /// Returns the parking area defined by attribute "parkingArea"
    // MSParkingArea*
    // NLTriggerBuilder::getParkingArea(const SUMOSAXAttributes& attrs, const std::string& tt, const std::string& tid) {
    //     bool ok = true;
    //     std::string objectID = attrs.getOpt<std::string>(SUMO_ATTR_PARKING_AREA, tid.c_str(), ok);
    //     if (!ok || objectID.size() == 0) {
    //         return nullptr;
    //     }
    //     MSParkingArea* pa = static_cast<MSParkingArea*>(MSNet::getInstance()->getStoppingPlace(objectID, SUMO_TAG_PARKING_AREA));
    //     if (pa == nullptr) {
    //         // Throw the exception only in case that the lane really does not exist in the network file
    //         // or it is broken.
    //         throw InvalidArgument("The parkingArea " + objectID + " to use within the " + tt + " '" + tid + "' is not known.");
    //     }
    //     return pa;
    // }
    #[allow(unused_variables)]
    pub fn get_parking_area(&mut self, attrs: &SUMOSAXAttributes, tt: &str, tid: &str) -> Ptr<MSParkingArea> {
        let mut ok = true;
        let objectID = attrs.get_opt::<String>(SUMO_ATTR_PARKING_AREA, tid, &mut ok, String::new(), true);
        if !ok || objectID.is_empty() {
            return Ptr::null();
        }
        let pa = MSNet::get_instance().m().get_stopping_place(&objectID, SUMO_TAG_PARKING_AREA).cast::<MSParkingArea>();
        if pa.is_null() {
            // Throw the exception only in case that the lane really does not exist in the network file
            // or it is broken.
            throw_invalid_argument(format!("The parkingArea {} to use within the {} '{}' is not known.", objectID, tt, tid));
        }
        pa
    }

    /// C++: `double NLTriggerBuilder::getPosition(const SUMOSAXAttributes &, MSLane *, const std::string &, const std::string &, MSEdge *)` [protected]
    /// returns the position on the lane checking it
    // double
    // NLTriggerBuilder::getPosition(const SUMOSAXAttributes& attrs,
    //                               MSLane* lane,
    //                               const std::string& tt, const std::string& tid,
    //                               MSEdge* edge) {
    //     assert(lane != nullptr || edge != nullptr);
    //     const double length = lane != nullptr ? lane->getLength() : edge->getLength();
    //     bool ok = true;
    //     double pos = attrs.get<double>(SUMO_ATTR_POSITION, nullptr, ok);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, nullptr, ok, false);
    //     if (!ok) {
    //         throw InvalidArgument("Error on parsing a position information.");
    //     }
    //     if (pos < 0) {
    //         pos = length + pos;
    //     }
    //     if (pos > length) {
    //         if (friendlyPos) {
    //             pos = length - (double) 0.1;
    //         } else {
    //             if (lane != nullptr) {
    //                 throw InvalidArgument("The position of " + tt + " '" + tid + "' lies beyond the lane's '" + lane->getID() + "' length.");
    //             } else {
    //                 throw InvalidArgument("The position of " + tt + " '" + tid + "' lies beyond the edge's '" + edge->getID() + "' length.");
    //             }
    //         }
    //     }
    //     return pos;
    // }
    #[allow(unused_variables)]
    pub fn get_position(&mut self, attrs: &SUMOSAXAttributes, lane: Ptr<MSLane>, tt: &str, tid: &str, edge: Ptr<MSEdge>) -> f64 {
        let length = if !lane.is_null() { lane.m().get_length() } else { edge.m().get_length() };
        let mut ok = true;
        let mut pos = attrs.get::<f64>(SUMO_ATTR_POSITION, "", &mut ok, true);
        let friendlyPos = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, "", &mut ok, false, true);
        if !ok {
            throw_invalid_argument("Error on parsing a position information.".to_string());
        }
        if pos < 0. {
            pos = length + pos;
        }
        if pos > length {
            if friendlyPos {
                pos = length - 0.1;
            } else if !lane.is_null() {
                throw_invalid_argument(format!("The position of {} '{}' lies beyond the lane's '{}' length.", tt, tid, lane.m().get_id()));
            } else {
                throw_invalid_argument(format!("The position of {} '{}' lies beyond the edge's '{}' length.", tt, tid, edge.m().get_id()));
            }
        }
        pos
    }

}

