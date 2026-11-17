//! Port of utils/router/CHRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::{RouterEdge, RouterVehicle};

/// C++ typedef `int Unidirectional::ConnectionVector`
pub type Unidirectional_ConnectionVector = Vec<CHBuilder_Connection>;

/// C++ typedef `std::pair<const typename SUMOAbstractRouter<E, V>::EdgeInfo *, const typename SUMOAbstractRouter<E, V>::EdgeInfo *> CHRouter::Meeting`
pub type CHRouter_Meeting = (Ptr<SUMOAbstractRouter_EdgeInfo>, Ptr<SUMOAbstractRouter_EdgeInfo>);

pub(crate) fn v_class<V: RouterVehicle>(v: Ptr<V>) -> SUMOVehicleClass {
    v.v_class()
}

fn msg_instance(warning: bool) -> Ptr<MsgHandler> {
    utils::common::msg_handler::with_instance(if warning { utils::common::msg_handler::MsgType::Warning } else { utils::common::msg_handler::MsgType::Error }, |h| Ptr::from_mut(h))
}

/// Computes the shortest path through a contracted network
/// C++ class `CHRouter` (utils/router/CHRouter.h)
#[repr(C)]
pub struct CHRouter<E, V> {
    /// C++ base class `SUMOAbstractRouter<E, V>`
    pub base: SUMOAbstractRouter<E, V>,
    /// all edges with numerical ids
    pub myEdges: Ptr<Vec<Ptr<E>>>,
    /// the unidirectional search queues
    pub myForwardSearch: CHRouter_Unidirectional<E, V>,
    pub myBackwardSearch: CHRouter_Unidirectional<E, V>,
    pub myHierarchyBuilder: Ptr<CHBuilder<E, V>>,
    pub myHierarchy: Ptr<CHBuilder_Hierarchy<E>>,
    /// the validity duration of one weight interval
    pub myWeightPeriod: SUMOTime,
    /// the validity duration of the current hierarchy (exclusive)
    pub myValidUntil: SUMOTime,
    /// the permissions for which the hierarchy was constructed
    pub mySVC: SUMOVehicleClass,
    _p: std::marker::PhantomData<(E, V,)>,
}

impl<E, V> std::ops::Deref for CHRouter<E, V> {
    type Target = SUMOAbstractRouter<E, V>;
    fn deref(&self) -> &SUMOAbstractRouter<E, V> {
        &self.base
    }
}
impl<E, V> std::ops::DerefMut for CHRouter<E, V> {
    fn deref_mut(&mut self) -> &mut SUMOAbstractRouter<E, V> {
        &mut self.base
    }
}

