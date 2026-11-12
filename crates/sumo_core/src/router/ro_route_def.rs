//! Port of router/RORouteDef.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Base class for a vehicle's route definition
/// C++ class `RORouteDef` (router/RORouteDef.h)
#[repr(C)]
pub struct RORouteDef {
    /// C++ base class `Named`
    pub base: Named,
    /// precomputed route for out-of-order computation
    pub myPrecomputed: Rc<RORoute>,
    /// Index of the route used within the last step
    pub myLastUsed: i32,
    /// The alternatives
    pub myAlternatives: Vec<Rc<RORoute>>,
    /// Information whether a new route was generated
    pub myNewRoute: bool,
    pub myTryRepair: bool,
    pub myMayBeDisconnected: bool,
    /// Whether this route should be silently discarded
    pub myDiscardSilent: bool,
}

impl std::ops::Deref for RORouteDef {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for RORouteDef {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

// C++ static members of RORouteDef (port as module-level statics, see AGENTS.md):
//   static bool RORouteDef::myUsingJTRR;
//   static bool RORouteDef::mySkipNewRoutes;

impl RORouteDef {
    /// C++: `void RORouteDef::RORouteDef(const std::string &, const int, const bool, const bool)`
    /// Constructor
    // RORouteDef::RORouteDef(const std::string& id, const int lastUsed,
    //                        const bool tryRepair, const bool mayBeDisconnected) :
    //     Named(StringUtils::convertUmlaute(id)),
    //     myPrecomputed(nullptr), myLastUsed(lastUsed), myTryRepair(tryRepair),
    //     myMayBeDisconnected(mayBeDisconnected),
    //     myDiscardSilent(false) {
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, lastUsed: i32, tryRepair: bool, mayBeDisconnected: bool) -> Self {
        todo!("port RORouteDef::RORouteDef")
    }

    /// C++: `void RORouteDef::~RORouteDef() noexcept` [virtual]
    /// Destructor
    // virtual ~RORouteDef() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RORouteDef::~RORouteDef")
    }

    /// C++: `void RORouteDef::addLoadedAlternative(std::shared_ptr<RORoute>)`
    /// Adds a single alternative loaded from the file An alternative may also be generated during DUA
    // void addLoadedAlternative(std::shared_ptr<RORoute> alternative) {
    //         myAlternatives.push_back(alternative);
    //     }
    #[allow(unused_variables)]
    pub fn add_loaded_alternative(&mut self, alternative: Rc<RORoute>) {
        todo!("port RORouteDef::addLoadedAlternative")
    }

    /// C++: `void RORouteDef::addAlternativeDef(const RORouteDef *)`
    /// Adds an alternative loaded from the file
    // void
    // RORouteDef::addAlternativeDef(const RORouteDef* alt) {
    //     std::copy(alt->myAlternatives.begin(), alt->myAlternatives.end(),
    //               back_inserter(myAlternatives));
    // }
    #[allow(unused_variables)]
    pub fn add_alternative_def(&mut self, alternative: Ptr<RORouteDef>) {
        todo!("port RORouteDef::addAlternativeDef")
    }

