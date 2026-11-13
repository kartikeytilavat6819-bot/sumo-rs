//! Port of utils/router/AStarRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::a_star_lookup_table::{AStarLookupTableKind, UNREACHABLE};
use crate::utils::router::ch_builder::{std_pop_heap, std_push_heap};
use crate::utils::router::router_edge::{RouterEdge, RouterVehicle};

fn msg_instance(warning: bool) -> Ptr<MsgHandler> {
    utils::common::msg_handler::with_instance(if warning { utils::common::msg_handler::MsgType::Warning } else { utils::common::msg_handler::MsgType::Error }, |h| Ptr::from_mut(h))
}

/// C++ `MIN2` / `MAX2` macros
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// C++ `Named::getIDSecure(vehicle)`
fn vehicle_id_secure<V: RouterVehicle>(vehicle: Ptr<V>) -> String {
    if vehicle.is_null() { "NULL".to_string() } else { vehicle.get_id() }
}

/// the edge of an EdgeInfo is always an `E` (SUMOAbstractRouter keeps it as an untyped pointer)
fn edge_numerical_id<E: RouterEdge<V>, V>(edge: Ptr<()>) -> i32 {
    edge.cast::<E>().numerical_id()
}

/// C++ typedef `FullLookupTable<E, V> AStarRouter::FLT`
pub type AStarRouter_FLT = Unmapped /* FullLookupTable<E, V> */;

/// C++ typedef `LandmarkLookupTable<E, V, M> AStarRouter::LMLT`
pub type AStarRouter_LMLT = Unmapped /* LandmarkLookupTable<E, V, M> */;

/// C++ typedef `AbstractLookupTable<E, V> AStarRouter::LookupTable`
pub type AStarRouter_LookupTable = Unmapped /* AbstractLookupTable<E, V> */;

