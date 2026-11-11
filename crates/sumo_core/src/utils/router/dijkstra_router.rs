//! Port of utils/router/DijkstraRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Computes the shortest path through a network using the Dijkstra algorithm.
/// C++ class `DijkstraRouter` (utils/router/DijkstraRouter.h)
pub struct DijkstraRouter<E, V> {
    /// C++ base class `SUMOAbstractRouter<E, V>`
    pub base: SUMOAbstractRouter<E, V>,
    /// whether to suppress warning/error if no route was found
    pub mySilent: bool,
    /// cache of the last query to enable automated bulk routing
    pub myLastQuery: Unmapped /* std::tuple<const E *, const V *, SUMOTime> */,
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
    pub fn new(edges: &Vec<Ptr<E>>, unbuildIsWarning: bool, effortOperation: SUMOAbstractRouter_Operation, ttOperation: SUMOAbstractRouter_Operation, silent: bool, calc: Ptr<EffortCalculator>, havePermissions: bool, haveRestrictions: bool) -> Self {
        todo!("port DijkstraRouter::DijkstraRouter<E, V>")
    }

    /// C++: `void DijkstraRouter::~DijkstraRouter<E, V>()` [virtual]
    /// Destructor
    // virtual ~DijkstraRouter() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port DijkstraRouter::~DijkstraRouter<E, V>")
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
        todo!("port DijkstraRouter::clone")
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
        todo!("port DijkstraRouter::compute")
    }

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
    pub fn new_2(edgeInfos: &Vec<SUMOAbstractRouter_EdgeInfo>, unbuildIsWarning: bool, effortOperation: SUMOAbstractRouter_Operation, ttOperation: SUMOAbstractRouter_Operation, silent: bool, calc: Ptr<EffortCalculator>, havePermissions: bool, haveRestrictions: bool) -> Self {
        todo!("port DijkstraRouter::DijkstraRouter<E, V>")
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
    pub fn call_op(&self, nod1: Ptr<SUMOAbstractRouter_EdgeInfo>, nod2: Ptr<SUMOAbstractRouter_EdgeInfo>) -> bool {
        todo!("port EdgeInfoByEffortComparator::operator()")
    }

}

