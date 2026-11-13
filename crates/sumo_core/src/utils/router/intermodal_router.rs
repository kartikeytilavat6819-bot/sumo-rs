//! Port of utils/router/IntermodalRouter.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `void (*)(IntermodalRouter<E, L, N, V> &) IntermodalRouter::CreateNetCallback`
pub type IntermodalRouter_CreateNetCallback = Unmapped /* void (*)(IntermodalRouter<E, L, N, V> &) */;

/// C++ typedef `IntermodalNetwork<E, L, N, V> IntermodalRouter::Network`
pub type IntermodalRouter_Network<E, L, N, V> = IntermodalNetwork<E, L, N, V>;

/// C++ typedef `IntermodalEdge<E, L, N, V> IntermodalRouter::_IntermodalEdge`
pub type IntermodalRouter__IntermodalEdge = Unmapped /* IntermodalEdge<E, L, N, V> */;

/// C++ typedef `IntermodalTrip<E, N, V> IntermodalRouter::_IntermodalTrip`
pub type IntermodalRouter__IntermodalTrip = Unmapped /* IntermodalTrip<E, N, V> */;

/// C++ typedef `AStarRouter<_IntermodalEdge, _IntermodalTrip, _MapMatcher> IntermodalRouter::_InternalAStar`
pub type IntermodalRouter__InternalAStar = AStarRouter<AccessEdge__IntermodalEdge, PedestrianRouter__IntermodalTrip, IntermodalRouter__MapMatcher>;

/// C++ typedef `DijkstraRouter<_IntermodalEdge, _IntermodalTrip> IntermodalRouter::_InternalDijkstra`
pub type IntermodalRouter__InternalDijkstra = DijkstraRouter<AccessEdge__IntermodalEdge, PedestrianRouter__IntermodalTrip>;

/// C++ typedef `SUMOAbstractRouter<_IntermodalEdge, _IntermodalTrip> IntermodalRouter::_InternalRouter`
pub type IntermodalRouter__InternalRouter = SUMOAbstractRouter<AccessEdge__IntermodalEdge, PedestrianRouter__IntermodalTrip>;

/// C++ typedef `MapMatcher<E, L, N> IntermodalRouter::_MapMatcher`
pub type IntermodalRouter__MapMatcher = Unmapped /* MapMatcher<E, L, N> */;

/// C++ typedef `typename SUMOAbstractRouter<E, SUMOVehicle>::Prohibitions IntermodalRouter::_Prohibitions`
pub type IntermodalRouter__Prohibitions = SUMOAbstractRouter_Prohibitions;

/// The router for pedestrians (on a bidirectional network of sidewalks and crossings)
/// C++ class `IntermodalRouter` (utils/router/IntermodalRouter.h)
#[repr(C)]
pub struct IntermodalRouter<E, L, N, V> {
    /// C++ base class `SUMOAbstractRouter<E, IntermodalTrip<E, N, V>>`
    pub base: SUMOAbstractRouter<E, IntermodalTrip<E, N, V>>,
    pub myAmClone: bool,
    pub myInternalRouter: Ptr<PedestrianRouter__InternalRouter>,
    pub myIntermodalNet: Ptr<IntermodalRouter_Network<E, L, N, V>>,
    pub myCallback: IntermodalRouter_CreateNetCallback,
    pub myCarWalkTransfer: i32,
    pub myTaxiWait: f64,
    pub myRoutingAlgorithm: String,
    pub myRoutingMode: i32,
    pub myExternalEffort: Ptr<EffortCalculator>,
    _p: std::marker::PhantomData<(E, L, N, V,)>,
}

impl<E, L, N, V> std::ops::Deref for IntermodalRouter<E, L, N, V> {
    type Target = SUMOAbstractRouter<E, IntermodalTrip<E, N, V>>;
    fn deref(&self) -> &SUMOAbstractRouter<E, IntermodalTrip<E, N, V>> {
        &self.base
    }
}
impl<E, L, N, V> std::ops::DerefMut for IntermodalRouter<E, L, N, V> {
    fn deref_mut(&mut self) -> &mut SUMOAbstractRouter<E, IntermodalTrip<E, N, V>> {
        &mut self.base
    }
}

