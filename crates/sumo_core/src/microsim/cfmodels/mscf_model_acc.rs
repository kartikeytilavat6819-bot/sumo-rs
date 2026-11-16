//! Port of microsim/cfmodels/MSCFModel_ACC.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mscf_model::*;
use crate::microsim::ms_globals::MSGlobals;
use utils::common::time::{delta_t, time2steps, steps2time, accel2speed, speed2accel, speed2dist, dist2speed, ts};
use utils::common::to_string::f64_to_string_prec;
use utils::geom::NUMERICAL_EPS;

fn acc_follow_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 { cf_cast::<MSCFModel_ACC>(m).follow_speed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred, usage) }
fn acc_insertion_follow_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>) -> f64 { cf_cast::<MSCFModel_ACC>(m).insertion_follow_speed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred) }
fn acc_stop_speed_2(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 { cf_cast::<MSCFModel_ACC>(m).stop_speed(veh, speed, gap, decel, usage) }
fn acc_interaction_gap(m: &MSCFModel, veh: Ptr<MSVehicle>, vL: f64) -> f64 { cf_cast::<MSCFModel_ACC>(m).interaction_gap(veh, vL) }
fn acc_get_secure_gap(m: &MSCFModel, veh: Ptr<MSVehicle>, pred: Ptr<MSVehicle>, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64) -> f64 { cf_cast::<MSCFModel_ACC>(m).get_secure_gap(veh, pred, speed, leaderSpeed, leaderMaxDecel) }
fn acc_maximum_lane_speed_cf(m: &MSCFModel, veh: Ptr<MSVehicle>, maxSpeed: f64, maxSpeedLane: f64) -> f64 { cf_cast::<MSCFModel_ACC>(m).maximum_lane_speed_cf(veh, maxSpeed, maxSpeedLane) }
fn acc_get_model_id(m: &MSCFModel) -> i32 { cf_cast::<MSCFModel_ACC>(m).get_model_id() }
fn acc_duplicate(m: &MSCFModel, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> { cf_cast::<MSCFModel_ACC>(m).duplicate(vtype) }
fn acc_create_vehicle_variables(m: &MSCFModel) -> Ptr<MSCFModel_VehicleVariables> { cf_cast::<MSCFModel_ACC>(m).create_vehicle_variables() }

/// Virtual table of MSCFModel_ACC: its overrides on top of the MSCFModel default table
pub const ACC_VT: MSCFModelVt = MSCFModelVt {
    follow_speed: acc_follow_speed,
    insertion_follow_speed: acc_insertion_follow_speed,
    stop_speed_2: acc_stop_speed_2,
    interaction_gap: acc_interaction_gap,
    get_secure_gap: acc_get_secure_gap,
    maximum_lane_speed_cf: acc_maximum_lane_speed_cf,
    get_model_id: acc_get_model_id,
    duplicate: acc_duplicate,
    create_vehicle_variables: acc_create_vehicle_variables,
    ..DEFAULT_MSCF_MODEL_VT
};

// C++ #defines in MSCFModel_ACC.cpp
const DEFAULT_SC_GAIN: f64 = -0.4;
const DEFAULT_GCC_GAIN_SPEED: f64 = 0.8;
const DEFAULT_GCC_GAIN_SPACE: f64 = 0.04;
const DEFAULT_GC_GAIN_SPEED: f64 = 0.07;
const DEFAULT_GC_GAIN_SPACE: f64 = 0.23;
const DEFAULT_CA_GAIN_SPACE: f64 = 0.8;
const DEFAULT_CA_GAIN_SPEED: f64 = 0.23;
const GAP_THRESHOLD_SPEEDCTRL: f64 = 120.;
const GAP_THRESHOLD_GAPCTRL: f64 = 100.;
const DEFAULT_EMERGENCY_OVERRIDE_THRESHOLD: f64 = 2.0;

/// C++ class `MSCFModel_ACC::ACCVehicleVariables` (microsim/cfmodels/MSCFModel_ACC.h)
#[repr(C)]
pub struct MSCFModel_ACC_ACCVehicleVariables {
    /// C++ base class `MSCFModel::VehicleVariables`
    pub base: MSCFModel_VehicleVariables,
    /// The vehicle's ACC control mode. 0 for speed control and 1 for gap control
    pub ACC_ControlMode: i32,
    pub lastUpdateTime: SUMOTime,
}

impl std::ops::Deref for MSCFModel_ACC_ACCVehicleVariables {
    type Target = MSCFModel_VehicleVariables;
    fn deref(&self) -> &MSCFModel_VehicleVariables {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_ACC_ACCVehicleVariables {
    fn deref_mut(&mut self) -> &mut MSCFModel_VehicleVariables {
        &mut self.base
    }
}

impl MSCFModel_ACC_ACCVehicleVariables {
    /// C++: `void ACCVehicleVariables::ACCVehicleVariables()`
    // ACCVehicleVariables() : ACC_ControlMode(0) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSCFModel_ACC_ACCVehicleVariables {
            base: MSCFModel_VehicleVariables {},
            ACC_ControlMode: 0,
            lastUpdateTime: 0, // C++ leaves lastUpdateTime uninitialized; 0 keeps the port deterministic
        }
    }

    /// C++: `void ACCVehicleVariables::saveState(OutputDevice &, const MSCFModel &) const` [const, virtual]
    /// Saves the vehicle variables
    // void
    // MSCFModel_ACC::ACCVehicleVariables::saveState(OutputDevice& out, const MSCFModel& /*cfm*/) const {
    //     out.openTag(SUMO_TAG_CFM_VARIABLES);
    //     out.writeAttr(SUMO_ATTR_ID, "ACC");
    //     std::ostringstream internals;
    //     internals << ACC_ControlMode << " ";
    //     internals << lastUpdateTime;
    //     out.writeAttr(SUMO_ATTR_STATE, internals.str());
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice, cfm: &MSCFModel) {
        out.open_tag_id(SUMO_TAG_CFM_VARIABLES);
        out.write_attr(SUMO_ATTR_ID, "ACC");
        let internals = format!("{} {}", self.ACC_ControlMode, self.lastUpdateTime);
        out.write_attr(SUMO_ATTR_STATE, internals.as_str());
        out.close_tag("");
        let _ = cfm;
    }

    /// C++: `void ACCVehicleVariables::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the vehicle variables from the given description
    // void
    // MSCFModel_ACC::ACCVehicleVariables::loadState(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string cfmID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (cfmID != "ACC") {
    //         throw ProcessError(TLF("incompatible carFollowModel '%' when loading state for ACC", cfmID));
    //     }
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> ACC_ControlMode;
    //     bis >> lastUpdateTime;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) -> Result<(), utils::common::util_exceptions::ProcessError> {
        let mut ok = true;
        let cfmID: String = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
        if cfmID != "ACC" {
            return Err(utils::common::util_exceptions::ProcessError::new(format!("incompatible carFollowModel '{}' when loading state for ACC", cfmID)));
        }
        let state = attrs.get_string(SUMO_ATTR_STATE);
        let mut bis = state.split_whitespace();
        if let Some(v) = bis.next().and_then(|s| s.parse::<i32>().ok()) {
            self.ACC_ControlMode = v;
        }
        if let Some(v) = bis.next().and_then(|s| s.parse::<SUMOTime>().ok()) {
            self.lastUpdateTime = v;
        }
        Ok(())
    }

}

