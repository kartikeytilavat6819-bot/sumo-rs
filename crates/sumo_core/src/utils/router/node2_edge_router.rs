//! Port of utils/router/Node2EdgeRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::a_star_lookup_table::AStarLookupTableKind;
use crate::utils::router::a_star_router::AStarRouter_EdgeInfoComparator;
use crate::utils::router::ch_builder::{std_pop_heap, std_push_heap};
use crate::utils::router::router_edge::{RouterEdge, RouterNode, RouterVehicle};

/// C++ `Named::getIDSecure(vehicle)`
fn vehicle_id_secure<V: RouterVehicle>(vehicle: Ptr<V>) -> String {
    if vehicle.is_null() { "NULL".to_string() } else { vehicle.get_id() }
}

/// C++ typedef `AbstractLookupTable<E, V> Node2EdgeRouter::LookupTable`
pub type Node2EdgeRouter_LookupTable = Unmapped /* AbstractLookupTable<E, V> */;

/// Computes shortest paths from a node to one edge (using A*) or to all edges (using Dijkstra's algorithm)
/// C++ class `Node2EdgeRouter` (utils/router/Node2EdgeRouter.h)
#[repr(C)]
pub struct Node2EdgeRouter<E, N, V, M> {
    /// C++ base class `AStarRouter<E, V, M>`
    pub base: AStarRouter<E, V, M>,
    _p: std::marker::PhantomData<(E, N, V, M,)>,
}

impl<E, N, V, M> std::ops::Deref for Node2EdgeRouter<E, N, V, M> {
    type Target = AStarRouter<E, V, M>;
    fn deref(&self) -> &AStarRouter<E, V, M> {
        &self.base
    }
}
impl<E, N, V, M> std::ops::DerefMut for Node2EdgeRouter<E, N, V, M> {
    fn deref_mut(&mut self) -> &mut AStarRouter<E, V, M> {
        &mut self.base
    }
}

impl<E: RouterEdge<V>, N, V: RouterVehicle, M> Node2EdgeRouter<E, N, V, M> {
    /// C++: `typename SUMOAbstractRouter<E, V>::EdgeInfo * Node2EdgeRouter::edgeInfo(const E *const)`
    /// Returns the edge information for the passed edge
    // typename SUMOAbstractRouter<E, V>::EdgeInfo* edgeInfo(const E* const edge) {
    //         return &(this->myEdgeInfos[edge->getNumericalID()]);
    //     }
    #[allow(unused_variables)]
    pub fn edge_info(&mut self, edge: Ptr<E>) -> Ptr<SUMOAbstractRouter_EdgeInfo> {
        Ptr::from_mut(&mut self.myEdgeInfos[edge.numerical_id() as usize])
    }

    /// C++: `const typename SUMOAbstractRouter<E, V>::EdgeInfo * Node2EdgeRouter::edgeInfo(const E *const) const` [const]
    /// Returns the edge information for the passed edge
    // const typename SUMOAbstractRouter<E, V>::EdgeInfo* edgeInfo(const E* const edge) const {
    //         return &(this->myEdgeInfos[edge->getNumericalID()]);
    //     }
    #[allow(unused_variables)]
    pub fn edge_info_2(&self, edge: Ptr<E>) -> Ptr<SUMOAbstractRouter_EdgeInfo> {
        Ptr::new(&self.myEdgeInfos[edge.numerical_id() as usize])
    }

