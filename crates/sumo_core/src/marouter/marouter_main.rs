//! Port of marouter/marouter_main.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::duarouter::duarouter_main::{effort_co, effort_co2, effort_elec, effort_fuel, effort_hc, effort_no_x, effort_pm_x, leak_router};
use crate::router::ro_net::ROMapMatcher;
use crate::utils::router::a_star_router::AStarRouter;
use crate::utils::router::ch_router::CHRouter;
use crate::utils::router::ch_router_wrapper::CHRouterWrapper;
use crate::utils::router::dijkstra_router::DijkstraRouter;
use crate::utils::router::sumo_abstract_router::{SUMOAbstractRouter, SUMOAbstractRouter_TypedOperation};
use utils::common::sumo_vehicle_class::SVC_IGNORING;

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
    OutputDevice::create_device_by_option("all-pairs-output", "", "", -1).unwrap_or_else(|e| panic!("{e}"));
    let outFile = OutputDevice::get_device_by_option("all-pairs-output").unwrap_or_else(|e| panic!("{e}"));
    // build the router
    let mut router: crate::utils::router::dijkstra_router::DijkstraRouter<ROEdge, ROVehicle> = crate::utils::router::dijkstra_router::DijkstraRouter::new(ROEdge::get_all_edges(), oc.get_bool("ignore-errors").unwrap_or_else(|e| panic!("{e}")), Some(get_travel_time), None, false, Ptr::null(), false, false).with_compute();
    let mut into: Vec<Ptr<ROEdge>> = Vec::new();
    let numInternalEdges: i32 = net.get_internal_edge_number();
    let numTotalEdges: i32 = net.get_edge_number();
    for i in numInternalEdges..numTotalEdges {
        let ei_edge: Ptr<ROEdge> = router.get_edge_info(i).edge.cast::<ROEdge>();
        if !ei_edge.is_internal() {
            router.compute(ei_edge, Ptr::null(), Ptr::null(), 0, &mut into, false);
            let fromEffort: f64 = router.get_effort(ei_edge, Ptr::null(), 0.0);
            for j in numInternalEdges..numTotalEdges {
                let heuTT: f64 = router.get_edge_info(j).effort - fromEffort;
                outFile.borrow_mut().write(&heuTT);
            }
        }
    }
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
    // build the router
    let measure = oc.get_string("weight-attribute").unwrap_or_else(|e| panic!("{e}"));
    let routingAlgorithm = oc.get_string("routing-algorithm").unwrap_or_else(|e| panic!("{e}"));
    let priorityFactor = oc.get_float("weights.priority-factor").unwrap_or_else(|e| panic!("{e}"));
    let mut begin = utils::common::time::string2time(&oc.get_string("begin").unwrap_or_else(|e| panic!("{e}"))).unwrap_or_else(|e| panic!("{e}"));
    let mut end = utils::common::time::string2time(&oc.get_string("end").unwrap_or_else(|e| panic!("{e}"))).unwrap_or_else(|e| panic!("{e}"));
    if oc.is_default("begin") && matrix.get_begin() >= 0 {
        begin = matrix.get_begin();
    }
    if oc.is_default("end") {
        end = if matrix.get_end() >= 0 { matrix.get_end() } else { utils::common::time::SUMOTIME_MAX };
    }
    let ignoreErrors = oc.get_bool("ignore-errors").unwrap_or_else(|e| panic!("{e}"));
    let ttOp: SUMOAbstractRouter_TypedOperation<ROEdge, ROVehicle> = if oc.get_int("paths").unwrap_or_else(|e| panic!("{e}")) > 1 { ROMAAssignments::get_penalized_tt } else { ROEdge::get_travel_time_static };
    let weightPeriod = if oc.is_set("weight-files", true).unwrap_or_else(|e| panic!("{e}")) { utils::common::time::string2time(&oc.get_string("weight-period").unwrap_or_else(|e| panic!("{e}"))).unwrap_or_else(|e| panic!("{e}")) } else { utils::common::time::SUMOTIME_MAX };
    // C++ `new` of the router, held as its SUMOAbstractRouter base (the routers live until the process exits)
    let routerPtr: Ptr<SUMOAbstractRouter<ROEdge, ROVehicle>> = if measure == "traveltime" && priorityFactor == 0.0 {
        match routingAlgorithm.as_str() {
            "dijkstra" => leak_router(DijkstraRouter::new(ROEdge::get_all_edges(), ignoreErrors, Some(ttOp), None, false, Ptr::null(), net.has_permissions(), false).with_compute()),
            "astar" => leak_router(AStarRouter::<ROEdge, ROVehicle, ROMapMatcher>::new(&ROEdge::get_all_edges(), ignoreErrors, Some(ttOp), None, net.has_permissions(), false).with_compute()),
            "CH" if !net.has_permissions() => leak_router(CHRouter::new(&ROEdge::get_all_edges(), ignoreErrors, Some(ROEdge::get_travel_time_static), SVC_IGNORING, weightPeriod, net.has_permissions(), false).with_compute()),
            "CHWrapper" | "CH" => leak_router(CHRouterWrapper::new(&ROEdge::get_all_edges(), ignoreErrors, Some(ROEdge::get_travel_time_static), begin, end, weightPeriod, net.has_permissions(), oc.get_int("routing-threads").unwrap_or_else(|e| panic!("{e}"))).with_compute()),
            "CCH" => {
                let cchGraph: Ptr<ROCCHGraph> = Ptr::from_raw(Box::into_raw(Box::new(CCHGraph::new(&ROEdge::get_all_edges()))));
                ROCCHMetrics::init(cchGraph, ROEdge::get_travel_time_static, begin, utils::common::time::SUMOTIME_MAX);
                let fallback = leak_router(AStarRouter::<ROEdge, ROVehicle, ROMapMatcher>::new(&ROEdge::get_all_edges(), ignoreErrors, Some(ROEdge::get_travel_time_static), None, net.has_permissions(), false).with_compute());
                leak_router(CCHRouter::<ROEdge, ROVehicle, ROCCHGraph>::new(cchGraph, ROCCHMetrics::get, Some(ROEdge::get_travel_time_static), ignoreErrors, fallback, None, Some(ROCCHMetrics::reset)))
            }
            _ => {
                panic!("{}", ProcessError::new(format!("Unknown routing Algorithm '{}'!", routingAlgorithm)));
            }
        }
    } else {
        let op: SUMOAbstractRouter_TypedOperation<ROEdge, ROVehicle> = if measure == "traveltime" {
            if ROEdge::init_priority_factor(priorityFactor) { ROEdge::get_travel_time_static_priority_factor } else { ROEdge::get_travel_time_static }
        } else if measure == "CO" {
            effort_co
        } else if measure == "CO2" {
            effort_co2
        } else if measure == "PMx" {
            effort_pm_x
        } else if measure == "HC" {
            effort_hc
        } else if measure == "NOx" {
            effort_no_x
        } else if measure == "fuel" {
            effort_fuel
        } else if measure == "electricity" {
            effort_elec
        } else if measure == "noise" {
            ROEdge::get_noise_effort
        } else {
            ROEdge::get_stored_effort
        };
        if measure != "traveltime" && !net.has_loaded_effort() {
            utils::common::msg_handler::write_warning(&format!("No weight data was loaded for attribute '{}'.", measure));
        }
        leak_router(DijkstraRouter::new(ROEdge::get_all_edges(), ignoreErrors, Some(op), Some(ttOp), false, Ptr::null(), net.has_permissions(), false).with_compute())
    };
    let router: &mut SUMOAbstractRouter<ROEdge, ROVehicle> = routerPtr.m();
    // prepare the output
    net.open_output(oc);
    // process route definitions
    if oc.is_set("timeline", true).unwrap_or_else(|e| panic!("{e}")) {
        let timeline = oc.get_string_vector("timeline").unwrap_or_else(|e| panic!("{e}")).clone();
        let dayInHours = oc.get_bool("timeline.day-in-hours").unwrap_or_else(|e| panic!("{e}"));
        let points = matrix.parse_time_line(&timeline, dayInHours);
        matrix.apply_curve(&points);
    }
    matrix.sort_by_begin_time();
    let mut haveOutput = OutputDevice::create_device_by_option("netload-output", "meandata", "", -1).unwrap_or_else(|e| panic!("{e}"));
    let defaultVType = net.get_vehicle_type_secure(utils::common::sumo_vehicle_class::DEFAULT_VTYPE_ID);
    let mut defaultVehicle = ROVehicle::new(&SUMOVehicleParameter::new(), Ptr::null(), defaultVType, Ptr::from_mut(&mut *net), Ptr::null());
    let netloadOutput: Ptr<OutputDevice> = if haveOutput {
        Ptr::from_raw(OutputDevice::get_device_by_option("netload-output").unwrap_or_else(|e| panic!("{e}")).as_ptr())
    } else {
        Ptr::null()
    };
    let mut a = ROMAAssignments::new(
        begin,
        end,
        oc.get_bool("additive-traffic").unwrap_or_else(|e| panic!("{e}")),
        oc.get_float("weight-adaption").unwrap_or_else(|e| panic!("{e}")),
        oc.get_int("max-alternatives").unwrap_or_else(|e| panic!("{e}")),
        oc.get_bool("capacities.default").unwrap_or_else(|e| panic!("{e}")),
        net,
        matrix,
        router,
        netloadOutput,
    );
    a.reset_flows();
    // C++ (HAVE_FOX): this is just to init the CHRouter with the default vehicle
    router.reset(Ptr::from_mut(&mut defaultVehicle));
    // shortcut: the worker thread pool (RONet::WorkerThread, routing-threads) is not ported, routing runs on this thread only
    let mut assignMethod = oc.get_string("assignment-method").unwrap_or_else(|e| panic!("{e}"));
    if assignMethod == "UE" {
        utils::common::msg_handler::write_warning("Deterministic user equilibrium ('UE') is not implemented yet, using stochastic method ('SUE').");
        assignMethod = "SUE".to_string();
    }
    if assignMethod == "incremental" {
        a.incremental(oc.get_int("max-iterations").unwrap_or_else(|e| panic!("{e}")), oc.get_bool("verbose").unwrap_or_else(|e| panic!("{e}")));
    } else if assignMethod == "SUE" {
        a.sue(
            oc.get_int("max-iterations").unwrap_or_else(|e| panic!("{e}")),
            oc.get_int("max-inner-iterations").unwrap_or_else(|e| panic!("{e}")),
            oc.get_int("paths").unwrap_or_else(|e| panic!("{e}")),
            oc.get_float("paths.penalty").unwrap_or_else(|e| panic!("{e}")),
            oc.get_float("tolerance").unwrap_or_else(|e| panic!("{e}")),
            oc.get_string("route-choice-method").unwrap_or_else(|e| panic!("{e}")),
        );
    }
    // update path costs and output
    let dev = net.get_route_output(false);
    if !dev.is_null() {
        let tazParamKeys: Vec<String> = if oc.is_set("taz-param", true).unwrap_or_else(|e| panic!("{e}")) {
            oc.get_string_vector("taz-param").unwrap_or_else(|e| panic!("{e}")).clone()
        } else {
            Vec::new()
        };
        let mut sortedOut: BTreeMap<SUMOTime, String> = BTreeMap::new();
        let mut lastEnd: SUMOTime = -1;
        let mut num: i32 = 0;
        let cells: Vec<Ptr<ODCell>> = matrix.get_cells().clone();
        let prefix = oc.get_string("prefix").unwrap_or_else(|e| panic!("{e}"));
        let ignoreType = oc.get_bool("ignore-vehicle-type").unwrap_or_else(|e| panic!("{e}"));
        for c in cells.iter().copied() {
            if c.begin >= end || c.end <= begin || c.pathsVector.is_empty() || c.pathsVector[0].get_edge_vector().is_empty() {
                continue;
            }
            if lastEnd >= 0 && lastEnd <= c.begin {
                for desc in sortedOut.values() {
                    dev.m().write_preformatted_tag(desc);
                }
                sortedOut.clear();
            }
            if c.departures.is_empty() {
                let b = std::cmp::max(begin, c.begin);
                let e = std::cmp::min(end, c.end);
                let numVehs: i32 = (c.vehicleNumber * (e - b) as f64 / (c.end - c.begin) as f64) as i32;
                let mut od = OutputDevice::new_string(1);
                let id = format!("{}{}", prefix, num);
                num += 1;
                od.open_tag_id(SUMO_TAG_FLOW).write_attr(SUMO_ATTR_ID, &id);
                od.write_attr(SUMO_ATTR_BEGIN, &time2string(b)).write_attr(SUMO_ATTR_END, &time2string(e));
                od.write_attr(SUMO_ATTR_NUMBER, &numVehs);
                matrix.write_default_attrs(&mut od, ignoreType, c);
                od.open_tag_id(SUMO_TAG_ROUTE_DISTRIBUTION);
                for r in c.pathsVector.iter() {
                    let costs = router.recompute_costs(r.get_edge_vector(), Ptr::from_mut(&mut defaultVehicle), begin, Ptr::null());
                    Ptr::from_raw(Rc::as_ptr(r) as *mut RORoute).m().set_costs(costs);
                    r.write_xml_definition(&mut od, Ptr::null(), true, true, false, false, "");
                }
                od.close_tag("");
                od.close_tag("");
                sortedOut.entry(c.begin).or_default().push_str(&od.get_string());
            } else {
                for (depTime, vehs) in c.departures.iter() {
                    if *depTime >= end || *depTime < begin {
                        continue;
                    }
                    for veh in vehs.iter() {
                        let mut od = OutputDevice::new_string(1);
                        let vtype: &str = if ignoreType || veh.vtypeid == utils::common::sumo_vehicle_class::DEFAULT_VTYPE_ID { "" } else { veh.vtypeid.as_str() };
                        veh.write(&mut od, oc, SUMO_TAG_VEHICLE, vtype);
                        od.open_tag_id(SUMO_TAG_ROUTE_DISTRIBUTION);
                        for r in c.pathsVector.iter() {
                            let costs = router.recompute_costs(r.get_edge_vector(), Ptr::from_mut(&mut defaultVehicle), begin, Ptr::null());
                            Ptr::from_raw(Rc::as_ptr(r) as *mut RORoute).m().set_costs(costs);
                            r.write_xml_definition(&mut od, Ptr::null(), true, true, false, false, "");
                        }
                        od.close_tag("");
                        if !tazParamKeys.is_empty() {
                            od.open_tag_id(SUMO_TAG_PARAM).write_attr(SUMO_ATTR_KEY, &tazParamKeys[0]).write_attr(SUMO_ATTR_VALUE, &c.origin).close_tag("");
                            if tazParamKeys.len() > 1 {
                                od.open_tag_id(SUMO_TAG_PARAM).write_attr(SUMO_ATTR_KEY, &tazParamKeys[1]).write_attr(SUMO_ATTR_VALUE, &c.destination).close_tag("");
                            }
                        }
                        od.close_tag("");
                        sortedOut.entry(*depTime).or_default().push_str(&od.get_string());
                    }
                }
            }
            if c.end > lastEnd {
                lastEnd = c.end;
            }
        }
        for desc in sortedOut.values() {
            dev.m().write_preformatted_tag(desc);
        }
        haveOutput = true;
    }
    if !haveOutput {
        panic!("{}", ProcessError::new("No output file given."));
    }
    // end the processing
    net.cleanup();
}

/// C++: `double (const ROEdge *const, const ROVehicle *const, double) getTravelTime`
// double
// getTravelTime(const ROEdge* const edge, const ROVehicle* const /* veh */, double /* time */) {
//     return edge->getLength() / edge->getSpeedLimit();
// }
#[allow(unused_variables)]
pub fn get_travel_time(edge: Ptr<ROEdge>, arg1: Ptr<ROVehicle>, arg2: f64) -> f64 {
    edge.get_length() / edge.get_speed_limit()
}