/// The ACC car-following model
/// C++ class `MSCFModel_ACC` (microsim/cfmodels/MSCFModel_ACC.h)
#[repr(C)]
pub struct MSCFModel_ACC {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    pub mySpeedControlGain: f64,
    pub myGapClosingControlGainSpeed: f64,
    pub myGapClosingControlGainSpace: f64,
    pub myGapControlGainSpeed: f64,
    pub myGapControlGainSpace: f64,
    pub myCollisionAvoidanceGainSpeed: f64,
    pub myCollisionAvoidanceGainSpace: f64,
    pub myApplyDriverstate: f64,
    pub myEmergencyThreshold: f64,
}

impl std::ops::Deref for MSCFModel_ACC {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_ACC {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

impl MSCFModel_ACC {
    /// C++: `void MSCFModel_ACC::MSCFModel_ACC(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_ACC::MSCFModel_ACC(const MSVehicleType* vtype) :
    //     MSCFModel(vtype),
    //     mySpeedControlGain(vtype->getParameter().getCFParam(SUMO_ATTR_SC_GAIN, DEFAULT_SC_GAIN)),
    //     myGapClosingControlGainSpeed(vtype->getParameter().getCFParam(SUMO_ATTR_GCC_GAIN_SPEED, DEFAULT_GCC_GAIN_SPEED)),
    //     myGapClosingControlGainSpace(vtype->getParameter().getCFParam(SUMO_ATTR_GCC_GAIN_SPACE, DEFAULT_GCC_GAIN_SPACE)),
    //     myGapControlGainSpeed(vtype->getParameter().getCFParam(SUMO_ATTR_GC_GAIN_SPEED, DEFAULT_GC_GAIN_SPEED)),
    //     myGapControlGainSpace(vtype->getParameter().getCFParam(SUMO_ATTR_GC_GAIN_SPACE, DEFAULT_GC_GAIN_SPACE)),
    //     myCollisionAvoidanceGainSpeed(vtype->getParameter().getCFParam(SUMO_ATTR_CA_GAIN_SPEED, DEFAULT_CA_GAIN_SPEED)),
    //     myCollisionAvoidanceGainSpace(vtype->getParameter().getCFParam(SUMO_ATTR_CA_GAIN_SPACE, DEFAULT_CA_GAIN_SPACE)),
    //     myApplyDriverstate(vtype->getParameter().getCFParam(SUMO_ATTR_APPLYDRIVERSTATE, 0)),
    //     myEmergencyThreshold(vtype->getParameter().getCFParam(SUMO_ATTR_CA_OVERRIDE, DEFAULT_EMERGENCY_OVERRIDE_THRESHOLD)) {
    //     // ACC does not drive very precise and often violates minGap
    //     myCollisionMinGapFactor = vtype->getParameter().getCFParam(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        let p = vtype.get_parameter();
        let mut base = MSCFModel::new(vtype);
        base.vt = &ACC_VT;
        let mySpeedControlGain = p.get_cf_param(SUMO_ATTR_SC_GAIN, DEFAULT_SC_GAIN);
        let myGapClosingControlGainSpeed = p.get_cf_param(SUMO_ATTR_GCC_GAIN_SPEED, DEFAULT_GCC_GAIN_SPEED);
        let myGapClosingControlGainSpace = p.get_cf_param(SUMO_ATTR_GCC_GAIN_SPACE, DEFAULT_GCC_GAIN_SPACE);
        let myGapControlGainSpeed = p.get_cf_param(SUMO_ATTR_GC_GAIN_SPEED, DEFAULT_GC_GAIN_SPEED);
        let myGapControlGainSpace = p.get_cf_param(SUMO_ATTR_GC_GAIN_SPACE, DEFAULT_GC_GAIN_SPACE);
        let myCollisionAvoidanceGainSpeed = p.get_cf_param(SUMO_ATTR_CA_GAIN_SPEED, DEFAULT_CA_GAIN_SPEED);
        let myCollisionAvoidanceGainSpace = p.get_cf_param(SUMO_ATTR_CA_GAIN_SPACE, DEFAULT_CA_GAIN_SPACE);
        let myApplyDriverstate = p.get_cf_param(SUMO_ATTR_APPLYDRIVERSTATE, 0.);
        let myEmergencyThreshold = p.get_cf_param(SUMO_ATTR_CA_OVERRIDE, DEFAULT_EMERGENCY_OVERRIDE_THRESHOLD);
        // ACC does not drive very precise and often violates minGap
        base.myCollisionMinGapFactor = p.get_cf_param(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
        MSCFModel_ACC {
            base,
            mySpeedControlGain,
            myGapClosingControlGainSpeed,
            myGapClosingControlGainSpace,
            myGapControlGainSpeed,
            myGapControlGainSpace,
            myCollisionAvoidanceGainSpeed,
            myCollisionAvoidanceGainSpace,
            myApplyDriverstate,
            myEmergencyThreshold,
        }
    }

