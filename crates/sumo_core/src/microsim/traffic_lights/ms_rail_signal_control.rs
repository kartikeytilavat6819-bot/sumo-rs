//! Port of microsim/traffic_lights/MSRailSignalControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::{Cell, RefCell};

/// storage for loaded deadlocks
/// C++ class `MSRailSignalControl::Deadlock` (microsim/traffic_lights/MSRailSignalControl.h)
#[derive(Clone)]
pub struct MSRailSignalControl_Deadlock {
    pub signals: Vec<Ptr<MSRailSignal>>,
    pub edges: Vec<ConstMSEdgeVector>,
    pub forwardEnds: ConstMSEdgeVector,
}

impl MSRailSignalControl_Deadlock {
}

/// A signal for rails
/// C++ class `MSRailSignalControl` (microsim/traffic_lights/MSRailSignalControl.h)
#[repr(C)]
pub struct MSRailSignalControl {
    /// C++ base class `MSNet::VehicleStateListener`
    pub base: MSNet_VehicleStateListener,
    /// all rail edges that are part of a known route
    pub myUsedEdges: BTreeSet<Ptr<MSEdge>>,
    pub myWaitRelations: BTreeMap<Ptr<SUMOVehicle>, MSRailSignalControl_WaitRelation>,
    pub myWrittenDeadlocks: BTreeSet<BTreeSet<Ptr<SUMOVehicle>>>,
    pub myLoadedDeadlocks: Vec<MSRailSignalControl_Deadlock>,
    pub mySignalDeadlocks: BTreeMap<Ptr<MSRailSignal>, Vec<(i32, i32)>>, // C++: std::map<const MSRailSignal *, std::vector<std::pair<int, int>>>
    pub myDriveWaySucc: BTreeMap<Ptr<MSDriveWay>, BTreeSet<Ptr<MSDriveWay>>>, // C++: std::map<const MSDriveWay *, std::set<const MSDriveWay *, ComparatorNumericalIdLess>>
    pub myDriveWayPred: BTreeMap<Ptr<MSDriveWay>, BTreeSet<Ptr<MSDriveWay>>>, // C++: std::map<const MSDriveWay *, std::set<const MSDriveWay *>>
    /// list of all rail signals
    pub mySignals: Vec<Ptr<MSRailSignal>>,
    /// list of signals that switched green along with driveway index
    pub mySwitchedGreenFlanks: Vec<(Ptr<MSLink>, i32)>,
    pub myDriveWayCompatibility: BTreeMap<(i32, i32), bool>, // C++: std::map<std::pair<int, int>, bool>
    pub myActiveSignals: BTreeSet<Ptr<MSRailSignal>>, // C++: std::set<MSRailSignal *, ComparatorNumericalIdLess>
    pub myMaxTrainLength: f64,
}

impl std::ops::Deref for MSRailSignalControl {
    type Target = MSNet_VehicleStateListener;
    fn deref(&self) -> &MSNet_VehicleStateListener {
        &self.base
    }
}
impl std::ops::DerefMut for MSRailSignalControl {
    fn deref_mut(&mut self) -> &mut MSNet_VehicleStateListener {
        &mut self.base
    }
}

// C++ static members of MSRailSignalControl (port as module-level statics, see AGENTS.md):
//   static SVCPermissions MSRailSignalControl::mySignalizedClasses;
//   static SVCPermissions MSRailSignalControl::myMBClasses;
//   static MSRailSignalControl * MSRailSignalControl::myInstance;

thread_local! {
    // static MSRailSignalControl * MSRailSignalControl::myInstance
    static MY_INSTANCE: RefCell<Option<Ptr<MSRailSignalControl>>> = const { RefCell::new(None) };
    // static SVCPermissions MSRailSignalControl::mySignalizedClasses
    static MY_SIGNALIZED_CLASSES: Cell<SVCPermissions> = const { Cell::new(SVC_UNSPECIFIED) };
    // static SVCPermissions MSRailSignalControl::myMBClasses
    static MY_MB_CLASSES: Cell<SVCPermissions> = const { Cell::new(SVC_UNSPECIFIED) };
}

/// the current singleton, without creating one
fn my_instance_opt() -> Option<Ptr<MSRailSignalControl>> {
    MY_INSTANCE.with(|c| *c.borrow())
}

/// static SVCPermissions MSRailSignalControl::mySignalizedClasses
fn my_signalized_classes() -> SVCPermissions {
    MY_SIGNALIZED_CLASSES.with(|c| c.get())
}

/// static SVCPermissions MSRailSignalControl::myMBClasses
fn my_mb_classes() -> SVCPermissions {
    MY_MB_CLASSES.with(|c| c.get())
}

/// the current singleton, creating it (and registering the listener) on first use
fn my_instance_ptr() -> Ptr<MSRailSignalControl> {
    MY_INSTANCE.with(|c| {
        let mut slot = c.borrow_mut();
        if slot.is_none() {
            let p = Ptr::from_raw(Box::into_raw(Box::new(MSRailSignalControl::new())));
            *slot = Some(p);
            // the listener base is the first field, so the addresses coincide
            MSNet::get_instance().m().add_vehicle_state_listener(p.cast::<MSNet_VehicleStateListener>());
        }
        slot.unwrap()
    })
}

