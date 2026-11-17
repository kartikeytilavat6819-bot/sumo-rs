//! Port of utils/router/AFCentralizedSPTree.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::a_star_lookup_table::UNREACHABLE;
use crate::utils::router::ch_builder::{std_pop_heap, std_push_heap};
use crate::utils::router::router_edge::{RouterEdge, RouterVehicle};

fn msg_instance(warning: bool) -> Ptr<MsgHandler> {
    utils::common::msg_handler::with_instance(if warning { utils::common::msg_handler::MsgType::Warning } else { utils::common::msg_handler::MsgType::Error }, |h| Ptr::from_mut(h))
}

/// C++ `MAX2` macro
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

/// C++ typedef `typename AFInfo<E>::ArcInfo AFCentralizedSPTree::ArcInfo`
pub type AFCentralizedSPTree_ArcInfo = AFInfo_ArcInfo;

/// C++ typedef `typename KDTreePartition<E, N, V>::Cell AFCentralizedSPTree::Cell`
pub type AFCentralizedSPTree_Cell = KDTreePartition_Cell;

/// C++ class `AFCentralizedSPTree` (utils/router/AFCentralizedSPTree.h)
pub struct AFCentralizedSPTree<E, N, V> {
    /// The min edge heap
    pub myFrontierList: Vec<Ptr<SUMOAbstractRouter_EdgeInfo>>,
    /// The list of visited edges (for resetting)
    pub myFound: Vec<Ptr<SUMOAbstractRouter_EdgeInfo>>,
    /// The container of edge information
    pub myEdgeInfos: Vec<SUMOAbstractRouter_EdgeInfo>,
    /// The edge informations specific to arc flag routing
    pub myArcInfos: Ptr<Vec<Ptr<AFCentralizedSPTree_ArcInfo>>>,
    /// The boolean flag indicating whether edge permissions need to be considered or not
    pub myHavePermissions: bool,
    /// The boolean flag indicating whether edge restrictions need to be considered or not
    pub myHaveRestrictions: bool,
    /// The handler for routing errors
    pub myErrorMsgHandler: Ptr<MsgHandler>,
    /// The object's operation to perform
    pub myEffortProvider: Ptr<SUMOAbstractRouter<E, V>>,
    /// The comparator
    pub myComparator: Box<AFCentralizedSPTree_EdgeInfoComparator>,
    /// The maximum speed in the network
    pub myMaxSpeed: f64,
    _p: std::marker::PhantomData<(E, N, V,)>,
}

impl<E: RouterEdge<V>, N, V: RouterVehicle> AFCentralizedSPTree<E, N, V> {
    /// C++: `void AFCentralizedSPTree::AFCentralizedSPTree<E, N, V>(const int &, int &, bool, SUMOAbstractRouter<E, V> *, const bool, const bool)`
    /// Constructor
    // AFCentralizedSPTree(const std::vector<E*>& edges, std::vector<ArcInfo*>& arcInfos, bool unbuildIsWarning,
    //                         SUMOAbstractRouter<E, V>* effortProvider,
    //                         const bool havePermissions = false, const bool haveRestrictions = false) :
    //         myArcInfos(arcInfos),
    //         myHavePermissions(havePermissions),
    //         myHaveRestrictions(haveRestrictions),
    //         myErrorMsgHandler(unbuildIsWarning ? MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance()),
    //         myEffortProvider(effortProvider),
    //         myMaxSpeed(NUMERICAL_EPS) {
    //         for (const E* const edge : edges) {
    //             myEdgeInfos.push_back(typename SUMOAbstractRouter<E, V>::EdgeInfo(edge));
    //             myMaxSpeed = MAX2(myMaxSpeed, edge->getSpeedLimit() * MAX2(1.0, edge->getLengthGeometryFactor()));
    //         }
    //         myComparator = new EdgeInfoComparator(myArcInfos);
    //     }
    #[allow(unused_variables)]
    pub fn new(edges: &Vec<Ptr<E>>, arcInfos: &mut Vec<Ptr<AFCentralizedSPTree_ArcInfo>>, unbuildIsWarning: bool, effortProvider: Ptr<SUMOAbstractRouter<E, V>>, havePermissions: bool, haveRestrictions: bool) -> Self {
        let mut myEdgeInfos = Vec::new();
        let mut myMaxSpeed = utils::geom::NUMERICAL_EPS;
        for edge in edges.iter() {
            myEdgeInfos.push(SUMOAbstractRouter_EdgeInfo::new(edge.cast::<()>()));
            myMaxSpeed = max2(myMaxSpeed, edge.speed_limit() * max2(1.0, edge.length_geometry_factor()));
        }
        let myArcInfos = Ptr::from_mut(arcInfos);
        Self {
            myFrontierList: Vec::new(),
            myFound: Vec::new(),
            myEdgeInfos,
            myArcInfos,
            myHavePermissions: havePermissions,
            myHaveRestrictions: haveRestrictions,
            myErrorMsgHandler: msg_instance(unbuildIsWarning),
            myEffortProvider: effortProvider,
            myComparator: Box::new(AFCentralizedSPTree_EdgeInfoComparator::new(myArcInfos)),
            myMaxSpeed,
            _p: std::marker::PhantomData,
        }
    }

}