    /// C++: `void MSCFModel_ACC::~MSCFModel_ACC() noexcept` [virtual]
    /// Destructor
    // MSCFModel_ACC::~MSCFModel_ACC() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is trivial: no owned resources to release
    }

    /// C++: `double MSCFModel_ACC::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_ACC::followSpeed(const MSVehicle* const veh, double speed, double gap2pred, double predSpeed, double predMaxDecel, const MSVehicle* const pred, const CalcReason /*usage*/) const {
    //     if (myApplyDriverstate) {
    //         applyHeadwayAndSpeedDifferencePerceptionErrors(veh, speed, gap2pred, predSpeed, predMaxDecel, pred);
    //     }
    //     const double desSpeed = MIN2(veh->getLane()->getSpeedLimit(), veh->getMaxSpeed());
    //     const double vACC = _v(veh, gap2pred, speed, predSpeed, desSpeed, true);
    //     const double vSafe = maximumSafeFollowSpeed(gap2pred, speed, predSpeed, predMaxDecel);
    //     if (vSafe + myEmergencyThreshold < vACC) {
    //         //ACCVehicleVariables* vars = (ACCVehicleVariables*)veh->getCarFollowVariables();
    //         //std::cout << SIMTIME << " veh=" << veh->getID() << " v=" << speed << " vL=" << predSpeed << " gap=" << gap2pred << " vACC=" << vACC << " vSafe=" << vSafe << " cm=" << vars->ACC_ControlMode << "\n";
    //         return vSafe + myEmergencyThreshold;
    //     }
    //     return vACC;
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        let mut gap2pred = gap2pred;
        let mut predSpeed = predSpeed;
        if self.myApplyDriverstate != 0. {
            self.apply_headway_and_speed_difference_perception_errors(veh, speed, &mut gap2pred, &mut predSpeed, predMaxDecel, pred);
        }
        let desSpeed = min2(veh.get_lane().get_speed_limit(), veh.get_max_speed());
        let vACC = self.v(veh, gap2pred, speed, predSpeed, desSpeed, true);
        let vSafe = self.maximum_safe_follow_speed(gap2pred, speed, predSpeed, predMaxDecel, false);
        if vSafe + self.myEmergencyThreshold < vACC {
            return vSafe + self.myEmergencyThreshold;
        }
        vACC
    }

