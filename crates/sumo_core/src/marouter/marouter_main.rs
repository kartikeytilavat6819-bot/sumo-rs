//! Port of marouter/marouter_main.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++: `void (RONet &, OptionsCont &) computeAllPairs`
// void
// computeAllPairs(RONet& net, OptionsCont& oc) {
//     OutputDevice::createDeviceByOption("all-pairs-output");
//     OutputDevice& outFile = OutputDevice::getDeviceByOption("all-pairs-output");
//     // build the router
//     typedef DijkstraRouter<ROEdge, ROVehicle> Dijkstra;
//     Dijkstra router(ROEdge::getAllEdges(), oc.getBool("ignore-errors"), &getTravelTime);
//     ConstROEdgeVector into;
//     const int numInternalEdges = net.getInternalEdgeNumber();
//     const int numTotalEdges = (int)net.getEdgeNumber();
//     for (int i = numInternalEdges; i < numTotalEdges; i++) {
//         const Dijkstra::EdgeInfo& ei = router.getEdgeInfo(i);
//         if (!ei.edge->isInternal()) {
//             router.compute(ei.edge, nullptr, nullptr, 0, into);
//             double fromEffort = router.getEffort(ei.edge, nullptr, 0);
//             for (int j = numInternalEdges; j < numTotalEdges; j++) {
//                 double heuTT = router.getEdgeInfo(j).effort - fromEffort;
//                 outFile << heuTT;
//                 /*
//                 if (heuTT >
//                         ei.edge->getDistanceTo(router.getEdgeInfo(j).edge)
//                         && router.getEdgeInfo(j).traveltime != std::numeric_limits<double>::max()
//                         ) {
//                     std::cout << " heuristic failure: from=" << ei.edge->getID() << " to=" << router.getEdgeInfo(j).edge->getID()
//                     << " fromEffort=" << fromEffort << " heuTT=" << heuTT << " airDist=" << ei.edge->getDistanceTo(router.getEdgeInfo(j).edge) << "\n";
//                 }
//                 */
//             }
//         }
//     }
// }
#[allow(unused_variables)]
pub fn compute_all_pairs(net: &mut RONet, oc: &mut OptionsCont) {
    todo!("port computeAllPairs")
}

