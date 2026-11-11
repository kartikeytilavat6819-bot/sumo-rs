//! Port of router/ROEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<std::pair<const ROEdge *, const ROEdge *>> ROConstEdgePairVector`
pub type ROConstEdgePairVector = Vec<(Ptr<ROEdge>, Ptr<ROEdge>)>;

/// C++ typedef `std::vector<ROEdge *> ROEdgeVector`
pub type ROEdgeVector = Vec<Ptr<ROEdge>>;

/// A basic edge for routing applications
/// C++ class `ROEdge` (router/ROEdge.h)
pub struct ROEdge {
    /// C++ base class `Named`
    pub base: Named,
    /// C++ base class `Parameterised`
    pub base2: Parameterised,
    /// the junctions for this edge
    pub myFromJunction: Ptr<RONode>,
    pub myToJunction: Ptr<RONode>,
    /// The index (numeric id) of the edge
    pub myIndex: i32,
    /// The edge priority (road class)
    pub myPriority: i32,
    /// the type of this edge
    pub myType: String,
    /// the routing type of the edge (used to look up vType and vClass specific routing preferences)
    pub myRoutingType: String,
    /// The maximum speed allowed on this edge
    pub mySpeed: f64,
    /// The length of the edge
    pub myLength: f64,
    /// whether the edge is a source or a sink
    pub myAmSink: bool,
    /// whether the edge is a source or a sink
    pub myAmSource: bool,
    /// Container storing passing time varying over time for the edge
    pub myTravelTimes: Unmapped /* ValueTimeLine<double> */,
    /// Information whether the time line shall be used instead of the length value
    pub myUsingTTTimeLine: bool,
    /// Container storing passing time varying over time for the edge
    pub myEfforts: Unmapped /* ValueTimeLine<double> */,
    /// Information whether the time line shall be used instead of the length value
    pub myUsingETimeLine: bool,
    /// List of edges that may be approached from this edge
    pub myFollowingEdges: ROEdgeVector,
    pub myFollowingViaEdges: ROConstEdgePairVector,
    /// List of edges that approached this edge
    pub myApproachingEdges: ROEdgeVector,
    /// The function of the edge
    pub myFunction: SumoXMLEdgeFunc,
    /// The vClass speed restrictions for this edge
    pub mySpeedRestrictions: Ptr<BTreeMap<SUMOVehicleClass, f64>>,
    /// This edge's lanes
    pub myLanes: Vec<Ptr<ROLane>>,
    /// The list of allowed vehicle classes combined across lanes
    pub myCombinedPermissions: SVCPermissions,
    /// the other taz-connector if this edge isTazConnector, otherwise nullptr
    pub myOtherTazConnector: Ptr<ROEdge>,
    /// the bidirectional rail edge or nullpr
    pub myBidiEdge: Ptr<ROEdge>,
    /// The bounding rectangle of end nodes incoming or outgoing edges for taz connectors or of my own start and end node for normal edges
    pub myBoundary: Boundary,
    /// flat penalty when computing traveltime
    pub myTimePenalty: f64,
    /// cached value of parameters which may restrict access
    pub myParamRestrictions: Vec<f64>,
    /// The successors available for a given vClass
    pub myClassesSuccessorMap: BTreeMap<SUMOVehicleClass, ROEdgeVector>,
    /// The successors with vias available for a given vClass
    pub myClassesViaSuccessorMap: BTreeMap<SUMOVehicleClass, ROConstEdgePairVector>,
    /// a reversed version for backward routing
    pub myReversedRoutingEdge: Ptr<ReversedEdge<ROEdge, ROVehicle>>,
    /// An extended version of the reversed edge for backward routing (used for the arc flag router)
    pub myFlippedRoutingEdge: Ptr<FlippedEdge<ROEdge, RONode, ROVehicle>>,
    pub myRailwayRoutingEdge: Ptr<RailEdge<ROEdge, ROVehicle>>,
}

impl std::ops::Deref for ROEdge {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for ROEdge {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

// C++ static members of ROEdge (port as module-level statics, see AGENTS.md):
//   static bool ROEdge::myInterpolate;
//   static bool ROEdge::myHaveEWarned;
//   static bool ROEdge::myHaveTTWarned;
//   static ROEdgeVector ROEdge::myEdges;
//   static double ROEdge::myPriorityFactor;
//   static double ROEdge::myMinEdgePriority;
//   static double ROEdge::myEdgePriorityRange;

impl ROEdge {
    /// C++: `void ROEdge::ROEdge(const std::string &, RONode *, RONode *, int, const int, const std::string &, const std::string &)`
    /// Constructor
    // ROEdge::ROEdge(const std::string& id, RONode* from, RONode* to, int index, const int priority, const std::string& type, const std::string& routingType) :
    //     Named(id),
    //     myFromJunction(from),
    //     myToJunction(to),
    //     myIndex(index),
    //     myPriority(priority),
    //     myType(type),
    //     myRoutingType(routingType),
    //     mySpeed(-1),
    //     myLength(0),
    //     myAmSink(false),
    //     myAmSource(false),
    //     myUsingTTTimeLine(false),
    //     myUsingETimeLine(false),
    //     mySpeedRestrictions(nullptr),
    //     myCombinedPermissions(0),
    //     myOtherTazConnector(nullptr),
    //     myBidiEdge(nullptr),
    //     myTimePenalty(0) {
    //     while ((int)myEdges.size() <= index) {
    //         myEdges.push_back(0);
    //     }
    //     myEdges[index] = this;
    //     if (from == nullptr && to == nullptr) {
    //         // TAZ edge, no lanes
    //         myCombinedPermissions = SVCAll;
    //     } else {
    //         // TODO we should not calculate the boundary here, the position for the nodes is not valid yet
    //         myBoundary.add(from->getPosition());
    //         myBoundary.add(to->getPosition());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(id: &str, from: Ptr<RONode>, to: Ptr<RONode>, index: i32, priority: i32, type_: &str, routingType: &str) -> Self {
        todo!("port ROEdge::ROEdge")
    }

