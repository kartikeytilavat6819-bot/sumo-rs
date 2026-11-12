//! Port of microsim/traffic_lights/MSRailSignalControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// storage for loaded deadlocks
/// C++ class `MSRailSignalControl::Deadlock` (microsim/traffic_lights/MSRailSignalControl.h)
pub struct MSRailSignalControl_Deadlock {
    pub signals: Vec<Ptr<MSRailSignal>>,
    pub edges: Vec<ConstMSEdgeVector>,
    pub forwardEnds: ConstMSEdgeVector,
}

impl MSRailSignalControl_Deadlock {
}

/// A signal for rails
/// C++ class `MSRailSignalControl` (microsim/traffic_lights/MSRailSignalControl.h)
#[repr(C)]
pub struct MSRailSignalControl {
    /// C++ base class `MSNet::VehicleStateListener`
    pub base: MSNet_VehicleStateListener,
    /// all rail edges that are part of a known route
    pub myUsedEdges: BTreeSet<Ptr<MSEdge>>,
    pub myWaitRelations: BTreeMap<Ptr<SUMOVehicle>, MSRailSignalControl_WaitRelation>,
    pub myWrittenDeadlocks: BTreeSet<BTreeSet<Ptr<SUMOVehicle>>>,
    pub myLoadedDeadlocks: Vec<MSRailSignalControl_Deadlock>,
    pub mySignalDeadlocks: BTreeMap<Ptr<MSRailSignal>, Vec<(i32, i32)>>, // C++: std::map<const MSRailSignal *, std::vector<std::pair<int, int>>>
    pub myDriveWaySucc: BTreeMap<Ptr<MSDriveWay>, BTreeSet<Ptr<MSDriveWay>>>, // C++: std::map<const MSDriveWay *, std::set<const MSDriveWay *, ComparatorNumericalIdLess>>
    pub myDriveWayPred: BTreeMap<Ptr<MSDriveWay>, BTreeSet<Ptr<MSDriveWay>>>, // C++: std::map<const MSDriveWay *, std::set<const MSDriveWay *>>
    /// list of all rail signals
    pub mySignals: Vec<Ptr<MSRailSignal>>,
    /// list of signals that switched green along with driveway index
    pub mySwitchedGreenFlanks: Vec<(Ptr<MSLink>, i32)>,
    pub myDriveWayCompatibility: BTreeMap<(i32, i32), bool>, // C++: std::map<std::pair<int, int>, bool>
    pub myActiveSignals: BTreeSet<Ptr<MSRailSignal>>, // C++: std::set<MSRailSignal *, ComparatorNumericalIdLess>
    pub myMaxTrainLength: f64,
}

impl std::ops::Deref for MSRailSignalControl {
    type Target = MSNet_VehicleStateListener;
    fn deref(&self) -> &MSNet_VehicleStateListener {
        &self.base
    }
}
impl std::ops::DerefMut for MSRailSignalControl {
    fn deref_mut(&mut self) -> &mut MSNet_VehicleStateListener {
        &mut self.base
    }
}

// C++ static members of MSRailSignalControl (port as module-level statics, see AGENTS.md):
//   static SVCPermissions MSRailSignalControl::mySignalizedClasses;
//   static SVCPermissions MSRailSignalControl::myMBClasses;
//   static MSRailSignalControl * MSRailSignalControl::myInstance;

