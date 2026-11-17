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
    pub myPrecomputed: std::cell::RefCell<Option<Rc<RORoute>>>,
    /// Index of the route used within the last step
    pub myLastUsed: i32,
    /// The alternatives
    pub myAlternatives: Vec<Rc<RORoute>>,
    /// Information whether a new route was generated
    pub myNewRoute: std::cell::Cell<bool>,
    pub myTryRepair: bool,
    pub myMayBeDisconnected: bool,
    /// Whether this route should be silently discarded
    pub myDiscardSilent: std::cell::Cell<bool>,
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
static MY_USING_JTRR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
static MY_SKIP_NEW_ROUTES: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// C++ `MsgHandler::getWarningInstance()` / `getErrorInstance()`
// shortcut: the instances are kept in a locked static, the pointer stays valid because they are only dropped by cleanup_on_end
fn msg_instance(warning: bool) -> Ptr<MsgHandler> {
    let t = if warning { MsgType::Warning } else { MsgType::Error };
    Ptr::from_raw(utils::common::msg_handler::with_instance(t, |h| h as *mut MsgHandler))
}

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
        RORouteDef {
            base: Named::new(&utils::common::string_utils::convert_umlaute(id)),
            myPrecomputed: std::cell::RefCell::new(None),
            myLastUsed: lastUsed,
            myAlternatives: Vec::new(),
            myNewRoute: std::cell::Cell::new(false),
            myTryRepair: tryRepair,
            myMayBeDisconnected: mayBeDisconnected,
            myDiscardSilent: std::cell::Cell::new(false),
        }
    }

    /// C++: `void RORouteDef::~RORouteDef() noexcept` [virtual]
    /// Destructor
    // virtual ~RORouteDef() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void RORouteDef::addLoadedAlternative(std::shared_ptr<RORoute>)`
    /// Adds a single alternative loaded from the file An alternative may also be generated during DUA
    // void addLoadedAlternative(std::shared_ptr<RORoute> alternative) {
    //         myAlternatives.push_back(alternative);
    //     }
    #[allow(unused_variables)]
    pub fn add_loaded_alternative(&mut self, alternative: Rc<RORoute>) {
        self.myAlternatives.push(alternative);
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
        for a in alternative.myAlternatives.iter() {
            self.myAlternatives.push(a.clone());
        }
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
        let mut i: usize = 0;
        while i < self.myAlternatives.len() {
            if (i as i32 != self.myLastUsed || MY_SKIP_NEW_ROUTES.load(std::sync::atomic::Ordering::Relaxed))
                && !self.myAlternatives[i].is_permitted(veh, errorHandler)
            {
                self.myAlternatives.remove(i);
                if self.myLastUsed > i as i32 {
                    self.myLastUsed -= 1;
                }
            } else {
                i += 1;
            }
        }
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
    pub fn build_current_route(&self, router: &mut SUMOAbstractRouter<ROEdge, ROVehicle>, begin: SUMOTime, veh: &ROVehicle) -> Option<Rc<RORoute>> {
        if self.myPrecomputed.borrow().is_none() {
            self.pre_compute_current_route(router, begin, veh);
        }
        self.myPrecomputed.borrow().clone()
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
        // C++ `mutable` members are written from this const method
        let me = self;
        me.myNewRoute.set(false);
        let ignoreErrors = utils::options::options_cont::get_bool("ignore-errors");
        let hasRestrictions = RONet::get_instance().has_param_restrictions();
        assert!(me.myAlternatives[0].get_edge_vector().len() > 0);
        let mh = msg_instance(ignoreErrors);
        if me.myAlternatives[0].get_first().prohibits(Ptr::new(veh), hasRestrictions)
            && (!utils::options::options_cont::get_bool("repair.from") || me.myMayBeDisconnected || me.myAlternatives[0].get_edge_vector().len() < 2)
        {
            let args: [&dyn ToStringPrec; 2] = [&veh.get_id().to_string(), &me.myAlternatives[0].get_first().get_id().to_string()];
            mh.m().informf("Vehicle '%' is not allowed to depart on edge '%'.", &args);
            return;
        } else if me.myAlternatives[0].get_last().prohibits(Ptr::new(veh), hasRestrictions)
            && (!utils::options::options_cont::get_bool("repair.to") || me.myMayBeDisconnected || me.myAlternatives[0].get_edge_vector().len() < 2)
        {
            // this check is not strictly necessary unless myTryRepair is set.
            // However, the error message is more helpful than "no connection found"
            let args: [&dyn ToStringPrec; 2] = [&veh.get_id().to_string(), &me.myAlternatives[0].get_last().get_id().to_string()];
            mh.m().informf("Vehicle '%' is not allowed to arrive on edge '%'.", &args);
            return;
        }
        let skipTripRouting = utils::options::options_cont::exists("write-trips")
            && utils::options::options_cont::get_bool("write-trips")
            && RouteCostCalculator::<RORoute, ROEdge, ROVehicle>::get_calculator().skip_route_calculation();
        if (me.myTryRepair && !skipTripRouting) || MY_USING_JTRR.load(std::sync::atomic::Ordering::Relaxed) {
            let mut newEdges: ConstROEdgeVector = Vec::new();
            let oldEdges = me.myAlternatives[0].get_edge_vector().clone();
            if me.repair_current_route(router, begin, veh, oldEdges, &mut newEdges, false) {
                if me.myAlternatives[0].get_edge_vector() != &newEdges {
                    if !me.myMayBeDisconnected {
                        write_warningf!("Repaired route of vehicle '%'.", veh.get_id());
                    }
                    me.myNewRoute.set(true);
                    let col = me.myAlternatives[0].get_color();
                    *me.myPrecomputed.borrow_mut() = Some(Rc::new(RORoute::new(self.get_id(), 0.0, me.myAlternatives[0].get_probability(), &newEdges, col, me.myAlternatives[0].get_stops())));
                } else {
                    *me.myPrecomputed.borrow_mut() = Some(me.myAlternatives[0].clone());
                }
            }
            return;
        }
        if RouteCostCalculator::<RORoute, ROEdge, ROVehicle>::get_calculator().skip_route_calculation() || utils::options::options_cont::get_bool("remove-loops") {
            if skipTripRouting || me.myAlternatives[me.myLastUsed as usize].is_valid(veh, ignoreErrors, Ptr::null()) {
                *me.myPrecomputed.borrow_mut() = Some(me.myAlternatives[me.myLastUsed as usize].clone());
            }
        } else {
            // build a new route to test whether it is better
            let oldEdges: ConstROEdgeVector = vec![self.get_origin(), self.get_destination()];
            let mut edges: ConstROEdgeVector = Vec::new();
            if me.repair_current_route(router, begin, veh, oldEdges, &mut edges, true) {
                if !edges.is_empty() && edges[0].is_taz_connector() {
                    edges.remove(0);
                }
                if !edges.is_empty() && edges[edges.len() - 1].is_taz_connector() {
                    edges.pop();
                }
                // check whether the same route was already used
                let mut existing: i32 = -1;
                for i in 0..me.myAlternatives.len() {
                    if &edges == me.myAlternatives[i].get_edge_vector() {
                        existing = i as i32;
                        break;
                    }
                }
                if existing >= 0 {
                    *me.myPrecomputed.borrow_mut() = Some(me.myAlternatives[existing as usize].clone());
                } else {
                    let col = me.myAlternatives[0].get_color();
                    *me.myPrecomputed.borrow_mut() = Some(Rc::new(RORoute::new(self.get_id(), 0.0, 1.0, &edges, col, me.myAlternatives[0].get_stops())));
                    me.myNewRoute.set(true);
                }
            }
        }
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
        // C++ `mutable` member (myDiscardSilent) is written from this const method
        let me = self;
        let mh = msg_instance(utils::options::options_cont::get_bool("ignore-errors"));
        let net = RONet::get_instance();
        let mut oldEdges = oldEdges;
        let initialSize = oldEdges.len() as i32;
        let hasRestrictions = net.has_param_restrictions();
        if net.has_prohibitions() && router.supports_prohibitions() {
            if net.get_prohibitions().len() > 0 && !router.has_prohibitions() {
                // shortcut: SUMOAbstractRouter_Prohibitions is keyed by Ptr<Unmapped> while RONet_Prohibitions is keyed by Ptr<ROEdge>, keys are cast, upgrade when the router is generic over E
                let prohibitions: SUMOAbstractRouter_Prohibitions = net.get_prohibitions().iter().map(|(k, v)| (k.cast::<Unmapped>(), *v)).collect();
                router.prohibit(&prohibitions);
            }
            net.m().update_lane_prohibitions(begin);
        }
        if initialSize == 1 {
            if MY_USING_JTRR.load(std::sync::atomic::Ordering::Relaxed) {
                // only ROJTRRouter is supposed to handle this type of input
                let ok = router.compute(oldEdges[0], Ptr::null(), Ptr::new(veh), begin, newEdges, false);
                me.myDiscardSilent.set(ok && newEdges.is_empty());
            } else {
                *newEdges = oldEdges;
            }
        } else {
            if oldEdges[0].prohibits(Ptr::new(veh), hasRestrictions) {
                // option repair.from is in effect
                let frontID = oldEdges[0].get_id().to_string();
                let mut k: usize = 0;
                while k < oldEdges.len() {
                    if oldEdges[k].prohibits(Ptr::new(veh), hasRestrictions) || oldEdges[k].is_internal() {
                        oldEdges.remove(k);
                    } else {
                        utils::common::msg_handler::write_message(&format!("Changing invalid starting edge '{}' to '{}' for vehicle '{}'.", frontID, oldEdges[k].get_id(), veh.get_id()));
                        break;
                    }
                }
            }
            if oldEdges.is_empty() {
                let args: [&dyn ToStringPrec; 1] = [&veh.get_id().to_string()];
                mh.m().informf("Could not find new starting edge for vehicle '%'.", &args);
                return false;
            }
            if oldEdges[oldEdges.len() - 1].prohibits(Ptr::new(veh), hasRestrictions) {
                // option repair.to is in effect
                let backID = oldEdges[oldEdges.len() - 1].get_id().to_string();
                // oldEdges cannot get empty here, otherwise we would have left the stage when checking "from"
                while !oldEdges.is_empty() && (oldEdges[oldEdges.len() - 1].prohibits(Ptr::new(veh), hasRestrictions) || oldEdges[oldEdges.len() - 1].is_internal()) {
                    oldEdges.pop();
                }
                utils::common::msg_handler::write_message(&format!(
                    "Changing invalid destination edge '{}' to edge '{}' for vehicle '{}'.",
                    backID,
                    oldEdges[oldEdges.len() - 1].get_id(),
                    veh.get_id()
                ));
            }
            let mandatory = veh.get_mandatory_edges(oldEdges[0], oldEdges[oldEdges.len() - 1]);
            assert!(mandatory.len() >= 2);
            // removed prohibited
            let mut k: usize = 0;
            while k < oldEdges.len() {
                if oldEdges[k].prohibits(Ptr::new(veh), hasRestrictions) || oldEdges[k].is_internal() {
                    // no need to check the mandatories here, this was done before
                    write_messagef!("Removing invalid edge '%' from route for vehicle '%'.", oldEdges[k].get_id(), veh.get_id());
                    oldEdges.remove(k);
                } else {
                    k += 1;
                }
            }
            // reconnect remaining edges
            if mandatory.len() > oldEdges.len() && initialSize > 2 {
                write_messagef!("There are stop edges which were not part of the original route for vehicle '%'.", veh.get_id());
            }
            let mut targets: ConstROEdgeVector = Vec::new();
            let mut checkPositions = false;
            if mandatory.len() >= oldEdges.len() {
                for m in mandatory.iter() {
                    targets.push(m.edge);
                    checkPositions |= m.pos >= 0.0;
                }
            } else {
                targets = oldEdges.clone();
            }
            newEdges.push(targets[0]);
            let mut nextMandatory: usize = 1;
            while nextMandatory != mandatory.len()
                && targets[0] == mandatory[nextMandatory].edge
                && (mandatory[nextMandatory].edge != mandatory[nextMandatory - 1].edge
                    || mandatory[nextMandatory].pos >= mandatory[nextMandatory - 1].pos
                    // ignore invalid via stop pos
                    || mandatory[nextMandatory].pos < 0.0)
            {
                nextMandatory += 1;
            }
            let mut lastMandatory: i32 = 0;
            let mut i: usize = 1;
            while i < targets.len() && nextMandatory != mandatory.len() {
                let prev = targets[i - 1];
                let cur = targets[i];
                if prev.is_connected_to(&cur, veh.get_v_class(), false) && (!utils::common::sumo_vehicle_class::is_railway(veh.get_v_class()) || prev.get_bidi_edge() != cur) {
                    newEdges.push(cur);
                } else {
                    if initialSize > 2 {
                        // only inform if the input is (probably) not a trip
                        write_messagef!("Edge '%' not connected to edge '%' for vehicle '%'.", targets[i - 1].get_id(), targets[i].get_id(), veh.get_id());
                    }
                    let mut last = newEdges[newEdges.len() - 1];
                    newEdges.pop();
                    if last.is_taz_connector() && newEdges.len() > 1 {
                        // assume this was a viaTaz
                        last = newEdges[newEdges.len() - 1];
                        newEdges.pop();
                    }
                    if veh.has_jumps() && mandatory[nextMandatory - 1].isJump {
                        while targets[i] != mandatory[nextMandatory].edge {
                            i += 1;
                        }
                        newEdges.push(last);
                        newEdges.push(targets[i]);
                    } else {
                        let numEdgesBefore = newEdges.len();
                        if self.myTryRepair && lastMandatory < newEdges.len() as i32 && last != newEdges[lastMandatory as usize] {
                            router.set_msg_handler(msg_instance(true));
                        }
                        let ok: bool;
                        if checkPositions && mandatory[i - 1].pos >= 0.0 && mandatory[i].pos >= 0.0 {
                            ok = router.compute_2(last, mandatory[i - 1].pos, targets[i], mandatory[i].pos, Ptr::new(veh), begin, newEdges, false);
                        } else {
                            ok = router.compute(last, targets[i], Ptr::new(veh), begin, newEdges, false);
                        }
                        router.set_msg_handler(mh);
                        if !ok {
                            // backtrack: try to route from last mandatory edge to next mandatory edge
                            if lastMandatory >= newEdges.len() as i32
                                || last == newEdges[lastMandatory as usize]
                                || !Self::back_track(router, &targets, &mut i, lastMandatory, mandatory[nextMandatory].edge, newEdges, veh, begin)
                            {
                                let args: [&dyn ToStringPrec; 2] = [&targets[i].get_id().to_string(), &veh.get_id().to_string()];
                                mh.m().informf("Mandatory edge '%' not reachable by vehicle '%'.", &args);
                                return false;
                            }
                        } else if !self.myMayBeDisconnected && !isTrip && last != targets[i] {
                            let airDist = last.get_to_junction().get_position().distance_to(targets[i].get_from_junction().get_position());
                            let mut repairDist = 0.0_f64;
                            let len = newEdges.len();
                            let mut it2 = numEdgesBefore + 1;
                            while it2 + 1 < len {
                                repairDist += newEdges[it2].get_length();
                                it2 += 1;
                            }
                            let detourFactor = repairDist / (if airDist > 1.0 { airDist } else { 1.0 });
                            let gap = repairDist - airDist;
                            let detour = if 0.0 > gap { 0.0 } else { gap };
                            let maxDetourFactor = utils::options::options_cont::get_float("repair.max-detour-factor");
                            if detourFactor > maxDetourFactor {
                                write_messagef!("    Backtracking to avoid detour of %m for gap of %m)", detour, airDist);
                                Self::back_track(router, &targets, &mut i, lastMandatory, mandatory[nextMandatory].edge, newEdges, veh, begin);
                            } else if detourFactor > 1.1 {
                                write_messagef!("    Taking detour of %m to avoid gap of %m)", detour, airDist);
                            }
                        }
                    }
                }
                while nextMandatory != mandatory.len()
                    && targets[i] == mandatory[nextMandatory].edge
                    && (mandatory[nextMandatory].edge != mandatory[nextMandatory - 1].edge
                        || mandatory[nextMandatory].pos >= mandatory[nextMandatory - 1].pos
                        // ignore invalid via stop pos
                        || mandatory[nextMandatory].pos < 0.0)
                {
                    nextMandatory += 1;
                    lastMandatory = newEdges.len() as i32 - 1;
                }
                i += 1;
            }
        }
        if veh.get_parameter().via.len() > 0 && veh.get_parameter().stops.len() > 0 {
            // check consistency of stops and vias
            let mut it: usize = 0;
            for stop in veh.get_parameter().stops.iter() {
                let e = net.get_edge(&stop.edge);
                match newEdges[it..].iter().position(|x| *x == e) {
                    Some(p) => it += p,
                    None => {
                        let args: [&dyn ToStringPrec; 2] = [&e.get_id().to_string(), &veh.get_id().to_string()];
                        mh.m().informf("Stop edge '%' is inconsistent with via edges for vehicle '%'.", &args);
                        return false;
                    }
                }
            }
        }
        true
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
    pub fn add_alternative(&mut self, router: &mut SUMOAbstractRouter<ROEdge, ROVehicle>, veh: Ptr<ROVehicle>, current: Rc<RORoute>, begin: SUMOTime, errorHandler: Ptr<MsgHandler>) -> Option<Rc<RORoute>> {
        let mut replaced: Option<Rc<RORoute>> = None;
        if self.myTryRepair || MY_USING_JTRR.load(std::sync::atomic::Ordering::Relaxed) {
            if self.myNewRoute.get() {
                replaced = Some(self.myAlternatives[0].clone());
                self.myAlternatives[0] = current.clone();
            }
            // shortcut: C++ throws ProcessError here, it panics instead, upgrade when add_alternative returns Result
            if !router.is_valid(current.get_edge_vector(), veh, utils::common::time::steps2time(begin)) {
                panic!("Route '{}' (vehicle '{}') is not valid.", self.get_id(), veh.get_id());
            }
            let mut costs = router.recompute_costs(current.get_edge_vector(), veh, begin, Ptr::null());
            if veh.has_jumps() {
                // @todo: jumpTime should be applied in recomputeCost to ensure the
                // correctness of time-dependent traveltimes
                costs += utils::common::time::steps2time(veh.get_jump_time());
            }
            current.set_costs(costs);
            return replaced;
        }
        // add the route when it's new
        let last = self.myAlternatives.len() - 1;
        if self.myAlternatives[last].get_probability() < 0.0 || !self.myAlternatives[last].is_permitted(veh, errorHandler) {
            // shortcut: C++ throws ProcessError here, it panics instead, upgrade when add_alternative returns Result
            if self.myAlternatives[last].get_probability() >= 0.0 && errorHandler == msg_instance(false) {
                panic!("Route '{}' (vehicle '{}') is not valid.", current.get_id(), veh.get_id());
            }
            self.myAlternatives.pop();
        }
        if self.myNewRoute.get() {
            self.myAlternatives.push(current.clone());
        }
        // recompute the costs and (when a new route was added) scale the probabilities
        let scale = (self.myAlternatives.len().wrapping_sub(1)) as f64 / (self.myAlternatives.len() as f64);
        for idx in 0..self.myAlternatives.len() {
            let alt = self.myAlternatives[idx].clone();
            // shortcut: C++ throws ProcessError here, it panics instead, upgrade when add_alternative returns Result
            if !router.is_valid(alt.get_edge_vector(), veh, utils::common::time::steps2time(begin)) {
                panic!("Route '{}' (vehicle '{}') is not valid.", current.get_id(), veh.get_id());
            }
            // recompute the costs for all routes
            let newCosts = router.recompute_costs(alt.get_edge_vector(), veh, begin, Ptr::null());
            assert!(self.myAlternatives.len() != 0);
            if self.myNewRoute.get() {
                if Rc::ptr_eq(&alt, &current) {
                    // set initial probability and costs
                    alt.set_probability(1.0 / (self.myAlternatives.len() as f64));
                    alt.set_costs(newCosts);
                } else {
                    // rescale probs for all others
                    let p = alt.get_probability() * scale;
                    alt.set_probability(p);
                }
            }
            let isActive = Rc::ptr_eq(&alt, &self.myAlternatives[self.myLastUsed as usize]);
            RouteCostCalculator::<RORoute, ROEdge, ROVehicle>::get_calculator().set_costs(alt.clone(), newCosts, isActive);
        }
        assert!(self.myAlternatives.len() != 0);
        RouteCostCalculator::<RORoute, ROEdge, ROVehicle>::get_calculator().calculate_probabilities(&self.myAlternatives, veh, veh.get_departure_time());
        let keepRoute = RouteCostCalculator::<RORoute, ROEdge, ROVehicle>::get_calculator().keep_route();
        if !RouteCostCalculator::<RORoute, ROEdge, ROVehicle>::get_calculator().keep_all_routes() && !keepRoute {
            // remove with probability of 0 (not mentioned in Gawron)
            let mut i: usize = 0;
            while i < self.myAlternatives.len() {
                if self.myAlternatives[i].get_probability() == 0.0 {
                    self.myAlternatives.remove(i);
                } else {
                    i += 1;
                }
            }
        }
        let maxNumber = RouteCostCalculator::<RORoute, ROEdge, ROVehicle>::get_calculator().get_max_route_number();
        if self.myAlternatives.len() as i32 > maxNumber {
            let last = self.myAlternatives[self.myLastUsed as usize].clone();
            // only keep the routes with highest probability
            // determinism: C++ std::sort is unstable; std_sort reproduces libstdc++ tie order (equal probabilities tie)
            utils::common::std_sort::std_sort_vec(&mut self.myAlternatives, |a, b| a.get_probability() > b.get_probability());
            if keepRoute {
                for i in 0..self.myAlternatives.len() {
                    if Rc::ptr_eq(&self.myAlternatives[i], &last) {
                        self.myLastUsed = i as i32;
                        break;
                    }
                }
                if self.myLastUsed >= maxNumber {
                    self.myAlternatives.swap((maxNumber - 1) as usize, self.myLastUsed as usize);
                    self.myLastUsed = maxNumber - 1;
                }
            }
            self.myAlternatives.truncate(maxNumber as usize);
        }
        // rescale probabilities
        let mut newSum: f64 = 0.0;
        for alt in self.myAlternatives.iter() {
            newSum += alt.get_probability();
        }
        assert!(newSum > 0.0);
        // @note newSum may be larger than 1 for numerical reasons
        for alt in self.myAlternatives.iter() {
            let p = alt.get_probability() / newSum;
            alt.set_probability(p);
        }

        // find the route to use
        if !keepRoute {
            let mut chosen = utils::common::rand_helper::rand();
            self.myLastUsed = 0;
            for alt in self.myAlternatives.iter() {
                chosen -= alt.get_probability();
                if chosen <= 0.0 {
                    return None;
                }
                self.myLastUsed += 1;
            }
        }
        None
    }

    /// C++: `const ROEdge * RORouteDef::getOrigin() const` [const]
    // const ROEdge*
    // RORouteDef::getOrigin() const {
    //     return myAlternatives.back()->getFirst();
    // }
    #[allow(unused_variables)]
    pub fn get_origin(&self) -> Ptr<ROEdge> {
        self.myAlternatives.last().unwrap().get_first()
    }

    /// C++: `const ROEdge * RORouteDef::getDestination() const` [const]
    // const ROEdge*
    // RORouteDef::getDestination() const {
    //     return myAlternatives.back()->getLast();
    // }
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<ROEdge> {
        self.myAlternatives.last().unwrap().get_last()
    }

    /// C++: `std::shared_ptr<const RORoute> RORouteDef::getFirstRoute() const` [const]
    // std::shared_ptr<const RORoute> getFirstRoute() const {
    //         if (myAlternatives.empty()) {
    //             return nullptr;
    //         }
    //         return myAlternatives.front();
    //     }
    #[allow(unused_variables)]
    pub fn get_first_route(&self) -> Option<Rc<RORoute>> {
        if self.myAlternatives.is_empty() {
            return None;
        }
        Some(self.myAlternatives[0].clone())
    }

    /// C++: `std::shared_ptr<const RORoute> RORouteDef::getUsedRoute() const` [const]
    // std::shared_ptr<const RORoute> getUsedRoute() const {
    //         return myAlternatives[myLastUsed];
    //     }
    #[allow(unused_variables)]
    pub fn get_used_route(&self) -> Rc<RORoute> {
        self.myAlternatives[self.myLastUsed as usize].clone()
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
    pub fn write_xml_definition<'a>(&self, dev: &'a mut OutputDevice, veh: Ptr<ROVehicle>, asAlternatives: bool, withExitTimes: bool, withCost: bool, withLength: bool) -> &'a mut OutputDevice {
        if asAlternatives {
            dev.open_tag_id(SUMO_TAG_ROUTE_DISTRIBUTION).write_attr(SUMO_ATTR_LAST, &self.myLastUsed);
            for i in 0..self.myAlternatives.len() {
                self.myAlternatives[i].write_xml_definition(dev, veh, true, true, withExitTimes, withLength, "");
            }
            dev.close_tag("");
            dev
        } else {
            self.myAlternatives[self.myLastUsed as usize].write_xml_definition(dev, veh, withCost, false, withExitTimes, withLength, "")
        }
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
        let mut result = Box::new(RORouteDef::new(id, 0, self.myTryRepair, self.myMayBeDisconnected));
        for route in self.myAlternatives.iter() {
            let col: Ptr<RGBColor> = if route.get_color().is_null() {
                Ptr::null()
            } else {
                Ptr::from_raw(Box::into_raw(Box::new(*route.get_color().get().unwrap())))
            };
            let mut newRoute = Rc::new(RORoute::new(id, route.get_costs(), route.get_probability(), route.get_edge_vector(), col, route.get_stops()));
            Rc::get_mut(&mut newRoute).unwrap().add_stop_offset(stopOffset);
            result.add_loaded_alternative(newRoute);
        }
        Ptr::from_raw(Box::into_raw(result))
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
        let mut sum = 0.;
        for r in self.myAlternatives.iter() {
            sum += r.get_probability();
        }
        sum
    }

    /// C++: `bool RORouteDef::discardSilent() const` [const]
    /// whether this route shall be silently discarded
    // bool discardSilent() const {
    //         return myDiscardSilent;
    //     }
    #[allow(unused_variables)]
    pub fn discard_silent(&self) -> bool {
        self.myDiscardSilent.get()
    }

    /// C++: `void RORouteDef::setUsingJTRR()` [static]
    // static void setUsingJTRR() {
    //         myUsingJTRR = true;
    //     }
    #[allow(unused_variables)]
    pub fn set_using_jtrr() {
        MY_USING_JTRR.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// C++: `void RORouteDef::setSkipNew()` [static]
    // static void setSkipNew() {
    //         mySkipNewRoutes = true;
    //     }
    #[allow(unused_variables)]
    pub fn set_skip_new() {
        MY_SKIP_NEW_ROUTES.store(true, std::sync::atomic::Ordering::Relaxed);
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
    pub fn back_track(router: &mut SUMOAbstractRouter<ROEdge, ROVehicle>, targets: &ConstROEdgeVector, i: &mut usize, lastMandatory: i32, nextMandatory: Ptr<ROEdge>, newEdges: &mut ConstROEdgeVector, veh: &ROVehicle, begin: SUMOTime) -> bool {
        let mut edges: ConstROEdgeVector = Vec::new();
        let ok = router.compute(newEdges[lastMandatory as usize], nextMandatory, Ptr::new(veh), begin, &mut edges, true);
        if !ok {
            return false;
        }
        while targets[*i] != nextMandatory {
            *i += 1;
        }
        newEdges.truncate((lastMandatory + 1) as usize);
        newEdges.extend(edges.iter().skip(1).cloned());
        true
    }

    /// C++: `void RORouteDef::RORouteDef(const RORouteDef &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &RORouteDef) -> Self {
        panic!("RORouteDef cannot be copied: the copy constructor is invalidated in C++")
    }

    /// C++: `RORouteDef & RORouteDef::operator=(const RORouteDef &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RORouteDef) -> &mut RORouteDef {
        panic!("RORouteDef cannot be assigned: the assignment operator is invalidated in C++")
    }

}

