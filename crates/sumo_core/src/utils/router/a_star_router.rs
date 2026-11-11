//! Port of utils/router/AStarRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `FullLookupTable<E, V> AStarRouter::FLT`
pub type AStarRouter_FLT = Unmapped /* FullLookupTable<E, V> */;

/// C++ typedef `LandmarkLookupTable<E, V, M> AStarRouter::LMLT`
pub type AStarRouter_LMLT = Unmapped /* LandmarkLookupTable<E, V, M> */;

/// C++ typedef `AbstractLookupTable<E, V> AStarRouter::LookupTable`
pub type AStarRouter_LookupTable = Unmapped /* AbstractLookupTable<E, V> */;

/// Computes the shortest path through a network using the A* algorithm.
/// C++ class `AStarRouter` (utils/router/AStarRouter.h)
pub struct AStarRouter<E, V, M> {
    /// C++ base class `SUMOAbstractRouter<E, V>`
    pub base: SUMOAbstractRouter<E, V>,
    pub myComparator: AStarRouter_EdgeInfoComparator,
    /// the lookup table for travel time heuristics
    pub myLookupTable: Rc<AStarRouter_LookupTable>,
    /// maximum speed in the network
    pub myMaxSpeed: f64,
    _p: std::marker::PhantomData<(E, V, M,)>,
}

impl<E, V, M> std::ops::Deref for AStarRouter<E, V, M> {
    type Target = SUMOAbstractRouter<E, V>;
    fn deref(&self) -> &SUMOAbstractRouter<E, V> {
        &self.base
    }
}
impl<E, V, M> std::ops::DerefMut for AStarRouter<E, V, M> {
    fn deref_mut(&mut self) -> &mut SUMOAbstractRouter<E, V> {
        &mut self.base
    }
}

impl<E, V, M> AStarRouter<E, V, M> {
    /// C++: `void AStarRouter::AStarRouter<E, V, M>(const std::vector<E *> &, bool, typename SUMOAbstractRouter<E, V>::Operation, const std::shared_ptr<const LookupTable>, const bool, const bool)`
    /// Constructor
    // AStarRouter(const std::vector<E*>& edges, bool unbuildIsWarning, typename SUMOAbstractRouter<E, V>::Operation operation, const std::shared_ptr<const LookupTable> lookup = nullptr,
    //                 const bool havePermissions = false, const bool haveRestrictions = false) :
    //         SUMOAbstractRouter<E, V>("AStarRouter", unbuildIsWarning, operation, nullptr, havePermissions, haveRestrictions),
    //         myLookupTable(lookup),
    //         myMaxSpeed(NUMERICAL_EPS) {
    //         for (const E* const edge : edges) {
    //             this->myEdgeInfos.push_back(typename SUMOAbstractRouter<E, V>::EdgeInfo(edge));
    //             myMaxSpeed = MAX2(myMaxSpeed, edge->getSpeedLimit() * MAX2(1.0, edge->getLengthGeometryFactor()));
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new(edges: &Vec<Ptr<E>>, unbuildIsWarning: bool, operation: SUMOAbstractRouter_Operation, lookup: Rc<AStarRouter_LookupTable>, havePermissions: bool, haveRestrictions: bool) -> Self {
        todo!("port AStarRouter::AStarRouter<E, V, M>")
    }

