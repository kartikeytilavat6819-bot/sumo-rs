//! Port of microsim/lcmodels/MSAbstractLaneChangeModel.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Virtual dispatch table for the polymorphic methods of MSAbstractLaneChangeModel.
/// Each subclass constructor installs its own table in `vt`; the base methods forward through it.
pub struct MSAbstractLaneChangeModelVt {
    pub inform: fn(&mut MSAbstractLaneChangeModel, Ptr<()>, Ptr<MSVehicle>) -> Ptr<()>,
    pub patch_speed: fn(&mut MSAbstractLaneChangeModel, f64, f64, f64, &MSCFModel) -> f64,
    pub changed: fn(&mut MSAbstractLaneChangeModel),
    pub wants_change: fn(&mut MSAbstractLaneChangeModel, i32, &mut MSAbstractLaneChangeModel_MSLCMessager, i32, &(Ptr<MSVehicle>, f64), &(Ptr<MSVehicle>, f64), &(Ptr<MSVehicle>, f64), &(Ptr<MSVehicle>, f64), &MSLane, &Vec<MSVehicle_LaneQ>, Ptr<Ptr<MSVehicle>>, Ptr<Ptr<MSVehicle>>) -> i32,
    pub wants_change_sublane: fn(&mut MSAbstractLaneChangeModel, i32, LaneChangeAction, &MSLeaderDistanceInfo, &MSLeaderDistanceInfo, &MSLeaderDistanceInfo, &MSLeaderDistanceInfo, &MSLeaderDistanceInfo, &MSLeaderDistanceInfo, &MSLane, &Vec<MSVehicle_LaneQ>, Ptr<Ptr<MSVehicle>>, Ptr<Ptr<MSVehicle>>, &mut f64, &mut f64, &mut i32) -> i32,
    pub update_expected_sublane_speeds: fn(&mut MSAbstractLaneChangeModel, &MSLeaderDistanceInfo, i32, i32),
    pub decide_direction: fn(&MSAbstractLaneChangeModel, MSAbstractLaneChangeModel_StateAndDist, MSAbstractLaneChangeModel_StateAndDist) -> MSAbstractLaneChangeModel_StateAndDist,
    pub set_own_state: fn(&mut MSAbstractLaneChangeModel, i32),
    pub prepare_step: fn(&mut MSAbstractLaneChangeModel),
    pub reset_speed_lat: fn(&mut MSAbstractLaneChangeModel),
    pub update_safe_lat_dist: fn(&mut MSAbstractLaneChangeModel, f64),
    pub get_safety_factor: fn(&MSAbstractLaneChangeModel) -> f64,
    pub compute_speed_lat: fn(&MSAbstractLaneChangeModel, f64, &mut f64, bool) -> f64,
    pub save_blocker_length: fn(&mut MSAbstractLaneChangeModel, f64, f64) -> bool,
    pub get_extra_reservation: fn(&MSAbstractLaneChangeModel, i32, f64) -> f64,
    pub set_parameter: fn(&mut MSAbstractLaneChangeModel, &str, &str),
    pub avoid_overtake_right: fn(&MSAbstractLaneChangeModel, Ptr<MSVehicle>, bool) -> bool,
    pub save_state: fn(&MSAbstractLaneChangeModel, &mut OutputDevice),
    pub load_state: fn(&mut MSAbstractLaneChangeModel, &SUMOSAXAttributes),
    pub reset_state: fn(&mut MSAbstractLaneChangeModel),
    pub get_opposite_safety_factor: fn(&MSAbstractLaneChangeModel) -> f64,
    pub debug_vehicle: fn(&MSAbstractLaneChangeModel) -> bool,
    pub get_parameter: fn(&MSAbstractLaneChangeModel, &str) -> String,
    pub get_desired_alignment: fn(&MSAbstractLaneChangeModel) -> LatAlignmentDefinition,
    pub check_change_before_committing: fn(&MSAbstractLaneChangeModel, Ptr<MSVehicle>, i32) -> i32,
}

fn default_inform(_: &mut MSAbstractLaneChangeModel, _: Ptr<()>, _: Ptr<MSVehicle>) -> Ptr<()> {
    panic!("MSAbstractLaneChangeModel::inform is pure virtual")
}

fn default_patch_speed(_: &mut MSAbstractLaneChangeModel, _: f64, _: f64, _: f64, _: &MSCFModel) -> f64 {
    panic!("MSAbstractLaneChangeModel::patchSpeed is pure virtual")
}

fn default_changed(_: &mut MSAbstractLaneChangeModel) {
    panic!("MSAbstractLaneChangeModel::changed is pure virtual")
}

