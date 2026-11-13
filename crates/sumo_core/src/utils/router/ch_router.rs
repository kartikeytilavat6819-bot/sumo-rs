//! Port of utils/router/CHRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::RouterEdge;

/// C++ typedef `int Unidirectional::ConnectionVector`
pub type Unidirectional_ConnectionVector = i32;

/// C++ typedef `std::pair<const typename SUMOAbstractRouter<E, V>::EdgeInfo *, const typename SUMOAbstractRouter<E, V>::EdgeInfo *> CHRouter::Meeting`
pub type CHRouter_Meeting = (Ptr<SUMOAbstractRouter_EdgeInfo>, Ptr<SUMOAbstractRouter_EdgeInfo>);

/// Computes the shortest path through a contracted network
/// C++ class `CHRouter` (utils/router/CHRouter.h)
#[repr(C)]
pub struct CHRouter<E, V> {
    /// C++ base class `SUMOAbstractRouter<E, V>`
    pub base: SUMOAbstractRouter<E, V>,
    /// all edges with numerical ids
    pub myEdges: Ptr<i32>,
    /// the unidirectional search queues
    pub myForwardSearch: CHRouter_Unidirectional,
    pub myBackwardSearch: CHRouter_Unidirectional,
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

impl<E: RouterEdge<V>, V> CHRouter<E, V> {
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
    pub fn new(edges: &i32, unbuildIsWarning: bool, operation: SUMOAbstractRouter_Operation, svc: SUMOVehicleClass, weightPeriod: SUMOTime, havePermissions: bool, haveRestrictions: bool) -> Self {
        todo!("port CHRouter::CHRouter<E, V>")
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
    pub fn new_2(edges: &i32, unbuildIsWarning: bool, operation: SUMOAbstractRouter_Operation, svc: SUMOVehicleClass, hierarchy: Ptr<CHBuilder_Hierarchy<E>>, havePermissions: bool, haveRestrictions: bool) -> Self {
        todo!("port CHRouter::CHRouter<E, V>")
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
        todo!("port CHRouter::~CHRouter<E, V>")
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
        todo!("port CHRouter::clone")
    }

    /// C++: `void CHRouter::prohibit(const int &)` [virtual]
    // virtual void prohibit(const std::map<const E*, RouterProhibition>& toProhibit) {
    //         if (toProhibit.size() > 0) {
    //             WRITE_WARNINGF(TL("Routing algorithm CH does not support dynamic closing of edges%"), "");
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn prohibit(&mut self, toProhibit: &i32) {
        todo!("port CHRouter::prohibit")
    }

    /// C++: `bool CHRouter::supportsProhibitions() const` [const, virtual]
    // virtual bool supportsProhibitions() const {
    //         WRITE_WARNINGF(TL("Routing algorithm CH does not support dynamic closing of edges and lanes%"), "");
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn supports_prohibitions(&self) -> bool {
        todo!("port CHRouter::supportsProhibitions")
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
        todo!("port CHRouter::reset")
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
    pub fn compute(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut i32, silent: bool) -> bool {
        todo!("port CHRouter::compute")
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
    pub fn build_path_from_meeting(&self, meeting: CHRouter_Meeting, into: &mut i32) {
        todo!("port CHRouter::buildPathFromMeeting")
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
        todo!("port CHRouter::getVia")
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
    pub fn call_op(&self, nod1: Ptr<SUMOAbstractRouter_EdgeInfo>, nod2: Ptr<SUMOAbstractRouter_EdgeInfo>) -> bool {
        todo!("port EdgeInfoByTTComparator::operator()")
    }

}

/// class for searching in one direction
/// C++ class `CHRouter::Unidirectional` (utils/router/CHRouter.h)
pub struct CHRouter_Unidirectional {
    /// the role of this search
    pub myAmForward: bool,
    /// the min edge heap
    pub myFrontier: i32,
    /// the set of visited (settled) Edges
    pub myFound: i32,
    /// The container of edge information
    pub myEdgeInfos: i32,
    pub myComparator: Unidirectional_EdgeInfoByTTComparator,
    pub myVehicle: Ptr<Unmapped /* V */>,
}

impl CHRouter_Unidirectional {
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
    pub fn new(edges: &i32, forward: bool) -> Self {
        todo!("port Unidirectional::Unidirectional")
    }

    /// C++: `bool Unidirectional::found(const E *const) const` [const]
    // inline bool found(const E* const edge) const {
    //             return myFound.count(edge) > 0;
    //         }
    #[allow(unused_variables)]
    pub fn found(&self, edge: Ptr<Unmapped /* E */>) -> bool {
        todo!("port Unidirectional::found")
    }

    /// C++: `typename SUMOAbstractRouter<E, V>::EdgeInfo * Unidirectional::getEdgeInfo(const E *const)`
    // inline typename SUMOAbstractRouter<E, V>::EdgeInfo* getEdgeInfo(const E* const edge) {
    //             return &(myEdgeInfos[edge->getNumericalID()]);
    //         }
    #[allow(unused_variables)]
    pub fn get_edge_info(&mut self, edge: Ptr<Unmapped /* E */>) -> Ptr<SUMOAbstractRouter_EdgeInfo> {
        todo!("port Unidirectional::getEdgeInfo")
    }

    /// C++: `const typename SUMOAbstractRouter<E, V>::EdgeInfo * Unidirectional::getEdgeInfo(const E *const) const` [const]
    // inline const typename SUMOAbstractRouter<E, V>::EdgeInfo* getEdgeInfo(const E* const edge) const {
    //             return &(myEdgeInfos[edge->getNumericalID()]);
    //         }
    #[allow(unused_variables)]
    pub fn get_edge_info_2(&self, edge: Ptr<Unmapped /* E */>) -> Ptr<SUMOAbstractRouter_EdgeInfo> {
        todo!("port Unidirectional::getEdgeInfo")
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
    pub fn init(&mut self, start: Ptr<Unmapped /* E */>, vehicle: Ptr<Unmapped /* V */>) {
        todo!("port Unidirectional::init")
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
    pub fn step(&mut self, uplinks: &i32, otherSearch: &CHRouter_Unidirectional, minTTSeen: &mut f64, meeting: &mut CHRouter_Meeting) -> bool {
        todo!("port Unidirectional::step")
    }

}

