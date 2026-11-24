//! Port of utils/router/SUMOAbstractRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::sumo_vehicle_class::{SVC_ALL, SVC_IGNORING};
use utils::common::time::steps2time;
use utils::common::msg_handler::{with_instance, MsgType};
use crate::utils::router::router_edge::{RouterEdge, RouterEdgeInfo, RouterVehicle};

/// C++ typedef `double (*)(const E *const, const V *const, double) SUMOAbstractRouter::Operation`
/// Typed for the generic router. The bare `SUMOAbstractRouter_Operation` stays a placeholder for the
/// microsim/logging bindings, whose C++ operation is a pointer-to-member, not a router function.
pub type SUMOAbstractRouter_TypedOperation<E, V> = fn(Ptr<E>, Ptr<V>, f64) -> f64;

/// The concrete router's `compute` (C++ virtual), installed by the subclass that implements it.
pub type SUMOAbstractRouter_ComputeFn<E, V> = fn(&mut SUMOAbstractRouter<E, V>, Ptr<E>, Ptr<E>, Ptr<V>, SUMOTime, &mut Vec<Ptr<E>>, bool) -> bool;
pub type SUMOAbstractRouter_ProhibitFn<E, V> = fn(&mut SUMOAbstractRouter<E, V>, &SUMOAbstractRouter_Prohibitions);
pub type SUMOAbstractRouter_SupportsProhibitionsFn<E, V> = fn(&SUMOAbstractRouter<E, V>) -> bool;
pub type SUMOAbstractRouter_ResetFn<E, V> = fn(&mut SUMOAbstractRouter<E, V>, Ptr<V>);
pub type SUMOAbstractRouter_SetBulkModeFn<E, V> = fn(&mut SUMOAbstractRouter<E, V>, bool);
pub type SUMOAbstractRouter_SetMsgHandlerFn<E, V> = fn(&mut SUMOAbstractRouter<E, V>, Ptr<MsgHandler>);
pub type SUMOAbstractRouter_CloneFn<E, V> = fn(&mut SUMOAbstractRouter<E, V>) -> Ptr<SUMOAbstractRouter<E, V>>;
/// The concrete router's `recomputeCosts` (C++ virtual), installed by subclasses that override it.
pub type SUMOAbstractRouter_RecomputeCostsFn<E, V> = fn(&SUMOAbstractRouter<E, V>, &Vec<Ptr<E>>, Ptr<V>, SUMOTime, Ptr<f64>) -> f64;

/// C++ typedef `double (*)(const E *const, const V *const, double) SUMOAbstractRouter::Operation`
pub type SUMOAbstractRouter_Operation = Unmapped /* double (*)(const E *const, const V *const, double) */;

/// C++ typedef `std::map<const E *, RouterProhibition> SUMOAbstractRouter::Prohibitions`
pub type SUMOAbstractRouter_Prohibitions = BTreeMap<Ptr<Unmapped /* E */>, RouterProhibition>;

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
        Self {
            edge: e,
            effort: f64::MAX,
            heuristicEffort: f64::MAX,
            leaveTime: 0.,
            prev: Ptr::null(),
            visited: false,
            prohibitedPermissions: SVC_ALL,
            prohibitionBegin: -1.,
            prohibitionEnd: -1.,
        }
    }

    /// C++: `void EdgeInfo::reset()`
    // inline void reset() {
    //             effort = std::numeric_limits<double>::max();
    //             heuristicEffort = std::numeric_limits<double>::max();
    //             visited = false;
    //         }
    #[allow(unused_variables)]
    pub fn reset(&mut self) {
        self.effort = f64::MAX;
        self.heuristicEffort = f64::MAX;
        self.visited = false;
    }

}

