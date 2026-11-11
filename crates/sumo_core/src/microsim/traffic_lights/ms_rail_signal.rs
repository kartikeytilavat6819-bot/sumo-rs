//! Port of microsim/traffic_lights/MSRailSignal.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::pair<const SUMOVehicle *const, const MSLink::ApproachingVehicleInformation> MSRailSignal::Approaching`
pub type MSRailSignal_Approaching = (Ptr<SUMOVehicle>, MSLink_ApproachingVehicleInformation);

/// C++ typedef `std::map<const MSLane *, int, ComparatorNumericalIdLess> MSRailSignal::LaneVisitedMap`
pub type MSRailSignal_LaneVisitedMap = BTreeMap<Ptr<MSLane>, i32>;

/// C++ class `MSRailSignal::LinkInfo` (microsim/traffic_lights/MSRailSignal.h)
pub struct MSRailSignal_LinkInfo {
    pub myLink: Ptr<MSLink>,
    /// all driveways immediately following this link
    pub myDriveways: Vec<Ptr<MSDriveWay>>,
    pub myLastRerouteTime: SUMOTime,
    pub myLastRerouteVehicle: Ptr<SUMOVehicle>,
    /// whether this link is used for railway operations
    pub myControlled: bool,
}

impl MSRailSignal_LinkInfo {
    /// C++: `void LinkInfo::LinkInfo(MSLink *)`
    /// constructor
    // MSRailSignal::LinkInfo::LinkInfo(MSLink* link):
    //     myLink(link) {
    //     reset();
    // }
    #[allow(unused_variables)]
    pub fn new(link: Ptr<MSLink>) -> Self {
        todo!("port LinkInfo::LinkInfo")
    }

    /// C++: `void LinkInfo::~LinkInfo()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port LinkInfo::~LinkInfo")
    }

    /// C++: `std::string LinkInfo::getID() const` [const]
    /// return id for this railsignal-link
    // std::string
    // MSRailSignal::LinkInfo::getID() const {
    //     return myLink->getTLLogic()->getID() + "_" + toString(myLink->getTLIndex());
    // }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> String {
        todo!("port LinkInfo::getID")
    }

    /// C++: `MSDriveWay & LinkInfo::getDriveWay(const SUMOVehicle *, int)`
    /// retrieve an existing Driveway or construct a new driveway based on the vehicles route
    // MSDriveWay&
    // MSRailSignal::LinkInfo::getDriveWay(const SUMOVehicle* veh, int searchStart) {
    //     MSEdge* first = &myLink->getLane()->getEdge();
    //     auto searchStartIt = searchStart < 0 ? veh->getCurrentRouteEdge() : veh->getRoute().begin() + searchStart;
    //     MSRouteIterator firstIt = std::find(searchStartIt, veh->getRoute().end(), first);
    //     if (firstIt == veh->getRoute().end()) {
    //         // possibly the vehicle has already gone past the first edge (i.e.
    //         // because first is short or the step-length is high)
    //         // lets look backward along the route
    //         // give some slack because the vehicle might have been braking from a higher speed and using ballistic integration
    //         double lookBack = SPEED2DIST(veh->getSpeed() + 10) + veh->getLength();
    //         int routeIndex = veh->getRoutePosition() - 1;
    //         while (lookBack > 0 && routeIndex > 0) {
    //             const MSEdge* prevEdge = veh->getRoute().getEdges()[routeIndex];
    //             if (prevEdge == first) {
    //                 firstIt = veh->getRoute().begin() + routeIndex;
    //                 break;
    //             }
    //             lookBack -= prevEdge->getLength();
    //             routeIndex--;
    //         }
    //     }
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(dynamic_cast<const MSRailSignal*>(myLink->getTLLogic()));
    //     if (firstIt == veh->getRoute().end()) {
    //         WRITE_WARNING("Invalid approach information to rail signal " + MSDriveWay::getClickableTLLinkID(myLink) + " after rerouting for vehicle '" + veh->getID()
    //                       + "' first driveway edge '" + first->getID() + "' time=" + time2string(MSNet::getInstance()->getCurrentTimeStep()) + ".");
    //         if (myDriveways.empty()) {
    //             ConstMSEdgeVector dummyRoute;
    //             dummyRoute.push_back(&myLink->getLane()->getEdge());
    //             MSDriveWay* dw = MSDriveWay::buildDriveWay(rs->getNewDrivewayID(), myLink, dummyRoute.begin(), dummyRoute.end());
    //             myDriveways.push_back(dw);
    //         }
    //         return *myDriveways.front();
    //     }
    //     //std::cout << SIMTIME << " veh=" << veh->getID() << " rsl=" << getID() << " dws=" << myDriveways.size() << "\n";
    //     return getDriveWay(firstIt, veh->getRoute().end(), veh->getID());
    // }
    #[allow(unused_variables)]
    pub fn get_drive_way(&mut self, arg0: Ptr<SUMOVehicle>, searchStart: i32) -> &mut MSDriveWay {
        todo!("port LinkInfo::getDriveWay")
    }

    /// C++: `MSDriveWay & LinkInfo::getDriveWay(MSRouteIterator, MSRouteIterator, const std::string &)`
    // MSDriveWay&
    // MSRailSignal::LinkInfo::getDriveWay(MSRouteIterator firstIt, MSRouteIterator endIt, const std::string& info) {
    //     for (MSDriveWay* dw : myDriveways) {
    //         if (dw->match(firstIt, endIt)) {
    //             return *dw;
    //         }
    // #ifdef DEBUG_SELECT_DRIVEWAY
    //         std::cout << SIMTIME << " rs=" << getID() << " veh=" << info << " other dwSignal=" << dw->foundSignal() << " dwRoute=" << toString(dw->getRoute()) << "\n";
    // #else
    //         UNUSED_PARAMETER(info);
    // #endif
    //     }
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(dynamic_cast<const MSRailSignal*>(myLink->getTLLogic()));
    //     MSDriveWay* dw = MSDriveWay::buildDriveWay(rs->getNewDrivewayID(), myLink, firstIt, endIt);
    //     dw->setVehicle(info);
    // #ifdef DEBUG_SELECT_DRIVEWAY
    //     std::cout << SIMTIME << " rs=" << getID() << " veh=" << info << " new dwSignal=" << dw->foundSignal() << " dwRoute=" << toString(dw->getRoute()) << "\n";
    // #endif
    //     myDriveways.push_back(dw);
    //     return *myDriveways.back();
    // }
    #[allow(unused_variables)]
    pub fn get_drive_way_2(&mut self, firstIt: MSRouteIterator, endIt: MSRouteIterator, info: &str) -> &mut MSDriveWay {
        todo!("port LinkInfo::getDriveWay")
    }

    /// C++: `void LinkInfo::reroute(SUMOVehicle *, const MSEdgeVector &)`
    /// try rerouting vehicle if reservation failed
    // void
    // MSRailSignal::LinkInfo::reroute(SUMOVehicle* veh, const MSEdgeVector& occupied) {
    //     MSDevice_Routing* rDev = static_cast<MSDevice_Routing*>(veh->getDevice(typeid(MSDevice_Routing)));
    //     const SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //     if (rDev != nullptr
    //             && rDev->mayRerouteRailSignal()
    //             && (myLastRerouteVehicle != veh
    //                 // reroute each vehicle only once if no periodic routing is allowed,
    //                 // otherwise with the specified period
    //                 || (rDev->getPeriod() > 0 && myLastRerouteTime + rDev->getPeriod() <= now))) {
    //         myLastRerouteVehicle = veh;
    //         myLastRerouteTime = now;
    //
    // #ifdef DEBUG_REROUTE
    //         ConstMSEdgeVector oldRoute = veh->getRoute().getEdges();
    //         if (DEBUG_COND_LINKINFO) {
    //             std::cout << SIMTIME << " reroute veh=" << veh->getID() << " rs=" << getID() << " occupied=" << toString(occupied) << "\n";
    //         }
    // #endif
    //         MSRoutingEngine::Prohibitions prohibited;
    //         for (MSEdge* e : occupied) {
    //             // indefinite occupation because vehicles might be in deadlock on their current routes
    //             prohibited[e].end = std::numeric_limits<double>::max();
    //         }
    //         MSRoutingEngine::reroute(*veh, now, "railSignal:" + getID(), false, true, prohibited);
    // #ifdef DEBUG_REROUTE
    //         // attention this works only if we are not parallel!
    //         if (DEBUG_COND_LINKINFO) {
    //             if (veh->getRoute().getEdges() != oldRoute) {
    //                 std::cout << "    rerouting successful\n";
    //             }
    //         }
    // #endif
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reroute(&mut self, veh: Ptr<SUMOVehicle>, occupied: &MSEdgeVector) {
        todo!("port LinkInfo::reroute")
    }

    /// C++: `void LinkInfo::reset()`
    /// init LinkInfo
    // void
    // MSRailSignal::LinkInfo::reset() {
    //     myLastRerouteTime = -1;
    //     myLastRerouteVehicle = nullptr;
    //     myDriveways.clear();
    //     myControlled = (myLink->getViaLaneOrLane()->getPermissions() & myLink->getLane()->getPermissions() & SVC_RAIL_CLASSES) != 0;
    // }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        todo!("port LinkInfo::reset")
    }

}