impl<E, N, V> AFCentralizedSPTree<E, N, V> {
    /// C++: `void AFCentralizedSPTree::~AFCentralizedSPTree<E, N, V>()`
    /// Destructor
    // ~AFCentralizedSPTree() {
    //         delete myComparator;
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ `delete myComparator`: the owned Box is released together with the tree
    }
}

impl<E: RouterEdge<V>, N, V: RouterVehicle> AFCentralizedSPTree<E, N, V> {

    /// C++: `bool AFCentralizedSPTree::isProhibited(const E *const, const V *const) const` [const]
    /// Returns true iff driving the given vehicle on the given edge is prohibited
    // bool isProhibited(const E* const edge, const V* const vehicle) const {
    //         return (myHavePermissions && edge->prohibits(vehicle)) || (myHaveRestrictions && edge->restricts(vehicle));
    //     }
    #[allow(unused_variables)]
    pub fn is_prohibited(&self, edge: Ptr<E>, vehicle: Ptr<V>) -> bool {
        (self.myHavePermissions && edge.prohibits(vehicle, false)) || (self.myHaveRestrictions && edge.restricts(vehicle))
    }

    /// C++: `bool AFCentralizedSPTree::computeCentralizedSPTree(SUMOTime, const Cell *, const V *const, const int &, bool)`
    /// Computes a shortest path tree for each boundary edge of the given cell, returns true iff this was successful
    // bool computeCentralizedSPTree(SUMOTime msTime, const Cell* cell, const V* const vehicle,
    //                                   const std::map<const E*, std::vector<const E*>>& incomingEdgesOfOutgoingBoundaryEdges,
    //                                   bool silent = false) {
    //         assert(cell != nullptr);
    //         const std::unordered_set<const E*>& fromEdges = cell->getOutgoingBoundaryEdges();
    //         if (fromEdges.empty()) { // nothing to do here
    //             return false;
    //         }
    //         double length = 0.; // dummy for the via edge cost update
    //         const SUMOVehicleClass vClass = vehicle == 0 ? SVC_IGNORING : vehicle->getVClass();
    //         std::vector<const E*> fromEdgesAsVector(fromEdges.begin(), fromEdges.end());
    //         init(fromEdgesAsVector, vehicle, msTime);
    //         size_t numberOfVisitedFromEdges = 0;
    //         bool minIsFromEdge = false;
    // #ifdef CSPT_DEBUG_LEVEL_0
    //         size_t numberOfTouchedSupercellEdges = 0;
    //         int num_visited = 0;
    // #endif
    // #ifdef _DEBUG
    //         const Cell* supercell = cell->getSupercell();
    // #endif
    //         const bool mayRevisit = true;
    //
    //         while (!myFrontierList.empty()) {
    // #ifdef CSPT_DEBUG_LEVEL_0
    //             num_visited += 1;
    // #endif
    //             // use the edge with the minimal length
    //             typename SUMOAbstractRouter<E, V>::EdgeInfo* minimumInfo = myFrontierList.front();
    //             const E* const minEdge = minimumInfo->edge;
    //             assert(!minEdge->isInternal());
    //             ArcInfo* minimumArcInfo = myArcInfos[minEdge->getNumericalID()];
    //             if (minimumInfo->visited || numberOfVisitedFromEdges < fromEdges.size()) {
    //                 minIsFromEdge = incomingEdgesOfOutgoingBoundaryEdges.find(minEdge) != incomingEdgesOfOutgoingBoundaryEdges.end();
    //             } else {
    //                 minIsFromEdge = false;
    //             }
    //             if (minIsFromEdge) {
    //                 if (numberOfVisitedFromEdges < fromEdges.size()) {
    //                     numberOfVisitedFromEdges++;
    //                 }
    //             }
    //             assert(incomingEdgesOfOutgoingBoundaryEdges.size() == fromEdges.size());
    //             assert(minimumArcInfo->key != std::numeric_limits<double>::max() && minimumArcInfo->key != UNREACHABLE);
    //             size_t index;
    // #ifdef CSPT_DEBUG_LEVEL_0
    //             if (supercell->contains(minEdge->getToJunction())) {
    //                 // minEdge is a supercell edge (we check for the to-node since we work on a backward graph)
    //                 if (!minimumArcInfo->touched) {
    //                     numberOfTouchedSupercellEdges++;
    //                     minimumArcInfo->touched = true;
    //                 }
    //             }
    // #endif
    // #ifdef CSPT_DEBUG_LEVEL_0
    //             if (num_visited % 500 == 0) {
    //                 std::cout << "num_visited: " << num_visited << ", numberOfTouchedSupercellEdges: " << numberOfTouchedSupercellEdges
    //                           << ", minimumArcInfo->key: " << minimumArcInfo->key << std::endl;
    //             }
    // #endif
    //             std::pop_heap(myFrontierList.begin(), myFrontierList.end(), *myComparator);
    //             myFrontierList.pop_back();
    //             myFound.push_back(minimumInfo);
    //             minimumInfo->visited = true;
    //             const double effortDelta = myEffortProvider->getEffort(minEdge, vehicle, minimumInfo->leaveTime);
    //             const double leaveTime = minimumInfo->leaveTime + myEffortProvider->getTravelTime(minEdge, vehicle, minimumInfo->leaveTime, effortDelta);
    //
    //             // check all ways from the edge with the minimal length
    //             for (const std::pair<const E*, const E*>& follower : minEdge->getViaSuccessors(vClass)) {
    //                 assert(!follower.first->isInternal());
    //                 bool wasPushedToHeap = false;
    //                 auto& followerInfo = myEdgeInfos[follower.first->getNumericalID()];
    //                 ArcInfo* followerArcInfo = myArcInfos[follower.first->getNumericalID()];
    //                 // check whether it can be used
    //                 if (isProhibited(follower.first, vehicle)) {
    //                     if (!silent) {
    //                         myErrorMsgHandler->inform("Vehicle '" + Named::getIDSecure(vehicle) + "' is not allowed on source edge '" + followerInfo.edge->getID() + "'.");
    //                     }
    //                     continue;
    //                 }
    //
    //                 if (followerArcInfo->effortsToBoundaryNodes.empty()) { // non-initialized non-supercell edge
    //                     assert(!supercell->contains(follower.first->getToJunction()));
    //                     std::fill_n(std::back_inserter(followerArcInfo->effortsToBoundaryNodes),
    //                                 minimumArcInfo->effortsToBoundaryNodes.size(), std::numeric_limits<double>::max());
    //                 }
    //
    //                 double key = std::numeric_limits<double>::max();
    //                 assert(followerArcInfo->effortsToBoundaryNodes.size() == minimumArcInfo->effortsToBoundaryNodes.size());
    //                 bool hasImproved = false;
    //                 // loop over all boundary nodes
    //                 for (index = 0; index < followerArcInfo->effortsToBoundaryNodes.size(); index++) {
    //                     // is minEdge a from-edge (i.e., an outgoing boundary edge of the passed cell),
    //                     // and 'index' not the index of its from-node (i.e., not of the 'own' boundary node)?
    //                     if (minIsFromEdge && (incomingEdgesOfOutgoingBoundaryEdges.at(minEdge))[index]) {
    //                         // if yes, assign the successor edges of the incoming edge of minEdge on the shortest route from
    //                         // the boundary node with the index 'index' to followersOfIncomingEdge
    //                         assert((incomingEdgesOfOutgoingBoundaryEdges.at(minEdge)).size()
    //                                == followerArcInfo->effortsToBoundaryNodes.size());
    //                         const std::vector<std::pair<const E*, const E*>>& followersOfIncomingEdge
    //                                 = ((incomingEdgesOfOutgoingBoundaryEdges.at(minEdge))[index])->getViaSuccessors(vClass);
    //                         // is the current follower among said successor edges?
    //                         bool turningAllowed = false;
    //                         for (std::pair<const E*, const E*> followerOfIncomingEdge : followersOfIncomingEdge) {
    //                             if (follower.first == followerOfIncomingEdge.first) {
    //                                 turningAllowed = true;
    //                                 break;
    //                             }
    //                         }
    //                         // if not, then turning from said incoming edge to the current follower is not allowed
    //                         // and we mustn't propagate the distance to said boundary node
    //                         // instead simply skip this follower
    //                         if (!turningAllowed) {
    //                             continue;
    //                         }
    //                     }
    //                     // propagate distances to other boundary nodes
    //                     double effortToFollower = minimumArcInfo->effortsToBoundaryNodes[index] == UNREACHABLE ?
    //                                               UNREACHABLE : minimumArcInfo->effortsToBoundaryNodes[index] + effortDelta;
    //                     if (effortToFollower == UNREACHABLE) {
    //                         continue; // no need to consider this follower
    //                     }
    //                     double time = leaveTime;
    //                     myEffortProvider->updateViaEdgeCost(follower.second, vehicle, time, effortToFollower, length);
    //                     if (effortToFollower < key) {
    //                         key = effortToFollower;
    //                     }
    //                     const double oldEffort = followerArcInfo->effortsToBoundaryNodes[index];
    //                     if (oldEffort != std::numeric_limits<double>::max()) {
    //                         wasPushedToHeap = true; // must have been pushed to heap during an earlier visit
    //                     }
    //
    //                     if ((!followerInfo.visited || mayRevisit)
    //                             && effortToFollower < oldEffort) {
    //                         hasImproved = true;
    //                         followerArcInfo->effortsToBoundaryNodes[index] = effortToFollower;
    //                     }
    //                 } // end index loop
    //
    //                 if (!hasImproved) {
    //                     continue; // no need to re-enque this follower, continue w/ next one
    //                 }
    //                 followerArcInfo->key = key;
    //                 if (!wasPushedToHeap) {
    //                     myFrontierList.push_back(&followerInfo);
    //                     std::push_heap(myFrontierList.begin(), myFrontierList.end(), *myComparator);
    //                 } else {
    //                     auto fi = std::find(myFrontierList.begin(), myFrontierList.end(), &followerInfo);
    //                     if (fi == myFrontierList.end()) { // has already been expanded, reinsert into frontier
    //                         assert(mayRevisit);
    //                         myFrontierList.push_back(&followerInfo);
    //                         std::push_heap(myFrontierList.begin(), myFrontierList.end(), *myComparator);
    //                     } else {
    //                         std::push_heap(myFrontierList.begin(), fi + 1, *myComparator);
    //                     }
    //                 }
    //             } // end follower loop
    //         } // end while (!myFrontierList.empty())
    //
    // #ifdef CSPT_DEBUG_LEVEL_0
    //         std::cout << "centralizedSPTree finished (queue empty)." << std::endl;
    // #endif
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn compute_centralized_sp_tree(&mut self, msTime: SUMOTime, cell: Ptr<KDTreePartition_Cell>, vehicle: Ptr<V>, incomingEdgesOfOutgoingBoundaryEdges: &BTreeMap<Ptr<E>, Vec<Ptr<E>>>, silent: bool) -> bool {
        let fromEdges = cell.get_outgoing_boundary_edges();
        if fromEdges.is_empty() {
            // nothing to do here
            return false;
        }
        let mut length = 0.; // dummy for the via edge cost update
        let vClass = if vehicle.is_null() { SVC_IGNORING } else { vehicle.v_class() };
        // determinism: C++ iterates a std::unordered_set here, so the order of the initial frontier is not reproducible either
        let fromEdgesAsVector: Vec<Ptr<E>> = fromEdges.iter().map(|e| e.cast::<E>()).collect();
        self.init(fromEdgesAsVector, vehicle, msTime);
        let mut numberOfVisitedFromEdges: usize = 0;
        let mut minIsFromEdge = false;
        let mayRevisit = true;

        while !self.myFrontierList.is_empty() {
            // use the edge with the minimal length
            let minimumInfo = self.myFrontierList[0];
            let minEdge = minimumInfo.edge.cast::<E>();
            let minimumArcInfo = (*self.myArcInfos)[minEdge.numerical_id() as usize];
            if minimumInfo.visited || numberOfVisitedFromEdges < fromEdges.len() {
                minIsFromEdge = incomingEdgesOfOutgoingBoundaryEdges.contains_key(&minEdge);
            } else {
                minIsFromEdge = false;
            }
            if minIsFromEdge && numberOfVisitedFromEdges < fromEdges.len() {
                numberOfVisitedFromEdges += 1;
            }

            std_pop_heap(&mut self.myFrontierList, &|a, b| self.myComparator.call_op::<E, V>(a, b));
            self.myFrontierList.pop();
            self.myFound.push(minimumInfo);
            minimumInfo.m().visited = true;
            let effortDelta = self.myEffortProvider.get_effort(minEdge, vehicle, minimumInfo.leaveTime);
            let leaveTime = minimumInfo.leaveTime + self.myEffortProvider.get_travel_time(minEdge, vehicle, minimumInfo.leaveTime, effortDelta);

            // check all ways from the edge with the minimal length
            for follower in minEdge.via_successors(vClass, false) {
                let followerInfo = Ptr::from_mut(&mut self.myEdgeInfos[follower.0.numerical_id() as usize]);
                let followerArcInfo = (*self.myArcInfos)[follower.0.numerical_id() as usize];
                // check whether it can be used
                if self.is_prohibited(follower.0, vehicle) {
                    if !silent {
                        self.myErrorMsgHandler.m().inform(&format!("Vehicle '{}' is not allowed on source edge '{}'.", vehicle_id_secure(vehicle), followerInfo.edge.cast::<E>().id()), true);
                    }
                    continue;
                }

                if followerArcInfo.effortsToBoundaryNodes.is_empty() {
                    // non-initialized non-supercell edge
                    followerArcInfo.m().effortsToBoundaryNodes = vec![f64::MAX; minimumArcInfo.effortsToBoundaryNodes.len()];
                }

                let mut key = f64::MAX;
                let mut hasImproved = false;
                let mut wasPushedToHeap = false;
                // loop over all boundary nodes
                for index in 0..followerArcInfo.effortsToBoundaryNodes.len() {
                    // is minEdge a from-edge (i.e., an outgoing boundary edge of the passed cell),
                    // and 'index' not the index of its from-node (i.e., not of the 'own' boundary node)?
                    if minIsFromEdge && !incomingEdgesOfOutgoingBoundaryEdges[&minEdge][index].is_null() {
                        // if yes, assign the successor edges of the incoming edge of minEdge on the shortest route from
                        // the boundary node with the index 'index' to followersOfIncomingEdge
                        let incomingEdge = incomingEdgesOfOutgoingBoundaryEdges[&minEdge][index];
                        let followersOfIncomingEdge = incomingEdge.via_successors(vClass, false);
                        // is the current follower among said successor edges?
                        let mut turningAllowed = false;
                        for followerOfIncomingEdge in followersOfIncomingEdge {
                            if follower.0 == followerOfIncomingEdge.0 {
                                turningAllowed = true;
                                break;
                            }
                        }
                        // if not, then turning from said incoming edge to the current follower is not allowed
                        // and we mustn't propagate the distance to said boundary node
                        // instead simply skip this follower
                        if !turningAllowed {
                            continue;
                        }
                    }
                    // propagate distances to other boundary nodes
                    let mut effortToFollower = if minimumArcInfo.effortsToBoundaryNodes[index] == UNREACHABLE { UNREACHABLE } else { minimumArcInfo.effortsToBoundaryNodes[index] + effortDelta };
                    if effortToFollower == UNREACHABLE {
                        continue; // no need to consider this follower
                    }
                    let mut time = leaveTime;
                    self.myEffortProvider.update_via_edge_cost(follower.1, vehicle, &mut time, &mut effortToFollower, &mut length);
                    if effortToFollower < key {
                        key = effortToFollower;
                    }
                    let oldEffort = followerArcInfo.effortsToBoundaryNodes[index];
                    if oldEffort != f64::MAX {
                        wasPushedToHeap = true; // must have been pushed to heap during an earlier visit
                    }

                    if (!followerInfo.visited || mayRevisit) && effortToFollower < oldEffort {
                        hasImproved = true;
                        followerArcInfo.m().effortsToBoundaryNodes[index] = effortToFollower;
                    }
                } // end index loop

                if !hasImproved {
                    continue; // no need to re-enque this follower, continue w/ next one
                }
                followerArcInfo.m().key = key;
                if !wasPushedToHeap {
                    self.myFrontierList.push(followerInfo);
                    std_push_heap(&mut self.myFrontierList, &|a, b| self.myComparator.call_op::<E, V>(a, b));
                } else {
                    match self.myFrontierList.iter().position(|p| *p == followerInfo) {
                        None => {
                            // has already been expanded, reinsert into frontier
                            self.myFrontierList.push(followerInfo);
                            std_push_heap(&mut self.myFrontierList, &|a, b| self.myComparator.call_op::<E, V>(a, b));
                        }
                        Some(fi) => std_push_heap(&mut self.myFrontierList[..=fi], &|a, b| self.myComparator.call_op::<E, V>(a, b)),
                    }
                }
            } // end follower loop
        } // end while (!myFrontierList.empty())

        true
    }

