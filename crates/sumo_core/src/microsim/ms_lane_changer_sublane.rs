//! Port of microsim/MSLaneChangerSublane.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::geom::POSITION_EPS;

// oc97: temporary, env-gated trace of indirect-left lateral decisions
#[inline]
fn oc97_trace(veh_id: &str, tag: &str, msg: impl std::fmt::Display) {
    if std::env::var_os("OC97_TRACE").is_none() { return; }
    if veh_id != "flow_0.15" && veh_id != "flow_0.16" && veh_id != "flow_0.52" { return; }
    let t = crate::microsim::ms_net::MSNet::get_instance().get_current_time_step();
    if !((t >= 140 && t <= 147) || (t >= 411 && t <= 417)) { return; }
    eprintln!("OC97 t={} {} {} {}", t, veh_id, tag, msg);
}

/// C++ typedef `MSAbstractLaneChangeModel::StateAndDist MSLaneChangerSublane::StateAndDist`
pub type MSLaneChangerSublane_StateAndDist = MSAbstractLaneChangeModel_StateAndDist;

/// Performs lane changing of vehicles
/// C++ class `MSLaneChangerSublane` (microsim/MSLaneChangerSublane.h)
#[repr(C)]
pub struct MSLaneChangerSublane {
    /// C++ base class `MSLaneChanger`
    pub base: MSLaneChanger,
    /// whether checkChangeOpposite was called for the current vehicle
    pub myCheckedChangeOpposite: bool,
}

impl std::ops::Deref for MSLaneChangerSublane {
    type Target = MSLaneChanger;
    fn deref(&self) -> &MSLaneChanger {
        &self.base
    }
}
impl std::ops::DerefMut for MSLaneChangerSublane {
    fn deref_mut(&mut self) -> &mut MSLaneChanger {
        &mut self.base
    }
}

fn sublane_check_change_opposite(lc: &mut MSLaneChanger, vehicle: Ptr<MSVehicle>, laneOffset: i32, targetLane: Ptr<MSLane>, leader: &(Ptr<MSVehicle>, f64), neighLead: &(Ptr<MSVehicle>, f64), neighFollow: &(Ptr<MSVehicle>, f64), preb: &Vec<MSVehicle_LaneQ>) -> bool {
    Ptr::from_mut(lc).cast::<MSLaneChangerSublane>().m().check_change_opposite(vehicle, laneOffset, targetLane, leader, neighLead, neighFollow, preb)
}

/// Virtual dispatch table of MSLaneChangerSublane; overrides checkChangeOpposite only
pub static MS_LANE_CHANGER_SUBLANE_VT: MSLaneChangerVt = MSLaneChangerVt {
    check_change_opposite: sublane_check_change_opposite,
};

impl MSLaneChangerSublane {
    /// C++: `void MSLaneChangerSublane::MSLaneChangerSublane(const std::vector<MSLane *> *, bool)`
    /// Constructor
    // MSLaneChangerSublane::MSLaneChangerSublane(const std::vector<MSLane*>* lanes, bool allowChanging) :
    //     MSLaneChanger(lanes, allowChanging) {
    //     // initialize siblings
    //     if (myChanger.front().lane->isInternal()) {
    //         for (ChangerIt ce = myChanger.begin(); ce != myChanger.end(); ++ce) {
    //             for (ChangerIt ce2 = myChanger.begin(); ce2 != myChanger.end(); ++ce2) {
    //                 if (ce != ce2 && ce->lane->getIncomingLanes().front().lane == ce2->lane->getIncomingLanes().front().lane) {
    //                     //std::cout << "addSibling lane=" << ce->lane->getID() << " offset=" << ce2->lane->getIndex() - ce->lane->getIndex() << "\n";
    //                     ce->siblings.push_back(ce2->lane->getIndex() - ce->lane->getIndex());
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(lanes: Ptr<Vec<Ptr<MSLane>>>, allowChanging: bool) -> Self {
        let mut base = MSLaneChanger::new(lanes, allowChanging);
        base.vt = &MS_LANE_CHANGER_SUBLANE_VT;
        let mut me = MSLaneChangerSublane {
            base,
            myCheckedChangeOpposite: false,
        };
        // initialize siblings
        if me.myChanger[0].lane.is_internal() {
            let n = me.myChanger.len();
            for ce in 0..n {
                for ce2 in 0..n {
                    if ce != ce2 && me.myChanger[ce].lane.get_incoming_lanes()[0].lane == me.myChanger[ce2].lane.get_incoming_lanes()[0].lane {
                        //std::cout << "addSibling lane=" << ce->lane->getID() << " offset=" << ce2->lane->getIndex() - ce->lane->getIndex() << "\n";
                        let offset = me.myChanger[ce2].lane.get_index() - me.myChanger[ce].lane.get_index();
                        me.myChanger[ce].siblings.push(offset);
                    }
                }
            }
        }
        me
    }

    /// C++: `void MSLaneChangerSublane::~MSLaneChangerSublane() noexcept` [virtual]
    /// Destructor.
    // MSLaneChangerSublane::~MSLaneChangerSublane() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool MSLaneChangerSublane::change()` [virtual, protected]
    /// Find a new candidate and try to change it.
    // bool
    // MSLaneChangerSublane::change() {
    //     // variant of change() for the sublane case
    //     myCandi = findCandidate();
    //     MSVehicle* vehicle = veh(myCandi);
    //     vehicle->getLaneChangeModel().clearNeighbors();
    // #ifdef DEBUG_ACTIONSTEPS
    //     if (DEBUG_COND) {
    //         std::cout << "\n" << SIMTIME << " CHANGE veh=" << vehicle->getID() << " lane=" << vehicle->getLane()->getID() << "\n";
    //     }
    // #endif
    //     assert(vehicle->getLane() == (*myCandi).lane);
    //     assert(!vehicle->getLaneChangeModel().isChangingLanes());
    //     if (/*!myAllowsChanging*/ vehicle->getLaneChangeModel().alreadyChanged() || vehicle->isStoppedOnLane()) {
    //         registerUnchanged(vehicle);
    //         if (vehicle->isStoppedOnLane()) {
    //             myCandi->lastStopped = vehicle;
    //         }
    //         return false;
    //     }
    //     if (!vehicle->isActive()) {
    // #ifdef DEBUG_ACTIONSTEPS
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " veh '" << vehicle->getID() << "' skips regular change checks." << std::endl;
    //         }
    // #endif
    //
    //         bool changed;
    //         // let TraCI influence the wish to change lanes during non-actionsteps
    //         checkTraCICommands(vehicle);
    //
    //         // Resume change
    //         changed = continueChangeSublane(vehicle, myCandi);
    // #ifdef DEBUG_ACTIONSTEPS
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " veh '" << vehicle->getID() << "' lcm->maneuverDist=" << vehicle->getLaneChangeModel().getManeuverDist()
    //                       << " lcm->speedLat=" << vehicle->getLaneChangeModel().getSpeedLat() << std::endl;
    //         }
    // #endif
    //         if (!changed) {
    //             registerUnchanged(vehicle);
    //         }
    //         return changed;
    //     }
    //
    // #ifdef DEBUG_ACTIONSTEPS
    //     if (DEBUG_COND) {
    //         std::cout << "\n" << SIMTIME << " veh '" << vehicle->getID() << "'  plans lanechange maneuver." << std::endl;
    //     }
    // #endif
    //     vehicle->updateBestLanes(); // needed?
    //     const bool isOpposite = vehicle->getLaneChangeModel().isOpposite();
    //     if (!isOpposite) {
    //         for (int i = 0; i < (int) myChanger.size(); ++i) {
    //             vehicle->adaptBestLanesOccupation(i, myChanger[i].dens);
    //         }
    //     }
    //     // update leaders beyond the current edge for all lanes
    //     for (ChangerIt ce = myChanger.begin(); ce != myChanger.end(); ++ce) {
    //         ce->aheadNext = getLeaders(ce, vehicle);
    //     }
    //     // update expected speeds
    //     int sublaneIndex = 0;
    //     for (ChangerIt ce = myChanger.begin(); ce != myChanger.end(); ++ce) {
    //         vehicle->getLaneChangeModel().updateExpectedSublaneSpeeds(ce->aheadNext, sublaneIndex, ce->lane->getIndex());
    //         for (int offset : ce->siblings) {
    //             // treat sibling lanes (internal lanes with the same origin lane) as if they have the same geometry
    //             ChangerIt ceSib = ce + offset;
    //             if (ceSib->lane->allowsVehicleClass(vehicle->getVClass())) {
    //                 vehicle->getLaneChangeModel().updateExpectedSublaneSpeeds(ceSib->aheadNext, sublaneIndex, ceSib->lane->getIndex());
    //             }
    //         }
    //         sublaneIndex += ce->ahead.numSublanes();
    //     }
    //
    //     // Check for changes to the opposite lane if vehicle is active
    // #ifdef DEBUG_ACTIONSTEPS
    //     if (DEBUG_COND) {
    //         std::cout << "  myChangeToOpposite=" << myChangeToOpposite << " isOpposite=" << isOpposite << " mayChangeRight=" << mayChange(-1) << " mayChangeLeft=" << mayChange(1) << "\n";
    //     }
    // #endif
    //
    //     const bool stopOpposite = hasOppositeStop(vehicle);
    //     const int traciState = vehicle->influenceChangeDecision(0);
    //     const bool traciRequestOpposite = !mayChange(1) && (traciState & LCA_LEFT) != 0;
    //
    //     if (myChangeToOpposite && (
    //                 // cannot overtake since there is only one usable lane (or emergency)
    //                 ((!mayChange(-1) && !mayChange(1)) || vehicle->getVClass() == SVC_EMERGENCY)
    //                 || traciRequestOpposite
    //                 || stopOpposite
    //                 // can alway come back from the opposite side
    //                 || isOpposite)) {
    //         const MSLeaderDistanceInfo& leaders = myCandi->aheadNext;
    //         if (leaders.hasVehicles() || isOpposite || stopOpposite || traciRequestOpposite) {
    //             std::pair<MSVehicle*, double> leader = findClosestLeader(leaders, vehicle);
    //             myCheckedChangeOpposite = false;
    //             if ((leader.first != nullptr || isOpposite || stopOpposite || traciRequestOpposite)
    //                     && changeOpposite(vehicle, leader, myCandi->lastStopped)) {
    //                 return true;
    //             } else if (myCheckedChangeOpposite) {
    //                 registerUnchanged(vehicle);
    //                 return false;
    //             }
    //             // try sublane change within current lane otherwise
    //         }
    //     }
    //
    //     LaneChangeAction alternatives = (LaneChangeAction)((mayChange(-1) ? LCA_RIGHT : LCA_NONE)
    //                                     | (mayChange(1) ? LCA_LEFT : LCA_NONE));
    //
    //     StateAndDist right = checkChangeHelper(vehicle, -1, alternatives);
    //     StateAndDist left = checkChangeHelper(vehicle, 1, alternatives);
    //     StateAndDist current = checkChangeHelper(vehicle, 0, alternatives);
    //
    //     StateAndDist decision = vehicle->getLaneChangeModel().decideDirection(current,
    //                             vehicle->getLaneChangeModel().decideDirection(right, left));
    // #ifdef DEBUG_DECISION
    //     if (vehicle->getLaneChangeModel().debugVehicle()) {
    //         std::cout << "\n" << SIMTIME << " decision=" << toString((LaneChangeAction)decision.state) << " dir=" << decision.dir << " latDist=" << decision.latDist << " maneuverDist=" << decision.maneuverDist << "\n";
    //     }
    // #endif
    //     vehicle->getLaneChangeModel().setOwnState(decision.state);
    //     if ((decision.state & LCA_WANTS_LANECHANGE) != 0 && (decision.state & LCA_BLOCKED) == 0) {
    //         // change if the vehicle wants to and is allowed to change
    // #ifdef DEBUG_MANEUVER
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " veh '" << vehicle->getID() << "' performing sublane change..." << std::endl;
    //         }
    // #endif
    //         const bool changed = startChangeSublane(vehicle, myCandi, decision.latDist, decision.maneuverDist);
    //         if (!changed) {
    //             registerUnchanged(vehicle);
    //         }
    //         return changed;
    //     }
    //     // @note this assumes vehicles can instantly abort any maneuvre in case of emergency
    //     abortLCManeuver(vehicle);
    //     registerUnchanged(vehicle);
    //
    //     if ((right.state & (LCA_URGENT)) != 0 && (left.state & (LCA_URGENT)) != 0) {
    //         // ... wants to go to the left AND to the right
    //         // just let them go to the right lane...
    //         left.state = 0;
    //     }
    //     return false;
    // }
    /// C++ `MSLaneChanger::laneChange(SUMOTime)` with the virtual `change()` resolved to the sublane override
    /// (the base loop is not reachable from `MSEdge` once the changer is stored as an `MSLaneChanger` pointer)
    pub fn lane_change_sublane(&mut self, t: SUMOTime) {
        self.init_changer();
        while self.veh_in_changer() {
            let haveChanged = self.change();
            self.update_changer(haveChanged);
        }
        self.update_lanes(t);
    }