    /// C++: `void AStarRouter::AStarRouter<E, V, M>(const std::vector<typename SUMOAbstractRouter<E, V>::EdgeInfo> &, bool, typename SUMOAbstractRouter<E, V>::Operation, const std::shared_ptr<const LookupTable>, const bool, const bool)`
    // AStarRouter(const std::vector<typename SUMOAbstractRouter<E, V>::EdgeInfo>& edgeInfos, bool unbuildIsWarning, typename SUMOAbstractRouter<E, V>::Operation operation, const std::shared_ptr<const LookupTable> lookup = nullptr,
    //                 const bool havePermissions = false, const bool haveRestrictions = false) :
    //         SUMOAbstractRouter<E, V>("AStarRouter", unbuildIsWarning, operation, nullptr, havePermissions, haveRestrictions),
    //         myLookupTable(lookup),
    //         myMaxSpeed(NUMERICAL_EPS) {
    //         for (const auto& edgeInfo : edgeInfos) {
    //             this->myEdgeInfos.push_back(typename SUMOAbstractRouter<E, V>::EdgeInfo(edgeInfo.edge));
    //             myMaxSpeed = MAX2(myMaxSpeed, edgeInfo.edge->getSpeedLimit() * edgeInfo.edge->getLengthGeometryFactor());
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new_2(edgeInfos: &Vec<SUMOAbstractRouter_EdgeInfo>, unbuildIsWarning: bool, operation: SUMOAbstractRouter_Operation, lookup: Rc<AStarRouter_LookupTable>, havePermissions: bool, haveRestrictions: bool) -> Self {
        todo!("port AStarRouter::AStarRouter<E, V, M>")
    }