/// Prohibitions and their estimated end time
/// C++ class `RouterProhibition` (utils/router/SUMOAbstractRouter.h)
#[derive(Clone, Copy, Debug)]
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
    /// The object's operation to perform (null in C++ for the intermodal and pedestrian bases; calling it then is a crash there too)
    pub myOperation: Option<SUMOAbstractRouter_TypedOperation<E, V>>,
    /// The object's operation to perform for travel times (null in C++ means the effort is used as is)
    pub myTTOperation: Option<SUMOAbstractRouter_TypedOperation<E, V>>,
    /// the concrete `compute` the pure-virtual base dispatches to (None: the base has no implementation)
    pub myCompute: Option<SUMOAbstractRouter_ComputeFn<E, V>>,
    /// the subclass `recomputeCosts` override (C++ virtual), None for the base behaviour
    pub myRecomputeCosts: Option<SUMOAbstractRouter_RecomputeCostsFn<E, V>>,
    /// the subclass prohibit override (C++ virtual `prohibit`), None for the base behaviour
    pub myProhibit: Option<SUMOAbstractRouter_ProhibitFn<E, V>>,
    /// the subclass supportsProhibitions override (C++ virtual), None for the base behaviour (true)
    pub mySupportsProhibitions: Option<SUMOAbstractRouter_SupportsProhibitionsFn<E, V>>,
    /// the subclass reset override (C++ virtual `reset`), None for the base no-op
    pub myReset: Option<SUMOAbstractRouter_ResetFn<E, V>>,
    /// the subclass setBulkMode override (C++ virtual `setBulkMode`), None for the base behaviour
    pub mySetBulkMode: Option<SUMOAbstractRouter_SetBulkModeFn<E, V>>,
    /// the subclass setMsgHandler override (C++ virtual `setMsgHandler`), None for the base behaviour
    pub mySetMsgHandler: Option<SUMOAbstractRouter_SetMsgHandlerFn<E, V>>,
    /// the subclass clone (C++ pure virtual `clone`)
    pub myClone: Option<SUMOAbstractRouter_CloneFn<E, V>>,
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
    pub fn new(type_: &str, unbuildIsWarning: bool, operation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, ttOperation: Option<SUMOAbstractRouter_TypedOperation<E, V>>, havePermissions: bool, haveRestrictions: bool) -> Self {
        Self {
            myErrorMsgHandler: msg_instance(unbuildIsWarning),
            myOperation: operation,
            myTTOperation: ttOperation,
            myCompute: None,
            myRecomputeCosts: None,
            myProhibit: None,
            mySupportsProhibitions: None,
            myReset: None,
            mySetBulkMode: None,
            mySetMsgHandler: None,
            myClone: None,
            myBulkMode: false,
            myAutoBulkMode: false,
            myAmClean: true,
            myHavePermissions: havePermissions,
            myHaveRestrictions: haveRestrictions,
            myProhibited: Default::default(),
            myEdgeInfos: Vec::new(),
            myFrontierList: Vec::new(),
            myFound: Vec::new(),
            myType: type_.to_string(),
            myQueryVisits: 0,
            myNumQueries: 0,
            myQueryStartTime: 0,
            myQueryTimeSum: 0,
            _p: std::marker::PhantomData,
        }
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
        Self {
            myErrorMsgHandler: other.myErrorMsgHandler,
            myOperation: other.myOperation,
            myTTOperation: other.myTTOperation,
            myCompute: other.myCompute,
            myRecomputeCosts: other.myRecomputeCosts,
            myProhibit: other.myProhibit,
            mySupportsProhibitions: other.mySupportsProhibitions,
            myReset: other.myReset,
            mySetBulkMode: other.mySetBulkMode,
            mySetMsgHandler: other.mySetMsgHandler,
            myClone: other.myClone,
            myBulkMode: false,
            myAutoBulkMode: false,
            myAmClean: true,
            myHavePermissions: other.myHavePermissions,
            myHaveRestrictions: other.myHaveRestrictions,
            myProhibited: Default::default(),
            myEdgeInfos: Vec::new(),
            myFrontierList: Vec::new(),
            myFound: Vec::new(),
            myType: other.myType.clone(),
            myQueryVisits: 0,
            myNumQueries: 0,
            myQueryStartTime: 0,
            myQueryTimeSum: 0,
            _p: std::marker::PhantomData,
        }
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
        if self.myNumQueries > 0 {
            let n = self.myNumQueries as f64;
            utils::common::msg_handler::write_message(&format!("{} answered {} queries and explored {} edges on average.", self.myType, self.myNumQueries, utils::common::to_string::to_string(&(self.myQueryVisits as f64 / n))));
            utils::common::msg_handler::write_message(&format!("{} spent {} answering queries ({}ms on average).", self.myType, utils::common::time::elapsed_ms2string(self.myQueryTimeSum), utils::common::to_string::to_string(&(self.myQueryTimeSum as f64 / n))));
        }
    }

    /// C++: `SUMOAbstractRouter<E, V> * SUMOAbstractRouter::clone()` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, V>> {
        match self.myClone {
            Some(clone) => clone(self),
            None => panic!("pure virtual method called"),
        }
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
        for edgeInfo in self.myFrontierList.iter() {
            edgeInfo.m().reset();
        }
        self.myFrontierList.clear();
        for edgeInfo in self.myFound.iter() {
            edgeInfo.m().reset();
        }
        self.myFound.clear();
        if edgeID > -1 {
            // add begin node
            let fromInfo = Ptr::from_mut(&mut self.myEdgeInfos[edgeID as usize]);
            let f = fromInfo.m();
            f.effort = 0.;
            f.heuristicEffort = 0.;
            f.prev = Ptr::null();
            f.leaveTime = steps2time(msTime);
            self.myFrontierList.push(fromInfo);
        }
        self.myAmClean = true;
    }

    /// C++: `void SUMOAbstractRouter::reset(const V *const)` [virtual]
    /// reset internal caches, used by CHRouter
    // virtual void reset(const V* const vehicle) {
    //         UNUSED_PARAMETER(vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn reset(&mut self, vehicle: Ptr<V>) {
        match self.myReset {
            Some(reset) => reset(self, vehicle),
            None => self.reset_base(vehicle),
        }
    }

    /// C++ `SUMOAbstractRouter::reset` body, for subclass overrides that call the base explicitly
    #[allow(unused_variables)]
    pub fn reset_base(&mut self, vehicle: Ptr<V>) {
        // C++ body is empty: UNUSED_PARAMETER(vehicle)
        let _ = vehicle;
    }

    /// C++: `void SUMOAbstractRouter::setMsgHandler(MsgHandler *const)` [virtual]
    // virtual void setMsgHandler(MsgHandler* const errorMsgHandler) {
    //         myErrorMsgHandler = errorMsgHandler;
    //     }
    #[allow(unused_variables)]
    pub fn set_msg_handler(&mut self, errorMsgHandler: Ptr<MsgHandler>) {
        match self.mySetMsgHandler {
            Some(set_msg_handler) => set_msg_handler(self, errorMsgHandler),
            None => self.set_msg_handler_base(errorMsgHandler),
        }
    }

    /// the base-class body of setMsgHandler (what `SUMOAbstractRouter<E, V>::setMsgHandler` names in C++)
    pub fn set_msg_handler_base(&mut self, errorMsgHandler: Ptr<MsgHandler>) {
        self.myErrorMsgHandler = errorMsgHandler;
    }

    /// C++: `const std::string & SUMOAbstractRouter::getType() const` [const]
    // const std::string& getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> &str {
        self.myType.as_str()
    }

    /// C++: `const typename SUMOAbstractRouter<E, V>::EdgeInfo & SUMOAbstractRouter::getEdgeInfo(int) const` [const]
    // inline const typename SUMOAbstractRouter<E, V>::EdgeInfo& getEdgeInfo(int index) const {
    //         return myEdgeInfos[index];
    //     }
    #[allow(unused_variables)]
    pub fn get_edge_info(&self, index: i32) -> &SUMOAbstractRouter_EdgeInfo {
        &self.myEdgeInfos[index as usize]
    }

    /// C++: `bool SUMOAbstractRouter::compute(const E *, const E *, const V *const, SUMOTime, std::vector<const E *> &, bool)` [virtual, pure virtual]
    /// Builds the route between the given edges using the minimum effort at the given time The definition of the effort depends on the wished routing scheme
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn compute(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool {
        // the subclass installs its override in myCompute (C++ virtual dispatch); a base object without one is a pure virtual call, which aborts in C++
        match self.myCompute {
            Some(compute) => compute(self, from, to, vehicle, msTime, into, silent),
            None => panic!("pure virtual method called"),
        }
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
    pub fn compute_2(&mut self, from: Ptr<E>, fromPos: f64, to: Ptr<E>, toPos: f64, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool where E: RouterEdge<V>, V: RouterVehicle {
        if from != to || fromPos <= toPos {
            self.compute(from, to, vehicle, msTime, into, silent)
        } else {
            self.compute_looped(from, to, vehicle, msTime, into, silent)
        }
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
    pub fn compute_looped(&mut self, from: Ptr<E>, to: Ptr<E>, vehicle: Ptr<V>, msTime: SUMOTime, into: &mut Vec<Ptr<E>>, silent: bool) -> bool
    where
        E: RouterEdge<V>,
        V: RouterVehicle,
    {
        if from != to {
            return self.compute(from, to, vehicle, msTime, into, silent);
        }
        let mut minEffort = f64::MAX;
        let mut best: Vec<Ptr<E>> = Vec::new();
        let vClass: SUMOVehicleClass = if vehicle.is_null() { SVC_IGNORING } else { vehicle.v_class() };
        for follower in from.via_successors(vClass, false).to_vec() {
            let mut tmp: Vec<Ptr<E>> = Vec::new();
            self.compute(follower.0, to, vehicle, msTime, &mut tmp, true);
            if tmp.len() > 0 {
                let effort = self.recompute_costs(&tmp, vehicle, msTime, Ptr::null());
                if effort < minEffort {
                    minEffort = effort;
                    best = tmp;
                }
            }
        }
        if minEffort != f64::MAX {
            into.push(from);
            into.extend(best);
            return true;
        } else if !silent && !self.myErrorMsgHandler.is_null() {
            let (fromId, toId) = (e_id(from), e_id(to));
            let args: [&dyn utils::common::to_string::ToStringPrec; 2] = [&fromId, &toId];
            self.myErrorMsgHandler.m().informf("No connection between edge '%' and edge '%' found.", &args);
        }
        false
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
    pub fn is_prohibited(&self, edge: Ptr<E>, vehicle: Ptr<V>, t: f64) -> bool
    where
        E: RouterEdge<V>,
        V: RouterVehicle,
    {
        if !self.myProhibited.is_empty() {
            let info = &self.myEdgeInfos[e_numerical_id(edge)];
            if (info.prohibitedPermissions & vehicle.v_class()) != vehicle.v_class() && info.prohibitionBegin <= t && info.prohibitionEnd == f64::MAX {
                return true;
            }
        }
        (self.myHavePermissions && edge.prohibits(vehicle, false)) || (self.myHaveRestrictions && edge.restricts(vehicle))
    }

    /// C++: `double SUMOAbstractRouter::getTravelTime(const E *const, const V *const, const double, const double) const` [const]
    // inline double getTravelTime(const E* const e, const V* const v, const double t, const double effort) const {
    //         return myTTOperation == nullptr ? effort : (*myTTOperation)(e, v, t);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self, e: Ptr<E>, v: Ptr<V>, t: f64, effort: f64) -> f64 {
        match self.myTTOperation {
            None => effort,
            Some(op) => op(e, v, t),
        }
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
    pub fn update_via_edge_cost(&self, viaEdge: Ptr<E>, v: Ptr<V>, time: &mut f64, effort: &mut f64, length: &mut f64) where E: RouterEdge<V>, V: RouterVehicle {
        let mut viaEdge = viaEdge;
        while !viaEdge.is_null() && viaEdge.is_internal() {
            let viaEffortDelta = self.get_effort(viaEdge, v, *time);
            *time += self.get_travel_time(viaEdge, v, *time, viaEffortDelta);
            *effort += viaEffortDelta;
            *length += viaEdge.length();
            let vs = viaEdge.via_successors(SVC_IGNORING, false);
            // shortcut: C++ front() on an empty list is UB and the official build never reaches it (tt_bugs ticket13108);
            // our internal edge lacks a via successor that C++ has. Stop instead of panicking; upgrade when that state is found.
            if vs.is_empty() {
                break;
            }
            viaEdge = vs[0].1;
        }
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
    pub fn update_via_cost(&self, prev: Ptr<E>, e: Ptr<E>, v: Ptr<V>, time: &mut f64, effort: &mut f64, length: &mut f64) where E: RouterEdge<V>, V: RouterVehicle {
        if !prev.is_null() {
            for follower in prev.via_successors(SVC_IGNORING, false).to_vec() {
                if follower.0 == e {
                    self.update_via_edge_cost(follower.1, v, time, effort, length);
                    break;
                }
            }
        }
        let effortDelta = self.get_effort(e, v, *time);
        *effort += effortDelta;
        *time += self.get_travel_time(e, v, *time, effortDelta);
        *length += e.length();
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
    pub fn is_valid(&self, edges: &Vec<Ptr<E>>, v: Ptr<V>, t: f64) -> bool where E: RouterEdge<V>, V: RouterVehicle {
        for e in edges.iter() {
            if self.is_prohibited(*e, v, t) {
                return false;
            }
        }
        true
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
    pub fn recompute_costs(&self, edges: &Vec<Ptr<E>>, v: Ptr<V>, msTime: SUMOTime, lengthp: Ptr<f64>) -> f64 where E: RouterEdge<V>, V: RouterVehicle {
        // C++ `recomputeCosts` is virtual: a RailwayRouter overrides it with reversal corrections.
        if let Some(f) = self.myRecomputeCosts {
            return f(self, edges, v, msTime, lengthp);
        }
        self.recompute_costs_base(edges, v, msTime, lengthp)
    }

    /// The non-virtual base implementation (C++ `SUMOAbstractRouter::recomputeCosts` body).
    /// Subclass overrides call this directly rather than re-entering the virtual slot.
    pub fn recompute_costs_base(&self, edges: &Vec<Ptr<E>>, v: Ptr<V>, msTime: SUMOTime, lengthp: Ptr<f64>) -> f64 where E: RouterEdge<V>, V: RouterVehicle {
        let mut time = steps2time(msTime);
        let mut effort = 0.;
        let mut length = 0.;
        let lengthp: Ptr<f64> = if lengthp.is_null() {
            Ptr::from_mut(&mut length)
        } else {
            *lengthp.m() = 0.;
            lengthp
        };
        let mut prev: Ptr<E> = Ptr::null();
        for e in edges.iter() {
            self.update_via_cost(prev, *e, v, &mut time, &mut effort, lengthp.m());
            prev = *e;
        }
        effort
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
    pub fn recompute_costs_pos(&self, edges: &Vec<Ptr<E>>, v: Ptr<V>, fromPos: f64, toPos: f64, msTime: SUMOTime, lengthp: Ptr<f64>) -> f64 where E: RouterEdge<V>, V: RouterVehicle {
        let mut effort = self.recompute_costs(edges, v, msTime, lengthp);
        if !edges.is_empty() {
            let mut first = edges[0];
            if first.length() == 0. {
                if edges.len() > 1 && edges[1].length() > 0. {
                    first = edges[1];
                } else {
                    return effort;
                }
            }
            let mut last = edges[edges.len() - 1];
            if last.length() == 0. {
                if edges.len() > 1 && edges[edges.len() - 2].length() > 0. {
                    last = edges[edges.len() - 2];
                } else {
                    return effort;
                }
            }
            assert!(first.length() > 0.);
            assert!(last.length() > 0.);
            let firstEffort = self.get_effort(first, v, steps2time(msTime));
            let lastEffort = self.get_effort(last, v, steps2time(msTime));
            effort -= firstEffort * fromPos / first.length();
            effort -= lastEffort * (last.length() - toPos) / last.length();
            if !lengthp.is_null() {
                *lengthp.m() -= fromPos + last.length() - toPos;
            }
        }
        effort
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
    pub fn set_hint(&mut self, route: &Vec<Ptr<E>>, routeBegin: usize, routeEnd: usize, v: Ptr<V>, msTime: SUMOTime) -> f64 where E: RouterEdge<V>, V: RouterVehicle {
        let mut time = steps2time(msTime);
        let mut effort = 0.;
        let mut length = 0.;
        let mut prev: Ptr<SUMOAbstractRouter_EdgeInfo> = Ptr::from_mut(&mut self.myEdgeInfos[e_numerical_id(route[routeBegin])]);
        self.init(e_numerical_id(route[routeBegin]) as i32, msTime);
        for e_idx in routeBegin + 1..routeEnd {
            let e = route[e_idx];
            // C++ calls isProhibited(*e, v) with two arguments, which does not compile if instantiated; the query time is passed here
            if self.is_prohibited(e, v, time) {
                return -1.;
            }
            let edge_info = Ptr::from_mut(&mut self.myEdgeInfos[e_numerical_id(e)]);
            edge_info.m().heuristicEffort = effort;
            edge_info.m().prev = prev;
            self.update_via_cost(prev.m().edge.cast::<E>(), e, v, &mut time, &mut effort, &mut length);
            edge_info.m().effort = effort;
            edge_info.m().leaveTime = time;
            self.myFound.push(edge_info);
            prev = edge_info;
        }
        effort
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
    pub fn get_effort(&self, e: Ptr<E>, v: Ptr<V>, t: f64) -> f64 where E: RouterEdge<V>, V: RouterVehicle {
        if !self.myProhibited.is_empty() && !self.myEdgeInfos.is_empty() {
            let info = &self.myEdgeInfos[e_numerical_id(e)];
            if info.prohibitionEnd > t && info.prohibitionBegin <= t && (info.prohibitedPermissions & v.v_class()) != v.v_class() {
                // pass edge after prohibition ends
                let end = info.prohibitionEnd;
                return (end - t) + call_op(self.myOperation, e, v, end);
            }
        }
        call_op(self.myOperation, e, v, t)
    }

    /// C++: `void SUMOAbstractRouter::startQuery()`
    // inline void startQuery() {
    //         myNumQueries++;
    //         myQueryStartTime = SysUtils::getCurrentMillis();
    //     }
    #[allow(unused_variables)]
    pub fn start_query(&mut self) {
        self.myNumQueries += 1;
        self.myQueryStartTime = current_millis();
    }

    /// C++: `void SUMOAbstractRouter::endQuery(int)`
    // inline void endQuery(int visits) {
    //         myQueryVisits += visits;
    //         myQueryTimeSum += (SysUtils::getCurrentMillis() - myQueryStartTime);
    //     }
    #[allow(unused_variables)]
    pub fn end_query(&mut self, visits: i32) {
        self.myQueryVisits += visits as i64;
        self.myQueryTimeSum += current_millis() - self.myQueryStartTime;
    }

    /// C++: `void SUMOAbstractRouter::setBulkMode(const bool)` [virtual]
    // virtual void setBulkMode(const bool mode) {
    //         myBulkMode = mode;
    //     }
    #[allow(unused_variables)]
    pub fn set_bulk_mode(&mut self, mode: bool) {
        match self.mySetBulkMode {
            Some(set_bulk_mode) => set_bulk_mode(self, mode),
            None => self.set_bulk_mode_base(mode),
        }
    }

    /// C++ `SUMOAbstractRouter::setBulkMode` body, for subclass overrides that call the base explicitly
    #[allow(unused_variables)]
    pub fn set_bulk_mode_base(&mut self, mode: bool) {
        self.myBulkMode = mode;
    }

    /// C++: `void SUMOAbstractRouter::setAutoBulkMode(const bool)`
    // inline void setAutoBulkMode(const bool mode) {
    //         myAutoBulkMode = mode;
    //     }
    #[allow(unused_variables)]
    pub fn set_auto_bulk_mode(&mut self, mode: bool) {
        self.myAutoBulkMode = mode;
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
    pub fn prohibit(&mut self, toProhibit: &SUMOAbstractRouter_Prohibitions) where E: RouterEdgeInfo {
        if let Some(prohibit) = self.myProhibit {
            return prohibit(self, toProhibit);
        }
        for item in self.myProhibited.keys() {
            let info = &mut self.myEdgeInfos[e_numerical_id(item.cast::<E>())];
            info.prohibitedPermissions = SVC_ALL;
            info.prohibitionBegin = -1.;
            info.prohibitionEnd = -1.;
        }
        for (item, rp) in toProhibit.iter() {
            let info = &mut self.myEdgeInfos[e_numerical_id(item.cast::<E>())];
            info.prohibitionBegin = rp.begin;
            info.prohibitionEnd = rp.end;
            info.prohibitedPermissions = rp.permissions;
        }
        self.myProhibited = toProhibit.clone();
    }

    /// C++: `bool SUMOAbstractRouter::hasProhibitions() const` [const]
    // bool hasProhibitions() const {
    //         return this->myProhibited.size() > 0;
    //     }
    #[allow(unused_variables)]
    pub fn has_prohibitions(&self) -> bool {
        !self.myProhibited.is_empty()
    }

    /// C++: `bool SUMOAbstractRouter::supportsProhibitions() const` [const, virtual]
    // virtual bool supportsProhibitions() const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn supports_prohibitions(&self) -> bool {
        if let Some(supports) = self.mySupportsProhibitions {
            return supports(self);
        }
        true
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
        let mut rbegin = rbegin;
        let mut tmp: Vec<Ptr<E>> = Vec::new();
        while !rbegin.is_null() {
            tmp.push(rbegin.m().edge.cast::<E>());
            rbegin = rbegin.m().prev;
        }
        edges.extend(tmp.into_iter().rev());
    }

    /// C++: `SUMOAbstractRouter<E, V> & SUMOAbstractRouter::operator=(const SUMOAbstractRouter<E, V> &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &SUMOAbstractRouter<E, V>) -> &mut SUMOAbstractRouter<E, V> {
        panic!("SUMOAbstractRouter assignment operator is private and invalidated in C++")
    }

}

// ---- helpers (file-private) ----

fn current_millis() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

/// MsgHandler::getWarningInstance() / getErrorInstance()
fn msg_instance(warning: bool) -> Ptr<MsgHandler> {
    utils::common::msg_handler::with_instance(if warning { MsgType::Warning } else { MsgType::Error }, |h| Ptr::from_mut(h))
}

/// C++ `E::getNumericalID()`
fn e_numerical_id<E: RouterEdgeInfo>(e: Ptr<E>) -> usize {
    e.edge_numerical_id() as usize
}

/// C++ `E::getID()`
fn e_id<E: RouterEdgeInfo>(e: Ptr<E>) -> String {
    e.edge_id()
}

/// C++ `(*myOperation)(e, v, t)`. A null operation is dereferenced by C++ too, so it is a crash here as well.
fn call_op<E, V>(op: Option<SUMOAbstractRouter_TypedOperation<E, V>>, e: Ptr<E>, v: Ptr<V>, t: f64) -> f64 {
    match op {
        Some(f) => f(e, v, t),
        None => panic!("SUMOAbstractRouter: null operation called (C++ dereferences nullptr here)"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::router::dijkstra_router::DijkstraRouter;

    /// The router reads only the class and the ignore flag of the vehicle; a null vehicle is used in the tests.
    struct TestVeh;

    impl RouterVehicle for TestVeh {
        fn v_class(&self) -> SUMOVehicleClass {
            // SVC_IGNORING is 0 (the null vehicle used by the other tests), for which
            // prohibition permission bits are a no-op. Use a real class here so the
            // prohibition check behaves like C++ with an actual vehicle.
            SVC_RAIL
        }
        fn get_id(&self) -> String {
            "veh".to_string()
        }
        fn get_max_speed(&self) -> f64 {
            0.
        }
        fn get_chosen_speed_factor(&self) -> f64 {
            1.
        }
        fn ignore_transient_permissions(&self) -> bool {
            false
        }
        fn length(&self) -> f64 {
            0.
        }
    }

    /// Network edge with the accessors the router reads: id, numerical id, length and the via successors.
    struct TestEdge {
        id: String,
        numericalID: i32,
        length: f64,
        viaSuccessors: Vec<(Ptr<TestEdge>, Ptr<TestEdge>)>,
    }

    impl RouterEdgeInfo for TestEdge {
        fn edge_numerical_id(&self) -> i32 {
            self.numericalID
        }
        fn edge_id(&self) -> String {
            self.id.clone()
        }
        fn edge_permissions(&self) -> SVCPermissions {
            SVC_ALL
        }
    }

    impl RouterEdge<TestVeh> for TestEdge {
        type Node = ();
        type Lane = ();
        fn id(&self) -> &str {
            &self.id
        }
        fn numerical_id(&self) -> i32 {
            self.numericalID
        }
        fn length(&self) -> f64 {
            self.length
        }
        fn length_geometry_factor(&self) -> f64 {
            1.
        }
        fn distance_to(&self, _other: Ptr<Self>, _doBoundaryEstimate: bool) -> f64 {
            0.
        }
        fn successors(&self, _vclass: SUMOVehicleClass) -> &[Ptr<Self>] {
            &[]
        }
        fn via_successors(&self, _vclass: SUMOVehicleClass, _ignoreTransient: bool) -> &[(Ptr<Self>, Ptr<Self>)] {
            &self.viaSuccessors
        }
        fn speed_limit(&self) -> f64 {
            1.
        }
        fn minimum_travel_time(&self, _veh: Ptr<TestVeh>) -> f64 {
            self.length
        }
        fn bidi_edge(&self) -> Ptr<Self> {
            Ptr::null()
        }
        fn travel_time_static(_edge: Ptr<Self>, _veh: Ptr<TestVeh>, _time: f64) -> f64 {
            0.
        }
        fn travel_time_aggregated(_edge: Ptr<Self>, _veh: Ptr<TestVeh>, _time: f64) -> f64 {
            0.
        }
        fn is_internal(&self) -> bool {
            false
        }
        fn prohibits(&self, _veh: Ptr<TestVeh>, _checkRestrictions: bool) -> bool {
            false
        }
        fn restricts(&self, _veh: Ptr<TestVeh>) -> bool {
            false
        }
    }

    /// Effort of an edge is its length, as in a plain shortest-path query
    fn length_effort(e: Ptr<TestEdge>, _v: Ptr<TestVeh>, _t: f64) -> f64 {
        e.length
    }

    fn edge(id: &str, numericalID: i32, length: f64) -> Box<TestEdge> {
        Box::new(TestEdge { id: id.to_string(), numericalID, length, viaSuccessors: Vec::new() })
    }

    /// Diamond A -> {B, C} -> D with lengths A=10, B=20, C=5, D=10.
    /// A-B-D costs 10 + 20 + 10 = 40 and A-C-D costs 10 + 5 + 10 = 25, so the route must be A, C, D.
    #[test]
    fn compute_picks_cheapest_route_on_diamond() {
        let boxes = vec![edge("A", 0, 10.), edge("B", 1, 20.), edge("C", 2, 5.), edge("D", 3, 10.)];
        let ptrs: Vec<Ptr<TestEdge>> = boxes.iter().map(|b| Ptr::from_box(b)).collect();
        for (from, to) in [(0usize, 1usize), (0, 2), (1, 3), (2, 3)] {
            ptrs[from].m().viaSuccessors.push((ptrs[to], Ptr::null()));
        }
        let op: SUMOAbstractRouter_TypedOperation<TestEdge, TestVeh> = length_effort;
        let mut router = DijkstraRouter::new(&ptrs, true, Some(op), None, true, Ptr::null(), false, false);

        let mut route: Vec<Ptr<TestEdge>> = Vec::new();
        assert!(router.compute(ptrs[0], ptrs[3], Ptr::null(), 0, &mut route, true));
        let ids: Vec<String> = route.iter().map(|e| e.id.clone()).collect();
        assert_eq!(ids, vec!["A", "C", "D"]);

        let mut none: Vec<Ptr<TestEdge>> = Vec::new();
        assert!(!router.compute(ptrs[3], ptrs[0], Ptr::null(), 0, &mut none, true));
        assert!(none.is_empty());
    }

    /// A closing with a finite end is temporary: the router may still use the edge
    /// (paying the wait), while `end == MAX` is a hard prohibition. This is the
    /// distinction the closingReroute `until` cases depend on.
    #[test]
    fn finite_prohibition_is_temporary() {
        let a = edge("A", 0, 10.);
        let a = Ptr::from_box(&a);
        let op: SUMOAbstractRouter_TypedOperation<TestEdge, TestVeh> = length_effort;
        let edges = vec![a];
        let mut router = DijkstraRouter::new(&edges, true, Some(op), None, true, Ptr::null(), false, false);
        let veh_box = Box::new(TestVeh);
        let veh = Ptr::from_box(&veh_box);

        let mut prohibitions = SUMOAbstractRouter_Prohibitions::new();
        // MSTriggeredRerouter.cpp:149 → getClosed() sets permissions to
        // SVC_AUTHORITY for `allow="authority"`; a rail vehicle is not covered.
        prohibitions.insert(a.cast::<()>(), RouterProhibition { begin: 0., end: 200., permissions: SVC_AUTHORITY });
        router.prohibit(&prohibitions);
        assert!(!router.is_prohibited(a, veh, 0.), "finite end must not be hard-prohibited");
        assert_eq!(router.get_effort(a, veh, 0.), 200. + 10., "finite end must add the wait time");

        let mut permanent = SUMOAbstractRouter_Prohibitions::new();
        permanent.insert(a.cast::<()>(), RouterProhibition { begin: 0., end: f64::MAX, permissions: SVC_AUTHORITY });
        router.prohibit(&permanent);
        assert!(router.is_prohibited(a, veh, 0.), "MAX end must be hard-prohibited");
        // getEffort() would return MAX + effort here; C++ never calls it for a
        // hard-prohibited edge because isProhibited() excludes it first.
    }
}


