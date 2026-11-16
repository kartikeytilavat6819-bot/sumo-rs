//! Port of utils/router/CCHGraph.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::RouterEdge;

/// C++ typedef `double (*)(const E *const, const V *const, double) CCHGraph::EffortOperation`
pub type CCHGraph_EffortOperation = Unmapped /* double (*)(const E *const, const V *const, double) */;

/// Metric-independent RoutingKit CCH topology over the PURE road graph.
/// C++ class `CCHGraph` (utils/router/CCHGraph.h)
pub struct CCHGraph<E, V> {
    /// node index -> backing edge
    pub myNodeToEdge: Vec<Ptr<E>>,
    /// edge numerical id -> node index (INVALID_NODE if not a node)
    pub myEdgeToNode: Vec<u32>,
    /// per-arc endpoints
    pub myArcTail: Vec<u32>,
    pub myArcHead: Vec<u32>,
    /// per-arc leading via/internal edge (nullptr if none) for path re-expansion
    pub myArcVia: Vec<Ptr<E>>,
    /// (tail node, head node) -> arc index
    pub myArcOf: HashMap<(u32, u32), u32>,
    /// per-arc CONNECTION-level permission bitmask, accumulated per primed class (see primeClassMask); mutable lazy cache
    pub myArcPerm: Vec<SVCPermissions>,
    /// the classes already primed into myArcPerm
    pub myPrimedClasses: std::cell::Cell<SVCPermissions>,
    /// edge numerical id -> arcs whose weight reads that edge (head + folded via edges); the reverse image of computeArcWeightRaw's inputs
    pub myEdgeToArcs: Vec<Vec<u32>>,
    /// TAZ-source connector edge -> its entry-edge road node ids
    pub myTazSrcNodes: HashMap<Ptr<E>, Vec<u32>>,
    /// TAZ-sink connector edge -> its exit-edge road node ids
    pub myTazSnkNodes: HashMap<Ptr<E>, Vec<u32>>,
    /// the immutable hierarchy
    pub myCCH: CustomizableContractionHierarchy,
    _p: std::marker::PhantomData<(E, V,)>,
}

/// C++ `static const std::vector<unsigned> empty` (local statics of tazSources/tazSinks/arcsOfEdge)
static EMPTY_NODES: Vec<u32> = Vec::new();

// C++ static members of CCHGraph (port as module-level statics, see AGENTS.md):
//   static const unsigned int CCHGraph::INVALID_NODE;
const INVALID_NODE: u32 = foreign::routingkit::constants::INVALID_ID;

