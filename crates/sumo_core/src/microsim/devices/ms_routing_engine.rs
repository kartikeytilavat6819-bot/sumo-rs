//! Port of microsim/devices/MSRoutingEngine.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::sumo_vehicle_class::{SVC_BICYCLE, SVC_IGNORING, SVC_PASSENGER};
use utils::common::time::{steps2time, string2time};
use utils::options::options_cont::{get_bool, get_float, get_int, get_string, is_default, is_set, OptionsCont};
use utils::common::util_exceptions::ProcessError;
use crate::utils::router::dijkstra_router::DijkstraRouter;
use crate::utils::router::a_star_router::AStarRouter;
use crate::utils::router::ch_router::CHRouter;
use crate::utils::router::ch_router_wrapper::CHRouterWrapper;
use crate::utils::router::railway_router::RailwayRouter;
use crate::utils::router::router_provider::RouterProvider;
use crate::microsim::ms_router_defs::{MSMapMatcher, MSRouterProvider};

/// C++ `StaticCommand<MSRoutingEngine>` (utils/common/StaticCommand.h): runs a static function at a time step
pub struct MSRoutingEngine_StaticCommand {
    pub f: fn(SUMOTime) -> SUMOTime,
}

impl crate::microsim::ms_event_control::Command for MSRoutingEngine_StaticCommand {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        (self.f)(currentTime)
    }
}

/// C++ typedef `int MSRoutingEngine::Prohibitions`
pub type MSRoutingEngine_Prohibitions = i32;

/// C++ typedef `std::pair<SUMOTime, int> MSRoutingEngine::TimeAndCount`
pub type MSRoutingEngine_TimeAndCount = (SUMOTime, i32);

/// A device that performs vehicle rerouting based on current edge speeds
/// C++ class `MSRoutingEngine` (microsim/devices/MSRoutingEngine.h)
pub struct MSRoutingEngine {
}

// C++ static members of MSRoutingEngine (port as module-level statics, see AGENTS.md):
//   static int MSRoutingEngine::myEffortFunc;
//   static Command * MSRoutingEngine::myEdgeWeightSettingCommand;
//   static double MSRoutingEngine::myAdaptationWeight;
//   static SUMOTime MSRoutingEngine::myAdaptationInterval;
//   static SUMOTime MSRoutingEngine::myLastAdaptation;
//   static int MSRoutingEngine::myAdaptationSteps;
//   static int MSRoutingEngine::myAdaptationStepsIndex;
//   static int MSRoutingEngine::myEdgeSpeeds;
//   static int MSRoutingEngine::myEdgeBikeSpeeds;
//   static int MSRoutingEngine::myEdgeTravelTimes;
//   static int MSRoutingEngine::myPastEdgeSpeeds;
//   static int MSRoutingEngine::myPastEdgeBikeSpeeds;
//   static bool MSRoutingEngine::myWithTaz;
//   static bool MSRoutingEngine::myBikeSpeeds;
//   static MSRouterProvider * MSRoutingEngine::myRouterProvider;
//   static int MSRoutingEngine::myCachedRoutes;
//   static double MSRoutingEngine::myPriorityFactor;
//   static double MSRoutingEngine::myMinEdgePriority;
//   static double MSRoutingEngine::myEdgePriorityRange;
//   static bool MSRoutingEngine::myDynamicRandomness;
//   static bool MSRoutingEngine::myHaveExtras;
//   static MSCCHGraph * MSRoutingEngine::myCCHGraph;
//   static MSCCHMetricFamily * MSRoutingEngine::myCCHLive;
//   static MSCCHMetricFamily * MSRoutingEngine::myCCHFreeflow;

impl MSRoutingEngine {
    /// C++: `void MSRoutingEngine::initWeightConstants(const OptionsCont &)` [static]
    /// initialize constants for using myPriorityFactor
    // void
    // MSRoutingEngine::initWeightConstants(const OptionsCont& oc) {
    //     if (oc.getFloat("weights.priority-factor") != 0) {
    //         myPriorityFactor = oc.getFloat("weights.priority-factor");
    //         if (myPriorityFactor < 0) {
    //             throw ProcessError(TL("weights.priority-factor cannot be negative."));
    //         }
    //         myMinEdgePriority = std::numeric_limits<double>::max();
    //         double maxEdgePriority = -std::numeric_limits<double>::max();
    //         for (const MSEdge* const edge : MSNet::getInstance()->getEdgeControl().getEdges()) {
    //             maxEdgePriority = MAX2(maxEdgePriority, (double)edge->getPriority());
    //             myMinEdgePriority = MIN2(myMinEdgePriority, (double)edge->getPriority());
    //         }
    //         myEdgePriorityRange = maxEdgePriority - myMinEdgePriority;
    //         if (myEdgePriorityRange == 0) {
    //             WRITE_WARNING(TL("Option weights.priority-factor does not take effect because all edges have the same priority"));
    //             myPriorityFactor = 0;
    //         }
    //     }
    //     myDynamicRandomness = oc.getBool("weights.random-factor.dynamic");
    //     myHaveExtras = gRoutingPreferences || myPriorityFactor != 0 || gWeightsRandomFactor != 1;
    // }
    #[allow(unused_variables)]
    pub fn init_weight_constants(oc: &OptionsCont) {
        let pf = oc_float(oc, "weights.priority-factor");
        let mut st = state();
        if pf != 0. {
            st.myPriorityFactor = pf;
            if st.myPriorityFactor < 0. {
                std::panic::panic_any(ProcessError::new("weights.priority-factor cannot be negative."));
            }
            st.myMinEdgePriority = f64::MAX;
            let mut maxEdgePriority = -f64::MAX;
            let edges: Vec<Ptr<MSEdge>> = MSNet::get_instance().m().get_edge_control().get_edges().clone();
            for edge in edges.iter() {
                maxEdgePriority = max2(maxEdgePriority, edge.get_priority() as f64);
                st.myMinEdgePriority = min2(st.myMinEdgePriority, edge.get_priority() as f64);
            }
            st.myEdgePriorityRange = maxEdgePriority - st.myMinEdgePriority;
            if st.myEdgePriorityRange == 0. {
                utils::common::msg_handler::write_warning("Option weights.priority-factor does not take effect because all edges have the same priority");
                st.myPriorityFactor = 0.;
            }
        }
        st.myDynamicRandomness = oc_bool(oc, "weights.random-factor.dynamic");
        st.myHaveExtras = utils::common::std_defs::g_routing_preferences() || st.myPriorityFactor != 0. || weights_random_factor() != 1.;
    }

