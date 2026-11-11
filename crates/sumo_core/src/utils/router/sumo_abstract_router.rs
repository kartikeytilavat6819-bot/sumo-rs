//! Port of utils/router/SUMOAbstractRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `double (*)(const E *const, const V *const, double) SUMOAbstractRouter::Operation`
pub type SUMOAbstractRouter_Operation = Unmapped /* double (*)(const E *const, const V *const, double) */;

/// C++ typedef `std::map<const E *, RouterProhibition> SUMOAbstractRouter::Prohibitions`
pub type SUMOAbstractRouter_Prohibitions = Unmapped /* std::map<const E *, RouterProhibition> */;

/// A definition about a route's edge with the effort needed to reach it and the information about the previous edge.
/// C++ class `SUMOAbstractRouter::EdgeInfo` (utils/router/SUMOAbstractRouter.h)
pub struct SUMOAbstractRouter_EdgeInfo {
    /// The current edge
    pub edge: Ptr<Unmapped /* E */>,
    /// Effort to reach the edge
    pub effort: f64,
    /// Estimated effort to reach the edge (effort + lower bound on remaining effort)
    pub heuristicEffort: f64,
    /// The time the vehicle leaves the edge
    pub leaveTime: f64,
    /// The previous edge
    pub prev: Ptr<SUMOAbstractRouter_EdgeInfo>,
    /// whether the edge was already evaluated
    pub visited: bool,
    /// temporary permission change
    pub prohibitedPermissions: SVCPermissions,
    /// the time at which a temporary prohibition begins
    pub prohibitionBegin: f64,
    /// the time at which a temporary prohibition ends
    pub prohibitionEnd: f64,
}

impl SUMOAbstractRouter_EdgeInfo {
    /// C++: `void EdgeInfo::EdgeInfo(const E *const)`
    /// Constructor
    // EdgeInfo(const E* const e)
    //             : edge(e), effort(std::numeric_limits<double>::max()),
    //               heuristicEffort(std::numeric_limits<double>::max()),
    //               leaveTime(0.), prev(nullptr), visited(false), prohibitedPermissions(SVCAll), prohibitionBegin(-1), prohibitionEnd(-1) {}
    #[allow(unused_variables)]
    pub fn new(e: Ptr<Unmapped /* E */>) -> Self {
        todo!("port EdgeInfo::EdgeInfo")
    }

    /// C++: `void EdgeInfo::reset()`
    // inline void reset() {
    //             effort = std::numeric_limits<double>::max();
    //             heuristicEffort = std::numeric_limits<double>::max();
    //             visited = false;
    //         }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        todo!("port EdgeInfo::reset")
    }

}

/// Prohibitions and their estimated end time
/// C++ class `RouterProhibition` (utils/router/SUMOAbstractRouter.h)
pub struct RouterProhibition {
    pub begin: f64,
    pub end: f64,
    pub permissions: SVCPermissions,
}

impl RouterProhibition {
}

/// The interface for routing the vehicles over the network.
/// C++ class `SUMOAbstractRouter` (utils/router/SUMOAbstractRouter.h)
pub struct SUMOAbstractRouter<E, V> {
    /// the handler for routing errors
    pub myErrorMsgHandler: Ptr<MsgHandler>,
    /// The object's operation to perform.
    pub myOperation: SUMOAbstractRouter_Operation,
    /// The object's operation to perform for travel times
    pub myTTOperation: SUMOAbstractRouter_Operation,
    /// whether we are currently operating several route queries in a bulk
    pub myBulkMode: bool,
    /// whether we are currently trying to detect bulk mode automatically
    pub myAutoBulkMode: bool,
    /// whether we are already initialized
    pub myAmClean: bool,
    /// whether edge permissions need to be considered
    pub myHavePermissions: bool,
    /// whether edge restrictions need to be considered
    pub myHaveRestrictions: bool,
    /// The list of explicitly prohibited edges and estimated end time of prohibition
    pub myProhibited: SUMOAbstractRouter_Prohibitions,
    /// The container of edge information
    pub myEdgeInfos: Vec<SUMOAbstractRouter_EdgeInfo>,
    /// A container for reusage of the min edge heap
    pub myFrontierList: Vec<Ptr<SUMOAbstractRouter_EdgeInfo>>,
    /// list of visited Edges (for resetting)
    pub myFound: Vec<Ptr<SUMOAbstractRouter_EdgeInfo>>,
    /// the type of this router
    pub myType: String,
    /// counters for performance logging
    pub myQueryVisits: i64,
    pub myNumQueries: i64,
    /// the time spent querying in milliseconds
    pub myQueryStartTime: i64,
    pub myQueryTimeSum: i64,
    _p: std::marker::PhantomData<(E, V,)>,
}