/// The `CHRouter::compute` reached from the base router's compute slot (C++ virtual dispatch)
pub fn ch_compute<E: RouterEdge<V>, V: RouterVehicle>(base: &mut SUMOAbstractRouter<E, V>, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool {
    Ptr::from_mut(base).cast::<CHRouter<E, V>>().m().compute(from, to, vehicle, msTime, into, silent)
}

pub fn ch_prohibit<E: RouterEdge<V>, V: RouterVehicle>(base: &mut SUMOAbstractRouter<E, V>, toProhibit: &SUMOAbstractRouter_Prohibitions) {
    Ptr::from_mut(base).cast::<CHRouter<E, V>>().m().prohibit(toProhibit)
}

impl<E: RouterEdge<V>, V: RouterVehicle> CHRouter<E, V> {
    /// Installs this router's compute and prohibit into the base, so a `SUMOAbstractRouter` pointer (C++ virtual calls) reaches them.
    pub fn with_compute(mut self) -> Self {
        self.base.myCompute = Some(ch_compute::<E, V>);
        self.base.myProhibit = Some(ch_prohibit::<E, V>);
        self
    }
}

impl<E: RouterEdge<V>, V: RouterVehicle> CHRouter<E, V> {
    /// C++: `void CHRouter::CHRouter<E, V>(const int &, bool, typename SUMOAbstractRouter<E, V>::Operation, const SUMOVehicleClass, SUMOTime, const bool, const bool)`
    /// Constructor
    // CHRouter(const std::vector<E*>& edges, bool unbuildIsWarning, typename SUMOAbstractRouter<E, V>::Operation operation,
    //              const SUMOVehicleClass svc,
    //              SUMOTime weightPeriod,
    //              const bool havePermissions, const bool haveRestrictions):
    //         SUMOAbstractRouter<E, V>("CHRouter", unbuildIsWarning, operation, nullptr, havePermissions, haveRestrictions),
    //         myEdges(edges),
    //         myForwardSearch(edges, true),
    //         myBackwardSearch(edges, false),
    //         myHierarchyBuilder(new CHBuilder<E, V>(edges, unbuildIsWarning, svc, havePermissions)),
    //         myHierarchy(nullptr),
    //         myWeightPeriod(weightPeriod),
    //         myValidUntil(0),
    //         mySVC(svc) {
    //     }
    #[allow(unused_variables)]
    pub fn new(edges: &Vec<Ptr<E>>, unbuildIsWarning: bool, operation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, svc: SUMOVehicleClass, weightPeriod: SUMOTime, havePermissions: bool, haveRestrictions: bool) -> Self {
        Self {
            base: SUMOAbstractRouter::new("CHRouter", unbuildIsWarning, operation, None, havePermissions, haveRestrictions),
            myEdges: Ptr::from_raw(edges as *const Vec<Ptr<E>> as *mut Vec<Ptr<E>>),
            myForwardSearch: CHRouter_Unidirectional::new(edges, true),
            myBackwardSearch: CHRouter_Unidirectional::new(edges, false),
            myHierarchyBuilder: Ptr::from_raw(Box::into_raw(Box::new(CHBuilder::new(edges, unbuildIsWarning, svc, havePermissions)))),
            myHierarchy: Ptr::null(),
            myWeightPeriod: weightPeriod,
            myValidUntil: 0,
            mySVC: svc,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void CHRouter::CHRouter<E, V>(const int &, bool, typename SUMOAbstractRouter<E, V>::Operation, const SUMOVehicleClass, typename CHBuilder<E, V>::Hierarchy *, const bool, const bool)`
    /// Cloning constructor, should be used only for time independent instances which build a hierarchy only once
    // CHRouter(const std::vector<E*>& edges, bool unbuildIsWarning, typename SUMOAbstractRouter<E, V>::Operation operation,
    //              const SUMOVehicleClass svc,
    //              typename CHBuilder<E, V>::Hierarchy* hierarchy,
    //              const bool havePermissions, const bool haveRestrictions) :
    //         SUMOAbstractRouter<E, V>("CHRouterClone", unbuildIsWarning, operation, nullptr, havePermissions, haveRestrictions),
    //         myEdges(edges),
    //         myForwardSearch(edges, true),
    //         myBackwardSearch(edges, false),
    //         myHierarchyBuilder(nullptr),
    //         myHierarchy(hierarchy),
    //         myWeightPeriod(SUMOTime_MAX),
    //         myValidUntil(SUMOTime_MAX),
    //         mySVC(svc) {
    //     }
    #[allow(unused_variables)]
    pub fn new_2(edges: &Vec<Ptr<E>>, unbuildIsWarning: bool, operation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, svc: SUMOVehicleClass, hierarchy: Ptr<CHBuilder_Hierarchy<E>>, havePermissions: bool, haveRestrictions: bool) -> Self {
        Self {
            base: SUMOAbstractRouter::new("CHRouterClone", unbuildIsWarning, operation, None, havePermissions, haveRestrictions),
            myEdges: Ptr::from_raw(edges as *const Vec<Ptr<E>> as *mut Vec<Ptr<E>>),
            myForwardSearch: CHRouter_Unidirectional::new(edges, true),
            myBackwardSearch: CHRouter_Unidirectional::new(edges, false),
            myHierarchyBuilder: Ptr::null(),
            myHierarchy: hierarchy,
            myWeightPeriod: SUMOTIME_MAX,
            myValidUntil: SUMOTIME_MAX,
            mySVC: svc,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void CHRouter::~CHRouter<E, V>()` [virtual]
    /// Destructor
    // virtual ~CHRouter() {
    //         if (myHierarchyBuilder != nullptr) {
    //             delete myHierarchy;
    //             delete myHierarchyBuilder;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if !self.myHierarchyBuilder.is_null() {
            unsafe { drop(Box::from_raw(self.myHierarchy.as_raw())) };
            let mut builder = unsafe { Box::from_raw(self.myHierarchyBuilder.as_raw()) };
            builder.drop_();
        }
    }

    /// C++: `SUMOAbstractRouter<E, V> * CHRouter::clone()` [virtual]
    // virtual SUMOAbstractRouter<E, V>* clone() {
    //         if (myWeightPeriod == SUMOTime_MAX && myHierarchy != nullptr) {
    //             // we only need one hierarchy
    //             return new CHRouter<E, V>(myEdges, this->myErrorMsgHandler == MsgHandler::getWarningInstance(), this->myOperation,
    //                                       mySVC, myHierarchy, this->myHavePermissions, this->myHaveRestrictions);
    //         }
    //         return new CHRouter<E, V>(myEdges, this->myErrorMsgHandler == MsgHandler::getWarningInstance(), this->myOperation,
    //                                   mySVC, myWeightPeriod, this->myHavePermissions, this->myHaveRestrictions);
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, V>> {
        let unbuildIsWarning = self.myErrorMsgHandler == msg_instance(true);
        if self.myWeightPeriod == utils::common::time::SUMOTIME_MAX && !self.myHierarchy.is_null() {
            // we only need one hierarchy
            let cloned: CHRouter<E, V> = CHRouter::new_2(self.myEdges.m(), unbuildIsWarning, self.myOperation, self.mySVC, self.myHierarchy, self.myHavePermissions, self.myHaveRestrictions);
        return Ptr::from_raw(Box::into_raw(Box::new(cloned))).cast();
        }
        let cloned2: CHRouter<E, V> = CHRouter::new(self.myEdges.m(), unbuildIsWarning, self.myOperation, self.mySVC, self.myWeightPeriod, self.myHavePermissions, self.myHaveRestrictions);
        Ptr::from_raw(Box::into_raw(Box::new(cloned2))).cast()
    }

    /// C++: `void CHRouter::prohibit(const int &)` [virtual]
    // virtual void prohibit(const std::map<const E*, RouterProhibition>& toProhibit) {
    //         if (toProhibit.size() > 0) {
    //             WRITE_WARNINGF(TL("Routing algorithm CH does not support dynamic closing of edges%"), "");
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn prohibit(&mut self, toProhibit: &SUMOAbstractRouter_Prohibitions) {
        if toProhibit.len() > 0 {
            utils::common::msg_handler::write_warning("Routing algorithm CH does not support dynamic closing of edges");
        }
    }

    /// C++: `bool CHRouter::supportsProhibitions() const` [const, virtual]
    // virtual bool supportsProhibitions() const {
    //         WRITE_WARNINGF(TL("Routing algorithm CH does not support dynamic closing of edges and lanes%"), "");
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn supports_prohibitions(&self) -> bool {
        utils::common::msg_handler::write_warning("Routing algorithm CH does not support dynamic closing of edges and lanes");
        false
    }

    /// C++: `void CHRouter::reset(const V *const)` [virtual]
    /// trigger hierarchy rebuild
    // virtual void reset(const V* const vehicle) {
    //         if (myValidUntil == 0) {
    //             myValidUntil = myWeightPeriod;
    //         }
    //         typename CHBuilder<E, V>::Hierarchy* newHierarchy = myHierarchyBuilder->buildContractionHierarchy(myValidUntil - myWeightPeriod, vehicle, this);
    //         if (myHierarchy == nullptr) {
    //             myHierarchy = newHierarchy;
    //         } else {
    //             *myHierarchy = *newHierarchy;
    //             delete newHierarchy;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn reset(&mut self, vehicle: Ptr<V>) {
        if self.myValidUntil == 0 {
            self.myValidUntil = self.myWeightPeriod;
        }
        let hierarchyBuilder = self.myHierarchyBuilder;
        let weightWindow = self.myValidUntil - self.myWeightPeriod;
        let selfPtr = Ptr::from_mut(self).cast();
        let newHierarchy = hierarchyBuilder.m().build_contraction_hierarchy(weightWindow, vehicle, selfPtr);
        if self.myHierarchy.is_null() {
            self.myHierarchy = newHierarchy;
        } else {
            *self.myHierarchy.m() = *unsafe { Box::from_raw(newHierarchy.as_raw()) };
        }
    }

    /// C++: `bool CHRouter::compute(const E *, const E *, const V *const, SUMOTime, int &, bool)` [virtual]
    /// Builds the route between the given edges using the minimum traveltime in the contracted graph
    // virtual bool compute(const E* from, const E* to, const V* const vehicle,
    //                          SUMOTime msTime, std::vector<const E*>& into, bool silent = false) {
    //         assert(from != nullptr && to != nullptr);
    //         // assert(myHierarchyBuilder.mySPTree->validatePermissions() || vehicle->getVClass() == mySVC || mySVC == SVC_IGNORING);
    //         // do we need to rebuild the hierarchy?
    //         if (msTime >= myValidUntil) {
    //             assert(myHierarchyBuilder != nullptr); // only time independent clones do not have a builder
    //             while (msTime >= myValidUntil) {
    //                 myValidUntil += myWeightPeriod;
    //             }
    //             this->reset(vehicle);
    //         }
    //         // ready for routing
    //         this->startQuery();
    //         myForwardSearch.init(from, vehicle);
    //         myBackwardSearch.init(to, vehicle);
    //         double minTTSeen = std::numeric_limits<double>::max();
    //         Meeting meeting(nullptr, nullptr);
    //         bool continueForward = true;
    //         bool continueBackward = true;
    //         int num_visited_fw = 0;
    //         int num_visited_bw = 0;
    //         bool result = true;
    //         while (continueForward || continueBackward) {
    //             if (continueForward) {
    //                 continueForward = myForwardSearch.step(myHierarchy->forwardUplinks, myBackwardSearch, minTTSeen, meeting);
    //                 num_visited_fw += 1;
    //             }
    //             if (continueBackward) {
    //                 continueBackward = myBackwardSearch.step(myHierarchy->backwardUplinks, myForwardSearch, minTTSeen, meeting);
    //                 num_visited_bw += 1;
    //             }
    //         }
    //         if (minTTSeen < std::numeric_limits<double>::max()) {
    //             buildPathFromMeeting(meeting, into);
    //         } else {
    //             if (!silent) {
    //                 this->myErrorMsgHandler->informf(TL("No connection between edge '%' and edge '%' found."), from->getID(), to->getID());
    //             }
    //             result = false;
    //         }
    // #ifdef CHRouter_DEBUG_QUERY_PERF
    //         std::cout << "visited " << num_visited_fw + num_visited_bw << " edges (" << num_visited_fw << "," << num_visited_bw << ") ,final path length: " + toString(into.size()) + ")\n";
    // #endif
    //         this->endQuery(num_visited_bw + num_visited_fw);
    //         return result;
    //     }
    #[allow(unused_variables)]
    pub fn compute(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool {
        assert!(!from.is_null() && !to.is_null());
        // assert(myHierarchyBuilder.mySPTree->validatePermissions() || vehicle->getVClass() == mySVC || mySVC == SVC_IGNORING);
        // do we need to rebuild the hierarchy?
        if msTime >= self.myValidUntil {
            assert!(!self.myHierarchyBuilder.is_null()); // only time independent clones do not have a builder
            while msTime >= self.myValidUntil {
                self.myValidUntil += self.myWeightPeriod;
            }
            self.reset(vehicle);
        }
        // ready for routing
        self.start_query();
        self.myForwardSearch.init(from, vehicle);
        self.myBackwardSearch.init(to, vehicle);
        let mut minTTSeen = f64::MAX;
        let mut meeting: CHRouter_Meeting = (Ptr::null(), Ptr::null());
        let mut continueForward = true;
        let mut continueBackward = true;
        let mut num_visited_fw = 0;
        let mut num_visited_bw = 0;
        let mut result = true;
        while continueForward || continueBackward {
            if continueForward {
                continueForward = self.myForwardSearch.step(&self.myHierarchy.m().forwardUplinks, &self.myBackwardSearch, &mut minTTSeen, &mut meeting);
                num_visited_fw += 1;
            }
            if continueBackward {
                continueBackward = self.myBackwardSearch.step(&self.myHierarchy.m().backwardUplinks, &self.myForwardSearch, &mut minTTSeen, &mut meeting);
                num_visited_bw += 1;
            }
        }
        if minTTSeen < f64::MAX {
            self.build_path_from_meeting(meeting, into);
        } else {
            if !silent {
                self.myErrorMsgHandler.m().informf("No connection between edge '%' and edge '%' found.", &[&e_id(from), &e_id(to)]);
            }
            result = false;
        }
        // #ifdef CHRouter_DEBUG_QUERY_PERF: "visited N edges (fw,bw) ,final path length" (debug only, off)
        self.end_query(num_visited_bw + num_visited_fw);
        result
    }

    /// C++: `void CHRouter::buildPathFromMeeting(Meeting, int &) const` [const]
    /// Builds the path from marked edges
    // void buildPathFromMeeting(Meeting meeting, std::vector<const E*>& into) const {
    //         std::deque<const E*> tmp;
    //         const auto* backtrack = meeting.first;
    //         while (backtrack != 0) {
    //             tmp.push_front((E*) backtrack->edge);  // !!!
    //             backtrack = backtrack->prev;
    //         }
    //         backtrack = meeting.second->prev; // don't use central edge twice
    //         while (backtrack != 0) {
    //             tmp.push_back((E*) backtrack->edge);  // !!!
    //             backtrack = backtrack->prev;
    //         }
    //         // expand shortcuts
    //         const E* prev = 0;
    //         while (!tmp.empty()) {
    //             const E* cur = tmp.front();
    //             tmp.pop_front();
    //             if (prev == 0) {
    //                 into.push_back(cur);
    //                 prev = cur;
    //             } else {
    //                 const E* via = getVia(prev, cur);
    //                 if (via == 0) {
    //                     into.push_back(cur);
    //                     prev = cur;
    //                 } else {
    //                     tmp.push_front(cur);
    //                     tmp.push_front(via);
    //                 }
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn build_path_from_meeting(&self, meeting: CHRouter_Meeting, into: &mut Vec<Ptr<E>>) {
        let mut tmp: std::collections::VecDeque<Ptr<E>> = std::collections::VecDeque::new();
        let mut backtrack = meeting.0;
        while !backtrack.is_null() {
            tmp.push_front(backtrack.m().edge.cast::<E>()); // !!!
            backtrack = backtrack.m().prev;
        }
        backtrack = meeting.1.m().prev; // don't use central edge twice
        while !backtrack.is_null() {
            tmp.push_back(backtrack.m().edge.cast::<E>()); // !!!
            backtrack = backtrack.m().prev;
        }
        // expand shortcuts
        let mut prev: Ptr<E> = Ptr::null();
        while let Some(cur) = tmp.pop_front() {
            if prev.is_null() {
                into.push(cur);
                prev = cur;
            } else {
                let via = self.get_via(prev, cur);
                if via.is_null() {
                    into.push(cur);
                    prev = cur;
                } else {
                    tmp.push_front(cur);
                    tmp.push_front(via);
                }
            }
        }
    }

    /// C++: `const E * CHRouter::getVia(const E *, const E *) const` [const, private]
    // const E* getVia(const E* forwardFrom, const E* forwardTo) const {
    //         typename CHBuilder<E, V>::ConstEdgePair forward(forwardFrom, forwardTo);
    //         typename CHBuilder<E, V>::ShortcutVia::const_iterator it = myHierarchy->shortcuts.find(forward);
    //         if (it != myHierarchy->shortcuts.end()) {
    //             return it->second;
    //         } else {
    //             return 0;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_via(&self, forwardFrom: Ptr<E>, forwardTo: Ptr<E>) -> Ptr<E> {
        let forward = (forwardFrom, forwardTo);
        match self.myHierarchy.m().shortcuts.get(&forward) {
            Some(via) => *via,
            None => Ptr::null(),
        }
    }

}

