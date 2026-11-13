//! Port of utils/router/RailwayRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::map<const E *, RouterProhibition> RailwayRouter::Prohibitions`
pub type RailwayRouter_Prohibitions = Unmapped /* std::map<const E *, RouterProhibition> */;

/// C++ typedef `DijkstraRouter<_RailEdge, V> RailwayRouter::_InternalDijkstra`
pub type RailwayRouter__InternalDijkstra<E, V> = DijkstraRouter<RailEdge__RailEdge<E, V>, V>;

/// C++ typedef `SUMOAbstractRouter<_RailEdge, V> RailwayRouter::_InternalRouter`
pub type RailwayRouter__InternalRouter<E, V> = SUMOAbstractRouter<RailEdge__RailEdge<E, V>, V>;

/// C++ typedef `RailEdge<E, V> RailwayRouter::_RailEdge`
pub type RailwayRouter__RailEdge = Unmapped /* RailEdge<E, V> */;

/// The router for rail vehicles for track networks where some sections may be used in both directions and trains may reverse depending on location and train length
/// C++ class `RailwayRouter` (utils/router/RailwayRouter.h)
#[repr(C)]
pub struct RailwayRouter<E, V> {
    /// C++ base class `SUMOAbstractRouter<E, V>`
    pub base: SUMOAbstractRouter<E, V>,
    pub myInternalRouter: Ptr<RailwayRouter__InternalRouter<E, V>>,
    pub myOriginal: Ptr<RailwayRouter<E, V>>,
    /// a RailEdge for every existing edge, filled on construction (but not in clones)
    pub myInitialEdges: Vec<Ptr<RailEdge__RailEdge<E, V>>>,
    /// complete rail network filled on demand (but not in clones)
    pub myRailEdges: Vec<Ptr<RailEdge__RailEdge<E, V>>>,
    /// whether to suppress warning/error if no route was found
    pub mySilent: bool,
    pub myMaxTrainLength: f64,
    /// track previous edge for correct bulk routing
    pub myLastFrom: Ptr<E>,
    _p: std::marker::PhantomData<(E, V,)>,
}

impl<E, V> std::ops::Deref for RailwayRouter<E, V> {
    type Target = SUMOAbstractRouter<E, V>;
    fn deref(&self) -> &SUMOAbstractRouter<E, V> {
        &self.base
    }
}
impl<E, V> std::ops::DerefMut for RailwayRouter<E, V> {
    fn deref_mut(&mut self) -> &mut SUMOAbstractRouter<E, V> {
        &mut self.base
    }
}

// C++ static members of RailwayRouter (port as module-level statics, see AGENTS.md):
//   static typename SUMOAbstractRouter<E, V>::Operation RailwayRouter::myStaticOperation;
//   static double RailwayRouter::myReversalPenalty;
//   static double RailwayRouter::myReversalPenaltyFactor;

impl<E, V> RailwayRouter<E, V> {
    /// C++: `void RailwayRouter::RailwayRouter<E, V>(const std::vector<E *> &, bool, typename SUMOAbstractRouter<E, V>::Operation, typename SUMOAbstractRouter<E, V>::Operation, bool, bool, const bool, double, double)`
    /// Constructor
    // RailwayRouter(const std::vector<E*>& edges, bool unbuildIsWarning, typename SUMOAbstractRouter<E, V>::Operation effortOperation,
    //                   typename SUMOAbstractRouter<E, V>::Operation ttOperation = nullptr, bool silent = false,
    //                   bool havePermissions = false, const bool haveRestrictions = false, double maxTrainLength = 5000,
    //                   double reversalPenalty = 60) :
    //         SUMOAbstractRouter<E, V>("RailwayRouter", unbuildIsWarning, effortOperation, ttOperation, havePermissions, haveRestrictions),
    //         myInternalRouter(nullptr), myOriginal(nullptr), mySilent(silent),
    //         myMaxTrainLength(maxTrainLength),
    //         myLastFrom(nullptr) {
    //         myReversalPenalty = reversalPenalty;
    //         myStaticOperation = effortOperation;
    //         for (const E* const edge : edges) {
    //             myInitialEdges.push_back(edge->getRailwayRoutingEdge());
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new(edges: &Vec<Ptr<E>>, unbuildIsWarning: bool, effortOperation: SUMOAbstractRouter_Operation, ttOperation: SUMOAbstractRouter_Operation, silent: bool, havePermissions: bool, haveRestrictions: bool, maxTrainLength: f64, reversalPenalty: f64) -> Self {
        todo!("port RailwayRouter::RailwayRouter<E, V>")
    }

