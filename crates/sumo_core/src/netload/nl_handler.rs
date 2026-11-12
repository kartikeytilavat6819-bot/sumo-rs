//! Port of netload/NLHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::{write_error, write_warning};
use utils::common::time::{sumotime_max_period, time2steps, SUMOTIME_MAX};
use utils::common::file_helpers::check_for_relativity;

/// C++ typedef `std::map<std::string, std::pair<std::string, std::string> > NLHandler::JunctionGraph`
pub type NLHandler_JunctionGraph = BTreeMap<String, (String, String)>;

/// C++ typedef `std::vector<MSLane*> NLHandler::LaneVector`
pub type NLHandler_LaneVector = Vec<Ptr<MSLane>>;

/// C++ `toString(SumoXMLTag)`: the canonical name of a tag
// shortcut: scans the TAGS table instead of the SumoXMLTags bijection, upgrade when sumo_xml_definitions exposes toString
fn tag_to_string(tag: SumoXMLTag) -> String {
    utils::xml::sumo_xml_definitions::TAGS.iter().find(|(_, v)| *v == tag).map(|(s, _)| s.to_string()).unwrap_or_default()
}

/// C++ `toString(SumoXMLAttr)`: the canonical name of an attribute
// shortcut: scans the ATTRS table instead of the SumoXMLAttrs bijection, upgrade when sumo_xml_definitions exposes toString
fn attr_to_string(attr: i32) -> String {
    utils::xml::sumo_xml_definitions::ATTRS.iter().find(|(_, v)| *v == attr).map(|(s, _)| s.to_string()).unwrap_or_default()
}

/// C++ `throw InvalidArgument(msg)`; caught by `try_catch_args`
fn throw_invalid_argument(msg: String) -> ! {
    std::panic::panic_any(ProcessError::InvalidArgument(msg))
}

/// C++ `try { body } catch (InvalidArgument& e) {...} catch (IOError& e) {...}`:
/// Err(e.what()) when one of those is thrown, other exceptions keep propagating
fn try_catch_args<R>(body: impl FnOnce() -> R) -> Result<R, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(body)).map_err(|payload| {
        let what = match payload.downcast_ref::<ProcessError>() {
            Some(e @ (ProcessError::InvalidArgument(_) | ProcessError::IOError(_))) => Some(e.to_string()),
            _ => payload.downcast_ref::<utils::geom::InvalidArgument>().map(|e| e.0.clone()),
        };
        match what {
            Some(w) => w,
            None => std::panic::resume_unwind(payload),
        }
    })
}

/// C++ `throw` of a ProcessError result that the caller does not catch locally
fn unwrap_or_throw<T>(r: Result<T, ProcessError>) -> T {
    r.unwrap_or_else(|e| std::panic::panic_any(e))
}

/// C++ implicit `MSStoppingPlace*` -> `Parameterised*` conversion (base2 is the Parameterised part)
fn stopping_place_as_parameterised(p: Ptr<MSStoppingPlace>) -> Ptr<Parameterised> {
    Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*p.as_raw()).base2) })
}

/// C++ implicit `MSLane*` -> `Parameterised*` conversion (base2 is the Parameterised part)
fn lane_as_parameterised(p: Ptr<MSLane>) -> Ptr<Parameterised> {
    Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*p.as_raw()).base2) })
}

/// C++ `parseVehicleClasses(allowedS, disallowedS, networkVersion)` (SUMOVehicleClass.cpp); the utils port
/// only has the single-argument overload, so the 3-argument one lives here until it is ported there
fn parse_vehicle_classes_v(allowed: &str, disallowed: &str, network_version: (i32, f64)) -> SVCPermissions {
    if allowed.is_empty() && disallowed.is_empty() {
        SVC_ALL
    } else if !allowed.is_empty() && !disallowed.is_empty() {
        write_warning("SVCPermissions must be specified either via 'allow' or 'disallow'. Ignoring 'disallow'");
        parse_vehicle_classes(allowed)
    } else if !allowed.is_empty() {
        parse_vehicle_classes(allowed)
    } else {
        invert_permissions(extra_disallowed(parse_vehicle_classes(disallowed), network_version))
    }
}

/// The XML-Handler for network loading
/// C++ class `NLHandler` (netload/NLHandler.h)
#[repr(C)]
pub struct NLHandler {
    /// C++ base class `MSRouteHandler`
    pub base: MSRouteHandler,
    /// The net to fill (preinitialised)
    pub myNet: Ptr<MSNet>,
    /// A builder for object actions
    pub myActionBuilder: NLDiscreteEventBuilder,
    /// Information whether the currently parsed edge is internal and not wished, here
    pub myCurrentIsInternalToSkip: bool,
    /// The detector builder to use
    pub myDetectorBuilder: Ptr<NLDetectorBuilder>,
    /// The trigger builder to use
    pub myTriggerBuilder: Ptr<NLTriggerBuilder>,
    /// The edge builder to use
    pub myEdgeControlBuilder: Ptr<NLEdgeControlBuilder>,
    /// The junction builder to use
    pub myJunctionControlBuilder: Ptr<NLJunctionControlBuilder>,
    /// The id of the current district
    pub myCurrentDistrictID: String,
    /// internal information whether a tls-logic is currently read
    pub myAmParsingTLLogicOrJunction: bool,
    /// The id of the currently processed WAUT
    pub myCurrentWAUTID: String,
    /// The id of the currently processed edge type
    pub myCurrentTypeID: String,
    /// The network offset
    pub myNetworkOffset: Position,
    /// The network's boundaries
    pub myOrigBoundary: Boundary,
    /// The network's boundaries
    pub myConvBoundary: Boundary,
    pub myCurrentIsBroken: bool,
    pub myHaveWarnedAboutInvalidTLType: bool,
    pub myLastEdgeParameters: Parameterised,
    pub myLastParameterised: Vec<Ptr<Parameterised>>,
    /// whether the loaded network contains internal lanes
    pub myHaveSeenInternalEdge: bool,
    /// Whether the network was built with higher speed on junctions
    pub myHaveJunctionHigherSpeeds: bool,
    /// whether the loaded network contains edges with default lengths
    pub myHaveSeenDefaultLength: bool,
    /// whether the loaded network contains explicit neighbor lanes
    pub myHaveSeenNeighs: bool,
    /// whether additional files contained type-specific speed limits
    pub myHaveSeenAdditionalSpeedRestrictions: bool,
    /// whether tls params were loaded
    pub myHaveSeenTLSParams: bool,
    /// the loaded network version
    pub myNetworkVersion: (i32, f64) /* MMVersion */,
    /// whether the location element was already loadee
    pub myNetIsLoaded: bool,
    /// rail signal for which constraints are being loaded
    pub myConstrainedSignal: Ptr<MSRailSignal>,
    /// the link element for the connection currently being parsed
    pub myCurrentLink: Ptr<MSLink>,
    pub myJunctionGraph: NLHandler_JunctionGraph,
    pub myPreviousEdgeIdx: i32,
}

impl std::ops::Deref for NLHandler {
    type Target = MSRouteHandler;
    fn deref(&self) -> &MSRouteHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NLHandler {
    fn deref_mut(&mut self) -> &mut MSRouteHandler {
        &mut self.base
    }
}

impl NLHandler {
    /// C++: `void NLHandler::NLHandler(const std::string &, MSNet &, NLDetectorBuilder &, NLTriggerBuilder &, NLEdgeControlBuilder &, NLJunctionControlBuilder &)`
    /// Constructor
    // NLHandler::NLHandler(const std::string& file, MSNet& net,
    //                      NLDetectorBuilder& detBuilder,
    //                      NLTriggerBuilder& triggerBuilder,
    //                      NLEdgeControlBuilder& edgeBuilder,
    //                      NLJunctionControlBuilder& junctionBuilder) :
    //     MSRouteHandler(file, true),
    //     myNet(net), myActionBuilder(net),
    //     myCurrentIsInternalToSkip(false),
    //     myDetectorBuilder(detBuilder), myTriggerBuilder(triggerBuilder),
    //     myEdgeControlBuilder(edgeBuilder), myJunctionControlBuilder(junctionBuilder),
    //     myAmParsingTLLogicOrJunction(false), myCurrentIsBroken(false),
    //     myHaveWarnedAboutInvalidTLType(false),
    //     myHaveSeenInternalEdge(false),
    //     myHaveJunctionHigherSpeeds(false),
    //     myHaveSeenDefaultLength(false),
    //     myHaveSeenNeighs(false),
    //     myHaveSeenAdditionalSpeedRestrictions(false),
    //     myHaveSeenTLSParams(false),
    //     myNetworkVersion(0, 0),
    //     myNetIsLoaded(false) {
    // }
    #[allow(unused_variables)]
    pub fn new(file: &str, net: &mut MSNet, detBuilder: &mut NLDetectorBuilder, triggerBuilder: &mut NLTriggerBuilder, edgeBuilder: &mut NLEdgeControlBuilder, junctionBuilder: &mut NLJunctionControlBuilder) -> Self {
        let myActionBuilder = NLDiscreteEventBuilder::new(net);
        NLHandler {
            base: MSRouteHandler::new(file, true),
            myNet: Ptr::from_mut(net),
            myActionBuilder,
            myCurrentIsInternalToSkip: false,
            myDetectorBuilder: Ptr::from_mut(detBuilder),
            myTriggerBuilder: Ptr::from_mut(triggerBuilder),
            myEdgeControlBuilder: Ptr::from_mut(edgeBuilder),
            myJunctionControlBuilder: Ptr::from_mut(junctionBuilder),
            myCurrentDistrictID: String::new(),
            myAmParsingTLLogicOrJunction: false,
            myCurrentWAUTID: String::new(),
            myCurrentTypeID: String::new(),
            myNetworkOffset: Position::default(),
            myOrigBoundary: Boundary::default(),
            myConvBoundary: Boundary::default(),
            myCurrentIsBroken: false,
            myHaveWarnedAboutInvalidTLType: false,
            myLastEdgeParameters: Parameterised::new(),
            myLastParameterised: Vec::new(),
            myHaveSeenInternalEdge: false,
            myHaveJunctionHigherSpeeds: false,
            myHaveSeenDefaultLength: false,
            myHaveSeenNeighs: false,
            myHaveSeenAdditionalSpeedRestrictions: false,
            myHaveSeenTLSParams: false,
            myNetworkVersion: (0, 0.0),
            myNetIsLoaded: false,
            myConstrainedSignal: Ptr::null(),
            myCurrentLink: Ptr::null(),
            myJunctionGraph: BTreeMap::new(),
            myPreviousEdgeIdx: 0,
        }
    }

    /// C++: `void NLHandler::~NLHandler()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++: `NLHandler::~NLHandler() {}`; the Rust fields are dropped by their own destructors
    }

    /// C++: `bool NLHandler::haveSeenInternalEdge() const` [const]
    // bool haveSeenInternalEdge() const {
    //         return myHaveSeenInternalEdge;
    //     }
    #[allow(unused_variables)]
    pub fn have_seen_internal_edge(&self) -> bool {
        self.myHaveSeenInternalEdge
    }

    /// C++: `bool NLHandler::hasJunctionHigherSpeeds() const` [const]
    // bool hasJunctionHigherSpeeds() const {
    //         return myHaveJunctionHigherSpeeds;
    //     }
    #[allow(unused_variables)]
    pub fn has_junction_higher_speeds(&self) -> bool {
        self.myHaveJunctionHigherSpeeds
    }

    /// C++: `bool NLHandler::haveSeenDefaultLength() const` [const]
    // bool haveSeenDefaultLength() const {
    //         return myHaveSeenDefaultLength;
    //     }
    #[allow(unused_variables)]
    pub fn have_seen_default_length(&self) -> bool {
        self.myHaveSeenDefaultLength
    }

    /// C++: `bool NLHandler::haveSeenNeighs() const` [const]
    // bool haveSeenNeighs() const {
    //         return myHaveSeenNeighs;
    //     }
    #[allow(unused_variables)]
    pub fn have_seen_neighs(&self) -> bool {
        self.myHaveSeenNeighs
    }

    /// C++: `bool NLHandler::haveSeenAdditionalSpeedRestrictions() const` [const]
    // bool haveSeenAdditionalSpeedRestrictions() const {
    //         return myHaveSeenAdditionalSpeedRestrictions;
    //     }
    #[allow(unused_variables)]
    pub fn have_seen_additional_speed_restrictions(&self) -> bool {
        self.myHaveSeenAdditionalSpeedRestrictions
    }

    /// C++: `bool NLHandler::haveSeenTLSParams() const` [const]
    // bool haveSeenTLSParams() const {
    //         return myHaveSeenTLSParams;
    //     }
    #[allow(unused_variables)]
    pub fn have_seen_tls_params(&self) -> bool {
        self.myHaveSeenTLSParams
    }

    /// C++: `MMVersion NLHandler::networkVersion() const` [const]
    // MMVersion networkVersion() const {
    //         return myNetworkVersion;
    //     }
    #[allow(unused_variables)]
    pub fn network_version(&self) -> (i32, f64) /* MMVersion */ {
        self.myNetworkVersion
    }