static DEFAULT_MS_ABSTRACT_LANE_CHANGE_MODEL_VT: MSAbstractLaneChangeModelVt = MSAbstractLaneChangeModelVt {
    inform: default_inform,
    patch_speed: default_patch_speed,
    changed: default_changed,
    wants_change: MSAbstractLaneChangeModel::wants_change_base,
    wants_change_sublane: MSAbstractLaneChangeModel::wants_change_sublane_base,
    update_expected_sublane_speeds: MSAbstractLaneChangeModel::update_expected_sublane_speeds_base,
    decide_direction: MSAbstractLaneChangeModel::decide_direction_base,
    set_own_state: MSAbstractLaneChangeModel::set_own_state_base,
    prepare_step: MSAbstractLaneChangeModel::prepare_step_base,
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

/// Interface for lane-change models
/// C++ class `MSAbstractLaneChangeModel` (microsim/lcmodels/MSAbstractLaneChangeModel.h)
pub struct MSAbstractLaneChangeModel {
    /// Virtual dispatch table; first field because this root class has no base
    pub vt: &'static MSAbstractLaneChangeModelVt,
    /// The vehicle this lane-changer belongs to
    pub myVehicle: Ptr<MSVehicle>,
    /// The current state of the vehicle
    pub myOwnState: i32,
    /// lane changing state from the previous simulation step
    pub myPreviousState: i32,
    /// lane changing state from step before the previous simulation step
    pub myPreviousState2: i32,
    pub mySavedStateRight: (i32, i32),
    pub mySavedStateCenter: (i32, i32),
    pub mySavedStateLeft: (i32, i32),
    pub myCanceledStateRight: i32,
    pub myCanceledStateCenter: i32,
    pub myCanceledStateLeft: i32,
    /// Cached info on lc-relevant neighboring vehicles @{
    /// None models a C++ nullptr shared_ptr
    pub myLeftFollowers: Option<Rc<MSLeaderDistanceInfo>>,
    pub myLeftLeaders: Option<Rc<MSLeaderDistanceInfo>>,
    pub myRightFollowers: Option<Rc<MSLeaderDistanceInfo>>,
    pub myRightLeaders: Option<Rc<MSLeaderDistanceInfo>>,
    /// the current lateral speed
    pub mySpeedLat: f64,
    /// the current lateral acceleration
    pub myAccelerationLat: f64,
    /// the current angle offset resulting from lane change and sigma
    pub myAngleOffset: f64,
    /// the angle offset of the previous time step resulting from lane change and sigma
    pub myPreviousAngleOffset: f64,
    /// the speed when committing to a change maneuver
    pub myCommittedSpeed: f64,
    /// progress of the lane change maneuver 0:started, 1:complete
    pub myLaneChangeCompletion: f64,
    /// direction of the lane change maneuver -1 means right, 1 means left
    pub myLaneChangeDirection: i32,
    /// whether the vehicle has already moved this step
    pub myAlreadyChanged: bool,
    /// A lane that is partially occupied by the front of the vehicle but that is not the primary lane
    pub myShadowLane: Ptr<MSLane>,
    pub myShadowFurtherLanes: Vec<Ptr<MSLane>>,
    pub myShadowFurtherLanesPosLat: Vec<f64>,
    /// The target lane for the vehicle's current maneuver
    pub myTargetLane: Ptr<MSLane>,
    pub myFurtherTargetLanes: Vec<Ptr<MSLane>>,
    /// the type of this model
    pub myModel: LaneChangeModel,
    /// list of lanes where the shadow vehicle is partial occupator
    pub myPartiallyOccupatedByShadow: Vec<Ptr<MSLane>>,
    pub myNoPartiallyOccupatedByShadow: Vec<Ptr<MSLane>>,
    /// the minimum lateral gaps to other vehicles that were found when last changing to the left and right
    pub myLastLateralGapLeft: f64,
    pub myLastLateralGapRight: f64,
    /// the actual minimum longitudinal distances to vehicles on the target lane
    pub myLastLeaderGap: f64,
    pub myLastFollowerGap: f64,
    /// the minimum longitudinal distances to vehicles on the target lane that would be necessary for stringent security
    pub myLastLeaderSecureGap: f64,
    pub myLastFollowerSecureGap: f64,
    /// acutal and secure distance to closest leader vehicle on the original when performing lane change
    pub myLastOrigLeaderGap: f64,
    pub myLastOrigLeaderSecureGap: f64,
    /// speeds of surrounding vehicles at the time of lane change
    pub myLastLeaderSpeed: f64,
    pub myLastFollowerSpeed: f64,
    pub myLastOrigLeaderSpeed: f64,
    /// Flag to prevent resetting the memorized values for LC relevant gaps until the LC output is triggered in the case of continuous LC.
    pub myDontResetLCGaps: bool,
    pub myStrategicLookahead: f64,
    pub myMaxSpeedLatStanding: f64,
    pub myMaxSpeedLatFactor: f64,
    pub myMaxDistLatStanding: f64,
    pub mySigma: f64,
    pub myOvertakeRightParam: f64,
    pub myAssertive: f64,
    pub myCooperativeHelpTime: SUMOTime,
    pub myCooperativeHelpThreshold: f64,
    pub myCooperativeMinSpeed: f64,
    /// whether this vehicle is driving with special permissions and behavior
    pub myHaveBlueLight: bool,
    pub myLCAccelerationAdvices: Vec<(f64, i32)>,
    pub myLastLaneChangeOffset: SUMOTime,
    /// links which are approached by the shadow vehicle
    pub myApproachedByShadow: std::cell::RefCell<Vec<Ptr<MSLink>>>,
    /// whether the vehicle is driving in the opposite direction
    pub myAmOpposite: bool,
    /// The complete lateral distance the vehicle wants to travel to finish its maneuver Only used by sublane model, currently.
    pub myManeuverDist: f64,
    /// Maneuver distance from the previous simulation step
    pub myPreviousManeuverDist: f64,
}

// C++ static members of MSAbstractLaneChangeModel (port as module-level statics, see AGENTS.md):
//   static const double MSAbstractLaneChangeModel::NO_NEIGHBOR;
//   static const double MSAbstractLaneChangeModel::UNDEFINED_LOOKAHEAD;
//   static bool MSAbstractLaneChangeModel::myAllowOvertakingRight;
//   static bool MSAbstractLaneChangeModel::myLCOutput;
//   static bool MSAbstractLaneChangeModel::myLCStartedOutput;
//   static bool MSAbstractLaneChangeModel::myLCEndedOutput;
//   static bool MSAbstractLaneChangeModel::myLCXYOutput;

use std::sync::atomic::{AtomicBool, Ordering};
use utils::common::time::{delta_t, dist2speed, speed2accel, speed2dist, steps2time, time2steps};
use utils::geom::NUMERICAL_EPS;
use utils::common::sumo_vehicle_class::SVC_EMERGENCY;
use utils::common::to_string::f64_to_string;
use utils::options::options_cont::{get_bool, is_set};

/// C++ SUMO_const_haltingSpeed (utils/common/StdDefs.h)
const SUMO_const_haltingSpeed: f64 = 0.1;
/// C++ VTYPEPARS_MAXSPEED_LAT_SET (utils/vehicle/SUMOVTypeParameter.h), private in ms_vehicle_type.rs
const VTYPEPARS_MAXSPEED_LAT_SET: i64 = 1 << 20;

/// C++ MSAbstractLaneChangeModel::NO_NEIGHBOR (std::numeric_limits<double>::max())
pub const NO_NEIGHBOR: f64 = f64::MAX;
/// C++ MSAbstractLaneChangeModel::UNDEFINED_LOOKAHEAD
pub const UNDEFINED_LOOKAHEAD: f64 = -1.0;
/// C++ MSAbstractLaneChangeModel::myAllowOvertakingRight
pub static MY_ALLOW_OVERTAKING_RIGHT: AtomicBool = AtomicBool::new(false);
/// C++ MSAbstractLaneChangeModel::myLCOutput
pub static MY_LC_OUTPUT: AtomicBool = AtomicBool::new(false);
/// C++ MSAbstractLaneChangeModel::myLCStartedOutput
pub static MY_LC_STARTED_OUTPUT: AtomicBool = AtomicBool::new(false);
/// C++ MSAbstractLaneChangeModel::myLCEndedOutput
pub static MY_LC_ENDED_OUTPUT: AtomicBool = AtomicBool::new(false);
/// C++ MSAbstractLaneChangeModel::myLCXYOutput
pub static MY_LC_XY_OUTPUT: AtomicBool = AtomicBool::new(false);

impl MSAbstractLaneChangeModel {
    /// C++: `void MSAbstractLaneChangeModel::initGlobalOptions(const OptionsCont &)` [static]
    /// init global model parameters
    // void
    // MSAbstractLaneChangeModel::initGlobalOptions(const OptionsCont& oc) {
    //     myAllowOvertakingRight = oc.getBool("lanechange.overtake-right");
    //     myLCOutput = oc.isSet("lanechange-output");
    //     myLCStartedOutput = oc.getBool("lanechange-output.started");
    //     myLCEndedOutput = oc.getBool("lanechange-output.ended");
    //     myLCXYOutput = oc.getBool("lanechange-output.xy");
    // }
    #[allow(unused_variables)]
    pub fn init_global_options(oc: &OptionsCont) {
        MY_ALLOW_OVERTAKING_RIGHT.store(get_bool("lanechange.overtake-right"), Ordering::Relaxed);
        MY_LC_OUTPUT.store(is_set("lanechange-output"), Ordering::Relaxed);
        MY_LC_STARTED_OUTPUT.store(get_bool("lanechange-output.started"), Ordering::Relaxed);
        MY_LC_ENDED_OUTPUT.store(get_bool("lanechange-output.ended"), Ordering::Relaxed);
        MY_LC_XY_OUTPUT.store(get_bool("lanechange-output.xy"), Ordering::Relaxed);
    }

    /// C++: `MSAbstractLaneChangeModel * MSAbstractLaneChangeModel::build(LaneChangeModel, MSVehicle &)` [static]
    /// Factory method for instantiating new lane changing models
    // MSAbstractLaneChangeModel*
    // MSAbstractLaneChangeModel::build(LaneChangeModel lcm, MSVehicle& v) {
    //     if (MSGlobals::gLateralResolution > 0 && lcm != LaneChangeModel::SL2015 && lcm != LaneChangeModel::DEFAULT) {
    //         throw ProcessError(TLF("Lane change model '%' is not compatible with sublane simulation", toString(lcm)));
    //     }
    //     switch (lcm) {
    //         case LaneChangeModel::DK2008:
    //             return new MSLCM_DK2008(v);
    //         case LaneChangeModel::LC2013:
    //             return new MSLCM_LC2013(v);
    //         case LaneChangeModel::LC2013_CC:
    //             return new MSLCM_LC2013_CC(v);
    //         case LaneChangeModel::SL2015:
    //             return new MSLCM_SL2015(v);
    //         case LaneChangeModel::DEFAULT:
    //             if (MSGlobals::gLateralResolution <= 0) {
    //                 return new MSLCM_LC2013(v);
    //             } else {
    //                 return new MSLCM_SL2015(v);
    //             }
    //         default:
    //             throw ProcessError(TLF("Lane change model '%' not implemented", toString(lcm)));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build(lcm: LaneChangeModel, vehicle: &mut MSVehicle) -> Ptr<MSAbstractLaneChangeModel> {
        if MSGlobals::lateral_resolution() > 0. && lcm != LaneChangeModel::SL2015 && lcm != LaneChangeModel::DEFAULT {
            panic!(
                "{}",
                ProcessError::Process(utils::tlf!(
                    "Lane change model '%' is not compatible with sublane simulation",
                    LaneChangeModels.get_string(lcm)
                ))
            );
        }
        // C++ `new` yields an owned pointer; the object is leaked into a raw pointer like C++ ownership.
        // The base pointer is taken from the `base` field, so no layout assumption is needed.
        match lcm {
            LaneChangeModel::DK2008 => {
                let raw: *mut MSLCM_DK2008 = Box::into_raw(Box::new(MSLCM_DK2008::new(vehicle)));
                Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) })
            }
            LaneChangeModel::LC2013 => {
                let raw: *mut MSLCM_LC2013 = Box::into_raw(Box::new(MSLCM_LC2013::new(vehicle)));
                Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) })
            }
            LaneChangeModel::LC2013_CC => {
                let raw: *mut MSLCM_LC2013_CC = Box::into_raw(Box::new(MSLCM_LC2013_CC::new(vehicle)));
                Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base.base) })
            }
            LaneChangeModel::SL2015 => {
                let raw: *mut MSLCM_SL2015 = Box::into_raw(Box::new(MSLCM_SL2015::new(vehicle)));
                Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) })
            }
            LaneChangeModel::DEFAULT => {
                if MSGlobals::lateral_resolution() <= 0. {
                    let raw: *mut MSLCM_LC2013 = Box::into_raw(Box::new(MSLCM_LC2013::new(vehicle)));
                    Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) })
                } else {
                    let raw: *mut MSLCM_SL2015 = Box::into_raw(Box::new(MSLCM_SL2015::new(vehicle)));
                    Ptr::from_raw(unsafe { std::ptr::addr_of_mut!((*raw).base) })
                }
            }
            _ => panic!(
                "{}",
                ProcessError::Process(utils::tlf!(
                    "Lane change model '%' not implemented",
                    LaneChangeModels.get_string(lcm)
                ))
            ),
        }
    }

    /// C++: `LaneChangeModel MSAbstractLaneChangeModel::getModelID() const` [const, virtual, pure virtual]
    /// Returns the model's ID;
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> LaneChangeModel {
        // pure virtual in C++; the stored model is what every subclass returns
        self.myModel
    }

    /// C++: `int MSAbstractLaneChangeModel::checkChangeBeforeCommitting(const MSVehicle *, int) const` [const, virtual]
    /// Informs the vehicle that it is about to be moved on an adjacent lane. The method can be used to re-evaluate the state of the vehicle and potentially abort the lane change. By default, if the method is not overridden by the lane change model implementation, nothing is altered and the vehicle will perform the lane change.
    // virtual int checkChangeBeforeCommitting(const MSVehicle* veh, int state) const {
    //         UNUSED_PARAMETER(veh);
    //         UNUSED_PARAMETER(state);
    //         return 0;
    //     }
    pub fn check_change_before_committing(&self, veh: Ptr<MSVehicle>, state: i32) -> i32 {
        (self.vt.check_change_before_committing)(self, veh, state)
    }

    #[allow(unused_variables)]
    pub fn check_change_before_committing_base(&self, veh: Ptr<MSVehicle>, state: i32) -> i32 {
        0
    }

    /// C++: `void MSAbstractLaneChangeModel::saveState(OutputDevice &) const` [const, virtual]
    /// Save the state of the laneChangeModel
    // void
    // MSAbstractLaneChangeModel::saveState(OutputDevice& out) const {
    //     std::vector<double> lcState;
    //     lcState.push_back((double)myOwnState);
    //     for (const auto& item : myLCAccelerationAdvices) {
    //         lcState.push_back(item.first);
    //         lcState.push_back((double)item.second);
    //     }
    //     out.writeAttr(SUMO_ATTR_LCSTATE_BASE, lcState);
    //
    //     if (MSGlobals::gLaneChangeDuration > 0) {
    //         out.writeAttr(SUMO_ATTR_LCSTATE, std::vector<double> {mySpeedLat, myLaneChangeCompletion, (double)myLaneChangeDirection});
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice) {
        (self.vt.save_state)(self, out)
    }

    pub fn save_state_base(&self, out: &mut OutputDevice) {
        let mut lc_state: Vec<f64> = vec![self.myOwnState as f64];
        for item in &self.myLCAccelerationAdvices {
            lc_state.push(item.0);
            lc_state.push(item.1 as f64);
        }
        out.write_attr(SUMO_ATTR_LCSTATE_BASE, &lc_state);

        if MSGlobals::lane_change_duration() > 0 {
            let lc = vec![self.mySpeedLat, self.myLaneChangeCompletion, self.myLaneChangeDirection as f64];
            out.write_attr(SUMO_ATTR_LCSTATE, &lc);
        }
    }

    /// C++: `void MSAbstractLaneChangeModel::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the laneChangeModel from the given attributes
    // void
    // MSAbstractLaneChangeModel::loadState(const SUMOSAXAttributes& attrs) {
    //     if (attrs.hasAttribute(SUMO_ATTR_LCSTATE_BASE)) {
    //         std::istringstream bis(attrs.getString(SUMO_ATTR_LCSTATE_BASE));
    //         double token;
    //         bis >> token;
    //         myOwnState = (int)token; // double is suffciently precise
    //         double prev = std::numeric_limits<double>::max();
    //         while (bis >> token) {
    //             if (prev != std::numeric_limits<double>::max()) {
    //                 myLCAccelerationAdvices.push_back(std::make_pair(prev, (int)token));
    //                 prev = std::numeric_limits<double>::max();
    //             }
    //             prev = token;
    //         }
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_LCSTATE)) {
    //         std::istringstream bis(attrs.getString(SUMO_ATTR_LCSTATE));
    //         bis >> mySpeedLat;
    //         bis >> myLaneChangeCompletion;
    //         bis >> myLaneChangeDirection;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        (self.vt.load_state)(self, attrs)
    }

    pub fn load_state_base(&mut self, attrs: &SUMOSAXAttributes) {
        if attrs.has_attribute(SUMO_ATTR_LCSTATE_BASE) {
            let s = attrs.get_string(SUMO_ATTR_LCSTATE_BASE);
            // istringstream >> double: a failed read yields 0 (C++11) and ends the stream
            let mut toks = s.split_whitespace().map_while(|t| t.parse::<f64>().ok());
            let token = toks.next().unwrap_or(0.0);
            self.myOwnState = token as i32; // double is sufficiently precise
            let mut prev = f64::MAX;
            for token in toks {
                if prev != f64::MAX {
                    self.myLCAccelerationAdvices.push((prev, token as i32));
                    prev = f64::MAX;
                }
                prev = token;
            }
        }
        if attrs.has_attribute(SUMO_ATTR_LCSTATE) {
            let s = attrs.get_string(SUMO_ATTR_LCSTATE);
            // istringstream >> semantics: a token that fails to parse sets its value to 0 and ends the stream
            let mut toks = s.split_whitespace();
            let mut ok = true;
            if let Some(t) = toks.next() {
                self.mySpeedLat = t.parse::<f64>().unwrap_or(0.0);
                ok = t.parse::<f64>().is_ok();
            }
            if ok {
                if let Some(t) = toks.next() {
                    self.myLaneChangeCompletion = t.parse::<f64>().unwrap_or(0.0);
                    ok = t.parse::<f64>().is_ok();
                }
            }
            if ok {
                if let Some(t) = toks.next() {
                    self.myLaneChangeDirection = t.parse::<i32>().unwrap_or(0);
                }
            }
        }
    }

    /// C++: `bool MSAbstractLaneChangeModel::haveLCOutput()` [static]
    /// whether lanechange-output is active
    // static bool haveLCOutput() {
    //         return myLCOutput;
    //     }
    #[allow(unused_variables)]
    pub fn have_lc_output() -> bool {
        MY_LC_OUTPUT.load(Ordering::Relaxed)
    }

    /// C++: `bool MSAbstractLaneChangeModel::enabledOutputLCStarted()` [static]
    /// whether start of maneuvers shall be recorede
    // static bool enabledOutputLCStarted() {
    //         return myLCStartedOutput;
    //     }
    #[allow(unused_variables)]
    pub fn enabled_output_lc_started() -> bool {
        MY_LC_STARTED_OUTPUT.load(Ordering::Relaxed)
    }

    /// C++: `bool MSAbstractLaneChangeModel::enabledOutputLCEnded()` [static]
    /// whether start of maneuvers shall be recorede
    // static bool enabledOutputLCEnded() {
    //         return myLCEndedOutput;
    //     }
    #[allow(unused_variables)]
    pub fn enabled_output_lc_ended() -> bool {
        MY_LC_ENDED_OUTPUT.load(Ordering::Relaxed)
    }

    /// C++: `void MSAbstractLaneChangeModel::MSAbstractLaneChangeModel(MSVehicle &, const LaneChangeModel)`
    /// Constructor
    // MSAbstractLaneChangeModel::MSAbstractLaneChangeModel(MSVehicle& v, const LaneChangeModel model) :
    //     myVehicle(v),
    //     myOwnState(0),
    //     myPreviousState(0),
    //     myPreviousState2(0),
    //     myCanceledStateRight(LCA_NONE),
    //     myCanceledStateCenter(LCA_NONE),
    //     myCanceledStateLeft(LCA_NONE),
    //     mySpeedLat(0),
    //     myAccelerationLat(0),
    //     myAngleOffset(0),
    //     myPreviousAngleOffset(0),
    //     myCommittedSpeed(0),
    //     myLaneChangeCompletion(1.0),
    //     myLaneChangeDirection(0),
    //     myAlreadyChanged(false),
    //     myShadowLane(nullptr),
    //     myTargetLane(nullptr),
    //     myModel(model),
    //     myLastLateralGapLeft(0.),
    //     myLastLateralGapRight(0.),
    //     myLastLeaderGap(0.),
    //     myLastFollowerGap(0.),
    //     myLastLeaderSecureGap(0.),
    //     myLastFollowerSecureGap(0.),
    //     myLastOrigLeaderGap(0.),
    //     myLastOrigLeaderSecureGap(0.),
    //     myLastLeaderSpeed(0),
    //     myLastFollowerSpeed(0),
    //     myLastOrigLeaderSpeed(0),
    //     myDontResetLCGaps(false),
    //     myStrategicLookahead(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_STRATEGIC_LOOKAHEAD, UNDEFINED_LOOKAHEAD)),
    //     myMaxSpeedLatStanding(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_MAXSPEEDLATSTANDING, v.getVehicleType().getMaxSpeedLat())),
    //     myMaxSpeedLatFactor(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_MAXSPEEDLATFACTOR, 1)),
    //     myMaxDistLatStanding(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_MAXDISTLATSTANDING,
    //                          // prevent lateral sliding for cars but permit for two-wheelers due to better maneuverability
    //                          (v.getVClass() & (SVC_BICYCLE | SVC_MOTORCYCLE | SVC_MOPED)) != 0 ? std::numeric_limits<double>::max() : 1.6)),
    //     mySigma(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_SIGMA, 0.0)),
    //     myOvertakeRightParam(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_OVERTAKE_RIGHT, 0)),
    //     myAssertive(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_ASSERTIVE, 1)),
    //     myCooperativeHelpTime(TIME2STEPS(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_COOPERATIVE_HELPTIME, 60))),
    //     myCooperativeHelpThreshold(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_COOPERATIVE_HELPTHRESHOLD, -1)),
    //     myCooperativeMinSpeed(v.getVehicleType().getParameter().getLCParam(SUMO_ATTR_LCA_COOPERATIVE_MINSPEED, 0)),
    //     myHaveBlueLight(v.getDevice(typeid(MSDevice_Bluelight)) != nullptr), // see MSVehicle::initDevices
    //     myLastLaneChangeOffset(0),
    //     myAmOpposite(false),
    //     myManeuverDist(0.),
    //     myPreviousManeuverDist(0.) {
    //     saveLCState(-1, LCA_UNKNOWN, LCA_UNKNOWN);
    //     saveLCState(0, LCA_UNKNOWN, LCA_UNKNOWN);
    //     saveLCState(1, LCA_UNKNOWN, LCA_UNKNOWN);
    // }
    #[allow(unused_variables)]
    pub fn new(v: &mut MSVehicle, model: LaneChangeModel) -> Self {
        let myVehicle = Ptr::from_mut(v);
        let vt = v.get_vehicle_type();
        let p = vt.get_parameter();
        let max_speed_lat = vt.get_max_speed_lat();
        let mut s = MSAbstractLaneChangeModel {
            vt: &DEFAULT_MS_ABSTRACT_LANE_CHANGE_MODEL_VT,
            myVehicle,
            myOwnState: 0,
            myPreviousState: 0,
            myPreviousState2: 0,
            mySavedStateRight: (0, 0),
            mySavedStateCenter: (0, 0),
            mySavedStateLeft: (0, 0),
            myCanceledStateRight: LCA_NONE,
            myCanceledStateCenter: LCA_NONE,
            myCanceledStateLeft: LCA_NONE,
            myLeftFollowers: None,
            myLeftLeaders: None,
            myRightFollowers: None,
            myRightLeaders: None,
            mySpeedLat: 0.,
            myAccelerationLat: 0.,
            myAngleOffset: 0.,
            myPreviousAngleOffset: 0.,
            myCommittedSpeed: 0.,
            myLaneChangeCompletion: 1.0,
            myLaneChangeDirection: 0,
            myAlreadyChanged: false,
            myShadowLane: Ptr::default(),
            myShadowFurtherLanes: Vec::new(),
            myShadowFurtherLanesPosLat: Vec::new(),
            myTargetLane: Ptr::default(),
            myFurtherTargetLanes: Vec::new(),
            myModel: model,
            myPartiallyOccupatedByShadow: Vec::new(),
            myNoPartiallyOccupatedByShadow: Vec::new(),
            myLastLateralGapLeft: 0.,
            myLastLateralGapRight: 0.,
            myLastLeaderGap: 0.,
            myLastFollowerGap: 0.,
            myLastLeaderSecureGap: 0.,
            myLastFollowerSecureGap: 0.,
            myLastOrigLeaderGap: 0.,
            myLastOrigLeaderSecureGap: 0.,
            myLastLeaderSpeed: 0.,
            myLastFollowerSpeed: 0.,
            myLastOrigLeaderSpeed: 0.,
            myDontResetLCGaps: false,
            myStrategicLookahead: p.get_lc_param(SUMO_ATTR_LCA_STRATEGIC_LOOKAHEAD, UNDEFINED_LOOKAHEAD),
            myMaxSpeedLatStanding: p.get_lc_param(SUMO_ATTR_LCA_MAXSPEEDLATSTANDING, max_speed_lat),
            myMaxSpeedLatFactor: p.get_lc_param(SUMO_ATTR_LCA_MAXSPEEDLATFACTOR, 1.),
            // prevent lateral sliding for cars but permit for two-wheelers due to better maneuverability
            myMaxDistLatStanding: p.get_lc_param(
                SUMO_ATTR_LCA_MAXDISTLATSTANDING,
                if (v.get_v_class() & (SVC_BICYCLE | SVC_MOTORCYCLE | SVC_MOPED)) != 0 { f64::MAX } else { 1.6 },
            ),
            mySigma: p.get_lc_param(SUMO_ATTR_LCA_SIGMA, 0.0),
            myOvertakeRightParam: p.get_lc_param(SUMO_ATTR_LCA_OVERTAKE_RIGHT, 0.),
            myAssertive: p.get_lc_param(SUMO_ATTR_LCA_ASSERTIVE, 1.),
            myCooperativeHelpTime: time2steps(p.get_lc_param(SUMO_ATTR_LCA_COOPERATIVE_HELPTIME, 60.)),
            myCooperativeHelpThreshold: p.get_lc_param(SUMO_ATTR_LCA_COOPERATIVE_HELPTHRESHOLD, -1.),
            myCooperativeMinSpeed: p.get_lc_param(SUMO_ATTR_LCA_COOPERATIVE_MINSPEED, 0.),
            // see MSVehicle::initDevices
            myHaveBlueLight: !v.get_device("bluelight").is_null(),
            myLCAccelerationAdvices: Vec::new(),
            myLastLaneChangeOffset: 0,
            myApproachedByShadow: std::cell::RefCell::new(Vec::new()),
            myAmOpposite: false,
            myManeuverDist: 0.,
            myPreviousManeuverDist: 0.,
        };
        s.save_lc_state(-1, LCA_UNKNOWN, LCA_UNKNOWN);
        s.save_lc_state(0, LCA_UNKNOWN, LCA_UNKNOWN);
        s.save_lc_state(1, LCA_UNKNOWN, LCA_UNKNOWN);
        s
    }

    /// C++: `void MSAbstractLaneChangeModel::~MSAbstractLaneChangeModel()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; Rust drops the fields itself
    }

    /// C++: `int MSAbstractLaneChangeModel::getOwnState() const` [const]
    // inline int getOwnState() const {
    //         return myOwnState;
    //     }
    #[allow(unused_variables)]
    pub fn get_own_state(&self) -> i32 {
        self.myOwnState
    }

    /// C++: `int MSAbstractLaneChangeModel::getPrevState() const` [const]
    // inline int getPrevState() const {
    //         /// at the time of this call myPreviousState already holds the new value
    //         return myPreviousState2;
    //     }
    #[allow(unused_variables)]
    pub fn get_prev_state(&self) -> i32 {
        // at the time of this call myPreviousState already holds the new value
        self.myPreviousState2
    }

    /// C++: `void MSAbstractLaneChangeModel::setOwnState(const int)` [virtual]
    // void
    // MSAbstractLaneChangeModel::setOwnState(const int state) {
    //     myPreviousState2 = myPreviousState;
    //     myOwnState = state;
    //     myPreviousState = state; // myOwnState is modified in prepareStep so we make a backup
    // }
    #[allow(unused_variables)]
    pub fn set_own_state(&mut self, state: i32) {
        (self.vt.set_own_state)(self, state)
    }

    pub fn set_own_state_base(&mut self, state: i32) {
        self.myPreviousState2 = self.myPreviousState;
        self.myOwnState = state;
        self.myPreviousState = state; // myOwnState is modified in prepareStep so we make a backup
    }

    /// C++: `void MSAbstractLaneChangeModel::setManeuverDist(const double)`
    /// Updates the remaining distance for the current maneuver while it is continued within non-action steps (only used by sublane model)
    // void
    // MSAbstractLaneChangeModel::setManeuverDist(const double dist) {
    // #ifdef DEBUG_MANEUVER
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME
    //                   << " veh=" << myVehicle.getID()
    //                   << " setManeuverDist() old=" << myManeuverDist << " new=" << dist
    //                   << std::endl;
    //     }
    // #endif
    //     myManeuverDist = fabs(dist) < NUMERICAL_EPS ? 0. : dist;
    //     // store value which may be modified by the model during the next step
    //     myPreviousManeuverDist = myManeuverDist;
    // }
    #[allow(unused_variables)]
    pub fn set_maneuver_dist(&mut self, dist: f64) {
        self.myManeuverDist = if dist.abs() < NUMERICAL_EPS { 0. } else { dist };
        // store value which may be modified by the model during the next step
        self.myPreviousManeuverDist = self.myManeuverDist;
    }

    /// C++: `double MSAbstractLaneChangeModel::getManeuverDist() const` [const]
    /// Returns the remaining unblocked distance for the current maneuver. (only used by sublane model)
    // double
    // MSAbstractLaneChangeModel::getManeuverDist() const {
    //     return myManeuverDist;
    // }
    #[allow(unused_variables)]
    pub fn get_maneuver_dist(&self) -> f64 {
        self.myManeuverDist
    }

    /// C++: `double MSAbstractLaneChangeModel::getPreviousManeuverDist() const` [const]
    // double
    // MSAbstractLaneChangeModel::getPreviousManeuverDist() const {
    //     return myPreviousManeuverDist;
    // }
    #[allow(unused_variables)]
    pub fn get_previous_maneuver_dist(&self) -> f64 {
        self.myPreviousManeuverDist
    }

    /// C++: `void MSAbstractLaneChangeModel::updateSafeLatDist(const double)` [virtual]
    /// Updates the value of safe lateral distances (in SL2015) during maneuver continuation in non-action steps
    // void
    // MSAbstractLaneChangeModel::updateSafeLatDist(const double travelledLatDist) {
    //     UNUSED_PARAMETER(travelledLatDist);
    // }
    #[allow(unused_variables)]
    pub fn update_safe_lat_dist(&mut self, travelledLatDist: f64) {
        (self.vt.update_safe_lat_dist)(self, travelledLatDist)
    }

    pub fn update_safe_lat_dist_base(&mut self, travelledLatDist: f64) {
        // C++ base body is empty (UNUSED_PARAMETER only); subclasses override

    }

    /// C++: `const std::pair<int, int> & MSAbstractLaneChangeModel::getSavedState(const int) const` [const]
    // const std::pair<int, int>& getSavedState(const int dir) const {
    //         if (dir == -1) {
    //             return mySavedStateRight;
    //         } else if (dir == 0) {
    //             return mySavedStateCenter;
    //         } else {
    //             return mySavedStateLeft;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_saved_state(&self, dir: i32) -> &(i32, i32) {
        if dir == -1 {
            &self.mySavedStateRight
        } else if dir == 0 {
            &self.mySavedStateCenter
        } else {
            &self.mySavedStateLeft
        }
    }

    /// C++: `void MSAbstractLaneChangeModel::saveLCState(const int, int, const int)`
    // void saveLCState(const int dir, int stateWithoutTraCI, const int state) {
    //         int canceledStrategic = getCanceledState(dir);
    //         // avoid conflicting directions
    //         if ((canceledStrategic & LCA_WANTS_LANECHANGE_OR_STAY) != 0) {
    //             stateWithoutTraCI = canceledStrategic;
    //         }
    //         const auto pair = std::make_pair(stateWithoutTraCI, state);
    //         if (dir == -1) {
    //             mySavedStateRight = pair;
    //         } else if (dir == 0) {
    //             mySavedStateCenter = pair;
    //         } else {
    //             mySavedStateLeft = pair;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn save_lc_state(&mut self, dir: i32, mut stateWithoutTraCI: i32, state: i32) {
        let canceledStrategic = *self.get_canceled_state(dir);
        // avoid conflicting directions
        if (canceledStrategic & LCA_WANTS_LANECHANGE_OR_STAY) != 0 {
            stateWithoutTraCI = canceledStrategic;
        }
        let pair = (stateWithoutTraCI, state);
        if dir == -1 {
            self.mySavedStateRight = pair;
        } else if dir == 0 {
            self.mySavedStateCenter = pair;
        } else {
            self.mySavedStateLeft = pair;
        }
    }

    /// C++: `void MSAbstractLaneChangeModel::saveNeighbors(const int, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &)`
    /// Saves the lane change relevant vehicles, which are currently on neighboring lanes in the given direction (as detected in wantsChangeSublane()). -> SL2015 case
    // void
    // MSAbstractLaneChangeModel::saveNeighbors(const int dir, const MSLeaderDistanceInfo& followers, const MSLeaderDistanceInfo& leaders) {
    //     if (dir == -1) {
    //         myLeftFollowers = std::make_shared<MSLeaderDistanceInfo>(followers);
    //         myLeftLeaders = std::make_shared<MSLeaderDistanceInfo>(leaders);
    //     } else if (dir == 1) {
    //         myRightFollowers = std::make_shared<MSLeaderDistanceInfo>(followers);
    //         myRightLeaders = std::make_shared<MSLeaderDistanceInfo>(leaders);
    //     } else {
    //         // dir \in {-1,1} !
    //         assert(false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_neighbors(&mut self, dir: i32, followers: &MSLeaderDistanceInfo, leaders: &MSLeaderDistanceInfo) {
        if dir == -1 {
            self.myLeftFollowers = Some(Rc::new(followers.clone()));
            self.myLeftLeaders = Some(Rc::new(leaders.clone()));
        } else if dir == 1 {
            self.myRightFollowers = Some(Rc::new(followers.clone()));
            self.myRightLeaders = Some(Rc::new(leaders.clone()));
        } else {
            // dir \in {-1,1} !
            debug_assert!(false);
        }
    }

    /// C++: `void MSAbstractLaneChangeModel::saveNeighbors(const int, const std::pair<MSVehicle *const, double> &, const std::pair<MSVehicle *const, double> &)`
    /// Saves the lane change relevant vehicles, which are currently on neighboring lanes in the given direction (as detected in wantsChange()). -> LC2013 case
    // void
    // MSAbstractLaneChangeModel::saveNeighbors(const int dir, const std::pair<MSVehicle* const, double>& follower, const std::pair<MSVehicle* const, double>& leader) {
    //     if (dir == -1) {
    //         myLeftFollowers = std::make_shared<MSLeaderDistanceInfo>(follower, myVehicle.getLane()->getWidth());
    //         myLeftLeaders = std::make_shared<MSLeaderDistanceInfo>(leader, myVehicle.getLane()->getWidth());
    //     } else if (dir == 1) {
    //         myRightFollowers = std::make_shared<MSLeaderDistanceInfo>(follower, myVehicle.getLane()->getWidth());
    //         myRightLeaders = std::make_shared<MSLeaderDistanceInfo>(leader, myVehicle.getLane()->getWidth());
    //     } else {
    //         // dir \in {-1,1} !
    //         assert(false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn save_neighbors_2(&mut self, dir: i32, follower: &(Ptr<MSVehicle>, f64), leader: &(Ptr<MSVehicle>, f64)) {
        let width = self.myVehicle.get_lane().get_width();
        if dir == -1 {
            self.myLeftFollowers = Some(Rc::new(MSLeaderDistanceInfo::new_2(follower, width)));
            self.myLeftLeaders = Some(Rc::new(MSLeaderDistanceInfo::new_2(leader, width)));
        } else if dir == 1 {
            self.myRightFollowers = Some(Rc::new(MSLeaderDistanceInfo::new_2(follower, width)));
            self.myRightLeaders = Some(Rc::new(MSLeaderDistanceInfo::new_2(leader, width)));
        } else {
            // dir \in {-1,1} !
            debug_assert!(false);
        }
    }

    /// C++: `void MSAbstractLaneChangeModel::clearNeighbors()`
    /// Clear info on neighboring vehicle from previous step
    // void
    // MSAbstractLaneChangeModel::clearNeighbors() {
    //     myLeftFollowers = nullptr;
    //     myLeftLeaders = nullptr;
    //     myRightFollowers = nullptr;
    //     myRightLeaders = nullptr;
    // }
    #[allow(unused_variables)]
    pub fn clear_neighbors(&mut self) {
        self.myLeftFollowers = None;
        self.myLeftLeaders = None;
        self.myRightFollowers = None;
        self.myRightLeaders = None;
    }

    /// C++: `const std::shared_ptr<MSLeaderDistanceInfo> MSAbstractLaneChangeModel::getFollowers(const int)`
    /// Returns the neighboring, lc-relevant followers for the last step in the requested direction
    // const std::shared_ptr<MSLeaderDistanceInfo>
    // MSAbstractLaneChangeModel::getFollowers(const int dir) {
    //     if (dir == -1) {
    //         return myLeftFollowers;
    //     } else if (dir == 1) {
    //         return myRightFollowers;
    //     } else {
    //         // dir \in {-1,1} !
    //         assert(false);
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_followers(&mut self, dir: i32) -> Option<Rc<MSLeaderDistanceInfo>> {
        if dir == -1 {
            return self.myLeftFollowers.clone();
        } else if dir == 1 {
            return self.myRightFollowers.clone();
        } else {
            // dir \in {-1,1} !
            debug_assert!(false);
        }
        None
    }

    /// C++: `const std::shared_ptr<MSLeaderDistanceInfo> MSAbstractLaneChangeModel::getLeaders(const int)`
    /// Returns the neighboring, lc-relevant leaders for the last step in the requested direction
    // const std::shared_ptr<MSLeaderDistanceInfo>
    // MSAbstractLaneChangeModel::getLeaders(const int dir) {
    //     if (dir == -1) {
    //         return myLeftLeaders;
    //     } else if (dir == 1) {
    //         return myRightLeaders;
    //     } else {
    //         // dir \in {-1,1} !
    //         assert(false);
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_leaders(&mut self, dir: i32) -> Option<Rc<MSLeaderDistanceInfo>> {
        if dir == -1 {
            return self.myLeftLeaders.clone();
        } else if dir == 1 {
            return self.myRightLeaders.clone();
        } else {
            // dir \in {-1,1} !
            debug_assert!(false);
        }
        None
    }

    /// C++: `int & MSAbstractLaneChangeModel::getCanceledState(const int)`
    // int& getCanceledState(const int dir) {
    //         if (dir == -1) {
    //             return myCanceledStateRight;
    //         } else if (dir == 0) {
    //             return myCanceledStateCenter;
    //         } else {
    //             return myCanceledStateLeft;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn get_canceled_state(&mut self, dir: i32) -> &mut i32 {
        if dir == -1 {
            &mut self.myCanceledStateRight
        } else if dir == 0 {
            &mut self.myCanceledStateCenter
        } else {
            &mut self.myCanceledStateLeft
        }
    }

    /// C++: `bool MSAbstractLaneChangeModel::isStrategicBlocked() const` [const]
    /// Returns whether this vehicle is blocked from performing a strategic change
    // bool
    // MSAbstractLaneChangeModel::isStrategicBlocked() const {
    //     const int stateRight = mySavedStateRight.second;
    //     if (
    //         (stateRight & LCA_STRATEGIC) != 0
    //         && (stateRight & LCA_RIGHT) != 0
    //         && (stateRight & LCA_BLOCKED) != 0) {
    //         return true;
    //     }
    //     const int stateLeft = mySavedStateLeft.second;
    //     if (
    //         (stateLeft & LCA_STRATEGIC) != 0
    //         && (stateLeft & LCA_LEFT) != 0
    //         && (stateLeft & LCA_BLOCKED) != 0) {
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_strategic_blocked(&self) -> bool {
        let stateRight = self.mySavedStateRight.1;
        if (stateRight & LCA_STRATEGIC) != 0 && (stateRight & LCA_RIGHT) != 0 && (stateRight & LCA_BLOCKED) != 0 {
            return true;
        }
        let stateLeft = self.mySavedStateLeft.1;
        if (stateLeft & LCA_STRATEGIC) != 0 && (stateLeft & LCA_LEFT) != 0 && (stateLeft & LCA_BLOCKED) != 0 {
            return true;
        }
        false
    }

    /// C++: `void MSAbstractLaneChangeModel::setFollowerGaps(CLeaderDist, double)`
    // void
    // MSAbstractLaneChangeModel::setFollowerGaps(CLeaderDist follower, double secGap)  {
    //     if (follower.first != 0) {
    //         myLastFollowerGap = follower.second + follower.first->getVehicleType().getMinGap();
    //         myLastFollowerSecureGap = secGap;
    //         myLastFollowerSpeed = follower.first->getSpeed();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_follower_gaps(&mut self, follower: CLeaderDist, secGap: f64) {
        if !follower.0.is_null() {
            self.myLastFollowerGap = follower.1 + follower.0.get_vehicle_type().get_min_gap();
            self.myLastFollowerSecureGap = secGap;
            self.myLastFollowerSpeed = follower.0.get_speed();
        }
    }

    /// C++: `void MSAbstractLaneChangeModel::setLeaderGaps(CLeaderDist, double)`
    // void
    // MSAbstractLaneChangeModel::setLeaderGaps(CLeaderDist leader, double secGap) {
    //     if (leader.first != 0) {
    //         myLastLeaderGap = leader.second + myVehicle.getVehicleType().getMinGap();
    //         myLastLeaderSecureGap = secGap;
    //         myLastLeaderSpeed = leader.first->getSpeed();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_leader_gaps(&mut self, arg0: CLeaderDist, secGap: f64) {
        if !arg0.0.is_null() {
            self.myLastLeaderGap = arg0.1 + self.myVehicle.get_vehicle_type().get_min_gap();
            self.myLastLeaderSecureGap = secGap;
            self.myLastLeaderSpeed = arg0.0.get_speed();
        }
    }

    /// C++: `void MSAbstractLaneChangeModel::setOrigLeaderGaps(CLeaderDist, double)`
    // void
    // MSAbstractLaneChangeModel::setOrigLeaderGaps(CLeaderDist leader, double secGap) {
    //     if (leader.first != 0) {
    //         myLastOrigLeaderGap = leader.second + myVehicle.getVehicleType().getMinGap();
    //         myLastOrigLeaderSecureGap = secGap;
    //         myLastOrigLeaderSpeed = leader.first->getSpeed();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_orig_leader_gaps(&mut self, arg0: CLeaderDist, secGap: f64) {
        if !arg0.0.is_null() {
            self.myLastOrigLeaderGap = arg0.1 + self.myVehicle.get_vehicle_type().get_min_gap();
            self.myLastOrigLeaderSecureGap = secGap;
            self.myLastOrigLeaderSpeed = arg0.0.get_speed();
        }
    }

    /// C++: `void MSAbstractLaneChangeModel::setFollowerGaps(const MSLeaderDistanceInfo &)`
    // void
    // MSAbstractLaneChangeModel::setFollowerGaps(const MSLeaderDistanceInfo& vehicles) {
    //     int rightmost;
    //     int leftmost;
    //     vehicles.getSubLanes(&myVehicle, 0, rightmost, leftmost);
    //     for (int i = rightmost; i <= leftmost; ++i) {
    //         CLeaderDist vehDist = vehicles[i];
    //         if (vehDist.first != 0) {
    //             const MSVehicle* leader = &myVehicle;
    //             const MSVehicle* follower = vehDist.first;
    //             const double netGap = vehDist.second + follower->getVehicleType().getMinGap();
    //             if (netGap < myLastFollowerGap && netGap >= 0) {
    //                 myLastFollowerGap = netGap;
    //                 myLastFollowerSecureGap = follower->getCarFollowModel().getSecureGap(follower, leader, follower->getSpeed(), leader->getSpeed(), leader->getCarFollowModel().getMaxDecel());
    //                 myLastFollowerSpeed = follower->getSpeed();
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_follower_gaps_2(&mut self, vehicles: &MSLeaderDistanceInfo) {
        let mut rightmost: i32 = 0;
        let mut leftmost: i32 = 0;
        vehicles.get_sub_lanes(self.myVehicle, 0., &mut rightmost, &mut leftmost);
        for i in rightmost..=leftmost {
            let vehDist = vehicles.index_op(i);
            if !vehDist.0.is_null() {
                let leader = self.myVehicle;
                let follower = vehDist.0;
                let netGap = vehDist.1 + follower.get_vehicle_type().get_min_gap();
                if netGap < self.myLastFollowerGap && netGap >= 0. {
                    self.myLastFollowerGap = netGap;
                    self.myLastFollowerSecureGap = follower.get_car_follow_model().get_secure_gap(
                        follower,
                        leader,
                        follower.get_speed(),
                        leader.get_speed(),
                        leader.get_car_follow_model().get_max_decel(),
                    );
                    self.myLastFollowerSpeed = follower.get_speed();
                }
            }
        }
    }

    /// C++: `void MSAbstractLaneChangeModel::setLeaderGaps(const MSLeaderDistanceInfo &)`
    // void
    // MSAbstractLaneChangeModel::setLeaderGaps(const MSLeaderDistanceInfo& vehicles) {
    //     int rightmost;
    //     int leftmost;
    //     vehicles.getSubLanes(&myVehicle, 0, rightmost, leftmost);
    //     for (int i = rightmost; i <= leftmost; ++i) {
    //         CLeaderDist vehDist = vehicles[i];
    //         if (vehDist.first != 0) {
    //             const MSVehicle* leader = vehDist.first;
    //             const MSVehicle* follower = &myVehicle;
    //             const double netGap = vehDist.second + follower->getVehicleType().getMinGap();
    //             if (netGap < myLastLeaderGap && netGap >= 0) {
    //                 myLastLeaderGap = netGap;
    //                 myLastLeaderSecureGap = follower->getCarFollowModel().getSecureGap(follower, leader, follower->getSpeed(), leader->getSpeed(), leader->getCarFollowModel().getMaxDecel());
    //                 myLastLeaderSpeed = leader->getSpeed();
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_leader_gaps_2(&mut self, vehicles: &MSLeaderDistanceInfo) {
        let mut rightmost: i32 = 0;
        let mut leftmost: i32 = 0;
        vehicles.get_sub_lanes(self.myVehicle, 0., &mut rightmost, &mut leftmost);
        for i in rightmost..=leftmost {
            let vehDist = vehicles.index_op(i);
            if !vehDist.0.is_null() {
                let leader = vehDist.0;
                let follower = self.myVehicle;
                let netGap = vehDist.1 + follower.get_vehicle_type().get_min_gap();
                if netGap < self.myLastLeaderGap && netGap >= 0. {
                    self.myLastLeaderGap = netGap;
                    self.myLastLeaderSecureGap = follower.get_car_follow_model().get_secure_gap(
                        follower,
                        leader,
                        follower.get_speed(),
                        leader.get_speed(),
                        leader.get_car_follow_model().get_max_decel(),
                    );
                    self.myLastLeaderSpeed = leader.get_speed();
                }
            }
        }
    }

    /// C++: `void MSAbstractLaneChangeModel::setOrigLeaderGaps(const MSLeaderDistanceInfo &)`
    // void
    // MSAbstractLaneChangeModel::setOrigLeaderGaps(const MSLeaderDistanceInfo& vehicles) {
    //     int rightmost;
    //     int leftmost;
    //     vehicles.getSubLanes(&myVehicle, 0, rightmost, leftmost);
    //     for (int i = rightmost; i <= leftmost; ++i) {
    //         CLeaderDist vehDist = vehicles[i];
    //         if (vehDist.first != 0) {
    //             const MSVehicle* leader = vehDist.first;
    //             const MSVehicle* follower = &myVehicle;
    //             const double netGap = vehDist.second + follower->getVehicleType().getMinGap();
    //             if (netGap < myLastOrigLeaderGap && netGap >= 0) {
    //                 myLastOrigLeaderGap = netGap;
    //                 myLastOrigLeaderSecureGap = follower->getCarFollowModel().getSecureGap(follower, leader, follower->getSpeed(), leader->getSpeed(), leader->getCarFollowModel().getMaxDecel());
    //                 myLastOrigLeaderSpeed = leader->getSpeed();
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_orig_leader_gaps_2(&mut self, vehicles: &MSLeaderDistanceInfo) {
        let mut rightmost: i32 = 0;
        let mut leftmost: i32 = 0;
        vehicles.get_sub_lanes(self.myVehicle, 0., &mut rightmost, &mut leftmost);
        for i in rightmost..=leftmost {
            let vehDist = vehicles.index_op(i);
            if !vehDist.0.is_null() {
                let leader = vehDist.0;
                let follower = self.myVehicle;
                let netGap = vehDist.1 + follower.get_vehicle_type().get_min_gap();
                if netGap < self.myLastOrigLeaderGap && netGap >= 0. {
                    self.myLastOrigLeaderGap = netGap;
                    self.myLastOrigLeaderSecureGap = follower.get_car_follow_model().get_secure_gap(
                        follower,
                        leader,
                        follower.get_speed(),
                        leader.get_speed(),
                        leader.get_car_follow_model().get_max_decel(),
                    );
                    self.myLastOrigLeaderSpeed = leader.get_speed();
                }
            }
        }
    }

    /// C++: `void MSAbstractLaneChangeModel::prepareStep()` [virtual]
    // void
    // MSAbstractLaneChangeModel::prepareStep() {
    //     getCanceledState(-1) = LCA_NONE;
    //     getCanceledState(0) = LCA_NONE;
    //     getCanceledState(1) = LCA_NONE;
    //     saveLCState(-1, LCA_UNKNOWN, LCA_UNKNOWN);
    //     saveLCState(0, LCA_UNKNOWN, LCA_UNKNOWN);
    //     saveLCState(1, LCA_UNKNOWN, LCA_UNKNOWN);
    //     myLastLateralGapRight = NO_NEIGHBOR;
    //     myLastLateralGapLeft = NO_NEIGHBOR;
    //     if (!myDontResetLCGaps) {
    //         myLastLeaderGap = NO_NEIGHBOR;
    //         myLastLeaderSecureGap = NO_NEIGHBOR;
    //         myLastFollowerGap = NO_NEIGHBOR;
    //         myLastFollowerSecureGap = NO_NEIGHBOR;
    //         myLastOrigLeaderGap = NO_NEIGHBOR;
    //         myLastOrigLeaderSecureGap = NO_NEIGHBOR;
    //         myLastLeaderSpeed = NO_NEIGHBOR;
    //         myLastFollowerSpeed = NO_NEIGHBOR;
    //         myLastOrigLeaderSpeed = NO_NEIGHBOR;
    //     }
    //     myCommittedSpeed = 0;
    // }
    #[allow(unused_variables)]
    pub fn prepare_step(&mut self) {
        (self.vt.prepare_step)(self)
    }

    pub fn prepare_step_base(&mut self) {
        *self.get_canceled_state(-1) = LCA_NONE;
        *self.get_canceled_state(0) = LCA_NONE;
        *self.get_canceled_state(1) = LCA_NONE;
        self.save_lc_state(-1, LCA_UNKNOWN, LCA_UNKNOWN);
        self.save_lc_state(0, LCA_UNKNOWN, LCA_UNKNOWN);
        self.save_lc_state(1, LCA_UNKNOWN, LCA_UNKNOWN);
        self.myLastLateralGapRight = NO_NEIGHBOR;
        self.myLastLateralGapLeft = NO_NEIGHBOR;
        if !self.myDontResetLCGaps {
            self.myLastLeaderGap = NO_NEIGHBOR;
            self.myLastLeaderSecureGap = NO_NEIGHBOR;
            self.myLastFollowerGap = NO_NEIGHBOR;
            self.myLastFollowerSecureGap = NO_NEIGHBOR;
            self.myLastOrigLeaderGap = NO_NEIGHBOR;
            self.myLastOrigLeaderSecureGap = NO_NEIGHBOR;
            self.myLastLeaderSpeed = NO_NEIGHBOR;
            self.myLastFollowerSpeed = NO_NEIGHBOR;
            self.myLastOrigLeaderSpeed = NO_NEIGHBOR;
        }
        self.myCommittedSpeed = 0.;
    }

    /// C++: `int MSAbstractLaneChangeModel::wantsChange(int, MSAbstractLaneChangeModel::MSLCMessager &, int, const std::pair<MSVehicle *, double> &, const std::pair<MSVehicle *, double> &, const std::pair<MSVehicle *, double> &, const std::pair<MSVehicle *, double> &, const MSLane &, const std::vector<MSVehicle::LaneQ> &, MSVehicle **, MSVehicle **)` [virtual]
    /// Called to examine whether the vehicle wants to change using the given laneOffset. This method gets the information about the surrounding vehicles and whether another lane may be more preferable
    // virtual int wantsChange(
    //         int laneOffset,
    //         MSAbstractLaneChangeModel::MSLCMessager& msgPass, int blocked,
    //         const std::pair<MSVehicle*, double>& leader,
    //         const std::pair<MSVehicle*, double>& follower,
    //         const std::pair<MSVehicle*, double>& neighLead,
    //         const std::pair<MSVehicle*, double>& neighFollow,
    //         const MSLane& neighLane,
    //         const std::vector<MSVehicle::LaneQ>& preb,
    //         MSVehicle** lastBlocked,
    //         MSVehicle** firstBlocked) {
    //         UNUSED_PARAMETER(laneOffset);
    //         UNUSED_PARAMETER(&msgPass);
    //         UNUSED_PARAMETER(blocked);
    //         UNUSED_PARAMETER(&leader);
    //         UNUSED_PARAMETER(&follower);
    //         UNUSED_PARAMETER(&neighLead);
    //         UNUSED_PARAMETER(&neighFollow);
    //         UNUSED_PARAMETER(&neighLane);
    //         UNUSED_PARAMETER(&preb);
    //         UNUSED_PARAMETER(lastBlocked);
    //         UNUSED_PARAMETER(firstBlocked);
    //         throw ProcessError("Method not implemented by model " + toString(myModel));
    //     }
    #[allow(unused_variables)]
    pub fn wants_change(&mut self, laneOffset: i32, msgPass: &mut MSAbstractLaneChangeModel_MSLCMessager, blocked: i32, leader: &(Ptr<MSVehicle>, f64), follower: &(Ptr<MSVehicle>, f64), neighLead: &(Ptr<MSVehicle>, f64), neighFollow: &(Ptr<MSVehicle>, f64), neighLane: &MSLane, preb: &Vec<MSVehicle_LaneQ>, lastBlocked: Ptr<Ptr<MSVehicle>>, firstBlocked: Ptr<Ptr<MSVehicle>>) -> i32 {
        (self.vt.wants_change)(self, laneOffset, msgPass, blocked, leader, follower, neighLead, neighFollow, neighLane, preb, lastBlocked, firstBlocked)
    }

    pub fn wants_change_base(&mut self, laneOffset: i32, msgPass: &mut MSAbstractLaneChangeModel_MSLCMessager, blocked: i32, leader: &(Ptr<MSVehicle>, f64), follower: &(Ptr<MSVehicle>, f64), neighLead: &(Ptr<MSVehicle>, f64), neighFollow: &(Ptr<MSVehicle>, f64), neighLane: &MSLane, preb: &Vec<MSVehicle_LaneQ>, lastBlocked: Ptr<Ptr<MSVehicle>>, firstBlocked: Ptr<Ptr<MSVehicle>>) -> i32 {
        panic!(
            "{}",
            ProcessError::Process(format!("Method not implemented by model {}", LaneChangeModels.get_string(self.myModel)))
        )
    }

    /// C++: `int MSAbstractLaneChangeModel::wantsChangeSublane(int, LaneChangeAction, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLeaderDistanceInfo &, const MSLane &, const std::vector<MSVehicle::LaneQ> &, MSVehicle **, MSVehicle **, double &, double &, int &)` [virtual]
    // virtual int wantsChangeSublane(
    //         int laneOffset,
    //         LaneChangeAction alternatives,
    //         const MSLeaderDistanceInfo& leaders,
    //         const MSLeaderDistanceInfo& followers,
    //         const MSLeaderDistanceInfo& blockers,
    //         const MSLeaderDistanceInfo& neighLeaders,
    //         const MSLeaderDistanceInfo& neighFollowers,
    //         const MSLeaderDistanceInfo& neighBlockers,
    //         const MSLane& neighLane,
    //         const std::vector<MSVehicle::LaneQ>& preb,
    //         MSVehicle** lastBlocked,
    //         MSVehicle** firstBlocked,
    //         double& latDist, double& targetDistLat, int& blocked) {
    //         UNUSED_PARAMETER(laneOffset);
    //         UNUSED_PARAMETER(alternatives);
    //         UNUSED_PARAMETER(&leaders);
    //         UNUSED_PARAMETER(&followers);
    //         UNUSED_PARAMETER(&blockers);
    //         UNUSED_PARAMETER(&neighLeaders);
    //         UNUSED_PARAMETER(&neighFollowers);
    //         UNUSED_PARAMETER(&neighBlockers);
    //         UNUSED_PARAMETER(&neighLane);
    //         UNUSED_PARAMETER(&preb);
    //         UNUSED_PARAMETER(lastBlocked);
    //         UNUSED_PARAMETER(firstBlocked);
    //         UNUSED_PARAMETER(latDist);
    //         UNUSED_PARAMETER(targetDistLat);
    //         UNUSED_PARAMETER(blocked);
    //         throw ProcessError("Method not implemented by model " + toString(myModel));
    //     }
    #[allow(unused_variables)]
    pub fn wants_change_sublane(&mut self, laneOffset: i32, alternatives: LaneChangeAction, leaders: &MSLeaderDistanceInfo, followers: &MSLeaderDistanceInfo, blockers: &MSLeaderDistanceInfo, neighLeaders: &MSLeaderDistanceInfo, neighFollowers: &MSLeaderDistanceInfo, neighBlockers: &MSLeaderDistanceInfo, neighLane: &MSLane, preb: &Vec<MSVehicle_LaneQ>, lastBlocked: Ptr<Ptr<MSVehicle>>, firstBlocked: Ptr<Ptr<MSVehicle>>, latDist: &mut f64, targetDistLat: &mut f64, blocked: &mut i32) -> i32 {
        (self.vt.wants_change_sublane)(self, laneOffset, alternatives, leaders, followers, blockers, neighLeaders, neighFollowers, neighBlockers, neighLane, preb, lastBlocked, firstBlocked, latDist, targetDistLat, blocked)
    }

    pub fn wants_change_sublane_base(&mut self, laneOffset: i32, alternatives: LaneChangeAction, leaders: &MSLeaderDistanceInfo, followers: &MSLeaderDistanceInfo, blockers: &MSLeaderDistanceInfo, neighLeaders: &MSLeaderDistanceInfo, neighFollowers: &MSLeaderDistanceInfo, neighBlockers: &MSLeaderDistanceInfo, neighLane: &MSLane, preb: &Vec<MSVehicle_LaneQ>, lastBlocked: Ptr<Ptr<MSVehicle>>, firstBlocked: Ptr<Ptr<MSVehicle>>, latDist: &mut f64, targetDistLat: &mut f64, blocked: &mut i32) -> i32 {
        panic!(
            "{}",
            ProcessError::Process(format!("Method not implemented by model {}", LaneChangeModels.get_string(self.myModel)))
        )
    }

    /// C++: `void MSAbstractLaneChangeModel::updateExpectedSublaneSpeeds(const MSLeaderDistanceInfo &, int, int)` [virtual]
    /// update expected speeds for each sublane of the current edge
    // virtual void updateExpectedSublaneSpeeds(const MSLeaderDistanceInfo& ahead, int sublaneOffset, int laneIndex) {
    //         UNUSED_PARAMETER(&ahead);
    //         UNUSED_PARAMETER(sublaneOffset);
    //         UNUSED_PARAMETER(laneIndex);
    //         throw ProcessError("Method not implemented by model " + toString(myModel));
    //     }
    #[allow(unused_variables)]
    pub fn update_expected_sublane_speeds(&mut self, ahead: &MSLeaderDistanceInfo, sublaneOffset: i32, laneIndex: i32) {
        (self.vt.update_expected_sublane_speeds)(self, ahead, sublaneOffset, laneIndex)
    }

    pub fn update_expected_sublane_speeds_base(&mut self, ahead: &MSLeaderDistanceInfo, sublaneOffset: i32, laneIndex: i32) {
        panic!(
            "{}",
            ProcessError::Process(format!("Method not implemented by model {}", LaneChangeModels.get_string(self.myModel)))
        )
    }

    /// C++: `StateAndDist MSAbstractLaneChangeModel::decideDirection(StateAndDist, StateAndDist) const` [const, virtual]
    /// decide in which direction to move in case both directions are desirable
    // virtual StateAndDist decideDirection(StateAndDist sd1, StateAndDist sd2) const {
    //         UNUSED_PARAMETER(sd1);
    //         UNUSED_PARAMETER(sd2);
    //         throw ProcessError("Method not implemented by model " + toString(myModel));
    //     }
    #[allow(unused_variables)]
    pub fn decide_direction(&self, sd1: MSAbstractLaneChangeModel_StateAndDist, sd2: MSAbstractLaneChangeModel_StateAndDist) -> MSAbstractLaneChangeModel_StateAndDist {
        (self.vt.decide_direction)(self, sd1, sd2)
    }

    pub fn decide_direction_base(&self, sd1: MSAbstractLaneChangeModel_StateAndDist, sd2: MSAbstractLaneChangeModel_StateAndDist) -> MSAbstractLaneChangeModel_StateAndDist {
        panic!(
            "{}",
            ProcessError::Process(format!("Method not implemented by model {}", LaneChangeModels.get_string(self.myModel)))
        )
    }

    /// C++: `void * MSAbstractLaneChangeModel::inform(void *, MSVehicle *)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn inform(&mut self, info: Ptr<()>, sender: Ptr<MSVehicle>) -> Ptr<()> {
        (self.vt.inform)(self, info, sender)
    }

    /// C++: `double MSAbstractLaneChangeModel::patchSpeed(const double, const double, const double, const MSCFModel &)` [virtual, pure virtual]
    /// Called to adapt the speed in order to allow a lane change. It uses information on LC-related desired speed-changes from the call to wantsChange() at the end of the previous simulation step
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn patch_speed(&mut self, min: f64, wanted: f64, max: f64, cfModel: &MSCFModel) -> f64 {
        (self.vt.patch_speed)(self, min, wanted, max, cfModel)
    }

    /// C++: `void MSAbstractLaneChangeModel::changed()` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn changed(&mut self) {
        (self.vt.changed)(self)
    }

    /// C++: `void MSAbstractLaneChangeModel::resetState()` [virtual]
    // virtual void resetState() {}
    #[allow(unused_variables)]
    pub fn reset_state(&mut self) {
        (self.vt.reset_state)(self)
    }

    pub fn reset_state_base(&mut self) {
        // C++ base body is empty; subclasses override

    }

    /// C++: `double MSAbstractLaneChangeModel::getSafetyFactor() const` [const, virtual]
    /// return factor for modifying the safety constraints of the car-following model
    // virtual double getSafetyFactor() const {
    //         return 1.0;
    //     }
    #[allow(unused_variables)]
    pub fn get_safety_factor(&self) -> f64 {
        (self.vt.get_safety_factor)(self)
    }

    pub fn get_safety_factor_base(&self) -> f64 {
        1.0

    }

    /// C++: `double MSAbstractLaneChangeModel::getOppositeSafetyFactor() const` [const, virtual]
    /// return factor for modifying the safety constraints for opposite-diretction overtaking of the car-following model
    // virtual double getOppositeSafetyFactor() const {
    //         return 1.0;
    //     }
    #[allow(unused_variables)]
    pub fn get_opposite_safety_factor(&self) -> f64 {
        (self.vt.get_opposite_safety_factor)(self)
    }

    pub fn get_opposite_safety_factor_base(&self) -> f64 {
        1.0

    }

    /// C++: `bool MSAbstractLaneChangeModel::debugVehicle() const` [const, virtual]
    /// whether the current vehicles shall be debugged
    // virtual bool debugVehicle() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn debug_vehicle(&self) -> bool {
        (self.vt.debug_vehicle)(self)
    }

    pub fn debug_vehicle_base(&self) -> bool {
        false

    }

    /// C++: `void MSAbstractLaneChangeModel::changedToOpposite()`
    /// called when a vehicle changes between lanes in opposite directions
    // void
    // MSAbstractLaneChangeModel::changedToOpposite() {
    //     myAmOpposite = !myAmOpposite;
    //     myAlreadyChanged = true;
    // }
    #[allow(unused_variables)]
    pub fn changed_to_opposite(&mut self) {
        self.myAmOpposite = !self.myAmOpposite;
        self.myAlreadyChanged = true;
    }

    /// C++: `void MSAbstractLaneChangeModel::unchanged()`
    // void unchanged() {
    //         if (myLastLaneChangeOffset > 0) {
    //             myLastLaneChangeOffset += DELTA_T;
    //         } else if (myLastLaneChangeOffset < 0) {
    //             myLastLaneChangeOffset -= DELTA_T;
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn unchanged(&mut self) {
        if self.myLastLaneChangeOffset > 0 {
            self.myLastLaneChangeOffset += delta_t();
        } else if self.myLastLaneChangeOffset < 0 {
            self.myLastLaneChangeOffset -= delta_t();
        }
    }

    /// C++: `MSLane * MSAbstractLaneChangeModel::getShadowLane() const` [const]
    /// Returns the lane the vehicle's shadow is on during continuous/sublane lane change
    // MSLane* getShadowLane() const {
    //         return myShadowLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_shadow_lane(&self) -> Ptr<MSLane> {
        self.myShadowLane
    }

    /// C++: `MSLane * MSAbstractLaneChangeModel::getShadowLane(const MSLane *) const` [const]
    /// return the shadow lane for the given lane
    // MSLane*
    // MSAbstractLaneChangeModel::getShadowLane(const MSLane* lane) const {
    //     return getShadowLane(lane, myVehicle.getLateralPositionOnLane());
    // }
    #[allow(unused_variables)]
    pub fn get_shadow_lane_2(&self, lane: Ptr<MSLane>) -> Ptr<MSLane> {
        self.get_shadow_lane_3(lane, self.myVehicle.get_lateral_position_on_lane())
    }

    /// C++: `MSLane * MSAbstractLaneChangeModel::getShadowLane(const MSLane *, double) const` [const]
    /// return the shadow lane for the given lane and lateral offset
    // MSLane*
    // MSAbstractLaneChangeModel::getShadowLane(const MSLane* lane, double posLat) const {
    //     if (std::find(myNoPartiallyOccupatedByShadow.begin(), myNoPartiallyOccupatedByShadow.end(), lane) == myNoPartiallyOccupatedByShadow.end()) {
    //         // initialize shadow lane
    //         const double overlap = myVehicle.getLateralOverlap(posLat, lane);
    // #ifdef DEBUG_SHADOWLANE
    //         if (debugVehicle()) {
    //             std::cout << SIMTIME << " veh=" << myVehicle.getID() << " posLat=" << posLat << " overlap=" << overlap << "\n";
    //         }
    // #endif
    //         if (myAmOpposite) {
    //             // return the neigh-lane in forward direction
    //             return lane->getParallelLane(1);
    //         } else if (overlap > NUMERICAL_EPS) {
    //             const int shadowDirection = posLat < 0 ? -1 : 1;
    //             return lane->getParallelLane(shadowDirection);
    //         } else if (isChangingLanes() && myLaneChangeCompletion < 0.5) {
    //             // "reserve" target lane even when there is no overlap yet
    //             return lane->getParallelLane(myLaneChangeDirection);
    //         } else {
    //             return nullptr;
    //         }
    //     } else {
    //         return nullptr;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_shadow_lane_3(&self, lane: Ptr<MSLane>, posLat: f64) -> Ptr<MSLane> {
        if !self.myNoPartiallyOccupatedByShadow.contains(&lane) {
            // initialize shadow lane
            let overlap = self.myVehicle.get_lateral_overlap_3(posLat, lane);
            if self.myAmOpposite {
                // return the neigh-lane in forward direction
                lane.get_parallel_lane(1, true)
            } else if overlap > NUMERICAL_EPS {
                let shadowDirection: i32 = if posLat < 0. { -1 } else { 1 };
                lane.get_parallel_lane(shadowDirection, true)
            } else if self.is_changing_lanes() && self.myLaneChangeCompletion < 0.5 {
                // "reserve" target lane even when there is no overlap yet
                lane.get_parallel_lane(self.myLaneChangeDirection, true)
            } else {
                Ptr::null()
            }
        } else {
            Ptr::null()
        }
    }

    /// C++: `const std::vector<MSLane *> & MSAbstractLaneChangeModel::getShadowFurtherLanes() const` [const]
    // const std::vector<MSLane*>& getShadowFurtherLanes() const {
    //         return myShadowFurtherLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_shadow_further_lanes(&self) -> &Vec<Ptr<MSLane>> {
        &self.myShadowFurtherLanes
    }

    /// C++: `const std::vector<double> & MSAbstractLaneChangeModel::getShadowFurtherLanesPosLat() const` [const]
    // const std::vector<double>& getShadowFurtherLanesPosLat() const {
    //         return myShadowFurtherLanesPosLat;
    //     }
    #[allow(unused_variables)]
    pub fn get_shadow_further_lanes_pos_lat(&self) -> &Vec<f64> {
        &self.myShadowFurtherLanesPosLat
    }

    /// C++: `MSLane * MSAbstractLaneChangeModel::getTargetLane() const` [const]
    /// Returns the lane the vehicle has committed to enter during a sublane lane change
    // MSLane* getTargetLane() const {
    //         return myTargetLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_target_lane(&self) -> Ptr<MSLane> {
        self.myTargetLane
    }

    /// C++: `const std::vector<MSLane *> & MSAbstractLaneChangeModel::getFurtherTargetLanes() const` [const]
    // const std::vector<MSLane*>& getFurtherTargetLanes() const {
    //         return myFurtherTargetLanes;
    //     }
    #[allow(unused_variables)]
    pub fn get_further_target_lanes(&self) -> &Vec<Ptr<MSLane>> {
        &self.myFurtherTargetLanes
    }

    /// C++: `SUMOTime MSAbstractLaneChangeModel::getLastLaneChangeOffset() const` [const]
    // inline SUMOTime getLastLaneChangeOffset() const {
    //         return myLastLaneChangeOffset;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_lane_change_offset(&self) -> SUMOTime {
        self.myLastLaneChangeOffset
    }

    /// C++: `bool MSAbstractLaneChangeModel::pastMidpoint() const` [const]
    /// return whether the vehicle passed the midpoint of a continuous lane change maneuver
    // inline bool pastMidpoint() const {
    //         return myLaneChangeCompletion >= 0.5;
    //     }
    #[allow(unused_variables)]
    pub fn past_midpoint(&self) -> bool {
        self.myLaneChangeCompletion >= 0.5
    }

    /// C++: `SUMOTime MSAbstractLaneChangeModel::remainingTime() const` [const]
    /// Compute the remaining time until LC completion
    // SUMOTime
    // MSAbstractLaneChangeModel::remainingTime() const {
    //     assert(isChangingLanes()); // Only to be called during ongoing lane change
    //     const SUMOVTypeParameter::SubParams& lcParams = myVehicle.getVehicleType().getParameter().getLCParams();
    //     if (lcParams.find(SUMO_ATTR_LCA_MAXSPEEDLATSTANDING) == lcParams.end() && lcParams.find(SUMO_ATTR_LCA_MAXSPEEDLATFACTOR) == lcParams.end()) {
    //         if (myVehicle.getVehicleType().wasSet(VTYPEPARS_MAXSPEED_LAT_SET)) {
    //             return TIME2STEPS((1. - myLaneChangeCompletion) * myManeuverDist / myVehicle.getVehicleType().getMaxSpeedLat());
    //         } else {
    //             return (SUMOTime)((1. - myLaneChangeCompletion) * (double)MSGlobals::gLaneChangeDuration);
    //         }
    //     }
    //     // Using maxSpeedLat(Factor/Standing)
    //     const bool urgent = (myOwnState & LCA_URGENT) != 0;
    //     return TIME2STEPS(estimateLCDuration(myVehicle.getSpeed(),
    //                                          fabs(myManeuverDist * (1 - myLaneChangeCompletion)),
    //                                          myVehicle.getCarFollowModel().getMaxDecel(), urgent));
    // }
    #[allow(unused_variables)]
    pub fn remaining_time(&self) -> SUMOTime {
        debug_assert!(self.is_changing_lanes()); // Only to be called during ongoing lane change
        let lcParams = &self.myVehicle.get_vehicle_type().get_parameter().lcParameter;
        if !lcParams.contains_key(&SUMO_ATTR_LCA_MAXSPEEDLATSTANDING) && !lcParams.contains_key(&SUMO_ATTR_LCA_MAXSPEEDLATFACTOR) {
            if self.myVehicle.get_vehicle_type().was_set(VTYPEPARS_MAXSPEED_LAT_SET) {
                return time2steps((1. - self.myLaneChangeCompletion) * self.myManeuverDist / self.myVehicle.get_vehicle_type().get_max_speed_lat());
            } else {
                return ((1. - self.myLaneChangeCompletion) * MSGlobals::lane_change_duration() as f64) as SUMOTime;
            }
        }
        // Using maxSpeedLat(Factor/Standing)
        let urgent = (self.myOwnState & LCA_URGENT) != 0;
        time2steps(self.estimate_lc_duration(
            self.myVehicle.get_speed(),
            (self.myManeuverDist * (1. - self.myLaneChangeCompletion)).abs(),
            self.myVehicle.get_car_follow_model().get_max_decel(),
            urgent,
        ))
    }

    /// C++: `double MSAbstractLaneChangeModel::estimateLCDuration(const double, const double, const double, bool) const` [const, virtual]
    /// Calculates the maximal time needed to complete a lane change maneuver if lcMaxSpeedLatFactor and lcMaxSpeedStanding are set and the vehicle breaks not harder than decel. LC when the vehicle starts breaking now. If lcMaxSpeedStanding==0 the completion may be impossible,
    // double
    // MSAbstractLaneChangeModel::estimateLCDuration(const double speed, const double remainingManeuverDist, const double decel, bool urgent) const {
    //
    //     const SUMOVTypeParameter::SubParams& lcParams = myVehicle.getVehicleType().getParameter().getLCParams();
    //     if (lcParams.find(SUMO_ATTR_LCA_MAXSPEEDLATSTANDING) == lcParams.end() && lcParams.find(SUMO_ATTR_LCA_MAXSPEEDLATFACTOR) == lcParams.end()) {
    //         if (!myVehicle.getVehicleType().wasSet(VTYPEPARS_MAXSPEED_LAT_SET)) {
    //             // no dependency of lateral speed on longitudinal speed. (Only called prior to LC initialization to determine whether it could be completed)
    //             return STEPS2TIME(MSGlobals::gLaneChangeDuration);
    //         } else {
    //             return remainingManeuverDist / myVehicle.getVehicleType().getMaxSpeedLat();
    //         }
    //     }
    //
    //     if (remainingManeuverDist == 0) {
    //         return 0;
    //     }
    //
    //     // Check argument assumptions
    //     assert(speed >= 0);
    //     assert(remainingManeuverDist >= 0);
    //     assert(decel > 0);
    //     assert(myVehicle.getVehicleType().getMaxSpeedLat() > 0);
    //     assert(myMaxSpeedLatStanding <= myVehicle.getVehicleType().getMaxSpeedLat());
    //     assert(myMaxSpeedLatStanding >= 0);
    //
    //     // for brevity
    //     const double v0 = speed;
    //     const double D = remainingManeuverDist;
    //     const double b = decel;
    //     const double wmin = myMaxSpeedLatStanding;
    //     const double f = myMaxSpeedLatFactor;
    //     const double wmax = myVehicle.getVehicleType().getMaxSpeedLat();
    //
    //     /* Here's the approach for the calculation of the required time for the LC:
    //      * To obtain the maximal LC-duration, for v(t) we assume that v(t)=max(0, v0-b*t),
    //      * Where v(t)=0 <=> t >= ts:=v0/b
    //      * For the lateral speed w(t) this gives:
    //      * w(t) = min(wmax, wmin + f*v(t))
    //      * The lateral distance covered until t is
    //      * d(t) = int_0^t w(s) ds
    //      * We distinguish three possibilities for the solution d(T)=D, where T is the time of the LC completion.
    //      * 1) w(T) = wmax, i.e. v(T)>(wmax-wmin)/f
    //      * 2) wmin < w(T) < wmax, i.e. (wmax-wmin)/f > v(T) > 0
    //      * 3) w(T) = wmin, i.e., v(T)=0
    //      */
    //     const double vm = (wmax - wmin) / f;
    //     double distSoFar = 0.;
    //     double timeSoFar = 0.;
    //     double v = v0;
    //     if (v > vm) {
    //         const double wmaxTime = (v0 - vm) / b;
    //         const double d1 = wmax * wmaxTime;
    //         if (d1 >= D) {
    //             return D / wmax;
    //         } else {
    //             distSoFar += d1;
    //             timeSoFar += wmaxTime;
    //             v = vm;
    //         }
    //     }
    //     if (v > 0) {
    //         /* Here, w(t1+t) = wmin + f*v(t1+t) = wmin + f*(v - b*t)
    //          * Thus, the additional lateral distance covered after time t is:
    //          * d2 = (wmin + f*v)*t - 0.5*f*b*t^2
    //          * and the additional lateral distance covered until v=0 at t=v/b is:
    //          * d2 = (wmin + 0.5*f*v)*t
    //          */
    //         const double t = v / b; // stop time
    //         const double d2 = (wmin + 0.5 * f * v) * t; // lateral distance covered until stop
    //         assert(d2 > 0);
    //         if (distSoFar + d2 >= D) {
    //             // LC is completed during this phase
    //             const double x = 0.5 * f * b;
    //             const double y = wmin + f * v;
    //             /* Solve D - distSoFar = y*t - x*t^2.
    //              * 0 = x*t^2 - y*t/x + (D - distSoFar)/x
    //              */
    //             const double p = 0.5 * y / x;
    //             const double q = (D - distSoFar) / x;
    //             assert(p * p - q > 0);
    //             const double t2 = p + sqrt(p * p - q);
    //             return timeSoFar + t2;
    //         } else {
    //             distSoFar += d2;
    //             timeSoFar += t;
    //             //v = 0;
    //         }
    //     }
    //     // If we didn't return yet this means the LC was not completed until the vehicle stops (if braking with rate b)
    //     if (wmin == 0) {
    //         // LC won't be completed if vehicle stands
    //         double maneuverDist = remainingManeuverDist;
    //         const double vModel = computeSpeedLat(maneuverDist, maneuverDist, urgent);
    //         double result = D / vModel;
    //         // make sure that the vehicle isn't braking to a stop during the manuever
    //         if (vModel > SUMO_const_haltingSpeed && (vModel + myVehicle.getAcceleration() * result) > SUMO_const_haltingSpeed) {
    //             // unless the model tells us something different
    //             return result;
    //         } else {
    //             return -1;
    //         }
    //     } else {
    //         // complete LC with lateral speed wmin
    //         return timeSoFar + (D - distSoFar) / wmin;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn estimate_lc_duration(&self, speed: f64, remainingManeuverDist: f64, decel: f64, urgent: bool) -> f64 {
        let lcParams = &self.myVehicle.get_vehicle_type().get_parameter().lcParameter;
        if !lcParams.contains_key(&SUMO_ATTR_LCA_MAXSPEEDLATSTANDING) && !lcParams.contains_key(&SUMO_ATTR_LCA_MAXSPEEDLATFACTOR) {
            if !self.myVehicle.get_vehicle_type().was_set(VTYPEPARS_MAXSPEED_LAT_SET) {
                // no dependency of lateral speed on longitudinal speed. (Only called prior to LC initialization to determine whether it could be completed)
                return steps2time(MSGlobals::lane_change_duration());
            } else {
                return remainingManeuverDist / self.myVehicle.get_vehicle_type().get_max_speed_lat();
            }
        }

        if remainingManeuverDist == 0. {
            return 0.;
        }

        // Check argument assumptions
        debug_assert!(speed >= 0.);
        debug_assert!(remainingManeuverDist >= 0.);
        debug_assert!(decel > 0.);
        debug_assert!(self.myVehicle.get_vehicle_type().get_max_speed_lat() > 0.);
        debug_assert!(self.myMaxSpeedLatStanding <= self.myVehicle.get_vehicle_type().get_max_speed_lat());
        debug_assert!(self.myMaxSpeedLatStanding >= 0.);

        // for brevity
        let v0 = speed;
        let D = remainingManeuverDist;
        let b = decel;
        let wmin = self.myMaxSpeedLatStanding;
        let f = self.myMaxSpeedLatFactor;
        let wmax = self.myVehicle.get_vehicle_type().get_max_speed_lat();

        /* Here's the approach for the calculation of the required time for the LC:
         * To obtain the maximal LC-duration, for v(t) we assume that v(t)=max(0, v0-b*t),
         * Where v(t)=0 <=> t >= ts:=v0/b
         * For the lateral speed w(t) this gives:
         * w(t) = min(wmax, wmin + f*v(t))
         * The lateral distance covered until t is
         * d(t) = int_0^t w(s) ds
         * We distinguish three possibilities for the solution d(T)=D, where T is the time of the LC completion.
         * 1) w(T) = wmax, i.e. v(T)>(wmax-wmin)/f
         * 2) wmin < w(T) < wmax, i.e. (wmax-wmin)/f > v(T) > 0
         * 3) w(T) = wmin, i.e., v(T)=0
         */
        let vm = (wmax - wmin) / f;
        let mut distSoFar = 0.;
        let mut timeSoFar = 0.;
        let mut v = v0;
        if v > vm {
            let wmaxTime = (v0 - vm) / b;
            let d1 = wmax * wmaxTime;
            if d1 >= D {
                return D / wmax;
            } else {
                distSoFar += d1;
                timeSoFar += wmaxTime;
                v = vm;
            }
        }
        if v > 0. {
            /* Here, w(t1+t) = wmin + f*v(t1+t) = wmin + f*(v - b*t)
             * Thus, the additional lateral distance covered after time t is:
             * d2 = (wmin + f*v)*t - 0.5*f*b*t^2
             * and the additional lateral distance covered until v=0 at t=v/b is:
             * d2 = (wmin + 0.5*f*v)*t
             */
            let t = v / b; // stop time
            let d2 = (wmin + 0.5 * f * v) * t; // lateral distance covered until stop
            debug_assert!(d2 > 0.);
            if distSoFar + d2 >= D {
                // LC is completed during this phase
                let x = 0.5 * f * b;
                let y = wmin + f * v;
                /* Solve D - distSoFar = y*t - x*t^2.
                 * 0 = x*t^2 - y*t/x + (D - distSoFar)/x
                 */
                let p = 0.5 * y / x;
                let q = (D - distSoFar) / x;
                debug_assert!(p * p - q > 0.);
                let t2 = p + (p * p - q).sqrt();
                return timeSoFar + t2;
            } else {
                distSoFar += d2;
                timeSoFar += t;
                //v = 0;
            }
        }
        // If we didn't return yet this means the LC was not completed until the vehicle stops (if braking with rate b)
        if wmin == 0. {
            // LC won't be completed if vehicle stands
            let mut maneuverDist = remainingManeuverDist;
            let vModel = self.compute_speed_lat(maneuverDist, &mut maneuverDist, urgent);
            let result = D / vModel;
            // make sure that the vehicle isn't braking to a stop during the manuever
            if vModel > SUMO_const_haltingSpeed && (vModel + self.myVehicle.get_acceleration() * result) > SUMO_const_haltingSpeed {
                // unless the model tells us something different
                return result;
            } else {
                return -1.;
            }
        } else {
            // complete LC with lateral speed wmin
            return timeSoFar + (D - distSoFar) / wmin;
        }
    }

    /// C++: `bool MSAbstractLaneChangeModel::isChangingLanes() const` [const]
    /// return true if the vehicle currently performs a lane change maneuver
    // inline bool isChangingLanes() const {
    //         return myLaneChangeCompletion < (1 - NUMERICAL_EPS);
    //     }
    #[allow(unused_variables)]
    pub fn is_changing_lanes(&self) -> bool {
        self.myLaneChangeCompletion < (1. - NUMERICAL_EPS)
    }

    /// C++: `double MSAbstractLaneChangeModel::getLaneChangeCompletion() const` [const]
    /// Get the current lane change completion ratio
    // inline double getLaneChangeCompletion() const {
    //         return myLaneChangeCompletion;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_change_completion(&self) -> f64 {
        self.myLaneChangeCompletion
    }

    /// C++: `int MSAbstractLaneChangeModel::getLaneChangeDirection() const` [const]
    /// return the direction of the current lane change maneuver
    // inline int getLaneChangeDirection() const {
    //         return myLaneChangeDirection;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane_change_direction(&self) -> i32 {
        self.myLaneChangeDirection
    }

    /// C++: `int MSAbstractLaneChangeModel::getShadowDirection() const` [const]
    /// return the direction in which the current shadow lane lies
    // int
    // MSAbstractLaneChangeModel::getShadowDirection() const {
    //     if (isChangingLanes()) {
    //         if (pastMidpoint()) {
    //             return -myLaneChangeDirection;
    //         } else {
    //             return myLaneChangeDirection;
    //         }
    //     } else if (myShadowLane == nullptr) {
    //         return 0;
    //     } else if (myAmOpposite) {
    //         // return neigh-lane in forward direction
    //         return 1;
    //     } else if (&myShadowLane->getEdge() == &myVehicle.getLane()->getEdge()) {
    //         return myShadowLane->getIndex() - myVehicle.getLane()->getIndex();
    //     } else {
    //         // overlap with opposite direction lane
    //         return 1;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_shadow_direction(&self) -> i32 {
        if self.is_changing_lanes() {
            if self.past_midpoint() {
                -self.myLaneChangeDirection
            } else {
                self.myLaneChangeDirection
            }
        } else if self.myShadowLane.is_null() {
            0
        } else if self.myAmOpposite {
            // return neigh-lane in forward direction
            1
        } else if std::ptr::eq(self.myShadowLane.get_edge(), self.myVehicle.get_lane().get_edge()) {
            self.myShadowLane.get_index() - self.myVehicle.get_lane().get_index()
        } else {
            // overlap with opposite direction lane
            1
        }
    }

    /// C++: `double MSAbstractLaneChangeModel::calcAngleOffset()`
    /// return the angle offset during a continuous change maneuver
    // double
    // MSAbstractLaneChangeModel::calcAngleOffset() {
    //     double result = 0.;
    //     if (!(fabs(mySpeedLat) < NUMERICAL_EPS && fabs(myPreviousAngleOffset * 180 / M_PI) < NUMERICAL_EPS)) {
    //         if (myVehicle.getLength() < sqrt(SPEED2DIST(mySpeedLat) * SPEED2DIST(mySpeedLat) + SPEED2DIST(myVehicle.getSpeed()) * SPEED2DIST(myVehicle.getSpeed()))) {
    //             result = atan2(mySpeedLat, myVehicle.getSpeed());
    //         } else {
    //             result = myPreviousAngleOffset + asin((sin(M_PI / 2 - myPreviousAngleOffset) * (SPEED2DIST(mySpeedLat) - tan(myPreviousAngleOffset) * SPEED2DIST(myVehicle.getSpeed()))) / myVehicle.getLength());
    //         }
    //     }
    //
    //     myAngleOffset = result;
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn calc_angle_offset(&mut self) -> f64 {
        let mut result = 0.;
        if !(self.mySpeedLat.abs() < NUMERICAL_EPS && (self.myPreviousAngleOffset * 180. / std::f64::consts::PI).abs() < NUMERICAL_EPS) {
            let speed = self.myVehicle.get_speed();
            if self.myVehicle.get_length() < (speed2dist(self.mySpeedLat) * speed2dist(self.mySpeedLat) + speed2dist(speed) * speed2dist(speed)).sqrt() {
                result = self.mySpeedLat.atan2(speed);
            } else {
                result = self.myPreviousAngleOffset
                    + ((std::f64::consts::PI / 2. - self.myPreviousAngleOffset).sin()
                        * (speed2dist(self.mySpeedLat) - self.myPreviousAngleOffset.tan() * speed2dist(speed))
                        / self.myVehicle.get_length())
                    .asin();
            }
        }

        self.myAngleOffset = result;
        result
    }

    /// C++: `double MSAbstractLaneChangeModel::getAngleOffset() const` [const]
    /// return the angle offset resulting from lane change and sigma
    // inline double getAngleOffset() const {
    //         return myAngleOffset;
    //     }
    #[allow(unused_variables)]
    pub fn get_angle_offset(&self) -> f64 {
        self.myAngleOffset
    }

    /// C++: `void MSAbstractLaneChangeModel::setAngleOffset(const double)`
    /// set the angle offset resulting from lane change and sigma
    // inline void setAngleOffset(const double angleOffset) {
    //         myAngleOffset = angleOffset;
    //     }
    #[allow(unused_variables)]
    pub fn set_angle_offset(&mut self, angleOffset: f64) {
        self.myAngleOffset = angleOffset;
    }

    /// C++: `void MSAbstractLaneChangeModel::setPreviousAngleOffset(const double)`
    /// set the angle offset of the previous time step
    // inline void setPreviousAngleOffset(const double angleOffset) {
    //         myPreviousAngleOffset = angleOffset;
    //     }
    #[allow(unused_variables)]
    pub fn set_previous_angle_offset(&mut self, angleOffset: f64) {
        self.myPreviousAngleOffset = angleOffset;
    }

    /// C++: `bool MSAbstractLaneChangeModel::alreadyChanged() const` [const]
    /// reset the flag whether a vehicle already moved to false
    // inline bool alreadyChanged() const {
    //         return myAlreadyChanged;
    //     }
    #[allow(unused_variables)]
    pub fn already_changed(&self) -> bool {
        self.myAlreadyChanged
    }

    /// C++: `void MSAbstractLaneChangeModel::resetChanged()`
    /// reset the flag whether a vehicle already moved to false
    // void resetChanged() {
    //         myAlreadyChanged = false;
    //     }
    #[allow(unused_variables)]
    pub fn reset_changed(&mut self) {
        self.myAlreadyChanged = false;
    }

    /// C++: `bool MSAbstractLaneChangeModel::startLaneChangeManeuver(MSLane *, MSLane *, int)`
    /// start the lane change maneuver and return whether it continues
    // bool
    // MSAbstractLaneChangeModel::startLaneChangeManeuver(MSLane* source, MSLane* target, int direction) {
    //     if (MSGlobals::gLaneChangeDuration > DELTA_T) {
    //         myLaneChangeCompletion = 0;
    //         myLaneChangeDirection = direction;
    //         setManeuverDist((target->getWidth() + source->getWidth()) * 0.5 * direction);
    //         myVehicle.switchOffSignal(MSVehicle::VEH_SIGNAL_BLINKER_RIGHT | MSVehicle::VEH_SIGNAL_BLINKER_LEFT);
    //         myVehicle.switchOnSignal(((direction == 1) != MSGlobals::gLefthand) ? MSVehicle::VEH_SIGNAL_BLINKER_LEFT : MSVehicle::VEH_SIGNAL_BLINKER_RIGHT);
    //         if (myLCOutput) {
    //             memorizeGapsAtLCInit();
    //         }
    //         return true;
    //     } else {
    //         primaryLaneChanged(source, target, direction);
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn start_lane_change_maneuver(&mut self, source: Ptr<MSLane>, target: Ptr<MSLane>, direction: i32) -> bool {
        if MSGlobals::lane_change_duration() > delta_t() {
            self.myLaneChangeCompletion = 0.;
            self.myLaneChangeDirection = direction;
            self.set_maneuver_dist((target.get_width() + source.get_width()) * 0.5 * direction as f64);
            self.myVehicle.m().switch_off_signal(MSVehicle_Signalling::VEH_SIGNAL_BLINKER_RIGHT as i32 | MSVehicle_Signalling::VEH_SIGNAL_BLINKER_LEFT as i32);
            let signal = if (direction == 1) != MSGlobals::lefthand() {
                MSVehicle_Signalling::VEH_SIGNAL_BLINKER_LEFT as i32
            } else {
                MSVehicle_Signalling::VEH_SIGNAL_BLINKER_RIGHT as i32
            };
            self.myVehicle.m().switch_on_signal(signal);
            if MY_LC_OUTPUT.load(Ordering::Relaxed) {
                self.memorize_gaps_at_lc_init();
            }
            true
        } else {
            self.primary_lane_changed(source, target, direction);
            false
        }
    }

    /// C++: `void MSAbstractLaneChangeModel::memorizeGapsAtLCInit()`
    /// Control for resetting the memorized values for LC relevant gaps until the LC output is triggered in the case of continuous LC.
    // void
    // MSAbstractLaneChangeModel::memorizeGapsAtLCInit() {
    //     myDontResetLCGaps = true;
    // }
    #[allow(unused_variables)]
    pub fn memorize_gaps_at_lc_init(&mut self) {
        self.myDontResetLCGaps = true;
    }

    /// C++: `void MSAbstractLaneChangeModel::clearGapsAtLCInit()`
    // void
    // MSAbstractLaneChangeModel::clearGapsAtLCInit() {
    //     myDontResetLCGaps = false;
    // }
    #[allow(unused_variables)]
    pub fn clear_gaps_at_lc_init(&mut self) {
        self.myDontResetLCGaps = false;
    }

    /// C++: `bool MSAbstractLaneChangeModel::updateCompletion()`
    // bool
    // MSAbstractLaneChangeModel::updateCompletion() {
    //     const bool pastBefore = pastMidpoint();
    //     // maneuverDist is not updated in the context of continuous lane changing but represents the full LC distance
    //     double maneuverDist = getManeuverDist();
    //     setSpeedLat(computeSpeedLat(0, maneuverDist, (myOwnState & LCA_URGENT) != 0));
    //     myLaneChangeCompletion += (SPEED2DIST(mySpeedLat) / myManeuverDist);
    //     return !pastBefore && pastMidpoint();
    // }
    #[allow(unused_variables)]
    pub fn update_completion(&mut self) -> bool {
        let pastBefore = self.past_midpoint();
        // maneuverDist is not updated in the context of continuous lane changing but represents the full LC distance
        let mut maneuverDist = self.get_maneuver_dist();
        let speedLat = self.compute_speed_lat(0., &mut maneuverDist, (self.myOwnState & LCA_URGENT) != 0);
        self.set_speed_lat(speedLat);
        self.myLaneChangeCompletion += speed2dist(self.mySpeedLat) / self.myManeuverDist;
        !pastBefore && self.past_midpoint()
    }

    /// C++: `void MSAbstractLaneChangeModel::updateShadowLane()`
    // void
    // MSAbstractLaneChangeModel::updateShadowLane() {
    //     if (!MSGlobals::gSublane) {
    //         // assume each vehicle drives at the center of its lane and act as if it fits
    //         return;
    //     }
    //     if (myShadowLane != nullptr) {
    // #ifdef DEBUG_SHADOWLANE
    //         if (debugVehicle()) {
    //             std::cout << SIMTIME << " updateShadowLane()\n";
    //         }
    // #endif
    //         myShadowLane->resetPartialOccupation(&myVehicle);
    //     }
    //     myShadowLane = getShadowLane(myVehicle.getLane());
    //     std::vector<MSLane*> passed;
    //     if (myShadowLane != nullptr) {
    //         myShadowLane->setPartialOccupation(&myVehicle);
    //         const std::vector<MSLane*>& further = myVehicle.getFurtherLanes();
    //         if (myAmOpposite) {
    //             assert(further.size() == 0);
    //         } else {
    //             const std::vector<double>& furtherPosLat = myVehicle.getFurtherLanesPosLat();
    //             assert(further.size() == furtherPosLat.size());
    //             passed.push_back(myShadowLane);
    //             for (int i = 0; i < (int)further.size(); ++i) {
    //                 MSLane* shadowFurther = getShadowLane(further[i], furtherPosLat[i]);
    // #ifdef DEBUG_SHADOWLANE
    //                 if (debugVehicle()) {
    //                     std::cout << SIMTIME << "   further=" << further[i]->getID() << " (posLat=" << furtherPosLat[i] << ") shadowFurther=" << Named::getIDSecure(shadowFurther) << "\n";
    //                 }
    // #endif
    //                 if (shadowFurther != nullptr && shadowFurther->getLinkTo(passed.back()) != nullptr) {
    //                     passed.push_back(shadowFurther);
    //                 }
    //             }
    //             std::reverse(passed.begin(), passed.end());
    //         }
    //     } else {
    //         if (isChangingLanes() && myVehicle.getLateralOverlap() > NUMERICAL_EPS) {
    //             WRITE_WARNING("Vehicle '" + myVehicle.getID() + "' could not finish continuous lane change (lane disappeared) time=" +
    //                           time2string(MSNet::getInstance()->getCurrentTimeStep()) + ".");
    //             endLaneChangeManeuver();
    //         }
    //     }
    // #ifdef DEBUG_SHADOWLANE
    //     if (debugVehicle()) {
    //         std::cout << SIMTIME << " updateShadowLane() veh=" << myVehicle.getID()
    //                   << " newShadowLane=" << Named::getIDSecure(myShadowLane)
    //                   << "\n   before:" << " myShadowFurtherLanes=" << toString(myShadowFurtherLanes) << " further=" << toString(myVehicle.getFurtherLanes()) << " passed=" << toString(passed);
    //         std::cout << std::endl;
    //     }
    // #endif
    //     myVehicle.updateFurtherLanes(myShadowFurtherLanes, myShadowFurtherLanesPosLat, passed);
    // #ifdef DEBUG_SHADOWLANE
    //     if (debugVehicle()) std::cout
    //                 << "\n   after:" << " myShadowFurtherLanes=" << toString(myShadowFurtherLanes) << "\n";
    // #endif
    // }
    #[allow(unused_variables)]
    pub fn update_shadow_lane(&mut self) {
        // C++: if (!MSGlobals::gSublane) return; — without sublanes every vehicle sits at lane center
        if !crate::microsim::ms_globals::sublane() {
            return;
        }
        if !self.myShadowLane.is_null() {
            self.myShadowLane.m().reset_partial_occupation(self.myVehicle);
        }
        self.myShadowLane = self.get_shadow_lane_2(self.myVehicle.get_lane());
        if std::env::var_os("LC06_TRACE").is_some() && self.myVehicle.get_id().to_string() == "rgt0" {
            let shadow = if self.myShadowLane.is_null() { "null".to_string() } else { self.myShadowLane.get_id().to_string() };
            eprintln!("SH {} veh=rgt0 lat={:x} lane={} shadow={}", MSNet::get_instance().get_current_time_step(), self.myVehicle.get_lateral_position_on_lane().to_bits(), self.myVehicle.get_lane().get_id(), shadow);
        }
        let mut passed: Vec<Ptr<MSLane>> = Vec::new();
        if !self.myShadowLane.is_null() {
            self.myShadowLane.m().set_partial_occupation(self.myVehicle);
            let further = self.myVehicle.get_further_lanes();
            if self.myAmOpposite {
                debug_assert!(further.len() == 0);
            } else {
                let furtherPosLat = self.myVehicle.get_further_lanes_pos_lat();
                debug_assert!(further.len() == furtherPosLat.len());
                passed.push(self.myShadowLane);
                for i in 0..further.len() {
                    let shadowFurther = self.get_shadow_lane_3(further[i], furtherPosLat[i]);
                    if !shadowFurther.is_null() && !shadowFurther.get_link_to(*passed.last().unwrap()).is_null() {
                        passed.push(shadowFurther);
                    }
                }
                passed.reverse();
            }
        } else {
            if self.is_changing_lanes() && self.myVehicle.get_lateral_overlap() > NUMERICAL_EPS {
                utils::common::msg_handler::write_warning(&format!(
                    "Vehicle '{}' could not finish continuous lane change (lane disappeared) time={}.",
                    self.myVehicle.get_id(),
                    time2string(MSNet::get_instance().get_current_time_step())
                ));
                self.end_lane_change_maneuver(MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE);
            }
        }
        self.myVehicle.m().update_further_lanes(&mut self.myShadowFurtherLanes, &mut self.myShadowFurtherLanesPosLat, &passed);
    }

    /// C++: `MSLane * MSAbstractLaneChangeModel::updateTargetLane()`
    // MSLane*
    // MSAbstractLaneChangeModel::updateTargetLane() {
    // #ifdef DEBUG_TARGET_LANE
    //     MSLane* oldTarget = myTargetLane;
    //     std::vector<MSLane*> oldFurtherTargets = myFurtherTargetLanes;
    //     if (debugVehicle()) {
    //         std::cout << SIMTIME << " veh '" << myVehicle.getID() << "' (lane=" << myVehicle.getLane()->getID() << ") updateTargetLane()"
    //                   << "\n   oldTarget: " << (oldTarget == nullptr ? "NULL" : oldTarget->getID())
    //                   << " oldFurtherTargets: " << toString(oldFurtherTargets);
    //     }
    // #endif
    //     if (myTargetLane != nullptr) {
    //         myTargetLane->resetManeuverReservation(&myVehicle);
    //     }
    //     // Clear old further target lanes
    //     for (MSLane* oldTargetLane : myFurtherTargetLanes) {
    //         if (oldTargetLane != nullptr) {
    //             oldTargetLane->resetManeuverReservation(&myVehicle);
    //         }
    //     }
    //     myFurtherTargetLanes.clear();
    //
    //     // Get new target lanes and issue a maneuver reservation.
    //     int targetDir;
    //     myTargetLane = determineTargetLane(targetDir);
    //     if (myTargetLane != nullptr) {
    //         myTargetLane->setManeuverReservation(&myVehicle);
    //         // further targets are just the target lanes corresponding to the vehicle's further lanes
    //         // @note In a neglectable amount of situations we might add a reservation for a shadow further lane.
    //         for (MSLane* furtherLane : myVehicle.getFurtherLanes()) {
    //             MSLane* furtherTargetLane = furtherLane->getParallelLane(targetDir);
    //             myFurtherTargetLanes.push_back(furtherTargetLane);
    //             if (furtherTargetLane != nullptr) {
    //                 furtherTargetLane->setManeuverReservation(&myVehicle);
    //             }
    //         }
    //     }
    // #ifdef DEBUG_TARGET_LANE
    //     if (debugVehicle()) {
    //         std::cout << "\n   newTarget (maneuverDist=" << myManeuverDist << " offset=" << targetDir << "): " << (myTargetLane == nullptr ? "NULL" : myTargetLane->getID())
    //                   << " newFurtherTargets: " << toString(myFurtherTargetLanes)
    //                   << std::endl;
    //     }
    // #endif
    //     return myTargetLane;
    // }
    #[allow(unused_variables)]
    pub fn update_target_lane(&mut self) -> Ptr<MSLane> {
        if !self.myTargetLane.is_null() {
            self.myTargetLane.m().reset_maneuver_reservation(self.myVehicle);
        }
        // Clear old further target lanes
        for &oldTargetLane in self.myFurtherTargetLanes.iter() {
            if !oldTargetLane.is_null() {
                oldTargetLane.m().reset_maneuver_reservation(self.myVehicle);
            }
        }
        self.myFurtherTargetLanes.clear();

        // Get new target lanes and issue a maneuver reservation.
        let mut targetDir: i32 = 0;
        self.myTargetLane = self.determine_target_lane(&mut targetDir);
        if !self.myTargetLane.is_null() {
            self.myTargetLane.m().set_maneuver_reservation(self.myVehicle);
            // further targets are just the target lanes corresponding to the vehicle's further lanes
            // @note In a neglectable amount of situations we might add a reservation for a shadow further lane.
            for &furtherLane in self.myVehicle.get_further_lanes().iter() {
                let furtherTargetLane = furtherLane.get_parallel_lane(targetDir, true);
                self.myFurtherTargetLanes.push(furtherTargetLane);
                if !furtherTargetLane.is_null() {
                    furtherTargetLane.m().set_maneuver_reservation(self.myVehicle);
                }
            }
        }
        self.myTargetLane
    }

    /// C++: `MSLane * MSAbstractLaneChangeModel::determineTargetLane(int &) const` [const]
    // MSLane*
    // MSAbstractLaneChangeModel::determineTargetLane(int& targetDir) const {
    //     targetDir = 0;
    //     if (myManeuverDist == 0) {
    //         return nullptr;
    //     }
    //     // Current lateral boundaries of the vehicle
    //     const double vehRight = myVehicle.getLateralPositionOnLane() - 0.5 * myVehicle.getWidth();
    //     const double vehLeft = myVehicle.getLateralPositionOnLane() + 0.5 * myVehicle.getWidth();
    //     const double halfLaneWidth = 0.5 * myVehicle.getLane()->getWidth();
    //
    //     if (vehRight + myManeuverDist < -halfLaneWidth) {
    //         // Vehicle intends to traverse the right lane boundary
    //         targetDir = -1;
    //     } else if (vehLeft + myManeuverDist > halfLaneWidth) {
    //         // Vehicle intends to traverse the left lane boundary
    //         targetDir = 1;
    //     }
    //     if (targetDir == 0) {
    //         // Presently, no maneuvering into another lane is begun.
    //         return nullptr;
    //     }
    //     MSLane* target = myVehicle.getLane()->getParallelLane(targetDir);
    //     if (target == nullptr || target == myShadowLane) {
    //         return nullptr;
    //     } else {
    //         return target;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn determine_target_lane(&self, targetDir: &mut i32) -> Ptr<MSLane> {
        *targetDir = 0;
        if self.myManeuverDist == 0. {
            return Ptr::null();
        }
        // Current lateral boundaries of the vehicle
        let vehRight = self.myVehicle.get_lateral_position_on_lane() - 0.5 * self.myVehicle.get_width();
        let vehLeft = self.myVehicle.get_lateral_position_on_lane() + 0.5 * self.myVehicle.get_width();
        let halfLaneWidth = 0.5 * self.myVehicle.get_lane().get_width();

        if vehRight + self.myManeuverDist < -halfLaneWidth {
            // Vehicle intends to traverse the right lane boundary
            *targetDir = -1;
        } else if vehLeft + self.myManeuverDist > halfLaneWidth {
            // Vehicle intends to traverse the left lane boundary
            *targetDir = 1;
        }
        if *targetDir == 0 {
            // Presently, no maneuvering into another lane is begun.
            return Ptr::null();
        }
        let target = self.myVehicle.get_lane().get_parallel_lane(*targetDir, true);
        if target.is_null() || target == self.myShadowLane {
            Ptr::null()
        } else {
            target
        }
    }

    /// C++: `void MSAbstractLaneChangeModel::endLaneChangeManeuver(const MSMoveReminder::Notification)`
    // void
    // MSAbstractLaneChangeModel::endLaneChangeManeuver(const MSMoveReminder::Notification reason) {
    //     UNUSED_PARAMETER(reason);
    //     myLaneChangeCompletion = 1;
    //     cleanupShadowLane();
    //     cleanupTargetLane();
    //     myNoPartiallyOccupatedByShadow.clear();
    //     myVehicle.switchOffSignal(MSVehicle::VEH_SIGNAL_BLINKER_RIGHT | MSVehicle::VEH_SIGNAL_BLINKER_LEFT);
    //     myVehicle.fixPosition();
    //     if (myAmOpposite && reason != MSMoveReminder::NOTIFICATION_LANE_CHANGE) {
    //         if (reason == MSMoveReminder::NOTIFICATION_PARKING && myVehicle.getNextStop().isOpposite) {
    //             // opposite driving continues after parking
    //         } else {
    //             // aborted maneuver
    // #ifdef DEBUG_OPPOSITE
    //             if (debugVehicle()) {
    //                 std::cout << SIMTIME << " veh=" << myVehicle.getID() << " aborted maneuver (no longer opposite)\n";
    //             }
    // #endif
    //             changedToOpposite();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn end_lane_change_maneuver(&mut self, reason: MSMoveReminder_Notification) {
        // UNUSED_PARAMETER(reason);
        self.myLaneChangeCompletion = 1.;
        self.cleanup_shadow_lane();
        self.cleanup_target_lane();
        self.myNoPartiallyOccupatedByShadow.clear();
        self.myVehicle.m().switch_off_signal(MSVehicle_Signalling::VEH_SIGNAL_BLINKER_RIGHT as i32 | MSVehicle_Signalling::VEH_SIGNAL_BLINKER_LEFT as i32);
        self.myVehicle.m().fix_position();
        if self.myAmOpposite && reason != MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE {
            if reason == MSMoveReminder_Notification::NOTIFICATION_PARKING && self.myVehicle.get_next_stop().isOpposite {
                // opposite driving continues after parking
            } else {
                // aborted maneuver
                self.changed_to_opposite();
            }
        }
    }

    /// C++: `void MSAbstractLaneChangeModel::cleanupShadowLane()`
    // void
    // MSAbstractLaneChangeModel::cleanupShadowLane() {
    //     if (myShadowLane != nullptr) {
    //         if (debugVehicle()) {
    //             std::cout << SIMTIME << " cleanupShadowLane\n";
    //         }
    //         myShadowLane->resetPartialOccupation(&myVehicle);
    //         myShadowLane = nullptr;
    //     }
    //     for (MSLane* further : myShadowFurtherLanes) {
    //         if (debugVehicle()) {
    //             std::cout << SIMTIME << " cleanupShadowLane2\n";
    //         }
    //         further->resetPartialOccupation(&myVehicle);
    //         if (further->getBidiLane() != nullptr) {
    //             further->getBidiLane()->resetPartialOccupation(&myVehicle);
    //         }
    //     }
    //     myShadowFurtherLanes.clear();
    //     myNoPartiallyOccupatedByShadow.clear();
    // }
    #[allow(unused_variables)]
    pub fn cleanup_shadow_lane(&mut self) {
        if !self.myShadowLane.is_null() {
            self.myShadowLane.m().reset_partial_occupation(self.myVehicle);
            self.myShadowLane = Ptr::null();
        }
        for &further in self.myShadowFurtherLanes.iter() {
            further.m().reset_partial_occupation(self.myVehicle);
            if !further.get_bidi_lane().is_null() {
                further.get_bidi_lane().m().reset_partial_occupation(self.myVehicle);
            }
        }
        self.myShadowFurtherLanes.clear();
        self.myNoPartiallyOccupatedByShadow.clear();
    }

    /// C++: `void MSAbstractLaneChangeModel::cleanupTargetLane()`
    // void
    // MSAbstractLaneChangeModel::cleanupTargetLane() {
    //     if (myTargetLane != nullptr) {
    //         if (debugVehicle()) {
    //             std::cout << SIMTIME << " cleanupTargetLane\n";
    //         }
    //         myTargetLane->resetManeuverReservation(&myVehicle);
    //         myTargetLane = nullptr;
    //     }
    //     for (std::vector<MSLane*>::const_iterator it = myFurtherTargetLanes.begin(); it != myFurtherTargetLanes.end(); ++it) {
    //         if (debugVehicle()) {
    //             std::cout << SIMTIME << " cleanupTargetLane\n";
    //         }
    //         if (*it != nullptr) {
    //             (*it)->resetManeuverReservation(&myVehicle);
    //         }
    //     }
    //     myFurtherTargetLanes.clear();
    // //    myNoPartiallyOccupatedByShadow.clear();
    // }
    #[allow(unused_variables)]
    pub fn cleanup_target_lane(&mut self) {
        if !self.myTargetLane.is_null() {
            self.myTargetLane.m().reset_maneuver_reservation(self.myVehicle);
            self.myTargetLane = Ptr::null();
        }
        for &further in self.myFurtherTargetLanes.iter() {
            if !further.is_null() {
                further.m().reset_maneuver_reservation(self.myVehicle);
            }
        }
        self.myFurtherTargetLanes.clear();
    }

    /// C++: `bool MSAbstractLaneChangeModel::saveBlockerLength(double, double)` [virtual]
    /// reserve space at the end of the lane to avoid dead locks
    // virtual bool saveBlockerLength(double /* length */, double /* foeLeftSpace */) {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn save_blocker_length(&mut self, arg0: f64, arg1: f64) -> bool {
        (self.vt.save_blocker_length)(self, arg0, arg1)
    }

    pub fn save_blocker_length_base(&mut self, arg0: f64, arg1: f64) -> bool {
        true
    }

    /// C++: `void MSAbstractLaneChangeModel::setShadowPartialOccupator(MSLane *)`
    // void setShadowPartialOccupator(MSLane* lane) {
    //         myPartiallyOccupatedByShadow.push_back(lane);
    //     }
    #[allow(unused_variables)]
    pub fn set_shadow_partial_occupator(&mut self, lane: Ptr<MSLane>) {
        self.myPartiallyOccupatedByShadow.push(lane);
    }

    /// C++: `void MSAbstractLaneChangeModel::setNoShadowPartialOccupator(MSLane *)`
    // void setNoShadowPartialOccupator(MSLane* lane) {
    //         myNoPartiallyOccupatedByShadow.push_back(lane);
    //     }
    #[allow(unused_variables)]
    pub fn set_no_shadow_partial_occupator(&mut self, lane: Ptr<MSLane>) {
        self.myNoPartiallyOccupatedByShadow.push(lane);
    }

    /// C++: `void MSAbstractLaneChangeModel::primaryLaneChanged(MSLane *, MSLane *, int)`
    /// called once when the vehicles primary lane changes
    // void
    // MSAbstractLaneChangeModel::primaryLaneChanged(MSLane* source, MSLane* target, int direction) {
    //     initLastLaneChangeOffset(direction);
    //     myVehicle.leaveLane(MSMoveReminder::NOTIFICATION_LANE_CHANGE, target);
    //     source->leftByLaneChange(&myVehicle);
    //     laneChangeOutput("change", source, target, direction); // record position on the source edge in case of opposite change
    //     if (&source->getEdge() != &target->getEdge()) {
    //         changedToOpposite();
    // #ifdef DEBUG_OPPOSITE
    //         if (debugVehicle()) {
    //             std::cout << SIMTIME << " veh=" << myVehicle.getID() << " primaryLaneChanged source=" << source->getID() << " target=" << target->getID() << " nowOpposite=" << myAmOpposite << "\n";
    //         }
    // #endif
    //         myVehicle.setTentativeLaneAndPosition(target, source->getOppositePos(myVehicle.getPositionOnLane()), -myVehicle.getLateralPositionOnLane());
    //         target->forceVehicleInsertion(&myVehicle, myVehicle.getPositionOnLane(), MSMoveReminder::NOTIFICATION_LANE_CHANGE, myVehicle.getLateralPositionOnLane());
    //     } else if (myAmOpposite) {
    // #ifdef DEBUG_OPPOSITE
    //         if (debugVehicle()) {
    //             std::cout << SIMTIME << " veh=" << myVehicle.getID() << " primaryLaneChanged source=" << source->getID() << " target=" << target->getID() << " stayOpposite\n";
    //         }
    // #endif
    //         myAlreadyChanged = true;
    //         myVehicle.setTentativeLaneAndPosition(target, myVehicle.getPositionOnLane(), myVehicle.getLateralPositionOnLane());
    //         if (!MSGlobals::gSublane) {
    //             // in the continous case, the vehicle is added to the target lane via MSLaneChanger::continueChange / registerHop
    //             // in the sublane case, the vehicle is added to the target lane via MSLaneChangerSublane::checkChangeOppositeSublane / MSLane::myTmpVehicles
    //             target->forceVehicleInsertion(&myVehicle, myVehicle.getPositionOnLane(), MSMoveReminder::NOTIFICATION_LANE_CHANGE, myVehicle.getLateralPositionOnLane());
    //         }
    //     } else {
    //         myVehicle.enterLaneAtLaneChange(target);
    //         target->enteredByLaneChange(&myVehicle);
    //     }
    //     // Assure that the drive items are up to date (even if the following step is no actionstep for the vehicle).
    //     // This is necessary because the lane advance uses the target lane from the corresponding drive item.
    //     myVehicle.updateDriveItems();
    //     changed();
    // }
    #[allow(unused_variables)]
    pub fn primary_lane_changed(&mut self, source: Ptr<MSLane>, target: Ptr<MSLane>, direction: i32) {
        self.init_last_lane_change_offset(direction);
        self.myVehicle.m().leave_lane(MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE, target);
        source.m().left_by_lane_change(self.myVehicle);
        self.lane_change_output("change", source, target, direction, 0.); // record position on the source edge in case of opposite change
        if !std::ptr::eq(source.get_edge() as *const MSEdge, target.get_edge() as *const MSEdge) {
            self.changed_to_opposite();
            let pos = source.get_opposite_pos(self.myVehicle.get_position_on_lane());
            self.myVehicle.m().set_tentative_lane_and_position(target, pos, -self.myVehicle.get_lateral_position_on_lane());
            target.m().force_vehicle_insertion(
                self.myVehicle,
                self.myVehicle.get_position_on_lane(),
                MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE,
                self.myVehicle.get_lateral_position_on_lane(),
            );
        } else if self.myAmOpposite {
            self.myAlreadyChanged = true;
            self.myVehicle.m().set_tentative_lane_and_position(target, self.myVehicle.get_position_on_lane(), self.myVehicle.get_lateral_position_on_lane());
            if !MSGlobals::sublane() {
                // in the continous case, the vehicle is added to the target lane via MSLaneChanger::continueChange / registerHop
                // in the sublane case, the vehicle is added to the target lane via MSLaneChangerSublane::checkChangeOppositeSublane / MSLane::myTmpVehicles
                target.m().force_vehicle_insertion(
                    self.myVehicle,
                    self.myVehicle.get_position_on_lane(),
                    MSMoveReminder_Notification::NOTIFICATION_LANE_CHANGE,
                    self.myVehicle.get_lateral_position_on_lane(),
                );
            }
        } else {
            self.myVehicle.m().enter_lane_at_lane_change(target);
            target.m().entered_by_lane_change(self.myVehicle);
        }
        // Assure that the drive items are up to date (even if the following step is no actionstep for the vehicle).
        // This is necessary because the lane advance uses the target lane from the corresponding drive item.
        self.myVehicle.m().update_drive_items();
        self.changed();
    }

    /// C++: `void MSAbstractLaneChangeModel::laneChangeOutput(const std::string &, MSLane *, MSLane *, int, double)`
    /// called once the vehicle ends a lane change manoeuvre (non-instant)
    // void
    // MSAbstractLaneChangeModel::laneChangeOutput(const std::string& tag, MSLane* source, MSLane* target, int direction, double maneuverDist) {
    //     if (myLCOutput) {
    //         OutputDevice& of = OutputDevice::getDeviceByOption("lanechange-output");
    //         of.openTag(tag);
    //         of.writeAttr(SUMO_ATTR_ID, myVehicle.getID());
    //         of.writeAttr(SUMO_ATTR_TYPE, myVehicle.getVehicleType().getID());
    //         of.writeAttr(SUMO_ATTR_TIME, time2string(MSNet::getInstance()->getCurrentTimeStep()));
    //         of.writeAttr(SUMO_ATTR_FROM, source->getID());
    //         of.writeAttr(SUMO_ATTR_TO, target->getID());
    //         of.writeAttr(SUMO_ATTR_DIR, direction);
    //         of.writeAttr(SUMO_ATTR_SPEED, myVehicle.getSpeed());
    //         of.writeAttr(SUMO_ATTR_POSITION, myVehicle.getPositionOnLane());
    //         of.writeAttr("reason", toString((LaneChangeAction)(myOwnState & ~(
    //                                             LCA_RIGHT | LCA_LEFT
    //                                             | LCA_AMBLOCKINGLEADER | LCA_AMBLOCKINGFOLLOWER
    //                                             | LCA_MRIGHT | LCA_MLEFT
    //                                             | LCA_AMBACKBLOCKER | LCA_AMBACKBLOCKER_STANDING))) + myVehicle.getParameter().getParameter("lcReason"));
    //         of.writeAttr("leaderGap", myLastLeaderGap == NO_NEIGHBOR ? "None" : toString(myLastLeaderGap), myLastLeaderGap == NO_NEIGHBOR);
    //         of.writeAttr("leaderSecureGap", myLastLeaderSecureGap == NO_NEIGHBOR ? "None" : toString(myLastLeaderSecureGap), myLastLeaderSecureGap == NO_NEIGHBOR);
    //         of.writeAttr("leaderSpeed", myLastLeaderSpeed == NO_NEIGHBOR ? "None" : toString(myLastLeaderSpeed), myLastLeaderSpeed == NO_NEIGHBOR);
    //         of.writeAttr("followerGap", myLastFollowerGap == NO_NEIGHBOR ? "None" : toString(myLastFollowerGap), myLastFollowerGap == NO_NEIGHBOR);
    //         of.writeAttr("followerSecureGap", myLastFollowerSecureGap == NO_NEIGHBOR ? "None" : toString(myLastFollowerSecureGap), myLastFollowerSecureGap == NO_NEIGHBOR);
    //         of.writeAttr("followerSpeed", myLastFollowerSpeed == NO_NEIGHBOR ? "None" : toString(myLastFollowerSpeed), myLastFollowerSpeed == NO_NEIGHBOR);
    //         of.writeAttr("origLeaderGap", myLastOrigLeaderGap == NO_NEIGHBOR ? "None" : toString(myLastOrigLeaderGap), myLastOrigLeaderGap == NO_NEIGHBOR);
    //         of.writeAttr("origLeaderSecureGap", myLastOrigLeaderSecureGap == NO_NEIGHBOR ? "None" : toString(myLastOrigLeaderSecureGap), myLastOrigLeaderSecureGap == NO_NEIGHBOR);
    //         of.writeAttr("origLeaderSpeed", myLastOrigLeaderSpeed == NO_NEIGHBOR ? "None" : toString(myLastOrigLeaderSpeed), myLastOrigLeaderSpeed == NO_NEIGHBOR);
    //         if (MSGlobals::gLateralResolution > 0) {
    //             const double latGap = direction < 0 ? myLastLateralGapRight : myLastLateralGapLeft;
    //             of.writeAttr("latGap", latGap == NO_NEIGHBOR ? "None" : toString(latGap), latGap == NO_NEIGHBOR);
    //             if (maneuverDist != 0) {
    //                 of.writeAttr("maneuverDistance", toString(maneuverDist));
    //             }
    //         }
    //         if (myLCXYOutput) {
    //             of.writeAttr(SUMO_ATTR_X, myVehicle.getPosition().x());
    //             of.writeAttr(SUMO_ATTR_Y, myVehicle.getPosition().y());
    //         }
    //         of.closeTag();
    //         if (MSGlobals::gLaneChangeDuration > DELTA_T) {
    //             clearGapsAtLCInit();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn lane_change_output(&mut self, tag: &str, source: Ptr<MSLane>, target: Ptr<MSLane>, direction: i32, maneuverDist: f64) {
        if MY_LC_OUTPUT.load(Ordering::Relaxed) {
            let ofRef = OutputDevice::get_device_by_option("lanechange-output").unwrap();
            let mut of = ofRef.borrow_mut();
            of.open_tag(tag);
            of.write_attr(SUMO_ATTR_ID, self.myVehicle.get_id());
            of.write_attr(SUMO_ATTR_TYPE, self.myVehicle.get_vehicle_type().get_id());
            of.write_attr(SUMO_ATTR_TIME, &time2string(MSNet::get_instance().get_current_time_step()));
            of.write_attr(SUMO_ATTR_FROM, source.get_id());
            of.write_attr(SUMO_ATTR_TO, target.get_id());
            of.write_attr(SUMO_ATTR_DIR, &direction);
            of.write_attr(SUMO_ATTR_SPEED, &self.myVehicle.get_speed());
            of.write_attr(SUMO_ATTR_POSITION, &self.myVehicle.get_position_on_lane());
            // C++ toString(LaneChangeAction): the names of all set bits, joined by '|'
            let action = self.myOwnState & !(LCA_RIGHT | LCA_LEFT | LCA_AMBLOCKINGLEADER | LCA_AMBLOCKINGFOLLOWER | LCA_MRIGHT | LCA_MLEFT | LCA_AMBACKBLOCKER | LCA_AMBACKBLOCKER_STANDING);
            let mut actionString = String::new();
            let mut hadOne = false;
            for name in LaneChangeActions.get_strings() {
                if (action & LaneChangeActions.get(&name).unwrap_or(0)) != 0 {
                    if hadOne {
                        actionString.push('|');
                    } else {
                        hadOne = true;
                    }
                    actionString.push_str(&name);
                }
            }
            let reason = format!("{}{}", actionString, self.myVehicle.get_parameter().get_parameter("lcReason", ""));
            of.write_attr("reason", &reason);
            // C++ writeAttr(name, value, isNull) always writes in XML output: NO_NEIGHBOR is written as "None"
            let gap = |v: f64| if v == NO_NEIGHBOR { "None".to_string() } else { f64_to_string(v) };
            of.write_attr("leaderGap", &gap(self.myLastLeaderGap));
            of.write_attr("leaderSecureGap", &gap(self.myLastLeaderSecureGap));
            of.write_attr("leaderSpeed", &gap(self.myLastLeaderSpeed));
            of.write_attr("followerGap", &gap(self.myLastFollowerGap));
            of.write_attr("followerSecureGap", &gap(self.myLastFollowerSecureGap));
            of.write_attr("followerSpeed", &gap(self.myLastFollowerSpeed));
            of.write_attr("origLeaderGap", &gap(self.myLastOrigLeaderGap));
            of.write_attr("origLeaderSecureGap", &gap(self.myLastOrigLeaderSecureGap));
            of.write_attr("origLeaderSpeed", &gap(self.myLastOrigLeaderSpeed));
            if MSGlobals::lateral_resolution() > 0. {
                let latGap = if direction < 0 { self.myLastLateralGapRight } else { self.myLastLateralGapLeft };
                of.write_attr("latGap", &gap(latGap));
                if maneuverDist != 0. {
                    of.write_attr("maneuverDistance", &f64_to_string(maneuverDist));
                }
            }
            if MY_LC_XY_OUTPUT.load(Ordering::Relaxed) {
                let pos = self.myVehicle.get_position(0.);
                of.write_attr(SUMO_ATTR_X, &pos.x());
                of.write_attr(SUMO_ATTR_Y, &pos.y());
            }
            of.close_tag("");
            drop(of);
            if MSGlobals::lane_change_duration() > delta_t() {
                self.clear_gaps_at_lc_init();
            }
        }
    }

    /// C++: `bool MSAbstractLaneChangeModel::sublaneChangeCompleted(const double) const` [const, virtual]
    /// whether the current change completes the manoeuvre
    // virtual bool sublaneChangeCompleted(const double latDist) const {
    //         UNUSED_PARAMETER(latDist);
    //         throw ProcessError("Method not implemented by model " + toString(myModel));
    //     }
    #[allow(unused_variables)]
    pub fn sublane_change_completed(&self, latDist: f64) -> bool {
        panic!(
            "{}",
            ProcessError::Process(format!("Method not implemented by model {}", LaneChangeModels.get_string(self.myModel)))
        );
    }

    /// C++: `void MSAbstractLaneChangeModel::setShadowApproachingInformation(MSLink *) const` [const]
    /// set approach information for the shadow vehicle
    // void
    // MSAbstractLaneChangeModel::setShadowApproachingInformation(MSLink* link) const {
    //     //std::cout << SIMTIME << " veh=" << myVehicle.getID() << " @=" << &myVehicle << " set shadow approaching=" << link->getViaLaneOrLane()->getID() << "\n";
    //     myApproachedByShadow.push_back(link);
    // }
    #[allow(unused_variables)]
    pub fn set_shadow_approaching_information(&self, link: Ptr<MSLink>) {
        self.myApproachedByShadow.borrow_mut().push(link);
    }

    /// C++: `void MSAbstractLaneChangeModel::removeShadowApproachingInformation() const` [const]
    // void
    // MSAbstractLaneChangeModel::removeShadowApproachingInformation() const {
    //     for (std::vector<MSLink*>::iterator it = myApproachedByShadow.begin(); it != myApproachedByShadow.end(); ++it) {
    //         //std::cout << SIMTIME << " veh=" << myVehicle.getID() << " @=" << &myVehicle << " remove shadow approaching=" << (*it)->getViaLaneOrLane()->getID() << "\n";
    //         (*it)->removeApproaching(&myVehicle);
    //     }
    //     myApproachedByShadow.clear();
    // }
    #[allow(unused_variables)]
    pub fn remove_shadow_approaching_information(&self) {
        for &link in self.myApproachedByShadow.borrow().iter() {
            link.m().remove_approaching(self.myVehicle.cast::<SUMOVehicle>());
        }
        self.myApproachedByShadow.borrow_mut().clear();
    }

    /// C++: `bool MSAbstractLaneChangeModel::isOpposite() const` [const]
    // bool isOpposite() const {
    //         return myAmOpposite;
    //     }
    #[allow(unused_variables)]
    pub fn is_opposite(&self) -> bool {
        self.myAmOpposite
    }

    /// C++: `int MSAbstractLaneChangeModel::getNormalizedLaneIndex()`
    /// brief return lane index that treats opposite lanes like normal lanes to the left of the forward lanes
    // int
    // MSAbstractLaneChangeModel::getNormalizedLaneIndex() {
    //     const int i = myVehicle.getLane()->getIndex();
    //     if (myAmOpposite) {
    //         return myVehicle.getLane()->getParallelOpposite()->getEdge().getNumLanes() + myVehicle.getLane()->getEdge().getNumLanes() - 1 - i;
    //     } else {
    //         return i;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_normalized_lane_index(&mut self) -> i32 {
        let i = self.myVehicle.get_lane().get_index();
        if self.myAmOpposite {
            self.myVehicle.get_lane().get_parallel_opposite().get_edge().get_num_lanes() + self.myVehicle.get_lane().get_edge().get_num_lanes() - 1 - i
        } else {
            i
        }
    }

    /// C++: `double MSAbstractLaneChangeModel::getCommittedSpeed() const` [const]
    // double getCommittedSpeed() const {
    //         return myCommittedSpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_committed_speed(&self) -> f64 {
        self.myCommittedSpeed
    }

    /// C++: `double MSAbstractLaneChangeModel::getSpeedLat() const` [const]
    /// return the lateral speed of the current lane change maneuver
    // double getSpeedLat() const {
    //         return mySpeedLat;
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_lat(&self) -> f64 {
        self.mySpeedLat
    }

    /// C++: `void MSAbstractLaneChangeModel::resetSpeedLat()` [virtual]
    // void
    // MSAbstractLaneChangeModel::resetSpeedLat() {
    //     if (MSGlobals::gLaneChangeDuration > 0 && !isChangingLanes()) {
    //         setSpeedLat(0);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_speed_lat(&mut self) {
        (self.vt.reset_speed_lat)(self)
    }

    pub fn reset_speed_lat_base(&mut self) {
        if MSGlobals::lane_change_duration() > 0 && !self.is_changing_lanes() {
            self.set_speed_lat(0.);
        }
    }

    /// C++: `double MSAbstractLaneChangeModel::getAccelerationLat() const` [const]
    /// return the lateral speed of the current lane change maneuver
    // double getAccelerationLat() const {
    //         return myAccelerationLat;
    //     }
    #[allow(unused_variables)]
    pub fn get_acceleration_lat(&self) -> f64 {
        self.myAccelerationLat
    }

    /// C++: `double MSAbstractLaneChangeModel::getStrategicLookahead() const` [const]
    // double getStrategicLookahead() const {
    //         return myStrategicLookahead;
    //     }
    #[allow(unused_variables)]
    pub fn get_strategic_lookahead(&self) -> f64 {
        self.myStrategicLookahead
    }

    /// C++: `void MSAbstractLaneChangeModel::setSpeedLat(double)`
    /// set the lateral speed and update lateral acceleraton
    // void
    // MSAbstractLaneChangeModel::setSpeedLat(double speedLat) {
    //     myAccelerationLat = SPEED2ACCEL(speedLat - mySpeedLat);
    //     mySpeedLat = speedLat;
    // }
    #[allow(unused_variables)]
    pub fn set_speed_lat(&mut self, speedLat: f64) {
        self.myAccelerationLat = speed2accel(speedLat - self.mySpeedLat);
        self.mySpeedLat = speedLat;
    }

    /// C++: `double MSAbstractLaneChangeModel::computeSpeedLat(double, double &, bool) const` [const, virtual]
    /// decides the next lateral speed depending on the remaining lane change distance to be covered and updates maneuverDist according to lateral safety constraints.
    // double
    // MSAbstractLaneChangeModel::computeSpeedLat(double /*latDist*/, double& maneuverDist, bool /*urgent*/) const {
    //     if (myVehicle.getVehicleType().wasSet(VTYPEPARS_MAXSPEED_LAT_SET)) {
    //         int stepsToChange = (int)ceil(fabs(maneuverDist) / SPEED2DIST(myVehicle.getVehicleType().getMaxSpeedLat()));
    //         return DIST2SPEED(maneuverDist / stepsToChange);
    //     } else {
    //         return maneuverDist / STEPS2TIME(MSGlobals::gLaneChangeDuration);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn compute_speed_lat(&self, latDist: f64, maneuverDist: &mut f64, urgent: bool) -> f64 {
        (self.vt.compute_speed_lat)(self, latDist, maneuverDist, urgent)
    }

    pub fn compute_speed_lat_base(&self, latDist: f64, maneuverDist: &mut f64, urgent: bool) -> f64 {
        if self.myVehicle.get_vehicle_type().was_set(VTYPEPARS_MAXSPEED_LAT_SET) {
            let stepsToChange = (maneuverDist.abs() / speed2dist(self.myVehicle.get_vehicle_type().get_max_speed_lat())).ceil() as i32;
            dist2speed(*maneuverDist / stepsToChange as f64)
        } else {
            *maneuverDist / steps2time(MSGlobals::lane_change_duration())
        }
    }

    /// C++: `double MSAbstractLaneChangeModel::getAssumedDecelForLaneChangeDuration() const` [const, virtual]
    /// Returns a deceleration value which is used for the estimation of the duration of a lane change.
    // double
    // MSAbstractLaneChangeModel::getAssumedDecelForLaneChangeDuration() const {
    //     return MAX2(LC_ASSUMED_DECEL, -myVehicle.getAcceleration());
    // }
    #[allow(unused_variables)]
    pub fn get_assumed_decel_for_lane_change_duration(&self) -> f64 {
        // C++ #define LC_ASSUMED_DECEL 1.0 (MSAbstractLaneChangeModel.cpp)
        let a = 1.0;
        let b = -self.myVehicle.get_acceleration();
        if a > b { a } else { b }
    }

    /// C++: `std::string MSAbstractLaneChangeModel::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this laneChangeModel. Throw exception for unsupported key
    // virtual std::string getParameter(const std::string& key) const {
    //         throw InvalidArgument("Parameter '" + key + "' is not supported for laneChangeModel of type '" + toString(myModel) + "'");
    //     }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        (self.vt.get_parameter)(self, key)
    }

    pub fn get_parameter_base(&self, key: &str) -> String {
        panic!(
            "{}",
            ProcessError::InvalidArgument(format!(
                "Parameter '{}' is not supported for laneChangeModel of type '{}'",
                key,
                LaneChangeModels.get_string(self.myModel)
            ))
        );
    }

    /// C++: `void MSAbstractLaneChangeModel::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this laneChangeModel. Throw exception for unsupported key
    // virtual void setParameter(const std::string& key, const std::string& value) {
    //         UNUSED_PARAMETER(value);
    //         throw InvalidArgument("Setting parameter '" + key + "' is not supported for laneChangeModel of type '" + toString(myModel) + "'");
    //     }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        (self.vt.set_parameter)(self, key, value)
    }

    pub fn set_parameter_base(&mut self, key: &str, value: &str) {
        panic!(
            "{}",
            ProcessError::InvalidArgument(format!(
                "Setting parameter '{}' is not supported for laneChangeModel of type '{}'",
                key,
                LaneChangeModels.get_string(self.myModel)
            ))
        );
    }

    /// C++: `double MSAbstractLaneChangeModel::getExtraReservation(int, double) const` [const, virtual]
    // double
    // MSAbstractLaneChangeModel::getExtraReservation(int bestLaneOffset, double neighExtraDist) const {
    //     if (neighExtraDist > myVehicle.getVehicleType().getLengthWithGap()) {
    //         return 0;
    //     }
    //     if (bestLaneOffset < -1) {
    //         return 20;
    //     } else if (bestLaneOffset > 1) {
    //         return 40;
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_extra_reservation(&self, bestLaneOffset: i32, neighExtraDist: f64) -> f64 {
        (self.vt.get_extra_reservation)(self, bestLaneOffset, neighExtraDist)
    }

    pub fn get_extra_reservation_base(&self, bestLaneOffset: i32, neighExtraDist: f64) -> f64 {
        if neighExtraDist > self.myVehicle.get_vehicle_type().get_length_with_gap() {
            return 0.;
        }
        if bestLaneOffset < -1 {
            return 20.;
        } else if bestLaneOffset > 1 {
            return 40.;
        }
        0.
    }

    /// C++: `void MSAbstractLaneChangeModel::checkTraCICommands()`
    /// Check for commands issued for the vehicle via TraCI and apply the appropriate state changes For the sublane case, this includes setting a new maneuver distance if appropriate.
    // void
    // MSAbstractLaneChangeModel::checkTraCICommands() {
    //     int newstate = myVehicle.influenceChangeDecision(myOwnState);
    //     int oldstate = myVehicle.getLaneChangeModel().getOwnState();
    //     if (myOwnState != newstate) {
    //         if (MSGlobals::gLateralResolution > 0.) {
    //             // Calculate and set the lateral maneuver distance corresponding to the change request
    //             // to induce a corresponding sublane change.
    //             const int dir = (newstate & LCA_RIGHT) != 0 ? -1 : ((newstate & LCA_LEFT) != 0 ? 1 : 0);
    //             // minimum distance to move the vehicle fully onto the lane at offset dir
    //             const double latLaneDist = myVehicle.lateralDistanceToLane(dir);
    //             if ((newstate & LCA_TRACI) != 0) {
    //                 if ((newstate & LCA_STAY) != 0) {
    //                     setManeuverDist(0.);
    //                 } else if (((newstate & LCA_RIGHT) != 0 && dir < 0)
    //                            || ((newstate & LCA_LEFT) != 0 && dir > 0)) {
    //                     setManeuverDist(latLaneDist);
    //                 }
    //             }
    //             if (myVehicle.hasInfluencer()) {
    //                 // lane change requests override sublane change requests
    //                 myVehicle.getInfluencer().resetLatDist();
    //             }
    //
    //         }
    //         setOwnState(newstate);
    //     } else {
    //         // Check for sublane change requests
    //         if (myVehicle.hasInfluencer() && myVehicle.getInfluencer().getLatDist() != 0) {
    //             const double maneuverDist = myVehicle.getInfluencer().getLatDist();
    //             myVehicle.getLaneChangeModel().setManeuverDist(maneuverDist);
    //             myVehicle.getInfluencer().resetLatDist();
    //             newstate |= LCA_TRACI;
    //             if (myOwnState != newstate) {
    //                 setOwnState(newstate);
    //             }
    //             if (gDebugFlag2) {
    //                 std::cout << "     traci influenced maneuverDist=" << maneuverDist << "\n";
    //             }
    //         }
    //     }
    //     if (gDebugFlag2) {
    //         std::cout << SIMTIME << " veh=" << myVehicle.getID() << " stateAfterTraCI=" << toString((LaneChangeAction)newstate) << " original=" << toString((LaneChangeAction)oldstate) << "\n";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn check_tra_ci_commands(&mut self) {
        // gDebugFlag2 std::cout diagnostics are not ported; they do not affect simulation results
        let mut newstate = self.myVehicle.m().influence_change_decision(self.myOwnState);
        if self.myOwnState != newstate {
            if MSGlobals::lateral_resolution() > 0. {
                // Calculate and set the lateral maneuver distance corresponding to the change request
                // to induce a corresponding sublane change.
                let dir: i32 = if (newstate & LCA_RIGHT) != 0 {
                    -1
                } else if (newstate & LCA_LEFT) != 0 {
                    1
                } else {
                    0
                };
                // minimum distance to move the vehicle fully onto the lane at offset dir
                let latLaneDist = self.myVehicle.lateral_distance_to_lane(dir);
                if (newstate & LCA_TRACI) != 0 {
                    if (newstate & LCA_STAY) != 0 {
                        self.set_maneuver_dist(0.);
                    } else if ((newstate & LCA_RIGHT) != 0 && dir < 0) || ((newstate & LCA_LEFT) != 0 && dir > 0) {
                        self.set_maneuver_dist(latLaneDist);
                    }
                }
                if self.myVehicle.has_influencer() {
                    // lane change requests override sublane change requests
                    self.myVehicle.m().get_influencer().reset_lat_dist();
                }
            }
            self.set_own_state(newstate);
        } else {
            // Check for sublane change requests
            if self.myVehicle.has_influencer() && self.myVehicle.m().get_influencer().get_lat_dist() != 0. {
                let maneuverDist = self.myVehicle.m().get_influencer().get_lat_dist();
                // C++ myVehicle.getLaneChangeModel() is this same object
                self.set_maneuver_dist(maneuverDist);
                self.myVehicle.m().get_influencer().reset_lat_dist();
                newstate |= LCA_TRACI;
                if self.myOwnState != newstate {
                    self.set_own_state(newstate);
                }
            }
        }
    }

    /// C++: `double MSAbstractLaneChangeModel::getForwardPos() const` [const]
    /// get vehicle position relative to the forward direction lane
    // double
    // MSAbstractLaneChangeModel::getForwardPos() const {
    //     return myAmOpposite ? myVehicle.getLane()->getLength() - myVehicle.getPositionOnLane() : myVehicle.getPositionOnLane();
    // }
    #[allow(unused_variables)]
    pub fn get_forward_pos(&self) -> f64 {
        if self.myAmOpposite {
            self.myVehicle.get_lane().get_length() - self.myVehicle.get_position_on_lane()
        } else {
            self.myVehicle.get_position_on_lane()
        }
    }

    /// C++: `bool MSAbstractLaneChangeModel::hasBlueLight() const` [const]
    // bool hasBlueLight() const {
    //         return myHaveBlueLight;
    //     }
    #[allow(unused_variables)]
    pub fn has_blue_light(&self) -> bool {
        self.myHaveBlueLight
    }

    /// C++: `LatAlignmentDefinition MSAbstractLaneChangeModel::getDesiredAlignment() const` [const, virtual]
    // virtual LatAlignmentDefinition getDesiredAlignment() const {
    //         return myVehicle.getVehicleType().getPreferredLateralAlignment();
    //     }
    #[allow(unused_variables)]
    pub fn get_desired_alignment(&self) -> LatAlignmentDefinition {
        (self.vt.get_desired_alignment)(self)
    }

    pub fn get_desired_alignment_base(&self) -> LatAlignmentDefinition {
        *self.myVehicle.get_vehicle_type().get_preferred_lateral_alignment()
    }

    /// C++: `double MSAbstractLaneChangeModel::getCooperativeHelpSpeed(const MSLane *, double) const` [const]
    /// return speed for helping a vehicle that is blocked from changing
    // double
    // MSAbstractLaneChangeModel::getCooperativeHelpSpeed(const MSLane* lane, double distToLaneEnd) const {
    //     if (myCooperativeHelpTime >= 0) {
    //         std::pair<double, SUMOTime> backAndWaiting = lane->getEdge().getLastBlocked(lane->getIndex());
    //         if (backAndWaiting.second >= myCooperativeHelpTime) {
    //             double gap = distToLaneEnd - lane->getLength() + backAndWaiting.first - myVehicle.getVehicleType().getMinGap() - NUMERICAL_EPS;
    //             if (backAndWaiting.first < 0) {
    //                 if (myVehicle.getLane()->getToJunction() == lane->getFromJunction()) {
    //                     if (myVehicle.getLane()->isInternal()) {
    //                         // already on the junction, potentially blocking lane change, do not stop
    //                         gap = -1;
    //                     } else {
    //                         // stop before entering the junction
    //                         gap = myVehicle.getLane()->getLength() - myVehicle.getPositionOnLane();
    //                     }
    //                 }
    //             }
    //             if (gap > 0) {
    //                 double stopSpeed = myVehicle.getCarFollowModel().stopSpeed(&myVehicle, myVehicle.getSpeed(), gap);
    //                 //if (myVehicle.isSelected() && stopSpeed < myVehicle.getSpeed()) {
    //                 //    std::cout << SIMTIME << " veh=" << myVehicle.getID() << " lane=" << lane->getID() << " dte=" << distToLaneEnd << " gap=" << gap << " backPos=" << backAndWaiting.first << " waiting=" << backAndWaiting.second << " helpTime=" << myCooperativeHelpTime << " stopSpeed=" << stopSpeed << " minNext=" << myVehicle.getCarFollowModel().minNextSpeed(myVehicle.getSpeed(), &myVehicle) << "\n";
    //                 //}
    //                 if (stopSpeed >= myVehicle.getCarFollowModel().minNextSpeed(myVehicle.getSpeed(), &myVehicle)) {
    //                     // regular braking is helpful
    //                     return stopSpeed;
    //                 }
    //             }
    //         }
    //     }
    //     // do not restrict speed
    //     return std::numeric_limits<double>::max();
    // }
    #[allow(unused_variables)]
    pub fn get_cooperative_help_speed(&self, lane: Ptr<MSLane>, distToLaneEnd: f64) -> f64 {
        if self.myCooperativeHelpTime >= 0 {
            let backAndWaiting = lane.get_edge().get_last_blocked(lane.get_index());
            if backAndWaiting.1 >= self.myCooperativeHelpTime {
                let mut gap = distToLaneEnd - lane.get_length() + backAndWaiting.0 - self.myVehicle.get_vehicle_type().get_min_gap() - NUMERICAL_EPS;
                if backAndWaiting.0 < 0. {
                    if self.myVehicle.get_lane().get_to_junction() == lane.get_from_junction() {
                        if self.myVehicle.get_lane().is_internal() {
                            // already on the junction, potentially blocking lane change, do not stop
                            gap = -1.;
                        } else {
                            // stop before entering the junction
                            gap = self.myVehicle.get_lane().get_length() - self.myVehicle.get_position_on_lane();
                        }
                    }
                }
                if gap > 0. {
                    let stopSpeed = self.get_car_follow_model().stop_speed(self.myVehicle, self.myVehicle.get_speed(), gap, MSCFModel_CalcReason::CURRENT);
                    if stopSpeed >= self.get_car_follow_model().min_next_speed(self.myVehicle.get_speed(), self.myVehicle) {
                        // regular braking is helpful
                        return stopSpeed;
                    }
                }
            }
        }
        // do not restrict speed
        f64::MAX
    }

    /// C++: `SUMOTime MSAbstractLaneChangeModel::getCooperativeHelpTime() const` [const]
    // SUMOTime getCooperativeHelpTime() const {
    //         return myCooperativeHelpTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_cooperative_help_time(&self) -> SUMOTime {
        self.myCooperativeHelpTime
    }

    /// C++: `double MSAbstractLaneChangeModel::getCooperativeHelpThreshold() const` [const]
    // double getCooperativeHelpThreshold() const {
    //         return myCooperativeHelpThreshold;
    //     }
    #[allow(unused_variables)]
    pub fn get_cooperative_help_threshold(&self) -> f64 {
        self.myCooperativeHelpThreshold
    }

    /// C++: `double MSAbstractLaneChangeModel::getCooperativeMinSpeed() const` [const]
    // double getCooperativeMinSpeed() const {
    //         return myCooperativeMinSpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_cooperative_min_speed(&self) -> f64 {
        self.myCooperativeMinSpeed
    }

    /// C++: `bool MSAbstractLaneChangeModel::avoidOvertakeRight(const MSVehicle *const, const bool) const` [const, virtual]
    // bool
    // MSAbstractLaneChangeModel::avoidOvertakeRight(const MSVehicle* const neighLeader, const bool allowProb) const {
    //     return (!myAllowOvertakingRight  // the highway case
    //             && !myVehicle.congested()
    //             && myVehicle.getVehicleType().getVehicleClass() != SVC_EMERGENCY
    //             && (!allowProb || myOvertakeRightParam == 0 || myOvertakeRightParam < RandHelper::rand(myVehicle.getRNG()))) ||
    //            (neighLeader != nullptr && neighLeader->isStopped()  // the bus stop case
    //             && neighLeader->getStops().front().busstop != nullptr
    //             && !StringUtils::toBool(neighLeader->getStops().front().busstop->getParameter("allowOvertakeRight", "true")));
    // }
    #[allow(unused_variables)]
    pub fn avoid_overtake_right(&self, neighLeader: Ptr<MSVehicle>, allowProb: bool) -> bool {
        (self.vt.avoid_overtake_right)(self, neighLeader, allowProb)
    }

    pub fn avoid_overtake_right_base(&self, neighLeader: Ptr<MSVehicle>, allowProb: bool) -> bool {
        (!MY_ALLOW_OVERTAKING_RIGHT.load(Ordering::Relaxed)  // the highway case
            && !self.myVehicle.congested()
            && self.myVehicle.get_vehicle_type().get_vehicle_class() != SVC_EMERGENCY
            && (!allowProb || self.myOvertakeRightParam == 0. || self.myOvertakeRightParam < {
                let rng = self.myVehicle.get_rng();
                if rng.is_null() { utils::common::rand_helper::rand() } else { rng.m().rand() }
            }))
            || (!neighLeader.is_null()  // the bus stop case
                && neighLeader.is_stopped()
                && !neighLeader.get_stops().front().unwrap().busstop.is_null()
                && !utils::common::string_utils::to_bool(&neighLeader.get_stops().front().unwrap().busstop.base2.get_parameter("allowOvertakeRight", "true")).unwrap())
    }

    /// C++: `bool MSAbstractLaneChangeModel::congested(const MSVehicle *const)` [virtual, protected]
    // bool
    // MSAbstractLaneChangeModel::congested(const MSVehicle* const neighLeader) {
    //     if (neighLeader == nullptr) {
    //         return false;
    //     }
    //     // Congested situation are relevant only on highways (maxSpeed > 70km/h)
    //     // and congested on German Highways means that the vehicles have speeds
    //     // below 60km/h. Overtaking on the right is allowed then.
    //     if ((myVehicle.getLane()->getSpeedLimit() <= 70.0 / 3.6) || (neighLeader->getLane()->getSpeedLimit() <= 70.0 / 3.6)) {
    //
    //         return false;
    //     }
    //     if (myVehicle.congested() && neighLeader->congested()) {
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn congested(&mut self, neighLeader: Ptr<MSVehicle>) -> bool {
        if neighLeader.is_null() {
            return false;
        }
        // Congested situation are relevant only on highways (maxSpeed > 70km/h)
        // and congested on German Highways means that the vehicles have speeds
        // below 60km/h. Overtaking on the right is allowed then.
        if (self.myVehicle.get_lane().get_speed_limit() <= 70.0 / 3.6) || (neighLeader.get_lane().get_speed_limit() <= 70.0 / 3.6) {
            return false;
        }
        if self.myVehicle.congested() && neighLeader.congested() {
            return true;
        }
        false
    }

    /// C++: `bool MSAbstractLaneChangeModel::predInteraction(const std::pair<MSVehicle *, double> &)` [virtual, protected]
    // bool
    // MSAbstractLaneChangeModel::predInteraction(const std::pair<MSVehicle*, double>& leader) {
    //     if (leader.first == 0) {
    //         return false;
    //     }
    //     // let's check it on highways only
    //     if (leader.first->getSpeed() < (80.0 / 3.6)) {
    //         return false;
    //     }
    //     return leader.second < getCarFollowModel().interactionGap(&myVehicle, leader.first->getSpeed());
    // }
    #[allow(unused_variables)]
    pub fn pred_interaction(&mut self, leader: &(Ptr<MSVehicle>, f64)) -> bool {
        if leader.0.is_null() {
            return false;
        }
        // let's check it on highways only
        if leader.0.get_speed() < (80.0 / 3.6) {
            return false;
        }
        leader.1 < self.get_car_follow_model().interaction_gap(self.myVehicle, leader.0.get_speed())
    }

    /// C++: `bool MSAbstractLaneChangeModel::cancelRequest(int, int)` [protected]
    /// whether the influencer cancels the given request
    // bool
    // MSAbstractLaneChangeModel::cancelRequest(int state, int laneOffset) {
    //     // store request before canceling
    //     getCanceledState(laneOffset) |= state;
    //     int ret = myVehicle.influenceChangeDecision(state);
    //     return ret != state;
    // }
    #[allow(unused_variables)]
    pub fn cancel_request(&mut self, state: i32, laneOffset: i32) -> bool {
        // store request before canceling
        *self.get_canceled_state(laneOffset) |= state;
        let ret = self.myVehicle.m().influence_change_decision(state);
        ret != state
    }

    /// C++: `double MSAbstractLaneChangeModel::getMaxSpeedLat2() const` [const, protected]
    /// return the max of maxSpeedLat and lcMaxSpeedLatStanding
    // double
    // MSAbstractLaneChangeModel::getMaxSpeedLat2() const {
    //     return MAX2(myVehicle.getVehicleType().getMaxSpeedLat(), myMaxSpeedLatStanding);
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed_lat2(&self) -> f64 {
        let a = self.myVehicle.get_vehicle_type().get_max_speed_lat();
        let b = self.myMaxSpeedLatStanding;
        if a > b { a } else { b }
    }

    /// C++: `void MSAbstractLaneChangeModel::addLCSpeedAdvice(const double, int)` [protected]
    /// Takes a vSafe (speed advice for speed in the next simulation step), converts it into an acceleration and stores it into myLCAccelerationAdvices.
    // void
    // MSAbstractLaneChangeModel::addLCSpeedAdvice(const double vSafe, int flag) {
    //     const double accel = SPEED2ACCEL(vSafe - myVehicle.getSpeed());
    //     myLCAccelerationAdvices.push_back({accel, flag});
    // }
    #[allow(unused_variables)]
    pub fn add_lc_speed_advice(&mut self, vSafe: f64, flag: i32) {
        let accel = speed2accel(vSafe - self.myVehicle.get_speed());
        self.myLCAccelerationAdvices.push((accel, flag));
    }

    /// C++: `bool MSAbstractLaneChangeModel::canOvertakeRight(const MSVehicle *const, const double, const double, const double, double &, double &) const` [const, protected]
    // bool
    // MSAbstractLaneChangeModel::canOvertakeRight(const MSVehicle* const nv, const double dist, const double maxSpeedDiff, const double helpOvertakeSpeed, double& vSafe, double& deltaV) const {
    //     deltaV = MAX2(maxSpeedDiff, myVehicle.getSpeed() - nv->getSpeed());
    //     if (deltaV > 0) {
    //         const double vMaxDecel = getCarFollowModel().getSpeedAfterMaxDecel(myVehicle.getSpeed());
    //         const double vSafeFollow = getCarFollowModel().followSpeed(
    //                                         &myVehicle, myVehicle.getSpeed(), dist, nv->getSpeed(), nv->getCarFollowModel().getMaxDecel());
    //         const double vStayBehind = nv->getSpeed() - helpOvertakeSpeed;
    //         if (vSafeFollow >= vMaxDecel) {
    //             vSafe = vSafeFollow;
    //         } else {
    //             vSafe = MAX2(vMaxDecel, vStayBehind);
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn can_overtake_right(&self, nv: Ptr<MSVehicle>, dist: f64, maxSpeedDiff: f64, helpOvertakeSpeed: f64, vSafe: &mut f64, deltaV: &mut f64) -> bool {
        {
            let a = maxSpeedDiff;
            let b = self.myVehicle.get_speed() - nv.get_speed();
            *deltaV = if a > b { a } else { b };
        }
        if *deltaV > 0. {
            let vMaxDecel = self.get_car_follow_model().get_speed_after_max_decel(self.myVehicle.get_speed());
            let vSafeFollow = self.get_car_follow_model().follow_speed(
                self.myVehicle,
                self.myVehicle.get_speed(),
                dist,
                nv.get_speed(),
                nv.get_car_follow_model().get_max_decel(),
                Ptr::null(),
                MSCFModel_CalcReason::CURRENT,
            );
            let vStayBehind = nv.get_speed() - helpOvertakeSpeed;
            if vSafeFollow >= vMaxDecel {
                *vSafe = vSafeFollow;
            } else {
                let a = vMaxDecel;
                let b = vStayBehind;
                *vSafe = if a > b { a } else { b };
            }
            return true;
        }
        false
    }

    /// C++: `const MSCFModel & MSAbstractLaneChangeModel::getCarFollowModel() const` [const, protected]
    /// The vehicle's car following model
    // inline const MSCFModel& getCarFollowModel() const {
    //         return myVehicle.getCarFollowModel();
    //     }
    #[allow(unused_variables)]
    pub fn get_car_follow_model(&self) -> &MSCFModel {
        self.myVehicle.get_car_follow_model()
    }

    /// C++: `void MSAbstractLaneChangeModel::initLastLaneChangeOffset(int)` [protected]
    // void
    // MSAbstractLaneChangeModel::initLastLaneChangeOffset(int dir) {
    //     if (dir > 0) {
    //         myLastLaneChangeOffset = 1;
    //     } else if (dir < 0) {
    //         myLastLaneChangeOffset = -1;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn init_last_lane_change_offset(&mut self, dir: i32) {
        if dir > 0 {
            self.myLastLaneChangeOffset = 1;
        } else if dir < 0 {
            self.myLastLaneChangeOffset = -1;
        }
    }

    /// C++: `MSAbstractLaneChangeModel & MSAbstractLaneChangeModel::operator=(const MSAbstractLaneChangeModel &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSAbstractLaneChangeModel) -> &mut MSAbstractLaneChangeModel {
        // C++ declares this operator private and never defines it, so nothing can reach it
        panic!("MSAbstractLaneChangeModel::operator= is declared but not defined in C++");
    }

}

