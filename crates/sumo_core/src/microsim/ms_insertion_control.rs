//! Port of microsim/MSInsertionControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Definition of vehicle flow with the current index for vehicle numbering
/// C++ class `MSInsertionControl::Flow` (microsim/MSInsertionControl.h)
pub struct MSInsertionControl_Flow {
    /// The parameters
    pub pars: Ptr<SUMOVehicleParameter>,
    /// the running index
    pub index: i32,
    /// the type scaling of this flow. Negative value indicates inhomogenous type distribution
    pub scale: f64,
}

impl MSInsertionControl_Flow {
}

/// Inserts vehicles into the network when their departure time is reached
/// C++ class `MSInsertionControl` (microsim/MSInsertionControl.h)
pub struct MSInsertionControl {
    /// The assigned vehicle control (needed for vehicle re-insertion and deletion)
    pub myVehicleControl: Ptr<MSVehicleControl>,
    /// All loaded vehicles sorted by their departure time
    pub myAllVeh: MSVehicleContainer,
    /// Buffers for vehicles that could not be inserted
    pub myPendingEmits: MSVehicleContainer_VehicleVector,
    /// Buffer for vehicles that may be inserted in the current step
    pub myEmitCandidates: BTreeSet<Ptr<SUMOVehicle>>,
    pub myAbortedEmits: BTreeSet<Ptr<SUMOVehicle>>,
    /// Container for periodical vehicle parameters
    pub myFlows: Vec<MSInsertionControl_Flow>,
    /// Cache for periodical vehicle ids and their most recent index for quicker checking
    pub myFlowIDs: BTreeMap<String, i32>,
    /// The maximum waiting time; vehicles waiting longer are deleted (-1: no deletion)
    pub myMaxDepartDelay: SUMOTime,
    /// Whether an edge on which a vehicle could not depart should be ignored in the same step
    pub myEagerInsertionCheck: bool,
    /// Storage for maximum vehicle number
    pub myMaxVehicleNumber: i32,
    /// Last time at which pending emits for each edge where counted
    pub myPendingEmitsUpdateTime: SUMOTime,
    /// the number of pending emits for each edge in the current time step
    pub myPendingEmitsForLane: BTreeMap<Ptr<MSLane>, i32>,
    /// The maximum random offset to be added to vehicles departure times (non-negative)
    pub myMaxRandomDepartOffset: SUMOTime,
    /// A random number generator for probabilistic flows
    pub myFlowRNG: SumoRNG,
}

impl MSInsertionControl {
    /// C++: `void MSInsertionControl::MSInsertionControl(MSVehicleControl &, SUMOTime, bool, int, SUMOTime)`
    /// Constructor
    // MSInsertionControl::MSInsertionControl(MSVehicleControl& vc,
    //                                        SUMOTime maxDepartDelay,
    //                                        bool eagerInsertionCheck,
    //                                        int maxVehicleNumber,
    //                                        SUMOTime randomDepartOffset) :
    //     myVehicleControl(vc),
    //     myMaxDepartDelay(maxDepartDelay),
    //     myEagerInsertionCheck(eagerInsertionCheck),
    //     myMaxVehicleNumber(maxVehicleNumber),
    //     myPendingEmitsUpdateTime(SUMOTime_MIN),
    //     myFlowRNG("flow") {
    //     myMaxRandomDepartOffset = randomDepartOffset;
    //     RandHelper::initRandGlobal(&myFlowRNG);
    // }
    #[allow(unused_variables)]
    pub fn new(vc: &mut MSVehicleControl, maxDepartDelay: SUMOTime, checkEdgesOnce: bool, maxVehicleNumber: i32, randomDepartOffset: SUMOTime) -> Self {
        todo!("port MSInsertionControl::MSInsertionControl")
    }