    /// C++: `void MSRoutingEngine::initWeightUpdate(SUMOTime)` [static]
    /// intialize period edge weight update
    // void
    // MSRoutingEngine::initWeightUpdate(SUMOTime lastAdaptation) {
    //     if (myAdaptationInterval == -1) {
    //         myEdgeWeightSettingCommand = nullptr;
    //         myLastAdaptation = lastAdaptation;
    //         const OptionsCont& oc = OptionsCont::getOptions();
    //         myWithTaz = oc.getBool("device.rerouting.with-taz");
    //         myAdaptationInterval = string2time(oc.getString("device.rerouting.adaptation-interval"));
    //         myAdaptationWeight = oc.getFloat("device.rerouting.adaptation-weight");
    //         const SUMOTime period = string2time(oc.getString("device.rerouting.period"));
    //         if (myAdaptationWeight < 1. && myAdaptationInterval > 0) {
    //             SUMOTime nextAdaptation = -1;
    //             if (lastAdaptation >= 0) {
    //                 nextAdaptation = lastAdaptation + myAdaptationInterval;
    //             }
    //             myEdgeWeightSettingCommand = new StaticCommand<MSRoutingEngine>(&MSRoutingEngine::adaptEdgeEfforts);
    //             MSNet::getInstance()->getEndOfTimestepEvents()->addEvent(myEdgeWeightSettingCommand, nextAdaptation);
    //         } else if (period > 0) {
    //             WRITE_WARNING(TL("Rerouting is useless if the edge weights do not get updated!"));
    //         }
    //         OutputDevice::createDeviceByOption("device.rerouting.output", "weights", "meandata_file.xsd");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_weight_update(lastAdaption: SUMOTime) {
        if state().myAdaptationInterval == -1 {
            state().myEdgeWeightSettingCommand = false;
            state().myLastAdaptation = lastAdaption;
            let withTaz = get_bool("device.rerouting.with-taz");
            let interval = string2time(&get_string("device.rerouting.adaptation-interval")).unwrap_or_else(|e| std::panic::panic_any(e));
            let weight = get_float("device.rerouting.adaptation-weight");
            let period = string2time(&get_string("device.rerouting.period")).unwrap_or_else(|e| std::panic::panic_any(e));
            {
                let mut st = state();
                st.myWithTaz = withTaz;
                st.myAdaptationInterval = interval;
                st.myAdaptationWeight = weight;
            }
            if weight < 1. && interval > 0 {
                let mut nextAdaptation: SUMOTime = -1;
                if lastAdaption >= 0 {
                    nextAdaptation = lastAdaption + interval;
                }
                let cmd: Ptr<dyn crate::microsim::ms_event_control::Command> = Ptr::from_raw(Box::into_raw(Box::new(MSRoutingEngine_StaticCommand { f: MSRoutingEngine::adapt_edge_efforts })) as *mut dyn crate::microsim::ms_event_control::Command);
                state().myEdgeWeightSettingCommand = true;
                MSNet::get_instance().m().get_end_of_timestep_events().m().add_event(cmd, nextAdaptation);
            } else if period > 0 {
                utils::common::msg_handler::write_warning("Rerouting is useless if the edge weights do not get updated!");
            }
            if let Err(e) = utils::iodevices::output_device::OutputDevice::create_device_by_option("device.rerouting.output", "weights", "meandata_file.xsd", 2) {
                std::panic::panic_any(e);
            }
        }
    }

    /// C++: `void MSRoutingEngine::initEdgeWeights(SUMOVehicleClass, SUMOTime, int)` [static]
    /// initialize the edge weights if not done before
    // void
    // MSRoutingEngine::initEdgeWeights(SUMOVehicleClass svc, SUMOTime lastAdaption, int index) {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     if (myAdaptationWeight == 0 || !oc.isDefault("device.rerouting.adaptation-steps")) {
    //         myAdaptationSteps = oc.getInt("device.rerouting.adaptation-steps");
    //     }
    //     if (myBikeSpeeds && svc == SVC_BICYCLE) {
    //         _initEdgeWeights(myEdgeBikeSpeeds, myPastEdgeBikeSpeeds);
    //     } else {
    //         _initEdgeWeights(myEdgeSpeeds, myPastEdgeSpeeds);
    //     }
    //     if (lastAdaption >= 0) {
    //         myLastAdaptation = lastAdaption;
    //     }
    //     if (index >= 0 && myAdaptationSteps > 0) {
    //         assert(index < myAdaptationSteps);
    //         myAdaptationStepsIndex = index;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_edge_weights(svc: SUMOVehicleClass, lastAdaption: SUMOTime, index: i32) {
        let mut st = state();
        if st.myAdaptationWeight == 0. || !is_default("device.rerouting.adaptation-steps") {
            st.myAdaptationSteps = get_int("device.rerouting.adaptation-steps");
        }
        let bike = st.myBikeSpeeds && svc == SVC_BICYCLE;
        drop(st);
        Self::init_edge_weights_2(bike);
        let mut st = state();
        if lastAdaption >= 0 {
            st.myLastAdaptation = lastAdaption;
        }
        if index >= 0 && st.myAdaptationSteps > 0 {
            assert!(index < st.myAdaptationSteps);
            st.myAdaptationStepsIndex = index;
        }
    }

    /// C++: `bool MSRoutingEngine::hasEdgeUpdates()` [static]
    /// returns whether any edge weight updates will take place
    // static bool hasEdgeUpdates() {
    //         return myEdgeWeightSettingCommand != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn has_edge_updates() -> bool {
        state().myEdgeWeightSettingCommand
    }

    /// C++: `SUMOTime MSRoutingEngine::getLastAdaptation()` [static]
    /// Information when the last edge weight adaptation occurred
    // static SUMOTime getLastAdaptation() {
    //         return myLastAdaptation;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_adaptation() -> SUMOTime {
        state().myLastAdaptation
    }

    /// C++: `bool MSRoutingEngine::haveExtras()` [static]
    // static bool haveExtras() {
    //         return myHaveExtras;
    //     }
    #[allow(unused_variables)]
    pub fn have_extras() -> bool {
        state().myHaveExtras
    }

    /// C++: `void MSRoutingEngine::applyExtras(const MSEdge *const, const SUMOVehicle *const, SUMOTime, double &)` [static]
    /// apply cost modifications from randomness, priorityFactor and preferences
    // static inline void applyExtras(const MSEdge* const e, const SUMOVehicle* const v, SUMOTime step, double& effort) {
    //         if (gWeightsRandomFactor != 1.) {
    //             long long int key = v->getRandomSeed() ^ e->getNumericalID();
    //             if (myDynamicRandomness) {
    //                 key ^= step;
    //             }
    //             effort *= (1 + RandHelper::randHash(key) * (gWeightsRandomFactor - 1));
    //         }
    //         if (myPriorityFactor != 0) {
    //             // lower priority should result in higher effort (and the edge with
    //             // minimum priority receives a factor of 1 + myPriorityFactor
    //             const double relativeInversePrio = 1 - ((e->getPriority() - myMinEdgePriority) / myEdgePriorityRange);
    //             effort *= 1 + relativeInversePrio * myPriorityFactor;
    //         }
    //         if (gRoutingPreferences) {
    //             effort /= MSNet::getInstance()->getPreference(e->getRoutingType(), v->getVTypeParameter());
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn apply_extras(e: Ptr<MSEdge>, v: Ptr<SUMOVehicle>, step: SUMOTime, effort: &mut f64) {
        if weights_random_factor() != 1. {
            let mut key: i64 = v_random_seed(v) ^ e.get_numerical_id() as i64;
            if state().myDynamicRandomness {
                key ^= step;
            }
            *effort *= 1. + utils::common::rand_helper::rand_hash(key) * (weights_random_factor() - 1.);
        }
        let (prioFactor, minPrio, prioRange) = {
            let st = state();
            (st.myPriorityFactor, st.myMinEdgePriority, st.myEdgePriorityRange)
        };
        if prioFactor != 0. {
            // lower priority should result in higher effort (and the edge with
            // minimum priority receives a factor of 1 + myPriorityFactor
            let relativeInversePrio = 1. - ((e.get_priority() as f64 - minPrio) / prioRange);
            *effort *= 1. + relativeInversePrio * prioFactor;
        }
        if utils::common::std_defs::g_routing_preferences() {
            let pref = MSNet::get_instance().get_preference(e.get_routing_type(), v_vtype_parameter(v));
            *effort /= pref;
        }
    }

    /// C++: `ConstMSRoutePtr MSRoutingEngine::getCachedRoute(const std::pair<const MSEdge *, const MSEdge *> &)` [static]
    /// return the cached route or nullptr on miss
    // ConstMSRoutePtr
    // MSRoutingEngine::getCachedRoute(const std::pair<const MSEdge*, const MSEdge*>& key) {
    // #ifdef HAVE_FOX
    //     // worker threads insert into the cache concurrently (RoutingTask::run)
    //     FXMutexLock lock(myRouteCacheMutex);
    // #endif
    //     auto routeIt = myCachedRoutes.find(key);
    //     if (routeIt != myCachedRoutes.end()) {
    //         return routeIt->second;
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_cached_route(key: &(Ptr<MSEdge>, Ptr<MSEdge>)) -> Option<ConstMSRoutePtr> {
        // shortcut: myCachedRoutes is never populated in this port (its writers are not ported), so a miss is the only answer, upgrade with RoutingTask
        let _ = key;
        None
    }

    /// C++: `const RoutingKit::CustomizableContractionHierarchyMetric * MSRoutingEngine::getPublishedCCHMetric(SUMOVehicleClass, SUMOTime, const SUMOVehicle *)` [static]
    /// the currently published (customized) CCH metric FOR A GIVEN vehicle class, or nullptr if that class has no CCH metric (then the caller falls back to A*). Lock-free: a plain atomic pointer read. The metric objects live for the whole run, so the raw pointer is always valid; the double buffer guarantees the pointer we hand out is not the one being customized. Called on the routing hot path -- allocation- and lock-free.
    // const RoutingKit::CustomizableContractionHierarchyMetric*
    // MSRoutingEngine::getPublishedCCHMetric(SUMOVehicleClass /* vClass */, SUMOTime /* time */, const SUMOVehicle* veh) {
    //     // the published metrics always track the live speeds; the query time
    //     // only matters for duarouter's per-weight-period metrics. Lookup is by
    //     // vehicle TYPE (the metric's exactness key); a vehicle-specific type
    //     // (TraCI-modified singular copy) routes on the exact fallback rather
    //     // than on another type's metric.
    //     if (veh == nullptr || myCCHLive == nullptr) {
    //         return nullptr;
    //     }
    //     const MSVehicleType* type = &veh->getVehicleType();
    //     if (type->isVehicleSpecific()) {
    //         return nullptr;
    //     }
    //     return myCCHLive->published(type, veh->getID());
    // }
    #[allow(unused_variables)]
    pub fn get_published_cch_metric(vClass: SUMOVehicleClass, time: SUMOTime, veh: Ptr<SUMOVehicle>) -> Ptr<Unmapped /* RoutingKit::CustomizableContractionHierarchyMetric: foreign::routingkit metric borrows the CCH (has a lifetime) */> {
        if veh.is_null() {
            return Ptr::null();
        }
        // shortcut: myCCHLive is never created in this port (initCCH is not ported), so C++ takes its nullptr path here, upgrade with the CCH port
        let _ = (vClass, time);
        Ptr::null()
    }

    /// C++: `const RoutingKit::CustomizableContractionHierarchyMetric * MSRoutingEngine::getFreeflowCCHMetric(SUMOVehicleClass, SUMOTime, const SUMOVehicle *)` [static]
    /// the free-flow CCH metric for MSNet's routers (TraCI / triggers / GUI), keyed by vehicle type and filled through MSNet::getTravelTime with a reference vehicle of the type. Free-flow efforts are static, so each metric customizes once; a runtime permission change re-customizes in place. Returns nullptr (-> exact A* fallback) whenever the query cannot be served by a shared metric: individual or global TraCI edge weights, a routing mode other than DEFAULT, or a vehicle-specific type. MAIN-THREAD ONLY -- MSNet's routers never run on the worker threads, which is what allows the synchronous lazy build and repair.
    // const RoutingKit::CustomizableContractionHierarchyMetric*
    // MSRoutingEngine::getFreeflowCCHMetric(SUMOVehicleClass /* vClass */, SUMOTime /* time */, const SUMOVehicle* veh) {
    //     // MAIN-THREAD ONLY (see header): MSNet's routers serve TraCI, triggers
    //     // and the GUI, never the rerouting worker threads, so the family's lazy
    //     // creation and stale repair may run synchronously right here.
    //     if (veh == nullptr) {
    //         return nullptr;
    //     }
    //     // Everything MSNet::getTravelTime reads ahead of the free-flow layer is
    //     // per-vehicle and cannot live in a shared metric -> exact fallback:
    //     // individual TraCI edge weights, global TraCI edge weights, a routing
    //     // mode other than DEFAULT (the AGGREGATED modes belong to the device
    //     // metrics; the transient-permission modes already fall back inside
    //     // CCHRouter::compute).
    //     if (veh->getRoutingMode() != libsumo::ROUTING_MODE_DEFAULT
    //             || !MSNet::getInstance()->getWeightsStorage().empty()) {
    //         return nullptr;
    //     }
    //     const MSVehicle* const msVeh = dynamic_cast<const MSVehicle*>(veh);
    //     if (msVeh != nullptr && !msVeh->getWeightsStorage().empty()) {
    //         return nullptr;
    //     }
    //     const MSVehicleType* type = &veh->getVehicleType();
    //     if (type->isVehicleSpecific()) {
    //         return nullptr;
    //     }
    //     if (myCCHFreeflow == nullptr) {
    //         // free-flow efforts are static: a STATIC family with a single weight
    //         // period, lazily customized per type on first query
    //         myCCHFreeflow = new MSCCHMetricFamily(
    //             ensureCCHGraph(), &MSNet::getTravelTime, 0, SUMOTime_MAX,
    //             &MSRoutingEngine::buildCCHRefVehicle, nullptr);
    //     }
    //     return myCCHFreeflow->get(type, type->getVehicleClass(),
    //                               MSNet::getInstance()->getCurrentTimeStep(), veh);
    // }
    #[allow(unused_variables)]
    pub fn get_freeflow_cch_metric(vClass: SUMOVehicleClass, time: SUMOTime, veh: Ptr<SUMOVehicle>) -> Ptr<Unmapped /* RoutingKit::CustomizableContractionHierarchyMetric: foreign::routingkit metric borrows the CCH (has a lifetime) */> {
        if veh.is_null() {
            return Ptr::null();
        }
        // shortcut: CCH graph and metric families are not ported yet, upgrade with the CCH portthe routing-mode and vehicle-specific checks and the creation of myCCHFreeflow need the CCH graph
        let _ = (vClass, time);
        panic!("shortcut: getFreeflowCCHMetric needs the CCH graph (ensureCCHGraph)")
    }

    /// C++: `MSCCHGraph * MSRoutingEngine::ensureCCHGraph()` [static]
    /// the shared CCH topology, built on first demand (used by MSNet::getRouterTT to construct its CCH router; the device path builds it through initCCH)
    // MSCCHGraph*
    // MSRoutingEngine::ensureCCHGraph() {
    //     if (myCCHGraph == nullptr) {
    //         myCCHGraph = new MSCCHGraph(MSEdge::getAllEdges());  // union topology; class masks prime at first fill
    //     }
    //     return myCCHGraph;
    // }
    #[allow(unused_variables)]
    pub fn ensure_cch_graph() -> Ptr<MSCCHGraph> {
        // shortcut: CCH graph and metric families are not ported yet, upgrade with the CCH portensureCCHGraph builds the shared CCH topology
        panic!("shortcut: ensureCCHGraph is not ported")
    }

    /// C++: `void MSRoutingEngine::initRouter(SUMOVehicle *)` [static]
    // void
    // MSRoutingEngine::initRouter(SUMOVehicle* vehicle) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     const std::string routingAlgorithm = oc.getString("routing-algorithm");
    //     const bool hasPermissions = MSNet::getInstance()->hasPermissions();
    //     myBikeSpeeds = oc.getBool("device.rerouting.bike-speeds");
    //     myEffortFunc = ((myHaveExtras || myBikeSpeeds) ? &MSRoutingEngine::getEffortExtra : &MSRoutingEngine::getEffort);
    //
    //     SUMOAbstractRouter<MSEdge, SUMOVehicle>* router = nullptr;
    //     if (routingAlgorithm == "dijkstra") {
    //         router = new DijkstraRouter<MSEdge, SUMOVehicle>(MSEdge::getAllEdges(), true, myEffortFunc, nullptr, false, nullptr, true);
    //     } else if (routingAlgorithm == "astar") {
    //         typedef AStarRouter<MSEdge, SUMOVehicle, MSMapMatcher> AStar;
    //         std::shared_ptr<const AStar::LookupTable> lookup = nullptr;
    //         if (oc.isSet("astar.all-distances")) {
    //             lookup = std::make_shared<const AStar::FLT>(oc.getString("astar.all-distances"), (int)MSEdge::getAllEdges().size());
    //         } else if (oc.isSet("astar.landmark-distances") && vehicle != nullptr) {
    //             const double speedFactor = vehicle->getChosenSpeedFactor();
    //             // we need an exemplary vehicle with speedFactor 1
    //             vehicle->setChosenSpeedFactor(1);
    //             CHRouterWrapper<MSEdge, SUMOVehicle> chrouter(
    //                 MSEdge::getAllEdges(), true, &MSNet::getTravelTime,
    //                 string2time(oc.getString("begin")), string2time(oc.getString("end")), SUMOTime_MAX, hasPermissions, 1);
    //             lookup = std::make_shared<const AStar::LMLT>(oc.getString("astar.landmark-distances"), MSEdge::getAllEdges(), &chrouter,
    //                      nullptr, vehicle, "", oc.getInt("device.rerouting.threads"), MSNet::getInstance()->getMapMatcher());
    //             vehicle->setChosenSpeedFactor(speedFactor);
    //         }
    //         router = new AStar(MSEdge::getAllEdges(), true, myEffortFunc, lookup, true);
    //     } else if (routingAlgorithm == "CH" && !hasPermissions) {
    //         const SUMOTime weightPeriod = myAdaptationInterval > 0 ? myAdaptationInterval : SUMOTime_MAX;
    //         router = new CHRouter<MSEdge, SUMOVehicle>(
    //             MSEdge::getAllEdges(), true, myEffortFunc, vehicle == nullptr ? SVC_PASSENGER : vehicle->getVClass(), weightPeriod, true, false);
    //     } else if (routingAlgorithm == "CHWrapper" || routingAlgorithm == "CH") {
    //         // use CHWrapper instead of CH if the net has permissions
    //         const SUMOTime weightPeriod = myAdaptationInterval > 0 ? myAdaptationInterval : SUMOTime_MAX;
    //         router = new CHRouterWrapper<MSEdge, SUMOVehicle>(
    //             MSEdge::getAllEdges(), true, myEffortFunc,
    //             string2time(oc.getString("begin")), string2time(oc.getString("end")), weightPeriod, hasPermissions, oc.getInt("device.rerouting.threads"));
    //     } else if (routingAlgorithm == "CCH") {
    //         // CCH metrics are keyed by vehicle TYPE and filled with a reference
    //         // vehicle of the type (customizeCCH), mirroring duarouter's
    //         // RODUACCHMetrics: type/class-specific routing preferences, the
    //         // bicycle speed table, the type's maximum speed and the static
    //         // priority multiplier are captured exactly per metric.
    //         // weights.random-factor freezes one realization per metric -- the
    //         // same approximation CHRouterWrapper makes when building its
    //         // hierarchies; exact per-vehicle randomization remains the domain of
    //         // dijkstra and astar.
    //         initCCH();  // build the immutable topology + publish an initial metric (once)
    //         // embedded fallback for non-passenger / prohibited / unreachable queries
    //         SUMOAbstractRouter<MSEdge, SUMOVehicle>* fallback =
    //             new AStarRouter<MSEdge, SUMOVehicle, MSMapMatcher>(MSEdge::getAllEdges(), true, myEffortFunc, nullptr, true);
    //         router = new CCHRouter<MSEdge, SUMOVehicle, MSCCHGraph>(
    //             myCCHGraph, &MSRoutingEngine::getPublishedCCHMetric, myEffortFunc, true, fallback);
    //     } else {
    //         throw ProcessError(TLF("Unknown routing algorithm '%'!", routingAlgorithm));
    //     }
    //
    //     RailwayRouter<MSEdge, SUMOVehicle>* railRouter = nullptr;
    //     if (MSNet::getInstance()->hasBidiEdges()) {
    //         railRouter = new RailwayRouter<MSEdge, SUMOVehicle>(MSEdge::getAllEdges(), true, myEffortFunc, nullptr, false, true, false,
    //                 oc.getFloat("railway.max-train-length"),
    //                 oc.getFloat("weights.reversal-penalty"));
    //     }
    //     const int carWalk = SUMOVehicleParserHelper::parseCarWalkTransfer(oc, MSDevice_Taxi::hasFleet() || MSNet::getInstance()->getInsertionControl().hasTaxiFlow());
    //     const double taxiWait = STEPS2TIME(string2time(OptionsCont::getOptions().getString("persontrip.taxi.waiting-time")));
    //     MSTransportableRouter* transRouter = new MSTransportableRouter(MSNet::adaptIntermodalRouter, carWalk, taxiWait, routingAlgorithm, 0);
    //     myRouterProvider = new MSRouterProvider(router, nullptr, transRouter, railRouter);
    // #ifndef THREAD_POOL
    // #ifdef HAVE_FOX
    //     MFXWorkerThread::Pool& threadPool = MSNet::getInstance()->getEdgeControl().getThreadPool();
    //     if (threadPool.size() > 0) {
    //         const std::vector<MFXWorkerThread*>& threads = threadPool.getWorkers();
    //         if (static_cast<MSEdgeControl::WorkerThread*>(threads.front())->setRouterProvider(myRouterProvider)) {
    //             for (std::vector<MFXWorkerThread*>::const_iterator t = threads.begin() + 1; t != threads.end(); ++t) {
    //                 static_cast<MSEdgeControl::WorkerThread*>(*t)->setRouterProvider(myRouterProvider->clone());
    //             }
    //         }
    //     }
    // #endif
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn init_router(vehicle: Ptr<SUMOVehicle>) {
        let routingAlgorithm = get_string("routing-algorithm");
        let hasPermissions = MSNet::get_instance().m().has_permissions();
        let bikeSpeeds = get_bool("device.rerouting.bike-speeds");
        let effortIsExtra = {
            let mut st = state();
            st.myBikeSpeeds = bikeSpeeds;
            st.myEffortFuncIsExtra = st.myHaveExtras || bikeSpeeds;
            st.myEffortFuncIsExtra
        };
        let effort: SUMOAbstractRouter_TypedOperation<MSEdge, SUMOVehicle> = if effortIsExtra { MSRoutingEngine::get_effort_extra } else { MSRoutingEngine::get_effort };
        let edges = MSEdge::get_all_edges();
        let router: Ptr<SUMOAbstractRouter<MSEdge, SUMOVehicle>> = if routingAlgorithm == "dijkstra" {
            let router = DijkstraRouter::<MSEdge, SUMOVehicle>::new(edges, true, Some(effort), None, false, Ptr::null(), true, false).with_compute();
            Ptr::from_raw(Box::into_raw(Box::new(router))).cast()
        } else if routingAlgorithm == "astar" {
            // shortcut: the astar.all-distances and astar.landmark-distances lookup tables are not ported, a run that sets them panics instead of routing without them
            if is_set("astar.all-distances") || is_set("astar.landmark-distances") {
                panic!("shortcut: astar lookup tables (astar.all-distances, astar.landmark-distances) are not ported")
            }
            let router = AStarRouter::<MSEdge, SUMOVehicle, MSMapMatcher>::new(edges, true, Some(effort), None, true, false).with_compute();
            Ptr::from_raw(Box::into_raw(Box::new(router))).cast()
        } else if routingAlgorithm == "CH" && !hasPermissions {
            let weightPeriod = if state().myAdaptationInterval > 0 { state().myAdaptationInterval } else { utils::common::time::SUMOTIME_MAX };
            let svc = if vehicle.is_null() { SVC_PASSENGER } else { v_vclass(vehicle) };
            let router = CHRouter::<MSEdge, SUMOVehicle>::new(edges, true, Some(effort), svc, weightPeriod, true, false).with_compute();
            Ptr::from_raw(Box::into_raw(Box::new(router))).cast()
        } else if routingAlgorithm == "CHWrapper" || routingAlgorithm == "CH" {
            // use CHWrapper instead of CH if the net has permissions
            let weightPeriod = if state().myAdaptationInterval > 0 { state().myAdaptationInterval } else { utils::common::time::SUMOTIME_MAX };
            let router = CHRouterWrapper::<MSEdge, SUMOVehicle>::new(edges, true, Some(effort), string2time(&get_string("begin")).unwrap_or_else(|e| std::panic::panic_any(e)), string2time(&get_string("end")).unwrap_or_else(|e| std::panic::panic_any(e)), weightPeriod, hasPermissions, get_int("device.rerouting.threads")).with_compute();
            Ptr::from_raw(Box::into_raw(Box::new(router))).cast()
        } else if routingAlgorithm == "CCH" {
            // shortcut: CCH routers need RoutingKit (CCHGraph, CCHMetricFamily), not ported
            panic!("shortcut: routing algorithm 'CCH' is not ported")
        } else {
            std::panic::panic_any(ProcessError::new(&format!("Unknown routing algorithm '{}'!", routingAlgorithm)))
        };
        let railRouter: Ptr<RailwayRouter<MSEdge, SUMOVehicle>> = if MSNet::get_instance().m().has_bidi_edges() {
            let router = RailwayRouter::<MSEdge, SUMOVehicle>::new(edges, true, Some(effort), None, false, true, false, get_float("railway.max-train-length"), get_float("weights.reversal-penalty")).with_compute();
            Ptr::from_raw(Box::into_raw(Box::new(router)))
        } else {
            Ptr::null()
        };
        // shortcut: MSTransportableRouter (pedestrian and intermodal routing) is not ported, the provider holds no pedestrian or intermodal router, upgrade with the transportable port
        let provider: MSRouterProvider = RouterProvider::new(router, Ptr::null(), Ptr::null(), railRouter);
        ROUTER_PROVIDER.with(|p| p.set(Some(Ptr::from_raw(Box::into_raw(Box::new(provider))))));
    }

    /// C++: `void MSRoutingEngine::reroute(SUMOVehicle &, const SUMOTime, const std::string &, const bool, const bool, const Prohibitions &)` [static]
    /// initiate the rerouting, create router / thread pool on first use
    // void
    // MSRoutingEngine::reroute(SUMOVehicle& vehicle, const SUMOTime currentTime, const std::string& info,
    //                          const bool onInit, const bool silent, const Prohibitions& prohibited) {
    //     if (myRouterProvider == nullptr) {
    //         initRouter(&vehicle);
    //     }
    //     auto& router = myRouterProvider->getVehicleRouter(vehicle.getVClass());
    // #ifndef THREAD_POOL
    // #ifdef HAVE_FOX
    //     MFXWorkerThread::Pool& threadPool = MSNet::getInstance()->getEdgeControl().getThreadPool();
    //     if (threadPool.size() > 0) {
    //         threadPool.add(new RoutingTask(vehicle, currentTime, info, onInit, silent, prohibited));
    //         return;
    //     }
    // #endif
    // #endif
    //     if (!prohibited.empty()) {
    //         router.prohibit(prohibited);
    //     }
    //     try {
    //         vehicle.reroute(currentTime, info, router, onInit, myWithTaz, silent);
    //     } catch (ProcessError&) {
    //         if (!silent) {
    //             if (!prohibited.empty()) {
    //                 router.prohibit(Prohibitions());
    //             }
    //             throw;
    //         }
    //     }
    //     if (!prohibited.empty()) {
    //         router.prohibit(Prohibitions());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reroute(vehicle: &mut SUMOVehicle, currentTime: SUMOTime, info: &str, onInit: bool, silent: bool, prohibited: &SUMOAbstractRouter_Prohibitions) {
        if ROUTER_PROVIDER.with(|p| p.get()).is_none() {
            Self::init_router(Ptr::from_mut(vehicle));
        }
        let provider = ROUTER_PROVIDER.with(|p| p.get()).unwrap();
        let router = provider.get_mut().unwrap().get_vehicle_router(vehicle.get_v_class());
        if !prohibited.is_empty() {
            router.prohibit(prohibited);
        }
        // shortcut: C++ hands the task to the thread pool when one is configured; this port always routes synchronously, and a silent reroute does not swallow ProcessError (no ported caller passes silent=true), upgrade with the thread pool
        vehicle.reroute(currentTime, info, router, onInit, Self::with_taz(), silent, Ptr::default());
        if !prohibited.is_empty() {
            router.prohibit(&Default::default());
        }
    }

    /// C++: `void MSRoutingEngine::reroute(MSTransportable &, const SUMOTime, const std::string &, const bool, const bool, const Prohibitions &)` [static]
    /// initiate the person rerouting, create router / thread pool on first use
    // void
    // MSRoutingEngine::reroute(MSTransportable& t, const SUMOTime currentTime, const std::string& info,
    //                          const bool onInit, const bool silent, const Prohibitions& prohibited) {
    //     MSTransportableRouter& router = getIntermodalRouterTT(t.getRNGIndex(), prohibited);
    // #ifndef THREAD_POOL
    // #ifdef HAVE_FOX
    //     MFXWorkerThread::Pool& threadPool = MSNet::getInstance()->getEdgeControl().getThreadPool();
    //     if (threadPool.size() > 0) {
    //         // threadPool.add(new RoutingTask(t, currentTime, info, onInit, silent, prohibited));
    //         return;
    //     }
    // #endif
    // #endif
    //     if (!prohibited.empty()) {
    //         router.prohibit(prohibited);
    //     }
    //     try {
    //         t.reroute(currentTime, info, router, onInit, myWithTaz, silent);
    //     } catch (ProcessError&) {
    //         if (!silent) {
    //             if (!prohibited.empty()) {
    //                 router.prohibit(Prohibitions());
    //             }
    //             throw;
    //         }
    //     }
    //     if (!prohibited.empty()) {
    //         router.prohibit(Prohibitions());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reroute_2(t: &mut MSTransportable, currentTime: SUMOTime, info: &str, onInit: bool, silent: bool, prohibited: &SUMOAbstractRouter_Prohibitions) {
        // shortcut: MSRouterProvider and the router types are not ported yet, upgrade with MSRouterProviderreroute needs getIntermodalRouterTT and MSTransportable::reroute
        let _ = (t, currentTime, info, onInit, silent, prohibited);
        panic!("shortcut: reroute (person) needs MSTransportableRouter")
    }

    /// C++: `void MSRoutingEngine::setEdgeTravelTime(const MSEdge *const, const double)` [static]
    /// adapt the known travel time for an edge
    // void
    // MSRoutingEngine::setEdgeTravelTime(const MSEdge* const edge, const double travelTime) {
    //     myEdgeSpeeds[edge->getNumericalID()] = edge->getLength() / travelTime;
    // }
    #[allow(unused_variables)]
    pub fn set_edge_travel_time(edge: Ptr<MSEdge>, travelTime: f64) {
        let id = edge.get_numerical_id() as usize;
        let len = edge.get_length();
        state().myEdgeSpeeds[id] = len / travelTime;
    }

    /// C++: `void MSRoutingEngine::cleanup()` [static]
    /// deletes the router instance
    // void
    // MSRoutingEngine::cleanup() {
    //     myAdaptationInterval = -1; // responsible for triggering initEdgeWeights
    //     myPastEdgeSpeeds.clear();
    //     myEdgeSpeeds.clear();
    //     myEdgeTravelTimes.clear();
    //     myPastEdgeBikeSpeeds.clear();
    //     myEdgeBikeSpeeds.clear();
    //     // @todo recheck. calling release crashes in parallel routing
    //     //for (auto& item : myCachedRoutes) {
    //     //    item.second->release();
    //     //}
    //     {
    // #ifdef HAVE_FOX
    //         FXMutexLock lock(myRouteCacheMutex);
    // #endif
    //         myCachedRoutes.clear();
    //     }
    //     myAdaptationStepsIndex = 0;
    //     cleanupCCH();
    // #ifdef HAVE_FOX
    //     if (MSGlobals::gNumThreads > 1) {
    //         // router deletion is done in thread destructor
    //         myRouterProvider = nullptr;
    //         return;
    //     }
    // #endif
    //     delete myRouterProvider;
    //     myRouterProvider = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        {
            let mut st = state();
            st.myAdaptationInterval = -1; // responsible for triggering initEdgeWeights
            st.myPastEdgeSpeeds.clear();
            st.myEdgeSpeeds.clear();
            st.myEdgeTravelTimes.clear();
            st.myPastEdgeBikeSpeeds.clear();
            st.myEdgeBikeSpeeds.clear();
            st.myAdaptationStepsIndex = 0;
        }
        // myCachedRoutes.clear(): the route cache is never populated in this port
        Self::cleanup_cch();
        // shortcut: the provider and its routers are leaked, not deleted (C++ delete), upgrade when the routers have a common owner
        ROUTER_PROVIDER.with(|p| p.set(None));
    }

    /// C++: `void MSRoutingEngine::cleanupCCH()` [static]
    /// tears down the CCH state (myCCHLive/myCCHFreeflow/myCCHGraph) only -- the ref vehicles owned by the CCH metric families hold a raw MSVehicleType* that MSVehicleControl frees, so this MUST run before MSNet deletes its MSVehicleControl (cleanup() itself runs far later, from MSNet::clearAll(), so this is called separately and first; it is also idempotent/safe to call again from cleanup() afterwards since it nulls out the pointers it deletes)
    // void
    // MSRoutingEngine::cleanupCCH() {
    //     // free the CCH state so a subsequent load (libsumo / GUI reload) rebuilds
    //     // it against the new network; the router clones referencing it were
    //     // deleted together with the worker threads / router provider.
    //     // Deleting the metric families' owned ref vehicles dereferences their
    //     // MSVehicleType*, so this must run BEFORE MSNet deletes its
    //     // MSVehicleControl (see the call in ~MSNet); the nullptr checks below
    //     // make a second call from cleanup() a safe no-op.
    //     delete myCCHLive;
    //     myCCHLive = nullptr;
    //     delete myCCHFreeflow;
    //     myCCHFreeflow = nullptr;
    //     delete myCCHGraph;
    //     myCCHGraph = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup_cch() {
        // shortcut: the CCH graph and families are never created in this port (ensureCCHGraph and initCCH are not ported), so there is nothing to free, upgrade with the CCH port
    }

    /// C++: `bool MSRoutingEngine::isEnabled()` [static]
    /// returns whether any routing actions take place
    // static bool isEnabled() {
    //         return !myWithTaz && myAdaptationInterval >= 0;
    //     }
    #[allow(unused_variables)]
    pub fn is_enabled() -> bool {
        let st = state();
        !st.myWithTaz && st.myAdaptationInterval >= 0
    }

    /// C++: `MSVehicleRouter & MSRoutingEngine::getRouterTT(const int, SUMOVehicleClass, const Prohibitions &)` [static]
    /// return the vehicle router instance
    // MSVehicleRouter&
    // MSRoutingEngine::getRouterTT(const int rngIndex, SUMOVehicleClass svc, const Prohibitions& prohibited) {
    //     if (myRouterProvider == nullptr) {
    //         initWeightUpdate();
    //         initEdgeWeights(svc);
    //         initRouter();
    //     }
    // #ifndef THREAD_POOL
    // #ifdef HAVE_FOX
    //     MFXWorkerThread::Pool& threadPool = MSNet::getInstance()->getEdgeControl().getThreadPool();
    //     if (threadPool.size() > 0) {
    //         auto& router = static_cast<MSEdgeControl::WorkerThread*>(threadPool.getWorkers()[rngIndex % MSGlobals::gNumThreads])->getRouter(svc);
    //         router.prohibit(prohibited);
    //         return router;
    //     }
    // #else
    //     UNUSED_PARAMETER(rngIndex);
    // #endif
    // #endif
    //     myRouterProvider->getVehicleRouter(svc).prohibit(prohibited);
    //     return myRouterProvider->getVehicleRouter(svc);
    // }
    #[allow(unused_variables)]
    pub fn get_router_tt(rngIndex: i32, svc: SUMOVehicleClass, prohibited: &SUMOAbstractRouter_Prohibitions) -> &'static mut MSVehicleRouter {
        if ROUTER_PROVIDER.with(|p| p.get()).is_none() {
            Self::init_weight_update(-1);
            Self::init_edge_weights(svc, -1, -1);
            Self::init_router(Ptr::null());
        }
        let provider = ROUTER_PROVIDER.with(|p| p.get()).unwrap();
        let router = provider.get_mut().unwrap().get_vehicle_router(svc);
        router.prohibit(prohibited);
        router
    }

    /// C++: `MSTransportableRouter & MSRoutingEngine::getIntermodalRouterTT(const int, const Prohibitions &)` [static]
    /// return the person router instance
    // MSTransportableRouter&
    // MSRoutingEngine::getIntermodalRouterTT(const int rngIndex, const Prohibitions& prohibited) {
    //     if (myRouterProvider == nullptr) {
    //         initWeightUpdate();
    //         initEdgeWeights(SVC_PEDESTRIAN);
    //         initRouter();
    //     }
    // #ifndef THREAD_POOL
    // #ifdef HAVE_FOX
    //     MFXWorkerThread::Pool& threadPool = MSNet::getInstance()->getEdgeControl().getThreadPool();
    //     if (threadPool.size() > 0) {
    //         auto& router = static_cast<MSEdgeControl::WorkerThread*>(threadPool.getWorkers()[rngIndex % MSGlobals::gNumThreads])->getIntermodalRouter();
    //         router.prohibit(prohibited);
    //         return router;
    //     }
    // #else
    //     UNUSED_PARAMETER(rngIndex);
    // #endif
    // #endif
    //     myRouterProvider->getIntermodalRouter().prohibit(prohibited);
    //     return myRouterProvider->getIntermodalRouter();
    // }
    #[allow(unused_variables)]
    pub fn get_intermodal_router_tt(rngIndex: i32, prohibited: &SUMOAbstractRouter_Prohibitions) -> &'static mut MSTransportableRouter {
        // shortcut: myRouterProvider is never constructed (initRouter is a stub), so there is no MSTransportableRouter to return; upgrade when initRouter builds the RouterProvider
        let _ = (rngIndex, prohibited);
        panic!("shortcut: getIntermodalRouterTT needs myRouterProvider, which initRouter never builds")
    }

    /// C++: `bool MSRoutingEngine::hasBikeSpeeds()` [static]
    /// whether the router collects bicycle speeds
    // static bool hasBikeSpeeds() {
    //         return myBikeSpeeds;
    //     }
    #[allow(unused_variables)]
    pub fn has_bike_speeds() -> bool {
        state().myBikeSpeeds
    }

    /// C++: `double MSRoutingEngine::getEffort(const MSEdge *const, const SUMOVehicle *const, double)` [static]
    /// Returns the effort to pass an edge
    // double
    // MSRoutingEngine::getEffort(const MSEdge* const e, const SUMOVehicle* const v, double) {
    //     const int id = e->getNumericalID();
    //     if (id < (int)myEdgeSpeeds.size()) {
    //         return MAX2(e->getLength() / MAX2(myEdgeSpeeds[id], NUMERICAL_EPS), e->getMinimumTravelTime(v));
    //     }
    //     return e->getMinimumTravelTime(v);
    // }
    #[allow(unused_variables)]
    pub fn get_effort(e: Ptr<MSEdge>, v: Ptr<SUMOVehicle>, t: f64) -> f64 {
        let id = e.get_numerical_id();
        let speed = {
            let st = state();
            if id < st.myEdgeSpeeds.len() as i32 { Some(st.myEdgeSpeeds[id as usize]) } else { None }
        };
        match speed {
            Some(s) => max2(e.get_length() / max2(s, utils::geom::NUMERICAL_EPS), e.get_minimum_travel_time(v)),
            None => e.get_minimum_travel_time(v),
        }
    }

    /// C++: `double MSRoutingEngine::getEffortBike(const MSEdge *const, const SUMOVehicle *const, double)` [static]
    // double
    // MSRoutingEngine::getEffortBike(const MSEdge* const e, const SUMOVehicle* const v, double) {
    //     const int id = e->getNumericalID();
    //     if (id < (int)myEdgeBikeSpeeds.size()) {
    //         return MAX2(e->getLength() / MAX2(myEdgeBikeSpeeds[id], NUMERICAL_EPS), e->getMinimumTravelTime(v));
    //     }
    //     return e->getMinimumTravelTime(v);
    // }
    #[allow(unused_variables)]
    pub fn get_effort_bike(e: Ptr<MSEdge>, v: Ptr<SUMOVehicle>, t: f64) -> f64 {
        let id = e.get_numerical_id();
        let speed = {
            let st = state();
            if id < st.myEdgeBikeSpeeds.len() as i32 { Some(st.myEdgeBikeSpeeds[id as usize]) } else { None }
        };
        match speed {
            Some(s) => max2(e.get_length() / max2(s, utils::geom::NUMERICAL_EPS), e.get_minimum_travel_time(v)),
            None => e.get_minimum_travel_time(v),
        }
    }

    /// C++: `double MSRoutingEngine::getEffortExtra(const MSEdge *const, const SUMOVehicle *const, double)` [static]
    // double
    // MSRoutingEngine::getEffortExtra(const MSEdge* const e, const SUMOVehicle* const v, double t) {
    //     double effort = (!myBikeSpeeds || v == nullptr || v->getVClass() != SVC_BICYCLE
    //                      ? getEffort(e, v, t)
    //                      : getEffortBike(e, v, t));
    //     applyExtras(e, v, SIMSTEP, effort);
    //     return effort;
    // }
    #[allow(unused_variables)]
    pub fn get_effort_extra(e: Ptr<MSEdge>, v: Ptr<SUMOVehicle>, t: f64) -> f64 {
        let bike = state().myBikeSpeeds;
        let mut effort = if !bike || v.is_null() || v_vclass(v) != SVC_BICYCLE {
            Self::get_effort(e, v, t)
        } else {
            Self::get_effort_bike(e, v, t)
        };
        Self::apply_extras(e, v, MSNet::get_instance().get_current_time_step(), &mut effort);
        effort
    }

    /// C++: `double MSRoutingEngine::getAssumedSpeed(const MSEdge *, const SUMOVehicle *)` [static]
    /// return current travel speed assumption
    // double
    // MSRoutingEngine::getAssumedSpeed(const MSEdge* edge, const SUMOVehicle* veh) {
    //     return edge->getLength() / myEffortFunc(edge, veh, 0);
    // }
    #[allow(unused_variables)]
    pub fn get_assumed_speed(edge: Ptr<MSEdge>, veh: Ptr<SUMOVehicle>) -> f64 {
        e_len_of(edge) / effort_func(edge, veh, 0.)
    }

    /// C++: `bool MSRoutingEngine::withTaz()` [static]
    /// whether taz-routing is enabled
    // static bool withTaz() {
    //         return myWithTaz;
    //     }
    #[allow(unused_variables)]
    pub fn with_taz() -> bool {
        state().myWithTaz
    }

    /// C++: `void MSRoutingEngine::addEdgeTravelTime(const MSEdge &, const SUMOTime)` [static]
    /// record actual travel time for an edge
    // void
    // MSRoutingEngine::addEdgeTravelTime(const MSEdge& edge, const SUMOTime travelTime) {
    //     TimeAndCount& tc = myEdgeTravelTimes[edge.getNumericalID()];
    //     tc.first += travelTime;
    //     tc.second += 1;
    // }
    #[allow(unused_variables)]
    pub fn add_edge_travel_time(edge: &MSEdge, travelTime: SUMOTime) {
        let id = edge.get_numerical_id() as usize;
        let mut st = state();
        let tc = &mut st.myEdgeTravelTimes[id];
        tc.0 += travelTime;
        tc.1 += 1;
    }

    /// C++: `void MSRoutingEngine::saveState(OutputDevice &)` [static]
    /// Saves the state (i.e. recorded speeds)
    // void
    // MSRoutingEngine::saveState(OutputDevice& out) {
    //     if (myEdgeSpeeds.size() == 0) {
    //         return;
    //     }
    //     out.openTag(SUMO_TAG_ROUTINGENGINE);
    //     out.writeAttr(SUMO_ATTR_LAST, myLastAdaptation);
    //     out.writeAttr(SUMO_ATTR_INDEX, myAdaptationStepsIndex);
    //     const MSEdgeVector& edges = MSNet::getInstance()->getEdgeControl().getEdges();
    //     for (const MSEdge* const e : edges) {
    //         if (e->isDelayed()) {
    //             const int id = e->getNumericalID();
    //             out.openTag(SUMO_TAG_EDGE);
    //             out.writeAttr(SUMO_ATTR_ID, e->getID());
    //             out.writeAttr(SUMO_ATTR_SPEED, myEdgeSpeeds[id]);
    //             if (myAdaptationSteps > 0) {
    //                 out.writeAttr(SUMO_ATTR_PASTSPEED, myPastEdgeSpeeds[id]);
    //             }
    //             if (myBikeSpeeds) {
    //                 out.writeAttr(SUMO_ATTR_BIKESPEED, myEdgeBikeSpeeds[id]);
    //                 if (myAdaptationSteps > 0) {
    //                     out.writeAttr(SUMO_ATTR_PASTBIKESPEED, myPastEdgeBikeSpeeds[id]);
    //                 }
    //             }
    //             out.closeTag();
    //         }
    //     }
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(out: &mut OutputDevice) {
        let st = state();
        if st.myEdgeSpeeds.len() == 0 {
            return;
        }
        out.open_tag_id(SUMO_TAG_ROUTINGENGINE);
        out.write_attr(SUMO_ATTR_LAST, &st.myLastAdaptation);
        out.write_attr(SUMO_ATTR_INDEX, &st.myAdaptationStepsIndex);
        let edges: Vec<Ptr<MSEdge>> = MSNet::get_instance().m().get_edge_control().get_edges().clone();
        for e in edges.iter() {
            if e.is_delayed() {
                let id = e.get_numerical_id() as usize;
                out.open_tag_id(SUMO_TAG_EDGE);
                out.write_attr(SUMO_ATTR_ID, e.get_id());
                out.write_attr(SUMO_ATTR_SPEED, &st.myEdgeSpeeds[id]);
                if st.myAdaptationSteps > 0 {
                    out.write_attr(SUMO_ATTR_PASTSPEED, &utils::common::to_string::join_to_string(&st.myPastEdgeSpeeds[id], " "));
                }
                if st.myBikeSpeeds {
                    out.write_attr(SUMO_ATTR_BIKESPEED, &st.myEdgeBikeSpeeds[id]);
                    if st.myAdaptationSteps > 0 {
                        out.write_attr(SUMO_ATTR_PASTBIKESPEED, &utils::common::to_string::join_to_string(&st.myPastEdgeBikeSpeeds[id], " "));
                    }
                }
                out.close_tag("");
            }
        }
        out.close_tag("");
    }

    /// C++: `void MSRoutingEngine::loadState(const SUMOSAXAttributes &)` [static]
    /// Loads the state
    // void
    // MSRoutingEngine::loadState(const SUMOSAXAttributes& attrs) {
    //     const MSEdge* const e = MSEdge::dictionary(attrs.getString(SUMO_ATTR_ID));
    //     e->markDelayed();
    //     const int id = e->getNumericalID();
    //     bool checkedSteps = false;
    //     bool checkedBikeSpeeds = false;
    //     bool ok = true;
    //     if ((int)myEdgeSpeeds.size() > id) {
    //         myEdgeSpeeds[id] = attrs.get<double>(SUMO_ATTR_SPEED, nullptr, ok);
    //         if (myBikeSpeeds) {
    //             if (attrs.hasAttribute(SUMO_ATTR_BIKESPEED)) {
    //                 myEdgeBikeSpeeds[id] = attrs.get<double>(SUMO_ATTR_BIKESPEED, nullptr, ok);
    //             } else if (!checkedBikeSpeeds) {
    //                 checkedBikeSpeeds = true;
    //                 WRITE_WARNING("Bike speeds missing in loaded state");
    //             }
    //         }
    //         if (myAdaptationSteps > 0) {
    //             const std::vector<double> speeds = attrs.getOpt<std::vector<double> >(SUMO_ATTR_PASTSPEED, nullptr, ok);
    //             if ((int)speeds.size() == myAdaptationSteps) {
    //                 myPastEdgeSpeeds[id] = speeds;
    //                 if (myBikeSpeeds && attrs.hasAttribute(SUMO_ATTR_PASTBIKESPEED)) {
    //                     myPastEdgeBikeSpeeds[id] = attrs.getOpt<std::vector<double> >(SUMO_ATTR_PASTBIKESPEED, nullptr, ok);
    //                 }
    //             } else if (!checkedSteps) {
    //                 checkedSteps = true;
    //                 WRITE_WARNING("Number of adaptation speeds in loaded state doesn't match option --device.rerouting.adaptation-steps");
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(attrs: &SUMOSAXAttributes) {
        let e = MSEdge::dictionary_2(&attrs.get_string(SUMO_ATTR_ID));
        e.mark_delayed();
        let id = e.get_numerical_id() as usize;
        let mut checkedSteps = false;
        let mut checkedBikeSpeeds = false;
        let mut ok = true;
        let mut st = state();
        if st.myEdgeSpeeds.len() > id {
            st.myEdgeSpeeds[id] = attrs.get::<f64>(SUMO_ATTR_SPEED, "", &mut ok, true);
            if st.myBikeSpeeds {
                if attrs.has_attribute(SUMO_ATTR_BIKESPEED) {
                    st.myEdgeBikeSpeeds[id] = attrs.get::<f64>(SUMO_ATTR_BIKESPEED, "", &mut ok, true);
                } else if !checkedBikeSpeeds {
                    checkedBikeSpeeds = true;
                    utils::common::msg_handler::write_warning("Bike speeds missing in loaded state");
                }
            }
            if st.myAdaptationSteps > 0 {
                let speeds: Vec<f64> = attrs.get_opt::<Vec<f64>>(SUMO_ATTR_PASTSPEED, "", &mut ok, Vec::new(), true);
                if speeds.len() as i32 == st.myAdaptationSteps {
                    st.myPastEdgeSpeeds[id] = speeds;
                    if st.myBikeSpeeds && attrs.has_attribute(SUMO_ATTR_PASTBIKESPEED) {
                        st.myPastEdgeBikeSpeeds[id] = attrs.get_opt::<Vec<f64>>(SUMO_ATTR_PASTBIKESPEED, "", &mut ok, Vec::new(), true);
                    }
                } else if !checkedSteps {
                    checkedSteps = true;
                    utils::common::msg_handler::write_warning("Number of adaptation speeds in loaded state doesn't match option --device.rerouting.adaptation-steps");
                }
            }
        }
    }

    /// C++: `SUMOTime MSRoutingEngine::adaptEdgeEfforts(SUMOTime)` [static, private]
    /// Adapt edge efforts by the current edge states
    // SUMOTime
    // MSRoutingEngine::adaptEdgeEfforts(SUMOTime currentTime) {
    //     initEdgeWeights(SVC_PASSENGER);
    //     if (myBikeSpeeds) {
    //         initEdgeWeights(SVC_BICYCLE);
    //     }
    //     if (MSNet::getInstance()->getVehicleControl().getDepartedVehicleNo() == 0) {
    //         return myAdaptationInterval;
    //     }
    //     {
    // #ifdef HAVE_FOX
    //         FXMutexLock lock(myRouteCacheMutex);
    // #endif
    //         myCachedRoutes.clear();
    //     }
    //     const MSEdgeVector& edges = MSNet::getInstance()->getEdgeControl().getEdges();
    //     const double newWeightFactor = (double)(1. - myAdaptationWeight);
    //     bool cchEdgeMoved = false;
    //     for (const MSEdge* const e : edges) {
    //         if (e->isDelayed()) {
    //             const int id = e->getNumericalID();
    //             double currSpeed = e->getMeanSpeed();
    //             if (MSGlobals::gWeightsSeparateTurns > 0 && e->getNumSuccessors() > 1) {
    //                 currSpeed = patchSpeedForTurns(e, currSpeed);
    //             }
    // #ifdef DEBUG_SEPARATE_TURNS
    //             if (DEBUG_COND(e->getLanes()[0])) {
    //                 std::cout << SIMTIME << " edge=" << e->getID()
    //                           << " meanSpeed=" << e->getMeanSpeed()
    //                           << " currSpeed=" << currSpeed
    //                           << " oldestSpeed=" << myPastEdgeSpeeds[id][myAdaptationStepsIndex]
    //                           << " oldAvg=" << myEdgeSpeeds[id]
    //                           << "\n";
    //             }
    // #endif
    //             const double oldSmoothedSpeed = myEdgeSpeeds[id];
    //             if (myAdaptationSteps > 0) {
    //                 // moving average
    //                 myEdgeSpeeds[id] += (currSpeed - myPastEdgeSpeeds[id][myAdaptationStepsIndex]) / myAdaptationSteps;
    //                 myPastEdgeSpeeds[id][myAdaptationStepsIndex] = currSpeed;
    //                 if (myBikeSpeeds) {
    //                     const double currBikeSpeed = e->getMeanSpeedBike();
    //                     myEdgeBikeSpeeds[id] += (currBikeSpeed - myPastEdgeBikeSpeeds[id][myAdaptationStepsIndex]) / myAdaptationSteps;
    //                     myPastEdgeBikeSpeeds[id][myAdaptationStepsIndex] = currBikeSpeed;
    //                 }
    //             } else {
    //                 // exponential moving average
    //                 if (currSpeed != myEdgeSpeeds[id]) {
    //                     myEdgeSpeeds[id] = myEdgeSpeeds[id] * myAdaptationWeight + currSpeed * newWeightFactor;
    //                 }
    //                 if (myBikeSpeeds) {
    //                     const double currBikeSpeed = e->getMeanSpeedBike();
    //                     if (currBikeSpeed != myEdgeBikeSpeeds[id]) {
    //                         myEdgeBikeSpeeds[id] = myEdgeBikeSpeeds[id] * myAdaptationWeight + currBikeSpeed * newWeightFactor;
    //                     }
    //                 }
    //             }
    //             // sparse CCH re-customization: remember which edges actually
    //             // moved; the update deadband is applied at the customize barrier
    //             if (myCCHLive != nullptr && myEdgeSpeeds[id] != oldSmoothedSpeed) {
    //                 myCCHLive->markDirty(e);
    //                 cchEdgeMoved = true;
    //             }
    //         }
    //     }
    //     if (myAdaptationSteps > 0) {
    //         myAdaptationStepsIndex = (myAdaptationStepsIndex + 1) % myAdaptationSteps;
    //     }
    //     myLastAdaptation = currentTime;
    //     // The device's CCH customization barrier: re-customize the metrics from
    //     // the freshly updated speed tables and publish (double-buffered). Runs
    //     // on the main thread after the speed update; no worker query is in
    //     // flight at this point. The family skips quiet barriers and runs that
    //     // never route (see CCHMetricFamily::atBarrier: measured on Lausanne, the
    //     // unconditional per-tick customize was 80% of meso wall time at
    //     // adaptation-interval 2, and ran even with rerouting probability 0).
    //     if (myCCHLive != nullptr) {
    //         myCCHLive->atBarrier(STEPS2TIME(currentTime), cchEdgeMoved);
    //     }
    //     if (OptionsCont::getOptions().isSet("device.rerouting.output")) {
    //         OutputDevice& dev = OutputDevice::getDeviceByOption("device.rerouting.output");
    //         dev.openTag(SUMO_TAG_INTERVAL);
    //         dev.writeAttr(SUMO_ATTR_ID, "device.rerouting");
    //         dev.writeAttr(SUMO_ATTR_BEGIN, STEPS2TIME(currentTime));
    //         dev.writeAttr(SUMO_ATTR_END, STEPS2TIME(currentTime + myAdaptationInterval));
    //         for (const MSEdge* e : edges) {
    //             dev.openTag(SUMO_TAG_EDGE);
    //             dev.writeAttr(SUMO_ATTR_ID, e->getID());
    //             dev.writeAttr("traveltime", myEffortFunc(e, nullptr, STEPS2TIME(currentTime)));
    //             if (myBikeSpeeds) {
    //                 // @note edge-priority is not included here
    //                 dev.writeAttr("traveltimeBike", getEffortBike(e, nullptr, STEPS2TIME(currentTime)));
    //             }
    //             dev.closeTag();
    //         }
    //         dev.closeTag();
    //     }
    //     return myAdaptationInterval;
    // }
    #[allow(unused_variables)]
    pub fn adapt_edge_efforts(currentTime: SUMOTime) -> SUMOTime {
        Self::init_edge_weights(SVC_PASSENGER, -1, -1);
        if state().myBikeSpeeds {
            Self::init_edge_weights(SVC_BICYCLE, -1, -1);
        }
        if MSNet::get_instance().m().get_vehicle_control().get_departed_vehicle_no() == 0 {
            return state().myAdaptationInterval;
        }
        // myCachedRoutes.clear(): the route cache is never populated in this port
        let edges: Vec<Ptr<MSEdge>> = MSNet::get_instance().m().get_edge_control().get_edges().clone();
        let sep = weights_separate_turns();
        {
            let mut st = state();
            let w = st.myAdaptationWeight;
            let newWeightFactor = 1. - w;
            for e in edges.iter() {
                if e.is_delayed() {
                    let id = e.get_numerical_id() as usize;
                    let mut currSpeed = e.get_mean_speed();
                    if sep > 0. && e.get_num_successors() > 1 {
                        currSpeed = patch_speed_for_turns_locked(&mut st, *e, currSpeed);
                    }
                    let oldSmoothedSpeed = st.myEdgeSpeeds[id];
                    let _ = oldSmoothedSpeed; // only used by the CCH dirty marking, which is not ported
                    if st.myAdaptationSteps > 0 {
                        let idx = st.myAdaptationStepsIndex as usize;
                        let steps = st.myAdaptationSteps as f64;
                        // moving average
                        let past = st.myPastEdgeSpeeds[id][idx];
                        st.myEdgeSpeeds[id] += (currSpeed - past) / steps;
                        st.myPastEdgeSpeeds[id][idx] = currSpeed;
                        if st.myBikeSpeeds {
                            let currBikeSpeed = e.get_mean_speed_bike();
                            let pastBike = st.myPastEdgeBikeSpeeds[id][idx];
                            st.myEdgeBikeSpeeds[id] += (currBikeSpeed - pastBike) / steps;
                            st.myPastEdgeBikeSpeeds[id][idx] = currBikeSpeed;
                        }
                    } else {
                        // exponential moving average
                        if currSpeed != st.myEdgeSpeeds[id] {
                            let cur = st.myEdgeSpeeds[id];
                            st.myEdgeSpeeds[id] = cur * w + currSpeed * newWeightFactor;
                        }
                        if st.myBikeSpeeds {
                            let currBikeSpeed = e.get_mean_speed_bike();
                            if currBikeSpeed != st.myEdgeBikeSpeeds[id] {
                                let curBike = st.myEdgeBikeSpeeds[id];
                                st.myEdgeBikeSpeeds[id] = curBike * w + currBikeSpeed * newWeightFactor;
                            }
                        }
                    }
                    // sparse CCH re-customization (myCCHLive) is not ported: myCCHLive is never created here
                }
            }
            if st.myAdaptationSteps > 0 {
                st.myAdaptationStepsIndex = (st.myAdaptationStepsIndex + 1) % st.myAdaptationSteps;
            }
            st.myLastAdaptation = currentTime;
        }
        // CCH customization barrier (myCCHLive->atBarrier) is skipped: myCCHLive is never created in this port
        if utils::options::options_cont::is_set("device.rerouting.output") {
            let dev = utils::iodevices::output_device::OutputDevice::get_device_by_option("device.rerouting.output").unwrap_or_else(|e| std::panic::panic_any(e));
            let mut dev = dev.borrow_mut();
            let bike = state().myBikeSpeeds;
            let interval = state().myAdaptationInterval;
            dev.open_tag_id(SUMO_TAG_INTERVAL);
            dev.write_attr(SUMO_ATTR_ID, "device.rerouting");
            dev.write_attr(SUMO_ATTR_BEGIN, &steps2time(currentTime));
            dev.write_attr(SUMO_ATTR_END, &steps2time(currentTime + interval));
            for e in edges.iter() {
                dev.open_tag_id(SUMO_TAG_EDGE);
                dev.write_attr(SUMO_ATTR_ID, e.get_id());
                dev.write_attr("traveltime", &effort_func(*e, Ptr::null(), steps2time(currentTime)));
                if bike {
                    // @note edge-priority is not included here
                    dev.write_attr("traveltimeBike", &MSRoutingEngine::get_effort_bike(*e, Ptr::null(), steps2time(currentTime)));
                }
                dev.close_tag("");
            }
            dev.close_tag("");
        }
        state().myAdaptationInterval
    }

    /// C++: `double MSRoutingEngine::patchSpeedForTurns(const MSEdge *, double)` [static, private]
    // double
    // MSRoutingEngine::patchSpeedForTurns(const MSEdge* edge, double currSpeed) {
    //     const double length = edge->getLength();
    //     double maxSpeed = 0;
    //     for (const auto& pair : edge->getViaSuccessors()) {
    //         if (pair.second == nullptr) {
    //             continue;
    //         }
    //         TimeAndCount& tc = myEdgeTravelTimes[pair.second->getNumericalID()];
    //         if (tc.second > 0) {
    //             const double avgSpeed = length / STEPS2TIME(tc.first / tc.second);
    //             maxSpeed = MAX2(avgSpeed, maxSpeed);
    //         }
    //     }
    //     if (maxSpeed > 0) {
    //         // perform correction
    //         const double correctedSpeed = MSGlobals::gWeightsSeparateTurns * maxSpeed + (1 - MSGlobals::gWeightsSeparateTurns) * currSpeed;
    //         for (const auto& pair : edge->getViaSuccessors()) {
    //             if (pair.second == nullptr) {
    //                 continue;
    //             }
    //             const int iid = pair.second->getNumericalID();
    //             TimeAndCount& tc = myEdgeTravelTimes[iid];
    //             if (tc.second > 0) {
    //                 const double avgSpeed = length / STEPS2TIME(tc.first / tc.second);
    //                 if (avgSpeed < correctedSpeed) {
    //                     double internalTT = pair.second->getLength() / pair.second->getSpeedLimit();
    //                     internalTT += (length / avgSpeed - length / correctedSpeed) * MSGlobals::gWeightsSeparateTurns;
    //                     const double origInternalSpeed = myEdgeSpeeds[iid];
    //                     const double newInternalSpeed = pair.second->getLength() / internalTT;
    //                     const double origCurrSpeed = myPastEdgeSpeeds[iid][myAdaptationStepsIndex];
    //
    //                     myEdgeSpeeds[iid] = newInternalSpeed;
    //                     // to ensure myEdgeSpeed reverts to the speed limit
    //                     // when there are no updates, we also have to patch
    //                     // myPastEdgeSpeeds with a virtual value that is consistent
    //                     // with the updated speed
    //                     // note: internal edges were handled before the normal ones
    //                     const double virtualSpeed = (newInternalSpeed - (origInternalSpeed - origCurrSpeed / myAdaptationSteps)) * myAdaptationSteps;
    //                     myPastEdgeSpeeds[iid][myAdaptationStepsIndex] = virtualSpeed;
    //
    // #ifdef DEBUG_SEPARATE_TURNS
    //                     if (DEBUG_COND(pair.second->getLanes()[0])) {
    //                         std::cout << SIMTIME << " edge=" << edge->getID() << " to=" << pair.first->getID() << " via=" << pair.second->getID()
    //                                   << " origSpeed=" << currSpeed
    //                                   << " maxSpeed=" << maxSpeed
    //                                   << " correctedSpeed=" << correctedSpeed
    //                                   << " avgSpeed=" << avgSpeed
    //                                   << " internalTT=" << internalTT
    //                                   << " internalSpeed=" << origInternalSpeed
    //                                   << " newInternalSpeed=" << newInternalSpeed
    //                                   << " virtualSpeed=" << virtualSpeed
    //                                   << "\n";
    //                     }
    // #endif
    //                 }
    //                 if (myAdaptationStepsIndex == 0) {
    //                     tc.first = 0;
    //                     tc.second = 0;
    //                 }
    //             }
    //         }
    //         return correctedSpeed;
    //     }
    //     return currSpeed;
    // }
    #[allow(unused_variables)]
    pub fn patch_speed_for_turns(edge: Ptr<MSEdge>, currSpeed: f64) -> f64 {
        let mut st = state();
        patch_speed_for_turns_locked(&mut st, edge, currSpeed)
    }

    /// C++: `void MSRoutingEngine::_initEdgeWeights(int &, int &)` [static, private]
    /// initialized edge speed storage into the given containers
    // void
    // MSRoutingEngine::_initEdgeWeights(std::vector<double>& edgeSpeeds, std::vector<std::vector<double> >& pastEdgeSpeeds) {
    //     if (edgeSpeeds.empty()) {
    //         const OptionsCont& oc = OptionsCont::getOptions();
    //         const bool useLoaded = oc.getBool("device.rerouting.init-with-loaded-weights");
    //         const double currentSecond = SIMTIME;
    //         for (const MSEdge* const edge : MSNet::getInstance()->getEdgeControl().getEdges()) {
    //             while (edge->getNumericalID() >= (int)edgeSpeeds.size()) {
    //                 edgeSpeeds.push_back(0);
    //                 if (myAdaptationSteps > 0) {
    //                     pastEdgeSpeeds.push_back(std::vector<double>());
    //                 }
    //                 if (MSGlobals::gWeightsSeparateTurns && edgeSpeeds == myEdgeSpeeds) {
    //                     myEdgeTravelTimes.push_back(TimeAndCount(0, 0));
    //                 }
    //             }
    //             if (useLoaded) {
    //                 edgeSpeeds[edge->getNumericalID()] = edge->getLength() / MSNet::getTravelTime(edge, nullptr, currentSecond);
    //             } else {
    //                 edgeSpeeds[edge->getNumericalID()] = edge->getMeanSpeed();
    //             }
    //             if (myAdaptationSteps > 0) {
    //                 pastEdgeSpeeds[edge->getNumericalID()] = std::vector<double>(myAdaptationSteps, edgeSpeeds[edge->getNumericalID()]);
    //             }
    //         }
    //         myLastAdaptation = MSNet::getInstance()->getCurrentTimeStep();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_edge_weights_2(bike: bool) {
        let empty = {
            let st = state();
            if bike { st.myEdgeBikeSpeeds.is_empty() } else { st.myEdgeSpeeds.is_empty() }
        };
        if empty {
            let useLoaded = get_bool("device.rerouting.init-with-loaded-weights");
            let currentSecond = steps2time(MSNet::get_instance().get_current_time_step());
            let sep = weights_separate_turns();
            let edges: Vec<Ptr<MSEdge>> = MSNet::get_instance().m().get_edge_control().get_edges().clone();
            // computed before taking the state lock: MSNet::get_travel_time may call back into get_effort, which locks the state
            let values: Vec<f64> = edges
                .iter()
                .map(|edge| {
                    if useLoaded {
                        edge.get_length() / MSNet::get_travel_time(*edge, Ptr::null(), currentSecond)
                    } else {
                        edge.get_mean_speed()
                    }
                })
                .collect();
            let mut guard = state();
            let st: &mut RoutingEngineState = &mut guard;
            for (edge, value) in edges.iter().zip(values.iter().copied()) {
                let id = edge.get_numerical_id() as usize;
                loop {
                    let len = if bike { st.myEdgeBikeSpeeds.len() } else { st.myEdgeSpeeds.len() };
                    if id < len {
                        break;
                    }
                    if bike {
                        st.myEdgeBikeSpeeds.push(0.);
                    } else {
                        st.myEdgeSpeeds.push(0.);
                    }
                    if st.myAdaptationSteps > 0 {
                        if bike {
                            st.myPastEdgeBikeSpeeds.push(Vec::new());
                        } else {
                            st.myPastEdgeSpeeds.push(Vec::new());
                        }
                    }
                    // C++ compares the two vectors by value; for the passenger vector the comparison is trivially true
                    let same = if bike { st.myEdgeBikeSpeeds == st.myEdgeSpeeds } else { true };
                    if sep != 0. && same {
                        st.myEdgeTravelTimes.push((0, 0));
                    }
                }
                if bike {
                    st.myEdgeBikeSpeeds[id] = value;
                } else {
                    st.myEdgeSpeeds[id] = value;
                }
                if st.myAdaptationSteps > 0 {
                    let steps = st.myAdaptationSteps as usize;
                    if bike {
                        st.myPastEdgeBikeSpeeds[id] = vec![value; steps];
                    } else {
                        st.myPastEdgeSpeeds[id] = vec![value; steps];
                    }
                }
            }
            st.myLastAdaptation = MSNet::get_instance().get_current_time_step();
        }
    }

    /// C++: `SumoRNG * MSRoutingEngine::getThreadRNG()` [static, private]
    /// returns RNG associated with the current thread
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_thread_rng() -> Ptr<SumoRNG> {
        // shortcut: C++ declares getThreadRNG() in MSRoutingEngine.h but never defines or calls it (dead declaration), so this is unreachable; delete it, not port it
        panic!("shortcut: MSRoutingEngine::getThreadRNG is a dead C++ declaration with no definition")
    }

    /// C++: `SUMOVehicle * MSRoutingEngine::buildCCHRefVehicle(const MSVehicleType *, int)` [static, private]
    /// construct the unregistered effort-reference vehicle for a type: never counted, inserted or given devices, with the type's mean speed factor and a deterministic random seed (the family's RefVehicleFactory; the slot picks the frozen random-factor realization, see device.rerouting.cch-ensemble)
    // SUMOVehicle*
    // MSRoutingEngine::buildCCHRefVehicle(const MSVehicleType* type, int slot) {
    //     // The effort-reference vehicle: constructed DIRECTLY (what buildVehicle
    //     // does minus initVehicle), so it is never counted in the vehicle
    //     // statistics, never given devices and never inserted -- it exists only
    //     // so every metric fill evaluates the same reference. The mean speed
    //     // factor makes it deterministic (the findRoute precedent) and the fixed
    //     // id makes the frozen weights.random-factor realization reproducible
    //     // (the random seed is a hash of the id).
    //     SUMOVehicleParameter* pars = new SUMOVehicleParameter();
    //     // slot 0 keeps the historical id (and with it the frozen random-factor
    //     // realization); ensemble slots differ only in the id, which seeds their
    //     // own realization -- see CCHMetricFamily::RefVehicleFactory
    //     pars->id = "cchRef:" + type->getID() + (slot == 0 ? "" : ":" + toString(slot));
    //     const MSEdge* refEdge = nullptr;
    //     for (const MSEdge* e : MSEdge::getAllEdges()) {
    //         if (!e->isInternal() && !e->isTazConnector()) {
    //             refEdge = e;
    //             break;
    //         }
    //     }
    //     ConstMSRoutePtr route = std::make_shared<MSRoute>(pars->id, ConstMSEdgeVector({refEdge}), false, nullptr, StopParVector());
    //     if (MSGlobals::gUseMesoSim) {
    //         return new MEVehicle(pars, route, const_cast<MSVehicleType*>(type),
    //                              type->getSpeedFactor().getParameter(0));
    //     }
    //     return new MSVehicle(pars, route, const_cast<MSVehicleType*>(type),
    //                          type->getSpeedFactor().getParameter(0));
    // }
    #[allow(unused_variables)]
    pub fn build_cch_ref_vehicle(type_: Ptr<MSVehicleType>, slot: i32) -> Ptr<SUMOVehicle> {
        // shortcut: CCH graph and metric families are not ported yet, upgrade with the CCH portbuildCCHRefVehicle builds the CCH reference vehicle (MSRoute and MEVehicle/MSVehicle construction)
        let _ = (type_, slot);
        panic!("shortcut: buildCCHRefVehicle needs the CCH port")
    }

    /// C++: `void MSRoutingEngine::initCCH()` [static, private]
    /// build the shared CCH + the live family with one metric per loaded type, publish initial metrics
    // void
    // MSRoutingEngine::initCCH() {
    //     if (myCCHLive != nullptr) {
    //         return;  // device-side state built once (the graph alone may already
    //         // exist for MSNet's free-flow router)
    //     }
    //     // One LIVE metric per vehicle TYPE known when routing starts (see
    //     // utils/router/CCHMetricFamily.h for the keying and buffer semantics).
    //     // All types share ONE CCH topology (the union); route files stream, so
    //     // types appearing later register through the family's wanted list and
    //     // get their metric at the next customization barrier.
    //     ensureCCHGraph();
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     myCCHLive = new MSCCHMetricFamily(
    //         myCCHGraph, myEffortFunc, &MSRoutingEngine::getEffort,
    //         oc.getFloat("device.rerouting.cch-update-threshold.factor"),
    //         STEPS2TIME(string2time(oc.getString("device.rerouting.cch-update-threshold.constant"))),
    //         &MSRoutingEngine::buildCCHRefVehicle,
    //         oc.getInt("device.rerouting.cch-ensemble"));
    //     std::vector<std::string> vtypeIDs;
    //     MSNet::getInstance()->getVehicleControl().insertVTypeIDs(vtypeIDs);
    //     for (const std::string& id : vtypeIDs) {
    //         if (MSNet::getInstance()->getVehicleControl().hasVTypeDistribution(id)) {
    //             // the members are in the list as well, resolving the distribution would draw from the default RNG
    //             continue;
    //         }
    //         const MSVehicleType* vt = MSNet::getInstance()->getVehicleControl().getVType(id, nullptr, true);
    //         if (vt != nullptr && !vt->isVehicleSpecific()) {
    //             myCCHLive->seedKey(vt);
    //         }
    //     }
    //     // publish initial metrics so the first queries succeed
    //     myCCHLive->customize(STEPS2TIME(MSNet::getInstance()->getCurrentTimeStep()));
    // }
    #[allow(unused_variables)]
    pub fn init_cch() {
        // shortcut: CCH graph and metric families are not ported yet, upgrade with the CCH portinitCCH builds the live CCH metric family
        panic!("shortcut: initCCH needs the CCH port")
    }

    /// C++: `void MSRoutingEngine::invalidateCCHEdge(const MSEdge *)` [static]
    /// a runtime permission change (closure / re-opening) hit this edge: invalidate the graph's primed connection masks and both families' metrics (queries divert to the exact fallback until the families re-customize)
    // void
    // MSRoutingEngine::invalidateCCHEdge(const MSEdge* e) {
    //     if (myCCHGraph == nullptr) {
    //         return;
    //     }
    //     // the primed connection masks reflect the OLD successor lists; the next
    //     // fill of either family re-primes from the live ones. Safe to do right
    //     // here: masks are only read at fill time, and every fill runs on the
    //     // main thread (the device barrier, the free-flow repair) -- never on a
    //     // query thread.
    //     myCCHGraph->invalidateClassMasks();
    //     if (myCCHFreeflow != nullptr) {
    //         myCCHFreeflow->flagPermissionsStale();
    //     }
    //     if (myCCHLive != nullptr) {
    //         myCCHLive->invalidateEdge(e);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn invalidate_cch_edge(e: Ptr<MSEdge>) {
        let _ = e;
        // myCCHGraph is never created in this port (ensureCCHGraph is not ported), so C++ returns at its nullptr check
    }

    /// C++: `void MSRoutingEngine::MSRoutingEngine(const MSRoutingEngine &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSRoutingEngine) -> Self {
        panic!("MSRoutingEngine copy constructor is private and invalidated in C++")
    }

    /// C++: `MSRoutingEngine & MSRoutingEngine::operator=(const MSRoutingEngine &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSRoutingEngine) -> &mut MSRoutingEngine {
        panic!("MSRoutingEngine assignment operator is private and invalidated in C++")
    }

}