impl<E: RouterEdge<V>, V> CCHGraph<E, V> {
    /// C++: `void CCHGraph::CCHGraph<E, V>(const int &)`
    /// Build the union line graph + CCH from the given edges.
    // explicit CCHGraph(const std::vector<E*>& allEdges) {
    //         // 1. dense node indexing over real (non-internal, non-taz) edges.
    //         unsigned maxNumID = 0;
    //         for (const E* e : allEdges) {
    //             maxNumID = MAX2(maxNumID, (unsigned)e->getNumericalID());
    //         }
    //         myEdgeToNode.assign(maxNumID + 1, INVALID_NODE);
    //         for (const E* e : allEdges) {
    //             // excluded are junction-internal edges and the district STAR
    //             // connectors ("<taz>-source"/"-sink", the high-degree phantom
    //             // edges -- see the class documentation). Legacy net edges with
    //             // function="connector" are ordinary routable geometry for the
    //             // exact routers and therefore stay ordinary nodes here; only the
    //             // paired district connectors (identified by their
    //             // otherTazConnector link) are kept out of the hierarchy.
    //             if (e->isInternal() || isStarConnector(e)) {
    //                 continue;
    //             }
    //             const unsigned node = (unsigned)myNodeToEdge.size();
    //             myEdgeToNode[e->getNumericalID()] = node;
    //             myNodeToEdge.push_back(e);
    //         }
    //         const unsigned nNodes = (unsigned)myNodeToEdge.size();
    //
    //         // 2. arcs from the union successors; coordinates for the orderer.
    //         std::vector<float> lon(nNodes);
    //         std::vector<float> lat(nNodes);
    //         for (unsigned n = 0; n < nNodes; n++) {
    //             const E* e = myNodeToEdge[n];
    //             Position p;
    //             if (!e->getLanes().empty()) {
    //                 p = e->getLanes()[0]->geometryPositionAtOffset(e->getLength() * 0.5);
    //             } else {
    //                 const auto* j = e->getToJunction() != nullptr ? e->getToJunction() : e->getFromJunction();
    //                 p = (j != nullptr) ? j->getPosition() : Position(0., 0.);
    //             }
    //             if (GeoConvHelper::getFinal().usingGeoProjection()) {
    //                 GeoConvHelper::getFinal().cartesian2geo(p);
    //             }
    //             lon[n] = (float)p.x();
    //             lat[n] = (float)p.y();
    //         }
    //         for (unsigned n = 0; n < nNodes; n++) {
    //             const E* u = myNodeToEdge[n];
    //             for (const auto& follower : u->getViaSuccessors(SVC_IGNORING)) {
    //                 const E* to = follower.first;
    //                 if (to == nullptr || to->getNumericalID() >= (int)myEdgeToNode.size()) {
    //                     continue;
    //                 }
    //                 const unsigned toNode = myEdgeToNode[to->getNumericalID()];
    //                 if (toNode == INVALID_NODE) {
    //                     continue;  // successor is internal/taz -> not a routable node
    //                 }
    //                 myArcOf[std::make_pair(n, toNode)] = (unsigned)myArcTail.size();
    //                 myArcTail.push_back(n);
    //                 myArcHead.push_back(toNode);
    //                 myArcVia.push_back(follower.second);  // leading internal edge or nullptr
    //             }
    //         }
    //         myArcPerm.assign(myArcTail.size(), 0);
    //         myPrimedClasses = 0;
    //         // 2b. reverse image for sparse re-customization: for every arc, the
    //         // edges its weight reads at fill time (head edge + folded via chain;
    //         // the tail edge is deliberately absent from the weight, see the arc
    //         // weight convention above)
    //         myEdgeToArcs.resize(myEdgeToNode.size());
    //         for (unsigned a = 0; a < (unsigned)myArcHead.size(); a++) {
    //             myEdgeToArcs[myNodeToEdge[myArcHead[a]]->getNumericalID()].push_back(a);
    //             const E* via = myArcVia[a];
    //             while (via != nullptr && via->isInternal()) {
    //                 if (via->getNumericalID() >= 0 && via->getNumericalID() < (int)myEdgeToArcs.size()) {
    //                     myEdgeToArcs[via->getNumericalID()].push_back(a);
    //                 }
    //                 const auto& vs = via->getViaSuccessors();
    //                 via = vs.empty() ? nullptr : vs.front().second;
    //             }
    //         }
    //
    //         // 3. TAZ member sets (query-time "phantom nodes"): successors for a
    //         // source connector (entry edges), predecessors for a sink connector.
    //         for (const E* e : allEdges) {
    //             if (!isStarConnector(e)) {
    //                 continue;
    //             }
    //             std::vector<unsigned> srcNodes;
    //             for (const auto& follower : e->getViaSuccessors(SVC_IGNORING)) {
    //                 const unsigned m = nodeOf(follower.first);
    //                 if (m != INVALID_NODE) {
    //                     srcNodes.push_back(m);
    //                 }
    //             }
    //             std::vector<unsigned> snkNodes;
    //             for (const E* pred : e->getPredecessors()) {
    //                 const unsigned m = nodeOf(pred);
    //                 if (m != INVALID_NODE) {
    //                     snkNodes.push_back(m);
    //                 }
    //             }
    //             if (!srcNodes.empty()) {
    //                 myTazSrcNodes[e] = std::move(srcNodes);
    //             }
    //             if (!snkNodes.empty()) {
    //                 myTazSnkNodes[e] = std::move(snkNodes);
    //             }
    //         }
    //
    //         // 4. metric-independent order + CCH topology (the one-time work).
    //         // @todo persist the order keyed by a network hash (recompute is fast)
    //         std::vector<unsigned> order = RoutingKit::compute_nested_node_dissection_order_using_inertial_flow(
    //                                           nNodes, myArcTail, myArcHead, lat, lon);
    //         myCCH = RoutingKit::CustomizableContractionHierarchy(order, myArcTail, myArcHead);
    // #ifdef CCH_DEBUG
    //         std::cout << "CCH: " << nNodes << " road nodes, "
    //                   << arcCount() << " arcs, "
    //                   << myCCH.cch_arc_count() << " cch-arcs (fill x" << (double)myCCH.cch_arc_count() / MAX2((unsigned)1, arcCount())
    //                   << "), " << myTazSrcNodes.size() << " TAZ sources, " << myTazSnkNodes.size() << " TAZ sinks." << std::endl;
    // #endif
    //     }
    #[allow(unused_variables)]
    pub fn new(allEdges: &Vec<Ptr<E>>) -> Self {
        // shortcut: needs E::getOtherTazConnector (for isStarConnector) and L::geometryPositionAtOffset plus junction getPosition (lane/junction geometry), none in RouterEdge
        panic!("shortcut: CCHGraph::new needs E::getOtherTazConnector and junction/lane geometry, not in RouterEdge")
    }

