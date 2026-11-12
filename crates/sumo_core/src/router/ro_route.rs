//! Port of router/RORoute.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A complete router's route
/// C++ class `RORoute` (router/RORoute.h)
#[repr(C)]
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
        // shortcut: C++ copies the stops into myStops; SUMOVehicleParameter::Stop has no Clone yet (utils/vehicle/sumo_vehicle_parameter.rs, outside this batch), add #[derive(Clone)] there and copy the stops
        panic!("RORoute::RORoute: copying SUMOVehicleParameter::Stop is not ported yet")
    }

    /// C++: `void RORoute::RORoute(const std::string &, const ConstROEdgeVector &)`
    /// Constructor
    // RORoute::RORoute(const std::string& id, const ConstROEdgeVector& route)
    //     : Named(StringUtils::convertUmlaute(id)), myCosts(0.0),
    //       myProbability(0.0), myRoute(route), myColor(nullptr), myStops() {}
    #[allow(unused_variables)]
    pub fn new_2(id: &str, route: &ConstROEdgeVector) -> Self {
        RORoute {
            base: Named::new(&utils::common::string_utils::convert_umlaute(id)),
            myCosts: 0.0,
            myProbability: 0.0,
            myRoute: route.clone(),
            myColor: Ptr::null(),
            myStops: Vec::new(),
        }
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
        RORoute {
            base: Named::new(src.get_id()),
            myCosts: src.myCosts,
            myProbability: src.myProbability,
            myRoute: src.myRoute.clone(),
            myColor: if src.myColor.is_null() { Ptr::null() } else { Ptr::from_raw(Box::into_raw(Box::new(*src.myColor))) },
            myStops: Vec::new(),
        }
    }

    /// C++: `void RORoute::~RORoute() noexcept` [virtual]
    /// Destructor
    // RORoute::~RORoute() {
    //     delete myColor;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        if !self.myColor.is_null() {
            // delete myColor (allocated with Box in the copy constructor or owned from the caller)
            drop(unsafe { Box::from_raw(self.myColor.as_raw()) });
        }
    }

    /// C++: `const ROEdge * RORoute::getFirst() const` [const]
    /// Returns the first edge in the route
    // const ROEdge* getFirst() const {
    //         return myRoute[0];
    //     }
    #[allow(unused_variables)]
    pub fn get_first(&self) -> Ptr<ROEdge> {
        self.myRoute[0]
    }

    /// C++: `const ROEdge * RORoute::getLast() const` [const]
    /// Returns the last edge in the route
    // const ROEdge* getLast() const {
    //         return myRoute.back();
    //     }
    #[allow(unused_variables)]
    pub fn get_last(&self) -> Ptr<ROEdge> {
        *self.myRoute.last().unwrap()
    }

    /// C++: `double RORoute::getCosts() const` [const]
    /// Returns the costs of the route
    // double getCosts() const {
    //         return myCosts;
    //     }
    #[allow(unused_variables)]
    pub fn get_costs(&self) -> f64 {
        self.myCosts
    }

    /// C++: `double RORoute::getProbability() const` [const]
    /// Returns the probability the driver will take this route with
    // double getProbability() const {
    //         return myProbability;
    //     }
    #[allow(unused_variables)]
    pub fn get_probability(&self) -> f64 {
        self.myProbability
    }

    /// C++: `void RORoute::setCosts(double)`
    /// Sets the costs of the route
    // void
    // RORoute::setCosts(double costs) {
    //     myCosts = costs;
    // }
    #[allow(unused_variables)]
    pub fn set_costs(&mut self, costs: f64) {
        self.myCosts = costs;
    }

    /// C++: `void RORoute::setProbability(double)`
    /// Sets the probability of the route
    // void
    // RORoute::setProbability(double prob) {
    //     myProbability = prob;
    // }
    #[allow(unused_variables)]
    pub fn set_probability(&mut self, prob: f64) {
        self.myProbability = prob;
    }

    /// C++: `int RORoute::size() const` [const]
    /// Returns the number of edges in this route
    // int size() const {
    //         return (int) myRoute.size();
    //     }
    #[allow(unused_variables)]
    pub fn size(&self) -> i32 {
        self.myRoute.len() as i32
    }

    /// C++: `const ConstROEdgeVector & RORoute::getEdgeVector() const` [const]
    /// Returns the list of edges this route consists of
    // const ConstROEdgeVector& getEdgeVector() const {
    //         return myRoute;
    //     }
    #[allow(unused_variables)]
    pub fn get_edge_vector(&self) -> &ConstROEdgeVector {
        &self.myRoute
    }

    /// C++: `const RGBColor * RORoute::getColor() const` [const]
    /// Returns this route's color
    // const RGBColor* getColor() const {
    //         return myColor;
    //     }
    #[allow(unused_variables)]
    pub fn get_color(&self) -> Ptr<RGBColor> {
        self.myColor
    }

    /// C++: `void RORoute::recheckForLoops(const ConstROEdgeVector &)`
    /// Checks whether this route contains loops and removes such
    // void
    // RORoute::recheckForLoops(const ConstROEdgeVector& mandatory) {
    //     ROHelper::recheckForLoops(myRoute, mandatory);
    // }
    #[allow(unused_variables)]
    pub fn recheck_for_loops(&mut self, mandatory: &ConstROEdgeVector) {
        crate::router::ro_helper::recheck_for_loops(&mut self.myRoute, mandatory);
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
        for i in 1..self.myRoute.len() {
            let prev = self.myRoute[i - 1];
            let cur = self.myRoute[i];
            if !prev.is_connected_to(&cur, veh.get_v_class(), false) {
                let args: [&dyn utils::common::to_string::ToStringPrec; 3] = [&prev.get_id(), &cur.get_id(), &veh.get_id()];
                if mh.is_null() {
                    let t = if ignoreErrors { utils::common::msg_handler::MsgType::Warning } else { utils::common::msg_handler::MsgType::Error };
                    utils::common::msg_handler::with_instance(t, |h| h.informf("Edge '%' not connected to edge '%' for vehicle '%'.", &args));
                } else {
                    mh.m().informf("Edge '%' not connected to edge '%' for vehicle '%'.", &args);
                }
                return false;
            }
        }
        true
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
        let hasRestrictions = RONet::get_instance().has_param_restrictions();
        let hasPermissions = RONet::get_instance().has_permissions();
        if hasRestrictions || hasPermissions {
            for e in &self.myRoute {
                if (hasPermissions && e.prohibits(veh, false)) || (hasRestrictions && e.restricts(veh)) {
                    let args: [&dyn utils::common::to_string::ToStringPrec; 2] = [&veh.get_id(), &e.get_id()];
                    mh.m().informf("Vehicle '%' is not permitted on Edge '%'", &args);
                    return false;
                }
            }
        }
        true
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
    pub fn write_xml_definition<'a>(&self, dev: &'a mut OutputDevice, veh: Ptr<ROVehicle>, withCosts: bool, asAlternatives: bool, withExitTimes: bool, withLength: bool, id: &str) -> &'a mut OutputDevice {
        dev.open_tag_id(SUMO_TAG_ROUTE);
        if !id.is_empty() {
            dev.write_attr(SUMO_ATTR_ID, id);
        }
        if withCosts {
            dev.write_attr(SUMO_ATTR_COST, &self.myCosts);
        }
        if asAlternatives {
            dev.set_precision(8);
            dev.write_attr(SUMO_ATTR_PROB, &self.myProbability);
            dev.set_precision(utils::common::std_defs::g_precision());
        }
        if !self.myColor.is_null() {
            dev.write_attr(SUMO_ATTR_COLOR, &(*self.myColor).to_string());
        }
        let normalEdges: Vec<String> = self.get_normal_edges().iter().map(|e| e.get_id().to_string()).collect();
        dev.write_attr(SUMO_ATTR_EDGES, &normalEdges.join(" "));
        if withExitTimes {
            let mut exitTimes: Vec<f64> = Vec::new();
            let mut time = utils::common::time::steps2time(veh.get_departure_time());
            for roe in &self.myRoute {
                time += roe.get_travel_time(veh, time);
                if !roe.is_internal() && !roe.is_taz_connector() {
                    exitTimes.push(time);
                }
            }
            dev.write_attr("exitTimes", &exitTimes);
        }
        if withLength {
            let mut length = 0.;
            for roe in &self.myRoute {
                length += roe.get_length();
            }
            dev.write_attr("routeLength", &length);
        }
        dev.close_tag("");
        dev
    }

    /// C++: `void RORoute::addProbability(double)`
    /// add additional vehicles/probability
    // void
    // RORoute::addProbability(double prob) {
    //     myProbability += prob;
    // }
    #[allow(unused_variables)]
    pub fn add_probability(&mut self, prob: f64) {
        self.myProbability += prob;
    }

    /// C++: `const StopParVector & RORoute::getStops() const` [const]
    /// Returns the list of stops this route contains
    // const StopParVector& getStops() const {
    //         return myStops;
    //     }
    #[allow(unused_variables)]
    pub fn get_stops(&self) -> &StopParVector {
        &self.myStops
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
        for stop in self.myStops.iter_mut() {
            if stop.until >= 0 {
                stop.until += offset;
            }
            if stop.arrival >= 0 {
                stop.arrival += offset;
            }
        }
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
        let mut tempRoute: ConstROEdgeVector = Vec::new();
        for roe in &self.myRoute {
            if !roe.is_internal() && !roe.is_taz_connector() {
                tempRoute.push(*roe);
            }
        }
        tempRoute
    }

    /// C++: `RORoute & RORoute::operator=(const RORoute &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RORoute) -> &mut RORoute {
        // C++ assignment operator is private and never defined: assignment is disallowed
        panic!("RORoute is not assignable (private operator= in C++)")
    }

}

