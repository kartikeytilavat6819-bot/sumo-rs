//! Port of microsim/MSEdgeControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::options::options_cont;
use utils::options::options_cont::with_options;

/// C++ MIN2(a, b) = a < b ? a : b
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

/// C++ typedef `std::vector<LaneUsage> MSEdgeControl::LaneUsageVector`
pub type MSEdgeControl_LaneUsageVector = Vec<MSEdgeControl_LaneUsage>;

/// A structure holding some basic information about a simulated lane
/// C++ class `MSEdgeControl::LaneUsage` (microsim/MSEdgeControl.h)
pub struct MSEdgeControl_LaneUsage {
    /// The described lane
    pub lane: Ptr<MSLane>,
    /// Information whether this lane is active
    pub amActive: bool,
    /// Information whether this lane belongs to a multi-lane edge
    pub haveNeighbors: bool,
}

impl MSEdgeControl_LaneUsage {
}

/// C++ `StopWatch<std::chrono::nanoseconds>` (utils/common/StopWatch.h), history container only
// shortcut: timing calibration and statistics output are only used under PARALLEL_STOPWATCH (compiled out in the default build), upgrade if profiling output is needed
#[derive(Default)]
pub struct MSEdgeControl_StopWatch {
    pub history: Vec<i64>,
}

/// Stores edges and lanes, performs moving of vehicle
/// C++ class `MSEdgeControl` (microsim/MSEdgeControl.h)
pub struct MSEdgeControl {
    /// Loaded edges
    pub myEdges: MSEdgeVector,
    /// Information about lanes' number of vehicles and neighbors
    pub myLanes: MSEdgeControl_LaneUsageVector,
    /// The list of active (not empty) lanes
    pub myActiveLanes: VecDeque<Ptr<MSLane>>,
    /// A storage for lanes which shall be integrated because vehicles have moved onto them
    pub myWithVehicles2Integrate: Vec<Ptr<MSLane>>, // C++: MFXSynchQue<MSLane *, std::vector<MSLane *>>
    /// Lanes which changed the state without informing the control
    pub myChangedStateLanes: BTreeMap<i32, Ptr<MSLane>>, // C++: std::set<MSLane *, ComparatorNumericalIdLess>, keyed by numerical id
    /// The list of active (not empty) lanes
    pub myLastLaneChange: Vec<SUMOTime>,
    /// Additional lanes for which collision checking must be performed
    pub myInactiveCheckCollisions: BTreeMap<i32, Ptr<MSLane>>, // C++: MFXSynchSet<MSLane *, std::set<MSLane *, ComparatorNumericalIdLess>>, keyed by numerical id
    pub myMinLengthGeometryFactor: f64,
    pub myStopWatch: Vec<MSEdgeControl_StopWatch>,
}

