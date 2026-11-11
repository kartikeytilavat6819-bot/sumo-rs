//! Port of router/RORoute.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A complete router's route
/// C++ class `RORoute` (router/RORoute.h)
pub struct RORoute {
    /// C++ base class `Named`
    pub base: Named,
    /// The costs of the route
    pub myCosts: f64,
    /// The probability the driver will take this route with
    pub myProbability: f64,
    /// The edges the route consists of
    pub myRoute: ConstROEdgeVector,
    /// The color of the route
    pub myColor: Ptr<RGBColor>,
    /// List of the stops on the parsed route
    pub myStops: StopParVector,
}

impl std::ops::Deref for RORoute {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for RORoute {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl RORoute {
    /// C++: `void RORoute::RORoute(const std::string &, double, double, const ConstROEdgeVector &, const RGBColor *const, const StopParVector &)`
    /// Constructor
    // RORoute::RORoute(const std::string& id, double costs, double prop,
    //                  const ConstROEdgeVector& route,
    //                  const RGBColor* const color,
    //                  const StopParVector& stops)
    //     : Named(StringUtils::convertUmlaute(id)), myCosts(costs),
    //       myProbability(prop), myRoute(route), myColor(color), myStops(stops) {}
    #[allow(unused_variables)]
    pub fn new(id: &str, costs: f64, prob: f64, route: &ConstROEdgeVector, color: Ptr<RGBColor>, stops: &StopParVector) -> Self {
        todo!("port RORoute::RORoute")
    }

    /// C++: `void RORoute::RORoute(const std::string &, const ConstROEdgeVector &)`
    /// Constructor
    // RORoute::RORoute(const std::string& id, const ConstROEdgeVector& route)
    //     : Named(StringUtils::convertUmlaute(id)), myCosts(0.0),
    //       myProbability(0.0), myRoute(route), myColor(nullptr), myStops() {}
    #[allow(unused_variables)]
    pub fn new_2(id: &str, route: &ConstROEdgeVector) -> Self {
        todo!("port RORoute::RORoute")
    }

    /// C++: `void RORoute::RORoute(const RORoute &)`
    /// Copy constructor
    // RORoute::RORoute(const RORoute& src)
    //     : Named(src.myID), myCosts(src.myCosts),
    //       myProbability(src.myProbability), myRoute(src.myRoute), myColor(nullptr) {
    //     if (src.myColor != nullptr) {
    //         myColor = new RGBColor(*src.myColor);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new_3(src: &RORoute) -> Self {
        todo!("port RORoute::RORoute")
    }

    /// C++: `void RORoute::~RORoute() noexcept` [virtual]
    /// Destructor
    // RORoute::~RORoute() {
    //     delete myColor;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port RORoute::~RORoute")
    }

    /// C++: `const ROEdge * RORoute::getFirst() const` [const]
    /// Returns the first edge in the route
    // const ROEdge* getFirst() const {
    //         return myRoute[0];
    //     }
    #[allow(unused_variables)]
    pub fn get_first(&self) -> Ptr<ROEdge> {
        todo!("port RORoute::getFirst")
    }

    /// C++: `const ROEdge * RORoute::getLast() const` [const]
    /// Returns the last edge in the route
    // const ROEdge* getLast() const {
    //         return myRoute.back();
    //     }
    #[allow(unused_variables)]
    pub fn get_last(&self) -> Ptr<ROEdge> {
        todo!("port RORoute::getLast")
    }

    /// C++: `double RORoute::getCosts() const` [const]
    /// Returns the costs of the route
    // double getCosts() const {
    //         return myCosts;
    //     }
    #[allow(unused_variables)]
    pub fn get_costs(&self) -> f64 {
        todo!("port RORoute::getCosts")
    }

    /// C++: `double RORoute::getProbability() const` [const]
    /// Returns the probability the driver will take this route with
    // double getProbability() const {
    //         return myProbability;
    //     }
    #[allow(unused_variables)]
    pub fn get_probability(&self) -> f64 {
        todo!("port RORoute::getProbability")
    }

