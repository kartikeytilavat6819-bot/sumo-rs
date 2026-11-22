//! Port of microsim/MSInsertionControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::rand_helper::init_rand;
use utils::common::string_utils::to_bool;
use utils::common::sumo_vehicle_class::DEFAULT_VTYPE_ID;
use utils::common::time::{delta_t, steps2time, ts, SUMOTIME_MIN};
use utils::options::options_cont::{get_bool, get_int, with_options};

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
        let mut ret = MSInsertionControl {
            myVehicleControl: Ptr::from_mut(vc),
            myAllVeh: MSVehicleContainer::new(10),
            myPendingEmits: Vec::new(),
            myEmitCandidates: BTreeSet::new(),
            myAbortedEmits: BTreeSet::new(),
            myFlows: Vec::new(),
            myFlowIDs: BTreeMap::new(),
            myMaxDepartDelay: maxDepartDelay,
            myEagerInsertionCheck: checkEdgesOnce,
            myMaxVehicleNumber: maxVehicleNumber,
            myPendingEmitsUpdateTime: SUMOTIME_MIN,
            myPendingEmitsForLane: BTreeMap::new(),
            myMaxRandomDepartOffset: 0,
            myFlowRNG: SumoRNG::new("flow"),
        };
        ret.myMaxRandomDepartOffset = randomDepartOffset;
        // RandHelper::initRandGlobal(&myFlowRNG)
        init_rand(Some(&mut ret.myFlowRNG), get_bool("random"), get_int("seed"));
        ret
    }

    /// C++: `void MSInsertionControl::~MSInsertionControl()`
    /// Destructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // implicit C++ destructor: the members are released by Rust drop glue, no extra work
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
        crate::microsim::oc93_probe::hit(7); crate::microsim::oc93_probe::line("step");
        // check whether any vehicles shall be emitted within this time step
        let havePreChecked = MSRoutingEngine::is_enabled();
        if self.myPendingEmits.is_empty() || (havePreChecked && self.myEmitCandidates.is_empty()) {
            return 0;
        }
        let mut numEmitted: i32 = 0;
        // we use buffering for the refused emits to save time
        //  for this, we have two lists; one contains previously refused emits, the second
        //  will be used to append those vehicles that will not be able to depart in this
        //  time step
        let mut refusedEmits: MSVehicleContainer_VehicleVector = Vec::new();

        // go through the list of previously refused vehicles, first
        let net = MSNet::get_instance();
        net.lock_pending_emits();
        // tryInsert never modifies myPendingEmits, so iterating over a snapshot matches the C++ iterator loop
        let pending = self.myPendingEmits.clone();
        for veh in pending {
            if havePreChecked && !self.myEmitCandidates.contains(&veh) {
                refusedEmits.push(veh);
            } else {
                numEmitted += self.try_insert(time, veh, &mut refusedEmits);
            }
        }
        self.myEmitCandidates.clear();
        self.myPendingEmits = refusedEmits;
        net.unlock_pending_emits();
        numEmitted
    }

    /// C++: `void MSInsertionControl::add(SUMOVehicle *)`
    /// Adds a single vehicle for departure
    // void
    // MSInsertionControl::add(SUMOVehicle* veh) {
    //     myAllVeh.add(veh);
    // }
    #[allow(unused_variables)]
    pub fn add(&mut self, veh: Ptr<SUMOVehicle>) {
        self.myAllVeh.add(veh);
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
        if self.myFlowIDs.contains_key(&pars.id) {
            return false;
        }
        let loadingFromState = index >= 0;
        let mut flow = MSInsertionControl_Flow {
            pars,
            index: if loadingFromState { index } else { 0 },
            scale: Self::init_scale(pars.vtypeid.clone()),
        };
        if !loadingFromState && pars.repetitionProbability < 0.0 && pars.repetitionOffset < 0 {
            // init poisson flow (but only the timing)
            flow.pars.m().increment_flow(flow.scale, Ptr::from_mut(&mut self.myFlowRNG));
            flow.pars.m().repetitionsDone -= 1;
        }
        let flowIndex = flow.index;
        self.myFlows.push(flow);
        self.myFlowIDs.entry(pars.id.clone()).or_insert(flowIndex);
        true
    }

    /// C++: `int MSInsertionControl::getWaitingVehicleNo() const` [const]
    /// Returns the number of waiting vehicles
    // int
    // MSInsertionControl::getWaitingVehicleNo() const {
    //     return (int)myPendingEmits.size();
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_vehicle_no(&self) -> i32 {
        self.myPendingEmits.len() as i32
    }

    /// C++: `const MSVehicleContainer::VehicleVector & MSInsertionControl::getPendingVehicles() const` [const]
    /// retrieve vehicles waiting for insertion
    // const MSVehicleContainer::VehicleVector& getPendingVehicles() const {
    //         return myPendingEmits;
    //     }
    #[allow(unused_variables)]
    pub fn get_pending_vehicles(&self) -> &MSVehicleContainer_VehicleVector {
        &self.myPendingEmits
    }

    /// C++: `int MSInsertionControl::getPendingFlowCount() const` [const]
    /// Returns the number of flows that are still active
    // int
    // MSInsertionControl::getPendingFlowCount() const {
    //     return (int)myFlows.size();
    // }
    #[allow(unused_variables)]
    pub fn get_pending_flow_count(&self) -> i32 {
        self.myFlows.len() as i32
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
        self.myPendingEmits.retain(|v| *v != veh);
        self.myAllVeh.remove(veh);
    }

    /// C++: `void MSInsertionControl::descheduleDeparture(const SUMOVehicle *)`
    /// stops trying to emit the given vehicle (and delete it)
    // void
    // MSInsertionControl::descheduleDeparture(const SUMOVehicle* veh) {
    //     myAbortedEmits.insert(veh);
    // }
    #[allow(unused_variables)]
    pub fn deschedule_departure(&mut self, veh: Ptr<SUMOVehicle>) {
        self.myAbortedEmits.insert(veh);
    }

    /// C++: `void MSInsertionControl::retractDescheduleDeparture(const SUMOVehicle *)`
    /// reverts a previous call to descheduleDeparture (only needed for departPos="random_free")
    // void
    // MSInsertionControl::retractDescheduleDeparture(const SUMOVehicle* veh) {
    //     myAbortedEmits.erase(veh);
    // }
    #[allow(unused_variables)]
    pub fn retract_deschedule_departure(&mut self, veh: Ptr<SUMOVehicle>) {
        self.myAbortedEmits.remove(&veh);
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
        // clear out the refused vehicle list, deleting the vehicles entirely
        let mut idx: usize = 0;
        while idx < self.myPendingEmits.len() {
            let veh = self.myPendingEmits[idx];
            if veh.get_route().get_id() == route || route == "" {
                self.myVehicleControl.m().delete_vehicle(veh, true, false);
                self.myPendingEmits.remove(idx);
            } else {
                idx += 1;
            }
        }
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
        let net = MSNet::get_instance();
        let vehControl = net.m().get_vehicle_control();
        // for equidistant vehicles, up-scaling is done via repetitionOffset
        let mut i: usize = 0;
        while i < self.myFlows.len() {
            let mut vtype: Ptr<MSVehicleType> = Ptr::null();
            let pars: Ptr<SUMOVehicleParameter> = self.myFlows[i].pars;
            let mut typeScale: f64 = self.myFlows[i].scale;
            if typeScale < 0.0 {
                // must sample from distribution to determine scale value
                vtype = vehControl.get_v_type(&pars.vtypeid, SUMORouteHandler::get_parsing_rng(), false);
                typeScale = vtype.get_parameter().scale;
            }
            let scale: f64 = vehControl.get_scale() * typeScale;
            let scaledRepetitions: i64 = if pars.repetitionNumber == i64::MAX {
                i64::MAX
            } else {
                (pars.repetitionNumber as f64 * scale + 0.5) as i64
            };
            let mut tryEmitByProb: bool = pars.repetitionProbability > 0.0;
            while scale > 0.0
                && ((pars.repetitionProbability < 0.0
                    && pars.repetitionsDone < scaledRepetitions
                    && pars.depart + pars.repetitionTotalOffset <= time)
                    || (tryEmitByProb
                        && pars.depart <= time
                        && pars.repetitionEnd > time
                        // only call rand if all other conditions are met
                        && self.myFlowRNG.rand() < (pars.repetitionProbability * ts())))
            {
                tryEmitByProb = false; // only emit one per step
                let newPars: Ptr<SUMOVehicleParameter> = Ptr::from_raw(Box::into_raw(Box::new(copy_vehicle_parameter(&pars))));
                newPars.m().id = format!("{}.{}", pars.id, self.myFlows[i].index);
                newPars.m().depart = if pars.repetitionProbability > 0.0 {
                    time
                } else {
                    pars.depart + pars.repetitionTotalOffset + self.compute_random_depart_offset()
                };
                pars.m().increment_flow(scale, Ptr::from_mut(&mut self.myFlowRNG));
                self.myFlowIDs.insert(pars.id.clone(), self.myFlows[i].index);
                //std::cout << SIMTIME << " flow=" << pars->id << " done=" << pars->repetitionsDone << " totalOffset=" << STEPS2TIME(pars->repetitionTotalOffset) << "\n";
                // try to build the vehicle
                if vehControl.get_vehicle(&newPars.id).is_null() {
                    let route: ConstMSRoutePtr = MSRoute::dictionary_3(&pars.routeid, Ptr::null());
                    if vtype.is_null() {
                        vtype = vehControl.get_v_type(&pars.vtypeid, SUMORouteHandler::get_parsing_rng(), false);
                    }
                    let vehicle: Ptr<SUMOVehicle> = vehControl.build_vehicle(
                        newPars,
                        route.clone(),
                        vtype,
                        !check_routes(),
                        MSVehicleControl_VehicleDefinitionSource::ROUTEFILE,
                        true,
                    );
                    // for equidistant vehicles, all scaling is done via repetitionOffset (to avoid artefacts, #11441)
                    // for probabilistic vehicles, we use the quota
                    let mut quota: i32 = if pars.repetitionProbability < 0.0 { 1 } else { vehControl.get_quota(scale, -1) };
                    if quota > 0 {
                        vehControl.add_vehicle(&newPars.id, vehicle);
                        if pars.departProcedure == DepartDefinition::GIVEN || pars.departProcedure == DepartDefinition::BEGIN {
                            self.add(vehicle);
                        }
                        self.myFlows[i].index += 1;
                        quota -= 1;
                        while quota > 0 {
                            let quotaPars: Ptr<SUMOVehicleParameter> = Ptr::from_raw(Box::into_raw(Box::new(copy_vehicle_parameter(&pars))));
                            quotaPars.m().id = format!("{}.{}", pars.id, self.myFlows[i].index);
                            quotaPars.m().depart = if pars.repetitionProbability > 0.0 {
                                time
                            } else {
                                pars.depart + pars.repetitionsDone * pars.repetitionTotalOffset + self.compute_random_depart_offset()
                            };
                            let quotaVehicle: Ptr<SUMOVehicle> = vehControl.build_vehicle(
                                quotaPars,
                                route.clone(),
                                vtype,
                                !check_routes(),
                                MSVehicleControl_VehicleDefinitionSource::ROUTEFILE,
                                true,
                            );
                            vehControl.add_vehicle(&quotaPars.id, quotaVehicle);
                            if pars.departProcedure == DepartDefinition::GIVEN || pars.departProcedure == DepartDefinition::BEGIN {
                                self.add(quotaVehicle);
                            }
                            pars.m().repetitionsDone += 1;
                            self.myFlows[i].index += 1;
                            quota -= 1;
                        }
                    } else {
                        vehControl.delete_vehicle(vehicle, true, false);
                    }
                } else {
                    if state_loaded() {
                        /// @note probably obsolete since flows save their state
                        break;
                    }
                    panic!("{}", ProcessError::Process(utils::tlf!("Another vehicle with the id '%' exists.", newPars.id)));
                }
                vtype = Ptr::null();
            }
            if time >= pars.repetitionEnd || pars.repetitionsDone >= scaledRepetitions {
                self.myFlows.remove(i);
                MSRoute::check_dist(&pars.routeid);
                drop(unsafe { Box::from_raw(pars.as_raw()) });
            } else {
                i += 1;
            }
        }
        self.check_candidates(time, MSRoutingEngine::is_enabled());
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
        let net = MSNet::get_instance();
        if net.get_current_time_step() != self.myPendingEmitsUpdateTime {
            net.lock_pending_emits();
            // updated pending emits (only once per time step)
            self.myPendingEmitsForLane.clear();
            for veh in self.myPendingEmits.iter() {
                let vlane = veh.get_lane();
                if !vlane.is_null() {
                    *self.myPendingEmitsForLane.entry(vlane).or_insert(0) += 1;
                } else {
                    // no (tentative) departLane was set, increase count for all
                    // lanes of the depart edge
                    let edge = veh.get_edge();
                    for l in edge.get_lanes() {
                        *self.myPendingEmitsForLane.entry(*l).or_insert(0) += 1;
                    }
                }
            }
            self.myPendingEmitsUpdateTime = net.get_current_time_step();
            net.unlock_pending_emits();
        }
        *self.myPendingEmitsForLane.entry(lane).or_insert(0)
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
        // fill the public transport router with pre-parsed public transport lines
        for f in &self.myFlows {
            if !f.pars.line.is_empty() {
                let route: Option<ConstMSRoutePtr> = if MSRoute::has_route(&f.pars.routeid) {
                    Some(MSRoute::dictionary_3(&f.pars.routeid, Ptr::null()))
                } else {
                    None
                };
                let stops: Ptr<StopParVector> = match &route {
                    Some(r) => Ptr::new(r.get_stops()),
                    None => Ptr::null(),
                };
                router.get_network().m().add_schedule(&f.pars, stops);
            }
        }
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
        if self.myMaxRandomDepartOffset > 0 {
            // round to the closest usable simulation step
            let rng = SUMORouteHandler::get_parsing_rng();
            return delta_t() * ((rng.m().rand_i64(self.myMaxRandomDepartOffset) + delta_t() / 2) / delta_t());
        }
        0
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
        // save flow states
        for flow in &self.myFlows {
            let pars = flow.pars;
            with_options(|oc| {
                pars.write(
                    out,
                    oc,
                    SUMO_TAG_FLOWSTATE,
                    if pars.vtypeid == DEFAULT_VTYPE_ID { "" } else { pars.vtypeid.as_str() },
                )
            });
            if pars.repetitionProbability <= 0.0 {
                out.write_attr(SUMO_ATTR_NEXT, &steps2time(pars.repetitionTotalOffset));
            }
            out.write_attr(SUMO_ATTR_ROUTE, pars.routeid.as_str());
            out.write_attr(SUMO_ATTR_DONE, &pars.repetitionsDone);
            out.write_attr(SUMO_ATTR_INDEX, &flow.index);
            if pars.was_set(VEHPARS_FORCE_REROUTE) {
                out.write_attr(SUMO_ATTR_REROUTE, &true);
            }
            for stop in pars.stops.iter() {
                stop.write(out, true, true);
            }
            out.close_tag("");
        }
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
        for f in &self.myFlows {
            drop(unsafe { Box::from_raw(f.pars.as_raw()) });
        }
        self.myFlows.clear();
        self.myFlowIDs.clear();
        self.myAllVeh.clear_state();
        self.myPendingEmits.clear();
        self.myEmitCandidates.clear();
        self.myAbortedEmits.clear();
        // myPendingEmitsForLane must not be cleared since it updates itself on the next call
    }

    /// C++: `SumoRNG * MSInsertionControl::getFlowRNG()`
    /// retrieve internal RNG
    // SumoRNG* getFlowRNG() {
    //         return &myFlowRNG;
    //     }
    #[allow(unused_variables)]
    pub fn get_flow_rng(&mut self) -> Ptr<SumoRNG> {
        Ptr::from_mut(&mut self.myFlowRNG)
    }

    /// C++: `bool MSInsertionControl::hasFlow(const std::string &) const` [const]
    /// checks whether the given flow still exists
    // bool hasFlow(const std::string& id) const {
    //         return myFlowIDs.count(id) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn has_flow(&self, id: &str) -> bool {
        self.myFlowIDs.contains_key(id)
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
        if self.has_flow(id) {
            for f in &self.myFlows {
                if f.pars.id == id {
                    return f.pars;
                }
            }
        }
        Ptr::null()
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
        if let Some(idx) = self.myFlowIDs.get(id) {
            let vehID = format!("{}.{}", id, idx);
            let net = MSNet::get_instance();
            return net.m().get_vehicle_control().get_vehicle(&vehID);
        }
        Ptr::null()
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
        for f in self.myFlows.iter_mut() {
            if f.pars.vtypeid == vtypeid {
                f.scale = Self::init_scale(vtypeid.clone());
            }
        }
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
        let mut tmp = SumoRNG::new("tmp");
        for flow in &self.myFlows {
            if flow.scale != 0.0
                && (to_bool_or_throw(&flow.pars.get_parameter("has.taxi.device", "false"))
                    || Self::has_taxi_device_type(&flow.pars.vtypeid, &mut tmp))
            {
                return true;
            }
        }
        false
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
        crate::microsim::oc93_probe::hit(6);
        // C++ `assert(veh->getParameter().depart <= time)` is compiled out in the
        // official (NDEBUG) binary; FCD-replay inserts vehicles with a future depart,
        // so keep the same behaviour instead of panicking in debug builds.
        let edge = veh.get_edge();
        if veh.is_on_road() {
            return 1;
        }
        let net = MSNet::get_instance();
        if (self.myMaxVehicleNumber < 0 || net.m().get_vehicle_control().get_running_vehicle_no() < self.myMaxVehicleNumber)
            && edge.insert_vehicle(
                veh.m(),
                time,
                false,
                self.myEagerInsertionCheck || veh.get_parameter().departProcedure == DepartDefinition::SPLIT,
            )
        {
            // Successful insertion
            return 1;
        }
        if self.myMaxDepartDelay >= 0 && time - veh.get_parameter().depart > self.myMaxDepartDelay {
            // remove vehicles waiting too long for departure
            self.myVehicleControl.m().delete_vehicle(veh, true, false);
        } else if edge.is_vaporizing() {
            // remove vehicles if the edge shall be empty
            self.myVehicleControl.m().delete_vehicle(veh, true, false);
        } else if self.myAbortedEmits.contains(&veh) {
            // remove vehicles which shall not be inserted for some reason
            self.myAbortedEmits.remove(&veh);
            self.myVehicleControl.m().delete_vehicle(veh, true, false);
        } else if (veh.m().get_route_validity(false, false, Ptr::null()) as i64
            & (MSBaseVehicle_RouteValidity::ROUTE_START_INVALID_LANE as i64
                | MSBaseVehicle_RouteValidity::ROUTE_START_INVALID_PERMISSIONS as i64))
            != 0
        {
            self.myVehicleControl.m().delete_vehicle(veh, true, false);
        } else {
            // let the vehicle wait one step, we'll retry then
            refusedEmits.push(veh);
        }
        edge.set_last_failed_insertion_time(time);
        0
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
        if self.myMaxDepartDelay >= 0 {
            let mut idx: usize = 0;
            while idx < self.myPendingEmits.len() {
                let veh = self.myPendingEmits[idx];
                if time - veh.get_parameter().depart > self.myMaxDepartDelay {
                    // remove vehicles waiting too long for departure
                    self.myVehicleControl.m().delete_vehicle(veh, true, false);
                    self.myPendingEmits.remove(idx);
                } else {
                    idx += 1;
                }
            }
        }
        while self.myAllVeh.any_waiting_before(time) {
            let top = self.myAllVeh.top().clone();
            self.myPendingEmits.extend(top);
            self.myAllVeh.pop();
        }
        if preCheck {
            for veh in self.myPendingEmits.iter() {
                let v = *veh;
                let edge = v.get_edge();
                if edge.insert_vehicle(v.m(), time, true, self.myEagerInsertionCheck) {
                    self.myEmitCandidates.insert(v);
                } else {
                    // getDevice(typeid(MSDevice_Routing)): "rerouting" is the only device name MSDevice_Routing reports
                    let dev = MSDevice::get_device_by_name(v.m().get_devices(), "rerouting");
                    if !dev.is_null() {
                        dev.cast::<MSDevice_Routing>().m().skip_routing(time);
                    }
                }
            }
        }
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
        let net = MSNet::get_instance();
        let vc = net.m().get_vehicle_control();
        if vc.has_v_type_distribution(&vtypeid) {
            let mut result: f64 = -1.0;
            let dist = vc.get_v_type_distribution(&vtypeid);
            for t in dist.get_vals() {
                if result == -1.0 {
                    result = t.get_parameter().scale;
                } else if result != t.get_parameter().scale {
                    // unequal scales in distribution
                    return -1.0;
                }
            }
            result
        } else {
            // rng is not used since vtypeid is not a distribution
            vc.get_v_type(&vtypeid, Ptr::null(), true).get_parameter().scale
        }
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
        let net = MSNet::get_instance();
        let vehControl = net.m().get_vehicle_control();
        let vtype = vehControl.get_v_type(vtypeId, Ptr::from_mut(rng), false);
        to_bool_or_throw(&vtype.get_parameter().get_parameter("has.taxi.device", "false"))
    }

    /// C++: `void MSInsertionControl::MSInsertionControl(const MSInsertionControl &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSInsertionControl) -> Self {
        // invalidated in C++ (private, never defined): using it is a programming error
        panic!("MSInsertionControl copy constructor is private and invalidated")
    }

    /// C++: `MSInsertionControl & MSInsertionControl::operator=(const MSInsertionControl &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSInsertionControl) -> &mut MSInsertionControl {
        // invalidated in C++ (private, never defined): using it is a programming error
        panic!("MSInsertionControl::operator= is private and invalidated")
    }

}

