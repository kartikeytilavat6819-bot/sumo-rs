//! Port of utils/router/RailwayRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::{RouterEdge, RouterVehicle};

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

thread_local! {
    /// C++ static `RailwayRouter::myReversalPenalty` (set by the constructor)
    static MY_REVERSAL_PENALTY: std::cell::Cell<f64> = std::cell::Cell::new(60.);
    /// C++ static `RailwayRouter::myReversalPenaltyFactor`
    static MY_REVERSAL_PENALTY_FACTOR: std::cell::Cell<f64> = std::cell::Cell::new(0.2);
}

/// MsgHandler::getWarningInstance() / getErrorInstance()
fn msg_instance(warning: bool) -> Ptr<MsgHandler> {
    utils::common::msg_handler::with_instance(if warning { MsgType::Warning } else { MsgType::Error }, |h| Ptr::from_mut(h))
}

/// The `RailwayRouter::compute` reached from the base router's compute slot (C++ virtual dispatch)
pub fn railway_compute<E: RouterEdge<V>, V: RouterVehicle>(base: &mut SUMOAbstractRouter<E, V>, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool {
    Ptr::from_mut(base).cast::<RailwayRouter<E, V>>().m().compute(from, to, vehicle, msTime, into, silent)
}

impl<E: RouterEdge<V>, V: RouterVehicle> RailwayRouter<E, V> {
    /// Installs this router's compute into the base, so a `SUMOAbstractRouter` pointer (C++ virtual call) reaches it.
    pub fn with_compute(mut self) -> Self {
        self.base.myCompute = Some(railway_compute::<E, V>);
        self
    }

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
    pub fn new(edges: &Vec<Ptr<E>>, unbuildIsWarning: bool, effortOperation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, ttOperation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, silent: bool, havePermissions: bool, haveRestrictions: bool, maxTrainLength: f64, reversalPenalty: f64) -> Self
    where
        E: RouterEdge<V>,
    {
        MY_REVERSAL_PENALTY.with(|p| p.set(reversalPenalty));
        // shortcut: effortOperation is not kept as myStaticOperation, SUMOAbstractRouter_Operation is Unmapped (fn pointer)
        let myInitialEdges = edges.iter().map(|e| e.railway_routing_edge()).collect();
        Self {
            base: SUMOAbstractRouter::new("RailwayRouter", unbuildIsWarning, effortOperation, ttOperation, havePermissions, haveRestrictions),
            myInternalRouter: Ptr::null(),
            myOriginal: Ptr::null(),
            myInitialEdges,
            myRailEdges: Vec::new(),
            mySilent: silent,
            myMaxTrainLength: maxTrainLength,
            myLastFrom: Ptr::null(),
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void RailwayRouter::~RailwayRouter<E, V>()` [virtual]
    /// Destructor
    // virtual ~RailwayRouter() {
    //         delete myInternalRouter;
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if !self.myInternalRouter.is_null() {
            let internal: Ptr<DijkstraRouter<RailEdge<E, V>, V>> = self.myInternalRouter.cast();
            drop(unsafe { Box::from_raw(internal.as_raw()) });
        }
    }

    /// C++: `SUMOAbstractRouter<E, V> * RailwayRouter::clone()`
    // SUMOAbstractRouter<E, V>* clone() {
    //         return new RailwayRouter<E, V>(this);
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, V>> {
        Ptr::from_raw(Box::into_raw(Box::new(Self::new_2(Ptr::from_mut(self))))).cast()
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
    pub fn compute(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool
    where
        E: RouterEdge<V>,
        V: RouterVehicle,
    {
        self.ensure_internal_router();
        if vehicle.length() > self.myMaxTrainLength {
            utils::common::msg_handler::write_warning(&format!(
                "Vehicle '{}' with length {} exceeds configured value of --railway.max-train-length {}",
                vehicle.id(),
                utils::common::to_string::to_string(&vehicle.length()),
                utils::common::to_string::to_string(&self.myMaxTrainLength)
            ));
        }
        self.compute_2(from, to, vehicle, msTime, into, silent, false)
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
    pub fn prohibit(&mut self, toProhibit: &SUMOAbstractRouter_Prohibitions)
    where
        E: RouterEdge<V>,
    {
        self.ensure_internal_router();
        let mut toProhibitRE: SUMOAbstractRouter_Prohibitions = Default::default();
        for (edge, prohibition) in toProhibit {
            let edge: Ptr<E> = (*edge).cast();
            // shortcut: SUMOAbstractRouter_Prohibitions is keyed by Ptr<Unmapped>, cast the address until that typedef is generic
            toProhibitRE.insert(edge.railway_routing_edge().cast(), *prohibition);
        }
        self.myInternalRouter.m().prohibit(&toProhibitRE);
        self.base.myProhibited = toProhibit.clone();
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
    pub fn recompute_costs(&self, edges: &Vec<Ptr<E>>, v: Ptr<V>, msTime: SUMOTime, lengthp: Ptr<f64>) -> f64
    where
        E: RouterEdge<V>,
        V: RouterVehicle,
    {
        let mut effort = self.base.recompute_costs(edges, v, msTime, lengthp);
        let mut prev: Ptr<E> = Ptr::null();
        // reversal corrections
        let mut timeCorrection = utils::common::time::steps2time(msTime);
        for e in edges.iter() {
            if !prev.is_null() && e.bidi_edge() == prev {
                if e.length() > v.length() {
                    // vehicle doesn't need to drive to the end of the edge
                    // @note exact timing correction for time-dependent effort is not done
                    let savingsFactor = 1. - v.length() / e.length();
                    let mut effortCorrection = 0.;
                    let mut lengthCorrection = 0.;
                    effortCorrection += self.get_effort(prev, v, timeCorrection);
                    self.update_via_cost(prev, *e, v, &mut timeCorrection, &mut effortCorrection, &mut lengthCorrection);
                    effort -= savingsFactor * effortCorrection;
                    if !lengthp.is_null() {
                        *lengthp.m() -= savingsFactor * lengthCorrection;
                    }
                }
                effort += MY_REVERSAL_PENALTY.with(|p| p.get());
            }
            prev = *e;
        }
        effort
    }

    /// C++: `void RailwayRouter::setBulkMode(const bool)`
    // inline void setBulkMode(const bool mode) {
    //         if (myInternalRouter != nullptr) {
    //             myInternalRouter->setBulkMode(mode);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn set_bulk_mode(&mut self, mode: bool) {
        if !self.myInternalRouter.is_null() {
            self.myInternalRouter.m().set_bulk_mode(mode);
        }
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
        Self {
            base: SUMOAbstractRouter::new_2(Ptr::from_mut(&mut other.m().base)),
            myInternalRouter: Ptr::null(),
            myOriginal: other,
            myInitialEdges: Vec::new(),
            myRailEdges: Vec::new(),
            mySilent: other.mySilent,
            myMaxTrainLength: other.myMaxTrainLength,
            myLastFrom: Ptr::null(),
            _p: std::marker::PhantomData,
        }
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
        if self.myInternalRouter.is_null() {
            let warning = self.base.myErrorMsgHandler == msg_instance(true);
            let edges = self.get_rail_edges().clone();
            let internal = DijkstraRouter::<RailEdge<E, V>, V>::new(&edges, warning, Some(Self::get_travel_time_static), None, self.mySilent, Ptr::null(), self.base.myHavePermissions, self.base.myHaveRestrictions);
            self.myInternalRouter = Ptr::from_raw(Box::into_raw(Box::new(internal))).cast();
        }
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
    pub fn compute_2(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool, avoidUnsafeBackTracking: bool) -> bool
    where
        E: RouterEdge<V>,
        V: RouterVehicle,
    {
        // make sure that the vehicle can turn-around when starting on a short edge (the virtual turn-around for this lies backwards along the route / track)
        // if reversals are forbidden (negative penalty), we don't need to check this
        let mut backLengths: Vec<f64> = Vec::new();
        let mut backDist = if MY_REVERSAL_PENALTY.with(|p| p.get()) >= 0. { vehicle.length() - from.length() } else { 0. };
        let mut start = from;
        while backDist > 0. {
            let prev = Self::get_straight_predecessor(start, into, backLengths.len() as i32);
            if prev.is_null() || prev == to {
                break;
            }
            backDist -= prev.length();
            if avoidUnsafeBackTracking && prev.successors(SVC_IGNORING).len() > 1 {
                let mut foundSwitch = false;
                for succ in prev.successors(SVC_IGNORING) {
                    if *succ != start && *succ != prev.bidi_edge() {
                        foundSwitch = true;
                        break;
                    }
                }
                if foundSwitch {
                    break;
                }
            }
            let back = if backLengths.is_empty() { vehicle.length().min(from.length()) } else { *backLengths.last().unwrap() };
            backLengths.push(prev.length() + back);
            start = prev;
        }

        let mut intoTmp: Vec<Ptr<RailEdge<E, V>>> = Vec::new();
        if self.myLastFrom != start {
            self.myInternalRouter.m().set_bulk_mode(false);
        }
        let mut success = self.myInternalRouter.m().compute(start.railway_routing_edge(), to.railway_routing_edge(), vehicle, msTime, &mut intoTmp, silent);
        self.myLastFrom = start;
        if success {
            let intoSize = into.len();
            let mut backIndex = backLengths.len() as i32;
            for railEdge in intoTmp.iter() {
                if !railEdge.get_original().is_null() {
                    backIndex -= 1;
                }
                // prevent premature reversal on back edge (extend train length)
                let length = if backIndex >= 0 { backLengths[backIndex as usize] } else { vehicle.length() };
                railEdge.insert_original_edges(length, into);
            }
            if backLengths.len() > 0 {
                let mut validBackTracking = intoSize + backLengths.len() <= into.len();
                // skip the virtual back-edges
                if validBackTracking {
                    into.drain(intoSize..intoSize + backLengths.len());
                    // C++ dereferences into[intoSize] without a bounds check
                    validBackTracking = into.get(intoSize) == Some(&from);
                }
                if !validBackTracking {
                    if !avoidUnsafeBackTracking {
                        // try again, this time with more safety (but unable to make use of turn-arounds on short edge)
                        into.truncate(intoSize);
                        return self.compute_2(from, to, vehicle, msTime, into, silent, true);
                    } else {
                        utils::common::msg_handler::write_warning(&format!(
                            "Railway routing failure due to turn-around on short edge '{}' for vehicle '{}' time={}.",
                            from.id(),
                            vehicle.id(),
                            utils::common::time::time2string(msTime)
                        ));
                        into.truncate(intoSize);
                        return false;
                    }
                }
            }
            // make sure that turnarounds don't use prohibited edges
            if into.iter().any(|e| self.base.myProhibited.contains_key(&e.cast::<Unmapped>())) {
                into.clear();
                success = false;
            }
        }
        success
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
        if !self.myOriginal.is_null() {
            return self.myOriginal.m().get_rail_edges();
        }
        if self.myRailEdges.is_empty() {
            self.myRailEdges = self.myInitialEdges.clone();
            let mut numericalID = self.myInitialEdges.last().unwrap().get_numerical_id() + 1;
            let permitReversal = MY_REVERSAL_PENALTY.with(|p| p.get()) >= 0.;
            for railEdge in self.myInitialEdges.clone() {
                railEdge.m().init(&mut self.myRailEdges, &mut numericalID, self.myMaxTrainLength, permitReversal);
            }
        }
        &self.myRailEdges
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
        if !edge.get_original().is_null() || edge.is_virtual() {
            // shortcut: the effort function myStaticOperation is Unmapped (SUMOAbstractRouter_Operation), upgrade when the operation type is ported
            panic!("shortcut: RailwayRouter::getTravelTimeStatic needs myStaticOperation");
        }
        // turnaround edge that is not virtual: the reversal penalty only
        MY_REVERSAL_PENALTY.with(|p| p.get())
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
    pub fn get_straight_predecessor(edge: Ptr<E>, prevRoute: &mut Vec<Ptr<E>>, backIndex: i32) -> Ptr<E>
    where
        E: RouterEdge<V>,
    {
        if prevRoute.len() as i32 > backIndex {
            return prevRoute[(prevRoute.len() as i32 - 1 - backIndex) as usize];
        }
        let mut result: Ptr<E> = Ptr::null();
        for cand in edge.predecessors() {
            if !cand.is_internal() && cand.bidi_edge() != edge {
                // predecessor not unique. Better abort with a warning
                if result.is_null() {
                    result = *cand;
                } else {
                    return Ptr::null();
                }
            }
        }
        result
    }

    /// C++: `RailwayRouter<E, V> & RailwayRouter::operator=(const RailwayRouter<E, V> &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &RailwayRouter<E, V>) -> &mut RailwayRouter<E, V> {
        panic!("RailwayRouter::operator= is declared but not defined")
    }

}