impl MSEdgeControl {
    /// C++: `void MSEdgeControl::MSEdgeControl(const std::vector<MSEdge *> &)`
    /// Constructor
    // MSEdgeControl::MSEdgeControl(const std::vector< MSEdge* >& edges)
    //     : myEdges(edges),
    //       myLanes(MSLane::dictSize()),
    //       myWithVehicles2Integrate(MSGlobals::gNumSimThreads > 1),
    //       myLastLaneChange(edges.size()),
    //       myInactiveCheckCollisions(MSGlobals::gNumSimThreads > 1),
    //       myMinLengthGeometryFactor(1.),
    // #ifdef THREAD_POOL
    //       myThreadPool(false, std::vector<int>(MSGlobals::gNumThreads, 0)),
    // #endif
    //       myStopWatch(3) {
    //     // build the usage definitions for lanes
    //     for (MSEdge* const edge : myEdges) {
    //         const std::vector<MSLane*>& lanes = edge->getLanes();
    //         if (!edge->hasLaneChanger()) {
    //             const int pos = lanes.front()->getNumericalID();
    //             myLanes[pos].lane = lanes.front();
    //             myLanes[pos].amActive = false;
    //             myLanes[pos].haveNeighbors = false;
    //             myMinLengthGeometryFactor = MIN2(edge->getLengthGeometryFactor(), myMinLengthGeometryFactor);
    //         } else {
    //             for (MSLane* const l : lanes) {
    //                 const int pos = l->getNumericalID();
    //                 myLanes[pos].lane = l;
    //                 myLanes[pos].amActive = false;
    //                 myLanes[pos].haveNeighbors = true;
    //                 myMinLengthGeometryFactor = MIN2(l->getLengthGeometryFactor(), myMinLengthGeometryFactor);
    //             }
    //             myLastLaneChange[edge->getNumericalID()] = -1;
    //         }
    //     }
    // #ifndef THREAD_POOL
    // #ifdef HAVE_FOX
    //     if (MSGlobals::gNumThreads > 1) {
    //         while (myThreadPool.size() < MSGlobals::gNumThreads) {
    //             new WorkerThread(myThreadPool);
    //         }
    //     }
    // #endif
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn new(edges: &Vec<Ptr<MSEdge>>) -> Self {
        // shortcut: worker thread pool (THREAD_POOL / HAVE_FOX) is not ported, all lane work runs serially, upgrade when ThreadPool lands
        let mut myLanes: MSEdgeControl_LaneUsageVector = (0..MSLane::dict_size())
            .map(|_| MSEdgeControl_LaneUsage { lane: Ptr::null(), amActive: false, haveNeighbors: false })
            .collect();
        let mut myMinLengthGeometryFactor: f64 = 1.;
        let mut myLastLaneChange: Vec<SUMOTime> = vec![0; edges.len()];
        // build the usage definitions for lanes
        for edge in edges.iter() {
            let lanes = edge.get_lanes();
            if !edge.has_lane_changer() {
                let pos = lanes[0].get_numerical_id() as usize;
                myLanes[pos].lane = lanes[0];
                myLanes[pos].amActive = false;
                myLanes[pos].haveNeighbors = false;
                myMinLengthGeometryFactor = min2(edge.get_length_geometry_factor(), myMinLengthGeometryFactor);
            } else {
                for l in lanes.iter() {
                    let pos = l.get_numerical_id() as usize;
                    myLanes[pos].lane = *l;
                    myLanes[pos].amActive = false;
                    myLanes[pos].haveNeighbors = true;
                    myMinLengthGeometryFactor = min2(l.get_length_geometry_factor_2(), myMinLengthGeometryFactor);
                }
                myLastLaneChange[edge.get_numerical_id() as usize] = -1;
            }
        }
        MSEdgeControl {
            myEdges: edges.clone(),
            myLanes,
            myActiveLanes: VecDeque::new(),
            myWithVehicles2Integrate: Vec::new(),
            myChangedStateLanes: BTreeMap::new(),
            myLastLaneChange,
            myInactiveCheckCollisions: BTreeMap::new(),
            myMinLengthGeometryFactor,
            myStopWatch: (0..3).map(|_| MSEdgeControl_StopWatch::default()).collect(),
        }
    }

