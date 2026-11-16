//! Port of microsim/lcmodels/MSLCM_DK2008.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::pair<double, int> MSLCM_DK2008::Info`
pub type MSLCM_DK2008_Info = (f64, i32);

// C++ #define constants of MSLCM_DK2008.cpp
const LOOK_FORWARD_SPEED_DIVIDER: f64 = 14.;
const LOOK_FORWARD_FAR: f64 = 15.;
const LOOK_FORWARD_NEAR: f64 = 5.;
const JAM_FACTOR2: f64 = 1.;
/// C++ SUMO_const_haltingSpeed (utils/common/StdDefs.h)
const SUMO_const_haltingSpeed: f64 = 0.1;

// MIN2/MAX2 from StdDefs.h (utils::geom's copies are pub(crate))
#[inline]
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
#[inline]
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// C++ implicit conversion `MSVehicle*` -> `SUMOTrafficObject*` (MSVehicle > MSBaseVehicle > SUMOVehicle > SUMOTrafficObject)
fn traffic_object_of(v: Ptr<MSVehicle>) -> Ptr<SUMOTrafficObject> {
    Ptr::from_mut(&mut v.m().base.base.base)
}

/// C++ `MSVehicle::LaneQ()` default-constructed value
fn lane_q_default() -> MSVehicle_LaneQ {
    MSVehicle_LaneQ {
        lane: Ptr::null(),
        length: 0.,
        currentLength: 0.,
        occupation: 0.,
        nextOccupation: 0.,
        bestLaneOffset: 0,
        allowsContinuation: false,
        bestContinuations: Vec::new(),
    }
}

/// C++ copy of a `MSVehicle::LaneQ` (the struct is not Clone in the port)
fn lane_q_copy(q: &MSVehicle_LaneQ) -> MSVehicle_LaneQ {
    MSVehicle_LaneQ {
        lane: q.lane,
        length: q.length,
        currentLength: q.currentLength,
        occupation: q.occupation,
        nextOccupation: q.nextOccupation,
        bestLaneOffset: q.bestLaneOffset,
        allowsContinuation: q.allowsContinuation,
        bestContinuations: q.bestContinuations.clone(),
    }
}

/// C++ `new Info(speed, state)`: the boxed message handed to the messager (freed by `inform`)
fn new_info(info: (f64, i32)) -> Ptr<()> {
    Ptr::from_raw(Box::into_raw(Box::new(info)) as *mut ())
}

/// A lane change model developed by D. Krajzewicz between 2004 and 2010
/// C++ class `MSLCM_DK2008` (microsim/lcmodels/MSLCM_DK2008.h)
#[repr(C)]
pub struct MSLCM_DK2008 {
    /// C++ base class `MSAbstractLaneChangeModel`
    pub base: MSAbstractLaneChangeModel,
    pub myChangeProbability: f64,
    pub myLeadingBlockerLength: f64,
    pub myLeftSpace: f64,
    pub myVSafes: Vec<f64>,
    pub myDontBrake: bool,
}

impl std::ops::Deref for MSLCM_DK2008 {
    type Target = MSAbstractLaneChangeModel;
    fn deref(&self) -> &MSAbstractLaneChangeModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSLCM_DK2008 {
    fn deref_mut(&mut self) -> &mut MSAbstractLaneChangeModel {
        &mut self.base
    }
}

fn dk2008_inform(lcm: &mut MSAbstractLaneChangeModel, info: Ptr<()>, sender: Ptr<MSVehicle>) -> Ptr<()> {
    Ptr::from_mut(lcm).cast::<MSLCM_DK2008>().m().inform(info, sender)
}

fn dk2008_patch_speed(lcm: &mut MSAbstractLaneChangeModel, min: f64, wanted: f64, max: f64, cfModel: &MSCFModel) -> f64 {
    Ptr::from_mut(lcm).cast::<MSLCM_DK2008>().m().patch_speed(min, wanted, max, cfModel)
}

fn dk2008_changed(lcm: &mut MSAbstractLaneChangeModel) {
    Ptr::from_mut(lcm).cast::<MSLCM_DK2008>().m().changed()
}

fn dk2008_wants_change(lcm: &mut MSAbstractLaneChangeModel, laneOffset: i32, msgPass: &mut MSAbstractLaneChangeModel_MSLCMessager, blocked: i32, leader: &(Ptr<MSVehicle>, f64), follower: &(Ptr<MSVehicle>, f64), neighLead: &(Ptr<MSVehicle>, f64), neighFollow: &(Ptr<MSVehicle>, f64), neighLane: &MSLane, preb: &Vec<MSVehicle_LaneQ>, lastBlocked: Ptr<Ptr<MSVehicle>>, firstBlocked: Ptr<Ptr<MSVehicle>>) -> i32 {
    Ptr::from_mut(lcm).cast::<MSLCM_DK2008>().m().wants_change(laneOffset, msgPass, blocked, leader, follower, neighLead, neighFollow, neighLane, preb, lastBlocked, firstBlocked)
}

fn dk2008_prepare_step(lcm: &mut MSAbstractLaneChangeModel) {
    Ptr::from_mut(lcm).cast::<MSLCM_DK2008>().m().prepare_step()
}

/// Virtual dispatch table of MSLCM_DK2008; overrides only the entries listed in MSLCM_DK2008.h
pub static MSLCM_DK2008_VT: MSAbstractLaneChangeModelVt = MSAbstractLaneChangeModelVt {
    inform: dk2008_inform,
    patch_speed: dk2008_patch_speed,
    changed: dk2008_changed,
    wants_change: dk2008_wants_change,
    wants_change_sublane: MSAbstractLaneChangeModel::wants_change_sublane_base,
    update_expected_sublane_speeds: MSAbstractLaneChangeModel::update_expected_sublane_speeds_base,
    decide_direction: MSAbstractLaneChangeModel::decide_direction_base,
    set_own_state: MSAbstractLaneChangeModel::set_own_state_base,
    prepare_step: dk2008_prepare_step,
    reset_speed_lat: MSAbstractLaneChangeModel::reset_speed_lat_base,
    update_safe_lat_dist: MSAbstractLaneChangeModel::update_safe_lat_dist_base,
    get_safety_factor: MSAbstractLaneChangeModel::get_safety_factor_base,
    compute_speed_lat: MSAbstractLaneChangeModel::compute_speed_lat_base,
    save_blocker_length: MSAbstractLaneChangeModel::save_blocker_length_base,
    get_extra_reservation: MSAbstractLaneChangeModel::get_extra_reservation_base,
    set_parameter: MSAbstractLaneChangeModel::set_parameter_base,
    avoid_overtake_right: MSAbstractLaneChangeModel::avoid_overtake_right_base,
    save_state: MSAbstractLaneChangeModel::save_state_base,
    load_state: MSAbstractLaneChangeModel::load_state_base,
    reset_state: MSAbstractLaneChangeModel::reset_state_base,
    get_opposite_safety_factor: MSAbstractLaneChangeModel::get_opposite_safety_factor_base,
    debug_vehicle: MSAbstractLaneChangeModel::debug_vehicle_base,
    get_parameter: MSAbstractLaneChangeModel::get_parameter_base,
    get_desired_alignment: MSAbstractLaneChangeModel::get_desired_alignment_base,
    check_change_before_committing: MSAbstractLaneChangeModel::check_change_before_committing_base,
};

impl MSLCM_DK2008 {
    /// C++: `void MSLCM_DK2008::MSLCM_DK2008(MSVehicle &)`
    // MSLCM_DK2008::MSLCM_DK2008(MSVehicle& v)
    //     : MSAbstractLaneChangeModel(v, LaneChangeModel::DK2008),
    //       myChangeProbability(0),
    //       myLeadingBlockerLength(0), myLeftSpace(0) {}
    #[allow(unused_variables)]
    pub fn new(v: &mut MSVehicle) -> Self {
        // C++ base-class initializer runs first
        let mut base = MSAbstractLaneChangeModel::new(v, LaneChangeModel::DK2008);
        base.vt = &MSLCM_DK2008_VT;
        MSLCM_DK2008 {
            base,
            myChangeProbability: 0.,
            myLeadingBlockerLength: 0.,
            myLeftSpace: 0.,
            myVSafes: Vec::new(),
            myDontBrake: false,
        }
    }

    /// C++: `void MSLCM_DK2008::~MSLCM_DK2008() noexcept` [virtual]
    // MSLCM_DK2008::~MSLCM_DK2008() {
    //     changed();
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        self.changed();
    }