    #[allow(unused_variables)]
    pub fn change(&mut self) -> bool {
        self.myCandi = self.find_candidate();
        let vehicle = self.veh(self.myCandi);
        vehicle.m().get_lane_change_model().clear_neighbors();

        if vehicle.m().get_lane_change_model().already_changed() || vehicle.is_stopped_on_lane() {
            self.register_unchanged(vehicle);
            if vehicle.is_stopped_on_lane() {
                let c = self.myCandi;
                self.myChanger[c].lastStopped = vehicle;
            }
            return false;
        }

        if !vehicle.is_active() {
            self.check_tra_ci_commands(vehicle);
            let mut from = self.myCandi;
            let changed = self.continue_change_sublane(vehicle, &mut from);
            self.myCandi = from;
            if !changed {
                self.register_unchanged(vehicle);
            }
            return changed;
        }

        vehicle.m().update_best_lanes(false, Ptr::null()); // needed?
        let isOpposite = vehicle.get_lane_change_model_2().is_opposite();
        if !isOpposite {
            for i in 0..self.myChanger.len() {
                let dens = self.myChanger[i].dens;
                vehicle.m().adapt_best_lanes_occupation(i as i32, dens);
            }
        }

        for ce in 0..self.myChanger.len() {
            let leaders = self.get_leaders(&ce, vehicle);
            self.myChanger[ce].aheadNext = leaders;
        }

        let mut sublaneIndex: i32 = 0;
        for ce in 0..self.myChanger.len() {
            let laneIdx = self.myChanger[ce].lane.get_index();
            vehicle.m().get_lane_change_model().update_expected_sublane_speeds(&self.myChanger[ce].aheadNext, sublaneIndex, laneIdx);
            let sibs = self.myChanger[ce].siblings.clone();
            for offset in sibs {
                let ceSib = (ce as i32 + offset) as usize;
                if self.myChanger[ceSib].lane.allows_vehicle_class(vehicle.get_v_class()) {
                    let sibLaneIdx = self.myChanger[ceSib].lane.get_index();
                    vehicle.m().get_lane_change_model().update_expected_sublane_speeds(&self.myChanger[ceSib].aheadNext, sublaneIndex, sibLaneIdx);
                }
            }
            sublaneIndex += self.myChanger[ce].ahead.num_sublanes();
        }

        let stopOpposite = MSLaneChanger::has_opposite_stop(vehicle);
        let traciState = vehicle.m().influence_change_decision(0);
        let traciRequestOpposite = !self.may_change(1) && (traciState & LCA_LEFT) != 0;
        if self.myChangeToOpposite
            && ((!self.may_change(-1) && !self.may_change(1)) || vehicle.get_v_class() == SVC_EMERGENCY || traciRequestOpposite || stopOpposite || isOpposite)
        {
            let leaders = &self.myChanger[self.myCandi].aheadNext;
            if leaders.has_vehicles() || isOpposite || stopOpposite || traciRequestOpposite {
                let leader = MSLaneChangerSublane::find_closest_leader(leaders, vehicle);
                self.myCheckedChangeOpposite = false;
                let lastStopped = self.myChanger[self.myCandi].lastStopped;
                if (!leader.0.is_null() || isOpposite || stopOpposite || traciRequestOpposite) && self.change_opposite(vehicle, leader, lastStopped) {
                    return true;
                } else if self.myCheckedChangeOpposite {
                    self.register_unchanged(vehicle);
                    return false;
                }
            }
        }

        let alternatives: LaneChangeAction = (if self.may_change(-1) { LCA_RIGHT } else { LCA_NONE }) | (if self.may_change(1) { LCA_LEFT } else { LCA_NONE });
        let right = self.check_change_helper(vehicle, -1, alternatives);
        let mut left = self.check_change_helper(vehicle, 1, alternatives);
        let current = self.check_change_helper(vehicle, 0, alternatives);
        let inner = vehicle.m().get_lane_change_model().decide_direction(sd_copy(&right), sd_copy(&left));
        let decision = vehicle.m().get_lane_change_model().decide_direction(sd_copy(&current), inner);
        // C++ debugVehicle() output sits under #ifdef DEBUG_DECISION (compiled out), not ported
        vehicle.m().get_lane_change_model().set_own_state(decision.state);
        if (decision.state & LCA_WANTS_LANECHANGE) != 0 && (decision.state & LCA_BLOCKED) == 0 {
            let mut from = self.myCandi;
            let changed = self.start_change_sublane(vehicle, &mut from, decision.latDist, decision.maneuverDist);
            self.myCandi = from;
            if !changed {
                self.register_unchanged(vehicle);
            }
            return changed;
        }

        self.abort_lc_maneuver(vehicle);
        self.register_unchanged(vehicle);
        if (right.state & LCA_URGENT) != 0 && (left.state & LCA_URGENT) != 0 {
            left.state = 0;
        }
        false
    }

