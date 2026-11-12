//! Port of microsim/MSStopOptimizer.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::RefCell;
use utils::common::time::time2steps;

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
        Self {
            myVehicle: veh,
            myRouter: Ptr::from_mut(router),
            myT: t,
            myMaxDelay: maxDelay,
        }
    }

    /// C++: `void MSStopOptimizer::~MSStopOptimizer() noexcept` [virtual]
    /// Destructor
    // virtual ~MSStopOptimizer() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; all members are non-owning Ptr/value fields, so there is nothing to release.
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
        // The C++ `bestStops` copy is never read, and the `skipped` index list only feeds
        // DEBUG_OPTIMIZE_SKIPPED output, so neither is materialised here.
        let simstep: SUMOTime = MSNet::get_instance().get_current_time_step();
        let vehicle = self.myVehicle.cast::<SUMOVehicle>();
        // intial solution is an upper bound on the acceptle total skipped priority
        let mut minSkipped: f64 = 0.0;
        let mut totalPrio: f64 = 0.0;
        let bestCost: f64 = self.myRouter.recompute_costs_pos(&edges, vehicle, sourcePos, 0.0, self.myT, Ptr::default());

        // each upcoming stop is part of a graph where we can start to search for a
        // better solution. We evaluate followers lazily and prune based on the best
        // solution already found
        // determinism: queue pop order among spnCompare-equivalent nodes follows libstdc++ heap layout (see MSStopOptimizer_StopPathQueue).
        let mut queue = MSStopOptimizer_StopPathQueue::new();

        let mut prev: Rc<RefCell<MSStopOptimizer_StopPathNode>> = Rc::new(RefCell::new(
            MSStopOptimizer_StopPathNode::new(self, &MSBaseVehicle_StopEdgeInfo::new(source, -1.0, simstep, sourcePos, false)),
        ));
        let mut bestNode = prev.clone();
        {
            let mut p = prev.borrow_mut();
            p.stopIndex = -1;
            p.routeIndex = 0;
            p.cost = bestCost;
            p.checked = true;
        }
        //std::cout << " i=-1 e=" << prev->edge->getID() << " sp=" << prev->skippedPrio << " rp=" << prev->reachedPrio << "\n";
        queue.push(prev.clone());

        for i in 0..stops.len() {
            let stop = &stops[i];
            if stop.skipped {
                minSkipped += stops[i].priority;
            } else {
                let ptr = Rc::new(RefCell::new(MSStopOptimizer_StopPathNode::new(self, stop)));
                {
                    let p = prev.borrow();
                    let mut q = ptr.borrow_mut();
                    q.skippedPrio = minSkipped;
                    q.reachedPrio = p.reachedPrio;
                    q.reachedMandatory = p.reachedMandatory;
                    if stop.priority >= 0.0 {
                        q.reachedPrio += stop.priority;
                    } else {
                        q.reachedMandatory += 1;
                    }
                    q.checked = true;
                    q.prev = Some(prev.clone());
                    q.stopIndex = i as i32;
                    let subEdges: ConstMSEdgeVector = edges[..stop.routeIndex as usize].to_vec();
                    q.cost = self.myRouter.recompute_costs_pos(&subEdges, vehicle, sourcePos, 0.0, self.myT, Ptr::default());
                    q.edges.splice(0..0, edges[p.routeIndex as usize..=stop.routeIndex as usize].iter().cloned());
                }
                prev = ptr.clone();
                bestNode = ptr.clone();
                queue.push(ptr);
            }
            totalPrio += if 0.0 > stop.priority { 0.0 } else { stop.priority }; // MAX2(0.0, stop.priority)
            //std::cout << " i=" << i << " e=" << ptr->edge->getID() << " sp=" << ptr->skippedPrio << " rp=" << ptr->reachedPrio << "\n";
        }

        // determinism: bestIntermediate is only looked up and inserted into, never iterated, so the map order is not observable.
        let mut bestIntermediate: BTreeMap<Ptr<MSEdge>, Rc<RefCell<MSStopOptimizer_StopPathNode>>> = BTreeMap::new();
        while let Some(ptr) = queue.pop() {
            let succ = MSStopOptimizer_StopPathNode::get_successor(&ptr, &*stops, minSkipped);
            if bestNode.borrow().lt_op(&ptr.borrow()) {
                let reached = ptr.borrow().reachedPrio;
                let candidate = totalPrio - reached;
                minSkipped = if minSkipped < candidate { minSkipped } else { candidate }; // MIN2
                bestNode = ptr.clone();
            }
            if let Some(succ) = succ {
                if minSkipped == 0.0 && ptr.borrow().trackChanges > bestNode.borrow().trackChanges {
                    // pruned by minTC
                    continue;
                }
                let key = ptr.borrow().edge;
                match bestIntermediate.get(&key).cloned() {
                    None => {
                        bestIntermediate.insert(key, ptr.clone());
                    }
                    Some(best) => {
                        assert!(best.borrow().checked);
                        assert!(ptr.borrow().checked);
                        if !Rc::ptr_eq(&best, &ptr) {
                            if !best.borrow().lt_op(&ptr.borrow()) {
                                // skip: existing best intermediate for this edge is not worse
                                continue;
                            }
                            bestIntermediate.insert(key, ptr.clone());
                        }
                    }
                }
                queue.push(ptr);
                queue.push(succ);
            }
        }
        for stop in stops.iter_mut() {
            // init true and later set false for all used stops
            if !stop.skipped {
                stop.backtracked = true;
                stop.skipped = true;
            }
        }
        let mut bestEdges: ConstMSEdgeVector = Vec::new();
        if bestNode.borrow().stopIndex < 0 {
            // all stops were skipped. End route on current edge
            bestEdges.push(source);
        }
        let mut cur: Option<Rc<RefCell<MSStopOptimizer_StopPathNode>>> = Some(bestNode.clone());
        while let Some(node) = cur.clone() {
            let mut n = node.borrow_mut();
            if n.stopIndex < 0 {
                break;
            }
            let idx = n.stopIndex as usize;
            stops[idx].skipped = false;
            stops[idx].backtracked = false;
            stops[idx].origEdge = n.origEdge;
            if !bestEdges.is_empty() && !n.edges.is_empty() {
                n.edges.pop();
            }
            bestEdges.splice(0..0, n.edges.iter().cloned());
            cur = n.prev.clone();
        }
        bestEdges
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
        let vehicle = self.myVehicle.cast::<SUMOVehicle>();
        self.myRouter.m().compute_2(from, fromPos, to, toPos, vehicle, self.myT, into, true);
        if into.len() > 0 {
            *cost = self.myRouter.recompute_costs_pos(&*into, vehicle, fromPos, toPos, self.myT, Ptr::default());
            //std::cout << " from=" << from->getID() << " fromPos=" << fromPos << " to=" << to->getID() << " toPos=" << toPos << " t=" << myT << " cost=" << cost << " arrival=" << arrival << " maxDelay=" << myMaxDelay << "\n";
            if self.myT + time2steps(*cost) <= arrival + self.myMaxDelay {
                return true;
            }
        }
        false
    }

}