    /// C++: `void CCHGraph::~CCHGraph<E, V>()` [virtual]
    // virtual ~CCHGraph() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {}

    /// C++: `const int & CCHGraph::cch() const` [const]
    /// the immutable CCH (share const& across clones)
    // const RoutingKit::CustomizableContractionHierarchy& cch() const {
    //         return myCCH;
    //     }
    #[allow(unused_variables)]
    pub fn cch(&self) -> &CustomizableContractionHierarchy {
        &self.myCCH
    }

    /// C++: `unsigned int CCHGraph::arcCount() const` [const]
    /// number of input arcs (== number of mapped connections)
    // unsigned arcCount() const {
    //         return (unsigned)myArcTail.size();
    //     }
    #[allow(unused_variables)]
    pub fn arc_count(&self) -> u32 {
        self.myArcTail.len() as u32
    }

    /// C++: `unsigned int CCHGraph::edgeIdSpace() const` [const]
    /// size of the edge numerical-id space the graph was built over (for callers keeping per-edge-id side arrays, e.g. CCHMetricFamily)
    // unsigned edgeIdSpace() const {
    //         return (unsigned)myEdgeToNode.size();
    //     }
    #[allow(unused_variables)]
    pub fn edge_id_space(&self) -> u32 {
        self.myEdgeToNode.len() as u32
    }

    /// C++: `unsigned int CCHGraph::nodeOf(const E *) const` [const]
    /// RoutingKit node index for an edge, or INVALID_NODE if not a node
    // unsigned nodeOf(const E* e) const {
    //         if (e == nullptr || e->getNumericalID() < 0 || e->getNumericalID() >= (int)myEdgeToNode.size()) {
    //             return INVALID_NODE;
    //         }
    //         return myEdgeToNode[e->getNumericalID()];
    //     }
    #[allow(unused_variables)]
    pub fn node_of(&self, e: Ptr<E>) -> u32 {
        if e.is_null() || e.numerical_id() < 0 || e.numerical_id() as usize >= self.myEdgeToNode.len() {
            return INVALID_NODE;
        }
        self.myEdgeToNode[e.numerical_id() as usize]
    }

    /// C++: `const E * CCHGraph::edgeOf(unsigned int) const` [const]
    /// the edge backing a RoutingKit node
    // const E* edgeOf(unsigned node) const {
    //         return myNodeToEdge[node];
    //     }
    #[allow(unused_variables)]
    pub fn edge_of(&self, node: u32) -> Ptr<E> {
        self.myNodeToEdge[node as usize]
    }

    /// C++: `const int & CCHGraph::tazSources(const E *) const` [const]
    /// road member nodes of a TAZ-source connector (its entry edges), to seed multi-SOURCE queries; empty if taz is not a known TAZ source.
    // const std::vector<unsigned>& tazSources(const E* taz) const {
    //         static const std::vector<unsigned> empty;
    //         const auto it = myTazSrcNodes.find(taz);
    //         return it == myTazSrcNodes.end() ? empty : it->second;
    //     }
    #[allow(unused_variables)]
    pub fn taz_sources(&self, taz: Ptr<E>) -> &Vec<u32> {
        self.myTazSrcNodes.get(&taz).unwrap_or(&EMPTY_NODES)
    }

