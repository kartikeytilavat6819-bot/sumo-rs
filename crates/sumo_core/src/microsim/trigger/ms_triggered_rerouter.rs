//! Port of microsim/trigger/MSTriggeredRerouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use utils::xml::generic_sax_handler::GenericSAXHandler;
use crate::prelude::*;
use std::cell::RefCell;
use crate::utils::router::intermodal_router::IntermodalRouter_TripItem;

/// C++ typedef `std::map<const MSEdge *, RouterProhibition> MSTriggeredRerouter::Prohibitions`
pub type MSTriggeredRerouter_Prohibitions = BTreeMap<Ptr<MSEdge>, RouterProhibition>;

/// C++ `WrappingCommand<MSTriggeredRerouter>(this, &MSTriggeredRerouter::setPermissions)`
pub struct MSTriggeredRerouter_SetPermissionsCommand {
    pub rerouter: Ptr<MSTriggeredRerouter>,
}

impl Command for MSTriggeredRerouter_SetPermissionsCommand {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        self.rerouter.m().set_permissions(currentTime)
    }
}

// C++ static members of MSTriggeredRerouter (see the comment block below the struct)
/// C++ `MSTriggeredRerouter::DEFAULT_MAXDELAY`
pub const MSTriggeredRerouter_DEFAULT_MAXDELAY: f64 = 7200.0;

thread_local! {
    /// C++ `MSTriggeredRerouter::mySpecialDest_keepDestination` (a sentinel MSEdge, compared by address)
    static MY_SPECIAL_DEST_KEEP_DESTINATION: Ptr<MSEdge> = Ptr::from_raw(Box::leak(Box::new(MSEdge::new("MSTriggeredRerouter_keepDestination", -1, SumoXMLEdgeFunc::UNKNOWN, "", "", "", -1, 0.0))) as *mut MSEdge);
    /// C++ `MSTriggeredRerouter::mySpecialDest_terminateRoute` (a sentinel MSEdge, compared by address)
    static MY_SPECIAL_DEST_TERMINATE_ROUTE: Ptr<MSEdge> = Ptr::from_raw(Box::leak(Box::new(MSEdge::new("MSTriggeredRerouter_terminateRoute", -1, SumoXMLEdgeFunc::UNKNOWN, "", "", "", -1, 0.0))) as *mut MSEdge);
    /// C++ `MSTriggeredRerouter::myInstances`
    static MY_INSTANCES: RefCell<BTreeMap<String, Ptr<MSTriggeredRerouter>>> = RefCell::new(BTreeMap::new());
}

/// C++ `MSTriggeredRerouter::mySpecialDest_keepDestination`
fn special_dest_keep_destination() -> Ptr<MSEdge> {
    MY_SPECIAL_DEST_KEEP_DESTINATION.with(|p| *p)
}

/// C++ `MSTriggeredRerouter::mySpecialDest_terminateRoute`
fn special_dest_terminate_route() -> Ptr<MSEdge> {
    MY_SPECIAL_DEST_TERMINATE_ROUTE.with(|p| *p)
}

/// Reroutes traffic objects passing an edge
/// C++ class `MSTriggeredRerouter` (microsim/trigger/MSTriggeredRerouter.h)
#[repr(C)]
pub struct MSTriggeredRerouter {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `MSMoveReminder`
    pub base2: MSMoveReminder,
    /// C++ base class `SUMOSAXHandler`
    pub base3: GenericSAXHandler,
    /// C++ base class `MSStoppingPlaceRerouter`
    pub base4: MSStoppingPlaceRerouter,
    /// edges where vehicles are notified
    pub myEdges: MSEdgeVector,
    /// List of rerouting definition intervals
    pub myIntervals: Vec<MSTriggeredRerouter_RerouteInterval>,
    /// The probability and the user-given probability
    pub myProbability: f64,
    /// The probability and the user-given probability
    pub myUserProbability: f64,
    /// Information whether the current rerouting probability is the user-given
    pub myAmInUserMode: bool,
    /// Information whether the rerouting will only take place on request
    pub myAmOptional: bool,
    /// Where are we located in the network
    pub myPosition: Position,
    /// At which distance are we activated
    pub myRadius: f64,
    pub myTimeThreshold: SUMOTime,
    /// The vehicle types to look for (empty means all)
    pub myVehicleTypes: BTreeSet<String>,
    /// used during loading
    pub myParsedRerouteInterval: MSTriggeredRerouter_RerouteInterval,
    /// whether this rerouter has loaded parkingReroute definitions
    pub myHaveParkProbs: bool,
    /// whether closed edges with known duration are loaded
    pub myHaveClosingUntil: bool,
    pub myBlockedStoppingPlaces: BTreeSet<Ptr<MSStoppingPlace>>,
}

impl std::ops::Deref for MSTriggeredRerouter {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for MSTriggeredRerouter {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

// C++ static members of MSTriggeredRerouter (port as module-level statics, see AGENTS.md):
//   static const double MSTriggeredRerouter::DEFAULT_MAXDELAY;
//   static MSEdge MSTriggeredRerouter::mySpecialDest_keepDestination;
//   static MSEdge MSTriggeredRerouter::mySpecialDest_terminateRoute;
//   static std::map<std::string, MSTriggeredRerouter *> MSTriggeredRerouter::myInstances;

impl MSTriggeredRerouter {
    /// C++: `void MSTriggeredRerouter::MSTriggeredRerouter(const std::string &, const MSEdgeVector &, double, bool, bool, SUMOTime, const std::string &, const Position &, const double)`
    /// Constructor
    // MSTriggeredRerouter::MSTriggeredRerouter(const std::string& id,
    //         const MSEdgeVector& edges, double prob, bool off, bool optional,
    //         SUMOTime timeThreshold, const std::string& vTypes, const Position& pos, const double radius) :
    //     Named(id),
    //     MSMoveReminder(id),
    //     MSStoppingPlaceRerouter("parking"),
    //     myEdges(edges),
    //     myProbability(prob),
    //     myUserProbability(prob),
    //     myAmInUserMode(false),
    //     myAmOptional(optional),
    //     myPosition(pos),
    //     myRadius(radius),
    //     myTimeThreshold(timeThreshold),
    //     myHaveParkProbs(false),
    //     myHaveClosingUntil(false)
    // {
    //     myInstances[id] = this;
    //     // build actors
    //     for (const MSEdge* const e : edges) {
    //         if (MSGlobals::gUseMesoSim) {
    //             MSGlobals::gMesoNet->getSegmentForEdge(*e)->addDetector(this);
    //         }
    //         for (MSLane* const lane : e->getLanes()) {
    //             lane->addMoveReminder(this);
    //         }
    //     }
    //     if (off) {
    //         setUserMode(true);
    //         setUserUsageProbability(0);
    //     }
    //     const std::vector<std::string> vt = StringTokenizer(vTypes).getVector();
    //     myVehicleTypes.insert(vt.begin(), vt.end());
    //     if (myPosition == Position::INVALID) {
    //         myPosition = edges.front()->getLanes()[0]->getShape()[0];
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, edges: &MSEdgeVector, prob: f64, off: bool, optional: bool, timeThreshold: SUMOTime, vTypes: &str, pos: &Position, radius: f64) -> Box<Self> {
        // Boxed because the constructor registers `this` (myInstances, the lanes' move reminders).
        let mut b = Box::new(MSTriggeredRerouter {
            base: Named::new(id),
            base2: MSMoveReminder::new(id, Ptr::default(), true),
            base3: utils::xml::sumo_sax_handler::new("", ""), // C++ default-constructs SUMOSAXHandler
            base4: MSStoppingPlaceRerouter::new("parking".to_string(), false, Default::default(), Default::default()),
            myEdges: edges.clone(),
            myIntervals: Vec::new(),
            myProbability: prob,
            myUserProbability: prob,
            myAmInUserMode: false,
            myAmOptional: optional,
            myPosition: pos.clone(),
            myRadius: radius,
            myTimeThreshold: timeThreshold,
            myVehicleTypes: BTreeSet::new(),
            myParsedRerouteInterval: MSTriggeredRerouter_RerouteInterval {
                id: 0,
                begin: 0,
                end: 0,
                closed: BTreeMap::new(),
                closedLanes: BTreeMap::new(),
                closedLanesAffected: Vec::new(),
                edgeProbs: Default::default(),
                routeProbs: Default::default(),
                parkProbs: Default::default(),
                isVia: false,
                permissionsAllowAll: false,
                overtakeLocations: Vec::new(),
                stopAlternatives: Vec::new(),
            },
            myHaveParkProbs: false,
            myHaveClosingUntil: false,
            myBlockedStoppingPlaces: BTreeSet::new(),
        });
        b.base2.vt = &MS_TRIGGERED_REROUTER_MSMOVE_REMINDER_VT;
        // C++ `myInstances[id] = this` and `lane->addMoveReminder(this)`; the box keeps the address stable
        let this_ptr = Ptr::from_box(&b);
        MY_INSTANCES.with(|m| m.borrow_mut().insert(b.base.get_id().to_string(), this_ptr));
        // shortcut: the meso segment registration (gUseMesoSim) is skipped, meso is not ported
        let rem = Ptr::from_mut(&mut b.base2);
        for e in b.myEdges.iter() {
            for lane in e.get_lanes().iter() {
                lane.m().add_move_reminder(rem, true);
            }
        }
        if off {
            b.set_user_mode(true);
            b.set_user_usage_probability(0.0);
        }
        b.myVehicleTypes.extend(StringTokenizer::new(vTypes).get_vector());
        if b.myPosition == Position::INVALID {
            b.myPosition = edges[0].get_lanes()[0].get_shape_2()[0];
        }
        b
    }

    /// C++: `void MSTriggeredRerouter::~MSTriggeredRerouter() noexcept` [virtual]
    /// Destructor
    // MSTriggeredRerouter::~MSTriggeredRerouter() {
    //     myInstances.erase(getID());
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        let id = self.base.get_id().to_string();
        MY_INSTANCES.with(|m| m.borrow_mut().remove(&id));
    }