    /// C++: `void AStarRouter::~AStarRouter<E, V, M>()` [virtual]
    /// Destructor
    // virtual ~AStarRouter() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port AStarRouter::~AStarRouter<E, V, M>")
    }

    /// C++: `SUMOAbstractRouter<E, V> * AStarRouter::clone()` [virtual]
    // virtual SUMOAbstractRouter<E, V>* clone() {
    //         return new AStarRouter<E, V, M>(this->myEdgeInfos, this->myErrorMsgHandler == MsgHandler::getWarningInstance(), this->myOperation, myLookupTable,
    //                                         this->myHavePermissions, this->myHaveRestrictions);
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, V>> {
        todo!("port AStarRouter::clone")
    }

    /// C++: `bool AStarRouter::compute(const E *, const E *, const V *const, SUMOTime, std::vector<const E *> &, bool)`
    /// Builds the route between the given edges using the minimum travel time
    // bool compute(const E* from, const E* to, const V* const vehicle,
    //                  SUMOTime msTime, std::vector<const E*>& into, bool silent = false) {
    //         assert(from != nullptr && to != nullptr);
    //         // check whether from and to can be used
    //         if (this->isProhibited(from, vehicle, STEPS2TIME(msTime))) {
    //             if (!silent) {
    //                 this->myErrorMsgHandler->inform("Vehicle '" + Named::getIDSecure(vehicle) + "' is not allowed on source edge '" + from->getID() + "'.");
    //             }
    //             return false;
    //         }
    //         // technically, a temporary permission might be lifted by the time of arrival
    //         if (this->isProhibited(to, vehicle, STEPS2TIME(msTime))) {
    //             if (!silent) {
    //                 this->myErrorMsgHandler->inform("Vehicle '" + Named::getIDSecure(vehicle) + "' is not allowed on destination edge '" + to->getID() + "'.");
    //             }
    //             return false;
    //         }
    //         double length = 0.; // dummy for the via edge cost update
    //         this->startQuery();
    // #ifdef ASTAR_DEBUG_QUERY
    //         if (ASTAR_DEBUG_COND) {
    //             std::cout << "DEBUG: starting search for '" << Named::getIDSecure(vehicle) << "' speed: " << MIN2(vehicle->getMaxSpeed(), myMaxSpeed * vehicle->getChosenSpeedFactor()) << " time: " << STEPS2TIME(msTime) << "\n";
    //         }
    // #endif
    //
    //         const SUMOVehicleClass vClass = vehicle == 0 ? SVC_IGNORING : vehicle->getVClass();
    //         if (this->myBulkMode && !this->myAmClean) {
    //             const auto& toInfo = this->myEdgeInfos[to->getNumericalID()];
    //             if (toInfo.visited) {
    //                 this->buildPathFrom(&toInfo, into);
    //                 this->endQuery(1);
    //                 return true;
    //             }
    //         } else {
    //             this->init(from->getNumericalID(), msTime);
    //             this->myAmClean = false;
    //         }
    //         // loop
    //         int num_visited = 0;
    //         const bool mayRevisit = myLookupTable != nullptr && !myLookupTable->consistent();
    //         const double speed = vehicle == nullptr ? myMaxSpeed : MIN2(vehicle->getMaxSpeed(), myMaxSpeed * vehicle->getChosenSpeedFactor());
    //         while (!this->myFrontierList.empty()) {
    //             num_visited += 1;
    //             // use the node with the minimal length
    //             auto* const minimumInfo = this->myFrontierList.front();
    //             const E* const minEdge = minimumInfo->edge;
    // #ifdef ASTAR_DEBUG_QUERY
    //             if (ASTAR_DEBUG_COND) {
    //                 std::cout << "DEBUG: hit=" << minEdge->getID()
    //                           << " TT=" << minimumInfo->effort
    //                           << " EF=" << this->getEffort(minEdge, vehicle, minimumInfo->leaveTime)
    //                           << " HT=" << minimumInfo->heuristicEffort
    //                           << " Q(TT,HT,Edge)=";
    //                 for (const auto& edgeInfo : this->myFrontierList) {
    //                     std::cout << "\n   " << edgeInfo->effort << "," << edgeInfo->heuristicEffort << "," << edgeInfo->edge->getID();
    //                 }
    //                 std::cout << "\n";
    //             }
    // #endif
    //             // check whether the destination node was already reached
    //             if (minEdge == to) {
    //                 this->buildPathFrom(minimumInfo, into);
    //                 this->endQuery(num_visited);
    // #ifdef ASTAR_DEBUG_QUERY_PERF
    //                 if (ASTAR_DEBUG_COND) {
    //                     std::cout << "visited " + toString(num_visited) + " edges (final path length=" + toString(into.size())
    //                               + " time=" + toString(this->recomputeCosts(into, vehicle, msTime))
    //                               + " edges=" + toString(into) + ")\n";
    //                 }
    // #endif
    // #ifdef ASTAR_DEBUG_VISITED
    //                 if (ASTAR_DEBUG_COND) {
    //                     OutputDevice& dev = OutputDevice::getDevice(StringUtils::replace(Named::getIDSecure(vehicle), ":", "_") + "_" + toString(STEPS2TIME(msTime)));
    //                     for (const auto& i : this->myEdgeInfos) {
    //                         if (i.visited) {
    //                             dev << "edge:" << i.edge->getID() << "\n";
    //                         }
    //                     }
    //                     dev.close();
    //                 }
    // #endif
    //                 return true;
    //             }
    //             std::pop_heap(this->myFrontierList.begin(), this->myFrontierList.end(), myComparator);
    //             this->myFrontierList.pop_back();
    //             this->myFound.push_back(minimumInfo);
    //             minimumInfo->visited = true;
    //             const double effortDelta = this->getEffort(minEdge, vehicle, minimumInfo->leaveTime);
    //             const double leaveTime = minimumInfo->leaveTime + this->getTravelTime(minEdge, vehicle, minimumInfo->leaveTime, effortDelta);
    //
    //             // admissible A* heuristic: straight line distance at maximum speed
    //             // this is calculated from the end of minEdge so it possibly includes via efforts to the followers
    //             const double heuristic_remaining = (myLookupTable == nullptr ? minEdge->getDistanceTo(to) / speed :
    //                                                 myLookupTable->lowerBound(minEdge, to, speed, vehicle->getChosenSpeedFactor(),
    //                                                         minEdge->getMinimumTravelTime(nullptr), to->getMinimumTravelTime(nullptr)));
    //             if (heuristic_remaining == UNREACHABLE) {
    //                 continue;
    //             }
    //             const double heuristicEffort = minimumInfo->effort + effortDelta + heuristic_remaining;
    //             // check all ways from the node with the minimal length
    //             for (const std::pair<const E*, const E*>& follower : minEdge->getViaSuccessors(vClass)) {
    //                 auto& followerInfo = this->myEdgeInfos[follower.first->getNumericalID()];
    //                 // check whether it can be used
    //                 if (this->isProhibited(follower.first, vehicle, leaveTime)) {
    //                     continue;
    //                 }
    //                 double effort = minimumInfo->effort + effortDelta;
    //                 double time = leaveTime;
    //                 this->updateViaEdgeCost(follower.second, vehicle, time, effort, length);
    //                 const double oldEffort = followerInfo.effort;
    //                 if ((!followerInfo.visited || mayRevisit) && effort < oldEffort) {
    //                     followerInfo.effort = effort;
    //                     // if we use the effort including the via effort below we would count the via twice as shown by the ticket676 test
    //                     // but we should never get below the real effort, see #12463
    //                     followerInfo.heuristicEffort = MAX2(MIN2(heuristicEffort, followerInfo.heuristicEffort), effort);
    //                     followerInfo.leaveTime = time;
    //                     followerInfo.prev = minimumInfo;
    // #ifdef ASTAR_DEBUG_QUERY_FOLLOWERS
    //                     if (ASTAR_DEBUG_COND) {
    //                         std::cout << "   follower=" << followerInfo.edge->getID()
    //                                   << " OEF=" << (oldEffort == std::numeric_limits<double>::max() ? "inf" : toString(oldEffort))
    //                                   << " TT=" << effort << " HR=" << heuristic_remaining << " HT=" << followerInfo.heuristicEffort << "\n";
    //                     }
    // #endif
    //                     if (oldEffort == std::numeric_limits<double>::max()) {
    //                         this->myFrontierList.push_back(&followerInfo);
    //                         std::push_heap(this->myFrontierList.begin(), this->myFrontierList.end(), myComparator);
    //                     } else {
    //                         auto fi = std::find(this->myFrontierList.begin(), this->myFrontierList.end(), &followerInfo);
    //                         if (fi == this->myFrontierList.end()) {
    //                             assert(mayRevisit);
    //                             this->myFrontierList.push_back(&followerInfo);
    //                             std::push_heap(this->myFrontierList.begin(), this->myFrontierList.end(), myComparator);
    //                         } else {
    //                             std::push_heap(this->myFrontierList.begin(), fi + 1, myComparator);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         this->endQuery(num_visited);
    // #ifdef ASTAR_DEBUG_QUERY_PERF
    //         if (ASTAR_DEBUG_COND) {
    //             std::cout << "visited " + toString(num_visited) + " edges (unsuccessful path length: " + toString(into.size()) + ")\n";
    //         }
    // #endif
    //         if (!silent) {
    //             this->myErrorMsgHandler->informf(TL("No connection between edge '%' and edge '%' found."), from->getID(), to->getID());
    //         }
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn compute(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool {
        todo!("port AStarRouter::compute")
    }

}

/// Class to compare (and so sort) nodes by their effort
/// C++ class `AStarRouter::EdgeInfoComparator` (utils/router/AStarRouter.h)
pub struct AStarRouter_EdgeInfoComparator {
}

impl AStarRouter_EdgeInfoComparator {
    /// C++: `bool EdgeInfoComparator::operator()(const typename SUMOAbstractRouter<E, V>::EdgeInfo *, const typename SUMOAbstractRouter<E, V>::EdgeInfo *) const` [const]
    /// Comparing method
    // bool operator()(const typename SUMOAbstractRouter<E, V>::EdgeInfo* nod1, const typename SUMOAbstractRouter<E, V>::EdgeInfo* nod2) const {
    //             if (nod1->heuristicEffort == nod2->heuristicEffort) {
    //                 return nod1->edge->getNumericalID() > nod2->edge->getNumericalID();
    //             }
    //             return nod1->heuristicEffort > nod2->heuristicEffort;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, nod1: Ptr<SUMOAbstractRouter_EdgeInfo>, nod2: Ptr<SUMOAbstractRouter_EdgeInfo>) -> bool {
        todo!("port EdgeInfoComparator::operator()")
    }

}