impl MSRailSignalControl {
    /// C++: `void MSRailSignalControl::~MSRailSignalControl() noexcept` [virtual]
    // MSRailSignalControl::~MSRailSignalControl() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSRailSignalControl::~MSRailSignalControl")
    }

    /// C++: `MSRailSignalControl & MSRailSignalControl::getInstance()` [static]
    // MSRailSignalControl&
    // MSRailSignalControl::getInstance() {
    //     if (myInstance == nullptr) {
    //         myInstance = new MSRailSignalControl();
    //         MSNet::getInstance()->addVehicleStateListener(myInstance);
    //     }
    //     return *myInstance;
    // }
    #[allow(unused_variables)]
    pub fn get_instance() -> &'static mut MSRailSignalControl {
        todo!("port MSRailSignalControl::getInstance")
    }

    /// C++: `bool MSRailSignalControl::hasInstance()` [static]
    // static bool hasInstance() {
    //         return myInstance != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn has_instance() -> bool {
        todo!("port MSRailSignalControl::hasInstance")
    }

    /// C++: `void MSRailSignalControl::cleanup()` [static]
    // void
    // MSRailSignalControl::cleanup() {
    //     delete myInstance;
    //     myInstance = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        todo!("port MSRailSignalControl::cleanup")
    }

    /// C++: `void MSRailSignalControl::clearState()` [static]
    /// Perform resets events when quick-loading state
    // void
    // MSRailSignalControl::clearState() {
    //     if (myInstance != nullptr) {
    //         myInstance->myDriveWayCompatibility.clear();
    //         myInstance->myDriveWaySucc.clear();
    //         myInstance->myDriveWayPred.clear();
    //         myInstance->myWrittenDeadlocks.clear();
    //         myInstance->mySignalDeadlocks.clear();
    //         myInstance->myLoadedDeadlocks.clear();
    //         //myInstance->myActiveSignals.clear();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_state() {
        todo!("port MSRailSignalControl::clearState")
    }

    /// C++: `void MSRailSignalControl::resetWaitRelations()`
    /// reset all waiting-for relationships at the start of the simulation step
    // void resetWaitRelations() {
    //         myWaitRelations.clear();
    //         myWrittenDeadlocks.clear();
    //     }
    #[allow(unused_variables)]
    pub fn reset_wait_relations(&mut self) {
        todo!("port MSRailSignalControl::resetWaitRelations")
    }

    /// C++: `void MSRailSignalControl::addWaitRelation(const SUMOVehicle *, const MSRailSignal *, const SUMOVehicle *, MSRailSignalConstraint *)`
    // void
    // MSRailSignalControl::addWaitRelation(const SUMOVehicle* waits, const MSRailSignal* rs, const SUMOVehicle* reason, MSRailSignalConstraint* constraint) {
    //     //std::cout << time2string(SIMSTEP) << " addWaitRelation waits=" << waits->getID() << " foe=" << reason->getID() << "\n";
    //     myWaitRelations[waits] = WaitRelation(rs, reason, constraint);
    // }
    #[allow(unused_variables)]
    pub fn add_wait_relation(&mut self, waits: Ptr<SUMOVehicle>, rs: Ptr<MSRailSignal>, reason: Ptr<SUMOVehicle>, constraint: Ptr<MSRailSignalConstraint>) {
        todo!("port MSRailSignalControl::addWaitRelation")
    }

    /// C++: `void MSRailSignalControl::addDrivewayFollower(const MSDriveWay *, const MSDriveWay *)`
    // void
    // MSRailSignalControl::addDrivewayFollower(const MSDriveWay* dw, const MSDriveWay* dw2) {
    //     //std::cout << " addDrivewayFollower " << dw->getID() << " " << dw2->getID() << "\n";
    //     myDriveWaySucc[dw].insert(dw2);
    //     myDriveWayPred[dw2].insert(dw);
    // }
    #[allow(unused_variables)]
    pub fn add_driveway_follower(&mut self, dw: Ptr<MSDriveWay>, dw2: Ptr<MSDriveWay>) {
        todo!("port MSRailSignalControl::addDrivewayFollower")
    }

    /// C++: `void MSRailSignalControl::addDWDeadlockChecks(const MSRailSignal *, MSDriveWay *)`
    /// check whether the given signal and driveway are part of a deadlock circle
    // void
    // MSRailSignalControl::addDWDeadlockChecks(const MSRailSignal* rs, MSDriveWay* dw) {
    //     auto itDL = mySignalDeadlocks.find(rs);
    //     if (itDL == mySignalDeadlocks.end()) {
    //         return;
    //     }
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //     std::cout << "    needsDeadlockChecks? rs=" << rs->getID() << " dw=" << dw->getID() << "\n";
    // #endif
    //     // the driveway dw ends at signal rs and could be part of a deadlock.
    //     // check whether we can form a sequence of driveways that link it with all others (railsignals)
    //     // findDeadlockFoes calls itself recursively until all circles have been found
    //     for (const auto& pair : itDL->second) {
    //         const Deadlock& dl = myLoadedDeadlocks[pair.first];
    //         std::vector<const MSDriveWay*> deadlockFoes;
    //         if (dl.edges.empty()) {
    //             findDeadlockFoes(dw, dl, pair.second, deadlockFoes);
    //         } else {
    //             const MSEdge* forwardEnd = dl.forwardEnds[pair.second];
    //             if (forwardEnd == &dw->getForward().back()->getEdge()) {
    //                 findDeadlockFoesFromRoutes(dw, dl, pair.second, deadlockFoes);
    //             } else {
    //                 for (MSDriveWay* cand : MSDriveWay::getEndingDriveWays(forwardEnd)) {
    //                     findDeadlockFoesFromRoutes(cand, dl, pair.second, deadlockFoes);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_dw_deadlock_checks(&mut self, rs: Ptr<MSRailSignal>, dw: Ptr<MSDriveWay>) {
        todo!("port MSRailSignalControl::addDWDeadlockChecks")
    }

    /// C++: `bool MSRailSignalControl::haveDeadlock(const SUMOVehicle *) const` [const]
    /// whether there is a circle in the waiting-for relationships that contains the given vehicle
    // bool
    // MSRailSignalControl::haveDeadlock(const SUMOVehicle* veh) const {
    //     std::set<const SUMOVehicle*> seen;
    //     std::vector<WaitRelation> list;
    //     const SUMOVehicle* cur = veh;
    //     std::vector<MSRailSignalConstraint*> constraints;
    //     std::vector<const SUMOVehicle*> constraintBlocked;
    //     std::vector<const MSRailSignal*> constraintSignals;
    //     //std::cout << time2string(SIMSTEP) << " haveDeadlock veh=" << veh->getID() << "\n";
    //     while (seen.count(cur) == 0) {
    //         auto it = myWaitRelations.find(cur);
    //         if (it != myWaitRelations.end()) {
    //             if (it->second.constraint != nullptr) {
    //                 constraints.push_back(it->second.constraint);
    //                 constraintBlocked.push_back(cur);
    //                 constraintSignals.push_back(it->second.railSignal);
    //             }
    //             seen.insert(cur);
    //             list.push_back(it->second);
    //             cur = it->second.foe;
    //         } else {
    //             return false;
    //         }
    //     }
    //     if (cur == veh) {
    //         const bool newDeadlock = myWrittenDeadlocks.count(seen) == 0;
    //         myWrittenDeadlocks.insert(seen);
    //         const OptionsCont& oc = OptionsCont::getOptions();
    //         MSRailSignalConstraint* resolved = nullptr;
    //         const SUMOVehicle* resolvedUnblocked = nullptr;
    //         const MSRailSignal* resolvedSignal = nullptr;
    //         if (!constraints.empty() && oc.getBool("time-to-teleport.remove-constraint")) {
    //             resolved = constraints.front();
    //             if (newDeadlock) {
    //                 std::vector<std::string> vehicles;
    //                 for (auto item : list) {
    //                     vehicles.push_back(item.foe->getID());
    //                 }
    //                 WRITE_WARNINGF("Deactivating constraint to resolve deadlock between vehicles % at time %.", toString(vehicles), time2string(SIMSTEP));
    //                 resolved->setActive(false);
    //                 resolvedUnblocked = constraintBlocked.front();
    //                 resolvedSignal = constraintSignals.front();
    //             }
    //         }
    //
    //         if (oc.isSet("deadlock-output")) {
    //             if (newDeadlock) {
    //                 myWrittenDeadlocks.insert(seen);
    //                 std::vector<std::string> signals;
    //                 std::vector<std::string> vehicles;
    //                 std::vector<std::string> tripIDs;
    //                 for (auto item : list) {
    //                     signals.push_back(item.railSignal == nullptr ? "INSERTION" : item.railSignal->getID());
    //                     vehicles.push_back(item.foe->getID());
    //                     tripIDs.push_back(item.foe->getParameter().getParameter("tripId", item.foe->getID()));
    //                 }
    //                 // rotate signals so their are aligned with the vehicle waiting
    //                 // at that signal instead of the foe that is blocking the signal
    //                 std::rotate(signals.begin(), signals.begin() + 1, signals.end());
    //                 OutputDevice& od = OutputDevice::getDeviceByOption("deadlock-output");
    //                 if (constraints.empty()) {
    //                     od.openTag(SUMO_TAG_DEADLOCK);
    //                 } else {
    //                     od.openTag("constraintDeadlock");
    //                 }
    //                 od.writeAttr(SUMO_ATTR_TIME, time2string(SIMSTEP));
    //                 od.writeAttr(SUMO_ATTR_SIGNALS, signals);
    //                 od.writeAttr("vehicles", vehicles);
    //                 if (!constraints.empty()) {
    //                     od.writeAttr("tripIds", tripIDs);
    //                 }
    //                 if (resolved != nullptr) {
    //                     od.openTag("resolvedConstraint");
    //                     od.writeAttr(SUMO_ATTR_ID, resolvedSignal->getID());
    //                     resolved->write(od, resolvedUnblocked->getParameter().getParameter("tripId", resolvedUnblocked->getID()));
    //                     od.closeTag();
    //                 }
    //                 for (auto item : list) {
    //                     int start = item.foe->getRoutePosition();
    //                     if (!MSGlobals::gUseMesoSim) {
    //                         const std::vector<MSLane*>& further = dynamic_cast<const MSVehicle*>(item.foe)->getFurtherLanes();
    //                         start = MAX2(0, start - (int)(MSGlobals::gUsingInternalLanes && MSNet::getInstance()->hasInternalLinks() ? further.size() / 2 : further.size()));
    //                     }
    //                     const MSRoute& route = item.foe->getRoute();
    //                     ConstMSEdgeVector future(route.begin() + start, route.end());
    //                     od.openTag(SUMO_TAG_ROUTEPART);
    //                     od.writeAttr(SUMO_ATTR_EDGES, toString(future));
    //                     od.closeTag();
    //                 }
    //                 od.closeTag();
    //             }
    //         }
    //         return resolved == nullptr;
    //     } else {
    //         // it's a deadlock but does not involve veh
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn have_deadlock(&self, veh: Ptr<SUMOVehicle>) -> bool {
        todo!("port MSRailSignalControl::haveDeadlock")
    }

    /// C++: `void MSRailSignalControl::loadDeadlockCheck(std::vector<const MSRailSignal *>)`
    // void
    // MSRailSignalControl::loadDeadlockCheck(std::vector<const MSRailSignal*> signals) {
    //     if (MSDriveWay::haveDriveWays()) {
    //         WRITE_WARNING("Deadlocks should be loaded before any vehicles");
    //     }
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //     std::cout << "    loadedDeadlockCheck signals=" << toString(signals) << "\n";
    // #endif
    //     const int deadlockIndex = (int)myLoadedDeadlocks.size();
    //     myLoadedDeadlocks.push_back(Deadlock());
    //     myLoadedDeadlocks.back().signals = signals;
    //     const int n = (int)signals.size();
    //     for (int i = 0; i < n; i++) {
    //         mySignalDeadlocks[signals[i]].push_back(std::make_pair(deadlockIndex, i));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_deadlock_check(&mut self, signals: Vec<Ptr<MSRailSignal>>) {
        todo!("port MSRailSignalControl::loadDeadlockCheck")
    }

    /// C++: `void MSRailSignalControl::loadDeadlockRoute(const ConstMSEdgeVector &)`
    // void
    // MSRailSignalControl::loadDeadlockRoute(const ConstMSEdgeVector& edges) {
    //     Deadlock& dl = myLoadedDeadlocks.back();
    //     const int signalIndex = (int)dl.edges.size();
    //     const MSRailSignal* rs = dl.signals[signalIndex];
    //     const MSJunction* j = rs->getJunction();
    //     auto it = edges.begin();
    //     while (it != edges.end() && (*it)->getToJunction() != j) {
    //         it++;
    //     }
    //     if (it == edges.end()) {
    //         throw ProcessError("Rail signal junction '" + j->getID() + "' not passed by loaded deadlock edges '" + toString(edges) + "'.");
    //     }
    //     const MSEdge* firstSignalEdge = *it;
    //     // maybe there is an earlier signal, in this case we must look for driveways that start there
    //     for (auto it2 = edges.begin(); it2 != it; it2++) {
    //         if ((*it2)->getToJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //             const MSLink* link = (*it2)->getLanes().front()->getLinkTo((*(it2 + 1))->getLanes().front());
    //             if (link != nullptr && link->isTLSControlled()) {
    //                 firstSignalEdge = *it2;
    //                 break;
    //             }
    //         }
    //     }
    //     dl.forwardEnds.push_back(firstSignalEdge);
    //     dl.edges.push_back(edges);
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //     std::cout << "      loadedDeadlockRoute signal=" << rs->getID() << " signalEdge=" << firstSignalEdge->getID() << " edges=" << toString(edges) << "\n";
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn load_deadlock_route(&mut self, edges: &ConstMSEdgeVector) {
        todo!("port MSRailSignalControl::loadDeadlockRoute")
    }

    /// C++: `void MSRailSignalControl::vehicleStateChanged(const SUMOVehicle *const, MSNet::VehicleState, const std::string &)` [virtual]
    /// Called if a vehicle changes its state
    // void
    // MSRailSignalControl::vehicleStateChanged(const SUMOVehicle* const vehicle, MSNet::VehicleState to, const std::string& /*info*/) {
    //     if (vehicle->isRail()) {
    //         std::string dummyMsg;
    //         if ((to == MSNet::VehicleState::BUILT && (!vehicle->getParameter().wasSet(VEHPARS_FORCE_REROUTE) || vehicle->hasValidRoute(dummyMsg)))
    //                 || to == MSNet::VehicleState::NEWROUTE) {
    //             // @note we could delay initialization until the departure time
    //             if (vehicle->getEdge()->getFunction() != SumoXMLEdgeFunc::CONNECTOR && isRailwayOrShared(vehicle->getEdge()->getPermissions())) {
    //                 MSRailSignal::initDriveWays(vehicle, to == MSNet::VehicleState::NEWROUTE);
    //             }
    //         }
    //         if (to == MSNet::VehicleState::BUILT) {
    //             myMaxTrainLength = MAX2(myMaxTrainLength, vehicle->getLength());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn vehicle_state_changed(&mut self, vehicle: Ptr<SUMOVehicle>, to: MSNet_VehicleState, info: &str) {
        todo!("port MSRailSignalControl::vehicleStateChanged")
    }

    /// C++: `void MSRailSignalControl::addSignal(MSRailSignal *)`
    // void
    // MSRailSignalControl::addSignal(MSRailSignal* signal) {
    //     mySignals.push_back(signal);
    //     for (const auto& links : signal->getLinks()) {
    //         for (const MSLink* link : links) {
    //             mySignalizedClasses |= (link->getPermissions() & myMBClasses);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_signal(&mut self, signal: Ptr<MSRailSignal>) {
        todo!("port MSRailSignalControl::addSignal")
    }

    /// C++: `const std::vector<MSRailSignal *> & MSRailSignalControl::getSignals() const` [const]
    // const std::vector<MSRailSignal*>& getSignals() const {
    //         return mySignals;
    //     }
    #[allow(unused_variables)]
    pub fn get_signals(&self) -> &Vec<Ptr<MSRailSignal>> {
        todo!("port MSRailSignalControl::getSignals")
    }

    /// C++: `void MSRailSignalControl::notifyApproach(const MSLink *)`
    /// switch rail signal to active
    // void
    // MSRailSignalControl::notifyApproach(const MSLink* link) {
    //     const MSRailSignal* rs = dynamic_cast<const MSRailSignal*>(link->getTLLogic());
    //     assert(rs != nullptr);
    //     myActiveSignals.insert(const_cast<MSRailSignal*>(rs));
    // }
    #[allow(unused_variables)]
    pub fn notify_approach(&mut self, link: Ptr<MSLink>) {
        todo!("port MSRailSignalControl::notifyApproach")
    }

    /// C++: `void MSRailSignalControl::updateSignals(SUMOTime)`
    /// update active rail signals
    // void
    // MSRailSignalControl::updateSignals(SUMOTime t) {
    //     UNUSED_PARAMETER(t);
    //     // there are 4 states for a signal
    //     // 1. approached and green
    //     // 2. approached and red
    //     // 3. not approached and trains could pass
    //     // 4. not approached and trains coult not pass
    //     //
    //     // for understanding conflicts better in sumo-gui, we want to show (3) as green. This
    //     // means we have to keep updating signals in state (4) until they change to (3)
    //
    //     //std::cout << SIMTIME << " activeSignals=" << myActiveSignals.size() << "\n";
    //     for (auto it = myActiveSignals.begin(); it != myActiveSignals.end();) {
    //         MSRailSignal* rs = *it;
    //         const bool keepActive = rs->updateCurrentPhase();
    //         // std::cout << SIMTIME << " update " << rs->getID() << " keepActive=" << keepActive << "\n";
    //         if (rs->isActive()) {
    //             rs->setTrafficLightSignals(t);
    //         }
    //         if (!keepActive) {
    //             it = myActiveSignals.erase(it);
    //         } else {
    //             it++;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_signals(&mut self, t: SUMOTime) {
        todo!("port MSRailSignalControl::updateSignals")
    }

    /// C++: `bool MSRailSignalControl::isSignalized(SUMOVehicleClass)` [static]
    // static bool isSignalized(SUMOVehicleClass svc) {
    //         return (mySignalizedClasses & svc) == svc;
    //     }
    #[allow(unused_variables)]
    pub fn is_signalized(svc: SUMOVehicleClass) -> bool {
        todo!("port MSRailSignalControl::isSignalized")
    }

    /// C++: `bool MSRailSignalControl::isMovingBlock(SVCPermissions)` [static]
    // static bool isMovingBlock(SVCPermissions svc) {
    //         return (myMBClasses & svc & SVC_RAIL_CLASSES) == (svc & SVC_RAIL_CLASSES);
    //     }
    #[allow(unused_variables)]
    pub fn is_moving_block(svc: SVCPermissions) -> bool {
        todo!("port MSRailSignalControl::isMovingBlock")
    }

    /// C++: `bool MSRailSignalControl::isUsingDriveWays(SVCPermissions)` [static]
    // static bool isUsingDriveWays(SVCPermissions svc) {
    //         return ((mySignalizedClasses | myMBClasses) & svc) == svc;
    //     }
    #[allow(unused_variables)]
    pub fn is_using_drive_ways(svc: SVCPermissions) -> bool {
        todo!("port MSRailSignalControl::isUsingDriveWays")
    }

    /// C++: `void MSRailSignalControl::initSignalized(SVCPermissions, SVCPermissions)` [static]
    // static void initSignalized(SVCPermissions svc, SVCPermissions mbSvc) {
    //         mySignalizedClasses = svc;
    //         myMBClasses = mbSvc;
    //     }
    #[allow(unused_variables)]
    pub fn init_signalized(svc: SVCPermissions, mbSvc: SVCPermissions) {
        todo!("port MSRailSignalControl::initSignalized")
    }

    /// C++: `void MSRailSignalControl::findDeadlockFoes(const MSDriveWay *, const Deadlock &, int, std::vector<const MSDriveWay *>)` [protected]
    // void
    // MSRailSignalControl::findDeadlockFoes(const MSDriveWay* dw, const Deadlock& dl, int signalIndex, std::vector<const MSDriveWay*> deadlockFoes) {
    //     if (deadlockFoes.size() < dl.signals.size()) {
    //         signalIndex = (signalIndex + 1) % dl.signals.size();
    //         const MSRailSignal* other = dl.signals[signalIndex];
    //         const MSJunction* otherJ = other->getJunction();
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //         std::cout << "      findDLfoes dw=" << dw->getID() << " foundFoes=" << toString(deadlockFoes) << " otherRS=" << other->getID() << "\n";
    // #endif
    //         deadlockFoes.push_back(dw);
    //         for (const MSDriveWay* follower : myDriveWaySucc[dw]) {
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //             std::cout << "        follower=" << follower->getID() << " followerFoes=" << toString(follower->getFoes()) << "\n";
    // #endif
    //             for (MSDriveWay* foe : follower->getFoes()) {
    //                 if (foe->getForward().back()->getToJunction() == otherJ) {
    //                     findDeadlockFoes(foe, dl, signalIndex, deadlockFoes);
    //                 } else if (foe->isSubDriveWay() && foe->getParent()->getForward().back()->getToJunction() == otherJ && foe->getParentRemainingLength() < myMaxTrainLength) {
    //                     // subdriveway-parents where the remaining part is short and thus a long train might reach back onto the subdriveway
    //                     // @note: technically we do not need to check this in meso because train ends cannot reach into multiple segments
    //                     // but it wouldn't help to make meso "better" by ignoring this foe relationship
    //                     findDeadlockFoes(foe->getParent(), dl, signalIndex, deadlockFoes);
    //                 }
    //             }
    //         }
    //     } else {
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //         std::cout << "        add deadlock check dlFoes=" << toString(deadlockFoes) << "\n";;
    // #endif
    //         for (const MSDriveWay* dldw : deadlockFoes) {
    //             if (!dldw->isDepartDriveway()) {
    //                 const MSRailSignal* rs = dynamic_cast<const MSRailSignal*>(dldw->getOrigin()->getTLLogic());
    //                 if (std::find(dl.signals.begin(), dl.signals.end(), rs) != dl.signals.end()) {
    //                     // moving into dldw frees up another part of the deadlock
    //                     continue;
    //                 }
    //             }
    //             std::vector<const MSDriveWay*> df(deadlockFoes);
    //             df.erase(std::find(df.begin(), df.end(), dldw));
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //             std::cout << "           dldw=" << dldw->getID() << " df=" << toString(df) << "\n";
    // #endif
    //             const_cast<MSDriveWay*>(dldw)->addDWDeadlock(df);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn find_deadlock_foes(&mut self, dw: Ptr<MSDriveWay>, dl: &MSRailSignalControl_Deadlock, signalIndex: i32, deadlockFoes: Vec<Ptr<MSDriveWay>>) {
        todo!("port MSRailSignalControl::findDeadlockFoes")
    }

    /// C++: `void MSRailSignalControl::findDeadlockFoesFromRoutes(const MSDriveWay *, const Deadlock &, int, std::vector<const MSDriveWay *>)` [protected]
    // void
    // MSRailSignalControl::findDeadlockFoesFromRoutes(const MSDriveWay* dw, const Deadlock& dl, int signalIndex, std::vector<const MSDriveWay*> deadlockFoes) {
    //     // the loaded edges must match the end of the driveway route
    //     const ConstMSEdgeVector& de = dl.edges[signalIndex];
    //     const size_t n = de.size();
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //     std::cout << "      findDLfoesFromRoutes dw=" << dw->getID() << " foundFoes=" << toString(deadlockFoes)
    //         << "\n        dle=" << toString(de)
    //         << "\n        dwe=" << toString(ConstMSEdgeVector(dw->getRoute().end() - MIN2(n, dw->getRoute().size()), dw->getRoute().end()))
    //         << "\n";
    // #endif
    //     if (dw->getRoute().size() >= n && std::equal(de.begin(), de.end(),
    //                 dw->getRoute().end() - n, dw->getRoute().end())) {
    //         deadlockFoes.push_back(dw);
    //         if (deadlockFoes.size() < dl.signals.size()) {
    //             signalIndex = (signalIndex + 1) % dl.signals.size();
    //             const MSEdge* forwardEnd = dl.forwardEnds[signalIndex];
    //             for (MSDriveWay* cand : MSDriveWay::getEndingDriveWays(forwardEnd)) {
    //                 findDeadlockFoesFromRoutes(cand, dl, signalIndex, deadlockFoes);
    //             }
    //         } else {
    //             findDeadlockFoes(dw, dl, signalIndex, deadlockFoes);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn find_deadlock_foes_from_routes(&mut self, dw: Ptr<MSDriveWay>, dl: &MSRailSignalControl_Deadlock, signalIndex: i32, deadlockFoes: Vec<Ptr<MSDriveWay>>) {
        todo!("port MSRailSignalControl::findDeadlockFoesFromRoutes")
    }

    /// C++: `void MSRailSignalControl::MSRailSignalControl()` [private]
    /// Constructor
    // MSRailSignalControl::MSRailSignalControl():
    //     myMaxTrainLength(OptionsCont::getOptions().getFloat("railway.max-train-length"))
    // {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port MSRailSignalControl::MSRailSignalControl")
    }

    /// C++: `void MSRailSignalControl::updateDriveways(const MSEdge *)` [private]
    /// compute additioanl deadlock-check requirements for registered driveways
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn update_driveways(&mut self, used: Ptr<MSEdge>) {
        todo!("port MSRailSignalControl::updateDriveways")
    }

}

/// C++ class `MSRailSignalControl::WaitRelation` (microsim/traffic_lights/MSRailSignalControl.h)
pub struct MSRailSignalControl_WaitRelation {
    pub railSignal: Ptr<MSRailSignal>,
    pub foe: Ptr<SUMOVehicle>,
    pub constraint: Ptr<MSRailSignalConstraint>,
}

impl MSRailSignalControl_WaitRelation {
    /// C++: `void WaitRelation::WaitRelation(const MSRailSignal *, const SUMOVehicle *, MSRailSignalConstraint *)`
    // WaitRelation(const MSRailSignal* _railSignal = nullptr, const SUMOVehicle* _foe = nullptr, MSRailSignalConstraint* _constraint = nullptr) :
    //             railSignal(_railSignal), foe(_foe), constraint(_constraint) {}
    #[allow(unused_variables)]
    pub fn new(_railSignal: Ptr<MSRailSignal>, _foe: Ptr<SUMOVehicle>, _constraint: Ptr<MSRailSignalConstraint>) -> Self {
        todo!("port WaitRelation::WaitRelation")
    }

}