    /// C++: `bool MSTriggeredRerouter::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Tries to reroute the vehicle
    // bool
    // MSTriggeredRerouter::notifyEnter(SUMOTrafficObject& tObject, MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     if (myAmOptional || myRadius != std::numeric_limits<double>::max()) {
    //         return true;
    //     }
    //     return triggerRouting(tObject, reason);
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        if self.myAmOptional || self.myRadius != f64::MAX {
            return true;
        }
        self.trigger_routing(veh, reason)
    }

    /// C++: `bool MSTriggeredRerouter::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Triggers rerouting (once) for vehicles that are already on the edge when the rerouter activates
    // bool
    // MSTriggeredRerouter::notifyMove(SUMOTrafficObject& veh, double /*oldPos*/,
    //                                 double /*newPos*/, double /*newSpeed*/) {
    //     return triggerRouting(veh, NOTIFICATION_JUNCTION);
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        self.trigger_routing(veh, MSMoveReminder_Notification::NOTIFICATION_JUNCTION)
    }

    /// C++: `bool MSTriggeredRerouter::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Removes the reminder
    // bool
    // MSTriggeredRerouter::notifyLeave(SUMOTrafficObject& /*veh*/, double /*lastPos*/,
    //                                  MSMoveReminder::Notification reason, const MSLane* /* enteredLane */) {
    //     return reason == NOTIFICATION_LANE_CHANGE;
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        reason == MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE
    }

    /// C++: `bool MSTriggeredRerouter::triggerRouting(SUMOTrafficObject &, MSMoveReminder::Notification)`
    // bool
    // MSTriggeredRerouter::triggerRouting(SUMOTrafficObject& tObject, MSMoveReminder::Notification reason) {
    //     if (!applies(tObject)) {
    //         return false;
    //     }
    //     if (myRadius != std::numeric_limits<double>::max() && tObject.getPosition().distanceTo(myPosition) > myRadius) {
    //         return true;
    //     }
    //     // check whether the vehicle shall be rerouted
    //     const SUMOTime now = MSNet::getInstance()->getCurrentTimeStep();
    //     const MSTriggeredRerouter::RerouteInterval* const rerouteDef = getCurrentReroute(now, tObject);
    //     if (rerouteDef == nullptr) {
    //         return true; // an active interval could appear later
    //     }
    //     const double prob = myAmInUserMode ? myUserProbability : myProbability;
    //     if (prob < 1 && RandHelper::rand(tObject.getRNG()) > prob) {
    //         return false; // XXX another interval could appear later but we would have to track whether the current interval was already tried
    //     }
    //     if (myTimeThreshold > 0 && MAX2(tObject.getWaitingTime(), tObject.getWaitingTime(true)) < myTimeThreshold) {
    //         return true; // waiting time may be reached later
    //     }
    //     if (reason == NOTIFICATION_LANE_CHANGE) {
    //         return false;
    //     }
    //     // if we have a closingLaneReroute it may still affect the network topology (i.e. by closing a turning lane).
    //     // Even if the topology stays unchanged, vehicles with a rerouting device could reroute around jams that form due to capacity reduction
    //     const bool hasReroutingDevice = tObject.getDevice(typeid(MSDevice_Routing)) != nullptr;
    //     const MSEdge* lastEdge = tObject.getRerouteDestination();
    // #ifdef DEBUG_REROUTER
    //     if (DEBUGCOND(tObject)) {
    //         std::cout << SIMTIME << " veh=" << tObject.getID() << " check rerouter " << getID() << " lane=" << Named::getIDSecure(tObject.getLane()) << " edge=" << tObject.getEdge()->getID() << " finalEdge=" << lastEdge->getID() /*<< " arrivalPos=" << tObject.getArrivalPos()*/ << "\n";
    //     }
    // #endif
    //
    //     if (rerouteDef->parkProbs.getOverallProb() > 0) {
    // #ifdef HAVE_FOX
    //         ScopedLocker<> lock(myNotificationMutex, MSGlobals::gNumSimThreads > 1);
    // #endif
    //         if (!tObject.isVehicle()) {
    //             return false;
    //         }
    //         SUMOVehicle& veh = static_cast<SUMOVehicle&>(tObject);
    //         bool newDestination = false;
    //         ConstMSEdgeVector newRoute;
    //         MSParkingArea* oldParkingArea = veh.getNextParkingArea();
    //         MSParkingArea* newParkingArea = rerouteParkingArea(rerouteDef, veh, newDestination, newRoute);
    //         if (newParkingArea != nullptr) {
    //             // adapt plans of any riders
    //             for (MSTransportable* p : veh.getPersons()) {
    //                 p->rerouteParkingArea(oldParkingArea, newParkingArea);
    //             }
    //
    //             if (newDestination && veh.getParameter().arrivalPosProcedure != ArrivalPosDefinition::DEFAULT) {
    //                 // update arrival parameters
    //                 SUMOVehicleParameter* newParameter = new SUMOVehicleParameter();
    //                 *newParameter = veh.getParameter();
    //                 newParameter->arrivalPosProcedure = ArrivalPosDefinition::GIVEN;
    //                 newParameter->arrivalPos = newParkingArea->getEndLanePosition();
    //                 veh.replaceParameter(newParameter);
    //             }
    //
    //             SUMOAbstractRouter<MSEdge, SUMOVehicle>& router = hasReroutingDevice
    //                     ? MSRoutingEngine::getRouterTT(veh.getRNGIndex(), veh.getVClass(), rerouteDef->getClosed())
    //                     : MSNet::getInstance()->getRouterTT(veh.getRNGIndex(), rerouteDef->getClosed());
    //             const double routeCost = router.recomputeCosts(newRoute, &veh, MSNet::getInstance()->getCurrentTimeStep());
    //             ConstMSEdgeVector prevEdges(veh.getCurrentRouteEdge(), veh.getRoute().end());
    //             resetClosedEdges(hasReroutingDevice, veh);
    //             const double previousCost = router.recomputeCosts(prevEdges, &veh, MSNet::getInstance()->getCurrentTimeStep());
    //             const double savings = previousCost - routeCost;
    //             //if (getID() == "ego") std::cout << SIMTIME << " pCost=" << previousCost << " cost=" << routeCost
    //             //        << " prevEdges=" << toString(prevEdges)
    //             //        << " newEdges=" << toString(edges)
    //             //        << "\n";
    //
    //             std::string errorMsg;
    //             if (veh.replaceParkingArea(newParkingArea, errorMsg)) {
    //                 veh.replaceRouteEdges(newRoute, routeCost, savings, getID() + ":" + toString(SUMO_TAG_PARKING_AREA_REROUTE), false, false, false);
    //                 if (oldParkingArea->isReservable()) {
    //                     oldParkingArea->removeSpaceReservation(&veh);
    //                 }
    //                 if (newParkingArea->isReservable()) {
    //                     newParkingArea->addSpaceReservation(&veh);
    //                 }
    //             } else {
    //                 WRITE_WARNING("Vehicle '" + veh.getID() + "' at rerouter '" + getID()
    //                               + "' could not reroute to new parkingArea '" + newParkingArea->getID()
    //                               + "' reason=" + errorMsg + ", time=" + time2string(MSNet::getInstance()->getCurrentTimeStep()) + ".");
    //             }
    //         } else {
    //             if (oldParkingArea && oldParkingArea->isReservable()) {
    //                 oldParkingArea->addSpaceReservation(&veh);
    //             }
    //         }
    //         return false;
    //     }
    //     if (rerouteDef->overtakeLocations.size() > 0) {
    //         if (!tObject.isVehicle()) {
    //             return false;
    //         }
    //         SUMOVehicle& veh = static_cast<SUMOVehicle&>(tObject);
    //         const ConstMSEdgeVector& oldEdges = veh.getRoute().getEdges();
    //         double bestSavings = -std::numeric_limits<double>::max();
    //         double netSaving;
    //         int bestIndex = -1;
    //         MSRouteIterator bestMainStart = oldEdges.end();
    //         std::pair<const SUMOVehicle*, MSRailSignal*> best_overtaker_signal(nullptr, nullptr);
    //         int index = -1;
    //         // sort locations by descending distance to vehicle
    //         std::vector<std::pair<int, int> > sortedLocs;
    //         for (const OvertakeLocation& oloc : rerouteDef->overtakeLocations) {
    //             index++;
    //             if (veh.getLength() > oloc.sidingLength) {
    //                 continue;
    //             }
    //             auto mainStart = std::find(veh.getCurrentRouteEdge(), oldEdges.end(), oloc.main.front());
    //             if (mainStart == oldEdges.end()
    //                     // exit main within
    //                     || ConstMSEdgeVector(mainStart, mainStart + oloc.main.size()) != oloc.cMain
    //                     // stop in main
    //                     || (veh.hasStops() && veh.getNextStop().edge < (mainStart + oloc.main.size()))) {
    //                 //std::cout << SIMTIME << " veh=" << veh.getID() << " wrong route or stop\n";
    //                 continue;
    //             }
    //             // negated iterator distance for descending order
    //             sortedLocs.push_back(std::make_pair(-(int)(mainStart - veh.getCurrentRouteEdge()), index));
    //         }
    //         std::sort(sortedLocs.begin(), sortedLocs.end());
    //         for (const auto& item : sortedLocs) {
    //             index = item.second;
    //             const OvertakeLocation& oloc = rerouteDef->overtakeLocations[index];
    //             auto mainStart = veh.getCurrentRouteEdge() - item.first;  // subtracting negative difference
    //             std::pair<const SUMOVehicle*, MSRailSignal*> overtaker_signal = overtakingTrain(veh, mainStart, oloc, netSaving);
    //             if (overtaker_signal.first != nullptr && netSaving > bestSavings) {
    //                 bestSavings = netSaving;
    //                 bestIndex = index;
    //                 best_overtaker_signal = overtaker_signal;
    //                 bestMainStart = mainStart;
    // #ifdef DEBUG_OVERTAKING
    //                 std::cout << "    newBest index=" << bestIndex << " saving=" << bestSavings << "\n";
    // #endif
    //             }
    //         }
    //         if (bestIndex >= 0) {
    //             const OvertakeLocation& oloc = rerouteDef->overtakeLocations[bestIndex];
    //             if (oloc.defer) {
    //                 return false;
    //             }
    //             SUMOAbstractRouter<MSEdge, SUMOVehicle>& router = hasReroutingDevice
    //                     ? MSRoutingEngine::getRouterTT(veh.getRNGIndex(), veh.getVClass(), rerouteDef->getClosed())
    //                     : MSNet::getInstance()->getRouterTT(veh.getRNGIndex(), rerouteDef->getClosed());
    //             ConstMSEdgeVector newEdges(veh.getCurrentRouteEdge(), bestMainStart);
    //             newEdges.insert(newEdges.end(), oloc.siding.begin(), oloc.siding.end());
    //             newEdges.insert(newEdges.end(), bestMainStart + oloc.main.size(), oldEdges.end());
    //             const double routeCost = router.recomputeCosts(newEdges, &veh, MSNet::getInstance()->getCurrentTimeStep());
    //             const double savings = (router.recomputeCosts(oloc.cMain, &veh, MSNet::getInstance()->getCurrentTimeStep())
    //                                     - router.recomputeCosts(oloc.cSiding, &veh, MSNet::getInstance()->getCurrentTimeStep()));
    //             const std::string info = getID() + ":" + toString(SUMO_TAG_OVERTAKING_REROUTE) + ":" + best_overtaker_signal.first->getID();
    //             veh.replaceRouteEdges(newEdges, routeCost, savings, info, false, false, false);
    //             oloc.sidingExit->addConstraint(veh.getID(), new MSRailSignalConstraint_Predecessor(
    //                                                MSRailSignalConstraint::PREDECESSOR, best_overtaker_signal.second, best_overtaker_signal.first->getID(), 100, true));
    //             resetClosedEdges(hasReroutingDevice, veh);
    //         }
    //         return false;
    //     }
    //     if (rerouteDef->stopAlternatives.size() > 0) {
    //         // somewhat similar to parkProbs but taking into account public transport schedule
    //         if (!tObject.isVehicle()) {
    //             return false;
    //         }
    //         checkStopSwitch(static_cast<MSBaseVehicle&>(tObject), rerouteDef);
    //     }
    //     // get rerouting params
    //     ConstMSRoutePtr newRoute = rerouteDef->routeProbs.getOverallProb() > 0 ? rerouteDef->routeProbs.get() : nullptr;
    //     // we will use the route if given rather than calling our own dijsktra...
    //     if (newRoute != nullptr) {
    // #ifdef DEBUG_REROUTER
    //         if (DEBUGCOND(tObject)) {
    //             std::cout << "    replacedRoute from routeDist " << newRoute->getID() << "\n";
    //         }
    // #endif
    //         tObject.replaceRoute(newRoute, getID());
    //         return false; // XXX another interval could appear later but we would have to track whether the currenty interval was already used
    //     }
    //     const MSEdge* newEdge = lastEdge;
    //     // ok, try using a new destination
    //     double newArrivalPos = -1;
    //     const MSEdgeVector closed = rerouteDef->getClosedEdges();
    //     const bool destUnreachable = std::find(closed.begin(), closed.end(), lastEdge) != closed.end();
    //     bool keepDestination = false;
    //     // if we have a closingReroute, only assign new destinations to vehicles which cannot reach their original destination
    //     // if we have a closingLaneReroute, no new destinations should be assigned
    //     if (closed.empty() || destUnreachable || (rerouteDef->isVia && affected(tObject.getUpcomingEdgeIDs(), closed))) {
    //         newEdge = rerouteDef->edgeProbs.getOverallProb() > 0 ? rerouteDef->edgeProbs.get() : lastEdge;
    //         assert(newEdge != nullptr);
    //         if (newEdge == &mySpecialDest_terminateRoute) {
    //             keepDestination = true;
    //             newEdge = tObject.getEdge();
    //             newArrivalPos = tObject.getPositionOnLane(); // instant arrival
    //         } else if (newEdge == &mySpecialDest_keepDestination || newEdge == lastEdge) {
    //             if (destUnreachable && rerouteDef->permissionsAllowAll) {
    //                 // if permissions aren't set vehicles will simply drive through
    //                 // the closing unless terminated. If the permissions are specified, assume that the user wants
    //                 // vehicles to stand and wait until the closing ends
    //                 WRITE_WARNINGF(TL("Cannot keep destination edge '%' for vehicle '%' due to closed edges. Terminating route."), lastEdge->getID(), tObject.getID());
    //                 newEdge = tObject.getEdge();
    //             } else {
    //                 newEdge = lastEdge;
    //             }
    //         }
    //     }
    //     ConstMSEdgeVector edges;
    //     std::vector<MSTransportableRouter::TripItem> items;
    //     // we have a new destination, let's replace the route (if it is affected)
    //     Prohibitions prohibited = rerouteDef->getClosed();
    //     if (rerouteDef->closed.empty() || destUnreachable || affected(tObject.getUpcomingEdgeIDs(), closed)) {
    //         if (tObject.ignoreTransientPermissions()) {
    //             // after learning about network changes, the driver must no longer forget them
    //             if (!myHaveClosingUntil) {
    //                 tObject.setRoutingMode(tObject.getRoutingMode() & ~libsumo::ROUTING_MODE_IGNORE_TRANSIENT_PERMISSIONS);
    //             } else if (hasReroutingDevice && dynamic_cast<MSDevice_Routing*>(tObject.getDevice(typeid(MSDevice_Routing)))->getPeriod() > 0) {
    //                 WRITE_WARNINGF(TL("Periodic rerouting and rerouting mode 8 for vehicle '%' are incompatible with closingReroute attribute 'until' in rerouter (vehicle may return to an invalid route."),
    //                         tObject.getID(), getID());
    //             }
    //         }
    //         if (tObject.isVehicle()) {
    //             SUMOVehicle& veh = static_cast<SUMOVehicle&>(tObject);
    //             ConstMSEdgeVector prevEdges = veh.getRoute().getEdges();
    //             const bool canChangeDest = rerouteDef->edgeProbs.getOverallProb() > 0;
    //             MSVehicleRouter& router = hasReroutingDevice
    //                                       ? MSRoutingEngine::getRouterTT(veh.getRNGIndex(), veh.getVClass(), prohibited)
    //                                       : MSNet::getInstance()->getRouterTT(veh.getRNGIndex(), prohibited);
    //             bool ok = false;
    //             try {
    //                 ok = veh.reroute(now, getID(), router, false, false, canChangeDest, newEdge);
    //             } catch (ProcessError&) {}
    //             if (!ok && !keepDestination && canChangeDest) {
    //                 // destination unreachable due to closed intermediate edges. pick among alternative targets
    //                 RandomDistributor<MSEdge*> edgeProbs2 = rerouteDef->edgeProbs;
    //                 edgeProbs2.remove(const_cast<MSEdge*>(newEdge));
    //                 while (!ok && edgeProbs2.getVals().size() > 0) {
    //                     newEdge = edgeProbs2.get();
    //                     edgeProbs2.remove(const_cast<MSEdge*>(newEdge));
    //                     if (newEdge == &mySpecialDest_terminateRoute) {
    //                         newEdge = veh.getEdge();
    //                         newArrivalPos = veh.getPositionOnLane(); // instant arrival
    //                         while (veh.hasStops()) {
    //                             veh.abortNextStop();
    //                         }
    //                     }
    //                     if (newEdge == &mySpecialDest_keepDestination && !rerouteDef->permissionsAllowAll) {
    //                         newEdge = lastEdge;
    //                         break;
    //                     }
    //                     try {
    //                         ok = veh.reroute(now, getID(), router, false, false, true, newEdge);
    //                     } catch (ProcessError&) {}
    //                 }
    //             }
    //             resetClosedEdges(hasReroutingDevice, tObject);
    //             if (ok) {
    //                 // since the old route was closed, savings would be infinite. This isn't useful
    //                 const double previousCost = router.recomputeCosts(prevEdges, &veh, MSNet::getInstance()->getCurrentTimeStep());
    //                 const double savings = previousCost - veh.getRoute().getCosts();
    //                 const_cast<MSRoute&>(veh.getRoute()).setSavings(savings);
    //             }
    //             if (!rerouteDef->isVia) {
    // #ifdef DEBUG_REROUTER
    //                 if (DEBUGCOND(tObject)) std::cout << "   rerouting:  newDest=" << newEdge->getID()
    //                                                       << " newEdges=" << toString(edges)
    //                                                       << " newArrivalPos=" << newArrivalPos << " numClosed=" << rerouteDef->closed.size()
    //                                                       << " destUnreachable=" << destUnreachable << " containsClosed=" << veh.getRoute().containsAnyOf(closed) << "\n";
    // #endif
    //                 if (ok && newArrivalPos != -1) {
    //                     // must be called here because replaceRouteEdges may also set the arrivalPos
    //                     veh.setArrivalPos(newArrivalPos);
    //                 }
    //
    //             }
    //         } else {
    //             // person rerouting here
    //             MSTransportableRouter& router = hasReroutingDevice
    //                                             ? MSRoutingEngine::getIntermodalRouterTT(tObject.getRNGIndex(), prohibited)
    //                                             : MSNet::getInstance()->getIntermodalRouter(tObject.getRNGIndex(), 0, prohibited);
    //             const bool success = router.compute(tObject.getEdge(), newEdge, tObject.getPositionOnLane(), "",
    //                                                 rerouteDef->isVia ? newEdge->getLength() / 2. : tObject.getParameter().arrivalPos, "",
    //                                                 tObject.getMaxSpeed(), nullptr, tObject.getVTypeParameter(), 0, now, items);
    //             if (!rerouteDef->isVia) {
    //                 if (success) {
    //                     for (const MSTransportableRouter::TripItem& it : items) {
    //                         if (!it.edges.empty() && !edges.empty() && edges.back() == it.edges.front()) {
    //                             edges.pop_back();
    //                         }
    //                         edges.insert(edges.end(), std::make_move_iterator(it.edges.begin()), std::make_move_iterator(it.edges.end()));
    //                         if (!edges.empty()) {
    //                             static_cast<MSPerson&>(tObject).replaceWalk(edges, tObject.getPositionOnLane(), 0, 1);
    //                         }
    //                     }
    //                 } else {
    //                     // maybe the pedestrian model still finds a way (JuPedSim)
    //                     static_cast<MSPerson&>(tObject).replaceWalk({tObject.getEdge(), newEdge}, tObject.getPositionOnLane(), 0, 1);
    //                 }
    //             }
    //         }
    //         if (!prohibited.empty()) {
    //             resetClosedEdges(hasReroutingDevice, tObject);
    //         }
    //     }
    //     // it was only a via so calculate the remaining part
    //     if (rerouteDef->isVia && newEdge != lastEdge) {
    //         if (tObject.isVehicle()) {
    //             SUMOVehicle& veh = static_cast<SUMOVehicle&>(tObject);
    //             if (!edges.empty()) {
    //                 edges.pop_back();
    //             }
    //             MSVehicleRouter& router = hasReroutingDevice
    //                                       ? MSRoutingEngine::getRouterTT(veh.getRNGIndex(), veh.getVClass(), prohibited)
    //                                       : MSNet::getInstance()->getRouterTT(veh.getRNGIndex(), prohibited);
    //             router.compute(newEdge, lastEdge, &veh, now, edges);
    //             const double routeCost = router.recomputeCosts(edges, &veh, now);
    //             hasReroutingDevice
    //             ? MSRoutingEngine::getRouterTT(veh.getRNGIndex(), veh.getVClass())
    //             : MSNet::getInstance()->getRouterTT(veh.getRNGIndex()); // reset closed edges
    //             const bool useNewRoute = veh.replaceRouteEdges(edges, routeCost, 0, getID());
    // #ifdef DEBUG_REROUTER
    //             if (DEBUGCOND(tObject)) std::cout << "   rerouting:  newDest=" << newEdge->getID()
    //                                                   << " newEdges=" << toString(edges)
    //                                                   << " useNewRoute=" << useNewRoute << " newArrivalPos=" << newArrivalPos << " numClosed=" << rerouteDef->closed.size()
    //                                                   << " destUnreachable=" << destUnreachable << " containsClosed=" << veh.getRoute().containsAnyOf(closed) << "\n";
    // #endif
    //             if (useNewRoute && newArrivalPos != -1) {
    //                 // must be called here because replaceRouteEdges may also set the arrivalPos
    //                 veh.setArrivalPos(newArrivalPos);
    //             }
    //         } else {
    //             // person rerouting here
    //             bool success = !items.empty();
    //             if (success) {
    //                 MSTransportableRouter& router = hasReroutingDevice
    //                                                 ? MSRoutingEngine::getIntermodalRouterTT(tObject.getRNGIndex(), prohibited)
    //                                                 : MSNet::getInstance()->getIntermodalRouter(tObject.getRNGIndex(), 0, prohibited);
    //                 success = router.compute(newEdge, lastEdge, newEdge->getLength() / 2., "",
    //                                          tObject.getParameter().arrivalPos, "",
    //                                          tObject.getMaxSpeed(), nullptr, tObject.getVTypeParameter(), 0, now, items);
    //             }
    //             if (success) {
    //                 for (const MSTransportableRouter::TripItem& it : items) {
    //                     if (!it.edges.empty() && !edges.empty() && edges.back() == it.edges.front()) {
    //                         edges.pop_back();
    //                     }
    //                     edges.insert(edges.end(), std::make_move_iterator(it.edges.begin()), std::make_move_iterator(it.edges.end()));
    //                 }
    //                 if (!edges.empty()) {
    //                     static_cast<MSPerson&>(tObject).replaceWalk(edges, tObject.getPositionOnLane(), 0, 1);
    //                 }
    //             } else {
    //                 // maybe the pedestrian model still finds a way (JuPedSim)
    //                 static_cast<MSPerson&>(tObject).replaceWalk({tObject.getEdge(), newEdge, lastEdge}, tObject.getPositionOnLane(), 0, 1);
    //             }
    //         }
    //         if (!prohibited.empty()) {
    //             resetClosedEdges(hasReroutingDevice, tObject);
    //         }
    //     }
    //     return false; // XXX another interval could appear later but we would have to track whether the currenty interval was already used
    // }
    #[allow(unused_variables)]
    pub fn trigger_routing(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification) -> bool {
        // shortcut: router prohibitions (SUMOAbstractRouter_Prohibitions) are Unmapped (`()`), so every router lookup receives `&()` instead of the closed-edge prohibitions; upgrade when that alias is ported
        let tObject = veh;
        let net = MSNet::get_instance();
        if !self.applies(tObject) {
            return false;
        }
        if self.myRadius != f64::MAX && tObject.get_position(0.0).distance_to(&self.myPosition) > self.myRadius {
            return true;
        }
        // check whether the vehicle shall be rerouted
        let now = net.get_current_time_step();
        let rerouteDef = self.get_current_reroute(now, tObject);
        if rerouteDef.is_null() {
            return true; // an active interval could appear later
        }
        let prob = if self.myAmInUserMode { self.myUserProbability } else { self.myProbability };
        if prob < 1.0 && tObject.get_rng().m().rand() > prob {
            return false; // XXX another interval could appear later but we would have to track whether the current interval was already tried
        }
        if self.myTimeThreshold > 0 && std::cmp::max(tObject.get_waiting_time(false), tObject.get_waiting_time(true)) < self.myTimeThreshold {
            return true; // waiting time may be reached later
        }
        if reason == MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE {
            return false;
        }
        // if we have a closingLaneReroute it may still affect the network topology (i.e. by closing a turning lane).
        // shortcut: C++ tObject.getDevice(typeid(MSDevice_Routing)); get_device takes an Unmapped type_info, so this is only a presence check on `&()`; upgrade when type-keyed device lookup is ported
        // C++ uses tObject.getDevice(typeid(MSDevice_Routing)) != nullptr. Our get_device takes an
        // Unmapped type_info and always yields null, so check the device by name instead (the name
        // MSBaseVehicle::createDevice compares against is "rerouting"). Without this the rerouter
        // believed no vehicle had a routing device and rerouted vehicles where C++ would not.
        let hasReroutingDevice = if tObject.is_vehicle() {
            let bv = Ptr::from_mut(tObject).cast::<crate::microsim::ms_base_vehicle::MSBaseVehicle>();
            bv.get().map(|x| x.has_rerouting_device()).unwrap_or(false)
        } else {
            false
        };
        let lastEdge = tObject.get_reroute_destination();

        if rerouteDef.parkProbs.get_overall_prob() > 0.0 {
            if !tObject.is_vehicle() {
                return false;
            }
            // static_cast<SUMOVehicle&>(tObject)
            let vehp: Ptr<SUMOVehicle> = Ptr::from_raw(tObject as *mut SUMOTrafficObject as *mut SUMOVehicle);
            let v = vehp.m();
            let mut newDestination = false;
            let mut newRoute: ConstMSEdgeVector = Vec::new();
            let oldParkingArea = v.get_next_parking_area();
            let newParkingArea = self.reroute_parking_area(rerouteDef, v, &mut newDestination, &mut newRoute);
            if !newParkingArea.is_null() {
                // adapt plans of any riders
                let persons: Vec<Ptr<MSTransportable>> = v.get_persons().clone();
                for p in persons {
                    p.m().reroute_parking_area(Ptr::from_raw(oldParkingArea.as_raw() as *mut MSStoppingPlace), Ptr::from_raw(newParkingArea.as_raw() as *mut MSStoppingPlace));
                }

                if newDestination && v.get_parameter().arrivalPosProcedure != ArrivalPosDefinition::DEFAULT {
                    // update arrival parameters (C++ `new SUMOVehicleParameter`, owned by the vehicle)
                    let mut newParameter = v.get_parameter().clone();
                    newParameter.arrivalPosProcedure = ArrivalPosDefinition::GIVEN;
                    newParameter.arrivalPos = newParkingArea.get_end_lane_position();
                    v.replace_parameter(Ptr::from_raw(Box::leak(Box::new(newParameter))));
                }

                let router: &mut MSVehicleRouter = if hasReroutingDevice {
                    MSRoutingEngine::get_router_tt(v.get_rng_index(), v.get_v_class(), &Default::default())
                } else {
                    net.get_router_tt(v.get_rng_index(), &Default::default())
                };
                let routeCost = router.recompute_costs(&newRoute, Ptr::from_mut(v), now, Ptr::default());
                let routeEdges: ConstMSEdgeVector = v.get_route().get_edges().clone();
                let prevEdges: ConstMSEdgeVector = routeEdges[v.get_route_position() as usize..].to_vec();
                self.reset_closed_edges(hasReroutingDevice, tObject);
                let previousCost = router.recompute_costs(&prevEdges, Ptr::from_mut(v), now, Ptr::default());
                let savings = previousCost - routeCost;

                let mut errorMsg = String::new();
                if v.replace_parking_area(newParkingArea, &mut errorMsg) {
                    let info = format!("{}:{}", self.base.get_id(), Tags.get_string(SUMO_TAG_PARKING_AREA_REROUTE));
                    v.replace_route_edges(&mut newRoute, routeCost, savings, &info, false, false, false, Ptr::default());
                    if oldParkingArea.is_reservable() {
                        oldParkingArea.m().remove_space_reservation(Ptr::from_mut(v));
                    }
                    if newParkingArea.is_reservable() {
                        newParkingArea.m().add_space_reservation(Ptr::from_mut(v));
                    }
                } else {
                    utils::common::msg_handler::write_warning(&format!("Vehicle '{}' at rerouter '{}' could not reroute to new parkingArea '{}' reason={}, time={}.", v.get_id(), self.base.get_id(), newParkingArea.get_id(), errorMsg, time2string(now)));
                }
            } else if !oldParkingArea.is_null() && oldParkingArea.is_reservable() {
                oldParkingArea.m().add_space_reservation(Ptr::from_mut(v));
            }
            return false;
        }
        if rerouteDef.overtakeLocations.len() > 0 {
            if !tObject.is_vehicle() {
                return false;
            }
            let vehp: Ptr<SUMOVehicle> = Ptr::from_raw(tObject as *mut SUMOTrafficObject as *mut SUMOVehicle);
            let v = vehp.m();
            let oldEdges: ConstMSEdgeVector = v.get_route().get_edges().clone();
            let mut bestSavings = -f64::MAX;
            let mut bestIndex: i32 = -1;
            let mut bestMainStart: usize = oldEdges.len(); // C++ oldEdges.end()
            let mut best_overtaker_signal: (Ptr<SUMOVehicle>, Ptr<MSRailSignal>) = (Ptr::default(), Ptr::default());
            let pos = v.get_route_position() as usize;
            let mut index: i32 = -1;
            // sort locations by descending distance to vehicle
            let mut sortedLocs: Vec<(i32, i32)> = Vec::new();
            for oloc in &rerouteDef.overtakeLocations {
                index += 1;
                if v.get_length() > oloc.sidingLength {
                    continue;
                }
                let mainStart = oldEdges[pos..].iter().position(|e| *e == oloc.main[0]).map(|i| pos + i);
                let mainStart = match mainStart {
                    None => continue, // mainStart == oldEdges.end()
                    Some(i) => i,
                };
                let end = mainStart + oloc.main.len();
                // exit main within
                // stop in main
                if oldEdges.get(mainStart..end) != Some(&oloc.cMain[..]) || (v.has_stops() && v.get_next_stop().edge < end) {
                    continue;
                }
                // negated iterator distance for descending order
                sortedLocs.push((-((mainStart as i32) - (pos as i32)), index));
            }
            sortedLocs.sort();
            for item in &sortedLocs {
                index = item.1;
                let oloc = &rerouteDef.overtakeLocations[index as usize];
                let mainStart = ((pos as i32) - item.0) as usize; // subtracting negative difference
                let mut netSaving: f64 = 0.0;
                let overtaker_signal = self.overtaking_train(v, mainStart, oloc, &mut netSaving);
                if !overtaker_signal.0.is_null() && netSaving > bestSavings {
                    bestSavings = netSaving;
                    bestIndex = index;
                    best_overtaker_signal = overtaker_signal;
                    bestMainStart = mainStart;
                }
            }
            if bestIndex >= 0 {
                let oloc = &rerouteDef.overtakeLocations[bestIndex as usize];
                if oloc.defer {
                    return false;
                }
                let router: &mut MSVehicleRouter = if hasReroutingDevice {
                    MSRoutingEngine::get_router_tt(v.get_rng_index(), v.get_v_class(), &Default::default())
                } else {
                    net.get_router_tt(v.get_rng_index(), &Default::default())
                };
                let mut newEdges: ConstMSEdgeVector = Vec::new();
                newEdges.extend_from_slice(&oldEdges[pos..bestMainStart]);
                newEdges.extend(oloc.siding.iter().copied());
                newEdges.extend_from_slice(&oldEdges[bestMainStart + oloc.main.len()..]);
                let routeCost = router.recompute_costs(&newEdges, Ptr::from_mut(v), now, Ptr::default());
                let savings = router.recompute_costs(&oloc.cMain, Ptr::from_mut(v), now, Ptr::default()) - router.recompute_costs(&oloc.cSiding, Ptr::from_mut(v), now, Ptr::default());
                let info = format!("{}:{}:{}", self.base.get_id(), Tags.get_string(SUMO_TAG_OVERTAKING_REROUTE), best_overtaker_signal.0.get_id());
                v.replace_route_edges(&mut newEdges, routeCost, savings, &info, false, false, false, Ptr::default());
                let constraint = Ptr::from_raw(Box::leak(Box::new(MSRailSignalConstraint_Predecessor::new(MSRailSignalConstraint_ConstraintType::PREDECESSOR, best_overtaker_signal.1, best_overtaker_signal.0.get_id(), 100, true))) as *mut MSRailSignalConstraint_Predecessor as *mut MSRailSignalConstraint);
                oloc.sidingExit.m().add_constraint(v.get_id(), constraint);
                self.reset_closed_edges(hasReroutingDevice, tObject);
            }
            return false;
        }
        if rerouteDef.stopAlternatives.len() > 0 {
            // somewhat similar to parkProbs but taking into account public transport schedule
            if !tObject.is_vehicle() {
                return false;
            }
            let ego = Ptr::from_raw(tObject as *mut SUMOTrafficObject as *mut MSBaseVehicle);
            self.check_stop_switch(ego.m(), rerouteDef);
        }
        // get rerouting params
        let newRoute: Option<ConstMSRoutePtr> = if rerouteDef.routeProbs.get_overall_prob() > 0.0 { Some(rerouteDef.routeProbs.get(None).unwrap()) } else { None };
        // we will use the route if given rather than calling our own dijsktra...
        if let Some(newRoute) = newRoute {
            tObject.replace_route(newRoute, self.base.get_id(), false, 0, true, true, Ptr::default());
            return false; // XXX another interval could appear later but we would have to track whether the currenty interval was already used
        }
        let mut newEdge: Ptr<MSEdge> = lastEdge;
        // ok, try using a new destination
        let mut newArrivalPos: f64 = -1.0;
        let closed: MSEdgeVector = rerouteDef.get_closed_edges();
        let destUnreachable = closed.contains(&lastEdge);
        let mut keepDestination = false;
        // if we have a closingReroute, only assign new destinations to vehicles which cannot reach their original destination
        // if we have a closingLaneReroute, no new destinations should be assigned
        if closed.is_empty() || destUnreachable || (rerouteDef.isVia && Self::affected(&tObject.get_upcoming_edge_i_ds(), &closed)) {
            newEdge = if rerouteDef.edgeProbs.get_overall_prob() > 0.0 { rerouteDef.edgeProbs.get(None).unwrap() } else { lastEdge };
            assert!(!newEdge.is_null());
            if newEdge == special_dest_terminate_route() {
                keepDestination = true;
                newEdge = tObject.get_edge();
                newArrivalPos = tObject.get_position_on_lane(); // instant arrival
            } else if newEdge == special_dest_keep_destination() || newEdge == lastEdge {
                if destUnreachable && rerouteDef.permissionsAllowAll {
                    // if permissions aren't set vehicles will simply drive through
                    // the closing unless terminated. If the permissions are specified, assume that the user wants
                    // vehicles to stand and wait until the closing ends
                    utils::common::msg_handler::write_warning(&format!("Cannot keep destination edge '{}' for vehicle '{}' due to closed edges. Terminating route.", lastEdge.get_id(), tObject.get_id()));
                    newEdge = tObject.get_edge();
                } else {
                    newEdge = lastEdge;
                }
            }
        }
        let mut edges: ConstMSEdgeVector = Vec::new();
        let mut items: Vec<IntermodalRouter_TripItem> = Vec::new();
        // we have a new destination, let's replace the route (if it is affected)
        let prohibited = rerouteDef.get_closed();
        if rerouteDef.closed.is_empty() || destUnreachable || Self::affected(&tObject.get_upcoming_edge_i_ds(), &closed) {
            if tObject.ignore_transient_permissions() {
                // after learning about network changes, the driver must no longer forget them
                if !self.myHaveClosingUntil {
                    tObject.set_routing_mode(tObject.get_routing_mode() & !ROUTING_MODE_IGNORE_TRANSIENT_PERMISSIONS);
                }
                // shortcut: C++ warns when hasReroutingDevice has a periodic rerouting period > 0 (MSDevice_Routing lookup by type is not ported); that warning is skipped
            }
            if tObject.is_vehicle() {
                let vehp: Ptr<SUMOVehicle> = Ptr::from_raw(tObject as *mut SUMOTrafficObject as *mut SUMOVehicle);
                let v = vehp.m();
                let prevEdges: ConstMSEdgeVector = v.get_route().get_edges().clone();
                let canChangeDest = rerouteDef.edgeProbs.get_overall_prob() > 0.0;
                let routerProhibited: SUMOAbstractRouter_Prohibitions = prohibited.iter().map(|(e, p)| (e.cast(), p.clone())).collect();
                let router: &mut MSVehicleRouter = if hasReroutingDevice {
                    MSRoutingEngine::get_router_tt(v.get_rng_index(), v.get_v_class(), &routerProhibited)
                } else {
                    net.get_router_tt(v.get_rng_index(), &routerProhibited)
                };
                let mut ok = false;
                // C++ try { ... } catch (ProcessError&) {}: reroute() reports failure through its return value here
                ok = v.reroute(now, self.base.get_id(), router, false, false, canChangeDest, newEdge);
                if !ok && !keepDestination && canChangeDest {
                    // destination unreachable due to closed intermediate edges. pick among alternative targets
                    let mut edgeProbs2 = rerouteDef.edgeProbs.clone();
                    edgeProbs2.remove(&newEdge);
                    while !ok && !edgeProbs2.get_vals().is_empty() {
                        newEdge = edgeProbs2.get(None).unwrap();
                        edgeProbs2.remove(&newEdge);
                        if newEdge == special_dest_terminate_route() {
                            newEdge = v.get_edge();
                            newArrivalPos = v.get_position_on_lane(); // instant arrival
                            while v.has_stops() {
                                v.abort_next_stop(0);
                            }
                        }
                        if newEdge == special_dest_keep_destination() && !rerouteDef.permissionsAllowAll {
                            newEdge = lastEdge;
                            break;
                        }
                        ok = v.reroute(now, self.base.get_id(), router, false, false, true, newEdge);
                    }
                }
                self.reset_closed_edges(hasReroutingDevice, tObject);
                if ok {
                    // since the old route was closed, savings would be infinite. This isn't useful
                    let previousCost = router.recompute_costs(&prevEdges, Ptr::from_mut(v), now, Ptr::default());
                    let savings = previousCost - v.get_route().get_costs();
                    Ptr::from_raw(v.get_route() as *const MSRoute as *mut MSRoute).m().set_savings(savings); // const_cast<MSRoute&>
                }
                if !rerouteDef.isVia && ok && newArrivalPos != -1.0 {
                    // must be called here because replaceRouteEdges may also set the arrivalPos
                    v.set_arrival_pos(newArrivalPos);
                }
            } else {
                // person rerouting here
                let router: &mut MSTransportableRouter = if hasReroutingDevice {
                    MSRoutingEngine::get_intermodal_router_tt(tObject.get_rng_index(), &Default::default())
                } else {
                    net.get_intermodal_router(tObject.get_rng_index(), 0, &Default::default())
                };
                let person = Ptr::from_raw(tObject as *mut SUMOTrafficObject as *mut MSPerson);
                let success = router.compute(tObject.get_edge(), newEdge, tObject.get_position_on_lane(), "",
                    if rerouteDef.isVia { newEdge.get_length() / 2. } else { tObject.get_parameter().arrivalPos }, "",
                    tObject.get_max_speed(), Ptr::default(), tObject.get_v_type_parameter(), 0, now, &mut items, 0.);
                if !rerouteDef.isVia {
                    if success {
                        for it in &items {
                            if !it.edges.is_empty() && !edges.is_empty() && edges.last() == it.edges.first().map(|e| e.cast::<MSEdge>()).as_ref() {
                                edges.pop();
                            }
                            edges.extend(it.edges.iter().map(|e| e.cast::<MSEdge>()));
                            if !edges.is_empty() {
                                person.m().replace_walk(&edges, tObject.get_position_on_lane(), 0, 1);
                            }
                        }
                    } else {
                        // maybe the pedestrian model still finds a way (JuPedSim)
                        person.m().replace_walk(&vec![tObject.get_edge(), newEdge], tObject.get_position_on_lane(), 0, 1);
                    }
                }
            }
            if !prohibited.is_empty() {
                self.reset_closed_edges(hasReroutingDevice, tObject);
            }
        }
        // it was only a via so calculate the remaining part
        if rerouteDef.isVia && newEdge != lastEdge {
            if tObject.is_vehicle() {
                let vehp: Ptr<SUMOVehicle> = Ptr::from_raw(tObject as *mut SUMOTrafficObject as *mut SUMOVehicle);
                let v = vehp.m();
                if !edges.is_empty() {
                    edges.pop();
                }
                let router: &mut MSVehicleRouter = if hasReroutingDevice {
                    MSRoutingEngine::get_router_tt(v.get_rng_index(), v.get_v_class(), &Default::default())
                } else {
                    net.get_router_tt(v.get_rng_index(), &Default::default())
                };
                router.compute(newEdge, lastEdge, Ptr::from_mut(v), now, &mut edges, false);
                let routeCost = router.recompute_costs(&edges, Ptr::from_mut(v), now, Ptr::default());
                if hasReroutingDevice {
                    MSRoutingEngine::get_router_tt(v.get_rng_index(), v.get_v_class(), &Default::default());
                } else {
                    net.get_router_tt(v.get_rng_index(), &Default::default());
                } // reset closed edges
                let useNewRoute = v.replace_route_edges(&mut edges, routeCost, 0.0, self.base.get_id(), false, false, false, Ptr::default());
                if useNewRoute && newArrivalPos != -1.0 {
                    // must be called here because replaceRouteEdges may also set the arrivalPos
                    v.set_arrival_pos(newArrivalPos);
                }
            } else {
                // person rerouting here
                let mut success = !items.is_empty();
                if success {
                    let router: &mut MSTransportableRouter = if hasReroutingDevice {
                        MSRoutingEngine::get_intermodal_router_tt(tObject.get_rng_index(), &Default::default())
                    } else {
                        net.get_intermodal_router(tObject.get_rng_index(), 0, &Default::default())
                    };
                    success = router.compute(newEdge, lastEdge, newEdge.get_length() / 2., "",
                        tObject.get_parameter().arrivalPos, "",
                        tObject.get_max_speed(), Ptr::default(), tObject.get_v_type_parameter(), 0, now, &mut items, 0.);
                }
                let person = Ptr::from_raw(tObject as *mut SUMOTrafficObject as *mut MSPerson);
                if success {
                    for it in &items {
                        if !it.edges.is_empty() && !edges.is_empty() && edges.last() == it.edges.first().map(|e| e.cast::<MSEdge>()).as_ref() {
                            edges.pop();
                        }
                        edges.extend(it.edges.iter().map(|e| e.cast::<MSEdge>()));
                    }
                    if !edges.is_empty() {
                        person.m().replace_walk(&edges, tObject.get_position_on_lane(), 0, 1);
                    }
                } else {
                    // maybe the pedestrian model still finds a way (JuPedSim)
                    person.m().replace_walk(&vec![tObject.get_edge(), newEdge, lastEdge], tObject.get_position_on_lane(), 0, 1);
                }
            }
            if !prohibited.is_empty() {
                self.reset_closed_edges(hasReroutingDevice, tObject);
            }
        }
        false // XXX another interval could appear later but we would have to track whether the currenty interval was already used
    }

    /// C++: `const RerouteInterval * MSTriggeredRerouter::getCurrentReroute(SUMOTime, SUMOTrafficObject &) const` [const]
    /// Returns the rerouting definition valid for the given time and object, nullptr if none
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_current_reroute(&self, time: SUMOTime, obj: &mut SUMOTrafficObject) -> Ptr<MSTriggeredRerouter_RerouteInterval> {
        // C++: MSTriggeredRerouter::getCurrentReroute(SUMOTime time, SUMOTrafficObject& obj) const
        for ri in &self.myIntervals {
            if ri.begin <= time && ri.end > time {
                if ri.edgeProbs.get_overall_prob() > 0.0 || ri.routeProbs.get_overall_prob() > 0.0 || ri.parkProbs.get_overall_prob() > 0.0 || !ri.stopAlternatives.is_empty() {
                    return Ptr::new(ri);
                }
                if !ri.closed.is_empty() || !ri.closedLanesAffected.is_empty() || !ri.overtakeLocations.is_empty() {
                    let edgeIndices = obj.get_upcoming_edge_i_ds();
                    if Self::affected(&edgeIndices, &ri.get_closed_edges()) || Self::affected(&edgeIndices, &ri.closedLanesAffected) {
                        return Ptr::new(ri);
                    }
                    for oloc in &ri.overtakeLocations {
                        if Self::affected(&edgeIndices, &oloc.main) {
                            return Ptr::new(ri);
                        }
                    }
                }
            }
        }
        Ptr::default()
    }

    /// C++: `SUMOTime MSTriggeredRerouter::setPermissions(const SUMOTime)`
    /// Sets the edge permission if there are any defined in the closingEdge
    // SUMOTime
    // MSTriggeredRerouter::setPermissions(const SUMOTime currentTime) {
    //     bool updateVehicles = false;
    //     for (const RerouteInterval& i : myIntervals) {
    //         if (i.begin == currentTime && !(i.closed.empty() && i.closedLanes.empty()) /*&& i.permissions != SVCAll*/) {
    //             for (const auto& settings : i.closed) {
    //                 if (settings.second.first != SVC_UNSPECIFIED) {
    //                     for (MSLane* lane : settings.first->getLanes()) {
    //                         //std::cout << SIMTIME << " closing: intervalID=" << i.id << " lane=" << lane->getID() << " prevPerm=" << getVehicleClassNames(lane->getPermissions()) << " new=" << getVehicleClassNames(settings.second.first) << "\n";
    //                         lane->setPermissions(settings.second.first, i.id);
    //                     }
    //                     settings.first->rebuildAllowedLanes();
    //                     updateVehicles = true;
    //                 }
    //             }
    //             for (std::pair<MSLane*, SVCPermissions> settings : i.closedLanes) {
    //                 settings.first->setPermissions(settings.second, i.id);
    //                 settings.first->getEdge().rebuildAllowedLanes();
    //                 updateVehicles = true;
    //             }
    //             MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(
    //                 new WrappingCommand<MSTriggeredRerouter>(this, &MSTriggeredRerouter::setPermissions), i.end);
    //         }
    //         if (i.end == currentTime && !(i.closed.empty() && i.closedLanes.empty()) /*&& i.permissions != SVCAll*/) {
    //             for (auto settings : i.closed) {
    //                 if (settings.second.first != SVC_UNSPECIFIED) {
    //                     for (MSLane* lane : settings.first->getLanes()) {
    //                         lane->resetPermissions(i.id);
    //                         //std::cout << SIMTIME << " opening: intervalID=" << i.id << " lane=" << lane->getID() << " restore prevPerm=" << getVehicleClassNames(lane->getPermissions()) << "\n";
    //                     }
    //                     settings.first->rebuildAllowedLanes();
    //                     updateVehicles = true;
    //                 }
    //             }
    //             for (std::pair<MSLane*, SVCPermissions> settings : i.closedLanes) {
    //                 settings.first->resetPermissions(i.id);
    //                 settings.first->getEdge().rebuildAllowedLanes();
    //                 updateVehicles = true;
    //             }
    //         }
    //     }
    //     if (updateVehicles) {
    //         MSNet::getInstance()->setPermissionsFound();
    //         // only vehicles on the affected lanes had their bestlanes updated so far
    //         for (MSEdge* e : myEdges) {
    //             // also updates vehicles
    //             e->rebuildAllowedTargets();
    //         }
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn set_permissions(&mut self, currentTime: SUMOTime) -> SUMOTime {
        let mut updateVehicles = false;
        let this: Ptr<MSTriggeredRerouter> = Ptr::from_mut(self);
        for i in &self.myIntervals {
            if i.begin == currentTime && !(i.closed.is_empty() && i.closedLanes.is_empty()) {
                for (edge, settings) in &i.closed {
                    if settings.0 != SVC_UNSPECIFIED {
                        for lane in edge.get_lanes() {
                            lane.m().set_permissions(settings.0, i.id);
                        }
                        edge.m().rebuild_allowed_lanes(false, false);
                        updateVehicles = true;
                    }
                }
                for (lane, perm) in &i.closedLanes {
                    lane.m().set_permissions(*perm, i.id);
                    lane.get_edge().rebuild_allowed_lanes(false, false);
                    updateVehicles = true;
                }
                let cmd: *mut dyn Command = Box::into_raw(Box::new(MSTriggeredRerouter_SetPermissionsCommand { rerouter: this }));
                MSNet::get_instance().m().get_begin_of_timestep_events().m().add_event(Ptr::from_raw(cmd), i.end);
            }
            if i.end == currentTime && !(i.closed.is_empty() && i.closedLanes.is_empty()) {
                for (edge, settings) in &i.closed {
                    if settings.0 != SVC_UNSPECIFIED {
                        for lane in edge.get_lanes() {
                            lane.m().reset_permissions(i.id);
                        }
                        edge.m().rebuild_allowed_lanes(false, false);
                        updateVehicles = true;
                    }
                }
                for (lane, _perm) in &i.closedLanes {
                    lane.m().reset_permissions(i.id);
                    lane.get_edge().rebuild_allowed_lanes(false, false);
                    updateVehicles = true;
                }
            }
        }
        if updateVehicles {
            MSNet::get_instance().set_permissions_found();
            // only vehicles on the affected lanes had their bestlanes updated so far
            for e in &self.myEdges {
                // also updates vehicles
                e.m().rebuild_allowed_targets(true);
            }
        }
        0
    }

    /// C++: `const RerouteInterval * MSTriggeredRerouter::getCurrentReroute(SUMOTime) const` [const]
    /// Returns the rerouting definition valid for the given time, 0 if none
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_current_reroute_2(&self, time: SUMOTime) -> Ptr<MSTriggeredRerouter_RerouteInterval> {
        // C++: MSTriggeredRerouter::getCurrentReroute(SUMOTime time) const
        for ri in &self.myIntervals {
            if ri.begin <= time && ri.end > time {
                if ri.edgeProbs.get_overall_prob() != 0.0 || ri.routeProbs.get_overall_prob() != 0.0 || ri.parkProbs.get_overall_prob() != 0.0
                    || !ri.closed.is_empty() || !ri.closedLanesAffected.is_empty() || !ri.overtakeLocations.is_empty() {
                    return Ptr::new(ri);
                }
            }
        }
        Ptr::default()
    }

    /// C++: `void MSTriggeredRerouter::setUserMode(bool)`
    /// Sets whether the process is currently steered by the user
    // void
    // MSTriggeredRerouter::setUserMode(bool val) {
    //     myAmInUserMode = val;
    // }
    #[allow(unused_variables)]
    pub fn set_user_mode(&mut self, val: bool) {
        self.myAmInUserMode = val;
    }

    /// C++: `void MSTriggeredRerouter::setUserUsageProbability(double)`
    /// Sets the probability with which a vehicle is rerouted given by the user
    // void
    // MSTriggeredRerouter::setUserUsageProbability(double prob) {
    //     myUserProbability = prob;
    // }
    #[allow(unused_variables)]
    pub fn set_user_usage_probability(&mut self, prob: f64) {
        self.myUserProbability = prob;
    }

    /// C++: `bool MSTriggeredRerouter::inUserMode() const` [const]
    /// Returns whether the user is setting the rerouting probability
    // bool
    // MSTriggeredRerouter::inUserMode() const {
    //     return myAmInUserMode;
    // }
    #[allow(unused_variables)]
    pub fn in_user_mode(&self) -> bool {
        self.myAmInUserMode
    }

    /// C++: `double MSTriggeredRerouter::getProbability() const` [const]
    /// Returns the rerouting probability
    // double
    // MSTriggeredRerouter::getProbability() const {
    //     return myAmInUserMode ? myUserProbability : myProbability;
    // }
    #[allow(unused_variables)]
    pub fn get_probability(&self) -> f64 {
        if self.myAmInUserMode { self.myUserProbability } else { self.myProbability }
    }

    /// C++: `double MSTriggeredRerouter::getUserProbability() const` [const]
    /// Returns the rerouting probability given by the user
    // double
    // MSTriggeredRerouter::getUserProbability() const {
    //     return myUserProbability;
    // }
    #[allow(unused_variables)]
    pub fn get_user_probability(&self) -> f64 {
        self.myUserProbability
    }

    /// C++: `bool MSTriggeredRerouter::isParkingRerouter() const` [const, virtual]
    // bool isParkingRerouter() const {
    //         return myHaveParkProbs;
    //     }
    #[allow(unused_variables)]
    pub fn is_parking_rerouter(&self) -> bool {
        self.myHaveParkProbs
    }

    /// C++: `const Position & MSTriggeredRerouter::getPosition()`
    // const Position& getPosition() {
    //         return myPosition;
    //     }
    #[allow(unused_variables)]
    pub fn get_position(&mut self) -> &Position {
        &self.myPosition
    }

    /// C++: `double MSTriggeredRerouter::getStoppingPlaceOccupancy(MSStoppingPlace *, const SUMOVehicle *)` [virtual]
    /// Return the number of occupied places of the stopping place
    // double
    // MSTriggeredRerouter::getStoppingPlaceOccupancy(MSStoppingPlace* sp, const SUMOVehicle* veh) {
    //     return (sp->getElement() == SUMO_TAG_PARKING_AREA
    //                     ? (double)dynamic_cast<MSParkingArea*>(sp)->getOccupancyIncludingRemoteReservations(veh)
    //                     : (double)sp->getStoppedVehicles().size());
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_place_occupancy(&mut self, sp: Ptr<MSStoppingPlace>, veh: Ptr<SUMOVehicle>) -> f64 {
        if sp.get_element() == SUMO_TAG_PARKING_AREA {
            Ptr::from_raw(sp.as_raw() as *mut MSParkingArea).get_occupancy_including_remote_reservations(veh) as f64
        } else {
            sp.get_stopped_vehicles().len() as f64
        }
    }

    /// C++: `double MSTriggeredRerouter::getLastStepStoppingPlaceOccupancy(MSStoppingPlace *, const SUMOVehicle *)` [virtual]
    /// Return the number of occupied places of the stopping place from the previous time step
    // double
    // MSTriggeredRerouter::getLastStepStoppingPlaceOccupancy(MSStoppingPlace* sp, const SUMOVehicle* veh) {
    //     return (sp->getElement() == SUMO_TAG_PARKING_AREA
    //                     ? (double)dynamic_cast<MSParkingArea*>(sp)->getLastStepOccupancyIncludingRemoteReservations(veh)
    //                     : (double)sp->getStoppedVehicles().size());
    // }
    #[allow(unused_variables)]
    pub fn get_last_step_stopping_place_occupancy(&mut self, sp: Ptr<MSStoppingPlace>, veh: Ptr<SUMOVehicle>) -> f64 {
        if sp.get_element() == SUMO_TAG_PARKING_AREA {
            Ptr::from_raw(sp.as_raw() as *mut MSParkingArea).get_last_step_occupancy_including_remote_reservations(veh) as f64
        } else {
            sp.get_stopped_vehicles().len() as f64
        }
    }

    /// C++: `double MSTriggeredRerouter::getStoppingPlaceCapacity(MSStoppingPlace *)` [virtual]
    /// Return the number of places the stopping place provides
    // double
    // MSTriggeredRerouter::getStoppingPlaceCapacity(MSStoppingPlace* sp) {
    //     if (myBlockedStoppingPlaces.count(sp) == 0) {
    //         return (double)(sp->getElement() == SUMO_TAG_PARKING_AREA
    //                         ? dynamic_cast<MSParkingArea*>(sp)->getCapacity()
    //                         // assume only one vehicle at a time (for stationReroute)
    //                         : 1.);
    //     } else {
    //         return 0.;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_place_capacity(&mut self, sp: Ptr<MSStoppingPlace>) -> f64 {
        if !self.myBlockedStoppingPlaces.contains(&sp) {
            if sp.get_element() == SUMO_TAG_PARKING_AREA {
                Ptr::from_raw(sp.as_raw() as *mut MSParkingArea).get_capacity() as f64
            } else {
                // assume only one vehicle at a time (for stationReroute)
                1.0
            }
        } else {
            0.0
        }
    }

    /// C++: `void MSTriggeredRerouter::rememberBlockedStoppingPlace(SUMOVehicle &, const MSStoppingPlace *, bool)` [virtual]
    /// store the blocked ParkingArea in the vehicle
    // void
    // MSTriggeredRerouter::rememberBlockedStoppingPlace(SUMOVehicle& veh, const MSStoppingPlace* parkingArea, bool blocked) {
    //     veh.rememberBlockedParkingArea(parkingArea, blocked);
    // }
    #[allow(unused_variables)]
    pub fn remember_blocked_stopping_place(&mut self, veh: &mut SUMOVehicle, parkingArea: Ptr<MSStoppingPlace>, blocked: bool) {
        veh.remember_blocked_parking_area(parkingArea, blocked);
    }

    /// C++: `void MSTriggeredRerouter::rememberStoppingPlaceScore(SUMOVehicle &, MSStoppingPlace *, const std::string &)` [virtual]
    /// store the score of the ParkingArea in the vehicle
    // void
    // MSTriggeredRerouter::rememberStoppingPlaceScore(SUMOVehicle& veh, MSStoppingPlace* parkingArea, const std::string& score) {
    //     veh.rememberParkingAreaScore(parkingArea, score);
    // }
    #[allow(unused_variables)]
    pub fn remember_stopping_place_score(&mut self, veh: &mut SUMOVehicle, parkingArea: Ptr<MSStoppingPlace>, score: &str) {
        veh.remember_parking_area_score(parkingArea, score);
    }

    /// C++: `void MSTriggeredRerouter::resetStoppingPlaceScores(SUMOVehicle &)` [virtual]
    /// reset all stored ParkingArea scores for this vehicle
    // void
    // MSTriggeredRerouter::resetStoppingPlaceScores(SUMOVehicle& veh) {
    //     veh.resetParkingAreaScores();
    // }
    #[allow(unused_variables)]
    pub fn reset_stopping_place_scores(&mut self, veh: &mut SUMOVehicle) {
        veh.reset_parking_area_scores();
    }

    /// C++: `SUMOTime MSTriggeredRerouter::sawBlockedStoppingPlace(SUMOVehicle &, MSStoppingPlace *, bool)` [virtual]
    /// get the time the ParkingArea was considered full from this vehicle
    // SUMOTime
    // MSTriggeredRerouter::sawBlockedStoppingPlace(SUMOVehicle& veh, MSStoppingPlace* parkingArea, bool local) {
    //     return veh.sawBlockedParkingArea(parkingArea, local);
    // }
    #[allow(unused_variables)]
    pub fn saw_blocked_stopping_place(&mut self, veh: &mut SUMOVehicle, parkingArea: Ptr<MSStoppingPlace>, local: bool) -> SUMOTime {
        veh.saw_blocked_parking_area(parkingArea, local)
    }

    /// C++: `int MSTriggeredRerouter::getNumberStoppingPlaceReroutes(SUMOVehicle &)` [virtual]
    /// ask how many times already the vehicle has been rerouted to another stopping place
    // int
    // MSTriggeredRerouter::getNumberStoppingPlaceReroutes(SUMOVehicle& veh) {
    //     return veh.getNumberParkingReroutes();
    // }
    #[allow(unused_variables)]
    pub fn get_number_stopping_place_reroutes(&mut self, veh: &mut SUMOVehicle) -> i32 {
        veh.get_number_parking_reroutes()
    }

    /// C++: `void MSTriggeredRerouter::setNumberStoppingPlaceReroutes(SUMOVehicle &, int)` [virtual]
    /// update the number of reroutes for the vehicle
    // void
    // MSTriggeredRerouter::setNumberStoppingPlaceReroutes(SUMOVehicle& veh, int value) {
    //     veh.setNumberParkingReroutes(value);
    // }
    #[allow(unused_variables)]
    pub fn set_number_stopping_place_reroutes(&mut self, veh: &mut SUMOVehicle, value: i32) {
        veh.set_number_parking_reroutes(value);
    }

    /// C++: `MSParkingArea * MSTriggeredRerouter::rerouteParkingArea(const MSTriggeredRerouter::RerouteInterval *, SUMOVehicle &, bool &, ConstMSEdgeVector &)`
    /// search for an alternative ParkingArea
    // MSParkingArea*
    // MSTriggeredRerouter::rerouteParkingArea(const MSTriggeredRerouter::RerouteInterval* rerouteDef,
    //                                         SUMOVehicle& veh, bool& newDestination, ConstMSEdgeVector& newRoute) {
    //     MSStoppingPlace* destStoppingPlace = veh.getNextParkingArea();
    //     if (destStoppingPlace == nullptr) {
    //         // not driving towards the right type of stop
    //         return nullptr;
    //     }
    //     std::vector<StoppingPlaceVisible> parks;
    //     for (auto cand : rerouteDef->parkProbs.getVals()) {
    //         if (cand.first->accepts(&veh)) {
    //             parks.push_back(cand);
    //         }
    //     }
    //     StoppingPlaceParamMap_t addInput = {};
    //     return dynamic_cast<MSParkingArea*>(rerouteStoppingPlace(destStoppingPlace, parks, rerouteDef->parkProbs.getProbs(), veh, newDestination, newRoute, addInput, rerouteDef->getClosed()));
    // }
    #[allow(unused_variables)]
    pub fn reroute_parking_area(&mut self, rerouteDef: Ptr<MSTriggeredRerouter_RerouteInterval>, veh: &mut SUMOVehicle, newDestination: &mut bool, newRoute: &mut ConstMSEdgeVector) -> Ptr<MSParkingArea> {
        let destStoppingPlace = Ptr::from_raw(veh.get_next_parking_area().as_raw() as *mut MSStoppingPlace);
        if destStoppingPlace.is_null() {
            // not driving towards the right type of stop
            return Ptr::default();
        }
        let mut parks: Vec<MSStoppingPlaceRerouter_StoppingPlaceVisible> = Vec::new();
        for cand in rerouteDef.parkProbs.get_vals() {
            if cand.0.accepts(Ptr::from_mut(veh)) {
                parks.push(*cand);
            }
        }
        let mut addInput: MSStoppingPlaceRerouter_StoppingPlaceParamMap_t = BTreeMap::new();
        // shortcut: closed-edge prohibitions (rerouteDef->getClosed()) are not forwarded, since SUMOAbstractRouter_Prohibitions is still Unmapped (`()`); upgrade when that alias is ported
        let probs = rerouteDef.parkProbs.get_probs().to_vec();
        let dest = self.base4.reroute_stopping_place(destStoppingPlace, &parks, &probs, veh, newDestination, newRoute, &mut addInput, &Default::default(), 0, true);
        if !dest.is_null() && dest.get_element() == SUMO_TAG_PARKING_AREA {
            Ptr::from_raw(dest.as_raw() as *mut MSParkingArea)
        } else {
            // dynamic_cast<MSParkingArea*> yields nullptr for other stopping places
            Ptr::default()
        }
    }

    /// C++: `std::pair<const SUMOVehicle *, MSRailSignal *> MSTriggeredRerouter::overtakingTrain(const SUMOVehicle &, ConstMSEdgeVector::const_iterator, const OvertakeLocation &, double &)`
    /// determine whether veh should switch from main to siding to be overtaken and return the overtaking vehicle or nullptr
    // std::pair<const SUMOVehicle*, MSRailSignal*>
    // MSTriggeredRerouter::overtakingTrain(const SUMOVehicle& veh,
    //                                      ConstMSEdgeVector::const_iterator mainStart,
    //                                      const OvertakeLocation& oloc,
    //                                      double& netSaving) {
    //     const ConstMSEdgeVector& route = veh.getRoute().getEdges();
    //     const MSEdgeVector& main = oloc.main;
    //     const double vMax = veh.getMaxSpeed();
    //     const double prio = veh.getFloatParam(toString(SUMO_TAG_OVERTAKING_REROUTE) + ".prio", false, DEFAULT_PRIO_OVERTAKEN, false);
    //     MSVehicleControl& c = MSNet::getInstance()->getVehicleControl();
    //     for (MSVehicleControl::constVehIt it_veh = c.loadedVehBegin(); it_veh != c.loadedVehEnd(); ++it_veh) {
    //         const MSBaseVehicle* veh2 = dynamic_cast<const MSBaseVehicle*>((*it_veh).second);
    //         if (veh2->isOnRoad() && veh2->getMaxSpeed() > vMax) {
    //             const double arrivalDelay = veh2->getStopArrivalDelay();
    //             const double delay = MAX2(veh2->getStopDelay(), arrivalDelay == INVALID_DOUBLE ? 0 : arrivalDelay);
    //             if (delay > veh2->getFloatParam(toString(SUMO_TAG_OVERTAKING_REROUTE) + ".maxDelay", false, DEFAULT_MAXDELAY, false)) {
    //                 continue;
    //             }
    //             const ConstMSEdgeVector& route2 = veh2->getRoute().getEdges();
    //             auto itOnMain2 = route2.end();
    //             int mainIndex = 0;
    //             for (const MSEdge* m : main) {
    //                 itOnMain2 = std::find(veh2->getCurrentRouteEdge(), route2.end(), m);
    //                 if (itOnMain2 != route2.end()) {
    //                     break;
    //                 }
    //                 mainIndex++;
    //             }
    //             if (itOnMain2 != route2.end() && itOnMain2 > veh2->getCurrentRouteEdge()) {
    //                 auto itOnMain = mainStart + mainIndex;
    //                 double timeToMain = 0;
    //                 for (auto it = veh.getCurrentRouteEdge(); it != itOnMain; it++) {
    //                     timeToMain += (*it)->getMinimumTravelTime(&veh);
    //                 }
    //                 // veh2 may be anywhere on the current edge so we have to discount
    //                 double timeToMain2 = -veh2->getEdge()->getMinimumTravelTime(veh2) * veh2->getPositionOnLane() / veh2->getEdge()->getLength();
    //                 double timeToLastSignal2 = timeToMain2;
    //                 for (auto it = veh2->getCurrentRouteEdge(); it != itOnMain2; it++) {
    //                     timeToMain2 += (*it)->getMinimumTravelTime(veh2);
    //                     auto signal = getRailSignal(*it);
    //                     if (signal) {
    //                         timeToLastSignal2 = timeToMain2;
    // #ifdef DEBUG_OVERTAKING
    //                         std::cout << "   lastBeforeMain2 " << signal->getID() << "\n";
    // #endif
    //                     }
    //                 }
    //                 double exitMainTime = timeToMain;
    //                 double exitMainBlockTime2 = timeToMain2;
    //                 double commonTime = 0;
    //                 double commonTime2 = 0;
    //                 int nCommon = 0;
    //                 auto exitMain2 = itOnMain2;
    //                 const MSRailSignal* firstAfterMain = nullptr;
    //                 const MSEdge* common = nullptr;
    //                 double vMinCommon = (*itOnMain)->getVehicleMaxSpeed(&veh);
    //                 double vMinCommon2 = (*itOnMain2)->getVehicleMaxSpeed(veh2);
    //                 while (itOnMain2 != route2.end()
    //                         && itOnMain != route.end()
    //                         && *itOnMain == *itOnMain2) {
    //                     common = *itOnMain;
    //                     commonTime += common->getMinimumTravelTime(&veh);
    //                     commonTime2 += common->getMinimumTravelTime(veh2);
    //                     vMinCommon = MIN2(vMinCommon, common->getVehicleMaxSpeed(&veh));
    //                     vMinCommon2 = MIN2(vMinCommon2, common->getVehicleMaxSpeed(veh2));
    //                     const bool onMain = nCommon < (int)main.size() - mainIndex;
    //                     if (onMain) {
    //                         exitMainTime = timeToMain + commonTime;
    //                     }
    //                     if (firstAfterMain == nullptr) {
    //                         exitMainBlockTime2 = timeToMain2 + commonTime2;
    //                     }
    //                     auto signal = getRailSignal(common);
    //                     if (signal) {
    //                         if (!onMain && firstAfterMain == nullptr) {
    //                             firstAfterMain = signal;
    // #ifdef DEBUG_OVERTAKING
    //                             std::cout << "   firstAfterMain " << signal->getID() << "\n";
    // #endif
    //                         }
    //                     }
    //                     nCommon++;
    //                     itOnMain++;
    //                     itOnMain2++;
    //                 }
    //                 const double vMaxLast = common->getVehicleMaxSpeed(&veh);
    //                 const double vMaxLast2 = common->getVehicleMaxSpeed(veh2);
    //                 commonTime += veh.getLength() / vMaxLast;
    //                 exitMainBlockTime2 += veh2->getLength() / vMaxLast2;
    //                 exitMain2 += MIN2(nCommon, (int)main.size() - mainIndex);
    //                 double timeLoss2 = MAX2(0.0, timeToMain + veh.getLength() / oloc.siding.front()->getVehicleMaxSpeed(&veh) - timeToLastSignal2);
    //                 const double saving = timeToMain + commonTime - (timeToMain2 + commonTime2) - timeLoss2;
    //                 const double loss = exitMainBlockTime2 - exitMainTime;
    //                 const double prio2 = veh2->getFloatParam(toString(SUMO_TAG_OVERTAKING_REROUTE) + ".prio", false, DEFAULT_PRIO_OVERTAKER, false);
    //                 // losses from acceleration after stopping at a signal
    //                 const double accelTimeLoss = loss > 0 ? 0.5 * vMinCommon / veh.getVehicleType().getCarFollowModel().getMaxAccel() : 0;
    //                 const double accelTimeLoss2 = timeLoss2 > 0 ? 0.5 * vMinCommon2 / veh2->getVehicleType().getCarFollowModel().getMaxAccel() : 0;
    //                 netSaving = prio2 * (saving - accelTimeLoss2) - prio * (loss + accelTimeLoss);
    // #ifdef DEBUG_OVERTAKING
    //                 std::cout << SIMTIME << " veh=" << veh.getID() << " veh2=" << veh2->getID()
    //                           << " sidingStart=" << oloc.siding.front()->getID()
    //                           << " ttm=" << timeToMain << " ttm2=" << timeToMain2
    //                           << " nCommon=" << nCommon << " cT=" << commonTime << " cT2=" << commonTime2
    //                           << " em=" << exitMainTime << " emb2=" << exitMainBlockTime2
    //                           << " ttls2=" << timeToLastSignal2
    //                           << " saving=" << saving << " loss=" << loss
    //                           << " atl=" << accelTimeLoss << " atl2=" << accelTimeLoss2 << " tl2=" << timeLoss2
    //                           << " prio=" << prio << " prio2=" << prio2 << " netSaving=" << netSaving << "\n";
    // #endif
    //                 if (netSaving > oloc.minSaving) {
    //                     MSRailSignal* s = findSignal(veh2->getCurrentRouteEdge(), exitMain2);
    //                     if (s != nullptr) {
    //                         return std::make_pair(veh2, s);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return std::make_pair(nullptr, nullptr);
    // }
    #[allow(unused_variables)]
    pub fn overtaking_train(&mut self, veh: &SUMOVehicle, mainStart: usize, oloc: &MSTriggeredRerouter_OvertakeLocation, netSaving: &mut f64) -> (Ptr<SUMOVehicle>, Ptr<MSRailSignal>) {
        // shortcut: the C++ loop over MSVehicleControl::loadedVehBegin()..loadedVehEnd() cannot be written yet (MSVehicleControl_constVehIt is Unmapped and no loaded-vehicle iterator exists), so no overtaking train is found; the candidate evaluation (timeToMain, common-edge savings, findSignal) is not ported, upgrade with the loaded-vehicle iterator
        let _ = (veh, mainStart, oloc, netSaving);
        (Ptr::default(), Ptr::default())
    }

    /// C++: `void MSTriggeredRerouter::checkStopSwitch(MSBaseVehicle &, const MSTriggeredRerouter::RerouteInterval *)`
    /// consider switching the location of the upcoming stop
    // void
    // MSTriggeredRerouter::checkStopSwitch(MSBaseVehicle& ego, const MSTriggeredRerouter::RerouteInterval* def) {
    //     myBlockedStoppingPlaces.clear();
    // #ifdef DEBUG_REROUTER
    //     std::cout << SIMTIME << " " << getID() << " ego=" << ego.getID() << "\n";
    // #endif
    //     if (!ego.hasStops()) {
    //         return;
    //     }
    //     const MSStop& stop = ego.getNextStop();
    //     if (stop.reached || stop.joinTriggered || (stop.pars.arrival < 0 && stop.pars.until < 0)) {
    //         return;
    //     }
    //     MSStoppingPlace* cur = nullptr;
    //     for (MSStoppingPlace* sp : stop.getPlaces()) {
    //         for (auto item : def->stopAlternatives) {
    //             if (sp == item.first) {
    //                 cur = sp;
    //                 break;
    //             }
    //         }
    //     }
    //     if (cur == nullptr) {
    //         return;
    //     }
    //     std::vector<const SUMOVehicle*> stopped = cur->getStoppedVehicles();
    // #ifdef DEBUG_REROUTER
    //     std::cout << SIMTIME << " " << getID() << " ego=" << ego.getID() << " stopped=" << toString(stopped) << "\n";
    // #endif
    //     SUMOTime stoppedDuration = -1;
    //     if (stopped.empty()) {
    //         /// look upstream for vehicles that stop on this lane before ego arrives
    //         const MSLane& stopLane = cur->getLane();
    //         MSVehicleControl& c = MSNet::getInstance()->getVehicleControl();
    //         for (MSVehicleControl::constVehIt it_veh = c.loadedVehBegin(); it_veh != c.loadedVehEnd(); ++it_veh) {
    //             const MSBaseVehicle* veh = dynamic_cast<const MSBaseVehicle*>((*it_veh).second);
    //             if (veh->isOnRoad() && veh->hasStops()) {
    //                 const MSStop& vehStop = veh->getNextStop();
    //                 if (vehStop.pars.lane == stopLane.getID()) {
    //                     myBlockedStoppingPlaces.insert(cur);
    //                     if (veh->isStopped()) {
    //                         // stopped somewhere else on the same lane
    //                         stoppedDuration = MAX3((SUMOTime)0, stoppedDuration, veh->getStopDuration());
    //                     } else {
    //                         std::pair<double, double> timeDist = veh->estimateTimeToNextStop();
    //                         SUMOTime timeTo = TIME2STEPS(timeDist.first);
    //                         stoppedDuration = MAX3((SUMOTime)0, stoppedDuration, timeTo + vehStop.getMinDuration(SIMSTEP + timeTo));
    //                     }
    //                 }
    //             }
    //         }
    //     } else {
    //         stoppedDuration = 0;
    //         for (const SUMOVehicle* veh : cur->getStoppedVehicles()) {
    //             stoppedDuration = MAX2(stoppedDuration, veh->getStopDuration());
    //         }
    //     }
    //     if (stoppedDuration < 0) {
    //         return;
    //     }
    //     /// @todo: consider time for conflict veh to leave the block
    //     const SUMOTime stopFree = SIMSTEP + stoppedDuration;
    //     const SUMOTime scheduledArrival = stop.pars.arrival >= 0 ? stop.pars.arrival : stop.pars.until - stop.pars.duration;
    // #ifdef DEBUG_REROUTER
    //     std::cout << SIMTIME << " " << getID() << " ego=" << ego.getID() << " stopFree=" << stopFree << " scheduledArrival=" << time2string(scheduledArrival) << "\n";
    // #endif
    //     if (stopFree < scheduledArrival) {
    //         // no conflict according to the schedule
    //         return;
    //     }
    //     const SUMOTime estimatedArrival = SIMSTEP + (stop.pars.arrival >= 0
    //                                       ? TIME2STEPS(ego.getStopArrivalDelay())
    //                                       : TIME2STEPS(ego.getStopDelay()) - stop.pars.duration);
    // #ifdef DEBUG_REROUTER
    //     std::cout << SIMTIME << " " << getID() << " ego=" << ego.getID() << " stopFree=" << stopFree << " estimatedArrival=" << time2string(estimatedArrival) << "\n";
    // #endif
    //     if (stopFree < estimatedArrival) {
    //         // no conflict when considering current delay
    //         return;
    //     }
    //     const std::vector<double> probs(def->stopAlternatives.size(), 1.);
    //     StoppingPlaceParamMap_t scores = {};
    //     bool newDestination;
    //     ConstMSEdgeVector newRoute;
    //     // @todo: consider future conflicts caused by rerouting
    //     // @todo: reject alternatives with large detour
    //     const MSStoppingPlace* alternative = rerouteStoppingPlace(nullptr, def->stopAlternatives, probs, ego, newDestination, newRoute, scores);
    // #ifdef DEBUG_REROUTER
    //     std::cout << SIMTIME << " " << getID() << " ego=" << ego.getID() << " alternative=" << Named::getIDSecure(alternative) << "\n";
    // #endif
    //     if (alternative != nullptr) {
    //         // @todo adapt plans of any riders
    //         //for (MSTransportable* p : ego.getPersons()) {
    //         //    p->rerouteParkingArea(ego.getNextParkingArea(), newParkingArea);
    //         //}
    //
    //         if (newDestination && ego.getParameter().arrivalPosProcedure != ArrivalPosDefinition::DEFAULT) {
    //             // update arrival parameters
    //             SUMOVehicleParameter* newParameter = new SUMOVehicleParameter();
    //             *newParameter = ego.getParameter();
    //             newParameter->arrivalPosProcedure = ArrivalPosDefinition::GIVEN;
    //             newParameter->arrivalPos = alternative->getEndLanePosition();
    //             ego.replaceParameter(newParameter);
    //         }
    //
    //         SUMOVehicleParameter::Stop newStop = stop.pars;
    //         newStop.lane = alternative->getLane().getID();
    //         newStop.startPos = alternative->getBeginLanePosition();
    //         newStop.endPos = alternative->getEndLanePosition();
    //         switch (alternative->getElement()) {
    //             case SUMO_TAG_PARKING_AREA:
    //                 newStop.parkingarea = alternative->getID();
    //                 break;
    //             case SUMO_TAG_CONTAINER_STOP:
    //                 newStop.containerstop = alternative->getID();
    //                 break;
    //             case SUMO_TAG_CHARGING_STATION:
    //                 newStop.chargingStation = alternative->getID();
    //                 break;
    //             case SUMO_TAG_OVERHEAD_WIRE_SEGMENT:
    //                 newStop.overheadWireSegment = alternative->getID();
    //                 break;
    //             case SUMO_TAG_BUS_STOP:
    //             case SUMO_TAG_TRAIN_STOP:
    //             default:
    //                 newStop.busstop = alternative->getID();
    //         }
    //         std::string errorMsg;
    //         if (!ego.replaceStop(0, newStop, getID() + ":" + toString(SUMO_TAG_STATION_REROUTE), false, errorMsg)) {
    //             WRITE_WARNING("Vehicle '" + ego.getID() + "' at rerouter '" + getID()
    //                           + "' could not perform stationReroute to '" + alternative->getID()
    //                           + "' reason=" + errorMsg + ", time=" + time2string(MSNet::getInstance()->getCurrentTimeStep()) + ".");
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_stop_switch(&mut self, veh: &mut MSBaseVehicle, def: Ptr<MSTriggeredRerouter_RerouteInterval>) {
        let ego = veh;
        let now = MSNet::get_instance().get_current_time_step();
        self.myBlockedStoppingPlaces.clear();
        if !ego.has_stops() {
            return;
        }
        let stop = ego.get_next_stop();
        if stop.reached || stop.joinTriggered || (stop.pars.arrival < 0 && stop.pars.until < 0) {
            return;
        }
        let stopPars = stop.pars.clone(); // copied before ego is mutated below
        // MSStop::getPlaces(): the non-null stopping places of the stop
        let mut places: Vec<Ptr<MSStoppingPlace>> = Vec::new();
        for p in [stop.busstop, stop.containerstop, stop.chargingStation, stop.overheadWireSegment] {
            if !p.is_null() {
                places.push(p);
            }
        }
        if !stop.parkingarea.is_null() {
            places.push(Ptr::from_raw(stop.parkingarea.as_raw() as *mut MSStoppingPlace));
        }
        let mut cur: Ptr<MSStoppingPlace> = Ptr::default();
        for sp in &places {
            for item in &def.stopAlternatives {
                if *sp == item.0 {
                    cur = *sp;
                    break;
                }
            }
        }
        if cur.is_null() {
            return;
        }
        let stopped: Vec<Ptr<SUMOVehicle>> = cur.get_stopped_vehicles();
        let mut stoppedDuration: SUMOTime = -1;
        if stopped.is_empty() {
            // shortcut: C++ looks upstream for vehicles stopping on the same lane, which needs the loaded-vehicle iterator (MSVehicleControl_constVehIt is Unmapped); no upstream vehicle is considered, upgrade with that iterator
        } else {
            stoppedDuration = 0;
            for v in &stopped {
                stoppedDuration = std::cmp::max(stoppedDuration, v.get_stop_duration());
            }
        }
        if stoppedDuration < 0 {
            return;
        }
        // @todo: consider time for conflict veh to leave the block
        let stopFree = now + stoppedDuration;
        let scheduledArrival = if stop.pars.arrival >= 0 { stop.pars.arrival } else { stop.pars.until - stop.pars.duration };
        if stopFree < scheduledArrival {
            // no conflict according to the schedule
            return;
        }
        let estimatedArrival = now + if stop.pars.arrival >= 0 {
            time2steps(ego.get_stop_arrival_delay())
        } else {
            time2steps(ego.get_stop_delay()) - stop.pars.duration
        };
        if stopFree < estimatedArrival {
            // no conflict when considering current delay
            return;
        }
        let probs: Vec<f64> = vec![1.0; def.stopAlternatives.len()];
        let mut scores: MSStoppingPlaceRerouter_StoppingPlaceParamMap_t = BTreeMap::new();
        let mut newDestination = false;
        let mut newRoute: ConstMSEdgeVector = Vec::new();
        // @todo: consider future conflicts caused by rerouting
        // @todo: reject alternatives with large detour
        let egoVeh = Ptr::from_raw(ego as *mut MSBaseVehicle as *mut SUMOVehicle);
        let alternative = self.base4.reroute_stopping_place(Ptr::default(), &def.stopAlternatives, &probs, egoVeh.m(), &mut newDestination, &mut newRoute, &mut scores, &Default::default(), 0, true);
        if !alternative.is_null() {
            if newDestination && ego.get_parameter().arrivalPosProcedure != ArrivalPosDefinition::DEFAULT {
                // update arrival parameters (C++ `new SUMOVehicleParameter`, owned by the vehicle)
                let mut newParameter = ego.get_parameter().clone();
                newParameter.arrivalPosProcedure = ArrivalPosDefinition::GIVEN;
                newParameter.arrivalPos = alternative.get_end_lane_position();
                ego.replace_parameter(Ptr::from_raw(Box::leak(Box::new(newParameter))));
            }

            let mut newStop = stopPars.clone();
            newStop.lane = alternative.get_lane().get_id().to_string();
            newStop.startPos = alternative.get_begin_lane_position();
            newStop.endPos = alternative.get_end_lane_position();
            let altId = alternative.get_id().to_string();
            match alternative.get_element() {
                SUMO_TAG_PARKING_AREA => newStop.parkingarea = altId.clone(),
                SUMO_TAG_CONTAINER_STOP => newStop.containerstop = altId.clone(),
                SUMO_TAG_CHARGING_STATION => newStop.chargingStation = altId.clone(),
                SUMO_TAG_OVERHEAD_WIRE_SEGMENT => newStop.overheadWireSegment = altId.clone(),
                // SUMO_TAG_BUS_STOP, SUMO_TAG_TRAIN_STOP and default
                _ => newStop.busstop = altId.clone(),
            }
            let info = format!("{}:{}", self.base.get_id(), Tags.get_string(SUMO_TAG_STATION_REROUTE));
            let mut errorMsg = String::new();
            if !ego.replace_stop(0, newStop, &info, false, &mut errorMsg) {
                utils::common::msg_handler::write_warning(&format!("Vehicle '{}' at rerouter '{}' could not perform stationReroute to '{}' reason={}, time={}.", ego.get_id(), self.base.get_id(), altId, errorMsg, time2string(now)));
            }
        }
    }

    /// C++: `MSRailSignal * MSTriggeredRerouter::findSignal(ConstMSEdgeVector::const_iterator, ConstMSEdgeVector::const_iterator)` [static]
    /// find the last downstream signal on the given route
    // MSRailSignal*
    // MSTriggeredRerouter::findSignal(ConstMSEdgeVector::const_iterator begin, ConstMSEdgeVector::const_iterator end) {
    //     auto it = end;
    //     do {
    //         it--;
    //         auto signal = getRailSignal(*it);
    //         if (signal != nullptr) {
    //             return signal;
    //         }
    //     } while (it != begin);
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn find_signal(route: &[Ptr<MSEdge>]) -> Ptr<MSRailSignal> {
        // the iterator range [begin, end) is passed as a slice; scanned from the back like the C++ do/while
        for e in route.iter().rev() {
            let signal = Self::get_rail_signal(*e);
            if !signal.is_null() {
                return signal;
            }
        }
        Ptr::default()
    }

    /// C++: `MSRailSignal * MSTriggeredRerouter::getRailSignal(const MSEdge *)` [static]
    /// return railsignal at that edge or nullptr
    // MSRailSignal*
    // MSTriggeredRerouter::getRailSignal(const MSEdge* edge) {
    //     if (edge->getToJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //         for (const MSLink* link : edge->getLanes().front()->getLinkCont()) {
    //             if (link->getTLLogic() != nullptr) {
    //                 return dynamic_cast<MSRailSignal*>(const_cast<MSTrafficLightLogic*>(link->getTLLogic()));
    //             }
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_rail_signal(edge: Ptr<MSEdge>) -> Ptr<MSRailSignal> {
        if edge.get_to_junction().get_type() == SumoXMLNodeType::RAIL_SIGNAL {
            for link in edge.get_lanes()[0].get_link_cont() {
                let tl = link.get_tl_logic();
                if !tl.is_null() {
                    // shortcut: C++ dynamic_cast<MSRailSignal*> has no runtime type tag on MSTrafficLightLogic yet; a TL logic on a rail-signal junction is taken to be the MSRailSignal, upgrade when MSTrafficLightLogic exposes its type
                    return Ptr::from_raw(tl.as_raw() as *mut MSRailSignal);
                }
            }
        }
        Ptr::default()
    }

    /// Apply interval-begin/end closures for every rerouter instance.
    ///
    /// C++ schedules `WrappingCommand<MSTriggeredRerouter>(this, &setPermissions)` on the
    /// begin-of-timestep event control for each interval that closes edges or lanes, and
    /// `setPermissions` re-schedules itself at the interval end. `WrappingCommand` is not ported, so
    /// `MSNet` calls this once per step instead: `setPermissions` only acts when `currentTime` equals an
    /// interval begin or end, so the observable behaviour is identical.
    ///
    /// Without this the closures never reach the lanes: the rerouter is registered as a move reminder and
    /// fires `trigger_routing`, but `nProh` stays empty because the permissions are never changed.
    pub fn apply_scheduled_permissions(currentTime: SUMOTime) {
        let instances = Self::get_instances();
        for (_, r) in instances.iter() {
            r.m().set_permissions(currentTime);
        }
    }

    /// C++: `const std::map<std::string, MSTriggeredRerouter *> & MSTriggeredRerouter::getInstances()` [static]
    /// return all rerouter instances
    // static const std::map<std::string, MSTriggeredRerouter*>& getInstances() {
    //         return myInstances;
    //     }
    #[allow(unused_variables)]
    pub fn get_instances() -> BTreeMap<String, Ptr<MSTriggeredRerouter>> {
        // returned by value: the C++ static map lives in a thread_local RefCell and cannot hand out a 'static reference
        MY_INSTANCES.with(|m| m.borrow().clone())
    }

    /// C++: `void MSTriggeredRerouter::checkParkingRerouteConsistency()` [static]
    /// issues warning for incomplete parkingReroute relationships
    // void
    // MSTriggeredRerouter::checkParkingRerouteConsistency() {
    //     // if a parkingArea is a rerouting target, it should generally have a
    //     // rerouter on its edge or vehicles will be stuck there once it's full.
    //     // The user should receive a Warning in this case
    //     std::set<MSEdge*> parkingRerouterEdges;
    //     std::map<MSParkingArea*, std::string, ComparatorIdLess> targetedParkingArea; // paID -> targetingRerouter
    //     for (const auto& rr : myInstances) {
    //         bool hasParkingReroute = false;
    //         for (const RerouteInterval& interval : rr.second->myIntervals) {
    //             if (interval.parkProbs.getOverallProb() > 0) {
    //                 hasParkingReroute = true;
    //                 for (const StoppingPlaceVisible& pav : interval.parkProbs.getVals()) {
    //                     targetedParkingArea[dynamic_cast<MSParkingArea*>(pav.first)] = rr.first;
    //                 }
    //             }
    //         }
    //         if (hasParkingReroute) {
    //             parkingRerouterEdges.insert(rr.second->myEdges.begin(), rr.second->myEdges.end());
    //         }
    //     }
    //     for (const auto& item : targetedParkingArea) {
    //         if (parkingRerouterEdges.count(&item.first->getLane().getEdge()) == 0) {
    //             WRITE_WARNINGF(TL("ParkingArea '%' is targeted by rerouter '%' but doesn't have its own rerouter. This may cause parking search to abort."),
    //                            item.first->getID(), item.second);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_parking_reroute_consistency() {
        // if a parkingArea is a rerouting target, it should generally have a
        // rerouter on its edge or vehicles will be stuck there once it's full.
        // The user should receive a Warning in this case
        let mut parkingRerouterEdges: BTreeSet<Ptr<MSEdge>> = BTreeSet::new();
        // std::map<MSParkingArea*, std::string, ComparatorIdLess>: kept as a Vec, sorted by ID below
        let mut targetedParkingArea: Vec<(Ptr<MSParkingArea>, String)> = Vec::new();
        for (rrId, rr) in MY_INSTANCES.with(|m| m.borrow().clone()) {
            let mut hasParkingReroute = false;
            for interval in &rr.myIntervals {
                if interval.parkProbs.get_overall_prob() > 0.0 {
                    hasParkingReroute = true;
                    for pav in interval.parkProbs.get_vals() {
                        let pa = Ptr::from_raw(pav.0.as_raw() as *mut MSParkingArea);
                        match targetedParkingArea.iter_mut().find(|(k, _)| *k == pa) {
                            Some(entry) => entry.1 = rrId.clone(),
                            None => targetedParkingArea.push((pa, rrId.clone())),
                        }
                    }
                }
            }
            if hasParkingReroute {
                parkingRerouterEdges.extend(rr.myEdges.iter().copied());
            }
        }
        // determinism: ComparatorIdLess orders the C++ map by ID, not by pointer
        targetedParkingArea.sort_by(|a, b| a.0.get_id().cmp(b.0.get_id()));
        for (pa, rerouterId) in &targetedParkingArea {
            let edge = Ptr::from_mut(pa.get_lane().get_edge());
            if !parkingRerouterEdges.contains(&edge) {
                utils::common::msg_handler::write_warning(&format!("ParkingArea '{}' is targeted by rerouter '{}' but doesn't have its own rerouter. This may cause parking search to abort.", pa.get_id(), rerouterId));
            }
        }
    }

    /// C++: `double MSTriggeredRerouter::getEvalDefaultWeight(std::string &)` [static]
    /// provide default values for evaluation components
    // static double getEvalDefaultWeight(std::string& paramName) {
    //         if (paramName == "") {
    //             return 1.;
    //         }
    //         return 0.;
    //     }
    #[allow(unused_variables)]
    pub fn get_eval_default_weight(paramName: &mut String) -> f64 {
        if paramName.is_empty() {
            1.0
        } else {
            0.0
        }
    }

    /// C++: `void MSTriggeredRerouter::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // MSTriggeredRerouter::myStartElement(int element,
    //                                     const SUMOSAXAttributes& attrs) {
    //     if (element == SUMO_TAG_INTERVAL) {
    //         bool ok = true;
    //         myParsedRerouteInterval = RerouteInterval();
    //         myParsedRerouteInterval.begin = attrs.getOptSUMOTimeReporting(SUMO_ATTR_BEGIN, nullptr, ok, -1);
    //         myParsedRerouteInterval.end = attrs.getOptSUMOTimeReporting(SUMO_ATTR_END, nullptr, ok, SUMOTime_MAX);
    //         if (myParsedRerouteInterval.begin >= myParsedRerouteInterval.end) {
    //             throw ProcessError(TLF("rerouter '%': interval end % is not after begin %.", getID(),
    //                                    time2string(myParsedRerouteInterval.end),
    //                                    time2string(myParsedRerouteInterval.begin)));
    //         }
    //     }
    //     if (element == SUMO_TAG_DEST_PROB_REROUTE) {
    //         // by giving probabilities of new destinations
    //         // get the destination edge
    //         std::string dest = attrs.getStringSecure(SUMO_ATTR_ID, "");
    //         if (dest == "") {
    //             throw ProcessError(TLF("rerouter '%': destProbReroute has no destination edge id.", getID()));
    //         }
    //         MSEdge* to = MSEdge::dictionary(dest);
    //         if (to == nullptr) {
    //             if (dest == "keepDestination") {
    //                 to = &mySpecialDest_keepDestination;
    //             } else if (dest == "terminateRoute") {
    //                 to = &mySpecialDest_terminateRoute;
    //             } else {
    //                 throw ProcessError(TLF("rerouter '%': Destination edge '%' is not known.", getID(), dest));
    //             }
    //         }
    //         // get the probability to reroute
    //         bool ok = true;
    //         double prob = attrs.getOpt<double>(SUMO_ATTR_PROB, getID().c_str(), ok, 1.);
    //         if (!ok) {
    //             throw ProcessError();
    //         }
    //         if (prob < 0) {
    //             throw ProcessError(TLF("rerouter '%': Attribute 'probability' for destination '%' is negative (must not).", getID(), dest));
    //         }
    //         // add
    //         myParsedRerouteInterval.edgeProbs.add(to, prob);
    //     }
    //
    //     if (element == SUMO_TAG_CLOSING_REROUTE) {
    //         // by closing edge
    //         const std::string& closed_id = attrs.getStringSecure(SUMO_ATTR_ID, "");
    //         MSEdge* const closedEdge = MSEdge::dictionary(closed_id);
    //         if (closedEdge == nullptr) {
    //             throw ProcessError(TLF("rerouter '%': Edge '%' to close is not known.", getID(), closed_id));
    //         }
    //         bool ok;
    //         SVCPermissions permissions = SVC_UNSPECIFIED;
    //         if (attrs.hasAttribute(SUMO_ATTR_ALLOW) || attrs.hasAttribute(SUMO_ATTR_DISALLOW)) {
    //             const std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, getID().c_str(), ok, "", false);
    //             const std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, getID().c_str(), ok, "");
    //             permissions = parseVehicleClasses(allow, disallow);
    //         }
    //         const SUMOTime until = attrs.getOptSUMOTimeReporting(SUMO_ATTR_UNTIL, nullptr, ok, TIME2STEPS(-1));
    //         myParsedRerouteInterval.closed[closedEdge] = std::make_pair(permissions, STEPS2TIME(until));
    //         myHaveClosingUntil |= until > 0;
    //     }
    //
    //     if (element == SUMO_TAG_CLOSING_LANE_REROUTE) {
    //         // by closing lane
    //         std::string closed_id = attrs.getStringSecure(SUMO_ATTR_ID, "");
    //         MSLane* closedLane = MSLane::dictionary(closed_id);
    //         if (closedLane == nullptr) {
    //             throw ProcessError(TLF("rerouter '%': Lane '%' to close is not known.", getID(), closed_id));
    //         }
    //         SVCPermissions permissions = SVC_AUTHORITY;
    //         if (attrs.hasAttribute(SUMO_ATTR_ALLOW) || attrs.hasAttribute(SUMO_ATTR_DISALLOW)) {
    //             bool ok;
    //             const std::string allow = attrs.getOpt<std::string>(SUMO_ATTR_ALLOW, getID().c_str(), ok, "", false);
    //             const std::string disallow = attrs.getOpt<std::string>(SUMO_ATTR_DISALLOW, getID().c_str(), ok, "");
    //             permissions = parseVehicleClasses(allow, disallow);
    //         }
    //         myParsedRerouteInterval.closedLanes[closedLane] = permissions;
    //     }
    //
    //     if (element == SUMO_TAG_ROUTE_PROB_REROUTE) {
    //         // by explicit rerouting using routes
    //         // check if route exists
    //         std::string routeStr = attrs.getStringSecure(SUMO_ATTR_ID, "");
    //         if (routeStr == "") {
    //             throw ProcessError(TLF("rerouter '%': routeProbReroute has no alternative route id.", getID()));
    //         }
    //         ConstMSRoutePtr route = MSRoute::dictionary(routeStr);
    //         if (route == nullptr) {
    //             throw ProcessError(TLF("rerouter '%': Alternative route '%' does not exist.", getID(), routeStr));
    //         }
    //
    //         // get the probability to reroute
    //         bool ok = true;
    //         double prob = attrs.getOpt<double>(SUMO_ATTR_PROB, getID().c_str(), ok, 1.);
    //         if (!ok) {
    //             throw ProcessError();
    //         }
    //         if (prob < 0) {
    //             throw ProcessError(TLF("rerouter '%': Attribute 'probability' for alternative route '%' is negative (must not).", getID(), routeStr));
    //         }
    //         // add
    //         myParsedRerouteInterval.routeProbs.add(route, prob);
    //     }
    //
    //     if (element == SUMO_TAG_PARKING_AREA_REROUTE) {
    //         std::string parkingarea = attrs.getStringSecure(SUMO_ATTR_ID, "");
    //         if (parkingarea == "") {
    //             throw ProcessError(TLF("rerouter '%': parkingAreaReroute requires a parkingArea id.", getID()));
    //         }
    //         MSParkingArea* pa = static_cast<MSParkingArea*>(MSNet::getInstance()->getStoppingPlace(parkingarea, SUMO_TAG_PARKING_AREA));
    //         if (pa == nullptr) {
    //             throw ProcessError(TLF("rerouter '%': parkingArea '%' is not known.", getID(), parkingarea));
    //         }
    //         // get the probability to reroute
    //         bool ok = true;
    //         const double prob = attrs.getOpt<double>(SUMO_ATTR_PROB, getID().c_str(), ok, 1.);
    //         if (!ok) {
    //             throw ProcessError();
    //         }
    //         if (prob < 0) {
    //             throw ProcessError(TLF("rerouter '%': Attribute 'probability' for parkingArea '%' is negative (must not).", getID(), parkingarea));
    //         }
    //         const bool visible = attrs.getOpt<bool>(SUMO_ATTR_VISIBLE, getID().c_str(), ok, false);
    //         // add
    //         myParsedRerouteInterval.parkProbs.add(std::make_pair(pa, visible), prob);
    //         myHaveParkProbs = true;
    //     }
    //
    //     if (element == SUMO_TAG_VIA_PROB_REROUTE) {
    //         // by giving probabilities of vias
    //         std::string viaID  = attrs.getStringSecure(SUMO_ATTR_ID, "");
    //         if (viaID == "") {
    //             throw ProcessError(TLF("rerouter '%': No via edge id given.", getID()));
    //         }
    //         MSEdge* const via = MSEdge::dictionary(viaID);
    //         if (via == nullptr) {
    //             throw ProcessError(TLF("rerouter '%': Via Edge '%' is not known.", getID(), viaID));
    //         }
    //         // get the probability to reroute
    //         bool ok = true;
    //         double prob = attrs.getOpt<double>(SUMO_ATTR_PROB, getID().c_str(), ok, 1.);
    //         if (!ok) {
    //             throw ProcessError();
    //         }
    //         if (prob < 0) {
    //             throw ProcessError(TLF("rerouter '%': Attribute 'probability' for via '%' is negative (must not).", getID(), viaID));
    //         }
    //         // add
    //         myParsedRerouteInterval.edgeProbs.add(via, prob);
    //         myParsedRerouteInterval.isVia = true;
    //     }
    //     if (element == SUMO_TAG_OVERTAKING_REROUTE) {
    //         // for letting a slow train use a siding to be overtaken by a fast train
    //         OvertakeLocation oloc;
    //         bool ok = true;
    //         for (const std::string& edgeID : attrs.get<std::vector<std::string> >(SUMO_ATTR_MAIN, getID().c_str(), ok)) {
    //             MSEdge* edge = MSEdge::dictionary(edgeID);
    //             if (edge == nullptr) {
    //                 throw InvalidArgument(TLF("The main edge '%' to use within rerouter '%' is not known.", edgeID, getID()));
    //             }
    //             oloc.main.push_back(edge);
    //             oloc.cMain.push_back(edge);
    //         }
    //         for (const std::string& edgeID : attrs.get<std::vector<std::string> >(SUMO_ATTR_SIDING, getID().c_str(), ok)) {
    //             MSEdge* edge = MSEdge::dictionary(edgeID);
    //             if (edge == nullptr) {
    //                 throw InvalidArgument(TLF("The siding edge '%' to use within rerouter '%' is not known.", edgeID, getID()));
    //             }
    //             oloc.siding.push_back(edge);
    //             oloc.cSiding.push_back(edge);
    //         }
    //         oloc.sidingExit = findSignal(oloc.cSiding.begin(), oloc.cSiding.end());
    //         if (oloc.sidingExit == nullptr) {
    //             throw InvalidArgument(TLF("The siding within rerouter '%' does not have a rail signal.", getID()));
    //         }
    //         for (auto it = oloc.cSiding.begin(); it != oloc.cSiding.end(); it++) {
    //             oloc.sidingLength += (*it)->getLength();
    //             if ((*it)->getToJunction()->getID() == oloc.sidingExit->getID()) {
    //                 break;
    //             }
    //         }
    //         oloc.minSaving = attrs.getOpt<double>(SUMO_ATTR_MINSAVING, getID().c_str(), ok, 300);
    //         const bool hasAlternatives = myParsedRerouteInterval.overtakeLocations.size() > 0;
    //         oloc.defer = attrs.getOpt<bool>(SUMO_ATTR_DEFER, getID().c_str(), ok, hasAlternatives);
    //         myParsedRerouteInterval.overtakeLocations.push_back(oloc);
    //     }
    //     if (element == SUMO_TAG_STATION_REROUTE) {
    //         // for letting a train switch it's stopping place in case of conflict
    //         const std::string stopID = attrs.getStringSecure(SUMO_ATTR_ID, "");
    //         if (stopID == "") {
    //             throw ProcessError(TLF("rerouter '%': stationReroute requires a stopping place id.", getID()));
    //         }
    //         MSStoppingPlace* stop = MSNet::getInstance()->getStoppingPlace(stopID);
    //         if (stop == nullptr) {
    //             throw ProcessError(TLF("rerouter '%': stopping place '%' is not known.", getID(), stopID));
    //         }
    //         myParsedRerouteInterval.stopAlternatives.push_back(std::make_pair(stop, true));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        // C++ `throw ProcessError(...)` becomes a panic carrying the same text (ProcessError Display)
        let rid = self.base.get_id().to_string();
        // shortcut: MSEdge::dictionary(id) getter is unported (MSEdge::dictionary_2 takes an Unmapped string_view, dictionary_hint is todo); lookup_edge returns the result of dictionary_hint, upgrade with MSEdge::dictionary
        let lookup_edge = |id: &str| -> Ptr<MSEdge> { MSEdge::dictionary_hint(id, -1) };
        if element == SUMO_TAG_INTERVAL {
            let mut ok = true;
            self.myParsedRerouteInterval = MSTriggeredRerouter_RerouteInterval {
                id: 0,
                begin: 0,
                end: 0,
                closed: BTreeMap::new(),
                closedLanes: BTreeMap::new(),
                closedLanesAffected: Vec::new(),
                edgeProbs: Default::default(),
                routeProbs: Default::default(),
                parkProbs: Default::default(),
                isVia: false,
                permissionsAllowAll: false,
                overtakeLocations: Vec::new(),
                stopAlternatives: Vec::new(),
            }; // C++ RerouteInterval()
            self.myParsedRerouteInterval.begin = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_BEGIN, "", &mut ok, -1, true);
            self.myParsedRerouteInterval.end = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_END, "", &mut ok, SUMOTIME_MAX, true);
            if self.myParsedRerouteInterval.begin >= self.myParsedRerouteInterval.end {
                panic!("{}", ProcessError::new(format!("rerouter '{}': interval end {} is not after begin {}.", rid, time2string(self.myParsedRerouteInterval.end), time2string(self.myParsedRerouteInterval.begin))));
            }
        }
        if element == SUMO_TAG_DEST_PROB_REROUTE {
            // by giving probabilities of new destinations
            // get the destination edge
            let dest = attrs.get_string_secure(SUMO_ATTR_ID, "");
            if dest == "" {
                panic!("{}", ProcessError::new(format!("rerouter '{}': destProbReroute has no destination edge id.", rid)));
            }
            let mut to = lookup_edge(&dest);
            if to.is_null() {
                if dest == "keepDestination" {
                    to = special_dest_keep_destination();
                } else if dest == "terminateRoute" {
                    to = special_dest_terminate_route();
                } else {
                    panic!("{}", ProcessError::new(format!("rerouter '{}': Destination edge '{}' is not known.", rid, dest)));
                }
            }
            // get the probability to reroute
            let mut ok = true;
            let prob = attrs.get_opt::<f64>(SUMO_ATTR_PROB, &rid, &mut ok, 1.0, true);
            if !ok {
                panic!("{}", ProcessError::default_msg());
            }
            if prob < 0.0 {
                panic!("{}", ProcessError::new(format!("rerouter '{}': Attribute 'probability' for destination '{}' is negative (must not).", rid, dest)));
            }
            // add
            self.myParsedRerouteInterval.edgeProbs.add(to, prob, false);
        }

        if element == SUMO_TAG_CLOSING_REROUTE {
            // by closing edge
            let closed_id = attrs.get_string_secure(SUMO_ATTR_ID, "");
            let closedEdge = lookup_edge(&closed_id);
            if closedEdge.is_null() {
                panic!("{}", ProcessError::new(format!("rerouter '{}': Edge '{}' to close is not known.", rid, closed_id)));
            }
            let mut ok = true;
            let mut permissions: SVCPermissions = SVC_UNSPECIFIED;
            if attrs.has_attribute(SUMO_ATTR_ALLOW) || attrs.has_attribute(SUMO_ATTR_DISALLOW) {
                let allow = attrs.get_opt::<String>(SUMO_ATTR_ALLOW, &rid, &mut ok, String::new(), false);
                let disallow = attrs.get_opt::<String>(SUMO_ATTR_DISALLOW, &rid, &mut ok, String::new(), true);
                permissions = parse_vehicle_classes_allow_disallow(&allow, &disallow, utils::common::std_defs::NETWORK_VERSION);
            }
            let until = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_UNTIL, "", &mut ok, time2steps(-1.0), true);
            self.myParsedRerouteInterval.closed.insert(closedEdge, (permissions, steps2time(until)));
            self.myHaveClosingUntil |= until > 0;
        }

        if element == SUMO_TAG_CLOSING_LANE_REROUTE {
            // by closing lane
            let closed_id = attrs.get_string_secure(SUMO_ATTR_ID, "");
            let closedLane = MSLane::dictionary_2(&closed_id);
            if closedLane.is_null() {
                panic!("{}", ProcessError::new(format!("rerouter '{}': Lane '{}' to close is not known.", rid, closed_id)));
            }
            let mut permissions: SVCPermissions = SVC_AUTHORITY;
            if attrs.has_attribute(SUMO_ATTR_ALLOW) || attrs.has_attribute(SUMO_ATTR_DISALLOW) {
                let mut ok = true;
                let allow = attrs.get_opt::<String>(SUMO_ATTR_ALLOW, &rid, &mut ok, String::new(), false);
                let disallow = attrs.get_opt::<String>(SUMO_ATTR_DISALLOW, &rid, &mut ok, String::new(), true);
                permissions = parse_vehicle_classes_allow_disallow(&allow, &disallow, utils::common::std_defs::NETWORK_VERSION);
            }
            self.myParsedRerouteInterval.closedLanes.insert(closedLane, permissions);
        }

        if element == SUMO_TAG_ROUTE_PROB_REROUTE {
            // by explicit rerouting using routes
            // check if route exists
            let routeStr = attrs.get_string_secure(SUMO_ATTR_ID, "");
            if routeStr == "" {
                panic!("{}", ProcessError::new(format!("rerouter '{}': routeProbReroute has no alternative route id.", rid)));
            }
            // C++ MSRoute::dictionary(id) returns nullptr for an unknown route; the null route stands for it (empty id)
            let route: ConstMSRoutePtr = MSRoute::dictionary_3(&routeStr, Ptr::null());
            if route.get_id().is_empty() {
                panic!("{}", ProcessError::new(format!("rerouter '{}': Alternative route '{}' does not exist.", rid, routeStr)));
            }
            // get the probability to reroute
            let mut ok = true;
            let prob = attrs.get_opt::<f64>(SUMO_ATTR_PROB, &rid, &mut ok, 1.0, true);
            if !ok {
                panic!("{}", ProcessError::default_msg());
            }
            if prob < 0.0 {
                panic!("{}", ProcessError::new(format!("rerouter '{}': Attribute 'probability' for alternative route '{}' is negative (must not).", rid, routeStr)));
            }
            self.myParsedRerouteInterval.routeProbs.add(route, prob, true);
        }

        if element == SUMO_TAG_PARKING_AREA_REROUTE {
            let parkingarea = attrs.get_string_secure(SUMO_ATTR_ID, "");
            if parkingarea == "" {
                panic!("{}", ProcessError::new(format!("rerouter '{}': parkingAreaReroute requires a parkingArea id.", rid)));
            }
            let pa = Ptr::from_raw(MSNet::get_instance().get_stopping_place(&parkingarea, SUMO_TAG_PARKING_AREA).as_raw() as *mut MSParkingArea);
            if pa.is_null() {
                panic!("{}", ProcessError::new(format!("rerouter '{}': parkingArea '{}' is not known.", rid, parkingarea)));
            }
            // get the probability to reroute
            let mut ok = true;
            let prob = attrs.get_opt::<f64>(SUMO_ATTR_PROB, &rid, &mut ok, 1.0, true);
            if !ok {
                panic!("{}", ProcessError::default_msg());
            }
            if prob < 0.0 {
                panic!("{}", ProcessError::new(format!("rerouter '{}': Attribute 'probability' for parkingArea '{}' is negative (must not).", rid, parkingarea)));
            }
            let visible = attrs.get_opt::<bool>(SUMO_ATTR_VISIBLE, &rid, &mut ok, false, true);
            // add
            self.myParsedRerouteInterval.parkProbs.add((Ptr::from_raw(pa.as_raw() as *mut MSStoppingPlace), visible), prob, false);
            self.myHaveParkProbs = true;
        }

        if element == SUMO_TAG_VIA_PROB_REROUTE {
            // by giving probabilities of vias
            let viaID = attrs.get_string_secure(SUMO_ATTR_ID, "");
            if viaID == "" {
                panic!("{}", ProcessError::new(format!("rerouter '{}': No via edge id given.", rid)));
            }
            let via = lookup_edge(&viaID);
            if via.is_null() {
                panic!("{}", ProcessError::new(format!("rerouter '{}': Via Edge '{}' is not known.", rid, viaID)));
            }
            // get the probability to reroute
            let mut ok = true;
            let prob = attrs.get_opt::<f64>(SUMO_ATTR_PROB, &rid, &mut ok, 1.0, true);
            if !ok {
                panic!("{}", ProcessError::default_msg());
            }
            if prob < 0.0 {
                panic!("{}", ProcessError::new(format!("rerouter '{}': Attribute 'probability' for via '{}' is negative (must not).", rid, viaID)));
            }
            // add
            self.myParsedRerouteInterval.edgeProbs.add(via, prob, false);
            self.myParsedRerouteInterval.isVia = true;
        }

        if element == SUMO_TAG_OVERTAKING_REROUTE {
            // for letting a slow train use a siding to be overtaken by a fast train
            let mut oloc = MSTriggeredRerouter_OvertakeLocation {
                main: Vec::new(),
                cMain: Vec::new(),
                siding: Vec::new(),
                cSiding: Vec::new(),
                sidingExit: Ptr::default(),
                sidingLength: 0.0,
                minSaving: 0.0,
                defer: false,
            };
            let mut ok = true;
            for edgeID in attrs.get::<Vec<String>>(SUMO_ATTR_MAIN, &rid, &mut ok, true) {
                let edge = lookup_edge(&edgeID);
                if edge.is_null() {
                    panic!("{}", ProcessError::new(format!("The main edge '{}' to use within rerouter '{}' is not known.", edgeID, rid)));
                }
                oloc.main.push(edge);
                oloc.cMain.push(edge);
            }
            for edgeID in attrs.get::<Vec<String>>(SUMO_ATTR_SIDING, &rid, &mut ok, true) {
                let edge = lookup_edge(&edgeID);
                if edge.is_null() {
                    panic!("{}", ProcessError::new(format!("The siding edge '{}' to use within rerouter '{}' is not known.", edgeID, rid)));
                }
                oloc.siding.push(edge);
                oloc.cSiding.push(edge);
            }
            oloc.sidingExit = Self::find_signal(&oloc.cSiding);
            if oloc.sidingExit.is_null() {
                panic!("{}", ProcessError::new(format!("The siding within rerouter '{}' does not have a rail signal.", rid)));
            }
            for it in &oloc.cSiding {
                oloc.sidingLength += it.get_length();
                if it.get_to_junction().get_id() == oloc.sidingExit.get_id() {
                    break;
                }
            }
            oloc.minSaving = attrs.get_opt::<f64>(SUMO_ATTR_MINSAVING, &rid, &mut ok, 300.0, true);
            let hasAlternatives = self.myParsedRerouteInterval.overtakeLocations.len() > 0;
            oloc.defer = attrs.get_opt::<bool>(SUMO_ATTR_DEFER, &rid, &mut ok, hasAlternatives, true);
            self.myParsedRerouteInterval.overtakeLocations.push(oloc);
        }
        if element == SUMO_TAG_STATION_REROUTE {
            // for letting a train switch it's stopping place in case of conflict
            let stopID = attrs.get_string_secure(SUMO_ATTR_ID, "");
            if stopID == "" {
                panic!("{}", ProcessError::new(format!("rerouter '{}': stationReroute requires a stopping place id.", rid)));
            }
            let stop = MSNet::get_instance().get_stopping_place_2(&stopID);
            if stop.is_null() {
                panic!("{}", ProcessError::new(format!("rerouter '{}': stopping place '{}' is not known.", rid, stopID)));
            }
            self.myParsedRerouteInterval.stopAlternatives.push((stop, true));
        }
    }

    /// C++: `void MSTriggeredRerouter::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // MSTriggeredRerouter::myEndElement(int element) {
    //     if (element == SUMO_TAG_INTERVAL) {
    //         // precompute permissionsAllowAll
    //         bool allowAll = true;
    //         for (const auto& entry : myParsedRerouteInterval.closed) {
    //             allowAll = allowAll && entry.second.first == SVC_UNSPECIFIED;
    //             if (!allowAll) {
    //                 break;
    //             }
    //         }
    //         myParsedRerouteInterval.permissionsAllowAll = allowAll;
    //
    //         for (auto paVi : myParsedRerouteInterval.parkProbs.getVals()) {
    //             dynamic_cast<MSParkingArea*>(paVi.first)->setNumAlternatives((int)myParsedRerouteInterval.parkProbs.getVals().size() - 1);
    //         }
    //         if (myParsedRerouteInterval.closedLanes.size() > 0) {
    //             // collect edges that are affect by a closed lane
    //             std::set<MSEdge*> affected;
    //             for (std::pair<MSLane*, SVCPermissions> settings : myParsedRerouteInterval.closedLanes) {
    //                 affected.insert(&settings.first->getEdge());
    //             }
    //             myParsedRerouteInterval.closedLanesAffected.insert(myParsedRerouteInterval.closedLanesAffected.begin(), affected.begin(), affected.end());
    //         }
    //         const SUMOTime closingBegin = myParsedRerouteInterval.begin;
    //         const SUMOTime simBegin = string2time(OptionsCont::getOptions().getString("begin"));
    //         if (closingBegin < simBegin && myParsedRerouteInterval.end > simBegin) {
    //             // interval started before simulation begin but is still active at
    //             // the start of the simulation
    //             myParsedRerouteInterval.begin = simBegin;
    //         }
    //         myIntervals.push_back(myParsedRerouteInterval);
    //         myIntervals.back().id = (long long int)&myIntervals.back();
    //         if (!(myParsedRerouteInterval.closed.empty() && myParsedRerouteInterval.closedLanes.empty())) {
    //             MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(
    //                 new WrappingCommand<MSTriggeredRerouter>(this, &MSTriggeredRerouter::setPermissions), myParsedRerouteInterval.begin);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        if element == SUMO_TAG_INTERVAL {
            // precompute permissionsAllowAll
            let mut allowAll = true;
            for (_, settings) in &self.myParsedRerouteInterval.closed {
                allowAll = allowAll && settings.0 == SVC_UNSPECIFIED;
                if !allowAll {
                    break;
                }
            }
            self.myParsedRerouteInterval.permissionsAllowAll = allowAll;

            let numAlternatives = self.myParsedRerouteInterval.parkProbs.get_vals().len() as i32 - 1;
            for paVi in self.myParsedRerouteInterval.parkProbs.get_vals() {
                Ptr::from_raw(paVi.0.as_raw() as *mut MSParkingArea).m().set_num_alternatives(numAlternatives);
            }
            if self.myParsedRerouteInterval.closedLanes.len() > 0 {
                // collect edges that are affect by a closed lane
                // determinism: std::set<MSEdge*> orders by address, as BTreeSet<Ptr<MSEdge>> does
                let mut affected: BTreeSet<Ptr<MSEdge>> = BTreeSet::new();
                for (lane, _) in &self.myParsedRerouteInterval.closedLanes {
                    affected.insert(Ptr::from_mut(lane.get_edge()));
                }
                // insert(begin, ...): the affected edges go in front of the existing ones
                let mut merged: MSEdgeVector = affected.into_iter().collect();
                merged.append(&mut self.myParsedRerouteInterval.closedLanesAffected);
                self.myParsedRerouteInterval.closedLanesAffected = merged;
            }
            let closingBegin = self.myParsedRerouteInterval.begin;
            let simBegin = string2time(&utils::options::options_cont::get_string("begin")).unwrap();
            if closingBegin < simBegin && self.myParsedRerouteInterval.end > simBegin {
                // interval started before simulation begin but is still active at
                // the start of the simulation
                self.myParsedRerouteInterval.begin = simBegin;
            }
            self.myIntervals.push(self.myParsedRerouteInterval.clone());
            let last = self.myIntervals.len() - 1;
            // C++: id = (long long int)&myIntervals.back(), the address of the interval
            self.myIntervals[last].id = &self.myIntervals[last] as *const MSTriggeredRerouter_RerouteInterval as i64;
            if !(self.myParsedRerouteInterval.closed.is_empty() && self.myParsedRerouteInterval.closedLanes.is_empty()) {
                let this: Ptr<MSTriggeredRerouter> = Ptr::from_mut(self);
                let begin = self.myParsedRerouteInterval.begin;
                let cmd: *mut dyn Command = Box::into_raw(Box::new(MSTriggeredRerouter_SetPermissionsCommand { rerouter: this }));
                MSNet::get_instance().m().get_begin_of_timestep_events().m().add_event(Ptr::from_raw(cmd), begin);
            }
        }
    }

    /// C++: `bool MSTriggeredRerouter::applies(const SUMOTrafficObject &) const` [const, protected]
    /// Checks whether the detector measures objects of the given type.
    // bool
    // MSTriggeredRerouter::applies(const SUMOTrafficObject& obj) const {
    //     if (myVehicleTypes.empty() || myVehicleTypes.count(obj.getVehicleType().getOriginalID()) > 0) {
    //         return true;
    //     } else {
    //         std::set<std::string> vTypeDists = MSNet::getInstance()->getVehicleControl().getVTypeDistributionMembership(obj.getVehicleType().getOriginalID());
    //         for (auto vTypeDist : vTypeDists) {
    //             if (myVehicleTypes.count(vTypeDist) > 0) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn applies(&self, obj: &SUMOTrafficObject) -> bool {
        if self.myVehicleTypes.is_empty() || self.myVehicleTypes.contains(obj.get_vehicle_type().get_original_id()) {
            return true;
        }
        let vTypeDists = MSNet::get_instance().get_vehicle_control().get_v_type_distribution_membership(obj.get_vehicle_type().get_original_id());
        for vTypeDist in &vTypeDists {
            if self.myVehicleTypes.contains(vTypeDist) {
                return true;
            }
        }
        false
    }

    /// C++: `void MSTriggeredRerouter::resetClosedEdges(bool, const SUMOTrafficObject &)` [protected]
    /// reset router after closing edges
    // void
    // MSTriggeredRerouter::resetClosedEdges(bool hasReroutingDevice, const SUMOTrafficObject& o) {
    //     // getRouterTT without prohibitions removes previous prohibitions
    //     if (o.isVehicle()) {
    //         hasReroutingDevice
    //         ? MSRoutingEngine::getRouterTT(o.getRNGIndex(), o.getVClass())
    //         : MSNet::getInstance()->getRouterTT(o.getRNGIndex());
    //     } else {
    //         hasReroutingDevice
    //         ? MSRoutingEngine::getIntermodalRouterTT(o.getRNGIndex())
    //         : MSNet::getInstance()->getIntermodalRouter(o.getRNGIndex(), 0);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_closed_edges(&mut self, hasReroutingDevice: bool, o: &SUMOTrafficObject) {
        // getRouterTT without prohibitions removes previous prohibitions (empty Prohibitions = `&()`)
        if o.is_vehicle() {
            if hasReroutingDevice {
                MSRoutingEngine::get_router_tt(o.get_rng_index(), o.get_v_class(), &Default::default());
            } else {
                MSNet::get_instance().get_router_tt(o.get_rng_index(), &Default::default());
            }
        } else if hasReroutingDevice {
            MSRoutingEngine::get_intermodal_router_tt(o.get_rng_index(), &Default::default());
        } else {
            MSNet::get_instance().get_intermodal_router(o.get_rng_index(), 0, &Default::default());
        }
    }

    /// C++: `bool MSTriggeredRerouter::affected(const std::set<SUMOTrafficObject::NumericalID> &, const MSEdgeVector &)` [static, protected]
    // bool
    // MSTriggeredRerouter::affected(const std::set<SUMOTrafficObject::NumericalID>& edgeIndices, const MSEdgeVector& closed) {
    //     for (const MSEdge* const e : closed) {
    //         if (edgeIndices.count(e->getNumericalID()) > 0) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn affected(edgeIndices: &BTreeSet<SUMOTrafficObject_NumericalID>, closed: &MSEdgeVector) -> bool {
        for e in closed {
            if edgeIndices.contains(&(e.get_numerical_id() as SUMOTrafficObject_NumericalID)) {
                return true;
            }
        }
        false
    }

    /// C++: `void MSTriggeredRerouter::MSTriggeredRerouter(const MSTriggeredRerouter &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTriggeredRerouter) -> Self {
        panic!("MSTriggeredRerouter copy constructor is declared private and never defined in C++")
    }

    /// C++: `MSTriggeredRerouter & MSTriggeredRerouter::operator=(const MSTriggeredRerouter &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTriggeredRerouter) -> &mut MSTriggeredRerouter {
        panic!("MSTriggeredRerouter assignment operator is declared private and never defined in C++")
    }

}

