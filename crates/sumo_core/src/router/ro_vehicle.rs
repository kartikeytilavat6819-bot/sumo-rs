//! Port of router/ROVehicle.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// information for mandatory edges
/// C++ class `ROVehicle::Mandatory` (router/ROVehicle.h)
pub struct ROVehicle_Mandatory {
    pub edge: Ptr<ROEdge>,
    pub pos: f64,
    pub isJump: bool,
}

impl ROVehicle_Mandatory {
    /// C++: `void Mandatory::Mandatory(const ROEdge *, double, SUMOTime)`
    // Mandatory(const ROEdge* e, double p, SUMOTime jump = -1):
    //             edge(e),
    //             pos(p),
    //             isJump(jump >= 0) {}
    #[allow(unused_variables)]
    pub fn new(e: Ptr<ROEdge>, p: f64, jump: SUMOTime) -> Self {
        ROVehicle_Mandatory { edge: e, pos: p, isJump: jump >= 0 }
    }

}

/// A vehicle as used by router
/// C++ class `ROVehicle` (router/ROVehicle.h)
#[repr(C)]
pub struct ROVehicle {
    /// C++ base class `RORoutable`
    pub base: RORoutable,
    /// The route the vehicle takes
    pub myRoute: Ptr<RORouteDef>,
    /// Whether this vehicle has any jumps defined
    pub myJumpTime: SUMOTime,
}

impl std::ops::Deref for ROVehicle {
    type Target = RORoutable;
    fn deref(&self) -> &RORoutable {
        &self.base
    }
}
impl std::ops::DerefMut for ROVehicle {
    fn deref_mut(&mut self) -> &mut RORoutable {
        &mut self.base
    }
}

// C++ static members of ROVehicle (port as module-level statics, see AGENTS.md):
//   static std::map<ConstROEdgeVector, std::string> ROVehicle::mySavedRoutes;
/// C++ std::map keyed by edge vector: BTreeMap orders keys like std::map (lexicographic by pointer)
static MY_SAVED_ROUTES: std::sync::Mutex<BTreeMap<ConstROEdgeVector, String>> = std::sync::Mutex::new(BTreeMap::new());
/// C++ `#define INVALID_STOP_POS -1` (ROVehicle.cpp)
const INVALID_STOP_POS: f64 = -1.;
/// C++ SUMOVehicleParameter.h stop index markers (same values as the other ports)
const STOP_INDEX_END: i32 = -1;
const STOP_INDEX_FIT: i32 = -2;

