//! Port of duarouter/duarouter_main.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++: `void (RONet &, ROLoader &, OptionsCont &) computeRoutes`
// void
// computeRoutes(RONet& net, ROLoader& loader, OptionsCont& oc) {
//     // initialise the loader
//     loader.openRoutes(net);
//     // build the router
//     auto ttFunction = gWeightsRandomFactor > 1 ? &ROEdge::getTravelTimeStaticRandomized : &ROEdge::getTravelTimeStatic;
//     SUMOAbstractRouter<ROEdge, ROVehicle>* router;
//     const std::string measure = oc.getString("weight-attribute");
//     const std::string routingAlgorithm = oc.getString("routing-algorithm");
//     const double priorityFactor = oc.getFloat("weights.priority-factor");
//     const SUMOTime begin = string2time(oc.getString("begin"));
//     const SUMOTime end = oc.isDefault("end") ? SUMOTime_MAX : string2time(oc.getString("end"));
//     DijkstraRouter<ROEdge, ROVehicle>::Operation op = ttFunction;
//     if (measure == "traveltime") {
//         if (priorityFactor != 0) {
//             if (ROEdge::initPriorityFactor(priorityFactor)) {
//                 if (gWeightsRandomFactor > 0) {
//                     op = &ROEdge::getTravelTimeStaticPriorityFactorRandomized;
//                 } else {
//                     op = &ROEdge::getTravelTimeStaticPriorityFactor;
//                 }
//             }
//         }
//     }
//
//     if (oc.isSet("restriction-params") &&
//             (routingAlgorithm == "CH" || routingAlgorithm == "CHWrapper")) {
//         throw ProcessError(TLF("Routing algorithm '%' does not support restriction-params", routingAlgorithm));
//     }
//
//     if (measure == "traveltime") {
//         if (routingAlgorithm == "dijkstra") {
//             router = new DijkstraRouter<ROEdge, ROVehicle>(ROEdge::getAllEdges(), oc.getBool("ignore-errors"), op, nullptr, false, nullptr, net.hasPermissions(), oc.isSet("restriction-params"));
//         } else if (routingAlgorithm == "astar") {
//             typedef AStarRouter<ROEdge, ROVehicle, ROMapMatcher> AStar;
//             std::shared_ptr<const AStar::LookupTable> lookup;
//             if (oc.isSet("astar.all-distances")) {
//                 lookup = std::make_shared<const AStar::FLT>(oc.getString("astar.all-distances"), (int)ROEdge::getAllEdges().size());
//             } else if (oc.isSet("astar.landmark-distances")) {
//                 /* CHRouterWrapper<ROEdge, ROVehicle> chrouter(
//                     ROEdge::getAllEdges(), true, &ROEdge::getTravelTimeStatic,
//                     begin, end, SUMOTime_MAX, 1); */
//                 DijkstraRouter<ROEdge, ROVehicle> forward(ROEdge::getAllEdges(), true, &ROEdge::getTravelTimeStatic);
//                 std::vector<ReversedEdge<ROEdge, ROVehicle>*> reversed;
//                 for (ROEdge* edge : ROEdge::getAllEdges()) {
//                     reversed.push_back(edge->getReversedRoutingEdge());
//                 }
//                 for (ReversedEdge<ROEdge, ROVehicle>* redge : reversed) {
//                     redge->init();
//                 }
//                 DijkstraRouter<ReversedEdge<ROEdge, ROVehicle>, ROVehicle> backward(reversed, true, &ReversedEdge<ROEdge, ROVehicle>::getTravelTimeStatic);
//                 ROVehicle defaultVehicle(SUMOVehicleParameter(), nullptr, net.getVehicleTypeSecure(DEFAULT_VTYPE_ID), &net);
//                 ROMapMatcher* mapMatcher = dynamic_cast<ROMapMatcher*>(loader.getRouteHandler());
//                 lookup = std::make_shared<const AStar::LMLT>(oc.getString("astar.landmark-distances"), ROEdge::getAllEdges(), &forward, &backward, &defaultVehicle,
//                          oc.isSet("astar.save-landmark-distances") ? oc.getString("astar.save-landmark-distances") : "", oc.getInt("routing-threads"), mapMatcher);
//             }
//             router = new AStar(ROEdge::getAllEdges(), oc.getBool("ignore-errors"), op, lookup, net.hasPermissions(), oc.isSet("restriction-params"));
//         } else if (routingAlgorithm == "CH" && !net.hasPermissions() && !gRoutingPreferences && !net.hasSpeedRestrictions()) {
//             const SUMOTime weightPeriod = (oc.isSet("weight-files") ?
//                                            string2time(oc.getString("weight-period")) :
//                                            SUMOTime_MAX);
//             router = new CHRouter<ROEdge, ROVehicle>(
//                 ROEdge::getAllEdges(), oc.getBool("ignore-errors"), op, SVC_IGNORING, weightPeriod, net.hasPermissions(), oc.isSet("restriction-params"));
//         } else if (routingAlgorithm == "CHWrapper" || routingAlgorithm == "CH") {
//             // use CHWrapper instead of CH if the net has permissions, preferences or speed restriction
//             const SUMOTime weightPeriod = (oc.isSet("weight-files") ?
//                                            string2time(oc.getString("weight-period")) :
//                                            SUMOTime_MAX);
//             router = new CHRouterWrapper<ROEdge, ROVehicle>(
//                 ROEdge::getAllEdges(), oc.getBool("ignore-errors"), op,
//                 begin, end, weightPeriod, net.hasPermissions(), oc.getInt("routing-threads"));
//         } else if (routingAlgorithm == "CCH") {
//             // One weight-independent hierarchy over the union graph; one
//             // customized metric per (vehicle type, weight period), built
//             // lazily on first query (see ROCCHMetrics.h). With
//             // time-dependent weight files this follows CHRouterWrapper's
//             // per-period semantics, but repeats only the metric
//             // customization, never the topology build.
//             // Because the metrics are keyed by vehicle type, the type and
//             // class specific routing preferences of the effort function are
//             // captured exactly. weights.random-factor freezes one random
//             // realization into each metric, the same approximation
//             // CHRouterWrapper makes when building its hierarchies.
//             const bool hasWeights = oc.isSet("weight-files") || oc.isSet("lane-weight-files");
//             const SUMOTime weightPeriod = hasWeights ? string2time(oc.getString("weight-period")) : SUMOTime_MAX;
//             // Process-lifetime singletons: the topology and metric store are
//             // shared by every router clone until duarouter exits.
//             ROCCHGraph* cchGraph = new ROCCHGraph(ROEdge::getAllEdges());
//             ROCCHMetrics::init(cchGraph, op, begin, weightPeriod);
//             SUMOAbstractRouter<ROEdge, ROVehicle>* fallback = new AStarRouter<ROEdge, ROVehicle, ROMapMatcher>(
//                 ROEdge::getAllEdges(), oc.getBool("ignore-errors"), op,
//                 nullptr, net.hasPermissions(), oc.isSet("restriction-params"));
//             // restriction-params are handled natively: each distinct
//             // restriction profile gets its own masked metric (see
//             // ROCCHMetrics::get); the period-end hook lets queries whose
//             // trip crosses a weight-period boundary re-query on the next
//             // period's metric.
//             router = new CCHRouter<ROEdge, ROVehicle, ROCCHGraph>(
//                 cchGraph, &ROCCHMetrics::get, op,
//                 oc.getBool("ignore-errors"), fallback,
//                 hasWeights ? &ROCCHMetrics::periodEnd : nullptr);
//         } else if (routingAlgorithm == "arcflag") {
//             /// @brief The number of levels in the k-d tree partition
//             constexpr auto NUMBER_OF_LEVELS = 5; //or 4 or 8
//             ROVehicle defaultVehicle(SUMOVehicleParameter(), nullptr, net.getVehicleTypeSecure(DEFAULT_VTYPE_ID), &net);
//             KDTreePartition<ROEdge, RONode, ROVehicle>* partition = new KDTreePartition<ROEdge, RONode, ROVehicle>(NUMBER_OF_LEVELS, ROEdge::getAllEdges(), net.hasPermissions(), oc.isSet("restriction-params"));
//             partition->init(&defaultVehicle);
//             auto reversedTtFunction = gWeightsRandomFactor > 1 ? &FlippedEdge<ROEdge, RONode, ROVehicle>::getTravelTimeStaticRandomized : &FlippedEdge<ROEdge, RONode, ROVehicle>::getTravelTimeStatic;
//             router = new AFRouter<ROEdge, RONode, ROVehicle, ROMapMatcher>(ROEdge::getAllEdges(),
//                     partition, oc.getBool("ignore-errors"), ttFunction, reversedTtFunction, (oc.isSet("weight-files") ? string2time(oc.getString("weight-period")) : SUMOTime_MAX),
//                     nullptr, nullptr, net.hasPermissions(), oc.isSet("restriction-params"));
//         } else {
//             throw ProcessError(TLF("Unknown routing algorithm '%'!", routingAlgorithm));
//         }
//     } else {
//         if (measure == "CO") {
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
//         router = new DijkstraRouter<ROEdge, ROVehicle>(
//             ROEdge::getAllEdges(), oc.getBool("ignore-errors"), op, ttFunction, false, nullptr, net.hasPermissions(), oc.isSet("restriction-params"));
//     }
//     const int carWalk = SUMOVehicleParserHelper::parseCarWalkTransfer(oc, true);
//     double taxiWait = STEPS2TIME(string2time(OptionsCont::getOptions().getString("persontrip.taxi.waiting-time")));
//
//     RailwayRouter<ROEdge, ROVehicle>* railRouter = nullptr;
//     if (net.hasBidiEdges()) {
//         railRouter = new RailwayRouter<ROEdge, ROVehicle>(ROEdge::getAllEdges(), true, op, ttFunction, false, net.hasPermissions(),
//                 oc.isSet("restriction-params"),
//                 oc.getFloat("railway.max-train-length"),
//                 oc.getFloat("weights.reversal-penalty"));
//     }
//     RORouterProvider provider(router, new PedestrianRouter<ROEdge, ROLane, RONode, ROVehicle>(),
//                               new ROIntermodalRouter(RONet::adaptIntermodalRouter, carWalk, taxiWait, routingAlgorithm),
//                               railRouter);
//     // process route definitions
//     try {
//         net.openOutput(oc);
//         loader.processRoutes(begin, end, string2time(oc.getString("route-steps")), net, provider);
//         net.writeIntermodal(oc, provider.getIntermodalRouter());
//         // end the processing
//         net.cleanup();
//     } catch (ProcessError&) {
//         net.cleanup();
//         throw;
//     }
// }
#[allow(unused_variables)]
pub fn compute_routes(net: &mut RONet, loader: &mut ROLoader, oc: &mut OptionsCont) {
    // initialise the loader
    loader.open_routes(net);
    // build the router
    let measure = oc.get_string("weight-attribute").unwrap();
    let routingAlgorithm = oc.get_string("routing-algorithm").unwrap();
    let priorityFactor = oc.get_float("weights.priority-factor").unwrap();
    let begin = string2time(&oc.get_string("begin").unwrap()).unwrap_or_else(|e| panic!("{e}"));
    let end = if oc.is_default("end") { SUMOTIME_MAX } else { string2time(&oc.get_string("end").unwrap()).unwrap_or_else(|e| panic!("{e}")) };
    if oc.is_set("restriction-params", true).unwrap() && (routingAlgorithm == "CH" || routingAlgorithm == "CHWrapper") {
        panic!("{}", ProcessError::Process(format!("Routing algorithm '{}' does not support restriction-params", routingAlgorithm)));
    }
    if measure == "traveltime" && priorityFactor != 0. {
        ROEdge::init_priority_factor(priorityFactor);
    }
    // shortcut: router construction (the DijkstraRouter/AStar/CH/CCH/arcflag/railway instances, the operation selection and the RORouterProvider) needs SUMOAbstractRouter_Operation, which is Unmapped () in sumo_abstract_router.rs, so the rest of computeRoutes is not ported; upgrade once the operation alias is a typed function pointer
    let _ = (net, measure, begin, end);
}