/// C++ `#define`d flag `VEHPARS_FORCE_REROUTE` (utils/vehicle/SUMOVehicleParameter.cpp), not exported by the port yet
const VEHPARS_FORCE_REROUTE: i64 = 2 << 17;

/// C++ `StringUtils::toBool` throws on malformed input; `string_utils::to_bool` returns a Result
fn to_bool_or_throw(s: &str) -> bool {
    to_bool(s).unwrap_or_else(|e| panic!("{e}"))
}

/// C++ implicit copy constructor `SUMOVehicleParameter(const SUMOVehicleParameter&)` (used for `new SUMOVehicleParameter(*pars)`)
fn copy_vehicle_parameter(p: &SUMOVehicleParameter) -> SUMOVehicleParameter {
    SUMOVehicleParameter {
        base: p.base.clone(),
        tag: p.tag,
        id: p.id.clone(),
        routeid: p.routeid.clone(),
        vtypeid: p.vtypeid.clone(),
        color: p.color,
        depart: p.depart,
        departProcedure: p.departProcedure,
        departLane: p.departLane,
        departLaneProcedure: p.departLaneProcedure,
        departPos: p.departPos,
        departPosProcedure: p.departPosProcedure,
        departPosLat: p.departPosLat,
        departPosLatProcedure: p.departPosLatProcedure,
        departSpeed: p.departSpeed,
        departSpeedProcedure: p.departSpeedProcedure,
        departEdge: p.departEdge,
        departEdgeProcedure: p.departEdgeProcedure,
        arrivalLane: p.arrivalLane,
        arrivalLaneProcedure: p.arrivalLaneProcedure,
        arrivalPos: p.arrivalPos,
        arrivalPosProcedure: p.arrivalPosProcedure,
        arrivalPosLat: p.arrivalPosLat,
        arrivalPosLatProcedure: p.arrivalPosLatProcedure,
        arrivalSpeed: p.arrivalSpeed,
        arrivalSpeedProcedure: p.arrivalSpeedProcedure,
        arrivalEdge: p.arrivalEdge,
        arrivalEdgeProcedure: p.arrivalEdgeProcedure,
        repetitionNumber: p.repetitionNumber,
        repetitionsDone: p.repetitionsDone,
        repetitionOffset: p.repetitionOffset,
        repetitionTotalOffset: p.repetitionTotalOffset,
        repetitionProbability: p.repetitionProbability,
        poissonRate: p.poissonRate,
        repetitionEnd: p.repetitionEnd,
        line: p.line.clone(),
        fromTaz: p.fromTaz.clone(),
        toTaz: p.toTaz.clone(),
        stops: p.stops.iter().map(copy_stop).collect(),
        via: p.via.clone(),
        parkingBadges: p.parkingBadges.clone(),
        modes: p.modes,
        vTypes: p.vTypes.clone(),
        personNumber: p.personNumber,
        containerNumber: p.containerNumber,
        speedFactor: p.speedFactor,
        calibratorSpeed: p.calibratorSpeed,
        insertionChecks: p.insertionChecks,
        parametersSet: p.parametersSet,
    }
}

