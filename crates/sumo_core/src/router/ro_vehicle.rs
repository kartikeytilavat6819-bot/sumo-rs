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
        todo!("port Mandatory::Mandatory")
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
        todo!("port ROVehicle::ROVehicle")
    }

    /// C++: `void ROVehicle::~ROVehicle() noexcept` [virtual]
    /// Destructor
    // ROVehicle::~ROVehicle() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ROVehicle::~ROVehicle")
    }

    /// C++: `RORouteDef * ROVehicle::getRouteDefinition() const` [const]
    /// Returns the definition of the route the vehicle takes
    // inline RORouteDef* getRouteDefinition() const {
    //         return myRoute;
    //     }
    #[allow(unused_variables)]
    pub fn get_route_definition(&self) -> Ptr<RORouteDef> {
        todo!("port ROVehicle::getRouteDefinition")
    }

    /// C++: `const ROEdge * ROVehicle::getDepartEdge() const` [const, virtual]
    /// Returns the first edge the vehicle takes
    // const ROEdge*
    // ROVehicle:: getDepartEdge() const {
    //     return myRoute->getFirstRoute()->getFirst();
    // }
    #[allow(unused_variables)]
    pub fn get_depart_edge(&self) -> Ptr<ROEdge> {
        todo!("port ROVehicle::getDepartEdge")
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
        todo!("port ROVehicle::computeRoute")
    }

    /// C++: `SUMOTime ROVehicle::getDepartureTime() const` [const]
    /// Returns the time the vehicle starts at, 0 for triggered vehicles
    // inline SUMOTime getDepartureTime() const {
    //         return MAX2(SUMOTime(0), getParameter().depart);
    //     }
    #[allow(unused_variables)]
    pub fn get_departure_time(&self) -> SUMOTime {
        todo!("port ROVehicle::getDepartureTime")
    }

    /// C++: `std::vector<Mandatory> ROVehicle::getMandatoryEdges(const ROEdge *, const ROEdge *) const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_mandatory_edges(&self, requiredStart: Ptr<ROEdge>, requiredEnd: Ptr<ROEdge>) -> Vec<ROVehicle_Mandatory> {
        todo!("port ROVehicle::getMandatoryEdges")
    }

    /// C++: `const SUMOVTypeParameter & ROVehicle::getVTypeParameter() const` [const]
    /// Returns the vehicle's type definition
    // inline const SUMOVTypeParameter& getVTypeParameter() const  {
    //         return *getType();
    //     }
    #[allow(unused_variables)]
    pub fn get_v_type_parameter(&self) -> &SUMOVTypeParameter {
        todo!("port ROVehicle::getVTypeParameter")
    }

    /// C++: `double ROVehicle::getLength() const` [const]
    /// Returns the vehicle's length
    // inline double getLength() const {
    //         return getType()->length;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        todo!("port ROVehicle::getLength")
    }

    /// C++: `bool ROVehicle::hasJumps() const` [const]
    // inline bool hasJumps() const {
    //         return myJumpTime >= 0;
    //     }
    #[allow(unused_variables)]
    pub fn has_jumps(&self) -> bool {
        todo!("port ROVehicle::hasJumps")
    }

    /// C++: `SUMOTime ROVehicle::getJumpTime() const` [const]
    // inline SUMOTime getJumpTime() const {
    //         return myJumpTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_jump_time(&self) -> SUMOTime {
        todo!("port ROVehicle::getJumpTime")
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
    //                 Position toPos = to->getLanes()[0]->getShape().positionAtOffset2D(to->getLanes()[0]->getShape().length2D());
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
    //                     Position viaPos = viaEdge->getLanes()[0]->getShape().positionAtOffset2D(viaEdge->getLanes()[0]->getShape().length2D() / 2);
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
        todo!("port ROVehicle::saveAsXML")
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
        todo!("port ROVehicle::addStop")
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
        todo!("port ROVehicle::updateIndex")
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