/// Groups data for an overtakingReroute
/// C++ class `MSTriggeredRerouter::OvertakeLocation` (microsim/trigger/MSTriggeredRerouter.h)
#[derive(Clone)]
pub struct MSTriggeredRerouter_OvertakeLocation {
    /// The list of main edges (const and non-const for different usage)
    pub main: MSEdgeVector,
    pub cMain: ConstMSEdgeVector,
    /// The list of siding edges
    pub siding: MSEdgeVector,
    pub cSiding: ConstMSEdgeVector,
    /// The rail signal at the end of the siding
    pub sidingExit: Ptr<MSRailSignal>,
    /// The usable length of the siding
    pub sidingLength: f64,
    /// The threshold in savings for triggering reroute
    pub minSaving: f64,
    /// whether the decision to use this siding should be deferred
    pub defer: bool,
}

impl MSTriggeredRerouter_OvertakeLocation {
}

/// Describes the rerouting definitions valid for an interval
/// C++ class `MSTriggeredRerouter::RerouteInterval` (microsim/trigger/MSTriggeredRerouter.h)
#[derive(Clone)]
pub struct MSTriggeredRerouter_RerouteInterval {
    /// unique ID for this interval
    pub id: i64,
    /// The begin time these definitions are valid
    pub begin: SUMOTime,
    /// The end time these definitions are valid
    pub end: SUMOTime,
    /// The map of closed edges to their permissions and expected end of closing
    pub closed: BTreeMap<Ptr<MSEdge>, (SVCPermissions, f64)>, // C++: std::map<MSEdge *, std::pair<SVCPermissions, double>>
    /// The list of closed lanes to their permissions
    pub closedLanes: BTreeMap<Ptr<MSLane>, SVCPermissions>,
    /// The list of edges that are affected by closed lanes
    pub closedLanesAffected: MSEdgeVector,
    /// The distributions of new destinations or vias to use
    pub edgeProbs: RandomDistributor<Ptr<MSEdge>>,
    /// The distributions of new routes to use
    pub routeProbs: RandomDistributor<ConstMSRoutePtr>,
    /// The distributions of new parking areas to use as destinations
    pub parkProbs: RandomDistributor<MSStoppingPlaceRerouter_StoppingPlaceVisible>,
    /// The edge probs are vias and not destinations
    pub isVia: bool,
    /// The permissions are all SVCAll
    pub permissionsAllowAll: bool,
    /// @{
    pub overtakeLocations: Vec<MSTriggeredRerouter_OvertakeLocation>,
    /// @{
    pub stopAlternatives: Vec<MSStoppingPlaceRerouter_StoppingPlaceVisible>,
}