/// C++: `void (RONet &, ROLoader &, OptionsCont &) initNet`
// void
// initNet(RONet& net, ROLoader& loader, OptionsCont& oc) {
//     // load the net
//     RODUAEdgeBuilder builder;
//     ROEdge::setGlobalOptions(oc.getBool("weights.interpolate"));
//     loader.loadNet(net, builder);
//     // load the weights when wished/available
//     if (oc.isSet("weight-files")) {
//         loader.loadWeights(net, "weight-files", oc.getString("weight-attribute"), false, oc.getBool("weights.expand"));
//     }
//     if (oc.isSet("lane-weight-files")) {
//         loader.loadWeights(net, "lane-weight-files", oc.getString("weight-attribute"), true, oc.getBool("weights.expand"));
//     }
//     if (oc.getBool("skip-new-routes")) {
//         RORouteDef::setSkipNew();
//     }
// }
#[allow(unused_variables)]
pub fn init_net(net: &mut RONet, loader: &mut ROLoader, oc: &mut OptionsCont) {
    // load the net
    let mut builder = RODUAEdgeBuilder::new();
    ROEdge::set_global_options(oc.get_bool("weights.interpolate").unwrap());
    loader.load_net(net, &mut builder);
    // load the weights when wished/available
    if oc.is_set("weight-files", true).unwrap() {
        loader.load_weights(net, "weight-files", &oc.get_string("weight-attribute").unwrap(), false, oc.get_bool("weights.expand").unwrap());
    }
    if oc.is_set("lane-weight-files", true).unwrap() {
        loader.load_weights(net, "lane-weight-files", &oc.get_string("weight-attribute").unwrap(), true, oc.get_bool("weights.expand").unwrap());
    }
    if oc.get_bool("skip-new-routes").unwrap() {
        RORouteDef::set_skip_new();
    }
}

