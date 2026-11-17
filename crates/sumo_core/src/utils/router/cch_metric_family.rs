//! Port of utils/router/CCHMetricFamily.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::router::ro_edge::ROEdge;
use crate::router::ro_vehicle::ROVehicle;
use crate::utils::router::cch_graph::CCHGraph;
use crate::utils::router::router_edge::RouterEdge;

/// C++ typedef `typename Graph::EffortOperation CCHMetricFamily::EffortOperation`
pub type CCHMetricFamily_EffortOperation<E, V> = CCHGraph_EffortOperation<E, V>;

/// C++ typedef `CCHGraph<E, V> CCHMetricFamily::Graph`
pub type CCHMetricFamily_Graph<E, V> = CCHGraph<E, V>;

/// C++ typedef `const int * CCHMetricFamily::MetricPtr`
pub type CCHMetricFamily_MetricPtr = Ptr<CCHMetric>;
/// A RoutingKit metric over the process-lifetime CCHGraph; it borrows its owner's input-weight buffer
/// (the C++ metric keeps a pointer to it), so the borrows are 'static raw borrows, see static_cch / static_weights
pub type CCHMetric = CustomizableContractionHierarchyMetric<'static>;

/// the graph outlives every metric (process-lifetime singleton in C++ too)
pub(crate) fn static_cch<E, V>(graph: Ptr<CCHGraph<E, V>>) -> &'static CustomizableContractionHierarchy {
    unsafe { &*(&graph.myCCH as *const CustomizableContractionHierarchy) }
}

/// the weight buffer lives as long as its metric and is only refilled in place (same length, no reallocation),
/// which the metric reads at its next customize like the C++ pointer does
pub(crate) fn static_weights(w: &Vec<u32>) -> &'static [u32] {
    unsafe { std::slice::from_raw_parts(w.as_ptr(), w.len()) }
}

/// C++ typedef `V *(*)(const K *, int) CCHMetricFamily::RefVehicleFactory`
pub type CCHMetricFamily_RefVehicleFactory<V, K> = Option<fn(Ptr<K>, i32) -> Ptr<V>>;

/// C++ typedef `void (*)(const Graph *, const V *, int &) CCHMetricFamily::WeightPatch`
pub type CCHMetricFamily_WeightPatch<E, V> = Option<fn(Ptr<CCHGraph<E, V>>, Ptr<V>, &mut Vec<u32>)>;

/// per-vehicle-type CCH metric store over a shared CCHGraph
/// C++ class `CCHMetricFamily` (utils/router/CCHMetricFamily.h)
pub struct CCHMetricFamily<E, V, K> {
    /// the immutable shared topology
    pub myGraph: Ptr<CCHGraph<E, V>>,
    /// the effort the metrics are filled from
    pub myFillEffort: CCHGraph_EffortOperation<E, V>,
    /// the type-independent effort of the LIVE deadband
    pub myGateEffort: Option<CCHGraph_EffortOperation<E, V>>,
    /// reference-vehicle factory (required LIVE, optional STATIC)
    pub myFactory: CCHMetricFamily_RefVehicleFactory<V, K>,
    /// post-fill weight hook (STATIC)
    pub myPatch: CCHMetricFamily_WeightPatch<E, V>,
    /// LIVE or STATIC (fixed at construction)
    pub myLive: bool,
    /// @{
    pub myBegin: SUMOTime,
    pub myWeightPeriod: SUMOTime,
    /// C++ std::map<pair<const K*, int>, StaticMetric>; ordered by (key address, period) like the C++ map
    pub myStaticMetrics: BTreeMap<(Ptr<K>, i32), CCHMetricFamily_StaticMetric<V>>,
    pub myStaticLock: std::sync::Mutex<()>,
    /// the efforts changed behind the cached metrics: refill and re-customize them all on the next get() (see flagStale)
    pub myStale: bool,
    /// the deadband bounds (see the LIVE constructor)
    pub myUpdateFactor: f64,
    pub myUpdateConstant: f64,
    /// frozen random-factor realizations per type (see the LIVE ctor)
    pub myEnsembleK: i32,
    /// every type's state, in creation order (barrier iteration) and by key (query lookup; only mutated while no query is in flight)
    pub myLiveMetrics: Vec<Ptr<CCHMetricFamily_LiveMetric<V>>>,
    pub myByKey: HashMap<Ptr<K>, Vec<Ptr<CCHMetricFamily_LiveMetric<V>>>>,
    /// types that queried but have no metric yet (demand streams, so types can appear after seeding); queries register them under the lock and route via the fallback until the next barrier builds their state
    pub myWantedKeys: Vec<Ptr<K>>,
    pub myWantedLock: std::sync::Mutex<()>,
    /// whether a query arrived since the last barrier (bootstraps type creation and re-arms after resets -- see atBarrier)
    pub myQueried: bool,
    /// a permission flip has not yet been customized into the published metrics (queries divert to the exact fallback while set)
    pub myMetricStale: bool,
    /// barrier counter of the tracked efforts and the counter value the published metrics were customized from: the staleness bound's safety invariant (see atBarrier / published). Both only change on the owner's thread at the barrier -- no query is in flight there -- the atomics just publish the values to the query threads.
    pub mySpeedEpoch: u64,
    pub myPublishedEpoch: u64,
    /// how many customize passes must run a FULL refill after a permission flip: 2 = both ping-pong buffers (the sparse path cannot repair connection-level mask changes: arcsOfEdge deliberately omits the tail side)
    pub myFullFillsPending: i32,
    /// per-buffer edge state for the sparse path: the gate effort each edge's arcs were last filled from (NaN = never applied / forced), the pending-dirty flag, and the pending list
    pub myAppliedEffort: [Vec<f64>; 2],
    pub myPendingFlag: [Vec<i32>; 2],
    pub myPendingList: [Vec<Ptr<E>>; 2],
    _p: std::marker::PhantomData<(E, V, K,)>,
}