impl<E, V> SUMOAbstractRouter<E, V> {
    /// C++: `void SUMOAbstractRouter::SUMOAbstractRouter<E, V>(const std::string &, bool, Operation, Operation, const bool, const bool)`
    /// Constructor
    // SUMOAbstractRouter(const std::string& type, bool unbuildIsWarning, Operation operation, Operation ttOperation,
    //                        const bool havePermissions, const bool haveRestrictions) :
    //         myErrorMsgHandler(unbuildIsWarning ? MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance()),
    //         myOperation(operation), myTTOperation(ttOperation),
    //         myBulkMode(false),
    //         myAutoBulkMode(false),
    //         myAmClean(true),
    //         myHavePermissions(havePermissions),
    //         myHaveRestrictions(haveRestrictions),
    //         myType(type),
    //         myQueryVisits(0),
    //         myNumQueries(0),
    //         myQueryStartTime(0),
    //         myQueryTimeSum(0) {
    //     }
    #[allow(unused_variables)]
    pub fn new(type_: &str, unbuildIsWarning: bool, operation: SUMOAbstractRouter_Operation, ttOperation: SUMOAbstractRouter_Operation, havePermissions: bool, haveRestrictions: bool) -> Self {
        todo!("port SUMOAbstractRouter::SUMOAbstractRouter<E, V>")
    }

    /// C++: `void SUMOAbstractRouter::SUMOAbstractRouter<E, V>(SUMOAbstractRouter<E, V> *)`
    /// Copy Constructor
    // SUMOAbstractRouter(SUMOAbstractRouter* other) :
    //         myErrorMsgHandler(other->myErrorMsgHandler),
    //         myOperation(other->myOperation), myTTOperation(other->myTTOperation),
    //         myBulkMode(false),
    //         myAutoBulkMode(false),
    //         myAmClean(true),
    //         myHavePermissions(other->myHavePermissions),
    //         myHaveRestrictions(other->myHaveRestrictions),
    //         myType(other->myType),
    //         myQueryVisits(0),
    //         myNumQueries(0),
    //         myQueryStartTime(0),
    //         myQueryTimeSum(0) { }
    #[allow(unused_variables)]
    pub fn new_2(other: Ptr<SUMOAbstractRouter<E, V>>) -> Self {
        todo!("port SUMOAbstractRouter::SUMOAbstractRouter<E, V>")
    }

