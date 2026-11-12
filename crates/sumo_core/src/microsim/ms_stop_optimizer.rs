//! Port of microsim/MSStopOptimizer.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `MSBaseVehicle::StopEdgeInfo StopEdgeInfo`
pub type StopEdgeInfo = MSBaseVehicle_StopEdgeInfo;

/// Finds a sequence of skips and stop alternatives to maximize priority of reached stops
/// C++ class `MSStopOptimizer` (microsim/MSStopOptimizer.h)
pub struct MSStopOptimizer {
    pub myVehicle: Ptr<MSBaseVehicle>,
    pub myRouter: Ptr<SUMOAbstractRouter<MSEdge, SUMOVehicle>>,
    pub myT: SUMOTime,
    pub myMaxDelay: SUMOTime,
}

impl MSStopOptimizer {
    /// C++: `void MSStopOptimizer::MSStopOptimizer(MSBaseVehicle *, SUMOAbstractRouter<MSEdge, SUMOVehicle> &, SUMOTime, SUMOTime)`
    /// Constructor
    // MSStopOptimizer(MSBaseVehicle* veh, SUMOAbstractRouter<MSEdge, SUMOVehicle>& router, SUMOTime t, SUMOTime maxDelay) :
    //         myVehicle(veh),
    //         myRouter(router),
    //         myT(t),
    //         myMaxDelay(maxDelay)
    //     {}
    #[allow(unused_variables)]
    pub fn new(veh: Ptr<MSBaseVehicle>, router: &mut SUMOAbstractRouter<MSEdge, SUMOVehicle>, t: SUMOTime, maxDelay: SUMOTime) -> Self {
        todo!("port MSStopOptimizer::MSStopOptimizer")
    }

