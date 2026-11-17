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
use crate::router::ro_net::ROMapMatcher;
use crate::utils::router::a_star_lookup_table::{AStarLookupTableKind, FullLookupTable, LandmarkLookupTable};
use crate::utils::router::reversed_edge::ReversedEdge;
use crate::router::ro_vehicle::ROVehicle;
use crate::utils::emissions::pollutants_interface::PollutantsInterface_EmissionType;
use utils::common::sumo_vehicle_class::DEFAULT_VTYPE_ID;
use crate::utils::vehicle::sumo_vehicle_parameter::SUMOVehicleParameter;
use crate::utils::router::a_star_router::AStarRouter;
use crate::utils::router::ch_router::CHRouter;
use crate::utils::router::ch_router_wrapper::CHRouterWrapper;
use crate::utils::router::dijkstra_router::DijkstraRouter;
use crate::utils::router::sumo_abstract_router::SUMOAbstractRouter;
use utils::common::sumo_vehicle_class::SVC_IGNORING;

// C++ `&ROEdge::getEmissionEffort<PollutantsInterface::X>`: one fn item per template instantiation
fn effort_co(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
    ROEdge::get_emission_effort(PollutantsInterface_EmissionType::CO, edge, veh, time)
}
fn effort_co2(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
    ROEdge::get_emission_effort(PollutantsInterface_EmissionType::CO2, edge, veh, time)
}
fn effort_pm_x(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
    ROEdge::get_emission_effort(PollutantsInterface_EmissionType::PM_X, edge, veh, time)
}
fn effort_hc(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
    ROEdge::get_emission_effort(PollutantsInterface_EmissionType::HC, edge, veh, time)
}
fn effort_no_x(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
    ROEdge::get_emission_effort(PollutantsInterface_EmissionType::NO_X, edge, veh, time)
}
fn effort_fuel(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
    ROEdge::get_emission_effort(PollutantsInterface_EmissionType::FUEL, edge, veh, time)
}
fn effort_elec(edge: Ptr<ROEdge>, veh: Ptr<ROVehicle>, time: f64) -> f64 {
    ROEdge::get_emission_effort(PollutantsInterface_EmissionType::ELEC, edge, veh, time)
}

/// C++ `new` of a router, held as its SUMOAbstractRouter base (the routers live until the process exits)
fn leak_router<R>(r: R) -> Ptr<SUMOAbstractRouter<ROEdge, ROVehicle>> {
    Ptr::from_raw(Box::into_raw(Box::new(r))).cast::<SUMOAbstractRouter<ROEdge, ROVehicle>>()
}