impl ROVehicle {
    /// C++: `void ROVehicle::ROVehicle(const SUMOVehicleParameter &, RORouteDef *, const SUMOVTypeParameter *, const RONet *, MsgHandler *)`
    /// Constructor
    // ROVehicle::ROVehicle(const SUMOVehicleParameter& pars,
    //                      RORouteDef* route, const SUMOVTypeParameter* type,
    //                      const RONet* net, MsgHandler* errorHandler):
    //     RORoutable(pars, type),
    //     myRoute(route),
    //     myJumpTime(-1) {
    //     getParameter().stops.clear();
    //     if (route != nullptr && route->getFirstRoute() != nullptr) {
    //         for (StopParVector::const_iterator s = route->getFirstRoute()->getStops().begin(); s != route->getFirstRoute()->getStops().end(); ++s) {
    //             addStop(*s, net, errorHandler);
    //         }
    //     }
    //     for (StopParVector::const_iterator s = pars.stops.begin(); s != pars.stops.end(); ++s) {
    //         addStop(*s, net, errorHandler);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(pars: &SUMOVehicleParameter, route: Ptr<RORouteDef>, type_: Ptr<SUMOVTypeParameter>, net: Ptr<RONet>, errorHandler: Ptr<MsgHandler>) -> Self {
        let mut v = ROVehicle {
            base: RORoutable::new(pars, type_),
            myRoute: route,
            myJumpTime: -1,
        };
        v.get_parameter_mutable().stops.clear();
        if !route.is_null() {
            // shortcut: getFirstRoute() returns Rc (never null), so the C++ nullptr check is not repeated
            let first = route.get_first_route().expect("first route");
            for s in first.get_stops().iter() {
                v.add_stop(s, net, errorHandler);
            }
        }
        for s in pars.stops.iter() {
            v.add_stop(s, net, errorHandler);
        }
        v
    }

    /// C++: `void ROVehicle::~ROVehicle() noexcept` [virtual]
    /// Destructor
    // ROVehicle::~ROVehicle() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `RORouteDef * ROVehicle::getRouteDefinition() const` [const]
    /// Returns the definition of the route the vehicle takes
    // inline RORouteDef* getRouteDefinition() const {
    //         return myRoute;
    //     }
    #[allow(unused_variables)]
    pub fn get_route_definition(&self) -> Ptr<RORouteDef> {
        self.myRoute
    }

    /// C++: `const ROEdge * ROVehicle::getDepartEdge() const` [const, virtual]
    /// Returns the first edge the vehicle takes
    // const ROEdge*
    // ROVehicle:: getDepartEdge() const {
    //     return myRoute->getFirstRoute()->getFirst();
    // }
    #[allow(unused_variables)]
    pub fn get_depart_edge(&self) -> Ptr<ROEdge> {
        self.myRoute.get_first_route().expect("first route").get_first()
    }

    /// C++: `void ROVehicle::computeRoute(const RORouterProvider &, const bool, MsgHandler *)` [virtual]
    // void
    // ROVehicle::computeRoute(const RORouterProvider& provider,
    //                         const bool removeLoops, MsgHandler* errorHandler) {
    //     SUMOAbstractRouter<ROEdge, ROVehicle>& router = provider.getVehicleRouter(getVClass());
    //     RORouteDef* const routeDef = getRouteDefinition();
    //     // check if the route definition is valid
    //     if (routeDef == nullptr) {
    //         errorHandler->informf(TL("The vehicle '%' has no valid route."), getID());
    //         myRoutingSuccess = false;
    //         return;
    //     }
    //     routeDef->validateAlternatives(this, errorHandler);
    //     if (routeDef->getFirstRoute() == nullptr) {
    //         // everything is invalid and no new routes will be computed
    //         myRoutingSuccess = false;
    //         return;
    //     }
    //     std::shared_ptr<RORoute> current = routeDef->buildCurrentRoute(router, getDepartureTime(), *this);
    //     if (current == nullptr || current->size() == 0) {
    //         if (current == nullptr || !routeDef->discardSilent()) {
    //             errorHandler->informf(TL("The vehicle '%' has no valid route."), getID());
    //         }
    //         myRoutingSuccess = false;
    //         return;
    //     }
    //     // check whether we have to evaluate the route for not containing loops
    //     if (removeLoops) {
    //         const ROEdge* requiredStart = (getParameter().departPosProcedure == DepartPosDefinition::GIVEN
    //                                        || getParameter().departLaneProcedure == DepartLaneDefinition::GIVEN ? current->getEdgeVector().front() : 0);
    //         const ROEdge* requiredEnd = (getParameter().arrivalPosProcedure == ArrivalPosDefinition::GIVEN
    //                                      || getParameter().arrivalLaneProcedure == ArrivalLaneDefinition::GIVEN ? current->getEdgeVector().back() : 0);
    //         ConstROEdgeVector mandatory;
    //         for (auto m : getMandatoryEdges(requiredStart, requiredEnd)) {
    //             mandatory.push_back(m.edge);
    //         }
    //         current->recheckForLoops(mandatory);
    //         // check whether the route is still valid
    //         if (current->size() == 0) {
    //             errorHandler->informf(TL("The vehicle '%' has no valid route (after removing loops)."), getID());
    //             myRoutingSuccess = false;
    //             return;
    //         }
    //     }
    //     if (RONet::getInstance()->getMaxTraveltime() > 0) {
    //         double costs = router.recomputeCosts(current->getEdgeVector(), this, getDepartureTime());
    //         if (costs > RONet::getInstance()->getMaxTraveltime()) {
    //             errorHandler->informf(TL("The vehicle '%' has no valid route (traveltime % exceeds max-traveltime)."), getID(), time2string(TIME2STEPS(costs)));
    //             myRoutingSuccess = false;
    //             return;
    //         }
    //     }
    //     // add built route
    //     std::shared_ptr<RORoute> replaced = routeDef->addAlternative(router, this, current, getDepartureTime(), errorHandler);
    //     if (replaced != nullptr) {
    //         if (getParameter().departEdge > 0) {
    //             updateIndex(replaced, current, getParameterMutable().departEdge);
    //         }
    //         if (getParameter().arrivalEdge >= 0) {
    //             updateIndex(replaced, current, getParameterMutable().arrivalEdge);
    //         }
    //     }
    //     myRoutingSuccess = true;
    // }
    #[allow(unused_variables)]
    pub fn compute_route(&mut self, provider: &RORouterProvider, removeLoops: bool, errorHandler: Ptr<MsgHandler>) {
        let router = provider.get_vehicle_router(self.get_v_class());
        let routeDef = self.get_route_definition();
        // check if the route definition is valid
        if routeDef.is_null() {
            let args: [&dyn ToStringPrec; 1] = [&self.get_id().to_string()];
            errorHandler.m().informf("The vehicle '%' has no valid route.", &args);
            self.myRoutingSuccess = false;
            return;
        }
        routeDef.m().validate_alternatives(Ptr::from_mut(self), errorHandler);
        // shortcut: getFirstRoute() returns Rc (never null), so the C++ "everything is invalid" nullptr check is not repeated
        let mut current = routeDef.build_current_route(router, self.get_departure_time(), self).expect("current route");
        // shortcut: buildCurrentRoute returns Rc (never null); only the empty-route case is checked
        if current.size() == 0 {
            if !routeDef.discard_silent() {
                let args: [&dyn ToStringPrec; 1] = [&self.get_id().to_string()];
                errorHandler.m().informf("The vehicle '%' has no valid route.", &args);
            }
            self.myRoutingSuccess = false;
            return;
        }
        // check whether we have to evaluate the route for not containing loops
        if removeLoops {
            let requiredStart = if self.get_parameter().departPosProcedure == DepartPosDefinition::GIVEN
                || self.get_parameter().departLaneProcedure == DepartLaneDefinition::GIVEN
            {
                current.get_edge_vector()[0]
            } else {
                Ptr::null()
            };
            let requiredEnd = if self.get_parameter().arrivalPosProcedure == ArrivalPosDefinition::GIVEN
                || self.get_parameter().arrivalLaneProcedure == ArrivalLaneDefinition::GIVEN
            {
                *current.get_edge_vector().last().unwrap()
            } else {
                Ptr::null()
            };
            let mut mandatory: ConstROEdgeVector = Vec::new();
            for m in self.get_mandatory_edges(requiredStart, requiredEnd) {
                mandatory.push(m.edge);
            }
            Rc::get_mut(&mut current).unwrap().recheck_for_loops(&mandatory);
            // check whether the route is still valid
            if current.size() == 0 {
                let args: [&dyn ToStringPrec; 1] = [&self.get_id().to_string()];
                errorHandler.m().informf("The vehicle '%' has no valid route (after removing loops).", &args);
                self.myRoutingSuccess = false;
                return;
            }
        }
        if RONet::get_instance().get_max_traveltime() > 0. {
            let costs = router.recompute_costs(current.get_edge_vector(), Ptr::from_mut(self), self.get_departure_time(), Ptr::null());
            if costs > RONet::get_instance().get_max_traveltime() {
                let args: [&dyn ToStringPrec; 2] = [&self.get_id().to_string(), &time2string(time2steps(costs))];
                errorHandler.m().informf("The vehicle '%' has no valid route (traveltime % exceeds max-traveltime).", &args);
                self.myRoutingSuccess = false;
                return;
            }
        }
        // add built route
        let replaced = routeDef.m().add_alternative(router, Ptr::from_mut(self), current.clone(), self.get_departure_time(), errorHandler);
        // shortcut: addAlternative returns Rc (never null), so the C++ "replaced != nullptr" check is not repeated
        if self.get_parameter().departEdge > 0 {
            let mut idx = self.get_parameter().departEdge;
            self.update_index(replaced.clone(), current.clone(), &mut idx);
            self.get_parameter_mutable().departEdge = idx;
        }
        if self.get_parameter().arrivalEdge >= 0 {
            let mut idx = self.get_parameter().arrivalEdge;
            self.update_index(replaced.clone(), current.clone(), &mut idx);
            self.get_parameter_mutable().arrivalEdge = idx;
        }
        self.myRoutingSuccess = true;
    }

    /// C++: `SUMOTime ROVehicle::getDepartureTime() const` [const]
    /// Returns the time the vehicle starts at, 0 for triggered vehicles
    // inline SUMOTime getDepartureTime() const {
    //         return MAX2(SUMOTime(0), getParameter().depart);
    //     }
    #[allow(unused_variables)]
    pub fn get_departure_time(&self) -> SUMOTime {
        std::cmp::max(0, self.get_parameter().depart)
    }

    /// C++: `std::vector<Mandatory> ROVehicle::getMandatoryEdges(const ROEdge *, const ROEdge *) const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_mandatory_edges(&self, requiredStart: Ptr<ROEdge>, requiredEnd: Ptr<ROEdge>) -> Vec<ROVehicle_Mandatory> {
        // C++ (ROVehicle.cpp): const RONet* net = RONet::getInstance(); std::vector<Mandatory> mandatory;
        let net = RONet::get_instance();
        let mut mandatory: Vec<ROVehicle_Mandatory> = Vec::new();
        if !requiredStart.is_null() {
            let mut departPos = 0.;
            if self.get_parameter().departPosProcedure == DepartPosDefinition::GIVEN {
                departPos = SUMOVehicleParameter::interpret_edge_pos(self.get_parameter().departPos, requiredStart.get_length(), SUMO_ATTR_DEPARTPOS, &format!("vehicle '{}'", self.get_id()), false);
            }
            mandatory.push(ROVehicle_Mandatory::new(requiredStart, departPos, -1));
        }
        if !self.get_parameter().via.is_empty() {
            // via takes precedence over stop edges
            for via in self.get_parameter().via.iter() {
                mandatory.push(ROVehicle_Mandatory::new(net.get_edge(via), INVALID_STOP_POS, -1));
            }
        } else {
            for stop in self.get_parameter().stops.iter() {
                let e = net.get_edge(&stop.edge);
                if e.is_internal() {
                    // the edges before and after the internal edge are mandatory
                    let before = e.get_normal_before();
                    let after = e.get_normal_after();
                    mandatory.push(ROVehicle_Mandatory::new(before, INVALID_STOP_POS, -1));
                    mandatory.push(ROVehicle_Mandatory::new(after, INVALID_STOP_POS, stop.jump));
                } else {
                    let endPos = SUMOVehicleParameter::interpret_edge_pos(stop.endPos, e.get_length(), SUMO_ATTR_ENDPOS, &format!("stop of vehicle '{}' on edge '{}'", self.get_id(), e.get_id()), false);
                    mandatory.push(ROVehicle_Mandatory::new(e, endPos, stop.jump));
                }
            }
        }
        if !requiredEnd.is_null() {
            let mut arrivalPos = INVALID_STOP_POS;
            if self.get_parameter().arrivalPosProcedure == ArrivalPosDefinition::GIVEN {
                arrivalPos = SUMOVehicleParameter::interpret_edge_pos(self.get_parameter().arrivalPos, requiredEnd.get_length(), SUMO_ATTR_ARRIVALPOS, &format!("vehicle '{}'", self.get_id()), false);
            }
            mandatory.push(ROVehicle_Mandatory::new(requiredEnd, arrivalPos, -1));
        }
        mandatory
    }

    /// C++: `const SUMOVTypeParameter & ROVehicle::getVTypeParameter() const` [const]
    /// Returns the vehicle's type definition
    // inline const SUMOVTypeParameter& getVTypeParameter() const  {
    //         return *getType();
    //     }
    #[allow(unused_variables)]
    pub fn get_v_type_parameter(&self) -> &SUMOVTypeParameter {
        self.get_type().get().unwrap()
    }

    /// C++: `double ROVehicle::getLength() const` [const]
    /// Returns the vehicle's length
    // inline double getLength() const {
    //         return getType()->length;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.get_type().length
    }

    /// C++: `bool ROVehicle::hasJumps() const` [const]
    // inline bool hasJumps() const {
    //         return myJumpTime >= 0;
    //     }
    #[allow(unused_variables)]
    pub fn has_jumps(&self) -> bool {
        self.myJumpTime >= 0
    }

    /// C++: `SUMOTime ROVehicle::getJumpTime() const` [const]
    // inline SUMOTime getJumpTime() const {
    //         return myJumpTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_jump_time(&self) -> SUMOTime {
        self.myJumpTime
    }

    /// C++: `void ROVehicle::saveAsXML(OutputDevice &, OutputDevice *const, bool, OptionsCont &, int) const` [const, virtual]
    /// Saves the complete vehicle description.
    // void
    // ROVehicle::saveAsXML(OutputDevice& os, OutputDevice* const typeos, bool asAlternatives, OptionsCont& options, int cloneIndex) const {
    //     if (typeos != nullptr && getType() != nullptr && !getType()->saved) {
    //         getType()->write(*typeos);
    //         getType()->saved = true;
    //     }
    //     if (getType() != nullptr && !getType()->saved) {
    //         getType()->write(os);
    //         getType()->saved = asAlternatives;
    //     }
    //
    //     const bool writeTrip = options.exists("write-trips") && options.getBool("write-trips");
    //     const bool writeGeoTrip = writeTrip && options.getBool("write-trips.geo");
    //     const bool writeJunctions = writeTrip && options.getBool("write-trips.junctions");
    //     const bool writeNamedRoute = !asAlternatives && options.getBool("named-routes");
    //     const bool writeCosts = options.exists("write-costs") && options.getBool("write-costs");
    //     const bool writeExit = options.exists("exit-times") && options.getBool("exit-times");
    //     const bool writeLength = options.exists("route-length") && options.getBool("route-length");
    //     const bool writeFlow = options.exists("keep-flows") && options.getBool("keep-flows") && isPartOfFlow();
    //
    //     std::string routeID;
    //     if (writeNamedRoute) {
    //         ConstROEdgeVector edges = myRoute->getUsedRoute()->getNormalEdges();
    //         auto it = mySavedRoutes.find(edges);
    //         if (it == mySavedRoutes.end()) {
    //             routeID = "r" + toString(mySavedRoutes.size());
    //             myRoute->getUsedRoute()->writeXMLDefinition(os, this, writeCosts, false, writeExit,
    //                     writeLength, routeID);
    //             mySavedRoutes[edges] = routeID;
    //         } else {
    //             routeID = it->second;
    //         }
    //     }
    //     const SumoXMLTag tag = writeFlow ? SUMO_TAG_FLOW : (writeTrip ? SUMO_TAG_TRIP : SUMO_TAG_VEHICLE);
    //     // write the vehicle (new style, with included routes)
    //     if (cloneIndex == 0) {
    //         getParameter().write(os, options, tag);
    //     } else {
    //         SUMOVehicleParameter p = getParameter();
    //         // @note id collisions may occur if scale-suffic occurs in other vehicle ids
    //         p.id += options.getString("scale-suffix") + toString(cloneIndex);
    //         p.write(os, options, tag);
    //     }
    //     // save the route
    //     if (writeTrip) {
    //         const ConstROEdgeVector edges = myRoute->getFirstRoute()->getEdgeVector();
    //         const ROEdge* from = nullptr;
    //         const ROEdge* to = nullptr;
    //         if (edges.size() > 0) {
    //             if (edges.front()->isTazConnector()) {
    //                 if (edges.size() > 1) {
    //                     from = edges[1];
    //                     if (from->isTazConnector() && writeJunctions && edges.front()->getSuccessors().size() > 0) {
    //                         // routing was skipped
    //                         from = edges.front()->getSuccessors(getVClass()).front();
    //                     }
    //                 }
    //             } else {
    //                 from = edges[0];
    //             }
    //             if (edges.back()->isTazConnector()) {
    //                 if (edges.size() > 1) {
    //                     to = edges[edges.size() - 2];
    //                     if (to->isTazConnector() && writeJunctions && edges.back()->getPredecessors().size() > 0) {
    //                         // routing was skipped
    //                         to = edges.back()->getPredecessors().front();
    //                     }
    //                 }
    //             } else {
    //                 to = edges[edges.size() - 1];
    //             }
    //         }
    //         if (from != nullptr) {
    //             if (writeGeoTrip) {
    //                 Position fromPos = from->getLanes()[0]->getShape().positionAtOffset2D(0);
    //                 if (GeoConvHelper::getFinal().usingGeoProjection()) {
    //                     os.setPrecision(gPrecisionGeo);
    //                     GeoConvHelper::getFinal().cartesian2geo(fromPos);
    //                     os.writeAttr(SUMO_ATTR_FROMLONLAT, fromPos);
    //                     os.setPrecision(gPrecision);
    //                 } else {
    //                     os.writeAttr(SUMO_ATTR_FROMXY, fromPos);
    //                 }
    //             } else if (writeJunctions) {
    //                 os.writeAttr(SUMO_ATTR_FROM_JUNCTION, from->getFromJunction()->getID());
    //             } else {
    //                 os.writeAttr(SUMO_ATTR_FROM, from->getID());
    //             }
    //         }
    //         if (to != nullptr) {
    //             if (writeGeoTrip) {
    //                 Position toPos = to->getLanes()[0]->getShape().positionAtOffset2D(to->getLanes()[0]->getShape().length2d());
    //                 if (GeoConvHelper::getFinal().usingGeoProjection()) {
    //                     os.setPrecision(gPrecisionGeo);
    //                     GeoConvHelper::getFinal().cartesian2geo(toPos);
    //                     os.writeAttr(SUMO_ATTR_TOLONLAT, toPos);
    //                     os.setPrecision(gPrecision);
    //                 } else {
    //                     os.writeAttr(SUMO_ATTR_TOXY, toPos);
    //                 }
    //             } else if (writeJunctions) {
    //                 os.writeAttr(SUMO_ATTR_TO_JUNCTION, to->getToJunction()->getID());
    //             } else {
    //                 os.writeAttr(SUMO_ATTR_TO, to->getID());
    //             }
    //         }
    //         if (getParameter().via.size() > 0) {
    //             std::vector<std::string> viaOut;
    //             SumoXMLAttr viaAttr = (writeGeoTrip
    //                                    ? (GeoConvHelper::getFinal().usingGeoProjection() ? SUMO_ATTR_VIALONLAT : SUMO_ATTR_VIAXY)
    //                                    : (writeJunctions ? SUMO_ATTR_VIAJUNCTIONS : SUMO_ATTR_VIA));
    //             for (const std::string& viaID : getParameter().via) {
    //                 const ROEdge* viaEdge = RONet::getInstance()->getEdge(viaID);
    //                 if (viaEdge->isTazConnector()) {
    //                     if (viaEdge->getPredecessors().size() == 0) {
    //                         continue;
    //                     }
    //                     // XXX used edge that was used in route
    //                     viaEdge = viaEdge->getPredecessors().front();
    //                 }
    //                 assert(viaEdge != nullptr);
    //                 if (writeGeoTrip) {
    //                     Position viaPos = viaEdge->getLanes()[0]->getShape().positionAtOffset2D(viaEdge->getLanes()[0]->getShape().length2d() / 2);
    //                     if (GeoConvHelper::getFinal().usingGeoProjection()) {
    //                         GeoConvHelper::getFinal().cartesian2geo(viaPos);
    //                         viaOut.push_back(toString(viaPos, gPrecisionGeo));
    //                     } else {
    //                         viaOut.push_back(toString(viaPos, gPrecision));
    //                     }
    //                 } else if (writeJunctions) {
    //                     viaOut.push_back(viaEdge->getToJunction()->getID());
    //                 } else {
    //                     viaOut.push_back(viaEdge->getID());
    //                 }
    //             }
    //             os.writeAttr(viaAttr, viaOut);
    //         }
    //     } else if (writeNamedRoute) {
    //         os.writeAttr(SUMO_ATTR_ROUTE, routeID);
    //     } else {
    //         myRoute->writeXMLDefinition(os, this, asAlternatives, writeExit, writeCosts, writeLength);
    //     }
    //     for (StopParVector::const_iterator stop = getParameter().stops.begin(); stop != getParameter().stops.end(); ++stop) {
    //         stop->write(os);
    //     }
    //     getParameter().writeParams(os);
    //     os.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_as_xml(&self, os: &mut OutputDevice, typeos: Ptr<OutputDevice>, asAlternatives: bool, options: &mut OptionsCont, cloneIndex: i32) {
        use utils::common::std_defs::{g_precision, g_precision_geo};
        use utils::common::to_string::to_string_prec;
        if !typeos.is_null() && !self.get_type().is_null() && !self.get_type().saved {
            self.get_type().write(typeos.m());
            self.get_type().m().saved = true;
        }
        if !self.get_type().is_null() && !self.get_type().saved {
            self.get_type().write(os);
            self.get_type().m().saved = asAlternatives;
        }

        let writeTrip = options.exists("write-trips") && options.get_bool("write-trips").unwrap();
        let writeGeoTrip = writeTrip && options.get_bool("write-trips.geo").unwrap();
        let writeJunctions = writeTrip && options.get_bool("write-trips.junctions").unwrap();
        let writeNamedRoute = !asAlternatives && options.get_bool("named-routes").unwrap();
        let writeCosts = options.exists("write-costs") && options.get_bool("write-costs").unwrap();
        let writeExit = options.exists("exit-times") && options.get_bool("exit-times").unwrap();
        let writeLength = options.exists("route-length") && options.get_bool("route-length").unwrap();
        let writeFlow = options.exists("keep-flows") && options.get_bool("keep-flows").unwrap() && self.is_part_of_flow();

        let mut routeID = String::new();
        if writeNamedRoute {
            let edges: ConstROEdgeVector = self.myRoute.get_used_route().get_normal_edges();
            let mut saved = MY_SAVED_ROUTES.lock().unwrap();
            let existing = saved.get(&edges).cloned();
            match existing {
                None => {
                    routeID = format!("r{}", saved.len());
                    self.myRoute.get_used_route().write_xml_definition(os, Ptr::new(self), writeCosts, false, writeExit, writeLength, &routeID);
                    saved.insert(edges, routeID.clone());
                }
                Some(id) => routeID = id,
            }
        }
        let tag = if writeFlow {
            SUMO_TAG_FLOW
        } else if writeTrip {
            SUMO_TAG_TRIP
        } else {
            SUMO_TAG_VEHICLE
        };
        // write the vehicle (new style, with included routes)
        if cloneIndex == 0 {
            self.get_parameter().write(os, options, tag, "");
        } else {
            let mut p = self.get_parameter().clone();
            // @note id collisions may occur if scale-suffic occurs in other vehicle ids
            p.id += &(options.get_string("scale-suffix").unwrap() + &cloneIndex.to_string());
            p.write(os, options, tag, "");
        }
        // save the route
        if writeTrip {
            let edges: ConstROEdgeVector = self.myRoute.get_first_route().expect("first route").get_edge_vector().clone();
            let mut from: Ptr<ROEdge> = Ptr::null();
            let mut to: Ptr<ROEdge> = Ptr::null();
            if edges.len() > 0 {
                if edges[0].is_taz_connector() {
                    if edges.len() > 1 {
                        from = edges[1];
                        if from.is_taz_connector() && writeJunctions && edges[0].get_successors(SVC_IGNORING).len() > 0 {
                            // routing was skipped
                            from = edges[0].get_successors(self.get_v_class())[0];
                        }
                    }
                } else {
                    from = edges[0];
                }
                let last = edges[edges.len() - 1];
                if last.is_taz_connector() {
                    if edges.len() > 1 {
                        to = edges[edges.len() - 2];
                        if to.is_taz_connector() && writeJunctions && last.get_predecessors().len() > 0 {
                            // routing was skipped
                            to = last.get_predecessors()[0];
                        }
                    }
                } else {
                    to = edges[edges.len() - 1];
                }
            }
            if !from.is_null() {
                if writeGeoTrip {
                    let mut fromPos = from.get_lanes()[0].get_shape().position_at_offset2d(0., 0., false);
                    if GeoConvHelper::with_final(|g| g.using_geo_projection()) {
                        os.set_precision(g_precision_geo());
                        GeoConvHelper::with_final(|g| g.cartesian2geo(&mut fromPos));
                        os.write_attr(SUMO_ATTR_FROMLONLAT, &fromPos);
                        os.set_precision(g_precision());
                    } else {
                        os.write_attr(SUMO_ATTR_FROMXY, &fromPos);
                    }
                } else if writeJunctions {
                    os.write_attr(SUMO_ATTR_FROM_JUNCTION, from.get_from_junction().get_id());
                } else {
                    os.write_attr(SUMO_ATTR_FROM, from.get_id());
                }
            }
            if !to.is_null() {
                if writeGeoTrip {
                    let shape = to.get_lanes()[0].get_shape();
                    let mut toPos = shape.position_at_offset2d(shape.length2d(), 0., false);
                    if GeoConvHelper::with_final(|g| g.using_geo_projection()) {
                        os.set_precision(g_precision_geo());
                        GeoConvHelper::with_final(|g| g.cartesian2geo(&mut toPos));
                        os.write_attr(SUMO_ATTR_TOLONLAT, &toPos);
                        os.set_precision(g_precision());
                    } else {
                        os.write_attr(SUMO_ATTR_TOXY, &toPos);
                    }
                } else if writeJunctions {
                    os.write_attr(SUMO_ATTR_TO_JUNCTION, to.get_to_junction().get_id());
                } else {
                    os.write_attr(SUMO_ATTR_TO, to.get_id());
                }
            }
            if !self.get_parameter().via.is_empty() {
                let mut viaOut: Vec<String> = Vec::new();
                let viaAttr = if writeGeoTrip {
                    if GeoConvHelper::with_final(|g| g.using_geo_projection()) { SUMO_ATTR_VIALONLAT } else { SUMO_ATTR_VIAXY }
                } else if writeJunctions {
                    SUMO_ATTR_VIAJUNCTIONS
                } else {
                    SUMO_ATTR_VIA
                };
                for viaID in self.get_parameter().via.iter() {
                    let mut viaEdge = RONet::get_instance().get_edge(viaID);
                    if viaEdge.is_taz_connector() {
                        if viaEdge.get_predecessors().len() == 0 {
                            continue;
                        }
                        // XXX used edge that was used in route
                        viaEdge = viaEdge.get_predecessors()[0];
                    }
                    if writeGeoTrip {
                        let shape = viaEdge.get_lanes()[0].get_shape();
                        let mut viaPos = shape.position_at_offset2d(shape.length2d() / 2., 0., false);
                        if GeoConvHelper::with_final(|g| g.using_geo_projection()) {
                            GeoConvHelper::with_final(|g| g.cartesian2geo(&mut viaPos));
                            viaOut.push(to_string_prec(&viaPos, g_precision_geo()));
                        } else {
                            viaOut.push(to_string_prec(&viaPos, g_precision()));
                        }
                    } else if writeJunctions {
                        viaOut.push(viaEdge.get_to_junction().get_id().to_string());
                    } else {
                        viaOut.push(viaEdge.get_id().to_string());
                    }
                }
                os.write_attr(viaAttr, &viaOut);
            }
        } else if writeNamedRoute {
            os.write_attr(SUMO_ATTR_ROUTE, routeID.as_str());
        } else {
            self.myRoute.write_xml_definition(os, Ptr::new(self), asAlternatives, writeExit, writeCosts, writeLength);
        }
        for stop in self.get_parameter().stops.iter() {
            stop.write(os);
        }
        self.get_parameter().write_params(os);
        os.close_tag("");
    }

    /// C++: `void ROVehicle::addStop(const SUMOVehicleParameter::Stop &, const RONet *, MsgHandler *)` [private]
    /// Adds a stop to this vehicle
    // void
    // ROVehicle::addStop(const SUMOVehicleParameter::Stop& stopPar, const RONet* net, MsgHandler* errorHandler) {
    //     const ROEdge* stopEdge = net->getEdge(stopPar.edge);
    //     assert(stopEdge != 0); // was checked when parsing the stop
    //     if (stopEdge->prohibits(this)) {
    //         if (errorHandler != nullptr) {
    //             errorHandler->informf(TL("Stop edge '%' does not allow vehicle '%'."), stopEdge->getID(), getID());
    //         }
    //         return;
    //     }
    //     // where to insert the stop
    //     StopParVector::iterator iter = getParameter().stops.begin();
    //     if (stopPar.index == STOP_INDEX_END || stopPar.index >= static_cast<int>(getParameter().stops.size())) {
    //         if (getParameter().stops.size() > 0) {
    //             iter = getParameter().stops.end();
    //         }
    //     } else {
    //         if (stopPar.index == STOP_INDEX_FIT) {
    //             iter = getParameter().stops.end();
    //         } else {
    //             iter += stopPar.index;
    //         }
    //     }
    //     getParameter().stops.insert(iter, stopPar);
    //     if (stopPar.jump >= 0) {
    //         if (stopEdge->isInternal()) {
    //             if (errorHandler != nullptr) {
    //                 errorHandler->informf(TL("Jumps are not supported from internal stop edge '%'."), stopEdge->getID());
    //             }
    //         } else {
    //             if (myJumpTime < 0) {
    //                 myJumpTime = 0;
    //             }
    //             myJumpTime += stopPar.jump;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_stop(&mut self, stopPar: &SUMOVehicleParameter_Stop, net: Ptr<RONet>, errorHandler: Ptr<MsgHandler>) {
        let stopEdge = net.get_edge(&stopPar.edge);
        // assert(stopEdge != 0); // was checked when parsing the stop
        if stopEdge.prohibits(Ptr::from_mut(self), true) {
            if !errorHandler.is_null() {
                let args: [&dyn ToStringPrec; 2] = [&stopEdge.get_id().to_string(), &self.get_id().to_string()];
                errorHandler.m().informf("Stop edge '%' does not allow vehicle '%'.", &args);
            }
            return;
        }
        // where to insert the stop
        let size = self.get_parameter().stops.len() as i32;
        let mut iter: usize = 0;
        if stopPar.index == STOP_INDEX_END || stopPar.index >= size {
            if size > 0 {
                iter = size as usize;
            }
        } else {
            if stopPar.index == STOP_INDEX_FIT {
                iter = size as usize;
            } else {
                iter += stopPar.index as usize;
            }
        }
        self.get_parameter_mutable().stops.insert(iter, stopPar.clone());
        if stopPar.jump >= 0 {
            if stopEdge.is_internal() {
                if !errorHandler.is_null() {
                    let args: [&dyn ToStringPrec; 1] = [&stopEdge.get_id().to_string()];
                    errorHandler.m().informf("Jumps are not supported from internal stop edge '%'.", &args);
                }
            } else {
                if self.myJumpTime < 0 {
                    self.myJumpTime = 0;
                }
                self.myJumpTime += stopPar.jump;
            }
        }
    }

    /// C++: `void ROVehicle::updateIndex(const std::shared_ptr<RORoute>, const std::shared_ptr<RORoute>, int &)` [private]
    /// update departEdge / arrivalEdge
    // void
    // ROVehicle::updateIndex(const std::shared_ptr<RORoute> replaced, const std::shared_ptr<RORoute> current, int& attr) {
    //     const ConstROEdgeVector& oldRoute = replaced->getEdgeVector();
    //     if ((int)oldRoute.size() > attr) {
    //         const ROEdge* old = oldRoute[attr];
    //         int skips = 0;
    //         for (int i = 0; i < attr; i++) {
    //             if (oldRoute[i] == old) {
    //                 skips++;
    //             }
    //         }
    //         int i = 0;
    //         for (const ROEdge* cand : current->getEdgeVector()) {
    //             if (cand == old) {
    //                 if (skips > 0) {
    //                     skips--;
    //                 } else {
    //                     attr = i;
    //                 }
    //             }
    //             i++;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_index(&mut self, replaced: Rc<RORoute>, current: Rc<RORoute>, attr: &mut i32) {
        let oldRoute = replaced.get_edge_vector();
        if oldRoute.len() as i32 > *attr {
            let old = oldRoute[*attr as usize];
            let mut skips = 0;
            for i in 0..*attr {
                if oldRoute[i as usize] == old {
                    skips += 1;
                }
            }
            let mut i = 0;
            for cand in current.get_edge_vector().iter() {
                if *cand == old {
                    if skips > 0 {
                        skips -= 1;
                    } else {
                        *attr = i;
                    }
                }
                i += 1;
            }
        }
    }

    /// C++: `void ROVehicle::ROVehicle(const ROVehicle &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &ROVehicle) -> Self {
        todo!("port ROVehicle::ROVehicle")
    }

    /// C++: `ROVehicle & ROVehicle::operator=(const ROVehicle &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &ROVehicle) -> &mut ROVehicle {
        todo!("port ROVehicle::operator=")
    }

}