/// Class to compare (and so sort) nodes by their effort
/// C++ class `Unidirectional::EdgeInfoByTTComparator` (utils/router/CHRouter.h)
pub struct Unidirectional_EdgeInfoByTTComparator {
}

impl Unidirectional_EdgeInfoByTTComparator {
    /// C++: `bool EdgeInfoByTTComparator::operator()(const typename SUMOAbstractRouter<E, V>::EdgeInfo *, const typename SUMOAbstractRouter<E, V>::EdgeInfo *) const` [const]
    /// Comparing method
    // bool operator()(const typename SUMOAbstractRouter<E, V>::EdgeInfo* nod1, const typename SUMOAbstractRouter<E, V>::EdgeInfo* nod2) const {
    //                 if (nod1->effort == nod2->effort) {
    //                     return nod1->edge->getNumericalID() > nod2->edge->getNumericalID();
    //                 }
    //                 return nod1->effort > nod2->effort;
    //             }
    #[allow(unused_variables)]
    pub fn call_op<E: RouterEdge<V>, V>(&self, nod1: Ptr<SUMOAbstractRouter_EdgeInfo>, nod2: Ptr<SUMOAbstractRouter_EdgeInfo>) -> bool {
        if nod1.m().effort == nod2.m().effort {
            e_numerical_id(nod1.m().edge.cast::<E>()) > e_numerical_id(nod2.m().edge.cast::<E>())
        } else {
            nod1.m().effort > nod2.m().effort
        }
    }

}