/// Computes the shortest path through a network using the A* algorithm.
/// C++ class `AStarRouter` (utils/router/AStarRouter.h)
#[repr(C)]
pub struct AStarRouter<E, V, M> {
    /// C++ base class `SUMOAbstractRouter<E, V>`
    pub base: SUMOAbstractRouter<E, V>,
    pub myComparator: AStarRouter_EdgeInfoComparator,
    /// the lookup table for travel time heuristics
    pub myLookupTable: Option<Rc<AStarLookupTableKind<E, V, M>>>,
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

impl<E: RouterEdge<V>, V: RouterVehicle, M> AStarRouter<E, V, M> {
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
    pub fn new(edges: &Vec<Ptr<E>>, unbuildIsWarning: bool, operation: SUMOAbstractRouter_Operation, lookup: Option<Rc<AStarLookupTableKind<E, V, M>>>, havePermissions: bool, haveRestrictions: bool) -> Self {
        let mut base = SUMOAbstractRouter::new("AStarRouter", unbuildIsWarning, operation, (), havePermissions, haveRestrictions);
        let mut myMaxSpeed = utils::geom::NUMERICAL_EPS;
        for edge in edges.iter() {
            base.myEdgeInfos.push(SUMOAbstractRouter_EdgeInfo::new(edge.cast::<()>()));
            myMaxSpeed = max2(myMaxSpeed, edge.speed_limit() * max2(1.0, edge.length_geometry_factor()));
        }
        Self {
            base,
            myComparator: AStarRouter_EdgeInfoComparator {},
            myLookupTable: lookup,
            myMaxSpeed,
            _p: std::marker::PhantomData,
        }
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
    pub fn new_2(edgeInfos: &Vec<SUMOAbstractRouter_EdgeInfo>, unbuildIsWarning: bool, operation: SUMOAbstractRouter_Operation, lookup: Option<Rc<AStarLookupTableKind<E, V, M>>>, havePermissions: bool, haveRestrictions: bool) -> Self {
        let mut base = SUMOAbstractRouter::new("AStarRouter", unbuildIsWarning, operation, (), havePermissions, haveRestrictions);
        let mut myMaxSpeed = utils::geom::NUMERICAL_EPS;
        for edgeInfo in edgeInfos.iter() {
            base.myEdgeInfos.push(SUMOAbstractRouter_EdgeInfo::new(edgeInfo.edge));
            let edge = edgeInfo.edge.cast::<E>();
            myMaxSpeed = max2(myMaxSpeed, edge.speed_limit() * edge.length_geometry_factor());
        }
        Self {
            base,
            myComparator: AStarRouter_EdgeInfoComparator {},
            myLookupTable: lookup,
            myMaxSpeed,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void AStarRouter::~AStarRouter<E, V, M>()` [virtual]
    /// Destructor
    // virtual ~AStarRouter() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `SUMOAbstractRouter<E, V> * AStarRouter::clone()` [virtual]
    // virtual SUMOAbstractRouter<E, V>* clone() {
    //         return new AStarRouter<E, V, M>(this->myEdgeInfos, this->myErrorMsgHandler == MsgHandler::getWarningInstance(), this->myOperation, myLookupTable,
    //                                         this->myHavePermissions, this->myHaveRestrictions);
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, V>> {
        let warning = self.base.myErrorMsgHandler == msg_instance(true);
        let clone: AStarRouter<E, V, M> = AStarRouter::new_2(&self.base.myEdgeInfos, warning, self.base.myOperation, self.myLookupTable.clone(), self.base.myHavePermissions, self.base.myHaveRestrictions);
        Ptr::from_raw(Box::into_raw(Box::new(clone))).cast::<SUMOAbstractRouter<E, V>>()
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
        // check whether from and to can be used
        if self.is_prohibited(from, vehicle, steps2time(msTime)) {
            if !silent {
                self.myErrorMsgHandler.m().inform(&format!("Vehicle '{}' is not allowed on source edge '{}'.", vehicle_id_secure(vehicle), from.id()), true);
            }
            return false;
        }
        // technically, a temporary permission might be lifted by the time of arrival
        if self.is_prohibited(to, vehicle, steps2time(msTime)) {
            if !silent {
                self.myErrorMsgHandler.m().inform(&format!("Vehicle '{}' is not allowed on destination edge '{}'.", vehicle_id_secure(vehicle), to.id()), true);
            }
            return false;
        }
        let mut length = 0.;
        self.start_query();
        let vClass = if vehicle.is_null() { SVC_IGNORING } else { vehicle.v_class() };
        if self.myBulkMode && !self.myAmClean {
            let toInfo = Ptr::from_mut(&mut self.myEdgeInfos[to.numerical_id() as usize]);
            if toInfo.visited {
                self.build_path_from(toInfo, into);
                self.end_query(1);
                return true;
            }
        } else {
            self.init(from.numerical_id(), msTime);
            self.myAmClean = false;
        }
        // loop
        let mut num_visited: i32 = 0;
        let mayRevisit = self.myLookupTable.as_ref().is_some_and(|lt| !lt.consistent());
        let speed = if vehicle.is_null() { self.myMaxSpeed } else { min2(vehicle.get_max_speed(), self.myMaxSpeed * vehicle.get_chosen_speed_factor()) };
        let cmp = AStarRouter_EdgeInfoComparator {};
        while !self.base.myFrontierList.is_empty() {
            num_visited += 1;
            // use the node with the minimal length
            let minimumInfo = self.base.myFrontierList[0];
            let minEdge = minimumInfo.edge.cast::<E>();
            // check whether the destination node was already reached
            if minEdge == to {
                self.build_path_from(minimumInfo, into);
                self.end_query(num_visited);
                return true;
            }
            std_pop_heap(&mut self.base.myFrontierList, &|a, b| cmp.call_op::<E, V>(a, b));
            self.base.myFrontierList.pop();
            self.base.myFound.push(minimumInfo);
            minimumInfo.m().visited = true;
            let effortDelta = self.get_effort(minEdge, vehicle, minimumInfo.leaveTime);
            let leaveTime = minimumInfo.leaveTime + self.get_travel_time(minEdge, vehicle, minimumInfo.leaveTime, effortDelta);

            // admissible A* heuristic: straight line distance at maximum speed
            // this is calculated from the end of minEdge so it possibly includes via efforts to the followers
            let heuristic_remaining = match &self.myLookupTable {
                None => minEdge.distance_to(to, false) / speed,
                Some(lt) => lt.lower_bound(minEdge, to, speed, vehicle.get_chosen_speed_factor(), minEdge.minimum_travel_time(Ptr::null()), to.minimum_travel_time(Ptr::null())),
            };
            if heuristic_remaining == UNREACHABLE {
                continue;
            }
            let heuristicEffort = minimumInfo.effort + effortDelta + heuristic_remaining;
            // check all ways from the node with the minimal length
            for follower in minEdge.via_successors(vClass, false) {
                let followerInfo = Ptr::from_mut(&mut self.myEdgeInfos[follower.0.numerical_id() as usize]);
                // check whether it can be used
                if self.is_prohibited(follower.0, vehicle, leaveTime) {
                    continue;
                }
                let mut effort = minimumInfo.effort + effortDelta;
                let mut time = leaveTime;
                self.update_via_edge_cost(follower.1, vehicle, &mut time, &mut effort, &mut length);
                let oldEffort = followerInfo.effort;
                if (!followerInfo.visited || mayRevisit) && effort < oldEffort {
                    followerInfo.m().effort = effort;
                    // if we use the effort including the via effort below we would count the via twice as shown by the ticket676 test
                    // but we should never get below the real effort, see #12463
                    followerInfo.m().heuristicEffort = max2(min2(heuristicEffort, followerInfo.heuristicEffort), effort);
                    followerInfo.m().leaveTime = time;
                    followerInfo.m().prev = minimumInfo;
                    if oldEffort == f64::MAX {
                        self.base.myFrontierList.push(followerInfo);
                        std_push_heap(&mut self.base.myFrontierList, &|a, b| cmp.call_op::<E, V>(a, b));
                    } else {
                        match self.base.myFrontierList.iter().position(|p| *p == followerInfo) {
                            None => {
                                self.base.myFrontierList.push(followerInfo);
                                std_push_heap(&mut self.base.myFrontierList, &|a, b| cmp.call_op::<E, V>(a, b));
                            }
                            Some(fi) => std_push_heap(&mut self.base.myFrontierList[..=fi], &|a, b| cmp.call_op::<E, V>(a, b)),
                        }
                    }
                }
            }
        }
        self.end_query(num_visited);
        if !silent {
            let (fromId, toId) = (from.id().to_string(), to.id().to_string());
            let args: [&dyn ToStringPrec; 2] = [&fromId, &toId];
            self.myErrorMsgHandler.m().informf("No connection between edge '%' and edge '%' found.", &args);
        }
        false
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
    pub fn call_op<E: RouterEdge<V>, V>(&self, nod1: Ptr<SUMOAbstractRouter_EdgeInfo>, nod2: Ptr<SUMOAbstractRouter_EdgeInfo>) -> bool {
        if nod1.heuristicEffort == nod2.heuristicEffort {
            return edge_numerical_id::<E, V>(nod1.edge) > edge_numerical_id::<E, V>(nod2.edge);
        }
        nod1.heuristicEffort > nod2.heuristicEffort
    }

}