    /// C++: `const int & CCHGraph::tazSinks(const E *) const` [const]
    /// road member nodes of a TAZ-sink connector (its exit edges), to seed multi-TARGET queries; empty if taz is not a known TAZ sink.
    // const std::vector<unsigned>& tazSinks(const E* taz) const {
    //         static const std::vector<unsigned> empty;
    //         const auto it = myTazSnkNodes.find(taz);
    //         return it == myTazSnkNodes.end() ? empty : it->second;
    //     }
    #[allow(unused_variables)]
    pub fn taz_sinks(&self, taz: Ptr<E>) -> &Vec<u32> {
        self.myTazSnkNodes.get(&taz).unwrap_or(&EMPTY_NODES)
    }

    /// C++: `const int & CCHGraph::arcsOfEdge(const E *) const` [const]
    /// the arcs whose weight depends on the given edge (arcs it heads plus arcs whose folded via chain contains it), for sparse re-customization; empty for edges that are not part of the graph
    // const std::vector<unsigned>& arcsOfEdge(const E* e) const {
    //         static const std::vector<unsigned> empty;
    //         if (e == nullptr || e->getNumericalID() < 0 || e->getNumericalID() >= (int)myEdgeToArcs.size()) {
    //             return empty;
    //         }
    //         return myEdgeToArcs[e->getNumericalID()];
    //     }
    #[allow(unused_variables)]
    pub fn arcs_of_edge(&self, e: Ptr<E>) -> &Vec<u32> {
        if e.is_null() || e.numerical_id() < 0 || e.numerical_id() as usize >= self.myEdgeToArcs.len() {
            return &EMPTY_NODES;
        }
        &self.myEdgeToArcs[e.numerical_id() as usize]
    }