impl MSTriggeredRerouter_RerouteInterval {
    /// C++: `Prohibitions RerouteInterval::getClosed() const` [const]
    // Prohibitions getClosed() const {
    //             Prohibitions v;
    //             for (const auto& settings : closed) {
    //                 // no permissions are changed but edges are forbidden for all during routing
    //                 v[settings.first].permissions = settings.second.first == SVC_UNSPECIFIED ? 0 : settings.second.first;
    //                 if (settings.second.second != -1) {
    //                     // end time is known
    //                     v[settings.first].end = settings.second.second;
    //                 }
    //             }
    //             return v;
    //         }
    #[allow(unused_variables)]
    pub fn get_closed(&self) -> MSTriggeredRerouter_Prohibitions {
        let mut v: MSTriggeredRerouter_Prohibitions = BTreeMap::new();
        for (edge, settings) in &self.closed {
            // no permissions are changed but edges are forbidden for all during routing
            // C++ default-constructs RouterProhibition: begin 0, end max, permissions SVC_IGNORING
            let p = v.entry(*edge).or_insert(RouterProhibition { begin: 0.0, end: f64::MAX, permissions: SVC_IGNORING });
            p.permissions = if settings.0 == SVC_UNSPECIFIED { 0 } else { settings.0 };
            if settings.1 != -1.0 {
                // end time is known
                p.end = settings.1;
            }
        }
        v
    }