    /// C++: `void Node2EdgeRouter::Node2EdgeRouter<E, N, V, M>(const int &, bool, typename SUMOAbstractRouter<E, V>::Operation, const std::shared_ptr<const LookupTable>, const bool, const bool)`
    /// Constructor
    // Node2EdgeRouter(const std::vector<E*>& edges, bool unbuildIsWarning, typename SUMOAbstractRouter<E, V>::Operation operation,
    //                     const std::shared_ptr<const LookupTable> lookup = nullptr,
    //                     const bool havePermissions = false, const bool haveRestrictions = false) :
    //         AStarRouter<E, V, M>(edges, unbuildIsWarning, operation, lookup, havePermissions, haveRestrictions) {
    //     }
    #[allow(unused_variables)]
    pub fn new(edges: &Vec<Ptr<E>>, unbuildIsWarning: bool, operation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, lookup: Option<Rc<AStarLookupTableKind<E, V, M>>>, havePermissions: bool, haveRestrictions: bool) -> Self {
        Self {
            base: AStarRouter::new(edges, unbuildIsWarning, operation, lookup, havePermissions, haveRestrictions),
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void Node2EdgeRouter::Node2EdgeRouter<E, N, V, M>(const int &, bool, typename SUMOAbstractRouter<E, V>::Operation, const std::shared_ptr<const LookupTable>, const bool, const bool)`
    /// Cloning constructor
    // Node2EdgeRouter(const std::vector<E*>& edges, bool unbuildIsWarning, typename SUMOAbstractRouter<E, V>::Operation operation,
    //                     const std::shared_ptr<const LookupTable> lookup = nullptr,
    //                     const bool havePermissions = false, const bool haveRestrictions = false) :
    //         AStarRouter<E, V, M>(edges, unbuildIsWarning, operation, lookup, havePermissions, haveRestrictions) {
    //     }
    #[allow(unused_variables)]
    pub fn new_2(edgeInfos: &Vec<SUMOAbstractRouter_EdgeInfo>, unbuildIsWarning: bool, operation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, lookup: Option<Rc<AStarLookupTableKind<E, V, M>>>, havePermissions: bool, haveRestrictions: bool) -> Self {
        Self {
            base: AStarRouter::new_2(edgeInfos, unbuildIsWarning, operation, lookup, havePermissions, haveRestrictions),
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void Node2EdgeRouter::~Node2EdgeRouter<E, N, V, M>()` [virtual]
    /// Destructor
    // virtual ~Node2EdgeRouter() {
    //         WRITE_MESSAGE("The following stats for AStarRouter (which might also be empty) belong to Node2EdgeRouter, a derived class:");
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        utils::common::msg_handler::write_message("The following stats for AStarRouter (which might also be empty) belong to Node2EdgeRouter, a derived class:");
    }

    /// C++: `SUMOAbstractRouter<E, V> * Node2EdgeRouter::clone()` [virtual]
    /// Cloning method
    // virtual SUMOAbstractRouter<E, V>* clone() {
    //         return new Node2EdgeRouter<E, N, V, M>(this->myEdgeInfos, this->myErrorMsgHandler == MsgHandler::getWarningInstance(), this->myOperation, this->myLookupTable,
    //                                                this->myHavePermissions, this->myHaveRestrictions);
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, V>> {
        let unbuildIsWarning = self.myErrorMsgHandler == utils::common::msg_handler::with_instance(utils::common::msg_handler::MsgType::Warning, |h| Ptr::from_mut(h));
        let router: Node2EdgeRouter<E, N, V, M> = Node2EdgeRouter::new_2(&self.myEdgeInfos, unbuildIsWarning, self.myOperation, self.myLookupTable.clone(), self.myHavePermissions, self.myHaveRestrictions);
        Ptr::from_raw(Box::into_raw(Box::new(router))).cast()
    }

    /// C++: `void Node2EdgeRouter::reset(const V *const)` [virtual]
    /// Reset method
    // virtual void reset(const V* const vehicle) {
    //         UNUSED_PARAMETER(vehicle);
    //         for (auto& edgeInfo : this->myFrontierList) {
    //             edgeInfo->reset();
    //         }
    //         this->myFrontierList.clear();
    //         for (auto& edgeInfo : this->myFound) {
    //             edgeInfo->reset();
    //         }
    //         this->myFound.clear();
    //     }
    #[allow(unused_variables)]
    pub fn reset(&mut self, vehicle: Ptr<V>) {
        for edgeInfo in self.myFrontierList.iter() {
            edgeInfo.m().reset();
        }
        self.myFrontierList.clear();
        for edgeInfo in self.myFound.iter() {
            edgeInfo.m().reset();
        }
        self.myFound.clear();
    }

    /// C++: `bool Node2EdgeRouter::computeNode2Edges(const N *, const int &, const V *const, SUMOTime, bool)`
    /// Builds the routes between the given node and all given edges using the minimum travel time
    // bool computeNode2Edges(const N* fromNode, const std::unordered_set<const E*>& toEdges, const V* const vehicle,
    //                            SUMOTime msTime, bool silent = false) {
    //         assert(fromNode != nullptr);
    //         // check whether fromNode and to can be used
    //         bool found = false;
    //         bool allVisited = true; // we try to disprove this
    //         for (const E* from : fromNode->getOutgoing()) {
    //             if (from->isInternal()) {
    //                 continue;
    //             }
    //             if (this->isProhibited(from, vehicle, STEPS2TIME(msTime))) {
    //                 if (!silent) {
    //                     this->myErrorMsgHandler->inform("Vehicle '" + Named::getIDSecure(vehicle) + "' is not allowed on source edge '" + from->getID() + "'.");
    //                 }
    //             } else {
    //                 found = true;
    //                 break;
    //             }
    //         }
    //         if (!found) {
    //             return false;
    //         }
    //         const std::vector<const E*>& fromEdges = fromNode->getOutgoing();
    //
    //         if (fromEdges.empty() || toEdges.empty()) { // nothing to do here
    //             return false;
    //         }
    //
    //         double length = 0.; // dummy for the via edge cost update
    //         this->startQuery();
    // #ifdef ASTAR_DEBUG_QUERY
    //         if (ASTAR_DEBUG_COND) {
    //             std::cout << "DEBUG: starting search for '" << Named::getIDSecure(vehicle) << "' speed: " << MIN2(vehicle->getMaxSpeed(), myMaxSpeed * vehicle->getChosenSpeedFactor()) << " time: " << STEPS2TIME(msTime) << "\n";
    //         }
    // #endif
    //
    //         const SUMOVehicleClass vClass = vehicle == 0 ? SVC_IGNORING : vehicle->getVClass();
    //         init(fromEdges, vehicle, msTime);
    //         this->myAmClean = false;
    //         // loop
    //         int num_visited = 0;
    //         while (!this->myFrontierList.empty()) {
    //             num_visited += 1;
    //             // use the edge with the minimal length
    //             auto* const minimumInfo = this->myFrontierList.front();
    //             const E* const minEdge = minimumInfo->edge;
    //             // check whether the destination edge was already reached
    //             if (toEdges.count(minEdge)) {
    //                 for (const E* to : toEdges) {
    //                     const auto& toInfo = this->myEdgeInfos[to->getNumericalID()];
    //                     if (!toInfo.visited) {
    //                         allVisited = false;
    //                         break;
    //                     }
    //                 }
    //                 if (allVisited) {
    //                     this->endQuery(num_visited);
    //                     return true;
    //                 }
    //             }
    //             std::pop_heap(this->myFrontierList.begin(), this->myFrontierList.end(), this->myComparator);
    //             this->myFrontierList.pop_back();
    //             this->myFound.push_back(minimumInfo);
    //             minimumInfo->visited = true;
    //             const double effortDelta = this->getEffort(minEdge, vehicle, minimumInfo->leaveTime);
    //             const double leaveTime = minimumInfo->leaveTime + this->getTravelTime(minEdge, vehicle, minimumInfo->leaveTime, effortDelta);
    //
    //             const double heuristic_remaining = 0;
    //             if (heuristic_remaining == UNREACHABLE) {
    //                 continue;
    //             }
    //             const double heuristicEffort = minimumInfo->effort + effortDelta + heuristic_remaining;
    //             // check all ways from the edge with the minimal length
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
    //                 if ((!followerInfo.visited) && effort < oldEffort) {
    //                     followerInfo.effort = effort;
    //                     // if we use the effort including the via effort below we would count the via twice as shown by the ticket676 test
    //                     // but we should never get below the real effort, see #12463
    //                     followerInfo.heuristicEffort = MAX2(MIN2(heuristicEffort, followerInfo.heuristicEffort), effort);
    //                     followerInfo.leaveTime = time;
    //                     followerInfo.prev = minimumInfo;
    //                     if (oldEffort == std::numeric_limits<double>::max()) {
    //                         this->myFrontierList.push_back(&followerInfo);
    //                         std::push_heap(this->myFrontierList.begin(), this->myFrontierList.end(), this->myComparator);
    //                     } else {
    //                         auto fi = std::find(this->myFrontierList.begin(), this->myFrontierList.end(), &followerInfo);
    //                         assert(fi != this->myFrontierList.end());
    //                         std::push_heap(this->myFrontierList.begin(), fi + 1, this->myComparator);
    //                     }
    //                 }
    //             }
    //         }
    //         this->endQuery(num_visited);
    //         for (const E* to : toEdges) {
    //             const auto& toInfo = this->myEdgeInfos[to->getNumericalID()];
    //             if (toInfo.visited) {
    //                 if (!silent) {
    //                     this->myErrorMsgHandler->informf("Only some connections between node '%' and the given edges were found.", fromNode->getID());
    //                 }
    //                 return true; // at least one connection could be found
    //             }
    //         }
    //         if (!silent) {
    //             this->myErrorMsgHandler->informf("No connections between node '%' and the given edges were found.", fromNode->getID());
    //         }
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn compute_node2_edges(&mut self, fromNode: Ptr<N>, toEdges: &std::collections::HashSet<Ptr<E>>, vehicle: Ptr<V>, msTime: SUMOTime, silent: bool) -> bool
    where
        N: RouterNode<Edge = E>,
    {
        // check whether fromNode and to can be used
        let mut found = false;
        let mut allVisited = true; // we try to disprove this
        for from in fromNode.m().outgoing().iter().copied() {
            if from.is_internal() {
                continue;
            }
            if self.is_prohibited(from, vehicle, steps2time(msTime)) {
                if !silent {
                    self.myErrorMsgHandler.m().inform(&format!("Vehicle '{}' is not allowed on source edge '{}'.", vehicle_id_secure(vehicle), from.id()), true);
                }
            } else {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
        let fromEdges: Vec<Ptr<E>> = fromNode.m().outgoing().to_vec();

        if fromEdges.is_empty() || toEdges.is_empty() {
            // nothing to do here
            return false;
        }

        let mut length = 0.; // dummy for the via edge cost update
        self.start_query();

        let vClass = if vehicle.is_null() { SVC_IGNORING } else { vehicle.v_class() };
        self.init(fromEdges, vehicle, msTime);
        self.myAmClean = false;
        // loop
        let mut num_visited: i32 = 0;
        let cmp = AStarRouter_EdgeInfoComparator {};
        while !self.myFrontierList.is_empty() {
            num_visited += 1;
            // use the edge with the minimal length
            let minimumInfo = self.myFrontierList[0];
            let minEdge = minimumInfo.edge.cast::<E>();
            // check whether the destination edge was already reached
            // determinism: C++ iterates an unordered_set here, but only whether every target is visited matters
            if toEdges.contains(&minEdge) {
                for to in toEdges.iter() {
                    if !self.myEdgeInfos[to.numerical_id() as usize].visited {
                        allVisited = false;
                        break;
                    }
                }
                if allVisited {
                    self.end_query(num_visited);
                    return true;
                }
            }
            std_pop_heap(&mut self.myFrontierList, &|a, b| cmp.call_op::<E, V>(a, b));
            self.myFrontierList.pop();
            self.myFound.push(minimumInfo);
            minimumInfo.m().visited = true;
            let effortDelta = self.get_effort(minEdge, vehicle, minimumInfo.leaveTime);
            let leaveTime = minimumInfo.leaveTime + self.get_travel_time(minEdge, vehicle, minimumInfo.leaveTime, effortDelta);

            let heuristic_remaining = 0.;
            if heuristic_remaining == UNREACHABLE {
                continue;
            }
            let heuristicEffort = minimumInfo.effort + effortDelta + heuristic_remaining;
            // check all ways from the edge with the minimal length
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
                if !followerInfo.visited && effort < oldEffort {
                    followerInfo.m().effort = effort;
                    // if we use the effort including the via effort below we would count the via twice as shown by the ticket676 test
                    // but we should never get below the real effort, see #12463
                    followerInfo.m().heuristicEffort = max2(min2(heuristicEffort, followerInfo.heuristicEffort), effort);
                    followerInfo.m().leaveTime = time;
                    followerInfo.m().prev = minimumInfo;
                    if oldEffort == f64::MAX {
                        self.myFrontierList.push(followerInfo);
                        std_push_heap(&mut self.myFrontierList, &|a, b| cmp.call_op::<E, V>(a, b));
                    } else {
                        let fi = self.myFrontierList.iter().position(|p| *p == followerInfo).expect("follower with a finite effort is on the frontier");
                        std_push_heap(&mut self.myFrontierList[..=fi], &|a, b| cmp.call_op::<E, V>(a, b));
                    }
                }
            }
        }
        self.end_query(num_visited);
        for to in toEdges.iter() {
            if self.myEdgeInfos[to.numerical_id() as usize].visited {
                if !silent {
                    let fromId = fromNode.m().id().to_string();
                    let args: [&dyn ToStringPrec; 1] = [&fromId];
                    self.myErrorMsgHandler.m().informf("Only some connections between node '%' and the given edges were found.", &args);
                }
                return true; // at least one connection could be found
            }
        }
        if !silent {
            let fromId = fromNode.m().id().to_string();
            let args: [&dyn ToStringPrec; 1] = [&fromId];
            self.myErrorMsgHandler.m().informf("No connections between node '%' and the given edges were found.", &args);
        }
        false
    }

    /// C++: `bool Node2EdgeRouter::computeNode2Edge(const N *, const E *, const V *const, SUMOTime, int &, bool)`
    /// Builds the route between the given node and edge using the minimum travel time
    // bool computeNode2Edge(const N* fromNode, const E* to, const V* const vehicle,
    //                           SUMOTime msTime, std::vector<const E*>& into, bool silent = false) {
    //         assert(fromNode != nullptr && to != nullptr);
    //         // check whether fromNode and to can be used
    //         bool found = false;
    //         for (const E* from : fromNode->getOutgoing()) {
    //             if (from->isInternal()) {
    //                 continue;
    //             }
    //             if (this->myEdgeInfos[from->getNumericalID()].prohibited || this->isProhibited(from, vehicle)) {
    //                 if (!silent) {
    //                     this->myErrorMsgHandler->inform("Vehicle '" + Named::getIDSecure(vehicle) + "' is not allowed on source edge '" + from->getID() + "'.");
    //                 }
    //             } else {
    //                 found = true;
    //                 break;
    //             }
    //         }
    //         if (!found) {
    //             return false;
    //         }
    //         const std::vector<const E*>& fromEdges = fromNode->getOutgoing();
    //
    //         if (fromEdges.empty()) { // nothing to do here
    //             return false;
    //         }
    //         if (this->myEdgeInfos[to->getNumericalID()].prohibited || this->isProhibited(to, vehicle)) {
    //             if (!silent) {
    //                 this->myErrorMsgHandler->inform("Vehicle '" + Named::getIDSecure(vehicle) + "' is not allowed on destination edge '" + to->getID() + "'.");
    //             }
    //             return false;
    //         }
    //         double length = 0.; // dummy for the via edge cost update
    //         this->startQuery();
    //
    //         const SUMOVehicleClass vClass = vehicle == 0 ? SVC_IGNORING : vehicle->getVClass();
    //         init(fromEdges, vehicle, msTime);
    //         this->myAmClean = false;
    //         // loop
    //         int num_visited = 0;
    //         const bool mayRevisit = this->myLookupTable != nullptr && !this->myLookupTable->consistent();
    //         const double speed = vehicle == nullptr ? this->myMaxSpeed : MIN2(vehicle->getMaxSpeed(), this->myMaxSpeed * vehicle->getChosenSpeedFactor());
    //         while (!this->myFrontierList.empty()) {
    //             num_visited += 1;
    //             // use the edge with the minimal length
    //             auto* const minimumInfo = this->myFrontierList.front();
    //             const E* const minEdge = minimumInfo->edge;
    //             // check whether the destination edge was already reached
    //             if (minEdge == to) {
    //                 this->buildPathFrom(minimumInfo, into);
    //                 this->endQuery(num_visited);
    //                 return true;
    //             }
    //             std::pop_heap(this->myFrontierList.begin(), this->myFrontierList.end(), this->myComparator);
    //             this->myFrontierList.pop_back();
    //             this->myFound.push_back(minimumInfo);
    //             minimumInfo->visited = true;
    //             const double effortDelta = this->getEffort(minEdge, vehicle, minimumInfo->leaveTime);
    //             const double leaveTime = minimumInfo->leaveTime + this->getTravelTime(minEdge, vehicle, minimumInfo->leaveTime, effortDelta);
    //
    //             // admissible A* heuristic: straight line distance at maximum speed
    //             // this is calculated from the end of minEdge so it possibly includes via efforts to the followers
    //             const double heuristic_remaining = (this->myLookupTable == nullptr ? minEdge->getDistanceTo(to) / speed :
    //                                                 this->myLookupTable->lowerBound(minEdge, to, speed, vehicle->getChosenSpeedFactor(),
    //                                                         minEdge->getMinimumTravelTime(nullptr), to->getMinimumTravelTime(nullptr)));
    //             //const double heuristic_remaining = 0.;
    //             if (heuristic_remaining == UNREACHABLE) {
    //                 continue;
    //             }
    //             const double heuristicEffort = minimumInfo->effort + effortDelta + heuristic_remaining;
    //             // check all ways from the edge with the minimal length
    //             for (const std::pair<const E*, const E*>& follower : minEdge->getViaSuccessors(vClass)) {
    //                 auto& followerInfo = this->myEdgeInfos[follower.first->getNumericalID()];
    //                 // check whether it can be used
    //                 if (followerInfo.prohibited || this->isProhibited(follower.first, vehicle)) {
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
    //                     if (oldEffort == std::numeric_limits<double>::max()) {
    //                         this->myFrontierList.push_back(&followerInfo);
    //                         std::push_heap(this->myFrontierList.begin(), this->myFrontierList.end(), this->myComparator);
    //                     } else {
    //                         auto fi = std::find(this->myFrontierList.begin(), this->myFrontierList.end(), &followerInfo);
    //                         if (fi == this->myFrontierList.end()) {
    //                             assert(mayRevisit);
    //                             this->myFrontierList.push_back(&followerInfo);
    //                             std::push_heap(this->myFrontierList.begin(), this->myFrontierList.end(), this->myComparator);
    //                         } else {
    //                             std::push_heap(this->myFrontierList.begin(), fi + 1, this->myComparator);
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //         this->endQuery(num_visited);
    //         if (!silent) {
    //             this->myErrorMsgHandler->informf("No connection between node '%' and edge '%' found.", fromNode->getID(), to->getID());
    //         }
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn compute_node2_edge(&mut self, fromNode: Ptr<N>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool {
        // C++ calls a two-argument isProhibited(edge, vehicle) and EdgeInfo::prohibited that no base class provides:
        // the member is never instantiated in C++ (it would not compile), so it has no behaviour to port
        panic!("Node2EdgeRouter::computeNode2Edge is never instantiated in C++")
    }

    /// C++: `void Node2EdgeRouter::updateViaCostUpToEdge(const E *const, const E *const, const V *const, double &, double &, double &) const` [const]
    /// Updates the via cost up to a given edge
    // template<class E, class N, class V, class M>
    // void Node2EdgeRouter<E, N, V, M>::updateViaCostUpToEdge(const E* const prev, const E* const e, const V* const v, double& time, double& effort, double& length) const {
    //     assert(prev && e);
    //     for (const std::pair<const E*, const E*>& follower : prev->getViaSuccessors()) {
    //         if (follower.first == e) {
    //             updateViaEdgeCost(follower.second, v, time, effort, length);
    //             break;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_via_cost_up_to_edge(&self, prev: Ptr<E>, e: Ptr<E>, v: Ptr<V>, time: &mut f64, effort: &mut f64, length: &mut f64) {
        for follower in prev.via_successors(SVC_IGNORING, false) {
            if follower.0 == e {
                self.update_via_edge_cost(follower.1, v, time, effort, length);
                break;
            }
        }
    }

    /// C++: `double Node2EdgeRouter::recomputeCostsNoLastEdge(const int &, const V *const, SUMOTime, double *) const` [const]
    /// Returns the recomputed cost for traversing the given edges, excluding the last one
    // double recomputeCostsNoLastEdge(const std::vector<const E*>& edges, const V* const v, SUMOTime msTime, double* lengthp = nullptr) const {
    //         const E* lastEdge = edges.back();
    //         double time = STEPS2TIME(msTime);
    //         double effort = 0.;
    //         double length = 0.;
    //         if (lengthp == nullptr) {
    //             lengthp = &length;
    //         } else {
    //             *lengthp = 0.;
    //         }
    //         const E* prev = nullptr;
    //         for (const E* const e : edges) {
    //             if (e == lastEdge) {
    //                 break;
    //             }
    //             if (isProhibited(e, v)) {
    //                 return -1;
    //             }
    //             updateViaCost(prev, e, v, time, effort, *lengthp);
    //             prev = e;
    //         }
    //         updateViaCostUpToEdge(prev, lastEdge, v, time, effort, *lengthp);
    //         return effort;
    //     }
    #[allow(unused_variables)]
    pub fn recompute_costs_no_last_edge(&self, edges: &Vec<Ptr<E>>, v: Ptr<V>, msTime: SUMOTime, lengthp: Ptr<f64>) -> f64 {
        let lastEdge = *edges.last().unwrap();
        let mut time = utils::common::time::steps2time(msTime);
        let mut effort = 0.;
        let mut length = 0.;
        let lengthp: &mut f64 = if lengthp.is_null() {
            &mut length
        } else {
            *lengthp.m() = 0.;
            lengthp.m()
        };
        let mut prev: Ptr<E> = Ptr::null();
        for e in edges.iter().copied() {
            if e == lastEdge {
                break;
            }
            // C++ calls a two-argument isProhibited(e, v) that SUMOAbstractRouter does not provide: never instantiated in C++ (it would not compile)
            if panic!("Node2EdgeRouter: two-argument isProhibited is never instantiated in C++") {
                return -1.;
            }
            self.update_via_cost(prev, e, v, &mut time, &mut effort, lengthp);
            prev = e;
        }
        self.update_via_cost_up_to_edge(prev, lastEdge, v, &mut time, &mut effort, lengthp);
        effort
    }

    /// C++: `void Node2EdgeRouter::setBulkMode(const bool)` [virtual]
    /// Bulk mode is not supported
    // virtual void setBulkMode(const bool mode) {
    //         UNUSED_PARAMETER(mode);
    //         throw std::runtime_error("Bulk mode is not supported by the node-to-edge router.");
    //     }
    #[allow(unused_variables)]
    pub fn set_bulk_mode(&mut self, mode: bool) {
        panic!("Bulk mode is not supported by the node-to-edge router.")
    }

    /// C++: `void Node2EdgeRouter::init(int, const V *const, const SUMOTime)` [protected]
    /// Initialize the node-to-edge router
    // template<class E, class N, class V, class M>
    // void Node2EdgeRouter<E, N, V, M>::init(std::vector<const E*> fromEdges, const V* const vehicle, const SUMOTime msTime) {
    //     // all EdgeInfos touched in the previous query are either in myFrontierList or myFound: clean those up
    //     for (auto& edgeInfo : this->myFrontierList) {
    //         edgeInfo->reset();
    //     }
    //     this->myFrontierList.clear();
    //     for (auto& edgeInfo : this->myFound) {
    //         edgeInfo->reset();
    //     }
    //     this->myFound.clear();
    //     for (const E* from : fromEdges) {
    //         if (from->isInternal()) {
    //             continue;
    //         }
    //         int edgeID = from->getNumericalID();
    //         auto& fromInfo = this->myEdgeInfos[edgeID];
    //         if (this->isProhibited(from, vehicle, STEPS2TIME(msTime))) {
    //             continue;
    //         }
    //         fromInfo.effort = 0.;
    //         fromInfo.heuristicEffort = 0.;
    //         fromInfo.prev = nullptr;
    //         fromInfo.leaveTime = STEPS2TIME(msTime);
    //         this->myFrontierList.push_back(&fromInfo);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, fromEdges: Vec<Ptr<E>>, vehicle: Ptr<V>, msTime: SUMOTime) {
        for edgeInfo in self.myFrontierList.iter() {
            edgeInfo.m().reset();
        }
        self.myFrontierList.clear();
        for edgeInfo in self.myFound.iter() {
            edgeInfo.m().reset();
        }
        self.myFound.clear();
        for from in fromEdges {
            if from.is_internal() {
                continue;
            }
            let edgeID = from.numerical_id();
            let fromInfo = Ptr::from_mut(&mut self.myEdgeInfos[edgeID as usize]);
            if self.is_prohibited(from, vehicle, steps2time(msTime)) {
                continue;
            }
            fromInfo.m().effort = 0.;
            fromInfo.m().heuristicEffort = 0.;
            fromInfo.m().prev = Ptr::null();
            fromInfo.m().leaveTime = steps2time(msTime);
            self.myFrontierList.push(fromInfo);
        }
    }

}