    /// C++: `Parameterised * NLHandler::addPredecessorConstraint(int, const SUMOSAXAttributes &, MSRailSignal *)` [static]
    // Parameterised*
    // NLHandler::addPredecessorConstraint(int element, const SUMOSAXAttributes& attrs, MSRailSignal* rs) {
    //     if (rs == nullptr) {
    //         throw InvalidArgument("Rail signal '" + toString((SumoXMLTag)element) + "' constraint must occur within a railSignalConstraints element");
    //     }
    //     bool ok = true;
    //     const std::string tripId = attrs.get<std::string>(SUMO_ATTR_TRIP_ID, nullptr, ok);
    //     const std::string signalID = attrs.get<std::string>(SUMO_ATTR_TLID, nullptr, ok);
    //     const std::string foesString = attrs.get<std::string>(SUMO_ATTR_FOES, nullptr, ok);
    //     const std::vector<std::string> foes = StringTokenizer(foesString).getVector();
    //     const int limit = attrs.getOpt<int>(SUMO_ATTR_LIMIT, nullptr, ok, (int)foes.size());
    //     const bool active = attrs.getOpt<bool>(SUMO_ATTR_ACTIVE, nullptr, ok, true);
    //
    //     if (!MSNet::getInstance()->getTLSControl().knows(signalID)) {
    //         throw InvalidArgument("Rail signal '" + signalID + "' in railSignalConstraints is not known");
    //     }
    //     MSRailSignal* signal = dynamic_cast<MSRailSignal*>(MSNet::getInstance()->getTLSControl().get(signalID).getDefault());
    //     if (signal == nullptr) {
    //         throw InvalidArgument("Traffic light '" + signalID + "' is not a rail signal");
    //     }
    //     MSRailSignalConstraint::ConstraintType type;
    //     switch (element) {
    //         case SUMO_TAG_PREDECESSOR:
    //             type = MSRailSignalConstraint::ConstraintType::PREDECESSOR;
    //             break;
    //         case SUMO_TAG_INSERTION_PREDECESSOR:
    //             type = MSRailSignalConstraint::ConstraintType::INSERTION_PREDECESSOR;
    //             break;
    //         case SUMO_TAG_FOE_INSERTION:
    //             type = MSRailSignalConstraint::ConstraintType::FOE_INSERTION;
    //             break;
    //         case SUMO_TAG_INSERTION_ORDER:
    //             type = MSRailSignalConstraint::ConstraintType::INSERTION_ORDER;
    //             break;
    //         case SUMO_TAG_BIDI_PREDECESSOR:
    //             type = MSRailSignalConstraint::ConstraintType::BIDI_PREDECESSOR;
    //             break;
    //         default:
    //             throw InvalidArgument("Unsupported rail signal constraint '" + toString((SumoXMLTag)element) + "'");
    //     }
    //     Parameterised* result = nullptr;
    //     if (ok) {
    //         for (const std::string& foe : foes) {
    //             MSRailSignalConstraint* c = new MSRailSignalConstraint_Predecessor(type, signal, foe, limit, active);
    //             rs->addConstraint(tripId, c);
    //             // XXX if there are multiple foes, only one constraint will receive the parameters
    //             result = c;
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn add_predecessor_constraint(element: i32, attrs: &SUMOSAXAttributes, rs: Ptr<MSRailSignal>) -> Ptr<Parameterised> {
        if rs.is_null() {
            throw_invalid_argument(format!("Rail signal '{}' constraint must occur within a railSignalConstraints element", tag_to_string(element)));
        }
        let mut ok = true;
        let tripId = attrs.get::<String>(SUMO_ATTR_TRIP_ID, "", &mut ok, true);
        let signalID = attrs.get::<String>(SUMO_ATTR_TLID, "", &mut ok, true);
        let foesString = attrs.get::<String>(SUMO_ATTR_FOES, "", &mut ok, true);
        let foes: Vec<String> = StringTokenizer::new(&foesString).get_vector();
        let limit = attrs.get_opt::<i32>(SUMO_ATTR_LIMIT, "", &mut ok, foes.len() as i32, true);
        let active = attrs.get_opt::<bool>(SUMO_ATTR_ACTIVE, "", &mut ok, true, true);

        let tlsControl = MSNet::get_instance().m().get_tls_control();
        if !tlsControl.knows(&signalID) {
            throw_invalid_argument(format!("Rail signal '{}' in railSignalConstraints is not known", signalID));
        }
        // shortcut: dynamic_cast<MSRailSignal*> has no runtime type check here, so a non-rail-signal TLS is not rejected, upgrade when MSTrafficLightLogic exposes its subtype
        let signal: Ptr<MSRailSignal> = tlsControl.get(&signalID).get_default().cast::<MSRailSignal>();
        if signal.is_null() {
            throw_invalid_argument(format!("Traffic light '{}' is not a rail signal", signalID));
        }
        let type_ = match element {
            SUMO_TAG_PREDECESSOR => MSRailSignalConstraint_ConstraintType::PREDECESSOR,
            SUMO_TAG_INSERTION_PREDECESSOR => MSRailSignalConstraint_ConstraintType::INSERTION_PREDECESSOR,
            SUMO_TAG_FOE_INSERTION => MSRailSignalConstraint_ConstraintType::FOE_INSERTION,
            SUMO_TAG_INSERTION_ORDER => MSRailSignalConstraint_ConstraintType::INSERTION_ORDER,
            SUMO_TAG_BIDI_PREDECESSOR => MSRailSignalConstraint_ConstraintType::BIDI_PREDECESSOR,
            _ => throw_invalid_argument(format!("Unsupported rail signal constraint '{}'", tag_to_string(element))),
        };
        let mut result: Ptr<Parameterised> = Ptr::null();
        if ok {
            for foe in foes {
                // C++ `new`: the constraint lives on for the whole simulation
                let c: Ptr<MSRailSignalConstraint_Predecessor> = Ptr::from_raw(Box::into_raw(Box::new(MSRailSignalConstraint_Predecessor::new(type_, signal, &foe, limit, active))));
                rs.m().add_constraint(&tripId, c.cast::<MSRailSignalConstraint>());
                // XXX if there are multiple foes, only one constraint will receive the parameters
                result = Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*c.as_raw()).base.base) });
            }
        }
        result
    }

    /// C++: `void NLHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // NLHandler::myStartElement(int element,
    //                           const SUMOSAXAttributes& attrs) {
    //     try {
    //         switch (element) {
    //             case SUMO_TAG_NET: {
    //                 bool ok;
    //                 MSGlobals::gLefthand = attrs.getOpt<bool>(SUMO_ATTR_LEFTHAND, nullptr, ok, false);
    //                 myHaveJunctionHigherSpeeds = attrs.getOpt<bool>(SUMO_ATTR_HIGHER_SPEED, nullptr, ok, false);
    //                 myNetworkVersion = StringUtils::toVersion(attrs.get<std::string>(SUMO_ATTR_VERSION, nullptr, ok, false));
    //                 break;
    //             }
    //             case SUMO_TAG_EDGE:
    //                 beginEdgeParsing(attrs);
    //                 break;
    //             case SUMO_TAG_LANE:
    //                 addLane(attrs);
    //                 break;
    //             case SUMO_TAG_NEIGH:
    //                 if (!myCurrentIsInternalToSkip) {
    //                     myEdgeControlBuilder.addNeigh(attrs.getString(SUMO_ATTR_LANE));
    //                 }
    //                 myHaveSeenNeighs = true;
    //                 break;
    //             case SUMO_TAG_JUNCTION:
    //                 openJunction(attrs);
    //                 initJunctionLogic(attrs);
    //                 break;
    //             case SUMO_TAG_PHASE:
    //                 addPhase(attrs);
    //                 break;
    //             case SUMO_TAG_CONDITION:
    //                 addCondition(attrs);
    //                 break;
    //             case SUMO_TAG_ASSIGNMENT:
    //                 addAssignment(attrs);
    //                 break;
    //             case SUMO_TAG_FUNCTION:
    //                 addFunction(attrs);
    //                 break;
    //             case SUMO_TAG_CONNECTION:
    //                 addConnection(attrs);
    //                 break;
    //             case SUMO_TAG_CONFLICT:
    //                 addConflict(attrs);
    //                 break;
    //             case SUMO_TAG_TLLOGIC:
    //                 initTrafficLightLogic(attrs);
    //                 break;
    //             case SUMO_TAG_REQUEST:
    //                 addRequest(attrs);
    //                 break;
    //             case SUMO_TAG_WAUT:
    //                 openWAUT(attrs);
    //                 break;
    //             case SUMO_TAG_WAUT_SWITCH:
    //                 addWAUTSwitch(attrs);
    //                 break;
    //             case SUMO_TAG_WAUT_JUNCTION:
    //                 addWAUTJunction(attrs);
    //                 break;
    //             case SUMO_TAG_E1DETECTOR:
    //             case SUMO_TAG_INDUCTION_LOOP:
    //                 addE1Detector(attrs);
    //                 break;
    //             case SUMO_TAG_E2DETECTOR:
    //             case SUMO_TAG_LANE_AREA_DETECTOR:
    //                 addE2Detector(attrs);
    //                 break;
    //             case SUMO_TAG_E3DETECTOR:
    //             case SUMO_TAG_ENTRY_EXIT_DETECTOR:
    //                 beginE3Detector(attrs);
    //                 break;
    //             case SUMO_TAG_DET_ENTRY:
    //                 addE3Entry(attrs);
    //                 break;
    //             case SUMO_TAG_DET_EXIT:
    //                 addE3Exit(attrs);
    //                 break;
    //             case SUMO_TAG_INSTANT_INDUCTION_LOOP:
    //                 addInstantE1Detector(attrs);
    //                 break;
    //             case SUMO_TAG_VSS:
    //                 myTriggerBuilder.parseAndBuildLaneSpeedTrigger(myNet, attrs, getFileName());
    //                 break;
    //             case SUMO_TAG_CALIBRATOR:
    //                 myTriggerBuilder.parseAndBuildCalibrator(myNet, attrs, getFileName());
    //                 break;
    //             case SUMO_TAG_REROUTER:
    //                 myTriggerBuilder.parseAndBuildRerouter(myNet, attrs);
    //                 break;
    //             case SUMO_TAG_BUS_STOP:
    //             case SUMO_TAG_TRAIN_STOP:
    //             case SUMO_TAG_CONTAINER_STOP:
    //                 myTriggerBuilder.parseAndBuildStoppingPlace(myNet, attrs, (SumoXMLTag)element);
    //                 myLastParameterised.push_back(myTriggerBuilder.getCurrentStop());
    //                 break;
    //             case SUMO_TAG_PARKING_SPACE:
    //                 myTriggerBuilder.parseAndAddLotEntry(attrs);
    //                 break;
    //             case SUMO_TAG_PARKING_AREA:
    //                 myTriggerBuilder.parseAndBeginParkingArea(myNet, attrs);
    //                 myLastParameterised.push_back(myTriggerBuilder.getCurrentStop());
    //                 break;
    //             case SUMO_TAG_ACCESS:
    //                 myTriggerBuilder.addAccess(myNet, attrs);
    //                 break;
    //             case SUMO_TAG_CHARGING_STATION:
    //                 myTriggerBuilder.parseAndBuildChargingStation(myNet, attrs);
    //                 myLastParameterised.push_back(myTriggerBuilder.getCurrentStop());
    //                 break;
    //             case SUMO_TAG_OVERHEAD_WIRE_SEGMENT:
    //                 myTriggerBuilder.parseAndBuildOverheadWireSegment(myNet, attrs);
    //                 break;
    //             case SUMO_TAG_OVERHEAD_WIRE_SECTION:
    //                 myTriggerBuilder.parseAndBuildOverheadWireSection(myNet, attrs);
    //                 break;
    //             case SUMO_TAG_TRACTION_SUBSTATION:
    //                 myTriggerBuilder.parseAndBuildTractionSubstation(myNet, attrs);
    //                 break;
    //             case SUMO_TAG_OVERHEAD_WIRE_CLAMP:
    //                 myTriggerBuilder.parseAndBuildOverheadWireClamp(myNet, attrs);
    //                 break;
    //             case SUMO_TAG_VTYPEPROBE:
    //                 addVTypeProbeDetector(attrs);
    //                 break;
    //             case SUMO_TAG_ROUTEPROBE:
    //                 addRouteProbeDetector(attrs);
    //                 break;
    //             case SUMO_TAG_MEANDATA_EDGE:
    //                 addEdgeLaneMeanData(attrs, SUMO_TAG_MEANDATA_EDGE);
    //                 break;
    //             case SUMO_TAG_MEANDATA_LANE:
    //                 addEdgeLaneMeanData(attrs, SUMO_TAG_MEANDATA_LANE);
    //                 break;
    //             case SUMO_TAG_TIMEDEVENT:
    //                 myActionBuilder.addAction(attrs, getFileName());
    //                 break;
    //             case SUMO_TAG_VAPORIZER:
    //                 myTriggerBuilder.buildVaporizer(attrs);
    //                 break;
    //             case SUMO_TAG_LOCATION:
    //                 setLocation(attrs);
    //                 break;
    //             case SUMO_TAG_TAZ:
    //                 addDistrict(attrs);
    //                 break;
    //             case SUMO_TAG_TAZSOURCE:
    //                 addDistrictEdge(attrs, true);
    //                 break;
    //             case SUMO_TAG_TAZSINK:
    //                 addDistrictEdge(attrs, false);
    //                 break;
    //             case SUMO_TAG_ROUNDABOUT:
    //                 addRoundabout(attrs);
    //                 break;
    //             case SUMO_TAG_TYPE: {
    //                 bool ok = true;
    //                 myCurrentTypeID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //                 break;
    //             }
    //             case SUMO_TAG_RESTRICTION: {
    //                 bool ok = true;
    //                 const SUMOVehicleClass svc = getVehicleClassID(attrs.get<std::string>(SUMO_ATTR_VCLASS, myCurrentTypeID.c_str(), ok));
    //                 const double speed = attrs.get<double>(SUMO_ATTR_SPEED, myCurrentTypeID.c_str(), ok);
    //                 if (ok) {
    //                     myNet.addRestriction(myCurrentTypeID, svc, speed);
    //                 }
    //                 if (myNetIsLoaded) {
    //                     myHaveSeenAdditionalSpeedRestrictions = true;
    //                 }
    //                 break;
    //             }
    //             case SUMO_TAG_PREFERENCE: {
    //                 bool ok = true;
    //                 const std::string routingType = attrs.get<std::string>(SUMO_ATTR_ROUTINGTYPE, nullptr, ok);
    //                 const double prio = attrs.get<double>(SUMO_ATTR_PRIORITY, routingType.c_str(), ok);
    //                 if (prio <= 0) {
    //                     throw InvalidArgument("In preference for routingType '" + routingType + "', priority must be positve");
    //                 }
    //                 if (attrs.hasAttribute(SUMO_ATTR_VCLASSES)) {
    //                     StringTokenizer st(attrs.get<std::string>(SUMO_ATTR_VCLASSES, routingType.c_str(), ok));
    //                     for (std::string className : st.getVector()) {
    //                         myNet.addPreference(routingType, getVehicleClassID(className), prio);
    //                     }
    //                 } else if (!attrs.hasAttribute(SUMO_ATTR_VTYPES)) {
    //                     // general preferenze applying to all types and vClasses
    //                     myNet.addPreference(routingType, "", prio);
    //                 }
    //                 if (attrs.hasAttribute(SUMO_ATTR_VTYPES)) {
    //                     StringTokenizer st(attrs.get<std::string>(SUMO_ATTR_VTYPES, routingType.c_str(), ok));
    //                     for (std::string typeName : st.getVector()) {
    //                         myNet.addPreference(routingType, typeName, prio);
    //                     }
    //                 }
    //                 break;
    //             }
    //             case SUMO_TAG_STOPOFFSET: {
    //                 bool ok = true;
    //                 const StopOffset stopOffset(attrs, ok);
    //                 if (!ok) {
    //                     WRITE_ERROR(myEdgeControlBuilder.reportCurrentEdgeOrLane());
    //                 } else {
    //                     myEdgeControlBuilder.addStopOffsets(stopOffset);
    //                 }
    //                 break;
    //             }
    //             case SUMO_TAG_RAILSIGNAL_CONSTRAINTS: {
    //                 bool ok = true;
    //                 const std::string signalID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //                 if (!MSNet::getInstance()->getTLSControl().knows(signalID)) {
    //                     throw InvalidArgument("Rail signal '" + signalID + "' in railSignalConstraints is not known");
    //                 }
    //                 myConstrainedSignal = dynamic_cast<MSRailSignal*>(MSNet::getInstance()->getTLSControl().get(signalID).getDefault());
    //                 if (myConstrainedSignal == nullptr) {
    //                     throw InvalidArgument("Traffic light '" + signalID + "' is not a rail signal");
    //                 }
    //                 break;
    //             }
    //             case SUMO_TAG_PREDECESSOR: // intended fall-through
    //             case SUMO_TAG_FOE_INSERTION: // intended fall-through
    //             case SUMO_TAG_INSERTION_PREDECESSOR: // intended fall-through
    //             case SUMO_TAG_INSERTION_ORDER: // intended fall-through
    //             case SUMO_TAG_BIDI_PREDECESSOR:
    //                 myLastParameterised.push_back(addPredecessorConstraint(element, attrs, myConstrainedSignal));
    //                 break;
    //             case SUMO_TAG_DEADLOCK:
    //                 addDeadlock(attrs);
    //                 break;
    //             case SUMO_TAG_ROUTEPART:
    //                 addDeadlockRoute(attrs);
    //                 break;
    //             default:
    //                 break;
    //         }
    //     } catch (InvalidArgument& e) {
    //         myCurrentIsBroken = true;
    //         WRITE_ERROR(e.what());
    //     }
    //     MSRouteHandler::myStartElement(element, attrs);
    //     if (element == SUMO_TAG_PARAM && !myCurrentIsBroken) {
    //         addParam(attrs);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        let result = try_catch_args(|| match element {
            SUMO_TAG_NET => {
                let mut ok = true;
                set_lefthand(attrs.get_opt::<bool>(SUMO_ATTR_LEFTHAND, "", &mut ok, false, true));
                self.myHaveJunctionHigherSpeeds = attrs.get_opt::<bool>(SUMO_ATTR_HIGHER_SPEED, "", &mut ok, false, true);
                self.myNetworkVersion = unwrap_or_throw(utils::common::string_utils::to_version(&attrs.get::<String>(SUMO_ATTR_VERSION, "", &mut ok, false)));
            }
            SUMO_TAG_EDGE => self.begin_edge_parsing(attrs),
            SUMO_TAG_LANE => self.add_lane(attrs),
            SUMO_TAG_NEIGH => {
                if !self.myCurrentIsInternalToSkip {
                    self.myEdgeControlBuilder.m().add_neigh(attrs.get_string(SUMO_ATTR_LANE));
                }
                self.myHaveSeenNeighs = true;
            }
            SUMO_TAG_JUNCTION => {
                self.open_junction(attrs);
                self.init_junction_logic(attrs);
            }
            SUMO_TAG_PHASE => self.add_phase(attrs),
            SUMO_TAG_CONDITION => self.add_condition(attrs),
            SUMO_TAG_ASSIGNMENT => self.add_assignment(attrs),
            SUMO_TAG_FUNCTION => self.add_function(attrs),
            SUMO_TAG_CONNECTION => self.add_connection(attrs),
            SUMO_TAG_CONFLICT => self.add_conflict(attrs),
            SUMO_TAG_TLLOGIC => self.init_traffic_light_logic(attrs),
            SUMO_TAG_REQUEST => self.add_request(attrs),
            SUMO_TAG_WAUT => self.open_waut(attrs),
            SUMO_TAG_WAUT_SWITCH => self.add_waut_switch(attrs),
            SUMO_TAG_WAUT_JUNCTION => self.add_waut_junction(attrs),
            SUMO_TAG_E1DETECTOR | SUMO_TAG_INDUCTION_LOOP => self.add_e1_detector(attrs),
            SUMO_TAG_E2DETECTOR | SUMO_TAG_LANE_AREA_DETECTOR => self.add_e2_detector(attrs),
            SUMO_TAG_E3DETECTOR | SUMO_TAG_ENTRY_EXIT_DETECTOR => self.begin_e3_detector(attrs),
            SUMO_TAG_DET_ENTRY => self.add_e3_entry(attrs),
            SUMO_TAG_DET_EXIT => self.add_e3_exit(attrs),
            SUMO_TAG_INSTANT_INDUCTION_LOOP => self.add_instant_e1_detector(attrs),
            SUMO_TAG_VSS => {
                let file = self.get_file_name().to_string();
                self.myTriggerBuilder.m().parse_and_build_lane_speed_trigger(self.myNet.m(), attrs, &file);
            }
            SUMO_TAG_CALIBRATOR => {
                let file = self.get_file_name().to_string();
                self.myTriggerBuilder.m().parse_and_build_calibrator(self.myNet.m(), attrs, &file);
            }
            SUMO_TAG_REROUTER => self.myTriggerBuilder.m().parse_and_build_rerouter(self.myNet.m(), attrs),
            SUMO_TAG_BUS_STOP | SUMO_TAG_TRAIN_STOP | SUMO_TAG_CONTAINER_STOP => {
                self.myTriggerBuilder.m().parse_and_build_stopping_place(self.myNet.m(), attrs, element);
                let stop = self.myTriggerBuilder.m().get_current_stop();
                self.myLastParameterised.push(stopping_place_as_parameterised(stop));
            }
            SUMO_TAG_PARKING_SPACE => self.myTriggerBuilder.m().parse_and_add_lot_entry(attrs),
            SUMO_TAG_PARKING_AREA => {
                self.myTriggerBuilder.m().parse_and_begin_parking_area(self.myNet.m(), attrs);
                let stop = self.myTriggerBuilder.m().get_current_stop();
                self.myLastParameterised.push(stopping_place_as_parameterised(stop));
            }
            SUMO_TAG_ACCESS => self.myTriggerBuilder.m().add_access(self.myNet.m(), attrs),
            SUMO_TAG_CHARGING_STATION => {
                self.myTriggerBuilder.m().parse_and_build_charging_station(self.myNet.m(), attrs);
                let stop = self.myTriggerBuilder.m().get_current_stop();
                self.myLastParameterised.push(stopping_place_as_parameterised(stop));
            }
            SUMO_TAG_OVERHEAD_WIRE_SEGMENT => self.myTriggerBuilder.m().parse_and_build_overhead_wire_segment(self.myNet.m(), attrs),
            SUMO_TAG_OVERHEAD_WIRE_SECTION => self.myTriggerBuilder.m().parse_and_build_overhead_wire_section(self.myNet.m(), attrs),
            SUMO_TAG_TRACTION_SUBSTATION => self.myTriggerBuilder.m().parse_and_build_traction_substation(self.myNet.m(), attrs),
            SUMO_TAG_OVERHEAD_WIRE_CLAMP => self.myTriggerBuilder.m().parse_and_build_overhead_wire_clamp(self.myNet.m(), attrs),
            SUMO_TAG_VTYPEPROBE => self.add_v_type_probe_detector(attrs),
            SUMO_TAG_ROUTEPROBE => self.add_route_probe_detector(attrs),
            SUMO_TAG_MEANDATA_EDGE => self.add_edge_lane_mean_data(attrs, SUMO_TAG_MEANDATA_EDGE),
            SUMO_TAG_MEANDATA_LANE => self.add_edge_lane_mean_data(attrs, SUMO_TAG_MEANDATA_LANE),
            SUMO_TAG_TIMEDEVENT => {
                let file = self.get_file_name().to_string();
                unwrap_or_throw(self.myActionBuilder.add_action(attrs, &file));
            }
            SUMO_TAG_VAPORIZER => self.myTriggerBuilder.m().build_vaporizer(attrs),
            SUMO_TAG_LOCATION => self.set_location(attrs),
            SUMO_TAG_TAZ => self.add_district(attrs),
            SUMO_TAG_TAZSOURCE => self.add_district_edge(attrs, true),
            SUMO_TAG_TAZSINK => self.add_district_edge(attrs, false),
            SUMO_TAG_ROUNDABOUT => self.add_roundabout(attrs),
            SUMO_TAG_TYPE => {
                let mut ok = true;
                self.myCurrentTypeID = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
            }
            SUMO_TAG_RESTRICTION => {
                let mut ok = true;
                let svc = unwrap_or_throw(utils::common::sumo_vehicle_class::get_vehicle_class_id(&attrs.get::<String>(SUMO_ATTR_VCLASS, &self.myCurrentTypeID, &mut ok, true)));
                let speed = attrs.get::<f64>(SUMO_ATTR_SPEED, &self.myCurrentTypeID, &mut ok, true);
                if ok {
                    self.myNet.m().add_restriction(&self.myCurrentTypeID, svc, speed);
                }
                if self.myNetIsLoaded {
                    self.myHaveSeenAdditionalSpeedRestrictions = true;
                }
            }
            SUMO_TAG_PREFERENCE => {
                let mut ok = true;
                let routingType = attrs.get::<String>(SUMO_ATTR_ROUTINGTYPE, "", &mut ok, true);
                let prio = attrs.get::<f64>(SUMO_ATTR_PRIORITY, &routingType, &mut ok, true);
                if prio <= 0.0 {
                    throw_invalid_argument(format!("In preference for routingType '{}', priority must be positve", routingType));
                }
                if attrs.has_attribute(SUMO_ATTR_VCLASSES) {
                    let vclasses = attrs.get::<String>(SUMO_ATTR_VCLASSES, &routingType, &mut ok, true);
                    for className in StringTokenizer::new(&vclasses).get_vector() {
                        let svc = unwrap_or_throw(utils::common::sumo_vehicle_class::get_vehicle_class_id(&className));
                        self.myNet.m().add_preference(&routingType, svc, prio);
                    }
                } else if !attrs.has_attribute(SUMO_ATTR_VTYPES) {
                    // general preferenze applying to all types and vClasses
                    self.myNet.m().add_preference_2(&routingType, String::new(), prio);
                }
                if attrs.has_attribute(SUMO_ATTR_VTYPES) {
                    let vtypes = attrs.get::<String>(SUMO_ATTR_VTYPES, &routingType, &mut ok, true);
                    for typeName in StringTokenizer::new(&vtypes).get_vector() {
                        self.myNet.m().add_preference_2(&routingType, typeName, prio);
                    }
                }
            }
            SUMO_TAG_STOPOFFSET => {
                // shortcut: the StopOffset(attrs, ok) constructor is not ported (utils/common/sumo_vehicle_class.rs), so stop offsets from the net file are not applied yet; upgrade when it is
                let _ = attrs;
            }
            SUMO_TAG_RAILSIGNAL_CONSTRAINTS => {
                let mut ok = true;
                let signalID = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
                let tls = MSNet::get_instance().m().get_tls_control();
                if !tls.knows(&signalID) {
                    throw_invalid_argument(format!("Rail signal '{}' in railSignalConstraints is not known", signalID));
                }
                // shortcut: dynamic_cast<MSRailSignal*> has no runtime type check here, upgrade when MSTrafficLightLogic exposes its subtype
                self.myConstrainedSignal = tls.get(&signalID).get_default().cast::<MSRailSignal>();
                if self.myConstrainedSignal.is_null() {
                    throw_invalid_argument(format!("Traffic light '{}' is not a rail signal", signalID));
                }
            }
            SUMO_TAG_PREDECESSOR | SUMO_TAG_FOE_INSERTION | SUMO_TAG_INSERTION_PREDECESSOR | SUMO_TAG_INSERTION_ORDER | SUMO_TAG_BIDI_PREDECESSOR => {
                let p = NLHandler::add_predecessor_constraint(element, attrs, self.myConstrainedSignal);
                self.myLastParameterised.push(p);
            }
            SUMO_TAG_DEADLOCK => self.add_deadlock(attrs),
            SUMO_TAG_ROUTEPART => self.add_deadlock_route(attrs),
            _ => {}
        });
        if let Err(what) = result {
            self.myCurrentIsBroken = true;
            write_error(&what);
        }
        self.base.my_start_element(element, attrs);
        if element == SUMO_TAG_PARAM && !self.myCurrentIsBroken {
            self.add_param(attrs);
        }
    }

    /// C++: `void NLHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // NLHandler::myEndElement(int element) {
    //     switch (element) {
    //         case SUMO_TAG_EDGE:
    //             closeEdge();
    //             break;
    //         case SUMO_TAG_LANE:
    //             myEdgeControlBuilder.closeLane();
    //             if (!myCurrentIsInternalToSkip && !myCurrentIsBroken) {
    //                 myLastParameterised.pop_back();
    //             }
    //             break;
    //         case SUMO_TAG_JUNCTION:
    //             if (!myCurrentIsBroken) {
    //                 try {
    //                     myJunctionControlBuilder.closeJunction(getFileName());
    //                 } catch (InvalidArgument& e) {
    //                     WRITE_ERROR(e.what());
    //                 }
    //             }
    //             myAmParsingTLLogicOrJunction = false;
    //             break;
    //         case SUMO_TAG_TLLOGIC:
    //             if (!myCurrentIsBroken) {
    //                 try {
    //                     myJunctionControlBuilder.closeTrafficLightLogic(getFileName());
    //                 } catch (InvalidArgument& e) {
    //                     for (MSPhaseDefinition* const phase : myJunctionControlBuilder.getLoadedPhases()) {
    //                         delete phase;
    //                     }
    //                     WRITE_ERROR(e.what());
    //                 }
    //             }
    //             myAmParsingTLLogicOrJunction = false;
    //             break;
    //         case SUMO_TAG_FUNCTION:
    //             closeFunction();
    //             break;
    //         case SUMO_TAG_WAUT:
    //             closeWAUT();
    //             break;
    //         case SUMO_TAG_RAILSIGNAL_CONSTRAINTS:
    //             myConstrainedSignal = nullptr;
    //             break;
    //         case SUMO_TAG_E1DETECTOR:
    //         case SUMO_TAG_INDUCTION_LOOP:
    //         case SUMO_TAG_INSTANT_INDUCTION_LOOP:
    //         case SUMO_TAG_E2DETECTOR:
    //         case SUMO_TAG_LANE_AREA_DETECTOR:
    //             if (!myCurrentIsBroken) {
    //                 myLastParameterised.pop_back();
    //             }
    //             break;
    //         case SUMO_TAG_E3DETECTOR:
    //         case SUMO_TAG_ENTRY_EXIT_DETECTOR:
    //             endE3Detector();
    //             if (!myCurrentIsBroken) {
    //                 myLastParameterised.pop_back();
    //             }
    //             break;
    //         case SUMO_TAG_PARKING_AREA:
    //             myTriggerBuilder.updateParkingAreaDefaultCapacity();
    //             myTriggerBuilder.endParkingArea();
    //             myLastParameterised.pop_back();
    //             break;
    //         case SUMO_TAG_BUS_STOP:
    //         case SUMO_TAG_TRAIN_STOP:
    //         case SUMO_TAG_CONTAINER_STOP:
    //         case SUMO_TAG_CHARGING_STATION:
    //             myTriggerBuilder.endStoppingPlace();
    //             myLastParameterised.pop_back();
    //             break;
    //         case SUMO_TAG_PREDECESSOR: // intended fall-through
    //         case SUMO_TAG_FOE_INSERTION: // intended fall-through
    //         case SUMO_TAG_INSERTION_PREDECESSOR: // intended fall-through
    //         case SUMO_TAG_INSERTION_ORDER: // intended fall-through
    //         case SUMO_TAG_BIDI_PREDECESSOR:
    //             myLastParameterised.pop_back();
    //             break;
    //         case SUMO_TAG_NET:
    //             // build junction graph
    //             for (JunctionGraph::iterator it = myJunctionGraph.begin(); it != myJunctionGraph.end(); ++it) {
    //                 MSEdge* edge = MSEdge::dictionary(it->first);
    //                 MSJunction* from = myJunctionControlBuilder.retrieve(it->second.first);
    //                 MSJunction* to = myJunctionControlBuilder.retrieve(it->second.second);
    //                 if (from == nullptr) {
    //                     WRITE_ERRORF(TL("Unknown from-node '%' for edge '%'."), it->second.first, it->first);
    //                     return;
    //                 }
    //                 if (to == nullptr) {
    //                     WRITE_ERRORF(TL("Unknown to-node '%' for edge '%'."), it->second.second, it->first);
    //                     return;
    //                 }
    //                 if (edge != nullptr) {
    //                     edge->setJunctions(from, to);
    //                     from->addOutgoing(edge);
    //                     to->addIncoming(edge);
    //                 }
    //             }
    //             myNetIsLoaded = true;
    //             break;
    //         default:
    //             break;
    //     }
    //     MSRouteHandler::myEndElement(element);
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        match element {
            SUMO_TAG_EDGE => self.close_edge(),
            SUMO_TAG_LANE => {
                self.myEdgeControlBuilder.m().close_lane();
                if !self.myCurrentIsInternalToSkip && !self.myCurrentIsBroken {
                    self.myLastParameterised.pop();
                }
            }
            SUMO_TAG_JUNCTION => {
                if !self.myCurrentIsBroken {
                    let basePath = self.get_file_name().to_string();
                    if let Err(what) = try_catch_args(|| self.myJunctionControlBuilder.m().close_junction(&basePath)) {
                        write_error(&what);
                    }
                }
                self.myAmParsingTLLogicOrJunction = false;
            }
            SUMO_TAG_TLLOGIC => {
                if !self.myCurrentIsBroken {
                    let basePath = self.get_file_name().to_string();
                    if let Err(what) = try_catch_args(|| self.myJunctionControlBuilder.m().close_traffic_light_logic(&basePath)) {
                        for phase in self.myJunctionControlBuilder.get_loaded_phases().clone() {
                            // C++ `delete phase`
                            if !phase.is_null() {
                                drop(unsafe { Box::from_raw(phase.as_raw()) });
                            }
                        }
                        write_error(&what);
                    }
                }
                self.myAmParsingTLLogicOrJunction = false;
            }
            SUMO_TAG_FUNCTION => self.close_function(),
            SUMO_TAG_WAUT => self.close_waut(),
            SUMO_TAG_RAILSIGNAL_CONSTRAINTS => self.myConstrainedSignal = Ptr::null(),
            SUMO_TAG_E1DETECTOR | SUMO_TAG_INDUCTION_LOOP | SUMO_TAG_INSTANT_INDUCTION_LOOP | SUMO_TAG_E2DETECTOR | SUMO_TAG_LANE_AREA_DETECTOR => {
                if !self.myCurrentIsBroken {
                    self.myLastParameterised.pop();
                }
            }
            SUMO_TAG_E3DETECTOR | SUMO_TAG_ENTRY_EXIT_DETECTOR => {
                self.end_e3_detector();
                if !self.myCurrentIsBroken {
                    self.myLastParameterised.pop();
                }
            }
            SUMO_TAG_PARKING_AREA => {
                self.myTriggerBuilder.m().update_parking_area_default_capacity();
                self.myTriggerBuilder.m().end_parking_area();
                self.myLastParameterised.pop();
            }
            SUMO_TAG_BUS_STOP | SUMO_TAG_TRAIN_STOP | SUMO_TAG_CONTAINER_STOP | SUMO_TAG_CHARGING_STATION => {
                self.myTriggerBuilder.m().end_stopping_place();
                self.myLastParameterised.pop();
            }
            SUMO_TAG_PREDECESSOR | SUMO_TAG_FOE_INSERTION | SUMO_TAG_INSERTION_PREDECESSOR | SUMO_TAG_INSERTION_ORDER | SUMO_TAG_BIDI_PREDECESSOR => {
                self.myLastParameterised.pop();
            }
            SUMO_TAG_NET => {
                // build junction graph
                for (edgeID, (fromID, toID)) in self.myJunctionGraph.iter() {
                    let edge = MSEdge::dictionary_2(edgeID);
                    let from = self.myJunctionControlBuilder.m().retrieve(fromID.clone());
                    let to = self.myJunctionControlBuilder.m().retrieve(toID.clone());
                    if from.is_null() {
                        write_error(&format!("Unknown from-node '{}' for edge '{}'.", fromID, edgeID));
                        return;
                    }
                    if to.is_null() {
                        write_error(&format!("Unknown to-node '{}' for edge '{}'.", toID, edgeID));
                        return;
                    }
                    if !edge.is_null() {
                        edge.m().set_junctions(from, to);
                        from.m().add_outgoing(edge);
                        to.m().add_incoming(edge);
                    }
                }
                self.myNetIsLoaded = true;
            }
            _ => {}
        }
        // shortcut: MSRouteHandler::myEndElement is not ported (no end-element hook on MSRouteHandler), so the base step is skipped; upgrade when microsim/ms_route_handler.rs ports it
    }

    /// C++: `void NLHandler::addParam(const SUMOSAXAttributes &)` [protected]
    // void
    // NLHandler::addParam(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string key = attrs.get<std::string>(SUMO_ATTR_KEY, nullptr, ok);
    //     // circumventing empty string test
    //     const std::string val = attrs.hasAttribute(SUMO_ATTR_VALUE) ? attrs.getString(SUMO_ATTR_VALUE) : "";
    //     if (myLastParameterised.size() > 0 && myLastParameterised.back() != nullptr) {
    //         myLastParameterised.back()->setParameter(key, val);
    //     }
    //     // set
    //     if (ok && myAmParsingTLLogicOrJunction) {
    //         assert(key != "");
    //         myJunctionControlBuilder.addParam(key, val);
    //         if (myNetIsLoaded) {
    //             myHaveSeenTLSParams = true;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_param(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let key = attrs.get::<String>(SUMO_ATTR_KEY, "", &mut ok, true);
        // circumventing empty string test
        let val = if attrs.has_attribute(SUMO_ATTR_VALUE) { attrs.get_string(SUMO_ATTR_VALUE) } else { String::new() };
        if let Some(last) = self.myLastParameterised.last() {
            if !last.is_null() {
                last.m().set_parameter(&key, &val);
            }
        }
        // set
        if ok && self.myAmParsingTLLogicOrJunction {
            debug_assert!(!key.is_empty());
            self.myJunctionControlBuilder.m().add_param(&key, &val);
            if self.myNetIsLoaded {
                self.myHaveSeenTLSParams = true;
            }
        }
    }

    /// C++: `void NLHandler::addE1Detector(const SUMOSAXAttributes &)` [virtual, protected]
    /// Builds an e1 detector using the given specification
    // void
    // NLHandler::addE1Detector(const SUMOSAXAttributes& attrs) {
    //     myCurrentIsBroken = false;
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     const SUMOTime period = attrs.getOptPeriod(id.c_str(), ok, SUMOTime_MAX_PERIOD);
    //     const double position = attrs.get<double>(SUMO_ATTR_POSITION, id.c_str(), ok);
    //     const double length = attrs.getOpt<double>(SUMO_ATTR_LENGTH, id.c_str(), ok, 0);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), ok, false);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok, "");
    //     const std::string vTypes = attrs.getOpt<std::string>(SUMO_ATTR_VTYPES, id.c_str(), ok, "");
    //     const std::string nextEdges = attrs.getOpt<std::string>(SUMO_ATTR_NEXT_EDGES, id.c_str(), ok, "");
    //     const bool mesoTLS = attrs.getOpt<bool>(SUMO_ATTR_MESO_TLS, id.c_str(), ok, false);
    //     const std::string lane = attrs.get<std::string>(SUMO_ATTR_LANE, id.c_str(), ok);
    //     const std::string file = attrs.get<std::string>(SUMO_ATTR_FILE, id.c_str(), ok);
    //     const std::string detectPersonsString = attrs.getOpt<std::string>(SUMO_ATTR_DETECT_PERSONS, id.c_str(), ok, "");
    //     const int detectPersons = parseDetectPersons(detectPersonsString, id, ok);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     try {
    //         Parameterised* det = myDetectorBuilder.buildInductLoop(id, lane, position, length, period,
    //                              FileHelpers::checkForRelativity(file, getFileName()),
    //                              friendlyPos, name, vTypes, nextEdges, mesoTLS, detectPersons);
    //         myLastParameterised.push_back(det);
    //     } catch (InvalidArgument& e) {
    //         myCurrentIsBroken = true;
    //         WRITE_ERROR(e.what());
    //     } catch (IOError& e) {
    //         myCurrentIsBroken = true;
    //         WRITE_ERROR(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_e1_detector(&mut self, attrs: &SUMOSAXAttributes) {
        self.myCurrentIsBroken = false;
        let mut ok = true;
        // get the id, report an error if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }
        let period = attrs.get_opt_period(&id, &mut ok, sumotime_max_period(), true);
        let position = attrs.get::<f64>(SUMO_ATTR_POSITION, &id, &mut ok, true);
        let length = attrs.get_opt::<f64>(SUMO_ATTR_LENGTH, &id, &mut ok, 0.0, true);
        let friendlyPos = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut ok, false, true);
        let name = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);
        let vTypes = attrs.get_opt::<String>(SUMO_ATTR_VTYPES, &id, &mut ok, String::new(), true);
        let nextEdges = attrs.get_opt::<String>(SUMO_ATTR_NEXT_EDGES, &id, &mut ok, String::new(), true);
        let mesoTLS = attrs.get_opt::<bool>(SUMO_ATTR_MESO_TLS, &id, &mut ok, false, true);
        let lane = attrs.get::<String>(SUMO_ATTR_LANE, &id, &mut ok, true);
        let file = attrs.get::<String>(SUMO_ATTR_FILE, &id, &mut ok, true);
        let detectPersonsString = attrs.get_opt::<String>(SUMO_ATTR_DETECT_PERSONS, &id, &mut ok, String::new(), true);
        let detectPersons = self.parse_detect_persons(&detectPersonsString, &id, &mut ok);
        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }
        let handlerFile = self.get_file_name().to_string();
        let result = try_catch_args(|| {
            let det = self.myDetectorBuilder.m().build_induct_loop(&id, &lane, position, length, period, &check_for_relativity(&file, &handlerFile), friendlyPos, name, &vTypes, &nextEdges, mesoTLS, detectPersons);
            self.myLastParameterised.push(det);
        });
        if let Err(what) = result {
            self.myCurrentIsBroken = true;
            write_error(&what);
        }
    }

    /// C++: `void NLHandler::addInstantE1Detector(const SUMOSAXAttributes &)` [virtual, protected]
    /// Builds an e1 detector using the given specification
    // void
    // NLHandler::addInstantE1Detector(const SUMOSAXAttributes& attrs) {
    //     myCurrentIsBroken = false;
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     const double position = attrs.get<double>(SUMO_ATTR_POSITION, id.c_str(), ok);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), ok, false);
    //     const std::string lane = attrs.get<std::string>(SUMO_ATTR_LANE, id.c_str(), ok);
    //     const std::string file = attrs.get<std::string>(SUMO_ATTR_FILE, id.c_str(), ok);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok, "");
    //     const std::string vTypes = attrs.getOpt<std::string>(SUMO_ATTR_VTYPES, id.c_str(), ok, "");
    //     const std::string nextEdges = attrs.getOpt<std::string>(SUMO_ATTR_NEXT_EDGES, id.c_str(), ok, "");
    //     const std::string detectPersonsString = attrs.getOpt<std::string>(SUMO_ATTR_DETECT_PERSONS, id.c_str(), ok, "");
    //     const int detectPersons = parseDetectPersons(detectPersonsString, id, ok);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     try {
    //         Parameterised* det = myDetectorBuilder.buildInstantInductLoop(id, lane, position, FileHelpers::checkForRelativity(file, getFileName()), friendlyPos, name, vTypes, nextEdges, detectPersons);
    //         myLastParameterised.push_back(det);
    //     } catch (InvalidArgument& e) {
    //         WRITE_ERROR(e.what());
    //     } catch (IOError& e) {
    //         WRITE_ERROR(e.what());
    //     }
    //     myCurrentIsBroken = true;
    // }
    #[allow(unused_variables)]
    pub fn add_instant_e1_detector(&mut self, attrs: &SUMOSAXAttributes) {
        self.myCurrentIsBroken = false;
        let mut ok = true;
        // get the id, report an error if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }
        let position = attrs.get::<f64>(SUMO_ATTR_POSITION, &id, &mut ok, true);
        let friendlyPos = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut ok, false, true);
        let lane = attrs.get::<String>(SUMO_ATTR_LANE, &id, &mut ok, true);
        let file = attrs.get::<String>(SUMO_ATTR_FILE, &id, &mut ok, true);
        let name = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);
        let vTypes = attrs.get_opt::<String>(SUMO_ATTR_VTYPES, &id, &mut ok, String::new(), true);
        let nextEdges = attrs.get_opt::<String>(SUMO_ATTR_NEXT_EDGES, &id, &mut ok, String::new(), true);
        let detectPersonsString = attrs.get_opt::<String>(SUMO_ATTR_DETECT_PERSONS, &id, &mut ok, String::new(), true);
        let detectPersons = self.parse_detect_persons(&detectPersonsString, &id, &mut ok);
        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }
        let handlerFile = self.get_file_name().to_string();
        let result = try_catch_args(|| {
            let det = self.myDetectorBuilder.m().build_instant_induct_loop(&id, &lane, position, &check_for_relativity(&file, &handlerFile), friendlyPos, name, &vTypes, &nextEdges, detectPersons);
            self.myLastParameterised.push(det);
        });
        if let Err(what) = result {
            write_error(&what);
        }
        self.myCurrentIsBroken = true;
    }

    /// C++: `void NLHandler::addE2Detector(const SUMOSAXAttributes &)` [virtual, protected]
    /// Builds an e2 detector using the given specification
    // void
    // NLHandler::addE2Detector(const SUMOSAXAttributes& attrs) {
    //     myCurrentIsBroken = false;
    //     // check whether this is a detector connected to a tls and optionally to a link
    //     bool ok = true;
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     const std::string lsaid = attrs.getOpt<std::string>(SUMO_ATTR_TLID, id.c_str(), ok, "");
    //     const std::string toLane = attrs.getOpt<std::string>(SUMO_ATTR_TO, id.c_str(), ok, "");
    //     const SUMOTime haltingTimeThreshold = attrs.getOptSUMOTimeReporting(SUMO_ATTR_HALTING_TIME_THRESHOLD, id.c_str(), ok, TIME2STEPS(1));
    //     const double haltingSpeedThreshold = attrs.getOpt<double>(SUMO_ATTR_HALTING_SPEED_THRESHOLD, id.c_str(), ok, 5.0f / 3.6f);
    //     const double jamDistThreshold = attrs.getOpt<double>(SUMO_ATTR_JAM_DIST_THRESHOLD, id.c_str(), ok, 10.0f);
    //     double position = attrs.getOpt<double>(SUMO_ATTR_POSITION, id.c_str(), ok, std::numeric_limits<double>::max());
    //     const double length = attrs.getOpt<double>(SUMO_ATTR_LENGTH, id.c_str(), ok, std::numeric_limits<double>::max());
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, id.c_str(), ok, false);
    //     const bool showDetector = attrs.getOpt<bool>(SUMO_ATTR_SHOW_DETECTOR, id.c_str(), ok, true);
    //     const std::string contStr = attrs.getOpt<std::string>(SUMO_ATTR_CONT, id.c_str(), ok, "");
    //     if (contStr != "") {
    //         WRITE_WARNINGF(TL("Ignoring deprecated argument 'cont' for E2 detector '%'"), id);
    //     }
    //     std::string lane = attrs.getOpt<std::string>(SUMO_ATTR_LANE, id.c_str(), ok, "");
    //     const std::string file = attrs.get<std::string>(SUMO_ATTR_FILE, id.c_str(), ok);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok, "");
    //     const std::string vTypes = attrs.getOpt<std::string>(SUMO_ATTR_VTYPES, id.c_str(), ok, "");
    //     const std::string nextEdges = attrs.getOpt<std::string>(SUMO_ATTR_NEXT_EDGES, id.c_str(), ok, "");
    //
    //     double endPosition = attrs.getOpt<double>(SUMO_ATTR_ENDPOS, id.c_str(), ok, std::numeric_limits<double>::max());
    //     const std::string lanes = attrs.getOpt<std::string>(SUMO_ATTR_LANES, id.c_str(), ok, ""); // lanes has priority to lane
    //     const std::string detectPersonsString = attrs.getOpt<std::string>(SUMO_ATTR_DETECT_PERSONS, id.c_str(), ok, "");
    //     const int detectPersons = parseDetectPersons(detectPersonsString, id, ok);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //
    //     bool lanesGiven = lanes != "";
    //     bool laneGiven = lane != "";
    //     if (!(lanesGiven || laneGiven)) {
    //         // in absence of any lane-specification assume specification by id
    //         WRITE_WARNING(TL("Trying to specify detector's lane by the given id since the argument 'lane' is missing."))
    //         lane = id;
    //         laneGiven = true;
    //     }
    //     bool lengthGiven = length != std::numeric_limits<double>::max();
    //     bool posGiven = position != std::numeric_limits<double>::max();
    //     bool endPosGiven = endPosition != std::numeric_limits<double>::max();
    //     bool lsaGiven = lsaid != "";
    //     bool toLaneGiven = toLane != "";
    //
    //     MSLane* clane = nullptr;
    //     std::vector<MSLane*> clanes;
    //     if (lanesGiven) {
    //         // If lanes is given, endPos and startPos are required. lane, and length are ignored
    //         std::string seps = " ,\t\n";
    //         StringTokenizer st = StringTokenizer(lanes, seps, true);
    // //        std::cout << "Parsing lanes..." << std::endl;
    //         while (st.hasNext()) {
    //             std::string nextLaneID = st.next();
    // //            std::cout << "Next: " << nextLaneID << std::endl;
    //             if (nextLaneID.find_first_of(seps) != nextLaneID.npos) {
    //                 continue;
    //             }
    //             clane = myDetectorBuilder.getLaneChecking(nextLaneID, SUMO_TAG_E2DETECTOR, id);
    //             clanes.push_back(clane);
    //         }
    //         if (clanes.size() == 0) {
    //             throw InvalidArgument("Malformed argument 'lanes' for E2Detector '" + id + "'.\nSpecify 'lanes' as a sequence of lane-IDs separated by whitespace or comma (',')");
    //         }
    //         if (laneGiven) {
    //             WRITE_WARNING("Ignoring argument 'lane' for E2Detector '" + id + "' since argument 'lanes' was given.\n"
    //                           "Usage combinations for positional specification: [lane, pos, length], [lane, endPos, length], or [lanes, pos, endPos]");
    //         }
    //         if (lengthGiven) {
    //             WRITE_WARNING("Ignoring argument 'length' for E2Detector '" + id + "' since argument 'lanes' was given.\n"
    //                           "Usage combinations for positional specification: [lane, pos, length], [lane, endPos, length], or [lanes, pos, endPos]");
    //         }
    //         if (!posGiven) {
    //             // assuming start pos == lane start
    //             position = 0;
    //             WRITE_WARNINGF(TL("Missing argument 'pos' for E2Detector '%'. Assuming detector start == lane start of lane '%'."), id, clanes[0]->getID());
    //         }
    //         if (!endPosGiven) {
    //             // assuming end pos == lane end
    //             endPosition = clanes[clanes.size() - 1]->getLength();
    //             WRITE_WARNINGF(TL("Missing argument 'endPos' for E2Detector '%'. Assuming detector end == lane end of lane '%'."), id, clanes[clanes.size() - 1]->getID());
    //         }
    //
    //     } else {
    //         if (!laneGiven) {
    //             std::stringstream ss;
    //             ss << "Missing argument 'lane' for E2Detector '" << id << "'."
    //                << "\nUsage combinations for positional specification: [lane, pos, length], [lane, endPos, length], or [lanes, pos, endPos]";
    //             throw InvalidArgument(ss.str());
    //         }
    //         clane = myDetectorBuilder.getLaneChecking(lane, SUMO_TAG_E2DETECTOR, id);
    //
    //         if (posGiven) {
    //             // start pos is given
    //             if (endPosGiven && lengthGiven) {
    //                 std::stringstream ss;
    //                 ss << "Ignoring argument 'endPos' for E2Detector '" << id << "' since argument 'pos' was given."
    //                    << "\nUsage combinations for positional specification: [lane, pos, length], [lane, endPos, length], or [lanes, pos, endPos]";
    //                 WRITE_WARNING(ss.str());
    //                 endPosition = std::numeric_limits<double>::max();
    //             }
    //             if (!lengthGiven && !endPosGiven) {
    //                 std::stringstream ss;
    //                 ss << "Missing arguments 'length'/'endPos' for E2Detector '" << id << "'. Assuming detector end == lane end of lane '" << lane << "'.";
    //                 WRITE_WARNING(ss.str());
    //                 endPosition = clane->getLength();
    //             }
    //         } else if (endPosGiven) {
    //             // endPos is given, pos is not given
    //             if (!lengthGiven) {
    //                 std::stringstream ss;
    //                 ss << "Missing arguments 'length'/'pos' for E2Detector '" << id << "'. Assuming detector start == lane start of lane '" << lane << "'.";
    //                 WRITE_WARNING(ss.str());
    //             }
    //         } else {
    //             std::stringstream ss;
    //             if (lengthGiven && fabs(length - clane->getLength()) > NUMERICAL_EPS) {
    //                 ss << "Incomplete positional specification for E2Detector '" << id << "'."
    //                    << "\nUsage combinations for positional specification: [lane, pos, length], [lane, endPos, length], or [lanes, pos, endPos]";
    //                 throw InvalidArgument(ss.str());
    //             }
    //             endPosition = clane->getLength();
    //             position = 0;
    //             ss << "Missing arguments 'pos'/'endPos' for E2Detector '" << id << "'. Assuming that the detector covers the whole lane '" << lane << "'.";
    //             WRITE_WARNING(ss.str());
    //         }
    //     }
    //
    //     // Period
    //
    //     SUMOTime period;
    //     if (!lsaGiven) {
    //         period = attrs.getOptPeriod(id.c_str(), ok, SUMOTime_MAX_PERIOD);
    //         if (!ok) {
    //             myCurrentIsBroken = true;
    //             return;
    //         }
    //     } else {
    //         period = attrs.getPeriod(id.c_str(), ok, false);
    //     }
    //
    //     // TLS
    //     MSTLLogicControl::TLSLogicVariants* tlls = nullptr;
    //     if (lsaGiven) {
    //         tlls = &myJunctionControlBuilder.getTLLogic(lsaid);
    //         if (tlls->getActive() == nullptr) {
    //             throw InvalidArgument("The detector '" + id + "' refers to an unknown lsa '" + lsaid + "'.");
    //         }
    //         if (period != -1) {
    //             WRITE_WARNINGF(TL("Ignoring argument 'period' for E2Detector '%' since argument 'tl' was given."), id);
    //             period = -1;
    //         }
    //     }
    //
    //     // Link
    //     MSLane* cToLane = nullptr;
    //     if (toLaneGiven) {
    //         cToLane = myDetectorBuilder.getLaneChecking(toLane, SUMO_TAG_E2DETECTOR, id);
    //     }
    //
    //     // File
    //     std::string filename;
    //     try {
    //         filename = FileHelpers::checkForRelativity(file, getFileName());
    //     } catch (IOError& e) {
    //         WRITE_ERROR(e.what());
    //     }
    //
    //     Parameterised* det;
    //     // Build detector
    //     if (lanesGiven) {
    //         // specification by a lane sequence
    //         det = myDetectorBuilder.buildE2Detector(id, clanes, position, endPosition, filename, period,
    //                                                 haltingTimeThreshold, haltingSpeedThreshold, jamDistThreshold,
    //                                                 name, vTypes, nextEdges, detectPersons, friendlyPos, showDetector,
    //                                                 tlls, cToLane);
    //     } else {
    //         // specification by start or end lane
    //         det = myDetectorBuilder.buildE2Detector(id, clane, position, endPosition, length, filename, period,
    //                                                 haltingTimeThreshold, haltingSpeedThreshold, jamDistThreshold,
    //                                                 name, vTypes, nextEdges, detectPersons, friendlyPos, showDetector,
    //                                                 tlls, cToLane);
    //     }
    //     myLastParameterised.push_back(det);
    // }
    #[allow(unused_variables)]
    pub fn add_e2_detector(&mut self, attrs: &SUMOSAXAttributes) {
        self.myCurrentIsBroken = false;
        // check whether this is a detector connected to a tls and optionally to a link
        let mut ok = true;
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        let lsaid = attrs.get_opt::<String>(SUMO_ATTR_TLID, &id, &mut ok, String::new(), true);
        let toLane = attrs.get_opt::<String>(SUMO_ATTR_TO, &id, &mut ok, String::new(), true);
        let haltingTimeThreshold = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_HALTING_TIME_THRESHOLD, &id, &mut ok, time2steps(1.0), true);
        let haltingSpeedThreshold = attrs.get_opt::<f64>(SUMO_ATTR_HALTING_SPEED_THRESHOLD, &id, &mut ok, (5.0f32 / 3.6f32) as f64, true);
        let jamDistThreshold = attrs.get_opt::<f64>(SUMO_ATTR_JAM_DIST_THRESHOLD, &id, &mut ok, 10.0f32 as f64, true);
        let mut position = attrs.get_opt::<f64>(SUMO_ATTR_POSITION, &id, &mut ok, f64::MAX, true);
        let length = attrs.get_opt::<f64>(SUMO_ATTR_LENGTH, &id, &mut ok, f64::MAX, true);
        let friendlyPos = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &id, &mut ok, false, true);
        let showDetector = attrs.get_opt::<bool>(SUMO_ATTR_SHOW_DETECTOR, &id, &mut ok, true, true);
        let contStr = attrs.get_opt::<String>(SUMO_ATTR_CONT, &id, &mut ok, String::new(), true);
        if contStr != "" {
            write_warning(&format!("Ignoring deprecated argument 'cont' for E2 detector '{}'", id));
        }
        let mut lane = attrs.get_opt::<String>(SUMO_ATTR_LANE, &id, &mut ok, String::new(), true);
        let file = attrs.get::<String>(SUMO_ATTR_FILE, &id, &mut ok, true);
        let name = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);
        let vTypes = attrs.get_opt::<String>(SUMO_ATTR_VTYPES, &id, &mut ok, String::new(), true);
        let nextEdges = attrs.get_opt::<String>(SUMO_ATTR_NEXT_EDGES, &id, &mut ok, String::new(), true);

        let mut endPosition = attrs.get_opt::<f64>(SUMO_ATTR_ENDPOS, &id, &mut ok, f64::MAX, true);
        let lanes = attrs.get_opt::<String>(SUMO_ATTR_LANES, &id, &mut ok, String::new(), true); // lanes has priority to lane
        let detectPersonsString = attrs.get_opt::<String>(SUMO_ATTR_DETECT_PERSONS, &id, &mut ok, String::new(), true);
        let detectPersons = self.parse_detect_persons(&detectPersonsString, &id, &mut ok);
        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }

        let mut lanesGiven = lanes != "";
        let mut laneGiven = lane != "";
        if !(lanesGiven || laneGiven) {
            // in absence of any lane-specification assume specification by id
            write_warning("Trying to specify detector's lane by the given id since the argument 'lane' is missing.");
            lane = id.clone();
            laneGiven = true;
        }
        let lengthGiven = length != f64::MAX;
        let mut posGiven = position != f64::MAX;
        let endPosGiven = endPosition != f64::MAX;
        let lsaGiven = lsaid != "";
        let toLaneGiven = toLane != "";

        let mut clane: Ptr<MSLane> = Ptr::null();
        let mut clanes: Vec<Ptr<MSLane>> = Vec::new();
        if lanesGiven {
            // If lanes is given, endPos and startPos are required. lane, and length are ignored
            let seps = " ,\t\n";
            let mut st = StringTokenizer::with_token(&lanes, seps, true);
            while st.has_next() {
                let nextLaneID = unwrap_or_throw(st.next());
                if nextLaneID.contains(|c| seps.contains(c)) {
                    continue;
                }
                clane = self.myDetectorBuilder.m().get_lane_checking(&nextLaneID, SUMO_TAG_E2DETECTOR, &id);
                clanes.push(clane);
            }
            if clanes.is_empty() {
                throw_invalid_argument(format!("Malformed argument 'lanes' for E2Detector '{}'.\nSpecify 'lanes' as a sequence of lane-IDs separated by whitespace or comma (',')", id));
            }
            if laneGiven {
                write_warning(&format!("Ignoring argument 'lane' for E2Detector '{}' since argument 'lanes' was given.\nUsage combinations for positional specification: [lane, pos, length], [lane, endPos, length], or [lanes, pos, endPos]", id));
            }
            if lengthGiven {
                write_warning(&format!("Ignoring argument 'length' for E2Detector '{}' since argument 'lanes' was given.\nUsage combinations for positional specification: [lane, pos, length], [lane, endPos, length], or [lanes, pos, endPos]", id));
            }
            if !posGiven {
                // assuming start pos == lane start
                position = 0.0;
                write_warning(&format!("Missing argument 'pos' for E2Detector '{}'. Assuming detector start == lane start of lane '{}'.", id, clanes[0].get_id()));
            }
            if !endPosGiven {
                // assuming end pos == lane end
                endPosition = clanes[clanes.len() - 1].get_length();
                write_warning(&format!("Missing argument 'endPos' for E2Detector '{}'. Assuming detector end == lane end of lane '{}'.", id, clanes[clanes.len() - 1].get_id()));
            }
        } else {
            if !laneGiven {
                throw_invalid_argument(format!("Missing argument 'lane' for E2Detector '{}'.\nUsage combinations for positional specification: [lane, pos, length], [lane, endPos, length], or [lanes, pos, endPos]", id));
            }
            clane = self.myDetectorBuilder.m().get_lane_checking(&lane, SUMO_TAG_E2DETECTOR, &id);

            if posGiven {
                // start pos is given
                if endPosGiven && lengthGiven {
                    write_warning(&format!("Ignoring argument 'endPos' for E2Detector '{}' since argument 'pos' was given.\nUsage combinations for positional specification: [lane, pos, length], [lane, endPos, length], or [lanes, pos, endPos]", id));
                    endPosition = f64::MAX;
                }
                if !lengthGiven && !endPosGiven {
                    write_warning(&format!("Missing arguments 'length'/'endPos' for E2Detector '{}'. Assuming detector end == lane end of lane '{}'.", id, lane));
                    endPosition = clane.get_length();
                }
            } else if endPosGiven {
                // endPos is given, pos is not given
                if !lengthGiven {
                    write_warning(&format!("Missing arguments 'length'/'pos' for E2Detector '{}'. Assuming detector start == lane start of lane '{}'.", id, lane));
                }
            } else {
                if lengthGiven && (length - clane.get_length()).abs() > utils::geom::NUMERICAL_EPS {
                    throw_invalid_argument(format!("Incomplete positional specification for E2Detector '{}'.\nUsage combinations for positional specification: [lane, pos, length], [lane, endPos, length], or [lanes, pos, endPos]", id));
                }
                endPosition = clane.get_length();
                position = 0.0;
                write_warning(&format!("Missing arguments 'pos'/'endPos' for E2Detector '{}'. Assuming that the detector covers the whole lane '{}'.", id, lane));
            }
        }

        // Period
        let mut period: SUMOTime;
        if !lsaGiven {
            period = attrs.get_opt_period(&id, &mut ok, sumotime_max_period(), true);
            if !ok {
                self.myCurrentIsBroken = true;
                return;
            }
        } else {
            period = attrs.get_period(&id, &mut ok, false);
        }

        // TLS
        let mut tlls: Ptr<MSTLLogicControl_TLSLogicVariants> = Ptr::null();
        if lsaGiven {
            tlls = Ptr::from_mut(self.myJunctionControlBuilder.m().get_tl_logic(&lsaid));
            if tlls.m().get_active().is_null() {
                throw_invalid_argument(format!("The detector '{}' refers to an unknown lsa '{}'.", id, lsaid));
            }
            if period != -1 {
                write_warning(&format!("Ignoring argument 'period' for E2Detector '{}' since argument 'tl' was given.", id));
                period = -1;
            }
        }

        // Link
        let mut cToLane: Ptr<MSLane> = Ptr::null();
        if toLaneGiven {
            cToLane = self.myDetectorBuilder.m().get_lane_checking(&toLane, SUMO_TAG_E2DETECTOR, &id);
        }

        // File
        // shortcut: checkForRelativity does not throw IOError in the port, so the C++ try/catch around it is not needed
        let filename = check_for_relativity(&file, &self.get_file_name().to_string());

        // Build detector
        let det: Ptr<Parameterised> = if lanesGiven {
            // specification by a lane sequence
            self.myDetectorBuilder.m().build_e2_detector_2(&id, clanes, position, endPosition, &filename, period, haltingTimeThreshold, haltingSpeedThreshold, jamDistThreshold, name, &vTypes, &nextEdges, detectPersons, friendlyPos, showDetector, tlls, cToLane)
        } else {
            // specification by start or end lane
            self.myDetectorBuilder.m().build_e2_detector(&id, clane, position, endPosition, length, &filename, period, haltingTimeThreshold, haltingSpeedThreshold, jamDistThreshold, name, &vTypes, &nextEdges, detectPersons, friendlyPos, showDetector, tlls, cToLane)
        };
        self.myLastParameterised.push(det);
    }

    /// C++: `void NLHandler::beginE3Detector(const SUMOSAXAttributes &)` [protected]
    /// Starts building of an e3 detector using the given specification
    // void
    // NLHandler::beginE3Detector(const SUMOSAXAttributes& attrs) {
    //     myCurrentIsBroken = false;
    //     bool ok = true;
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     const SUMOTime period = attrs.getOptPeriod(id.c_str(), ok, SUMOTime_MAX_PERIOD);
    //     const SUMOTime haltingTimeThreshold = attrs.getOptSUMOTimeReporting(SUMO_ATTR_HALTING_TIME_THRESHOLD, id.c_str(), ok, TIME2STEPS(1));
    //     const double haltingSpeedThreshold = attrs.getOpt<double>(SUMO_ATTR_HALTING_SPEED_THRESHOLD, id.c_str(), ok, 5.0f / 3.6f);
    //     const std::string file = attrs.get<std::string>(SUMO_ATTR_FILE, id.c_str(), ok);
    //     const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok, "");
    //     const std::string vTypes = attrs.getOpt<std::string>(SUMO_ATTR_VTYPES, id.c_str(), ok, "");
    //     const std::string nextEdges = attrs.getOpt<std::string>(SUMO_ATTR_NEXT_EDGES, id.c_str(), ok, "");
    //     const bool openEntry = attrs.getOpt<bool>(SUMO_ATTR_OPEN_ENTRY, id.c_str(), ok, false);
    //     const bool expectArrival = attrs.getOpt<bool>(SUMO_ATTR_EXPECT_ARRIVAL, id.c_str(), ok, false);
    //     const std::string detectPersonsString = attrs.getOpt<std::string>(SUMO_ATTR_DETECT_PERSONS, id.c_str(), ok, "");
    //     const int detectPersons = parseDetectPersons(detectPersonsString, id, ok);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     try {
    //         Parameterised* det = myDetectorBuilder.beginE3Detector(id,
    //                              FileHelpers::checkForRelativity(file, getFileName()),
    //                              period, haltingSpeedThreshold, haltingTimeThreshold, name, vTypes, nextEdges, detectPersons, openEntry, expectArrival);
    //         myLastParameterised.push_back(det);
    //     } catch (InvalidArgument& e) {
    //         myCurrentIsBroken = true;
    //         WRITE_ERROR(e.what());
    //     } catch (IOError& e) {
    //         myCurrentIsBroken = true;
    //         WRITE_ERROR(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn begin_e3_detector(&mut self, attrs: &SUMOSAXAttributes) {
        self.myCurrentIsBroken = false;
        let mut ok = true;
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        let period = attrs.get_opt_period(&id, &mut ok, sumotime_max_period(), true);
        let haltingTimeThreshold = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_HALTING_TIME_THRESHOLD, &id, &mut ok, time2steps(1.0), true);
        let haltingSpeedThreshold = attrs.get_opt::<f64>(SUMO_ATTR_HALTING_SPEED_THRESHOLD, &id, &mut ok, (5.0f32 / 3.6f32) as f64, true);
        let file = attrs.get::<String>(SUMO_ATTR_FILE, &id, &mut ok, true);
        let name = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);
        let vTypes = attrs.get_opt::<String>(SUMO_ATTR_VTYPES, &id, &mut ok, String::new(), true);
        let nextEdges = attrs.get_opt::<String>(SUMO_ATTR_NEXT_EDGES, &id, &mut ok, String::new(), true);
        let openEntry = attrs.get_opt::<bool>(SUMO_ATTR_OPEN_ENTRY, &id, &mut ok, false, true);
        let expectArrival = attrs.get_opt::<bool>(SUMO_ATTR_EXPECT_ARRIVAL, &id, &mut ok, false, true);
        let detectPersonsString = attrs.get_opt::<String>(SUMO_ATTR_DETECT_PERSONS, &id, &mut ok, String::new(), true);
        let detectPersons = self.parse_detect_persons(&detectPersonsString, &id, &mut ok);
        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }
        let handlerFile = self.get_file_name().to_string();
        let result = try_catch_args(|| {
            let det = self.myDetectorBuilder.m().begin_e3_detector(&id, &check_for_relativity(&file, &handlerFile), period, haltingSpeedThreshold, haltingTimeThreshold, name, &vTypes, &nextEdges, detectPersons, openEntry, expectArrival);
            self.myLastParameterised.push(det);
        });
        if let Err(what) = result {
            self.myCurrentIsBroken = true;
            write_error(&what);
        }
    }

    /// C++: `void NLHandler::addE3Entry(const SUMOSAXAttributes &)` [protected]
    /// Adds an entry to the currently processed e3 detector
    // void
    // NLHandler::addE3Entry(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const double position = attrs.get<double>(SUMO_ATTR_POSITION, myDetectorBuilder.getCurrentE3ID().c_str(), ok);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, myDetectorBuilder.getCurrentE3ID().c_str(), ok, false);
    //     const std::string lane = attrs.get<std::string>(SUMO_ATTR_LANE, myDetectorBuilder.getCurrentE3ID().c_str(), ok);
    //     if (!ok) {
    //         return;
    //     }
    //     myDetectorBuilder.addE3Entry(lane, position, friendlyPos);
    // }
    #[allow(unused_variables)]
    pub fn add_e3_entry(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let e3id = self.myDetectorBuilder.m().get_current_e3_id();
        let position = attrs.get::<f64>(SUMO_ATTR_POSITION, &e3id, &mut ok, true);
        let friendlyPos = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &e3id, &mut ok, false, true);
        let lane = attrs.get::<String>(SUMO_ATTR_LANE, &e3id, &mut ok, true);
        if !ok {
            return;
        }
        self.myDetectorBuilder.m().add_e3_entry(&lane, position, friendlyPos);
    }

    /// C++: `void NLHandler::addE3Exit(const SUMOSAXAttributes &)` [protected]
    /// Adds an exit to the currently processed e3 detector
    // void
    // NLHandler::addE3Exit(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const double position = attrs.get<double>(SUMO_ATTR_POSITION, myDetectorBuilder.getCurrentE3ID().c_str(), ok);
    //     const bool friendlyPos = attrs.getOpt<bool>(SUMO_ATTR_FRIENDLY_POS, myDetectorBuilder.getCurrentE3ID().c_str(), ok, false);
    //     const std::string lane = attrs.get<std::string>(SUMO_ATTR_LANE, myDetectorBuilder.getCurrentE3ID().c_str(), ok);
    //     if (!ok) {
    //         return;
    //     }
    //     myDetectorBuilder.addE3Exit(lane, position, friendlyPos);
    // }
    #[allow(unused_variables)]
    pub fn add_e3_exit(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let e3id = self.myDetectorBuilder.m().get_current_e3_id();
        let position = attrs.get::<f64>(SUMO_ATTR_POSITION, &e3id, &mut ok, true);
        let friendlyPos = attrs.get_opt::<bool>(SUMO_ATTR_FRIENDLY_POS, &e3id, &mut ok, false, true);
        let lane = attrs.get::<String>(SUMO_ATTR_LANE, &e3id, &mut ok, true);
        if !ok {
            return;
        }
        self.myDetectorBuilder.m().add_e3_exit(&lane, position, friendlyPos);
    }

    /// C++: `void NLHandler::endE3Detector()` [virtual, protected]
    /// Builds of an e3 detector using collected values
    // void
    // NLHandler::endE3Detector() {
    //     try {
    //         myDetectorBuilder.endE3Detector();
    //     } catch (InvalidArgument& e) {
    //         WRITE_ERROR(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn end_e3_detector(&mut self) {
        let result = try_catch_args(|| self.myDetectorBuilder.m().end_e3_detector());
        if let Err(what) = result {
            write_error(&what);
        }
    }

    /// C++: `void NLHandler::addVTypeProbeDetector(const SUMOSAXAttributes &)` [virtual, protected]
    /// Builds a vtype-detector using the given specification
    // void
    // NLHandler::addVTypeProbeDetector(const SUMOSAXAttributes& attrs) {
    //     WRITE_WARNING(TL("VTypeProbes are deprecated. Use fcd-output devices (assigned to the vType) instead."));
    //     bool ok = true;
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     SUMOTime period = attrs.getOptPeriod(id.c_str(), ok, SUMOTime_MAX_PERIOD);
    //     std::string type = attrs.getStringSecure(SUMO_ATTR_TYPE, "");
    //     std::string file = attrs.get<std::string>(SUMO_ATTR_FILE, id.c_str(), ok);
    //     if (!ok) {
    //         return;
    //     }
    //     try {
    //         myDetectorBuilder.buildVTypeProbe(id, type, period, FileHelpers::checkForRelativity(file, getFileName()));
    //     } catch (InvalidArgument& e) {
    //         WRITE_ERROR(e.what());
    //     } catch (IOError& e) {
    //         WRITE_ERROR(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_v_type_probe_detector(&mut self, attrs: &SUMOSAXAttributes) {
        write_warning("VTypeProbes are deprecated. Use fcd-output devices (assigned to the vType) instead.");
        let mut ok = true;
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        let period = attrs.get_opt_period(&id, &mut ok, sumotime_max_period(), true);
        let type_ = attrs.get_string_secure(SUMO_ATTR_TYPE, "");
        let file = attrs.get::<String>(SUMO_ATTR_FILE, &id, &mut ok, true);
        if !ok {
            return;
        }
        let handlerFile = self.get_file_name().to_string();
        let result = try_catch_args(|| {
            self.myDetectorBuilder.m().build_v_type_probe(&id, &type_, period, &check_for_relativity(&file, &handlerFile));
        });
        if let Err(what) = result {
            write_error(&what);
        }
    }

    /// C++: `void NLHandler::addRouteProbeDetector(const SUMOSAXAttributes &)` [virtual, protected]
    /// Builds a routeProbe-detector using the given specification
    // void
    // NLHandler::addRouteProbeDetector(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     SUMOTime period = attrs.getOptPeriod(id.c_str(), ok, SUMOTime_MAX_PERIOD);
    //     SUMOTime begin = attrs.getOptSUMOTimeReporting(SUMO_ATTR_BEGIN, id.c_str(), ok, -1);
    //     std::string edge = attrs.get<std::string>(SUMO_ATTR_EDGE, id.c_str(), ok);
    //     std::string file = attrs.get<std::string>(SUMO_ATTR_FILE, id.c_str(), ok);
    //     const std::string vTypes = attrs.getOpt<std::string>(SUMO_ATTR_VTYPES, id.c_str(), ok, "");
    //     if (!ok) {
    //         return;
    //     }
    //     try {
    //         myDetectorBuilder.buildRouteProbe(id, edge, period, begin,
    //                                           FileHelpers::checkForRelativity(file, getFileName()), vTypes);
    //     } catch (InvalidArgument& e) {
    //         WRITE_ERROR(e.what());
    //     } catch (IOError& e) {
    //         WRITE_ERROR(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_route_probe_detector(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        let period = attrs.get_opt_period(&id, &mut ok, sumotime_max_period(), true);
        let begin = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_BEGIN, &id, &mut ok, -1, true);
        let edge = attrs.get::<String>(SUMO_ATTR_EDGE, &id, &mut ok, true);
        let file = attrs.get::<String>(SUMO_ATTR_FILE, &id, &mut ok, true);
        let vTypes = attrs.get_opt::<String>(SUMO_ATTR_VTYPES, &id, &mut ok, String::new(), true);
        if !ok {
            return;
        }
        let handlerFile = self.get_file_name().to_string();
        let result = try_catch_args(|| {
            self.myDetectorBuilder.m().build_route_probe(&id, &edge, period, begin, &check_for_relativity(&file, &handlerFile), &vTypes);
        });
        if let Err(what) = result {
            write_error(&what);
        }
    }

    /// C++: `void NLHandler::addEdgeLaneMeanData(const SUMOSAXAttributes &, int)` [virtual, protected]
    /// Builds edge or lane base mean data collector using the given specification
    // void
    // NLHandler::addEdgeLaneMeanData(const SUMOSAXAttributes& attrs, int objecttype) {
    //     bool ok = true;
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     const double maxTravelTime = attrs.getOpt<double>(SUMO_ATTR_MAX_TRAVELTIME, id.c_str(), ok, 100000);
    //     const double minSamples = attrs.getOpt<double>(SUMO_ATTR_MIN_SAMPLES, id.c_str(), ok, 0);
    //     const double haltingSpeedThreshold = attrs.getOpt<double>(SUMO_ATTR_HALTING_SPEED_THRESHOLD, id.c_str(), ok, POSITION_EPS);
    //     const double haltingSpeedThresholdRel = attrs.getOpt<double>(SUMO_ATTR_HALTING_SPEED_THRESHOLD_RELATIVE, id.c_str(), ok, 0);
    //     const std::string excludeEmpty = attrs.getOpt<std::string>(SUMO_ATTR_EXCLUDE_EMPTY, id.c_str(), ok, "false");
    //     const bool withInternal = attrs.getOpt<bool>(SUMO_ATTR_WITH_INTERNAL, id.c_str(), ok, false);
    //     const bool trackVehicles = attrs.getOpt<bool>(SUMO_ATTR_TRACK_VEHICLES, id.c_str(), ok, false);
    //     const std::string detectPersonsString = attrs.getOpt<std::string>(SUMO_ATTR_DETECT_PERSONS, id.c_str(), ok, "");
    //     const std::string file = attrs.get<std::string>(SUMO_ATTR_FILE, id.c_str(), ok);
    //     const std::string type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, id.c_str(), ok, "");
    //     std::string vtypes = attrs.getOpt<std::string>(SUMO_ATTR_VTYPES, id.c_str(), ok, "");
    //     const std::string writeAttributes = attrs.getOpt<std::string>(SUMO_ATTR_WRITE_ATTRIBUTES, id.c_str(), ok, "");
    //     const SUMOTime period = attrs.getOptPeriod(id.c_str(), ok, -1);
    //     const SUMOTime begin = attrs.getOptSUMOTimeReporting(SUMO_ATTR_BEGIN, id.c_str(), ok, string2time(OptionsCont::getOptions().getString("begin")));
    //     const SUMOTime end = attrs.getOptSUMOTimeReporting(SUMO_ATTR_END, id.c_str(), ok, string2time(OptionsCont::getOptions().getString("end")));
    //     std::vector<std::string> edgeIDs = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_EDGES, id.c_str(), ok);
    //     const std::string edgesFile = attrs.getOpt<std::string>(SUMO_ATTR_EDGESFILE, id.c_str(), ok, "");
    //     const std::string aggregateStr = attrs.getOpt<std::string>(SUMO_ATTR_AGGREGATE, id.c_str(), ok, "false");
    //     AggregateType aggregate = AggregateType::NO;
    //     if (aggregateStr == "taz") {
    //         aggregate = AggregateType::TAZ;
    //     } else if (StringUtils::toBool(aggregateStr)) {
    //         aggregate = AggregateType::YES;
    //     }
    //     if (!ok) {
    //         return;
    //     }
    //     int detectPersons = 0;
    //     for (std::string mode : StringTokenizer(detectPersonsString).getVector()) {
    //         if (SUMOXMLDefinitions::PersonModeValues.hasString(mode)) {
    //             detectPersons |= (int)SUMOXMLDefinitions::PersonModeValues.get(mode);
    //         } else {
    //             WRITE_ERRORF(TL("Invalid person mode '%' in edgeData definition '%'"), mode, id);
    //             return;
    //         }
    //     }
    //     if (edgesFile != "") {
    //         std::ifstream strm(edgesFile.c_str());
    //         if (!strm.good()) {
    //             throw ProcessError("Could not load names of edges for edgeData definition '" + id + "' from '" + edgesFile + "'.");
    //         }
    //         while (strm.good()) {
    //             std::string name;
    //             strm >> name;
    //             // maybe we're loading an edge-selection
    //             if (StringUtils::startsWith(name, "edge:")) {
    //                 edgeIDs.push_back(name.substr(5));
    //             } else if (name != "") {
    //                 edgeIDs.push_back(name);
    //             }
    //         }
    //     }
    //     std::vector<MSEdge*> edges;
    //     for (const std::string& edgeID : edgeIDs) {
    //         MSEdge* edge = MSEdge::dictionary(edgeID);
    //         if (edge == nullptr) {
    //             WRITE_ERRORF(TL("Unknown edge '%' in edgeData definition '%'"), edgeID, id);
    //             return;
    //         }
    //         edges.push_back(edge);
    //     }
    //     bool useLanes = objecttype == SUMO_TAG_MEANDATA_LANE;
    //     if (useLanes && MSGlobals::gUseMesoSim && !OptionsCont::getOptions().getBool("meso-lane-queue")) {
    //         WRITE_WARNINGF(TL("LaneData '%' requested for mesoscopic simulation but --meso-lane-queue is not active. Falling back to edgeData."), id);
    //         useLanes = false;
    //     }
    //     try {
    //         myDetectorBuilder.createEdgeLaneMeanData(id, period, begin, end,
    //                 type, useLanes, excludeEmpty, withInternal, trackVehicles, detectPersons,
    //                 maxTravelTime, minSamples, haltingSpeedThreshold, haltingSpeedThresholdRel, vtypes, writeAttributes, edges, aggregate,
    //                 FileHelpers::checkForRelativity(file, getFileName()));
    //     } catch (InvalidArgument& e) {
    //         WRITE_ERROR(e.what());
    //     } catch (IOError& e) {
    //         WRITE_ERROR(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_edge_lane_mean_data(&mut self, attrs: &SUMOSAXAttributes, objecttype: i32) {
        let mut ok = true;
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        let maxTravelTime = attrs.get_opt::<f64>(SUMO_ATTR_MAX_TRAVELTIME, &id, &mut ok, 100000.0, true);
        let minSamples = attrs.get_opt::<f64>(SUMO_ATTR_MIN_SAMPLES, &id, &mut ok, 0.0, true);
        let haltingSpeedThreshold = attrs.get_opt::<f64>(SUMO_ATTR_HALTING_SPEED_THRESHOLD, &id, &mut ok, utils::geom::POSITION_EPS, true);
        let haltingSpeedThresholdRel = attrs.get_opt::<f64>(SUMO_ATTR_HALTING_SPEED_THRESHOLD_RELATIVE, &id, &mut ok, 0.0, true);
        let excludeEmpty = attrs.get_opt::<String>(SUMO_ATTR_EXCLUDE_EMPTY, &id, &mut ok, "false".to_string(), true);
        let withInternal = attrs.get_opt::<bool>(SUMO_ATTR_WITH_INTERNAL, &id, &mut ok, false, true);
        let trackVehicles = attrs.get_opt::<bool>(SUMO_ATTR_TRACK_VEHICLES, &id, &mut ok, false, true);
        let detectPersonsString = attrs.get_opt::<String>(SUMO_ATTR_DETECT_PERSONS, &id, &mut ok, String::new(), true);
        let file = attrs.get::<String>(SUMO_ATTR_FILE, &id, &mut ok, true);
        let type_ = attrs.get_opt::<String>(SUMO_ATTR_TYPE, &id, &mut ok, String::new(), true);
        let mut vtypes = attrs.get_opt::<String>(SUMO_ATTR_VTYPES, &id, &mut ok, String::new(), true);
        let writeAttributes = attrs.get_opt::<String>(SUMO_ATTR_WRITE_ATTRIBUTES, &id, &mut ok, String::new(), true);
        let period = attrs.get_opt_period(&id, &mut ok, -1, true);
        let begin = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_BEGIN, &id, &mut ok, unwrap_or_throw(utils::common::time::string2time(&utils::options::options_cont::get_string("begin"))), true);
        let end = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_END, &id, &mut ok, unwrap_or_throw(utils::common::time::string2time(&utils::options::options_cont::get_string("end"))), true);
        let mut edgeIDs: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_EDGES, &id, &mut ok, Vec::new(), true);
        let edgesFile = attrs.get_opt::<String>(SUMO_ATTR_EDGESFILE, &id, &mut ok, String::new(), true);
        let aggregateStr = attrs.get_opt::<String>(SUMO_ATTR_AGGREGATE, &id, &mut ok, "false".to_string(), true);
        let mut aggregate = AggregateType::NO;
        if aggregateStr == "taz" {
            aggregate = AggregateType::TAZ;
        } else if unwrap_or_throw(utils::common::string_utils::to_bool(&aggregateStr)) {
            aggregate = AggregateType::YES;
        }
        if !ok {
            return;
        }
        let mut detectPersons: i32 = 0;
        for mode in StringTokenizer::new(&detectPersonsString).get_vector() {
            if PersonModeValues.has_string(&mode) {
                detectPersons |= unwrap_or_throw(PersonModeValues.get(&mode)) as i32;
            } else {
                write_error(&format!("Invalid person mode '{}' in edgeData definition '{}'", mode, id));
                return;
            }
        }
        if edgesFile != "" {
            // shortcut: reads the whole file at once instead of streaming tokens with operator>>, same tokens for ordinary text files
            match std::fs::read_to_string(&edgesFile) {
                Ok(content) => {
                    for name in content.split_whitespace() {
                        // maybe we're loading an edge-selection
                        if let Some(edgeID) = name.strip_prefix("edge:") {
                            edgeIDs.push(edgeID.to_string());
                        } else {
                            edgeIDs.push(name.to_string());
                        }
                    }
                }
                Err(_) => std::panic::panic_any(ProcessError::new(format!("Could not load names of edges for edgeData definition '{}' from '{}'.", id, edgesFile))),
            }
        }
        let mut edges: Vec<Ptr<MSEdge>> = Vec::new();
        for edgeID in &edgeIDs {
            let edge = MSEdge::dictionary_2(edgeID);
            if edge.is_null() {
                write_error(&format!("Unknown edge '{}' in edgeData definition '{}'", edgeID, id));
                return;
            }
            edges.push(edge);
        }
        let mut useLanes = objecttype == SUMO_TAG_MEANDATA_LANE;
        if useLanes && use_meso_sim() && !utils::options::options_cont::get_bool("meso-lane-queue") {
            write_warning(&format!("LaneData '{}' requested for mesoscopic simulation but --meso-lane-queue is not active. Falling back to edgeData.", id));
            useLanes = false;
        }
        let handlerFile = self.get_file_name().to_string();
        let result = try_catch_args(|| {
            self.myDetectorBuilder.m().create_edge_lane_mean_data(
                &id,
                period,
                begin,
                end,
                &type_,
                useLanes,
                &excludeEmpty,
                withInternal,
                trackVehicles,
                detectPersons,
                maxTravelTime,
                minSamples,
                haltingSpeedThreshold,
                haltingSpeedThresholdRel,
                &vtypes,
                &writeAttributes,
                edges,
                aggregate,
                &check_for_relativity(&file, &handlerFile),
            );
        });
        if let Err(what) = result {
            write_error(&what);
        }
    }

    /// C++: `void NLHandler::addDeadlock(const SUMOSAXAttributes &)` [virtual, protected]
    /// Loads deadlock information for preparing additional rail signal checks
    // void
    // NLHandler::addDeadlock(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::vector<std::string> signalIDs = attrs.get<std::vector<std::string>>(SUMO_ATTR_SIGNALS, nullptr, ok);
    //     std::vector<const MSRailSignal*> signals;
    //     for (const std::string& id : signalIDs) {
    //         const MSTrafficLightLogic* tll = myJunctionControlBuilder.getTLLogicControlToUse().getActive(id);
    //         const MSRailSignal* rs = dynamic_cast<const MSRailSignal*>(tll);
    //         if (rs != nullptr) {
    //             signals.push_back(rs);
    //         } else {
    //             throw InvalidArgument("Rail signal '" + toString(id) + "' in " + toString(SUMO_TAG_DEADLOCK) + " is not known");
    //         }
    //     }
    //     MSRailSignalControl::getInstance().loadDeadlockCheck(signals);
    // }
    #[allow(unused_variables)]
    pub fn add_deadlock(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let signalIDs = attrs.get::<Vec<String>>(SUMO_ATTR_SIGNALS, "", &mut ok, true);
        let mut signals: Vec<Ptr<MSRailSignal>> = Vec::new();
        for id in signalIDs {
            let tll = self.myJunctionControlBuilder.m().get_tl_logic_control_to_use().get_active(&id);
            // shortcut: dynamic_cast<const MSRailSignal*> has no runtime type check here, upgrade when MSTrafficLightLogic exposes its subtype
            let rs = tll.cast::<MSRailSignal>();
            if !rs.is_null() {
                signals.push(rs);
            } else {
                throw_invalid_argument(format!("Rail signal '{}' in {} is not known", id, tag_to_string(SUMO_TAG_DEADLOCK)));
            }
        }
        MSRailSignalControl::get_instance().load_deadlock_check(signals);
    }

    /// C++: `void NLHandler::addDeadlockRoute(const SUMOSAXAttributes &)` [virtual, protected]
    // void
    // NLHandler::addDeadlockRoute(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     ConstMSEdgeVector edges;
    //     MSEdge::parseEdgesList(attrs.get<std::string>(SUMO_ATTR_EDGES, "deadlock", ok), edges, "deadlock");
    //     MSRailSignalControl::getInstance().loadDeadlockRoute(edges);
    // }
    #[allow(unused_variables)]
    pub fn add_deadlock_route(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let mut edges: ConstMSEdgeVector = Vec::new();
        MSEdge::parse_edges_list(&attrs.get::<String>(SUMO_ATTR_EDGES, "deadlock", &mut ok, true), &mut edges, "deadlock");
        MSRailSignalControl::get_instance().load_deadlock_route(&edges);
    }

    /// C++: `void NLHandler::closeEdge()` [virtual, protected]
    /// Closes the process of building an edge
    // void
    // NLHandler::closeEdge() {
    //     myLastParameterised.clear();
    //     // omit internal edges if not wished and broken edges
    //     if (myCurrentIsInternalToSkip || myCurrentIsBroken) {
    //         return;
    //     }
    //     try {
    //         MSEdge* e = myEdgeControlBuilder.closeEdge();
    //         MSEdge::dictionary(e->getID(), e);
    //         e->updateParameters(myLastEdgeParameters.getParametersMap());
    //     } catch (InvalidArgument& e) {
    //         WRITE_ERROR(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn close_edge(&mut self) {
        self.myLastParameterised.clear();
        // omit internal edges if not wished and broken edges
        if self.myCurrentIsInternalToSkip || self.myCurrentIsBroken {
            return;
        }
        let result = try_catch_args(|| {
            let e = self.myEdgeControlBuilder.m().close_edge();
            let id = e.get_id().to_string();
            MSEdge::dictionary(&id, e);
            e.m().base2.update_parameters(self.myLastEdgeParameters.get_parameters_map());
        });
        if let Err(what) = result {
            write_error(&what);
        }
    }

    /// C++: `void NLHandler::beginEdgeParsing(const SUMOSAXAttributes &)` [private]
    /// begins the processing of an edge
    // void
    // NLHandler::beginEdgeParsing(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     myCurrentIsBroken = false;
    //     // get the id, report an error if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     // parse the function
    //     const SumoXMLEdgeFunc func = attrs.getOpt<SumoXMLEdgeFunc>(SUMO_ATTR_FUNCTION, id.c_str(), ok, SumoXMLEdgeFunc::NORMAL);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     // omit internal edges if not wished
    //     if (id[0] == ':') {
    //         myHaveSeenInternalEdge = true;
    //         if (!MSGlobals::gUsingInternalLanes && (func == SumoXMLEdgeFunc::CROSSING || func == SumoXMLEdgeFunc::WALKINGAREA)) {
    //             myCurrentIsInternalToSkip = true;
    //             return;
    //         }
    //         std::string junctionID = SUMOXMLDefinitions::getJunctionIDFromInternalEdge(id);
    //         myJunctionGraph[id] = std::make_pair(junctionID, junctionID);
    //     } else {
    //         myHaveSeenDefaultLength |= !attrs.hasAttribute(SUMO_ATTR_LENGTH);
    //         myJunctionGraph[id] = std::make_pair(
    //                                   attrs.get<std::string>(SUMO_ATTR_FROM, id.c_str(), ok),
    //                                   attrs.get<std::string>(SUMO_ATTR_TO, id.c_str(), ok));
    //     }
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     myCurrentIsInternalToSkip = false;
    //     // get the street name
    //     const std::string streetName = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok, "");
    //     // get the edge type
    //     const std::string edgeType = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, id.c_str(), ok, "");
    //     const std::string routingType = attrs.getOpt<std::string>(SUMO_ATTR_ROUTINGTYPE, id.c_str(), ok, "");
    //     // get the edge priority (only for visualization)
    //     const int priority = attrs.getOpt<int>(SUMO_ATTR_PRIORITY, id.c_str(), ok, -1); // default taken from netbuild/NBFrame option 'default.priority'
    //     // get the bidi-edge
    //     const std::string bidi = attrs.getOpt<std::string>(SUMO_ATTR_BIDI, id.c_str(), ok, "");
    //     // get the kilometrage/mileage (for visualization and output)
    //     const double distance = attrs.getOpt<double>(SUMO_ATTR_DISTANCE, id.c_str(), ok, 0);
    //
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     //
    //     try {
    //         myEdgeControlBuilder.beginEdgeParsing(id, func, streetName, edgeType, routingType, priority, bidi, distance);
    //     } catch (InvalidArgument& e) {
    //         WRITE_ERROR(e.what());
    //         myCurrentIsBroken = true;
    //     }
    //
    //     if (func == SumoXMLEdgeFunc::CROSSING) {
    //         //get the crossingEdges attribute (to implement the other side of the road pushbutton)
    //         const std::string crossingEdges = attrs.getOpt<std::string>(SUMO_ATTR_CROSSING_EDGES, id.c_str(), ok, "");
    //         if (!crossingEdges.empty()) {
    //             std::vector<std::string> crossingEdgesVector;
    //             StringTokenizer edges(crossingEdges);
    //             while (edges.hasNext()) {
    //                 crossingEdgesVector.push_back(edges.next());
    //             }
    //             myEdgeControlBuilder.addCrossingEdges(crossingEdgesVector);
    //         }
    //     }
    //     myLastEdgeParameters.clearParameter();
    //     myLastParameterised.push_back(&myLastEdgeParameters);
    // }
    #[allow(unused_variables)]
    pub fn begin_edge_parsing(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        self.myCurrentIsBroken = false;
        // get the id, report an error if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }
        // parse the function
        let func = attrs.get_opt::<SumoXMLEdgeFunc>(SUMO_ATTR_FUNCTION, &id, &mut ok, SumoXMLEdgeFunc::NORMAL, true);
        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }
        // omit internal edges if not wished
        if id.starts_with(':') {
            self.myHaveSeenInternalEdge = true;
            if !using_internal_lanes() && (func == SumoXMLEdgeFunc::CROSSING || func == SumoXMLEdgeFunc::WALKINGAREA) {
                self.myCurrentIsInternalToSkip = true;
                return;
            }
            let junctionID = utils::xml::sumo_xml_definitions::get_junction_id_from_internal_edge(&id);
            self.myJunctionGraph.insert(id.clone(), (junctionID.clone(), junctionID));
        } else {
            self.myHaveSeenDefaultLength |= !attrs.has_attribute(SUMO_ATTR_LENGTH);
            let from = attrs.get::<String>(SUMO_ATTR_FROM, &id, &mut ok, true);
            let to = attrs.get::<String>(SUMO_ATTR_TO, &id, &mut ok, true);
            self.myJunctionGraph.insert(id.clone(), (from, to));
        }
        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }
        self.myCurrentIsInternalToSkip = false;
        // get the street name
        let streetName = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);
        // get the edge type
        let edgeType = attrs.get_opt::<String>(SUMO_ATTR_TYPE, &id, &mut ok, String::new(), true);
        let routingType = attrs.get_opt::<String>(SUMO_ATTR_ROUTINGTYPE, &id, &mut ok, String::new(), true);
        // get the edge priority (only for visualization)
        let priority = attrs.get_opt::<i32>(SUMO_ATTR_PRIORITY, &id, &mut ok, -1, true); // default taken from netbuild/NBFrame option 'default.priority'
        // get the bidi-edge
        let bidi = attrs.get_opt::<String>(SUMO_ATTR_BIDI, &id, &mut ok, String::new(), true);
        // get the kilometrage/mileage (for visualization and output)
        let distance = attrs.get_opt::<f64>(SUMO_ATTR_DISTANCE, &id, &mut ok, 0.0, true);

        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }
        let result = try_catch_args(|| {
            self.myEdgeControlBuilder.m().begin_edge_parsing(&id, func, &streetName, &edgeType, &routingType, priority, &bidi, distance);
        });
        if let Err(what) = result {
            write_error(&what);
            self.myCurrentIsBroken = true;
        }

        if func == SumoXMLEdgeFunc::CROSSING {
            // get the crossingEdges attribute (to implement the other side of the road pushbutton)
            let crossingEdges = attrs.get_opt::<String>(SUMO_ATTR_CROSSING_EDGES, &id, &mut ok, String::new(), true);
            if !crossingEdges.is_empty() {
                let mut crossingEdgesVector: Vec<String> = Vec::new();
                let mut edges = StringTokenizer::new(&crossingEdges);
                while edges.has_next() {
                    crossingEdgesVector.push(unwrap_or_throw(edges.next()));
                }
                self.myEdgeControlBuilder.m().add_crossing_edges(&crossingEdgesVector);
            }
        }
        self.myLastEdgeParameters.clear_parameter();
        self.myLastParameterised.push(Ptr::from_mut(&mut self.myLastEdgeParameters));
    }

    /// C++: `void NLHandler::addLane(const SUMOSAXAttributes &)` [private]
    /// adds a lane to the previously opened edge
    // void
    // NLHandler::addLane(const SUMOSAXAttributes& attrs) {
    //     // omit internal edges if not wished and broken edges
    //     if (myCurrentIsInternalToSkip || myCurrentIsBroken) {
    //         return;
    //     }
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     const double maxSpeed = attrs.get<double>(SUMO_ATTR_SPEED, id.c_str(), ok);
    //     const double friction = attrs.getOpt<double>(SUMO_ATTR_FRICTION, id.c_str(), ok, (double)(1.), false);
    //     const double length = attrs.get<double>(SUMO_ATTR_LENGTH, id.c_str(), ok);
    //     // sanity check values that could lead to crashing later on
    //     if (std::isnan(length)) {
    //         WRITE_ERRORF(TL("Attribute length of lane '%' is invalid (%)"), id, length);
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     if (std::isnan(maxSpeed)) {
    //         WRITE_ERRORF(TL("Attribute maxSpeed of lane '%' is invalid (%)"), id, maxSpeed);
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     const std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, id.c_str(), ok, "", false);
    //     const std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, id.c_str(), ok, "");
    //     const std::string changeLeftS = attrs.getOpt<std::string>(SUMO_ATTR_CHANGE_LEFT, id.c_str(), ok, "");
    //     const std::string changeRightS = attrs.getOpt<std::string>(SUMO_ATTR_CHANGE_RIGHT, id.c_str(), ok, "");
    //     const double width = attrs.getOpt<double>(SUMO_ATTR_WIDTH, id.c_str(), ok, SUMO_const_laneWidth);
    //     const PositionVector shape = attrs.get<PositionVector>(SUMO_ATTR_SHAPE, id.c_str(), ok);
    //     const PositionVector outlineShape = attrs.getOpt<PositionVector>(SUMO_ATTR_OUTLINESHAPE, id.c_str(), ok, PositionVector());
    //     const int index = attrs.get<int>(SUMO_ATTR_INDEX, id.c_str(), ok);
    //     const bool isRampAccel = attrs.getOpt<bool>(SUMO_ATTR_ACCELERATION, id.c_str(), ok, false);
    //     const std::string type = attrs.getOpt<std::string>(SUMO_ATTR_TYPE, id.c_str(), ok, "");
    //     if (shape.size() < 2 && myEdgeControlBuilder.getCurrentEdgeFunction() != SumoXMLEdgeFunc::WALKINGAREA) {
    //         WRITE_ERRORF(TL("Shape of lane '%' is broken.\n Can not build according edge."), id);
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     const SVCPermissions permissions = parseVehicleClasses(allow, disallow, myNetworkVersion);
    //     SVCPermissions changeLeft = parseVehicleClasses(changeLeftS, "", myNetworkVersion);
    //     SVCPermissions changeRight = parseVehicleClasses(changeRightS, "", myNetworkVersion);
    //     if (MSGlobals::gLefthand) {
    //         // internally, changeLeft always checks for the higher lane index
    //         // even though the higher lane index is to the right in a left-hand network
    //         std::swap(changeLeft, changeRight);
    //     }
    //     if (permissions != SVCAll || changeLeft != SVCAll || changeRight != SVCAll) {
    //         myNet.setPermissionsFound();
    //     }
    //     myCurrentIsBroken |= !ok;
    //     if (!myCurrentIsBroken) {
    //         try {
    //             MSLane* lane = myEdgeControlBuilder.addLane(id, maxSpeed, friction, length, shape, width, permissions, changeLeft, changeRight, index, isRampAccel, type, outlineShape);
    //             // insert the lane into the lane-dictionary, checking
    //             if (!MSLane::dictionary(id, lane)) {
    //                 delete lane;
    //                 WRITE_ERRORF(TL("Another lane with the id '%' exists."), id);
    //                 myCurrentIsBroken = true;
    //                 myLastParameterised.push_back(nullptr);
    //             } else {
    //                 myLastParameterised.push_back(lane);
    //             }
    //         } catch (InvalidArgument& e) {
    //             WRITE_ERROR(e.what());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_lane(&mut self, attrs: &SUMOSAXAttributes) {
        // omit internal edges if not wished and broken edges
        if self.myCurrentIsInternalToSkip || self.myCurrentIsBroken {
            return;
        }
        let mut ok = true;
        // get the id, report an error if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }
        let maxSpeed = attrs.get::<f64>(SUMO_ATTR_SPEED, &id, &mut ok, true);
        let friction = attrs.get_opt::<f64>(SUMO_ATTR_FRICTION, &id, &mut ok, 1.0, false);
        let length = attrs.get::<f64>(SUMO_ATTR_LENGTH, &id, &mut ok, true);
        // sanity check values that could lead to crashing later on
        if length.is_nan() {
            // shortcut: C++ streams NaN as "nan" through ostream; written literally here
            write_error(&format!("Attribute length of lane '{}' is invalid (nan)", id));
            self.myCurrentIsBroken = true;
            return;
        }
        if maxSpeed.is_nan() {
            write_error(&format!("Attribute maxSpeed of lane '{}' is invalid (nan)", id));
            self.myCurrentIsBroken = true;
            return;
        }
        let allow = attrs.get_opt::<String>(SUMO_ATTR_ALLOW, &id, &mut ok, String::new(), false);
        let disallow = attrs.get_opt::<String>(SUMO_ATTR_DISALLOW, &id, &mut ok, String::new(), true);
        let changeLeftS = attrs.get_opt::<String>(SUMO_ATTR_CHANGE_LEFT, &id, &mut ok, String::new(), true);
        let changeRightS = attrs.get_opt::<String>(SUMO_ATTR_CHANGE_RIGHT, &id, &mut ok, String::new(), true);
        let width = attrs.get_opt::<f64>(SUMO_ATTR_WIDTH, &id, &mut ok, utils::common::std_defs::SUMO_CONST_LANE_WIDTH, true);
        let shape = attrs.get::<PositionVector>(SUMO_ATTR_SHAPE, &id, &mut ok, true);
        let outlineShape = attrs.get_opt::<PositionVector>(SUMO_ATTR_OUTLINESHAPE, &id, &mut ok, PositionVector::default(), true);
        let index = attrs.get::<i32>(SUMO_ATTR_INDEX, &id, &mut ok, true);
        let isRampAccel = attrs.get_opt::<bool>(SUMO_ATTR_ACCELERATION, &id, &mut ok, false, true);
        let type_ = attrs.get_opt::<String>(SUMO_ATTR_TYPE, &id, &mut ok, String::new(), true);
        if shape.size() < 2 && self.myEdgeControlBuilder.m().get_current_edge_function() != SumoXMLEdgeFunc::WALKINGAREA {
            write_error(&format!("Shape of lane '{}' is broken.\n Can not build according edge.", id));
            self.myCurrentIsBroken = true;
            return;
        }
        let permissions = parse_vehicle_classes_v(&allow, &disallow, self.myNetworkVersion);
        let mut changeLeft = parse_vehicle_classes_v(&changeLeftS, "", self.myNetworkVersion);
        let mut changeRight = parse_vehicle_classes_v(&changeRightS, "", self.myNetworkVersion);
        if lefthand() {
            // internally, changeLeft always checks for the higher lane index
            // even though the higher lane index is to the right in a left-hand network
            std::mem::swap(&mut changeLeft, &mut changeRight);
        }
        if permissions != SVC_ALL || changeLeft != SVC_ALL || changeRight != SVC_ALL {
            self.myNet.m().set_permissions_found();
        }
        self.myCurrentIsBroken |= !ok;
        if !self.myCurrentIsBroken {
            let result = try_catch_args(|| {
                self.myEdgeControlBuilder.m().add_lane(&id, maxSpeed, friction, length, &shape, width, permissions, changeLeft, changeRight, index, isRampAccel, &type_, &outlineShape)
            });
            match result {
                Ok(lane) => {
                    // insert the lane into the lane-dictionary, checking
                    if !MSLane::dictionary(&id, lane) {
                        // C++ `delete lane`
                        drop(unsafe { Box::from_raw(lane.as_raw()) });
                        write_error(&format!("Another lane with the id '{}' exists.", id));
                        self.myCurrentIsBroken = true;
                        self.myLastParameterised.push(Ptr::null());
                    } else {
                        self.myLastParameterised.push(lane_as_parameterised(lane));
                    }
                }
                Err(what) => write_error(&what),
            }
        }
    }

    /// C++: `void NLHandler::addRequest(const SUMOSAXAttributes &)` [private]
    /// adds a request item to the current junction logic
    // void
    // NLHandler::addRequest(const SUMOSAXAttributes& attrs) {
    //     if (myCurrentIsBroken) {
    //         return;
    //     }
    //     bool ok = true;
    //     int request = attrs.get<int>(SUMO_ATTR_INDEX, nullptr, ok);
    //     bool cont = false;
    //     cont = attrs.getOpt<bool>(SUMO_ATTR_CONT, nullptr, ok, false);
    //     std::string response = attrs.get<std::string>(SUMO_ATTR_RESPONSE, nullptr, ok);
    //     std::string foes = attrs.get<std::string>(SUMO_ATTR_FOES, nullptr, ok);
    //     if (!ok) {
    //         return;
    //     }
    //     // store received information
    //     if (request >= 0 && response.length() > 0) {
    //         try {
    //             myJunctionControlBuilder.addLogicItem(request, response, foes, cont);
    //         } catch (InvalidArgument& e) {
    //             WRITE_ERROR(e.what());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_request(&mut self, attrs: &SUMOSAXAttributes) {
        if self.myCurrentIsBroken {
            return;
        }
        let mut ok = true;
        let request = attrs.get::<i32>(SUMO_ATTR_INDEX, "", &mut ok, true);
        let cont = attrs.get_opt::<bool>(SUMO_ATTR_CONT, "", &mut ok, false, true);
        let response = attrs.get::<String>(SUMO_ATTR_RESPONSE, "", &mut ok, true);
        let foes = attrs.get::<String>(SUMO_ATTR_FOES, "", &mut ok, true);
        if !ok {
            return;
        }
        // store received information
        if request >= 0 && !response.is_empty() {
            if let Err(what) = try_catch_args(|| self.myJunctionControlBuilder.m().add_logic_item(request, &response, &foes, cont)) {
                write_error(&what);
            }
        }
    }

    /// C++: `void NLHandler::initJunctionLogic(const SUMOSAXAttributes &)` [private]
    /// begins the reading of a junction row logic
    // void
    // NLHandler::initJunctionLogic(const SUMOSAXAttributes& attrs) {
    //     if (myCurrentIsBroken) {
    //         return;
    //     }
    //     myAmParsingTLLogicOrJunction = true;
    //     bool ok = true;
    //     // we either a have a junction or a legacy network with ROWLogic
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (ok) {
    //         myJunctionControlBuilder.initJunctionLogic(id);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_junction_logic(&mut self, attrs: &SUMOSAXAttributes) {
        if self.myCurrentIsBroken {
            return;
        }
        self.myAmParsingTLLogicOrJunction = true;
        let mut ok = true;
        // we either a have a junction or a legacy network with ROWLogic
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if ok {
            self.myJunctionControlBuilder.m().init_junction_logic(&id);
        }
    }

    /// C++: `void NLHandler::initTrafficLightLogic(const SUMOSAXAttributes &)` [private]
    /// begins the reading of a traffic lights logic
    // void
    // NLHandler::initTrafficLightLogic(const SUMOSAXAttributes& attrs) {
    //     myCurrentIsBroken = false;
    //     myAmParsingTLLogicOrJunction = true;
    //     bool ok = true;
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     std::string programID = attrs.getOpt<std::string>(SUMO_ATTR_PROGRAMID, id.c_str(), ok, "<unknown>");
    //     TrafficLightType type = TrafficLightType::STATIC;
    //     std::string typeS;
    //     if (myJunctionControlBuilder.getTLLogicControlToUse().get(id, programID) == nullptr) {
    //         // SUMO_ATTR_TYPE is not needed when only modifying the offset of an
    //         // existing program
    //         typeS = attrs.get<std::string>(SUMO_ATTR_TYPE, id.c_str(), ok);
    //         if (!ok) {
    //             myCurrentIsBroken = true;
    //             return;
    //         }
    //         if (SUMOXMLDefinitions::TrafficLightTypes.hasString(typeS)) {
    //             type = SUMOXMLDefinitions::TrafficLightTypes.get(typeS);
    //         } else {
    //             WRITE_ERRORF(TL("Traffic light '%' has unknown type '%'."), id, typeS);
    //         }
    //         if (MSGlobals::gUseMesoSim && type != TrafficLightType::STATIC
    //                 && type != TrafficLightType::ACTUATED
    //                 && type != TrafficLightType::RAIL_SIGNAL
    //                 && type != TrafficLightType::RAIL_CROSSING
    //                 && type != TrafficLightType::OFF) {
    //             if (!myHaveWarnedAboutInvalidTLType) {
    //                 WRITE_WARNINGF(TL("Traffic light type '%' cannot be used in mesoscopic simulation. Using '%' as fallback."), toString(type), toString(TrafficLightType::STATIC));
    //                 myHaveWarnedAboutInvalidTLType = true;
    //             }
    //             type = TrafficLightType::STATIC;
    //         }
    //     }
    //     SUMOTime offset = attrs.getOptOffsetReporting(SUMO_ATTR_OFFSET, id.c_str(), ok, 0);
    //     if (offset == SUMOTime_MAX) {
    //         offset = string2time(OptionsCont::getOptions().getString("begin"));
    //     }
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     myJunctionControlBuilder.initTrafficLightLogic(id, programID, type, offset);
    // }
    #[allow(unused_variables)]
    pub fn init_traffic_light_logic(&mut self, attrs: &SUMOSAXAttributes) {
        self.myCurrentIsBroken = false;
        self.myAmParsingTLLogicOrJunction = true;
        let mut ok = true;
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        let programID = attrs.get_opt::<String>(SUMO_ATTR_PROGRAMID, &id, &mut ok, "<unknown>".to_string(), true);
        let mut type_ = TrafficLightType::STATIC;
        let mut typeS = String::new();
        if self.myJunctionControlBuilder.m().get_tl_logic_control_to_use().get_2(&id, &programID).is_null() {
            // SUMO_ATTR_TYPE is not needed when only modifying the offset of an
            // existing program
            typeS = attrs.get::<String>(SUMO_ATTR_TYPE, &id, &mut ok, true);
            if !ok {
                self.myCurrentIsBroken = true;
                return;
            }
            if TrafficLightTypes.has_string(&typeS) {
                type_ = unwrap_or_throw(TrafficLightTypes.get(&typeS));
            } else {
                write_error(&format!("Traffic light '{}' has unknown type '{}'.", id, typeS));
            }
            if use_meso_sim() && type_ != TrafficLightType::STATIC && type_ != TrafficLightType::ACTUATED && type_ != TrafficLightType::RAIL_SIGNAL && type_ != TrafficLightType::RAIL_CROSSING && type_ != TrafficLightType::OFF {
                if !self.myHaveWarnedAboutInvalidTLType {
                    write_warning(&format!(
                        "Traffic light type '{}' cannot be used in mesoscopic simulation. Using '{}' as fallback.",
                        TrafficLightTypes.get_string(type_).unwrap_or_default(),
                        TrafficLightTypes.get_string(TrafficLightType::STATIC).unwrap_or_default()
                    ));
                    self.myHaveWarnedAboutInvalidTLType = true;
                }
                type_ = TrafficLightType::STATIC;
            }
        }
        let mut offset = attrs.get_opt_offset_reporting(SUMO_ATTR_OFFSET, &id, &mut ok, 0, true);
        if offset == SUMOTIME_MAX {
            offset = unwrap_or_throw(utils::common::time::string2time(&utils::options::options_cont::get_string("begin")));
        }
        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }
        self.myJunctionControlBuilder.m().init_traffic_light_logic(&id, &programID, type_, offset);
    }

    /// C++: `void NLHandler::addPhase(const SUMOSAXAttributes &)` [private]
    /// adds a phase to the traffic lights logic currently build
    // void
    // NLHandler::addPhase(const SUMOSAXAttributes& attrs) {
    //     // try to get the phase definition
    //     bool ok = true;
    //     const std::string& id = myJunctionControlBuilder.getActiveKey();
    //     const SUMOTime tDefault = MSPhaseDefinition::UNSPECIFIED_DURATION;
    //
    //     const SUMOTime duration = attrs.getSUMOTimeReporting(SUMO_ATTR_DURATION, myJunctionControlBuilder.getActiveKey().c_str(), ok);
    //     const std::string state = attrs.get<std::string>(SUMO_ATTR_STATE, nullptr, ok);
    //     if (duration == 0) {
    //         WRITE_ERROR("Duration of phase " + toString(myJunctionControlBuilder.getLoadedPhases().size())
    //                     + " for tlLogic '" + myJunctionControlBuilder.getActiveKey()
    //                     + "' program '" + myJunctionControlBuilder.getActiveSubKey() + "' is zero.");
    //         return;
    //     }
    //     if (!ok) {
    //         return;
    //     }
    //     MSPhaseDefinition* phase = new MSPhaseDefinition(duration, state);
    //
    //     // if the traffic light is an actuated traffic light, try to get
    //     //  the minimum and maximum durations
    //     phase->minDuration = attrs.getOptSUMOTimeReporting(SUMO_ATTR_MINDURATION, id.c_str(), ok, duration);
    //     // if minDur is set but not maxDur, assume high maxDur (avoid using the absolute max in case we do some arithmetic later)
    //     SUMOTime defaultMaxDur = attrs.hasAttribute(SUMO_ATTR_MINDURATION) ? std::numeric_limits<int>::max() : duration;
    //     phase->maxDuration = attrs.getOptSUMOTimeReporting(SUMO_ATTR_MAXDURATION, id.c_str(), ok, defaultMaxDur);
    //     phase->earliestEnd = attrs.getOptSUMOTimeReporting(SUMO_ATTR_EARLIEST_END, id.c_str(), ok, tDefault);
    //     phase->latestEnd = attrs.getOptSUMOTimeReporting(SUMO_ATTR_LATEST_END, id.c_str(), ok, tDefault);
    //     phase->nextPhases = attrs.getOpt<std::vector<int> >(SUMO_ATTR_NEXT, id.c_str(), ok);
    //     phase->earlyTarget = attrs.getOpt<std::string>(SUMO_ATTR_EARLY_TARGET, id.c_str(), ok);
    //     phase->finalTarget = attrs.getOpt<std::string>(SUMO_ATTR_FINAL_TARGET, id.c_str(), ok);
    //     phase->name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok);
    //
    //     phase->vehext = attrs.getOptSUMOTimeReporting(SUMO_ATTR_VEHICLEEXTENSION, id.c_str(), ok, tDefault);
    //     phase->yellow = attrs.getOptSUMOTimeReporting(SUMO_ATTR_YELLOW, id.c_str(), ok, tDefault);
    //     phase->red = attrs.getOptSUMOTimeReporting(SUMO_ATTR_RED, id.c_str(), ok, tDefault);
    //
    //     if (attrs.hasAttribute(SUMO_ATTR_TYPE)) {
    //         //SOTL attributes
    //         //If the type attribute is not present, the parsed phase is of type "undefined" (MSPhaseDefinition constructor),
    //         //in this way SOTL traffic light logic can recognize the phase as unsuitable or decides other
    //         //behaviors. See SOTL traffic light logic implementations.
    //         std::string phaseTypeString;
    //         try {
    //             phaseTypeString = attrs.get<std::string>(SUMO_ATTR_TYPE, id.c_str(), ok, false);
    //         } catch (EmptyData&) {
    //             MsgHandler::getWarningInstance()->inform("Empty type definition. Assuming phase type as SUMOSOTL_TagAttrDefinitions::SOTL_ATTL_TYPE_TRANSIENT");
    //             phase->myTransientNotDecisional = false;
    //         }
    //         if (phaseTypeString.find("decisional") != std::string::npos) {
    //             phase->myTransientNotDecisional = false;
    //         } else if (phaseTypeString.find("transient") != std::string::npos) {
    //             phase->myTransientNotDecisional = true;
    //         } else {
    //             MsgHandler::getWarningInstance()->inform("SOTL_ATTL_TYPE_DECISIONAL nor SOTL_ATTL_TYPE_TRANSIENT. Assuming phase type as SUMOSOTL_TagAttrDefinitions::SOTL_ATTL_TYPE_TRANSIENT");
    //             phase->myTransientNotDecisional = false;
    //         }
    //         phase->myCommit = (phaseTypeString.find("commit") != std::string::npos);
    //
    //         if (phaseTypeString.find("target") != std::string::npos) {
    //             std::string delimiter(" ,;");
    //             //Phase declared as target, getting targetLanes attribute
    //             try {
    //                 phase->myTargetLaneSet = StringTokenizer(attrs.getStringSecure(SUMO_ATTR_TARGETLANE, ""), " ,;", true).getVector();
    //             } catch (EmptyData&) {
    //                 MsgHandler::getErrorInstance()->inform("Missing targetLane definition for the target phase.");
    //                 delete phase;
    //                 return;
    //             }
    //         }
    //     }
    //
    //     if (phase->maxDuration < phase->minDuration) {
    //         WRITE_WARNINGF(TL("maxDur % should not be smaller than minDir % in phase of tlLogic %"), phase->maxDuration, phase->minDuration, id);
    //         phase->maxDuration = phase->duration;
    //     }
    //
    //     phase->myLastSwitch = string2time(OptionsCont::getOptions().getString("begin")) - 1; // SUMOTime-option
    //     myJunctionControlBuilder.addPhase(phase);
    // }
    #[allow(unused_variables)]
    pub fn add_phase(&mut self, attrs: &SUMOSAXAttributes) {
        // try to get the phase definition
        let mut ok = true;
        let id = self.myJunctionControlBuilder.m().get_active_key().to_string();
        let tDefault = UNSPECIFIED_DURATION;

        let duration = attrs.get_sumo_time_reporting(SUMO_ATTR_DURATION, &id, &mut ok, true);
        let state = attrs.get::<String>(SUMO_ATTR_STATE, "", &mut ok, true);
        if duration == 0 {
            let loaded = self.myJunctionControlBuilder.m().get_loaded_phases().len();
            write_error(&format!(
                "Duration of phase {} for tlLogic '{}' program '{}' is zero.",
                loaded,
                self.myJunctionControlBuilder.m().get_active_key(),
                self.myJunctionControlBuilder.m().get_active_sub_key()
            ));
            return;
        }
        if !ok {
            return;
        }
        let mut phase = MSPhaseDefinition::new(duration, &state, "");

        // if the traffic light is an actuated traffic light, try to get
        //  the minimum and maximum durations
        phase.minDuration = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_MINDURATION, &id, &mut ok, duration, true);
        // if minDur is set but not maxDur, assume high maxDur (avoid using the absolute max in case we do some arithmetic later)
        let defaultMaxDur = if attrs.has_attribute(SUMO_ATTR_MINDURATION) { i32::MAX as SUMOTime } else { duration };
        phase.maxDuration = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_MAXDURATION, &id, &mut ok, defaultMaxDur, true);
        phase.earliestEnd = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_EARLIEST_END, &id, &mut ok, tDefault, true);
        phase.latestEnd = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_LATEST_END, &id, &mut ok, tDefault, true);
        phase.nextPhases = attrs.get_opt::<Vec<i32>>(SUMO_ATTR_NEXT, &id, &mut ok, Vec::new(), true);
        phase.earlyTarget = attrs.get_opt::<String>(SUMO_ATTR_EARLY_TARGET, &id, &mut ok, String::new(), true);
        phase.finalTarget = attrs.get_opt::<String>(SUMO_ATTR_FINAL_TARGET, &id, &mut ok, String::new(), true);
        phase.name = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);

        phase.vehext = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_VEHICLEEXTENSION, &id, &mut ok, tDefault, true);
        phase.yellow = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_YELLOW, &id, &mut ok, tDefault, true);
        phase.red = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_RED, &id, &mut ok, tDefault, true);

        if attrs.has_attribute(SUMO_ATTR_TYPE) {
            // SOTL attributes
            // If the type attribute is not present, the parsed phase is of type "undefined" (MSPhaseDefinition constructor),
            // in this way SOTL traffic light logic can recognize the phase as unsuitable or decides other
            // behaviors. See SOTL traffic light logic implementations.
            let mut phaseTypeString = String::new();
            // C++ `catch (EmptyData&)`: an empty type makes the get() fail without a value
            let mut typeOk = true;
            let typeValue = attrs.get::<String>(SUMO_ATTR_TYPE, &id, &mut typeOk, false);
            if typeOk {
                phaseTypeString = typeValue;
            } else {
                write_warning("Empty type definition. Assuming phase type as SUMOSOTL_TagAttrDefinitions::SOTL_ATTL_TYPE_TRANSIENT");
                phase.myTransientNotDecisional = false;
            }
            if phaseTypeString.contains("decisional") {
                phase.myTransientNotDecisional = false;
            } else if phaseTypeString.contains("transient") {
                phase.myTransientNotDecisional = true;
            } else {
                write_warning("SOTL_ATTL_TYPE_DECISIONAL nor SOTL_ATTL_TYPE_TRANSIENT. Assuming phase type as SUMOSOTL_TagAttrDefinitions::SOTL_ATTL_TYPE_TRANSIENT");
                phase.myTransientNotDecisional = false;
            }
            phase.myCommit = phaseTypeString.contains("commit");

            if phaseTypeString.contains("target") {
                // Phase declared as target, getting targetLanes attribute
                // (the C++ `catch (EmptyData&)` around getStringSecure is unreachable: that getter never throws)
                let targetLanes = attrs.get_string_secure(SUMO_ATTR_TARGETLANE, "");
                phase.myTargetLaneSet = StringTokenizer::with_token(&targetLanes, " ,;", true).get_vector();
            }
        }

        if phase.maxDuration < phase.minDuration {
            write_warning(&format!("maxDur {} should not be smaller than minDir {} in phase of tlLogic {}", phase.maxDuration, phase.minDuration, id));
            phase.maxDuration = phase.duration;
        }

        phase.myLastSwitch = unwrap_or_throw(utils::common::time::string2time(&utils::options::options_cont::get_string("begin"))) - 1; // SUMOTime-option
        // C++ `new`: the phase is owned by the junction control builder
        self.myJunctionControlBuilder.m().add_phase(Ptr::from_raw(Box::into_raw(Box::new(phase))));
    }

    /// C++: `void NLHandler::addCondition(const SUMOSAXAttributes &)` [private]
    /// adds a switching condition to the traffic lights logic currently build
    // void
    // NLHandler::addCondition(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     const std::string value = attrs.get<std::string>(SUMO_ATTR_VALUE, id.c_str(), ok);
    //     if (!myJunctionControlBuilder.addCondition(id, value)) {
    //         WRITE_ERRORF(TL("Duplicate condition '%' in tlLogic '%'"), id, myJunctionControlBuilder.getActiveKey());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_condition(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        let value = attrs.get::<String>(SUMO_ATTR_VALUE, &id, &mut ok, true);
        if !self.myJunctionControlBuilder.m().add_condition(&id, &value) {
            write_error(&format!("Duplicate condition '{}' in tlLogic '{}'", id, self.myJunctionControlBuilder.m().get_active_key()));
        }
    }

    /// C++: `void NLHandler::addAssignment(const SUMOSAXAttributes &)` [private]
    /// adds a switching condition assignment to the traffic lights logic currently build
    // void
    // NLHandler::addAssignment(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     const std::string check = attrs.get<std::string>(SUMO_ATTR_CHECK, nullptr, ok);
    //     const std::string value = attrs.get<std::string>(SUMO_ATTR_VALUE, id.c_str(), ok);
    //     myJunctionControlBuilder.addAssignment(id, check, value);
    // }
    #[allow(unused_variables)]
    pub fn add_assignment(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        let check = attrs.get::<String>(SUMO_ATTR_CHECK, "", &mut ok, true);
        let value = attrs.get::<String>(SUMO_ATTR_VALUE, &id, &mut ok, true);
        self.myJunctionControlBuilder.m().add_assignment(&id, &check, &value);
    }

    /// C++: `void NLHandler::addFunction(const SUMOSAXAttributes &)` [private]
    /// adds a switching condition function to the traffic lights logic currently build
    // void
    // NLHandler::addFunction(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     const int nArgs = attrs.get<int>(SUMO_ATTR_NARGS, nullptr, ok);
    //     myJunctionControlBuilder.addFunction(id, nArgs);
    // }
    #[allow(unused_variables)]
    pub fn add_function(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        let nArgs = attrs.get::<i32>(SUMO_ATTR_NARGS, "", &mut ok, true);
        self.myJunctionControlBuilder.m().add_function(&id, nArgs);
    }

    /// C++: `void NLHandler::closeFunction()` [private]
    /// adds a switching condition function to the traffic lights logic currently build
    // void
    // NLHandler::closeFunction() {
    //     myJunctionControlBuilder.closeFunction();
    // }
    #[allow(unused_variables)]
    pub fn close_function(&mut self) {
        self.myJunctionControlBuilder.m().close_function();
    }

    /// C++: `void NLHandler::openJunction(const SUMOSAXAttributes &)` [virtual, private]
    /// opens a junction for processing
    // void
    // NLHandler::openJunction(const SUMOSAXAttributes& attrs) {
    //     myCurrentIsBroken = false;
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     PositionVector shape;
    //     if (attrs.hasAttribute(SUMO_ATTR_SHAPE)) {
    //         // inner junctions have no shape
    //         shape = attrs.getOpt<PositionVector>(SUMO_ATTR_SHAPE, id.c_str(), ok, PositionVector());
    //         if (shape.size() > 2) {
    //             shape.closePolygon();
    //         }
    //     }
    //     double x = attrs.get<double>(SUMO_ATTR_X, id.c_str(), ok);
    //     double y = attrs.get<double>(SUMO_ATTR_Y, id.c_str(), ok);
    //     double z = attrs.getOpt<double>(SUMO_ATTR_Z, id.c_str(), ok, 0);
    //     const SumoXMLNodeType type = attrs.get<SumoXMLNodeType>(SUMO_ATTR_TYPE, id.c_str(), ok);
    //     std::string key = attrs.getOpt<std::string>(SUMO_ATTR_KEY, id.c_str(), ok, "");
    //     std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, id.c_str(), ok, "");
    //     // incoming lanes
    //     std::vector<MSLane*> incomingLanes;
    //     parseLanes(id, attrs.getStringSecure(SUMO_ATTR_INCLANES, ""), incomingLanes, ok);
    //     // internal lanes
    //     std::vector<MSLane*> internalLanes;
    //     if (MSGlobals::gUsingInternalLanes) {
    //         parseLanes(id, attrs.getStringSecure(SUMO_ATTR_INTLANES, ""), internalLanes, ok);
    //     }
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //     } else {
    //         try {
    //             myJunctionControlBuilder.openJunction(id, key, type, Position(x, y, z), shape, incomingLanes, internalLanes, name);
    //         } catch (InvalidArgument& e) {
    //             WRITE_ERROR(e.what() + std::string("\n Can not build according junction."));
    //             myCurrentIsBroken = true;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn open_junction(&mut self, attrs: &SUMOSAXAttributes) {
        self.myCurrentIsBroken = false;
        let mut ok = true;
        // get the id, report an error if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }
        let mut shape = PositionVector::default();
        if attrs.has_attribute(SUMO_ATTR_SHAPE) {
            // inner junctions have no shape
            shape = attrs.get_opt::<PositionVector>(SUMO_ATTR_SHAPE, &id, &mut ok, PositionVector::default(), true);
            if shape.size() > 2 {
                shape.close_polygon();
            }
        }
        let x = attrs.get::<f64>(SUMO_ATTR_X, &id, &mut ok, true);
        let y = attrs.get::<f64>(SUMO_ATTR_Y, &id, &mut ok, true);
        let z = attrs.get_opt::<f64>(SUMO_ATTR_Z, &id, &mut ok, 0.0, true);
        let type_ = attrs.get::<SumoXMLNodeType>(SUMO_ATTR_TYPE, &id, &mut ok, true);
        let key = attrs.get_opt::<String>(SUMO_ATTR_KEY, &id, &mut ok, String::new(), true);
        let name = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);
        // incoming lanes
        let mut incomingLanes: Vec<Ptr<MSLane>> = Vec::new();
        self.parse_lanes(&id, &attrs.get_string_secure(SUMO_ATTR_INCLANES, ""), &mut incomingLanes, &mut ok);
        // internal lanes
        let mut internalLanes: Vec<Ptr<MSLane>> = Vec::new();
        if using_internal_lanes() {
            self.parse_lanes(&id, &attrs.get_string_secure(SUMO_ATTR_INTLANES, ""), &mut internalLanes, &mut ok);
        }
        if !ok {
            self.myCurrentIsBroken = true;
        } else {
            let result = try_catch_args(|| {
                self.myJunctionControlBuilder.m().open_junction(&id, &key, type_, Position::new3(x, y, z), &shape, &incomingLanes, &internalLanes, &name);
            });
            if let Err(what) = result {
                write_error(&format!("{}\n Can not build according junction.", what));
                self.myCurrentIsBroken = true;
            }
        }
    }

    /// C++: `void NLHandler::parseLanes(const std::string &, const std::string &, int &, bool &)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn parse_lanes(&mut self, junctionID: &str, def: &str, into: &mut Vec<Ptr<MSLane>>, ok: &mut bool) {
        let mut st = StringTokenizer::with_token(def, " ", false);
        while *ok && st.has_next() {
            let laneID = unwrap_or_throw(st.next());
            let lane = MSLane::dictionary_2(&laneID);
            if !using_internal_lanes() && laneID.starts_with(':') {
                continue;
            }
            if lane.is_null() {
                write_error(&format!("An unknown lane ('{}') was tried to be set as incoming to junction '{}'.", laneID, junctionID));
                *ok = false;
                continue;
            }
            into.push(lane);
        }
    }

    /// C++: `void NLHandler::addConnection(const SUMOSAXAttributes &)` [private]
    /// adds a connection
    // void
    // NLHandler::addConnection(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string fromID = attrs.get<std::string>(SUMO_ATTR_FROM, nullptr, ok);
    //     const std::string toID = attrs.get<std::string>(SUMO_ATTR_TO, nullptr, ok);
    //     if (!MSGlobals::gUsingInternalLanes && (fromID[0] == ':' || toID[0] == ':')) {
    //         std::string tlID = attrs.getOpt<std::string>(SUMO_ATTR_TLID, nullptr, ok, "");
    //         if (tlID != "") {
    //             int tlLinkIdx = attrs.get<int>(SUMO_ATTR_TLLINKINDEX, nullptr, ok);
    //             myJunctionControlBuilder.getTLLogic(tlID).ignoreLinkIndex(tlLinkIdx);
    //         }
    //         return;
    //     }
    //
    //     myCurrentLink = nullptr;
    //     try {
    //         const int fromLaneIdx = attrs.get<int>(SUMO_ATTR_FROM_LANE, nullptr, ok);
    //         const int toLaneIdx = attrs.get<int>(SUMO_ATTR_TO_LANE, nullptr, ok);
    //         LinkDirection dir = parseLinkDir(attrs.get<std::string>(SUMO_ATTR_DIR, nullptr, ok));
    //         LinkState state = parseLinkState(attrs.get<std::string>(SUMO_ATTR_STATE, nullptr, ok));
    //         const double foeVisibilityDistance = attrs.getOpt<double>(SUMO_ATTR_VISIBILITY_DISTANCE, nullptr, ok, state == LINKSTATE_ZIPPER ? 100 : 4.5);
    //         const bool keepClear = attrs.getOpt<bool>(SUMO_ATTR_KEEP_CLEAR, nullptr, ok, true);
    //         const bool indirect = attrs.getOpt<bool>(SUMO_ATTR_INDIRECT, nullptr, ok, false);
    //         std::string tlID = attrs.getOpt<std::string>(SUMO_ATTR_TLID, nullptr, ok, "");
    //         std::string viaID = attrs.getOpt<std::string>(SUMO_ATTR_VIA, nullptr, ok, "");
    //
    //         MSEdge* from = MSEdge::dictionaryHint(fromID, myPreviousEdgeIdx);
    //         if (from == nullptr) {
    //             WRITE_ERRORF(TL("Unknown from-edge '%' in connection."), fromID);
    //             return;
    //         }
    //         myPreviousEdgeIdx = from->getNumericalID();
    //         MSEdge* to = MSEdge::dictionary(toID);
    //         if (to == nullptr) {
    //             WRITE_ERRORF(TL("Unknown to-edge '%' in connection."), toID);
    //             return;
    //         }
    //         if (fromLaneIdx < 0 || fromLaneIdx >= (int)from->getLanes().size() ||
    //                 toLaneIdx < 0 || toLaneIdx >= (int)to->getLanes().size()) {
    //             WRITE_ERRORF(TL("Invalid lane index in connection from '%' to '%'."), from->getID(), to->getID());
    //             return;
    //         }
    //         MSLane* fromLane = from->getLanes()[fromLaneIdx];
    //         MSLane* toLane = to->getLanes()[toLaneIdx];
    //         assert(fromLane);
    //         assert(toLane);
    //
    //         MSTrafficLightLogic* logic = nullptr;
    //         int tlLinkIdx = -1;
    //         if (tlID != "") {
    //             tlLinkIdx = attrs.get<int>(SUMO_ATTR_TLLINKINDEX, nullptr, ok);
    //             // make sure that the index is in range
    //             logic = myJunctionControlBuilder.getTLLogic(tlID).getActive();
    //             if ((tlLinkIdx < 0 || tlLinkIdx >= (int)logic->getCurrentPhaseDef().getState().size())
    //                     && logic->getLogicType() != TrafficLightType::RAIL_SIGNAL
    //                     && logic->getLogicType() != TrafficLightType::RAIL_CROSSING) {
    //                 WRITE_ERROR("Invalid " + toString(SUMO_ATTR_TLLINKINDEX) + " '" + toString(tlLinkIdx) +
    //                             "' in connection controlled by '" + tlID + "'");
    //                 return;
    //             }
    //             if (!ok) {
    //                 return;
    //             }
    //         }
    //         double length;
    //         // build the link
    //         MSLane* via = nullptr;
    //         if (viaID != "" && MSGlobals::gUsingInternalLanes) {
    //             via = MSLane::dictionary(viaID);
    //             if (via == nullptr) {
    //                 WRITE_ERROR("An unknown lane ('" + viaID +
    //                             "') should be set as a via-lane for lane '" + toLane->getID() + "'.");
    //                 return;
    //             }
    //             length = via->getLength();
    //         } else if (toLane->isCrossing()) {
    //             length = toLane->getLength();
    //         } else {
    //             length = fromLane->getShape()[-1].distanceTo(toLane->getShape()[0]);
    //         }
    //         myCurrentLink = new MSLink(fromLane, toLane, via, dir, state, length, foeVisibilityDistance, keepClear, logic, tlLinkIdx, indirect);
    //         if (via != nullptr) {
    //             via->addIncomingLane(fromLane, myCurrentLink);
    //         } else {
    //             toLane->addIncomingLane(fromLane, myCurrentLink);
    //         }
    //         toLane->addApproachingLane(fromLane, myNetworkVersion < MMVersion(0, 25));
    //
    //         // if a traffic light is responsible for it, inform the traffic light
    //         // check whether this link is controlled by a traffic light
    //         // we can not reuse logic here because it might be an inactive one
    //         if (tlID != "") {
    //             myJunctionControlBuilder.getTLLogic(tlID).addLink(myCurrentLink, fromLane, tlLinkIdx);
    //         }
    //         // add the link
    //         fromLane->addLink(myCurrentLink);
    //
    //     } catch (InvalidArgument& e) {
    //         WRITE_ERROR(e.what());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_connection(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let fromID = attrs.get::<String>(SUMO_ATTR_FROM, "", &mut ok, true);
        let toID = attrs.get::<String>(SUMO_ATTR_TO, "", &mut ok, true);
        if !using_internal_lanes() && (fromID.starts_with(':') || toID.starts_with(':')) {
            let tlID = attrs.get_opt::<String>(SUMO_ATTR_TLID, "", &mut ok, String::new(), true);
            if tlID != "" {
                let tlLinkIdx = attrs.get::<i32>(SUMO_ATTR_TLLINKINDEX, "", &mut ok, true);
                self.myJunctionControlBuilder.m().get_tl_logic(&tlID).ignore_link_index(tlLinkIdx);
            }
            return;
        }

        self.myCurrentLink = Ptr::null();
        let result = try_catch_args(|| {
            let fromLaneIdx = attrs.get::<i32>(SUMO_ATTR_FROM_LANE, "", &mut ok, true);
            let toLaneIdx = attrs.get::<i32>(SUMO_ATTR_TO_LANE, "", &mut ok, true);
            let dirS = attrs.get::<String>(SUMO_ATTR_DIR, "", &mut ok, true);
            let dir = self.parse_link_dir(&dirS);
            let stateS = attrs.get::<String>(SUMO_ATTR_STATE, "", &mut ok, true);
            let state = self.parse_link_state(&stateS);
            let foeVisibilityDistance = attrs.get_opt::<f64>(SUMO_ATTR_VISIBILITY_DISTANCE, "", &mut ok, if state == LINKSTATE_ZIPPER { 100.0 } else { 4.5 }, true);
            let keepClear = attrs.get_opt::<bool>(SUMO_ATTR_KEEP_CLEAR, "", &mut ok, true, true);
            let indirect = attrs.get_opt::<bool>(SUMO_ATTR_INDIRECT, "", &mut ok, false, true);
            let tlID = attrs.get_opt::<String>(SUMO_ATTR_TLID, "", &mut ok, String::new(), true);
            let viaID = attrs.get_opt::<String>(SUMO_ATTR_VIA, "", &mut ok, String::new(), true);

            let from = MSEdge::dictionary_hint(&fromID, self.myPreviousEdgeIdx);
            if from.is_null() {
                write_error(&format!("Unknown from-edge '{}' in connection.", fromID));
                return;
            }
            self.myPreviousEdgeIdx = from.get_numerical_id();
            let to = MSEdge::dictionary_2(&toID);
            if to.is_null() {
                write_error(&format!("Unknown to-edge '{}' in connection.", toID));
                return;
            }
            if fromLaneIdx < 0 || fromLaneIdx as usize >= from.get_lanes().len() || toLaneIdx < 0 || toLaneIdx as usize >= to.get_lanes().len() {
                write_error(&format!("Invalid lane index in connection from '{}' to '{}'.", from.get_id(), to.get_id()));
                return;
            }
            let fromLane = from.get_lanes()[fromLaneIdx as usize];
            let toLane = to.get_lanes()[toLaneIdx as usize];

            let mut logic: Ptr<MSTrafficLightLogic> = Ptr::null();
            let mut tlLinkIdx: i32 = -1;
            if tlID != "" {
                tlLinkIdx = attrs.get::<i32>(SUMO_ATTR_TLLINKINDEX, "", &mut ok, true);
                // make sure that the index is in range
                logic = self.myJunctionControlBuilder.m().get_tl_logic(&tlID).get_active();
                let stateLen = logic.get_current_phase_def().get_state().len() as i32;
                if (tlLinkIdx < 0 || tlLinkIdx >= stateLen) && logic.get_logic_type() != TrafficLightType::RAIL_SIGNAL && logic.get_logic_type() != TrafficLightType::RAIL_CROSSING {
                    write_error(&format!("Invalid {} '{}' in connection controlled by '{}'", attr_to_string(SUMO_ATTR_TLLINKINDEX), tlLinkIdx, tlID));
                    return;
                }
                if !ok {
                    return;
                }
            }

            let length: f64;
            // build the link
            let mut via: Ptr<MSLane> = Ptr::null();
            if viaID != "" && using_internal_lanes() {
                via = MSLane::dictionary_2(&viaID);
                if via.is_null() {
                    write_error(&format!("An unknown lane ('{}') should be set as a via-lane for lane '{}'.", viaID, toLane.get_id()));
                    return;
                }
                length = via.get_length();
            } else if toLane.is_crossing() {
                length = toLane.get_length();
            } else {
                length = fromLane.get_shape(false).at(-1).distance_to(toLane.get_shape(false).at(0));
            }
            self.myCurrentLink = Ptr::from_raw(Box::into_raw(Box::new(MSLink::new(fromLane, toLane, via, dir, state, length, foeVisibilityDistance, keepClear, logic, tlLinkIdx, indirect))));
            if !via.is_null() {
                via.m().add_incoming_lane(fromLane, self.myCurrentLink);
            } else {
                toLane.m().add_incoming_lane(fromLane, self.myCurrentLink);
            }
            toLane.m().add_approaching_lane(fromLane, self.myNetworkVersion < (0, 25.0));

            // if a traffic light is responsible for it, inform the traffic light
            // check whether this link is controlled by a traffic light
            // we can not reuse logic here because it might be an inactive one
            if tlID != "" {
                self.myJunctionControlBuilder.m().get_tl_logic(&tlID).add_link(self.myCurrentLink, fromLane, tlLinkIdx);
            }
            // add the link
            fromLane.m().add_link(self.myCurrentLink);
        });
        if let Err(what) = result {
            write_error(&what);
        }
    }

    /// C++: `void NLHandler::addConflict(const SUMOSAXAttributes &)` [private]
    // void
    // NLHandler::addConflict(const SUMOSAXAttributes& attrs) {
    //     if (myCurrentLink == nullptr) {
    //         throw InvalidArgument(toString(SUMO_TAG_CONFLICT) + " must occur within a " + toString(SUMO_TAG_CONNECTION) + " element");
    //     }
    //     if (!MSGlobals::gUsingInternalLanes) {
    //         return;
    //     }
    //     bool ok = true;
    //     const std::string fromID = attrs.get<std::string>(SUMO_ATTR_FROM, nullptr, ok);
    //     const std::string toID = attrs.get<std::string>(SUMO_ATTR_TO, nullptr, ok);
    //     const int fromLaneIdx = attrs.get<int>(SUMO_ATTR_FROM_LANE, nullptr, ok);
    //     const int toLaneIdx = attrs.get<int>(SUMO_ATTR_TO_LANE, nullptr, ok);
    //     double startPos = attrs.get<double>(SUMO_ATTR_STARTPOS, nullptr, ok);
    //     double endPos = attrs.get<double>(SUMO_ATTR_ENDPOS, nullptr, ok);
    //     MSEdge* from = MSEdge::dictionary(fromID);
    //     if (from == nullptr) {
    //         WRITE_ERRORF(TL("Unknown from-edge '%' in conflict."), fromID);
    //         return;
    //     }
    //     MSEdge* to = MSEdge::dictionary(toID);
    //     if (to == nullptr) {
    //         WRITE_ERRORF(TL("Unknown to-edge '%' in conflict."), toID);
    //         return;
    //     }
    //     if (fromLaneIdx < 0 || fromLaneIdx >= (int)from->getLanes().size() ||
    //             toLaneIdx < 0 || toLaneIdx >= (int)to->getLanes().size()) {
    //         WRITE_ERRORF(TL("Invalid lane index in conflict with '%' to '%'."), from->getID(), to->getID());
    //         return;
    //     }
    //     MSLane* fromLane = from->getLanes()[fromLaneIdx];
    //     MSLane* toLane = to->getLanes()[toLaneIdx];
    //     assert(fromLane);
    //     assert(toLane);
    //     myCurrentLink->addCustomConflict(fromLane, toLane, startPos, endPos);
    // }
    #[allow(unused_variables)]
    pub fn add_conflict(&mut self, attrs: &SUMOSAXAttributes) {
        if self.myCurrentLink.is_null() {
            throw_invalid_argument(format!("{} must occur within a {} element", tag_to_string(SUMO_TAG_CONFLICT), tag_to_string(SUMO_TAG_CONNECTION)));
        }
        if !using_internal_lanes() {
            return;
        }
        let mut ok = true;
        let fromID = attrs.get::<String>(SUMO_ATTR_FROM, "", &mut ok, true);
        let toID = attrs.get::<String>(SUMO_ATTR_TO, "", &mut ok, true);
        let fromLaneIdx = attrs.get::<i32>(SUMO_ATTR_FROM_LANE, "", &mut ok, true);
        let toLaneIdx = attrs.get::<i32>(SUMO_ATTR_TO_LANE, "", &mut ok, true);
        let startPos = attrs.get::<f64>(SUMO_ATTR_STARTPOS, "", &mut ok, true);
        let endPos = attrs.get::<f64>(SUMO_ATTR_ENDPOS, "", &mut ok, true);
        let from = MSEdge::dictionary_2(&fromID);
        if from.is_null() {
            write_error(&format!("Unknown from-edge '{}' in conflict.", fromID));
            return;
        }
        let to = MSEdge::dictionary_2(&toID);
        if to.is_null() {
            write_error(&format!("Unknown to-edge '{}' in conflict.", toID));
            return;
        }
        if fromLaneIdx < 0 || fromLaneIdx as usize >= from.get_lanes().len() || toLaneIdx < 0 || toLaneIdx as usize >= to.get_lanes().len() {
            write_error(&format!("Invalid lane index in conflict with '{}' to '{}'.", from.get_id(), to.get_id()));
            return;
        }
        let fromLane = from.get_lanes()[fromLaneIdx as usize];
        let toLane = to.get_lanes()[toLaneIdx as usize];
        self.myCurrentLink.m().add_custom_conflict(fromLane, toLane, startPos, endPos);
    }

    /// C++: `void NLHandler::openWAUT(const SUMOSAXAttributes &)` [virtual, private]
    // void
    // NLHandler::openWAUT(const SUMOSAXAttributes& attrs) {
    //     myCurrentIsBroken = false;
    //     bool ok = true;
    //     // get the id, report an error if not given or empty...
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     SUMOTime refTime = attrs.getOptSUMOTimeReporting(SUMO_ATTR_REF_TIME, id.c_str(), ok, 0);
    //     SUMOTime period = attrs.getOptSUMOTimeReporting(SUMO_ATTR_PERIOD, id.c_str(), ok, 0);
    //     std::string startProg = attrs.get<std::string>(SUMO_ATTR_START_PROG, id.c_str(), ok);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //     }
    //     if (!myCurrentIsBroken) {
    //         myCurrentWAUTID = id;
    //         try {
    //             myJunctionControlBuilder.getTLLogicControlToUse().addWAUT(refTime, id, startProg, period);
    //         } catch (InvalidArgument& e) {
    //             WRITE_ERROR(e.what());
    //             myCurrentIsBroken = true;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn open_waut(&mut self, attrs: &SUMOSAXAttributes) {
        self.myCurrentIsBroken = false;
        let mut ok = true;
        // get the id, report an error if not given or empty...
        let id = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }
        let refTime = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_REF_TIME, &id, &mut ok, 0, true);
        let period = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_PERIOD, &id, &mut ok, 0, true);
        let startProg = attrs.get::<String>(SUMO_ATTR_START_PROG, &id, &mut ok, true);
        if !ok {
            self.myCurrentIsBroken = true;
        }
        if !self.myCurrentIsBroken {
            self.myCurrentWAUTID = id.clone();
            let result = try_catch_args(|| {
                self.myJunctionControlBuilder.m().get_tl_logic_control_to_use().add_waut(refTime, &id, &startProg, period);
            });
            if let Err(what) = result {
                write_error(&what);
                self.myCurrentIsBroken = true;
            }
        }
    }

    /// C++: `void NLHandler::addWAUTSwitch(const SUMOSAXAttributes &)` [private]
    // void
    // NLHandler::addWAUTSwitch(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     SUMOTime t = attrs.getSUMOTimeReporting(SUMO_ATTR_TIME, myCurrentWAUTID.c_str(), ok);
    //     std::string to = attrs.get<std::string>(SUMO_ATTR_TO, myCurrentWAUTID.c_str(), ok);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //     }
    //     if (!myCurrentIsBroken) {
    //         try {
    //             myJunctionControlBuilder.getTLLogicControlToUse().addWAUTSwitch(myCurrentWAUTID, t, to);
    //         } catch (InvalidArgument& e) {
    //             WRITE_ERROR(e.what());
    //             myCurrentIsBroken = true;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_waut_switch(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let t = attrs.get_sumo_time_reporting(SUMO_ATTR_TIME, &self.myCurrentWAUTID, &mut ok, true);
        let to = attrs.get::<String>(SUMO_ATTR_TO, &self.myCurrentWAUTID, &mut ok, true);
        if !ok {
            self.myCurrentIsBroken = true;
        }
        if !self.myCurrentIsBroken {
            let result = try_catch_args(|| {
                self.myJunctionControlBuilder.m().get_tl_logic_control_to_use().add_waut_switch(&self.myCurrentWAUTID, t, &to);
            });
            if let Err(what) = result {
                write_error(&what);
                self.myCurrentIsBroken = true;
            }
        }
    }

    /// C++: `void NLHandler::addWAUTJunction(const SUMOSAXAttributes &)` [private]
    // void
    // NLHandler::addWAUTJunction(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     std::string wautID = attrs.get<std::string>(SUMO_ATTR_WAUT_ID, nullptr, ok);
    //     std::string junctionID = attrs.get<std::string>(SUMO_ATTR_JUNCTION_ID, nullptr, ok);
    //     std::string procedure = attrs.getOpt<std::string>(SUMO_ATTR_PROCEDURE, nullptr, ok, "");
    //     bool synchron = attrs.getOpt<bool>(SUMO_ATTR_SYNCHRON, nullptr, ok, false);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //     }
    //     try {
    //         if (!myCurrentIsBroken) {
    //             myJunctionControlBuilder.getTLLogicControlToUse().addWAUTJunction(wautID, junctionID, procedure, synchron);
    //         }
    //     } catch (InvalidArgument& e) {
    //         WRITE_ERROR(e.what());
    //         myCurrentIsBroken = true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_waut_junction(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let wautID = attrs.get::<String>(SUMO_ATTR_WAUT_ID, "", &mut ok, true);
        let junctionID = attrs.get::<String>(SUMO_ATTR_JUNCTION_ID, "", &mut ok, true);
        let procedure = attrs.get_opt::<String>(SUMO_ATTR_PROCEDURE, "", &mut ok, String::new(), true);
        let synchron = attrs.get_opt::<bool>(SUMO_ATTR_SYNCHRON, "", &mut ok, false, true);
        if !ok {
            self.myCurrentIsBroken = true;
        }
        let result = try_catch_args(|| {
            if !self.myCurrentIsBroken {
                self.myJunctionControlBuilder.m().get_tl_logic_control_to_use().add_waut_junction(&wautID, &junctionID, &procedure, synchron);
            }
        });
        if let Err(what) = result {
            write_error(&what);
            self.myCurrentIsBroken = true;
        }
    }

    /// C++: `void NLHandler::setLocation(const SUMOSAXAttributes &)` [private]
    /// Parses network location description
    // void
    // NLHandler::setLocation(const SUMOSAXAttributes& attrs) {
    //     if (myNetIsLoaded) {
    //         //WRITE_WARNING(TL("POIs and Polygons should be loaded using option --po-files"))
    //         return;
    //     }
    //     bool ok = true;
    //     PositionVector s = attrs.get<PositionVector>(SUMO_ATTR_NET_OFFSET, nullptr, ok);
    //     Boundary convBoundary = attrs.get<Boundary>(SUMO_ATTR_CONV_BOUNDARY, nullptr, ok);
    //     Boundary origBoundary = attrs.get<Boundary>(SUMO_ATTR_ORIG_BOUNDARY, nullptr, ok);
    //     std::string proj = attrs.get<std::string>(SUMO_ATTR_ORIG_PROJ, nullptr, ok);
    //     if (ok) {
    //         Position networkOffset = s[0];
    //         GeoConvHelper::init(proj, networkOffset, origBoundary, convBoundary);
    //         if (OptionsCont::getOptions().getBool("fcd-output.geo") && !GeoConvHelper::getFinal().usingGeoProjection()) {
    //             WRITE_WARNING(TL("no valid geo projection loaded from network. fcd-output.geo will not work"));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_location(&mut self, attrs: &SUMOSAXAttributes) {
        if self.myNetIsLoaded {
            //WRITE_WARNING(TL("POIs and Polygons should be loaded using option --po-files"))
            return;
        }
        let mut ok = true;
        let s = attrs.get::<PositionVector>(SUMO_ATTR_NET_OFFSET, "", &mut ok, true);
        let convBoundary = attrs.get::<Boundary>(SUMO_ATTR_CONV_BOUNDARY, "", &mut ok, true);
        let origBoundary = attrs.get::<Boundary>(SUMO_ATTR_ORIG_BOUNDARY, "", &mut ok, true);
        let proj = attrs.get::<String>(SUMO_ATTR_ORIG_PROJ, "", &mut ok, true);
        if ok {
            let networkOffset = *s.at(0);
            GeoConvHelper::init(&proj, networkOffset, origBoundary, convBoundary, 1.0);
            if utils::options::options_cont::get_bool("fcd-output.geo") && !GeoConvHelper::with_final(|g| g.using_geo_projection()) {
                write_warning("no valid geo projection loaded from network. fcd-output.geo will not work");
            }
        }
    }

    /// C++: `void NLHandler::addDistrict(const SUMOSAXAttributes &)` [private]
    /// Parses a district and creates a pseudo edge for it
    // void
    // NLHandler::addDistrict(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     myCurrentIsBroken = false;
    //     // get the id, report an error if not given or empty...
    //     myCurrentDistrictID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (!ok) {
    //         myCurrentIsBroken = true;
    //         return;
    //     }
    //     try {
    //         const std::string sinkID = myCurrentDistrictID + "-sink";
    //         const std::string sourceID = myCurrentDistrictID + "-source";
    //
    //         MSEdge* sink = MSEdge::dictionary(sinkID);
    //         if (sink == nullptr) {
    //             sink = myEdgeControlBuilder.buildEdge(sinkID, SumoXMLEdgeFunc::CONNECTOR, "", "", "", -1, 0);
    //             MSEdge::dictionary(sinkID, sink);
    //             sink->initialize(new std::vector<MSLane*>());
    //         } else {
    //             if (OptionsCont::getOptions().getBool("junction-taz")
    //                     && myNet.getJunctionControl().get(myCurrentDistrictID) != nullptr) {
    //                 // overwrite junction taz
    //                 sink->resetTAZ(myNet.getJunctionControl().get(myCurrentDistrictID));
    //                 WRITE_WARNINGF(TL("Replacing junction-taz '%' with loaded TAZ."), myCurrentDistrictID);
    //             } else {
    //                 throw InvalidArgument("Another edge with the id '" + sinkID + "' exists.");
    //             }
    //         }
    //         MSEdge* source = MSEdge::dictionary(sourceID);
    //         if (source == nullptr) {
    //             source = myEdgeControlBuilder.buildEdge(sourceID, SumoXMLEdgeFunc::CONNECTOR, "", "", "", -1, 0);
    //             MSEdge::dictionary(sourceID, source);
    //             source->initialize(new std::vector<MSLane*>());
    //         } else {
    //             if (OptionsCont::getOptions().getBool("junction-taz")
    //                     && myNet.getJunctionControl().get(myCurrentDistrictID) != nullptr) {
    //                 // overwrite junction taz
    //                 source->resetTAZ(myNet.getJunctionControl().get(myCurrentDistrictID));
    //             } else {
    //                 throw InvalidArgument("Another edge with the id '" + sourceID + "' exists.");
    //             }
    //         }
    //         sink->setOtherTazConnector(source);
    //         source->setOtherTazConnector(sink);
    //         const std::vector<std::string>& desc = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_EDGES, myCurrentDistrictID.c_str(), ok);
    //         for (const std::string& eID : desc) {
    //             MSEdge* edge = MSEdge::dictionary(eID);
    //             // check whether the edge exists
    //             if (edge == nullptr) {
    //                 throw InvalidArgument("The edge '" + eID + "' within district '" + myCurrentDistrictID + "' is not known.");
    //             }
    //             source->addSuccessor(edge);
    //             edge->addSuccessor(sink);
    //         }
    //         source->setParameter("taz", myCurrentDistrictID);
    //         sink->setParameter("taz", myCurrentDistrictID);
    //         RGBColor color = attrs.getOpt<RGBColor>(SUMO_ATTR_COLOR, myCurrentDistrictID.c_str(), ok, RGBColor::parseColor("1.0,.33,.33"));
    //         const std::string name = attrs.getOpt<std::string>(SUMO_ATTR_NAME, myCurrentDistrictID.c_str(), ok, "");
    //         source->setParameter("tazColor", toString(color));
    //         sink->setParameter("tazColor", toString(color));
    //
    //         if (attrs.hasAttribute(SUMO_ATTR_SHAPE)) {
    //             PositionVector shape = attrs.get<PositionVector>(SUMO_ATTR_SHAPE, myCurrentDistrictID.c_str(), ok);
    //             const bool fill = attrs.getOpt<bool>(SUMO_ATTR_FILL, myCurrentDistrictID.c_str(), ok, false);
    //             if (shape.size() != 0) {
    //                 if (!myNet.getShapeContainer().addPolygon(myCurrentDistrictID, "taz", color, 0, 0, "", shape, false, fill, 1.0, false, name)) {
    //                     WRITE_WARNINGF(TL("Skipping visualization of taz '%', polygon already exists."), myCurrentDistrictID);
    //                 } else {
    //                     myLastParameterised.push_back(myNet.getShapeContainer().getPolygons().get(myCurrentDistrictID));
    //                     myCurrentIsBroken = false;
    //                 }
    //             }
    //         }
    //     } catch (InvalidArgument& e) {
    //         WRITE_ERROR(e.what());
    //         myCurrentIsBroken = true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_district(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        self.myCurrentIsBroken = false;
        // get the id, report an error if not given or empty...
        self.myCurrentDistrictID = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
        if !ok {
            self.myCurrentIsBroken = true;
            return;
        }
        let id = self.myCurrentDistrictID.clone();
        let result = try_catch_args(|| {
            let sinkID = format!("{}-sink", id);
            let sourceID = format!("{}-source", id);
            let junctionTaz = utils::options::options_cont::get_bool("junction-taz");

            let mut sink = MSEdge::dictionary_2(&sinkID);
            if sink.is_null() {
                sink = self.myEdgeControlBuilder.m().build_edge(&sinkID, SumoXMLEdgeFunc::CONNECTOR, "", "", "", -1, 0.0);
                MSEdge::dictionary(&sinkID, sink);
                sink.m().initialize(Ptr::from_raw(Box::into_raw(Box::new(Vec::new()))));
            } else {
                let junction = self.myNet.m().get_junction_control().get(&id).copied().unwrap_or(Ptr::null());
                if junctionTaz && !junction.is_null() {
                    // overwrite junction taz
                    sink.m().reset_taz(junction);
                    write_warning(&format!("Replacing junction-taz '{}' with loaded TAZ.", id));
                } else {
                    throw_invalid_argument(format!("Another edge with the id '{}' exists.", sinkID));
                }
            }
            let mut source = MSEdge::dictionary_2(&sourceID);
            if source.is_null() {
                source = self.myEdgeControlBuilder.m().build_edge(&sourceID, SumoXMLEdgeFunc::CONNECTOR, "", "", "", -1, 0.0);
                MSEdge::dictionary(&sourceID, source);
                source.m().initialize(Ptr::from_raw(Box::into_raw(Box::new(Vec::new()))));
            } else {
                let junction = self.myNet.m().get_junction_control().get(&id).copied().unwrap_or(Ptr::null());
                if junctionTaz && !junction.is_null() {
                    // overwrite junction taz
                    source.m().reset_taz(junction);
                } else {
                    throw_invalid_argument(format!("Another edge with the id '{}' exists.", sourceID));
                }
            }
            sink.m().set_other_taz_connector(source);
            source.m().set_other_taz_connector(sink);
            let desc = attrs.get_opt::<Vec<String>>(SUMO_ATTR_EDGES, &id, &mut ok, Vec::new(), true);
            for eID in &desc {
                let edge = MSEdge::dictionary_2(eID);
                // check whether the edge exists
                if edge.is_null() {
                    throw_invalid_argument(format!("The edge '{}' within district '{}' is not known.", eID, id));
                }
                source.m().add_successor(edge, Ptr::null());
                edge.m().add_successor(sink, Ptr::null());
            }
            source.m().base2.set_parameter("taz", &id);
            sink.m().base2.set_parameter("taz", &id);
            // shortcut: getOpt<RGBColor> is not in the SUMOSAXAttributes FromAttr set, so the color is parsed directly; a bad color is not reported as a broken district
            let color = if attrs.has_attribute(SUMO_ATTR_COLOR) {
                unwrap_or_throw(utils::common::rgb_color::RGBColor::parse_color(&attrs.get_string(SUMO_ATTR_COLOR)))
            } else {
                unwrap_or_throw(utils::common::rgb_color::RGBColor::parse_color("1.0,.33,.33"))
            };
            source.m().base2.set_parameter("tazColor", &color.to_string());
            sink.m().base2.set_parameter("tazColor", &color.to_string());

            let name = attrs.get_opt::<String>(SUMO_ATTR_NAME, &id, &mut ok, String::new(), true);
            if attrs.has_attribute(SUMO_ATTR_SHAPE) {
                let shape = attrs.get::<PositionVector>(SUMO_ATTR_SHAPE, &id, &mut ok, true);
                let fill = attrs.get_opt::<bool>(SUMO_ATTR_FILL, &id, &mut ok, false, true);
                if shape.size() != 0 {
                    if !self.myNet.m().get_shape_container().add_polygon(&id, "taz", &color, 0.0, 0.0, "", &shape, false, fill, 1.0, false, &name) {
                        write_warning(&format!("Skipping visualization of taz '{}', polygon already exists.", id));
                    } else {
                        let poly = self.myNet.m().get_shape_container().get_polygons().get(&id).copied().unwrap_or(Ptr::null());
                        self.myLastParameterised.push(Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*poly.as_raw()).base2) }));
                        self.myCurrentIsBroken = false;
                    }
                }
            }
        });
        if let Err(what) = result {
            write_error(&what);
            self.myCurrentIsBroken = true;
        }
    }

    /// C++: `void NLHandler::addDistrictEdge(const SUMOSAXAttributes &, bool)` [private]
    /// Parses a district edge and connects it to the district
    // void
    // NLHandler::addDistrictEdge(const SUMOSAXAttributes& attrs, bool isSource) {
    //     if (myCurrentIsBroken) {
    //         // earlier error
    //         return;
    //     }
    //     bool ok = true;
    //     std::string id = attrs.get<std::string>(SUMO_ATTR_ID, myCurrentDistrictID.c_str(), ok);
    //     MSEdge* succ = MSEdge::dictionary(id);
    //     if (succ != nullptr) {
    //         // connect edge
    //         if (isSource) {
    //             MSEdge::dictionary(myCurrentDistrictID + "-source")->addSuccessor(succ);
    //         } else {
    //             succ->addSuccessor(MSEdge::dictionary(myCurrentDistrictID + "-sink"));
    //         }
    //     } else {
    //         WRITE_ERRORF(TL("At district '%': succeeding edge '%' does not exist."), myCurrentDistrictID, id);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_district_edge(&mut self, attrs: &SUMOSAXAttributes, isSource: bool) {
        if self.myCurrentIsBroken {
            // earlier error
            return;
        }
        let mut ok = true;
        let cur = self.myCurrentDistrictID.clone();
        let id = attrs.get::<String>(SUMO_ATTR_ID, &cur, &mut ok, true);
        let succ = MSEdge::dictionary_2(&id);
        if !succ.is_null() {
            // connect edge
            if isSource {
                MSEdge::dictionary_2(&format!("{}-source", cur)).m().add_successor(succ, Ptr::null());
            } else {
                succ.m().add_successor(MSEdge::dictionary_2(&format!("{}-sink", cur)), Ptr::null());
            }
        } else {
            write_error(&format!("At district '{}': succeeding edge '{}' does not exist.", cur, id));
        }
    }

    /// C++: `void NLHandler::addRoundabout(const SUMOSAXAttributes &)` [private]
    /// Parses a roundabout and sets flags for the edges
    // void
    // NLHandler::addRoundabout(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::vector<std::string>& edgeIDs = attrs.get<std::vector<std::string> >(SUMO_ATTR_EDGES, nullptr, ok);
    //     if (ok) {
    //         for (const std::string& eID : edgeIDs) {
    //             MSEdge* edge = MSEdge::dictionary(eID);
    //             if (edge == nullptr) {
    //                 WRITE_ERRORF(TL("Unknown edge '%' in roundabout"), eID);
    //             } else {
    //                 edge->markAsRoundabout();
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_roundabout(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let edgeIDs = attrs.get::<Vec<String>>(SUMO_ATTR_EDGES, "", &mut ok, true);
        if ok {
            for eID in &edgeIDs {
                let edge = MSEdge::dictionary_2(eID);
                if edge.is_null() {
                    write_error(&format!("Unknown edge '{}' in roundabout", eID));
                } else {
                    edge.m().mark_as_roundabout();
                }
            }
        }
    }

    /// C++: `void NLHandler::closeWAUT()` [private]
    // void
    // NLHandler::closeWAUT() {
    //     if (!myCurrentIsBroken) {
    //         try {
    //             myJunctionControlBuilder.getTLLogicControlToUse().closeWAUT(myCurrentWAUTID);
    //         } catch (InvalidArgument& e) {
    //             WRITE_ERROR(e.what());
    //             myCurrentIsBroken = true;
    //         }
    //     }
    //     myCurrentWAUTID = "";
    // }
    #[allow(unused_variables)]
    pub fn close_waut(&mut self) {
        if !self.myCurrentIsBroken {
            let wautID = self.myCurrentWAUTID.clone();
            let result = try_catch_args(|| self.myJunctionControlBuilder.m().get_tl_logic_control_to_use().close_waut(&wautID));
            if let Err(what) = result {
                write_error(&what);
                self.myCurrentIsBroken = true;
            }
        }
        self.myCurrentWAUTID = String::new();
    }

    /// C++: `LinkDirection NLHandler::parseLinkDir(const std::string &)` [private]
    /// Parses the given character into an enumeration typed link direction
    // LinkDirection
    // NLHandler::parseLinkDir(const std::string& dir) {
    //     if (SUMOXMLDefinitions::LinkDirections.hasString(dir)) {
    //         return SUMOXMLDefinitions::LinkDirections.get(dir);
    //     } else {
    //         throw InvalidArgument("Unrecognised link direction '" + dir + "'.");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_link_dir(&mut self, dir: &str) -> LinkDirection {
        if LinkDirections.has_string(dir) {
            unwrap_or_throw(LinkDirections.get(dir))
        } else {
            throw_invalid_argument(format!("Unrecognised link direction '{}'.", dir));
        }
    }

    /// C++: `LinkState NLHandler::parseLinkState(const std::string &)` [private]
    /// Parses the given character into an enumeration typed link state
    // LinkState
    // NLHandler::parseLinkState(const std::string& state) {
    //     if (SUMOXMLDefinitions::LinkStates.hasString(state)) {
    //         return SUMOXMLDefinitions::LinkStates.get(state);
    //     } else {
    //         if (state == "t") { // legacy networks
    //             // WRITE_WARNING(TL("Obsolete link state 't'. Use 'o' instead"));
    //             return LINKSTATE_TL_OFF_BLINKING;
    //         } else {
    //             throw InvalidArgument("Unrecognised link state '" + state + "'.");
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn parse_link_state(&mut self, state: &str) -> LinkState {
        if LinkStates.has_string(state) {
            unwrap_or_throw(LinkStates.get(state))
        } else if state == "t" {
            // legacy networks
            // WRITE_WARNING(TL("Obsolete link state 't'. Use 'o' instead"));
            LINKSTATE_TL_OFF_BLINKING
        } else {
            throw_invalid_argument(format!("Unrecognised link state '{}'.", state));
        }
    }

    /// C++: `int NLHandler::parseDetectPersons(const std::string &, const std::string &, bool &)` [private]
    // int
    // NLHandler::parseDetectPersons(const std::string& detectPersonsString, const std::string& id, bool& ok) {
    //     int detectPersons = 0;
    //     for (std::string mode : StringTokenizer(detectPersonsString).getVector()) {
    //         if (SUMOXMLDefinitions::PersonModeValues.hasString(mode)) {
    //             detectPersons |= (int)SUMOXMLDefinitions::PersonModeValues.get(mode);
    //         } else {
    //             WRITE_ERRORF(TL("Invalid person mode '%' in E1 detector definition '%'"), mode, id);
    //             ok = false;
    //             return 0;
    //         }
    //     }
    //     return detectPersons;
    // }
    #[allow(unused_variables)]
    pub fn parse_detect_persons(&mut self, detectPersonsString: &str, id: &str, ok: &mut bool) -> i32 {
        let mut detectPersons: i32 = 0;
        for mode in StringTokenizer::new(detectPersonsString).get_vector() {
            if PersonModeValues.has_string(&mode) {
                detectPersons |= unwrap_or_throw(PersonModeValues.get(&mode)) as i32;
            } else {
                write_error(&format!("Invalid person mode '{}' in E1 detector definition '{}'", mode, id));
                *ok = false;
                return 0;
            }
        }
        detectPersons
    }

    /// C++: `void NLHandler::NLHandler(const NLHandler &)` [private]
    /// invalid copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &NLHandler) -> Self {
        // C++ declares the copy constructor private and never defines it
        std::panic::panic_any(ProcessError::new("NLHandler copy constructor is private and undefined in C++"))
    }

    /// C++: `NLHandler & NLHandler::operator=(const NLHandler &)` [private]
    /// invalid assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &NLHandler) -> &mut NLHandler {
        // C++ declares the assignment operator private and never defines it
        std::panic::panic_any(ProcessError::new("NLHandler assignment operator is private and undefined in C++"))
    }

}