// ---- helpers (file-private) ----

/// The numeric MSRoutingEngine statics. Statics that hold unported types (the router provider, the CCH families,
/// and the route cache) are not modelled here; the edge-weight StaticCommand is owned by the event queue (see the shortcut notes at their uses).
struct RoutingEngineState {
    myEdgeSpeeds: Vec<f64>,
    myEdgeBikeSpeeds: Vec<f64>,
    myEdgeTravelTimes: Vec<(SUMOTime, i32)>,
    myPastEdgeSpeeds: Vec<Vec<f64>>,
    myPastEdgeBikeSpeeds: Vec<Vec<f64>>,
    myAdaptationSteps: i32,
    myAdaptationStepsIndex: i32,
    myAdaptationInterval: SUMOTime,
    myLastAdaptation: SUMOTime,
    myAdaptationWeight: f64,
    myWithTaz: bool,
    myBikeSpeeds: bool,
    myPriorityFactor: f64,
    myMinEdgePriority: f64,
    myEdgePriorityRange: f64,
    myDynamicRandomness: bool,
    myHaveExtras: bool,
    /// true when myEffortFunc points at getEffortExtra, false for getEffort (the C++ static default)
    myEffortFuncIsExtra: bool,
    /// C++ `myEdgeWeightSettingCommand != nullptr` (the command itself is owned by the end-of-timestep event queue)
    myEdgeWeightSettingCommand: bool,
}