/// A signal for rails
/// C++ class `MSRailSignal` (microsim/traffic_lights/MSRailSignal.h)
pub struct MSRailSignal {
    /// C++ base class `MSTrafficLightLogic`
    pub base: MSTrafficLightLogic,
    pub myNumericalID: i32,
    /// data storage for every link at this node (more than one when directly guarding a switch)
    pub myLinkInfos: Vec<MSRailSignal_LinkInfo>,
    /// The list of phases this logic uses
    pub myPhases: MSTrafficLightLogic_Phases,
    /// The current phase
    pub myCurrentPhase: MSPhaseDefinition,
    /// MSTrafficLightLogic requires that the phase index changes whenever signals change their state
    pub myPhaseIndex: i32,
    /// whether the signal is in moving block mode (only protects from oncoming and flanking trains)
    pub myMovingBlock: bool,
    /// map from tripId to constraint list
    pub myConstraints: BTreeMap<String, Vec<Ptr<MSRailSignalConstraint>>>,
    /// running number of driveways created for this signal
    pub myDriveWayIndex: i32,
}

impl std::ops::Deref for MSRailSignal {
    type Target = MSTrafficLightLogic;
    fn deref(&self) -> &MSTrafficLightLogic {
        &self.base
    }
}
impl std::ops::DerefMut for MSRailSignal {
    fn deref_mut(&mut self) -> &mut MSTrafficLightLogic {
        &mut self.base
    }
}

// C++ static members of MSRailSignal (port as module-level statics, see AGENTS.md):
//   static int MSRailSignal::myRSIndex;
//   static bool MSRailSignal::myStoreVehicles;
//   static VehicleVector MSRailSignal::myBlockingVehicles;
//   static VehicleVector MSRailSignal::myRivalVehicles;
//   static VehicleVector MSRailSignal::myPriorityVehicles;
//   static std::string MSRailSignal::myConstraintInfo;
//   static std::vector<const MSDriveWay *> MSRailSignal::myBlockingDriveWays;
//   static std::string MSRailSignal::myRequestedDriveWay;

impl MSRailSignal {
    /// C++: `void MSRailSignal::MSRailSignal(MSTLLogicControl &, const std::string &, const std::string &, SUMOTime, const Parameterised::Map &)`
    /// Constructor
    // MSRailSignal::MSRailSignal(MSTLLogicControl& tlcontrol,
    //                            const std::string& id, const std::string& programID, SUMOTime delay,
    //                            const Parameterised::Map& parameters) :
    //     MSTrafficLightLogic(tlcontrol, id, programID, 0, TrafficLightType::RAIL_SIGNAL, delay, parameters),
    //     myNumericalID(myRSIndex++),
    //     myCurrentPhase(DELTA_T, std::string(SUMO_MAX_CONNECTIONS, 'X')), // dummy phase
    //     myPhaseIndex(0),
    //     myDriveWayIndex(0) {
    //     myDefaultCycleTime = DELTA_T;
    //     myMovingBlock = OptionsCont::getOptions().getBool("railsignal-moving-block");
    //     mySwitchCommand->deschedule(this);
    // }
    #[allow(unused_variables)]
    pub fn new(tlcontrol: &mut MSTLLogicControl, id: &str, programID: &str, delay: SUMOTime, parameters: &Unmapped /* Parameterised::Map */) -> Self {
        todo!("port MSRailSignal::MSRailSignal")
    }

