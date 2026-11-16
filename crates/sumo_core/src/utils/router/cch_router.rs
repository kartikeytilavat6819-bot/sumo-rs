//! Port of utils/router/CCHRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::RouterEdge;

/// C++ typedef `MetricPtr (*)(SUMOVehicleClass, SUMOTime, const V *) CCHRouter::MetricProvider`
pub type CCHRouter_MetricProvider<V> = fn(SUMOVehicleClass, SUMOTime, Ptr<V>) -> CCHRouter_MetricPtr;

/// C++ typedef `const int * CCHRouter::MetricPtr`
pub type CCHRouter_MetricPtr = Ptr<i32>;

/// C++ typedef `typename SUMOAbstractRouter<E, V>::Operation CCHRouter::Operation`
pub type CCHRouter_Operation = SUMOAbstractRouter_Operation;

/// C++ typedef `SUMOTime (*)(SUMOTime) CCHRouter::PeriodEnd`
pub type CCHRouter_PeriodEnd = Option<fn(SUMOTime) -> SUMOTime>;

/// C++ typedef `typename SUMOAbstractRouter<E, V>::Prohibitions CCHRouter::Prohibitions`
pub type CCHRouter_Prohibitions = SUMOAbstractRouter_Prohibitions;

/// C++ typedef `void (*)(const V *) CCHRouter::ResetHook`
pub type CCHRouter_ResetHook<V> = Option<fn(Ptr<V>)>;

/// Contraction-hierarchy router over the edge graph mapped by GRAPH.
/// C++ class `CCHRouter` (utils/router/CCHRouter.h)
#[repr(C)]
pub struct CCHRouter<E, V, GRAPH> {
    /// C++ base class `SUMOAbstractRouter<E, V>`
    pub base: SUMOAbstractRouter<E, V>,
    /// C++ `const GRAPH *`; GRAPH is always CCHGraph<E, V> in SUMO (ROCCHGraph / MSCCHGraph)
    pub myGraph: Ptr<CCHGraph<E, V>>,
    pub myMetricProvider: CCHRouter_MetricProvider<V>,
    pub myQuery: i32,
    pub myBoundMetric: CCHRouter_MetricPtr,
    pub myFallback: Ptr<SUMOAbstractRouter<E, V>>,
    pub myPeriodEnd: CCHRouter_PeriodEnd,
    pub myOnReset: CCHRouter_ResetHook<V>,
    pub myProhibitionActive: bool,
    pub myProhibited: SUMOAbstractRouter_Prohibitions,
    _p: std::marker::PhantomData<(E, V, GRAPH,)>,
}

impl<E, V, GRAPH> std::ops::Deref for CCHRouter<E, V, GRAPH> {
    type Target = SUMOAbstractRouter<E, V>;
    fn deref(&self) -> &SUMOAbstractRouter<E, V> {
        &self.base
    }
}
impl<E, V, GRAPH> std::ops::DerefMut for CCHRouter<E, V, GRAPH> {
    fn deref_mut(&mut self) -> &mut SUMOAbstractRouter<E, V> {
        &mut self.base
    }
}

impl<E: RouterEdge<V>, V, GRAPH> CCHRouter<E, V, GRAPH> {
    /// C++: `void CCHRouter::CCHRouter<E, V, GRAPH>(const GRAPH *, MetricProvider, Operation, const bool, SUMOAbstractRouter<E, V> *, PeriodEnd, ResetHook)`
    /// Constructor.
    // CCHRouter(const GRAPH* graph, MetricProvider provider,
    //               Operation operation, const bool unbuildIsWarning,
    //               SUMOAbstractRouter<E, V>* fallback,
    //               PeriodEnd periodEnd = nullptr, ResetHook onReset = nullptr) :
    //         SUMOAbstractRouter<E, V>("CCHRouter", unbuildIsWarning, operation, nullptr, false, false),
    //         myGraph(graph), myMetricProvider(provider), myFallback(fallback),
    //         myPeriodEnd(periodEnd), myOnReset(onReset), myProhibitionActive(false) {
    //     }
    #[allow(unused_variables)]
    pub fn new(graph: Ptr<CCHGraph<E, V>>, provider: CCHRouter_MetricProvider<V>, operation: SUMOAbstractRouter_Operation, unbuildIsWarning: bool, fallback: Ptr<SUMOAbstractRouter<E, V>>, periodEnd: CCHRouter_PeriodEnd, onReset: CCHRouter_ResetHook<V>) -> Self {
        // shortcut: myQuery is a placeholder i32 (CustomizableContractionHierarchyQuery borrows the metric, lifetime not yet modelled), so the struct cannot be built; upgrade with the typed query
        panic!("shortcut: CCHRouter::CCHRouter needs the typed CustomizableContractionHierarchyQuery for myQuery")
    }