    /// C++: `void RailwayRouter::~RailwayRouter<E, V>()` [virtual]
    /// Destructor
    // virtual ~RailwayRouter() {
    //         delete myInternalRouter;
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RailwayRouter::~RailwayRouter<E, V>")
    }

    /// C++: `SUMOAbstractRouter<E, V> * RailwayRouter::clone()`
    // SUMOAbstractRouter<E, V>* clone() {
    //         return new RailwayRouter<E, V>(this);
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, V>> {
        todo!("port RailwayRouter::clone")
    }

    /// C++: `bool RailwayRouter::compute(const E *, const E *, const V *const, SUMOTime, std::vector<const E *> &, bool)`
    /// Builds the route between the given edges using the minimum effort at the given time The definition of the effort depends on the wished routing scheme
    // bool compute(const E* from, const E* to, const V* const vehicle, SUMOTime msTime, std::vector<const E*>& into, bool silent = false) {
    //         ensureInternalRouter();
    //         if (vehicle->getLength() > myMaxTrainLength) {
    //             WRITE_WARNINGF("Vehicle '%' with length % exceeds configured value of --railway.max-train-length %",
    //                            vehicle->getID(), toString(vehicle->getLength()), toString(myMaxTrainLength));
    //         }
    //         return _compute(from, to, vehicle, msTime, into, silent, false);
    //     }
    #[allow(unused_variables)]
    pub fn compute(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool {
        todo!("port RailwayRouter::compute")
    }

    /// C++: `void RailwayRouter::prohibit(const Prohibitions &)`
    // void prohibit(const Prohibitions& toProhibit) {
    //         ensureInternalRouter();
    //         typename _InternalRouter::Prohibitions _toProhibit;
    //         for (auto item : toProhibit) {
    //             _toProhibit[item.first->getRailwayRoutingEdge()] = item.second;
    //         }
    //         myInternalRouter->prohibit(_toProhibit);
    //         this->myProhibited = toProhibit;
    // #ifdef RailwayRouter_DEBUG_ROUTES
    //         std::cout << "RailRouter numProhibitions=" << toProhibit.size() << "\n";
    // #endif
    //     }
    #[allow(unused_variables)]
    pub fn prohibit(&mut self, toProhibit: &SUMOAbstractRouter_Prohibitions) {
        todo!("port RailwayRouter::prohibit")
    }

    /// C++: `double RailwayRouter::recomputeCosts(const std::vector<const E *> &, const V *const, SUMOTime, double *) const` [const]
    // double recomputeCosts(const std::vector<const E*>& edges, const V* const v, SUMOTime msTime, double* lengthp = nullptr) const {
    //         double effort = SUMOAbstractRouter<E, V>::recomputeCosts(edges, v, msTime, lengthp);
    //         const E* prev = nullptr;
    //         // reversal corrections
    //         double timeCorrection = STEPS2TIME(msTime);
    //         for (const E* const e : edges) {
    //             if (prev != nullptr && e->getBidiEdge() == prev) {
    //                 if (e->getLength() > v->getLength()) {
    //                     // vehicle doesn't need to drive to the end of the edge
    //                     // @note exact timing correction for time-dependent effort is not done
    //                     const double savingsFactor = 1 - v->getLength() / e->getLength();
    //                     double effortCorrection = 0;
    //                     double lengthCorrection = 0.;
    //                     effortCorrection += this->getEffort(prev, v, timeCorrection);
    //                     this->updateViaCost(prev, e, v, timeCorrection, effortCorrection, lengthCorrection);
    //                     effort -= savingsFactor * effortCorrection;
    //                     if (lengthp != nullptr) {
    //                         *lengthp -= savingsFactor * lengthCorrection;
    //                     }
    //                 }
    //                 effort += myReversalPenalty;
    //             }
    //             prev = e;
    //         }
    //         return effort;
    //     }
    #[allow(unused_variables)]
    pub fn recompute_costs(&self, edges: &Vec<Ptr<E>>, v: Ptr<V>, msTime: SUMOTime, lengthp: Ptr<f64>) -> f64 {
        todo!("port RailwayRouter::recomputeCosts")
    }

    /// C++: `void RailwayRouter::setBulkMode(const bool)`
    // inline void setBulkMode(const bool mode) {
    //         if (myInternalRouter != nullptr) {
    //             myInternalRouter->setBulkMode(mode);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn set_bulk_mode(&mut self, mode: bool) {
        todo!("port RailwayRouter::setBulkMode")
    }

    /// C++: `void RailwayRouter::RailwayRouter<E, V>(RailwayRouter<E, V> *)` [private]
    // RailwayRouter(RailwayRouter* other) :
    //         SUMOAbstractRouter<E, V>(other),
    //         myInternalRouter(nullptr),
    //         myOriginal(other),
    //         mySilent(other->mySilent),
    //         myMaxTrainLength(other->myMaxTrainLength)
    //     {}
    #[allow(unused_variables)]
    pub fn new_2(other: Ptr<RailwayRouter<E, V>>) -> Self {
        todo!("port RailwayRouter::RailwayRouter<E, V>")
    }

    /// C++: `void RailwayRouter::ensureInternalRouter()` [private]
    // void ensureInternalRouter() {
    //         if (myInternalRouter == nullptr) {
    //             myInternalRouter = new _InternalDijkstra(getRailEdges(), this->myErrorMsgHandler == MsgHandler::getWarningInstance(), &getTravelTimeStatic,
    //                     nullptr, mySilent, nullptr, this->myHavePermissions, this->myHaveRestrictions);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn ensure_internal_router(&mut self) {
        todo!("port RailwayRouter::ensureInternalRouter")
    }

    /// C++: `bool RailwayRouter::_compute(const E *, const E *, const V *const, SUMOTime, std::vector<const E *> &, bool, bool)` [private]
    // bool _compute(const E* from, const E* to, const V* const vehicle, SUMOTime msTime, std::vector<const E*>& into, bool silent, bool avoidUnsafeBackTracking) {
    //         // make sure that the vehicle can turn-around when starting on a short edge (the virtual turn-around for this lies backwards along the route / track)
    //         // if reversals are forbidden (negative penalty), we don't need to check this
    //         std::vector<double> backLengths;
    //         double backDist = myReversalPenalty >= 0 ? vehicle->getLength() - from->getLength() : 0;
    //         const E* start = from;
    //         while (backDist > 0) {
    //             const E* prev = getStraightPredecessor(start, into, (int)backLengths.size());
    //             if (prev == nullptr || prev == to) {
    // #ifdef RailwayRouter_DEBUG_ROUTES
    //                 std::cout << " Could not determine back edge for vehicle '" << vehicle->getID() << "' when routing from edge '" << from->getID() << "' at time=" << time2string(msTime) << "\n";
    // #endif
    //                 //WRITE_WARNING("Could not determine back edge for vehicle '" + vehicle->getID() + "' when routing from edge '" + from->getID() + "' at time=" + time2string(msTime));
    //                 break;
    //             }
    //             backDist -= prev->getLength();
    //             if (avoidUnsafeBackTracking && prev->getSuccessors().size() > 1) {
    //                 bool foundSwitch = false;
    //                 for (const E* succ : prev->getSuccessors()) {
    //                     if (succ != start && succ != prev->getBidiEdge()) {
    //                         foundSwitch = true;
    //                         break;
    //                     }
    //                 }
    //                 if (foundSwitch) {
    //                     break;
    //                 }
    //             }
    //             backLengths.push_back(prev->getLength() + (backLengths.empty()
    //                                   ? MIN2(vehicle->getLength(), from->getLength())
    //                                   : backLengths.back()));
    //             start = prev;
    //         }
    //
    //         std::vector<const _RailEdge*> intoTmp;
    //         if (myLastFrom != start) {
    //             myInternalRouter->setBulkMode(false);
    //         }
    //         bool success = myInternalRouter->compute(start->getRailwayRoutingEdge(), to->getRailwayRoutingEdge(), vehicle, msTime, intoTmp, silent);
    //         myLastFrom = start;
    // #ifdef RailwayRouter_DEBUG_ROUTES
    //         std::cout << "RailRouter veh=" << vehicle->getID() << " from=" << from->getID() << " to=" << to->getID() << " t=" << time2string(msTime)
    //                   << " safe=" << avoidUnsafeBackTracking << " success=" << success << " into=" << toString(into) << "\n";
    // #endif
    //         if (success) {
    //             const size_t intoSize = into.size();
    //             int backIndex = (int)backLengths.size();
    //             for (const _RailEdge* railEdge : intoTmp) {
    //                 if (railEdge->getOriginal() != nullptr) {
    //                     backIndex--;
    //                 }
    //                 // prevent premature reversal on back edge (extend train length)
    //                 const double length = backIndex >= 0 ? backLengths[backIndex] : vehicle->getLength();
    //                 railEdge->insertOriginalEdges(length, into);
    //             }
    // #ifdef RailwayRouter_DEBUG_ROUTES
    //             std::cout << "RailRouter: internal result=" << toString(intoTmp) << "\n";
    //             std::cout << "RailRouter: expanded result=" << toString(into) << "\n";
    //             std::cout << "RailRouter: backLengths=" << toString(backLengths) << " bls=" << backLengths.size() << " intoSize=" << intoSize << " final result=" << toString(into) << "\n";
    // #endif
    //             if (backLengths.size() > 0) {
    //                 bool validBackTracking = intoSize + backLengths.size() <= into.size();
    //                 // skip the virtual back-edges
    //                 if (validBackTracking) {
    //                     into.erase(into.begin() + intoSize, into.begin() + intoSize + backLengths.size());
    //                     validBackTracking = *(into.begin() + intoSize) == from;
    //                 }
    //                 if (!validBackTracking) {
    //                     if (!avoidUnsafeBackTracking) {
    //                         // try again, this time with more safety (but unable to
    //                         // make use of turn-arounds on short edge)
    //                         into.erase(into.begin() + intoSize, into.end());
    //                         // we are starting from a different edge and are thus violating the assumptions of bulk mode
    //                         success = _compute(from, to, vehicle, msTime, into, silent, true);
    //                         return success;
    //                     } else {
    //                         WRITE_WARNING("Railway routing failure due to turn-around on short edge '" + from->getID()
    //                                       + "' for vehicle '" + vehicle->getID() + "' time=" + time2string(msTime) + ".");
    //                         into.erase(into.begin() + intoSize, into.end());
    //                         return false;
    //                     }
    //                 }
    //             }
    //             if (this->myProhibited.size() > 0) {
    //                 // make sure that turnarounds don't use prohibited edges
    //                 for (const E* e : into) {
    //                     if (this->myProhibited.find(e) != this->myProhibited.end()) {
    //                         into.clear();
    //                         success = false;
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //         return success;
    //
    //     }
    #[allow(unused_variables)]
    pub fn compute_2(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool, avoidUnsafeBackTracking: bool) -> bool {
        todo!("port RailwayRouter::_compute")
    }

    /// C++: `const std::vector<_RailEdge *> & RailwayRouter::getRailEdges()` [private]
    // const std::vector<_RailEdge*>& getRailEdges() {
    //         if (myOriginal != nullptr) {
    //             return myOriginal->getRailEdges();
    //         }
    // #ifdef HAVE_FOX
    //         FXMutexLock locker(myLock);
    // #endif
    //         if (myRailEdges.empty()) {
    //             myRailEdges = myInitialEdges;
    //             int numericalID = myInitialEdges.back()->getNumericalID() + 1;
    //             for (_RailEdge* railEdge : myInitialEdges) {
    //                 railEdge->init(myRailEdges, numericalID, myMaxTrainLength, myReversalPenalty >= 0);
    //             }
    //         }
    //         return myRailEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_rail_edges(&mut self) -> &Vec<Ptr<RailEdge__RailEdge<E, V>>> {
        todo!("port RailwayRouter::getRailEdges")
    }

    /// C++: `double RailwayRouter::getTravelTimeStatic(const RailEdge<E, V> *const, const V *const, double)` [static, private]
    // static inline double getTravelTimeStatic(const RailEdge<E, V>* const edge, const V* const veh, double time) {
    //         if (edge->getOriginal() != nullptr) {
    //             return (*myStaticOperation)(edge->getOriginal(), veh, time);
    //         } else {
    //             // turnaround edge
    //             if (edge->isVirtual()) {
    //                 // add up time for replacement edges
    //                 std::vector<const E*> repl;
    //                 edge->insertOriginalEdges(veh->getLength(), repl);
    //                 assert(repl.size() > 0);
    //                 double seenDist = 0;
    //                 double result = 0;
    //                 repl.pop_back(); // last edge must not be used fully
    //                 for (const E* e : repl) {
    //                     result += (*myStaticOperation)(e, veh, time + result);
    //                     seenDist += e->getLength();
    //                 }
    //                 const double lengthOnLastEdge = MAX2(0.0, veh->getLength() - seenDist);
    //                 return result + myReversalPenalty + lengthOnLastEdge * myReversalPenaltyFactor;
    //             } else {
    //                 // if the edge from which this turnaround starts is longer
    //                 // than the vehicle then we are overstimating the cost
    //                 // (because the turnaround may happen before driving to the end)
    //                 // However, unless the vehicle starts on this edge, we should be taking a
    //                 // virtual turnaround at the end of the previous edge. Otherwise, the exaggerated cost doesn't
    //                 return myReversalPenalty;
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static(edge: Ptr<RailEdge<E, V>>, veh: Ptr<V>, time: f64) -> f64 {
        todo!("port RailwayRouter::getTravelTimeStatic")
    }

    /// C++: `const E * RailwayRouter::getStraightPredecessor(const E *, std::vector<const E *> &, int)` [static, private]
    // static const E* getStraightPredecessor(const E* edge, std::vector<const E*>& prevRoute, int backIndex) {
    //         const E* result = nullptr;
    // #ifdef RailwayRouter_DEBUG_ROUTES
    //         std::cout << "  getStraightPredecessor edge=" << edge->getID() << " prevRouteSize=" << prevRoute.size() << " backIndex=" << backIndex << "\n";
    // #endif
    //         if ((int)prevRoute.size() > backIndex) {
    //             return prevRoute[(int)prevRoute.size() - 1 - backIndex];
    //         }
    //         for (const E* cand : edge->getPredecessors()) {
    //             if (!cand->isInternal() && cand->getBidiEdge() != edge) {
    //                 //std::cout << "    cand=" << cand->getID() << "\n";
    //                 if (result == nullptr) {
    //                     result = cand;
    //                 } else {
    //                     // predecessor not unique. Better abort with a warning
    //                     return nullptr;
    //                 }
    //             }
    //         }
    //         return result;
    //     }
    #[allow(unused_variables)]
    pub fn get_straight_predecessor(edge: Ptr<E>, prevRoute: &mut Vec<Ptr<E>>, backIndex: i32) -> Ptr<E> {
        todo!("port RailwayRouter::getStraightPredecessor")
    }

    /// C++: `RailwayRouter<E, V> & RailwayRouter::operator=(const RailwayRouter<E, V> &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &RailwayRouter<E, V>) -> &mut RailwayRouter<E, V> {
        todo!("port RailwayRouter::operator=")
    }

}