    /// C++: `void MSStopOptimizer::~MSStopOptimizer() noexcept` [virtual]
    /// Destructor
    // virtual ~MSStopOptimizer() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSStopOptimizer::~MSStopOptimizer")
    }

    /// C++: `ConstMSEdgeVector MSStopOptimizer::optimizeSkipped(const MSEdge *, double, std::vector<StopEdgeInfo> &, ConstMSEdgeVector) const` [const]
    // ConstMSEdgeVector
    // MSStopOptimizer::optimizeSkipped(const MSEdge* source, double sourcePos, std::vector<StopEdgeInfo>& stops, ConstMSEdgeVector edges) const {
    //     // intial solution is an upper bound on the acceptle total skipped priority
    //     double minSkipped = 0;
    //     double totalPrio = 0;
    //     std::vector<StopEdgeInfo> bestStops = stops;
    //     double bestCost = myRouter.recomputeCostsPos(edges, myVehicle, sourcePos, 0, myT);
    //     std::vector<int> skipped;
    //
    //     // each upcoming stop is part of a graph where we can start to search for a
    //     // better solution. We evaluate followers lazily and prune based on the best
    //     // solution already found
    //     std::priority_queue<std::shared_ptr<StopPathNode>, std::vector<std::shared_ptr<StopPathNode> >, spnCompare> queue;
    //
    //     auto prev = std::make_shared<StopPathNode>(*this, StopEdgeInfo(source, -1, SIMSTEP, sourcePos));
    //     std::shared_ptr<StopPathNode> bestNode = prev;
    //     prev->stopIndex = -1;
    //     prev->routeIndex = 0;
    //     prev->cost = bestCost;
    //     prev->checked = true;
    //     //std::cout << " i=-1 e=" << prev->edge->getID() << " sp=" << prev->skippedPrio << " rp=" << prev->reachedPrio << "\n";
    //     queue.push(prev);
    //
    //     for (int i = 0; i < (int)stops.size(); i++) {
    //         const StopEdgeInfo& stop = stops[i];
    //         if (stop.skipped) {
    //             skipped.push_back(i);
    //             minSkipped += stops[i].priority;
    //         } else {
    //             auto ptr = std::make_shared<StopPathNode>(*this, stop);
    //             ptr->skippedPrio = minSkipped;
    //             ptr->reachedPrio = prev->reachedPrio;
    //             ptr->reachedMandatory = prev->reachedMandatory;
    //             if (stop.priority >= 0) {
    //                 ptr->reachedPrio += stop.priority;
    //             } else {
    //                 ptr->reachedMandatory += 1;
    //             }
    //             ptr->checked = true;
    //             ptr->prev = prev;
    //             ptr->stopIndex = i;
    //             ConstMSEdgeVector subEdges(edges.begin(), edges.begin() + stop.routeIndex);
    //             ptr->cost = myRouter.recomputeCostsPos(subEdges, myVehicle, sourcePos, 0, myT);
    //             ptr->edges.insert(ptr->edges.begin(), edges.begin() + prev->routeIndex, edges.begin() + stop.routeIndex + 1);
    //             prev = ptr;
    //             bestNode = ptr;
    //             queue.push(ptr);
    //         }
    //         totalPrio += MAX2(0.0, stop.priority);
    //         //std::cout << " i=" << i << " e=" << ptr->edge->getID() << " sp=" << ptr->skippedPrio << " rp=" << ptr->reachedPrio << "\n";
    //     }
    // #ifdef DEBUG_OPTIMIZE_SKIPPED
    //     std::cout << SIMTIME << " optimizeSkipped veh=" << myVehicle->getID()
    //               << " source=" << source->getID() << " sourcePos=" << sourcePos
    //               << " nStops=" << stops.size() << " skipped=" << toString(skipped) << " qSize=" << queue.size()
    //               << " minSkipped=" << minSkipped << " totalPrio=" << totalPrio << "\n";
    //     for (const StopEdgeInfo& stop : stops) {
    //         std::cout << "   edge=" << stop.edge->getID() << " name=" << stop.nameTag.first << " wasSkipped=" << stop.skipped << " prio=" << stop.priority << "\n";
    //     }
    //     std::cout << " bestNode edge=" << bestNode->edge->getID() << " rPrio=" << bestNode->reachedPrio << " cost=" << bestNode->cost << "\n";
    // #endif
    //
    //     std::map<const MSEdge*, std::shared_ptr<StopPathNode> > bestIntermediate;
    //     while (!queue.empty()) {
    //         auto ptr = queue.top();
    //         queue.pop();
    // #ifdef DEBUG_OPTIMIZE_SKIPPED
    //         std::cout << "  qSize=" << queue.size() << " topNode edge=" << ptr->edge->getID()
    //                   << " name='" << ptr->nameTag.first << "' skippedPrio=" << ptr->skippedPrio
    //                   << " trackChanges=" << ptr->trackChanges
    //                   << " cost=" << ptr->cost
    //                   << " reachedPrio=" << ptr->reachedPrio
    //                   << " mandatory=" << ptr->reachedMandatory
    //                   << " stopIndex=" << ptr->stopIndex << "\n";
    // #endif
    //         auto succ = ptr->getSuccessor(stops, minSkipped);
    // #ifdef DEBUG_OPTIMIZE_SKIPPED
    //         std::cout << "      cost2=" << ptr->cost << " reachedPrio2=" << ptr->reachedPrio << " mandatory2=" << ptr->reachedMandatory << " succ=" << (succ == nullptr ? "NULL" : succ->edge->getID()) << "\n";
    // #endif
    //         if (*bestNode < *ptr) {
    //             minSkipped = MIN2(minSkipped, totalPrio - ptr->reachedPrio);
    // #ifdef DEBUG_OPTIMIZE_SKIPPED
    //             std::cout << "   newBestNode edge=" << bestNode->edge->getID() << " minSkipped=" << minSkipped
    //                       << " orm=" << bestNode->reachedMandatory << " rm=" << ptr->reachedMandatory
    //                       << " orp=" << bestNode->reachedPrio << " rp=" << ptr->reachedPrio
    //                       << " otc=" << bestNode->trackChanges << " tc=" << ptr->trackChanges
    //                       << " oc=" << bestNode->cost << " c=" << ptr->cost
    //                       << "\n";
    // #endif
    //             bestNode = ptr;
    //         }
    //         if (succ != nullptr) {
    //             if (minSkipped == 0 && ptr->trackChanges > bestNode->trackChanges) {
    // #ifdef DEBUG_OPTIMIZE_SKIPPED
    //                 std::cout << "pruned by minTC=" << bestNode->trackChanges << "\n";
    // #endif
    //                 continue;
    //             }
    //             auto it = bestIntermediate.find(ptr->edge);
    //             if (it == bestIntermediate.end()) {
    //                 bestIntermediate[ptr->edge] = ptr;
    // #ifdef DEBUG_OPTIMIZE_SKIPPED
    //                 std::cout << "firstBest " << ptr->edge->getID() << "\n";
    // #endif
    //             } else {
    //                 auto best = it->second;
    //                 assert(best->checked);
    //                 assert(ptr->checked);
    //                 if (best == ptr) {
    // #ifdef DEBUG_OPTIMIZE_SKIPPED
    //                     std::cout << "alreadyBest " << ptr->edge->getID() << " rp=" << ptr->reachedPrio << " tc=" << ptr->trackChanges << " c=" << ptr->cost << "\n";
    // #endif
    //                 } else if (!(*best < *ptr)) {
    // #ifdef DEBUG_OPTIMIZE_SKIPPED
    //                     std::cout << "skip " << ptr->edge->getID()
    //                               << " orp=" << best->reachedPrio << " rp=" << ptr->reachedPrio
    //                               << " otc=" << best->trackChanges << " tc=" << ptr->trackChanges
    //                               << " oc=" << best->cost << " c=" << ptr->cost
    //                               << "\n";
    // #endif
    //                     continue;
    //                 } else {
    //                     bestIntermediate[ptr->edge] = ptr;
    // #ifdef DEBUG_OPTIMIZE_SKIPPED
    //                     std::cout << "newBest " << ptr->edge->getID()
    //                               << " orp=" << best->reachedPrio << " rp=" << ptr->reachedPrio
    //                               << " otc=" << best->trackChanges << " tc=" << ptr->trackChanges
    //                               << " oc=" << best->cost << " c=" << ptr->cost
    //                               << "\n";
    // #endif
    //                 }
    //             }
    //             queue.push(ptr);
    //             queue.push(succ);
    //         }
    //     }
    //     for (auto& stop : stops) {
    //         // init true and later set false for all used stops
    //         if (!stop.skipped) {
    //             stop.backtracked = true;
    //             stop.skipped = true;
    //         }
    //     }
    //     ConstMSEdgeVector bestEdges;
    //     if (bestNode->stopIndex < 0) {
    //         // all stops were skipped. End route on current edge
    //         bestEdges.push_back(source);
    //     }
    //     while (bestNode != nullptr && bestNode->stopIndex >= 0) {
    // #ifdef DEBUG_OPTIMIZE_SKIPPED
    //         std::cout << "  revBestNode index=" << bestNode->stopIndex << " edge=" << bestNode->edge->getID() << " name=" << bestNode->nameTag.first << " tc=" << bestNode->trackChanges << " c=" << bestNode->cost  << "\n";
    // #endif
    //         stops[bestNode->stopIndex].skipped = false;
    //         stops[bestNode->stopIndex].backtracked = false;
    //         stops[bestNode->stopIndex].origEdge = bestNode->origEdge;
    //         if (!bestEdges.empty() && !bestNode->edges.empty()) {
    //             bestNode->edges.pop_back();
    //         }
    //         bestEdges.insert(bestEdges.begin(), bestNode->edges.begin(), bestNode->edges.end());
    //         bestNode = bestNode->prev;
    //     }
    // #ifdef DEBUG_OPTIMIZE_SKIPPED
    //     std::cout << "oldEdges=" << toString(edges) << "\nnewEdges=" << toString(bestEdges) << "\n";
    // #endif
    //     return bestEdges;
    // }
    #[allow(unused_variables)]
    pub fn optimize_skipped(&self, source: Ptr<MSEdge>, sourcePos: f64, stops: &mut Vec<StopEdgeInfo>, edges: ConstMSEdgeVector) -> ConstMSEdgeVector {
        todo!("port MSStopOptimizer::optimizeSkipped")
    }

    /// C++: `bool MSStopOptimizer::reachableInTime(const MSEdge *, double, const MSEdge *, double, SUMOTime, ConstMSEdgeVector &, double &) const` [const, private]
    // bool
    // MSStopOptimizer::reachableInTime(const MSEdge* from, double fromPos,
    //                                  const MSEdge* to, double toPos,
    //                                  SUMOTime arrival,
    //                                  ConstMSEdgeVector& into, double& cost) const {
    //     myRouter.compute(from, fromPos, to, toPos, myVehicle, myT, into, true);
    //     if (into.size() > 0) {
    //         cost = myRouter.recomputeCostsPos(into, myVehicle, fromPos, toPos, myT);
    //         //std::cout << " from=" << from->getID() << " fromPos=" << fromPos << " to=" << to->getID() << " toPos=" << toPos << " t=" << myT << " cost=" << cost << " arrival=" << arrival << " maxDelay=" << myMaxDelay << "\n";
    //         if (myT + TIME2STEPS(cost) <= arrival + myMaxDelay) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn reachable_in_time(&self, from: Ptr<MSEdge>, fromPos: f64, to: Ptr<MSEdge>, toPos: f64, arrival: SUMOTime, into: &mut ConstMSEdgeVector, cost: &mut f64) -> bool {
        todo!("port MSStopOptimizer::reachableInTime")
    }

}