impl MSRailSignalControl {
    /// C++: `void MSRailSignalControl::~MSRailSignalControl() noexcept` [virtual]
    // MSRailSignalControl::~MSRailSignalControl() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `MSRailSignalControl & MSRailSignalControl::getInstance()` [static]
    // MSRailSignalControl&
    // MSRailSignalControl::getInstance() {
    //     if (myInstance == nullptr) {
    //         myInstance = new MSRailSignalControl();
    //         MSNet::getInstance()->addVehicleStateListener(myInstance);
    //     }
    //     return *myInstance;
    // }
    #[allow(unused_variables)]
    pub fn get_instance() -> &'static mut MSRailSignalControl {
        unsafe { &mut *my_instance_ptr().as_raw() }
    }

    /// C++: `bool MSRailSignalControl::hasInstance()` [static]
    // static bool hasInstance() {
    //         return myInstance != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn has_instance() -> bool {
        my_instance_opt().is_some()
    }

    /// C++: `void MSRailSignalControl::cleanup()` [static]
    // void
    // MSRailSignalControl::cleanup() {
    //     delete myInstance;
    //     myInstance = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn cleanup() {
        if let Some(p) = MY_INSTANCE.with(|c| c.borrow_mut().take()) {
            unsafe { drop(Box::from_raw(p.as_raw())) };
        }
    }

    /// C++: `void MSRailSignalControl::clearState()` [static]
    /// Perform resets events when quick-loading state
    // void
    // MSRailSignalControl::clearState() {
    //     if (myInstance != nullptr) {
    //         myInstance->myDriveWayCompatibility.clear();
    //         myInstance->myDriveWaySucc.clear();
    //         myInstance->myDriveWayPred.clear();
    //         myInstance->myWrittenDeadlocks.clear();
    //         myInstance->mySignalDeadlocks.clear();
    //         myInstance->myLoadedDeadlocks.clear();
    //         //myInstance->myActiveSignals.clear();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn clear_state() {
        if let Some(p) = my_instance_opt() {
            let instance = unsafe { &mut *p.as_raw() };
            instance.myDriveWayCompatibility.clear();
            instance.myDriveWaySucc.clear();
            instance.myDriveWayPred.clear();
            instance.myWrittenDeadlocks.clear();
            instance.mySignalDeadlocks.clear();
            instance.myLoadedDeadlocks.clear();
            //myActiveSignals.clear();
        }
    }

    /// C++: `void MSRailSignalControl::resetWaitRelations()`
    /// reset all waiting-for relationships at the start of the simulation step
    // void resetWaitRelations() {
    //         myWaitRelations.clear();
    //         myWrittenDeadlocks.clear();
    //     }
    #[allow(unused_variables)]
    pub fn reset_wait_relations(&mut self) {
        self.myWaitRelations.clear();
        self.myWrittenDeadlocks.clear();
    }

    /// C++: `void MSRailSignalControl::addWaitRelation(const SUMOVehicle *, const MSRailSignal *, const SUMOVehicle *, MSRailSignalConstraint *)`
    // void
    // MSRailSignalControl::addWaitRelation(const SUMOVehicle* waits, const MSRailSignal* rs, const SUMOVehicle* reason, MSRailSignalConstraint* constraint) {
    //     //std::cout << time2string(SIMSTEP) << " addWaitRelation waits=" << waits->getID() << " foe=" << reason->getID() << "\n";
    //     myWaitRelations[waits] = WaitRelation(rs, reason, constraint);
    // }
    #[allow(unused_variables)]
    pub fn add_wait_relation(&mut self, waits: Ptr<SUMOVehicle>, rs: Ptr<MSRailSignal>, reason: Ptr<SUMOVehicle>, constraint: Ptr<MSRailSignalConstraint>) {
        //std::cout << time2string(SIMSTEP) << " addWaitRelation waits=" << waits->getID() << " foe=" << reason->getID() << "\n";
        self.myWaitRelations
            .insert(waits, MSRailSignalControl_WaitRelation::new(rs, reason, constraint));
    }

    /// C++: `void MSRailSignalControl::addDrivewayFollower(const MSDriveWay *, const MSDriveWay *)`
    // void
    // MSRailSignalControl::addDrivewayFollower(const MSDriveWay* dw, const MSDriveWay* dw2) {
    //     //std::cout << " addDrivewayFollower " << dw->getID() << " " << dw2->getID() << "\n";
    //     myDriveWaySucc[dw].insert(dw2);
    //     myDriveWayPred[dw2].insert(dw);
    // }
    #[allow(unused_variables)]
    pub fn add_driveway_follower(&mut self, dw: Ptr<MSDriveWay>, dw2: Ptr<MSDriveWay>) {
        //std::cout << " addDrivewayFollower " << dw->getID() << " " << dw2->getID() << "\n";
        // determinism: C++ orders these sets by MSDriveWay numerical id, the Rust set orders by address
        self.myDriveWaySucc.entry(dw).or_default().insert(dw2);
        self.myDriveWayPred.entry(dw2).or_default().insert(dw);
    }

    /// C++: `void MSRailSignalControl::addDWDeadlockChecks(const MSRailSignal *, MSDriveWay *)`
    /// check whether the given signal and driveway are part of a deadlock circle
    // void
    // MSRailSignalControl::addDWDeadlockChecks(const MSRailSignal* rs, MSDriveWay* dw) {
    //     auto itDL = mySignalDeadlocks.find(rs);
    //     if (itDL == mySignalDeadlocks.end()) {
    //         return;
    //     }
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //     std::cout << "    needsDeadlockChecks? rs=" << rs->getID() << " dw=" << dw->getID() << "\n";
    // #endif
    //     // the driveway dw ends at signal rs and could be part of a deadlock.
    //     // check whether we can form a sequence of driveways that link it with all others (railsignals)
    //     // findDeadlockFoes calls itself recursively until all circles have been found
    //     for (const auto& pair : itDL->second) {
    //         const Deadlock& dl = myLoadedDeadlocks[pair.first];
    //         std::vector<const MSDriveWay*> deadlockFoes;
    //         if (dl.edges.empty()) {
    //             findDeadlockFoes(dw, dl, pair.second, deadlockFoes);
    //         } else {
    //             const MSEdge* forwardEnd = dl.forwardEnds[pair.second];
    //             if (forwardEnd == &dw->getForward().back()->getEdge()) {
    //                 findDeadlockFoesFromRoutes(dw, dl, pair.second, deadlockFoes);
    //             } else {
    //                 for (MSDriveWay* cand : MSDriveWay::getEndingDriveWays(forwardEnd)) {
    //                     findDeadlockFoesFromRoutes(cand, dl, pair.second, deadlockFoes);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_dw_deadlock_checks(&mut self, rs: Ptr<MSRailSignal>, dw: Ptr<MSDriveWay>) {
        let it_dl = match self.mySignalDeadlocks.get(&rs) {
            Some(v) => v.clone(),
            None => return,
        };
        // the driveway dw ends at signal rs and could be part of a deadlock.
        // check whether we can form a sequence of driveways that link it with all others (railsignals)
        // findDeadlockFoes calls itself recursively until all circles have been found
        for pair in it_dl {
            // cloned so that the recursive calls can borrow self mutably
            let dl = self.myLoadedDeadlocks[pair.0 as usize].clone();
            let deadlock_foes: Vec<Ptr<MSDriveWay>> = Vec::new();
            if dl.edges.is_empty() {
                self.find_deadlock_foes(dw, &dl, pair.1, deadlock_foes);
            } else {
                let forward_end = dl.forwardEnds[pair.1 as usize];
                let dw_end_edge = dw.get_forward().last().map(|l| l.myEdge).unwrap_or_default();
                if forward_end == dw_end_edge {
                    self.find_deadlock_foes_from_routes(dw, &dl, pair.1, deadlock_foes);
                } else {
                    for cand in MSDriveWay::get_ending_drive_ways(forward_end).clone() {
                        self.find_deadlock_foes_from_routes(cand, &dl, pair.1, deadlock_foes.clone());
                    }
                }
            }
        }
    }

    /// C++: `bool MSRailSignalControl::haveDeadlock(const SUMOVehicle *) const` [const]
    /// whether there is a circle in the waiting-for relationships that contains the given vehicle
    // bool
    // MSRailSignalControl::haveDeadlock(const SUMOVehicle* veh) const {
    //     std::set<const SUMOVehicle*> seen;
    //     std::vector<WaitRelation> list;
    //     const SUMOVehicle* cur = veh;
    //     std::vector<MSRailSignalConstraint*> constraints;
    //     std::vector<const SUMOVehicle*> constraintBlocked;
    //     std::vector<const MSRailSignal*> constraintSignals;
    //     //std::cout << time2string(SIMSTEP) << " haveDeadlock veh=" << veh->getID() << "\n";
    //     while (seen.count(cur) == 0) {
    //         auto it = myWaitRelations.find(cur);
    //         if (it != myWaitRelations.end()) {
    //             if (it->second.constraint != nullptr) {
    //                 constraints.push_back(it->second.constraint);
    //                 constraintBlocked.push_back(cur);
    //                 constraintSignals.push_back(it->second.railSignal);
    //             }
    //             seen.insert(cur);
    //             list.push_back(it->second);
    //             cur = it->second.foe;
    //         } else {
    //             return false;
    //         }
    //     }
    //     if (cur == veh) {
    //         const bool newDeadlock = myWrittenDeadlocks.count(seen) == 0;
    //         myWrittenDeadlocks.insert(seen);
    //         const OptionsCont& oc = OptionsCont::getOptions();
    //         MSRailSignalConstraint* resolved = nullptr;
    //         const SUMOVehicle* resolvedUnblocked = nullptr;
    //         const MSRailSignal* resolvedSignal = nullptr;
    //         if (!constraints.empty() && oc.getBool("time-to-teleport.remove-constraint")) {
    //             resolved = constraints.front();
    //             if (newDeadlock) {
    //                 std::vector<std::string> vehicles;
    //                 for (auto item : list) {
    //                     vehicles.push_back(item.foe->getID());
    //                 }
    //                 WRITE_WARNINGF("Deactivating constraint to resolve deadlock between vehicles % at time %.", toString(vehicles), time2string(SIMSTEP));
    //                 resolved->setActive(false);
    //                 resolvedUnblocked = constraintBlocked.front();
    //                 resolvedSignal = constraintSignals.front();
    //             }
    //         }
    //
    //         if (oc.isSet("deadlock-output")) {
    //             if (newDeadlock) {
    //                 myWrittenDeadlocks.insert(seen);
    //                 std::vector<std::string> signals;
    //                 std::vector<std::string> vehicles;
    //                 std::vector<std::string> tripIDs;
    //                 for (auto item : list) {
    //                     signals.push_back(item.railSignal == nullptr ? "INSERTION" : item.railSignal->getID());
    //                     vehicles.push_back(item.foe->getID());
    //                     tripIDs.push_back(item.foe->getParameter().getParameter("tripId", item.foe->getID()));
    //                 }
    //                 // rotate signals so their are aligned with the vehicle waiting
    //                 // at that signal instead of the foe that is blocking the signal
    //                 std::rotate(signals.begin(), signals.begin() + 1, signals.end());
    //                 OutputDevice& od = OutputDevice::getDeviceByOption("deadlock-output");
    //                 if (constraints.empty()) {
    //                     od.openTag(SUMO_TAG_DEADLOCK);
    //                 } else {
    //                     od.openTag("constraintDeadlock");
    //                 }
    //                 od.writeAttr(SUMO_ATTR_TIME, time2string(SIMSTEP));
    //                 od.writeAttr(SUMO_ATTR_SIGNALS, signals);
    //                 od.writeAttr("vehicles", vehicles);
    //                 if (!constraints.empty()) {
    //                     od.writeAttr("tripIds", tripIDs);
    //                 }
    //                 if (resolved != nullptr) {
    //                     od.openTag("resolvedConstraint");
    //                     od.writeAttr(SUMO_ATTR_ID, resolvedSignal->getID());
    //                     resolved->write(od, resolvedUnblocked->getParameter().getParameter("tripId", resolvedUnblocked->getID()));
    //                     od.closeTag();
    //                 }
    //                 for (auto item : list) {
    //                     int start = item.foe->getRoutePosition();
    //                     if (!MSGlobals::gUseMesoSim) {
    //                         const std::vector<MSLane*>& further = dynamic_cast<const MSVehicle*>(item.foe)->getFurtherLanes();
    //                         start = MAX2(0, start - (int)(MSGlobals::gUsingInternalLanes && MSNet::getInstance()->hasInternalLinks() ? further.size() / 2 : further.size()));
    //                     }
    //                     const MSRoute& route = item.foe->getRoute();
    //                     ConstMSEdgeVector future(route.begin() + start, route.end());
    //                     od.openTag(SUMO_TAG_ROUTEPART);
    //                     od.writeAttr(SUMO_ATTR_EDGES, toString(future));
    //                     od.closeTag();
    //                 }
    //                 od.closeTag();
    //             }
    //         }
    //         return resolved == nullptr;
    //     } else {
    //         // it's a deadlock but does not involve veh
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn have_deadlock(&mut self, veh: Ptr<SUMOVehicle>) -> bool {
        let mut seen: BTreeSet<Ptr<SUMOVehicle>> = BTreeSet::new();
        let mut list: Vec<MSRailSignalControl_WaitRelation> = Vec::new();
        let mut cur = veh;
        let mut constraints: Vec<Ptr<MSRailSignalConstraint>> = Vec::new();
        let mut constraint_blocked: Vec<Ptr<SUMOVehicle>> = Vec::new();
        let mut constraint_signals: Vec<Ptr<MSRailSignal>> = Vec::new();
        //std::cout << time2string(SIMSTEP) << " haveDeadlock veh=" << veh->getID() << "\n";
        while !seen.contains(&cur) {
            match self.myWaitRelations.get(&cur) {
                Some(relation) => {
                    if !relation.constraint.is_null() {
                        constraints.push(relation.constraint);
                        constraint_blocked.push(cur);
                        constraint_signals.push(relation.railSignal);
                    }
                    seen.insert(cur);
                    list.push(relation.clone());
                    cur = relation.foe;
                }
                None => return false,
            }
        }
        if cur == veh {
            let new_deadlock = !self.myWrittenDeadlocks.contains(&seen);
            self.myWrittenDeadlocks.insert(seen.clone());
            let mut resolved: Ptr<MSRailSignalConstraint> = Ptr::null();
            let mut resolved_unblocked: Ptr<SUMOVehicle> = Ptr::null();
            let mut resolved_signal: Ptr<MSRailSignal> = Ptr::null();
            if !constraints.is_empty() && utils::options::options_cont::get_bool("time-to-teleport.remove-constraint") {
                resolved = constraints[0];
                if new_deadlock {
                    let vehicles: Vec<String> = list.iter().map(|item| item.foe.get_id().to_string()).collect();
                    utils::write_warningf!("Deactivating constraint to resolve deadlock between vehicles % at time %.", vehicles.join(" "), utils::common::time::time2string(MSNet::get_instance().get_current_time_step()));
                    resolved.m().set_active(false);
                    resolved_unblocked = constraint_blocked[0];
                    resolved_signal = constraint_signals[0];
                }
            }

            if utils::options::options_cont::is_set("deadlock-output") {
                if new_deadlock {
                    self.myWrittenDeadlocks.insert(seen.clone());
                    let mut signals: Vec<String> = Vec::new();
                    let mut vehicles: Vec<String> = Vec::new();
                    let mut trip_ids: Vec<String> = Vec::new();
                    for item in &list {
                        signals.push(if item.railSignal.is_null() {
                            "INSERTION".to_string()
                        } else {
                            item.railSignal.get_id().to_string()
                        });
                        vehicles.push(item.foe.get_id().to_string());
                        trip_ids.push(
                            item.foe
                                .cast::<MSBaseVehicle>()
                                .get_parameter()
                                .get_parameter("tripId", item.foe.get_id()),
                        );
                    }
                    // rotate signals so their are aligned with the vehicle waiting
                    // at that signal instead of the foe that is blocking the signal
                    signals.rotate_left(1);
                    let od = OutputDevice::get_device_by_option("deadlock-output").unwrap_or_else(|e| panic!("{e}"));
                    let mut dev = od.borrow_mut();
                    if constraints.is_empty() {
                        dev.open_tag_id(SUMO_TAG_DEADLOCK);
                    } else {
                        dev.open_tag("constraintDeadlock");
                    }
                    dev.write_attr(SUMO_ATTR_TIME, &utils::common::time::time2string(MSNet::get_instance().get_current_time_step()));
                    dev.write_attr(SUMO_ATTR_SIGNALS, &signals);
                    dev.write_attr("vehicles", &vehicles);
                    if !constraints.is_empty() {
                        dev.write_attr("tripIds", &trip_ids);
                    }
                    if !resolved.is_null() {
                        dev.open_tag("resolvedConstraint");
                        dev.write_attr(SUMO_ATTR_ID, &resolved_signal.get_id());
                        let unblocked_id = resolved_unblocked.get_id().to_string();
                        let trip_id = resolved_unblocked
                            .cast::<MSBaseVehicle>()
                            .get_parameter()
                            .get_parameter("tripId", &unblocked_id);
                        resolved.get().unwrap().write(&mut dev, &trip_id);
                        dev.close_tag("");
                    }
                    for item in &list {
                        let mut start = item.foe.cast::<MSBaseVehicle>().get_route_position();
                        if !MSGlobals::use_meso_sim() {
                            let further = item.foe.cast::<MSVehicle>().get_further_lanes();
                            let skipped = if MSGlobals::using_internal_lanes() && MSNet::get_instance().has_internal_links() {
                                further.len() / 2
                            } else {
                                further.len()
                            };
                            start = std::cmp::max(0, start - skipped as i32);
                        }
                        let route = item.foe.cast::<MSBaseVehicle>().get_route();
                        let future: Vec<String> = route
                            .get_edges()
                            .iter()
                            .skip(start as usize)
                            .map(|e| e.get_id().to_string())
                            .collect();
                        dev.open_tag_id(SUMO_TAG_ROUTEPART);
                        dev.write_attr(SUMO_ATTR_EDGES, &future.join(" "));
                        dev.close_tag("");
                    }
                    dev.close_tag("");
                }
            }
            resolved.is_null()
        } else {
            // it's a deadlock but does not involve veh
            false
        }
    }

    /// C++: `void MSRailSignalControl::loadDeadlockCheck(std::vector<const MSRailSignal *>)`
    // void
    // MSRailSignalControl::loadDeadlockCheck(std::vector<const MSRailSignal*> signals) {
    //     if (MSDriveWay::haveDriveWays()) {
    //         WRITE_WARNING("Deadlocks should be loaded before any vehicles");
    //     }
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //     std::cout << "    loadedDeadlockCheck signals=" << toString(signals) << "\n";
    // #endif
    //     const int deadlockIndex = (int)myLoadedDeadlocks.size();
    //     myLoadedDeadlocks.push_back(Deadlock());
    //     myLoadedDeadlocks.back().signals = signals;
    //     const int n = (int)signals.size();
    //     for (int i = 0; i < n; i++) {
    //         mySignalDeadlocks[signals[i]].push_back(std::make_pair(deadlockIndex, i));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_deadlock_check(&mut self, signals: Vec<Ptr<MSRailSignal>>) {
        if MSDriveWay::have_drive_ways() {
            utils::common::msg_handler::write_warning("Deadlocks should be loaded before any vehicles");
        }
        let deadlock_index = self.myLoadedDeadlocks.len() as i32;
        self.myLoadedDeadlocks.push(MSRailSignalControl_Deadlock {
            signals: signals.clone(),
            edges: Vec::new(),
            forwardEnds: Vec::new(),
        });
        let n = signals.len() as i32;
        for i in 0..n {
            self.mySignalDeadlocks
                .entry(signals[i as usize])
                .or_default()
                .push((deadlock_index, i));
        }
    }

    /// C++: `void MSRailSignalControl::loadDeadlockRoute(const ConstMSEdgeVector &)`
    // void
    // MSRailSignalControl::loadDeadlockRoute(const ConstMSEdgeVector& edges) {
    //     Deadlock& dl = myLoadedDeadlocks.back();
    //     const int signalIndex = (int)dl.edges.size();
    //     const MSRailSignal* rs = dl.signals[signalIndex];
    //     const MSJunction* j = rs->getJunction();
    //     auto it = edges.begin();
    //     while (it != edges.end() && (*it)->getToJunction() != j) {
    //         it++;
    //     }
    //     if (it == edges.end()) {
    //         throw ProcessError("Rail signal junction '" + j->getID() + "' not passed by loaded deadlock edges '" + toString(edges) + "'.");
    //     }
    //     const MSEdge* firstSignalEdge = *it;
    //     // maybe there is an earlier signal, in this case we must look for driveways that start there
    //     for (auto it2 = edges.begin(); it2 != it; it2++) {
    //         if ((*it2)->getToJunction()->getType() == SumoXMLNodeType::RAIL_SIGNAL) {
    //             const MSLink* link = (*it2)->getLanes().front()->getLinkTo((*(it2 + 1))->getLanes().front());
    //             if (link != nullptr && link->isTLSControlled()) {
    //                 firstSignalEdge = *it2;
    //                 break;
    //             }
    //         }
    //     }
    //     dl.forwardEnds.push_back(firstSignalEdge);
    //     dl.edges.push_back(edges);
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //     std::cout << "      loadedDeadlockRoute signal=" << rs->getID() << " signalEdge=" << firstSignalEdge->getID() << " edges=" << toString(edges) << "\n";
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn load_deadlock_route(&mut self, edges: &ConstMSEdgeVector) {
        let (signal_index, rs) = {
            let dl = self.myLoadedDeadlocks.last().unwrap();
            let signal_index = dl.edges.len() as i32;
            (signal_index, dl.signals[signal_index as usize])
        };
        let j = rs.get_junction();
        let mut found: Option<usize> = None;
        for (i, e) in edges.iter().enumerate() {
            if e.get_to_junction() == j {
                found = Some(i);
                break;
            }
        }
        let index = match found {
            Some(i) => i,
            None => panic!(
                "{}",
                ProcessError::new(format!(
                    "Rail signal junction '{}' not passed by loaded deadlock edges '{}'.",
                    j.get_id(),
                    edges.iter().map(|e| e.get_id().to_string()).collect::<Vec<_>>().join(" ")
                ))
            ),
        };
        let mut first_signal_edge = edges[index];
        // maybe there is an earlier signal, in this case we must look for driveways that start there
        for i in 0..index {
            if edges[i].get_to_junction().get_type() == SumoXMLNodeType::RAIL_SIGNAL {
                let link = edges[i]
                    .get_lanes()
                    .first()
                    .unwrap()
                    .get_link_to(*edges[i + 1].get_lanes().first().unwrap());
                if !link.is_null() && link.is_tls_controlled() {
                    first_signal_edge = edges[i];
                    break;
                }
            }
        }
        let dl = self.myLoadedDeadlocks.last_mut().unwrap();
        dl.forwardEnds.push(first_signal_edge);
        dl.edges.push(edges.clone());
    }

    /// C++: `void MSRailSignalControl::vehicleStateChanged(const SUMOVehicle *const, MSNet::VehicleState, const std::string &)` [virtual]
    /// Called if a vehicle changes its state
    // void
    // MSRailSignalControl::vehicleStateChanged(const SUMOVehicle* const vehicle, MSNet::VehicleState to, const std::string& /*info*/) {
    //     if (vehicle->isRail()) {
    //         std::string dummyMsg;
    //         if ((to == MSNet::VehicleState::BUILT && (!vehicle->getParameter().wasSet(VEHPARS_FORCE_REROUTE) || vehicle->hasValidRoute(dummyMsg)))
    //                 || to == MSNet::VehicleState::NEWROUTE) {
    //             // @note we could delay initialization until the departure time
    //             if (vehicle->getEdge()->getFunction() != SumoXMLEdgeFunc::CONNECTOR && isRailwayOrShared(vehicle->getEdge()->getPermissions())) {
    //                 MSRailSignal::initDriveWays(vehicle, to == MSNet::VehicleState::NEWROUTE);
    //             }
    //         }
    //         if (to == MSNet::VehicleState::BUILT) {
    //             myMaxTrainLength = MAX2(myMaxTrainLength, vehicle->getLength());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn vehicle_state_changed(&mut self, vehicle: Ptr<SUMOVehicle>, to: MSNet_VehicleState, info: &str) {
        let veh = vehicle.cast::<MSBaseVehicle>();
        if veh.is_rail() {
            let mut dummy_msg = String::new();
            if (to == MSNet_VehicleState::BUILT
                && (!veh.get_parameter().was_set(VEHPARS_FORCE_REROUTE) || veh.has_valid_route(&mut dummy_msg, veh.get_route_ptr())))
                || to == MSNet_VehicleState::NEWROUTE
            {
                // @note we could delay initialization until the departure time
                let edge = veh.get_edge();
                if edge.get_function() != SumoXMLEdgeFunc::CONNECTOR
                    && utils::common::sumo_vehicle_class::is_railway_or_shared(edge.get_permissions())
                {
                    MSRailSignal::init_drive_ways(vehicle, to == MSNet_VehicleState::NEWROUTE);
                }
            }
            if to == MSNet_VehicleState::BUILT {
                self.myMaxTrainLength = std::cmp::max(self.myMaxTrainLength, veh.get_length());
            }
        }
    }

    /// C++: `void MSRailSignalControl::addSignal(MSRailSignal *)`
    // void
    // MSRailSignalControl::addSignal(MSRailSignal* signal) {
    //     mySignals.push_back(signal);
    //     for (const auto& links : signal->getLinks()) {
    //         for (const MSLink* link : links) {
    //             mySignalizedClasses |= (link->getPermissions() & myMBClasses);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_signal(&mut self, signal: Ptr<MSRailSignal>) {
        self.mySignals.push(signal);
        for links in signal.get_links() {
            for link in links {
                MY_SIGNALIZED_CLASSES.with(|c| c.set(c.get() | (link.get_permissions() & my_mb_classes())));
            }
        }
    }

    /// C++: `const std::vector<MSRailSignal *> & MSRailSignalControl::getSignals() const` [const]
    // const std::vector<MSRailSignal*>& getSignals() const {
    //         return mySignals;
    //     }
    #[allow(unused_variables)]
    pub fn get_signals(&self) -> &Vec<Ptr<MSRailSignal>> {
        &self.mySignals
    }

    /// C++: `void MSRailSignalControl::notifyApproach(const MSLink *)`
    /// switch rail signal to active
    // void
    // MSRailSignalControl::notifyApproach(const MSLink* link) {
    //     const MSRailSignal* rs = dynamic_cast<const MSRailSignal*>(link->getTLLogic());
    //     assert(rs != nullptr);
    //     myActiveSignals.insert(const_cast<MSRailSignal*>(rs));
    // }
    #[allow(unused_variables)]
    pub fn notify_approach(&mut self, link: Ptr<MSLink>) {
        // the link's logic is a rail signal (the link is controlled by one)
        let rs: Ptr<MSRailSignal> = link.get_tl_logic().cast::<MSRailSignal>();
        debug_assert!(!rs.is_null());
        self.myActiveSignals.insert(rs);
    }

    /// C++: `void MSRailSignalControl::updateSignals(SUMOTime)`
    /// update active rail signals
    // void
    // MSRailSignalControl::updateSignals(SUMOTime t) {
    //     UNUSED_PARAMETER(t);
    //     // there are 4 states for a signal
    //     // 1. approached and green
    //     // 2. approached and red
    //     // 3. not approached and trains could pass
    //     // 4. not approached and trains coult not pass
    //     //
    //     // for understanding conflicts better in sumo-gui, we want to show (3) as green. This
    //     // means we have to keep updating signals in state (4) until they change to (3)
    //
    //     //std::cout << SIMTIME << " activeSignals=" << myActiveSignals.size() << "\n";
    //     for (auto it = myActiveSignals.begin(); it != myActiveSignals.end();) {
    //         MSRailSignal* rs = *it;
    //         const bool keepActive = rs->updateCurrentPhase();
    //         // std::cout << SIMTIME << " update " << rs->getID() << " keepActive=" << keepActive << "\n";
    //         if (rs->isActive()) {
    //             rs->setTrafficLightSignals(t);
    //         }
    //         if (!keepActive) {
    //             it = myActiveSignals.erase(it);
    //         } else {
    //             it++;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_signals(&mut self, t: SUMOTime) {
        // there are 4 states for a signal
        // 1. approached and green
        // 2. approached and red
        // 3. not approached and trains could pass
        // 4. not approached and trains coult not pass
        //
        // for understanding conflicts better in sumo-gui, we want to show (3) as green. This
        // means we have to keep updating signals in state (4) until they change to (3)

        //std::cout << SIMTIME << " activeSignals=" << myActiveSignals.size() << "\n";
        // determinism: C++ orders myActiveSignals by numerical id, the Rust set orders by address
        let mut signals: Vec<Ptr<MSRailSignal>> = self.myActiveSignals.iter().copied().collect();
        signals.sort_by_key(|rs| rs.get_numerical_id());
        let mut remaining: Vec<Ptr<MSRailSignal>> = Vec::new();
        for rs in signals {
            let keep_active = rs.m().update_current_phase();
            // std::cout << SIMTIME << " update " << rs->getID() << " keepActive=" << keepActive << "\n";
            if rs.is_active() {
                rs.m().set_traffic_light_signals(t);
            }
            if keep_active {
                remaining.push(rs);
            }
        }
        self.myActiveSignals = remaining.into_iter().collect();
    }

    /// C++: `bool MSRailSignalControl::isSignalized(SUMOVehicleClass)` [static]
    // static bool isSignalized(SUMOVehicleClass svc) {
    //         return (mySignalizedClasses & svc) == svc;
    //     }
    #[allow(unused_variables)]
    pub fn is_signalized(svc: SUMOVehicleClass) -> bool {
        (my_signalized_classes() & svc) == svc
    }

    /// C++: `bool MSRailSignalControl::isMovingBlock(SVCPermissions)` [static]
    // static bool isMovingBlock(SVCPermissions svc) {
    //         return (myMBClasses & svc & SVC_RAIL_CLASSES) == (svc & SVC_RAIL_CLASSES);
    //     }
    #[allow(unused_variables)]
    pub fn is_moving_block(svc: SVCPermissions) -> bool {
        (my_mb_classes() & svc & SVC_RAIL_CLASSES) == (svc & SVC_RAIL_CLASSES)
    }

    /// C++: `bool MSRailSignalControl::isUsingDriveWays(SVCPermissions)` [static]
    // static bool isUsingDriveWays(SVCPermissions svc) {
    //         return ((mySignalizedClasses | myMBClasses) & svc) == svc;
    //     }
    #[allow(unused_variables)]
    pub fn is_using_drive_ways(svc: SVCPermissions) -> bool {
        ((my_signalized_classes() | my_mb_classes()) & svc) == svc
    }

    /// C++: `void MSRailSignalControl::initSignalized(SVCPermissions, SVCPermissions)` [static]
    // static void initSignalized(SVCPermissions svc, SVCPermissions mbSvc) {
    //         mySignalizedClasses = svc;
    //         myMBClasses = mbSvc;
    //     }
    #[allow(unused_variables)]
    pub fn init_signalized(svc: SVCPermissions, mbSvc: SVCPermissions) {
        MY_SIGNALIZED_CLASSES.with(|c| c.set(svc));
        MY_MB_CLASSES.with(|c| c.set(mbSvc));
    }

    /// C++: `void MSRailSignalControl::findDeadlockFoes(const MSDriveWay *, const Deadlock &, int, std::vector<const MSDriveWay *>)` [protected]
    // void
    // MSRailSignalControl::findDeadlockFoes(const MSDriveWay* dw, const Deadlock& dl, int signalIndex, std::vector<const MSDriveWay*> deadlockFoes) {
    //     if (deadlockFoes.size() < dl.signals.size()) {
    //         signalIndex = (signalIndex + 1) % dl.signals.size();
    //         const MSRailSignal* other = dl.signals[signalIndex];
    //         const MSJunction* otherJ = other->getJunction();
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //         std::cout << "      findDLfoes dw=" << dw->getID() << " foundFoes=" << toString(deadlockFoes) << " otherRS=" << other->getID() << "\n";
    // #endif
    //         deadlockFoes.push_back(dw);
    //         for (const MSDriveWay* follower : myDriveWaySucc[dw]) {
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //             std::cout << "        follower=" << follower->getID() << " followerFoes=" << toString(follower->getFoes()) << "\n";
    // #endif
    //             for (MSDriveWay* foe : follower->getFoes()) {
    //                 if (foe->getForward().back()->getToJunction() == otherJ) {
    //                     findDeadlockFoes(foe, dl, signalIndex, deadlockFoes);
    //                 } else if (foe->isSubDriveWay() && foe->getParent()->getForward().back()->getToJunction() == otherJ && foe->getParentRemainingLength() < myMaxTrainLength) {
    //                     // subdriveway-parents where the remaining part is short and thus a long train might reach back onto the subdriveway
    //                     // @note: technically we do not need to check this in meso because train ends cannot reach into multiple segments
    //                     // but it wouldn't help to make meso "better" by ignoring this foe relationship
    //                     findDeadlockFoes(foe->getParent(), dl, signalIndex, deadlockFoes);
    //                 }
    //             }
    //         }
    //     } else {
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //         std::cout << "        add deadlock check dlFoes=" << toString(deadlockFoes) << "\n";;
    // #endif
    //         for (const MSDriveWay* dldw : deadlockFoes) {
    //             if (!dldw->isDepartDriveway()) {
    //                 const MSRailSignal* rs = dynamic_cast<const MSRailSignal*>(dldw->getOrigin()->getTLLogic());
    //                 if (std::find(dl.signals.begin(), dl.signals.end(), rs) != dl.signals.end()) {
    //                     // moving into dldw frees up another part of the deadlock
    //                     continue;
    //                 }
    //             }
    //             std::vector<const MSDriveWay*> df(deadlockFoes);
    //             df.erase(std::find(df.begin(), df.end(), dldw));
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //             std::cout << "           dldw=" << dldw->getID() << " df=" << toString(df) << "\n";
    // #endif
    //             const_cast<MSDriveWay*>(dldw)->addDWDeadlock(df);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn find_deadlock_foes(&mut self, dw: Ptr<MSDriveWay>, dl: &MSRailSignalControl_Deadlock, signalIndex: i32, deadlockFoes: Vec<Ptr<MSDriveWay>>) {
        let mut deadlock_foes = deadlockFoes;
        if deadlock_foes.len() < dl.signals.len() {
            let signal_index = (signalIndex + 1) % dl.signals.len() as i32;
            let other = dl.signals[signal_index as usize];
            let other_j = other.get_junction();
            deadlock_foes.push(dw);
            // determinism: C++ orders the successors by MSDriveWay numerical id, the Rust set orders by address
            let followers: Vec<Ptr<MSDriveWay>> = {
                let succ = self.myDriveWaySucc.entry(dw).or_default();
                succ.iter().copied().collect()
            };
            let max_train_length = self.myMaxTrainLength;
            for follower in followers {
                for foe in follower.get_foes() {
                    if foe.get_forward().last().unwrap().myEdge.get_to_junction() == other_j {
                        self.find_deadlock_foes(*foe, dl, signal_index, deadlock_foes.clone());
                    } else if foe.is_sub_drive_way()
                        && foe.get_parent().get_forward().last().unwrap().myEdge.get_to_junction() == other_j
                        && foe.get_parent_remaining_length() < max_train_length
                    {
                        // subdriveway-parents where the remaining part is short and thus a long train might reach back onto the subdriveway
                        // @note: technically we do not need to check this in meso because train ends cannot reach into multiple segments
                        // but it wouldn't help to make meso "better" by ignoring this foe relationship
                        self.find_deadlock_foes(foe.get_parent(), dl, signal_index, deadlock_foes.clone());
                    }
                }
            }
        } else {
            for dldw in &deadlock_foes {
                if !dldw.is_depart_driveway() {
                    let logic = dldw.get_origin().get_tl_logic();
                    // the origin's logic is a rail signal only if it is one of the deadlock's signals
                    let is_deadlock_signal = dl.signals.iter().any(|s| s.cast::<MSTrafficLightLogic>() == logic);
                    if is_deadlock_signal {
                        // moving into dldw frees up another part of the deadlock
                        continue;
                    }
                }
                let mut df = deadlock_foes.clone();
                if let Some(pos) = df.iter().position(|x| x == dldw) {
                    df.remove(pos);
                }
                dldw.m().add_dw_deadlock(&df);
            }
        }
    }

    /// C++: `void MSRailSignalControl::findDeadlockFoesFromRoutes(const MSDriveWay *, const Deadlock &, int, std::vector<const MSDriveWay *>)` [protected]
    // void
    // MSRailSignalControl::findDeadlockFoesFromRoutes(const MSDriveWay* dw, const Deadlock& dl, int signalIndex, std::vector<const MSDriveWay*> deadlockFoes) {
    //     // the loaded edges must match the end of the driveway route
    //     const ConstMSEdgeVector& de = dl.edges[signalIndex];
    //     const size_t n = de.size();
    // #ifdef DEBUG_BUILD_DEADLOCK_CHECK
    //     std::cout << "      findDLfoesFromRoutes dw=" << dw->getID() << " foundFoes=" << toString(deadlockFoes)
    //         << "\n        dle=" << toString(de)
    //         << "\n        dwe=" << toString(ConstMSEdgeVector(dw->getRoute().end() - MIN2(n, dw->getRoute().size()), dw->getRoute().end()))
    //         << "\n";
    // #endif
    //     if (dw->getRoute().size() >= n && std::equal(de.begin(), de.end(),
    //                 dw->getRoute().end() - n, dw->getRoute().end())) {
    //         deadlockFoes.push_back(dw);
    //         if (deadlockFoes.size() < dl.signals.size()) {
    //             signalIndex = (signalIndex + 1) % dl.signals.size();
    //             const MSEdge* forwardEnd = dl.forwardEnds[signalIndex];
    //             for (MSDriveWay* cand : MSDriveWay::getEndingDriveWays(forwardEnd)) {
    //                 findDeadlockFoesFromRoutes(cand, dl, signalIndex, deadlockFoes);
    //             }
    //         } else {
    //             findDeadlockFoes(dw, dl, signalIndex, deadlockFoes);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn find_deadlock_foes_from_routes(&mut self, dw: Ptr<MSDriveWay>, dl: &MSRailSignalControl_Deadlock, signalIndex: i32, deadlockFoes: Vec<Ptr<MSDriveWay>>) {
        let mut deadlock_foes = deadlockFoes;
        // the loaded edges must match the end of the driveway route
        let de = &dl.edges[signalIndex as usize];
        let n = de.len();
        let route = dw.get_route();
        if route.len() >= n && route[route.len() - n..].iter().zip(de.iter()).all(|(a, b)| a == b) {
            deadlock_foes.push(dw);
            if deadlock_foes.len() < dl.signals.len() {
                let signal_index = (signalIndex + 1) % dl.signals.len() as i32;
                let forward_end = dl.forwardEnds[signal_index as usize];
                for cand in MSDriveWay::get_ending_drive_ways(forward_end).clone() {
                    self.find_deadlock_foes_from_routes(cand, dl, signal_index, deadlock_foes.clone());
                }
            } else {
                self.find_deadlock_foes(dw, dl, signalIndex, deadlock_foes.clone());
            }
        }
    }

    /// C++: `void MSRailSignalControl::MSRailSignalControl()` [private]
    /// Constructor
    // MSRailSignalControl::MSRailSignalControl():
    //     myMaxTrainLength(OptionsCont::getOptions().getFloat("railway.max-train-length"))
    // {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSRailSignalControl {
            base: MSNet_VehicleStateListener::new(),
            myUsedEdges: BTreeSet::new(),
            myWaitRelations: BTreeMap::new(),
            myWrittenDeadlocks: BTreeSet::new(),
            myLoadedDeadlocks: Vec::new(),
            mySignalDeadlocks: BTreeMap::new(),
            myDriveWaySucc: BTreeMap::new(),
            myDriveWayPred: BTreeMap::new(),
            mySignals: Vec::new(),
            mySwitchedGreenFlanks: Vec::new(),
            myDriveWayCompatibility: BTreeMap::new(),
            myActiveSignals: BTreeSet::new(),
            myMaxTrainLength: utils::options::options_cont::get_float("railway.max-train-length"),
        }
    }

    /// C++: `void MSRailSignalControl::updateDriveways(const MSEdge *)` [private]
    /// compute additioanl deadlock-check requirements for registered driveways
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn update_driveways(&mut self, used: Ptr<MSEdge>) {
        // C++ declares this private method but never defines nor calls it
    }

}