/// A class responsible for exchanging messages between cars involved in lane-change interaction
/// C++ class `MSAbstractLaneChangeModel::MSLCMessager` (microsim/lcmodels/MSAbstractLaneChangeModel.h)
pub struct MSAbstractLaneChangeModel_MSLCMessager {
    /// The leader on the informed vehicle's lane
    pub myLeader: Ptr<MSVehicle>,
    /// The leader on the lane the vehicle want to change to
    pub myNeighLeader: Ptr<MSVehicle>,
    /// The follower on the lane the vehicle want to change to
    pub myNeighFollower: Ptr<MSVehicle>,
}

impl MSAbstractLaneChangeModel_MSLCMessager {
    /// C++: `void MSLCMessager::MSLCMessager(MSVehicle *, MSVehicle *, MSVehicle *)`
    /// Constructor
    // MSLCMessager(MSVehicle* leader,  MSVehicle* neighLead, MSVehicle* neighFollow)
    //             : myLeader(leader), myNeighLeader(neighLead),
    //               myNeighFollower(neighFollow) { }
    #[allow(unused_variables)]
    pub fn new(leader: Ptr<MSVehicle>, neighLead: Ptr<MSVehicle>, neighFollow: Ptr<MSVehicle>) -> Self {
        Self { myLeader: leader, myNeighLeader: neighLead, myNeighFollower: neighFollow }
    }