    /// C++: `void MSLaneChangerSublane::initChanger()` [virtual, protected]
    /// Initialize the changer before looping over all vehicles.
    // void
    // MSLaneChangerSublane::initChanger() {
    //     MSLaneChanger::initChanger();
    //     // Prepare myChanger with a safe state.
    //     for (ChangerIt ce = myChanger.begin(); ce != myChanger.end(); ++ce) {
    //         ce->ahead = ce->lane->getPartialBeyond();
    //         ce->outsideBounds.clear();
    // //        std::cout << SIMTIME << " initChanger lane=" << ce->lane->getID() << " vehicles=" << toString(ce->lane->myVehicles) << "\n";
    // //        std::cout << SIMTIME << " initChanger lane=" << ce->lane->getID() << " partial vehicles=" << toString(ce->lane->myPartialVehicles) << "\n";
    // //        std::cout << SIMTIME << " initChanger lane=" << ce->lane->getID() << " partial vehicles beyond=" << toString(ce->ahead.toString()) << "\n";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_changer(&mut self) {
        MSLaneChanger::init_changer(&mut self.base);
        for ce in self.myChanger.iter_mut() {
            ce.ahead = ce.lane.get_partial_beyond();
            ce.outsideBounds.clear();
        }
    }

    /// C++: `void MSLaneChangerSublane::updateChanger(bool)` [virtual, protected]
    /// After the possible change, update the changer.
    // void
    // MSLaneChangerSublane::updateChanger(bool vehHasChanged) {
    //     MSLaneChanger::updateChanger(vehHasChanged);
    //     if (!vehHasChanged) {
    //         MSVehicle* lead = myCandi->lead;
    //         //std::cout << SIMTIME << " updateChanger lane=" << myCandi->lane->getID() << " lead=" << Named::getIDSecure(lead) << " lsol=" << lead->getLeftSideOnLane() << " rsol=" << lead->getRightSideOnLane() << "\n";
    //         if (lead->getLeftSideOnLane() < 0 || lead->getRightSideOnLane() > myCandi->lane->getWidth()) {
    //             myCandi->outsideBounds.push_back(lead);
    //         } else {
    //             myCandi->ahead.addLeader(lead, false, 0);
    //         }
    //         MSLane* shadowLane = lead->getLaneChangeModel().getShadowLane();
    //         if (shadowLane != nullptr && &shadowLane->getEdge() == &lead->getLane()->getEdge()) {
    //             assert(shadowLane->getIndex() < (int)myChanger.size());
    //             const double latOffset = lead->getLane()->getRightSideOnEdge() - shadowLane->getRightSideOnEdge();
    //             //std::cout << SIMTIME << " updateChanger shadowLane=" << shadowLane->getID() << " lead=" << Named::getIDSecure(lead) << "\n";
    //             (myChanger.begin() + shadowLane->getIndex())->ahead.addLeader(lead, false, latOffset);
    //         }
    //     }
    //     //std::cout << SIMTIME << " updateChanger: lane=" << myCandi->lane->getID() << " lead=" << Named::getIDSecure(myCandi->lead) << " ahead=" << myCandi->ahead.toString() << " vehHasChanged=" << vehHasChanged << "\n";
    //     //for (ChangerIt ce = myChanger.begin(); ce != myChanger.end(); ++ce) {
    //     //    std::cout << " lane=" << ce->lane->getID() << " vehicles=" << toString(ce->lane->myVehicles) << "\n";
    //     //}
    // }
    #[allow(unused_variables)]
    pub fn update_changer(&mut self, vehHasChanged: bool) {
        MSLaneChanger::update_changer(&mut self.base, vehHasChanged);
        if !vehHasChanged {
            let c = self.myCandi;
            let lead = self.myChanger[c].lead;
            if lead.get_left_side_on_lane() < 0.0 || lead.get_right_side_on_lane() > self.myChanger[c].lane.get_width() {
                self.myChanger[c].outsideBounds.push(lead);
            } else {
                self.myChanger[c].ahead.add_leader(lead, false, 0.0);
            }
            let shadowLane = lead.get_lane_change_model_2().get_shadow_lane();
            if !shadowLane.is_null() && (shadowLane.get_edge() as *const MSEdge) == (lead.get_lane().get_edge() as *const MSEdge) {
                let latOffset = lead.get_lane().get_right_side_on_edge() - shadowLane.get_right_side_on_edge();
                let idx = shadowLane.get_index() as usize;
                self.myChanger[idx].ahead.add_leader(lead, false, latOffset);
            }
        }
    }

    /// C++: `int MSLaneChangerSublane::checkChangeSublane(int, LaneChangeAction, const std::vector<MSVehicle::LaneQ> &, double &, double &) const` [const, protected]
    /// check whether sub-lane changing in the given direction is desirable and possible
    // int
    // MSLaneChangerSublane::checkChangeSublane(
    //     int laneOffset,
    //     LaneChangeAction alternatives,
    //     const std::vector<MSVehicle::LaneQ>& preb,
    //     double& latDist,
    //     double& maneuverDist) const {
    //
    //     ChangerIt target = myCandi + laneOffset;
    //     MSVehicle* vehicle = veh(myCandi);
    //     const MSLane& neighLane = *(target->lane);
    //     int blocked = 0;
    //
    //     MSLeaderDistanceInfo neighLeaders = target->aheadNext;
    //     MSLeaderDistanceInfo neighFollowers = target->lane->getFollowersOnConsecutive(vehicle, vehicle->getBackPositionOnLane(), true);
    //     MSLeaderDistanceInfo neighBlockers(neighLane.getWidth(), vehicle, vehicle->getLane()->getRightSideOnEdge() - neighLane.getRightSideOnEdge());
    //     MSLeaderDistanceInfo leaders = myCandi->aheadNext;
    //     addOutsideLeaders(vehicle, leaders);
    //     MSLeaderDistanceInfo followers = myCandi->lane->getFollowersOnConsecutive(vehicle, vehicle->getBackPositionOnLane(), true);
    //     MSLeaderDistanceInfo blockers(vehicle->getLane()->getWidth(), vehicle, 0);
    //
    //     // consider sibling lanes of the origin and target lane
    //     for (int offset : myCandi->siblings) {
    //         // treat sibling lanes (internal lanes with the same origin lane) as if they have the same geometry
    //         ChangerIt ceSib = myCandi + offset;
    //         MSLeaderDistanceInfo sibFollowers = ceSib->lane->getFollowersOnConsecutive(vehicle, vehicle->getBackPositionOnLane(), true);
    //         if (sibFollowers.hasVehicles()) {
    //             followers.addLeaders(sibFollowers);
    //         }
    //         if (ceSib->aheadNext.hasVehicles()) {
    //             leaders.addLeaders(ceSib->aheadNext);
    //         }
    // #ifdef DEBUG_SURROUNDING
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " ego=" << vehicle->getID() << " ahead=" << myCandi->aheadNext.toString() << " sib=" << ceSib->lane->getID() << " sibAhead=" << ceSib->aheadNext.toString() << " leaders=" << leaders.toString() << "\n";
    //         }
    // #endif
    //     }
    //     for (int offset : target->siblings) {
    //         // treat sibling lanes (internal lanes with the same origin lane) as if they have the same geometry
    //         ChangerIt ceSib = target + offset;
    //         MSLeaderDistanceInfo sibFollowers = ceSib->lane->getFollowersOnConsecutive(vehicle, vehicle->getBackPositionOnLane(), true);
    //         if (sibFollowers.hasVehicles()) {
    //             neighFollowers.addLeaders(sibFollowers);
    //         }
    //         if (ceSib->aheadNext.hasVehicles()) {
    //             neighLeaders.addLeaders(ceSib->aheadNext);
    //         }
    // #ifdef DEBUG_SURROUNDING
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " ego=" << vehicle->getID() << " neighAhead=" << target->aheadNext.toString() << " sib=" << ceSib->lane->getID() << " sibAhead=" << ceSib->aheadNext.toString() << " neighLeaders=" << neighLeaders.toString() << "\n";
    //         }
    // #endif
    //     }
    //
    //     // break leader symmetry
    //     if (laneOffset == -1 && neighLeaders.hasVehicles()) {
    //         neighLeaders.moveSamePosTo(vehicle, neighFollowers);
    //     }
    //
    // #ifdef DEBUG_SURROUNDING
    //     if (DEBUG_COND) std::cout << SIMTIME
    //                                   << " checkChangeSublane: veh=" << vehicle->getID()
    //                                   << " laneOffset=" << laneOffset
    //                                   << "\n  leaders=" << leaders.toString()
    //                                   << "\n  neighLeaders=" << neighLeaders.toString()
    //                                   << "\n  followers=" << followers.toString()
    //                                   << "\n  neighFollowers=" << neighFollowers.toString()
    //                                   << "\n";
    // #endif
    //
    //
    //     const int wish = vehicle->getLaneChangeModel().wantsChangeSublane(
    //                          laneOffset, alternatives,
    //                          leaders, followers, blockers,
    //                          neighLeaders, neighFollowers, neighBlockers,
    //                          neighLane, preb,
    //                          &(myCandi->lastBlocked), &(myCandi->firstBlocked), latDist, maneuverDist, blocked);
    //
    //
    //     if (checkOpened && (blocked & LCA_BLOCKED) == 0 && (wish & LCA_WANTS_LANECHANGE) != 0
    //             && vehicle->getLane()->isNormal()
    //             && vehicle->getBestLanesContinuation().size() > 1) {
    //         const MSLink* link = vehicle->getLane()->getLinkTo(vehicle->getBestLanesContinuation()[1]);
    //         if (link != nullptr && link->isEntryLink()) {
    //             const MSLink* link2 = link->getParallelLink(laneOffset);
    //             if (link2 != nullptr) {
    //                 auto api = link->getApproachingPtr(vehicle);
    //                 if (api != nullptr) {
    //                     if (!link2->opened(api->arrivalTime, api->arrivalSpeed, api->leaveSpeed, vehicle->getLength(),
    //                                        vehicle->getImpatience(), vehicle->getCarFollowModel().getMaxDecel(), vehicle->getWaitingTime(), vehicle->getLateralPositionOnLane(),
    //                                        nullptr, false, vehicle, api->dist)) {
    //                         //std::cout << SIMTIME << " unsafeLC " << vehicle->getID() << "\n";
    //                         blocked |= LCA_BLOCKED;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     int state = blocked | wish;
    //
    //     // XXX
    //     // do are more careful (but expensive) check to ensure that a
    //     // safety-critical leader is not being overlooked
    //
    //     // let TraCI influence the wish to change lanes and the security to take
    //     const int oldstate = state;
    //     state = vehicle->influenceChangeDecision(state);
    // #ifdef DEBUG_STATE
    //     if (DEBUG_COND && state != oldstate) {
    //         std::cout << SIMTIME << " veh=" << vehicle->getID() << " stateAfterTraCI=" << toString((LaneChangeAction)state) << " original=" << toString((LaneChangeAction)oldstate) << "\n";
    //     }
    // #endif
    //     vehicle->getLaneChangeModel().getCanceledState(laneOffset) |= blocked;
    //     vehicle->getLaneChangeModel().saveLCState(laneOffset, oldstate, state);
    //     if (laneOffset != 0) {
    //         vehicle->getLaneChangeModel().saveNeighbors(laneOffset, neighFollowers, neighLeaders);
    //     }
    //     return state;
    // }
    #[allow(unused_variables)]
    pub fn check_change_sublane(&self, laneOffset: i32, alternatives: LaneChangeAction, preb: &Vec<MSVehicle_LaneQ>, latDist: &mut f64, maneuverDist: &mut f64) -> i32 {
        let c = self.myCandi;
        let target = ((c as i32) + laneOffset) as usize;
        let vehicle = self.veh(c);
        let neighLane = self.myChanger[target].lane;
        let mut blocked: i32 = 0;

        let backPos = vehicle.get_back_position_on_lane_2();
        let mut neighLeaders = self.myChanger[target].aheadNext.clone();
        let mut neighFollowers = self.myChanger[target].lane.get_followers_on_consecutive(vehicle, backPos, true, -1.0, MSLane_MinorLinkMode::FOLLOW_ALWAYS, false);
        let mut neighBlockers = MSLeaderDistanceInfo::new(neighLane.get_width(), vehicle, vehicle.get_lane().get_right_side_on_edge() - neighLane.get_right_side_on_edge());
        let mut leaders = self.myChanger[c].aheadNext.clone();
        self.add_outside_leaders(vehicle, &mut leaders);
        let mut followers = self.myChanger[c].lane.get_followers_on_consecutive(vehicle, backPos, true, -1.0, MSLane_MinorLinkMode::FOLLOW_ALWAYS, false);
        let mut blockers = MSLeaderDistanceInfo::new(vehicle.get_lane().get_width(), vehicle, 0.0);

        // consider sibling lanes of the origin and target lane
        for offset in self.myChanger[c].siblings.clone() {
            // treat sibling lanes (internal lanes with the same origin lane) as if they have the same geometry
            let ceSib = ((c as i32) + offset) as usize;
            let mut sibFollowers = self.myChanger[ceSib].lane.get_followers_on_consecutive(vehicle, backPos, true, -1.0, MSLane_MinorLinkMode::FOLLOW_ALWAYS, false);
            if sibFollowers.has_vehicles() {
                followers.add_leaders(&mut sibFollowers);
            }
            let sib = Ptr::new(&self.myChanger[ceSib]);
            if sib.aheadNext.has_vehicles() {
                leaders.add_leaders(&mut sib.m().aheadNext);
            }
        }
        for offset in self.myChanger[target].siblings.clone() {
            // treat sibling lanes (internal lanes with the same origin lane) as if they have the same geometry
            let ceSib = ((target as i32) + offset) as usize;
            let mut sibFollowers = self.myChanger[ceSib].lane.get_followers_on_consecutive(vehicle, backPos, true, -1.0, MSLane_MinorLinkMode::FOLLOW_ALWAYS, false);
            if sibFollowers.has_vehicles() {
                neighFollowers.add_leaders(&mut sibFollowers);
            }
            let sib = Ptr::new(&self.myChanger[ceSib]);
            if sib.aheadNext.has_vehicles() {
                neighLeaders.add_leaders(&mut sib.m().aheadNext);
            }
        }

        // break leader symmetry
        if laneOffset == -1 && neighLeaders.has_vehicles() {
            neighLeaders.move_same_pos_to(vehicle, &mut neighFollowers);
        }

        let wish = vehicle.m().get_lane_change_model().wants_change_sublane(
            laneOffset,
            alternatives,
            &leaders,
            &followers,
            &blockers,
            &neighLeaders,
            &neighFollowers,
            &neighBlockers,
            &*neighLane,
            preb,
            Ptr::new(&self.myChanger[c].lastBlocked),
            Ptr::new(&self.myChanger[c].firstBlocked),
            latDist,
            maneuverDist,
            &mut blocked,
        );

        if self.checkOpened && (blocked & LCA_BLOCKED) == 0 && (wish & LCA_WANTS_LANECHANGE) != 0
            && vehicle.get_lane().is_normal() && vehicle.get_best_lanes_continuation().len() > 1 {
            let link = vehicle.get_lane().get_link_to(vehicle.get_best_lanes_continuation()[1]);
            if !link.is_null() && link.is_entry_link() {
                let link2 = link.get_parallel_link(laneOffset);
                if !link2.is_null() {
                    let api = link.get_approaching_ptr(vehicle.cast::<SUMOVehicle>());
                    if !api.is_null() {
                        if !link2.opened(
                            api.arrivalTime,
                            api.arrivalSpeed,
                            api.leaveSpeed,
                            vehicle.get_length(),
                            vehicle.get_impatience(),
                            vehicle.get_car_follow_model().get_max_decel(),
                            vehicle.get_waiting_time(false),
                            vehicle.get_lateral_position_on_lane(),
                            Ptr::null(),
                            false,
                            vehicle.cast::<SUMOTrafficObject>(),
                            api.dist,
                        ) {
                            blocked |= LCA_BLOCKED;
                        }
                    }
                }
            }
        }

        // let TraCI influence the wish to change lanes and the security to take
        let mut state = blocked | wish;
        let oldstate = state;
        state = vehicle.m().influence_change_decision(state);
        *vehicle.m().get_lane_change_model().get_canceled_state(laneOffset) |= blocked;
        vehicle.m().get_lane_change_model().save_lc_state(laneOffset, oldstate, state);
        if laneOffset != 0 {
            vehicle.m().get_lane_change_model().save_neighbors(laneOffset, &neighFollowers, &neighLeaders);
        }
        state
    }

    /// C++: `bool MSLaneChangerSublane::checkChangeOpposite(MSVehicle *, int, MSLane *, const std::pair<MSVehicle *const, double> &, const std::pair<MSVehicle *const, double> &, const std::pair<MSVehicle *const, double> &, const std::vector<MSVehicle::LaneQ> &)` [virtual, protected]
    // bool
    // MSLaneChangerSublane::checkChangeOpposite(
    //     MSVehicle* vehicle,
    //     int laneOffset,
    //     MSLane* targetLane,
    //     const std::pair<MSVehicle* const, double>& leader,
    //     const std::pair<MSVehicle* const, double>& neighLead,
    //     const std::pair<MSVehicle* const, double>& neighFollow,
    //     const std::vector<MSVehicle::LaneQ>& preb) {
    //     myCheckedChangeOpposite = true;
    //
    //     UNUSED_PARAMETER(leader);
    //     UNUSED_PARAMETER(neighLead);
    //     UNUSED_PARAMETER(neighFollow);
    //
    //     const MSLane& neighLane = *targetLane;
    //     MSLane* curLane = myCandi->lane;
    //
    //     MSLeaderDistanceInfo neighLeaders(targetLane->getWidth(), nullptr, 0);
    //     MSLeaderDistanceInfo neighFollowers(targetLane->getWidth(), nullptr, 0);
    //     MSLeaderDistanceInfo neighBlockers(targetLane->getWidth(), nullptr, 0);
    //     MSLeaderDistanceInfo leaders(curLane->getWidth(), nullptr, 0);
    //     MSLeaderDistanceInfo followers(curLane->getWidth(), nullptr, 0);
    //     MSLeaderDistanceInfo blockers(curLane->getWidth(), nullptr, 0);
    //
    //     const double backPosOnTarget = vehicle->getLane()->getOppositePos(vehicle->getBackPositionOnLane());
    //     if (vehicle->getLaneChangeModel().isOpposite()) {
    //         leaders = curLane->getFollowersOnConsecutive(vehicle, vehicle->getPositionOnLane(), true, -1, MSLane::MinorLinkMode::FOLLOW_ONCOMING);
    //         leaders.fixOppositeGaps(false);
    //         curLane->addLeaders(vehicle, vehicle->getBackPositionOnLane(), followers);
    //         followers.fixOppositeGaps(true);
    //         neighFollowers = targetLane->getFollowersOnConsecutive(vehicle, backPosOnTarget, true);
    //         neighFollowers.fixOppositeGaps(false);
    //         // artificially increase the position to ensure that ego is not added as a leader
    //         const double posOnTarget = backPosOnTarget + vehicle->getVehicleType().getLength() + POSITION_EPS;
    //         targetLane->addLeaders(vehicle, posOnTarget, neighLeaders);
    //         neighLeaders.patchGaps(2 * POSITION_EPS);
    //         int sublaneIndex = 0;
    //         for (int i = 0; i < targetLane->getIndex(); i++) {
    //             sublaneIndex += MSLeaderInfo(targetLane->getEdge().getLanes()[i]->getWidth()).numSublanes();
    //         }
    //         vehicle->getLaneChangeModel().updateExpectedSublaneSpeeds(neighLeaders, sublaneIndex, targetLane->getIndex());
    //     } else {
    //         leaders = myCandi->aheadNext;
    //         followers = myCandi->lane->getFollowersOnConsecutive(vehicle, vehicle->getBackPositionOnLane(), true);
    //         const double posOnTarget = backPosOnTarget - vehicle->getVehicleType().getLength();
    //         targetLane->addLeaders(vehicle, backPosOnTarget, neighFollowers, true);
    //         neighFollowers.fixOppositeGaps(true);
    //         neighLeaders = targetLane->getFollowersOnConsecutive(vehicle, posOnTarget, true);
    //         neighLeaders.fixOppositeGaps(false);
    //     }
    //
    //
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //     if (DEBUG_COND) std::cout << SIMTIME
    //                                   << " checkChangeOppositeSublane: veh=" << vehicle->getID()
    //                                   << " isOpposite=" << vehicle->getLaneChangeModel().isOpposite()
    //                                   << " laneOffset=" << laneOffset
    //                                   << "\n  leaders=" << leaders.toString()
    //                                   << "\n  neighLeaders=" << neighLeaders.toString()
    //                                   << "\n  followers=" << followers.toString()
    //                                   << "\n  neighFollowers=" << neighFollowers.toString()
    //                                   << "\n";
    // #endif
    //
    //     LaneChangeAction alternatives = (LaneChangeAction)((mayChange(-1) ? LCA_RIGHT : LCA_NONE)
    //                                     | (mayChange(1) ? LCA_LEFT : LCA_NONE));
    //
    //     int blocked = 0;
    //     double latDist = 0;
    //     double maneuverDist = 0;
    //     const int wish = vehicle->getLaneChangeModel().wantsChangeSublane(
    //                          laneOffset, alternatives,
    //                          leaders, followers, blockers,
    //                          neighLeaders, neighFollowers, neighBlockers,
    //                          neighLane, preb,
    //                          &(myCandi->lastBlocked), &(myCandi->firstBlocked), latDist, maneuverDist, blocked);
    //     int state = blocked | wish;
    //
    //     const int oldstate = state;
    //     state = vehicle->influenceChangeDecision(state);
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //     if (DEBUG_COND && state != oldstate) {
    //         std::cout << SIMTIME << " veh=" << vehicle->getID() << " stateAfterTraCI=" << toString((LaneChangeAction)state) << " original=" << toString((LaneChangeAction)oldstate) << "\n";
    //     }
    // #endif
    //     vehicle->getLaneChangeModel().saveLCState(laneOffset, oldstate, state);
    //     if (laneOffset != 0) {
    //         vehicle->getLaneChangeModel().saveNeighbors(laneOffset, neighFollowers, neighLeaders);
    //     }
    //
    //     if ((state & LCA_WANTS_LANECHANGE) != 0 && (state & LCA_BLOCKED) == 0) {
    //         // change if the vehicle wants to and is allowed to change
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " veh '" << vehicle->getID() << "' performing sublane change latDist=" << latDist << " maneuverDist=" << maneuverDist << "\n";
    //         }
    // #endif
    //         vehicle->getLaneChangeModel().setOwnState(state);
    //         return startChangeSublane(vehicle, myCandi, latDist, maneuverDist);
    //     } else {
    //         vehicle->getLaneChangeModel().setSpeedLat(0);
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_change_opposite(&mut self, vehicle: Ptr<MSVehicle>, laneOffset: i32, targetLane: Ptr<MSLane>, leader: &(Ptr<MSVehicle>, f64), neighLead: &(Ptr<MSVehicle>, f64), neighFollow: &(Ptr<MSVehicle>, f64), preb: &Vec<MSVehicle_LaneQ>) -> bool {
        self.myCheckedChangeOpposite = true;

        let neighLane = targetLane;
        let curLane = self.myChanger[self.myCandi].lane;

        let mut neighLeaders = MSLeaderDistanceInfo::new(targetLane.get_width(), Ptr::null(), 0.0);
        let mut neighFollowers = MSLeaderDistanceInfo::new(targetLane.get_width(), Ptr::null(), 0.0);
        let mut neighBlockers = MSLeaderDistanceInfo::new(targetLane.get_width(), Ptr::null(), 0.0);
        let mut leaders = MSLeaderDistanceInfo::new(curLane.get_width(), Ptr::null(), 0.0);
        let mut followers = MSLeaderDistanceInfo::new(curLane.get_width(), Ptr::null(), 0.0);
        let mut blockers = MSLeaderDistanceInfo::new(curLane.get_width(), Ptr::null(), 0.0);

        let backPosOnTarget = vehicle.get_lane().get_opposite_pos(vehicle.get_back_position_on_lane_2());
        if vehicle.get_lane_change_model_2().is_opposite() {
            leaders = curLane.get_followers_on_consecutive(vehicle, vehicle.get_position_on_lane(), true, -1.0, MSLane_MinorLinkMode::FOLLOW_ONCOMING, false);
            leaders.fix_opposite_gaps(false);
            curLane.m().add_leaders(vehicle, vehicle.get_back_position_on_lane_2(), &mut followers, false);
            followers.fix_opposite_gaps(true);
            neighFollowers = targetLane.get_followers_on_consecutive(vehicle, backPosOnTarget, true, -1.0, MSLane_MinorLinkMode::FOLLOW_ALWAYS, false);
            neighFollowers.fix_opposite_gaps(false);
            // artificially increase the position to ensure that ego is not added as a leader
            let posOnTarget = backPosOnTarget + vehicle.get_vehicle_type().get_length() + POSITION_EPS;
            targetLane.m().add_leaders(vehicle, posOnTarget, &mut neighLeaders, false);
            neighLeaders.patch_gaps(2.0 * POSITION_EPS);
            let mut sublaneIndex: i32 = 0;
            for i in 0..targetLane.get_index() {
                sublaneIndex += MSLeaderInfo::new(targetLane.get_edge().get_lanes()[i as usize].get_width(), Ptr::null(), 0.0).num_sublanes();
            }
            vehicle.m().get_lane_change_model().update_expected_sublane_speeds(&neighLeaders, sublaneIndex, targetLane.get_index());
        } else {
            leaders = self.myChanger[self.myCandi].aheadNext.clone();
            followers = curLane.get_followers_on_consecutive(vehicle, vehicle.get_back_position_on_lane_2(), true, -1.0, MSLane_MinorLinkMode::FOLLOW_ALWAYS, false);
            let posOnTarget = backPosOnTarget - vehicle.get_vehicle_type().get_length();
            targetLane.m().add_leaders(vehicle, backPosOnTarget, &mut neighFollowers, true);
            neighFollowers.fix_opposite_gaps(true);
            neighLeaders = targetLane.get_followers_on_consecutive(vehicle, posOnTarget, true, -1.0, MSLane_MinorLinkMode::FOLLOW_ALWAYS, false);
            neighLeaders.fix_opposite_gaps(false);
        }

        let alternatives: LaneChangeAction = (if self.may_change(-1) { LCA_RIGHT } else { LCA_NONE })
            | (if self.may_change(1) { LCA_LEFT } else { LCA_NONE });

        let mut blocked: i32 = 0;
        let mut latDist: f64 = 0.0;
        let mut maneuverDist: f64 = 0.0;
        let wish = vehicle.m().get_lane_change_model().wants_change_sublane(
            laneOffset,
            alternatives,
            &leaders,
            &followers,
            &blockers,
            &neighLeaders,
            &neighFollowers,
            &neighBlockers,
            &*neighLane,
            preb,
            Ptr::new(&self.myChanger[self.myCandi].lastBlocked),
            Ptr::new(&self.myChanger[self.myCandi].firstBlocked),
            &mut latDist,
            &mut maneuverDist,
            &mut blocked,
        );
        let mut state = blocked | wish;

        let oldstate = state;
        state = vehicle.m().influence_change_decision(state);
        vehicle.m().get_lane_change_model().save_lc_state(laneOffset, oldstate, state);
        if laneOffset != 0 {
            vehicle.m().get_lane_change_model().save_neighbors(laneOffset, &neighFollowers, &neighLeaders);
        }

        if (state & LCA_WANTS_LANECHANGE) != 0 && (state & LCA_BLOCKED) == 0 {
            // change if the vehicle wants to and is allowed to change
            vehicle.m().get_lane_change_model().set_own_state(state);
            let mut from = self.myCandi;
            let changed = self.start_change_sublane(vehicle, &mut from, latDist, maneuverDist);
            self.myCandi = from;
            return changed;
        } else {
            vehicle.m().get_lane_change_model().set_speed_lat(0.0);
            return false;
        }
    }

    /// C++: `bool MSLaneChangerSublane::continueChangeSublane(MSVehicle *, ChangerIt &)` [protected]
    /// Continue a sublane-lane change maneuver and return whether the midpoint was passed in this step
    // bool
    // MSLaneChangerSublane::continueChangeSublane(MSVehicle* vehicle, ChangerIt& from) {
    //     // lateral distance to complete maneuver
    //     double remLatDist = vehicle->getLaneChangeModel().getManeuverDist();
    //     if (remLatDist == 0) {
    //         return false;
    //     }
    //     const bool urgent = (vehicle->getLaneChangeModel().getOwnState() & LCA_URGENT) != 0;
    //     const double nextLatDist = SPEED2DIST(vehicle->getLaneChangeModel().computeSpeedLat(remLatDist, remLatDist, urgent));
    // #ifdef DEBUG_MANEUVER
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " vehicle '" << vehicle->getID() << "' continueChangeSublane()"
    //                   << " remLatDist=" << remLatDist << " nextLatDist=" << nextLatDist
    //                   << std::endl;
    //     }
    // #endif
    //
    //     const bool changed = startChangeSublane(vehicle, from, nextLatDist, remLatDist);
    //     return changed;
    // }
    #[allow(unused_variables)]
    pub fn continue_change_sublane(&mut self, vehicle: Ptr<MSVehicle>, from: &mut MSLaneChanger_ChangerIt) -> bool {
        let mut remLatDist = vehicle.m().get_lane_change_model().get_maneuver_dist();
        if remLatDist == 0.0 {
            return false;
        }
        let urgent = (vehicle.m().get_lane_change_model().get_own_state() & LCA_URGENT) != 0;
        let speedLat = vehicle.m().get_lane_change_model().compute_speed_lat(remLatDist, &mut remLatDist, urgent);
        let nextLatDist = speed2dist(speedLat);
        let changed = self.start_change_sublane(vehicle, from, nextLatDist, remLatDist);
        changed
    }

    /// C++: `bool MSLaneChangerSublane::startChangeSublane(MSVehicle *, ChangerIt &, double, double)` [protected]
    /// change by the specified amount and return whether a new lane was entered
    // bool
    // MSLaneChangerSublane::startChangeSublane(MSVehicle* vehicle, ChangerIt& from, double latDist, double maneuverDist) {
    //     if (vehicle->isRemoteControlled()) {
    //         return false;
    //     }
    //     MSLane* source = from->lane;
    //     // Prevent continuation of LC beyond lane borders if change is not allowed
    //     double distToRightLaneBorder = vehicle->getLane()->getWidth() * 0.5 + vehicle->getLateralPositionOnLane() - vehicle->getWidth() * 0.5;
    //     double distToLeftLaneBorder = vehicle->getLane()->getWidth() * 0.5 - vehicle->getLateralPositionOnLane() - vehicle->getWidth() * 0.5;
    //     if (vehicle->getLaneChangeModel().isOpposite()) {
    //         std::swap(distToRightLaneBorder, distToLeftLaneBorder);
    //     }
    //     // determine direction of LC
    //     int direction = 0;
    //     if (latDist > 0 && latDist > distToLeftLaneBorder) {
    //         direction = 1;
    //     } else if (latDist < 0 && -latDist > distToRightLaneBorder) {
    //         direction = -1;
    //     }
    //     const int changerDirection = vehicle->getLaneChangeModel().isOpposite() ? -direction : direction;
    //     ChangerIt to = from;
    // #ifdef DEBUG_MANEUVER
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " vehicle '" << vehicle->getID() << "' latDist=" << latDist << "  maneuverDist=" << maneuverDist
    //                   << " distRight=" << distToRightLaneBorder << " distLeft=" << distToLeftLaneBorder
    //                   << " dir=" << direction << " cDir=" << changerDirection << " mayChange=" << mayChange(changerDirection) << "\n";
    //     }
    // #endif
    //     if (mayChange(changerDirection)) {
    //         to = from + changerDirection;
    //     } else if (changerDirection == 1 && source->getOpposite() != nullptr) {
    //         // change to the opposite direction lane
    //         to = (source->getOpposite()->getEdge().myLaneChanger->getChanger().end() - 1);
    //     } else {
    //         // This may occur during maneuver continuation in non-actionsteps.
    //         // TODO: Understand better why and test later if additional sublane actionstep debugging resolves this
    //         // (XXX: perhaps one should try to extrapolate check for this case before to avoid maneuver initialization
    //         //       similar as for continuous LC in MSLaneChanger::checkChange())
    //         //assert(false);
    //         abortLCManeuver(vehicle);
    //         return false;
    //     }
    //
    //     // The following does:
    //     // 1) update vehicles lateral position according to latDist and target lane
    //     // 2) distinguish several cases
    //     //   a) vehicle moves completely within the same lane
    //     //   b) vehicle intersects another lane
    //     //      - vehicle must be moved to the lane where its midpoint is (either old or new)
    //     //      - shadow vehicle must be created/moved to the other lane if the vehicle intersects it
    //     // 3) updated dens of all lanes that hold the vehicle or its shadow
    //
    //     vehicle->myState.myPosLat += latDist * (vehicle->getLaneChangeModel().isOpposite() ? -1 : 1);
    //     for (int i = 0; i < (int)vehicle->myFurtherLanesPosLat.size(); i++) {
    //         vehicle->myFurtherLanesPosLat[i] += latDist * (vehicle->getLaneChangeModel().isOpposite() ? -1 : 1);
    //     }
    //     vehicle->myCachedPosition = Position::INVALID;
    //     vehicle->getLaneChangeModel().setSpeedLat(DIST2SPEED(latDist));
    // #ifdef DEBUG_MANEUVER
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " vehicle '" << vehicle->getID() << "' with maneuverDist=" << maneuverDist
    //                   << " and committedSpeed=" << vehicle->getLaneChangeModel().getCommittedSpeed()
    //                   << " increments lateral position by latDist=" << latDist << std::endl;
    //     }
    // #endif
    // #ifdef DEBUG_SURROUNDING
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " vehicle '" << vehicle->getID() << "'\n    to->ahead=" << to->ahead.toString()
    //                   << "'\n    to->aheadNext=" << to->aheadNext.toString()
    //                   << std::endl;
    //     }
    // #endif
    //     const bool completedPriorManeuver = fabs(vehicle->getLaneChangeModel().getPreviousManeuverDist()) < NUMERICAL_EPS;
    //     const bool completedManeuver = fabs(maneuverDist - latDist) < NUMERICAL_EPS;
    //     vehicle->getLaneChangeModel().updateSafeLatDist(latDist);
    //
    //     // current maneuver is aborted when direction or reason changes
    //     const int priorReason = vehicle->getLaneChangeModel().getPrevState() & LCA_CHANGE_REASONS & ~LCA_SUBLANE;
    //     const int reason = vehicle->getLaneChangeModel().getOwnState() & LCA_CHANGE_REASONS & ~LCA_SUBLANE;
    // #ifdef DEBUG_MANEUVER
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " vehicle '" << vehicle->getID()
    //                   << "' completedPriorManeuver=" << completedPriorManeuver
    //                   << " completedManeuver=" << completedManeuver
    //                   << " priorReason=" << toString((LaneChangeAction)priorReason)
    //                   << " reason=" << toString((LaneChangeAction)reason)
    //                   << " priorManeuverDist=" << vehicle->getLaneChangeModel().getPreviousManeuverDist()
    //                   << " maneuverDist=" << maneuverDist
    //                   << " latDist=" << latDist
    //                   << std::endl;
    //     }
    // #endif
    //     if (!completedManeuver && !completedPriorManeuver && priorReason != 0 &&
    //             (vehicle->getLaneChangeModel().getPreviousManeuverDist() * latDist < 0
    //              || priorReason != reason)) {
    //         const int priorDirection = vehicle->getLaneChangeModel().getPreviousManeuverDist() > 0 ? 1 : -1;
    //         // original from cannot be reconstructed
    // #ifdef DEBUG_MANEUVER
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " startChangeSublane abort priorReason=" << toString((LaneChangeAction)priorReason)
    //                       << " reason=" << toString((LaneChangeAction)reason) << " veh=" << vehicle->getID() << "\n";
    //         }
    // #endif
    //         outputLCEnded(vehicle, from, from, priorDirection);
    //     }
    //
    //     outputLCStarted(vehicle, from, to, direction, maneuverDist);
    //     vehicle->getLaneChangeModel().setManeuverDist(maneuverDist - latDist);
    //     const bool changedToNewLane = checkChangeToNewLane(vehicle, direction, from, to);
    //
    //     MSLane* oldShadowLane = vehicle->getLaneChangeModel().getShadowLane();
    //     vehicle->getLaneChangeModel().updateShadowLane();
    //     MSLane* shadowLane = vehicle->getLaneChangeModel().getShadowLane();
    //     if (shadowLane != nullptr && shadowLane != oldShadowLane
    //             && &shadowLane->getEdge() == &source->getEdge()) {
    //         assert(oldShadowLane == 0 || vehicle->getLaneChangeModel().isOpposite() || to != from);
    //         const double latOffset = vehicle->getLane()->getRightSideOnEdge() - shadowLane->getRightSideOnEdge();
    //         (myChanger.begin() + shadowLane->getIndex())->ahead.addLeader(vehicle, false, latOffset);
    //     }
    //     if (completedManeuver) {
    //         outputLCEnded(vehicle, from, to, direction);
    //     }
    //
    //     // Update maneuver reservations on target lanes
    //     MSLane* targetLane = vehicle->getLaneChangeModel().updateTargetLane();
    //     if (!changedToNewLane && targetLane != nullptr
    //             && vehicle->getActionStepLength() > DELTA_T
    //             && &targetLane->getEdge() == &source->getEdge()
    //        ) {
    //         const int dir = (vehicle->getLaneChangeModel().getManeuverDist() > 0 ? 1 : -1);
    //         ChangerIt target = from + dir;
    //         const double actionStepDist = dir * vehicle->getVehicleType().getMaxSpeedLat() * vehicle->getActionStepLengthSecs();
    //         const double latOffset = vehicle->getLatOffset(targetLane) + actionStepDist;
    //         target->ahead.addLeader(vehicle, false, latOffset);
    //         //std::cout << SIMTIME << " veh=" << vehicle->getID() << " target=" << targetLane->getID()
    //         //    << " actionStepDist=" << actionStepDist << " latOffset=" << latOffset
    //         //    << " targetAhead=" << target->ahead.toString() << "\n";
    //     }
    //
    //     // compute new angle of the vehicle from the x- and y-distances travelled within last time step
    //     // (should happen last because primaryLaneChanged() also triggers angle computation)
    //     // this part of the angle comes from the orientation of our current lane
    //     double laneAngle = vehicle->computeAngle();
    //     if (vehicle->getLaneChangeModel().isOpposite()) {
    //         // reverse lane angle
    //         laneAngle += M_PI;
    //     }
    // #ifdef DEBUG_MANEUVER
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " startChangeSublane"
    //                   << " oldLane=" << from->lane->getID()
    //                   << " newLane=" << to->lane->getID()
    //                   << " shadowLane=" << (shadowLane != nullptr ? shadowLane->getID() : "NULL")
    //                   << " targetLane=" << (targetLane != nullptr ? targetLane->getID() : "NULL")
    //                   << " maneuverDist=" << vehicle->getLaneChangeModel().getManeuverDist()
    //                   << " latDist=" << latDist
    //                   << " oldShadowLane=" << Named::getIDSecure(oldShadowLane)
    //                   << " newShadowLane=" << Named::getIDSecure(vehicle->getLaneChangeModel().getShadowLane())
    //                   << " laneA=" << RAD2DEG(laneAngle)
    //                   << " oldA=" << RAD2DEG(vehicle->getAngle())
    //                   << " oppposite=" << vehicle->getLaneChangeModel().isOpposite()
    //                   << " changedToNewLane=" << changedToNewLane
    //                   << "\n";
    //     }
    // #endif
    //     vehicle->setAngle(laneAngle, completedManeuver);
    //
    //     // check if a traci maneuver must continue
    //     // getOwnState is reset to 0 when changing lanes so we use the stored reason
    //     if ((reason & LCA_TRACI) != 0) {
    // #ifdef DEBUG_MANEUVER
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " continue TraCI-maneuver remainingLatDist=" << vehicle->getLaneChangeModel().getManeuverDist() << "\n";
    //         }
    // #endif
    //         vehicle->getInfluencer().setSublaneChange(vehicle->getLaneChangeModel().getManeuverDist());
    //     }
    //     from->lane->requireCollisionCheck();
    //     to->lane->requireCollisionCheck();
    //     return changedToNewLane;
    // }
    #[allow(unused_variables)]
    pub fn start_change_sublane(&mut self, vehicle: Ptr<MSVehicle>, from: &mut MSLaneChanger_ChangerIt, latDist: f64, maneuverDist: f64) -> bool {
        if vehicle.is_remote_controlled() {
            return false;
        }
        let fromIdx: usize = *from;
        let source = self.myChanger[fromIdx].lane;
        // Prevent continuation of LC beyond lane borders if change is not allowed
        let mut distToRightLaneBorder = vehicle.get_lane().get_width() * 0.5 + vehicle.get_lateral_position_on_lane() - vehicle.get_width() * 0.5;
        let mut distToLeftLaneBorder = vehicle.get_lane().get_width() * 0.5 - vehicle.get_lateral_position_on_lane() - vehicle.get_width() * 0.5;
        if vehicle.get_lane_change_model_2().is_opposite() {
            std::mem::swap(&mut distToRightLaneBorder, &mut distToLeftLaneBorder);
        }

        // determine direction of LC
        let mut direction: i32 = 0;
        if latDist > 0.0 && latDist > distToLeftLaneBorder {
            direction = 1;
        } else if latDist < 0.0 && -latDist > distToRightLaneBorder {
            direction = -1;
        }
        let changerDirection: i32 = if vehicle.get_lane_change_model_2().is_opposite() { -direction } else { direction };
        let mut to: usize = fromIdx;
        // C++ `to` is an iterator; in the opposite branch it points into the opposite
        // edge's changer, which is not indexable via self.myChanger. Carry the lane.
        let mut toLane: Ptr<MSLane> = source;
        if self.may_change(changerDirection) {
            to = ((fromIdx as i32) + changerDirection) as usize;
            toLane = self.myChanger[to].lane;
        } else if changerDirection == 1 && !source.get_opposite().is_null() {
            // change to the opposite direction lane
            let opp = source.get_opposite();
            let oppChanger = opp.get_edge().myLaneChanger.m().get_changer();
            to = oppChanger.len() - 1;
            toLane = oppChanger[to].lane;
        } else {
            // This may occur during maneuver continuation in non-actionsteps.
            self.abort_lc_maneuver(vehicle);
            return false;
        }

        // The following does:
        // 1) update vehicles lateral position according to latDist and target lane
        // 2) distinguish several cases
        //   a) vehicle moves completely within the same lane
        //   b) vehicle intersects another lane
        //      - vehicle must be moved to the lane where its midpoint is (either old or new)
        //      - shadow vehicle must be created/moved to the other lane if the vehicle intersects it
        // 3) updated dens of all lanes that hold the vehicle or its shadow
        let sign: f64 = if vehicle.get_lane_change_model_2().is_opposite() { -1.0 } else { 1.0 };
        let oldPosLat = vehicle.get_lateral_position_on_lane();
        oc97_trace(vehicle.get_id(), "lat", format!("latDist={} maneuverDist={} oldPosLat={} dir={}", latDist, maneuverDist, oldPosLat, direction));
        vehicle.m().myState.myPosLat += latDist * sign;
        for p in vehicle.m().myFurtherLanesPosLat.iter_mut() {
            *p += latDist * sign;
        }
        vehicle.m().myCachedPosition.set(Position::INVALID);
        vehicle.m().get_lane_change_model().set_speed_lat(dist2speed(latDist));
        oc97_trace(vehicle.get_id(), "latDone", format!("newPosLat={} speedLat={}", vehicle.get_lateral_position_on_lane(), dist2speed(latDist)));

        let completedPriorManeuver = vehicle.get_lane_change_model_2().get_previous_maneuver_dist().abs() < utils::geom::NUMERICAL_EPS;
        let completedManeuver = (maneuverDist - latDist).abs() < utils::geom::NUMERICAL_EPS;
        vehicle.m().get_lane_change_model().update_safe_lat_dist(latDist);

        // current maneuver is aborted when direction or reason changes
        let priorReason = vehicle.get_lane_change_model_2().get_prev_state() & LCA_CHANGE_REASONS & !LCA_SUBLANE;
        let reason = vehicle.get_lane_change_model_2().get_own_state() & LCA_CHANGE_REASONS & !LCA_SUBLANE;
        if !completedManeuver && !completedPriorManeuver && priorReason != 0
            && (vehicle.get_lane_change_model_2().get_previous_maneuver_dist() * latDist < 0.0 || priorReason != reason) {
            let priorDirection: i32 = if vehicle.get_lane_change_model_2().get_previous_maneuver_dist() > 0.0 { 1 } else { -1 };
            // original from cannot be reconstructed
            let mut f: usize = fromIdx;
            let mut t: usize = fromIdx;
            self.output_lc_ended(vehicle, &mut f, &mut t, priorDirection);
        }

        self.output_lc_started(vehicle, from, &mut to, direction, maneuverDist);
        vehicle.m().get_lane_change_model().set_maneuver_dist(maneuverDist - latDist);
        let changedToNewLane = self.check_change_to_new_lane(vehicle, direction, fromIdx, to, toLane);

        let oldShadowLane = vehicle.get_lane_change_model_2().get_shadow_lane();
        vehicle.m().get_lane_change_model().update_shadow_lane();
        let shadowLane = vehicle.get_lane_change_model_2().get_shadow_lane();
        if !shadowLane.is_null() && shadowLane != oldShadowLane
            && (shadowLane.get_edge() as *const MSEdge) == (source.get_edge() as *const MSEdge) {
            debug_assert!(oldShadowLane.is_null() || vehicle.get_lane_change_model_2().is_opposite() || to != fromIdx);
            let latOffset = vehicle.get_lane().get_right_side_on_edge() - shadowLane.get_right_side_on_edge();
            let shadowIdx = shadowLane.get_index() as usize;
            self.myChanger[shadowIdx].ahead.add_leader(vehicle, false, latOffset);
        }
        if completedManeuver {
            self.output_lc_ended(vehicle, from, &mut to, direction);
        }

        // Update maneuver reservations on target lanes
        let targetLane = vehicle.m().get_lane_change_model().update_target_lane();
        if !changedToNewLane && !targetLane.is_null() && vehicle.get_action_step_length() > delta_t()
            && (targetLane.get_edge() as *const MSEdge) == (source.get_edge() as *const MSEdge) {
            let dir: i32 = if vehicle.get_lane_change_model_2().get_maneuver_dist() > 0.0 { 1 } else { -1 };
            let target = ((fromIdx as i32) + dir) as usize;
            let actionStepDist = (dir as f64) * vehicle.get_vehicle_type().get_max_speed_lat() * vehicle.get_action_step_length_secs();
            let latOffset = vehicle.get_lat_offset(targetLane) + actionStepDist;
            self.myChanger[target].ahead.add_leader(vehicle, false, latOffset);
        }

        // compute new angle of the vehicle from the x- and y-distances travelled within last time step
        // (should happen last because primaryLaneChanged() also triggers angle computation)
        // this part of the angle comes from the orientation of our current lane
        let mut laneAngle = vehicle.compute_angle();
        if vehicle.get_lane_change_model_2().is_opposite() {
            // reverse lane angle
            laneAngle += std::f64::consts::PI;
        }
        vehicle.m().set_angle(laneAngle, completedManeuver);
        // check if a traci maneuver must continue
        // getOwnState is reset to 0 when changing lanes so we use the stored reason
        if (reason & LCA_TRACI) != 0 {
            let remaining = vehicle.get_lane_change_model_2().get_maneuver_dist();
            vehicle.m().get_influencer().set_sublane_change(remaining);
        }
        self.myChanger[fromIdx].lane.m().require_collision_check();
        toLane.m().require_collision_check();
        changedToNewLane
    }

    /// C++: `bool MSLaneChangerSublane::checkChangeToNewLane(MSVehicle *, const int, ChangerIt, ChangerIt)` [protected]
    /// check whether the given vehicle has entered the new lane 'to->lane' during a sublane LC-step
    // bool
    // MSLaneChangerSublane::checkChangeToNewLane(MSVehicle* vehicle, const int direction, ChangerIt from, ChangerIt to) {
    //     const int oppositeSign = vehicle->getLaneChangeModel().isOpposite() ? -1 : 1;
    //     const bool opposite = (&from->lane->getEdge() != &to->lane->getEdge());
    //     const bool changedToNewLane = (to->lane != from->lane
    //                                    && fabs(vehicle->getLateralPositionOnLane()) > 0.5 * vehicle->getLane()->getWidth()
    //                                    && (mayChange(direction * oppositeSign) || opposite));
    //     if (changedToNewLane) {
    //         vehicle->myState.myPosLat -= direction * 0.5 * (from->lane->getWidth() + to->lane->getWidth()) * oppositeSign;
    //         if (!opposite) {
    //             to->lane->myTmpVehicles.insert(to->lane->myTmpVehicles.begin(), vehicle);
    //             to->dens += vehicle->getVehicleType().getLengthWithGap();
    //         }
    //         if (MSAbstractLaneChangeModel::haveLCOutput()) {
    //             if (!vehicle->isActive()) {
    //                 // update leaders beyond the current edge for all lanes
    //                 // @note to->aheadNext and from->aheadNext are only needed for output in non-action steps.
    //                 to->aheadNext = getLeaders(to, vehicle);
    //                 from->aheadNext = getLeaders(from, vehicle);
    //             }
    //             vehicle->getLaneChangeModel().setLeaderGaps(to->aheadNext);
    //             vehicle->getLaneChangeModel().setFollowerGaps(to->lane->getFollowersOnConsecutive(vehicle, vehicle->getBackPositionOnLane(), true));
    //             vehicle->getLaneChangeModel().setOrigLeaderGaps(from->aheadNext);
    //         }
    //         vehicle->getLaneChangeModel().startLaneChangeManeuver(from->lane, to->lane, direction);
    //         if (!opposite) {
    //             to->ahead.addLeader(vehicle, false, 0);
    //         }
    //     } else {
    //         from->ahead.addLeader(vehicle, false, 0);
    //     }
    //     return changedToNewLane;
    // }
    #[allow(unused_variables)]
    pub fn check_change_to_new_lane(&mut self, vehicle: Ptr<MSVehicle>, direction: i32, from: MSLaneChanger_ChangerIt, to: MSLaneChanger_ChangerIt, toLane: Ptr<MSLane>) -> bool {
        let oppositeSign: i32 = if vehicle.get_lane_change_model_2().is_opposite() { -1 } else { 1 };
        let opposite: bool = (self.myChanger[from].lane.get_edge() as *const MSEdge) != (toLane.get_edge() as *const MSEdge);
        let changedToNewLane: bool = toLane != self.myChanger[from].lane
            && vehicle.get_lateral_position_on_lane().abs() > 0.5 * vehicle.get_lane().get_width()
            && (self.may_change(direction * oppositeSign) || opposite);
        if changedToNewLane {
            vehicle.m().myState.myPosLat -= (direction as f64) * 0.5 * (self.myChanger[from].lane.get_width() + toLane.get_width()) * (oppositeSign as f64);
            if !opposite {
                toLane.m().myTmpVehicles.insert(0, vehicle);
                if to < self.myChanger.len() {
                    self.myChanger[to].dens += vehicle.get_vehicle_type().get_length_with_gap();
                }
            }
            if MSAbstractLaneChangeModel::have_lc_output() && to < self.myChanger.len() {
                if !vehicle.is_active() {
                    // update leaders beyond the current edge for all lanes
                    // @note to->aheadNext and from->aheadNext are only needed for output in non-action steps.
                    let toLeaders = self.get_leaders(&to, vehicle);
                    self.myChanger[to].aheadNext = toLeaders;
                    let fromLeaders = self.get_leaders(&from, vehicle);
                    self.myChanger[from].aheadNext = fromLeaders;
                }
                let backPos = vehicle.get_back_position_on_lane_2();
                let followers = toLane.get_followers_on_consecutive(vehicle, backPos, true, -1.0, MSLane_MinorLinkMode::FOLLOW_ALWAYS, false);
                let lcm = vehicle.m().get_lane_change_model();
                lcm.set_leader_gaps_2(&self.myChanger[to].aheadNext);
                lcm.set_follower_gaps_2(&followers);
                lcm.set_orig_leader_gaps_2(&self.myChanger[from].aheadNext);
            }
            let _ = vehicle.m().get_lane_change_model().start_lane_change_maneuver(self.myChanger[from].lane, toLane, direction);
            if !opposite && to < self.myChanger.len() {
                self.myChanger[to].ahead.add_leader(vehicle, false, 0.0);
            }
        } else {
            self.myChanger[from].ahead.add_leader(vehicle, false, 0.0);
        }
        changedToNewLane
    }

    /// C++: `MSLeaderDistanceInfo MSLaneChangerSublane::getLeaders(const ChangerIt &, const MSVehicle *) const` [const, protected]
    /// get leaders for ego on the given lane
    // MSLeaderDistanceInfo
    // MSLaneChangerSublane::getLeaders(const ChangerIt& target, const MSVehicle* vehicle) const {
    //     // get the leading vehicle on the lane to change to
    // #ifdef DEBUG_SURROUNDING
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " getLeaders lane=" << target->lane->getID() << " ego=" << vehicle->getID() << " ahead=" << target->ahead.toString() << "\n";
    //     }
    // #endif
    //     MSLeaderDistanceInfo result(target->lane->getWidth(), nullptr, 0);
    //     int sublaneShift = 0;
    //     if (target->lane == vehicle->getLane()) {
    //         if (vehicle->getLeftSideOnLane() < -MSGlobals::gLateralResolution) {
    //             sublaneShift = int(-vehicle->getLeftSideOnLane() / MSGlobals::gLateralResolution);
    //         } else if (vehicle->getRightSideOnLane() > target->lane->getWidth() + MSGlobals::gLateralResolution) {
    //             sublaneShift = -int((vehicle->getRightSideOnLane() - target->lane->getWidth()) / MSGlobals::gLateralResolution);
    //         }
    //         result.setSublaneOffset(sublaneShift);
    //     }
    //     for (int i = 0; i < target->ahead.numSublanes(); ++i) {
    //         const MSVehicle* veh = target->ahead[i];
    //         if (veh != nullptr) {
    //             const double gap = veh->getBackPositionOnLane(target->lane) - vehicle->getPositionOnLane() - vehicle->getVehicleType().getMinGap();
    // #ifdef DEBUG_SURROUNDING
    //             if (DEBUG_COND) {
    //                 std::cout << " ahead lead=" << veh->getID() << " leadBack=" << veh->getBackPositionOnLane() << " gap=" << gap << " sublaneShift=" << sublaneShift << "\n";
    //             }
    // #endif
    //             if (i + sublaneShift >= 0 && i + sublaneShift < result.numSublanes()) {
    //                 result.addLeader(veh, gap, 0, i + sublaneShift);
    //             }
    //         }
    //     }
    //     if (sublaneShift != 0) {
    //         for (MSVehicle* cand : target->outsideBounds) {
    //             const double gap = cand->getBackPositionOnLane() - vehicle->getPositionOnLane() - vehicle->getVehicleType().getMinGap();
    //             result.addLeader(cand, gap);
    //         }
    //     }
    // #ifdef DEBUG_SURROUNDING
    //     if (DEBUG_COND) {
    //         std::cout << "   outsideBounds=" << toString(target->outsideBounds) << " result=" << result.toString() << "\n";
    //     }
    // #endif
    //     // @note use the exact same vehiclePos as in getLastVehicleInformation() to avoid missing vehicles
    //     target->lane->addLeaders(vehicle, vehicle->getPositionOnLane(vehicle->getLane()), result);
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_leaders(&self, target: &MSLaneChanger_ChangerIt, ego: Ptr<MSVehicle>) -> MSLeaderDistanceInfo {
        // get the leading vehicle on the lane to change to
        let t = *target;
        let tlane = self.myChanger[t].lane;
        let mut result = MSLeaderDistanceInfo::new(tlane.get_width(), Ptr::null(), 0.0);
        let mut sublaneShift: i32 = 0;
        if tlane == ego.get_lane() {
            if ego.get_left_side_on_lane() < -MSGlobals::lateral_resolution() {
                sublaneShift = (-ego.get_left_side_on_lane() / MSGlobals::lateral_resolution()) as i32;
            } else if ego.get_right_side_on_lane() > tlane.get_width() + MSGlobals::lateral_resolution() {
                sublaneShift = -(((ego.get_right_side_on_lane() - tlane.get_width()) / MSGlobals::lateral_resolution()) as i32);
            }
            result.set_sublane_offset(sublaneShift);
        }
        for i in 0..self.myChanger[t].ahead.num_sublanes() {
            let veh = self.myChanger[t].ahead.index_op(i);
            if !veh.is_null() {
                let gap = veh.get_back_position_on_lane(tlane) - ego.get_position_on_lane() - ego.get_vehicle_type().get_min_gap();
                if i + sublaneShift >= 0 && i + sublaneShift < result.num_sublanes() {
                    result.add_leader(veh, gap, 0.0, i + sublaneShift);
                }
            }
        }
        if sublaneShift != 0 {
            for cand in self.myChanger[t].outsideBounds.clone() {
                let gap = cand.get_back_position_on_lane_2() - ego.get_position_on_lane() - ego.get_vehicle_type().get_min_gap();
                result.add_leader(cand, gap, 0.0, -1);
            }
        }
        // @note use the exact same vehiclePos as in getLastVehicleInformation() to avoid missing vehicles
        tlane.m().add_leaders(ego, ego.get_position_on_lane_2(ego.get_lane()), &mut result, false);
        result
    }

    /// C++: `void MSLaneChangerSublane::abortLCManeuver(MSVehicle *)` [protected]
    /// immediately stop lane-changing and register vehicle as unchanged
    // void
    // MSLaneChangerSublane::abortLCManeuver(MSVehicle* vehicle) {
    // #ifdef DEBUG_MANEUVER
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh '" << vehicle->getID() << "' aborts LC-continuation."
    //                   << std::endl;
    //     }
    // #endif
    //     const int priorReason = vehicle->getLaneChangeModel().getPrevState() & LCA_CHANGE_REASONS & ~LCA_SUBLANE;
    //     const int reason = vehicle->getLaneChangeModel().getOwnState() & LCA_CHANGE_REASONS & ~LCA_SUBLANE;
    //     if (priorReason != 0 && priorReason != reason && vehicle->getLaneChangeModel().getPreviousManeuverDist() != 0) {
    //         // original from cannot be reconstructed
    //         const int priorDirection = vehicle->getLaneChangeModel().getPreviousManeuverDist() > 0 ? 1 : -1;
    // #ifdef DEBUG_MANEUVER
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " abortLCManeuver priorReason=" << toString((LaneChangeAction)priorReason)
    //                       << " reason=" << toString((LaneChangeAction)reason) << " veh=" << vehicle->getID() << "\n";
    //         }
    // #endif
    //         outputLCEnded(vehicle, myCandi, myCandi, priorDirection);
    //     }
    //     const double updatedSpeedLat = vehicle->getLaneChangeModel().getSpeedLat() != 0;
    //     vehicle->getLaneChangeModel().setSpeedLat(0);
    //     vehicle->getLaneChangeModel().setManeuverDist(0.);
    //     vehicle->getLaneChangeModel().updateTargetLane();
    //     if (updatedSpeedLat) {
    //         // update angle after having reset lateral speed
    //         vehicle->setAngle(vehicle->computeAngle());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn abort_lc_maneuver(&mut self, vehicle: Ptr<MSVehicle>) {
        let priorReason = vehicle.get_lane_change_model_2().get_prev_state() & LCA_CHANGE_REASONS & !LCA_SUBLANE;
        let reason = vehicle.get_lane_change_model_2().get_own_state() & LCA_CHANGE_REASONS & !LCA_SUBLANE;
        if priorReason != 0 && priorReason != reason && vehicle.get_lane_change_model_2().get_previous_maneuver_dist() != 0.0 {
            // original from cannot be reconstructed
            let priorDirection: i32 = if vehicle.get_lane_change_model_2().get_previous_maneuver_dist() > 0.0 { 1 } else { -1 };
            let mut from: usize = self.myCandi;
            let mut to: usize = self.myCandi;
            self.output_lc_ended(vehicle, &mut from, &mut to, priorDirection);
        }
        let updatedSpeedLat: bool = vehicle.get_lane_change_model_2().get_speed_lat() != 0.0;
        vehicle.m().get_lane_change_model().set_speed_lat(0.0);
        vehicle.m().get_lane_change_model().set_maneuver_dist(0.0);
        vehicle.m().get_lane_change_model().update_target_lane();
        if updatedSpeedLat {
            // update angle after having reset lateral speed
            let angle = vehicle.compute_angle();
            vehicle.m().set_angle(angle, false);
        }
    }

    /// C++: `StateAndDist MSLaneChangerSublane::checkChangeHelper(MSVehicle *, int, LaneChangeAction)` [protected]
    /// helper function that calls checkChangeSublane and sets blocker information
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn check_change_helper(&mut self, vehicle: Ptr<MSVehicle>, laneOffset: i32, alternatives: LaneChangeAction) -> MSAbstractLaneChangeModel_StateAndDist {
        let mut result = MSAbstractLaneChangeModel_StateAndDist { state: 0, latDist: 0.0, maneuverDist: 0.0, dir: 0 };
        if self.may_change(laneOffset) {
            if laneOffset != 0 && vehicle.get_lane_change_model_2().is_opposite() {
                return result;
            }
            let precomputed: Vec<MSVehicle_LaneQ>;
            let preb: &Vec<MSVehicle_LaneQ> = if vehicle.get_lane_change_model_2().is_opposite() {
                precomputed = MSLaneChanger::get_best_lanes_opposite(vehicle, Ptr::null(), 1000.0);
                &precomputed
            } else {
                vehicle.get_best_lanes()
            };
            result.state = self.check_change_sublane(laneOffset, alternatives, preb, &mut result.latDist, &mut result.maneuverDist);
            result.dir = laneOffset;
            if (result.state & LCA_WANTS_LANECHANGE) != 0 && (result.state & LCA_URGENT) != 0 && (result.state & LCA_BLOCKED) != 0 {
                let t = ((self.myCandi as i32) + laneOffset) as usize;
                self.myChanger[t].lastBlocked = vehicle;
                if self.myChanger[t].firstBlocked.is_null() {
                    self.myChanger[t].firstBlocked = vehicle;
                }
            }
        }
        result
    }

    /// C++: `std::pair<MSVehicle *, double> MSLaneChangerSublane::findClosestLeader(const MSLeaderDistanceInfo &, const MSVehicle *)` [static, protected]
    /// find the closest leader that prevents ego vehicle from passing on the current lane
    // std::pair<MSVehicle*, double>
    // MSLaneChangerSublane::findClosestLeader(const MSLeaderDistanceInfo& leaders, const MSVehicle* vehicle) {
    //     const double egoWidth = vehicle->getVehicleType().getWidth() + vehicle->getVehicleType().getMinGapLat();
    //     std::pair<MSVehicle*, double> leader(nullptr, std::numeric_limits<double>::max());
    //     for (int i = 0; i < leaders.numSublanes(); ++i) {
    //         CLeaderDist cand = leaders[i];
    //         if (cand.first != nullptr) {
    //             double rightSide = cand.first->getRightSideOnLane();
    //             if (cand.first->getLane() != vehicle->getLane()) {
    //                 // the candidate may be a parial (sideways) occupier so getRightSideOnLane() cannot be used
    //                 rightSide += (cand.first->getCenterOnEdge(cand.first->getLane())
    //                               - vehicle->getCenterOnEdge(vehicle->getLane()));
    //             }
    // #ifdef DEBUG_CHANGE_OPPOSITE
    //             if (vehicle->isSelected()) {
    //                 std::cout << SIMTIME << " cand=" << cand.first->getID() << " rightSide=" << rightSide << "\n";
    //             }
    // #endif
    //             if (cand.second < leader.second
    //                     && rightSide < egoWidth
    //                     && vehicle->getLane()->getWidth() - rightSide - cand.first->getVehicleType().getWidth() < egoWidth) {
    //                 leader.first = const_cast<MSVehicle*>(cand.first);
    //                 leader.second = cand.second;
    //             }
    //         }
    //     }
    //     return leader;
    // }
    #[allow(unused_variables)]
    pub fn find_closest_leader(leaders: &MSLeaderDistanceInfo, vehicle: Ptr<MSVehicle>) -> (Ptr<MSVehicle>, f64) {
        let egoWidth = vehicle.get_vehicle_type().get_width() + vehicle.get_vehicle_type().get_min_gap_lat();
        let mut leader: (Ptr<MSVehicle>, f64) = (Ptr::null(), f64::MAX);
        for i in 0..leaders.num_sublanes() {
            let cand = leaders.index_op(i);
            if !cand.0.is_null() {
                let mut rightSide = cand.0.get_right_side_on_lane();
                if cand.0.get_lane() != vehicle.get_lane() {
                    // the candidate may be a parial (sideways) occupier so getRightSideOnLane() cannot be used
                    rightSide += cand.0.get_center_on_edge(cand.0.get_lane()) - vehicle.get_center_on_edge(vehicle.get_lane());
                }
                if cand.1 < leader.1
                    && rightSide < egoWidth
                    && vehicle.get_lane().get_width() - rightSide - cand.0.get_vehicle_type().get_width() < egoWidth {
                    leader = (cand.0, cand.1);
                }
            }
        }
        leader
    }

    /// C++: `void MSLaneChangerSublane::outputLCStarted(MSVehicle *, ChangerIt &, ChangerIt &, int, double)` [protected]
    /// optional output for start of lane-change maneuvre
    // void
    // MSLaneChangerSublane::outputLCStarted(MSVehicle* vehicle, ChangerIt& from, ChangerIt& to, int direction, double maneuverDist) {
    //     if (MSAbstractLaneChangeModel::haveLCOutput() && MSAbstractLaneChangeModel::enabledOutputLCStarted()
    //             // non-sublane change started
    //             && ((vehicle->getLaneChangeModel().getOwnState() & (LCA_CHANGE_REASONS & ~LCA_SUBLANE)) != 0)
    //             && ((vehicle->getLaneChangeModel().getOwnState() & LCA_STAY) == 0)
    //             // no changing for the same reason in previous step (either not wanted or blocked)
    //             && ((vehicle->getLaneChangeModel().getPrevState() & LCA_CHANGE_REASONS & ~LCA_SUBLANE) !=
    //                 (vehicle->getLaneChangeModel().getOwnState() & LCA_CHANGE_REASONS & ~LCA_SUBLANE)
    //                 || ((vehicle->getLaneChangeModel().getPrevState() & LCA_STAY) != 0)
    //                 || ((vehicle->getLaneChangeModel().getPrevState() & LCA_BLOCKED) != 0))
    //        ) {
    // #ifdef DEBUG_STATE
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " veh=" << vehicle->getID() << " laneChangeStarted state=" << toString((LaneChangeAction)vehicle->getLaneChangeModel().getOwnState())
    //                       << " prevState=" << toString((LaneChangeAction)vehicle->getLaneChangeModel().getPrevState())
    //                       << " filter=" << toString((LaneChangeAction)(LCA_CHANGE_REASONS & ~LCA_SUBLANE))
    //                       << " filtered=" << toString((LaneChangeAction)(vehicle->getLaneChangeModel().getOwnState() & (LCA_CHANGE_REASONS & ~LCA_SUBLANE)))
    //                       << "\n";
    //         }
    // #endif
    //         vehicle->getLaneChangeModel().setLeaderGaps(to->aheadNext);
    //         vehicle->getLaneChangeModel().setFollowerGaps(to->lane->getFollowersOnConsecutive(vehicle, vehicle->getBackPositionOnLane(), true));
    //         vehicle->getLaneChangeModel().setOrigLeaderGaps(from->aheadNext);
    //         vehicle->getLaneChangeModel().laneChangeOutput("changeStarted", from->lane, to->lane, direction, maneuverDist);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn output_lc_started(&mut self, vehicle: Ptr<MSVehicle>, from: &mut MSLaneChanger_ChangerIt, to: &mut MSLaneChanger_ChangerIt, direction: i32, maneuverDist: f64) {
        if MSAbstractLaneChangeModel::have_lc_output() && MSAbstractLaneChangeModel::enabled_output_lc_started()
            // non-sublane change started
            && ((vehicle.get_lane_change_model_2().get_own_state() & (LCA_CHANGE_REASONS & !LCA_SUBLANE)) != 0)
            && ((vehicle.get_lane_change_model_2().get_own_state() & LCA_STAY) == 0)
            // no changing for the same reason in previous step (either not wanted or blocked)
            && ((vehicle.get_lane_change_model_2().get_prev_state() & LCA_CHANGE_REASONS & !LCA_SUBLANE)
                    != (vehicle.get_lane_change_model_2().get_own_state() & LCA_CHANGE_REASONS & !LCA_SUBLANE)
                || ((vehicle.get_lane_change_model_2().get_prev_state() & LCA_STAY) != 0)
                || ((vehicle.get_lane_change_model_2().get_prev_state() & LCA_BLOCKED) != 0))
        {
            let backPos = vehicle.get_back_position_on_lane_2();
            let followers = self.myChanger[*to].lane.get_followers_on_consecutive(vehicle, backPos, true, -1.0, MSLane_MinorLinkMode::FOLLOW_ALWAYS, false);
            let lcm = vehicle.m().get_lane_change_model();
            lcm.set_leader_gaps_2(&self.myChanger[*to].aheadNext);
            lcm.set_follower_gaps_2(&followers);
            lcm.set_orig_leader_gaps_2(&self.myChanger[*from].aheadNext);
            lcm.lane_change_output("changeStarted", self.myChanger[*from].lane, self.myChanger[*to].lane, direction, maneuverDist);
        }
    }

    /// C++: `void MSLaneChangerSublane::outputLCEnded(MSVehicle *, ChangerIt &, ChangerIt &, int)` [protected]
    /// optional output for end of lane-change maneuvre
    // void
    // MSLaneChangerSublane::outputLCEnded(MSVehicle* vehicle, ChangerIt& from, ChangerIt& to, int direction) {
    //     if (MSAbstractLaneChangeModel::haveLCOutput() && MSAbstractLaneChangeModel::enabledOutputLCEnded()
    //             // non-sublane change ended
    //             && ((vehicle->getLaneChangeModel().getOwnState() & (LCA_CHANGE_REASONS & ~LCA_SUBLANE)) != 0)) {
    //         vehicle->getLaneChangeModel().setLeaderGaps(to->aheadNext);
    //         vehicle->getLaneChangeModel().setFollowerGaps(to->lane->getFollowersOnConsecutive(vehicle, vehicle->getBackPositionOnLane(), true));
    //         vehicle->getLaneChangeModel().setOrigLeaderGaps(from->aheadNext);
    //         vehicle->getLaneChangeModel().laneChangeOutput("changeEnded", from->lane, to->lane, direction);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn output_lc_ended(&mut self, vehicle: Ptr<MSVehicle>, from: &mut MSLaneChanger_ChangerIt, to: &mut MSLaneChanger_ChangerIt, direction: i32) {
        if MSAbstractLaneChangeModel::have_lc_output() && MSAbstractLaneChangeModel::enabled_output_lc_ended()
            // non-sublane change ended
            && ((vehicle.get_lane_change_model_2().get_own_state() & (LCA_CHANGE_REASONS & !LCA_SUBLANE)) != 0) {
            let backPos = vehicle.get_back_position_on_lane_2();
            let followers = self.myChanger[*to].lane.get_followers_on_consecutive(vehicle, backPos, true, -1.0, MSLane_MinorLinkMode::FOLLOW_ALWAYS, false);
            let lcm = vehicle.m().get_lane_change_model();
            lcm.set_leader_gaps_2(&self.myChanger[*to].aheadNext);
            lcm.set_follower_gaps_2(&followers);
            lcm.set_orig_leader_gaps_2(&self.myChanger[*from].aheadNext);
            lcm.lane_change_output("changeEnded", self.myChanger[*from].lane, self.myChanger[*to].lane, direction, 0.0);
        }
    }

    /// C++: `void MSLaneChangerSublane::addOutsideLeaders(const MSVehicle *, MSLeaderDistanceInfo &) const` [const, protected]
    // void
    // MSLaneChangerSublane::addOutsideLeaders(const MSVehicle* vehicle, MSLeaderDistanceInfo& leaders) const {
    //     if (vehicle->getLaneChangeModel().getShadowLane() == nullptr) {
    //         const MSLane* lane = vehicle->getLane();
    //         const double rightOL = vehicle->getRightSideOnLane(lane);
    //         const double leftOL = vehicle->getLeftSideOnLane(lane);
    //         const bool outsideLeft = rightOL > lane->getWidth();
    // #ifdef DEBUG_SURROUNDING
    //         if (DEBUG_COND) {
    //             std::cout << SIMTIME << " addOutsideLeaders veh=" << vehicle->getID() << " lane=" << lane->getID() << " rightOL=" << rightOL << " leftOL=" << leftOL << "\n";
    //         }
    // #endif
    //         if (leftOL < 0 || outsideLeft) {
    //             int sublaneOffset = 0;
    //             if (outsideLeft) {
    //                 sublaneOffset = MIN2(0, -(int)ceil((leftOL - lane->getWidth()) / MSGlobals::gLateralResolution) + 1);
    //             } else {
    //                 sublaneOffset = MAX2(0, (int)ceil(-rightOL / MSGlobals::gLateralResolution) - 1);
    //             }
    //             if (sublaneOffset != 0) {
    //                 leaders.setSublaneOffset(sublaneOffset);
    // #ifdef DEBUG_SURROUNDING
    //                 if (DEBUG_COND) {
    //                     std::cout << "   sublaneOffset=" << sublaneOffset << " outsideLeft=" << outsideLeft << "\n";
    //                 }
    // #endif
    //                 for (const MSVehicle* cand : lane->myTmpVehicles) {
    // #ifdef DEBUG_SURROUNDING
    //                     if (DEBUG_COND) {
    //                         std::cout << " cand=" << cand->getID() << " cLOE=" << cand->getLeftSideOnEdge() << " cROE=" << cand->getRightSideOnEdge() << "\n";
    //                     }
    // #endif
    //                     if (cand->getPositionOnLane() > vehicle->getPositionOnLane()
    //                             && ((!outsideLeft && cand->getLeftSideOnEdge() < 0)
    //                                 || (outsideLeft && cand->getLeftSideOnEdge() > lane->getEdge().getWidth()))) {
    //                         const double gap = cand->getPositionOnLane() - vehicle->getPositionOnLane() - cand->getLength() - vehicle->getVehicleType().getMinGap();
    //                         leaders.addLeader(cand, gap);
    // #ifdef DEBUG_SURROUNDING
    //                         if (DEBUG_COND) {
    //                             std::cout << "   outsideLeader=" << cand->getID() << " ahead=" << leaders.toString() << "\n";
    //                         }
    // #endif
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_outside_leaders(&self, vehicle: Ptr<MSVehicle>, leaders: &mut MSLeaderDistanceInfo) {
        if vehicle.get_lane_change_model_2().get_shadow_lane().is_null() {
            let lane = vehicle.get_lane();
            let rightOL = vehicle.get_right_side_on_lane_2(lane);
            let leftOL = vehicle.get_left_side_on_lane_2(lane);
            let outsideLeft = rightOL > lane.get_width();
            if leftOL < 0.0 || outsideLeft {
                let mut sublaneOffset: i32 = 0;
                if outsideLeft {
                    sublaneOffset = std::cmp::min(0, -(((leftOL - lane.get_width()) / MSGlobals::lateral_resolution()).ceil() as i32) + 1);
                } else {
                    sublaneOffset = std::cmp::max(0, (((-rightOL) / MSGlobals::lateral_resolution()).ceil() as i32) - 1);
                }
                if sublaneOffset != 0 {
                    leaders.set_sublane_offset(sublaneOffset);
                    for cand in lane.myTmpVehicles.clone() {
                        if cand.get_position_on_lane() > vehicle.get_position_on_lane()
                            && ((!outsideLeft && cand.get_left_side_on_edge(cand.get_lane()) < 0.0)
                                || (outsideLeft && cand.get_left_side_on_edge(cand.get_lane()) > lane.get_edge().get_width())) {
                            let gap = cand.get_position_on_lane() - vehicle.get_position_on_lane() - cand.get_length() - vehicle.get_vehicle_type().get_min_gap();
                            leaders.add_leader(cand, gap, 0.0, -1);
                        }
                    }
                }
            }
        }
    }

    /// C++: `void MSLaneChangerSublane::MSLaneChangerSublane()` [private]
    /// Default constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2() -> Self {
        let mut base = MSLaneChanger::new_2();
        base.vt = &MS_LANE_CHANGER_SUBLANE_VT;
        MSLaneChangerSublane {
            base,
            myCheckedChangeOpposite: false,
        }
    }

    /// C++: `void MSLaneChangerSublane::MSLaneChangerSublane(const MSLaneChangerSublane &)` [private]
    /// Copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(arg0: &MSLaneChangerSublane) -> Self {
        let mut base = MSLaneChanger::new_3(&arg0.base);
        base.vt = &MS_LANE_CHANGER_SUBLANE_VT;
        MSLaneChangerSublane {
            base,
            myCheckedChangeOpposite: arg0.myCheckedChangeOpposite,
        }
    }

    /// C++: `MSLaneChangerSublane & MSLaneChangerSublane::operator=(const MSLaneChangerSublane &)` [private]
    /// Assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSLaneChangerSublane) -> &mut MSLaneChangerSublane {
        self.base.assign_op(&arg0.base);
        self.myCheckedChangeOpposite = arg0.myCheckedChangeOpposite;
        self
    }

}


/// Field-wise copy of MSAbstractLaneChangeModel::StateAndDist (C++ passes it by value; the Rust struct is not Clone)

fn sd_copy(s: &MSAbstractLaneChangeModel_StateAndDist) -> MSAbstractLaneChangeModel_StateAndDist {
    MSAbstractLaneChangeModel_StateAndDist::new(s.state, s.latDist, s.maneuverDist, s.dir)
}