    /// C++: `void RORoute::setCosts(double)`
    /// Sets the costs of the route
    // void
    // RORoute::setCosts(double costs) {
    //     myCosts = costs;
    // }
    #[allow(unused_variables)]
    pub fn set_costs(&mut self, costs: f64) {
        todo!("port RORoute::setCosts")
    }

    /// C++: `void RORoute::setProbability(double)`
    /// Sets the probability of the route
    // void
    // RORoute::setProbability(double prob) {
    //     myProbability = prob;
    // }
    #[allow(unused_variables)]
    pub fn set_probability(&mut self, prob: f64) {
        todo!("port RORoute::setProbability")
    }

    /// C++: `int RORoute::size() const` [const]
    /// Returns the number of edges in this route
    // int size() const {
    //         return (int) myRoute.size();
    //     }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        todo!("port RORoute::size")
    }

    /// C++: `const ConstROEdgeVector & RORoute::getEdgeVector() const` [const]
    /// Returns the list of edges this route consists of
    // const ConstROEdgeVector& getEdgeVector() const {
    //         return myRoute;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge_vector(&self) -> &ConstROEdgeVector {
        todo!("port RORoute::getEdgeVector")
    }

    /// C++: `const RGBColor * RORoute::getColor() const` [const]
    /// Returns this route's color
    // const RGBColor* getColor() const {
    //         return myColor;
    //     }
    #[allow(unused_variables)]
    pub fn get_color(&self) -> Ptr<RGBColor> {
        todo!("port RORoute::getColor")
    }

    /// C++: `void RORoute::recheckForLoops(const ConstROEdgeVector &)`
    /// Checks whether this route contains loops and removes such
    // void
    // RORoute::recheckForLoops(const ConstROEdgeVector& mandatory) {
    //     ROHelper::recheckForLoops(myRoute, mandatory);
    // }
    #[allow(unused_variables)]
    pub fn recheck_for_loops(&mut self, mandatory: &ConstROEdgeVector) {
        todo!("port RORoute::recheckForLoops")
    }

    /// C++: `bool RORoute::isValid(const ROVehicle &, bool, MsgHandler *) const` [const]
    /// check whether the route is connected for the given vehicle
    // bool
    // RORoute::isValid(const ROVehicle& veh, bool ignoreErrors, MsgHandler* mh) const {
    //     if (mh == nullptr) {
    //         mh = ignoreErrors ? MsgHandler::getWarningInstance() : MsgHandler::getErrorInstance();
    //     }
    //     for (ConstROEdgeVector::const_iterator i = myRoute.begin() + 1; i != myRoute.end(); ++i) {
    //         const ROEdge* prev = *(i - 1);
    //         const ROEdge* cur = *i;
    //         if (!prev->isConnectedTo(*cur, veh.getVClass())) {
    //             mh->informf("Edge '%' not connected to edge '%' for vehicle '%'.", prev->getID(), cur->getID(), veh.getID());
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn is_valid(&self, veh: &ROVehicle, ignoreErrors: bool, mh: Ptr<MsgHandler>) -> bool {
        todo!("port RORoute::isValid")
    }

    /// C++: `bool RORoute::isPermitted(const ROVehicle *, MsgHandler *) const` [const]
    /// check whether the route is permitted for the given vehicle
    // bool
    // RORoute::isPermitted(const ROVehicle* veh, MsgHandler* mh) const {
    //     const bool hasRestrictions = RONet::getInstance()->hasParamRestrictions();
    //     const bool hasPermissions = RONet::getInstance()->hasPermissions();
    //     if (hasRestrictions || hasPermissions) {
    //         for (const ROEdge* e : myRoute) {
    //             if ((hasPermissions && e->prohibits(veh)) || (hasRestrictions && e->restricts(veh))) {
    //                 mh->informf("Vehicle '%' is not permitted on Edge '%'", veh->getID(), e->getID());
    //                 return false;
    //             }
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn is_permitted(&self, veh: Ptr<ROVehicle>, mh: Ptr<MsgHandler>) -> bool {
        todo!("port RORoute::isPermitted")
    }

    /// C++: `OutputDevice & RORoute::writeXMLDefinition(OutputDevice &, const ROVehicle *const, const bool, const bool, const bool, const bool, const std::string &) const` [const]
    // OutputDevice&
    // RORoute::writeXMLDefinition(OutputDevice& dev, const ROVehicle* const veh,
    //                             const bool withCosts,
    //                             const bool asAlternatives,
    //                             const bool withExitTimes,
    //                             const bool withLength,
    //                             const std::string& id) const {
    //     dev.openTag(SUMO_TAG_ROUTE);
    //     if (id != "") {
    //         dev.writeAttr(SUMO_ATTR_ID, id);
    //     }
    //     if (withCosts) {
    //         dev.writeAttr(SUMO_ATTR_COST, myCosts);
    //     }
    //     if (asAlternatives) {
    //         dev.setPrecision(8);
    //         dev.writeAttr(SUMO_ATTR_PROB, myProbability);
    //         dev.setPrecision();
    //     }
    //     if (myColor != nullptr) {
    //         dev.writeAttr(SUMO_ATTR_COLOR, *myColor);
    //     }
    //     dev.writeAttr(SUMO_ATTR_EDGES, getNormalEdges());
    //     if (withExitTimes) {
    //         std::vector<double> exitTimes;
    //         double time = STEPS2TIME(veh->getDepartureTime());
    //         for (const ROEdge* const roe : myRoute) {
    //             time += roe->getTravelTime(veh, time);
    //             if (!roe->isInternal() && !roe->isTazConnector()) {
    //                 exitTimes.push_back(time);
    //             }
    //         }
    //         dev.writeAttr("exitTimes", exitTimes);
    //     }
    //     if (withLength) {
    //         double length = 0.;
    //         for (const ROEdge* const roe : myRoute) {
    //             length += roe->getLength();
    //         }
    //         dev.writeAttr("routeLength", length);
    //     }
    //     dev.closeTag();
    //     return dev;
    // }
    #[allow(unused_variables)]
    pub fn write_xml_definition(&self, dev: &mut OutputDevice, veh: Ptr<ROVehicle>, withCosts: bool, asAlternatives: bool, withExitTimes: bool, withLength: bool, id: &str) -> &mut OutputDevice {
        todo!("port RORoute::writeXMLDefinition")
    }

    /// C++: `void RORoute::addProbability(double)`
    /// add additional vehicles/probability
    // void
    // RORoute::addProbability(double prob) {
    //     myProbability += prob;
    // }
    #[allow(unused_variables)]
    pub fn add_probability(&mut self, prob: f64) {
        todo!("port RORoute::addProbability")
    }

    /// C++: `const StopParVector & RORoute::getStops() const` [const]
    /// Returns the list of stops this route contains
    // const StopParVector& getStops() const {
    //         return myStops;
    //     }
    #[allow(unused_variables)]
    pub fn get_stops(&self) -> &StopParVector {
        todo!("port RORoute::getStops")
    }

    /// C++: `void RORoute::addStopOffset(const SUMOTime)`
    /// Adapts the until and arrival time of all stops by the given offset
    // void
    // RORoute::addStopOffset(const SUMOTime offset) {
    //     for (StopParVector::iterator stop = myStops.begin(); stop != myStops.end(); ++stop) {
    //         if (stop->until >= 0) {
    //             stop->until += offset;
    //         }
    //         if (stop->arrival >= 0) {
    //             stop->arrival += offset;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_stop_offset(&mut self, offset: SUMOTime) {
        todo!("port RORoute::addStopOffset")
    }

    /// C++: `ConstROEdgeVector RORoute::getNormalEdges() const` [const]
    /// return edges that shall be written in the route definition
    // ConstROEdgeVector
    // RORoute::getNormalEdges() const {
    //     ConstROEdgeVector tempRoute;
    //     for (const ROEdge* roe : myRoute) {
    //         if (!roe->isInternal() && !roe->isTazConnector()) {
    //             tempRoute.push_back(roe);
    //         }
    //     }
    //     return tempRoute;
    // }
    #[allow(unused_variables)]
    pub fn get_normal_edges(&self) -> ConstROEdgeVector {
        todo!("port RORoute::getNormalEdges")
    }

    /// C++: `RORoute & RORoute::operator=(const RORoute &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RORoute) -> &mut RORoute {
        todo!("port RORoute::operator=")
    }

}