/// The XML-Handler for shapes loading network loading
/// C++ class `NLShapeHandler` (netload/NLHandler.h)
#[repr(C)]
pub struct NLShapeHandler {
    /// C++ base class `ShapeHandler`
    pub base: ShapeHandler,
}

impl std::ops::Deref for NLShapeHandler {
    type Target = ShapeHandler;
    fn deref(&self) -> &ShapeHandler {
        &self.base
    }
}
impl std::ops::DerefMut for NLShapeHandler {
    fn deref_mut(&mut self) -> &mut ShapeHandler {
        &mut self.base
    }
}

impl NLShapeHandler {
    /// C++: `void NLShapeHandler::NLShapeHandler(const std::string &, ShapeContainer &)`
    // NLShapeHandler(const std::string& file, ShapeContainer& sc) :
    //         ShapeHandler(file, sc) {}
    #[allow(unused_variables)]
    pub fn new(file: &str, sc: &mut ShapeContainer) -> Self {
        // C++ default for the GeoConvHelper argument of ShapeHandler is nullptr
        NLShapeHandler { base: ShapeHandler::new(file, sc, Ptr::null()) }
    }

    /// C++: `void NLShapeHandler::~NLShapeHandler() noexcept` [virtual]
    /// Destructor
    // virtual ~NLShapeHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++: `virtual ~NLShapeHandler() {}`; nothing to release beyond the base's fields
    }

    /// C++: `Position NLShapeHandler::getLanePos(const std::string &, const std::string &, double, bool, double)` [virtual]
    // Position
    // NLShapeHandler::getLanePos(const std::string& poiID, const std::string& laneID, double lanePos, bool friendlyPos, double lanePosLat) {
    //     MSLane* lane = MSLane::dictionary(laneID);
    //     if (lane == nullptr) {
    //         WRITE_ERRORF(TL("Lane '%' to place poi '%' on is not known."), laneID, poiID);
    //         return Position::INVALID;
    //     }
    //     if (lanePos < 0) {
    //         lanePos = lane->getLength() + lanePos;
    //     }
    //     if ((lanePos < 0) && friendlyPos) {
    //         lanePos = 0;
    //     }
    //     if ((lanePos > lane->getLength()) && friendlyPos) {
    //         lanePos = lane->getLength();
    //     }
    //     if (lanePos < 0 || lanePos > lane->getLength()) {
    //         WRITE_WARNINGF(TL("lane position % for poi '%' is not valid."), toString(lanePos), poiID);
    //     }
    //     return lane->geometryPositionAtOffset(lanePos, -lanePosLat);
    // }
    #[allow(unused_variables)]
    pub fn get_lane_pos(&mut self, poiID: &str, laneID: &str, lanePos: f64, friendlyPos: bool, lanePosLat: f64) -> Position {
        let lane = MSLane::dictionary_2(laneID);
        if lane.is_null() {
            write_error(&format!("Lane '{}' to place poi '{}' on is not known.", laneID, poiID));
            return Position::INVALID;
        }
        let mut lanePos = lanePos;
        if lanePos < 0.0 {
            lanePos = lane.get_length() + lanePos;
        }
        if (lanePos < 0.0) && friendlyPos {
            lanePos = 0.0;
        }
        if (lanePos > lane.get_length()) && friendlyPos {
            lanePos = lane.get_length();
        }
        if lanePos < 0.0 || lanePos > lane.get_length() {
            write_warning(&format!("lane position {} for poi '{}' is not valid.", utils::common::to_string::f64_to_string_default(lanePos), poiID));
        }
        lane.geometry_position_at_offset(lanePos, -lanePosLat)
    }

    /// C++: `bool NLShapeHandler::addLanePosParams()` [virtual]
    // virtual bool addLanePosParams() {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn add_lane_pos_params(&mut self) -> bool {
        true
    }

}