    /// C++: `void AFCentralizedSPTree::init(int, const V *const, SUMOTime)` [protected]
    /// Initialize the arc flag router
    // template<class E, class N, class V>
    // void AFCentralizedSPTree<E, N, V>::init(std::vector<const E*> fromEdges, const V* const vehicle, SUMOTime msTime) {
    //     // all EdgeInfos touched in the previous query are either in myFrontierList or myFound: clean those up
    //     for (auto& edgeInfo : myFrontierList) {
    //         edgeInfo->reset();
    //     }
    //     myFrontierList.clear();
    //     for (auto& edgeInfo : myFound) {
    //         edgeInfo->reset();
    //     }
    //     for (auto& arcInfo : myArcInfos) {
    //         arcInfo->reset(); // does not reset effortsToBoundaryNodes
    //     }
    //     myFound.clear();
    //     for (const E* from : fromEdges) {
    //         if (from->isInternal()) {
    //             continue;
    //         }
    //         int edgeID = from->getNumericalID();
    //         auto& fromInfo = myEdgeInfos[edgeID];
    //         if (isProhibited(from, vehicle)) {
    //             continue;
    //         }
    //         fromInfo.heuristicEffort = 0.;
    //         fromInfo.prev = nullptr;
    //         fromInfo.leaveTime = STEPS2TIME(msTime);
    //         myFrontierList.push_back(&fromInfo);
    //         ArcInfo* fromArcInfo = myArcInfos[edgeID];
    //         fromArcInfo->key = 0;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init(&mut self, fromEdges: Vec<Ptr<E>>, vehicle: Ptr<V>, msTime: SUMOTime) {
        // all EdgeInfos touched in the previous query are either in myFrontierList or myFound: clean those up
        for edgeInfo in self.myFrontierList.iter() {
            edgeInfo.m().reset();
        }
        self.myFrontierList.clear();
        for edgeInfo in self.myFound.iter() {
            edgeInfo.m().reset();
        }
        for arcInfo in (*self.myArcInfos).iter() {
            arcInfo.m().reset(); // does not reset effortsToBoundaryNodes
        }
        self.myFound.clear();
        for from in fromEdges {
            if from.is_internal() {
                continue;
            }
            let edgeID = from.numerical_id();
            let fromInfo = Ptr::from_mut(&mut self.myEdgeInfos[edgeID as usize]);
            if self.is_prohibited(from, vehicle) {
                continue;
            }
            fromInfo.m().heuristicEffort = 0.;
            fromInfo.m().prev = Ptr::null();
            fromInfo.m().leaveTime = steps2time(msTime);
            self.myFrontierList.push(fromInfo);
            let fromArcInfo = (*self.myArcInfos)[edgeID as usize];
            fromArcInfo.m().key = 0.;
        }
    }

}