    /// C++: `void MSLCMessager::~MSLCMessager() noexcept`
    /// Destructor
    // ~MSLCMessager() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `void * MSLCMessager::informLeader(void *, MSVehicle *)`
    /// Informs the leader on the same lane
    // void* informLeader(void* info, MSVehicle* sender) {
    //             assert(myLeader != 0);
    //             return myLeader->getLaneChangeModel().inform(info, sender);
    //         }
    #[allow(unused_variables)]
    pub fn inform_leader(&mut self, info: Ptr<()>, sender: Ptr<MSVehicle>) -> Ptr<()> {
        debug_assert!(!self.myLeader.is_null());
        self.myLeader.m().get_lane_change_model().inform(info, sender)
    }

    /// C++: `void * MSLCMessager::informNeighLeader(void *, MSVehicle *)`
    /// Informs the leader on the desired lane
    // void* informNeighLeader(void* info, MSVehicle* sender) {
    //             assert(myNeighLeader != 0);
    //             return myNeighLeader->getLaneChangeModel().inform(info, sender);
    //         }
    #[allow(unused_variables)]
    pub fn inform_neigh_leader(&mut self, info: Ptr<()>, sender: Ptr<MSVehicle>) -> Ptr<()> {
        debug_assert!(!self.myNeighLeader.is_null());
        self.myNeighLeader.m().get_lane_change_model().inform(info, sender)
    }