    /// C++: `void MSEdgeControl::~MSEdgeControl()`
    /// Destructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: thread pool is not ported (work runs serially), so there are no workers to clear; PARALLEL_STOPWATCH output is compiled out in the default build
    }

    /// C++: `void MSEdgeControl::patchActiveLanes()`
    /// Resets information whether a lane is active for all lanes
    // void
    // MSEdgeControl::patchActiveLanes() {
    //     for (std::set<MSLane*, ComparatorNumericalIdLess>::iterator i = myChangedStateLanes.begin(); i != myChangedStateLanes.end(); ++i) {
    //         LaneUsage& lu = myLanes[(*i)->getNumericalID()];
    //         // if the lane was inactive but is now...
    //         if (!lu.amActive && (*i)->getVehicleNumber() > 0) {
    //             // ... add to active lanes and mark as such
    //             if (lu.haveNeighbors) {
    //                 myActiveLanes.push_front(*i);
    //             } else {
    //                 myActiveLanes.push_back(*i);
    //             }
    //             lu.amActive = true;
    //         }
    //     }
    //     myChangedStateLanes.clear();
    // }
    #[allow(unused_variables)]
    pub fn patch_active_lanes(&mut self) {
        let changed: Vec<Ptr<MSLane>> = self.myChangedStateLanes.values().copied().collect();
        for lane in changed {
            let lu = &mut self.myLanes[lane.get_numerical_id() as usize];
            // if the lane was inactive but is now...
            if !lu.amActive && lane.get_vehicle_number() > 0 {
                // ... add to active lanes and mark as such
                if lu.haveNeighbors {
                    self.myActiveLanes.push_front(lane);
                } else {
                    self.myActiveLanes.push_back(lane);
                }
                lu.amActive = true;
            }
        }
        self.myChangedStateLanes.clear();
    }

    /// C++: `void MSEdgeControl::planMovements(SUMOTime)`
    /// Compute safe velocities for all vehicles based on positions and speeds from the last time step. Also registers ApproachingVehicleInformation for all links
    // void
    // MSEdgeControl::planMovements(SUMOTime t) {
    // #ifdef PARALLEL_STOPWATCH
    //     myStopWatch[0].start();
    // #endif
    // #ifdef THREAD_POOL
    //     std::vector<std::future<void>> results;
    // #endif
    //     for (std::list<MSLane*>::iterator i = myActiveLanes.begin(); i != myActiveLanes.end();) {
    //         const int vehNum = (*i)->getVehicleNumber();
    //         if (vehNum == 0) {
    //             myLanes[(*i)->getNumericalID()].amActive = false;
    //             i = myActiveLanes.erase(i);
    //         } else {
    // #ifdef THREAD_POOL
    //             if (MSGlobals::gNumSimThreads > 1) {
    //                 results.push_back(myThreadPool.executeAsync([i, t](int) {
    //                     (*i)->planMovements(t);
    //                 }, (*i)->getRNGIndex() % MSGlobals::gNumSimThreads));
    //                 ++i;
    //                 continue;
    //             }
    // #else
    // #ifdef HAVE_FOX
    //             if (MSGlobals::gNumSimThreads > 1) {
    //                 myThreadPool.add((*i)->getPlanMoveTask(t), (*i)->getRNGIndex() % myThreadPool.size());
    //                 ++i;
    //                 continue;
    //             }
    // #endif
    // #endif
    //             (*i)->planMovements(t);
    //             ++i;
    //         }
    //     }
    // #ifdef THREAD_POOL
    //     for (auto& r : results) {
    //         r.wait();
    //     }
    // #else
    // #ifdef HAVE_FOX
    //     if (MSGlobals::gNumSimThreads > 1) {
    //         myThreadPool.waitAll(false);
    //     }
    // #endif
    // #endif
    // #ifdef PARALLEL_STOPWATCH
    //     myStopWatch[0].stop();
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn plan_movements(&mut self, t: SUMOTime) {
        // shortcut: multi-threaded planning (gNumSimThreads > 1) is not ported, lanes are planned serially, upgrade when ThreadPool lands
        let mut i = 0;
        while i < self.myActiveLanes.len() {
            let lane = self.myActiveLanes[i];
            let vehNum = lane.get_vehicle_number();
            if vehNum == 0 {
                self.myLanes[lane.get_numerical_id() as usize].amActive = false;
                self.myActiveLanes.remove(i);
            } else {
                lane.m().plan_movements(t);
                i += 1;
            }
        }
    }

    /// C++: `void MSEdgeControl::setJunctionApproaches()`
    /// Register junction approaches for all vehicles after velocities have been planned. This is a prerequisite for executeMovements
    // void
    // MSEdgeControl::setJunctionApproaches() {
    //     for (MSLane* const lane : myActiveLanes) {
    //         lane->setJunctionApproaches();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_junction_approaches(&mut self) {
        for lane in self.myActiveLanes.iter() {
            lane.set_junction_approaches();
        }
    }

    /// C++: `void MSEdgeControl::executeMovements(SUMOTime)`
    /// Executes planned vehicle movements with regards to right-of-way
    // void
    // MSEdgeControl::executeMovements(SUMOTime t) {
    // #ifdef PARALLEL_STOPWATCH
    //     myStopWatch[1].start();
    // #endif
    //     std::vector<MSLane*> wasActive(myActiveLanes.begin(), myActiveLanes.end());
    //     myWithVehicles2Integrate.clear();
    // #ifdef PARALLEL_EXEC_MOVE
    // #ifdef THREAD_POOL
    //     if (MSGlobals::gNumSimThreads > 1) {
    //         for (MSLane* const lane : myActiveLanes) {
    //             myThreadPool.executeAsync([lane, t](int) {
    //                 lane->executeMovements(t);
    //             }, lane->getRNGIndex() % MSGlobals::gNumSimThreads);
    //         }
    //         myThreadPool.waitAll();
    //     }
    // #else
    // #ifdef HAVE_FOX
    //     if (MSGlobals::gNumSimThreads > 1) {
    //         for (MSLane* const lane : myActiveLanes) {
    //             myThreadPool.add(lane->getExecuteMoveTask(t), lane->getRNGIndex() % myThreadPool.size());
    //         }
    //         myThreadPool.waitAll(false);
    //     }
    // #endif
    // #endif
    // #endif
    //     for (std::list<MSLane*>::iterator i = myActiveLanes.begin(); i != myActiveLanes.end();) {
    //         if (
    // #ifdef PARALLEL_EXEC_MOVE
    //             MSGlobals::gNumSimThreads <= 1 &&
    // #endif
    //             (*i)->getVehicleNumber() > 0) {
    //             (*i)->executeMovements(t);
    //         }
    //         if ((*i)->getVehicleNumber() == 0) {
    //             myLanes[(*i)->getNumericalID()].amActive = false;
    //             i = myActiveLanes.erase(i);
    //         } else {
    //             ++i;
    //         }
    //     }
    //     for (MSLane* lane : wasActive) {
    //         lane->updateLengthSum();
    //     }
    //     // arrived vehicles should not influence lane changing
    //     MSNet::getInstance()->getVehicleControl().removePending();
    //     std::vector<MSLane*>& toIntegrate = myWithVehicles2Integrate.getContainer();
    //     std::sort(toIntegrate.begin(), toIntegrate.end(), ComparatorIdLess());
    //     /// @todo: sorting only needed to account for lane-ordering dependencies.
    //     //This should disappear when parallelization is working. Until then it would
    //     //be better to use ComparatorNumericalIdLess instead of ComparatorIdLess
    //     myWithVehicles2Integrate.unlock();
    //     for (MSLane* const lane : toIntegrate) {
    //         const bool wasInactive = lane->getVehicleNumber() == 0;
    //         lane->integrateNewVehicles();
    //         if (wasInactive && lane->getVehicleNumber() > 0) {
    //             LaneUsage& lu = myLanes[lane->getNumericalID()];
    //             if (!lu.amActive) {
    //                 if (lu.haveNeighbors) {
    //                     myActiveLanes.push_front(lane);
    //                 } else {
    //                     myActiveLanes.push_back(lane);
    //                 }
    //                 lu.amActive = true;
    //             }
    //         }
    //     }
    // #ifdef PARALLEL_STOPWATCH
    //     myStopWatch[1].stop();
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn execute_movements(&mut self, t: SUMOTime) {
        let wasActive: Vec<Ptr<MSLane>> = self.myActiveLanes.iter().copied().collect();
        self.myWithVehicles2Integrate.clear();
        // shortcut: multi-threaded execution (gNumSimThreads > 1) is not ported, lanes execute serially, upgrade when ThreadPool lands
        let mut i = 0;
        while i < self.myActiveLanes.len() {
            let lane = self.myActiveLanes[i];
            if lane.get_vehicle_number() > 0 {
                lane.m().execute_movements(t);
            }
            if lane.get_vehicle_number() == 0 {
                self.myLanes[lane.get_numerical_id() as usize].amActive = false;
                self.myActiveLanes.remove(i);
            } else {
                i += 1;
            }
        }
        for lane in wasActive {
            lane.m().update_length_sum();
        }
        // arrived vehicles should not influence lane changing
        MSNet::get_instance().m().get_vehicle_control().remove_pending();
        // determinism: lane ids are unique, so the id sort has no ties
        self.myWithVehicles2Integrate.sort_by(|a, b| a.get_id().cmp(b.get_id()));
        let toIntegrate = self.myWithVehicles2Integrate.clone();
        for lane in toIntegrate {
            let wasInactive = lane.get_vehicle_number() == 0;
            lane.m().integrate_new_vehicles();
            if wasInactive && lane.get_vehicle_number() > 0 {
                let lu = &mut self.myLanes[lane.get_numerical_id() as usize];
                if !lu.amActive {
                    if lu.haveNeighbors {
                        self.myActiveLanes.push_front(lane);
                    } else {
                        self.myActiveLanes.push_back(lane);
                    }
                    lu.amActive = true;
                }
            }
        }
    }

    /// C++: `void MSEdgeControl::needsVehicleIntegration(MSLane *const)`
    // void needsVehicleIntegration(MSLane* const l) {
    //         myWithVehicles2Integrate.push_back(l);
    //     }
    #[allow(unused_variables)]
    pub fn needs_vehicle_integration(&mut self, l: Ptr<MSLane>) {
        self.myWithVehicles2Integrate.push(l);
    }

    /// C++: `void MSEdgeControl::changeLanes(const SUMOTime)`
    /// Moves (precomputes) critical vehicles
    // void
    // MSEdgeControl::changeLanes(const SUMOTime t) {
    //     std::vector<MSLane*> toAdd;
    // #ifdef PARALLEL_CHANGE_LANES
    //     std::vector<const MSEdge*> recheckLaneUsage;
    // #endif
    //     MSGlobals::gComputeLC = true;
    //     for (const MSLane* const l : myActiveLanes) {
    //         if (myLanes[l->getNumericalID()].haveNeighbors) {
    //             const MSEdge& edge = l->getEdge();
    //             if (myLastLaneChange[edge.getNumericalID()] != t) {
    //                 myLastLaneChange[edge.getNumericalID()] = t;
    // #ifdef PARALLEL_CHANGE_LANES
    //                 if (MSGlobals::gNumSimThreads > 1) {
    //                     MSLane* lane = edge.getLanes()[0];
    //                     myThreadPool.add(lane->getLaneChangeTask(t), lane->getRNGIndex() % myThreadPool.size());
    //                     recheckLaneUsage.push_back(&edge);
    //                 } else {
    // #endif
    //                     edge.changeLanes(t);
    //                     for (MSLane* const lane : edge.getLanes()) {
    //                         LaneUsage& lu = myLanes[lane->getNumericalID()];
    //                         //if ((*i)->getID() == "disabled") {
    //                         //    std::cout << SIMTIME << " vehicles=" << toString((*i)->getVehiclesSecure()) << "\n";
    //                         //    (*i)->releaseVehicles();
    //                         //}
    //                         if (lane->getVehicleNumber() > 0 && !lu.amActive) {
    //                             toAdd.push_back(lane);
    //                             lu.amActive = true;
    //                         }
    //                         if (MSGlobals::gLateralResolution > 0) {
    //                             lane->sortManeuverReservations();
    //                         }
    //                     }
    // #ifdef PARALLEL_CHANGE_LANES
    //                 }
    // #endif
    //             }
    //         } else {
    //             break;
    //         }
    //     }
    //
    // #ifdef PARALLEL_CHANGE_LANES
    //     if (MSGlobals::gNumSimThreads > 1) {
    //         myThreadPool.waitAll(false);
    //         for (const MSEdge* e : recheckLaneUsage) {
    //             for (MSLane* const l : e->getLanes()) {
    //                 LaneUsage& lu = myLanes[l->getNumericalID()];
    //                 if (l->getVehicleNumber() > 0 && !lu.amActive) {
    //                     toAdd.push_back(l);
    //                     lu.amActive = true;
    //                 }
    //                 if (MSGlobals::gLateralResolution > 0) {
    //                     l->sortManeuverReservations();
    //                 }
    //             }
    //         }
    //     }
    // #endif
    //
    //     MSGlobals::gComputeLC = false;
    //     for (std::vector<MSLane*>::iterator i = toAdd.begin(); i != toAdd.end(); ++i) {
    //         myActiveLanes.push_front(*i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn change_lanes(&mut self, t: SUMOTime) {
        let mut toAdd: Vec<Ptr<MSLane>> = Vec::new();
        // shortcut: multi-threaded lane changing (gNumSimThreads > 1) is not ported, edges change lanes serially, upgrade when ThreadPool lands
        let lateralResolution = MSGlobals::lateral_resolution();
        MSGlobals::set_compute_lc(true);
        let active: Vec<Ptr<MSLane>> = self.myActiveLanes.iter().copied().collect();
        for l in active {
            if self.myLanes[l.get_numerical_id() as usize].haveNeighbors {
                let edge = l.get_edge();
                let eid = edge.get_numerical_id() as usize;
                if self.myLastLaneChange[eid] != t {
                    self.myLastLaneChange[eid] = t;
                    edge.change_lanes(t);
                    for lane in edge.get_lanes().clone() {
                        let lu = &mut self.myLanes[lane.get_numerical_id() as usize];
                        if lane.get_vehicle_number() > 0 && !lu.amActive {
                            toAdd.push(lane);
                            lu.amActive = true;
                        }
                        if lateralResolution > 0. {
                            lane.m().sort_maneuver_reservations();
                        }
                    }
                }
            } else {
                break;
            }
        }
        MSGlobals::set_compute_lc(false);
        for lane in toAdd {
            self.myActiveLanes.push_front(lane);
        }
    }

    /// C++: `void MSEdgeControl::detectCollisions(SUMOTime, const std::string &)`
    /// Detect collisions
    // void
    // MSEdgeControl::detectCollisions(SUMOTime timestep, const std::string& stage) {
    //     // Detections is made by the edge's lanes, therefore hand over.
    //     for (MSLane* lane : myActiveLanes) {
    //         if (lane->needsCollisionCheck()) {
    //             lane->detectCollisions(timestep, stage);
    //         }
    //     }
    //     if (myInactiveCheckCollisions.size() > 0) {
    //         for (MSLane* lane : myInactiveCheckCollisions.getContainer()) {
    //             lane->detectCollisions(timestep, stage);
    //         }
    //         myInactiveCheckCollisions.clear();
    //         myInactiveCheckCollisions.unlock();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn detect_collisions(&mut self, timestep: SUMOTime, stage: &str) {
        // Detections is made by the edge's lanes, therefore hand over.
        let active: Vec<Ptr<MSLane>> = self.myActiveLanes.iter().copied().collect();
        for lane in active {
            if lane.needs_collision_check() {
                lane.m().detect_collisions(timestep, stage);
            }
        }
        if !self.myInactiveCheckCollisions.is_empty() {
            let inactive: Vec<Ptr<MSLane>> = self.myInactiveCheckCollisions.values().copied().collect();
            for lane in inactive {
                lane.m().detect_collisions(timestep, stage);
            }
            self.myInactiveCheckCollisions.clear();
        }
    }

    /// C++: `const MSEdgeVector & MSEdgeControl::getEdges() const` [const]
    /// Returns loaded edges
    // const MSEdgeVector& getEdges() const {
    //         return myEdges;
    //     }
    #[allow(unused_variables)]
    pub fn get_edges(&self) -> &MSEdgeVector {
        &self.myEdges
    }

    /// C++: `void MSEdgeControl::gotActive(MSLane *)`
    /// Informs the control that the given lane got active
    // void
    // MSEdgeControl::gotActive(MSLane* l) {
    //     myChangedStateLanes.insert(l);
    // }
    #[allow(unused_variables)]
    pub fn got_active(&mut self, l: Ptr<MSLane>) {
        self.myChangedStateLanes.insert(l.get_numerical_id(), l);
    }

    /// C++: `void MSEdgeControl::checkCollisionForInactive(MSLane *)`
    /// trigger collision checking for inactive lane
    // void
    // MSEdgeControl::checkCollisionForInactive(MSLane* l) {
    //     myInactiveCheckCollisions.insert(l);
    // }
    #[allow(unused_variables)]
    pub fn check_collision_for_inactive(&mut self, l: Ptr<MSLane>) {
        self.myInactiveCheckCollisions.insert(l.get_numerical_id(), l);
    }

    /// C++: `void MSEdgeControl::setAdditionalRestrictions()`
    /// apply additional restrictions
    // void
    // MSEdgeControl::setAdditionalRestrictions() {
    //     for (MSEdge* e : myEdges) {
    //         e->inferEdgeType();
    //         const std::vector<MSLane*>& lanes = e->getLanes();
    //         for (std::vector<MSLane*>::const_iterator j = lanes.begin(); j != lanes.end(); ++j) {
    //             (*j)->initRestrictions();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_additional_restrictions(&mut self) {
        for e in self.myEdges.iter() {
            e.m().infer_edge_type();
            let lanes = e.get_lanes();
            for j in lanes.iter() {
                j.m().init_restrictions();
            }
        }
    }

    /// C++: `void MSEdgeControl::buildMesoSegments()`
    /// build meso segments after meso edge types are loaded
    // void
    // MSEdgeControl::buildMesoSegments() {
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     for (MSEdge* edge : myEdges) {
    //         if (!edge->getLanes().empty()) {
    //             MSGlobals::gMesoNet->buildSegmentsFor(*edge, oc);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_meso_segments(&mut self) {
        with_options(|oc| {
            for edge in self.myEdges.iter() {
                if !edge.get_lanes().is_empty() {
                    crate::microsim::ms_globals::meso_net().m().build_segments_for(edge, oc);
                }
            }
        });
    }

    /// C++: `void MSEdgeControl::saveState(OutputDevice &)`
    /// Saves the current state into the given stream
    // void
    // MSEdgeControl::saveState(OutputDevice& out) {
    //     out.openTag(SUMO_TAG_EDGECONTROL);
    //     out.writeAttr(SUMO_ATTR_LANES, myActiveLanes);
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        out.open_tag_id(SUMO_TAG_EDGECONTROL);
        let ids: Vec<String> = self.myActiveLanes.iter().map(|l| l.get_id().to_string()).collect();
        out.write_attr(SUMO_ATTR_LANES, &ids);
        out.close_tag("");
    }

    /// C++: `void MSEdgeControl::setActiveLanes(std::list<MSLane *>)`
    /// Reconstruct the current state
    // void
    // MSEdgeControl::setActiveLanes(std::list<MSLane*> lanes) {
    //     myActiveLanes = lanes;
    //     for (MSLane* lane : lanes) {
    //         myLanes[lane->getNumericalID()].amActive = true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_active_lanes(&mut self, lanes: VecDeque<Ptr<MSLane>>) {
        self.myActiveLanes = lanes;
        for lane in self.myActiveLanes.iter() {
            self.myLanes[lane.get_numerical_id() as usize].amActive = true;
        }
    }

    /// C++: `void MSEdgeControl::MSEdgeControl(const MSEdgeControl &)` [private]
    /// Copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSEdgeControl) -> Self {
        panic!("MSEdgeControl copy constructor is invalidated")
    }

    /// C++: `MSEdgeControl & MSEdgeControl::operator=(const MSEdgeControl &)` [private]
    /// Assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSEdgeControl) -> &mut MSEdgeControl {
        panic!("MSEdgeControl assignment operator is invalidated")
    }

}