    /// C++: `void MSRailSignal::init(NLDetectorBuilder &)` [virtual]
    /// Initialises the rail signal with information about adjacent rail signals
    // void
    // MSRailSignal::init(NLDetectorBuilder&) {
    //     if (myLanes.size() == 0) {
    //         WRITE_WARNINGF(TL("Rail signal at junction '%' does not control any links"), getID());
    //     }
    //     SVCPermissions outgoingPermissions = 0;
    //     for (LinkVector& links : myLinks) { //for every link index
    //         if (links.size() != 1) {
    //             throw ProcessError("At railSignal '" + getID() + "' found " + toString(links.size())
    //                                + " links controlled by index " + toString(links[0]->getTLIndex()));
    //         }
    //         myLinkInfos.push_back(LinkInfo(links[0]));
    //         outgoingPermissions |= links[0]->getPermissions();
    //     }
    //     updateCurrentPhase();
    //     setTrafficLightSignals(MSNet::getInstance()->getCurrentTimeStep());
    //     myNumLinks = (int)myLinks.size();
    //     MSRailSignalControl::getInstance().addSignal(this);
    //     myMovingBlock |= MSRailSignalControl::isMovingBlock(outgoingPermissions);
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, nb: &mut NLDetectorBuilder) {
        todo!("port MSRailSignal::init")
    }

    /// C++: `void MSRailSignal::~MSRailSignal() noexcept` [virtual]
    /// Destructor
    // MSRailSignal::~MSRailSignal() {
    //     removeConstraints();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSRailSignal::~MSRailSignal")
    }

    /// C++: `int MSRailSignal::getNumericalID() const` [const]
    // inline int getNumericalID() const {
    //         return myNumericalID;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        todo!("port MSRailSignal::getNumericalID")
    }

    /// C++: `void MSRailSignal::setParameter(const std::string &, const std::string &)` [virtual]
    /// Sets a parameter and updates internal constants
    // void
    // MSRailSignal::setParameter(const std::string& key, const std::string& value) {
    //     // some pre-defined parameters can be updated at runtime
    //     if (key == "moving-block") {
    //         bool movingBlock = StringUtils::toBool(value);
    //         if (movingBlock != myMovingBlock) {
    //             // recompute driveways
    //             myMovingBlock = movingBlock;
    //             for (LinkInfo& li : myLinkInfos) {
    //                 li.reset();
    //             }
    //             updateCurrentPhase();
    //             setTrafficLightSignals(MSNet::getInstance()->getCurrentTimeStep());
    //         }
    //     }
    //     Parameterised::setParameter(key, value);
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        todo!("port MSRailSignal::setParameter")
    }

    /// C++: `void MSRailSignal::addLink(MSLink *, MSLane *, int)` [virtual]
    /// Adds a link on building
    // void
    // MSRailSignal::addLink(MSLink* link, MSLane* lane, int pos) {
    //     if (pos >= 0) {
    //         MSTrafficLightLogic::addLink(link, lane, pos);
    //     } // ignore uncontrolled link
    // }
    #[allow(unused_variables)]
    pub fn add_link(&mut self, link: Ptr<MSLink>, lane: Ptr<MSLane>, pos: i32) {
        todo!("port MSRailSignal::addLink")
    }

    /// C++: `void MSRailSignal::adaptLinkInformationFrom(const MSTrafficLightLogic &)` [virtual]
    /// Applies information about controlled links and lanes from the given logic
    // void
    // MSRailSignal::adaptLinkInformationFrom(const MSTrafficLightLogic& logic) {
    //     MSTrafficLightLogic::adaptLinkInformationFrom(logic);
    //     updateCurrentPhase();
    // }
    #[allow(unused_variables)]
    pub fn adapt_link_information_from(&mut self, logic: &MSTrafficLightLogic) {
        todo!("port MSRailSignal::adaptLinkInformationFrom")
    }

    /// C++: `bool MSRailSignal::updateCurrentPhase()`
    /// updates the current phase of the signal and return whether it should be kept active
    // bool
    // MSRailSignal::updateCurrentPhase() {
    // #ifdef DEBUG_SIGNALSTATE
    //     gDebugFlag4 = DEBUG_COND;
    // #endif
    //     bool keepActive = false;
    //     // green by default so vehicles can be inserted at the borders of the network
    //     std::string state(myLinks.size(), 'G');
    //     for (LinkInfo& li : myLinkInfos) {
    //         if (li.myLink->getApproaching().size() > 0 && li.myControlled) {
    //             keepActive = true;
    //             Approaching closest = li.myLink->getClosest();
    //             MSDriveWay& driveway = li.getDriveWay(closest.first);
    //             //std::cout << SIMTIME << " signal=" << getTLLinkID(li.myLink) << " veh=" << closest.first->getID() << " dw:\n";
    //             //driveway.writeBlocks(*OutputDevice_COUT::getDevice());
    //             const bool mustWait = !constraintsAllow(closest.first, true);
    //             MSEdgeVector occupied;
    //             if (mustWait || !driveway.reserve(closest, occupied)) {
    //                 state[li.myLink->getTLIndex()] = 'r';
    //                 if (occupied.size() > 0) {
    //                     li.reroute(const_cast<SUMOVehicle*>(closest.first), occupied);
    //                 }
    // #ifdef DEBUG_SIGNALSTATE
    //                 if (gDebugFlag4) {
    //                     std::cout << SIMTIME << " rsl=" << li.getID() << " veh=" << closest.first->getID() << " notReserved\n";
    //                 }
    // #endif
    //             } else {
    //                 state[li.myLink->getTLIndex()] = 'G';
    // #ifdef DEBUG_SIGNALSTATE
    //                 if (gDebugFlag4) {
    //                     std::cout << SIMTIME << " rsl=" << li.getID() << " veh=" << closest.first->getID() << " reserved\n";
    //                 }
    // #endif
    //             }
    //         } else if (li.myControlled) {
    //             if (li.myDriveways.empty()) {
    // #ifdef DEBUG_SIGNALSTATE
    //                 if (gDebugFlag4) {
    //                     std::cout << SIMTIME << " rsl=" << li.getID() << " red for unitialized signal (no driveways yet)\n";
    //                 }
    // #endif
    //                 state[li.myLink->getTLIndex()] = 'r';
    //             } else {
    //                 const MSDriveWay& driveway = *li.myDriveways.front();
    //                 MSEdgeVector occupied;
    //                 if (driveway.foeDriveWayOccupied(true, nullptr, occupied) || driveway.foeDriveWayApproached()) {
    //                     keepActive = true;
    // #ifdef DEBUG_SIGNALSTATE
    //                     if (gDebugFlag4) {
    //                         std::cout << SIMTIME << " rsl=" << li.getID() << " red for default driveway " << driveway.getID() << "\n";
    //                     }
    // #endif
    //                     state[li.myLink->getTLIndex()] = 'r';
    //                 } else {
    // #ifdef DEBUG_SIGNALSTATE
    //                     if (gDebugFlag4) {
    //                         std::cout << SIMTIME << " rsl=" << li.getID() << " green for default driveway " << driveway.getID() << "\n";
    //                     }
    // #endif
    //                 }
    //             }
    //         } else {
    //             state[li.myLink->getTLIndex()] = 'O';
    //         }
    //     }
    //     if (myCurrentPhase.getState() != state) {
    //         myCurrentPhase.setState(state);
    //         myPhaseIndex = 1 - myPhaseIndex;
    //         // set link priorities
    //         setTrafficLightSignals(SIMSTEP);
    //         // execute switch actions (3D-gui)
    //         //const MSTLLogicControl::TLSLogicVariants& vars = myTLControl.get(myTLLogic->getID());
    //         //vars.executeOnSwitchActions();
    //     }
    // #ifdef DEBUG_SIGNALSTATE
    //     gDebugFlag4 = false;
    // #endif
    //     return keepActive;
    // }
    #[allow(unused_variables)]
    pub fn update_current_phase(&mut self) -> bool {
        todo!("port MSRailSignal::updateCurrentPhase")
    }

    /// C++: `SUMOTime MSRailSignal::trySwitch()` [virtual]
    /// Switches to the next phase
    // SUMOTime
    // MSRailSignal::trySwitch() {
    //     // deschedule regular traffic light event,
    //     // updateCurrentPhase is instead called from MSRailSignalControl::updateSignals
    //     return SUMOTime_MAX;
    // }
    #[allow(unused_variables)]
    pub fn try_switch(&mut self) -> SUMOTime {
        todo!("port MSRailSignal::trySwitch")
    }

    /// C++: `int MSRailSignal::getPhaseNumber() const` [const, virtual]
    /// Returns the number of phases
    // int
    // MSRailSignal::getPhaseNumber() const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_phase_number(&self) -> i32 {
        todo!("port MSRailSignal::getPhaseNumber")
    }

    /// C++: `const Phases & MSRailSignal::getPhases() const` [const, virtual]
    /// Returns the phases of this tls program
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_phases(&self) -> &MSTrafficLightLogic_Phases {
        todo!("port MSRailSignal::getPhases")
    }

    /// C++: `const MSPhaseDefinition & MSRailSignal::getPhase(int) const` [const, virtual]
    /// Returns the definition of the phase from the given position within the plan
    // const MSPhaseDefinition&
    // MSRailSignal::getPhase(int) const {
    //     return myCurrentPhase;
    // }
    #[allow(unused_variables)]
    pub fn get_phase(&self, givenstep: i32) -> &MSPhaseDefinition {
        todo!("port MSRailSignal::getPhase")
    }

    /// C++: `bool MSRailSignal::getsMajorGreen(int) const` [const, virtual]
    /// whether the given link index ever turns 'G'
    // bool getsMajorGreen(int /*linkIndex*/) const override {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn gets_major_green(&self, arg0: i32) -> bool {
        todo!("port MSRailSignal::getsMajorGreen")
    }

    /// C++: `int MSRailSignal::getCurrentPhaseIndex() const` [const, virtual]
    /// Returns the current index within the program
    // int
    // MSRailSignal::getCurrentPhaseIndex() const {
    //     return myPhaseIndex;
    // }
    #[allow(unused_variables)]
    pub fn get_current_phase_index(&self) -> i32 {
        todo!("port MSRailSignal::getCurrentPhaseIndex")
    }

    /// C++: `const MSPhaseDefinition & MSRailSignal::getCurrentPhaseDef() const` [const, virtual]
    /// Returns the definition of the current phase
    // const MSPhaseDefinition&
    // MSRailSignal::getCurrentPhaseDef() const {
    //     return myCurrentPhase;
    // }
    #[allow(unused_variables)]
    pub fn get_current_phase_def(&self) -> &MSPhaseDefinition {
        todo!("port MSRailSignal::getCurrentPhaseDef")
    }

    /// C++: `void MSRailSignal::resetLastSwitch(SUMOTime)` [virtual]
    /// @}
    // void
    // MSRailSignal::resetLastSwitch(SUMOTime t) {
    //     myCurrentPhase.myLastSwitch = t;
    // }
    #[allow(unused_variables)]
    pub fn reset_last_switch(&mut self, t: SUMOTime) {
        todo!("port MSRailSignal::resetLastSwitch")
    }

    /// C++: `SUMOTime MSRailSignal::getPhaseIndexAtTime(SUMOTime) const` [const, virtual]
    /// Returns the index of the logic at the given simulation step
    // SUMOTime
    // MSRailSignal::getPhaseIndexAtTime(SUMOTime) const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_phase_index_at_time(&self, simStep: SUMOTime) -> SUMOTime {
        todo!("port MSRailSignal::getPhaseIndexAtTime")
    }

    /// C++: `SUMOTime MSRailSignal::getOffsetFromIndex(int) const` [const, virtual]
    /// Returns the position (start of a phase during a cycle) from of a given step
    // SUMOTime
    // MSRailSignal::getOffsetFromIndex(int) const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_offset_from_index(&self, index: i32) -> SUMOTime {
        todo!("port MSRailSignal::getOffsetFromIndex")
    }

    /// C++: `int MSRailSignal::getIndexFromOffset(SUMOTime) const` [const, virtual]
    /// Returns the step (the phasenumber) of a given position of the cycle
    // int
    // MSRailSignal::getIndexFromOffset(SUMOTime) const {
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_index_from_offset(&self, offset: SUMOTime) -> i32 {
        todo!("port MSRailSignal::getIndexFromOffset")
    }

    /// C++: `void MSRailSignal::changeStepAndDuration(MSTLLogicControl &, SUMOTime, int, SUMOTime)` [virtual]
    /// Changes the current phase and her duration
    // void changeStepAndDuration(MSTLLogicControl& tlcontrol, SUMOTime simStep, int step, SUMOTime stepDuration) override {
    //         UNUSED_PARAMETER(tlcontrol);
    //         UNUSED_PARAMETER(simStep);
    //         UNUSED_PARAMETER(step);
    //         UNUSED_PARAMETER(stepDuration);
    //     }
    #[allow(unused_variables)]
    pub fn change_step_and_duration(&mut self, tlcontrol: &mut MSTLLogicControl, simStep: SUMOTime, step: i32, stepDuration: SUMOTime) {
        todo!("port MSRailSignal::changeStepAndDuration")
    }

    /// C++: `VehicleVector MSRailSignal::getBlockingVehicles(int)` [virtual]
    /// return vehicles that block the intersection/rail signal for vehicles that wish to pass the given linkIndex
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_blocking_vehicles(&mut self, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
        todo!("port MSRailSignal::getBlockingVehicles")
    }

    /// C++: `std::string MSRailSignal::getBlockingVehicleIDs() const` [const]
    // std::string
    // MSRailSignal::getBlockingVehicleIDs() const {
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(this);
    //     if (myLinkInfos.size() == 1) {
    //         return toString(rs->getBlockingVehicles(0));
    //     } else {
    //         std::string result;
    //         for (int i = 0; i < (int)myLinkInfos.size(); i++) {
    //             result += toString(i) + ": " + toString(rs->getBlockingVehicles(i)) + ";";
    //         }
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_blocking_vehicle_i_ds(&self) -> String {
        todo!("port MSRailSignal::getBlockingVehicleIDs")
    }

    /// C++: `VehicleVector MSRailSignal::getBlockingVehicles(const MSDriveWay *)` [static]
    /// return vehicles that block the given driveWay
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_blocking_vehicles_2(dw: Ptr<MSDriveWay>) -> MSVehicleContainer_VehicleVector {
        todo!("port MSRailSignal::getBlockingVehicles")
    }

    /// C++: `VehicleVector MSRailSignal::getRivalVehicles(int)` [virtual]
    /// return vehicles that approach the intersection/rail signal and are in conflict with vehicles that wish to pass the given linkIndex
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_rival_vehicles(&mut self, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
        todo!("port MSRailSignal::getRivalVehicles")
    }

    /// C++: `std::string MSRailSignal::getRivalVehicleIDs() const` [const]
    // std::string
    // MSRailSignal::getRivalVehicleIDs() const {
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(this);
    //     if (myLinkInfos.size() == 1) {
    //         return toString(rs->getRivalVehicles(0));
    //     } else {
    //         std::string result;
    //         for (int i = 0; i < (int)myLinkInfos.size(); i++) {
    //             result += toString(i) + ": " + toString(rs->getRivalVehicles(i)) + ";";
    //         }
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_rival_vehicle_i_ds(&self) -> String {
        todo!("port MSRailSignal::getRivalVehicleIDs")
    }

    /// C++: `VehicleVector MSRailSignal::getPriorityVehicles(int)` [virtual]
    /// return vehicles that approach the intersection/rail signal and have priority over vehicles that wish to pass the given linkIndex
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_priority_vehicles(&mut self, linkIndex: i32) -> MSVehicleContainer_VehicleVector {
        todo!("port MSRailSignal::getPriorityVehicles")
    }

    /// C++: `std::string MSRailSignal::getPriorityVehicleIDs() const` [const]
    // std::string
    // MSRailSignal::getPriorityVehicleIDs() const {
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(this);
    //     if (myLinkInfos.size() == 1) {
    //         return toString(rs->getPriorityVehicles(0));
    //     } else {
    //         std::string result;
    //         for (int i = 0; i < (int)myLinkInfos.size(); i++) {
    //             result += toString(i) + ": " + toString(rs->getPriorityVehicles(i)) + ";";
    //         }
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_priority_vehicle_i_ds(&self) -> String {
        todo!("port MSRailSignal::getPriorityVehicleIDs")
    }

    /// C++: `std::string MSRailSignal::getConstraintInfo(int)`
    /// return information regarding active rail signal constraints for the closest approaching vehicle
    // std::string
    // MSRailSignal::getConstraintInfo(int linkIndex) {
    //     storeTraCIVehicles(linkIndex);
    //     return myConstraintInfo;
    // }
    #[allow(unused_variables)]
    pub fn get_constraint_info(&mut self, linkIndex: i32) -> String {
        todo!("port MSRailSignal::getConstraintInfo")
    }

    /// C++: `std::string MSRailSignal::getConstraintInfo() const` [const]
    // std::string
    // MSRailSignal::getConstraintInfo() const {
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(this);
    //     if (myLinkInfos.size() == 1) {
    //         return rs->getConstraintInfo(0);
    //     } else {
    //         std::string result;
    //         for (int i = 0; i < (int)myLinkInfos.size(); i++) {
    //             result += toString(i) + ": " + rs->getConstraintInfo(i);
    //         }
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_constraint_info_2(&self) -> String {
        todo!("port MSRailSignal::getConstraintInfo")
    }

    /// C++: `std::vector<const MSDriveWay *> MSRailSignal::getBlockingDriveWays(int)` [virtual]
    // std::vector<const MSDriveWay*>
    // MSRailSignal::getBlockingDriveWays(int linkIndex) {
    //     storeTraCIVehicles(linkIndex);
    //     return myBlockingDriveWays;
    // }
    #[allow(unused_variables)]
    pub fn get_blocking_drive_ways(&mut self, linkIndex: i32) -> Vec<Ptr<MSDriveWay>> {
        todo!("port MSRailSignal::getBlockingDriveWays")
    }

    /// C++: `std::string MSRailSignal::getBlockingDriveWayIDs() const` [const]
    // std::string
    // MSRailSignal::getBlockingDriveWayIDs() const {
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(this);
    //     if (myLinkInfos.size() == 1) {
    //         return toString(rs->getBlockingDriveWays(0));
    //     } else {
    //         std::string result;
    //         for (int i = 0; i < (int)myLinkInfos.size(); i++) {
    //             result += toString(i) + ": " + toString(rs->getBlockingDriveWays(i)) + ";";
    //         }
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_blocking_drive_way_i_ds(&self) -> String {
        todo!("port MSRailSignal::getBlockingDriveWayIDs")
    }

    /// C++: `std::vector<const MSDriveWay *> MSRailSignal::getBlockingDriveWays(const MSDriveWay *)` [static]
    /// return driveways that block the given driveWay
    // std::vector<const MSDriveWay*>
    // MSRailSignal::getBlockingDriveWays(const MSDriveWay* dw) {
    //     storeTraCIVehicles(dw);
    //     return myBlockingDriveWays;
    // }
    #[allow(unused_variables)]
    pub fn get_blocking_drive_ways_2(dw: Ptr<MSDriveWay>) -> Vec<Ptr<MSDriveWay>> {
        todo!("port MSRailSignal::getBlockingDriveWays")
    }

    /// C++: `std::string MSRailSignal::getRequestedDriveWay(int)` [virtual]
    // std::string
    // MSRailSignal::getRequestedDriveWay(int linkIndex) {
    //     storeTraCIVehicles(linkIndex);
    //     return myRequestedDriveWay;
    // }
    #[allow(unused_variables)]
    pub fn get_requested_drive_way(&mut self, linkIndex: i32) -> String {
        todo!("port MSRailSignal::getRequestedDriveWay")
    }

    /// C++: `std::string MSRailSignal::getRequestedDriveWay() const` [const]
    // std::string
    // MSRailSignal::getRequestedDriveWay() const {
    //     MSRailSignal* rs = const_cast<MSRailSignal*>(this);
    //     if (myLinkInfos.size() == 1) {
    //         return toString(rs->getRequestedDriveWay(0));
    //     } else {
    //         std::string result;
    //         for (int i = 0; i < (int)myLinkInfos.size(); i++) {
    //             result += toString(i) + ": " + toString(rs->getRequestedDriveWay(i)) + ";";
    //         }
    //         return result;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_requested_drive_way_2(&self) -> String {
        todo!("port MSRailSignal::getRequestedDriveWay")
    }

    /// C++: `void MSRailSignal::writeBlocks(OutputDevice &, bool) const` [const]
    /// write rail signal block output for all links and driveways
    // void
    // MSRailSignal::writeBlocks(OutputDevice& od, bool writeVehicles) const {
    //     od.openTag("railSignal");
    //     od.writeAttr(SUMO_ATTR_ID, getID());
    //     for (const LinkInfo& li : myLinkInfos) {
    //         MSLink* link = li.myLink;
    //         od.openTag("link");
    //         od.writeAttr(SUMO_ATTR_TLLINKINDEX, link->getTLIndex());
    //         od.writeAttr(SUMO_ATTR_FROM, link->getLaneBefore()->getID());
    //         od.writeAttr(SUMO_ATTR_TO, link->getViaLaneOrLane()->getID());
    //         for (const MSDriveWay* dw : li.myDriveways) {
    //             if (writeVehicles) {
    //                 dw->writeBlockVehicles(od);
    //             } else {
    //                 dw->writeBlocks(od);
    //             }
    //         }
    //         od.closeTag(); // link
    //     }
    //     od.closeTag(); // railSignal
    // }
    #[allow(unused_variables)]
    pub fn write_blocks(&self, od: &mut OutputDevice, writeVehicles: bool) {
        todo!("port MSRailSignal::writeBlocks")
    }

    /// C++: `void MSRailSignal::addConstraint(const std::string &, MSRailSignalConstraint *)`
    /// register constraint for signal switching
    // void
    // MSRailSignal::addConstraint(const std::string& tripId, MSRailSignalConstraint* constraint) {
    //     myConstraints[tripId].push_back(constraint);
    // }
    #[allow(unused_variables)]
    pub fn add_constraint(&mut self, tripId: &str, constraint: Ptr<MSRailSignalConstraint>) {
        todo!("port MSRailSignal::addConstraint")
    }

    /// C++: `const std::map<std::string, std::vector<MSRailSignalConstraint *>> & MSRailSignal::getConstraints() const` [const]
    /// @{
    // const std::map<std::string, std::vector<MSRailSignalConstraint*> >&  getConstraints() const {
    //         return myConstraints;
    //     }
    #[allow(unused_variables)]
    pub fn get_constraints(&self) -> &BTreeMap<String, Vec<Ptr<MSRailSignalConstraint>>> {
        todo!("port MSRailSignal::getConstraints")
    }

    /// C++: `bool MSRailSignal::removeConstraint(const std::string &, MSRailSignalConstraint *)`
    /// remove constraint for signal switching
    // bool
    // MSRailSignal::removeConstraint(const std::string& tripId, MSRailSignalConstraint* constraint) {
    //     if (myConstraints.count(tripId) != 0) {
    //         auto& constraints = myConstraints[tripId];
    //         auto it = std::find(constraints.begin(), constraints.end(), constraint);
    //         if (it != constraints.end()) {
    //             delete *it;
    //             constraints.erase(it);
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn remove_constraint(&mut self, tripId: &str, constraint: Ptr<MSRailSignalConstraint>) -> bool {
        todo!("port MSRailSignal::removeConstraint")
    }

    /// C++: `void MSRailSignal::removeConstraints()`
    // void
    // MSRailSignal::removeConstraints() {
    //     for (auto item : myConstraints) {
    //         for (MSRailSignalConstraint* c : item.second) {
    //             delete c;
    //         }
    //     }
    //     myConstraints.clear();
    // }
    #[allow(unused_variables)]
    pub fn remove_constraints(&mut self) {
        todo!("port MSRailSignal::removeConstraints")
    }

    /// C++: `void MSRailSignal::updateDriveway(int)`
    /// update driveway for extended deadlock protection
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn update_driveway(&mut self, numericalID: i32) {
        todo!("port MSRailSignal::updateDriveway")
    }

    /// C++: `bool MSRailSignal::hasInsertionConstraint(MSLink *, const MSVehicle *, std::string &, bool &)` [static]
    // bool
    // MSRailSignal::hasInsertionConstraint(MSLink* link, const MSVehicle* veh, std::string& info, bool& isInsertionOrder) {
    //     if (link->getJunction() != nullptr && link->getJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //         const MSRailSignal* rs = dynamic_cast<const MSRailSignal*>(link->getTLLogic());
    //         if (rs != nullptr && rs->myConstraints.size() > 0) {
    //             const std::string tripID = veh->getParameter().getParameter("tripId", veh->getID());
    //             auto it = rs->myConstraints.find(tripID);
    //             if (it != rs->myConstraints.end()) {
    //                 for (MSRailSignalConstraint* c : it->second) {
    //                     if (c->isInsertionConstraint() && !c->cleared()) {
    // #ifdef DEBUG_SIGNALSTATE
    //                         if (DEBUG_HELPER(rs)) {
    //                             std::cout << SIMTIME << " rsl=" << rs->getID() << " insertion constraint '" << c->getDescription() << "' for vehicle '" << veh->getID() << "' not cleared\n";
    //                         }
    // #endif
    //                         info = c->getDescription();
    //                         isInsertionOrder = c->getType() == MSRailSignalConstraint::ConstraintType::INSERTION_ORDER;
    //                         if (MSGlobals::gTimeToTeleportRSDeadlock > 0) {
    //                             const SUMOVehicle* foe = c->getFoe();
    //                             if (foe != nullptr) {
    //                                 MSRailSignalControl::getInstance().addWaitRelation(veh, rs, foe, c);
    //                             }
    //                         }
    //                         return true;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_insertion_constraint(link: Ptr<MSLink>, veh: Ptr<MSVehicle>, info: &mut String, isInsertionOrder: &mut bool) -> bool {
        todo!("port MSRailSignal::hasInsertionConstraint")
    }

    /// C++: `void MSRailSignal::initDriveWays(const SUMOVehicle *, bool)` [static]
    // void
    // MSRailSignal::initDriveWays(const SUMOVehicle* ego, bool update) {
    //     const ConstMSEdgeVector& edges = ego->getRoute().getEdges();
    //     int endIndex = ego->getParameter().arrivalEdge;
    //     if (endIndex < 0) {
    //         endIndex = (int)edges.size() - 1;
    //     }
    //     int departIndex = ego->getParameter().departEdge;
    //     MSDriveWay* prev = nullptr;
    //     if (update && ego->hasDeparted()) {
    //         // find last rail signal on the route and obtain the driveway
    //         const MSEdge* next = ego->getEdge();
    //         for (int i = ego->getRoutePosition() - 1; i > departIndex; i--) {
    //             const MSEdge* e = ego->getRoute().getEdges()[i];
    //             if (e->getToJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //                 const MSLink* link = e->getLanes().front()->getLinkTo(next->getLanes().front());
    //                 //std::cout << SIMTIME << " veh=" << ego->getID() << " rp=" << ego->getRoutePosition()
    //                 //    << " i=" << i << " e=" << e->getID() << " next=" << next->getID() << " link=" << (link == nullptr ? "NUL" : link->getDescription()) << "\n";
    //                 if (link != nullptr && link->isTLSControlled()) {
    //                     MSRailSignal* rs = const_cast<MSRailSignal*>(dynamic_cast<const MSRailSignal*>(link->getTLLogic()));
    //                     LinkInfo& li = rs->myLinkInfos[link->getTLIndex()];
    //                     prev = &li.getDriveWay(ego, i);
    //                     departIndex = ego->getRoutePosition();
    //                     break;
    //                 }
    //             }
    //             next = e;
    //         }
    //     }
    //     if (prev == nullptr) {
    //         prev = const_cast<MSDriveWay*>(MSDriveWay::getDepartureDriveway(ego, true));
    //     }
    //     if (update && ego->hasDeparted()) {
    //         MSBaseVehicle* veh = dynamic_cast<MSBaseVehicle*>(const_cast<SUMOVehicle*>(ego));
    //         if (!prev->hasTrain(veh) && prev->notifyEnter(*veh, prev->NOTIFICATION_REROUTE, nullptr) && !veh->hasReminder(prev)) {
    //             veh->addReminder(prev, 1);
    //         }
    //     }
    //     for (int i = departIndex; i <= endIndex - 1; i++) {
    //         const MSEdge* e = edges[i];
    //         if (e->isNormal() && e->getToJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //             const MSEdge* e2 = edges[i + 1];
    //             for (MSLane* lane : e->getLanes()) {
    //                 for (MSLink* link : lane->getLinkCont()) {
    //                     if (&link->getLane()->getEdge() == e2) {
    //                         MSRailSignal* rs = const_cast<MSRailSignal*>(dynamic_cast<const MSRailSignal*>(link->getTLLogic()));
    //                         if (rs != nullptr) {
    //                             LinkInfo& li = rs->myLinkInfos[link->getTLIndex()];
    //                             // init driveway
    //                             MSDriveWay* dw = &li.getDriveWay(ego, i);
    //                             MSRailSignalControl::getInstance().addDrivewayFollower(prev, dw);
    //                             MSRailSignalControl::getInstance().addDWDeadlockChecks(rs, prev);
    //                             MSRailSignalControl::getInstance().notifyApproach(link);
    //                             prev = dw;
    //                             if (update && rs->isActive()) {
    //                                 // vehicle may have rerouted its intial trip
    //                                 // after the states have been set
    //                                 // @note: This is a hack because it could lead to invalid tls-output
    //                                 // (it's still an improvement over switching based on default driveways)
    //                                 if (!ego->hasDeparted()) {
    //                                     rs->updateCurrentPhase();
    //                                     rs->setTrafficLightSignals(SIMSTEP);
    //                                 } else if (ego->hasDeparted() && i <= ego->getRoutePosition()) {
    //                                     MSBaseVehicle* veh = dynamic_cast<MSBaseVehicle*>(const_cast<SUMOVehicle*>(ego));
    //                                     if (!dw->hasTrain(veh) && dw->notifyEnter(*veh, dw->NOTIFICATION_REROUTE, nullptr) && !veh->hasReminder(dw)) {
    //                                         veh->addReminder(dw, 1);
    //                                         for (MSDriveWay* sub : dw->getSubDriveWays()) {
    //                                             if (!sub->hasTrain(veh) && sub->notifyEnter(*veh, dw->NOTIFICATION_REROUTE, nullptr) && !veh->hasReminder(sub)) {
    //                                                 veh->addReminder(sub, 1);
    //                                             }
    //                                         }
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_drive_ways(ego: Ptr<SUMOVehicle>, update: bool) {
        todo!("port MSRailSignal::initDriveWays")
    }

    /// C++: `const MSDriveWay & MSRailSignal::retrieveDriveWay(int) const` [const]
    // const MSDriveWay&
    // MSRailSignal::retrieveDriveWay(int numericalID) const {
    //     for (const LinkInfo& li : myLinkInfos) {
    //         for (const MSDriveWay* dw : li.myDriveways) {
    //             if (dw->getNumericalID() == numericalID) {
    //                 return *dw;
    //             }
    //         }
    //     }
    //     throw ProcessError("Invalid driveway id " + toString(numericalID) + " at railSignal '" + getID() + "'");
    // }
    #[allow(unused_variables)]
    pub fn retrieve_drive_way(&self, numericalID: i32) -> &MSDriveWay {
        todo!("port MSRailSignal::retrieveDriveWay")
    }

    /// C++: `const MSDriveWay & MSRailSignal::retrieveDriveWayForVeh(int, const SUMOVehicle *)`
    // const MSDriveWay&
    // MSRailSignal::retrieveDriveWayForVeh(int tlIndex, const SUMOVehicle* veh) {
    //     return myLinkInfos[tlIndex].getDriveWay(veh);
    // }
    #[allow(unused_variables)]
    pub fn retrieve_drive_way_for_veh(&mut self, tlIndex: i32, veh: Ptr<SUMOVehicle>) -> &MSDriveWay {
        todo!("port MSRailSignal::retrieveDriveWayForVeh")
    }

    /// C++: `const MSDriveWay & MSRailSignal::retrieveDriveWayForRoute(int, MSRouteIterator, MSRouteIterator)`
    // const MSDriveWay&
    // MSRailSignal::retrieveDriveWayForRoute(int tlIndex, MSRouteIterator first, MSRouteIterator end) {
    //     return myLinkInfos[tlIndex].getDriveWay(first, end);
    // }
    #[allow(unused_variables)]
    pub fn retrieve_drive_way_for_route(&mut self, tlIndex: i32, first: MSRouteIterator, end: MSRouteIterator) -> &MSDriveWay {
        todo!("port MSRailSignal::retrieveDriveWayForRoute")
    }

    /// C++: `const std::vector<MSDriveWay *> MSRailSignal::retrieveDriveWays(int) const` [const]
    // const std::vector<MSDriveWay*>
    // MSRailSignal::retrieveDriveWays(int tlIndex) const {
    //     return myLinkInfos[tlIndex].myDriveways;
    // }
    #[allow(unused_variables)]
    pub fn retrieve_drive_ways(&self, tlIndex: i32) -> Vec<Ptr<MSDriveWay>> {
        todo!("port MSRailSignal::retrieveDriveWays")
    }

    /// C++: `bool MSRailSignal::constraintsAllow(const SUMOVehicle *, bool) const` [const]
    /// whether the given vehicle is free to drive
    // bool
    // MSRailSignal::constraintsAllow(const SUMOVehicle* veh, bool storeWaitRelation) const {
    //     if (myConstraints.size() == 0) {
    //         return true;
    //     } else {
    //         const std::string tripID = veh->getParameter().getParameter("tripId", veh->getID());
    //         auto it = myConstraints.find(tripID);
    //         if (it != myConstraints.end()) {
    //             for (MSRailSignalConstraint* c : it->second) {
    //                 // ignore insertion constraints here
    //                 if (!c->isInsertionConstraint() && !c->cleared()) {
    // #ifdef DEBUG_SIGNALSTATE
    //                     if (gDebugFlag4) {
    //                         std::cout << "  constraint '" << c->getDescription() << "' not cleared\n";
    //                     }
    // #endif
    //                     if (storeWaitRelation && MSGlobals::gTimeToTeleportRSDeadlock > 0
    //                             && veh->getWaitingTime() > veh->getVehicleType().getCarFollowModel().getStartupDelay()) {
    //                         const SUMOVehicle* foe = c->getFoe();
    //                         if (foe != nullptr) {
    //                             MSRailSignalControl::getInstance().addWaitRelation(veh, this, foe, c);
    //                         }
    //                     }
    //                     if (myStoreVehicles) {
    //                         myConstraintInfo = c->getDescription();
    //                     }
    //                     return false;
    //                 }
    //             }
    //         }
    //         return true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn constraints_allow(&self, veh: Ptr<SUMOVehicle>, storeWaitRelation: bool) -> bool {
        todo!("port MSRailSignal::constraintsAllow")
    }

    /// C++: `bool MSRailSignal::isMovingBlock() const` [const]
    // bool isMovingBlock() const {
    //         return myMovingBlock;
    //     }
    #[allow(unused_variables)]
    pub fn is_moving_block(&self) -> bool {
        todo!("port MSRailSignal::isMovingBlock")
    }

    /// C++: `const MSJunction * MSRailSignal::getJunction() const` [const]
    /// get the unique junction controlled by this signal
    // const MSJunction*
    // MSRailSignal::getJunction() const {
    //     return myLinkInfos.front().myLink->getJunction();
    // }
    #[allow(unused_variables)]
    pub fn get_junction(&self) -> Ptr<MSJunction> {
        todo!("port MSRailSignal::getJunction")
    }

    /// C++: `const MSDriveWay & MSRailSignal::getExistingDriveWay(const MSLink *, MSRouteIterator, MSRouteIterator)`
    // const MSDriveWay&
    // MSRailSignal::getExistingDriveWay(const MSLink* link, MSRouteIterator firstIt, MSRouteIterator endIt) {
    //     LinkInfo& li = myLinkInfos[link->getTLIndex()];
    //     const MSDriveWay& cand = li.getDriveWay(firstIt, endIt, "_tmp_");
    //     // an existing driveway should be found
    //     assert(cand.getFirstVehicle() != "_tmp_");
    //     return cand;
    // }
    #[allow(unused_variables)]
    pub fn get_existing_drive_way(&mut self, link: Ptr<MSLink>, firstIt: MSRouteIterator, endIt: MSRouteIterator) -> &MSDriveWay {
        todo!("port MSRailSignal::getExistingDriveWay")
    }

    /// C++: `bool MSRailSignal::storeVehicles()` [static]
    // static bool storeVehicles() {
    //         return myStoreVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn store_vehicles() -> bool {
        todo!("port MSRailSignal::storeVehicles")
    }

    /// C++: `VehicleVector & MSRailSignal::blockingVehicles()` [static]
    // static VehicleVector& blockingVehicles() {
    //         return myBlockingVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn blocking_vehicles() -> &'static mut MSVehicleContainer_VehicleVector {
        todo!("port MSRailSignal::blockingVehicles")
    }

    /// C++: `std::vector<const MSDriveWay *> & MSRailSignal::blockingDriveWays()` [static]
    // static std::vector<const MSDriveWay*>& blockingDriveWays() {
    //         return myBlockingDriveWays;
    //     }
    #[allow(unused_variables)]
    pub fn blocking_drive_ways() -> &'static mut Vec<Ptr<MSDriveWay>> {
        todo!("port MSRailSignal::blockingDriveWays")
    }

    /// C++: `VehicleVector & MSRailSignal::rivalVehicles()` [static]
    // static VehicleVector& rivalVehicles() {
    //         return myRivalVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn rival_vehicles() -> &'static mut MSVehicleContainer_VehicleVector {
        todo!("port MSRailSignal::rivalVehicles")
    }

    /// C++: `VehicleVector & MSRailSignal::priorityVehicles()` [static]
    // static VehicleVector& priorityVehicles() {
    //         return myPriorityVehicles;
    //     }
    #[allow(unused_variables)]
    pub fn priority_vehicles() -> &'static mut MSVehicleContainer_VehicleVector {
        todo!("port MSRailSignal::priorityVehicles")
    }

    /// C++: `std::string MSRailSignal::describeLinks(std::vector<MSLink *>)` [static]
    /// print link descriptions
    // std::string
    // MSRailSignal::describeLinks(std::vector<MSLink*> links) {
    //     std::string result;
    //     for (MSLink* link : links) {
    //         result += link->getDescription() + " ";
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn describe_links(links: Vec<Ptr<MSLink>>) -> String {
        todo!("port MSRailSignal::describeLinks")
    }

    /// C++: `std::string MSRailSignal::getNewDrivewayID()` [protected]
    // std::string
    // MSRailSignal::getNewDrivewayID() {
    //     return getID() + "." + toString(myDriveWayIndex++);
    // }
    #[allow(unused_variables)]
    pub fn get_new_driveway_id(&mut self) -> String {
        todo!("port MSRailSignal::getNewDrivewayID")
    }

    /// C++: `void MSRailSignal::storeTraCIVehicles(int)` [protected]
    /// update vehicle lists for traci calls
    // void
    // MSRailSignal::storeTraCIVehicles(int linkIndex) {
    //     resetStored();
    //     myStoreVehicles = true;
    //     LinkInfo& li = myLinkInfos[linkIndex];
    //     if (li.myLink->getApproaching().size() > 0) {
    //         Approaching closest = li.myLink->getClosest();
    //         MSDriveWay& driveway = li.getDriveWay(closest.first);
    //         MSEdgeVector occupied;
    //         myRequestedDriveWay = driveway.getID();
    //         // call for side effects
    //         driveway.reserve(closest, occupied);
    //         constraintsAllow(closest.first);
    //     } else if (li.myDriveways.size() > 0) {
    //         li.myDriveways.front()->conflictLaneOccupied();
    //         li.myDriveways.front()->foeDriveWayApproached();
    //     }
    //     myStoreVehicles = false;
    // }
    #[allow(unused_variables)]
    pub fn store_tra_ci_vehicles(&mut self, linkIndex: i32) {
        todo!("port MSRailSignal::storeTraCIVehicles")
    }

    /// C++: `void MSRailSignal::storeTraCIVehicles(const MSDriveWay *)` [static, protected]
    // void
    // MSRailSignal::storeTraCIVehicles(const MSDriveWay* dw) {
    //     resetStored();
    //     myStoreVehicles = true;
    //     MSEdgeVector occupied;
    //     // call for side effects
    //     dw->foeDriveWayOccupied(true, nullptr, occupied);
    //     myStoreVehicles = false;
    // }
    #[allow(unused_variables)]
    pub fn store_tra_ci_vehicles_2(dw: Ptr<MSDriveWay>) {
        todo!("port MSRailSignal::storeTraCIVehicles")
    }

    /// C++: `void MSRailSignal::resetStored()` [static, protected]
    /// reset temporary storage for injected conflict output
    // void
    // MSRailSignal::resetStored() {
    //     myBlockingVehicles.clear();
    //     myRivalVehicles.clear();
    //     myPriorityVehicles.clear();
    //     myConstraintInfo = "";
    //     myBlockingDriveWays.clear();
    //     myRequestedDriveWay = "";
    // }
    #[allow(unused_variables)]
    pub fn reset_stored() {
        todo!("port MSRailSignal::resetStored")
    }

}