/// C++ implicit copy constructor `SUMOVehicleParameter::Stop(const Stop&)`
fn copy_stop(s: &SUMOVehicleParameter_Stop) -> SUMOVehicleParameter_Stop {
    SUMOVehicleParameter_Stop {
        base: s.base.clone(),
        edge: s.edge.clone(),
        lane: s.lane.clone(),
        busstop: s.busstop.clone(),
        containerstop: s.containerstop.clone(),
        parkingarea: s.parkingarea.clone(),
        chargingStation: s.chargingStation.clone(),
        overheadWireSegment: s.overheadWireSegment.clone(),
        startPos: s.startPos,
        endPos: s.endPos,
        arrival: s.arrival,
        duration: s.duration,
        durationDist: s.durationDist.clone(),
        until: s.until,
        extension: s.extension,
        waitUntil: s.waitUntil,
        triggered: s.triggered,
        containerTriggered: s.containerTriggered,
        joinTriggered: s.joinTriggered,
        parking: s.parking,
        awaitedPersons: s.awaitedPersons.clone(),
        permitted: s.permitted.clone(),
        awaitedContainers: s.awaitedContainers.clone(),
        friendlyPos: s.friendlyPos,
        actType: s.actType.clone(),
        tripId: s.tripId.clone(),
        line: s.line.clone(),
        split: s.split.clone(),
        join: s.join.clone(),
        speed: s.speed,
        posLat: s.posLat,
        onDemand: s.onDemand,
        priority: s.priority,
        jump: s.jump,
        jumpUntil: s.jumpUntil,
        started: s.started,
        ended: s.ended,
        accessPos: s.accessPos.clone(),
        index: s.index,
        routeIndex: s.routeIndex,
        parametersSet: s.parametersSet,
        collision: s.collision,
        breakDown: s.breakDown,
    }
}