/// class for searching in one direction
/// C++ class `CHRouter::Unidirectional` (utils/router/CHRouter.h)
pub struct CHRouter_Unidirectional<E, V> {
    /// the role of this search
    pub myAmForward: bool,
    /// the min edge heap
    pub myFrontier: Vec<Ptr<SUMOAbstractRouter_EdgeInfo>>,
    /// the set of visited (settled) Edges
    pub myFound: std::collections::BTreeSet<Ptr<E>>,
    /// The container of edge information
    pub myEdgeInfos: Vec<SUMOAbstractRouter_EdgeInfo>,
    pub myComparator: Unidirectional_EdgeInfoByTTComparator,
    pub myVehicle: Ptr<V>,
}

impl<E: RouterEdge<V>, V: RouterVehicle> CHRouter_Unidirectional<E, V> {
    /// C++: `void Unidirectional::Unidirectional(const int &, bool)`
    /// Constructor
    // Unidirectional(const std::vector<E*>& edges, bool forward):
    //             myAmForward(forward),
    //             myVehicle(0) {
    //             for (const E* const e : edges) {
    //                 myEdgeInfos.push_back(typename SUMOAbstractRouter<E, V>::EdgeInfo(e));
    //             }
    //         }
    #[allow(unused_variables)]
    pub fn new(edges: &Vec<Ptr<E>>, forward: bool) -> Self {
        Self {
            myAmForward: forward,
            myFrontier: Vec::new(),
            myFound: std::collections::BTreeSet::new(),
            // shortcut: SUMOAbstractRouter::EdgeInfo.edge is Unmapped (), so edges are stored with cast(); upgrade when EdgeInfo is generic over E
            myEdgeInfos: edges.iter().map(|e| SUMOAbstractRouter_EdgeInfo::new(e.cast())).collect(),
            myComparator: Unidirectional_EdgeInfoByTTComparator {},
            myVehicle: Ptr::null(),
        }
    }