    /// C++: `void ROEdge::ROEdge(const std::string &, const RONode *, const RONode *, SVCPermissions)`
    /// Constructor for dummy edge, only used when building the connectivity graph *
    // ROEdge::ROEdge(const std::string& id, const RONode* from, const RONode* to, SVCPermissions p) :
    //     Named(id),
    //     myFromJunction(const_cast<RONode*>(from)),
    //     myToJunction(const_cast<RONode*>(to)),
    //     myIndex(-1),
    //     myPriority(0),
    //     myType(""),
    //     mySpeed(std::numeric_limits<double>::max()),
    //     myLength(0),
    //     myAmSink(false),
    //     myAmSource(false),
    //     myUsingTTTimeLine(false),
    //     myUsingETimeLine(false),
    //     myCombinedPermissions(p),
    //     myOtherTazConnector(nullptr),
    //     myBidiEdge(nullptr),
    //     myTimePenalty(0)
    // { }
    #[allow(unused_variables)]
    pub fn new_2(id: &str, from: Ptr<RONode>, to: Ptr<RONode>, p: SVCPermissions) -> Self {
        todo!("port ROEdge::ROEdge")
    }

    /// C++: `void ROEdge::~ROEdge() noexcept` [virtual]
    /// Destructor
    // ROEdge::~ROEdge() {
    //     for (ROLane* const lane : myLanes) {
    //         delete lane;
    //     }
    //     delete myReversedRoutingEdge;
    //     delete myFlippedRoutingEdge;
    //     delete myRailwayRoutingEdge;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ROEdge::~ROEdge")
    }

    /// C++: `void ROEdge::addLane(ROLane *)` [virtual]
    /// Adds a lane to the edge while loading
    // void
    // ROEdge::addLane(ROLane* lane) {
    //     const double speed = lane->getSpeed();
    //     if (speed > mySpeed) {
    //         mySpeed = speed;
    //         myLength = lane->getLength();
    //     }
    //     mySpeed = speed > mySpeed ? speed : mySpeed;
    //     myLanes.push_back(lane);
    //
    //     // integrate new allowed classes
    //     myCombinedPermissions |= lane->getPermissions();
    // }
    #[allow(unused_variables)]
    pub fn add_lane(&mut self, lane: Ptr<ROLane>) {
        todo!("port ROEdge::addLane")
    }