/// information used during skip/alternative optimization
/// C++ class `MSStopOptimizer::StopPathNode` (microsim/MSStopOptimizer.h)
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
    /// C++ `std::shared_ptr<StopPathNode> prev = nullptr`; None models nullptr.
    pub prev: Option<Rc<RefCell<MSStopOptimizer_StopPathNode>>>,
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
        let mut base = MSBaseVehicle_StopEdgeInfo::new(o.edge, o.priority, o.arrival, o.pos, false);
        base.nameTag = o.nameTag.clone();
        base.delay = o.delay;
        base.routeIndex = o.routeIndex;
        Self {
            base,
            base2: (),
            so: Ptr::new(_so),
            skippedPrio: 0.0,
            reachedPrio: 0.0,
            reachedMandatory: 0,
            trackChanges: 0,
            cost: 0.0,
            stopIndex: 0,
            numSkipped: 0,
            altIndex: -1,
            checked: false,
            edges: Vec::new(),
            prev: None,
        }
    }

    /// C++: `std::shared_ptr<StopPathNode> StopPathNode::getSuccessor(const std::vector<StopEdgeInfo> &, double)`
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    /// Takes the shared handle because C++ `shared_from_this()` is needed for `succ->prev`.
    #[allow(unused_variables)]
    pub fn get_successor(this: &Rc<RefCell<MSStopOptimizer_StopPathNode>>, stops: &Vec<StopEdgeInfo>, minSkipped: f64) -> Option<Rc<RefCell<MSStopOptimizer_StopPathNode>>> {
        let mut n = this.borrow_mut();
        if !n.checked {
            n.checked = true;
            // @todo caching, bulk-routing
            let mut newCost: f64 = 0.0;
            let prevRc = n.prev.clone().expect("StopPathNode::getSuccessor called on unchecked node without prev");
            let (prevEdge, prevPos) = {
                let p = prevRc.borrow();
                (p.edge, p.pos)
            };
            let (edge, pos, arrival, so) = (n.edge, n.pos, n.arrival, n.so);
            let mut into = std::mem::take(&mut n.edges);
            let reachable = so.reachable_in_time(prevEdge, prevPos, edge, pos, arrival, &mut into, &mut newCost);
            n.edges = into;
            if !reachable {
                // indicate failure
                n.reachedMandatory = -1;
                return None;
            }
            let priority = n.priority;
            n.reachedPrio += if 0.0 > priority { 0.0 } else { priority }; // MAX2(0.0, priority)
            n.cost += newCost;
            if priority < 0.0 {
                n.reachedMandatory += 1;
            }
        }
        let mut nextIndex: i32 = n.stopIndex + n.numSkipped + 1;
        while nextIndex < stops.len() as i32 {
            let next: &StopEdgeInfo = &stops[nextIndex as usize];
            // always try the default track first
            if n.altIndex == -1 {
                n.altIndex += 1;
                let mut succ = MSStopOptimizer_StopPathNode::new(&*n.so, next);
                succ.skippedPrio = n.skippedPrio;
                succ.reachedPrio = n.reachedPrio;
                succ.reachedMandatory = n.reachedMandatory;
                succ.trackChanges = n.trackChanges;
                succ.cost = n.cost;
                succ.prev = Some(this.clone());
                succ.stopIndex = nextIndex;
                return Some(Rc::new(RefCell::new(succ)));
            }
            let net = MSNet::get_instance();
            let alternatives = net.get_stopping_place_alternatives(&next.nameTag.0, next.nameTag.1);
            if next.priority < 0.0 {
                // next stop can neither be skipped nor changed
                return None;
            }
            while n.altIndex < alternatives.len() as i32 {
                let alt: Ptr<MSStoppingPlace> = alternatives[n.altIndex as usize];
                let altEdge: Ptr<MSEdge> = Ptr::from_mut(alt.get_lane().get_edge());
                n.altIndex += 1;
                if altEdge == next.edge || !alt.get_lane().allows_vehicle_class(n.so.myVehicle.get_v_class()) {
                    continue;
                }
                let mut succ = MSStopOptimizer_StopPathNode::new(&*n.so, next);
                succ.trackChanges = n.trackChanges;
                succ.pos = alt.get_end_lane_position();
                succ.skippedPrio = n.skippedPrio;
                succ.reachedPrio = n.reachedPrio;
                succ.reachedMandatory = n.reachedMandatory;
                succ.cost = n.cost;
                succ.prev = Some(this.clone());
                succ.stopIndex = nextIndex;
                succ.origEdge = succ.edge;
                succ.edge = altEdge;
                succ.trackChanges += 1;
                return Some(Rc::new(RefCell::new(succ)));
            }
            n.skippedPrio += next.priority;
            if n.skippedPrio >= minSkipped {
                // cannot improve on current best solution
                return None;
            }
            nextIndex += 1;
            n.altIndex = -1;
            n.numSkipped += 1;
        }
        None
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
        if self.reachedMandatory == b.reachedMandatory {
            if self.reachedPrio == b.reachedPrio {
                if self.trackChanges == b.trackChanges {
                    return self.cost > b.cost;
                }
                return self.trackChanges > b.trackChanges;
            }
            return self.reachedPrio < b.reachedPrio;
        }
        self.reachedMandatory < b.reachedMandatory
    }

}