    /// C++: `void RORouteDef::validateAlternatives(const ROVehicle *, MsgHandler *)`
    /// removes invalid alternatives and raise an error or warning *
    // void
    // RORouteDef::validateAlternatives(const ROVehicle* veh, MsgHandler* errorHandler) {
    //     for (int i = 0; i < (int)myAlternatives.size();) {
    //         if ((i != myLastUsed || mySkipNewRoutes) && !myAlternatives[i]->isPermitted(veh, errorHandler)) {
    //             myAlternatives.erase(myAlternatives.begin() + i);
    //             if (myLastUsed > i) {
    //                 myLastUsed--;
    //             }
    //         } else {
    //             i++;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn validate_alternatives(&mut self, veh: Ptr<ROVehicle>, errorHandler: Ptr<MsgHandler>) {
        todo!("port RORouteDef::validateAlternatives")
    }

    /// C++: `std::shared_ptr<RORoute> RORouteDef::buildCurrentRoute(SUMOAbstractRouter<ROEdge, ROVehicle> &, SUMOTime, const ROVehicle &) const` [const]
    /// Triggers building of the complete route (via preComputeCurrentRoute) or returns precomputed route
    // std::shared_ptr<RORoute>
    // RORouteDef::buildCurrentRoute(SUMOAbstractRouter<ROEdge, ROVehicle>& router,
    //                               SUMOTime begin, const ROVehicle& veh) const {
    //     if (myPrecomputed == nullptr) {
    //         preComputeCurrentRoute(router, begin, veh);
    //     }
    //     return myPrecomputed;
    // }
    #[allow(unused_variables)]
    pub fn build_current_route(&self, router: &mut SUMOAbstractRouter<ROEdge, ROVehicle>, begin: SUMOTime, veh: &ROVehicle) -> Rc<RORoute> {
        todo!("port RORouteDef::buildCurrentRoute")
    }

    /// C++: `void RORouteDef::preComputeCurrentRoute(SUMOAbstractRouter<ROEdge, ROVehicle> &, SUMOTime, const ROVehicle &) const` [const]
    /// Builds the complete route (or chooses her from the list of alternatives, when existing)
    // void
    // RORouteDef::preComputeCurrentRoute(SUMOAbstractRouter<ROEdge, ROVehicle>& router,
    //                                    SUMOTime begin, const ROVehicle& veh) const {
    //     myNewRoute = false;
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     const bool ignoreErrors = oc.getBool("ignore-errors");
    //     const bool hasRestrictions = RONet::getInstance()->hasParamRestrictions();
    //     assert(myAlternatives[0]->getEdgeVector().size() > 0);
    //     MsgHandler* mh = ignoreErrors ? MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance();
    //     if (myAlternatives[0]->getFirst()->prohibits(&veh, hasRestrictions) && (!oc.getBool("repair.from")
    //             // do not try to reassign starting edge for trip input
    //             || myMayBeDisconnected || myAlternatives[0]->getEdgeVector().size() < 2)) {
    //         mh->informf(TL("Vehicle '%' is not allowed to depart on edge '%'."), veh.getID(), myAlternatives[0]->getFirst()->getID());
    //         return;
    //     } else if (myAlternatives[0]->getLast()->prohibits(&veh, hasRestrictions) && (!oc.getBool("repair.to")
    //                // do not try to reassign destination edge for trip input
    //                || myMayBeDisconnected || myAlternatives[0]->getEdgeVector().size() < 2)) {
    //         // this check is not strictly necessary unless myTryRepair is set.
    //         // However, the error message is more helpful than "no connection found"
    //         mh->informf(TL("Vehicle '%' is not allowed to arrive on edge '%'."), veh.getID(), myAlternatives[0]->getLast()->getID());
    //         return;
    //     }
    //     const bool skipTripRouting = (oc.exists("write-trips") && oc.getBool("write-trips")
    //                                   && RouteCostCalculator<RORoute, ROEdge, ROVehicle>::getCalculator().skipRouteCalculation());
    //     if ((myTryRepair && !skipTripRouting) || myUsingJTRR) {
    //         ConstROEdgeVector newEdges;
    //         if (repairCurrentRoute(router, begin, veh, myAlternatives[0]->getEdgeVector(), newEdges)) {
    //             if (myAlternatives[0]->getEdgeVector() != newEdges) {
    //                 if (!myMayBeDisconnected) {
    //                     WRITE_WARNINGF(TL("Repaired route of vehicle '%'."), veh.getID());
    //                 }
    //                 myNewRoute = true;
    //                 RGBColor* col = myAlternatives[0]->getColor() != nullptr ? new RGBColor(*myAlternatives[0]->getColor()) : nullptr;
    //                 myPrecomputed = std::make_shared<RORoute>(myID, 0, myAlternatives[0]->getProbability(), newEdges, col, myAlternatives[0]->getStops());
    //             } else {
    //                 myPrecomputed = myAlternatives[0];
    //             }
    //         }
    //         return;
    //     }
    //     if (RouteCostCalculator<RORoute, ROEdge, ROVehicle>::getCalculator().skipRouteCalculation()
    //             || OptionsCont::getOptions().getBool("remove-loops")) {
    //         if (skipTripRouting || myAlternatives[myLastUsed]->isValid(veh, ignoreErrors)) {
    //             myPrecomputed = myAlternatives[myLastUsed];
    //         }
    //     } else {
    //         // build a new route to test whether it is better
    //         ConstROEdgeVector oldEdges({getOrigin(), getDestination()});
    //         ConstROEdgeVector edges;
    //         if (repairCurrentRoute(router, begin, veh, oldEdges, edges, true)) {
    //             if (edges.front()->isTazConnector()) {
    //                 edges.erase(edges.begin());
    //             }
    //             if (edges.back()->isTazConnector()) {
    //                 edges.pop_back();
    //             }
    //             // check whether the same route was already used
    //             int existing = -1;
    //             for (int i = 0; i < (int)myAlternatives.size(); i++) {
    //                 if (edges == myAlternatives[i]->getEdgeVector()) {
    //                     existing = i;
    //                     break;
    //                 }
    //             }
    //             if (existing >= 0) {
    //                 myPrecomputed = myAlternatives[existing];
    //             } else {
    //                 RGBColor* col = myAlternatives[0]->getColor() != nullptr ? new RGBColor(*myAlternatives[0]->getColor()) : nullptr;
    //                 myPrecomputed = std::make_shared<RORoute>(myID, 0, 1, edges, col, myAlternatives[0]->getStops());
    //                 myNewRoute = true;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn pre_compute_current_route(&self, router: &mut SUMOAbstractRouter<ROEdge, ROVehicle>, begin: SUMOTime, veh: &ROVehicle) {
        todo!("port RORouteDef::preComputeCurrentRoute")
    }

    /// C++: `bool RORouteDef::repairCurrentRoute(SUMOAbstractRouter<ROEdge, ROVehicle> &, SUMOTime, const ROVehicle &, ConstROEdgeVector, ConstROEdgeVector &, bool) const` [const]
    /// Builds the complete route (or chooses her from the list of alternatives, when existing)
    // bool
    // RORouteDef::repairCurrentRoute(SUMOAbstractRouter<ROEdge, ROVehicle>& router,
    //                                SUMOTime begin, const ROVehicle& veh,
    //                                ConstROEdgeVector oldEdges, ConstROEdgeVector& newEdges,
    //                                bool isTrip) const {
    //     MsgHandler* mh = (OptionsCont::getOptions().getBool("ignore-errors") ?
    //                       MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance());
    //     RONet* net = RONet::getInstance();
    //     const int initialSize = (int)oldEdges.size();
    //     const bool hasRestrictions = RONet::getInstance()->hasParamRestrictions();
    //     if (net->hasProhibitions() && router.supportsProhibitions()) {
    //         if (net->getProhibitions().size() > 0 && !router.hasProhibitions()) {
    //             router.prohibit(net->getProhibitions());
    //         }
    //         net->updateLaneProhibitions(begin);
    //     }
    //     if (initialSize == 1) {
    //         if (myUsingJTRR) {
    //             /// only ROJTRRouter is supposed to handle this type of input
    //             bool ok = router.compute(oldEdges.front(), nullptr, &veh, begin, newEdges);
    //             myDiscardSilent = ok && newEdges.size() == 0;
    //         } else {
    //             newEdges = oldEdges;
    //         }
    //     } else {
    //         if (oldEdges.front()->prohibits(&veh, hasRestrictions)) {
    //             // option repair.from is in effect
    //             const std::string& frontID = oldEdges.front()->getID();
    //             for (ConstROEdgeVector::iterator i = oldEdges.begin(); i != oldEdges.end();) {
    //                 if ((*i)->prohibits(&veh, hasRestrictions) || (*i)->isInternal()) {
    //                     i = oldEdges.erase(i);
    //                 } else {
    //                     WRITE_MESSAGE("Changing invalid starting edge '" + frontID
    //                                   + "' to '" + (*i)->getID() + "' for vehicle '" + veh.getID() + "'.");
    //                     break;
    //                 }
    //             }
    //         }
    //         if (oldEdges.size() == 0) {
    //             mh->informf(TL("Could not find new starting edge for vehicle '%'."), veh.getID());
    //             return false;
    //         }
    //         if (oldEdges.back()->prohibits(&veh, hasRestrictions)) {
    //             // option repair.to is in effect
    //             const std::string& backID = oldEdges.back()->getID();
    //             // oldEdges cannot get empty here, otherwise we would have left the stage when checking "from"
    //             while (oldEdges.back()->prohibits(&veh, hasRestrictions) || oldEdges.back()->isInternal()) {
    //                 oldEdges.pop_back();
    //             }
    //             WRITE_MESSAGE("Changing invalid destination edge '" + backID
    //                           + "' to edge '" + oldEdges.back()->getID() + "' for vehicle '" + veh.getID() + "'.");
    //         }
    //         std::vector<ROVehicle::Mandatory> mandatory = veh.getMandatoryEdges(oldEdges.front(), oldEdges.back());
    //         assert(mandatory.size() >= 2);
    //         // removed prohibited
    //         for (ConstROEdgeVector::iterator i = oldEdges.begin(); i != oldEdges.end();) {
    //             if ((*i)->prohibits(&veh, hasRestrictions) || (*i)->isInternal()) {
    //                 // no need to check the mandatories here, this was done before
    //                 WRITE_MESSAGEF(TL("Removing invalid edge '%' from route for vehicle '%'."), (*i)->getID(), veh.getID());
    //                 i = oldEdges.erase(i);
    //             } else {
    //                 ++i;
    //             }
    //         }
    //         // reconnect remaining edges
    //         if (mandatory.size() > oldEdges.size() && initialSize > 2) {
    //             WRITE_MESSAGEF(TL("There are stop edges which were not part of the original route for vehicle '%'."), veh.getID());
    //         }
    //         ConstROEdgeVector targets;
    //         bool checkPositions = false;
    //         if (mandatory.size() >= oldEdges.size()) {
    //             for (auto m : mandatory) {
    //                 targets.push_back(m.edge);
    //                 checkPositions |= m.pos >= 0;
    //             }
    //         } else {
    //             targets = oldEdges;
    //         }
    //         newEdges.push_back(targets.front());
    //         auto nextMandatory = mandatory.begin() + 1;
    //         while (nextMandatory != mandatory.end()
    //                 && targets.front() == nextMandatory->edge
    //                 && (nextMandatory->edge != (nextMandatory - 1)->edge
    //                     || nextMandatory->pos >= (nextMandatory - 1)->pos
    //                     // ignore invalid via stop pos
    //                     || nextMandatory->pos < 0)) {
    //             nextMandatory++;
    //         }
    //         int lastMandatory = 0;
    //         for (ConstROEdgeVector::const_iterator i = targets.begin() + 1;
    //                 i != targets.end() && nextMandatory != mandatory.end(); ++i) {
    //             const ROEdge* prev = *(i - 1);
    //             const ROEdge* cur = *i;
    //             if (prev->isConnectedTo(*cur, veh.getVClass()) && (!isRailway(veh.getVClass()) || prev->getBidiEdge() != cur)) {
    //                 newEdges.push_back(cur);
    //             } else {
    //                 if (initialSize > 2) {
    //                     // only inform if the input is (probably) not a trip
    //                     WRITE_MESSAGEF(TL("Edge '%' not connected to edge '%' for vehicle '%'."), (*(i - 1))->getID(), (*i)->getID(), veh.getID());
    //                 }
    //                 const ROEdge* last = newEdges.back();
    //                 newEdges.pop_back();
    //                 if (last->isTazConnector() && newEdges.size() > 1) {
    //                     // assume this was a viaTaz
    //                     last = newEdges.back();
    //                     newEdges.pop_back();
    //                 }
    //                 if (veh.hasJumps() && (nextMandatory - 1)->isJump) {
    //                     while (*i != nextMandatory->edge) {
    //                         ++i;
    //                     }
    //                     newEdges.push_back(last);
    //                     newEdges.push_back(*i);
    //                     //std::cout << " skipJump mIndex=" << (nextMandatory - 1 - mandatory.begin()) << " last=" << last->getID() << " next=" << (*i)->getID() << " newEdges=" << toString(newEdges) << "\n";
    //                 } else {
    //                     int numEdgesBefore = (int)newEdges.size();
    //                     //                router.setHint(targets.begin(), i, &veh, begin);
    //                     if (myTryRepair && lastMandatory < (int)newEdges.size() && last != newEdges[lastMandatory]) {
    //                         router.setMsgHandler(MsgHandler::getWarningInstance());
    //                     }
    //                     bool ok;
    //                     if (checkPositions
    //                             && mandatory[i - targets.begin() - 1].pos >= 0
    //                             && mandatory[i - targets.begin()].pos >= 0) {
    //                         ok = router.compute(
    //                                  last, mandatory[i - targets.begin() - 1].pos,
    //                                  *i, mandatory[i - targets.begin()].pos,
    //                                  &veh, begin, newEdges);
    //                     } else {
    //                         ok = router.compute(last, *i, &veh, begin, newEdges);
    //                     }
    //                     router.setMsgHandler(mh);
    //                     if (!ok) {
    //                         // backtrack: try to route from last mandatory edge to next mandatory edge
    //                         // XXX add option for backtracking in smaller increments
    //                         // (i.e. previous edge to edge after *i)
    //                         // we would then need to decide whether we have found a good
    //                         // tradeoff between faithfulness to the input data and detour-length
    //                         if (lastMandatory >= (int)newEdges.size() || last == newEdges[lastMandatory] || !backTrack(router, i, lastMandatory, nextMandatory->edge, newEdges, veh, begin)) {
    //                             mh->informf(TL("Mandatory edge '%' not reachable by vehicle '%'."), (*i)->getID(), veh.getID());
    //                             return false;
    //                         }
    //                     } else if (!myMayBeDisconnected && !isTrip && last != (*i)) {
    //                         double airDist = last->getToJunction()->getPosition().distanceTo(
    //                                              (*i)->getFromJunction()->getPosition());
    //                         double repairDist = 0;
    //                         for (auto it2 = (newEdges.begin() + numEdgesBefore + 1); it2 != newEdges.end() && it2 != newEdges.end() - 1; it2++) {
    //                             repairDist += (*it2)->getLength();
    //                         }
    //                         const double detourFactor = repairDist / MAX2(airDist, 1.0);
    //                         const double detour = MAX2(0.0, repairDist - airDist);
    //                         const double maxDetourFactor = OptionsCont::getOptions().getFloat("repair.max-detour-factor");
    //                         if (detourFactor > maxDetourFactor) {
    //                             WRITE_MESSAGEF("    Backtracking to avoid detour of %m for gap of %m)", detour, airDist);
    //                             backTrack(router, i, lastMandatory, nextMandatory->edge, newEdges, veh, begin);
    //                         } else if (detourFactor > 1.1) {
    //                             WRITE_MESSAGEF("    Taking detour of %m to avoid gap of %m)", detour, airDist);
    //                         }
    //                     }
    //                 }
    //             }
    //             while (nextMandatory != mandatory.end()
    //                     && *i == nextMandatory->edge
    //                     && (nextMandatory->edge != (nextMandatory - 1)->edge
    //                         || nextMandatory->pos >= (nextMandatory - 1)->pos
    //                         // ignore invalid via stop pos
    //                         || nextMandatory->pos < 0)) {
    //                 nextMandatory++;
    //                 lastMandatory = (int)newEdges.size() - 1;
    //             }
    //         }
    //         if (veh.getParameter().via.size() > 0 && veh.getParameter().stops.size() > 0) {
    //             // check consistency of stops and vias
    //             auto it = newEdges.begin();
    //             for (const auto& stop : veh.getParameter().stops) {
    //                 const ROEdge* e = net->getEdge(stop.edge);
    //                 it = std::find(it, newEdges.end(), e);
    //                 if (it == newEdges.end()) {
    //                     mh->informf(TL("Stop edge '%' is inconsistent with via edges for vehicle '%'."), e->getID(), veh.getID());
    //                     return false;
    //                 }
    //             }
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn repair_current_route(&self, router: &mut SUMOAbstractRouter<ROEdge, ROVehicle>, begin: SUMOTime, veh: &ROVehicle, oldEdges: ConstROEdgeVector, newEdges: &mut ConstROEdgeVector, isTrip: bool) -> bool {
        todo!("port RORouteDef::repairCurrentRoute")
    }

    /// C++: `std::shared_ptr<RORoute> RORouteDef::addAlternative(SUMOAbstractRouter<ROEdge, ROVehicle> &, const ROVehicle *const, std::shared_ptr<RORoute>, SUMOTime, MsgHandler *)`
    /// Adds an alternative to the list of routes and returns the route that was replaced or nullptr
    // std::shared_ptr<RORoute>
    // RORouteDef::addAlternative(SUMOAbstractRouter<ROEdge, ROVehicle>& router,
    //                            const ROVehicle* const veh, std::shared_ptr<RORoute> current, SUMOTime begin,
    //                            MsgHandler* errorHandler) {
    //     std::shared_ptr<RORoute> replaced = nullptr;
    //     if (myTryRepair || myUsingJTRR) {
    //         if (myNewRoute) {
    //             replaced = myAlternatives[0];
    //             myAlternatives[0] = current;
    //         }
    //         if (!router.isValid(current->getEdgeVector(), veh, STEPS2TIME(begin))) {
    //             throw ProcessError("Route '" + getID() + "' (vehicle '" + veh->getID() + "') is not valid.");
    //         }
    //         double costs = router.recomputeCosts(current->getEdgeVector(), veh, begin);
    //         if (veh->hasJumps()) {
    //             // @todo: jumpTime should be applied in recomputeCost to ensure the
    //             // correctness of time-dependent traveltimes
    //             costs += STEPS2TIME(veh->getJumpTime());
    //         }
    //         current->setCosts(costs);
    //         return replaced;
    //     }
    //     // add the route when it's new
    //     if (myAlternatives.back()->getProbability() < 0 || !myAlternatives.back()->isPermitted(veh, errorHandler)) {
    //         if (myAlternatives.back()->getProbability() >= 0 && errorHandler == MsgHandler::getErrorInstance()) {
    //             throw ProcessError("Route '" + current->getID() + "' (vehicle '" + veh->getID() + "') is not valid.");
    //         }
    //         myAlternatives.pop_back();
    //     }
    //     if (myNewRoute) {
    //         myAlternatives.push_back(current);
    //     }
    //     // recompute the costs and (when a new route was added) scale the probabilities
    //     const double scale = double(myAlternatives.size() - 1) / double(myAlternatives.size());
    //     for (const std::shared_ptr<RORoute>& alt : myAlternatives) {
    //         if (!router.isValid(alt->getEdgeVector(), veh, STEPS2TIME(begin))) {
    //             throw ProcessError("Route '" + current->getID() + "' (vehicle '" + veh->getID() + "') is not valid.");
    //         }
    //         // recompute the costs for all routes
    //         const double newCosts = router.recomputeCosts(alt->getEdgeVector(), veh, begin);
    //         assert(myAlternatives.size() != 0);
    //         if (myNewRoute) {
    //             if (alt == current) {
    //                 // set initial probability and costs
    //                 alt->setProbability(1. / (double) myAlternatives.size());
    //                 alt->setCosts(newCosts);
    //             } else {
    //                 // rescale probs for all others
    //                 alt->setProbability(alt->getProbability() * scale);
    //             }
    //         }
    //         RouteCostCalculator<RORoute, ROEdge, ROVehicle>::getCalculator().setCosts(alt, newCosts, alt == myAlternatives[myLastUsed]);
    //     }
    //     assert(myAlternatives.size() != 0);
    //     RouteCostCalculator<RORoute, ROEdge, ROVehicle>::getCalculator().calculateProbabilities(myAlternatives, veh, veh->getDepartureTime());
    //     const bool keepRoute = RouteCostCalculator<RORoute, ROEdge, ROVehicle>::getCalculator().keepRoute();
    //     if (!RouteCostCalculator<RORoute, ROEdge, ROVehicle>::getCalculator().keepAllRoutes() && !keepRoute) {
    //         // remove with probability of 0 (not mentioned in Gawron)
    //         for (std::vector<std::shared_ptr<RORoute>>::iterator i = myAlternatives.begin(); i != myAlternatives.end();) {
    //             if ((*i)->getProbability() == 0) {
    //                 i = myAlternatives.erase(i);
    //             } else {
    //                 i++;
    //             }
    //         }
    //     }
    //     int maxNumber = RouteCostCalculator<RORoute, ROEdge, ROVehicle>::getCalculator().getMaxRouteNumber();
    //     if ((int)myAlternatives.size() > maxNumber) {
    //         std::shared_ptr<const RORoute> last = myAlternatives[myLastUsed];
    //         // only keep the routes with highest probability
    //         sort(myAlternatives.begin(), myAlternatives.end(), [](const std::shared_ptr<const RORoute> a, const std::shared_ptr<const RORoute> b) {
    //             return a->getProbability() > b->getProbability();
    //         });
    //         if (keepRoute) {
    //             for (int i = 0; i < (int)myAlternatives.size(); i++) {
    //                 if (myAlternatives[i] == last) {
    //                     myLastUsed = i;
    //                     break;
    //                 }
    //             }
    //             if (myLastUsed >= maxNumber) {
    //                 std::swap(myAlternatives[maxNumber - 1], myAlternatives[myLastUsed]);
    //                 myLastUsed = maxNumber - 1;
    //             }
    //         }
    //         myAlternatives.erase(myAlternatives.begin() + maxNumber, myAlternatives.end());
    //     }
    //     // rescale probabilities
    //     double newSum = 0.;
    //     for (const std::shared_ptr<RORoute>& alt : myAlternatives) {
    //         newSum += alt->getProbability();
    //     }
    //     assert(newSum > 0);
    //     // @note newSum may be larger than 1 for numerical reasons
    //     for (const std::shared_ptr<RORoute>& alt : myAlternatives) {
    //         alt->setProbability(alt->getProbability() / newSum);
    //     }
    //
    //     // find the route to use
    //     if (!keepRoute) {
    //         double chosen = RandHelper::rand();
    //         myLastUsed = 0;
    //         for (const std::shared_ptr<RORoute>& alt : myAlternatives) {
    //             chosen -= alt->getProbability();
    //             if (chosen <= 0) {
    //                 return nullptr;
    //             }
    //             myLastUsed++;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn add_alternative(&mut self, router: &mut SUMOAbstractRouter<ROEdge, ROVehicle>, arg1: Ptr<ROVehicle>, current: Rc<RORoute>, begin: SUMOTime, errorHandler: Ptr<MsgHandler>) -> Rc<RORoute> {
        todo!("port RORouteDef::addAlternative")
    }

    /// C++: `const ROEdge * RORouteDef::getOrigin() const` [const]
    // const ROEdge*
    // RORouteDef::getOrigin() const {
    //     return myAlternatives.back()->getFirst();
    // }
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<ROEdge> {
        todo!("port RORouteDef::getOrigin")
    }

    /// C++: `const ROEdge * RORouteDef::getDestination() const` [const]
    // const ROEdge*
    // RORouteDef::getDestination() const {
    //     return myAlternatives.back()->getLast();
    // }
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<ROEdge> {
        todo!("port RORouteDef::getDestination")
    }

    /// C++: `std::shared_ptr<const RORoute> RORouteDef::getFirstRoute() const` [const]
    // std::shared_ptr<const RORoute> getFirstRoute() const {
    //         if (myAlternatives.empty()) {
    //             return nullptr;
    //         }
    //         return myAlternatives.front();
    //     }
    #[allow(unused_variables)]
    pub fn get_first_route(&self) -> Rc<RORoute> {
        todo!("port RORouteDef::getFirstRoute")
    }

    /// C++: `std::shared_ptr<const RORoute> RORouteDef::getUsedRoute() const` [const]
    // std::shared_ptr<const RORoute> getUsedRoute() const {
    //         return myAlternatives[myLastUsed];
    //     }
    #[allow(unused_variables)]
    pub fn get_used_route(&self) -> Rc<RORoute> {
        todo!("port RORouteDef::getUsedRoute")
    }

    /// C++: `OutputDevice & RORouteDef::writeXMLDefinition(OutputDevice &, const ROVehicle *const, bool, bool, bool, bool) const` [const]
    /// Saves the built route / route alternatives
    // OutputDevice&
    // RORouteDef::writeXMLDefinition(OutputDevice& dev, const ROVehicle* const veh,
    //                                bool asAlternatives, bool withExitTimes, bool withCost, bool withLength) const {
    //     if (asAlternatives) {
    //         dev.openTag(SUMO_TAG_ROUTE_DISTRIBUTION).writeAttr(SUMO_ATTR_LAST, myLastUsed);
    //         for (int i = 0; i != (int)myAlternatives.size(); i++) {
    //             myAlternatives[i]->writeXMLDefinition(dev, veh, true, true, withExitTimes, withLength);
    //         }
    //         dev.closeTag();
    //         return dev;
    //     } else {
    //         return myAlternatives[myLastUsed]->writeXMLDefinition(dev, veh, withCost, false, withExitTimes, withLength);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn write_xml_definition(&self, dev: &mut OutputDevice, veh: Ptr<ROVehicle>, asAlternatives: bool, withExitTimes: bool, withCost: bool, withLength: bool) -> &mut OutputDevice {
        todo!("port RORouteDef::writeXMLDefinition")
    }

    /// C++: `RORouteDef * RORouteDef::copy(const std::string &, const SUMOTime) const` [const]
    /// Returns a deep copy of the route definition.
    // RORouteDef*
    // RORouteDef::copy(const std::string& id, const SUMOTime stopOffset) const {
    //     RORouteDef* result = new RORouteDef(id, 0, myTryRepair, myMayBeDisconnected);
    //     for (const std::shared_ptr<const RORoute> route : myAlternatives) {
    //         RGBColor* col = route->getColor() != nullptr ? new RGBColor(*route->getColor()) : nullptr;
    //         std::shared_ptr<RORoute> newRoute = std::make_shared<RORoute>(id, route->getCosts(), route->getProbability(), route->getEdgeVector(), col, route->getStops());
    //         newRoute->addStopOffset(stopOffset);
    //         result->addLoadedAlternative(newRoute);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn copy(&self, id: &str, stopOffset: SUMOTime) -> Ptr<RORouteDef> {
        todo!("port RORouteDef::copy")
    }

    /// C++: `double RORouteDef::getOverallProb() const` [const]
    /// Returns the sum of the probablities of the contained routes
    // double
    // RORouteDef::getOverallProb() const {
    //     double sum = 0.;
    //     for (const std::shared_ptr<const RORoute> r : myAlternatives) {
    //         sum += r->getProbability();
    //     }
    //     return sum;
    // }
    #[allow(unused_variables)]
    pub fn get_overall_prob(&self) -> f64 {
        todo!("port RORouteDef::getOverallProb")
    }

    /// C++: `bool RORouteDef::discardSilent() const` [const]
    /// whether this route shall be silently discarded
    // bool discardSilent() const {
    //         return myDiscardSilent;
    //     }
    #[allow(unused_variables)]
    pub fn discard_silent(&self) -> bool {
        todo!("port RORouteDef::discardSilent")
    }

    /// C++: `void RORouteDef::setUsingJTRR()` [static]
    // static void setUsingJTRR() {
    //         myUsingJTRR = true;
    //     }
    #[allow(unused_variables)]
    pub fn set_using_jtrr() {
        todo!("port RORouteDef::setUsingJTRR")
    }

    /// C++: `void RORouteDef::setSkipNew()` [static]
    // static void setSkipNew() {
    //         mySkipNewRoutes = true;
    //     }
    #[allow(unused_variables)]
    pub fn set_skip_new() {
        todo!("port RORouteDef::setSkipNew")
    }

    /// C++: `bool RORouteDef::backTrack(SUMOAbstractRouter<ROEdge, ROVehicle> &, ConstROEdgeVector::const_iterator &, int, const ROEdge *, ConstROEdgeVector &, const ROVehicle &, SUMOTime)` [static, protected]
    /// backtrack to last mandatory edge and route to next mandatory
    // bool
    // RORouteDef::backTrack(SUMOAbstractRouter<ROEdge, ROVehicle>& router,
    //                       ConstROEdgeVector::const_iterator& i, int lastMandatory, const ROEdge* nextMandatory,
    //                       ConstROEdgeVector& newEdges, const ROVehicle& veh, SUMOTime begin) {
    //     ConstROEdgeVector edges;
    //     bool ok = router.compute(newEdges[lastMandatory], nextMandatory, &veh, begin, edges, true);
    //     if (!ok) {
    //         return false;
    //     }
    //
    //     while (*i != nextMandatory) {
    //         ++i;
    //     }
    //     newEdges.erase(newEdges.begin() + lastMandatory + 1, newEdges.end());
    //     std::copy(edges.begin() + 1, edges.end(), back_inserter(newEdges));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn back_track(router: &mut SUMOAbstractRouter<ROEdge, ROVehicle>, i: &mut Unmapped /* ConstROEdgeVector::const_iterator */, lastMandatory: i32, nextMandatory: Ptr<ROEdge>, newEdges: &mut ConstROEdgeVector, veh: &ROVehicle, begin: SUMOTime) -> bool {
        todo!("port RORouteDef::backTrack")
    }

    /// C++: `void RORouteDef::RORouteDef(const RORouteDef &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &RORouteDef) -> Self {
        todo!("port RORouteDef::RORouteDef")
    }

    /// C++: `RORouteDef & RORouteDef::operator=(const RORouteDef &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RORouteDef) -> &mut RORouteDef {
        todo!("port RORouteDef::operator=")
    }

}