impl<E: RouterEdge<V>, V: crate::utils::router::router_edge::RouterVehicle, K> CCHMetricFamily<E, V, K> {
    /// C++: `void CCHMetricFamily::CCHMetricFamily<E, V, K>(const Graph *, EffortOperation, SUMOTime, SUMOTime, RefVehicleFactory, WeightPatch)`
    /// construct a STATIC family (lazy build per (type, period))
    // CCHMetricFamily(const Graph* graph, EffortOperation fillEffort,
    //                     SUMOTime begin, SUMOTime weightPeriod,
    //                     RefVehicleFactory factory, WeightPatch patch) :
    //         myGraph(graph), myFillEffort(fillEffort), myGateEffort(nullptr),
    //         myFactory(factory), myPatch(patch), myLive(false),
    //         myBegin(begin), myWeightPeriod(weightPeriod),
    //         myUpdateFactor(1.), myUpdateConstant(0.) {
    //     }
    #[allow(unused_variables)]
    pub fn new(graph: Ptr<CCHGraph<E, V>>, fillEffort: CCHGraph_EffortOperation<E, V>, begin: SUMOTime, weightPeriod: SUMOTime, factory: CCHMetricFamily_RefVehicleFactory<V, K>, patch: CCHMetricFamily_WeightPatch<E, V>) -> Self {
        CCHMetricFamily {
            myGraph: graph,
            myFillEffort: fillEffort,
            myGateEffort: None,
            myFactory: factory,
            myPatch: patch,
            myLive: false,
            myBegin: begin,
            myWeightPeriod: weightPeriod,
            myStaticMetrics: BTreeMap::new(),
            myStaticLock: std::sync::Mutex::new(()),
            myStale: false,
            myUpdateFactor: 1.,
            myUpdateConstant: 0.,
            myEnsembleK: 1,
            myLiveMetrics: Vec::new(),
            myByKey: HashMap::new(),
            myWantedKeys: Vec::new(),
            myWantedLock: std::sync::Mutex::new(()),
            myQueried: false,
            myMetricStale: false,
            mySpeedEpoch: 0,
            myPublishedEpoch: 0,
            myFullFillsPending: 0,
            myAppliedEffort: [Vec::new(), Vec::new()],
            myPendingFlag: [Vec::new(), Vec::new()],
            myPendingList: [Vec::new(), Vec::new()],
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void CCHMetricFamily::CCHMetricFamily<E, V, K>(const Graph *, EffortOperation, EffortOperation, double, double, RefVehicleFactory, int)`
    /// construct a LIVE family (double-buffered, barrier-customized)
    // CCHMetricFamily(const Graph* graph, EffortOperation fillEffort,
    //                     EffortOperation gateEffort, double updateFactor,
    //                     double updateConstant, RefVehicleFactory factory,
    //                     int ensembleK = 1) :
    //         myGraph(graph), myFillEffort(fillEffort), myGateEffort(gateEffort),
    //         myFactory(factory), myPatch(nullptr), myLive(true),
    //         myBegin(0), myWeightPeriod(SUMOTime_MAX),
    //         myUpdateFactor(updateFactor), myUpdateConstant(updateConstant),
    //         myEnsembleK(ensembleK > 1 ? ensembleK : 1) {
    //         const unsigned space = myGraph->edgeIdSpace();
    //         for (int i = 0; i < 2; i++) {
    //             // NaN = "never applied": the first pending occurrence always
    //             // passes the deadband and primes the entry
    //             myAppliedEffort[i].assign(space, std::numeric_limits<double>::quiet_NaN());
    //             myPendingFlag[i].assign(space, 0);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn new_2(graph: Ptr<CCHGraph<E, V>>, fillEffort: CCHGraph_EffortOperation<E, V>, gateEffort: CCHGraph_EffortOperation<E, V>, updateFactor: f64, updateConstant: f64, factory: CCHMetricFamily_RefVehicleFactory<V, K>, ensembleK: i32) -> Self {
        let space = graph.edge_id_space() as usize;
        let mut this = CCHMetricFamily {
            myGraph: graph,
            myFillEffort: fillEffort,
            myGateEffort: Some(gateEffort),
            myFactory: factory,
            myPatch: None,
            myLive: true,
            myBegin: 0,
            myWeightPeriod: utils::common::time::SUMOTIME_MAX,
            myStaticMetrics: BTreeMap::new(),
            myStaticLock: std::sync::Mutex::new(()),
            myStale: false,
            myUpdateFactor: updateFactor,
            myUpdateConstant: updateConstant,
            myEnsembleK: if ensembleK > 1 { ensembleK } else { 1 },
            myLiveMetrics: Vec::new(),
            myByKey: HashMap::new(),
            myWantedKeys: Vec::new(),
            myWantedLock: std::sync::Mutex::new(()),
            myQueried: false,
            myMetricStale: false,
            mySpeedEpoch: 0,
            myPublishedEpoch: 0,
            myFullFillsPending: 0,
            myAppliedEffort: [Vec::new(), Vec::new()],
            myPendingFlag: [Vec::new(), Vec::new()],
            myPendingList: [Vec::new(), Vec::new()],
            _p: std::marker::PhantomData,
        };
        for i in 0..2 {
            // NaN = "never applied": the first pending occurrence always
            // passes the deadband and primes the entry
            this.myAppliedEffort[i] = vec![f64::NAN; space];
            this.myPendingFlag[i] = vec![0; space];
        }
        this
    }

    /// C++: `void CCHMetricFamily::~CCHMetricFamily<E, V, K>()`
    // ~CCHMetricFamily() {
    //         for (LiveMetric* c : myLiveMetrics) {
    //             delete c->refVehicle;
    //             delete c;
    //         }
    //         for (auto& item : myStaticMetrics) {
    //             delete item.second.refVehicle;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        for c in self.myLiveMetrics.drain(..) {
            let rv = c.m().refVehicle;
            if !rv.is_null() {
                unsafe { drop(Box::from_raw(rv.as_raw())) };
            }
            unsafe { drop(Box::from_raw(c.as_raw())) };
        }
        for item in self.myStaticMetrics.values() {
            let rv = item.refVehicle;
            if !rv.is_null() {
                unsafe { drop(Box::from_raw(rv.as_raw())) };
            }
        }
    }

    /// C++: `MetricPtr CCHMetricFamily::get(const K *, SUMOVehicleClass, SUMOTime, const V *)`
    /// the metric for (type key, period of time), built on the first query of the pair; nullptr before init. Thread-safe: the mutex serialises concurrent first queries from parallel routing threads (satisfying primeClassMask's synchronization contract); afterwards the map is read-only for that key (map references are address-stable). A pending flagStale() first refills and re-customizes every cached metric from the live efforts and permissions -- in place, so the flagging host guarantees that no query is in flight (the simulation's main thread, marouter's reset between two iterations).
    // MetricPtr get(const K* key, SUMOVehicleClass vClass, SUMOTime time, const V* veh) {
    //         std::lock_guard<std::mutex> lock(myStaticLock);
    //         if (myStale) {
    //             for (auto& item : myStaticMetrics) {
    //                 StaticMetric& sm = item.second;
    //                 fillStatic(sm, item.first.second, time, sm.refVehicle != nullptr ? sm.refVehicle : veh, veh);
    //                 sm.metric->customize();
    //             }
    //             myStale = false;
    //         }
    //         const int period = periodOf(time);
    //         const std::pair<const K*, int> mapKey(key, period);
    //         auto it = myStaticMetrics.find(mapKey);
    //         if (it == myStaticMetrics.end()) {
    //             StaticMetric& sm = myStaticMetrics[mapKey];
    //             sm.vClass = vClass;
    //             const V* ref = veh;
    //             if (myFactory != nullptr) {
    //                 sm.refVehicle = myFactory(key, 0);
    //                 ref = sm.refVehicle;
    //             }
    //             fillStatic(sm, period, time, ref, veh);
    //             sm.metric.reset(new RoutingKit::CustomizableContractionHierarchyMetric(
    //                                 myGraph->cch(), sm.weights));
    //             sm.metric->customize();
    //             return sm.metric.get();
    //         }
    //         return it->second.metric.get();
    //     }
    #[allow(unused_variables)]
    pub fn get(&mut self, key: Ptr<K>, vClass: SUMOVehicleClass, time: SUMOTime, veh: Ptr<V>) -> CCHMetricFamily_MetricPtr {
        // C++ std::lock_guard on myStaticLock: the router applications route on one thread here (no FOX pool)
        if self.myStale {
            let mut metrics = std::mem::take(&mut self.myStaticMetrics);
            for (k, sm) in metrics.iter_mut() {
                let ref_ = if !sm.refVehicle.is_null() { sm.refVehicle } else { veh };
                self.fill_static(sm, k.1, time, ref_, veh);
                sm.metric.as_mut().unwrap().customize();
            }
            self.myStaticMetrics = metrics;
            self.myStale = false;
        }
        let period = self.period_of(time);
        let mapKey = (key, period);
        if !self.myStaticMetrics.contains_key(&mapKey) {
            let mut sm = CCHMetricFamily_StaticMetric { weights: Vec::new(), metric: None, vClass: SVC_PASSENGER, refVehicle: Ptr::null(), filled: false, patched: Vec::new() };
            sm.vClass = vClass;
            let mut ref_ = veh;
            if let Some(factory) = self.myFactory {
                sm.refVehicle = factory(key, 0);
                ref_ = sm.refVehicle;
            }
            self.fill_static(&mut sm, period, time, ref_, veh);
            let mut metric = Box::new(CCHMetric::new(static_cch(self.myGraph), static_weights(&sm.weights)));
            metric.customize();
            let result = Ptr::new(&*metric);
            sm.metric = Some(metric);
            self.myStaticMetrics.insert(mapKey, sm);
            return result;
        }
        Ptr::new(&**self.myStaticMetrics[&mapKey].metric.as_ref().unwrap())
    }

    /// C++: `SUMOTime CCHMetricFamily::periodEnd(SUMOTime) const` [const]
    /// the end of the weight period containing the given time (SUMOTime_MAX when the weights are static); CCHRouter's PeriodEnd hook
    // SUMOTime periodEnd(SUMOTime time) const {
    //         if (myWeightPeriod <= 0 || myWeightPeriod == SUMOTime_MAX) {
    //             return SUMOTime_MAX;
    //         }
    //         return myBegin + (periodOf(time) + 1) * myWeightPeriod;
    //     }
    #[allow(unused_variables)]
    pub fn period_end(&self, time: SUMOTime) -> SUMOTime {
        if self.myWeightPeriod <= 0 || self.myWeightPeriod == SUMOTIME_MAX {
            return SUMOTIME_MAX;
        }
        return self.myBegin + (self.period_of(time) as SUMOTime + 1) * self.myWeightPeriod;
    }

    /// C++: `void CCHMetricFamily::flagStale()`
    /// the efforts behind the cached metrics changed -- a runtime permission change in the simulation (the caller must also invalidate the graph's class masks), or marouter's travel times after an assignment iteration (CCHRouter::reset): every cached metric is refilled and re-customized on the next get().
    // void flagStale() {
    //         std::lock_guard<std::mutex> lock(myStaticLock);
    //         myStale = true;
    //     }
    #[allow(unused_variables)]
    pub fn flag_stale(&mut self) {
        let _lock = self.myStaticLock.lock();
        self.myStale = true;
    }

    /// C++: `void CCHMetricFamily::flagPermissionsStale()`
    /// flagStale() under the simulation's name for the permission case
    // void flagPermissionsStale() {
    //         flagStale();
    //     }
    #[allow(unused_variables)]
    pub fn flag_permissions_stale(&mut self) {
        self.flag_stale();
    }

    /// C++: `void CCHMetricFamily::seedKey(const K *)`
    /// allocate the metric state (all ensemble slots) for one type; call only while no query is in flight (seeding at init, or the customization barrier)
    // void seedKey(const K* key) {
    //         if (myByKey.count(key) == 0) {
    //             std::vector<LiveMetric*>& slots = myByKey[key];
    //             for (int k = 0; k < myEnsembleK; k++) {
    //                 slots.push_back(buildLiveMetric(key, k));
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn seed_key(&mut self, key: Ptr<K>) {
        if !self.myByKey.contains_key(&key) {
            let mut slots = Vec::new();
            for k in 0..self.myEnsembleK {
                slots.push(self.build_live_metric(key, k));
            }
            self.myByKey.insert(key, slots);
        }
    }

    /// C++: `void CCHMetricFamily::markDirty(const E *)`
    /// queue an edge whose gate effort changed (both buffers; the deadband applies later, at the customize barrier)
    // void markDirty(const E* e) {
    //         const int id = e->getNumericalID();
    //         if (id < 0 || id >= (int)myPendingFlag[0].size()) {
    //             return;
    //         }
    //         for (int i = 0; i < 2; i++) {
    //             if (!myPendingFlag[i][id]) {
    //                 myPendingFlag[i][id] = 1;
    //                 myPendingList[i].push_back(e);
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn mark_dirty(&mut self, e: Ptr<E>) {
        let id = e.numerical_id();
        if id < 0 || id as usize >= self.myPendingFlag[0].len() {
            return;
        }
        for i in 0..2 {
            if self.myPendingFlag[i][id as usize] == 0 {
                self.myPendingFlag[i][id as usize] = 1;
                self.myPendingList[i].push(e);
            }
        }
    }

    /// C++: `void CCHMetricFamily::invalidateEdge(const E *)`
    /// a runtime permission change (closure / re-opening) hit this edge: queue it bypassing the deadband and divert every query to the exact fallback until customize() has refilled both buffers wholesale (the sparse path cannot repair tail-side arcs of a connection-level change). The caller must also invalidate the graph's class masks.
    // void invalidateEdge(const E* e) {
    //         const int id = e->getNumericalID();
    //         if (id < 0 || id >= (int)myPendingFlag[0].size()) {
    //             return;
    //         }
    //         markDirty(e);
    //         // NaN sentinel: the deadband always passes, so the flip reaches the
    //         // metric at the next barrier even though the speed table did not move
    //         myAppliedEffort[0][id] = std::numeric_limits<double>::quiet_NaN();
    //         myAppliedEffort[1][id] = std::numeric_limits<double>::quiet_NaN();
    //         myFullFillsPending = 2;
    //         myMetricStale.store(true, std::memory_order_release);
    //     }
    #[allow(unused_variables)]
    pub fn invalidate_edge(&mut self, e: Ptr<E>) {
        let id = e.numerical_id();
        if id < 0 || id as usize >= self.myPendingFlag[0].len() {
            return;
        }
        self.mark_dirty(e);
        self.myAppliedEffort[0][id as usize] = f64::NAN;
        self.myAppliedEffort[1][id as usize] = f64::NAN;
        self.myFullFillsPending = 2;
        self.myMetricStale = true;
    }

    /// C++: `void CCHMetricFamily::atBarrier(double, bool)`
    /// the owner's customization barrier: bump the speed epoch when an effort moved and re-customize when it matters. Lazy, following the CH weightPeriod pattern: metric state is only built for types somebody routes on (myLiveMetrics stays empty in a run that never routes, which then pays nothing here). Once any metric exists, every barrier where an effort moved re-customizes, so a query never sees a metric more than one barrier old -- CH's weightPeriod bound. The queried flag only bootstraps type creation; a pending permission flip forces the customize even without queries.
    // void atBarrier(double now, bool effortsMoved) {
    //         if (effortsMoved) {
    //             mySpeedEpoch.fetch_add(1, std::memory_order_release);
    //         }
    //         if (myQueried.exchange(false, std::memory_order_acq_rel)
    //                 || myMetricStale.load(std::memory_order_acquire)
    //                 || (effortsMoved && !myLiveMetrics.empty())) {
    //             customize(now);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn at_barrier(&mut self, now: f64, effortsMoved: bool) {
        if effortsMoved {
            self.mySpeedEpoch += 1;
        }
        let queried = self.myQueried;
        self.myQueried = false;
        if queried || self.myMetricStale || (effortsMoved && !self.myLiveMetrics.is_empty()) {
            self.customize(now);
        }
    }

    /// C++: `void CCHMetricFamily::customize(double)`
    /// refill + customize + publish every type's metric from the live efforts and permissions. Call only while no query is in flight (the publish itself is lock-free: release store here, acquire loads on the query hot path).
    // void customize(double now) {
    //         // types that queried since the last barrier without a metric
    //         // (streamed route files, TraCI-added types): create their state now
    //         {
    //             std::lock_guard<std::mutex> lock(myWantedLock);
    //             for (const K* key : myWantedKeys) {
    //                 seedKey(key);
    //             }
    //             myWantedKeys.clear();
    //         }
    //         if (myLiveMetrics.empty()) {
    //             return;
    //         }
    //         // buffers flip in lockstep across types (every call processes all)
    //         const int backShared = 1 - myLiveMetrics.front()->frontIndex;
    //         // Permission flip pending: run a FULL refill on this pass's back
    //         // buffer (the other buffer gets its full refill on the next pass --
    //         // myFullFillsPending counts both down). The fills below re-prime the
    //         // graph's connection masks, which the flip's caller invalidated.
    //         bool forceFull = false;
    //         if (myFullFillsPending > 0) {
    //             myFullFillsPending--;
    //             forceFull = true;
    //         }
    //         // Deadband pass, once, on the type-shared gate effort: accept edges
    //         // whose effort moved by more than BOTH bounds since this buffer last
    //         // applied them; rejected edges stay pending (see the constructor doc)
    //         std::vector<const E*> accepted;
    //         if (!forceFull && myLiveMetrics.front()->metric[backShared] != nullptr) {
    //             std::vector<const E*> stillPending;
    //             for (const E* e : myPendingList[backShared]) {
    //                 const int id = e->getNumericalID();
    //                 const double effNow = myGateEffort(e, nullptr, now);
    //                 const double effApplied = myAppliedEffort[backShared][id];
    //                 bool pass = true;
    //                 // without a threshold every marked edge passes: the gate effort may be clamped
    //                 // at the minimum travel time while a type with speedFactor > 1 still sees changes
    //                 if ((myUpdateFactor > 1. || myUpdateConstant > 0.) && !std::isnan(effApplied) && effApplied > 0. && effNow > 0.) {
    //                     const double hi = MAX2(effNow, effApplied);
    //                     const double lo = MIN2(effNow, effApplied);
    //                     pass = (hi / lo > myUpdateFactor) && (hi - lo > myUpdateConstant);
    //                 }
    //                 if (pass) {
    //                     accepted.push_back(e);
    //                     myAppliedEffort[backShared][id] = effNow;
    //                     myPendingFlag[backShared][id] = 0;
    //                 } else {
    //                     stillPending.push_back(e);
    //                 }
    //             }
    //             myPendingList[backShared].swap(stillPending);
    //         }
    //         for (LiveMetric* c : myLiveMetrics) {
    //             const int back = 1 - c->frontIndex;
    //             // Fill from the live efforts, masking arcs the class is not
    //             // permitted on -- this is where per-class permissions AND active
    //             // closures become inf_weight. The fill reference is the type's
    //             // OWNED refVehicle, so the effort captures the type's extras
    //             // exactly (including each folded via edge inside viaChainEffort)
    //             // and freezes ONE random realization per metric -- the same
    //             // approximation CHRouterWrapper makes per hierarchy; exact
    //             // per-vehicle randomization remains the domain of the exact
    //             // routers.
    //             if (c->metric[back] == nullptr) {
    //                 // first use of this buffer for THIS type only (streamed/late
    //                 // vType): no previous state to diff against for this type,
    //                 // but every other already-running type's sparse deadband
    //                 // state is unaffected, so this branch must NOT trigger the
    //                 // shared wipe below (which would reset the type-shared
    //                 // myPendingList/myAppliedEffort for everybody just because
    //                 // one type is new)
    //                 myGraph->fillInputWeights(myFillEffort, c->vClass, c->refVehicle, now, c->weight[back]);
    //                 c->metric[back] = std::make_shared<RoutingKit::CustomizableContractionHierarchyMetric>(
    //                                       myGraph->cch(), c->weight[back]);
    //                 c->metric[back]->customize();  // serial; avoids OpenMP oversubscription under FOX
    //             } else if (forceFull) {
    //                 // permission flip: wholesale refill in place from the
    //                 // re-primed masks and live permissions (the metric already
    //                 // references weight[back]), then a full customize. This is
    //                 // the ONLY case that invalidates the type-shared deadband
    //                 // state below, since it is the only one that actually
    //                 // changes what "applied" means for every type at once.
    //                 myGraph->fillInputWeights(myFillEffort, c->vClass, c->refVehicle, now, c->weight[back]);
    //                 c->metric[back]->customize();
    //             } else {
    //                 // SPARSE PATH: metric[back] is the customization of the
    //                 // current contents of weight[back]. Recompute only the arcs
    //                 // of accepted edges (edge->arc reverse image), write the
    //                 // ones that moved, and propagate through affected triangles
    //                 // only. Cost scales with traffic transitions, not with the
    //                 // network.
    //                 std::vector<unsigned>& applied = c->weight[back];
    //                 c->partial->reset(myGraph->cch());
    //                 unsigned changed = 0;
    //                 for (const E* e : accepted) {
    //                     for (const unsigned a : myGraph->arcsOfEdge(e)) {
    //                         const unsigned newW = myGraph->computeArcWeight(a, myFillEffort, c->vClass, c->refVehicle, now);
    //                         if (newW != applied[a]) {
    //                             applied[a] = newW;
    //                             c->partial->update_arc(a);  // takes INPUT arc ids
    //                             ++changed;
    //                         }
    //                     }
    //                 }
    //                 if (changed > 0) {
    //                     c->partial->customize(*c->metric[back]);
    //                 }
    //             }
    //             // publish lock-free: release store at the barrier, worker
    //             // acquire-loads on the hot path
    //             c->frontIndex = back;
    //             c->front.store(c->metric[back].get(), std::memory_order_release);
    //         }
    //         if (forceFull) {
    //             // a permission flip's full refill matched every arc to the live
    //             // efforts/masks for this buffer: type-shared deadband entries
    //             // are stale; NaN re-arms the first-change auto-pass. (A plain
    //             // per-type first-fill above does NOT reach here -- it must not
    //             // wipe deadband state that other, already-running types still
    //             // depend on.)
    //             for (const E* e : myPendingList[backShared]) {
    //                 myPendingFlag[backShared][e->getNumericalID()] = 0;
    //             }
    //             myPendingList[backShared].clear();
    //             myAppliedEffort[backShared].assign(myAppliedEffort[backShared].size(),
    //                                                std::numeric_limits<double>::quiet_NaN());
    //         }
    //         // every published front now reflects the live permissions (the other
    //         // buffer, if still pending a full refill, is repaired before its
    //         // next publish) and the live efforts as of this barrier (both epoch
    //         // counters only move on the owner's thread at the barrier, so the
    //         // pair cannot tear)
    //         myMetricStale.store(false, std::memory_order_release);
    //         myPublishedEpoch.store(mySpeedEpoch.load(std::memory_order_relaxed),
    //                                std::memory_order_release);
    //     }
    #[allow(unused_variables)]
    pub fn customize(&mut self, now: f64) {
        {
            // C++ std::lock_guard on myWantedLock (single-threaded here)
            let wanted = std::mem::take(&mut self.myWantedKeys);
            for key in wanted {
                self.seed_key(key);
            }
        }
        if self.myLiveMetrics.is_empty() {
            return;
        }
        let backShared = (1 - self.myLiveMetrics[0].frontIndex) as usize;
        let mut forceFull = false;
        if self.myFullFillsPending > 0 {
            self.myFullFillsPending -= 1;
            forceFull = true;
        }
        let mut accepted: Vec<Ptr<E>> = Vec::new();
        if !forceFull && self.myLiveMetrics[0].metric[backShared].is_some() {
            let mut stillPending: Vec<Ptr<E>> = Vec::new();
            let gate = self.myGateEffort.expect("LIVE CCHMetricFamily without gate effort");
            for &e in self.myPendingList[backShared].iter() {
                let id = e.numerical_id() as usize;
                let effNow = gate(e, Ptr::null(), now);
                let effApplied = self.myAppliedEffort[backShared][id];
                let mut pass = true;
                if !effApplied.is_nan() && effApplied > 0. && effNow > 0. {
                    let hi = if effNow > effApplied { effNow } else { effApplied };
                    let lo = if effNow < effApplied { effNow } else { effApplied };
                    pass = (hi / lo > self.myUpdateFactor) && (hi - lo > self.myUpdateConstant);
                }
                if pass {
                    accepted.push(e);
                    self.myAppliedEffort[backShared][id] = effNow;
                    self.myPendingFlag[backShared][id] = 0;
                } else {
                    stillPending.push(e);
                }
            }
            self.myPendingList[backShared] = stillPending;
        }
        let graph = self.myGraph;
        for &cp in self.myLiveMetrics.iter() {
            let c = cp.m();
            let back = (1 - c.frontIndex) as usize;
            if c.metric[back].is_none() {
                graph.fill_input_weights(self.myFillEffort, c.vClass, c.refVehicle, now, &mut c.weight[back]);
                let mut metric = Box::new(CCHMetric::new(static_cch(graph), static_weights(&c.weight[back])));
                metric.customize(); // serial; avoids OpenMP oversubscription under FOX
                c.metric[back] = Some(metric);
            } else if forceFull {
                graph.fill_input_weights(self.myFillEffort, c.vClass, c.refVehicle, now, &mut c.weight[back]);
                c.metric[back].as_mut().unwrap().customize();
            } else {
                let partial = c.partial.as_mut().unwrap();
                partial.reset(static_cch(graph));
                let mut changed: u32 = 0;
                for &e in accepted.iter() {
                    for &a in graph.arcs_of_edge(e).iter() {
                        let newW = graph.compute_arc_weight(a, self.myFillEffort, c.vClass, c.refVehicle, now);
                        if newW != c.weight[back][a as usize] {
                            c.weight[back][a as usize] = newW;
                            partial.update_arc(a); // takes INPUT arc ids
                            changed += 1;
                        }
                    }
                }
                if changed > 0 {
                    partial.customize(c.metric[back].as_mut().unwrap());
                }
            }
            c.frontIndex = back as i32;
            c.front = Ptr::new(&**c.metric[back].as_ref().unwrap());
        }
        if forceFull {
            for &e in self.myPendingList[backShared].iter() {
                self.myPendingFlag[backShared][e.numerical_id() as usize] = 0;
            }
            self.myPendingList[backShared].clear();
            let n = self.myAppliedEffort[backShared].len();
            self.myAppliedEffort[backShared] = vec![f64::NAN; n];
        }
        self.myMetricStale = false;
        self.myPublishedEpoch = self.mySpeedEpoch;
    }

    /// C++: `MetricPtr CCHMetricFamily::published(const K *, const std::string &)`
    /// the published metric for a type key and querying vehicle id (the id picks the ensemble slot -- see the LIVE constructor), or nullptr if the query must route on the exact fallback. Query hot path, any thread.
    // MetricPtr published(const K* key, const std::string& vehID) {
    //         const auto it = myByKey.find(key);
    //         if (it == myByKey.end()) {
    //             // streamed-in type without a metric yet: register it for
    //             // creation at the next barrier and fall back
    //             {
    //                 std::lock_guard<std::mutex> lock(myWantedLock);
    //                 myWantedKeys.push_back(key);
    //             }
    //             myQueried.store(true, std::memory_order_relaxed);
    //             return nullptr;
    //         }
    //         // mark the metric as consumed. Test before set: an unconditional
    //         // store from every query would ping-pong the cache line between
    //         // cores; the read is shared and the store fires once per barrier
    //         // window (relaxed: only gates bootstrap/re-arm, never visibility)
    //         if (!myQueried.load(std::memory_order_relaxed)) {
    //             myQueried.store(true, std::memory_order_relaxed);
    //         }
    //         // A permission flip that is not yet customized in makes every
    //         // published metric unusable: it may still route THROUGH a
    //         // just-closed edge, and CCHRouter's coverage test consults the LIVE
    //         // permissions, which would wrongly bless the query. Route exactly
    //         // until the barrier repairs the metrics.
    //         if (myMetricStale.load(std::memory_order_acquire)) {
    //             return nullptr;
    //         }
    //         // Safety invariant: never serve a metric the efforts have moved away
    //         // from -- the exact routers see live weights and CH is bounded to
    //         // one weightPeriod. atBarrier re-customizes at every moved barrier
    //         // once metrics exist, so this cannot fire in steady state; if a gap
    //         // ever opens, this burst routes on the exact fallback (live weights)
    //         // and the queried flag set above arms the customize that closes it.
    //         if (myPublishedEpoch.load(std::memory_order_acquire)
    //                 != mySpeedEpoch.load(std::memory_order_acquire)) {
    //             return nullptr;
    //         }
    //         // stable slot assignment: FNV-1a over the id (std::hash is
    //         // implementation-defined and would break cross-platform test
    //         // reproducibility)
    //         const LiveMetric* c = myEnsembleK == 1 ? it->second.front()
    //                               : it->second[fnv1a(vehID) % myEnsembleK];
    //         return c->front.load(std::memory_order_acquire);
    //     }
    #[allow(unused_variables)]
    pub fn published(&mut self, key: Ptr<K>, vehID: &str) -> CCHMetricFamily_MetricPtr {
        let slots = match self.myByKey.get(&key) {
            Some(slots) => slots.clone(),
            None => {
                self.myWantedKeys.push(key);
                self.myQueried = true;
                return Ptr::default();
            }
        };
        if !self.myQueried {
            self.myQueried = true;
        }
        if self.myMetricStale {
            return Ptr::default();
        }
        if self.myPublishedEpoch != self.mySpeedEpoch {
            return Ptr::default();
        }
        let c = if self.myEnsembleK == 1 {
            slots[0]
        } else {
            slots[(CCHMetricFamily::<E, V, K>::fnv1a(vehID) % self.myEnsembleK as u64) as usize]
        };
        c.front
    }

    /// C++: `bool CCHMetricFamily::empty() const` [const]
    /// whether any type has live metric state yet
    // bool empty() const {
    //         return myLiveMetrics.empty();
    //     }
    #[allow(unused_variables)]
    pub fn empty(&self) -> bool {
        self.myLiveMetrics.is_empty()
    }

    /// C++: `int CCHMetricFamily::periodOf(SUMOTime) const` [const, private]
    /// the weight period containing time: 0 is everything before begin and the whole run when the weights are static
    // int periodOf(SUMOTime time) const {
    //         if (myWeightPeriod > 0 && myWeightPeriod != SUMOTime_MAX && time > myBegin) {
    //             return (int)((time - myBegin) / myWeightPeriod);
    //         }
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn period_of(&self, time: SUMOTime) -> i32 {
        if self.myWeightPeriod > 0 && self.myWeightPeriod != SUMOTIME_MAX && time > self.myBegin {
            return ((time - self.myBegin) / self.myWeightPeriod) as i32;
        }
        return 0;
    }

    /// C++: `void CCHMetricFamily::fillStatic(StaticMetric &, int, SUMOTime, const V *, const V *)` [private]
    /// (re)fill a STATIC metric's input weights from the live efforts. A weight-period grid evaluates at the period's begin, so each pair is customized once per fill; a single period evaluates at the query time -- free-flow efforts are time-invariant anyway, and marouter's per-iteration travel times are exactly what a Dijkstra query at that time sees. The patch runs once, on the first fill, and what it wrote is re-applied on every refill: the masked set depends only on the metric's type, whereas the vehicle triggering a refill may be of another type.
    // void fillStatic(StaticMetric& sm, int period, SUMOTime time, const V* ref, const V* veh) {
    //         const double fillTime = myWeightPeriod == SUMOTime_MAX
    //                                 ? STEPS2TIME(time) : STEPS2TIME(myBegin + period * myWeightPeriod);
    //         myGraph->fillInputWeights(myFillEffort, sm.vClass, ref, fillTime, sm.weights);
    //         if (!sm.filled) {
    //             sm.filled = true;
    //             if (myPatch != nullptr) {
    //                 const std::vector<unsigned> unpatched(sm.weights);
    //                 myPatch(myGraph, veh, sm.weights);
    //                 for (unsigned a = 0; a < (unsigned)sm.weights.size(); a++) {
    //                     if (sm.weights[a] != unpatched[a]) {
    //                         sm.patched.emplace_back(a, sm.weights[a]);
    //                     }
    //                 }
    //             }
    //         } else {
    //             for (const auto& p : sm.patched) {
    //                 sm.weights[p.first] = p.second;
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn fill_static(&self, sm: &mut CCHMetricFamily_StaticMetric<V>, period: i32, time: SUMOTime, ref_: Ptr<V>, veh: Ptr<V>) {
        let fillTime = if self.myWeightPeriod == utils::common::time::SUMOTIME_MAX {
            utils::common::time::steps2time(time)
        } else {
            utils::common::time::steps2time(self.myBegin + period as SUMOTime * self.myWeightPeriod)
        };
        self.myGraph.fill_input_weights(self.myFillEffort, sm.vClass, ref_, fillTime, &mut sm.weights);
        if !sm.filled {
            sm.filled = true;
            if let Some(patch) = self.myPatch {
                let unpatched = sm.weights.clone();
                patch(self.myGraph, veh, &mut sm.weights);
                for a in 0..sm.weights.len() as u32 {
                    if sm.weights[a as usize] != unpatched[a as usize] {
                        sm.patched.push((a, sm.weights[a as usize]));
                    }
                }
            }
        } else {
            for &(a, w) in &sm.patched {
                sm.weights[a as usize] = w;
            }
        }
    }

    /// C++: `uint64_t CCHMetricFamily::fnv1a(const std::string &)` [static, private]
    /// stable 64-bit FNV-1a for the ensemble slot assignment
    // static uint64_t fnv1a(const std::string& s) {
    //         uint64_t h = 1469598103934665603ull;
    //         for (const char ch : s) {
    //             h = (h ^ (unsigned char)ch) * 1099511628211ull;
    //         }
    //         return h;
    //     }
    #[allow(unused_variables)]
    pub fn fnv1a(s: &str) -> u64 {
        let mut h: u64 = 1469598103934665603;
        for ch in s.bytes() {
            h = (h ^ ch as u64).wrapping_mul(1099511628211);
        }
        h
    }

    /// C++: `LiveMetric * CCHMetricFamily::buildLiveMetric(const K *, int)` [private]
    /// allocate LIVE metric state for one (type, ensemble slot) (owner's thread only)
    // LiveMetric* buildLiveMetric(const K* key, int slot) {
    //         LiveMetric* c = new LiveMetric();
    //         c->refVehicle = myFactory(key, slot);
    //         c->vClass = c->refVehicle->getVClass();
    //         c->weight[0].resize(myGraph->arcCount());
    //         c->weight[1].resize(myGraph->arcCount());
    //         c->partial = std::make_shared<RoutingKit::CustomizableContractionHierarchyPartialCustomization>(myGraph->cch());
    //         myLiveMetrics.push_back(c);
    //         return c;
    //     }
    #[allow(unused_variables)]
    pub fn build_live_metric(&mut self, key: Ptr<K>, slot: i32) -> Ptr<CCHMetricFamily_LiveMetric<V>> {
        let refVehicle = (self.myFactory.expect("LIVE CCHMetricFamily without reference-vehicle factory"))(key, slot);
        let n = self.myGraph.arc_count() as usize;
        let c = Box::new(CCHMetricFamily_LiveMetric {
            weight: [vec![0; n], vec![0; n]],
            metric: [None, None],
            front: Ptr::null(),
            frontIndex: 1, // first customize uses back = 0
            vClass: refVehicle.v_class(),
            refVehicle,
            partial: Some(Box::new(CustomizableContractionHierarchyPartialCustomization::new(static_cch(self.myGraph)))),
        });
        let c = Ptr::from_raw(Box::into_raw(c));
        self.myLiveMetrics.push(c);
        c
    }

    // C++: `void CCHMetricFamily::CCHMetricFamily<E, V, K>(const CCHMetricFamily<E, V, K> &)` [private]
    // @}
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    // deleted in C++ (= delete): no Rust counterpart

    // C++: `CCHMetricFamily<E, V, K> & CCHMetricFamily::operator=(const CCHMetricFamily<E, V, K> &)` [private]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    // deleted in C++ (= delete): no Rust counterpart

}

/// per-type LIVE metric state. Heap-owned: the atomic makes it non-movable, so it cannot live in a map by value.
/// C++ class `CCHMetricFamily::LiveMetric` (utils/router/CCHMetricFamily.h)
pub struct CCHMetricFamily_LiveMetric<V> {
    pub weight: [Vec<u32>; 2],
    pub metric: [Option<Box<CCHMetric>>; 2],
    pub front: CCHMetricFamily_MetricPtr,
    pub frontIndex: i32,
    pub vClass: SUMOVehicleClass,
    /// OWNED effort-reference vehicle (see RefVehicleFactory)
    pub refVehicle: Ptr<V>,
    /// partial-customization worker (queue over the shared CCH); one per metric -- its queue is drained by every customize()
    pub partial: Option<Box<CustomizableContractionHierarchyPartialCustomization<'static>>>,
}

impl<V> CCHMetricFamily_LiveMetric<V> {
}

/// per-(type, period) STATIC metric state. RoutingKit metrics BORROW their input-weight buffer, so buffer and metric live together in the map (std::map nodes are address-stable).
/// C++ class `CCHMetricFamily::StaticMetric` (utils/router/CCHMetricFamily.h)
pub struct CCHMetricFamily_StaticMetric<V> {
    pub weights: Vec<u32>,
    pub metric: Option<Box<CCHMetric>>,
    pub vClass: SUMOVehicleClass,
    /// OWNED reference vehicle (nullptr when filling with the querying vehicle)
    pub refVehicle: Ptr<V>,
    /// whether the first fill (which runs the patch) is done
    pub filled: bool,
    /// the (arc, weight) pairs the patch wrote on the first fill, re-applied on every refill
    pub patched: Vec<(u32, u32)>,
}

impl<V> CCHMetricFamily_StaticMetric<V> {
}