/// information used during skip/alternative optimization
/// C++ class `MSStopOptimizer::StopPathNode` (microsim/MSStopOptimizer.h)
#[repr(C)]
pub struct MSStopOptimizer_StopPathNode {
    /// C++ base class `StopEdgeInfo`
    pub base: StopEdgeInfo,
    /// C++ base class `std::enable_shared_from_this<StopPathNode>`
    pub base2: Unmapped /* std::enable_shared_from_this<StopPathNode> */,
    pub so: Ptr<MSStopOptimizer>,
    pub skippedPrio: f64,
    pub reachedPrio: f64,
    pub reachedMandatory: i32,
    pub trackChanges: i32,
    pub cost: f64,
    pub stopIndex: i32,
    pub numSkipped: i32,
    pub altIndex: i32,
    pub checked: bool,
    pub edges: ConstMSEdgeVector,
    pub prev: Rc<MSStopOptimizer_StopPathNode>,
}

impl std::ops::Deref for MSStopOptimizer_StopPathNode {
    type Target = StopEdgeInfo;
    fn deref(&self) -> &StopEdgeInfo {
        &self.base
    }
}
impl std::ops::DerefMut for MSStopOptimizer_StopPathNode {
    fn deref_mut(&mut self) -> &mut StopEdgeInfo {
        &mut self.base
    }
}