/// C++ class `AFCentralizedSPTree::EdgeInfoComparator` (utils/router/AFCentralizedSPTree.h)
pub struct AFCentralizedSPTree_EdgeInfoComparator {
    pub myArcInfos: Ptr<Vec<Ptr<AFCentralizedSPTree_ArcInfo>>>,
}

impl AFCentralizedSPTree_EdgeInfoComparator {
    /// C++: `void EdgeInfoComparator::EdgeInfoComparator(int &)`
    /// Constructor
    // EdgeInfoComparator(std::vector<ArcInfo*>& arcInfos) : myArcInfos(arcInfos) {
    //         }
    #[allow(unused_variables)]
    pub fn new(arcInfos: Ptr<Vec<Ptr<AFCentralizedSPTree_ArcInfo>>>) -> Self {
        Self { myArcInfos: arcInfos }
    }

    /// C++: `bool EdgeInfoComparator::operator()(const typename SUMOAbstractRouter<E, V>::EdgeInfo *, const typename SUMOAbstractRouter<E, V>::EdgeInfo *) const` [const]
    /// Comparing method
    // bool operator()(const typename SUMOAbstractRouter<E, V>::EdgeInfo* edgeInfo1,
    //                         const typename SUMOAbstractRouter<E, V>::EdgeInfo* edgeInfo2) const {
    //             int index1 = edgeInfo1->edge->getNumericalID();
    //             int index2 = edgeInfo2->edge->getNumericalID();
    //             ArcInfo* arcInfo1 = myArcInfos[index1];
    //             ArcInfo* arcInfo2 = myArcInfos[index2];
    //             double key1 = arcInfo1->key;
    //             double key2 = arcInfo2->key;
    //
    //             if (key1 == key2) { // tie
    //                 return index1 > index2;
    //             }
    //             return key1 > key2;
    //         }
    #[allow(unused_variables)]
    pub fn call_op<E: RouterEdge<V>, V>(&self, edgeInfo1: Ptr<SUMOAbstractRouter_EdgeInfo>, edgeInfo2: Ptr<SUMOAbstractRouter_EdgeInfo>) -> bool {
        let index1 = edge_numerical_id::<E, V>(edgeInfo1.edge);
        let index2 = edge_numerical_id::<E, V>(edgeInfo2.edge);
        let key1 = (*self.myArcInfos)[index1 as usize].key;
        let key2 = (*self.myArcInfos)[index2 as usize].key;

        if key1 == key2 {
            // tie
            return index1 > index2;
        }
        key1 > key2
    }

}

