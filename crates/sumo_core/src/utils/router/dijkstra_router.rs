//! Port of utils/router/DijkstraRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::ch_builder::{std_pop_heap, std_push_heap};
use crate::utils::router::router_edge::{RouterEdge, RouterVehicle};

fn msg_instance(warning: bool) -> Ptr<MsgHandler> {
    utils::common::msg_handler::with_instance(if warning { utils::common::msg_handler::MsgType::Warning } else { utils::common::msg_handler::MsgType::Error }, |h| Ptr::from_mut(h))
}

/// C++ `Named::getIDSecure(vehicle)`
fn vehicle_id_secure<V: RouterVehicle>(vehicle: Ptr<V>) -> String {
    if vehicle.is_null() { "NULL".to_string() } else { vehicle.get_id() }
}

/// the edge of an EdgeInfo is always an `E` (SUMOAbstractRouter keeps it as an untyped pointer)
fn edge_numerical_id<E: RouterEdge<V>, V>(edge: Ptr<()>) -> i32 {
    edge.cast::<E>().numerical_id()
}

/// The `DijkstraRouter::compute` reached from the base router's compute slot (C++ virtual dispatch)
pub fn dijkstra_compute<E: RouterEdge<V>, V: RouterVehicle>(base: &mut SUMOAbstractRouter<E, V>, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool {
    Ptr::from_mut(base).cast::<DijkstraRouter<E, V>>().m().compute(from, to, vehicle, msTime, into, silent)
}

/// Computes the shortest path through a network using the Dijkstra algorithm.
/// C++ class `DijkstraRouter` (utils/router/DijkstraRouter.h)
#[repr(C)]
pub struct DijkstraRouter<E, V> {
    /// C++ base class `SUMOAbstractRouter<E, V>`
    pub base: SUMOAbstractRouter<E, V>,
    /// whether to suppress warning/error if no route was found
    pub mySilent: bool,
    /// cache of the last query to enable automated bulk routing
    pub myLastQuery: (Ptr<E>, Ptr<V>, SUMOTime),
    pub myExternalEffort: Ptr<EffortCalculator>,
    pub myComparator: DijkstraRouter_EdgeInfoByEffortComparator,
    _p: std::marker::PhantomData<(E, V,)>,
}

impl<E, V> std::ops::Deref for DijkstraRouter<E, V> {
    type Target = SUMOAbstractRouter<E, V>;
    fn deref(&self) -> &SUMOAbstractRouter<E, V> {
        &self.base
    }
}
impl<E, V> std::ops::DerefMut for DijkstraRouter<E, V> {
    fn deref_mut(&mut self) -> &mut SUMOAbstractRouter<E, V> {
        &mut self.base
    }
}

impl<E, V> DijkstraRouter<E, V> {
    /// C++: `void DijkstraRouter::DijkstraRouter<E, V>(const std::vector<E *> &, bool, typename SUMOAbstractRouter<E, V>::Operation, typename SUMOAbstractRouter<E, V>::Operation, bool, EffortCalculator *, const bool, const bool)`
    /// Constructor
    // DijkstraRouter(const std::vector<E*>& edges, bool unbuildIsWarning, typename SUMOAbstractRouter<E, V>::Operation effortOperation,
    //                    typename SUMOAbstractRouter<E, V>::Operation ttOperation = nullptr, bool silent = false, EffortCalculator* calc = nullptr,
    //                    const bool havePermissions = false, const bool haveRestrictions = false) :
    //         SUMOAbstractRouter<E, V>("DijkstraRouter", unbuildIsWarning, effortOperation, ttOperation, havePermissions, haveRestrictions),
    //         mySilent(silent), myExternalEffort(calc) {
    //         for (typename std::vector<E*>::const_iterator i = edges.begin(); i != edges.end(); ++i) {
    //             this->myEdgeInfos.push_back(typename SUMOAbstractRouter<E, V>::EdgeInfo(*i));
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new(edges: &Vec<Ptr<E>>, unbuildIsWarning: bool, effortOperation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, ttOperation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, silent: bool, calc: Ptr<EffortCalculator>, havePermissions: bool, haveRestrictions: bool) -> Self {
        let mut base = SUMOAbstractRouter::new("DijkstraRouter", unbuildIsWarning, effortOperation, ttOperation, havePermissions, haveRestrictions);
        for e in edges.iter() {
            base.myEdgeInfos.push(SUMOAbstractRouter_EdgeInfo::new(e.cast::<()>()));
        }
        Self {
            base,
            mySilent: silent,
            myLastQuery: Default::default(),
            myExternalEffort: calc,
            myComparator: DijkstraRouter_EdgeInfoByEffortComparator {},
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void DijkstraRouter::~DijkstraRouter<E, V>()` [virtual]
    /// Destructor
    // virtual ~DijkstraRouter() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `SUMOAbstractRouter<E, V> * DijkstraRouter::clone()` [virtual]
    // virtual SUMOAbstractRouter<E, V>* clone() {
    //         auto clone = new DijkstraRouter<E, V>(this->myEdgeInfos, this->myErrorMsgHandler == MsgHandler::getWarningInstance(),
    //                                               this->myOperation, this->myTTOperation, mySilent, myExternalEffort,
    //                                               this->myHavePermissions, this->myHaveRestrictions);
    //         clone->setAutoBulkMode(this->myAutoBulkMode);
    //         return clone;
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, V>> {
        let warning = self.base.myErrorMsgHandler == msg_instance(true);
        let mut clone: DijkstraRouter<E, V> = DijkstraRouter::new_2(&self.base.myEdgeInfos, warning, self.base.myOperation, self.base.myTTOperation, self.mySilent, self.myExternalEffort, self.base.myHavePermissions, self.base.myHaveRestrictions);
        clone.set_auto_bulk_mode(self.base.myAutoBulkMode);
        clone.base.myCompute = self.base.myCompute;
        Ptr::from_raw(Box::into_raw(Box::new(clone))).cast::<SUMOAbstractRouter<E, V>>()
    }

}

impl<E: RouterEdge<V>, V: RouterVehicle> DijkstraRouter<E, V> {
    /// Installs this router's compute into the base, so a `SUMOAbstractRouter` pointer (C++ virtual call) reaches it.
    pub fn with_compute(mut self) -> Self {
        self.base.myCompute = Some(dijkstra_compute::<E, V>);
        self
    }

    /// C++: `bool DijkstraRouter::compute(const E *, const E *, const V *const, SUMOTime, std::vector<const E *> &, bool)`
    /// Builds the route between the given edges using the minimum effort at the given time The definition of the effort depends on the wished routing scheme
    // bool compute(const E* from, const E* to, const V* const vehicle,
    //                  SUMOTime msTime, std::vector<const E*>& into, bool silent = false) {
    //         assert(from != nullptr && (vehicle == nullptr || to != nullptr));
    //         // check whether from and to can be used
    //         if (this->isProhibited(from, vehicle, STEPS2TIME(msTime))) {
    //             if (!silent) {
    //                 this->myErrorMsgHandler->inform("Vehicle '" + Named::getIDSecure(vehicle) + "' is not allowed on source edge '" + from->getID() + "'.");
    //             }
    //             return false;
    //         }
    //         // technically, a temporary permission might be lifted by the time of arrival
    //         if (to != nullptr && this->isProhibited(to, vehicle, STEPS2TIME(msTime))) {
    //             if (!silent) {
    //                 this->myErrorMsgHandler->inform("Vehicle '" + Named::getIDSecure(vehicle) + "' is not allowed on destination edge '" + to->getID() + "'.");
    //             }
    //             return false;
    //         }
    //         double length = 0.; // dummy for the via edge cost update
    //         this->startQuery();
    // #if defined(DijkstraRouter_DEBUG_QUERY) || defined(DijkstraRouter_DEBUG_QUERY_PERF)
    //         std::cout << "DEBUG: starting search for '" << Named::getIDSecure(vehicle) << "' from '" << Named::getIDSecure(from) << "' to '" << Named::getIDSecure(to) << "' time: " << STEPS2TIME(msTime) << "\n";
    // #endif
    //         const SUMOVehicleClass vClass = vehicle == nullptr ? SVC_IGNORING : vehicle->getVClass();
    //         const bool ignoreTransient = vehicle == nullptr ? false : vehicle->ignoreTransientPermissions();
    //         std::tuple<const E*, const V*, SUMOTime> query = std::make_tuple(from, vehicle, msTime);
    //         if (((this->myBulkMode && from == std::get<0>(myLastQuery)) || (this->myAutoBulkMode && query == myLastQuery)) && !this->myAmClean) {
    // #ifdef DijkstraRouter_DEBUG_BULKMODE
    //             if (query != myLastQuery) {
    //                 std::cout << " invalid bulk mode. myLastQuery="
    //                           << std::get<0>(myLastQuery)->getID() << ","
    //                           << std::get<1>(myLastQuery)->getID() << ","
    //                           << time2string(std::get<2>(myLastQuery))
    //                           << " query="
    //                           << std::get<0>(query)->getID() << ","
    //                           << std::get<1>(query)->getID() << ","
    //                           << time2string(std::get<2>(query))
    //                           << "\n";
    //             } else {
    //                 std::cout << " using bulk mode\n";
    //             }
    // #endif
    //             const auto& toInfo = this->myEdgeInfos[to->getNumericalID()];
    //             if (toInfo.visited) {
    //                 this->buildPathFrom(&toInfo, into);
    //                 this->endQuery(1);
    // #ifdef DijkstraRouter_DEBUG_QUERY_PERF
    //                 std::cout << " instant bulk success for vehicle " << vehicle->getID() << "\n";
    // #endif
    //                 return true;
    //             }
    //         } else {
    //             this->init(from->getNumericalID(), msTime);
    //             if (myExternalEffort != nullptr) {
    //                 myExternalEffort->setInitialState(from->getNumericalID());
    //             }
    //             this->myAmClean = false;
    //         }
    //         myLastQuery = query;
    //         // loop
    //         int num_visited = 0;
    // #ifdef DijkstraRouter_DEBUG_QUERY_VISITED
    //         DijkstraRouter_DEBUG_QUERY_VISITED_OUT << "<edgeData>\n <interval begin=\"" << time2string(msTime) << "\" end=\"" << toString(msTime + 1000) << "\" id=\"" << Named::getIDSecure(vehicle) << "\">\n";
    // #endif
    //         while (!this->myFrontierList.empty()) {
    //             num_visited += 1;
    //             // use the node with the minimal length
    //             auto* const minimumInfo = this->myFrontierList.front();
    //             const E* const minEdge = minimumInfo->edge;
    // #ifdef DijkstraRouter_DEBUG_QUERY
    //             std::cout << "DEBUG: hit=" << minEdge->getID()
    //                       << " TT=" << minimumInfo->effort
    //                       << " EF=" << this->getEffort(minEdge, vehicle, minimumInfo->leaveTime)
    //                       << " Leave: " << minimumInfo->leaveTime << " Q: ";
    // #ifdef DijkstraRouter_DEBUG_QUERY_FRONTIER
    //             for (const auto& edgeInfo : this->myFrontierList) {
    //                 std::cout << "\n   " << edgeInfo->effort << ", " << edgeInfo->edge->getID();
    //             }
    // #endif
    //             std::cout << "\n";
    // #endif
    // #ifdef DijkstraRouter_DEBUG_QUERY_VISITED
    //             DijkstraRouter_DEBUG_QUERY_VISITED_OUT << "  <edge id=\"" << minEdge->getID() << "\" index=\"" << num_visited << "\" cost=\"" << minimumInfo->effort << "\" time=\"" << minimumInfo->leaveTime << "\"/>\n";
    // #endif
    //             minimumInfo->visited = true;
    //             // check whether the destination node was already reached
    //             if (minEdge == to) {
    //                 //propagate last external effort state to destination edge
    //                 if (myExternalEffort != nullptr) {
    //                     myExternalEffort->update(minEdge->getNumericalID(), minimumInfo->prev->edge->getNumericalID(), minEdge->getLength());
    //                 }
    //                 this->buildPathFrom(minimumInfo, into);
    //                 this->endQuery(num_visited);
    // #ifdef DijkstraRouter_DEBUG_QUERY_PERF
    //                 const double cost = this->recomputeCosts(into, vehicle, msTime);
    //                 std::cout << "visited " + toString(num_visited) + " edges (final path length=" + toString(into.size()) + " cost=" << cost << " edges=" + toString(into) + ")\n";
    // #endif
    // #ifdef DijkstraRouter_DEBUG_QUERY_VISITED
    //                 DijkstraRouter_DEBUG_QUERY_VISITED_OUT << " </interval>\n</edgeData>\n";
    // #endif
    //                 return true;
    //             }
    //             std::pop_heap(this->myFrontierList.begin(), this->myFrontierList.end(), myComparator);
    //             this->myFrontierList.pop_back();
    //             this->myFound.push_back(minimumInfo);
    //             const double effortDelta = this->getEffort(minEdge, vehicle, minimumInfo->leaveTime);
    //             const double leaveTime = minimumInfo->leaveTime + this->getTravelTime(minEdge, vehicle, minimumInfo->leaveTime, effortDelta);
    //             if (myExternalEffort != nullptr) {
    //                 myExternalEffort->update(minEdge->getNumericalID(), minimumInfo->prev->edge->getNumericalID(), minEdge->getLength());
    //             }
    //             // check all ways from the node with the minimal length
    //             for (const std::pair<const E*, const E*>& follower : minEdge->getViaSuccessors(vClass, ignoreTransient)) {
    //                 auto& followerInfo = this->myEdgeInfos[follower.first->getNumericalID()];
    //                 // check whether it can be used
    //                 if (this->isProhibited(follower.first, vehicle, leaveTime)) {
    //                     continue;
    //                 }
    //                 double effort = minimumInfo->effort + effortDelta;
    //                 double time = leaveTime;
    //                 this->updateViaEdgeCost(follower.second, vehicle, time, effort, length);
    //                 assert(effort >= minimumInfo->effort);
    //                 assert(time >= minimumInfo->leaveTime);
    //                 const double oldEffort = followerInfo.effort;
    //                 if (!followerInfo.visited && effort < oldEffort) {
    //                     followerInfo.effort = effort;
    //                     followerInfo.leaveTime = time;
    //                     followerInfo.prev = minimumInfo;
    //                     if (oldEffort == std::numeric_limits<double>::max()) {
    //                         this->myFrontierList.push_back(&followerInfo);
    //                         std::push_heap(this->myFrontierList.begin(), this->myFrontierList.end(), myComparator);
    //                     } else {
    //                         std::push_heap(this->myFrontierList.begin(),
    //                                        std::find(this->myFrontierList.begin(), this->myFrontierList.end(), &followerInfo) + 1,
    //                                        myComparator);
    //                     }
    //                 }
    //             }
    //         }
    //         this->endQuery(num_visited);
    // #ifdef DijkstraRouter_DEBUG_QUERY_PERF
    //         std::cout << "visited " + toString(num_visited) + " edges (unsuccessful path length: " + toString(into.size()) + ")\n";
    // #endif
    //         if (to != nullptr && !mySilent && !silent) {
    //             this->myErrorMsgHandler->informf(TL("No connection between edge '%' and edge '%' found."), from->getID(), to->getID());
    //         }
    // #ifdef DijkstraRouter_DEBUG_QUERY_VISITED
    //         DijkstraRouter_DEBUG_QUERY_VISITED_OUT << " </interval>\n</edgeData>\n";
    // #endif
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn compute(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool {
        // check whether from and to can be used
        if self.is_prohibited(from, vehicle, steps2time(msTime)) {
            if !silent {
                self.myErrorMsgHandler.m().inform(&format!("Vehicle '{}' is not allowed on source edge '{}'.", vehicle_id_secure(vehicle), from.id()), true);
            }
            return false;
        }
        // technically, a temporary permission might be lifted by the time of arrival
        if !to.is_null() && self.is_prohibited(to, vehicle, steps2time(msTime)) {
            if !silent {
                self.myErrorMsgHandler.m().inform(&format!("Vehicle '{}' is not allowed on destination edge '{}'.", vehicle_id_secure(vehicle), to.id()), true);
            }
            return false;
        }
        let mut length = 0.;
        self.start_query();
        let vClass = if vehicle.is_null() { SVC_IGNORING } else { vehicle.v_class() };
        let ignoreTransient = if vehicle.is_null() { false } else { vehicle.ignore_transient_permissions() };
        let query = (from, vehicle, msTime);
        let cmp = DijkstraRouter_EdgeInfoByEffortComparator {};
        if ((self.myBulkMode && from == self.myLastQuery.0) || (self.myAutoBulkMode && query == self.myLastQuery)) && !self.myAmClean {
            let toInfo = Ptr::from_mut(&mut self.myEdgeInfos[to.numerical_id() as usize]);
            if toInfo.visited {
                self.build_path_from(toInfo, into);
                self.end_query(1);
                return true;
            }
        } else {
            self.init(from.numerical_id(), msTime);
            if !self.myExternalEffort.is_null() {
                self.myExternalEffort.m().set_initial_state(from.numerical_id());
            }
            self.myAmClean = false;
        }
        self.myLastQuery = query;
        // loop
        let mut num_visited: i32 = 0;
        while !self.myFrontierList.is_empty() {
            num_visited += 1;
            // use the node with the minimal length
            let minimumInfo = self.myFrontierList[0];
            let minEdge = minimumInfo.edge.cast::<E>();
            minimumInfo.m().visited = true;
            // check whether the destination node was already reached
            if minEdge == to {
                //propagate last external effort state to destination edge
                if !self.myExternalEffort.is_null() {
                    self.myExternalEffort.m().update(minEdge.numerical_id(), minimumInfo.prev.edge.cast::<E>().numerical_id(), minEdge.length());
                }
                self.build_path_from(minimumInfo, into);
                self.end_query(num_visited);
                return true;
            }
            std_pop_heap(&mut self.myFrontierList, &|a, b| cmp.call_op::<E, V>(a, b));
            self.myFrontierList.pop();
            self.myFound.push(minimumInfo);
            let effortDelta = self.get_effort(minEdge, vehicle, minimumInfo.leaveTime);
            let leaveTime = minimumInfo.leaveTime + self.get_travel_time(minEdge, vehicle, minimumInfo.leaveTime, effortDelta);
            if !self.myExternalEffort.is_null() {
                self.myExternalEffort.m().update(minEdge.numerical_id(), minimumInfo.prev.edge.cast::<E>().numerical_id(), minEdge.length());
            }
            // check all ways from the node with the minimal length
            for follower in minEdge.via_successors(vClass, ignoreTransient) {
                let followerInfo = Ptr::from_mut(&mut self.myEdgeInfos[follower.0.numerical_id() as usize]);
                // check whether it can be used
                if self.is_prohibited(follower.0, vehicle, leaveTime) {
                    continue;
                }
                let mut effort = minimumInfo.effort + effortDelta;
                let mut time = leaveTime;
                self.update_via_edge_cost(follower.1, vehicle, &mut time, &mut effort, &mut length);
                let oldEffort = followerInfo.effort;
                if !followerInfo.visited && effort < oldEffort {
                    followerInfo.m().effort = effort;
                    followerInfo.m().leaveTime = time;
                    followerInfo.m().prev = minimumInfo;
                    if oldEffort == f64::MAX {
                        self.myFrontierList.push(followerInfo);
                        std_push_heap(&mut self.myFrontierList, &|a, b| cmp.call_op::<E, V>(a, b));
                    } else {
                        let fi = self.myFrontierList.iter().position(|p| *p == followerInfo).unwrap();
                        std_push_heap(&mut self.myFrontierList[..=fi], &|a, b| cmp.call_op::<E, V>(a, b));
                    }
                }
            }
        }
        self.end_query(num_visited);
        if !to.is_null() && !self.mySilent && !silent {
            let (fromId, toId) = (from.id().to_string(), to.id().to_string());
            let args: [&dyn ToStringPrec; 2] = [&fromId, &toId];
            self.myErrorMsgHandler.m().informf("No connection between edge '%' and edge '%' found.", &args);
        }
        false
    }

}

impl<E, V> DijkstraRouter<E, V> {
    /// C++: `void DijkstraRouter::DijkstraRouter<E, V>(const std::vector<typename SUMOAbstractRouter<E, V>::EdgeInfo> &, bool, typename SUMOAbstractRouter<E, V>::Operation, typename SUMOAbstractRouter<E, V>::Operation, bool, EffortCalculator *, const bool, const bool)` [private]
    // DijkstraRouter(const std::vector<typename SUMOAbstractRouter<E, V>::EdgeInfo>& edgeInfos, bool unbuildIsWarning,
    //                    typename SUMOAbstractRouter<E, V>::Operation effortOperation, typename SUMOAbstractRouter<E, V>::Operation ttOperation, bool silent, EffortCalculator* calc,
    //                    const bool havePermissions, const bool haveRestrictions) :
    //         SUMOAbstractRouter<E, V>("DijkstraRouter", unbuildIsWarning, effortOperation, ttOperation, havePermissions, haveRestrictions),
    //         mySilent(silent),
    //         myExternalEffort(calc) {
    //         for (const auto& edgeInfo : edgeInfos) {
    //             this->myEdgeInfos.push_back(typename SUMOAbstractRouter<E, V>::EdgeInfo(edgeInfo.edge));
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new_2(edgeInfos: &Vec<SUMOAbstractRouter_EdgeInfo>, unbuildIsWarning: bool, effortOperation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, ttOperation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, silent: bool, calc: Ptr<EffortCalculator>, havePermissions: bool, haveRestrictions: bool) -> Self {
        let mut base = SUMOAbstractRouter::new("DijkstraRouter", unbuildIsWarning, effortOperation, ttOperation, havePermissions, haveRestrictions);
        for edgeInfo in edgeInfos.iter() {
            base.myEdgeInfos.push(SUMOAbstractRouter_EdgeInfo::new(edgeInfo.edge));
        }
        Self {
            base,
            mySilent: silent,
            myLastQuery: Default::default(),
            myExternalEffort: calc,
            myComparator: DijkstraRouter_EdgeInfoByEffortComparator {},
            _p: std::marker::PhantomData,
        }
    }

}

/// Class to compare (and so sort) nodes by their effort
/// C++ class `DijkstraRouter::EdgeInfoByEffortComparator` (utils/router/DijkstraRouter.h)
pub struct DijkstraRouter_EdgeInfoByEffortComparator {
}

impl DijkstraRouter_EdgeInfoByEffortComparator {
    /// C++: `bool EdgeInfoByEffortComparator::operator()(const typename SUMOAbstractRouter<E, V>::EdgeInfo *, const typename SUMOAbstractRouter<E, V>::EdgeInfo *) const` [const]
    /// Comparing method
    // bool operator()(const typename SUMOAbstractRouter<E, V>::EdgeInfo* nod1, const typename SUMOAbstractRouter<E, V>::EdgeInfo* nod2) const {
    //             if (nod1->effort == nod2->effort) {
    //                 return nod1->edge->getNumericalID() > nod2->edge->getNumericalID();
    //             }
    //             return nod1->effort > nod2->effort;
    //         }
    #[allow(unused_variables)]
    pub fn call_op<E: RouterEdge<V>, V>(&self, nod1: Ptr<SUMOAbstractRouter_EdgeInfo>, nod2: Ptr<SUMOAbstractRouter_EdgeInfo>) -> bool {
        if nod1.effort == nod2.effort {
            return edge_numerical_id::<E, V>(nod1.edge) > edge_numerical_id::<E, V>(nod2.edge);
        }
        nod1.effort > nod2.effort
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::router::router_edge::RouterEdgeInfo;
    use utils::common::sumo_vehicle_class::{SVC_ALL, SVC_PASSENGER};
    use utils::common::sumo_vehicle_class::SVCPermissions;

    /// Test edge: a diamond in the edge graph, in -> {top, bot} -> out
    struct TEdge {
        num: i32,
        name: String,
        length: f64,
        succ: Vec<Ptr<TEdge>>,
    }

    struct TVeh;

    impl RouterVehicle for TVeh {
        fn v_class(&self) -> SUMOVehicleClass { SVC_PASSENGER }
        fn get_id(&self) -> String { "v".to_string() }
        fn get_max_speed(&self) -> f64 { 10. }
        fn get_chosen_speed_factor(&self) -> f64 { 1. }
        fn ignore_transient_permissions(&self) -> bool { false }
        fn length(&self) -> f64 { 5. }
    }

    impl RouterEdgeInfo for TEdge {
        fn edge_numerical_id(&self) -> i32 { self.num }
        fn edge_id(&self) -> String { self.name.clone() }
        fn edge_permissions(&self) -> SVCPermissions { SVC_ALL }
    }

    impl RouterEdge<TVeh> for TEdge {
        type Node = ();
        type Lane = ();
        fn id(&self) -> &str { &self.name }
        fn numerical_id(&self) -> i32 { self.num }
        fn length(&self) -> f64 { self.length }
        fn length_geometry_factor(&self) -> f64 { 1. }
        fn distance_to(&self, _other: Ptr<Self>, _do_boundary_estimate: bool) -> f64 { 0. }
        fn successors(&self, _vclass: SUMOVehicleClass) -> &[Ptr<Self>] { &self.succ }
        fn via_successors(&self, _vclass: SUMOVehicleClass, _ignore_transient: bool) -> &[(Ptr<Self>, Ptr<Self>)] { &[] }
        fn speed_limit(&self) -> f64 { 10. }
        fn minimum_travel_time(&self, _veh: Ptr<TVeh>) -> f64 { self.length / 10. }
        fn bidi_edge(&self) -> Ptr<Self> { Ptr::null() }
        fn travel_time_static(edge: Ptr<Self>, _veh: Ptr<TVeh>, _time: f64) -> f64 { edge.length }
        fn travel_time_aggregated(edge: Ptr<Self>, _veh: Ptr<TVeh>, _time: f64) -> f64 { edge.length }
        fn is_internal(&self) -> bool { false }
        fn prohibits(&self, _veh: Ptr<TVeh>, _check_restrictions: bool) -> bool { false }
        fn restricts(&self, _veh: Ptr<TVeh>) -> bool { false }
    }

    /// effort = edge length, so the shortest route is the one with the least total length
    fn test_effort(e: Ptr<TEdge>, _v: Ptr<TVeh>, _t: f64) -> f64 { e.length }

    fn new_edge(num: i32, name: &str, length: f64) -> Ptr<TEdge> {
        Ptr::from_raw(Box::into_raw(Box::new(TEdge { num, name: name.to_string(), length, succ: Vec::new() })))
    }

    fn names(route: &[Ptr<TEdge>]) -> Vec<String> {
        route.iter().map(|e| e.name.clone()).collect()
    }

    #[test]
    fn detours_around_a_prohibited_edge_through_the_base_router() {
        let inE = new_edge(0, "in", 1.);
        let top = new_edge(1, "top", 10.);
        let bot = new_edge(2, "bot", 20.);
        let outE = new_edge(3, "out", 1.);
        inE.m().succ = vec![top, bot];
        top.m().succ = vec![outE];
        bot.m().succ = vec![outE];
        let edges = vec![inE, top, bot, outE];
        let mut veh = TVeh;
        let vehPtr = Ptr::from_mut(&mut veh);

        let mut router: DijkstraRouter<TEdge, TVeh> = DijkstraRouter::new(&edges, true, Some(test_effort), None, false, Ptr::null(), false, false).with_compute();

        let mut plain: Vec<Ptr<TEdge>> = Vec::new();
        assert!(router.base.compute(inE, outE, vehPtr, 0, &mut plain, true));
        assert_eq!(names(&plain), vec!["in", "top", "out"]);

        let mut prohibitions: SUMOAbstractRouter_Prohibitions = Default::default();
        prohibitions.insert(top.cast::<()>(), RouterProhibition { begin: 0., end: f64::MAX, permissions: 0 });
        router.base.prohibit(&prohibitions);

        let mut detour: Vec<Ptr<TEdge>> = Vec::new();
        assert!(router.base.compute(inE, outE, vehPtr, 0, &mut detour, true));
        assert_eq!(names(&detour), vec!["in", "bot", "out"]);
    }
}