    /// C++: `MSEdgeVector RerouteInterval::getClosedEdges() const` [const]
    // MSEdgeVector getClosedEdges() const {
    //             MSEdgeVector v;
    //             for (const auto& settings : closed) {
    //                 v.push_back(settings.first);
    //             }
    //             return v;
    //         }
    #[allow(unused_variables)]
    pub fn get_closed_edges(&self) -> MSEdgeVector {
        self.closed.keys().copied().collect()
    }

}


fn rerouter_of(mr: &mut MSMoveReminder) -> &mut MSTriggeredRerouter {
    // base2 is not the first field: step back from the embedded MSMoveReminder to the enclosing rerouter
    let p = (mr as *mut MSMoveReminder as *mut u8).wrapping_sub(std::mem::offset_of!(MSTriggeredRerouter, base2)) as *mut MSTriggeredRerouter;
    unsafe { &mut *p }
}
fn ms_triggered_rerouter_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    rerouter_of(mr).notify_enter(veh, reason, enteredLane)
}
fn ms_triggered_rerouter_notify_move(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
    rerouter_of(mr).notify_move(veh, oldPos, newPos, newSpeed)
}
fn ms_triggered_rerouter_notify_leave(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
    rerouter_of(mr).notify_leave(veh, lastPos, reason, enteredLane)
}

static MS_TRIGGERED_REROUTER_MSMOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
    notify_enter: ms_triggered_rerouter_notify_enter,
    notify_move: ms_triggered_rerouter_notify_move,
    notify_leave: ms_triggered_rerouter_notify_leave,
};