/// C++ class `MSRailSignalControl::WaitRelation` (microsim/traffic_lights/MSRailSignalControl.h)
#[derive(Clone)]
pub struct MSRailSignalControl_WaitRelation {
    pub railSignal: Ptr<MSRailSignal>,
    pub foe: Ptr<SUMOVehicle>,
    pub constraint: Ptr<MSRailSignalConstraint>,
}

impl MSRailSignalControl_WaitRelation {
    /// C++: `void WaitRelation::WaitRelation(const MSRailSignal *, const SUMOVehicle *, MSRailSignalConstraint *)`
    // WaitRelation(const MSRailSignal* _railSignal = nullptr, const SUMOVehicle* _foe = nullptr, MSRailSignalConstraint* _constraint = nullptr) :
    //             railSignal(_railSignal), foe(_foe), constraint(_constraint) {}
    #[allow(unused_variables)]
    pub fn new(_railSignal: Ptr<MSRailSignal>, _foe: Ptr<SUMOVehicle>, _constraint: Ptr<MSRailSignalConstraint>) -> Self {
        MSRailSignalControl_WaitRelation {
            railSignal: _railSignal,
            foe: _foe,
            constraint: _constraint,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    /// the class filters derived from the signalized / moving-block classes
    #[test]
    fn signalized_classes() {
        MSRailSignalControl::init_signalized(SVC_BUS, SVC_RAIL);
        assert!(MSRailSignalControl::is_signalized(SVC_BUS));
        assert!(!MSRailSignalControl::is_signalized(SVC_TRAM));
        assert!(MSRailSignalControl::is_moving_block(SVC_RAIL));
        assert!(!MSRailSignalControl::is_moving_block(SVC_TRAM));
        assert!(MSRailSignalControl::is_using_drive_ways(SVC_BUS));
        assert!(!MSRailSignalControl::is_using_drive_ways(SVC_TRAM));
    }
}