    /// C++: `double MSCFModel_ACC::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // double
    // MSCFModel_ACC::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double decel, const CalcReason /*usage*/) const {
    //     if (myApplyDriverstate) {
    //         applyHeadwayPerceptionError(veh, speed, gap);
    //     }
    //     // NOTE: This allows return of smaller values than minNextSpeed().
    //     // Only relevant for the ballistic update: We give the argument headway=TS, to assure that
    //     // the stopping position is approached with a uniform deceleration also for tau!=TS.
    //     return MIN2(maximumSafeStopSpeed(gap, decel, speed, false, veh->getActionStepLengthSecs()), maxNextSpeed(speed, veh));
    // }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        let mut gap = gap2pred;
        if self.myApplyDriverstate != 0. {
            self.apply_headway_perception_error(veh, speed, &mut gap);
        }
        // NOTE: This allows return of smaller values than minNextSpeed().
        // Only relevant for the ballistic update: We give the argument headway=TS, to assure that
        // the stopping position is approached with a uniform deceleration also for tau!=TS.
        min2(self.maximum_safe_stop_speed(gap, decel, speed, false, veh.get_action_step_length_secs(), true), self.max_next_speed(speed, veh))
    }

    /// C++: `double MSCFModel_ACC::getSecureGap(const MSVehicle *const, const MSVehicle *const, const double, const double, const double) const` [const, virtual]
    /// Returns the a gap such that the gap mode acceleration of the follower is zero
    // double
    // MSCFModel_ACC::getSecureGap(const MSVehicle* const /*veh*/, const MSVehicle* const /*pred*/, const double speed, const double leaderSpeed, const double /* leaderMaxDecel */) const {
    //     // Accel in gap mode should vanish:
    //     //      0 = myGapControlGainSpeed * (leaderSpeed - speed) + myGapControlGainSpace * (g - myHeadwayTime * speed);
    //     // <=>  myGapControlGainSpace * g = - myGapControlGainSpeed * (leaderSpeed - speed) + myGapControlGainSpace * myHeadwayTime * speed;
    //     // <=>  g = - myGapControlGainSpeed * (leaderSpeed - speed) / myGapControlGainSpace + myHeadwayTime * speed;
    //     return myGapControlGainSpeed * (speed - leaderSpeed) / myGapControlGainSpace + myHeadwayTime * speed;
    // }
    #[allow(unused_variables)]
    pub fn get_secure_gap(&self, veh: Ptr<MSVehicle>, pred: Ptr<MSVehicle>, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64) -> f64 {
        // Accel in gap mode should vanish:
        //      0 = myGapControlGainSpeed * (leaderSpeed - speed) + myGapControlGainSpace * (g - myHeadwayTime * speed);
        // <=>  myGapControlGainSpace * g = - myGapControlGainSpeed * (leaderSpeed - speed) + myGapControlGainSpace * myHeadwayTime * speed;
        // <=>  g = - myGapControlGainSpeed * (leaderSpeed - speed) / myGapControlGainSpace + myHeadwayTime * speed;
        self.myGapControlGainSpeed * (speed - leaderSpeed) / self.myGapControlGainSpace + self.myHeadwayTime * speed
    }

    /// C++: `double MSCFModel_ACC::insertionFollowSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const) const` [const, virtual]
    /// Computes the vehicle's acceptable speed at insertion
    // double
    // MSCFModel_ACC::insertionFollowSpeed(const MSVehicle* const v, double speed, double gap2pred, double predSpeed, double predMaxDecel, const MSVehicle* const /*pred*/) const {
    // //#ifdef DEBUG_ACC
    // //        std::cout << "MSCFModel_ACC::insertionFollowSpeed(), speed="<<speed<< std::endl;
    // //#endif
    //     // iterate to find a stationary value for
    //     //    speed = followSpeed(v, speed, gap2pred, predSpeed, predMaxDecel, nullptr, CalcReason::FUTURE)
    //     const int max_iter = 50;
    //     int n_iter = 0;
    //     const double tol = 0.1;
    //     const double damping = 0.1;
    //
    //     double res = speed;
    //     while (n_iter < max_iter) {
    //         // proposed acceleration
    //         const double a = SPEED2ACCEL(followSpeed(v, res, gap2pred, predSpeed, predMaxDecel, nullptr, CalcReason::FUTURE) - res);
    //         res = res + damping * a;
    // //#ifdef DEBUG_ACC
    // //        std::cout << "   n_iter=" << n_iter << ", a=" << a << ", res=" << res << std::endl;
    // //#endif
    //         if (fabs(a) < tol) {
    //             break;
    //         } else {
    //             n_iter++;
    //         }
    //     }
    //     return res;
    // }
    #[allow(unused_variables)]
    pub fn insertion_follow_speed(&self, v: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>) -> f64 {
        // iterate to find a stationary value for
        //    speed = followSpeed(v, speed, gap2pred, predSpeed, predMaxDecel, nullptr, CalcReason::FUTURE)
        let max_iter = 50;
        let mut n_iter = 0;
        let tol = 0.1;
        let damping = 0.1;

        let mut res = speed;
        while n_iter < max_iter {
            // proposed acceleration
            let a = speed2accel(self.follow_speed(v, res, gap2pred, predSpeed, predMaxDecel, Ptr::null(), MSCFModel_CalcReason::FUTURE) - res);
            res = res + damping * a;
            if a.abs() < tol {
                break;
            } else {
                n_iter += 1;
            }
        }
        res
    }

    /// C++: `double MSCFModel_ACC::interactionGap(const MSVehicle *const, double) const` [const, virtual]
    /// Returns the maximum gap at which an interaction between both vehicles occurs
    // double
    // MSCFModel_ACC::interactionGap(const MSVehicle* const /*veh */, double /* vL */) const {
    //     /*maximum radar range is ACC is enabled*/
    //     return 250;
    // }
    #[allow(unused_variables)]
    pub fn interaction_gap(&self, arg0: Ptr<MSVehicle>, vL: f64) -> f64 {
        /*maximum radar range is ACC is enabled*/
        250.
    }

    /// C++: `double MSCFModel_ACC::maximumLaneSpeedCF(const MSVehicle *const, double, double) const` [const, virtual]
    /// Returns the maximum velocity the CF-model wants to achieve in the next step
    // double maximumLaneSpeedCF(const MSVehicle* const veh, double maxSpeed, double maxSpeedLane) const {
    //         double result = MIN2(maxSpeed, maxSpeedLane);
    //         if (myApplyDriverstate) {
    //             applyOwnSpeedPerceptionError(veh, result);
    //         }
    //         return result;
    //     }
    #[allow(unused_variables)]
    pub fn maximum_lane_speed_cf(&self, veh: Ptr<MSVehicle>, maxSpeed: f64, maxSpeedLane: f64) -> f64 {
        let mut result = min2(maxSpeed, maxSpeedLane);
        if self.myApplyDriverstate != 0. {
            self.apply_own_speed_perception_error(veh, &mut result);
        }
        result
    }

    /// C++: `int MSCFModel_ACC::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_ACC;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        SUMO_TAG_CF_ACC as i32
    }

    /// C++: `MSCFModel * MSCFModel_ACC::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_ACC::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_ACC(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        // shortcut: duplicate() boxes the copy like C++ `new`; the owner must free it once model ownership is ported
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_ACC::new(vtype))) as *mut MSCFModel)
    }

    /// C++: `VehicleVariables * MSCFModel_ACC::createVehicleVariables() const` [const, virtual]
    // VehicleVariables* createVehicleVariables() const {
    //         ACCVehicleVariables* ret = new ACCVehicleVariables();
    //         ret->ACC_ControlMode = 0;
    //         ret->lastUpdateTime = 0;
    //         return ret;
    //     }
    #[allow(unused_variables)]
    pub fn create_vehicle_variables(&self) -> Ptr<MSCFModel_VehicleVariables> {
        // shortcut: allocated with Box::into_raw like C++ `new`; the owning vehicle must free it once ported
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_ACC_ACCVehicleVariables::new())) as *mut MSCFModel_VehicleVariables)
    }

    /// C++: `double MSCFModel_ACC::_v(const MSVehicle *const, const double, const double, const double, const double, const bool) const` [const, private]
    // double
    // MSCFModel_ACC::_v(const MSVehicle* const veh, const double gap2pred, const double speed,
    //                   const double predSpeed, const double desSpeed, const bool /* respectMinGap */) const {
    //
    //     double accelACC = 0;
    //     double gapLimit_SC = GAP_THRESHOLD_SPEEDCTRL; // lower gap limit in meters to enable speed control law
    //     double gapLimit_GC = GAP_THRESHOLD_GAPCTRL; // upper gap limit in meters to enable gap control law
    //
    // #ifdef DEBUG_ACC
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " MSCFModel_ACC::_v() for veh '" << veh->getID() << "'\n"
    //                   << "        gap=" << gap2pred << " speed="  << speed << " predSpeed=" << predSpeed
    //                   << " desSpeed=" << desSpeed << " tau=" << myHeadwayTime << std::endl;
    //     }
    // #endif
    //
    //
    //     /* Velocity error */
    //     double vErr = speed - desSpeed;
    //     int setControlMode = 0;
    //     ACCVehicleVariables* vars = (ACCVehicleVariables*) veh->getCarFollowVariables();
    //     if (vars->lastUpdateTime != MSNet::getInstance()->getCurrentTimeStep()) {
    //         vars->lastUpdateTime = MSNet::getInstance()->getCurrentTimeStep();
    //         setControlMode = 1;
    //     }
    //     if (gap2pred > gapLimit_SC) {
    //
    // #ifdef DEBUG_ACC
    //         if (DEBUG_COND) {
    //             std::cout << "        applying speedControl" << std::endl;
    //         }
    // #endif
    //         // Find acceleration - Speed control law
    //         accelACC = accelSpeedControl(vErr);
    //         // Set cl to vehicle parameters
    //         if (setControlMode) {
    //             vars->ACC_ControlMode = 0;
    //         }
    //     } else if (gap2pred < gapLimit_GC) {
    //         // Find acceleration - Gap control law
    //         accelACC = accelGapControl(veh, gap2pred, speed, predSpeed, vErr);
    //         // Set cl to vehicle parameters
    //         if (setControlMode) {
    //             vars->ACC_ControlMode = 1;
    //         }
    //     } else {
    //         // Follow previous applied law
    //         int cm = vars->ACC_ControlMode;
    //         if (!cm) {
    //
    // #ifdef DEBUG_ACC
    //             if (DEBUG_COND) {
    //                 std::cout << "        applying speedControl" << std::endl;
    //             }
    // #endif
    //             accelACC = accelSpeedControl(vErr);
    //         } else {
    //             accelACC = accelGapControl(veh, gap2pred, speed, predSpeed, vErr);
    //         }
    //
    //     }
    //
    //     double newSpeed = speed + ACCEL2SPEED(accelACC);
    //
    // #ifdef DEBUG_ACC
    //     if (DEBUG_COND) {
    //         std::cout << "        result: accel=" << accelACC << " newSpeed="  << newSpeed << std::endl;
    //     }
    // #endif
    //
    //     return MAX2(0., newSpeed);
    // }
    #[allow(unused_variables)]
    pub fn v(&self, veh: Ptr<MSVehicle>, gap2pred: f64, mySpeed: f64, predSpeed: f64, desSpeed: f64, respectMinGap: bool) -> f64 {
        let speed = mySpeed;
        let mut accelACC = 0.;
        let gapLimit_SC = GAP_THRESHOLD_SPEEDCTRL; // lower gap limit in meters to enable speed control law
        let gapLimit_GC = GAP_THRESHOLD_GAPCTRL; // upper gap limit in meters to enable gap control law

        /* Velocity error */
        let vErr = speed - desSpeed;
        let vars = veh.get_car_follow_variables().cast::<MSCFModel_ACC_ACCVehicleVariables>();
        let now = MSNet::get_instance().get_current_time_step();
        let mut setControlMode = false;
        if vars.lastUpdateTime != now {
            vars.m().lastUpdateTime = now;
            setControlMode = true;
        }
        if gap2pred > gapLimit_SC {
            // Find acceleration - Speed control law
            accelACC = self.accel_speed_control(vErr);
            // Set cl to vehicle parameters
            if setControlMode {
                vars.m().ACC_ControlMode = 0;
            }
        } else if gap2pred < gapLimit_GC {
            // Find acceleration - Gap control law
            accelACC = self.accel_gap_control(veh, gap2pred, speed, predSpeed, vErr);
            // Set cl to vehicle parameters
            if setControlMode {
                vars.m().ACC_ControlMode = 1;
            }
        } else {
            // Follow previous applied law
            let cm = vars.ACC_ControlMode;
            if cm == 0 {
                accelACC = self.accel_speed_control(vErr);
            } else {
                accelACC = self.accel_gap_control(veh, gap2pred, speed, predSpeed, vErr);
            }
        }

        let newSpeed = speed + accel2speed(accelACC);

        max2(0., newSpeed)
    }

    /// C++: `double MSCFModel_ACC::accelSpeedControl(double) const` [const, private]
    // double MSCFModel_ACC::accelSpeedControl(double vErr) const {
    //     // Speed control law
    //     return mySpeedControlGain * vErr;
    // }
    #[allow(unused_variables)]
    pub fn accel_speed_control(&self, vErr: f64) -> f64 {
        // Speed control law
        self.mySpeedControlGain * vErr
    }

    /// C++: `double MSCFModel_ACC::accelGapControl(const MSVehicle *const, const double, const double, const double, double) const` [const, private]
    // double
    // MSCFModel_ACC::accelGapControl(const MSVehicle* const /* veh */, const double gap2pred, const double speed, const double predSpeed, double vErr) const {
    //     // Gap control law
    //     double gclAccel = 0.0;
    //     const double deltaVel = predSpeed - speed;
    //
    //     // see dynamic gap margin definition from (Xiao et. al, 2018)[3], equation 5 reformulated as min/max to avoid discontinuities
    //     const double d0 = MAX2(0., MIN2(75. / speed - 5., 2.));
    //     // this is equation 4, gap2pred is the difference in vehicle positions minus the length
    //     const double spacingErr = gap2pred - myHeadwayTime * speed - d0;
    //
    //
    //     if (fabs(spacingErr) < 0.2 && fabs(vErr) < 0.1) {
    //         // gap mode
    //         gclAccel = myGapControlGainSpeed * deltaVel + myGapControlGainSpace * spacingErr;
    // #ifdef DEBUG_ACC
    //         if (DEBUG_COND) {
    //             std::cout << "        applying gap control:  spacingErr=" << spacingErr << " speedErr=" << vErr << std::endl;
    //         }
    // #endif
    //     } else if (spacingErr < 0)  {
    //         // collision avoidance mode
    //         gclAccel = myCollisionAvoidanceGainSpeed * deltaVel + myCollisionAvoidanceGainSpace * spacingErr;
    // #ifdef DEBUG_ACC
    //         if (DEBUG_COND) {
    //             std::cout << "        applying collision avoidance:  spacingErr=" << spacingErr << " speedErr=" << vErr << std::endl;
    //         }
    // #endif
    //     } else {
    //         // gap closing mode
    //         gclAccel = myGapClosingControlGainSpeed * deltaVel + myGapClosingControlGainSpace * spacingErr;
    // #ifdef DEBUG_ACC
    //         if (DEBUG_COND) {
    //             std::cout << "        applying gap closing:  spacingErr=" << spacingErr << " speedErr=" << vErr << std::endl;
    //         }
    // #endif
    //     }
    //     return gclAccel;
    // }
    #[allow(unused_variables)]
    pub fn accel_gap_control(&self, veh: Ptr<MSVehicle>, gap2pred: f64, speed: f64, predSpeed: f64, vErr: f64) -> f64 {
        // Gap control law
        let mut gclAccel = 0.0;
        let deltaVel = predSpeed - speed;

        // see dynamic gap margin definition from (Xiao et. al, 2018)[3], equation 5 reformulated as min/max to avoid discontinuities
        let d0 = max2(0., min2(75. / speed - 5., 2.));
        // this is equation 4, gap2pred is the difference in vehicle positions minus the length
        let spacingErr = gap2pred - self.myHeadwayTime * speed - d0;

        if spacingErr.abs() < 0.2 && vErr.abs() < 0.1 {
            // gap mode
            gclAccel = self.myGapControlGainSpeed * deltaVel + self.myGapControlGainSpace * spacingErr;
        } else if spacingErr < 0. {
            // collision avoidance mode
            gclAccel = self.myCollisionAvoidanceGainSpeed * deltaVel + self.myCollisionAvoidanceGainSpace * spacingErr;
        } else {
            // gap closing mode
            gclAccel = self.myGapClosingControlGainSpeed * deltaVel + self.myGapClosingControlGainSpace * spacingErr;
        }
        gclAccel
    }

    /// C++: `MSCFModel_ACC & MSCFModel_ACC::operator=(const MSCFModel_ACC &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSCFModel_ACC) -> &mut MSCFModel_ACC {
        // shortcut: C++ operator= is private and never defined (C++ cannot call it); panic mirrors that
        panic!("MSCFModel_ACC::operator= is private and not defined in C++")
    }

}