    /// C++: `bool Unidirectional::found(const E *const) const` [const]
    // inline bool found(const E* const edge) const {
    //             return myFound.count(edge) > 0;
    //         }
    #[allow(unused_variables)]
    pub fn found(&self, edge: Ptr<E>) -> bool {
        self.myFound.contains(&edge)
    }

    /// C++: `typename SUMOAbstractRouter<E, V>::EdgeInfo * Unidirectional::getEdgeInfo(const E *const)`
    // inline typename SUMOAbstractRouter<E, V>::EdgeInfo* getEdgeInfo(const E* const edge) {
    //             return &(myEdgeInfos[edge->getNumericalID()]);
    //         }
    #[allow(unused_variables)]
    pub fn get_edge_info(&mut self, edge: Ptr<E>) -> Ptr<SUMOAbstractRouter_EdgeInfo> {
        Ptr::from_mut(&mut self.myEdgeInfos[e_numerical_id(edge) as usize])
    }

    /// C++: `const typename SUMOAbstractRouter<E, V>::EdgeInfo * Unidirectional::getEdgeInfo(const E *const) const` [const]
    // inline const typename SUMOAbstractRouter<E, V>::EdgeInfo* getEdgeInfo(const E* const edge) const {
    //             return &(myEdgeInfos[edge->getNumericalID()]);
    //         }
    #[allow(unused_variables)]
    pub fn get_edge_info_2(&self, edge: Ptr<E>) -> Ptr<SUMOAbstractRouter_EdgeInfo> {
        Ptr::new(&self.myEdgeInfos[e_numerical_id(edge) as usize])
    }