    /// C++: `void SUMOAbstractRouter::~SUMOAbstractRouter<E, V>()` [virtual]
    /// Destructor
    // virtual ~SUMOAbstractRouter() {
    //         if (myNumQueries > 0) {
    //             WRITE_MESSAGE(myType + " answered " + toString(myNumQueries) + " queries and explored " + toString((double)myQueryVisits / (double)myNumQueries) +  " edges on average.");
    //             WRITE_MESSAGE(myType + " spent " + elapsedMs2string(myQueryTimeSum) + " answering queries (" + toString((double)myQueryTimeSum / (double)myNumQueries) +  "ms on average).");
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port SUMOAbstractRouter::~SUMOAbstractRouter<E, V>")
    }

    /// C++: `SUMOAbstractRouter<E, V> * SUMOAbstractRouter::clone()` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, V>> {
        todo!("port SUMOAbstractRouter::clone")
    }

    /// C++: `void SUMOAbstractRouter::init(const int, const SUMOTime)`
    // inline void init(const int edgeID, const SUMOTime msTime) {
    // //        if (!myAmClean) {
    //         // all EdgeInfos touched in the previous query are either in myFrontierList or myFound: clean those up
    //         for (auto& edgeInfo : myFrontierList) {
    //             edgeInfo->reset();
    //         }
    //         myFrontierList.clear();
    //         for (auto& edgeInfo : myFound) {
    //             edgeInfo->reset();
    //         }
    //         myFound.clear();
    //         if (edgeID > -1) {
    //             // add begin node
    //             auto& fromInfo = myEdgeInfos[edgeID];
    //             fromInfo.effort = 0.;
    //             fromInfo.heuristicEffort = 0.;
    //             fromInfo.prev = nullptr;
    //             fromInfo.leaveTime = STEPS2TIME(msTime);
    //             myFrontierList.push_back(&fromInfo);
    //         }
    //         myAmClean = true;
    // //        }
    //     }
    #[allow(unused_variables)]
    pub fn init(&mut self, edgeID: i32, msTime: SUMOTime) {
        todo!("port SUMOAbstractRouter::init")
    }

    /// C++: `void SUMOAbstractRouter::reset(const V *const)` [virtual]
    /// reset internal caches, used by CHRouter
    // virtual void reset(const V* const vehicle) {
    //         UNUSED_PARAMETER(vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn reset(&mut self, vehicle: Ptr<V>) {
        todo!("port SUMOAbstractRouter::reset")
    }

    /// C++: `void SUMOAbstractRouter::setMsgHandler(MsgHandler *const)` [virtual]
    // virtual void setMsgHandler(MsgHandler* const errorMsgHandler) {
    //         myErrorMsgHandler = errorMsgHandler;
    //     }
    #[allow(unused_variables)]
    pub fn set_msg_handler(&mut self, errorMsgHandler: Ptr<MsgHandler>) {
        todo!("port SUMOAbstractRouter::setMsgHandler")
    }

    /// C++: `const std::string & SUMOAbstractRouter::getType() const` [const]
    // const std::string& getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> &str {
        todo!("port SUMOAbstractRouter::getType")
    }

    /// C++: `const typename SUMOAbstractRouter<E, V>::EdgeInfo & SUMOAbstractRouter::getEdgeInfo(int) const` [const]
    // inline const typename SUMOAbstractRouter<E, V>::EdgeInfo& getEdgeInfo(int index) const {
    //         return myEdgeInfos[index];
    //     }
    #[allow(unused_variables)]
    pub fn get_edge_info(&self, index: i32) -> &SUMOAbstractRouter_EdgeInfo {
        todo!("port SUMOAbstractRouter::getEdgeInfo")
    }

    /// C++: `bool SUMOAbstractRouter::compute(const E *, const E *, const V *const, SUMOTime, std::vector<const E *> &, bool)` [virtual, pure virtual]
    /// Builds the route between the given edges using the minimum effort at the given time The definition of the effort depends on the wished routing scheme
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn compute(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool {
        todo!("port SUMOAbstractRouter::compute")
    }

    /// C++: `bool SUMOAbstractRouter::compute(const E *, double, const E *, double, const V *const, SUMOTime, std::vector<const E *> &, bool)`
    /// Builds the route between the given edges using the minimum effort at the given time, also taking into account position along the edges to ensure currect handling of looped routes The definition of the effort depends on the wished routing scheme
    // inline bool compute(
    //         const E* from, double fromPos,
    //         const E* to, double toPos,
    //         const V* const vehicle,
    //         SUMOTime msTime, std::vector<const E*>& into, bool silent = false) {
    //         if (from != to || fromPos <= toPos) {
    //             return compute(from, to, vehicle, msTime, into, silent);
    //         } else {
    //             return computeLooped(from, to, vehicle, msTime, into, silent);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn compute_2(&mut self, from: Ptr<E>, fromPos: f64, to: Ptr<E>, toPos: f64, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool {
        todo!("port SUMOAbstractRouter::compute")
    }

    /// C++: `bool SUMOAbstractRouter::computeLooped(const E *, const E *, const V *const, SUMOTime, std::vector<const E *> &, bool)`
    /// Builds the route between the given edges using the minimum effort at the given time if from == to, return the shortest looped route
    // inline bool computeLooped(const E* from, const E* to, const V* const vehicle,
    //                               SUMOTime msTime, std::vector<const E*>& into, bool silent = false) {
    //         if (from != to) {
    //             return compute(from, to, vehicle, msTime, into, silent);
    //         }
    //         double minEffort = std::numeric_limits<double>::max();
    //         std::vector<const E*> best;
    //         const SUMOVehicleClass vClass = vehicle == 0 ? SVC_IGNORING : vehicle->getVClass();
    //         for (const std::pair<const E*, const E*>& follower : from->getViaSuccessors(vClass)) {
    //             std::vector<const E*> tmp;
    //             compute(follower.first, to, vehicle, msTime, tmp, true);
    //             if (tmp.size() > 0) {
    //                 double effort = recomputeCosts(tmp, vehicle, msTime);
    //                 if (effort < minEffort) {
    //                     minEffort = effort;
    //                     best = tmp;
    //                 }
    //             }
    //         }
    //         if (minEffort != std::numeric_limits<double>::max()) {
    //             into.push_back(from);
    //             std::copy(best.begin(), best.end(), std::back_inserter(into));
    //             return true;
    //         } else if (!silent && myErrorMsgHandler != nullptr) {
    //             myErrorMsgHandler->informf(TL("No connection between edge '%' and edge '%' found."), from->getID(), to->getID());
    //         }
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn compute_looped(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool {
        todo!("port SUMOAbstractRouter::computeLooped")
    }

    /// C++: `bool SUMOAbstractRouter::isProhibited(const E *const, const V *const, double) const` [const]
    // inline bool isProhibited(const E* const edge, const V* const vehicle, double t) const {
    //         return (myProhibited.size() > 0
    //                 && (myEdgeInfos[edge->getNumericalID()].prohibitedPermissions & vehicle->getVClass()) != vehicle->getVClass()
    //                 && myEdgeInfos[edge->getNumericalID()].prohibitionBegin <= t
    //                 && myEdgeInfos[edge->getNumericalID()].prohibitionEnd == std::numeric_limits<double>::max())
    //                || (myHavePermissions && edge->prohibits(vehicle)) || (myHaveRestrictions && edge->restricts(vehicle));
    //     }
    #[allow(unused_variables)]
    pub fn is_prohibited(&self, edge: Ptr<E>, vehicle: Ptr<V>, t: f64) -> bool {
        todo!("port SUMOAbstractRouter::isProhibited")
    }

    /// C++: `double SUMOAbstractRouter::getTravelTime(const E *const, const V *const, const double, const double) const` [const]
    // inline double getTravelTime(const E* const e, const V* const v, const double t, const double effort) const {
    //         return myTTOperation == nullptr ? effort : (*myTTOperation)(e, v, t);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self, e: Ptr<E>, v: Ptr<V>, t: f64, effort: f64) -> f64 {
        todo!("port SUMOAbstractRouter::getTravelTime")
    }

    /// C++: `void SUMOAbstractRouter::updateViaEdgeCost(const E *, const V *const, double &, double &, double &) const` [const]
    // inline void updateViaEdgeCost(const E* viaEdge, const V* const v, double& time, double& effort, double& length) const {
    //         while (viaEdge != nullptr && viaEdge->isInternal()) {
    //             const double viaEffortDelta = this->getEffort(viaEdge, v, time);
    //             time += getTravelTime(viaEdge, v, time, viaEffortDelta);
    //             effort += viaEffortDelta;
    //             length += viaEdge->getLength();
    //             viaEdge = viaEdge->getViaSuccessors().front().second;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn update_via_edge_cost(&self, viaEdge: Ptr<E>, v: Ptr<V>, time: &mut f64, effort: &mut f64, length: &mut f64) {
        todo!("port SUMOAbstractRouter::updateViaEdgeCost")
    }

    /// C++: `void SUMOAbstractRouter::updateViaCost(const E *const, const E *const, const V *const, double &, double &, double &) const` [const]
    // inline void updateViaCost(const E* const prev, const E* const e, const V* const v, double& time, double& effort, double& length) const {
    //         if (prev != nullptr) {
    //             for (const std::pair<const E*, const E*>& follower : prev->getViaSuccessors()) {
    //                 if (follower.first == e) {
    //                     updateViaEdgeCost(follower.second, v, time, effort, length);
    //                     break;
    //                 }
    //             }
    //         }
    //         const double effortDelta = this->getEffort(e, v, time);
    //         effort += effortDelta;
    //         time += getTravelTime(e, v, time, effortDelta);
    //         length += e->getLength();
    //     }
    #[allow(unused_variables)]
    pub fn update_via_cost(&self, prev: Ptr<E>, e: Ptr<E>, v: Ptr<V>, time: &mut f64, effort: &mut f64, length: &mut f64) {
        todo!("port SUMOAbstractRouter::updateViaCost")
    }

    /// C++: `bool SUMOAbstractRouter::isValid(const std::vector<const E *> &, const V *const, double) const` [const]
    // bool isValid(const std::vector<const E*>& edges, const V* const v, double t) const {
    //         for (const E* const e : edges) {
    //             if (isProhibited(e, v, t)) {
    //                 return false;
    //             }
    //         }
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn is_valid(&self, edges: &Vec<Ptr<E>>, v: Ptr<V>, t: f64) -> bool {
        todo!("port SUMOAbstractRouter::isValid")
    }

    /// C++: `double SUMOAbstractRouter::recomputeCosts(const std::vector<const E *> &, const V *const, SUMOTime, double *) const` [const, virtual]
    // virtual double recomputeCosts(const std::vector<const E*>& edges, const V* const v, SUMOTime msTime, double* lengthp = nullptr) const {
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
    //             updateViaCost(prev, e, v, time, effort, *lengthp);
    //             prev = e;
    //         }
    //         return effort;
    //     }
    #[allow(unused_variables)]
    pub fn recompute_costs(&self, edges: &Vec<Ptr<E>>, v: Ptr<V>, msTime: SUMOTime, lengthp: Ptr<f64>) -> f64 {
        todo!("port SUMOAbstractRouter::recomputeCosts")
    }

    /// C++: `double SUMOAbstractRouter::recomputeCostsPos(const std::vector<const E *> &, const V *const, double, double, SUMOTime, double *) const` [const]
    // inline double recomputeCostsPos(const std::vector<const E*>& edges, const V* const v, double fromPos, double toPos, SUMOTime msTime, double* lengthp = nullptr) const {
    //         double effort = recomputeCosts(edges, v, msTime, lengthp);
    //         if (!edges.empty()) {
    //             const E* first = edges.front();
    //             if (first->getLength() == 0) {
    //                 if (edges.size() > 1 && edges[1]->getLength() > 0) {
    //                     first = edges[1];
    //                 } else {
    //                     return effort;
    //                 }
    //             }
    //             const E* last = edges.back();
    //             if (last->getLength() == 0) {
    //                 if (edges.size() > 1 && edges[edges.size() - 2]->getLength() > 0) {
    //                     last = edges[edges.size() - 2];
    //                 } else {
    //                     return effort;
    //                 }
    //             }
    //             assert(first->getLength() > 0);
    //             assert(last->getLength() > 0);
    //             double firstEffort = this->getEffort(first, v, STEPS2TIME(msTime));
    //             double lastEffort = this->getEffort(last, v, STEPS2TIME(msTime));
    //             effort -= firstEffort * fromPos / first->getLength();
    //             effort -= lastEffort * (last->getLength() - toPos) / last->getLength();
    //             if (lengthp != nullptr) {
    //                 (*lengthp) -= fromPos + last->getLength() - toPos;
    //             }
    //         }
    //         return effort;
    //     }
    #[allow(unused_variables)]
    pub fn recompute_costs_pos(&self, edges: &Vec<Ptr<E>>, v: Ptr<V>, fromPos: f64, toPos: f64, msTime: SUMOTime, lengthp: Ptr<f64>) -> f64 {
        todo!("port SUMOAbstractRouter::recomputeCostsPos")
    }

    /// C++: `double SUMOAbstractRouter::setHint(const typename std::vector<const E *>::const_iterator, const typename std::vector<const E *>::const_iterator, const V *const, SUMOTime)`
    // inline double setHint(const typename std::vector<const E*>::const_iterator routeBegin, const typename std::vector<const E*>::const_iterator routeEnd, const V* const v, SUMOTime msTime) {
    //         double time = STEPS2TIME(msTime);
    //         double effort = 0.;
    //         double length = 0.;
    //         const EdgeInfo* prev = &myEdgeInfos[(*routeBegin)->getNumericalID()];
    //         init((*routeBegin)->getNumericalID(), msTime);
    //         for (auto e = routeBegin + 1; e != routeEnd; ++e) {
    //             if (isProhibited(*e, v)) {
    //                 return -1;
    //             }
    //             auto& edgeInfo = myEdgeInfos[(*e)->getNumericalID()];
    //             edgeInfo.heuristicEffort = effort;
    //             edgeInfo.prev = prev;
    //             updateViaCost(prev->edge, *e, v, time, effort, length);
    //             edgeInfo.effort = effort;
    //             edgeInfo.leaveTime = time;
    //             myFound.push_back(&edgeInfo);
    //             prev = &edgeInfo;
    // #ifdef ROUTER_DEBUG_HINT
    //             if (ROUTER_DEBUG_COND) {
    //                 std::cout << "DEBUG: hit=" << (*e)->getID()
    //                           << " TT=" << edgeInfo.effort
    //                           << " EF=" << this->getEffort(*e, v, edgeInfo.leaveTime)
    //                           << " HT=" << edgeInfo.heuristicEffort << "\n";
    //             }
    // #endif
    //         }
    //         return effort;
    //     }
    #[allow(unused_variables)]
    pub fn set_hint(&mut self, routeBegin: Unmapped /* std::vector<const E *>::const_iterator */, routeEnd: Unmapped /* std::vector<const E *>::const_iterator */, v: Ptr<V>, msTime: SUMOTime) -> f64 {
        todo!("port SUMOAbstractRouter::setHint")
    }

    /// C++: `double SUMOAbstractRouter::getEffort(const E *const, const V *const, double) const` [const]
    // inline double getEffort(const E* const e, const V* const v, double t) const {
    //         if (this->myProhibited.size() > 0 && myEdgeInfos.size() > 0
    //                 && myEdgeInfos[e->getNumericalID()].prohibitionEnd > t
    //                 && myEdgeInfos[e->getNumericalID()].prohibitionBegin <= t
    //                 && (myEdgeInfos[e->getNumericalID()].prohibitedPermissions & v->getVClass()) != v->getVClass()) {
    //             // pass edge after prohibition ends
    //             return (myEdgeInfos[e->getNumericalID()].prohibitionEnd - t) + (*myOperation)(e, v, myEdgeInfos[e->getNumericalID()].prohibitionEnd);
    //         } else {
    //             return (*myOperation)(e, v, t);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_effort(&self, e: Ptr<E>, v: Ptr<V>, t: f64) -> f64 {
        todo!("port SUMOAbstractRouter::getEffort")
    }

    /// C++: `void SUMOAbstractRouter::startQuery()`
    // inline void startQuery() {
    //         myNumQueries++;
    //         myQueryStartTime = SysUtils::getCurrentMillis();
    //     }
    #[allow(unused_variables)]
    pub fn start_query(&mut self) {
        todo!("port SUMOAbstractRouter::startQuery")
    }

    /// C++: `void SUMOAbstractRouter::endQuery(int)`
    // inline void endQuery(int visits) {
    //         myQueryVisits += visits;
    //         myQueryTimeSum += (SysUtils::getCurrentMillis() - myQueryStartTime);
    //     }
    #[allow(unused_variables)]
    pub fn end_query(&mut self, visits: i32) {
        todo!("port SUMOAbstractRouter::endQuery")
    }

    /// C++: `void SUMOAbstractRouter::setBulkMode(const bool)` [virtual]
    // virtual void setBulkMode(const bool mode) {
    //         myBulkMode = mode;
    //     }
    #[allow(unused_variables)]
    pub fn set_bulk_mode(&mut self, mode: bool) {
        todo!("port SUMOAbstractRouter::setBulkMode")
    }

    /// C++: `void SUMOAbstractRouter::setAutoBulkMode(const bool)`
    // inline void setAutoBulkMode(const bool mode) {
    //         myAutoBulkMode = mode;
    //     }
    #[allow(unused_variables)]
    pub fn set_auto_bulk_mode(&mut self, mode: bool) {
        todo!("port SUMOAbstractRouter::setAutoBulkMode")
    }

    /// C++: `void SUMOAbstractRouter::prohibit(const Prohibitions &)` [virtual]
    // virtual void prohibit(const Prohibitions& toProhibit) {
    //         for (auto item : this->myProhibited) {
    //             myEdgeInfos[item.first->getNumericalID()].prohibitedPermissions = SVCAll;
    //             myEdgeInfos[item.first->getNumericalID()].prohibitionBegin = -1;
    //             myEdgeInfos[item.first->getNumericalID()].prohibitionEnd = -1;
    //         }
    //         for (auto item : toProhibit) {
    //             myEdgeInfos[item.first->getNumericalID()].prohibitionBegin = item.second.begin;
    //             myEdgeInfos[item.first->getNumericalID()].prohibitionEnd = item.second.end;
    //             myEdgeInfos[item.first->getNumericalID()].prohibitedPermissions = item.second.permissions;
    //             //std::cout << item.first->getID() << " " << item.second.begin << " " << item.second.end << " (" << getVehicleClassNames(item.second.permissions) << ")\n";
    //         }
    //         this->myProhibited = toProhibit;
    //     }
    #[allow(unused_variables)]
    pub fn prohibit(&mut self, toProhibit: &SUMOAbstractRouter_Prohibitions) {
        todo!("port SUMOAbstractRouter::prohibit")
    }

    /// C++: `bool SUMOAbstractRouter::hasProhibitions() const` [const]
    // bool hasProhibitions() const {
    //         return this->myProhibited.size() > 0;
    //     }
    #[allow(unused_variables)]
    pub fn has_prohibitions(&self) -> bool {
        todo!("port SUMOAbstractRouter::hasProhibitions")
    }

    /// C++: `bool SUMOAbstractRouter::supportsProhibitions() const` [const, virtual]
    // virtual bool supportsProhibitions() const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn supports_prohibitions(&self) -> bool {
        todo!("port SUMOAbstractRouter::supportsProhibitions")
    }

    /// C++: `void SUMOAbstractRouter::buildPathFrom(const typename SUMOAbstractRouter<E, V>::EdgeInfo *, std::vector<const E *> &)`
    /// Builds the path from marked edges
    // void buildPathFrom(const typename SUMOAbstractRouter<E, V>::EdgeInfo* rbegin, std::vector<const E*>& edges) {
    //         std::vector<const E*> tmp;
    //         while (rbegin != nullptr) {
    //             tmp.push_back(rbegin->edge);
    //             rbegin = rbegin->prev;
    //         }
    //         std::copy(tmp.rbegin(), tmp.rend(), std::back_inserter(edges));
    //     }
    #[allow(unused_variables)]
    pub fn build_path_from(&mut self, rbegin: Ptr<SUMOAbstractRouter_EdgeInfo>, edges: &mut Vec<Ptr<E>>) {
        todo!("port SUMOAbstractRouter::buildPathFrom")
    }

    /// C++: `SUMOAbstractRouter<E, V> & SUMOAbstractRouter::operator=(const SUMOAbstractRouter<E, V> &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &SUMOAbstractRouter<E, V>) -> &mut SUMOAbstractRouter<E, V> {
        todo!("port SUMOAbstractRouter::operator=")
    }

}