    /// C++: `void CCHGraph::expandNodePath(const int &, int &) const` [const]
    /// Map a RoutingKit node path back to the edge sequence.
    // void expandNodePath(const std::vector<unsigned>& nodePath,
    //                         std::vector<const E*>& into) const {
    //         into.reserve(into.size() + nodePath.size());
    //         for (unsigned node : nodePath) {
    //             into.push_back(myNodeToEdge[node]);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn expand_node_path(&self, nodePath: &Vec<u32>, into: &mut Vec<Ptr<E>>) {
        into.reserve(into.len() + nodePath.len());
        for &node in nodePath {
            into.push(self.myNodeToEdge[node as usize]);
        }
    }

    /// C++: `double CCHGraph::viaChainEffort(const E *, EffortOperation, const V *, double)` [static]
    /// effort accumulated crossing the internal/via chain that leads from one real edge onto its successor (mirrors SUMOAbstractRouter::updateViaEdgeCost).
    // static double viaChainEffort(const E* via, EffortOperation effort,
    //                                  const V* veh, double time) {
    //         double sum = 0.;
    //         while (via != nullptr && via->isInternal()) {
    //             sum += effort(via, veh, time);
    //             const auto& vs = via->getViaSuccessors();
    //             via = vs.empty() ? nullptr : vs.front().second;
    //         }
    //         return sum;
    //     }
    #[allow(unused_variables)]
    pub fn via_chain_effort(via: Ptr<E>, effort: CCHGraph_EffortOperation, veh: Ptr<V>, time: f64) -> f64 {
        // shortcut: CCHGraph_EffortOperation is Unmapped, so effort(via, veh, time) cannot be called; the alias needs E,V params (rocch_metrics.rs outside this batch uses it bare)
        panic!("shortcut: CCHGraph::viaChainEffort needs typed EffortOperation")
    }

    /// C++: `unsigned int CCHGraph::computeArcWeight(unsigned int, EffortOperation, SUMOVehicleClass, const V *, double) const` [const]
    /// Recompute the input weight of one arc -- the exact per-arc body of fillInputWeights (same masking, folding and rounding), exposed so a sparse update can refresh only the arcs of edges that actually moved.
    // unsigned computeArcWeight(unsigned a, EffortOperation effort, SUMOVehicleClass maskClass,
    //                               const V* veh, double time) const {
    //         if (maskClass != SVC_IGNORING
    //                 && (((myArcPerm[a] & maskClass) == 0)
    //                     || ((edgeOf(myArcHead[a])->getPermissions() & maskClass) == 0))) {
    //             return RoutingKit::inf_weight;
    //         }
    //         return computeArcWeightRaw(a, effort, veh, time);
    //     }
    #[allow(unused_variables)]
    pub fn compute_arc_weight(&self, a: u32, effort: CCHGraph_EffortOperation, maskClass: SUMOVehicleClass, veh: Ptr<V>, time: f64) -> u32 {
        if maskClass != utils::common::sumo_vehicle_class::SVC_IGNORING
                && (((self.myArcPerm[a as usize] & maskClass) == 0)
                    || ({
                        // shortcut: E::getPermissions (edge-level permissions) is not in RouterEdge, add it to the trait to upgrade
                        let perm: SVCPermissions = panic!("shortcut: CCHGraph::computeArcWeight needs E::getPermissions, not in RouterEdge");
                        (perm & maskClass) == 0
                    }))
        {
            return foreign::routingkit::constants::INF_WEIGHT;
        }
        return self.compute_arc_weight_raw(a, effort, veh, time);
    }

    /// C++: `void CCHGraph::fillInputWeights(EffortOperation, SUMOVehicleClass, const V *, double, int &) const` [const]
    /// Fill a centisecond input-weight buffer for one vehicle class.
    // void fillInputWeights(EffortOperation effort, SUMOVehicleClass maskClass,
    //                           const V* veh, double time,
    //                           std::vector<unsigned>& weight) const {
    //         primeClassMask(maskClass);
    //         weight.resize(arcCount());
    //         for (unsigned a = 0; a < arcCount(); a++) {
    //             weight[a] = computeArcWeight(a, effort, maskClass, veh, time);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn fill_input_weights(&self, effort: CCHGraph_EffortOperation, maskClass: SUMOVehicleClass, veh: Ptr<V>, time: f64, weight: &mut Vec<u32>) {
        self.prime_class_mask(maskClass);
        weight.resize(self.arc_count() as usize, 0);
        for a in 0..self.arc_count() {
            weight[a as usize] = self.compute_arc_weight(a, effort, maskClass, veh, time);
        }
    }

    /// C++: `void CCHGraph::invalidateClassMasks() const` [const]
    /// Drop every primed connection mask so the next fill re-primes from the CURRENT successor lists.
    // void invalidateClassMasks() const {
    //         myArcPerm.assign(myArcPerm.size(), 0);
    //         myPrimedClasses = 0;
    //     }
    #[allow(unused_variables)]
    pub fn invalidate_class_masks(&self) {
        Ptr::new(self).m().myArcPerm.fill(0);
        self.myPrimedClasses.set(0);
    }

    /// C++: `bool CCHGraph::isStarConnector(const E *)` [static, private]
    /// whether the edge is a district star connector (the paired "<taz>-source"/"-sink" phantom edge) as opposed to a legacy function="connector" net edge, which stays routable
    // static bool isStarConnector(const E* e) {
    //         return e->isTazConnector() && e->getOtherTazConnector() != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn is_star_connector(e: Ptr<E>) -> bool {
        // shortcut: E::getOtherTazConnector is not in RouterEdge, add it to the trait to upgrade
        panic!("shortcut: CCHGraph::isStarConnector needs E::getOtherTazConnector, not in RouterEdge")
    }

    /// C++: `void CCHGraph::primeClassMask(SUMOVehicleClass) const` [const, private]
    /// Record which arcs vClass may traverse in the per-arc CONNECTION-level permission bitmask -- exactly the arcs getViaSuccessors(vClass) yields (edge-level getPermissions() is too coarse on mixed-lane edges). No-op if the class was primed before.
    // void primeClassMask(SUMOVehicleClass vClass) const {
    //         if (vClass == SVC_IGNORING || (myPrimedClasses & vClass) == vClass) {
    //             return;
    //         }
    //         for (unsigned n = 0; n < (unsigned)myNodeToEdge.size(); n++) {
    //             for (const auto& follower : myNodeToEdge[n]->getViaSuccessors(vClass)) {
    //                 const unsigned toNode = nodeOf(follower.first);
    //                 if (toNode == INVALID_NODE) {
    //                     continue;
    //                 }
    //                 const auto it = myArcOf.find(std::make_pair(n, toNode));
    //                 if (it != myArcOf.end()) {
    //                     myArcPerm[it->second] |= (SVCPermissions)vClass;
    //                 }
    //             }
    //         }
    //         myPrimedClasses |= (SVCPermissions)vClass;
    //     }
    #[allow(unused_variables)]
    pub fn prime_class_mask(&self, vClass: SUMOVehicleClass) {
        if vClass == utils::common::sumo_vehicle_class::SVC_IGNORING || (self.myPrimedClasses.get() & vClass) == vClass {
            return;
        }
        let this = Ptr::new(self);
        for n in 0..self.myNodeToEdge.len() as u32 {
            for follower in self.myNodeToEdge[n as usize].via_successors(vClass, false) {
                let toNode = self.node_of(follower.0);
                if toNode == INVALID_NODE {
                    continue;
                }
                if let Some(&a) = self.myArcOf.get(&(n, toNode)) {
                    this.m().myArcPerm[a as usize] |= vClass;
                }
            }
        }
        self.myPrimedClasses.set(self.myPrimedClasses.get() | vClass);
    }

    /// C++: `unsigned int CCHGraph::computeArcWeightRaw(unsigned int, EffortOperation, const V *, double) const` [const, private]
    /// The unmasked weight of one arc: via-chain effort + head-edge effort, rounded to centiseconds and clamped below inf_weight (NaN / inf / overflow become inf_weight = forbidden).
    // unsigned computeArcWeightRaw(unsigned a, EffortOperation effort,
    //                                  const V* veh, double time) const {
    //         const E* to = myNodeToEdge[myArcHead[a]];
    //         const double bigEffort = (double)(RoutingKit::inf_weight - 1) / 100.0;
    //         const double eff = viaChainEffort(myArcVia[a], effort, veh, time)
    //                            + effort(to, veh, time);
    //         if (!(eff < bigEffort)) {
    //             return RoutingKit::inf_weight;
    //         }
    //         const long long cs = std::llround(eff * 100.0);
    //         return (unsigned)(cs < 0 ? 0 : (cs >= (long long)RoutingKit::inf_weight
    //                                         ? RoutingKit::inf_weight - 1 : cs));
    //     }
    #[allow(unused_variables)]
    pub fn compute_arc_weight_raw(&self, a: u32, effort: CCHGraph_EffortOperation, veh: Ptr<V>, time: f64) -> u32 {
        // shortcut: CCHGraph_EffortOperation is still an Unmapped placeholder (rocch_metrics uses the non-generic alias), so effort(to, veh, time) cannot be called; upgrade with the typedef
        panic!("shortcut: CCHGraph::computeArcWeightRaw needs typed EffortOperation")
    }

    /// C++: `void CCHGraph::CCHGraph<E, V>(const CCHGraph<E, V> &)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &CCHGraph<E, V>) -> Self {
        CCHGraph {
            myNodeToEdge: arg0.myNodeToEdge.clone(),
            myEdgeToNode: arg0.myEdgeToNode.clone(),
            myArcTail: arg0.myArcTail.clone(),
            myArcHead: arg0.myArcHead.clone(),
            myArcVia: arg0.myArcVia.clone(),
            myArcOf: arg0.myArcOf.clone(),
            myArcPerm: arg0.myArcPerm.clone(),
            myPrimedClasses: std::cell::Cell::new(arg0.myPrimedClasses.get()),
            myEdgeToArcs: arg0.myEdgeToArcs.clone(),
            myTazSrcNodes: arg0.myTazSrcNodes.clone(),
            myTazSnkNodes: arg0.myTazSnkNodes.clone(),
            myCCH: arg0.myCCH.clone(),
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `CCHGraph<E, V> & CCHGraph::operator=(const CCHGraph<E, V> &)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &CCHGraph<E, V>) -> &mut CCHGraph<E, V> {
        *self = CCHGraph::new_2(arg0);
        self
    }

}