    /// C++: `void Unidirectional::init(const E *const, const V *const)`
    // void init(const E* const start, const V* const vehicle) {
    //             assert(vehicle != 0);
    //             // all EdgeInfos touched in the previous query are either in myFrontier or myFound: clean those up
    //             for (auto ei : myFrontier) {
    //                 ei->reset();
    //             }
    //             myFrontier.clear();
    //             for (auto edge : myFound) {
    //                 getEdgeInfo(edge)->reset();
    //             }
    //             myFound.clear();
    //             myVehicle = vehicle;
    //             auto startInfo = getEdgeInfo(start);
    //             startInfo->effort = 0.;
    //             startInfo->prev = nullptr;
    //             myFrontier.push_back(startInfo);
    //         }
    #[allow(unused_variables)]
    pub fn init(&mut self, start: Ptr<E>, vehicle: Ptr<V>) {
        assert!(!vehicle.is_null());
        // all EdgeInfos touched in the previous query are either in myFrontier or myFound: clean those up
        for ei in self.myFrontier.iter() {
            ei.m().reset();
        }
        self.myFrontier.clear();
        for edge in self.myFound.clone() {
            self.get_edge_info(edge).m().reset();
        }
        self.myFound.clear();
        self.myVehicle = vehicle;
        let startInfo = self.get_edge_info(start);
        startInfo.m().effort = 0.;
        startInfo.m().prev = Ptr::null();
        self.myFrontier.push(startInfo);
    }