    /// C++: `void MSInsertionControl::~MSInsertionControl()`
    /// Destructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSInsertionControl::~MSInsertionControl")
    }

    /// C++: `int MSInsertionControl::emitVehicles(SUMOTime)`
    /// Emits vehicles that want to depart at the given time
    // int
    // MSInsertionControl::emitVehicles(SUMOTime time) {
    //     // check whether any vehicles shall be emitted within this time step
    //     const bool havePreChecked = MSRoutingEngine::isEnabled();
    //     if (myPendingEmits.empty() || (havePreChecked && myEmitCandidates.empty())) {
    //         return 0;
    //     }
    //     int numEmitted = 0;
    //     // we use buffering for the refused emits to save time
    //     //  for this, we have two lists; one contains previously refused emits, the second
    //     //  will be used to append those vehicles that will not be able to depart in this
    //     //  time step
    //     MSVehicleContainer::VehicleVector refusedEmits;
    //
    //     // go through the list of previously refused vehicles, first
    //     MSNet::getInstance()->lockPendingEmits();
    //     MSVehicleContainer::VehicleVector::const_iterator veh;
    //     for (veh = myPendingEmits.begin(); veh != myPendingEmits.end(); veh++) {
    //         if (havePreChecked && (myEmitCandidates.count(*veh) == 0)) {
    //             refusedEmits.push_back(*veh);
    //         } else {
    //             numEmitted += tryInsert(time, *veh, refusedEmits);
    //         }
    //     }
    //     myEmitCandidates.clear();
    //     myPendingEmits = refusedEmits;
    //     MSNet::getInstance()->unlockPendingEmits();
    //     return numEmitted;
    // }
    #[allow(unused_variables)]
    pub fn emit_vehicles(&mut self, time: SUMOTime) -> i32 {
        todo!("port MSInsertionControl::emitVehicles")
    }

    /// C++: `void MSInsertionControl::add(SUMOVehicle *)`
    /// Adds a single vehicle for departure
    // void
    // MSInsertionControl::add(SUMOVehicle* veh) {
    //     myAllVeh.add(veh);
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, veh: Ptr<SUMOVehicle>) {
        todo!("port MSInsertionControl::add")
    }

    /// C++: `bool MSInsertionControl::addFlow(SUMOVehicleParameter *const, int)`
    /// Adds parameter for a vehicle flow for departure
    // bool
    // MSInsertionControl::addFlow(SUMOVehicleParameter* const pars, int index) {
    //     if (myFlowIDs.count(pars->id) > 0) {
    //         return false;
    //     }
    //     const bool loadingFromState = index >= 0;
    //     Flow flow{pars, loadingFromState ? index : 0, initScale(pars->vtypeid)};
    //     if (!loadingFromState && pars->repetitionProbability < 0 && pars->repetitionOffset < 0) {
    //         // init poisson flow (but only the timing)
    //         flow.pars->incrementFlow(flow.scale, &myFlowRNG);
    //         flow.pars->repetitionsDone--;
    //     }
    //     myFlows.emplace_back(flow);
    //     myFlowIDs.insert(std::make_pair(pars->id, flow.index));
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn add_flow(&mut self, pars: Ptr<SUMOVehicleParameter>, index: i32) -> bool {
        todo!("port MSInsertionControl::addFlow")
    }

    /// C++: `int MSInsertionControl::getWaitingVehicleNo() const` [const]
    /// Returns the number of waiting vehicles
    // int
    // MSInsertionControl::getWaitingVehicleNo() const {
    //     return (int)myPendingEmits.size();
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_vehicle_no(&self) -> i32 {
        todo!("port MSInsertionControl::getWaitingVehicleNo")
    }

    /// C++: `const MSVehicleContainer::VehicleVector & MSInsertionControl::getPendingVehicles() const` [const]
    /// retrieve vehicles waiting for insertion
    // const MSVehicleContainer::VehicleVector& getPendingVehicles() const {
    //         return myPendingEmits;
    //     }
    #[allow(unused_variables)]
    pub fn get_pending_vehicles(&self) -> &MSVehicleContainer_VehicleVector {
        todo!("port MSInsertionControl::getPendingVehicles")
    }

    /// C++: `int MSInsertionControl::getPendingFlowCount() const` [const]
    /// Returns the number of flows that are still active
    // int
    // MSInsertionControl::getPendingFlowCount() const {
    //     return (int)myFlows.size();
    // }
    #[allow(unused_variables)]
    pub fn get_pending_flow_count(&self) -> i32 {
        todo!("port MSInsertionControl::getPendingFlowCount")
    }

    /// C++: `void MSInsertionControl::alreadyDeparted(SUMOVehicle *)`
    /// stops trying to emit the given vehicle (because it already departed)
    // void
    // MSInsertionControl::alreadyDeparted(SUMOVehicle* veh) {
    //     myPendingEmits.erase(std::remove(myPendingEmits.begin(), myPendingEmits.end(), veh), myPendingEmits.end());
    //     myAllVeh.remove(veh);
    // }
    #[allow(unused_variables)]
    pub fn already_departed(&mut self, veh: Ptr<SUMOVehicle>) {
        todo!("port MSInsertionControl::alreadyDeparted")
    }

    /// C++: `void MSInsertionControl::descheduleDeparture(const SUMOVehicle *)`
    /// stops trying to emit the given vehicle (and delete it)
    // void
    // MSInsertionControl::descheduleDeparture(const SUMOVehicle* veh) {
    //     myAbortedEmits.insert(veh);
    // }
    #[allow(unused_variables)]
    pub fn deschedule_departure(&mut self, veh: Ptr<SUMOVehicle>) {
        todo!("port MSInsertionControl::descheduleDeparture")
    }

    /// C++: `void MSInsertionControl::retractDescheduleDeparture(const SUMOVehicle *)`
    /// reverts a previous call to descheduleDeparture (only needed for departPos="random_free")
    // void
    // MSInsertionControl::retractDescheduleDeparture(const SUMOVehicle* veh) {
    //     myAbortedEmits.erase(veh);
    // }
    #[allow(unused_variables)]
    pub fn retract_deschedule_departure(&mut self, veh: Ptr<SUMOVehicle>) {
        todo!("port MSInsertionControl::retractDescheduleDeparture")
    }

    /// C++: `void MSInsertionControl::clearPendingVehicles(const std::string &)`
    /// clears out all pending vehicles from a route, "" for all routes
    // void
    // MSInsertionControl::clearPendingVehicles(const std::string& route) {
    //     //clear out the refused vehicle list, deleting the vehicles entirely
    //     MSVehicleContainer::VehicleVector::iterator veh;
    //     for (veh = myPendingEmits.begin(); veh != myPendingEmits.end();) {
    //         if ((*veh)->getRoute().getID() == route || route == "") {
    //             myVehicleControl.deleteVehicle(*veh, true);
    //             veh = myPendingEmits.erase(veh);
    //         } else {
    //             ++veh;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_pending_vehicles(&mut self, route: &str) {
        todo!("port MSInsertionControl::clearPendingVehicles")
    }

    /// C++: `void MSInsertionControl::determineCandidates(SUMOTime)`
    /// Checks for all vehicles whether they can be emitted
    // void
    // MSInsertionControl::determineCandidates(SUMOTime time) {
    //     MSVehicleControl& vehControl = MSNet::getInstance()->getVehicleControl();
    //     // for equidistant vehicles, up-scaling is done via repetitionOffset
    //     for (std::vector<Flow>::iterator i = myFlows.begin(); i != myFlows.end();) {
    //         MSVehicleType* vtype = nullptr;
    //         SUMOVehicleParameter* const pars = i->pars;
    //         double typeScale = i->scale;
    //         if (typeScale < 0) {
    //             // must sample from distribution to determine scale value
    //             vtype = vehControl.getVType(pars->vtypeid, MSRouteHandler::getParsingRNG());
    //             typeScale = vtype->getParameter().scale;
    //         }
    //         const double scale = vehControl.getScale() * typeScale;
    //         const long long int scaledRepetitions = pars->repetitionNumber == std::numeric_limits<long long int>::max() ? std::numeric_limits<long long int>::max() :
    //                                                 (long long int)((double)pars->repetitionNumber * scale + 0.5);
    //         bool tryEmitByProb = pars->repetitionProbability > 0;
    //         while (scale > 0 && ((pars->repetitionProbability < 0
    //                               && pars->repetitionsDone < scaledRepetitions
    //                               && pars->depart + pars->repetitionTotalOffset <= time)
    //                              || (tryEmitByProb
    //                                  && pars->depart <= time
    //                                  && pars->repetitionEnd > time
    //                                  // only call rand if all other conditions are met
    //                                  && RandHelper::rand(&myFlowRNG) < (pars->repetitionProbability * TS))
    //                             )) {
    //             tryEmitByProb = false; // only emit one per step
    //             SUMOVehicleParameter* const newPars = new SUMOVehicleParameter(*pars);
    //             newPars->id = pars->id + "." + toString(i->index);
    //             newPars->depart = pars->repetitionProbability > 0 ? time : pars->depart + pars->repetitionTotalOffset + computeRandomDepartOffset();
    //             pars->incrementFlow(scale, &myFlowRNG);
    //             myFlowIDs[pars->id] = i->index;
    //             //std::cout << SIMTIME << " flow=" << pars->id << " done=" << pars->repetitionsDone << " totalOffset=" << STEPS2TIME(pars->repetitionTotalOffset) << "\n";
    //             // try to build the vehicle
    //             if (vehControl.getVehicle(newPars->id) == nullptr) {
    //                 ConstMSRoutePtr const route = MSRoute::dictionary(pars->routeid);
    //                 if (vtype == nullptr) {
    //                     vtype = vehControl.getVType(pars->vtypeid, MSRouteHandler::getParsingRNG());
    //                 }
    //                 SUMOVehicle* const vehicle = vehControl.buildVehicle(newPars, route, vtype, !MSGlobals::gCheckRoutes);
    //                 // for equidistant vehicles, all scaling is done via repetitionOffset (to avoid artefacts, #11441)
    //                 // for probabilistic vehicles, we use the quota
    //                 int quota = pars->repetitionProbability < 0 ? 1 : vehControl.getQuota(scale);
    //                 if (quota > 0) {
    //                     vehControl.addVehicle(newPars->id, vehicle);
    //                     if (pars->departProcedure == DepartDefinition::GIVEN || pars->departProcedure == DepartDefinition::BEGIN) {
    //                         add(vehicle);
    //                     }
    //                     i->index++;
    //                     while (--quota > 0) {
    //                         SUMOVehicleParameter* const quotaPars = new SUMOVehicleParameter(*pars);
    //                         quotaPars->id = pars->id + "." + toString(i->index);
    //                         quotaPars->depart = pars->repetitionProbability > 0 ? time :
    //                                             pars->depart + pars->repetitionsDone * pars->repetitionTotalOffset + computeRandomDepartOffset();
    //                         SUMOVehicle* const quotaVehicle = vehControl.buildVehicle(quotaPars, route, vtype, !MSGlobals::gCheckRoutes);
    //                         vehControl.addVehicle(quotaPars->id, quotaVehicle);
    //                         if (pars->departProcedure == DepartDefinition::GIVEN || pars->departProcedure == DepartDefinition::BEGIN) {
    //                             add(quotaVehicle);
    //                         }
    //                         pars->repetitionsDone++;
    //                         i->index++;
    //                     }
    //                 } else {
    //                     vehControl.deleteVehicle(vehicle, true);
    //                 }
    //             } else {
    //                 if (MSGlobals::gStateLoaded) {
    //                     /// @note probably obsolete since flows save their state
    //                     break;
    //                 }
    //                 throw ProcessError(TLF("Another vehicle with the id '%' exists.", newPars->id));
    //             }
    //             vtype = nullptr;
    //         }
    //         if (time >= pars->repetitionEnd || pars->repetitionsDone >= scaledRepetitions) {
    //             i = myFlows.erase(i);
    //             MSRoute::checkDist(pars->routeid);
    //             delete pars;
    //         } else {
    //             ++i;
    //         }
    //     }
    //     checkCandidates(time, MSRoutingEngine::isEnabled());
    // }
    #[allow(unused_variables)]
    pub fn determine_candidates(&mut self, time: SUMOTime) {
        todo!("port MSInsertionControl::determineCandidates")
    }

    /// C++: `int MSInsertionControl::getPendingEmits(const MSLane *)`
    /// return the number of pending emits for the given lane
    // int
    // MSInsertionControl::getPendingEmits(const MSLane* lane) {
    //     const MSNet* net = MSNet::getInstance();
    //     if (net->getCurrentTimeStep() != myPendingEmitsUpdateTime) {
    //         net->lockPendingEmits();
    //         // updated pending emits (only once per time step)
    //         myPendingEmitsForLane.clear();
    //         for (const SUMOVehicle* const veh : myPendingEmits) {
    //             const MSLane* const vlane = veh->getLane();
    //             if (vlane != nullptr) {
    //                 myPendingEmitsForLane[vlane]++;
    //             } else {
    //                 // no (tentative) departLane was set, increase count for all
    //                 // lanes of the depart edge
    //                 for (const MSLane* const l : veh->getEdge()->getLanes()) {
    //                     myPendingEmitsForLane[l]++;
    //                 }
    //             }
    //         }
    //         myPendingEmitsUpdateTime = MSNet::getInstance()->getCurrentTimeStep();
    //         net->unlockPendingEmits();
    //     }
    //     return myPendingEmitsForLane[lane];
    // }
    #[allow(unused_variables)]
    pub fn get_pending_emits(&mut self, lane: Ptr<MSLane>) -> i32 {
        todo!("port MSInsertionControl::getPendingEmits")
    }

    /// C++: `void MSInsertionControl::adaptIntermodalRouter(MSTransportableRouter &) const` [const]
    // void
    // MSInsertionControl::adaptIntermodalRouter(MSTransportableRouter& router) const {
    //     // fill the public transport router with pre-parsed public transport lines
    //     for (const Flow& f : myFlows) {
    //         if (f.pars->line != "") {
    //             ConstMSRoutePtr const route = MSRoute::dictionary(f.pars->routeid);
    //             router.getNetwork()->addSchedule(*f.pars, route == nullptr ? nullptr : &route->getStops());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn adapt_intermodal_router(&self, router: &mut MSTransportableRouter) {
        todo!("port MSInsertionControl::adaptIntermodalRouter")
    }

    /// C++: `SUMOTime MSInsertionControl::computeRandomDepartOffset() const` [const]
    /// compute (optional) random offset to the departure time
    // SUMOTime
    // MSInsertionControl::computeRandomDepartOffset() const {
    //     if (myMaxRandomDepartOffset > 0) {
    //         // round to the closest usable simulation step
    //         return DELTA_T * ((RandHelper::rand(myMaxRandomDepartOffset, MSRouteHandler::getParsingRNG()) + DELTA_T / 2) / DELTA_T);
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn compute_random_depart_offset(&self) -> SUMOTime {
        todo!("port MSInsertionControl::computeRandomDepartOffset")
    }

    /// C++: `void MSInsertionControl::saveState(OutputDevice &)`
    /// Saves the current state into the given stream
    // void
    // MSInsertionControl::saveState(OutputDevice& out) {
    //     // save flow states
    //     for (const Flow& flow : myFlows) {
    //         flow.pars->write(out, OptionsCont::getOptions(), SUMO_TAG_FLOWSTATE,
    //                          flow.pars->vtypeid == DEFAULT_VTYPE_ID ? "" : flow.pars->vtypeid);
    //         if (flow.pars->repetitionProbability <= 0) {
    //             out.writeAttr(SUMO_ATTR_NEXT, STEPS2TIME(flow.pars->repetitionTotalOffset));
    //         }
    //         out.writeAttr(SUMO_ATTR_ROUTE, flow.pars->routeid);
    //         out.writeAttr(SUMO_ATTR_DONE, flow.pars->repetitionsDone);
    //         out.writeAttr(SUMO_ATTR_INDEX, flow.index);
    //         if (flow.pars->wasSet(VEHPARS_FORCE_REROUTE)) {
    //             out.writeAttr(SUMO_ATTR_REROUTE, true);
    //         }
    //         for (const SUMOVehicleParameter::Stop& stop : flow.pars->stops) {
    //             stop.write(out);
    //         }
    //         out.closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        todo!("port MSInsertionControl::saveState")
    }

    /// C++: `void MSInsertionControl::clearState()`
    /// Remove all vehicles before quick-loading state
    // void
    // MSInsertionControl::clearState() {
    //     for (const Flow& f : myFlows) {
    //         delete (f.pars);
    //     }
    //     myFlows.clear();
    //     myFlowIDs.clear();
    //     myAllVeh.clearState();
    //     myPendingEmits.clear();
    //     myEmitCandidates.clear();
    //     myAbortedEmits.clear();
    //     // myPendingEmitsForLane must not be cleared since it updates itself on the next call
    // }
    #[allow(unused_variables)]
    pub fn clear_state(&mut self) {
        todo!("port MSInsertionControl::clearState")
    }

    /// C++: `SumoRNG * MSInsertionControl::getFlowRNG()`
    /// retrieve internal RNG
    // SumoRNG* getFlowRNG() {
    //         return &myFlowRNG;
    //     }
    #[allow(unused_variables)]
    pub fn get_flow_rng(&mut self) -> Ptr<SumoRNG> {
        todo!("port MSInsertionControl::getFlowRNG")
    }

    /// C++: `bool MSInsertionControl::hasFlow(const std::string &) const` [const]
    /// checks whether the given flow still exists
    // bool hasFlow(const std::string& id) const {
    //         return myFlowIDs.count(id) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn has_flow(&self, id: &str) -> bool {
        todo!("port MSInsertionControl::hasFlow")
    }

    /// C++: `const SUMOVehicleParameter * MSInsertionControl::getFlowPars(const std::string &) const` [const]
    /// return parameters for the given flow
    // const SUMOVehicleParameter*
    // MSInsertionControl::getFlowPars(const std::string& id) const {
    //     if (hasFlow(id)) {
    //         for (const Flow& f : myFlows) {
    //             if (f.pars->id == id) {
    //                 return f.pars;
    //             }
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_flow_pars(&self, id: &str) -> Ptr<SUMOVehicleParameter> {
        todo!("port MSInsertionControl::getFlowPars")
    }

    /// C++: `SUMOVehicle * MSInsertionControl::getLastFlowVehicle(const std::string &) const` [const]
    /// return the last vehicle for the given flow
    // SUMOVehicle*
    // MSInsertionControl::getLastFlowVehicle(const std::string& id) const {
    //     const auto it = myFlowIDs.find(id);
    //     if (it != myFlowIDs.end()) {
    //         const std::string vehID = id + "." + toString(it->second);
    //         return MSNet::getInstance()->getVehicleControl().getVehicle(vehID);
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_last_flow_vehicle(&self, id: &str) -> Ptr<SUMOVehicle> {
        todo!("port MSInsertionControl::getLastFlowVehicle")
    }

    /// C++: `void MSInsertionControl::updateScale(const std::string)`
    /// updates the flow scale value to keep track of TraCI-induced change
    // void
    // MSInsertionControl::updateScale(const std::string vtypeid) {
    //     for (Flow& f : myFlows) {
    //         if (f.pars->vtypeid == vtypeid) {
    //             f.scale = initScale(vtypeid);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_scale(&mut self, vtypeid: String) {
        todo!("port MSInsertionControl::updateScale")
    }

    /// C++: `bool MSInsertionControl::hasTaxiFlow() const` [const]
    // bool
    // MSInsertionControl::hasTaxiFlow() const {
    //     SumoRNG tmp("tmp");
    //     for (const Flow& flow : myFlows) {
    //         if (flow.scale != 0 &&
    //                 (StringUtils::toBool(flow.pars->getParameter("has.taxi.device", "false"))
    //                  || hasTaxiDeviceType(flow.pars->vtypeid, tmp))) {
    //             return true;
    //         }
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn has_taxi_flow(&self) -> bool {
        todo!("port MSInsertionControl::hasTaxiFlow")
    }

    /// C++: `int MSInsertionControl::tryInsert(SUMOTime, SUMOVehicle *, MSVehicleContainer::VehicleVector &)` [private]
    /// Tries to emit the vehicle
    // int
    // MSInsertionControl::tryInsert(SUMOTime time, SUMOVehicle* veh,
    //                               MSVehicleContainer::VehicleVector& refusedEmits) {
    //     assert(veh->getParameter().depart <= time);
    //     const MSEdge& edge = *veh->getEdge();
    //     if (veh->isOnRoad()) {
    //         return 1;
    //     }
    //     if ((myMaxVehicleNumber < 0 || (int)MSNet::getInstance()->getVehicleControl().getRunningVehicleNo() < myMaxVehicleNumber)
    //             && edge.insertVehicle(*veh, time, false, myEagerInsertionCheck || veh->getParameter().departProcedure == DepartDefinition::SPLIT)) {
    //         // Successful insertion
    //         return 1;
    //     }
    //     if (myMaxDepartDelay >= 0 && time - veh->getParameter().depart > myMaxDepartDelay) {
    //         // remove vehicles waiting too long for departure
    //         myVehicleControl.deleteVehicle(veh, true);
    //     } else if (edge.isVaporizing()) {
    //         // remove vehicles if the edge shall be empty
    //         myVehicleControl.deleteVehicle(veh, true);
    //     } else if (myAbortedEmits.count(veh) > 0) {
    //         // remove vehicles which shall not be inserted for some reason
    //         myAbortedEmits.erase(veh);
    //         myVehicleControl.deleteVehicle(veh, true);
    //     } else if ((veh->getRouteValidity(false) & (
    //                     MSBaseVehicle::ROUTE_START_INVALID_LANE
    //                     | MSBaseVehicle::ROUTE_START_INVALID_PERMISSIONS)) != 0) {
    //         myVehicleControl.deleteVehicle(veh, true);
    //     } else {
    //         // let the vehicle wait one step, we'll retry then
    //         refusedEmits.push_back(veh);
    //     }
    //     edge.setLastFailedInsertionTime(time);
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn try_insert(&mut self, time: SUMOTime, veh: Ptr<SUMOVehicle>, refusedEmits: &mut MSVehicleContainer_VehicleVector) -> i32 {
        todo!("port MSInsertionControl::tryInsert")
    }

    /// C++: `void MSInsertionControl::checkCandidates(SUMOTime, const bool)` [private]
    /// Adds all vehicles that should have been emitted earlier to the refuse container
    // void
    // MSInsertionControl::checkCandidates(SUMOTime time, const bool preCheck) {
    //     if (myMaxDepartDelay >= 0) {
    //         for (auto it = myPendingEmits.begin(); it != myPendingEmits.end();) {
    //             SUMOVehicle* veh = *it;
    //             if (time - veh->getParameter().depart > myMaxDepartDelay) {
    //                 // remove vehicles waiting too long for departure
    //                 myVehicleControl.deleteVehicle(veh, true);
    //                 it = myPendingEmits.erase(it);
    //             } else {
    //                 it++;
    //             }
    //         }
    //     }
    //     while (myAllVeh.anyWaitingBefore(time)) {
    //         const MSVehicleContainer::VehicleVector& top = myAllVeh.top();
    //         copy(top.begin(), top.end(), back_inserter(myPendingEmits));
    //         myAllVeh.pop();
    //     }
    //     if (preCheck) {
    //         MSVehicleContainer::VehicleVector::const_iterator veh;
    //         for (veh = myPendingEmits.begin(); veh != myPendingEmits.end(); veh++) {
    //             SUMOVehicle* const v = *veh;
    //             const MSEdge* const edge = v->getEdge();
    //             if (edge->insertVehicle(*v, time, true, myEagerInsertionCheck)) {
    //                 myEmitCandidates.insert(v);
    //             } else {
    //                 MSDevice_Routing* dev = static_cast<MSDevice_Routing*>(v->getDevice(typeid(MSDevice_Routing)));
    //                 if (dev != nullptr) {
    //                     dev->skipRouting(time);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_candidates(&mut self, time: SUMOTime, preCheck: bool) {
        todo!("port MSInsertionControl::checkCandidates")
    }

    /// C++: `double MSInsertionControl::initScale(const std::string)` [static, private]
    /// init scale value of flow
    // double
    // MSInsertionControl::initScale(const std::string vtypeid) {
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     if (vc.hasVTypeDistribution(vtypeid)) {
    //         double result = -1;
    //         const RandomDistributor<MSVehicleType*>* dist = vc.getVTypeDistribution(vtypeid);
    //         for (const MSVehicleType* t : dist->getVals()) {
    //             if (result == -1) {
    //                 result = t->getParameter().scale;
    //             } else if (result != t->getParameter().scale) {
    //                 // unequal scales in distribution
    //                 return -1;
    //             }
    //         }
    //         return result;
    //     } else {
    //         // rng is not used since vtypeid is not a distribution
    //         return vc.getVType(vtypeid, nullptr, true)->getParameter().scale;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_scale(vtypeid: String) -> f64 {
        todo!("port MSInsertionControl::initScale")
    }

    /// C++: `bool MSInsertionControl::hasTaxiDeviceType(const std::string &, SumoRNG &)` [static, private]
    // bool
    // MSInsertionControl::hasTaxiDeviceType(const std::string& vtypeId, SumoRNG& rng) {
    //     MSVehicleControl& vehControl = MSNet::getInstance()->getVehicleControl();
    //     const MSVehicleType* vtype = vehControl.getVType(vtypeId, &rng);
    //     return StringUtils::toBool(vtype->getParameter().getParameter("has.taxi.device", "false"));
    // }
    #[allow(unused_variables)]
    pub fn has_taxi_device_type(vtypeId: &str, rng: &mut SumoRNG) -> bool {
        todo!("port MSInsertionControl::hasTaxiDeviceType")
    }

    /// C++: `void MSInsertionControl::MSInsertionControl(const MSInsertionControl &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSInsertionControl) -> Self {
        todo!("port MSInsertionControl::MSInsertionControl")
    }

    /// C++: `MSInsertionControl & MSInsertionControl::operator=(const MSInsertionControl &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSInsertionControl) -> &mut MSInsertionControl {
        todo!("port MSInsertionControl::operator=")
    }

}