// determinism: all routing-engine statics live behind one lock; callers must not hold it across another call that locks it
static ROUTING_ENGINE_STATE: std::sync::Mutex<RoutingEngineState> = std::sync::Mutex::new(RoutingEngineState {
    myEdgeSpeeds: Vec::new(),
    myEdgeBikeSpeeds: Vec::new(),
    myEdgeTravelTimes: Vec::new(),
    myPastEdgeSpeeds: Vec::new(),
    myPastEdgeBikeSpeeds: Vec::new(),
    myAdaptationSteps: 0,
    myAdaptationStepsIndex: 0,
    myAdaptationInterval: -1,
    myLastAdaptation: -1,
    myAdaptationWeight: 0.,
    myWithTaz: false,
    myBikeSpeeds: false,
    myPriorityFactor: 0.,
    myMinEdgePriority: 0.,
    myEdgePriorityRange: 0.,
    myDynamicRandomness: false,
    myHaveExtras: false,
    myEffortFuncIsExtra: false,
    myEdgeWeightSettingCommand: false,
});

thread_local! {
    /// C++ static `MSRoutingEngine::myRouterProvider`; thread-local because the worker-thread pool is not ported
    static ROUTER_PROVIDER: std::cell::Cell<Option<Ptr<MSRouterProvider>>> = const { std::cell::Cell::new(None) };
}