/// Mirrors `std::priority_queue<shared_ptr<StopPathNode>, vector, spnCompare>` using the libstdc++
/// push_heap/pop_heap sift order, so nodes that compare equal pop in the same order as in C++.
pub struct MSStopOptimizer_StopPathQueue {
    v: Vec<Rc<RefCell<MSStopOptimizer_StopPathNode>>>,
}

impl MSStopOptimizer_StopPathQueue {
    pub fn new() -> Self {
        Self { v: Vec::new() }
    }

    pub fn push(&mut self, x: Rc<RefCell<MSStopOptimizer_StopPathNode>>) {
        self.v.push(x.clone());
        let n = self.v.len() as i64;
        Self::push_heap(&mut self.v, n - 1, 0, x);
    }

    pub fn pop(&mut self) -> Option<Rc<RefCell<MSStopOptimizer_StopPathNode>>> {
        let len = self.v.len() as i64;
        if len == 0 {
            return None;
        }
        // libstdc++ pop_heap: move back into the hole at the top, sift the old back down, then pop_back
        let value = self.v[(len - 1) as usize].clone();
        self.v[(len - 1) as usize] = self.v[0].clone();
        Self::adjust_heap(&mut self.v, 0, len - 1, value);
        self.v.pop()
    }

    fn push_heap(v: &mut Vec<Rc<RefCell<MSStopOptimizer_StopPathNode>>>, mut hole: i64, top: i64, value: Rc<RefCell<MSStopOptimizer_StopPathNode>>) {
        let cmp = MSStopOptimizer_spnCompare {};
        let mut parent = (hole - 1) / 2;
        while hole > top && cmp.call_op(&v[parent as usize], &value) {
            v[hole as usize] = v[parent as usize].clone();
            hole = parent;
            parent = (hole - 1) / 2;
        }
        v[hole as usize] = value;
    }

