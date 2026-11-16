//! Port of microsim/MSStateHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use utils::xml::generic_sax_handler::GenericSAXHandler;
use crate::prelude::*;
use utils::common::std_defs::{g_precision, set_g_precision};

/// version written into saved states (C++ VERSION_STRING)
// shortcut: VERSION_STRING is not a shared constant in utils yet, keep in sync with crates/sumo/src/main.rs VERSION_STRING
const STATE_VERSION_STRING: &str = "v1.28.0-0000000000";

/// keep a copy of the attributes for later use (C++ `attrs.clone()`); the C++ owner deletes it, here it is leaked
// shortcut: cached attribute clones are leaked, upgrade when the owner holds a typed Box
fn leak_attrs(attrs: &SUMOSAXAttributes) -> Ptr<SUMOSAXAttributes> {
    Ptr::from_mut(Box::leak(Box::new(attrs.clone())))
}

/// Parser and output filter for routes and vehicles state saving and loading
/// C++ class `MSStateHandler` (microsim/MSStateHandler.h)
#[repr(C)]
pub struct MSStateHandler {
    /// C++ base class `MSRouteHandler`
    pub base: MSRouteHandler,
    /// offset
    pub myOffset: SUMOTime,
    /// time
    pub myTime: SUMOTime,
    /// segment
    pub mySegment: Ptr<MESegment>,
    /// current lane being loaded
    pub myCurrentLane: Ptr<MSLane>,
    /// current link being loaded
    pub myCurrentLink: Ptr<MSLink>,
    /// que index
    pub myQueIndex: i32,
    /// cached attrs (used when loading vehicles or transportables)
    pub myAttrs: Ptr<SUMOSAXAttributes>,
    /// cached attrs for delayed loading of MSVehicleControl state
    pub myVCAttrs: Ptr<SUMOSAXAttributes>,
    /// cached device and reminder attrs (used when loading vehicles)
    pub myDeviceAttrs: Vec<Ptr<SUMOSAXAttributes>>,
    pub myReminderAttrs: Vec<Ptr<SUMOSAXAttributes>>,
    pub myCFMAttrs: Ptr<SUMOSAXAttributes>,
    /// the last object that potentially carries parameters
    pub myLastParameterised: Ptr<Parameterised>,
    /// vehicles that shall be removed when loading state
    pub myVehiclesToRemove: BTreeSet<String>,
    /// vehicles that were removed when loading state
    pub myRemoved: i32,
    /// index of the currently parse flowState
    pub myFlowIndex: i32,
    /// rail signal for which constraints are being loaded
    pub myConstrainedSignal: Ptr<MSRailSignal>,
    /// vehicles that have arrived (C++ std::set<SUMOVehicle*>, kept as a duplicate-free Vec)
    pub myArrived: Vec<Ptr<SUMOVehicle>>,
}

impl std::ops::Deref for MSStateHandler {
    type Target = MSRouteHandler;
    fn deref(&self) -> &MSRouteHandler {
        &self.base
    }
}
impl std::ops::DerefMut for MSStateHandler {
    fn deref_mut(&mut self) -> &mut MSRouteHandler {
        &mut self.base
    }
}

// C++ MSStateHandler::closeVehicle overrides MSRouteHandler::closeVehicle, dispatched through SUMORouteHandler::vt
fn state_handler_close_vehicle(d: &mut SUMORouteHandler) {
    Ptr::from_mut(d).cast::<MSStateHandler>().m().close_vehicle()
}

/// C++ MSStateHandler overrides, installed in SUMORouteHandler::vt by MSStateHandler::new
static MS_STATE_HANDLER_VT: crate::utils::vehicle::sumo_route_handler::SUMORouteHandlerVt = crate::utils::vehicle::sumo_route_handler::SUMORouteHandlerVt {
    close_vehicle: state_handler_close_vehicle,
    ..crate::microsim::ms_route_handler::MS_ROUTE_HANDLER_VT
};

impl MSStateHandler {
    /// C++: `void MSStateHandler::MSStateHandler(const std::string &, const SUMOTime)`
    /// standard constructor
    // MSStateHandler::MSStateHandler(const std::string& file, const SUMOTime offset) :
    //     MSRouteHandler(file, true),
    //     myOffset(offset),
    //     mySegment(nullptr),
    //     myCurrentLane(nullptr),
    //     myCurrentLink(nullptr),
    //     myAttrs(nullptr),
    //     myVCAttrs(nullptr),
    //     myCFMAttrs(nullptr),
    //     myLastParameterised(nullptr),
    //     myRemoved(0),
    //     myFlowIndex(-1),
    //     myConstrainedSignal(nullptr) {
    //     myAmLoadingState = true;
    //     const std::vector<std::string> vehIDs = OptionsCont::getOptions().getStringVector("load-state.remove-vehicles");
    //     myVehiclesToRemove.insert(vehIDs.begin(), vehIDs.end());
    //     myAllowInternalRoutes = true;
    // }
    #[allow(unused_variables)]
    pub fn new(file: &str, offset: SUMOTime) -> Self {
        let mut sh = MSStateHandler {
            base: MSRouteHandler::new(file, true),
            myOffset: offset,
            myTime: 0,
            mySegment: Ptr::null(),
            myCurrentLane: Ptr::null(),
            myCurrentLink: Ptr::null(),
            myQueIndex: 0,
            myAttrs: Ptr::null(),
            myVCAttrs: Ptr::null(),
            myDeviceAttrs: Vec::new(),
            myReminderAttrs: Vec::new(),
            myCFMAttrs: Ptr::null(),
            myLastParameterised: Ptr::null(),
            myVehiclesToRemove: utils::options::options_cont::get_string_vector("load-state.remove-vehicles")
                .into_iter()
                .collect(),
            myRemoved: 0,
            myFlowIndex: -1,
            myConstrainedSignal: Ptr::null(),
            myArrived: Vec::new(),
        };
        sh.myAmLoadingState = true;
        sh.myAllowInternalRoutes = true;
        sh.base.base.vt = &MS_STATE_HANDLER_VT;
        sh
    }

