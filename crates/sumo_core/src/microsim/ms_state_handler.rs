//! Port of microsim/MSStateHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use utils::xml::generic_sax_handler::GenericSAXHandler;
use crate::prelude::*;

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
    pub myDeviceAttrs: i32,
    pub myReminderAttrs: i32,
    pub myCFMAttrs: Ptr<SUMOSAXAttributes>,
    /// the last object that potentially carries parameters
    pub myLastParameterised: Ptr<Parameterised>,
    /// vehicles that shall be removed when loading state
    pub myVehiclesToRemove: BTreeSet<String>,
    /// vehicles that were removed when loading state
    pub myRemoved: i32,
    /// the loaded counter of meso leader events (-1 if not given)
    pub myMesoEventCounter: i64,
    /// index of the currently parse flowState
    pub myFlowIndex: i32,
    /// rail signal for which constraints are being loaded
    pub myConstrainedSignal: Ptr<MSRailSignal>,
    /// vehicles that have arrived
    pub myArrived: i32,
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
    //     myMesoEventCounter(-1),
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
            myDeviceAttrs: 0,
            myReminderAttrs: 0,
            myCFMAttrs: Ptr::null(),
            myLastParameterised: Ptr::null(),
            myVehiclesToRemove: utils::options::options_cont::get_string_vector("load-state.remove-vehicles")
                .into_iter()
                .collect(),
            myRemoved: 0,
            myMesoEventCounter: -1,
            myFlowIndex: -1,
            myConstrainedSignal: Ptr::null(),
            myArrived: 0,
        };
        sh.myAmLoadingState = true;
        sh.myAllowInternalRoutes = true;
        sh
    }

    /// C++: `void MSStateHandler::~MSStateHandler()` [virtual]
    /// standard destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ body only frees the attribute holders (myDeviceAttrs, myReminderAttrs, myAttrs); not owned here yet
        // shortcut: the attribute holders are i32 placeholders in the skeleton, so nothing is freed until they are typed
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
    //     MSNet::getInstance()->getDetectorControl().saveState(out);
    //     MSRoutingEngine::saveState(out);
    //     out.close();
    //     gPrecision = defaultPrecision;
    // }
    #[allow(unused_variables)]
    pub fn save_state(file: &str, step: SUMOTime, usePrefix: bool) {
        todo!("port MSStateHandler::saveState")
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
    //             myMesoEventCounter = attrs.getOpt<long long int>(SUMO_ATTR_COUNT, nullptr, ok, -1);
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
    //         case SUMO_TAG_INDUCTION_LOOP: {
    //             MSDetectorFileOutput* const det = MSNet::getInstance()->getDetectorControl().getTypedDetectors(SUMO_TAG_INDUCTION_LOOP).get(attrs.getString(SUMO_ATTR_ID));
    //             // the detector may belong to an output which is not loaded
    //             if (det != nullptr) {
    //                 det->loadState(attrs, myOffset);
    //             }
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
        todo!("port MSStateHandler::myStartElement")
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
    //             if (use_meso_sim() && myMesoEventCounter >= 0) {
    //                 MELoop::setEventCounter(myMesoEventCounter);
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
        todo!("port MSStateHandler::myEndElement")
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
        todo!("port MSStateHandler::closeVehicle")
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
        // shortcut: FlowRNG (MSInsertionControl::getFlowRNG), MSDevice/BTreceiver/OUProcess/ToC RNGs and MSLane::saveRNGStates are not ported, their states are not written yet
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
        todo!("port MSStateTimeHandler::getTime")
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