pub fn compute_routes(net: &mut RONet, loader: &mut ROLoader, oc: &mut OptionsCont) {
    // initialise the loader
    loader.open_routes(net);
    // build the router
    let measure = oc.get_string("weight-attribute").unwrap();
    let routingAlgorithm = oc.get_string("routing-algorithm").unwrap();
    let priorityFactor = oc.get_float("weights.priority-factor").unwrap();
    let randomFactor = oc.get_float("weights.random-factor").unwrap();
    let begin = string2time(&oc.get_string("begin").unwrap()).unwrap_or_else(|e| panic!("{e}"));
    let end = if oc.is_default("end") { SUMOTIME_MAX } else { string2time(&oc.get_string("end").unwrap()).unwrap_or_else(|e| panic!("{e}")) };
    let ttFunction: crate::utils::router::sumo_abstract_router::SUMOAbstractRouter_TypedOperation<ROEdge, ROVehicle> = if randomFactor > 1. { ROEdge::get_travel_time_static_randomized } else { ROEdge::get_travel_time_static };
    let mut op = ttFunction;
    if measure == "traveltime" && priorityFactor != 0. && ROEdge::init_priority_factor(priorityFactor) {
        op = if randomFactor > 0. { ROEdge::get_travel_time_static_priority_factor_randomized } else { ROEdge::get_travel_time_static_priority_factor };
    }
    if oc.is_set("restriction-params", true).unwrap() && (routingAlgorithm == "CH" || routingAlgorithm == "CHWrapper") {
        panic!("{}", ProcessError::Process(format!("Routing algorithm '{}' does not support restriction-params", routingAlgorithm)));
    }
    let ignoreErrors = oc.get_bool("ignore-errors").unwrap();
    let restrictionParams = oc.is_set("restriction-params", true).unwrap();
    // C++ `new`: the routers live until the process exits
    let weightPeriod = if oc.is_set("weight-files", true).unwrap() { string2time(&oc.get_string("weight-period").unwrap()).unwrap_or_else(|e| panic!("{e}")) } else { SUMOTIME_MAX };
    let router: Ptr<SUMOAbstractRouter<ROEdge, ROVehicle>> = if measure == "traveltime" {
        if routingAlgorithm == "dijkstra" {
            leak_router(DijkstraRouter::new(ROEdge::get_all_edges(), ignoreErrors, Some(op), None, false, Ptr::null(), net.has_permissions(), restrictionParams).with_compute())
        } else if routingAlgorithm == "astar" {
            let mut lookup: Option<std::rc::Rc<AStarLookupTableKind<ROEdge, ROVehicle, ROMapMatcher>>> = None;
            if oc.is_set("astar.all-distances", true).unwrap() {
                lookup = Some(std::rc::Rc::new(AStarLookupTableKind::Full(FullLookupTable::new(&oc.get_string("astar.all-distances").unwrap(), ROEdge::get_all_edges().len() as i32))));
            } else if oc.is_set("astar.landmark-distances", true).unwrap() {
                let forward = leak_router(DijkstraRouter::new(ROEdge::get_all_edges(), true, Some(ROEdge::get_travel_time_static), None, false, Ptr::null(), false, false).with_compute());
                let reversed: Vec<Ptr<ReversedEdge<ROEdge, ROVehicle>>> = ROEdge::get_all_edges().iter().map(|edge| edge.get_reversed_routing_edge()).collect();
                for redge in reversed.iter() {
                    redge.m().init();
                }
                let backward = Ptr::from_raw(Box::into_raw(Box::new(DijkstraRouter::new(&reversed, true, Some(ReversedEdge::<ROEdge, ROVehicle>::get_travel_time_static), None, false, Ptr::null(), false, false).with_compute()))).cast::<SUMOAbstractRouter<ReversedEdge<ROEdge, ROVehicle>, ROVehicle>>();
                let defaultVehicle: Ptr<ROVehicle> = Ptr::from_raw(Box::into_raw(Box::new(ROVehicle::new(&SUMOVehicleParameter::new(), Ptr::null(), net.get_vehicle_type_secure(DEFAULT_VTYPE_ID), Ptr::from_mut(&mut *net), Ptr::null()))));
                let saveFile = if oc.is_set("astar.save-landmark-distances", true).unwrap() { oc.get_string("astar.save-landmark-distances").unwrap() } else { String::new() };
                lookup = Some(std::rc::Rc::new(AStarLookupTableKind::Landmark(LandmarkLookupTable::new(&oc.get_string("astar.landmark-distances").unwrap(), ROEdge::get_all_edges(), forward, backward, defaultVehicle, &saveFile, oc.get_int("routing-threads").unwrap(), Ptr::null()))));
            }
            leak_router(AStarRouter::<ROEdge, ROVehicle, ROMapMatcher>::new(&ROEdge::get_all_edges(), ignoreErrors, Some(op), lookup, net.has_permissions(), restrictionParams).with_compute())
        } else if routingAlgorithm == "CH" && !net.has_permissions() && !utils::common::std_defs::g_routing_preferences() && !net.has_speed_restrictions() {
            leak_router(CHRouter::new(&ROEdge::get_all_edges(), ignoreErrors, Some(op), SVC_IGNORING, weightPeriod, net.has_permissions(), restrictionParams).with_compute())
        } else if routingAlgorithm == "CHWrapper" || routingAlgorithm == "CH" {
            leak_router(CHRouterWrapper::new(&ROEdge::get_all_edges(), ignoreErrors, Some(op), begin, end, weightPeriod, net.has_permissions(), oc.get_int("routing-threads").unwrap()).with_compute())
        } else {
            // shortcut: CCH and AFRouter are not wired in duarouter yet (their metric and arc-flag tables are not ported), upgrade when computeRoutes can construct them
            panic!("shortcut: routing algorithm '{}' is not wired in duarouter yet", routingAlgorithm);
        }
    } else {
        op = match measure.as_str() {
            "CO" => effort_co,
            "CO2" => effort_co2,
            "PMx" => effort_pm_x,
            "HC" => effort_hc,
            "NOx" => effort_no_x,
            "fuel" => effort_fuel,
            "electricity" => effort_elec,
            "noise" => ROEdge::get_noise_effort,
            _ => ROEdge::get_stored_effort,
        };
        if !net.has_loaded_effort() {
            write_warningf!("No weight data was loaded for attribute '%'.", measure);
        }
        leak_router(DijkstraRouter::new(ROEdge::get_all_edges(), ignoreErrors, Some(op), Some(ttFunction), false, Ptr::null(), net.has_permissions(), restrictionParams).with_compute())
    };
    let carWalk = crate::utils::vehicle::sumo_vehicle_parser_helper::SUMOVehicleParserHelper::parse_car_walk_transfer(oc, true);
    let taxiWait = utils::common::time::steps2time(string2time(&utils::options::options_cont::get_string("persontrip.taxi.waiting-time")).unwrap_or_else(|e| panic!("{e}")));
    // shortcut: the RailwayRouter is not wired in duarouter yet (its effort operation is not a typed fn pointer), upgrade when railway routing is ported here
    let mut railRouter: Ptr<crate::utils::router::railway_router::RailwayRouter<ROEdge, ROVehicle>> = Ptr::null();
    if net.has_bidi_edges() {
        panic!("shortcut: the railway router is not wired in duarouter yet");
    }
    // shortcut: ROIntermodalRouter is not ported, so the intermodal router is null and net.write_intermodal writes nothing
    let _ = (carWalk, taxiWait);
    let pedRouter: Ptr<crate::utils::router::pedestrian_router::PedestrianRouter<ROEdge, ROLane, RONode, ROVehicle>> = Ptr::from_mut(Box::leak(Box::new(crate::utils::router::pedestrian_router::PedestrianRouter::new())));
    let provider: crate::router::ro_routable::RORouterProvider = crate::utils::router::router_provider::RouterProvider::new(router, pedRouter, Ptr::null(), railRouter);
    // process route definitions
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        net.open_output(oc);
        loader.process_routes(begin, end, string2time(&oc.get_string("route-steps").unwrap()).unwrap_or_else(|e| panic!("{e}")), net, &provider);
        net.write_intermodal(oc);
    }));
    net.cleanup();
    if let Err(payload) = result {
        std::panic::resume_unwind(payload);
    }
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
    let mut builder = crate::duarouter::rodua_edge_builder::RODUAEdgeBuilder::new();
    crate::router::ro_edge::ROEdge::set_global_options(oc.get_bool("weights.interpolate").unwrap());
    loader.load_net(net, &mut builder);
    // load the weights when wished/available
    if oc.is_set("weight-files", true).unwrap() {
        loader.load_weights(net, "weight-files", &oc.get_string("weight-attribute").unwrap(), false, oc.get_bool("weights.expand").unwrap());
    }
    if oc.is_set("lane-weight-files", true).unwrap() {
        loader.load_weights(net, "lane-weight-files", &oc.get_string("weight-attribute").unwrap(), true, oc.get_bool("weights.expand").unwrap());
    }
    if oc.get_bool("skip-new-routes").unwrap() {
        crate::router::ro_route_def::RORouteDef::set_skip_new();
    }
}