    /// C++: `void ROEdge::addSuccessor(ROEdge *, ROEdge *, std::string)` [virtual]
    /// Adds information about a connected edge
    // void
    // ROEdge::addSuccessor(ROEdge* s, ROEdge* via, std::string) {
    //     if (isInternal()) {
    //         // for internal edges after an internal junction,
    //         // this is called twice and only the second call counts
    //         myFollowingEdges.clear();
    //         myFollowingViaEdges.clear();
    //     }
    //     if (find(myFollowingEdges.begin(), myFollowingEdges.end(), s) == myFollowingEdges.end()) {
    //         myFollowingEdges.push_back(s);
    //         myFollowingViaEdges.push_back(std::make_pair(s, via));
    //         if (isTazConnector() && s->getFromJunction() != nullptr) {
    //             myBoundary.add(s->getFromJunction()->getPosition());
    //         }
    //         if (!isInternal()) {
    //             s->myApproachingEdges.push_back(this);
    //             if (s->isTazConnector() && getToJunction() != nullptr) {
    //                 s->myBoundary.add(getToJunction()->getPosition());
    //             }
    //         }
    //         if (via != nullptr) {
    //             if (via->myApproachingEdges.size() == 0) {
    //                 via->myApproachingEdges.push_back(this);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_successor(&mut self, s: Ptr<ROEdge>, via: Ptr<ROEdge>, dir: String) {
        todo!("port ROEdge::addSuccessor")
    }

    /// C++: `void ROEdge::setFunction(SumoXMLEdgeFunc)`
    /// Sets the function of the edge
    // inline void setFunction(SumoXMLEdgeFunc func) {
    //         myFunction = func;
    //     }
    #[allow(unused_variables)]
    pub fn set_function(&mut self, func: SumoXMLEdgeFunc) {
        todo!("port ROEdge::setFunction")
    }

    /// C++: `void ROEdge::setSource(const bool)`
    /// Sets whether the edge is a source
    // inline void setSource(const bool isSource = true) {
    //         myAmSource = isSource;
    //     }
    #[allow(unused_variables)]
    pub fn set_source(&mut self, isSource: bool) {
        todo!("port ROEdge::setSource")
    }

    /// C++: `void ROEdge::setSink(const bool)`
    /// Sets whether the edge is a sink
    // inline void setSink(const bool isSink = true) {
    //         myAmSink = isSink;
    //     }
    #[allow(unused_variables)]
    pub fn set_sink(&mut self, isSink: bool) {
        todo!("port ROEdge::setSink")
    }

    /// C++: `void ROEdge::setSpeedRestrictions(const std::map<SUMOVehicleClass, double> *)`
    /// Sets the vehicle class specific speed limits of the edge
    // inline void setSpeedRestrictions(const std::map<SUMOVehicleClass, double>* restrictions) {
    //         mySpeedRestrictions = restrictions;
    //     }
    #[allow(unused_variables)]
    pub fn set_speed_restrictions(&mut self, restrictions: Ptr<BTreeMap<SUMOVehicleClass, f64>>) {
        todo!("port ROEdge::setSpeedRestrictions")
    }

    /// C++: `void ROEdge::setTimePenalty(double)`
    // inline void setTimePenalty(double value) {
    //         myTimePenalty = value;
    //     }
    #[allow(unused_variables)]
    pub fn set_time_penalty(&mut self, value: f64) {
        todo!("port ROEdge::setTimePenalty")
    }

    /// C++: `double ROEdge::getTimePenalty() const` [const]
    // inline double getTimePenalty() const {
    //         return myTimePenalty;
    //     }
    #[allow(unused_variables)]
    pub fn get_time_penalty(&self) -> f64 {
        todo!("port ROEdge::getTimePenalty")
    }

    /// C++: `bool ROEdge::isNormal() const` [const]
    /// return whether this edge is a normal edge
    // inline bool isNormal() const {
    //         return myFunction == SumoXMLEdgeFunc::NORMAL;
    //     }
    #[allow(unused_variables)]
    pub fn is_normal(&self) -> bool {
        todo!("port ROEdge::isNormal")
    }

    /// C++: `bool ROEdge::isInternal() const` [const]
    /// return whether this edge is an internal edge
    // inline bool isInternal() const {
    //         return myFunction == SumoXMLEdgeFunc::INTERNAL;
    //     }
    #[allow(unused_variables)]
    pub fn is_internal(&self) -> bool {
        todo!("port ROEdge::isInternal")
    }

    /// C++: `bool ROEdge::isCrossing() const` [const]
    /// return whether this edge is a pedestrian crossing
    // inline bool isCrossing() const {
    //         return myFunction == SumoXMLEdgeFunc::CROSSING;
    //     }
    #[allow(unused_variables)]
    pub fn is_crossing(&self) -> bool {
        todo!("port ROEdge::isCrossing")
    }

    /// C++: `bool ROEdge::isWalkingArea() const` [const]
    /// return whether this edge is walking area
    // inline bool isWalkingArea() const {
    //         return myFunction == SumoXMLEdgeFunc::WALKINGAREA;
    //     }
    #[allow(unused_variables)]
    pub fn is_walking_area(&self) -> bool {
        todo!("port ROEdge::isWalkingArea")
    }

    /// C++: `bool ROEdge::isTazConnector() const` [const]
    // inline bool isTazConnector() const {
    //         return myFunction == SumoXMLEdgeFunc::CONNECTOR;
    //     }
    #[allow(unused_variables)]
    pub fn is_taz_connector(&self) -> bool {
        todo!("port ROEdge::isTazConnector")
    }

    /// C++: `void ROEdge::setOtherTazConnector(const ROEdge *)`
    // void setOtherTazConnector(const ROEdge* edge) {
    //         myOtherTazConnector = edge;
    //     }
    #[allow(unused_variables)]
    pub fn set_other_taz_connector(&mut self, edge: Ptr<ROEdge>) {
        todo!("port ROEdge::setOtherTazConnector")
    }

    /// C++: `const ROEdge * ROEdge::getOtherTazConnector() const` [const]
    // const ROEdge* getOtherTazConnector() const {
    //         return myOtherTazConnector;
    //     }
    #[allow(unused_variables)]
    pub fn get_other_taz_connector(&self) -> Ptr<ROEdge> {
        todo!("port ROEdge::getOtherTazConnector")
    }

    /// C++: `void ROEdge::buildTimeLines(const std::string &, const bool)`
    /// Builds the internal representation of the travel time/effort
    // void
    // ROEdge::buildTimeLines(const std::string& measure, const bool boundariesOverride) {
    //     if (myUsingETimeLine) {
    //         double value = myLength / mySpeed;
    //         const SUMOEmissionClass c = PollutantsInterface::getClassByName("HBEFA4/default");
    //         if (measure == "CO") {
    //             value = PollutantsInterface::compute(c, PollutantsInterface::CO, mySpeed, 0, 0, nullptr) * value; // @todo: give correct slope
    //         }
    //         if (measure == "CO2") {
    //             value = PollutantsInterface::compute(c, PollutantsInterface::CO2, mySpeed, 0, 0, nullptr) * value; // @todo: give correct slope
    //         }
    //         if (measure == "HC") {
    //             value = PollutantsInterface::compute(c, PollutantsInterface::HC, mySpeed, 0, 0, nullptr) * value; // @todo: give correct slope
    //         }
    //         if (measure == "PMx") {
    //             value = PollutantsInterface::compute(c, PollutantsInterface::PM_X, mySpeed, 0, 0, nullptr) * value; // @todo: give correct slope
    //         }
    //         if (measure == "NOx") {
    //             value = PollutantsInterface::compute(c, PollutantsInterface::NO_X, mySpeed, 0, 0, nullptr) * value; // @todo: give correct slope
    //         }
    //         if (measure == "fuel") {
    //             value = PollutantsInterface::compute(c, PollutantsInterface::FUEL, mySpeed, 0, 0, nullptr) * value; // @todo: give correct slope
    //         }
    //         if (measure == "electricity") {
    //             value = PollutantsInterface::compute(c, PollutantsInterface::ELEC, mySpeed, 0, 0, nullptr) * value; // @todo: give correct slope
    //         }
    //         myEfforts.fillGaps(value, boundariesOverride);
    //     }
    //     if (myUsingTTTimeLine) {
    //         myTravelTimes.fillGaps(myLength / mySpeed + myTimePenalty, boundariesOverride);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_time_lines(&mut self, measure: &str, boundariesOverride: bool) {
        todo!("port ROEdge::buildTimeLines")
    }

    /// C++: `void ROEdge::cacheParamRestrictions(const std::vector<std::string> &)`
    // void
    // ROEdge::cacheParamRestrictions(const std::vector<std::string>& restrictionKeys) {
    //     for (const std::string& key : restrictionKeys) {
    //         const std::string value = getParameter(key, "1e40");
    //         myParamRestrictions.push_back(StringUtils::toDouble(value));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn cache_param_restrictions(&mut self, restrictionKeys: &Vec<String>) {
        todo!("port ROEdge::cacheParamRestrictions")
    }

    /// C++: `SumoXMLEdgeFunc ROEdge::getFunction() const` [const]
    /// Returns the function of the edge
    // inline SumoXMLEdgeFunc getFunction() const {
    //         return myFunction;
    //     }
    #[allow(unused_variables)]
    pub fn get_function(&self) -> SumoXMLEdgeFunc {
        todo!("port ROEdge::getFunction")
    }

    /// C++: `bool ROEdge::isSink() const` [const]
    /// Returns whether the edge acts as a sink
    // inline bool isSink() const {
    //         return myAmSink;
    //     }
    #[allow(unused_variables)]
    pub fn is_sink(&self) -> bool {
        todo!("port ROEdge::isSink")
    }

    /// C++: `double ROEdge::getLength() const` [const]
    /// Returns the length of the edge
    // double getLength() const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        todo!("port ROEdge::getLength")
    }

    /// C++: `int ROEdge::getNumericalID() const` [const]
    /// Returns the index (numeric id) of the edge
    // int getNumericalID() const {
    //         return myIndex;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> i32 {
        todo!("port ROEdge::getNumericalID")
    }

    /// C++: `double ROEdge::getSpeedLimit() const` [const]
    /// Returns the speed allowed on this edge
    // double getSpeedLimit() const {
    //         return mySpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_limit(&self) -> f64 {
        todo!("port ROEdge::getSpeedLimit")
    }

    /// C++: `double ROEdge::getLengthGeometryFactor() const` [const]
    /// return a lower bound on shape.length() / myLength that is
    // double
    // ROEdge::getLengthGeometryFactor() const {
    //     return myLanes.empty() ? 1. : myLanes[0]->getShape().length() / myLanes[0]->getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_length_geometry_factor(&self) -> f64 {
        todo!("port ROEdge::getLengthGeometryFactor")
    }

    /// C++: `int ROEdge::getNumLanes() const` [const]
    /// Returns the number of lanes this edge has
    // int getNumLanes() const {
    //         return (int) myLanes.size();
    //     }
    #[allow(unused_variables)]
    pub fn get_num_lanes(&self) -> i32 {
        todo!("port ROEdge::getNumLanes")
    }

    /// C++: `bool ROEdge::isConnectedTo(const ROEdge &, const SUMOVehicleClass, bool) const` [const]
    /// returns the information whether this edge is directly connected to the given
    // bool
    // ROEdge::isConnectedTo(const ROEdge& e, const SUMOVehicleClass vClass, bool ignoreTransientPermissions) const {
    //     // @todo needs to be used with #12501
    //     UNUSED_PARAMETER(ignoreTransientPermissions);
    //     const ROEdgeVector& followers = getSuccessors(vClass);
    //     return std::find(followers.begin(), followers.end(), &e) != followers.end();
    // }
    #[allow(unused_variables)]
    pub fn is_connected_to(&self, e: &ROEdge, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> bool {
        todo!("port ROEdge::isConnectedTo")
    }

    /// C++: `bool ROEdge::prohibits(const ROVehicle *const, bool) const` [const]
    /// Returns whether this edge prohibits the given vehicle to pass it
    // inline bool prohibits(const ROVehicle* const vehicle, bool checkRestrictions = false) const {
    //         const SUMOVehicleClass vclass = vehicle->getVClass();
    //         return (myCombinedPermissions & vclass) != vclass || (checkRestrictions && restricts(vehicle));
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, vehicle: Ptr<ROVehicle>, checkRestrictions: bool) -> bool {
        todo!("port ROEdge::prohibits")
    }

    /// C++: `SVCPermissions ROEdge::getPermissions() const` [const]
    // inline SVCPermissions getPermissions() const {
    //         return myCombinedPermissions;
    //     }
    #[allow(unused_variables)]
    pub fn get_permissions(&self) -> SVCPermissions {
        todo!("port ROEdge::getPermissions")
    }

    /// C++: `bool ROEdge::restricts(const ROVehicle *const) const` [const]
    /// Returns whether this edge has restriction parameters forbidding the given vehicle to pass it
    // inline bool restricts(const ROVehicle* const vehicle) const {
    //         const std::vector<double>& vTypeRestrictions = vehicle->getType()->paramRestrictions;
    //         assert(vTypeRestrictions.size() == myParamRestrictions.size());
    //         for (int i = 0; i < (int)vTypeRestrictions.size(); i++) {
    //             if (vTypeRestrictions[i] > myParamRestrictions[i]) {
    //                 return true;
    //             }
    //         }
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn restricts(&self, vehicle: Ptr<ROVehicle>) -> bool {
        todo!("port ROEdge::restricts")
    }

    /// C++: `bool ROEdge::allFollowersProhibit(const ROVehicle *const) const` [const]
    /// Returns whether this edge succeeding edges prohibit the given vehicle to pass them
    // bool
    // ROEdge::allFollowersProhibit(const ROVehicle* const vehicle) const {
    //     for (ROEdgeVector::const_iterator i = myFollowingEdges.begin(); i != myFollowingEdges.end(); ++i) {
    //         if (!(*i)->prohibits(vehicle)) {
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn all_followers_prohibit(&self, vehicle: Ptr<ROVehicle>) -> bool {
        todo!("port ROEdge::allFollowersProhibit")
    }

    /// C++: `void ROEdge::addEffort(double, double, double)`
    /// Adds a weight value
    // void
    // ROEdge::addEffort(double value, double timeBegin, double timeEnd) {
    //     myEfforts.add(timeBegin, timeEnd, value);
    //     myUsingETimeLine = true;
    // }
    #[allow(unused_variables)]
    pub fn add_effort(&mut self, value: f64, timeBegin: f64, timeEnd: f64) {
        todo!("port ROEdge::addEffort")
    }

    /// C++: `void ROEdge::addTravelTime(double, double, double)`
    /// Adds a travel time value
    // void
    // ROEdge::addTravelTime(double value, double timeBegin, double timeEnd) {
    //     myTravelTimes.add(timeBegin, timeEnd, value);
    //     myUsingTTTimeLine = true;
    // }
    #[allow(unused_variables)]
    pub fn add_travel_time(&mut self, value: f64, timeBegin: f64, timeEnd: f64) {
        todo!("port ROEdge::addTravelTime")
    }

    /// C++: `int ROEdge::getNumSuccessors() const` [const]
    /// Returns the number of edges this edge is connected to
    // int
    // ROEdge::getNumSuccessors() const {
    //     if (myAmSink) {
    //         return 0;
    //     }
    //     return (int) myFollowingEdges.size();
    // }
    #[allow(unused_variables)]
    pub fn get_num_successors(&self) -> i32 {
        todo!("port ROEdge::getNumSuccessors")
    }

    /// C++: `const ROEdgeVector & ROEdge::getSuccessors(SUMOVehicleClass) const` [const]
    /// Returns the following edges, restricted by vClass
    // const ROEdgeVector&
    // ROEdge::getSuccessors(SUMOVehicleClass vClass) const {
    //     if (vClass == SVC_IGNORING || !RONet::getInstance()->hasPermissions() || isTazConnector()) {
    //         return myFollowingEdges;
    //     }
    // #ifdef HAVE_FOX
    //     FXMutexLock locker(myLock);
    // #endif
    //     std::map<SUMOVehicleClass, ROEdgeVector>::const_iterator i = myClassesSuccessorMap.find(vClass);
    //     if (i != myClassesSuccessorMap.end()) {
    //         // can use cached value
    //         return i->second;
    //     }
    //     // this vClass is requested for the first time. rebuild all successors
    //     std::set<ROEdge*> followers;
    //     for (const ROLane* const lane : myLanes) {
    //         if ((lane->getPermissions() & vClass) != 0) {
    //             for (const auto& next : lane->getOutgoingViaLanes()) {
    //                 if ((next.first->getPermissions() & vClass) != 0 && (next.second == nullptr || (next.second->getPermissions() & vClass) != 0)) {
    //                     followers.insert(&next.first->getEdge());
    //                 }
    //             }
    //         }
    //     }
    //     // also add district edges (they are not connected at the lane level
    //     for (ROEdgeVector::const_iterator it = myFollowingEdges.begin(); it != myFollowingEdges.end(); ++it) {
    //         if ((*it)->isTazConnector()) {
    //             followers.insert(*it);
    //         }
    //     }
    //     myClassesSuccessorMap[vClass].insert(myClassesSuccessorMap[vClass].begin(),
    //                                          followers.begin(), followers.end());
    //     return myClassesSuccessorMap[vClass];
    // }
    #[allow(unused_variables)]
    pub fn get_successors(&self, vClass: SUMOVehicleClass) -> &ROEdgeVector {
        todo!("port ROEdge::getSuccessors")
    }

    /// C++: `const ROConstEdgePairVector & ROEdge::getViaSuccessors(SUMOVehicleClass, bool) const` [const]
    /// Returns the following edges including vias, restricted by vClass
    // const ROConstEdgePairVector&
    // ROEdge::getViaSuccessors(SUMOVehicleClass vClass, bool /*ignoreTransientPermissions*/) const {
    //     if (vClass == SVC_IGNORING || !RONet::getInstance()->hasPermissions() || isTazConnector()) {
    //         return myFollowingViaEdges;
    //     }
    // #ifdef HAVE_FOX
    //     FXMutexLock locker(myLock);
    // #endif
    //     std::map<SUMOVehicleClass, ROConstEdgePairVector>::const_iterator i = myClassesViaSuccessorMap.find(vClass);
    //     if (i != myClassesViaSuccessorMap.end()) {
    //         // can use cached value
    //         return i->second;
    //     }
    //     // this vClass is requested for the first time. rebuild all successors
    //     std::set<std::pair<const ROEdge*, const ROEdge*> > followers;
    //     for (const ROLane* const lane : myLanes) {
    //         if ((lane->getPermissions() & vClass) != 0) {
    //             for (const auto& next : lane->getOutgoingViaLanes()) {
    //                 if ((next.first->getPermissions() & vClass) != 0 && (next.second == nullptr || (next.second->getPermissions() & vClass) != 0)) {
    //                     followers.insert(std::make_pair(&next.first->getEdge(), next.second));
    //                 }
    //             }
    //         }
    //     }
    //     // also add district edges (they are not connected at the lane level
    //     for (const ROEdge* e : myFollowingEdges) {
    //         if (e->isTazConnector()) {
    //             followers.insert(std::make_pair(e, e));
    //         }
    //     }
    //     myClassesViaSuccessorMap[vClass].insert(myClassesViaSuccessorMap[vClass].begin(),
    //                                             followers.begin(), followers.end());
    //     return myClassesViaSuccessorMap[vClass];
    // }
    #[allow(unused_variables)]
    pub fn get_via_successors(&self, vClass: SUMOVehicleClass, ignoreTransientPermissions: bool) -> &ROConstEdgePairVector {
        todo!("port ROEdge::getViaSuccessors")
    }

    /// C++: `void ROEdge::resetSuccessors()`
    /// reset after lane permissions changes
    // void resetSuccessors() {
    //         myClassesSuccessorMap.clear();
    //         myClassesViaSuccessorMap.clear();
    //     }
    #[allow(unused_variables)]
    pub fn reset_successors(&mut self) {
        todo!("port ROEdge::resetSuccessors")
    }

    /// C++: `int ROEdge::getNumPredecessors() const` [const]
    /// Returns the number of edges connected to this edge
    // int
    // ROEdge::getNumPredecessors() const {
    //     if (myAmSource) {
    //         return 0;
    //     }
    //     return (int) myApproachingEdges.size();
    // }
    #[allow(unused_variables)]
    pub fn get_num_predecessors(&self) -> i32 {
        todo!("port ROEdge::getNumPredecessors")
    }

    /// C++: `const ROEdgeVector & ROEdge::getPredecessors() const` [const]
    /// Returns the edge at the given position from the list of incoming edges
    // const ROEdgeVector& getPredecessors() const {
    //         return myApproachingEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_predecessors(&self) -> &ROEdgeVector {
        todo!("port ROEdge::getPredecessors")
    }

    /// C++: `const ROEdge * ROEdge::getNormalBefore() const` [const]
    /// if this edge is an internal edge, return its first normal predecessor, otherwise the edge itself
    // const ROEdge*
    // ROEdge::getNormalBefore() const {
    //     const ROEdge* result = this;
    //     while (result->isInternal()) {
    //         assert(myApproachingEdges.size() == 1);
    //         result = result->myApproachingEdges.front();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_normal_before(&self) -> Ptr<ROEdge> {
        todo!("port ROEdge::getNormalBefore")
    }

    /// C++: `const ROEdge * ROEdge::getNormalAfter() const` [const]
    /// if this edge is an internal edge, return its first normal successor, otherwise the edge itself
    // const ROEdge*
    // ROEdge::getNormalAfter() const {
    //     const ROEdge* result = this;
    //     while (result->isInternal()) {
    //         assert(myFollowingEdges.size() == 1);
    //         result = result->myFollowingEdges.front();
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_normal_after(&self) -> Ptr<ROEdge> {
        todo!("port ROEdge::getNormalAfter")
    }

    /// C++: `double ROEdge::getEffort(const ROVehicle *const, double) const` [const]
    /// Returns the effort for this edge
    // double
    // ROEdge::getEffort(const ROVehicle* const veh, double time) const {
    //     double ret = 0;
    //     if (!getStoredEffort(time, ret)) {
    //         return myLength / getMaxSpeed(veh);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_effort(&self, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        todo!("port ROEdge::getEffort")
    }

    /// C++: `bool ROEdge::hasLoadedTravelTime(double) const` [const]
    /// Returns whether a travel time for this edge was loaded
    // bool
    // ROEdge::hasLoadedTravelTime(double time) const {
    //     return myUsingTTTimeLine && myTravelTimes.describesTime(time);
    // }
    #[allow(unused_variables)]
    pub fn has_loaded_travel_time(&self, time: f64) -> bool {
        todo!("port ROEdge::hasLoadedTravelTime")
    }

    /// C++: `double ROEdge::getTravelTime(const ROVehicle *const, double) const` [const]
    /// Returns the travel time for this edge
    // double
    // ROEdge::getTravelTime(const ROVehicle* const veh, double time) const {
    //     if (myUsingTTTimeLine) {
    //         if (myTravelTimes.describesTime(time)) {
    //             double lineTT = myTravelTimes.getValue(time);
    //             if (myInterpolate) {
    //                 const double inTT = lineTT;
    //                 const double split = (double)(myTravelTimes.getSplitTime(time, time + inTT) - time);
    //                 if (split >= 0) {
    //                     lineTT = myTravelTimes.getValue(time + inTT) * ((double)1. - split / inTT) + split;
    //                 }
    //             }
    //             return MAX2(getMinimumTravelTime(veh), lineTT);
    //         } else {
    //             if (!myHaveTTWarned) {
    //                 WRITE_WARNINGF(TL("No interval matches passed time=% in edge '%'.\n Using edge's length / max speed."), time, myID);
    //                 myHaveTTWarned = true;
    //             }
    //         }
    //     }
    //     const double speed = veh != nullptr ? getMaxSpeed(veh) : mySpeed;
    //     return myLength / speed + myTimePenalty;
    // }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        todo!("port ROEdge::getTravelTime")
    }

    /// C++: `double ROEdge::getEffortStatic(const ROEdge *const, const ROVehicle *const, double)` [static]
    /// Returns the effort for the given edge
    // static inline double getEffortStatic(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //         return edge->getEffort(veh, time);
    //     }
    #[allow(unused_variables)]
    pub fn get_effort_static(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        todo!("port ROEdge::getEffortStatic")
    }

    /// C++: `double ROEdge::getTravelTimeStatic(const ROEdge *const, const ROVehicle *const, double)` [static]
    /// Returns the travel time for the given edge
    // static inline double getTravelTimeStatic(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //         return edge->getTravelTime(veh, time) * getRoutingFactor(edge, veh);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        todo!("port ROEdge::getTravelTimeStatic")
    }

    /// C++: `double ROEdge::getTravelTimeStaticRandomized(const ROEdge *const, const ROVehicle *const, double)` [static]
    // static inline double getTravelTimeStaticRandomized(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //         return edge->getTravelTime(veh, time)
    //                * (1 + RandHelper::randHash(veh->getRandomSeed() ^ edge->getNumericalID()) * (gWeightsRandomFactor - 1))
    //                * getRoutingFactor(edge, veh);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static_randomized(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        todo!("port ROEdge::getTravelTimeStaticRandomized")
    }

    /// C++: `double ROEdge::getTravelTimeAggregated(const ROEdge *const, const ROVehicle *const, double)` [static]
    /// Alias for getTravelTimeStatic (there is no routing device to provide aggregated travel times)
    // static inline double getTravelTimeAggregated(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //         return edge->getTravelTime(veh, time) * getRoutingFactor(edge, veh);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_aggregated(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        todo!("port ROEdge::getTravelTimeAggregated")
    }

    /// C++: `double ROEdge::getTravelTimeStaticPriorityFactor(const ROEdge *const, const ROVehicle *const, double)` [static]
    /// Return traveltime weighted by edge priority (scaled penalty for low-priority edges)
    // static inline double getTravelTimeStaticPriorityFactor(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //         double result = edge->getTravelTime(veh, time);
    //         // lower priority should result in higher effort (and the edge with
    //         // minimum priority receives a factor of myPriorityFactor
    //         const double relativeInversePrio = 1 - ((edge->getPriority() - myMinEdgePriority) / myEdgePriorityRange);
    //         result *= 1 + relativeInversePrio * myPriorityFactor;
    //         return result * getRoutingFactor(edge, veh);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static_priority_factor(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        todo!("port ROEdge::getTravelTimeStaticPriorityFactor")
    }

    /// C++: `double ROEdge::getTravelTimeStaticPriorityFactorRandomized(const ROEdge *const, const ROVehicle *const, double)` [static]
    /// Return traveltime weighted by edge priority (scaled penalty for low-priority edges)
    // static inline double getTravelTimeStaticPriorityFactorRandomized(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //         double result = edge->getTravelTime(veh, time);
    //         // lower priority should result in higher effort (and the edge with
    //         // minimum priority receives a factor of myPriorityFactor
    //         const double relativeInversePrio = 1 - ((edge->getPriority() - myMinEdgePriority) / myEdgePriorityRange);
    //         result *= 1 + relativeInversePrio * myPriorityFactor;
    //         result *= (1 + RandHelper::randHash(veh->getRandomSeed() ^ edge->getNumericalID()) * (gWeightsRandomFactor - 1));
    //         return result * getRoutingFactor(edge, veh);
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time_static_priority_factor_randomized(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        todo!("port ROEdge::getTravelTimeStaticPriorityFactorRandomized")
    }

    /// C++: `double ROEdge::getRoutingFactor(const ROEdge *const, const ROVehicle *const)` [static]
    // static inline double getRoutingFactor(const ROEdge* const edge, const ROVehicle* const veh) {
    //         return gRoutingPreferences ?  1 / edge->getPreference(veh->getVTypeParameter()) : 1;
    //     }
    #[allow(unused_variables)]
    pub fn get_routing_factor(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>) -> f64 {
        todo!("port ROEdge::getRoutingFactor")
    }

    /// C++: `double ROEdge::getMinimumTravelTime(const ROVehicle *const) const` [const]
    /// Returns a lower bound for the travel time on this edge without using any stored timeLine
    // inline double getMinimumTravelTime(const ROVehicle* const veh) const {
    //         if (isTazConnector()) {
    //             return 0;
    //         } else if (veh != 0) {
    //             return myLength / getMaxSpeed(veh);
    //         } else {
    //             return myLength / mySpeed;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_minimum_travel_time(&self, veh: Ptr<ROVehicle>) -> f64 {
        todo!("port ROEdge::getMinimumTravelTime")
    }

    /// C++: `double ROEdge::getMaxSpeed(const RORoutable *const) const` [const]
    // inline double getMaxSpeed(const RORoutable* const veh) const {
    //         return MIN2(veh->getMaxSpeed(), veh->getChosenSpeedFactor() * getVClassMaxSpeed(veh->getVClass()));
    //     }
    #[allow(unused_variables)]
    pub fn get_max_speed(&self, veh: Ptr<RORoutable>) -> f64 {
        todo!("port ROEdge::getMaxSpeed")
    }

    /// C++: `double ROEdge::getVClassMaxSpeed(SUMOVehicleClass) const` [const]
    /// Returns the lane's maximum speed, given a vehicle's speed limit adaptation
    // inline double getVClassMaxSpeed(SUMOVehicleClass vclass) const {
    //         if (mySpeedRestrictions != 0) {
    //             std::map<SUMOVehicleClass, double>::const_iterator r = mySpeedRestrictions->find(vclass);
    //             if (r != mySpeedRestrictions->end()) {
    //                 return r->second;
    //             }
    //         }
    //         return mySpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_v_class_max_speed(&self, vclass: SUMOVehicleClass) -> f64 {
        todo!("port ROEdge::getVClassMaxSpeed")
    }

    /// C++: `double ROEdge::getEmissionEffort(const ROEdge *const, const ROVehicle *const, double)` [static]
    // template<PollutantsInterface::EmissionType ET>
    //     static double getEmissionEffort(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //         double ret = 0;
    //         if (!edge->getStoredEffort(time, ret)) {
    //             const SUMOVTypeParameter* const type = veh->getType();
    //             const double vMax = edge->getMaxSpeed(veh);
    //             const double accel = type->getCFParam(SUMO_ATTR_ACCEL, SUMOVTypeParameter::getDefaultAccel(type->vehicleClass)) * type->getCFParam(SUMO_ATTR_SIGMA, SUMOVTypeParameter::getDefaultImperfection(type->vehicleClass)) / 2.;
    //             ret = PollutantsInterface::computeDefault(type->emissionClass, ET, vMax, accel, 0, edge->getTravelTime(veh, time), nullptr); // @todo: give correct slope
    //         }
    //         return ret;
    //     }
    #[allow(unused_variables)]
    pub fn get_emission_effort() -> f64 {
        todo!("port ROEdge::getEmissionEffort")
    }

    /// C++: `double ROEdge::getNoiseEffort(const ROEdge *const, const ROVehicle *const, double)` [static]
    // double
    // ROEdge::getNoiseEffort(const ROEdge* const edge, const ROVehicle* const veh, double time) {
    //     double ret = 0;
    //     if (!edge->getStoredEffort(time, ret)) {
    //         ret = HelpersHarmonoise::computeNoise(veh->getType()->emissionClass, edge->getMaxSpeed(veh), 0);
    //     }
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn get_noise_effort(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
        todo!("port ROEdge::getNoiseEffort")
    }

    /// C++: `double ROEdge::getStoredEffort(const ROEdge *const, const ROVehicle *const, double)` [static]
    // static double getStoredEffort(const ROEdge* const edge, const ROVehicle* const /*veh*/, double time) {
    //         double ret = 0;
    //         edge->getStoredEffort(time, ret);
    //         return ret;
    //     }
    #[allow(unused_variables)]
    pub fn get_stored_effort(edge: Ptr<ROEdge>, arg1: Ptr<ROVehicle>, time: f64) -> f64 {
        todo!("port ROEdge::getStoredEffort")
    }

    /// C++: `double ROEdge::getDistanceTo(const ROEdge *, const bool) const` [const]
    /// optimistic distance heuristic for use in routing
    // double
    // ROEdge::getDistanceTo(const ROEdge* other, const bool doBoundaryEstimate) const {
    //     assert(this != other);
    //     if (doBoundaryEstimate) {
    //         return myBoundary.distanceTo2D(other->myBoundary);
    //     }
    //     if (isTazConnector()) {
    //         if (other->isTazConnector()) {
    //             return myBoundary.distanceTo2D(other->myBoundary);
    //         }
    //         return myBoundary.distanceTo2D(other->getFromJunction()->getPosition());
    //     }
    //     if (other->isTazConnector()) {
    //         return other->myBoundary.distanceTo2D(getToJunction()->getPosition());
    //     }
    //     return getLanes()[0]->getShape()[-1].distanceTo2D(other->getLanes()[0]->getShape()[0]);
    //     //return getToJunction()->getPosition().distanceTo2D(other->getFromJunction()->getPosition());
    // }
    #[allow(unused_variables)]
    pub fn get_distance_to(&self, other: Ptr<ROEdge>, doBoundaryEstimate: bool) -> f64 {
        todo!("port ROEdge::getDistanceTo")
    }

    /// C++: `const ROEdgeVector & ROEdge::getAllEdges()` [static]
    /// Returns all ROEdges
    // const ROEdgeVector&
    // ROEdge::getAllEdges() {
    //     return myEdges;
    // }
    #[allow(unused_variables)]
    pub fn get_all_edges() -> &'static ROEdgeVector {
        todo!("port ROEdge::getAllEdges")
    }

    /// C++: `void ROEdge::setGlobalOptions(const bool)` [static]
    // static void setGlobalOptions(const bool interpolate) {
    //         myInterpolate = interpolate;
    //     }
    #[allow(unused_variables)]
    pub fn set_global_options(interpolate: bool) {
        todo!("port ROEdge::setGlobalOptions")
    }

    /// C++: `void ROEdge::disableTimelineWarning()` [static]
    // static void disableTimelineWarning() {
    //         myHaveTTWarned = true;
    //     }
    #[allow(unused_variables)]
    pub fn disable_timeline_warning() {
        todo!("port ROEdge::disableTimelineWarning")
    }

    /// C++: `const Position ROEdge::getStopPosition(const SUMOVehicleParameter::Stop &)` [static]
    /// return the coordinates of the center of the given stop
    // const Position
    // ROEdge::getStopPosition(const SUMOVehicleParameter::Stop& stop) {
    //     const double middle = (stop.endPos + stop.startPos) / 2.;
    //     const ROEdge* const edge = RONet::getInstance()->getEdge(SUMOXMLDefinitions::getEdgeIDFromLane(stop.lane));
    //     return (edge->getFromJunction()->getPosition() + edge->getToJunction()->getPosition()) * (middle / edge->getLength());
    // }
    #[allow(unused_variables)]
    pub fn get_stop_position(stop: &SUMOVehicleParameter_Stop) -> Position {
        todo!("port ROEdge::getStopPosition")
    }

    /// C++: `double ROEdge::getPreference(const SUMOVTypeParameter &) const` [const]
    /// return loaded edge preference based on routingType
    // inline double getPreference(const SUMOVTypeParameter& pars) const {
    //         return RONet::getInstance()->getPreference(getRoutingType(), pars);
    //     }
    #[allow(unused_variables)]
    pub fn get_preference(&self, pars: &SUMOVTypeParameter) -> f64 {
        todo!("port ROEdge::getPreference")
    }

    /// C++: `int ROEdge::getPriority() const` [const]
    /// get edge priority (road class)
    // int getPriority() const {
    //         return myPriority;
    //     }
    #[allow(unused_variables)]
    pub fn get_priority(&self) -> i32 {
        todo!("port ROEdge::getPriority")
    }

    /// C++: `const std::string & ROEdge::getType() const` [const]
    /// get edge type
    // const std::string& getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> &str {
        todo!("port ROEdge::getType")
    }

    /// C++: `const std::string & ROEdge::getRoutingType() const` [const]
    // const std::string& getRoutingType() const {
    //         return myRoutingType.empty() ? myType : myRoutingType;
    //     }
    #[allow(unused_variables)]
    pub fn get_routing_type(&self) -> &str {
        todo!("port ROEdge::getRoutingType")
    }

    /// C++: `const RONode * ROEdge::getFromJunction() const` [const]
    // const RONode* getFromJunction() const {
    //         return myFromJunction;
    //     }
    #[allow(unused_variables)]
    pub fn get_from_junction(&self) -> Ptr<RONode> {
        todo!("port ROEdge::getFromJunction")
    }

    /// C++: `const RONode * ROEdge::getToJunction() const` [const]
    // const RONode* getToJunction() const {
    //         return myToJunction;
    //     }
    #[allow(unused_variables)]
    pub fn get_to_junction(&self) -> Ptr<RONode> {
        todo!("port ROEdge::getToJunction")
    }

    /// C++: `const std::vector<ROLane *> & ROEdge::getLanes() const` [const]
    /// Returns this edge's lanes
    // const std::vector<ROLane*>& getLanes() const {
    //         return myLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_lanes(&self) -> &Vec<Ptr<ROLane>> {
        todo!("port ROEdge::getLanes")
    }

    /// C++: `const ROEdge * ROEdge::getBidiEdge() const` [const]
    /// return opposite superposable/congruent edge, if it exist and 0 else
    // inline const ROEdge* getBidiEdge() const {
    //         return myBidiEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_bidi_edge(&self) -> Ptr<ROEdge> {
        todo!("port ROEdge::getBidiEdge")
    }

    /// C++: `void ROEdge::setBidiEdge(const ROEdge *)`
    /// set opposite superposable/congruent edge
    // inline void setBidiEdge(const ROEdge* bidiEdge) {
    //         myBidiEdge = bidiEdge;
    //     }
    #[allow(unused_variables)]
    pub fn set_bidi_edge(&mut self, bidiEdge: Ptr<ROEdge>) {
        todo!("port ROEdge::setBidiEdge")
    }

    /// C++: `ReversedEdge<ROEdge, ROVehicle> * ROEdge::getReversedRoutingEdge() const` [const]
    // ReversedEdge<ROEdge, ROVehicle>* getReversedRoutingEdge() const {
    //         if (myReversedRoutingEdge == nullptr) {
    //             myReversedRoutingEdge = new ReversedEdge<ROEdge, ROVehicle>(this);
    //         }
    //         return myReversedRoutingEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_reversed_routing_edge(&self) -> Ptr<ReversedEdge<ROEdge, ROVehicle>> {
        todo!("port ROEdge::getReversedRoutingEdge")
    }

    /// C++: `FlippedEdge<ROEdge, RONode, ROVehicle> * ROEdge::getFlippedRoutingEdge() const` [const]
    // FlippedEdge<ROEdge, RONode, ROVehicle>* getFlippedRoutingEdge() const {
    //         if (myFlippedRoutingEdge == nullptr) {
    //             myFlippedRoutingEdge = new FlippedEdge<ROEdge, RONode, ROVehicle>(this);
    //         }
    //         return myFlippedRoutingEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_flipped_routing_edge(&self) -> Ptr<FlippedEdge<ROEdge, RONode, ROVehicle>> {
        todo!("port ROEdge::getFlippedRoutingEdge")
    }

    /// C++: `RailEdge<ROEdge, ROVehicle> * ROEdge::getRailwayRoutingEdge() const` [const]
    // RailEdge<ROEdge, ROVehicle>* getRailwayRoutingEdge() const {
    //         if (myRailwayRoutingEdge == nullptr) {
    //             myRailwayRoutingEdge = new RailEdge<ROEdge, ROVehicle>(this);
    //         }
    //         return myRailwayRoutingEdge;
    //     }
    #[allow(unused_variables)]
    pub fn get_railway_routing_edge(&self) -> Ptr<RailEdge<ROEdge, ROVehicle>> {
        todo!("port ROEdge::getRailwayRoutingEdge")
    }

    /// C++: `bool ROEdge::hasStoredEffort() const` [const]
    /// whether effort data was loaded for this edge
    // bool hasStoredEffort() const {
    //         return myUsingETimeLine;
    //     }
    #[allow(unused_variables)]
    pub fn has_stored_effort(&self) -> bool {
        todo!("port ROEdge::hasStoredEffort")
    }

    /// C++: `bool ROEdge::initPriorityFactor(double)` [static]
    /// initialize priority factor range
    // bool
    // ROEdge::initPriorityFactor(double priorityFactor) {
    //     myPriorityFactor = priorityFactor;
    //     double maxEdgePriority = -std::numeric_limits<double>::max();
    //     for (ROEdge* edge : myEdges) {
    //         maxEdgePriority = MAX2(maxEdgePriority, (double)edge->getPriority());
    //         myMinEdgePriority = MIN2(myMinEdgePriority, (double)edge->getPriority());
    //     }
    //     myEdgePriorityRange = maxEdgePriority - myMinEdgePriority;
    //     if (myEdgePriorityRange == 0) {
    //         WRITE_WARNING(TL("Option weights.priority-factor does not take effect because all edges have the same priority."));
    //         myPriorityFactor = 0;
    //         return false;
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn init_priority_factor(priorityFactor: f64) -> bool {
        todo!("port ROEdge::initPriorityFactor")
    }

    /// C++: `bool ROEdge::getStoredEffort(double, double &) const` [const, protected]
    /// Retrieves the stored effort
    // bool
    // ROEdge::getStoredEffort(double time, double& ret) const {
    //     if (myUsingETimeLine) {
    //         if (!myEfforts.describesTime(time)) {
    //             if (!myHaveEWarned) {
    //                 WRITE_WARNINGF(TL("No interval matches passed time=% in edge '%'.\n Using edge's length / edge's speed."), time, myID);
    //                 myHaveEWarned = true;
    //             }
    //             return false;
    //         }
    //         if (myInterpolate) {
    //             const double inTT = myTravelTimes.getValue(time);
    //             const double ratio = (myEfforts.getSplitTime(time, time + inTT) - time) / inTT;
    //             if (ratio >= 0.) {
    //                 ret = ratio * myEfforts.getValue(time) + (1. - ratio) * myEfforts.getValue(time + inTT);
    //                 return true;
    //             }
    //         }
    //         ret = myEfforts.getValue(time);
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn get_stored_effort_2(&self, time: f64, ret: &mut f64) -> bool {
        todo!("port ROEdge::getStoredEffort")
    }

    /// C++: `void ROEdge::ROEdge(const ROEdge &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(src: &ROEdge) -> Self {
        todo!("port ROEdge::ROEdge")
    }

    /// C++: `ROEdge & ROEdge::operator=(const ROEdge &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROEdge) -> &mut ROEdge {
        todo!("port ROEdge::operator=")
    }

}