fn state() -> std::sync::MutexGuard<'static, RoutingEngineState> {
    ROUTING_ENGINE_STATE.lock().unwrap_or_else(|e| e.into_inner())
}

/// MAX2 (StdDefs.h): a > b ? a : b
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// MIN2 (StdDefs.h): a < b ? a : b
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

fn oc_float(oc: &OptionsCont, name: &str) -> f64 {
    oc.get_float(name).unwrap_or_else(|e| std::panic::panic_any(e))
}

fn oc_bool(oc: &OptionsCont, name: &str) -> bool {
    oc.get_bool(name).unwrap_or_else(|e| std::panic::panic_any(e))
}

// shortcut: gWeightsRandomFactor is not a ported global, read the option it is copied from on each use, upgrade when std_defs gains it
fn weights_random_factor() -> f64 {
    get_float("weights.random-factor")
}

fn weights_separate_turns() -> f64 {
    crate::microsim::ms_globals::weights_separate_turns()
}

/// C++ myEffortFunc(e, v, t): getEffortExtra or getEffort, depending on the flag set by initRouter
fn effort_func(e: Ptr<MSEdge>, v: Ptr<SUMOVehicle>, t: f64) -> f64 {
    let extra = state().myEffortFuncIsExtra;
    if extra {
        MSRoutingEngine::get_effort_extra(e, v, t)
    } else {
        MSRoutingEngine::get_effort(e, v, t)
    }
}