    fn adjust_heap(v: &mut Vec<Rc<RefCell<MSStopOptimizer_StopPathNode>>>, mut hole: i64, len: i64, value: Rc<RefCell<MSStopOptimizer_StopPathNode>>) {
        let cmp = MSStopOptimizer_spnCompare {};
        let top = hole;
        let mut second = hole;
        while second < (len - 1) / 2 {
            second = 2 * (second + 1);
            if cmp.call_op(&v[second as usize], &v[(second - 1) as usize]) {
                second -= 1;
            }
            v[hole as usize] = v[second as usize].clone();
            hole = second;
        }
        if (len & 1) == 0 && second == (len - 2) / 2 {
            second = 2 * (second + 1);
            v[hole as usize] = v[(second - 1) as usize].clone();
            hole = second - 1;
        }
        Self::push_heap(v, hole, top, value);
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
    pub fn call_op(&self, a: &Rc<RefCell<MSStopOptimizer_StopPathNode>>, b: &Rc<RefCell<MSStopOptimizer_StopPathNode>>) -> bool {
        let a = a.borrow();
        let b = b.borrow();
        if a.skippedPrio == b.skippedPrio {
            if a.stopIndex == b.stopIndex {
                return a.lt_op(&*b);
            }
            return a.stopIndex < b.stopIndex;
        }
        a.skippedPrio > b.skippedPrio
    }

}