    /// C++: `bool Unidirectional::step(const int &, const Unidirectional &, double &, Meeting &)`
    /// explore on element from the frontier,update minTTSeen and meeting if an EdgeInfo found by the otherSearch is encountered returns whether stepping should continue
    // bool step(const std::vector<ConnectionVector>& uplinks, const Unidirectional& otherSearch, double& minTTSeen, Meeting& meeting) {
    //             // pop the node with the minimal length
    //             auto* const minimumInfo = myFrontier.front();
    //             std::pop_heap(myFrontier.begin(), myFrontier.end(), myComparator);
    //             myFrontier.pop_back();
    //             // check for a meeting with the other search
    //             const E* const minEdge = minimumInfo->edge;
    // #ifdef CHRouter_DEBUG_QUERY
    //             std::cout << "DEBUG: " << (myAmForward ? "Forward" : "Backward") << " hit '" << minEdge->getID() << "' Q: ";
    //             for (typename std::vector<EdgeInfo*>::iterator it = myFrontier.begin(); it != myFrontier.end(); it++) {
    //                 std::cout << (*it)->traveltime << "," << (*it)->edge->getID() << " ";
    //             }
    //             std::cout << "\n";
    // #endif
    //             if (otherSearch.found(minEdge)) {
    //                 const auto* const otherInfo = otherSearch.getEdgeInfo(minEdge);
    //                 const double ttSeen = minimumInfo->effort + otherInfo->effort;
    // #ifdef CHRouter_DEBUG_QUERY
    //                 std::cout << "DEBUG: " << (myAmForward ? "Forward" : "Backward") << "-Search hit other search at '" << minEdge->getID() << "', tt: " << ttSeen << " \n";
    // #endif
    //                 if (ttSeen < minTTSeen) {
    //                     minTTSeen = ttSeen;
    //                     if (myAmForward) {
    //                         meeting.first = minimumInfo;
    //                         meeting.second = otherInfo;
    //                     } else {
    //                         meeting.first = otherInfo;
    //                         meeting.second = minimumInfo;
    //                     }
    //                 }
    //             }
    //             // prepare next steps
    //             minimumInfo->visited = true;
    //             // XXX we only need to keep found elements if they have a higher rank than the lowest rank in the other search queue
    //             myFound.insert(minimumInfo->edge);
    //             for (const auto& uplink : uplinks[minEdge->getNumericalID()]) {
    //                 const auto upwardInfo = &myEdgeInfos[uplink.target];
    //                 const double effort = minimumInfo->effort + uplink.cost;
    //                 const SUMOVehicleClass svc = myVehicle->getVClass();
    //                 // check whether it can be used
    //                 if ((uplink.permissions & svc) != svc) {
    //                     continue;
    //                 }
    //                 const double oldEffort = upwardInfo->effort;
    //                 if (!upwardInfo->visited && effort < oldEffort) {
    //                     upwardInfo->effort = effort;
    //                     upwardInfo->prev = minimumInfo;
    //                     if (oldEffort == std::numeric_limits<double>::max()) {
    //                         myFrontier.push_back(upwardInfo);
    //                         std::push_heap(myFrontier.begin(), myFrontier.end(), myComparator);
    //                     } else {
    //                         std::push_heap(myFrontier.begin(),
    //                                        std::find(myFrontier.begin(), myFrontier.end(), upwardInfo) + 1,
    //                                        myComparator);
    //                     }
    //                 }
    //             }
    //             // @note: this effectively does a full dijkstra search.
    //             // the effort compared to the naive stopping criterion is thus
    //             // quadrupled. We could implement a better stopping criterion (Holte)
    //             // However since the search shall take place in a contracted graph
    //             // it probably does not matter
    //             return !myFrontier.empty() && myFrontier.front()->effort < minTTSeen;
    //         }
    #[allow(unused_variables)]
    pub fn step(&mut self, uplinks: &Vec<Vec<CHBuilder_Connection>>, otherSearch: &CHRouter_Unidirectional<E, V>, minTTSeen: &mut f64, meeting: &mut CHRouter_Meeting) -> bool {
        // pop the node with the minimal length
        let minimumInfo = self.myFrontier[0];
        std_pop_heap(&mut self.myFrontier, &|a, b| self.myComparator.call_op::<E, V>(a, b));
        self.myFrontier.pop();
        // check for a meeting with the other search
        let minEdge = minimumInfo.m().edge.cast::<E>();
        if otherSearch.found(minEdge) {
            let otherInfo = otherSearch.get_edge_info_2(minEdge);
            let ttSeen = minimumInfo.m().effort + otherInfo.m().effort;
            if ttSeen < *minTTSeen {
                *minTTSeen = ttSeen;
                if self.myAmForward {
                    meeting.0 = minimumInfo;
                    meeting.1 = otherInfo;
                } else {
                    meeting.0 = otherInfo;
                    meeting.1 = minimumInfo;
                }
            }
        }
        // prepare next steps
        minimumInfo.m().visited = true;
        // XXX we only need to keep found elements if they have a higher rank than the lowest rank in the other search queue
        self.myFound.insert(minEdge);
        for uplink in uplinks[e_numerical_id(minEdge) as usize].iter() {
            let upwardInfo = Ptr::from_mut(&mut self.myEdgeInfos[uplink.target as usize]);
            let effort = minimumInfo.m().effort + uplink.cost;
            let svc = v_class(self.myVehicle);
            // check whether it can be used
            if (uplink.permissions & svc) != svc {
                continue;
            }
            let oldEffort = upwardInfo.m().effort;
            if !upwardInfo.m().visited && effort < oldEffort {
                upwardInfo.m().effort = effort;
                upwardInfo.m().prev = minimumInfo;
                if oldEffort == f64::MAX {
                    self.myFrontier.push(upwardInfo);
                    std_push_heap(&mut self.myFrontier, &|a, b| self.myComparator.call_op::<E, V>(a, b));
                } else {
                    let pos = self.myFrontier.iter().position(|x| x.as_raw() == upwardInfo.as_raw()).unwrap();
                    std_push_heap(&mut self.myFrontier[..pos + 1], &|a, b| self.myComparator.call_op::<E, V>(a, b));
                }
            }
        }
        // @note: this effectively does a full dijkstra search.
        // the effort compared to the naive stopping criterion is thus
        // quadrupled. We could implement a better stopping criterion (Holte)
        // However since the search shall take place in a contracted graph
        // it probably does not matter
        !self.myFrontier.is_empty() && self.myFrontier[0].m().effort < *minTTSeen
    }

}


// helpers used by the ported CH code to read edge identity through the router trait
fn e_id<E: RouterEdge<V>, V>(e: Ptr<E>) -> String { e.id().to_string() }
fn e_numerical_id<E: RouterEdge<V>, V>(e: Ptr<E>) -> i32 { e.numerical_id() }