fn e_len_of(e: Ptr<MSEdge>) -> f64 {
    e.get_length()
}

fn v_random_seed(v: Ptr<SUMOVehicle>) -> i64 {
    v.m().get_random_seed()
}

fn v_vclass(v: Ptr<SUMOVehicle>) -> SUMOVehicleClass {
    v.m().get_v_class()
}

fn v_vtype_parameter<'a>(v: Ptr<SUMOVehicle>) -> &'a SUMOVTypeParameter {
    v.get().expect("null vehicle").get_v_type_parameter()
}

/// patchSpeedForTurns on an already locked state (the C++ function body)
fn patch_speed_for_turns_locked(st: &mut RoutingEngineState, edge: Ptr<MSEdge>, currSpeed: f64) -> f64 {
    let length = edge.get_length();
    let mut maxSpeed = 0.;
    for pair in edge.get_via_successors(SVC_IGNORING, false).iter() {
        if pair.1.is_null() {
            continue;
        }
        let tc = st.myEdgeTravelTimes[pair.1.get_numerical_id() as usize];
        if tc.1 > 0 {
            // C++ integer division of SUMOTime by int, then STEPS2TIME
            let avgSpeed = length / steps2time(tc.0 / tc.1 as i64);
            maxSpeed = max2(avgSpeed, maxSpeed);
        }
    }
    if maxSpeed > 0. {
        // perform correction
        let sep = weights_separate_turns();
        let correctedSpeed = sep * maxSpeed + (1. - sep) * currSpeed;
        for pair in edge.get_via_successors(SVC_IGNORING, false).iter() {
            if pair.1.is_null() {
                continue;
            }
            let iid = pair.1.get_numerical_id() as usize;
            let tc = st.myEdgeTravelTimes[iid];
            if tc.1 > 0 {
                let avgSpeed = length / steps2time(tc.0 / tc.1 as i64);
                if avgSpeed < correctedSpeed {
                    let mut internalTT = pair.1.get_length() / pair.1.get_speed_limit();
                    internalTT += (length / avgSpeed - length / correctedSpeed) * sep;
                    let origInternalSpeed = st.myEdgeSpeeds[iid];
                    let newInternalSpeed = pair.1.get_length() / internalTT;
                    let idx = st.myAdaptationStepsIndex as usize;
                    let origCurrSpeed = st.myPastEdgeSpeeds[iid][idx];
                    st.myEdgeSpeeds[iid] = newInternalSpeed;
                    // to ensure myEdgeSpeed reverts to the speed limit when there are no updates, patch
                    // myPastEdgeSpeeds with a virtual value that is consistent with the updated speed
                    // note: internal edges were handled before the normal ones
                    let steps = st.myAdaptationSteps as f64;
                    let virtualSpeed = (newInternalSpeed - (origInternalSpeed - origCurrSpeed / steps)) * steps;
                    st.myPastEdgeSpeeds[iid][idx] = virtualSpeed;
                }
                if st.myAdaptationStepsIndex == 0 {
                    st.myEdgeTravelTimes[iid] = (0, 0);
                }
            }
        }
        return correctedSpeed;
    }
    currSpeed
}