impl<E, L, N, V> IntermodalRouter<E, L, N, V> {
    /// C++: `void IntermodalRouter::IntermodalRouter<E, L, N, V>(CreateNetCallback, const int, double, const std::string &, const int, EffortCalculator *)`
    /// Constructor
    // IntermodalRouter(CreateNetCallback callback, const int carWalkTransfer, double taxiWait, const std::string& routingAlgorithm,
    //                      const int routingMode = 0, EffortCalculator* calc = nullptr) :
    //         SUMOAbstractRouter<E, _IntermodalTrip>("IntermodalRouter", true, nullptr, nullptr, false, false),
    //         myAmClone(false), myInternalRouter(nullptr), myIntermodalNet(nullptr),
    //         myCallback(callback), myCarWalkTransfer(carWalkTransfer), myTaxiWait(taxiWait),
    //         myRoutingAlgorithm(routingAlgorithm),
    //         myRoutingMode(routingMode), myExternalEffort(calc) {
    //     }
    #[allow(unused_variables)]
    pub fn new(callback: IntermodalRouter_CreateNetCallback, carWalkTransfer: i32, taxiWait: f64, routingAlgorithm: &str, routingMode: i32, calc: Ptr<EffortCalculator>) -> Self {
        Self {
            base: SUMOAbstractRouter::new("IntermodalRouter", true, (), (), false, false),
            myAmClone: false,
            myInternalRouter: Ptr::null(),
            myIntermodalNet: Ptr::null(),
            myCallback: callback,
            myCarWalkTransfer: carWalkTransfer,
            myTaxiWait: taxiWait,
            myRoutingAlgorithm: routingAlgorithm.to_string(),
            myRoutingMode: routingMode,
            myExternalEffort: calc,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void IntermodalRouter::~IntermodalRouter<E, L, N, V>()` [virtual]
    /// Destructor
    // virtual ~IntermodalRouter() {
    //         delete myInternalRouter;
    //         if (!myAmClone) {
    //             delete myIntermodalNet;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if !self.myInternalRouter.is_null() {
            drop(unsafe { Box::from_raw(self.myInternalRouter.as_raw()) });
        }
        if !self.myAmClone && !self.myIntermodalNet.is_null() {
            drop(unsafe { Box::from_raw(self.myIntermodalNet.as_raw()) });
        }
    }

    /// C++: `SUMOAbstractRouter<E, _IntermodalTrip> * IntermodalRouter::clone()`
    // SUMOAbstractRouter<E, _IntermodalTrip>* clone() {
    //         createNet();
    //         return new IntermodalRouter<E, L, N, V>(myIntermodalNet, myCarWalkTransfer, myTaxiWait, myRoutingAlgorithm, myRoutingMode, myExternalEffort);
    //     }
    #[allow(unused_variables)]
    pub fn clone(&mut self) -> Ptr<SUMOAbstractRouter<E, PedestrianRouter__IntermodalTrip>> {
        todo!("port IntermodalRouter::clone")
    }

    /// C++: `int IntermodalRouter::getCarWalkTransfer() const` [const]
    // int getCarWalkTransfer() const {
    //         return myCarWalkTransfer;
    //     }
    #[allow(unused_variables)]
    pub fn get_car_walk_transfer(&self) -> i32 {
        self.myCarWalkTransfer
    }

    /// C++: `bool IntermodalRouter::compute(const E *, const E *, const double, const std::string &, const double, const std::string &, const double, const V *const, const SUMOVTypeParameter &, const SVCPermissions, const SUMOTime, std::vector<TripItem> &, const double)`
    /// Builds the route between the given edges using the minimum effort at the given time The definition of the effort depends on the wished routing scheme
    // bool compute(const E* from, const E* to,
    //                  const double departPos, const std::string& originStopID,
    //                  const double arrivalPos, const std::string& stopID,
    //                  const double speed, const V* const vehicle,
    //                  const SUMOVTypeParameter& pars,
    //                  const SVCPermissions modeSet, const SUMOTime msTime,
    //                  std::vector<TripItem>& into, const double externalFactor = 0.) {
    //         createNet();
    //         _IntermodalTrip trip(from, to, departPos, arrivalPos, speed, msTime, nullptr, pars, vehicle, modeSet, myExternalEffort, externalFactor);
    //         std::vector<const _IntermodalEdge*> intoEdges;
    //         //std::cout << "compute from=" << from->getID() << " to=" << to->getID() << " dPos=" << departPos << " aPos=" << arrivalPos << " stopID=" << stopID << " speed=" << speed << " veh=" << Named::getIDSecure(vehicle) << " modeSet=" << modeSet << " t=" << msTime << " iFrom=" << myIntermodalNet->getDepartEdge(from, trip.departPos)->getID() << " iTo=" << (stopID != "" ? myIntermodalNet->getStopEdge(stopID) : myIntermodalNet->getArrivalEdge(to, trip.arrivalPos))->getID() << "\n";
    //         const _IntermodalEdge* iFrom = originStopID != "" ? myIntermodalNet->getStopEdge(originStopID) : myIntermodalNet->getDepartEdge(from, trip.departPos);
    //         const _IntermodalEdge* iTo = stopID != "" ? myIntermodalNet->getStopEdge(stopID) : myIntermodalNet->getArrivalEdge(to, trip.arrivalPos);
    //         const bool success = myInternalRouter->compute(iFrom, iTo, &trip, msTime, intoEdges, true);
    //         if (success) {
    //             std::string lastLine = "";
    //             const _IntermodalEdge* lastLineEdge = nullptr;
    //             double lastLineTime = STEPS2TIME(msTime);
    //             double time = STEPS2TIME(msTime);
    //             double effort = 0.;
    //             double length = 0.;
    //             const _IntermodalEdge* prev = nullptr;
    //             for (const _IntermodalEdge* iEdge : intoEdges) {
    //                 bool addedEdge = false;
    //                 if (iEdge->includeInRoute(false)) {
    //                     if (iEdge->getLine() == "!stop") {
    //                         if (into.size() > 0) {
    //                             // previous stage ends at stop
    //                             into.back().destStop = iEdge->getID();
    //                             if (myExternalEffort != nullptr) {
    //                                 into.back().description = myExternalEffort->output(iEdge->getNumericalID());
    //                             }
    //                             if (lastLine == "!ped") {
    //                                 lastLine = ""; // a stop always starts a new trip item
    //                             }
    //                         } else {
    //                             // trip starts at stop
    //                             lastLine = "";
    //                             into.push_back(TripItem("!stop"));
    //                             into.back().destStop = iEdge->getID();
    //                         }
    //                     } else {
    //                         if (iEdge->getLine() != lastLine || loopedLineTransfer(lastLineEdge, iEdge, lastLineTime, time)) {
    //                             lastLine = iEdge->getLine();
    //                             lastLineEdge = iEdge;
    //                             lastLineTime = time;
    //                             if (lastLine == "!car") {
    //                                 into.push_back(TripItem(vehicle->getID()));
    //                                 into.back().vType = vehicle->getParameter().vtypeid;
    //                             } else if (lastLine == "!ped") {
    //                                 into.push_back(TripItem());
    //                             } else {
    //                                 into.push_back(TripItem(lastLine));
    //                                 into.back().depart = iEdge->getIntended(time, into.back().intended);
    //                             }
    //                             into.back().departPos = iEdge->getStartPos();
    //                         }
    //                         if (into.back().edges.empty() || into.back().edges.back() != iEdge->getEdge()) {
    //                             into.back().edges.push_back(iEdge->getEdge());
    //                             into.back().arrivalPos = iEdge->getEndPos();
    //                             addedEdge = true;
    //                         }
    //                     }
    //                 }
    //                 const double prevTime = time;
    //                 const double prevEffort = effort;
    //                 const double prevLength = length;
    //                 myInternalRouter->updateViaCost(prev, iEdge, &trip, time, effort, length);
    //                 // correct intermodal length:
    //                 length += iEdge->getPartialLength(&trip) - iEdge->getLength();
    //                 prev = iEdge;
    //                 if (!into.empty()) {
    //                     into.back().traveltime += time - prevTime;
    //                     into.back().cost += effort - prevEffort;
    //                     into.back().length += length - prevLength;
    //                     if (into.back().depart < 0) {
    //                         into.back().depart = prevTime;
    //                     }
    //                     if (addedEdge) {
    //                         into.back().exitTimes.push_back(time);
    //                     }
    //                 }
    //             }
    //         } else {
    //             const std::string oType = originStopID != "" ? "stop" : "edge";
    //             const std::string oID = originStopID != "" ? originStopID : from->getID();
    //             const std::string dType = stopID != "" ? "stop" : "edge";
    //             const std::string dID = stopID != "" ? stopID : to->getID();
    //             const std::string vClass = vehicle == nullptr ? "" : (" with vClass " + getVehicleClassNames(vehicle->getVClass()));
    //             this->myErrorMsgHandler->informf(TL("No connection between % '%' and % '%' found%."), oType, oID, dType, dID, vClass);
    //         }
    //         if (into.size() > 0) {
    //             into.back().arrivalPos = arrivalPos;
    //         }
    // #ifdef IntermodalRouter_DEBUG_ROUTES
    //         double time = STEPS2TIME(msTime);
    //         for (const _IntermodalEdge* iEdge : intoEdges) {
    //             const double edgeEffort = myInternalRouter->getEffort(iEdge, &trip, time);
    //             time += edgeEffort;
    //             std::cout << iEdge->getID() << "(" << iEdge->getLine() << "): " << edgeEffort << " l=" << iEdge->getLength() << " pL=" << iEdge->getPartialLength(&trip) << "\n";
    //         }
    //         std::cout << TIME2STEPS(msTime) << " trip from " << from->getID() << " to " << (to != nullptr ? to->getID() : stopID)
    //                   << " departPos=" << trip.departPos
    //                   << " arrivalPos=" << trip.arrivalPos
    //                   << " modes=" << getVehicleClassNames(modeSet)
    //                   << " vehType=" << (vehicle == nullptr ? "NULL" : vehicle->getVTypeParameter().id)
    //                   << " edges=" << toString(intoEdges)
    // //                  << " resultEdges=" << toString(into)
    //                   << " time=" << time
    //                   << "\n";
    // #endif
    //         return success;
    //     }
    #[allow(unused_variables)]
    pub fn compute(&mut self, from: Ptr<E>, to: Ptr<E>, departPos: f64, originStopID: &str, arrivalPos: f64, stopID: &str, speed: f64, vehicle: Ptr<V>, pars: &SUMOVTypeParameter, modeSet: SVCPermissions, msTime: SUMOTime, into: &mut Vec<IntermodalRouter_TripItem>, externalFactor: f64) -> bool {
        todo!("port IntermodalRouter::compute")
    }

    /// C++: `bool IntermodalRouter::compute(const E *, const E *, const _IntermodalTrip *const, SUMOTime, std::vector<const E *> &, bool)`
    /// Builds the route between the given edges using the minimum effort at the given time The definition of the effort depends on the wished routing scheme
    // bool compute(const E*, const E*, const _IntermodalTrip* const,
    //                  SUMOTime, std::vector<const E*>&, bool) {
    //         throw ProcessError(TL("Do not use this method"));
    //     }
    #[allow(unused_variables)]
    pub fn compute_2(&mut self, arg0: Ptr<E>, arg1: Ptr<E>, arg2: Ptr<PedestrianRouter__IntermodalTrip>, arg3: SUMOTime, arg4: &mut Vec<Ptr<E>>, arg5: bool) -> bool {
        panic!("{}", ProcessError::new("Do not use this method"))
    }

    /// C++: `void IntermodalRouter::setBulkMode(const bool)`
    // inline void setBulkMode(const bool mode) {
    //         SUMOAbstractRouter<E, _IntermodalTrip>::setBulkMode(mode);
    //         if (myInternalRouter != nullptr) {
    //             myInternalRouter->setBulkMode(mode);
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn set_bulk_mode(&mut self, mode: bool) {
        self.base.set_bulk_mode(mode);
        if !self.myInternalRouter.is_null() {
            self.myInternalRouter.m().set_bulk_mode(mode);
        }
    }

    /// C++: `void IntermodalRouter::prohibit(const _Prohibitions &)`
    // void prohibit(const _Prohibitions& toProhibit) {
    //         createNet();
    //         typename _InternalRouter::Prohibitions toProhibitPE;
    //         for (auto item : toProhibit) {
    //             toProhibitPE[myIntermodalNet->getBothDirections(item.first).first] = item.second;
    //             toProhibitPE[myIntermodalNet->getBothDirections(item.first).second] = item.second;
    //             toProhibitPE[myIntermodalNet->getCarEdge(item.first)] = item.second;
    //         }
    //         myInternalRouter->prohibit(toProhibitPE);
    //     }
    #[allow(unused_variables)]
    pub fn prohibit(&mut self, toProhibit: &IntermodalRouter__Prohibitions) {
        todo!("port IntermodalRouter::prohibit")
    }

    /// C++: `void IntermodalRouter::writeNetwork(OutputDevice &)`
    // void writeNetwork(OutputDevice& dev) {
    //         createNet();
    //         for (_IntermodalEdge* e : myIntermodalNet->getAllEdges()) {
    //             dev.openTag(SUMO_TAG_EDGE);
    //             dev.writeAttr(SUMO_ATTR_ID, e->getID());
    //             dev.writeAttr(SUMO_ATTR_LINE, e->getLine());
    //             dev.writeAttr(SUMO_ATTR_LENGTH, e->getLength());
    //             dev.writeAttr("successors", toString(e->getSuccessors(SVC_IGNORING)));
    //             dev.closeTag();
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn write_network(&mut self, dev: &mut OutputDevice) {
        todo!("port IntermodalRouter::writeNetwork")
    }

    /// C++: `void IntermodalRouter::writeWeights(OutputDevice &)`
    // void writeWeights(OutputDevice& dev) {
    //         createNet();
    //         SUMOVTypeParameter dummyVT(DEFAULT_PEDTYPE_ID, SVC_PEDESTRIAN);
    //         _IntermodalTrip trip(nullptr, nullptr, 0., 0., DEFAULT_PEDESTRIAN_SPEED, 0, nullptr,
    //                              dummyVT, nullptr, SVC_PASSENGER | SVC_BICYCLE | SVC_BUS);
    //         for (_IntermodalEdge* e : myIntermodalNet->getAllEdges()) {
    //             dev.openTag(SUMO_TAG_EDGE);
    //             dev.writeAttr(SUMO_ATTR_ID, e->getID());
    //             dev.writeAttr("traveltime", e->getTravelTime(&trip, 0.));
    //             dev.writeAttr("effort", e->getEffort(&trip, 0.));
    //             dev.closeTag();
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn write_weights(&mut self, dev: &mut OutputDevice) {
        todo!("port IntermodalRouter::writeWeights")
    }

    /// C++: `Network * IntermodalRouter::getNetwork() const` [const]
    // Network* getNetwork() const {
    //         return myIntermodalNet;
    //     }
    #[allow(unused_variables)]
    pub fn get_network(&self) -> Ptr<IntermodalRouter_Network<E, L, N, V>> {
        self.myIntermodalNet
    }

    /// C++: `EffortCalculator * IntermodalRouter::getExternalEffort() const` [const]
    // EffortCalculator* getExternalEffort() const {
    //         return myExternalEffort;
    //     }
    #[allow(unused_variables)]
    pub fn get_external_effort(&self) -> Ptr<EffortCalculator> {
        self.myExternalEffort
    }

    /// C++: `void IntermodalRouter::IntermodalRouter<E, L, N, V>(Network *, const int, double, const std::string &, const int, EffortCalculator *)` [private]
    // IntermodalRouter(Network* net, const int carWalkTransfer, double taxiWait, const std::string& routingAlgorithm,
    //                      const int routingMode, EffortCalculator* calc) :
    //         SUMOAbstractRouter<E, _IntermodalTrip>("IntermodalRouterClone", true, nullptr, nullptr, false, false),
    //         myAmClone(true), myInternalRouter(nullptr), myIntermodalNet(net),
    //         myCarWalkTransfer(carWalkTransfer),
    //         myTaxiWait(taxiWait),
    //         myRoutingAlgorithm(routingAlgorithm), myRoutingMode(routingMode), myExternalEffort(calc) {
    //         createNet();
    //     }
    #[allow(unused_variables)]
    pub fn new_2(net: Ptr<IntermodalRouter_Network<E, L, N, V>>, carWalkTransfer: i32, taxiWait: f64, routingAlgorithm: &str, routingMode: i32, calc: Ptr<EffortCalculator>) -> Self {
        todo!("port IntermodalRouter::IntermodalRouter<E, L, N, V>")
    }

    /// C++: `double IntermodalRouter::getCombined(const _IntermodalEdge *const, const _IntermodalTrip *const, double)` [static, private]
    // static inline double getCombined(const _IntermodalEdge* const edge, const _IntermodalTrip* const trip, double time) {
    //         return edge->getTravelTime(trip, time) + trip->externalFactor * trip->calc->getEffort(edge->getNumericalID());
    //     }
    #[allow(unused_variables)]
    pub fn get_combined(edge: Ptr<AccessEdge__IntermodalEdge>, trip: Ptr<PedestrianRouter__IntermodalTrip>, time: f64) -> f64 {
        todo!("port IntermodalRouter::getCombined")
    }

    /// C++: `void IntermodalRouter::createNet()` [private]
    // inline void createNet() {
    //         if (myIntermodalNet == nullptr) {
    //             myIntermodalNet = new Network(E::getAllEdges(), false, myCarWalkTransfer);
    //             myIntermodalNet->addCarEdges(E::getAllEdges(), myTaxiWait);
    //             myCallback(*this);
    //         }
    //         if (myInternalRouter == nullptr) {
    //             switch (myRoutingMode) {
    //                 case 0:
    //                     if (myRoutingAlgorithm == "astar") {
    //                         myInternalRouter = new _InternalAStar(myIntermodalNet->getAllEdges(), true,
    //                                                               gWeightsRandomFactor > 1 ? &_IntermodalEdge::getTravelTimeStaticRandomized : &_IntermodalEdge::getTravelTimeStatic, nullptr, true);
    //                     } else {
    //                         myInternalRouter = new _InternalDijkstra(myIntermodalNet->getAllEdges(), true,
    //                                 gWeightsRandomFactor > 1 ? &_IntermodalEdge::getTravelTimeStaticRandomized : &_IntermodalEdge::getTravelTimeStatic, nullptr, false, nullptr, true);
    //                     }
    //                     break;
    //                 case 1:
    //                     myInternalRouter = new _InternalDijkstra(myIntermodalNet->getAllEdges(), true, &_IntermodalEdge::getTravelTimeAggregated, nullptr, false, nullptr, true);
    //                     break;
    //                 case 2:
    //                     myInternalRouter = new _InternalDijkstra(myIntermodalNet->getAllEdges(), true, &_IntermodalEdge::getEffortStatic, &_IntermodalEdge::getTravelTimeStatic, false, nullptr, true);
    //                     break;
    //                 case 3:
    //                     if (myExternalEffort != nullptr) {
    //                         std::vector<std::string> edgeLines;
    //                         for (const auto e : myIntermodalNet->getAllEdges()) {
    //                             edgeLines.push_back(e->getLine());
    //                         }
    //                         myExternalEffort->init(edgeLines);
    //                     }
    //                     myInternalRouter = new _InternalDijkstra(myIntermodalNet->getAllEdges(), true, &getCombined, &_IntermodalEdge::getTravelTimeStatic, false, myExternalEffort, true);
    //                     break;
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn create_net(&mut self) {
        todo!("port IntermodalRouter::createNet")
    }

    /// C++: `bool IntermodalRouter::loopedLineTransfer(const _IntermodalEdge *, const _IntermodalEdge *, double, double)` [private]
    // bool loopedLineTransfer(const _IntermodalEdge* prev, const _IntermodalEdge* cur, double prevTime, double time) {
    //         assert(prev != nullptr);
    //         if (myIntermodalNet->isLooped(cur->getLine())) {
    //             // check if the last two edges are served by different vehicles
    //             std::string intended1;
    //             std::string intended2;
    //             prev->getIntended(prevTime, intended1);
    //             cur->getIntended(time, intended2);
    //             return intended1 != intended2;
    //         }
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn looped_line_transfer(&mut self, prev: Ptr<AccessEdge__IntermodalEdge>, cur: Ptr<AccessEdge__IntermodalEdge>, prevTime: f64, time: f64) -> bool {
        todo!("port IntermodalRouter::loopedLineTransfer")
    }

    /// C++: `IntermodalRouter<E, L, N, V> & IntermodalRouter::operator=(const IntermodalRouter<E, L, N, V> &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &IntermodalRouter<E, L, N, V>) -> &mut IntermodalRouter<E, L, N, V> {
        panic!("IntermodalRouter::operator= is declared but not defined")
    }

}

/// C++ class `IntermodalRouter::TripItem` (utils/router/IntermodalRouter.h)
pub struct IntermodalRouter_TripItem {
    pub line: String,
    pub vType: String,
    pub destStop: String,
    pub intended: String,
    pub depart: f64,
    pub edges: Vec<Ptr<Unmapped /* E */>>,
    pub traveltime: f64,
    pub cost: f64,
    pub length: f64,
    pub departPos: f64,
    pub arrivalPos: f64,
    pub description: String,
    pub exitTimes: Vec<f64>,
}

impl IntermodalRouter_TripItem {
    /// C++: `void TripItem::TripItem(const std::string &)`
    // TripItem(const std::string& _line = "") :
    //             line(_line), intended(_line) {}
    #[allow(unused_variables)]
    pub fn new(_line: &str) -> Self {
        Self {
            line: _line.to_string(),
            vType: String::new(),
            destStop: String::new(),
            intended: _line.to_string(),
            depart: -1.,
            edges: Vec::new(),
            traveltime: 0.,
            cost: 0.,
            length: 0.,
            departPos: utils::common::std_defs::INVALID_DOUBLE,
            arrivalPos: utils::common::std_defs::INVALID_DOUBLE,
            description: String::new(),
            exitTimes: Vec::new(),
        }
    }

}