    /// C++: `void MSStateHandler::~MSStateHandler()` [virtual]
    /// standard destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body only frees the attribute holders (myDeviceAttrs, myReminderAttrs, myAttrs); not owned here yet
        // shortcut: the attribute clones are leaked (leak_attrs), so nothing is freed here
    }

    /// C++: `void MSStateHandler::saveState(const std::string &, SUMOTime, bool)` [static]
    /// Saves the current state
    // void
    // MSStateHandler::saveState(const std::string& file, SUMOTime step, bool usePrefix) {
    //     OutputDevice& out = OutputDevice::getDevice(file, usePrefix);
    //     const int statePrecision = OptionsCont::getOptions().getInt("save-state.precision");
    //     out.setPrecision(statePrecision);
    //     const int defaultPrecision = gPrecision;
    //     gPrecision = statePrecision;
    //     std::map<SumoXMLAttr, std::string> attrs;
    //     attrs[SUMO_ATTR_VERSION] = VERSION_STRING;
    //     attrs[SUMO_ATTR_TIME] = time2string(step);
    //     attrs[SUMO_ATTR_TYPE] = use_meso_sim() ? "meso" : "micro";
    //     if (use_meso_sim()) {
    //         attrs[SUMO_ATTR_COUNT] = toString(MELoop::getEventCounter());
    //     }
    //     if (OptionsCont::getOptions().getBool("save-state.constraints")) {
    //         attrs[SUMO_ATTR_CONSTRAINTS] = "1";
    //     }
    //     if (MSDriveWay::haveDriveWays()) {
    //         attrs[SUMO_ATTR_RAIL] = "1";
    //     }
    //     out.writeXMLHeader("snapshot", "state_file.xsd", attrs);
    //     if (OptionsCont::getOptions().getBool("save-state.rng")) {
    //         saveRNGs(out);
    //         if (!use_meso_sim()) {
    //             MSNet::getInstance()->getEdgeControl().saveState(out);
    //         }
    //     }
    //     const MSDispatch* dispatcher = MSDevice_Taxi::getDispatchAlgorithm();
    //     if (dispatcher != nullptr) {
    //         // save early to pre-empty initialization from loaded persons
    //         dispatcher->saveState(out, MSDevice_Taxi::getNextDispatchTime());
    //     }
    //     MSRoute::dict_saveState(out);
    //     MSNet::getInstance()->getVehicleControl().saveState(out);
    //     MSNet::getInstance()->getInsertionControl().saveState(out);
    //     if (OptionsCont::getOptions().getBool("save-state.transportables")) {
    //         if (MSNet::getInstance()->hasPersons()) {
    //             out.openTag(SUMO_TAG_TRANSPORTABLES).writeAttr(SUMO_ATTR_TYPE, "person");
    //             MSNet::getInstance()->getPersonControl().saveState(out);
    //             out.closeTag();
    //         }
    //         if (MSNet::getInstance()->hasContainers()) {
    //             out.openTag(SUMO_TAG_TRANSPORTABLES).writeAttr(SUMO_ATTR_TYPE, "container");
    //             MSNet::getInstance()->getContainerControl().saveState(out);
    //             out.closeTag();
    //         }
    //     }
    //     MSVehicleTransfer::getInstance()->saveState(out);
    //     for (MSEdge* const edge : MSEdge::getAllEdges()) {
    //         if (use_meso_sim()) {
    //             for (MESegment* s = meso_net()->getSegmentForEdge(*edge); s != nullptr; s = s->getNextSegment()) {
    //                 s->saveState(out);
    //             }
    //         } else {
    //             for (MSLane* const lane : edge->getLanes()) {
    //                 lane->saveState(out);
    //             }
    //         }
    //     }
    //     MSNet::getInstance()->getTLSControl().saveState(out);
    //     MSRoutingEngine::saveState(out);
    //     out.close();
    //     gPrecision = defaultPrecision;
    // }
    #[allow(unused_variables)]
    pub fn save_state(file: &str, step: SUMOTime, usePrefix: bool) {
        let outRef = OutputDevice::get_device(file, usePrefix).unwrap_or_else(|e| panic!("{e}"));
        let mut out = outRef.borrow_mut();
        let statePrecision = utils::options::options_cont::get_int("save-state.precision");
        out.set_precision(statePrecision);
        let defaultPrecision = g_precision();
        set_g_precision(statePrecision);
        let mut attrs: BTreeMap<SumoXMLAttr, String> = BTreeMap::new();
        attrs.insert(SUMO_ATTR_VERSION, STATE_VERSION_STRING.to_string());
        attrs.insert(SUMO_ATTR_TIME, time2string(step));
        attrs.insert(SUMO_ATTR_TYPE, if use_meso_sim() { "meso".to_string() } else { "micro".to_string() });
        if use_meso_sim() {
            attrs.insert(SUMO_ATTR_COUNT, MELoop::get_event_counter().to_string());
        }
        if utils::options::options_cont::get_bool("save-state.constraints") {
            attrs.insert(SUMO_ATTR_CONSTRAINTS, "1".to_string());
        }
        if MSDriveWay::have_drive_ways() {
            attrs.insert(SUMO_ATTR_RAIL, "1".to_string());
        }
        out.write_xml_header("snapshot", "state_file.xsd", attrs, true);
        if utils::options::options_cont::get_bool("save-state.rng") {
            Self::save_rn_gs(&mut out);
            if !use_meso_sim() {
                MSNet::get_instance().m().get_edge_control().save_state(&mut out);
            }
        }
        let dispatcher = MSDevice_Taxi::get_dispatch_algorithm();
        if !dispatcher.is_null() {
            // save early to pre-empty initialization from loaded persons
            dispatcher.m().save_state(&mut out, MSDevice_Taxi::get_next_dispatch_time());
        }
        MSRoute::dict_save_state(&mut out);
        MSNet::get_instance().m().get_vehicle_control().save_state(&mut out);
        MSNet::get_instance().m().get_insertion_control().save_state(&mut out);
        if utils::options::options_cont::get_bool("save-state.transportables") {
            if MSNet::get_instance().m().has_persons() {
                out.open_tag_id(SUMO_TAG_TRANSPORTABLES).write_attr(SUMO_ATTR_TYPE, "person");
                MSNet::get_instance().m().get_person_control().save_state(&mut out);
                out.close_tag("");
            }
            if MSNet::get_instance().m().has_containers() {
                out.open_tag_id(SUMO_TAG_TRANSPORTABLES).write_attr(SUMO_ATTR_TYPE, "container");
                MSNet::get_instance().m().get_container_control().save_state(&mut out);
                out.close_tag("");
            }
        }
        MSVehicleTransfer::get_instance().m().save_state(&mut out);
        for edge in MSEdge::get_all_edges().iter() {
            if use_meso_sim() {
                // shortcut: meso segment save needs MELoop::getSegmentForEdge with a position and the segment chain, meso save-state is not ported
                panic!("shortcut: meso save-state is not ported");
            } else {
                for lane in edge.get_lanes().iter() {
                    lane.m().save_state(&mut out);
                }
            }
        }
        MSNet::get_instance().m().get_tls_control().save_state(&mut out);
        MSRoutingEngine::save_state(&mut out);
        out.close().unwrap_or_else(|e| panic!("{e}"));
        set_g_precision(defaultPrecision);
    }

    /// C++: `SUMOTime MSStateHandler::getTime() const` [const]
    /// get time
    // SUMOTime getTime() const {
    //         return myTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_time(&self) -> SUMOTime {
        self.myTime
    }

    /// C++: `void MSStateHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // MSStateHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     MSRouteHandler::myStartElement(element, attrs);
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     switch (element) {
    //         case SUMO_TAG_SNAPSHOT: {
    //             myTime = string2time(attrs.getString(SUMO_ATTR_TIME));
    //             const std::string& version = attrs.getString(SUMO_ATTR_VERSION);
    //             if (version != VERSION_STRING) {
    //                 WRITE_WARNINGF(TL("State was written with sumo version % (present: %)!"), version, VERSION_STRING);
    //             }
    //             bool ok;
    //             if (attrs.getOpt<bool>(SUMO_ATTR_CONSTRAINTS, nullptr, ok, false)) {
    //                 MSRailSignalConstraint::clearAll();
    //             }
    //             if (attrs.getOpt<bool>(SUMO_ATTR_RAIL, nullptr, ok, false)) {
    //                 // init before loading any vehicles to ensure that driveways are built early
    //                 MSRailSignalControl::getInstance();
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_RNGSTATE: {
    //             if (attrs.hasAttribute(SUMO_ATTR_DEFAULT)) {
    //                 RandHelper::loadState(attrs.getString(SUMO_ATTR_DEFAULT));
    //             }
    //             if (attrs.hasAttribute(SUMO_ATTR_RNG_ROUTEHANDLER)) {
    //                 RandHelper::loadState(attrs.getString(SUMO_ATTR_RNG_ROUTEHANDLER), MSRouteHandler::getParsingRNG());
    //             }
    //             if (attrs.hasAttribute(SUMO_ATTR_RNG_INSERTIONCONTROL)) {
    //                 RandHelper::loadState(attrs.getString(SUMO_ATTR_RNG_INSERTIONCONTROL), MSNet::getInstance()->getInsertionControl().getFlowRNG());
    //             }
    //             if (attrs.hasAttribute(SUMO_ATTR_RNG_DEVICE)) {
    //                 RandHelper::loadState(attrs.getString(SUMO_ATTR_RNG_DEVICE), MSDevice::getEquipmentRNG());
    //             }
    //             if (attrs.hasAttribute(SUMO_ATTR_RNG_DEVICE_BT)) {
    //                 RandHelper::loadState(attrs.getString(SUMO_ATTR_RNG_DEVICE_BT), MSDevice_BTreceiver::getRNG());
    //             }
    //             if (attrs.hasAttribute(SUMO_ATTR_RNG_DRIVERSTATE)) {
    //                 RandHelper::loadState(attrs.getString(SUMO_ATTR_RNG_DRIVERSTATE), OUProcess::getRNG());
    //             }
    //             if (attrs.hasAttribute(SUMO_ATTR_RNG_DEVICE_TOC)) {
    //                 RandHelper::loadState(attrs.getString(SUMO_ATTR_RNG_DEVICE_TOC), MSDevice_ToC::getResponseTimeRNG());
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_RNGLANE: {
    //             const int index = attrs.getInt(SUMO_ATTR_INDEX);
    //             const std::string state = attrs.getString(SUMO_ATTR_STATE);
    //             MSLane::loadRNGState(index, state);
    //             break;
    //         }
    //         case SUMO_TAG_EDGECONTROL: {
    //             bool ok;
    //             std::list<MSLane*> activeLanes;
    //             const std::vector<std::string>& laneIDs = attrs.get<std::vector<std::string> >(SUMO_ATTR_LANES, nullptr, ok, false);
    //             for (const std::string& laneID : laneIDs) {
    //                 MSLane* lane = MSLane::dictionary(laneID);
    //                 if (lane == nullptr) {
    //                     throw ProcessError(TLF("Unknown lane '%' in loaded state.", laneID));
    //                 }
    //                 activeLanes.push_back(lane);
    //             }
    //             MSNet::getInstance()->getEdgeControl().setActiveLanes(activeLanes);
    //             break;
    //         }
    //         case SUMO_TAG_ROUTINGENGINE: {
    //             bool ok = true;
    //             const SUMOTime lastAdaptation = attrs.get<SUMOTime>(SUMO_ATTR_LAST, nullptr, ok);
    //             const int index = attrs.get<int>(SUMO_ATTR_INDEX, nullptr, ok);
    //             if (lastAdaptation >= 0) {
    //                 MSRoutingEngine::initWeightUpdate(lastAdaptation);
    //             }
    //             MSRoutingEngine::initEdgeWeights(SVC_PASSENGER, lastAdaptation, index);
    //             if (OptionsCont::getOptions().getBool("device.rerouting.bike-speeds")) {
    //                 MSRoutingEngine::initEdgeWeights(SVC_BICYCLE);
    //             }
    //             if (use_meso_sim()) {
    //                 for (const MSEdge* e : MSEdge::getAllEdges()) {
    //                     for (MESegment* segment = meso_net()->getSegmentForEdge(*e); segment != nullptr; segment = segment->getNextSegment()) {
    //                         segment->resetCachedSpeeds();
    //                     }
    //                 }
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_EDGE: {
    // #ifdef HAVE_FOX
    //             MSRoutingEngine::loadState(attrs);
    // #endif
    //             break;
    //         }
    //         case SUMO_TAG_DELAY: {
    //             if (myVCAttrs != nullptr) {
    //                 delete myVCAttrs;
    //             }
    //             bool ok;
    //             MSNet::getInstance()->setLoaderTime(attrs.getOpt<SUMOTime>(SUMO_ATTR_LOADERTIME, nullptr, ok, 0));
    //             myVCAttrs = attrs.clone();
    //             break;
    //         }
    //         case SUMO_TAG_FLOWSTATE: {
    //             bool ok;
    //             SUMOVehicleParameter* pars = SUMOVehicleParserHelper::parseFlowAttributes(SUMO_TAG_FLOWSTATE, attrs, true, true, -1, -1, true);
    //             pars->repetitionsDone = attrs.get<int>(SUMO_ATTR_DONE, pars->id.c_str(), ok);
    //             pars->repetitionTotalOffset = attrs.getOptSUMOTimeReporting(SUMO_ATTR_NEXT, pars->id.c_str(), ok, 0);
    //             myFlowIndex = attrs.getInt(SUMO_ATTR_INDEX);
    //             myVehicleParameter = pars;
    //             break;
    //         }
    //         case SUMO_TAG_VTYPE: {
    //             myLastParameterised = myCurrentVType;
    //             break;
    //         }
    //         case SUMO_TAG_VEHICLE: {
    //             myLastParameterised = myVehicleParameter;
    //             myAttrs = attrs.clone();
    //             break;
    //         }
    //         case SUMO_TAG_DEVICE: {
    //             myDeviceAttrs.push_back(attrs.clone());
    //             break;
    //         }
    //         case SUMO_TAG_CFM_VARIABLES: {
    //             myCFMAttrs = attrs.clone();
    //             break;
    //         }
    //         case SUMO_TAG_REMINDER: {
    //             myReminderAttrs.push_back(attrs.clone());
    //             break;
    //         }
    //         case SUMO_TAG_VEHICLETRANSFER: {
    //             MSVehicleTransfer::getInstance()->loadState(attrs, myOffset, vc);
    //             break;
    //         }
    //         case SUMO_TAG_SEGMENT: {
    //             const std::string& segmentID = attrs.getString(SUMO_ATTR_ID);
    //             const MSEdge* const edge = MSEdge::dictionary(segmentID.substr(0, segmentID.rfind(":")));
    //             int idx = StringUtils::toInt(segmentID.substr(segmentID.rfind(":") + 1));
    //             mySegment = meso_net()->getSegmentForEdge(*edge);
    //             while (idx-- > 0 && mySegment != nullptr) {
    //                 mySegment = mySegment->getNextSegment();
    //             }
    //             if (mySegment == nullptr) {
    //                 throw ProcessError(TLF("Unknown segment '%' in loaded state.", segmentID));
    //             }
    //             myQueIndex = 0;
    //             break;
    //         }
    //         case SUMO_TAG_LANE: {
    //             bool ok;
    //             const std::string laneID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //             myCurrentLane = MSLane::dictionary(laneID);
    //             if (myCurrentLane == nullptr) {
    //                 throw ProcessError(TLF("Unknown lane '%' in loaded state.", laneID));
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_VIEWSETTINGS_VEHICLES: {
    //             bool ok;
    //             const std::vector<std::string>& vehIDs = attrs.get<std::vector<std::string> >(SUMO_ATTR_VALUE, nullptr, ok, false);
    //             std::vector<SUMOVehicle*> vehs;
    //             for (const std::string& id : vehIDs) {
    //                 SUMOVehicle* v = vc.getVehicle(id);
    //                 // vehicle could be removed due to options
    //                 if (v != nullptr) {
    //                     vehs.push_back(v);
    //                     myArrived.erase(v);
    //                 }
    //             }
    //             if (use_meso_sim()) {
    //                 if (myQueIndex >= mySegment->numQueues()) {
    //                     throw ProcessError(TLF("Invalid queue index '%' on segment '%'. Check for consistency of lane numbers and queue options.", myQueIndex, mySegment->getID()));
    //                 }
    //                 const SUMOTime blockTime = StringUtils::toLong(attrs.getString(SUMO_ATTR_TIME));
    //                 const SUMOTime entryBlockTime = StringUtils::toLong(attrs.getString(SUMO_ATTR_BLOCKTIME));
    //                 mySegment->loadState(vehs, blockTime - myOffset, entryBlockTime - myOffset, myQueIndex);
    //                 if (attrs.hasAttribute(SUMO_ATTR_STATE)) {
    //                     mySegment->loadQueueExtraState(myQueIndex, attrs.getString(SUMO_ATTR_STATE), myOffset);
    //                 }
    //                 myQueIndex++;
    //             } else {
    //                 myCurrentLane->loadState(vehs);
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_LINK: {
    //             bool ok;
    //             myCurrentLink = nullptr;
    //             const std::string toLaneID = attrs.get<std::string>(SUMO_ATTR_TO, nullptr, ok);
    //             for (MSLink* link : myCurrentLane->getLinkCont()) {
    //                 if (link->getViaLaneOrLane()->getID() == toLaneID) {
    //                     myCurrentLink = link;
    //                 }
    //             }
    //             if (myCurrentLink == nullptr) {
    //                 throw ProcessError("Unknown link from lane '" + myCurrentLane->getID() + "' to lane '" + toLaneID + "' in loaded state");
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_APPROACHING: {
    //             bool ok;
    //             const std::string vehID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //             const SUMOTime arrivalTime = attrs.get<SUMOTime>(SUMO_ATTR_ARRIVALTIME, nullptr, ok);
    //             const double arrivalSpeed = attrs.get<double>(SUMO_ATTR_ARRIVALSPEED, nullptr, ok);
    //             const double leaveSpeed = attrs.get<double>(SUMO_ATTR_DEPARTSPEED, nullptr, ok);
    //             const bool setRequest = attrs.get<bool>(SUMO_ATTR_REQUEST, nullptr, ok);
    //             const double arrivalSpeedBraking = attrs.get<double>(SUMO_ATTR_ARRIVALSPEEDBRAKING, nullptr, ok);
    //             const SUMOTime waitingTime = attrs.get<SUMOTime>(SUMO_ATTR_WAITINGTIME, nullptr, ok);
    //             const double dist = attrs.get<double>(SUMO_ATTR_DISTANCE, nullptr, ok);
    //             const double latOffset = attrs.getOpt<double>(SUMO_ATTR_POSITION_LAT, nullptr, ok, 0);
    //             SUMOVehicle* veh = vc.getVehicle(vehID);
    //             myCurrentLink->setApproaching(veh, arrivalTime, arrivalSpeed, leaveSpeed, setRequest, arrivalSpeedBraking, waitingTime, dist, latOffset);
    //             if (!use_meso_sim()) {
    //                 MSVehicle* microVeh = dynamic_cast<MSVehicle*>(veh);
    //                 microVeh->loadPreviousApproaching(myCurrentLink, setRequest, arrivalTime, arrivalSpeed, arrivalSpeedBraking, dist, leaveSpeed);
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_RAILSIGNAL_CONSTRAINT_TRACKER: {
    //             MSRailSignalConstraint_Predecessor::loadState(attrs);
    //             break;
    //         }
    //         case SUMO_TAG_DRIVEWAY:
    //         case SUMO_TAG_SUBDRIVEWAY: {
    //             MSDriveWay::loadState(attrs, element);
    //             break;
    //         }
    //         case SUMO_TAG_PARAM: {
    //             bool ok;
    //             const std::string key = attrs.get<std::string>(SUMO_ATTR_KEY, nullptr, ok);
    //             // circumventing empty string test
    //             const std::string val = attrs.hasAttribute(SUMO_ATTR_VALUE) ? attrs.getString(SUMO_ATTR_VALUE) : "";
    //             assert(myLastParameterised != 0);
    //             if (myLastParameterised != nullptr) {
    //                 myLastParameterised->setParameter(key, val);
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_TRANSPORTABLES:
    //             if (attrs.getString(SUMO_ATTR_TYPE) == "person") {
    //                 MSNet::getInstance()->getPersonControl().loadState(attrs.getString(SUMO_ATTR_STATE));
    //             }
    //             if (attrs.getString(SUMO_ATTR_TYPE) == "container") {
    //                 MSNet::getInstance()->getContainerControl().loadState(attrs.getString(SUMO_ATTR_STATE));
    //             }
    //             break;
    //         case SUMO_TAG_PERSON:
    //         case SUMO_TAG_CONTAINER:
    //             myAttrs = attrs.clone();
    //             break;
    //         case SUMO_TAG_RAILSIGNAL_CONSTRAINTS: {
    //             bool ok = true;
    //             const std::string signalID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //             if (!MSNet::getInstance()->getTLSControl().knows(signalID)) {
    //                 throw InvalidArgument("Rail signal '" + signalID + "' in railSignalConstraints loaded from state is not known");
    //             }
    //             myConstrainedSignal = dynamic_cast<MSRailSignal*>(MSNet::getInstance()->getTLSControl().get(signalID).getDefault());
    //             if (myConstrainedSignal == nullptr) {
    //                 throw InvalidArgument("Traffic light '" + signalID + "' is not a rail signal");
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_PREDECESSOR: // intended fall-through
    //         case SUMO_TAG_INSERTION_PREDECESSOR: // intended fall-through
    //         case SUMO_TAG_FOE_INSERTION: // intended fall-through
    //         case SUMO_TAG_INSERTION_ORDER: // intended fall-through
    //         case SUMO_TAG_BIDI_PREDECESSOR:
    //             myLastParameterised = NLHandler::addPredecessorConstraint(element, attrs, myConstrainedSignal);
    //             break;
    //         case SUMO_TAG_TLLOGIC: {
    //             bool ok;
    //             const std::string tlID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //             const std::string programID = attrs.get<std::string>(SUMO_ATTR_PROGRAMID, tlID.c_str(), ok);
    //             const int phase = attrs.get<int>(SUMO_ATTR_PHASE, tlID.c_str(), ok);
    //             const SUMOTime spentDuration = attrs.get<SUMOTime>(SUMO_ATTR_DURATION, tlID.c_str(), ok);
    //             const SUMOTime nextSwitch = attrs.get<SUMOTime>(SUMO_ATTR_UNTIL, tlID.c_str(), ok);
    //             const SUMOTime timeInCycle = attrs.get<SUMOTime>(SUMO_ATTR_CYCLETIME, tlID.c_str(), ok);
    //             const bool active = attrs.get<bool>(SUMO_ATTR_ACTIVE, tlID.c_str(), ok);
    //             MSTLLogicControl& tlc = MSNet::getInstance()->getTLSControl();
    //             MSTrafficLightLogic* tl = tlc.get(tlID, programID);
    //             if (tl == nullptr) {
    //                 if (programID == "online") {
    //                     WRITE_WARNINGF(TL("Ignoring program '%' for traffic light '%' in loaded state"), programID, tlID);
    //                     return;
    //                 } else {
    //                     throw ProcessError("Unknown program '" + programID + "' for traffic light '" + tlID + "'");
    //                 }
    //             }
    //             if (phase >= tl->getPhaseNumber()) {
    //                 throw ProcessError("Invalid phase '" + toString(phase) + "' for traffic light '" + tlID + "'");
    //             }
    //             // might not be set if the phase happens to match and there are multiple programs
    //             tl->loadState(tlc, myTime, phase, spentDuration, nextSwitch, timeInCycle, active);
    //             break;
    //         }
    //         case SUMO_TAG_DISPATCHER: {
    //             bool ok = true;
    //             SUMOTime next = attrs.get<SUMOTime>(SUMO_ATTR_NEXT, "dispatcher", ok);
    //             MSDevice_Taxi::initDispatch(next);
    //             MSDevice_Taxi::getDispatchAlgorithm()->loadState(attrs);
    //             break;
    //         }
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        self.base.my_start_element(element, attrs);
        let mut ok = true;
        match element {
            SUMO_TAG_SNAPSHOT => {
                self.myTime = utils::common::time::string2time(&attrs.get_string(SUMO_ATTR_TIME)).unwrap_or_else(|e| panic!("{e}"));
                let version = attrs.get_string(SUMO_ATTR_VERSION);
                if version != STATE_VERSION_STRING {
                    utils::common::msg_handler::write_warning(&format!("State was written with sumo version {} (present: {})!", version, STATE_VERSION_STRING));
                }
                if attrs.get_opt::<bool>(SUMO_ATTR_CONSTRAINTS, "", &mut ok, false, false) {
                    // shortcut: MSRailSignalConstraint::clearAll is not ported under a verified name, state with constraints cannot be loaded yet
                    panic!("shortcut: railSignal constraints in saved state are not ported");
                }
                if attrs.get_opt::<bool>(SUMO_ATTR_RAIL, "", &mut ok, false, false) {
                    // init before loading any vehicles to ensure that driveways are built early
                    MSRailSignalControl::get_instance();
                }
            }
            SUMO_TAG_RNGSTATE => {
                if attrs.has_attribute(SUMO_ATTR_DEFAULT) {
                    let state = attrs.get_string(SUMO_ATTR_DEFAULT);
                    utils::common::rand_helper::with_default(|rng| rng.load_state(&state));
                }
                if attrs.has_attribute(SUMO_ATTR_RNG_ROUTEHANDLER) {
                    SUMORouteHandler::get_parsing_rng().m().load_state(&attrs.get_string(SUMO_ATTR_RNG_ROUTEHANDLER));
                }
                if attrs.has_attribute(SUMO_ATTR_RNG_INSERTIONCONTROL) {
                    MSNet::get_instance().m().get_insertion_control().get_flow_rng().m().load_state(&attrs.get_string(SUMO_ATTR_RNG_INSERTIONCONTROL));
                }
                if attrs.has_attribute(SUMO_ATTR_RNG_DEVICE) {
                    MSDevice::get_equipment_rng().m().load_state(&attrs.get_string(SUMO_ATTR_RNG_DEVICE));
                }
                if attrs.has_attribute(SUMO_ATTR_RNG_DEVICE_BT) {
                    MSDevice_BTreceiver::get_rng().m().load_state(&attrs.get_string(SUMO_ATTR_RNG_DEVICE_BT));
                }
                if attrs.has_attribute(SUMO_ATTR_RNG_DRIVERSTATE) {
                    OUProcess::get_rng().m().load_state(&attrs.get_string(SUMO_ATTR_RNG_DRIVERSTATE));
                }
                if attrs.has_attribute(SUMO_ATTR_RNG_DEVICE_TOC) {
                    MSDevice_ToC::get_response_time_rng().m().load_state(&attrs.get_string(SUMO_ATTR_RNG_DEVICE_TOC));
                }
            }
            SUMO_TAG_RNGLANE => {
                let index = attrs.get_int(SUMO_ATTR_INDEX).unwrap_or_else(|e| panic!("{e}"));
                let state = attrs.get_string(SUMO_ATTR_STATE);
                MSLane::load_rng_state(index, &state);
            }
            SUMO_TAG_EDGECONTROL => {
                let laneIDs: Vec<String> = attrs.get::<Vec<String>>(SUMO_ATTR_LANES, "", &mut ok, false);
                let mut activeLanes: VecDeque<Ptr<MSLane>> = VecDeque::new();
                for laneID in laneIDs.iter() {
                    let lane = MSLane::dictionary_2(laneID);
                    if lane.is_null() {
                        panic!("{}", ProcessError::new(utils::tlf!("Unknown lane '%' in loaded state.", laneID)));
                    }
                    activeLanes.push_back(lane);
                }
                MSNet::get_instance().m().get_edge_control().set_active_lanes(activeLanes);
            }
            SUMO_TAG_ROUTINGENGINE => {
                let lastAdaptation = attrs.get::<i64>(SUMO_ATTR_LAST, "", &mut ok, true);
                let index = attrs.get::<i32>(SUMO_ATTR_INDEX, "", &mut ok, true);
                if lastAdaptation >= 0 {
                    MSRoutingEngine::init_weight_update(lastAdaptation);
                }
                MSRoutingEngine::init_edge_weights(SVC_PASSENGER, lastAdaptation, index);
                if utils::options::options_cont::get_bool("device.rerouting.bike-speeds") || use_meso_sim() {
                    // shortcut: bicycle edge weights and meso segment speed reset are not ported
                    panic!("shortcut: bike-speed routing init and meso cached speeds are not ported");
                }
            }
            SUMO_TAG_EDGE => {
                MSRoutingEngine::load_state(attrs);
            }
            SUMO_TAG_DELAY => {
                MSNet::get_instance().m().set_loader_time(attrs.get_opt::<i64>(SUMO_ATTR_LOADERTIME, "", &mut ok, 0, false));
                self.myVCAttrs = leak_attrs(attrs);
            }
            SUMO_TAG_FLOWSTATE => {
                let pars = SUMOVehicleParserHelper::parse_flow_attributes(SUMO_TAG_FLOWSTATE, attrs, true, true, -1, -1, true);
                pars.m().repetitionsDone = attrs.get::<i64>(SUMO_ATTR_DONE, "", &mut ok, true);
                pars.m().repetitionTotalOffset = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_NEXT, "", &mut ok, 0, true);
                self.myFlowIndex = attrs.get_int(SUMO_ATTR_INDEX).unwrap_or_else(|e| panic!("{e}"));
                self.myVehicleParameter = pars;
            }
            SUMO_TAG_VTYPE => {
                self.myLastParameterised = self.myCurrentVType.cast::<Parameterised>();
            }
            SUMO_TAG_VEHICLE => {
                self.myLastParameterised = self.myVehicleParameter.cast::<Parameterised>();
                self.myAttrs = leak_attrs(attrs);
            }
            SUMO_TAG_DEVICE => {
                self.myDeviceAttrs.push(leak_attrs(attrs));
            }
            SUMO_TAG_REMINDER => {
                self.myReminderAttrs.push(leak_attrs(attrs));
            }
            SUMO_TAG_CFM_VARIABLES => {
                self.myCFMAttrs = leak_attrs(attrs);
            }
            SUMO_TAG_VEHICLETRANSFER => {
                let net = MSNet::get_instance();
                let vc: &mut MSVehicleControl = net.m().get_vehicle_control();
                MSVehicleTransfer::get_instance().m().load_state(attrs, self.myOffset, vc).unwrap_or_else(|e| panic!("{e}"));
            }
            SUMO_TAG_SEGMENT => {
                // shortcut: meso segment lookup needs MSEdge::dictionary by id and MESegment chain walking, meso state is not ported
                panic!("shortcut: meso segment state is not ported");
            }
            SUMO_TAG_LANE => {
                let laneID = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
                self.myCurrentLane = MSLane::dictionary_2(&laneID);
                if self.myCurrentLane.is_null() {
                    panic!("{}", ProcessError::new(utils::tlf!("Unknown lane '%' in loaded state.", laneID)));
                }
            }
            SUMO_TAG_VIEWSETTINGS_VEHICLES => {
                let vehIDs: Vec<String> = attrs.get::<Vec<String>>(SUMO_ATTR_VALUE, "", &mut ok, false);
                let net = MSNet::get_instance();
                let vc: &mut MSVehicleControl = net.m().get_vehicle_control();
                let mut vehs: Vec<Ptr<SUMOVehicle>> = Vec::new();
                for id in vehIDs.iter() {
                    let v = vc.get_vehicle(id);
                    // vehicle could be removed due to options
                    if !v.is_null() {
                        vehs.push(v);
                        self.myArrived.retain(|a| *a != v);
                    }
                }
                if use_meso_sim() {
                    // shortcut: meso segment loadState and queue state need the MESegment chain, not ported
                    panic!("shortcut: meso view-settings state is not ported");
                } else {
                    self.myCurrentLane.m().load_state(&vehs);
                }
            }
            SUMO_TAG_LINK => {
                let toLaneID = attrs.get::<String>(SUMO_ATTR_TO, "", &mut ok, true);
                self.myCurrentLink = Ptr::null();
                let lane = self.myCurrentLane;
                for link in lane.m().get_link_cont().iter() {
                    if link.m().get_via_lane_or_lane().m().get_id().to_string() == toLaneID {
                        self.myCurrentLink = *link;
                    }
                }
                if self.myCurrentLink.is_null() {
                    panic!("{}", ProcessError::new(format!("Unknown link from lane '{}' to lane '{}' in loaded state", lane.m().get_id(), toLaneID)));
                }
            }
            SUMO_TAG_APPROACHING => {
                let vehID = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
                let arrivalTime = attrs.get::<i64>(SUMO_ATTR_ARRIVALTIME, "", &mut ok, true);
                let arrivalSpeed = attrs.get::<f64>(SUMO_ATTR_ARRIVALSPEED, "", &mut ok, true);
                let leaveSpeed = attrs.get::<f64>(SUMO_ATTR_DEPARTSPEED, "", &mut ok, true);
                let setRequest = attrs.get::<bool>(SUMO_ATTR_REQUEST, "", &mut ok, true);
                let arrivalSpeedBraking = attrs.get::<f64>(SUMO_ATTR_ARRIVALSPEEDBRAKING, "", &mut ok, true);
                let waitingTime = attrs.get::<i64>(SUMO_ATTR_WAITINGTIME, "", &mut ok, true);
                let dist = attrs.get::<f64>(SUMO_ATTR_DISTANCE, "", &mut ok, true);
                let latOffset = attrs.get_opt::<f64>(SUMO_ATTR_POSITION_LAT, "", &mut ok, 0.0, true);
                let net = MSNet::get_instance();
                let vc: &mut MSVehicleControl = net.m().get_vehicle_control();
                let veh = vc.get_vehicle(&vehID);
                self.myCurrentLink.m().set_approaching(veh, arrivalTime, arrivalSpeed, leaveSpeed, setRequest, arrivalSpeedBraking, waitingTime, dist, latOffset);
                if !use_meso_sim() {
                    // shortcut: dynamic_cast<MSVehicle*> is unchecked, approaching state is only loaded for micro vehicles
                    veh.cast::<MSVehicle>().m().load_previous_approaching(self.myCurrentLink, setRequest, arrivalTime, arrivalSpeed, arrivalSpeedBraking, dist, leaveSpeed);
                }
            }
            SUMO_TAG_RAILSIGNAL_CONSTRAINT_TRACKER | SUMO_TAG_RAILSIGNAL_CONSTRAINTS | SUMO_TAG_PREDECESSOR | SUMO_TAG_INSERTION_PREDECESSOR | SUMO_TAG_FOE_INSERTION | SUMO_TAG_INSERTION_ORDER | SUMO_TAG_BIDI_PREDECESSOR => {
                // shortcut: rail signal constraint loading is not ported
                panic!("shortcut: railSignal constraint state is not ported");
            }
            SUMO_TAG_TLLOGIC => {
                let tlID = attrs.get::<String>(SUMO_ATTR_ID, "", &mut ok, true);
                let programID = attrs.get::<String>(SUMO_ATTR_PROGRAMID, &tlID, &mut ok, true);
                let phase = attrs.get::<i32>(SUMO_ATTR_PHASE, &tlID, &mut ok, true);
                let spentDuration = attrs.get::<i64>(SUMO_ATTR_DURATION, &tlID, &mut ok, true);
                let nextSwitch = attrs.get::<i64>(SUMO_ATTR_UNTIL, &tlID, &mut ok, true);
                let timeInCycle = attrs.get::<i64>(SUMO_ATTR_CYCLETIME, &tlID, &mut ok, true);
                let active = attrs.get::<bool>(SUMO_ATTR_ACTIVE, &tlID, &mut ok, true);
                let net = MSNet::get_instance();
                let tlc: &mut MSTLLogicControl = net.m().get_tls_control();
                let tl = tlc.get_2(&tlID, &programID);
                if tl.is_null() {
                    if programID == "online" {
                        utils::common::msg_handler::write_warning(&utils::tlf!("Ignoring program '%' for traffic light '%' in loaded state", programID, tlID));
                        return;
                    } else {
                        panic!("{}", ProcessError::new(format!("Unknown program '{}' for traffic light '{}'", programID, tlID)));
                    }
                }
                if phase >= tl.m().get_phase_number() {
                    panic!("{}", ProcessError::new(format!("Invalid phase '{}' for traffic light '{}'", phase, tlID)));
                }
                // might not be set if the phase happens to match and there are multiple programs
                tl.m().load_state(tlc, self.myTime, phase, spentDuration, nextSwitch, timeInCycle, active);
            }
            SUMO_TAG_DRIVEWAY | SUMO_TAG_SUBDRIVEWAY => {
                MSDriveWay::load_state(attrs, element);
            }
            SUMO_TAG_PARAM => {
                let key = attrs.get::<String>(SUMO_ATTR_KEY, "", &mut ok, true);
                // circumventing empty string test
                let val = if attrs.has_attribute(SUMO_ATTR_VALUE) { attrs.get_string(SUMO_ATTR_VALUE) } else { String::new() };
                if !self.myLastParameterised.is_null() {
                    self.myLastParameterised.m().set_parameter(&key, &val);
                }
            }
            SUMO_TAG_TRANSPORTABLES => {
                let typeStr = attrs.get_string(SUMO_ATTR_TYPE);
                if typeStr == "person" {
                    MSNet::get_instance().m().get_person_control().load_state(&attrs.get_string(SUMO_ATTR_STATE));
                }
                if typeStr == "container" {
                    MSNet::get_instance().m().get_container_control().load_state(&attrs.get_string(SUMO_ATTR_STATE));
                }
            }
            SUMO_TAG_PERSON | SUMO_TAG_CONTAINER => {
                self.myAttrs = leak_attrs(attrs);
            }
            SUMO_TAG_DISPATCHER => {
                // shortcut: MSDevice_Taxi::initDispatch is not ported under a verified name
                panic!("shortcut: taxi dispatcher state is not ported");
            }
            _ => {}
        }
    }

    /// C++: `void MSStateHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // MSStateHandler::myEndElement(int element) {
    //     MSRouteHandler::myEndElement(element);
    //     switch (element) {
    //         case SUMO_TAG_PERSON:
    //         case SUMO_TAG_CONTAINER: {
    //             MSTransportableControl& tc = (element == SUMO_TAG_PERSON ? MSNet::getInstance()->getPersonControl() : MSNet::getInstance()->getContainerControl());
    //             MSTransportable* transportable = tc.get(myAttrs->getString(SUMO_ATTR_ID));
    //             transportable->loadState(myAttrs->getString(SUMO_ATTR_STATE));
    //             tc.fixLoadCount(transportable);
    //             delete myAttrs;
    //             myAttrs = nullptr;
    //             break;
    //         }
    //         case SUMO_TAG_FLOWSTATE: {
    //             MSNet::getInstance()->getInsertionControl().addFlow(myVehicleParameter, myFlowIndex);
    //             myVehicleParameter = nullptr;
    //             break;
    //         }
    //         case SUMO_TAG_SNAPSHOT: {
    //             if (myVCAttrs == nullptr) {
    //                 throw ProcessError(TL("Could not load vehicle control state"));
    //             }
    //             MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //             vc.setState(myVCAttrs->getInt(SUMO_ATTR_NUMBER),
    //                         myVCAttrs->getInt(SUMO_ATTR_BEGIN),
    //                         myVCAttrs->getInt(SUMO_ATTR_END),
    //                         myVCAttrs->getFloat(SUMO_ATTR_DEPART),
    //                         myVCAttrs->getFloat(SUMO_ATTR_TIME),
    //                         myVCAttrs->getFloat(SUMO_ATTR_SPEEDFACTOR),
    //                         myVCAttrs->getFloat(SUMO_ATTR_DECEL));
    //             if (myRemoved > 0) {
    //                 WRITE_MESSAGEF(TL("Removed % vehicles while loading state."), toString(myRemoved));
    //                 vc.discountStateRemoved(myRemoved);
    //             }
    //             for (SUMOVehicle* v : myArrived) {
    //                 // state was created with active option --keep-after-arrival
    //                 vc.deleteKeptVehicle(v);
    //             }
    //             if (!use_meso_sim()) {
    //                 for (MSVehicleControl::constVehIt i = vc.loadedVehBegin(); i != vc.loadedVehEnd(); ++i) {
    //                     MSVehicle* microVeh = dynamic_cast<MSVehicle*>((*i).second);
    //                     if (microVeh->hasDeparted() && microVeh->getLane() != nullptr) {
    //                         // occupancy update must happen after all lane states have been loaded
    //                         microVeh->updateBestLanes();
    //                     }
    //                 }
    //             }
    //             MSDevice_Taxi::finalizeLoadState();
    //             break;
    //         }
    //         default:
    //             break;
    //     }
    //     if (element != SUMO_TAG_PARAM && myVehicleParameter == nullptr && myCurrentVType == nullptr) {
    //         myLastParameterised = nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        self.base.my_end_element(element);
        match element {
            SUMO_TAG_PERSON | SUMO_TAG_CONTAINER => {
                let net = MSNet::get_instance();
                let tc = if element == SUMO_TAG_PERSON { net.m().get_person_control() } else { net.m().get_container_control() };
                let transportable = tc.get(&self.myAttrs.get_string(SUMO_ATTR_ID));
                // shortcut: MSTransportable::loadState and fixLoadCount are not wired here, loaded transportable state is refused
                let _ = transportable;
                panic!("shortcut: transportable (person/container) state loading is not ported");
            }
            SUMO_TAG_FLOWSTATE => {
                MSNet::get_instance().m().get_insertion_control().add_flow(self.myVehicleParameter, self.myFlowIndex);
                self.myVehicleParameter = Ptr::null();
            }
            SUMO_TAG_SNAPSHOT => {
                if self.myVCAttrs.is_null() {
                    panic!("{}", ProcessError::new("Could not load vehicle control state"));
                }
                let net = MSNet::get_instance();
                let vc: &mut MSVehicleControl = net.m().get_vehicle_control();
                vc.set_state(
                    self.myVCAttrs.get_int(SUMO_ATTR_NUMBER).unwrap_or_else(|e| panic!("{e}")),
                    self.myVCAttrs.get_int(SUMO_ATTR_BEGIN).unwrap_or_else(|e| panic!("{e}")),
                    self.myVCAttrs.get_int(SUMO_ATTR_END).unwrap_or_else(|e| panic!("{e}")),
                    self.myVCAttrs.get_float(SUMO_ATTR_DEPART).unwrap_or_else(|e| panic!("{e}")),
                    self.myVCAttrs.get_float(SUMO_ATTR_TIME).unwrap_or_else(|e| panic!("{e}")),
                    self.myVCAttrs.get_float(SUMO_ATTR_SPEEDFACTOR).unwrap_or_else(|e| panic!("{e}")),
                    self.myVCAttrs.get_float(SUMO_ATTR_DECEL).unwrap_or_else(|e| panic!("{e}")),
                );
                if self.myRemoved > 0 {
                    utils::common::msg_handler::write_message(&format!("Removed {} vehicles while loading state.", self.myRemoved));
                    vc.discount_state_removed(self.myRemoved);
                }
                for v in self.myArrived.iter() {
                    // state was created with active option --keep-after-arrival
                    vc.delete_kept_vehicle(*v);
                }
                if !use_meso_sim() {
                    // shortcut: loaded vehicles are read from myVehicleDict directly (C++ loadedVehBegin..End), dynamic_cast is unchecked
                    for veh in vc.myVehicleDict.values() {
                        let microVeh = veh.cast::<MSVehicle>();
                        if microVeh.m().has_departed() && !microVeh.m().get_lane().is_null() {
                            // occupancy update must happen after all lane states have been loaded
                            microVeh.m().update_best_lanes(false, Ptr::null());
                        }
                    }
                }
                MSDevice_Taxi::finalize_load_state();
            }
            _ => {}
        }
        if element != SUMO_TAG_PARAM && self.myVehicleParameter.is_null() && self.myCurrentVType.is_null() {
            self.myLastParameterised = Ptr::null();
        }
    }

    /// C++: `void MSStateHandler::closeVehicle()` [virtual, protected]
    /// Ends the processing of a vehicle
    // void
    // MSStateHandler::closeVehicle() {
    //     assert(myVehicleParameter != nullptr);
    //     myVehicleParameter->depart -= myOffset;
    //     // the vehicle was already counted in MSVehicleControl::setState
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     // make a copy because myVehicleParameter is reset in closeVehicle()
    //     const std::string vehID = myVehicleParameter->id;
    //     if (myVehiclesToRemove.count(vehID) == 0) {
    //
    //         // devices that influence simulation behavior must replicate stochastic assignment
    //         // also, setting the parameter avoids extra calls to MSDevice::myEquipmentRNG (which would pollute replication)
    //         std::vector<std::string> deviceNames;
    //         for (auto attrs : myDeviceAttrs) {
    //             deviceNames.push_back(MSDevice::getDeviceName(attrs->getString(SUMO_ATTR_ID)));
    //         }
    //         myVehicleParameter->setParameter(MSDevice::LOADSTATE_DEVICENAMES, toString(deviceNames));
    //         MSRouteHandler::closeVehicle();
    //         SUMOVehicle* v = vc.getVehicle(vehID);
    //         // special case: transportable devices are not assigned by options
    //         if (std::find(deviceNames.begin(), deviceNames.end(), "person") != deviceNames.end()) {
    //             dynamic_cast<MSBaseVehicle*>(v)->initTransportableDevice(true);
    //         }
    //         if (std::find(deviceNames.begin(), deviceNames.end(), "container") != deviceNames.end()) {
    //             dynamic_cast<MSBaseVehicle*>(v)->initTransportableDevice(false);
    //         }
    //         // clean up added param after initializing devices in closeVehicle
    //         ((SUMOVehicleParameter&)v->getParameter()).unsetParameter(MSDevice::LOADSTATE_DEVICENAMES);
    //         if (v == nullptr) {
    //             throw ProcessError(TLF("Could not load vehicle '%' from state", vehID));
    //         }
    //         v->setChosenSpeedFactor(myAttrs->getFloat(SUMO_ATTR_SPEEDFACTOR));
    //         v->loadState(*myAttrs, myOffset);
    //
    //         if (v->hasDeparted()) {
    //             // vehicle already departed: disable pre-insertion rerouting and enable regular routing behavior
    //             MSDevice_Routing* routingDevice = static_cast<MSDevice_Routing*>(v->getDevice(typeid(MSDevice_Routing)));
    //             if (routingDevice != nullptr) {
    //                 routingDevice->notifyEnter(*v, MSMoveReminder::NOTIFICATION_DEPARTED);
    //             }
    //             MSNet::getInstance()->getInsertionControl().alreadyDeparted(v);
    //             if (MSRailSignalControl::hasInstance()) {
    //                 // register route for deadlock prevention (vehicleStateChanged would not be called otherwise)
    //                 MSRailSignalControl::getInstance().vehicleStateChanged(v, MSNet::VehicleState::NEWROUTE, "loadState");
    //             }
    //             vc.handleTriggeredDepart(v, false);
    //             if (v->hasArrived()) {
    //                 myArrived.insert(v);
    //             }
    //         }
    //         while (!myDeviceAttrs.empty()) {
    //             const std::string attrID = myDeviceAttrs.back()->getString(SUMO_ATTR_ID);
    //             for (MSVehicleDevice* const dev : v->getDevices()) {
    //                 if (dev->getID() == attrID) {
    //                     dev->loadState(*myDeviceAttrs.back());
    //                 }
    //             }
    //             delete myDeviceAttrs.back();
    //             myDeviceAttrs.pop_back();
    //         }
    //         bool ok = true;
    //         while (!myReminderAttrs.empty()) {
    //             const std::string attrID = myReminderAttrs.back()->getString(SUMO_ATTR_ID);
    //             const SUMOTime time = myReminderAttrs.back()->get<SUMOTime>(SUMO_ATTR_TIME, nullptr, ok, false);
    //             const double pos = myReminderAttrs.back()->get<double>(SUMO_ATTR_POSITION, nullptr, ok, false);
    //             const auto& remDict = MSNet::getInstance()->getDetectorControl().getAllReminders();
    //             auto it = remDict.find(attrID);
    //             if (it != remDict.end()) {
    //                 it->second->loadReminderState(v->getNumericalID(), time, pos);
    //             }
    //             delete myReminderAttrs.back();
    //             myReminderAttrs.pop_back();
    //         }
    //         if (myCFMAttrs != nullptr) {
    //             assert(!use_meso_sim());
    //             MSVehicle* microVeh = dynamic_cast<MSVehicle*>(v);
    //             const MSCFModel::VehicleVariables* vars = microVeh->getCarFollowVariables();
    //             if (vars != nullptr) {
    //                 const_cast<MSCFModel::VehicleVariables*>(vars)->loadState(*myCFMAttrs);
    //             }
    //             delete myCFMAttrs;
    //             myCFMAttrs = nullptr;
    //         }
    //     } else {
    //         const std::string embeddedRouteID = "!" + myVehicleParameter->id;
    //         if (MSRoute::hasRoute(embeddedRouteID)) {
    //             ConstMSRoutePtr embedded = MSRoute::dictionary(embeddedRouteID);
    //             embedded->checkRemoval();
    //         }
    //         delete myVehicleParameter;
    //
    //         myVehicleParameter = nullptr;
    //         myRemoved++;
    //         while (!myDeviceAttrs.empty()) {
    //             delete myDeviceAttrs.back();
    //             myDeviceAttrs.pop_back();
    //         }
    //         while (!myReminderAttrs.empty()) {
    //             delete myReminderAttrs.back();
    //             myReminderAttrs.pop_back();
    //         }
    //     }
    //     delete myAttrs;
    // }
    #[allow(unused_variables)]
    pub fn close_vehicle(&mut self) {
        self.myVehicleParameter.m().depart -= self.myOffset;
        // the vehicle was already counted in MSVehicleControl::setState
        let net = MSNet::get_instance();
        let vc: &mut MSVehicleControl = net.m().get_vehicle_control();
        // make a copy because myVehicleParameter is reset in closeVehicle()
        let vehID = self.myVehicleParameter.m().id.clone();
        if !self.myVehiclesToRemove.contains(&vehID) {
            // shortcut: the embedded route "!<id>" checkRemoval is not ported under a verified name
            let deviceNames: Vec<String> = self.myDeviceAttrs.iter().map(|a| MSDevice::get_device_name(&a.get_string(SUMO_ATTR_ID))).collect();
            self.myVehicleParameter.m().set_parameter(crate::microsim::devices::ms_device::LOADSTATE_DEVICENAMES, &deviceNames.join(" "));
            self.base.close_vehicle();
            let v: Ptr<SUMOVehicle> = vc.get_vehicle(&vehID);
            if v.is_null() {
                panic!("{}", ProcessError::new(format!("Could not load vehicle '{}' from state", vehID)));
            }
            if deviceNames.iter().any(|d| d == "person" || d == "container") {
                // shortcut: initTransportableDevice for loaded persons and containers is not ported
                panic!("shortcut: transportable devices in loaded state are not ported");
            }
            let base: Ptr<MSBaseVehicle> = v.cast::<MSBaseVehicle>();
            base.m().myParameter.m().unset_parameter(crate::microsim::devices::ms_device::LOADSTATE_DEVICENAMES);
            let attrs: &SUMOSAXAttributes = &*self.myAttrs;
            base.m().set_chosen_speed_factor(attrs.get_float(SUMO_ATTR_SPEEDFACTOR).unwrap_or_else(|e| panic!("{e}")));
            v.cast::<MSVehicle>().m().load_state(attrs, self.myOffset);
            if base.has_departed() {
                // shortcut: the MSDevice_Routing notifyEnter call is skipped, device lookup by type_info is not ported
                MSNet::get_instance().m().get_insertion_control().already_departed(v);
                // shortcut: the rail-signal vehicleStateChanged registration is skipped, its signature is not verified
                vc.handle_triggered_depart(v, false);
                if base.has_arrived() && !self.myArrived.contains(&v) {
                    self.myArrived.push(v);
                }
            }
            while let Some(attrs) = self.myDeviceAttrs.pop() {
                let attrID = attrs.get_string(SUMO_ATTR_ID);
                for dev in base.m().get_devices().clone() {
                    if dev.m().base2.get_id() == attrID {
                        if attrID.starts_with("tripinfo_") {
                            dev.cast::<MSDevice_Tripinfo>().m().load_state(&attrs);
                        } else {
                            // shortcut: only tripinfo device states are restored, other device types need a device vtable
                            panic!("shortcut: device '{}' state restore is not ported", attrID);
                        }
                    }
                }
            }
            let mut ok = true;
            while let Some(attrs) = self.myReminderAttrs.pop() {
                let attrID = attrs.get_string(SUMO_ATTR_ID);
                let rattrs: &SUMOSAXAttributes = &*attrs;
                let time = rattrs.get::<i64>(SUMO_ATTR_TIME, "", &mut ok, false);
                let pos = rattrs.get::<f64>(SUMO_ATTR_POSITION, "", &mut ok, false);
                let net = MSNet::get_instance();
                let remDict = net.m().get_detector_control().get_all_reminders();
                if let Some(rem) = remDict.get(&attrID) {
                    rem.m().load_reminder_state(v.get_numerical_id() as i64, time, pos);
                }
            }
            if !self.myCFMAttrs.is_null() {
                let vars = v.cast::<MSVehicle>().m().get_car_follow_variables();
                if !vars.is_null() {
                    vars.m().load_state(&*self.myCFMAttrs);
                }
                self.myCFMAttrs = Ptr::null();
            }
        } else {
            self.myVehicleParameter = Ptr::null();
            self.myRemoved += 1;
            self.myDeviceAttrs.clear();
            self.myReminderAttrs.clear();
            self.myAttrs = Ptr::null();
        }
    }

    /// C++: `void MSStateHandler::saveRNGs(OutputDevice &)` [static, private]
    /// save the state of random number generators
    // void
    // MSStateHandler::saveRNGs(OutputDevice& out) {
    //     out.openTag(SUMO_TAG_RNGSTATE);
    //     out.writeAttr(SUMO_ATTR_DEFAULT, RandHelper::saveState());
    //     out.writeAttr(SUMO_ATTR_RNG_ROUTEHANDLER, RandHelper::saveState(MSRouteHandler::getParsingRNG()));
    //     out.writeAttr(SUMO_ATTR_RNG_INSERTIONCONTROL, RandHelper::saveState(MSNet::getInstance()->getInsertionControl().getFlowRNG()));
    //     out.writeAttr(SUMO_ATTR_RNG_DEVICE, RandHelper::saveState(MSDevice::getEquipmentRNG()));
    //     out.writeAttr(SUMO_ATTR_RNG_DEVICE_BT, RandHelper::saveState(MSDevice_BTreceiver::getRNG()));
    //     out.writeAttr(SUMO_ATTR_RNG_DRIVERSTATE, RandHelper::saveState(OUProcess::getRNG()));
    //     out.writeAttr(SUMO_ATTR_RNG_DEVICE_TOC, RandHelper::saveState(MSDevice_ToC::getResponseTimeRNG()));
    //     MSLane::saveRNGStates(out);
    //     out.closeTag();
    //
    // }
    #[allow(unused_variables)]
    pub fn save_rn_gs(out: &mut OutputDevice) {
        out.open_tag_id(SUMO_TAG_RNGSTATE);
        out.write_attr(SUMO_ATTR_DEFAULT, &utils::common::rand_helper::with_default(|rng| rng.save_state()));
        out.write_attr(SUMO_ATTR_RNG_ROUTEHANDLER, &SUMORouteHandler::get_parsing_rng().m().save_state());
        out.write_attr(SUMO_ATTR_RNG_INSERTIONCONTROL, &MSNet::get_instance().m().get_insertion_control().get_flow_rng().m().save_state());
        out.write_attr(SUMO_ATTR_RNG_DEVICE, &MSDevice::get_equipment_rng().m().save_state());
        out.write_attr(SUMO_ATTR_RNG_DEVICE_BT, &MSDevice_BTreceiver::get_rng().m().save_state());
        out.write_attr(SUMO_ATTR_RNG_DRIVERSTATE, &OUProcess::get_rng().m().save_state());
        out.write_attr(SUMO_ATTR_RNG_DEVICE_TOC, &MSDevice_ToC::get_response_time_rng().m().save_state());
        MSLane::save_rng_states(out);
        out.close_tag("");
    }

    /// C++: `void MSStateHandler::MSStateHandler(const MSStateHandler &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &MSStateHandler) -> Self {
        // C++ copy constructor is deleted ("Invalidated"), copying is a programming error
        panic!("MSStateHandler copy constructor is invalidated in C++ and must not be used");
    }

    /// C++: `MSStateHandler & MSStateHandler::operator=(const MSStateHandler &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSStateHandler) -> &mut MSStateHandler {
        // C++ assignment operator is deleted ("Invalidated"), assignment is a programming error
        panic!("MSStateHandler assignment operator is invalidated in C++ and must not be used");
    }

}

/// handler to read only the simulation time from a state
/// C++ class `MSStateHandler::MSStateTimeHandler` (microsim/MSStateHandler.h)
#[repr(C)]
pub struct MSStateHandler_MSStateTimeHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    pub myTime: SUMOTime,
}

impl std::ops::Deref for MSStateHandler_MSStateTimeHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for MSStateHandler_MSStateTimeHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl MSStateHandler_MSStateTimeHandler {
    /// C++: `SUMOTime MSStateTimeHandler::getTime(const std::string &)` [static]
    /// parse time from state file
    // SUMOTime
    // MSStateHandler::MSStateTimeHandler::getTime(const std::string& fileName) {
    //     // build handler and parser
    //     MSStateTimeHandler handler;
    //     handler.setFileName(fileName);
    //     handler.myTime = -1;
    //     SUMOSAXReader* parser = XMLSubSys::getSAXReader(handler);
    //     try {
    //         if (!parser->parseFirst(fileName)) {
    //             delete parser;
    //             throw ProcessError(TLF("Can not read XML-file '%'.", fileName));
    //         }
    //     } catch (ProcessError&) {
    //         delete parser;
    //         throw;
    //     }
    //     // parse
    //     while (parser->parseNext() && handler.myTime != -1);
    //     // clean up
    //     if (handler.myTime == -1) {
    //         delete parser;
    //         throw ProcessError(TLF("Could not parse time from state file '%'", fileName));
    //     }
    //     delete parser;
    //     return handler.myTime;
    // }
    #[allow(unused_variables)]
    pub fn get_time(fileName: &str) -> SUMOTime {
        // build handler and parser
        let mut handler = MSStateHandler_MSStateTimeHandler { base: utils::xml::sumo_sax_handler::new("", ""), myTime: 0 };
        handler.set_file_name(fileName);
        handler.myTime = -1;
        let mut parser = utils::xml::xml_sub_sys::get_sax_reader(false, false);
        if !parser.parse_first(&mut handler, fileName).unwrap_or_else(|e| panic!("{}", e)) {
            panic!("{}", ProcessError::new(format!("Can not read XML-file '{}'.", fileName)));
        }
        // parse
        while parser.parse_next(&mut handler).unwrap_or_else(|e| panic!("{}", e)) && handler.myTime != -1 {}
        // clean up
        if handler.myTime == -1 {
            panic!("{}", ProcessError::new(format!("Could not parse time from state file '{}'", fileName)));
        }
        handler.myTime
    }

    /// C++: `void MSStateTimeHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    // void
    // MSStateHandler::MSStateTimeHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     if (element == SUMO_TAG_SNAPSHOT) {
    //         myTime = string2time(attrs.getString(SUMO_ATTR_TIME));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        if element == SUMO_TAG_SNAPSHOT {
            self.myTime = utils::common::time::string2time(&attrs.get_string(SUMO_ATTR_TIME))
                .unwrap_or_else(|e| std::panic::panic_any(e));
        }
    }

}


impl utils::xml::generic_sax_handler::SaxHandler for MSStateHandler {
    fn handler(&mut self) -> &mut utils::xml::generic_sax_handler::GenericSAXHandler { self.base.handler() }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        MSStateHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        MSStateHandler::my_end_element(self, element);
        Ok(())
    }
}

impl utils::xml::generic_sax_handler::SaxHandler for MSStateHandler_MSStateTimeHandler {
    fn handler(&mut self) -> &mut utils::xml::generic_sax_handler::GenericSAXHandler { &mut self.base }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        MSStateHandler_MSStateTimeHandler::my_start_element(self, element, attrs);
        Ok(())
    }
}