    /// C++: `void CCHRouter::CCHRouter<E, V, GRAPH>(CCHRouter<E, V, GRAPH> *)`
    /// clone constructor: share graph + provider, clone the fallback, fresh query scratch
    // CCHRouter(CCHRouter* other) :
    //         SUMOAbstractRouter<E, V>(other),
    //         myGraph(other->myGraph), myMetricProvider(other->myMetricProvider),
    //         myFallback(other->myFallback->clone()), myPeriodEnd(other->myPeriodEnd),
    //         myOnReset(other->myOnReset),
    //         myProhibitionActive(other->myProhibitionActive), myProhibited(other->myProhibited) {
    //     }
    #[allow(unused_variables)]
    pub fn new_2(other: Ptr<CCHRouter<E, V, GRAPH>>) -> Self {
        // shortcut: needs the typed myQuery (see CCHRouter::new) and the base SUMOAbstractRouter copy constructor, which is not ported
        panic!("shortcut: CCHRouter clone ctor needs typed myQuery and the SUMOAbstractRouter copy ctor")
    }

    /// C++: `void CCHRouter::~CCHRouter<E, V, GRAPH>()` [virtual]
    // virtual ~CCHRouter() {
    //         delete myFallback;
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: assumes myFallback was allocated with Box (the C++ router owns it); callers that use Box::leak must not drop the router
        if !self.myFallback.is_null() {
            unsafe { drop(Box::from_raw(self.myFallback.as_raw())) };
        }
    }

    /// C++: `SUMOAbstractRouter<E, V> * CCHRouter::clone()` [virtual]
    // virtual SUMOAbstractRouter<E, V>* clone() {
    //         return new CCHRouter<E, V, GRAPH>(this);
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, V>> {
        // shortcut: depends on the clone constructor (CCHRouter::new_2), which is not ported yet
        panic!("shortcut: CCHRouter::clone needs CCHRouter::new_2")
    }

    /// C++: `void CCHRouter::reset(const V *const)` [virtual]
    /// the host's efforts changed (see ResetHook): notify the metric store and reset the fallback's caches. Must not run while another clone is mid-query; marouter resets once its worker threads have finished the iteration.
    // virtual void reset(const V* const vehicle) {
    //         if (myOnReset != nullptr) {
    //             myOnReset(vehicle);
    //         }
    //         myFallback->reset(vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn reset(&mut self, vehicle: Ptr<V>) {
        if let Some(hook) = self.myOnReset {
            hook(vehicle);
        }
        self.myFallback.m().reset(vehicle);
    }

    /// C++: `bool CCHRouter::compute(const E *, const E *, const V *const, SUMOTime, int &, bool)`
    // bool compute(const E* from, const E* to, const V* const vehicle,
    //                  SUMOTime msTime, std::vector<const E*>& into, bool silent = false) {
    //         // A CCH query can only avoid an edge the metric already encodes as inf:
    //         // RoutingKit's query has no per-arc blacklist, and a query-time skip is
    //         // unsound because shortcut arcs bake in shortest paths THROUGH the edge
    //         // (SUMO's own CHRouter refuses prohibitions for the same reason). So we
    //         // run CCH when the metric can express the request and fall back to the
    //         // exact A* router otherwise: (a) no metric for this class, or (b) an
    //         // active prohibition that is NOT already a live permission-closure this
    //         // class's metric encodes (prohibitionsCoveredByMetric).
    //         const SUMOVehicleClass vClass = vehicle == nullptr ? SVC_PASSENGER : vehicle->getVClass();
    //         // A vehicle that routes on the ORIGINAL permissions (routing mode bit
    //         // ROUTING_MODE_IGNORE_TRANSIENT_PERMISSIONS) cannot use the shared
    //         // metric, which bakes in the LIVE permissions, transient closures
    //         // included -- route it exactly, like Dijkstra/A* do natively.
    //         if (vehicle != nullptr && vehicle->ignoreTransientPermissions()) {
    //             return myFallback->compute(from, to, vehicle, msTime, into, silent);
    //         }
    //         MetricPtr metric = myMetricProvider(vClass, msTime, vehicle);
    //         if (metric == nullptr || (myProhibitionActive && !prohibitionsCoveredByMetric(vClass))) {
    //             return myFallback->compute(from, to, vehicle, msTime, into, silent);
    //         }
    //         // Endpoints -> node SETS: a district star connector expands to its
    //         // member edges (phantom-node seeding); everything that is a graph
    //         // node -- including legacy function="connector" net edges -- routes
    //         // as a single node. TAZ member edges are the class-union set, so
    //         // filter to edges this class may enter.
    //         const bool fromTaz = from->isTazConnector() && myGraph->nodeOf(from) == GRAPH::INVALID_NODE;
    //         const bool toTaz = to->isTazConnector() && myGraph->nodeOf(to) == GRAPH::INVALID_NODE;
    //         // Mirror the exact routers' endpoint rejection (AStarRouter::compute):
    //         // an endpoint the vehicle may not use fails loudly instead of quietly
    //         // seeding a query on it. The metric masks only arcs BETWEEN nodes --
    //         // the source edge's own closure never enters it (its effort is added
    //         // as a plain source seed), so without this check a query from a
    //         // hard-closed edge would "succeed". TAZ members are filtered below.
    //         if (vehicle != nullptr) {
    //             if (!fromTaz && from->prohibits(vehicle)) {
    //                 if (!silent && this->myErrorMsgHandler != nullptr) {
    //                     this->myErrorMsgHandler->inform("Vehicle '" + Named::getIDSecure(vehicle) + "' is not allowed on source edge '" + from->getID() + "'.");
    //                 }
    //                 return false;
    //             }
    //             if (!toTaz && to->prohibits(vehicle)) {
    //                 if (!silent && this->myErrorMsgHandler != nullptr) {
    //                     this->myErrorMsgHandler->inform("Vehicle '" + Named::getIDSecure(vehicle) + "' is not allowed on destination edge '" + to->getID() + "'.");
    //                 }
    //                 return false;
    //             }
    //         }
    //         std::vector<unsigned> srcBuf, tgtBuf;
    //         const std::vector<unsigned>* sources;
    //         const std::vector<unsigned>* targets;
    //         if (fromTaz) {
    //             for (const unsigned m : myGraph->tazSources(from)) {
    //                 if ((myGraph->edgeOf(m)->getPermissions() & vClass) != 0) {
    //                     srcBuf.push_back(m);
    //                 }
    //             }
    //             sources = &srcBuf;
    //         } else {
    //             const unsigned s = myGraph->nodeOf(from);
    //             if (s != GRAPH::INVALID_NODE) {
    //                 srcBuf.push_back(s);
    //             }
    //             sources = &srcBuf;
    //         }
    //         if (toTaz) {
    //             for (const unsigned m : myGraph->tazSinks(to)) {
    //                 if ((myGraph->edgeOf(m)->getPermissions() & vClass) != 0) {
    //                     tgtBuf.push_back(m);
    //                 }
    //             }
    //             targets = &tgtBuf;
    //         } else {
    //             const unsigned t = myGraph->nodeOf(to);
    //             if (t != GRAPH::INVALID_NODE) {
    //                 tgtBuf.push_back(t);
    //             }
    //             targets = &tgtBuf;
    //         }
    //         if (sources->empty() || targets->empty()) {
    //             // unmappable endpoint (e.g. internal edge, or a TAZ with no member
    //             // in this class's graph) -> exact fallback.
    //             return myFallback->compute(from, to, vehicle, msTime, into, silent);
    //         }
    //
    //         this->startQuery();
    //         const double t = STEPS2TIME(msTime);
    //         if (!runQuery(metric, *sources, *targets, vehicle, t)) {
    //             this->endQuery(0);
    //             if (!silent && this->myErrorMsgHandler != nullptr) {
    //                 this->myErrorMsgHandler->informf(TL("No connection between edge '%' and edge '%' found."),
    //                                                  from->getID(), to->getID());
    //             }
    //             return false;
    //         }
    //         std::vector<const E*> path;
    //         buildPath(from, to, fromTaz, toTaz, path);
    //         int visited = (int)path.size();
    //         // Weight-period boundary: the metric of the depart period is exact as
    //         // long as the trip finishes inside that period (efforts are constant
    //         // within it, and no time-propagated optimum can cross the boundary if
    //         // this trip does not: any path entering the next period costs at
    //         // least the remaining period, which already exceeds this result).
    //         // When the trip does cross, ALSO query the next period's metric and
    //         // keep whichever path is cheaper under the true time-propagating
    //         // walk (recomputeCosts). Exact for the common single-crossing case
    //         // where the optimum follows one period's structure; trips spanning
    //         // several periods keep the better of the two candidates.
    //         if (myPeriodEnd != nullptr && myQuery.get_distance() > 0) {
    //             const SUMOTime boundary = myPeriodEnd(msTime);
    //             if (boundary != SUMOTime_MAX
    //                     && msTime + TIME2STEPS((double)myQuery.get_distance() / 100.) >= boundary) {
    //                 MetricPtr altMetric = myMetricProvider(vClass, boundary, vehicle);
    //                 if (altMetric != nullptr && altMetric != metric
    //                         && runQuery(altMetric, *sources, *targets, vehicle, t)) {
    //                     std::vector<const E*> altPath;
    //                     buildPath(from, to, fromTaz, toTaz, altPath);
    //                     visited += (int)altPath.size();
    //                     if (this->recomputeCosts(altPath, vehicle, msTime)
    //                             < this->recomputeCosts(path, vehicle, msTime)) {
    //                         path.swap(altPath);
    //                     }
    //                 }
    //             }
    //         }
    //         into.insert(into.end(), path.begin(), path.end());
    //         this->endQuery(visited);
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn compute(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool {
        // shortcut: runQuery needs the typed CustomizableContractionHierarchyQuery (myQuery); the trait bounds for isTazConnector/prohibits/id are in place
        panic!("shortcut: CCHRouter::compute needs the typed myQuery for runQuery")
    }

    /// C++: `void CCHRouter::prohibit(const Prohibitions &)`
    /// prohibitions: a closure that is already a live permission change is served by the per-class metric (CCH); anything else routes via the exact fallback. Keep our own copy of the set so compute() can classify each query -- deliberately NOT delegating to the base implementation, which writes per-edge prohibition state into myEdgeInfos that this router never populates.
    // void prohibit(const Prohibitions& toProhibit) {
    //         myProhibited = toProhibit;
    //         myProhibitionActive = !toProhibit.empty();
    //         myFallback->prohibit(toProhibit);
    //     }
    #[allow(unused_variables)]
    pub fn prohibit(&mut self, toProhibit: &SUMOAbstractRouter_Prohibitions) where V: crate::utils::router::router_edge::RouterVehicle {
        self.myProhibited = toProhibit.clone();
        self.myProhibitionActive = !toProhibit.is_empty();
        self.myFallback.m().prohibit(toProhibit);
    }

    /// C++: `bool CCHRouter::supportsProhibitions() const` [const]
    // bool supportsProhibitions() const {
    //         return true;  // handled by delegating to the fallback
    //     }
    #[allow(unused_variables)]
    pub fn supports_prohibitions(&self) -> bool {
        true
    }

    /// C++: `void CCHRouter::setBulkMode(const bool)`
    // void setBulkMode(const bool mode) {
    //         SUMOAbstractRouter<E, V>::setBulkMode(mode);
    //         myFallback->setBulkMode(mode);
    //     }
    #[allow(unused_variables)]
    pub fn set_bulk_mode(&mut self, mode: bool) {
        self.base.set_bulk_mode(mode);
        self.myFallback.m().set_bulk_mode(mode);
    }

    /// C++: `void CCHRouter::setMsgHandler(MsgHandler *const)`
    /// keep the fallback reporting through the same handler (duarouter swaps handlers around route repair; CHRouterWrapper propagates likewise)
    // void setMsgHandler(MsgHandler* const errorMsgHandler) {
    //         SUMOAbstractRouter<E, V>::setMsgHandler(errorMsgHandler);
    //         myFallback->setMsgHandler(errorMsgHandler);
    //     }
    #[allow(unused_variables)]
    pub fn set_msg_handler(&mut self, errorMsgHandler: Ptr<MsgHandler>) {
        self.base.set_msg_handler(errorMsgHandler);
        self.myFallback.m().set_msg_handler(errorMsgHandler);
    }

    /// C++: `bool CCHRouter::runQuery(MetricPtr, const int &, const int &, const V *const, double)` [private]
    /// Bind the query to metric (rebinding only on change), seed all sources and targets and run it; true iff a connection was found.
    // bool runQuery(MetricPtr metric, const std::vector<unsigned>& sources,
    //                   const std::vector<unsigned>& targets, const V* const vehicle, double t) {
    //         if (metric != myBoundMetric) {
    //             myQuery.reset(*metric);
    //             myBoundMetric = metric;
    //         } else {
    //             myQuery.reset();
    //         }
    //         const double bigEff = (double)(RoutingKit::inf_weight - 1) / 100.0;
    //         for (const unsigned s : sources) {
    //             const double eff = this->getEffort(myGraph->edgeOf(s), vehicle, t);
    //             const unsigned d = eff < bigEff ? (unsigned)llround(eff * 100.0) : RoutingKit::inf_weight - 1;
    //             myQuery.add_source(s, d);
    //         }
    //         for (const unsigned tt : targets) {
    //             myQuery.add_target(tt, 0);
    //         }
    //         myQuery.run();
    //         return myQuery.get_distance() < RoutingKit::inf_weight;
    //     }
    #[allow(unused_variables)]
    pub fn run_query(&mut self, metric: CCHRouter_MetricPtr, sources: &Vec<u32>, targets: &Vec<u32>, vehicle: Ptr<V>, t: f64) -> bool {
        // shortcut: myQuery is a placeholder i32 (CustomizableContractionHierarchyQuery borrows the metric), so reset/add_source/add_target/run cannot be called yet
        panic!("shortcut: CCHRouter::runQuery needs the typed CustomizableContractionHierarchyQuery")
    }

    /// C++: `void CCHRouter::buildPath(const E *, const E *, bool, bool, int &)` [private]
    /// expand the last query's node path, bracketing it with the TAZ connectors where the endpoints are zones
    // void buildPath(const E* from, const E* to, bool fromTaz, bool toTaz,
    //                    std::vector<const E*>& path) {
    //         const std::vector<unsigned> nodePath = myQuery.get_node_path();
    //         if (fromTaz) {
    //             path.push_back(from);
    //         }
    //         myGraph->expandNodePath(nodePath, path);
    //         if (toTaz) {
    //             path.push_back(to);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn build_path(&mut self, from: Ptr<E>, to: Ptr<E>, fromTaz: bool, toTaz: bool, path: &mut Vec<Ptr<E>>) {
        // shortcut: myQuery is a placeholder i32, so get_node_path() cannot be called yet; the rest of the expansion is ported
        let nodePath: Vec<u32> = panic!("shortcut: CCHRouter::buildPath needs the typed CustomizableContractionHierarchyQuery");
        if fromTaz {
            path.push(from);
        }
        self.myGraph.expand_node_path(&nodePath, path);
        if toTaz {
            path.push(to);
        }
    }

    /// C++: `bool CCHRouter::prohibitionsCoveredByMetric(SUMOVehicleClass) const` [const, private]
    /// Can the per-class CCH metric already express every edge this prohibition set forbids for vClass? True iff each forbidden edge is ALSO closed for the class in the live network (getPermissions()) -- the exact condition fillInputWeights() turns into inf_weight. A prohibited edge that still permits the class is an arbitrary per-query prohibition no shared metric can encode, so the caller must use the exact fallback. (For time-bounded closures A* itself avoids the edge via this same live permission signal -- SUMOAbstractRouter::isProhibited -- not the per-query branch, so this is faithful to A*'s own behavior.)
    // bool prohibitionsCoveredByMetric(SUMOVehicleClass vClass) const {
    //         for (const auto& item : myProhibited) {
    //             if ((item.second.permissions & vClass) != vClass          // forbids vClass here...
    //                     && (item.first->getPermissions() & vClass) != 0) {  // ...but edge still permits it live
    //                 return false;
    //             }
    //         }
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn prohibitions_covered_by_metric(&self, vClass: SUMOVehicleClass) -> bool {
        for item in self.myProhibited.iter() {
            if (item.1.permissions & vClass) != vClass
                // shortcut: E::getPermissions (edge-level permissions) is not in RouterEdge, add it to the trait to upgrade
                && ({
                    let perm: SVCPermissions = panic!("shortcut: CCHRouter::prohibitionsCoveredByMetric needs E::getPermissions, not in RouterEdge");
                    (perm & vClass) != 0
                })
            {
                return false;
            }
        }
        true
    }

    /// C++: `CCHRouter<E, V, GRAPH> & CCHRouter::operator=(const CCHRouter<E, V, GRAPH> &)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &CCHRouter<E, V, GRAPH>) -> &mut CCHRouter<E, V, GRAPH> {
        // shortcut: needs the typed myQuery and the SUMOAbstractRouter copy assignment, neither ported yet
        panic!("shortcut: CCHRouter::operator= needs typed myQuery and SUMOAbstractRouter assignment")
    }

}