impl MSStopOptimizer_StopPathNode {
    /// C++: `void StopPathNode::StopPathNode(const MSStopOptimizer &, const StopEdgeInfo &)`
    // StopPathNode(const MSStopOptimizer& _so, const StopEdgeInfo& o):
    //             StopEdgeInfo(o.edge, o.priority, o.arrival, o.pos),
    //             so(_so) {
    //             nameTag = o.nameTag;
    //             delay = o.delay;
    //             routeIndex = o.routeIndex;
    //         }
    #[allow(unused_variables)]
    pub fn new(_so: &MSStopOptimizer, o: &StopEdgeInfo) -> Self {
        todo!("port StopPathNode::StopPathNode")
    }

    /// C++: `std::shared_ptr<StopPathNode> StopPathNode::getSuccessor(const std::vector<StopEdgeInfo> &, double)`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_successor(&mut self, stops: &Vec<StopEdgeInfo>, minSkipped: f64) -> Rc<MSStopOptimizer_StopPathNode> {
        todo!("port StopPathNode::getSuccessor")
    }

    /// C++: `bool StopPathNode::operator<(const StopPathNode &) const` [const]
    // bool operator<(const StopPathNode& b) const {
    //             if (reachedMandatory == b.reachedMandatory) {
    //                 if (reachedPrio == b.reachedPrio) {
    //                     if (trackChanges == b.trackChanges) {
    //                         return cost > b.cost;
    //                     }
    //                     return trackChanges > b.trackChanges;
    //                 }
    //                 return reachedPrio < b.reachedPrio;
    //             }
    //             return reachedMandatory < b.reachedMandatory;
    //         }
    #[allow(unused_variables)]
    pub fn lt_op(&self, b: &MSStopOptimizer_StopPathNode) -> bool {
        todo!("port StopPathNode::operator<")
    }

}

/// C++ class `MSStopOptimizer::spnCompare` (microsim/MSStopOptimizer.h)
pub struct MSStopOptimizer_spnCompare {
}

impl MSStopOptimizer_spnCompare {
    /// C++: `bool spnCompare::operator()(const std::shared_ptr<StopPathNode> &, const std::shared_ptr<StopPathNode> &) const` [const]
    // bool operator()(const std::shared_ptr<StopPathNode>& a,
    //                         const std::shared_ptr<StopPathNode>& b) const {
    //             if (a->skippedPrio == b->skippedPrio) {
    //                 if (a->stopIndex == b->stopIndex) {
    //                     return *a < *b;
    //                 }
    //                 return a->stopIndex < b->stopIndex;
    //             }
    //             return a->skippedPrio > b->skippedPrio;
    //         }
    #[allow(unused_variables)]
    pub fn call_op(&self, a: &Rc<MSStopOptimizer_StopPathNode>, b: &Rc<MSStopOptimizer_StopPathNode>) -> bool {
        todo!("port spnCompare::operator()")
    }

}