    /// C++: `void * MSLCMessager::informNeighFollower(void *, MSVehicle *)`
    /// Informs the follower on the desired lane
    // void* informNeighFollower(void* info, MSVehicle* sender) {
    //             assert(myNeighFollower != 0);
    //             return myNeighFollower->getLaneChangeModel().inform(info, sender);
    //         }
    #[allow(unused_variables)]
    pub fn inform_neigh_follower(&mut self, info: Ptr<()>, sender: Ptr<MSVehicle>) -> Ptr<()> {
        debug_assert!(!self.myNeighFollower.is_null());
        self.myNeighFollower.m().get_lane_change_model().inform(info, sender)
    }

}

/// C++ class `MSAbstractLaneChangeModel::StateAndDist` (microsim/lcmodels/MSAbstractLaneChangeModel.h)
pub struct MSAbstractLaneChangeModel_StateAndDist {
    pub state: i32,
    pub latDist: f64,
    pub maneuverDist: f64,
    pub dir: i32,
}

impl MSAbstractLaneChangeModel_StateAndDist {
    /// C++: `void StateAndDist::StateAndDist(int, double, double, int)`
    // StateAndDist(int _state, double _latDist, double _targetDist, int _dir) :
    //             state(_state),
    //             latDist(_latDist),
    //             maneuverDist(_targetDist),
    //             dir(_dir) {}
    #[allow(unused_variables)]
    pub fn new(_state: i32, _latDist: f64, _targetDist: f64, _dir: i32) -> Self {
        Self { state: _state, latDist: _latDist, maneuverDist: _targetDist, dir: _dir }
    }

    /// C++: `bool StateAndDist::sameDirection(const StateAndDist &) const` [const]
    // bool sameDirection(const StateAndDist& other) const {
    //             return latDist * other.latDist > 0;
    //         }
    #[allow(unused_variables)]
    pub fn same_direction(&self, other: &MSAbstractLaneChangeModel_StateAndDist) -> bool {
        self.latDist * other.latDist > 0.
    }

}

