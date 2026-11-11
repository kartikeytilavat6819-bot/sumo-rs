//! Port of microsim/devices/MSRoutingEngine.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

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
        todo!("port MSRoutingEngine::initWeightConstants")
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
        todo!("port MSRoutingEngine::initWeightUpdate")
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
        todo!("port MSRoutingEngine::initEdgeWeights")
    }

    /// C++: `bool MSRoutingEngine::hasEdgeUpdates()` [static]
    /// returns whether any edge weight updates will take place
    // static bool hasEdgeUpdates() {
    //         return myEdgeWeightSettingCommand != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn has_edge_updates() -> bool {
        todo!("port MSRoutingEngine::hasEdgeUpdates")
    }

    /// C++: `SUMOTime MSRoutingEngine::getLastAdaptation()` [static]
    /// Information when the last edge weight adaptation occurred
    // static SUMOTime getLastAdaptation() {
    //         return myLastAdaptation;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_adaptation() -> SUMOTime {
        todo!("port MSRoutingEngine::getLastAdaptation")
    }

    /// C++: `bool MSRoutingEngine::haveExtras()` [static]
    // static bool haveExtras() {
    //         return myHaveExtras;
    //     }
    #[allow(unused_variables)]
    pub fn have_extras() -> bool {
        todo!("port MSRoutingEngine::haveExtras")
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
        todo!("port MSRoutingEngine::applyExtras")
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
    pub fn get_cached_route(key: &(Ptr<MSEdge>, Ptr<MSEdge>)) -> ConstMSRoutePtr {
        todo!("port MSRoutingEngine::getCachedRoute")
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
        todo!("port MSRoutingEngine::getPublishedCCHMetric")
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
        todo!("port MSRoutingEngine::getFreeflowCCHMetric")
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
        todo!("port MSRoutingEngine::ensureCCHGraph")
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
        todo!("port MSRoutingEngine::initRouter")
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
        todo!("port MSRoutingEngine::reroute")
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
        todo!("port MSRoutingEngine::reroute")
    }

    /// C++: `void MSRoutingEngine::setEdgeTravelTime(const MSEdge *const, const double)` [static]
    /// adapt the known travel time for an edge
    // void
    // MSRoutingEngine::setEdgeTravelTime(const MSEdge* const edge, const double travelTime) {
    //     myEdgeSpeeds[edge->getNumericalID()] = edge->getLength() / travelTime;
    // }
    #[allow(unused_variables)]
    pub fn set_edge_travel_time(edge: Ptr<MSEdge>, travelTime: f64) {
        todo!("port MSRoutingEngine::setEdgeTravelTime")
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
        todo!("port MSRoutingEngine::cleanup")
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
        todo!("port MSRoutingEngine::cleanupCCH")
    }

    /// C++: `bool MSRoutingEngine::isEnabled()` [static]
    /// returns whether any routing actions take place
    // static bool isEnabled() {
    //         return !myWithTaz && myAdaptationInterval >= 0;
    //     }
    #[allow(unused_variables)]
    pub fn is_enabled() -> bool {
        todo!("port MSRoutingEngine::isEnabled")
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
        todo!("port MSRoutingEngine::getRouterTT")
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
        todo!("port MSRoutingEngine::getIntermodalRouterTT")
    }

    /// C++: `bool MSRoutingEngine::hasBikeSpeeds()` [static]
    /// whether the router collects bicycle speeds
    // static bool hasBikeSpeeds() {
    //         return myBikeSpeeds;
    //     }
    #[allow(unused_variables)]
    pub fn has_bike_speeds() -> bool {
        todo!("port MSRoutingEngine::hasBikeSpeeds")
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
        todo!("port MSRoutingEngine::getEffort")
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
        todo!("port MSRoutingEngine::getEffortBike")
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
        todo!("port MSRoutingEngine::getEffortExtra")
    }

    /// C++: `double MSRoutingEngine::getAssumedSpeed(const MSEdge *, const SUMOVehicle *)` [static]
    /// return current travel speed assumption
    // double
    // MSRoutingEngine::getAssumedSpeed(const MSEdge* edge, const SUMOVehicle* veh) {
    //     return edge->getLength() / myEffortFunc(edge, veh, 0);
    // }
    #[allow(unused_variables)]
    pub fn get_assumed_speed(edge: Ptr<MSEdge>, veh: Ptr<SUMOVehicle>) -> f64 {
        todo!("port MSRoutingEngine::getAssumedSpeed")
    }

    /// C++: `bool MSRoutingEngine::withTaz()` [static]
    /// whether taz-routing is enabled
    // static bool withTaz() {
    //         return myWithTaz;
    //     }
    #[allow(unused_variables)]
    pub fn with_taz() -> bool {
        todo!("port MSRoutingEngine::withTaz")
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
        todo!("port MSRoutingEngine::addEdgeTravelTime")
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
        todo!("port MSRoutingEngine::saveState")
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
        todo!("port MSRoutingEngine::loadState")
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
        todo!("port MSRoutingEngine::adaptEdgeEfforts")
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
        todo!("port MSRoutingEngine::patchSpeedForTurns")
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
    pub fn init_edge_weights_2(edgeSpeeds: &mut i32, pastEdgeSpeeds: &mut i32) {
        todo!("port MSRoutingEngine::_initEdgeWeights")
    }

    /// C++: `SumoRNG * MSRoutingEngine::getThreadRNG()` [static, private]
    /// returns RNG associated with the current thread
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_thread_rng() -> Ptr<SumoRNG> {
        todo!("port MSRoutingEngine::getThreadRNG")
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
        todo!("port MSRoutingEngine::buildCCHRefVehicle")
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
        todo!("port MSRoutingEngine::initCCH")
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
        todo!("port MSRoutingEngine::invalidateCCHEdge")
    }

    /// C++: `void MSRoutingEngine::MSRoutingEngine(const MSRoutingEngine &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new(arg0: &MSRoutingEngine) -> Self {
        todo!("port MSRoutingEngine::MSRoutingEngine")
    }

    /// C++: `MSRoutingEngine & MSRoutingEngine::operator=(const MSRoutingEngine &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSRoutingEngine) -> &mut MSRoutingEngine {
        todo!("port MSRoutingEngine::operator=")
    }

}