/// C++: `void (RONet &, OptionsCont &, ODMatrix &) computeRoutes`
// void
// computeRoutes(RONet& net, OptionsCont& oc, ODMatrix& matrix) {
//     // build the router
//     SUMOAbstractRouter<ROEdge, ROVehicle>* router = nullptr;
//     const std::string measure = oc.getString("weight-attribute");
//     const std::string routingAlgorithm = oc.getString("routing-algorithm");
//     const double priorityFactor = oc.getFloat("weights.priority-factor");
//     SUMOTime begin = string2time(oc.getString("begin"));
//     SUMOTime end = string2time(oc.getString("end"));
//     if (oc.isDefault("begin") && matrix.getBegin() >= 0) {
//         begin = matrix.getBegin();
//     }
//     if (oc.isDefault("end")) {
//         end = matrix.getEnd() >= 0 ? matrix.getEnd() : SUMOTime_MAX;
//     }
//     DijkstraRouter<ROEdge, ROVehicle>::Operation ttOp = oc.getInt("paths") > 1 ? &ROMAAssignments::getPenalizedTT : &ROEdge::getTravelTimeStatic;
//     if (measure == "traveltime" && priorityFactor == 0) {
//         if (routingAlgorithm == "dijkstra") {
//             router = new DijkstraRouter<ROEdge, ROVehicle>(ROEdge::getAllEdges(), oc.getBool("ignore-errors"), ttOp, nullptr, false, nullptr, net.hasPermissions());
//         } else if (routingAlgorithm == "astar") {
//             router = new AStarRouter<ROEdge, ROVehicle, ROMapMatcher>(ROEdge::getAllEdges(), oc.getBool("ignore-errors"), ttOp, nullptr, net.hasPermissions());
//         } else if (routingAlgorithm == "CH" && !net.hasPermissions()) {
//             const SUMOTime weightPeriod = (oc.isSet("weight-files") ?
//                                            string2time(oc.getString("weight-period")) :
//                                            SUMOTime_MAX);
//             router = new CHRouter<ROEdge, ROVehicle>(ROEdge::getAllEdges(), oc.getBool("ignore-errors"), &ROEdge::getTravelTimeStatic, SVC_IGNORING, weightPeriod, net.hasPermissions(), false);
//         } else if (routingAlgorithm == "CHWrapper" || routingAlgorithm == "CH") {
//             // use CHWrapper instead of CH if the net has permissions
//             const SUMOTime weightPeriod = (oc.isSet("weight-files") ?
//                                            string2time(oc.getString("weight-period")) :
//                                            SUMOTime_MAX);
//             router = new CHRouterWrapper<ROEdge, ROVehicle>(
//                 ROEdge::getAllEdges(), oc.getBool("ignore-errors"), &ROEdge::getTravelTimeStatic,
//                 begin, end, weightPeriod, net.hasPermissions(), oc.getInt("routing-threads"));
//         } else if (routingAlgorithm == "CCH") {
//             // One weight-independent hierarchy over the union graph and one
//             // customized metric for the default vehicle's type, filled from
//             // the travel times of the current assignment iteration.
//             // ROMAAssignments resets the router after every iteration (where
//             // CH rebuilds its whole hierarchy); the reset hook flags the
//             // metric for re-customization at the next query, so every
//             // iteration routes on exactly the travel times a Dijkstra query
//             // would see. Like CH, the metric cannot follow the k shortest
//             // path penalties (see ROMAFrame::checkOptions).
//             ROCCHGraph* cchGraph = new ROCCHGraph(ROEdge::getAllEdges());
//             ROCCHMetrics::init(cchGraph, &ROEdge::getTravelTimeStatic, begin, SUMOTime_MAX);
//             SUMOAbstractRouter<ROEdge, ROVehicle>* fallback = new AStarRouter<ROEdge, ROVehicle, ROMapMatcher>(
//                 ROEdge::getAllEdges(), oc.getBool("ignore-errors"), &ROEdge::getTravelTimeStatic, nullptr, net.hasPermissions());
//             router = new CCHRouter<ROEdge, ROVehicle, ROCCHGraph>(
//                 cchGraph, &ROCCHMetrics::get, &ROEdge::getTravelTimeStatic,
//                 oc.getBool("ignore-errors"), fallback, nullptr, &ROCCHMetrics::reset);
//         } else {
//             throw ProcessError(TLF("Unknown routing Algorithm '%'!", routingAlgorithm));
//         }
//     } else {
//         DijkstraRouter<ROEdge, ROVehicle>::Operation op;
//         if (measure == "traveltime") {
//             if (ROEdge::initPriorityFactor(priorityFactor)) {
//                 op = &ROEdge::getTravelTimeStaticPriorityFactor;
//             } else {
//                 op = &ROEdge::getTravelTimeStatic;
//             }
//         } else if (measure == "CO") {
//             op = &ROEdge::getEmissionEffort<PollutantsInterface::CO>;
//         } else if (measure == "CO2") {
//             op = &ROEdge::getEmissionEffort<PollutantsInterface::CO2>;
//         } else if (measure == "PMx") {
//             op = &ROEdge::getEmissionEffort<PollutantsInterface::PM_X>;
//         } else if (measure == "HC") {
//             op = &ROEdge::getEmissionEffort<PollutantsInterface::HC>;
//         } else if (measure == "NOx") {
//             op = &ROEdge::getEmissionEffort<PollutantsInterface::NO_X>;
//         } else if (measure == "fuel") {
//             op = &ROEdge::getEmissionEffort<PollutantsInterface::FUEL>;
//         } else if (measure == "electricity") {
//             op = &ROEdge::getEmissionEffort<PollutantsInterface::ELEC>;
//         } else if (measure == "noise") {
//             op = &ROEdge::getNoiseEffort;
//         } else {
//             op = &ROEdge::getStoredEffort;
//         }
//         if (measure != "traveltime" && !net.hasLoadedEffort()) {
//             WRITE_WARNINGF(TL("No weight data was loaded for attribute '%'."), measure);
//         }
//         router = new DijkstraRouter<ROEdge, ROVehicle>(ROEdge::getAllEdges(), oc.getBool("ignore-errors"), op, ttOp, false, nullptr, net.hasPermissions());
//     }
//     try {
//         const RORouterProvider provider(router, nullptr, nullptr, nullptr);
//         // prepare the output
//         net.openOutput(oc);
//         // process route definitions
//         if (oc.isSet("timeline")) {
//             matrix.applyCurve(matrix.parseTimeLine(oc.getStringVector("timeline"), oc.getBool("timeline.day-in-hours")));
//         }
//         matrix.sortByBeginTime();
//         bool haveOutput = OutputDevice::createDeviceByOption("netload-output", "meandata");
//         ROVehicle defaultVehicle(SUMOVehicleParameter(), nullptr, net.getVehicleTypeSecure(DEFAULT_VTYPE_ID), &net);
//         ROMAAssignments a(begin, end, oc.getBool("additive-traffic"), oc.getFloat("weight-adaption"),
//                           oc.getInt("max-alternatives"), oc.getBool("capacities.default"), net, matrix, *router,
//                           haveOutput ? &OutputDevice::getDeviceByOption("netload-output") : nullptr);
//         a.resetFlows();
// #ifdef HAVE_FOX
//         // this is just to init the CHRouter with the default vehicle
//         router->reset(&defaultVehicle);
//         const int maxNumThreads = oc.getInt("routing-threads");
//         while ((int)net.getThreadPool().size() < maxNumThreads) {
//             new RONet::WorkerThread(net.getThreadPool(), provider);
//         }
// #endif
//         std::string assignMethod = oc.getString("assignment-method");
//         if (assignMethod == "UE") {
//             WRITE_WARNING(TL("Deterministic user equilibrium ('UE') is not implemented yet, using stochastic method ('SUE')."));
//             assignMethod = "SUE";
//         }
//         if (assignMethod == "incremental") {
//             a.incremental(oc.getInt("max-iterations"), oc.getBool("verbose"));
//         } else if (assignMethod == "SUE") {
//             a.sue(oc.getInt("max-iterations"), oc.getInt("max-inner-iterations"),
//                   oc.getInt("paths"), oc.getFloat("paths.penalty"), oc.getFloat("tolerance"), oc.getString("route-choice-method"));
//         }
//         // update path costs and output
//         OutputDevice* dev = net.getRouteOutput();
//         if (dev != nullptr) {
//             std::vector<std::string> tazParamKeys;
//             if (oc.isSet("taz-param")) {
//                 tazParamKeys = oc.getStringVector("taz-param");
//             }
//             std::map<SUMOTime, std::string> sortedOut;
//             SUMOTime lastEnd = -1;
//             int num = 0;
//             for (const ODCell* const c : matrix.getCells()) {
//                 if (c->begin >= end || c->end <= begin ||
//                         c->pathsVector.empty() || c->pathsVector.front()->getEdgeVector().empty()) {
//                     continue;
//                 }
//                 if (lastEnd >= 0 && lastEnd <= c->begin) {
//                     for (std::map<SUMOTime, std::string>::const_iterator desc = sortedOut.begin(); desc != sortedOut.end(); ++desc) {
//                         dev->writePreformattedTag(desc->second);
//                     }
//                     sortedOut.clear();
//                 }
//                 if (c->departures.empty()) {
//                     const SUMOTime b = MAX2(begin, c->begin);
//                     const SUMOTime e = MIN2(end, c->end);
//                     const int numVehs = int(c->vehicleNumber * (double)(e - b) / (double)(c->end - c->begin));
//                     OutputDevice_String od(1);
//                     od.openTag(SUMO_TAG_FLOW).writeAttr(SUMO_ATTR_ID, oc.getString("prefix") + toString(num++));
//                     od.writeAttr(SUMO_ATTR_BEGIN, time2string(b)).writeAttr(SUMO_ATTR_END, time2string(e));
//                     od.writeAttr(SUMO_ATTR_NUMBER, numVehs);
//                     matrix.writeDefaultAttrs(od, oc.getBool("ignore-vehicle-type"), c);
//                     od.openTag(SUMO_TAG_ROUTE_DISTRIBUTION);
//                     for (const std::shared_ptr<RORoute>& r : c->pathsVector) {
//                         r->setCosts(router->recomputeCosts(r->getEdgeVector(), &defaultVehicle, begin));
//                         r->writeXMLDefinition(od, nullptr, true, true, false, false);
//                     }
//                     od.closeTag();
//                     od.closeTag();
//                     sortedOut[c->begin] += od.getString();
//                 } else {
//                     const bool ignoreType = oc.getBool("ignore-vehicle-type");
//                     for (auto deps : c->departures) {
//                         if (deps.first >= end || deps.first < begin) {
//                             continue;
//                         }
//                         const std::string routeDistId = c->origin + "_" + c->destination + "_" + time2string(c->begin) + "_" + time2string(c->end);
//                         for (const SUMOVehicleParameter& veh : deps.second) {
//                             OutputDevice_String od(1);
//
//                             veh.write(od, OptionsCont::getOptions(), SUMO_TAG_VEHICLE, ignoreType || veh.vtypeid == DEFAULT_VTYPE_ID ? "" : veh.vtypeid);
//                             od.openTag(SUMO_TAG_ROUTE_DISTRIBUTION);
//                             for (const std::shared_ptr<RORoute>& r : c->pathsVector) {
//                                 r->setCosts(router->recomputeCosts(r->getEdgeVector(), &defaultVehicle, begin));
//                                 r->writeXMLDefinition(od, nullptr, true, true, false, false);
//                             }
//                             od.closeTag();
//                             if (!tazParamKeys.empty()) {
//                                 od.openTag(SUMO_TAG_PARAM).writeAttr(SUMO_ATTR_KEY, tazParamKeys[0]).writeAttr(SUMO_ATTR_VALUE, c->origin).closeTag();
//                                 if (tazParamKeys.size() > 1) {
//                                     od.openTag(SUMO_TAG_PARAM).writeAttr(SUMO_ATTR_KEY, tazParamKeys[1]).writeAttr(SUMO_ATTR_VALUE, c->destination).closeTag();
//                                 }
//                             }
//                             od.closeTag();
//                             sortedOut[deps.first] += od.getString();
//                         }
//                     }
//                 }
//                 if (c->end > lastEnd) {
//                     lastEnd = c->end;
//                 }
//             }
//             for (std::map<SUMOTime, std::string>::const_iterator desc = sortedOut.begin(); desc != sortedOut.end(); ++desc) {
//                 dev->writePreformattedTag(desc->second);
//             }
//             haveOutput = true;
//         }
//         if (!haveOutput) {
//             throw ProcessError(TL("No output file given."));
//         }
//         // end the processing
//         net.cleanup();
//     } catch (ProcessError&) {
//         net.cleanup();
//         throw;
//     }
// }
#[allow(unused_variables)]
pub fn compute_routes(net: &mut RONet, oc: &mut OptionsCont, matrix: &mut ODMatrix) {
    todo!("port computeRoutes")
}

/// C++: `double (const ROEdge *const, const ROVehicle *const, double) getTravelTime`
// double
// getTravelTime(const ROEdge* const edge, const ROVehicle* const /* veh */, double /* time */) {
//     return edge->getLength() / edge->getSpeedLimit();
// }
#[allow(unused_variables)]
pub fn get_travel_time(edge: Ptr<ROEdge>, arg1: Ptr<ROVehicle>, arg2: f64) -> f64 {
    todo!("port getTravelTime")
}