    /// C++: `LaneChangeModel MSLCM_DK2008::getModelID() const` [const, virtual]
    /// Returns the model's id
    // LaneChangeModel getModelID() const {
    //         return LaneChangeModel::DK2008;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> LaneChangeModel {
        LaneChangeModel::DK2008
    }

    /// C++: `int MSLCM_DK2008::wantsChange(int, MSAbstractLaneChangeModel::MSLCMessager &, int, const std::pair<MSVehicle *, double> &, const std::pair<MSVehicle *, double> &, const std::pair<MSVehicle *, double> &, const std::pair<MSVehicle *, double> &, const MSLane &, const std::vector<MSVehicle::LaneQ> &, MSVehicle **, MSVehicle **)` [virtual]
    /// Called to examine whether the vehicle wants to change using the given laneOffset. This method gets the information about the surrounding vehicles and whether another lane may be more preferable
    // int
    // MSLCM_DK2008::wantsChange(
    //     int laneOffset,
    //     MSAbstractLaneChangeModel::MSLCMessager& msgPass, int blocked,
    //     const std::pair<MSVehicle*, double>& leader,
    //     const std::pair<MSVehicle*, double>& /*follower*/,
    //     const std::pair<MSVehicle*, double>& neighLead,
    //     const std::pair<MSVehicle*, double>& neighFollow,
    //     const MSLane& neighLane,
    //     const std::vector<MSVehicle::LaneQ>& preb,
    //     MSVehicle** lastBlocked,
    //     MSVehicle** firstBlocked) {
    //     UNUSED_PARAMETER(firstBlocked);
    //     return (laneOffset == -1 ?
    //             wantsChangeToRight(msgPass, blocked, leader, neighLead, neighFollow, neighLane, preb, lastBlocked, firstBlocked)
    //             : wantsChangeToLeft(msgPass, blocked, leader, neighLead, neighFollow, neighLane, preb, lastBlocked, firstBlocked));
    //
    // }
    #[allow(unused_variables)]
    pub fn wants_change(&mut self, laneOffset: i32, msgPass: &mut MSAbstractLaneChangeModel_MSLCMessager, blocked: i32, leader: &(Ptr<MSVehicle>, f64), follower: &(Ptr<MSVehicle>, f64), neighLead: &(Ptr<MSVehicle>, f64), neighFollow: &(Ptr<MSVehicle>, f64), neighLane: &MSLane, preb: &Vec<MSVehicle_LaneQ>, lastBlocked: Ptr<Ptr<MSVehicle>>, firstBlocked: Ptr<Ptr<MSVehicle>>) -> i32 {
        // firstBlocked is unused in the C++ body (UNUSED_PARAMETER)
        if laneOffset == -1 {
            self.wants_change_to_right(msgPass, blocked, leader, neighLead, neighFollow, neighLane, preb, lastBlocked, firstBlocked)
        } else {
            self.wants_change_to_left(msgPass, blocked, leader, neighLead, neighFollow, neighLane, preb, lastBlocked, firstBlocked)
        }
    }

    /// C++: `void * MSLCM_DK2008::inform(void *, MSVehicle *)` [virtual]
    // void*
    // MSLCM_DK2008::inform(void* info, MSVehicle* /*sender*/) {
    //     Info* pinfo = (Info*) info;
    //     //myOwnState &= 0xffffffff; // reset all bits of MyLCAEnum but only those
    //     myOwnState |= pinfo->second;
    //     delete pinfo;
    //     return (void*) true;
    // }
    #[allow(unused_variables)]
    pub fn inform(&mut self, info: Ptr<()>, sender: Ptr<MSVehicle>) -> Ptr<()> {
        // sender is unused in the C++ body
        // Info* pinfo = (Info*) info; ... delete pinfo;
        let pinfo = unsafe { Box::from_raw(info.as_raw() as *mut MSLCM_DK2008_Info) };
        //myOwnState &= 0xffffffff; // reset all bits of MyLCAEnum but only those
        self.myOwnState |= pinfo.1;
        // C++ returns (void*) true
        Ptr::from_raw(1usize as *mut ())
    }

    /// C++: `double MSLCM_DK2008::patchSpeed(const double, const double, const double, const MSCFModel &)` [virtual]
    /// Called to adapt the speed in order to allow a lane change.
    // double
    // MSLCM_DK2008::patchSpeed(const double min, const double wanted, const double max, const MSCFModel& cfModel) {
    //     int state = myOwnState;
    //
    //     // letting vehicles merge in at the end of the lane in case of counter-lane change, step#2
    //     double MAGIC_offset = 1.;
    //     //   if we want to change and have a blocking leader and there is enough room for him in front of us
    //     if (myLeadingBlockerLength != 0) {
    //         double space = myLeftSpace - myLeadingBlockerLength - MAGIC_offset - myVehicle.getVehicleType().getMinGap();
    //         if (space > 0) {
    //             // compute speed for decelerating towards a place which allows the blocking leader to merge in in front
    //             double safe = cfModel.stopSpeed(&myVehicle, myVehicle.getSpeed(), space, MSCFModel::CalcReason::LANE_CHANGE);
    //             // if we are approaching this place
    //             if (safe < wanted) {
    //                 // return this speed as the speed to use
    //                 return MAX2(min, safe);
    //             }
    //         }
    //     }
    //
    //     // just to make sure to be notified about lane chaning end
    //     if (myVehicle.getLane()->getEdge().getLanes().size() == 1 || myVehicle.getLane()->getEdge().isInternal()) {
    //         // remove chaning information if on a road with a single lane
    //         changed();
    //         return wanted;
    //     }
    //
    //     double nVSafe = wanted;
    //     bool gotOne = false;
    //     for (std::vector<double>::const_iterator i = myVSafes.begin(); i != myVSafes.end(); ++i) {
    //         double v = (*i);
    //         if (v >= min && v <= max) {
    //             nVSafe = MIN2(v, nVSafe);
    //             gotOne = true;
    //         }
    //     }
    //
    //     // check whether the vehicle is blocked
    //     if ((state & LCA_WANTS_LANECHANGE) != 0) {
    //         if (gotOne && !myDontBrake) {
    //             return nVSafe;
    //         }
    //         // check whether the vehicle maybe has to be swapped with one of
    //         //  the blocking vehicles
    //         if ((state & LCA_BLOCKED) != 0) {
    //             if ((state & LCA_BLOCKED_BY_LEADER) != 0) {
    //                 // if interacting with leader and not too slow
    //                 return (min + wanted) / (double) 2.0;
    //             }
    //             if ((state & LCA_BLOCKED_BY_FOLLOWER) != 0) {
    //                 return (max + wanted) / (double) 2.0;
    //             }
    //             return (min + wanted) / (double) 2.0;
    //         }
    //     }
    //
    //
    //     // decelerate if being a blocking follower
    //     //  (and does not have to change lanes)
    //     if ((state & LCA_AMBLOCKINGFOLLOWER) != 0) {
    //         if (fabs(max - myVehicle.getCarFollowModel().maxNextSpeed(myVehicle.getSpeed(), &myVehicle)) < 0.001 && min == 0) { // !!! was standing
    //             return 0;
    //         }
    //         return (min + wanted) / (double) 2.0;
    //     }
    //     if ((state & LCA_AMBACKBLOCKER) != 0) {
    //         if (max <= myVehicle.getCarFollowModel().maxNextSpeed(myVehicle.getSpeed(), &myVehicle) && min == 0) { // !!! was standing
    //             return min;
    //         }
    //     }
    //     if ((state & LCA_AMBACKBLOCKER_STANDING) != 0) {
    //         return min;
    //     }
    //     // accelerate if being a blocking leader or blocking follower not able to brake
    //     //  (and does not have to change lanes)
    //     if ((state & LCA_AMBLOCKINGLEADER) != 0) {
    //         return (max + wanted) / (double) 2.0;
    //     }
    //     if ((state & LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0) {
    //         if (max <= myVehicle.getCarFollowModel().maxNextSpeed(myVehicle.getSpeed(), &myVehicle) && min == 0) { // !!! was standing
    //             return wanted;
    //         }
    //         return (min + wanted) / (double) 2.0;
    //     }
    //     return wanted;
    // }
    #[allow(unused_variables)]
    pub fn patch_speed(&mut self, min: f64, wanted: f64, max: f64, cfModel: &MSCFModel) -> f64 {
        let myVehicle = self.myVehicle;
        let state = self.myOwnState;

        // letting vehicles merge in at the end of the lane in case of counter-lane change, step#2
        let MAGIC_offset = 1.;
        //   if we want to change and have a blocking leader and there is enough room for him in front of us
        if self.myLeadingBlockerLength != 0. {
            let space = self.myLeftSpace - self.myLeadingBlockerLength - MAGIC_offset - myVehicle.get_vehicle_type().get_min_gap();
            if space > 0. {
                // compute speed for decelerating towards a place which allows the blocking leader to merge in in front
                let safe = cfModel.stop_speed(myVehicle, myVehicle.get_speed(), space, MSCFModel_CalcReason::LANE_CHANGE);
                // if we are approaching this place
                if safe < wanted {
                    // return this speed as the speed to use
                    return max2(min, safe);
                }
            }
        }

        // just to make sure to be notified about lane chaning end
        if myVehicle.get_lane().get_edge().get_lanes().len() == 1 || myVehicle.get_lane().get_edge().is_internal() {
            // remove chaning information if on a road with a single lane
            self.changed();
            return wanted;
        }

        let mut nVSafe = wanted;
        let mut gotOne = false;
        for &v in self.myVSafes.iter() {
            if v >= min && v <= max {
                nVSafe = min2(v, nVSafe);
                gotOne = true;
            }
        }

        // check whether the vehicle is blocked
        if (state & LCA_WANTS_LANECHANGE) != 0 {
            if gotOne && !self.myDontBrake {
                return nVSafe;
            }
            // check whether the vehicle maybe has to be swapped with one of
            //  the blocking vehicles
            if (state & LCA_BLOCKED) != 0 {
                if (state & LCA_BLOCKED_BY_LEADER) != 0 {
                    // if interacting with leader and not too slow
                    return (min + wanted) / 2.0;
                }
                if (state & LCA_BLOCKED_BY_FOLLOWER) != 0 {
                    return (max + wanted) / 2.0;
                }
                return (min + wanted) / 2.0;
            }
        }

        // decelerate if being a blocking follower
        //  (and does not have to change lanes)
        if (state & LCA_AMBLOCKINGFOLLOWER) != 0 {
            if (max - myVehicle.get_car_follow_model().max_next_speed(myVehicle.get_speed(), myVehicle)).abs() < 0.001 && min == 0. {
                // !!! was standing
                return 0.;
            }
            return (min + wanted) / 2.0;
        }
        if (state & LCA_AMBACKBLOCKER) != 0 {
            if max <= myVehicle.get_car_follow_model().max_next_speed(myVehicle.get_speed(), myVehicle) && min == 0. {
                // !!! was standing
                return min;
            }
        }
        if (state & LCA_AMBACKBLOCKER_STANDING) != 0 {
            return min;
        }
        // accelerate if being a blocking leader or blocking follower not able to brake
        //  (and does not have to change lanes)
        if (state & LCA_AMBLOCKINGLEADER) != 0 {
            return (max + wanted) / 2.0;
        }
        if (state & LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0 {
            if max <= myVehicle.get_car_follow_model().max_next_speed(myVehicle.get_speed(), myVehicle) && min == 0. {
                // !!! was standing
                return wanted;
            }
            return (min + wanted) / 2.0;
        }
        wanted
    }

    /// C++: `void MSLCM_DK2008::changed()` [virtual]
    // void
    // MSLCM_DK2008::changed() {
    //     myOwnState = 0;
    //     myChangeProbability = 0;
    //     myLeadingBlockerLength = 0;
    //     myLeftSpace = 0;
    //     myVSafes.clear();
    //     myDontBrake = false;
    // }
    #[allow(unused_variables)]
    pub fn changed(&mut self) {
        self.myOwnState = 0;
        self.myChangeProbability = 0.;
        self.myLeadingBlockerLength = 0.;
        self.myLeftSpace = 0.;
        self.myVSafes.clear();
        self.myDontBrake = false;
    }

    /// C++: `void MSLCM_DK2008::prepareStep()` [virtual]
    // void
    // MSLCM_DK2008::prepareStep() {
    //     MSAbstractLaneChangeModel::prepareStep();
    //     myOwnState = 0;
    //     myLeadingBlockerLength = 0;
    //     myLeftSpace = 0;
    //     myVSafes.clear();
    //     myDontBrake = false;
    //     // truncate myChangeProbability to work around numerical instability between different builds
    //     myChangeProbability = ceil(myChangeProbability * 100000.0) * 0.00001;
    //     resetSpeedLat();
    // }
    #[allow(unused_variables)]
    pub fn prepare_step(&mut self) {
        self.base.prepare_step_base();
        self.myOwnState = 0;
        self.myLeadingBlockerLength = 0.;
        self.myLeftSpace = 0.;
        self.myVSafes.clear();
        self.myDontBrake = false;
        // truncate myChangeProbability to work around numerical instability between different builds
        self.myChangeProbability = (self.myChangeProbability * 100000.0).ceil() * 0.00001;
        self.reset_speed_lat();
    }

    /// C++: `int MSLCM_DK2008::wantsChangeToRight(MSAbstractLaneChangeModel::MSLCMessager &, int, const std::pair<MSVehicle *, double> &, const std::pair<MSVehicle *, double> &, const std::pair<MSVehicle *, double> &, const MSLane &, const std::vector<MSVehicle::LaneQ> &, MSVehicle **, MSVehicle **)` [virtual, protected]
    /// Called to examine whether the vehicle wants to change to right This method gets the information about the surrounding vehicles and whether another lane may be more preferable
    // int
    // MSLCM_DK2008::wantsChangeToRight(MSAbstractLaneChangeModel::MSLCMessager& msgPass,
    //                                  int blocked,
    //                                  const std::pair<MSVehicle*, double>& leader,
    //                                  const std::pair<MSVehicle*, double>& neighLead,
    //                                  const std::pair<MSVehicle*, double>& neighFollow,
    //                                  const MSLane& neighLane,
    //                                  const std::vector<MSVehicle::LaneQ>& preb,
    //                                  MSVehicle** lastBlocked,
    //                                  MSVehicle** firstBlocked) {
    //     UNUSED_PARAMETER(firstBlocked);
    //     MSVehicle::LaneQ curr, best;
    //     int bestLaneOffset = 0;
    //     double currentDist = 0;
    //     double neighDist = 0;
    //     double neighExtDist = 0;
    //     double currExtDist = 0;
    //     int currIdx = 0;
    //     const MSLane* prebLane = myVehicle.getLane();
    //     if (prebLane->getEdge().isInternal()) {
    //         // internal edges are not kept inside the bestLanes structure
    //         prebLane = prebLane->getLinkCont()[0]->getLane();
    //     }
    //     for (int p = 0; p < (int) preb.size(); ++p) {
    //         if (preb[p].lane == prebLane && p > 0) {
    //             curr = preb[p];
    //             bestLaneOffset = curr.bestLaneOffset;
    //             currentDist = curr.length;
    //             currExtDist = curr.lane->getLength();
    //             neighDist = preb[p - 1].length;
    //             neighExtDist = preb[p - 1].lane->getLength();
    //             best = preb[p + bestLaneOffset];
    //             currIdx = p;
    //             break;
    //         }
    //     }
    //
    //     // keep information about being a leader/follower
    //     int ret = (myOwnState & 0xffff0000);
    //
    //     if (leader.first != 0
    //             &&
    //             (myOwnState & LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0
    //             &&
    //             (leader.first->getLaneChangeModel().getOwnState()&LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0) {
    //
    //         myOwnState &= (0xffffffff - LCA_AMBLOCKINGFOLLOWER_DONTBRAKE);
    //         if (myVehicle.getSpeed() > SUMO_const_haltingSpeed) {
    //             myOwnState |= LCA_AMBACKBLOCKER;
    //         } else {
    //             ret |= LCA_AMBACKBLOCKER;
    //             myDontBrake = true;
    //         }
    //     }
    //
    //     // process information about the last blocked vehicle
    //     //  if this vehicle is blocking someone in front, we maybe decelerate to let him in
    //     if ((*lastBlocked) != nullptr) {
    //         double gap = (*lastBlocked)->getPositionOnLane() - (*lastBlocked)->getVehicleType().getLength() - myVehicle.getPositionOnLane() - myVehicle.getVehicleType().getMinGap();
    //         if (gap > 0.1) {
    //             if (myVehicle.getSpeed() < ACCEL2SPEED(myVehicle.getCarFollowModel().getMaxDecel())) {
    //                 if ((*lastBlocked)->getSpeed() < SUMO_const_haltingSpeed) {
    //                     ret |= LCA_AMBACKBLOCKER_STANDING;
    //                 } else {
    //                     ret |= LCA_AMBACKBLOCKER;
    //                 }
    //                 myVSafes.push_back(getCarFollowModel().followSpeed(&myVehicle, myVehicle.getSpeed(), (double)(gap - 0.1), (*lastBlocked)->getSpeed(), (*lastBlocked)->getCarFollowModel().getMaxDecel()));
    //                 (*lastBlocked) = nullptr;
    //             }
    //             return ret;
    //         }
    //     }
    //
    //     // we try to estimate the distance which is necessary to get on a lane
    //     //  we have to get on in order to keep our route
    //     // we assume we need something that depends on our velocity
    //     // and compare this with the free space on our wished lane
    //     //
    //     // if the free space is somehow less than the space we need, we should
    //     //  definitely try to get to the desired lane
    //     //
    //     // this rule forces our vehicle to change the lane if a lane changing is necessary soon
    //     double rv = myVehicle.getSpeed() > LOOK_FORWARD_SPEED_DIVIDER
    //                 ? myVehicle.getSpeed() * (double) LOOK_FORWARD_FAR
    //                 : myVehicle.getSpeed() * (double) LOOK_FORWARD_NEAR;
    //     rv += myVehicle.getVehicleType().getLengthWithGap() * (double) 2.;
    //
    //     double tdist = currentDist - myVehicle.getPositionOnLane() - best.occupation * (double) JAM_FACTOR2;
    //
    //     // assert(best.length > curr.length);
    //     // XXX if (curr.length != best.length) && ...
    //     if (fabs(best.length - curr.length) > MIN2((double) .1, best.lane->getLength()) && bestLaneOffset < 0 && currentDistDisallows(tdist/*currentDist*/, bestLaneOffset, rv)) {
    //         informBlocker(msgPass, blocked, LCA_MRIGHT, neighLead, neighFollow);
    //         if (neighLead.second > 0 && neighLead.second > leader.second) {
    //             myVSafes.push_back(getCarFollowModel().followSpeed(&myVehicle, myVehicle.getSpeed(), neighLead.second, neighLead.first->getSpeed(), neighLead.first->getCarFollowModel().getMaxDecel()) - (double) 0.5);
    //         }
    //
    //         // letting vehicles merge in at the end of the lane in case of counter-lane change, step#1, right
    //         //   if there is a leader and he wants to change to left (we want to change to right)
    //         if (neighLead.first != 0 && (neighLead.first->getLaneChangeModel().getOwnState()&LCA_LEFT) != 0) {
    //             // save at least his length in myLeadingBlockerLength
    //             myLeadingBlockerLength = MAX2(neighLead.first->getVehicleType().getLengthWithGap(), myLeadingBlockerLength);
    //             // save the left space
    //             myLeftSpace = currentDist - myVehicle.getPositionOnLane();
    //         }
    //         //
    //
    //         return ret | LCA_RIGHT | LCA_STRATEGIC | LCA_URGENT;
    //     }
    //
    //
    //     // the opposite lane-changing direction should be done than the one examined herein
    //     //  we'll check whether we assume we could change anyhow and get back in time...
    //     //
    //     // this rule prevents the vehicle from moving in opposite direction of the best lane
    //     //  unless the way till the end where the vehicle has to be on the best lane
    //     //  is long enough
    //     double maxJam = MAX2(preb[currIdx - 1].occupation, preb[currIdx].occupation);
    //     double neighLeftPlace = MAX2((double) 0, neighDist - myVehicle.getPositionOnLane() - maxJam);
    //     if (bestLaneOffset >= 0 && (currentDistDisallows(neighLeftPlace, bestLaneOffset + 2, rv))) {
    //         // ...we will not change the lane if not
    //         return ret | LCA_STAY | LCA_STRATEGIC;
    //     }
    //
    //
    //     // if the current lane is the best and a lane-changing would cause a situation
    //     //  of which we assume we will not be able to return to the lane we have to be on...
    //     //
    //     // this rule prevents the vehicle from leaving the current, best lane when it is
    //     //  close to this lane's end
    //     if (currExtDist > neighExtDist && (neighLeftPlace * 2. < rv/*||currE[currIdx+1].length<currentDist*/)) {
    //         return ret | LCA_STAY | LCA_STRATEGIC;
    //     }
    //
    //     // let's also regard the case where the vehicle is driving on a highway...
    //     //  in this case, we do not want to get to the dead-end of an on-ramp
    //     //
    //     // THIS RULE APPLIES ONLY TO CHANGING TO THE RIGHT LANE
    //     if (bestLaneOffset == 0 && preb[currIdx - 1].bestLaneOffset != 0 && myVehicle.getLane()->getSpeedLimit() > 80. / 3.6) {
    //         return ret | LCA_STAY | LCA_STRATEGIC;
    //     }
    //     // --------
    //
    //     // -------- make place on current lane if blocking follower
    //     if (amBlockingFollowerPlusNB()
    //             &&
    //             (currentDistAllows(neighDist, bestLaneOffset, rv) || neighDist >= currentDist)) {
    //
    //         return ret | LCA_RIGHT | LCA_COOPERATIVE | LCA_URGENT;
    //     }
    //     // --------
    //
    //
    //     // -------- security checks for krauss
    //     //  (vsafe fails when gap<0)
    //     if ((blocked & LCA_BLOCKED) != 0) {
    //         return ret;
    //     }
    //     // --------
    //
    //     // -------- higher speed
    //     if ((congested(neighLead.first) && neighLead.second < 20) || predInteraction(leader)) { //!!!
    //         return ret;
    //     }
    //     double thisLaneVSafe = myVehicle.getLane()->getVehicleMaxSpeed(&myVehicle);
    //     double neighLaneVSafe = neighLane.getVehicleMaxSpeed(&myVehicle);
    //     if (neighLead.first == 0) {
    //         neighLaneVSafe = MIN2(neighLaneVSafe, getCarFollowModel().followSpeed(&myVehicle, myVehicle.getSpeed(), neighDist, 0, 0));
    //     } else {
    //         // @todo: what if leader is below safe gap?!!!
    //         neighLaneVSafe = MIN2(neighLaneVSafe, getCarFollowModel().followSpeed(&myVehicle, myVehicle.getSpeed(), neighLead.second, neighLead.first->getSpeed(), neighLead.first->getCarFollowModel().getMaxDecel()));
    //     }
    //     if (leader.first == 0) {
    //         thisLaneVSafe = MIN2(thisLaneVSafe, getCarFollowModel().followSpeed(&myVehicle, myVehicle.getSpeed(), currentDist, 0, 0));
    //     } else {
    //         // @todo: what if leader is below safe gap?!!!
    //         thisLaneVSafe = MIN2(thisLaneVSafe, getCarFollowModel().followSpeed(&myVehicle, myVehicle.getSpeed(), leader.second, leader.first->getSpeed(), leader.first->getCarFollowModel().getMaxDecel()));
    //     }
    //
    //     thisLaneVSafe = MIN2(thisLaneVSafe, myVehicle.getMaxSpeed());
    //     neighLaneVSafe = MIN2(neighLaneVSafe, myVehicle.getMaxSpeed());
    //     if (thisLaneVSafe - neighLaneVSafe > 5. / 3.6) {
    //         // ok, the current lane is faster than the right one...
    //         if (myChangeProbability < 0) {
    //             myChangeProbability *= pow(0.5, TS);
    //         }
    //     } else {
    //         // ok, the right lane is faster than the current
    //         myChangeProbability -= TS * ((neighLaneVSafe - thisLaneVSafe) / (myVehicle.getLane()->getVehicleMaxSpeed(&myVehicle)));
    //     }
    //
    //     // let's recheck the "Rechtsfahrgebot"
    //     double vmax = myVehicle.getLane()->getVehicleMaxSpeed(&myVehicle);
    //     vmax -= (double)(5. / 2.6);
    //     if (neighLaneVSafe >= vmax) {
    //         myChangeProbability -= TS * ((neighLaneVSafe - vmax) / (vmax));
    //     }
    //
    //     if (myChangeProbability < -2 && neighDist / MAX2((double) .1, myVehicle.getSpeed()) > 20.) { //./MAX2((double) .1, myVehicle.getSpeed())) { // -.1
    //         return ret | LCA_RIGHT | LCA_SPEEDGAIN;
    //     }
    //     // --------
    //
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn wants_change_to_right(&mut self, msgPass: &mut MSAbstractLaneChangeModel_MSLCMessager, blocked: i32, leader: &(Ptr<MSVehicle>, f64), neighLead: &(Ptr<MSVehicle>, f64), neighFollow: &(Ptr<MSVehicle>, f64), neighLane: &MSLane, preb: &Vec<MSVehicle_LaneQ>, lastBlocked: Ptr<Ptr<MSVehicle>>, firstBlocked: Ptr<Ptr<MSVehicle>>) -> i32 {
        // UNUSED_PARAMETER(firstBlocked);
        let mut blocked = blocked;
        let myVehicle = self.myVehicle;
        let mut curr = lane_q_default();
        let mut best = lane_q_default();
        let mut bestLaneOffset: i32 = 0;
        let mut currentDist: f64 = 0.;
        let mut neighDist: f64 = 0.;
        let mut neighExtDist: f64 = 0.;
        let mut currExtDist: f64 = 0.;
        let mut currIdx: i32 = 0;
        let mut prebLane = myVehicle.get_lane();
        if prebLane.get_edge().is_internal() {
            // internal edges are not kept inside the bestLanes structure
            prebLane = prebLane.get_link_cont()[0].get_lane();
        }
        for p in 0..preb.len() {
            if preb[p].lane == prebLane && p > 0 {
                curr = lane_q_copy(&preb[p]);
                bestLaneOffset = curr.bestLaneOffset;
                currentDist = curr.length;
                currExtDist = curr.lane.get_length();
                neighDist = preb[p - 1].length;
                neighExtDist = preb[p - 1].lane.get_length();
                best = lane_q_copy(&preb[(p as i32 + bestLaneOffset) as usize]);
                currIdx = p as i32;
                break;
            }
        }

        // keep information about being a leader/follower
        let mut ret: i32 = self.myOwnState & (0xffff0000u32 as i32);

        if !leader.0.is_null()
            && (self.myOwnState & LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0
            && (leader.0.m().get_lane_change_model().get_own_state() & LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0
        {
            self.myOwnState &= !LCA_AMBLOCKINGFOLLOWER_DONTBRAKE;
            if myVehicle.get_speed() > SUMO_const_haltingSpeed {
                self.myOwnState |= LCA_AMBACKBLOCKER;
            } else {
                ret |= LCA_AMBACKBLOCKER;
                self.myDontBrake = true;
            }
        }

        // process information about the last blocked vehicle
        //  if this vehicle is blocking someone in front, we maybe decelerate to let him in
        if !(*lastBlocked).is_null() {
            let lb = *lastBlocked;
            let gap = lb.get_position_on_lane() - lb.get_vehicle_type().get_length() - myVehicle.get_position_on_lane() - myVehicle.get_vehicle_type().get_min_gap();
            if gap > 0.1 {
                if myVehicle.get_speed() < accel2speed(myVehicle.get_car_follow_model().get_max_decel()) {
                    if lb.get_speed() < SUMO_const_haltingSpeed {
                        ret |= LCA_AMBACKBLOCKER_STANDING;
                    } else {
                        ret |= LCA_AMBACKBLOCKER;
                    }
                    let vsafe = myVehicle.get_car_follow_model().follow_speed(
                        myVehicle,
                        myVehicle.get_speed(),
                        gap - 0.1,
                        lb.get_speed(),
                        lb.get_car_follow_model().get_max_decel(),
                        Ptr::null(),
                        MSCFModel_CalcReason::CURRENT,
                    );
                    self.myVSafes.push(vsafe);
                    *lastBlocked.m() = Ptr::null();
                }
                return ret;
            }
        }

        // we try to estimate the distance which is necessary to get on a lane
        //  we have to get on in order to keep our route
        // we assume we need something that depends on our velocity
        // and compare this with the free space on our wished lane
        //
        // if the free space is somehow less than the space we need, we should
        //  definitely try to get to the desired lane
        //
        // this rule forces our vehicle to change the lane if a lane changing is necessary soon
        let mut rv = if myVehicle.get_speed() > LOOK_FORWARD_SPEED_DIVIDER {
            myVehicle.get_speed() * LOOK_FORWARD_FAR
        } else {
            myVehicle.get_speed() * LOOK_FORWARD_NEAR
        };
        rv += myVehicle.get_vehicle_type().get_length_with_gap() * 2.;

        let tdist = currentDist - myVehicle.get_position_on_lane() - best.occupation * JAM_FACTOR2;

        // assert(best.length > curr.length);
        // XXX if (curr.length != best.length) && ...
        if (best.length - curr.length).abs() > min2(0.1, best.lane.get_length()) && bestLaneOffset < 0 && self.current_dist_disallows(tdist, bestLaneOffset, rv) {
            self.inform_blocker(msgPass, &mut blocked, LCA_MRIGHT, neighLead, neighFollow);
            if neighLead.1 > 0. && neighLead.1 > leader.1 {
                let vsafe = myVehicle.get_car_follow_model().follow_speed(
                    myVehicle,
                    myVehicle.get_speed(),
                    neighLead.1,
                    neighLead.0.get_speed(),
                    neighLead.0.get_car_follow_model().get_max_decel(),
                    Ptr::null(),
                    MSCFModel_CalcReason::CURRENT,
                ) - 0.5;
                self.myVSafes.push(vsafe);
            }

            // letting vehicles merge in at the end of the lane in case of counter-lane change, step#1, right
            //   if there is a leader and he wants to change to left (we want to change to right)
            if !neighLead.0.is_null() && (neighLead.0.m().get_lane_change_model().get_own_state() & LCA_LEFT) != 0 {
                // save at least his length in myLeadingBlockerLength
                self.myLeadingBlockerLength = max2(neighLead.0.get_vehicle_type().get_length_with_gap(), self.myLeadingBlockerLength);
                // save the left space
                self.myLeftSpace = currentDist - myVehicle.get_position_on_lane();
            }

            return ret | LCA_RIGHT | LCA_STRATEGIC | LCA_URGENT;
        }

        // the opposite lane-changing direction should be done than the one examined herein
        //  we'll check whether we assume we could change anyhow and get back in time...
        //
        // this rule prevents the vehicle from moving in opposite direction of the best lane
        //  unless the way till the end where the vehicle has to be on the best lane
        //  is long enough
        let maxJam = max2(preb[(currIdx - 1) as usize].occupation, preb[currIdx as usize].occupation);
        let neighLeftPlace = max2(0., neighDist - myVehicle.get_position_on_lane() - maxJam);
        if bestLaneOffset >= 0 && self.current_dist_disallows(neighLeftPlace, bestLaneOffset + 2, rv) {
            // ...we will not change the lane if not
            return ret | LCA_STAY | LCA_STRATEGIC;
        }

        // if the current lane is the best and a lane-changing would cause a situation
        //  of which we assume we will not be able to return to the lane we have to be on...
        //
        // this rule prevents the vehicle from leaving the current, best lane when it is
        //  close to this lane's end
        if currExtDist > neighExtDist && (neighLeftPlace * 2. < rv) {
            return ret | LCA_STAY | LCA_STRATEGIC;
        }

        // let's also regard the case where the vehicle is driving on a highway...
        //  in this case, we do not want to get to the dead-end of an on-ramp
        //
        // THIS RULE APPLIES ONLY TO CHANGING TO THE RIGHT LANE
        if bestLaneOffset == 0 && preb[(currIdx - 1) as usize].bestLaneOffset != 0 && myVehicle.get_lane().get_speed_limit() > 80. / 3.6 {
            return ret | LCA_STAY | LCA_STRATEGIC;
        }
        // --------

        // -------- make place on current lane if blocking follower
        if self.am_blocking_follower_plus_nb() && (self.current_dist_allows(neighDist, bestLaneOffset, rv) || neighDist >= currentDist) {
            return ret | LCA_RIGHT | LCA_COOPERATIVE | LCA_URGENT;
        }
        // --------

        // -------- security checks for krauss
        //  (vsafe fails when gap<0)
        if (blocked & LCA_BLOCKED) != 0 {
            return ret;
        }
        // --------

        // -------- higher speed
        if (self.congested(neighLead.0) && neighLead.1 < 20.) || self.pred_interaction(leader) {
            // !!!
            return ret;
        }
        let mut thisLaneVSafe = myVehicle.get_lane().get_vehicle_max_speed(traffic_object_of(myVehicle));
        let mut neighLaneVSafe = neighLane.get_vehicle_max_speed(traffic_object_of(myVehicle));
        if neighLead.0.is_null() {
            neighLaneVSafe = min2(
                neighLaneVSafe,
                myVehicle.get_car_follow_model().follow_speed(myVehicle, myVehicle.get_speed(), neighDist, 0., 0., Ptr::null(), MSCFModel_CalcReason::CURRENT),
            );
        } else {
            // @todo: what if leader is below safe gap?!!!
            neighLaneVSafe = min2(
                neighLaneVSafe,
                myVehicle.get_car_follow_model().follow_speed(
                    myVehicle,
                    myVehicle.get_speed(),
                    neighLead.1,
                    neighLead.0.get_speed(),
                    neighLead.0.get_car_follow_model().get_max_decel(),
                    Ptr::null(),
                    MSCFModel_CalcReason::CURRENT,
                ),
            );
        }
        if leader.0.is_null() {
            thisLaneVSafe = min2(
                thisLaneVSafe,
                myVehicle.get_car_follow_model().follow_speed(myVehicle, myVehicle.get_speed(), currentDist, 0., 0., Ptr::null(), MSCFModel_CalcReason::CURRENT),
            );
        } else {
            // @todo: what if leader is below safe gap?!!!
            thisLaneVSafe = min2(
                thisLaneVSafe,
                myVehicle.get_car_follow_model().follow_speed(
                    myVehicle,
                    myVehicle.get_speed(),
                    leader.1,
                    leader.0.get_speed(),
                    leader.0.get_car_follow_model().get_max_decel(),
                    Ptr::null(),
                    MSCFModel_CalcReason::CURRENT,
                ),
            );
        }

        thisLaneVSafe = min2(thisLaneVSafe, myVehicle.get_max_speed());
        neighLaneVSafe = min2(neighLaneVSafe, myVehicle.get_max_speed());
        if thisLaneVSafe - neighLaneVSafe > 5. / 3.6 {
            // ok, the current lane is faster than the right one...
            if self.myChangeProbability < 0. {
                self.myChangeProbability *= 0.5f64.powf(ts());
            }
        } else {
            // ok, the right lane is faster than the current
            self.myChangeProbability -= ts() * ((neighLaneVSafe - thisLaneVSafe) / (myVehicle.get_lane().get_vehicle_max_speed(traffic_object_of(myVehicle))));
        }

        // let's recheck the "Rechtsfahrgebot"
        let mut vmax = myVehicle.get_lane().get_vehicle_max_speed(traffic_object_of(myVehicle));
        vmax -= 5. / 2.6;
        if neighLaneVSafe >= vmax {
            self.myChangeProbability -= ts() * ((neighLaneVSafe - vmax) / (vmax));
        }

        if self.myChangeProbability < -2. && neighDist / max2(0.1, myVehicle.get_speed()) > 20. {
            // ./MAX2((double) .1, myVehicle.getSpeed())) { // -.1
            return ret | LCA_RIGHT | LCA_SPEEDGAIN;
        }
        // --------

        ret
    }

    /// C++: `int MSLCM_DK2008::wantsChangeToLeft(MSAbstractLaneChangeModel::MSLCMessager &, int, const std::pair<MSVehicle *, double> &, const std::pair<MSVehicle *, double> &, const std::pair<MSVehicle *, double> &, const MSLane &, const std::vector<MSVehicle::LaneQ> &, MSVehicle **, MSVehicle **)` [virtual, protected]
    /// Called to examine whether the vehicle wants to change to left This method gets the information about the surrounding vehicles and whether another lane may be more preferable
    // int
    // MSLCM_DK2008::wantsChangeToLeft(MSAbstractLaneChangeModel::MSLCMessager& msgPass,
    //                                 int blocked,
    //                                 const std::pair<MSVehicle*, double>& leader,
    //                                 const std::pair<MSVehicle*, double>& neighLead,
    //                                 const std::pair<MSVehicle*, double>& neighFollow,
    //                                 const MSLane& neighLane,
    //                                 const std::vector<MSVehicle::LaneQ>& preb,
    //                                 MSVehicle** lastBlocked,
    //                                 MSVehicle** firstBlocked) {
    //     UNUSED_PARAMETER(firstBlocked);
    //     MSVehicle::LaneQ curr, best;
    //     int bestLaneOffset = 0;
    //     double currentDist = 0;
    //     double neighDist = 0;
    //     double neighExtDist = 0;
    //     double currExtDist = 0;
    //     int currIdx = 0;
    //     const MSLane* prebLane = myVehicle.getLane();
    //     if (prebLane->getEdge().isInternal()) {
    //         // internal edges are not kept inside the bestLanes structure
    //         prebLane = prebLane->getLinkCont()[0]->getLane();
    //     }
    //     for (int p = 0; p < (int) preb.size(); ++p) {
    //         if (preb[p].lane == prebLane) {
    //             curr = preb[p];
    //             bestLaneOffset = curr.bestLaneOffset;
    //             currentDist = curr.length;
    //             currExtDist = curr.lane->getLength();
    //             neighDist = preb[p + 1].length;
    //             neighExtDist = preb[p + 1].lane->getLength();
    //             best = preb[p + bestLaneOffset];
    //             currIdx = p;
    //             break;
    //         }
    //     }
    //     // keep information about being a leader/follower
    //     int ret = (myOwnState & 0xffff0000);
    //
    //     // ?!!!
    //     if (leader.first != 0
    //             &&
    //             (myOwnState & LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0
    //             &&
    //             (leader.first->getLaneChangeModel().getOwnState()&LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0) {
    //
    //         myOwnState &= (0xffffffff - LCA_AMBLOCKINGFOLLOWER_DONTBRAKE);
    //         if (myVehicle.getSpeed() > SUMO_const_haltingSpeed) {
    //             myOwnState |= LCA_AMBACKBLOCKER;
    //         } else {
    //             ret |= LCA_AMBACKBLOCKER;
    //             myDontBrake = true;
    //         }
    //     }
    //
    //     // process information about the last blocked vehicle
    //     //  if this vehicle is blocking someone in front, we maybe decelerate to let him in
    //     if ((*lastBlocked) != nullptr) {
    //         double gap = (*lastBlocked)->getPositionOnLane() - (*lastBlocked)->getVehicleType().getLength() - myVehicle.getPositionOnLane() - myVehicle.getVehicleType().getMinGap();
    //         if (gap > 0.1) {
    //             if (myVehicle.getSpeed() < ACCEL2SPEED(myVehicle.getCarFollowModel().getMaxDecel())) {
    //                 if ((*lastBlocked)->getSpeed() < SUMO_const_haltingSpeed) {
    //                     ret |= LCA_AMBACKBLOCKER_STANDING;
    //                 } else {
    //                     ret |= LCA_AMBACKBLOCKER;
    //                 }
    //                 myVSafes.push_back(getCarFollowModel().followSpeed(&myVehicle, myVehicle.getSpeed(), (double)(gap - 0.1), (*lastBlocked)->getSpeed(), (*lastBlocked)->getCarFollowModel().getMaxDecel()));
    //                 (*lastBlocked) = nullptr;
    //             }
    //             return ret;
    //         }
    //     }
    //
    //     // we try to estimate the distance which is necessary to get on a lane
    //     //  we have to get on in order to keep our route
    //     // we assume we need something that depends on our velocity
    //     // and compare this with the free space on our wished lane
    //     //
    //     // if the free space is somehow less than the space we need, we should
    //     //  definitely try to get to the desired lane
    //     //
    //     // this rule forces our vehicle to change the lane if a lane changing is necessary soon
    //     double lv = myVehicle.getSpeed() > LOOK_FORWARD_SPEED_DIVIDER
    //                 ? myVehicle.getSpeed() * (double) LOOK_FORWARD_FAR
    //                 : myVehicle.getSpeed() * (double) LOOK_FORWARD_NEAR;
    //     lv += myVehicle.getVehicleType().getLengthWithGap() * (double) 2.;
    //
    //
    //     double tdist = currentDist - myVehicle.getPositionOnLane() - best.occupation * (double) JAM_FACTOR2;
    //     if (fabs(best.length - curr.length) > MIN2((double) .1, best.lane->getLength()) && bestLaneOffset > 0
    //             &&
    //             currentDistDisallows(tdist/*currentDist*/, bestLaneOffset, lv)) {
    //         informBlocker(msgPass, blocked, LCA_MLEFT, neighLead, neighFollow);
    //         if (neighLead.second > 0 && neighLead.second > leader.second) {
    //             myVSafes.push_back(getCarFollowModel().followSpeed(&myVehicle, myVehicle.getSpeed(), neighLead.second, neighLead.first->getSpeed(), neighLead.first->getCarFollowModel().getMaxDecel()) - (double) 0.5);
    //         }
    //
    //         // letting vehicles merge in at the end of the lane in case of counter-lane change, step#1, left
    //         //   if there is a leader and he wants to change to right (we want to change to left)
    //         if (neighLead.first != 0 && (neighLead.first->getLaneChangeModel().getOwnState()&LCA_RIGHT) != 0) {
    //             // save at least his length in myLeadingBlockerLength
    //             myLeadingBlockerLength = MAX2(neighLead.first->getVehicleType().getLengthWithGap(), myLeadingBlockerLength);
    //             // save the left space
    //             myLeftSpace = currentDist - myVehicle.getPositionOnLane();
    //         }
    //         //
    //
    //         return ret | LCA_LEFT | LCA_STRATEGIC | LCA_URGENT;
    //     }
    //
    //     // the opposite lane-changing direction should be rather done, not
    //     //  the one examined herein
    //     //  we'll check whether we assume we could change anyhow and get back in time...
    //     //
    //     // this rule prevents the vehicle from moving in opposite direction of the best lane
    //     //  unless the way till the end where the vehicle has to be on the best lane
    //     //  is long enough
    //     double maxJam = MAX2(preb[currIdx + 1].occupation, preb[currIdx].occupation);
    //     double neighLeftPlace = MAX2((double) 0, neighDist - myVehicle.getPositionOnLane() - maxJam);
    //     if (bestLaneOffset <= 0 && (currentDistDisallows(neighLeftPlace, bestLaneOffset - 2, lv))) {
    //         // ...we will not change the lane if not
    //         return ret | LCA_STAY | LCA_STRATEGIC;
    //     }
    //
    //
    //     // if the current lane is the best and a lane-changing would cause a situation
    //     //  of which we assume we will not be able to return to the lane we have to be on...
    //     //
    //     // this rule prevents the vehicle from leaving the current, best lane when it is
    //     //  close to this lane's end
    //     if (currExtDist > neighExtDist && (neighLeftPlace * 2. < lv/*||currE[currIdx+1].length<currentDist*/)) {
    //         // ... let's not change the lane
    //         return ret | LCA_STAY | LCA_STRATEGIC;
    //     }
    //
    //     /*
    //     // let's also regard the case where the vehicle is driving on a highway...
    //     //  in this case, we do not want to get to the dead-end of an on-ramp
    //     if(bestLaneOffset==0&&myVehicle.getLane().getMaxSpeed()>80./3.6) {
    //         return ret;
    //     }
    //     */
    //
    //
    //     /*
    //     // if the current lane is the
    //     if(bestLaneOffset==0&&(neighDist==0||curr.seenVehicles2*JAM_FACTOR>=neighExtDist-curr.length)) {
    //         return ret;
    //     }
    //     */
    //     // --------
    //
    //     // -------- make place on current lane if blocking follower
    //     if (amBlockingFollowerPlusNB()
    //             &&
    //             (currentDistAllows(neighDist, bestLaneOffset, lv) || neighDist >= currentDist)) {
    //
    //         return ret | LCA_LEFT | LCA_COOPERATIVE | LCA_URGENT;
    //     }
    //     // --------
    //
    //     // -------- security checks for krauss
    //     //  (vsafe fails when gap<0)
    //     if ((blocked & LCA_BLOCKED) != 0) {
    //         return ret;
    //     }
    //
    //     // -------- higher speed
    //     if ((congested(neighLead.first) && neighLead.second < 20) || predInteraction(leader)) { //!!!
    //         return ret;
    //     }
    //     double neighLaneVSafe = neighLane.getVehicleMaxSpeed(&myVehicle);
    //     double thisLaneVSafe = myVehicle.getLane()->getVehicleMaxSpeed(&myVehicle);
    //     if (neighLead.first == 0) {
    //         neighLaneVSafe = MIN2(neighLaneVSafe, getCarFollowModel().followSpeed(&myVehicle, myVehicle.getSpeed(), neighDist, 0, 0)); // !!! warum nicht die Folgesgeschw.?
    //     } else {
    //         // @todo: what if leader is below safe gap?!!!
    //         neighLaneVSafe = MIN2(neighLaneVSafe, getCarFollowModel().followSpeed(&myVehicle, myVehicle.getSpeed(), neighLead.second, neighLead.first->getSpeed(), neighLead.first->getCarFollowModel().getMaxDecel()));
    //     }
    //     if (leader.first == 0) {
    //         thisLaneVSafe = MIN2(thisLaneVSafe, getCarFollowModel().followSpeed(&myVehicle, myVehicle.getSpeed(), currentDist, 0, 0));
    //     } else {
    //         // @todo: what if leader is below safe gap?!!!
    //         thisLaneVSafe = MIN2(thisLaneVSafe, getCarFollowModel().followSpeed(&myVehicle, myVehicle.getSpeed(), leader.second, leader.first->getSpeed(), leader.first->getCarFollowModel().getMaxDecel()));
    //     }
    //     thisLaneVSafe = MIN2(thisLaneVSafe, myVehicle.getMaxSpeed());
    //     neighLaneVSafe = MIN2(neighLaneVSafe, myVehicle.getMaxSpeed());
    //     if (thisLaneVSafe > neighLaneVSafe) {
    //         // this lane is better
    //         if (myChangeProbability > 0) {
    //             myChangeProbability *= pow(0.5, TS);
    //         }
    //     } else {
    //         // right lane is better
    //         myChangeProbability += TS * ((neighLaneVSafe - thisLaneVSafe) / (myVehicle.getLane()->getVehicleMaxSpeed(&myVehicle))); // !!! Fahrzeuggeschw.!
    //     }
    //     if (myChangeProbability > .2 && neighDist / MAX2((double) .1, myVehicle.getSpeed()) > 20.) { // .1
    //         return ret | LCA_LEFT | LCA_SPEEDGAIN | LCA_URGENT;
    //     }
    //     // --------
    //
    //     return ret;
    // }
    #[allow(unused_variables)]
    pub fn wants_change_to_left(&mut self, msgPass: &mut MSAbstractLaneChangeModel_MSLCMessager, blocked: i32, leader: &(Ptr<MSVehicle>, f64), neighLead: &(Ptr<MSVehicle>, f64), neighFollow: &(Ptr<MSVehicle>, f64), neighLane: &MSLane, preb: &Vec<MSVehicle_LaneQ>, lastBlocked: Ptr<Ptr<MSVehicle>>, firstBlocked: Ptr<Ptr<MSVehicle>>) -> i32 {
        // UNUSED_PARAMETER(firstBlocked);
        let mut blocked = blocked;
        let myVehicle = self.myVehicle;
        let mut curr = lane_q_default();
        let mut best = lane_q_default();
        let mut bestLaneOffset: i32 = 0;
        let mut currentDist: f64 = 0.;
        let mut neighDist: f64 = 0.;
        let mut neighExtDist: f64 = 0.;
        let mut currExtDist: f64 = 0.;
        let mut currIdx: i32 = 0;
        let mut prebLane = myVehicle.get_lane();
        if prebLane.get_edge().is_internal() {
            // internal edges are not kept inside the bestLanes structure
            prebLane = prebLane.get_link_cont()[0].get_lane();
        }
        for p in 0..preb.len() {
            if preb[p].lane == prebLane {
                curr = lane_q_copy(&preb[p]);
                bestLaneOffset = curr.bestLaneOffset;
                currentDist = curr.length;
                currExtDist = curr.lane.get_length();
                neighDist = preb[p + 1].length;
                neighExtDist = preb[p + 1].lane.get_length();
                best = lane_q_copy(&preb[(p as i32 + bestLaneOffset) as usize]);
                currIdx = p as i32;
                break;
            }
        }
        // keep information about being a leader/follower
        let mut ret: i32 = self.myOwnState & (0xffff0000u32 as i32);

        // ?!!!
        if !leader.0.is_null()
            && (self.myOwnState & LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0
            && (leader.0.m().get_lane_change_model().get_own_state() & LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0
        {
            self.myOwnState &= !LCA_AMBLOCKINGFOLLOWER_DONTBRAKE;
            if myVehicle.get_speed() > SUMO_const_haltingSpeed {
                self.myOwnState |= LCA_AMBACKBLOCKER;
            } else {
                ret |= LCA_AMBACKBLOCKER;
                self.myDontBrake = true;
            }
        }

        // process information about the last blocked vehicle
        //  if this vehicle is blocking someone in front, we maybe decelerate to let him in
        if !(*lastBlocked).is_null() {
            let lb = *lastBlocked;
            let gap = lb.get_position_on_lane() - lb.get_vehicle_type().get_length() - myVehicle.get_position_on_lane() - myVehicle.get_vehicle_type().get_min_gap();
            if gap > 0.1 {
                if myVehicle.get_speed() < accel2speed(myVehicle.get_car_follow_model().get_max_decel()) {
                    if lb.get_speed() < SUMO_const_haltingSpeed {
                        ret |= LCA_AMBACKBLOCKER_STANDING;
                    } else {
                        ret |= LCA_AMBACKBLOCKER;
                    }
                    let vsafe = myVehicle.get_car_follow_model().follow_speed(
                        myVehicle,
                        myVehicle.get_speed(),
                        gap - 0.1,
                        lb.get_speed(),
                        lb.get_car_follow_model().get_max_decel(),
                        Ptr::null(),
                        MSCFModel_CalcReason::CURRENT,
                    );
                    self.myVSafes.push(vsafe);
                    *lastBlocked.m() = Ptr::null();
                }
                return ret;
            }
        }

        // we try to estimate the distance which is necessary to get on a lane
        //  we have to get on in order to keep our route
        // we assume we need something that depends on our velocity
        // and compare this with the free space on our wished lane
        //
        // if the free space is somehow less than the space we need, we should
        //  definitely try to get to the desired lane
        //
        // this rule forces our vehicle to change the lane if a lane changing is necessary soon
        let mut lv = if myVehicle.get_speed() > LOOK_FORWARD_SPEED_DIVIDER {
            myVehicle.get_speed() * LOOK_FORWARD_FAR
        } else {
            myVehicle.get_speed() * LOOK_FORWARD_NEAR
        };
        lv += myVehicle.get_vehicle_type().get_length_with_gap() * 2.;

        let tdist = currentDist - myVehicle.get_position_on_lane() - best.occupation * JAM_FACTOR2;
        if (best.length - curr.length).abs() > min2(0.1, best.lane.get_length()) && bestLaneOffset > 0 && self.current_dist_disallows(tdist, bestLaneOffset, lv) {
            self.inform_blocker(msgPass, &mut blocked, LCA_MLEFT, neighLead, neighFollow);
            if neighLead.1 > 0. && neighLead.1 > leader.1 {
                let vsafe = myVehicle.get_car_follow_model().follow_speed(
                    myVehicle,
                    myVehicle.get_speed(),
                    neighLead.1,
                    neighLead.0.get_speed(),
                    neighLead.0.get_car_follow_model().get_max_decel(),
                    Ptr::null(),
                    MSCFModel_CalcReason::CURRENT,
                ) - 0.5;
                self.myVSafes.push(vsafe);
            }

            // letting vehicles merge in at the end of the lane in case of counter-lane change, step#1, left
            //   if there is a leader and he wants to change to right (we want to change to left)
            if !neighLead.0.is_null() && (neighLead.0.m().get_lane_change_model().get_own_state() & LCA_RIGHT) != 0 {
                // save at least his length in myLeadingBlockerLength
                self.myLeadingBlockerLength = max2(neighLead.0.get_vehicle_type().get_length_with_gap(), self.myLeadingBlockerLength);
                // save the left space
                self.myLeftSpace = currentDist - myVehicle.get_position_on_lane();
            }

            return ret | LCA_LEFT | LCA_STRATEGIC | LCA_URGENT;
        }

        // the opposite lane-changing direction should be rather done, not
        //  the one examined herein
        //  we'll check whether we assume we could change anyhow and get back in time...
        //
        // this rule prevents the vehicle from moving in opposite direction of the best lane
        //  unless the way till the end where the vehicle has to be on the best lane
        //  is long enough
        let maxJam = max2(preb[(currIdx + 1) as usize].occupation, preb[currIdx as usize].occupation);
        let neighLeftPlace = max2(0., neighDist - myVehicle.get_position_on_lane() - maxJam);
        if bestLaneOffset <= 0 && self.current_dist_disallows(neighLeftPlace, bestLaneOffset - 2, lv) {
            // ...we will not change the lane if not
            return ret | LCA_STAY | LCA_STRATEGIC;
        }

        // if the current lane is the best and a lane-changing would cause a situation
        //  of which we assume we will not be able to return to the lane we have to be on...
        //
        // this rule prevents the vehicle from leaving the current, best lane when it is
        //  close to this lane's end
        if currExtDist > neighExtDist && (neighLeftPlace * 2. < lv) {
            // ... let's not change the lane
            return ret | LCA_STAY | LCA_STRATEGIC;
        }
        // --------

        // -------- make place on current lane if blocking follower
        if self.am_blocking_follower_plus_nb() && (self.current_dist_allows(neighDist, bestLaneOffset, lv) || neighDist >= currentDist) {
            return ret | LCA_LEFT | LCA_COOPERATIVE | LCA_URGENT;
        }
        // --------

        // -------- security checks for krauss
        //  (vsafe fails when gap<0)
        if (blocked & LCA_BLOCKED) != 0 {
            return ret;
        }

        // -------- higher speed
        if (self.congested(neighLead.0) && neighLead.1 < 20.) || self.pred_interaction(leader) {
            // !!!
            return ret;
        }
        let mut neighLaneVSafe = neighLane.get_vehicle_max_speed(traffic_object_of(myVehicle));
        let mut thisLaneVSafe = myVehicle.get_lane().get_vehicle_max_speed(traffic_object_of(myVehicle));
        if neighLead.0.is_null() {
            // !!! warum nicht die Folgesgeschw.?
            neighLaneVSafe = min2(
                neighLaneVSafe,
                myVehicle.get_car_follow_model().follow_speed(myVehicle, myVehicle.get_speed(), neighDist, 0., 0., Ptr::null(), MSCFModel_CalcReason::CURRENT),
            );
        } else {
            // @todo: what if leader is below safe gap?!!!
            neighLaneVSafe = min2(
                neighLaneVSafe,
                myVehicle.get_car_follow_model().follow_speed(
                    myVehicle,
                    myVehicle.get_speed(),
                    neighLead.1,
                    neighLead.0.get_speed(),
                    neighLead.0.get_car_follow_model().get_max_decel(),
                    Ptr::null(),
                    MSCFModel_CalcReason::CURRENT,
                ),
            );
        }
        if leader.0.is_null() {
            thisLaneVSafe = min2(
                thisLaneVSafe,
                myVehicle.get_car_follow_model().follow_speed(myVehicle, myVehicle.get_speed(), currentDist, 0., 0., Ptr::null(), MSCFModel_CalcReason::CURRENT),
            );
        } else {
            // @todo: what if leader is below safe gap?!!!
            thisLaneVSafe = min2(
                thisLaneVSafe,
                myVehicle.get_car_follow_model().follow_speed(
                    myVehicle,
                    myVehicle.get_speed(),
                    leader.1,
                    leader.0.get_speed(),
                    leader.0.get_car_follow_model().get_max_decel(),
                    Ptr::null(),
                    MSCFModel_CalcReason::CURRENT,
                ),
            );
        }
        thisLaneVSafe = min2(thisLaneVSafe, myVehicle.get_max_speed());
        neighLaneVSafe = min2(neighLaneVSafe, myVehicle.get_max_speed());
        if thisLaneVSafe > neighLaneVSafe {
            // this lane is better
            if self.myChangeProbability > 0. {
                self.myChangeProbability *= 0.5f64.powf(ts());
            }
        } else {
            // right lane is better
            // !!! Fahrzeuggeschw.!
            self.myChangeProbability += ts() * ((neighLaneVSafe - thisLaneVSafe) / (myVehicle.get_lane().get_vehicle_max_speed(traffic_object_of(myVehicle))));
        }
        if self.myChangeProbability > 0.2 && neighDist / max2(0.1, myVehicle.get_speed()) > 20. {
            // .1
            return ret | LCA_LEFT | LCA_SPEEDGAIN | LCA_URGENT;
        }
        // --------

        ret
    }

    /// C++: `void MSLCM_DK2008::informBlocker(MSAbstractLaneChangeModel::MSLCMessager &, int &, int, const std::pair<MSVehicle *, double> &, const std::pair<MSVehicle *, double> &)` [protected]
    // void
    // MSLCM_DK2008::informBlocker(MSAbstractLaneChangeModel::MSLCMessager& msgPass,
    //                             int& blocked,
    //                             int dir,
    //                             const std::pair<MSVehicle*, double>& neighLead,
    //                             const std::pair<MSVehicle*, double>& neighFollow) {
    //     if ((blocked & LCA_BLOCKED_BY_FOLLOWER) != 0) {
    //         assert(neighFollow.first != 0);
    //         MSVehicle* nv = neighFollow.first;
    //         double decelGap =
    //             neighFollow.second
    //             + SPEED2DIST(myVehicle.getSpeed()) * (double) 2.0
    //             - MAX2(nv->getSpeed() - (double) ACCEL2DIST(nv->getCarFollowModel().getMaxDecel()) * (double) 2.0, (double) 0);
    //         if (neighFollow.second > 0 && decelGap > 0 && decelGap >= nv->getCarFollowModel().getSecureGap(nv, &myVehicle,
    //                 nv->getSpeed(), myVehicle.getSpeed(), myVehicle.getCarFollowModel().getMaxDecel())) {
    //             double vsafe = getCarFollowModel().followSpeed(&myVehicle, myVehicle.getSpeed(), neighFollow.second, neighFollow.first->getSpeed(), neighFollow.first->getCarFollowModel().getMaxDecel());
    //             msgPass.informNeighFollower(new Info(vsafe, dir | LCA_AMBLOCKINGFOLLOWER), &myVehicle);
    //         } else {
    //             double vsafe = neighFollow.second <= 0 ? 0 : getCarFollowModel().followSpeed(&myVehicle, myVehicle.getSpeed(), neighFollow.second, neighFollow.first->getSpeed(), neighFollow.first->getCarFollowModel().getMaxDecel());
    //             msgPass.informNeighFollower(new Info(vsafe, dir | LCA_AMBLOCKINGFOLLOWER_DONTBRAKE), &myVehicle);
    //         }
    //     }
    //     if ((blocked & LCA_BLOCKED_BY_LEADER) != 0) {
    //         if (neighLead.first != 0 && neighLead.second > 0) {
    //             msgPass.informNeighLeader(new Info(0, dir | LCA_AMBLOCKINGLEADER), &myVehicle);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn inform_blocker(&mut self, msgPass: &mut MSAbstractLaneChangeModel_MSLCMessager, blocked: &mut i32, dir: i32, neighLead: &(Ptr<MSVehicle>, f64), neighFollow: &(Ptr<MSVehicle>, f64)) {
        let myVehicle = self.myVehicle;
        if (*blocked & LCA_BLOCKED_BY_FOLLOWER) != 0 {
            debug_assert!(!neighFollow.0.is_null());
            let nv = neighFollow.0;
            let decelGap = neighFollow.1 + speed2dist(myVehicle.get_speed()) * 2.0
                - max2(nv.get_speed() - accel2dist(nv.get_car_follow_model().get_max_decel()) * 2.0, 0.);
            if neighFollow.1 > 0. && decelGap > 0.
                && decelGap >= nv.get_car_follow_model().get_secure_gap(nv, myVehicle, nv.get_speed(), myVehicle.get_speed(), myVehicle.get_car_follow_model().get_max_decel())
            {
                let vsafe = myVehicle.get_car_follow_model().follow_speed(
                    myVehicle,
                    myVehicle.get_speed(),
                    neighFollow.1,
                    neighFollow.0.get_speed(),
                    neighFollow.0.get_car_follow_model().get_max_decel(),
                    Ptr::null(),
                    MSCFModel_CalcReason::CURRENT,
                );
                msgPass.inform_neigh_follower(new_info((vsafe, dir | LCA_AMBLOCKINGFOLLOWER)), myVehicle);
            } else {
                let vsafe = if neighFollow.1 <= 0. {
                    0.
                } else {
                    myVehicle.get_car_follow_model().follow_speed(
                        myVehicle,
                        myVehicle.get_speed(),
                        neighFollow.1,
                        neighFollow.0.get_speed(),
                        neighFollow.0.get_car_follow_model().get_max_decel(),
                        Ptr::null(),
                        MSCFModel_CalcReason::CURRENT,
                    )
                };
                msgPass.inform_neigh_follower(new_info((vsafe, dir | LCA_AMBLOCKINGFOLLOWER_DONTBRAKE)), myVehicle);
            }
        }
        if (*blocked & LCA_BLOCKED_BY_LEADER) != 0 {
            if !neighLead.0.is_null() && neighLead.1 > 0. {
                msgPass.inform_neigh_leader(new_info((0., dir | LCA_AMBLOCKINGLEADER)), myVehicle);
            }
        }
    }

    /// C++: `bool MSLCM_DK2008::amBlockingLeader()` [protected]
    // inline bool amBlockingLeader() {
    //         return (myOwnState & LCA_AMBLOCKINGLEADER) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn am_blocking_leader(&mut self) -> bool {
        (self.myOwnState & LCA_AMBLOCKINGLEADER) != 0
    }

    /// C++: `bool MSLCM_DK2008::amBlockingFollower()` [protected]
    // inline bool amBlockingFollower() {
    //         return (myOwnState & LCA_AMBLOCKINGFOLLOWER) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn am_blocking_follower(&mut self) -> bool {
        (self.myOwnState & LCA_AMBLOCKINGFOLLOWER) != 0
    }

    /// C++: `bool MSLCM_DK2008::amBlockingFollowerNB()` [protected]
    // inline bool amBlockingFollowerNB() {
    //         return (myOwnState & LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn am_blocking_follower_nb(&mut self) -> bool {
        (self.myOwnState & LCA_AMBLOCKINGFOLLOWER_DONTBRAKE) != 0
    }

    /// C++: `bool MSLCM_DK2008::amBlockingFollowerPlusNB()` [protected]
    // inline bool amBlockingFollowerPlusNB() {
    //         return (myOwnState & (LCA_AMBLOCKINGFOLLOWER | LCA_AMBLOCKINGFOLLOWER_DONTBRAKE)) != 0;
    //     }
    #[allow(unused_variables)]
    pub fn am_blocking_follower_plus_nb(&mut self) -> bool {
        (self.myOwnState & (LCA_AMBLOCKINGFOLLOWER | LCA_AMBLOCKINGFOLLOWER_DONTBRAKE)) != 0
    }

    /// C++: `bool MSLCM_DK2008::currentDistDisallows(double, int, double)` [protected]
    // inline bool currentDistDisallows(double dist, int laneOffset, double lookForwardDist) {
    //         return dist / (abs(laneOffset)) < lookForwardDist;
    //     }
    #[allow(unused_variables)]
    pub fn current_dist_disallows(&mut self, dist: f64, laneOffset: i32, lookForwardDist: f64) -> bool {
        dist / (laneOffset.abs() as f64) < lookForwardDist
    }

    /// C++: `bool MSLCM_DK2008::currentDistAllows(double, int, double)` [protected]
    // inline bool currentDistAllows(double dist, int laneOffset, double lookForwardDist) {
    //         return dist / abs(laneOffset) > lookForwardDist;
    //     }
    #[allow(unused_variables)]
    pub fn current_dist_allows(&mut self, dist: f64, laneOffset: i32, lookForwardDist: f64) -> bool {
        dist / (laneOffset.abs() as f64) > lookForwardDist
    }

}

