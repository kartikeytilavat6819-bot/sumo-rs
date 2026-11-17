//! Port of microsim/cfmodels/MSCFModel.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::{INVALID_DOUBLE, SUMO_CONST_HALTING_SPEED};
use utils::common::time::{delta_t, time2steps, accel2speed, speed2accel, accel2dist, speed2dist, dist2speed, ts};
use utils::common::to_string::f64_to_string_prec;
use utils::geom::NUMERICAL_EPS;

/// C++ `LinearApproxHelpers::LinearApproxMap` (`std::map<double, double>`), kept as a vector sorted by key.
pub type LinearApproxMap = Vec<(f64, f64)>;

/// C++ `INVALID_SPEED` macro (`299792458 + 1`), defined in MSCFModel.h
pub const INVALID_SPEED: f64 = 299792459.0;

/// C++ `EMERGENCY_DECEL_AMPLIFIER` macro, defined in MSCFModel.h
pub const EMERGENCY_DECEL_AMPLIFIER: f64 = 1.2;

/// C++ `MIN2` / `MAX2` / `MIN3` / `MAX3` templates (utils/common/StdDefs.h), same operand order
pub(crate) fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
pub(crate) fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
pub(crate) fn min3(a: f64, b: f64, c: f64) -> f64 {
    min2(c, if a < b { a } else { b })
}
pub(crate) fn max3(a: f64, b: f64, c: f64) -> f64 {
    max2(c, if a > b { a } else { b })
}

/// C++ `MSGlobals::gSemiImplicitEulerUpdate`, `gComputeLC`, `gDefaultEmergencyDecel`
pub(crate) fn g_semi_implicit_euler_update() -> bool {
    crate::microsim::ms_globals::semi_implicit_euler_update()
}
pub(crate) fn g_compute_lc() -> bool {
    crate::microsim::ms_globals::compute_lc()
}
pub(crate) fn g_default_emergency_decel() -> f64 {
    crate::microsim::ms_globals::default_emergency_decel()
}

/// C++ implicit upcast `MSVehicle* -> SUMOTrafficObject*` (MSVehicle -> MSBaseVehicle -> SUMOVehicle -> SUMOTrafficObject)
pub(crate) fn traffic_object(veh: Ptr<MSVehicle>) -> Ptr<SUMOTrafficObject> {
    Ptr::from_mut(&mut veh.m().base.base.base)
}

/// C++ `LinearApproxHelpers::getInterpolatedValue` (std::map lower_bound semantics on a sorted vector)
pub(crate) fn get_interpolated_value(map: &LinearApproxMap, axisValue: f64) -> f64 {
    let low = map.partition_point(|(k, _)| *k < axisValue);
    if low == map.len() {
        return map[map.len() - 1].1;
    }
    if low == 0 {
        return map[0].1;
    }
    let (k1, v1) = map[low];
    let (k0, v0) = map[low - 1];
    let range = k1 - k0;
    let dist = axisValue - k0;
    let weight = dist / range;
    (1. - weight) * v0 + weight * v1
}

/// C++ enum `MSCFModel::CalcReason`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSCFModel_CalcReason {
    CURRENT = 0,
    FUTURE = 1,
    CURRENT_WAIT = 2,
    LANE_CHANGE = 3,
}

/// Virtual dispatch table for the polymorphic methods of MSCFModel.
/// Each subclass constructor installs its own table in `vt`; the base methods forward through it.
pub struct MSCFModelVt {
    pub finalize_speed: fn(&MSCFModel, Ptr<MSVehicle>, f64) -> f64,
    pub patch_speed_before_lc: fn(&MSCFModel, Ptr<MSVehicle>, f64, f64) -> f64,
    pub free_speed: fn(&MSCFModel, Ptr<MSVehicle>, f64, f64, f64, bool, MSCFModel_CalcReason) -> f64,
    pub follow_speed: fn(&MSCFModel, Ptr<MSVehicle>, f64, f64, f64, f64, Ptr<MSVehicle>, MSCFModel_CalcReason) -> f64,
    pub insertion_follow_speed: fn(&MSCFModel, Ptr<MSVehicle>, f64, f64, f64, f64, Ptr<MSVehicle>) -> f64,
    pub insertion_stop_speed: fn(&MSCFModel, Ptr<MSVehicle>, f64, f64) -> f64,
    pub stop_speed_2: fn(&MSCFModel, Ptr<MSVehicle>, f64, f64, f64, MSCFModel_CalcReason) -> f64,
    pub follow_speed_transient: fn(&MSCFModel, f64, Ptr<MSVehicle>, f64, f64, f64, f64) -> f64,
    pub interaction_gap: fn(&MSCFModel, Ptr<MSVehicle>, f64) -> f64,
    pub maximum_lane_speed_cf: fn(&MSCFModel, Ptr<MSVehicle>, f64, f64) -> f64,
    pub get_model_id: fn(&MSCFModel) -> i32,
    pub duplicate: fn(&MSCFModel, Ptr<MSVehicleType>) -> Ptr<MSCFModel>,
    pub create_vehicle_variables: fn(&MSCFModel) -> Ptr<MSCFModel_VehicleVariables>,
    pub save_vehicle_variables: fn(&MSCFModel, &MSCFModel_VehicleVariables, &mut OutputDevice),
    pub load_vehicle_variables: fn(&MSCFModel, &mut MSCFModel_VehicleVariables, &SUMOSAXAttributes),
    pub get_headway_time: fn(&MSCFModel) -> f64,
    pub set_headway_time: fn(&mut MSCFModel, f64),
    pub get_imperfection: fn(&MSCFModel) -> f64,
    pub set_imperfection: fn(&mut MSCFModel, f64),
    pub set_max_decel: fn(&mut MSCFModel, f64),
    pub startup_delay_stopped: fn(&MSCFModel) -> bool,
    pub max_next_speed: fn(&MSCFModel, f64, Ptr<MSVehicle>) -> f64,
    pub max_next_safe_min: fn(&MSCFModel, f64, Ptr<MSVehicle>) -> f64,
    pub min_next_speed: fn(&MSCFModel, f64, Ptr<MSVehicle>) -> f64,
    pub min_next_speed_emergency: fn(&MSCFModel, f64, Ptr<MSVehicle>) -> f64,
    pub brake_gap_2: fn(&MSCFModel, f64, f64, f64) -> f64,
    pub get_secure_gap: fn(&MSCFModel, Ptr<MSVehicle>, Ptr<MSVehicle>, f64, f64, f64) -> f64,
    pub get_parameter: fn(&MSCFModel, Ptr<MSVehicle>, &str) -> String,
    pub set_parameter: fn(&MSCFModel, Ptr<MSVehicle>, &str, &str) -> Result<(), InvalidArgument>,
}

fn default_finalize_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
    m.finalize_speed_base(veh, vPos)
}
fn default_patch_speed_before_lc(m: &MSCFModel, veh: Ptr<MSVehicle>, vMin: f64, vMax: f64) -> f64 {
    m.patch_speed_before_lc_base(veh, vMin, vMax)
}
fn default_free_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, seen: f64, maxSpeed: f64, onInsertion: bool, usage: MSCFModel_CalcReason) -> f64 {
    m.free_speed_base(veh, speed, seen, maxSpeed, onInsertion, usage)
}
fn default_follow_speed(_m: &MSCFModel, _veh: Ptr<MSVehicle>, _speed: f64, _gap2pred: f64, _predSpeed: f64, _predMaxDecel: f64, _pred: Ptr<MSVehicle>, _usage: MSCFModel_CalcReason) -> f64 {
    panic!("MSCFModel::followSpeed is pure virtual")
}
fn default_insertion_follow_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>) -> f64 {
    m.insertion_follow_speed_base(veh, speed, gap2pred, predSpeed, predMaxDecel, pred)
}
fn default_insertion_stop_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap: f64) -> f64 {
    m.insertion_stop_speed_base(veh, speed, gap)
}
fn default_stop_speed_2(_m: &MSCFModel, _veh: Ptr<MSVehicle>, _speed: f64, _gap: f64, _decel: f64, _usage: MSCFModel_CalcReason) -> f64 {
    panic!("MSCFModel::stopSpeed is pure virtual")
}
fn default_follow_speed_transient(m: &MSCFModel, duration: f64, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64) -> f64 {
    m.follow_speed_transient_base(duration, veh, speed, gap2pred, predSpeed, predMaxDecel)
}
fn default_interaction_gap(m: &MSCFModel, veh: Ptr<MSVehicle>, vL: f64) -> f64 {
    m.interaction_gap_base(veh, vL)
}
fn default_maximum_lane_speed_cf(m: &MSCFModel, veh: Ptr<MSVehicle>, maxSpeed: f64, maxSpeedLane: f64) -> f64 {
    m.maximum_lane_speed_cf_base(veh, maxSpeed, maxSpeedLane)
}
fn default_get_model_id(_m: &MSCFModel) -> i32 {
    panic!("MSCFModel::getModelID is pure virtual")
}
fn default_duplicate(_m: &MSCFModel, _vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
    panic!("MSCFModel::duplicate is pure virtual")
}
fn default_create_vehicle_variables(m: &MSCFModel) -> Ptr<MSCFModel_VehicleVariables> {
    m.create_vehicle_variables_base()
}
fn default_save_vehicle_variables(m: &MSCFModel, _vars: &MSCFModel_VehicleVariables, _out: &mut OutputDevice) {
    utils::common::msg_handler::write_warning(&format!("carFollowModel of vType '{}' cannot save state", m.myType.get_id()));
}
fn default_load_vehicle_variables(_m: &MSCFModel, _vars: &mut MSCFModel_VehicleVariables, _attrs: &SUMOSAXAttributes) {}
fn default_get_headway_time(m: &MSCFModel) -> f64 {
    m.get_headway_time_base()
}
fn default_set_headway_time(m: &mut MSCFModel, headwayTime: f64) {
    m.set_headway_time_base(headwayTime)
}
fn default_get_imperfection(m: &MSCFModel) -> f64 {
    m.get_imperfection_base()
}
fn default_set_imperfection(m: &mut MSCFModel, imperfection: f64) {
    m.set_imperfection_base(imperfection)
}
fn default_set_max_decel(m: &mut MSCFModel, decel: f64) {
    m.set_max_decel_base(decel)
}
fn default_startup_delay_stopped(m: &MSCFModel) -> bool {
    m.startup_delay_stopped_base()
}
fn default_max_next_speed(m: &MSCFModel, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
    m.max_next_speed_base(speed, veh)
}
fn default_max_next_safe_min(m: &MSCFModel, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
    m.max_next_safe_min_base(speed, veh)
}
fn default_min_next_speed(m: &MSCFModel, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
    m.min_next_speed_base(speed, veh)
}
fn default_min_next_speed_emergency(m: &MSCFModel, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
    m.min_next_speed_emergency_base(speed, veh)
}
fn default_brake_gap_2(m: &MSCFModel, speed: f64, decel: f64, headwayTime: f64) -> f64 {
    m.brake_gap_2_base(speed, decel, headwayTime)
}
fn default_get_secure_gap(m: &MSCFModel, veh: Ptr<MSVehicle>, pred: Ptr<MSVehicle>, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64) -> f64 {
    m.get_secure_gap_base(veh, pred, speed, leaderSpeed, leaderMaxDecel)
}
fn default_get_parameter(m: &MSCFModel, veh: Ptr<MSVehicle>, key: &str) -> String {
    m.get_parameter_base(veh, key)
}
fn default_set_parameter(m: &MSCFModel, veh: Ptr<MSVehicle>, key: &str, value: &str) -> Result<(), InvalidArgument> {
    m.set_parameter_base(veh, key, value)
}

/// C++ static_cast from `MSCFModel` to a derived class. Derived structs embed their base first (repr(C)),
/// and the `vt` installed by the derived constructor guarantees the dynamic type.
pub(crate) fn cf_cast<T>(cf: &MSCFModel) -> &T {
    // SAFETY: T's first field chain starts with MSCFModel, and vt guarantees cf is a T
    unsafe { &*(cf as *const MSCFModel as *const T) }
}

/// Mutable counterpart of `cf_cast`
pub(crate) fn cf_cast_mut<T>(cf: &mut MSCFModel) -> &mut T {
    // SAFETY: see cf_cast
    unsafe { &mut *(cf as *mut MSCFModel as *mut T) }
}

/// Table installed by MSCFModel itself: every virtual runs the base body (pure ones panic)
pub(crate) const DEFAULT_MSCF_MODEL_VT: MSCFModelVt = MSCFModelVt {
    finalize_speed: default_finalize_speed,
    patch_speed_before_lc: default_patch_speed_before_lc,
    free_speed: default_free_speed,
    follow_speed: default_follow_speed,
    insertion_follow_speed: default_insertion_follow_speed,
    insertion_stop_speed: default_insertion_stop_speed,
    stop_speed_2: default_stop_speed_2,
    follow_speed_transient: default_follow_speed_transient,
    interaction_gap: default_interaction_gap,
    maximum_lane_speed_cf: default_maximum_lane_speed_cf,
    get_model_id: default_get_model_id,
    duplicate: default_duplicate,
    create_vehicle_variables: default_create_vehicle_variables,
    save_vehicle_variables: default_save_vehicle_variables,
    load_vehicle_variables: default_load_vehicle_variables,
    get_headway_time: default_get_headway_time,
    set_headway_time: default_set_headway_time,
    get_imperfection: default_get_imperfection,
    set_imperfection: default_set_imperfection,
    set_max_decel: default_set_max_decel,
    startup_delay_stopped: default_startup_delay_stopped,
    max_next_speed: default_max_next_speed,
    max_next_safe_min: default_max_next_safe_min,
    min_next_speed: default_min_next_speed,
    min_next_speed_emergency: default_min_next_speed_emergency,
    brake_gap_2: default_brake_gap_2,
    get_secure_gap: default_get_secure_gap,
    get_parameter: default_get_parameter,
    set_parameter: default_set_parameter,
};

impl MSCFModel {
    pub fn finalize_speed(&self, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
        (self.vt.finalize_speed)(self, veh, vPos)
    }
    pub fn patch_speed_before_lc(&self, veh: Ptr<MSVehicle>, vMin: f64, vMax: f64) -> f64 {
        (self.vt.patch_speed_before_lc)(self, veh, vMin, vMax)
    }
    pub fn free_speed(&self, veh: Ptr<MSVehicle>, speed: f64, seen: f64, maxSpeed: f64, onInsertion: bool, usage: MSCFModel_CalcReason) -> f64 {
        (self.vt.free_speed)(self, veh, speed, seen, maxSpeed, onInsertion, usage)
    }
    pub fn insertion_follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>) -> f64 {
        (self.vt.insertion_follow_speed)(self, veh, speed, gap2pred, predSpeed, predMaxDecel, pred)
    }
    pub fn insertion_stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap: f64) -> f64 {
        (self.vt.insertion_stop_speed)(self, veh, speed, gap)
    }
    pub fn follow_speed_transient(&self, duration: f64, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64) -> f64 {
        (self.vt.follow_speed_transient)(self, duration, veh, speed, gap2pred, predSpeed, predMaxDecel)
    }
    pub fn interaction_gap(&self, veh: Ptr<MSVehicle>, vL: f64) -> f64 {
        (self.vt.interaction_gap)(self, veh, vL)
    }
    pub fn maximum_lane_speed_cf(&self, veh: Ptr<MSVehicle>, maxSpeed: f64, maxSpeedLane: f64) -> f64 {
        (self.vt.maximum_lane_speed_cf)(self, veh, maxSpeed, maxSpeedLane)
    }
    pub fn create_vehicle_variables(&self) -> Ptr<MSCFModel_VehicleVariables> {
        (self.vt.create_vehicle_variables)(self)
    }
    pub fn save_vehicle_variables(&self, vars: &MSCFModel_VehicleVariables, out: &mut OutputDevice) {
        (self.vt.save_vehicle_variables)(self, vars, out)
    }
    pub fn load_vehicle_variables(&self, vars: &mut MSCFModel_VehicleVariables, attrs: &SUMOSAXAttributes) {
        (self.vt.load_vehicle_variables)(self, vars, attrs)
    }
    pub fn get_headway_time(&self) -> f64 {
        (self.vt.get_headway_time)(self)
    }
    pub fn set_headway_time(&mut self, headwayTime: f64) {
        (self.vt.set_headway_time)(self, headwayTime)
    }
    pub fn get_imperfection(&self) -> f64 {
        (self.vt.get_imperfection)(self)
    }
    pub fn set_imperfection(&mut self, imperfection: f64) {
        (self.vt.set_imperfection)(self, imperfection)
    }
    pub fn set_max_decel(&mut self, decel: f64) {
        (self.vt.set_max_decel)(self, decel)
    }
    pub fn startup_delay_stopped(&self) -> bool {
        (self.vt.startup_delay_stopped)(self)
    }
    pub fn max_next_speed(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        (self.vt.max_next_speed)(self, speed, veh)
    }
    pub fn max_next_safe_min(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        (self.vt.max_next_safe_min)(self, speed, veh)
    }
    pub fn min_next_speed(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        (self.vt.min_next_speed)(self, speed, veh)
    }
    pub fn min_next_speed_emergency(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        (self.vt.min_next_speed_emergency)(self, speed, veh)
    }
    pub fn brake_gap_2(&self, speed: f64, decel: f64, headwayTime: f64) -> f64 {
        (self.vt.brake_gap_2)(self, speed, decel, headwayTime)
    }
    pub fn get_secure_gap(&self, veh: Ptr<MSVehicle>, pred: Ptr<MSVehicle>, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64) -> f64 {
        (self.vt.get_secure_gap)(self, veh, pred, speed, leaderSpeed, leaderMaxDecel)
    }
}

/// The car-following model abstraction
/// C++ class `MSCFModel` (microsim/cfmodels/MSCFModel.h)
#[repr(C)]
#[derive(Clone)]
pub struct MSCFModel {
    /// The type to which this model definition belongs to
    pub myType: Ptr<MSVehicleType>,
    /// The vehicle's maximum acceleration [m/s^2]
    pub myAccel: f64,
    /// The vehicle's maximum deceleration [m/s^2]
    pub myDecel: f64,
    /// The vehicle's maximum emergency deceleration [m/s^2]
    pub myEmergencyDecel: f64,
    /// The vehicle's deceleration as expected by surrounding traffic [m/s^2]
    pub myApparentDecel: f64,
    /// The factor of minGap that must be maintained to avoid a collision event
    pub myCollisionMinGapFactor: f64,
    /// The driver's desired time headway (aka reaction time tau) [s]
    pub myHeadwayTime: f64,
    /// The startup delay after halting [s]
    pub myStartupDelay: SUMOTime,
    /// The vehicle's maximum acceleration profile [m/s^2]
    pub myMaxAccelProfile: LinearApproxMap,
    /// The vehicle's desired acceleration profile [m/s^2]
    pub myDesAccelProfile: LinearApproxMap,
    /// Virtual dispatch table; installed by the most derived constructor
    pub vt: &'static MSCFModelVt,
}

impl MSCFModel {
    /// C++: `void MSCFModel::MSCFModel(const MSVehicleType *)`
    /// Constructor
    // MSCFModel::MSCFModel(const MSVehicleType* vtype) :
    //     myType(vtype),
    //     myAccel(vtype->getParameter().getCFParam(SUMO_ATTR_ACCEL, SUMOVTypeParameter::getDefaultAccel(vtype->getParameter().vehicleClass))),
    //     myDecel(vtype->getParameter().getCFParam(SUMO_ATTR_DECEL, SUMOVTypeParameter::getDefaultDecel(vtype->getParameter().vehicleClass))),
    //     myEmergencyDecel(vtype->getParameter().getCFParam(SUMO_ATTR_EMERGENCYDECEL,
    //                      SUMOVTypeParameter::getDefaultEmergencyDecel(vtype->getParameter().vehicleClass, myDecel, MSGlobals::gDefaultEmergencyDecel))),
    //     myApparentDecel(vtype->getParameter().getCFParam(SUMO_ATTR_APPARENTDECEL, myDecel)),
    //     myCollisionMinGapFactor(vtype->getParameter().getCFParam(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 1)),
    //     myHeadwayTime(vtype->getParameter().getCFParam(SUMO_ATTR_TAU, 1.0)),
    //     myStartupDelay(TIME2STEPS(vtype->getParameter().getCFParam(SUMO_ATTR_STARTUP_DELAY, 0.0))),
    //     myMaxAccelProfile(vtype->getParameter().getCFProfile(SUMO_ATTR_MAXACCEL_PROFILE, SUMOVTypeParameter::getDefaultMaxAccelProfile(vtype->getParameter().vehicleClass, myAccel))),
    //     myDesAccelProfile(vtype->getParameter().getCFProfile(SUMO_ATTR_DESACCEL_PROFILE, SUMOVTypeParameter::getDefaultDesAccelProfile(vtype->getParameter().vehicleClass, myAccel)))
    // { }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        let p = vtype.get_parameter();
        let vc = p.vehicleClass;
        let myAccel = p.get_cf_param(SUMO_ATTR_ACCEL, SUMOVTypeParameter::get_default_accel(vc));
        let myDecel = p.get_cf_param(SUMO_ATTR_DECEL, SUMOVTypeParameter::get_default_decel(vc));
        let myEmergencyDecel = p.get_cf_param(SUMO_ATTR_EMERGENCYDECEL, SUMOVTypeParameter::get_default_emergency_decel(vc, myDecel, g_default_emergency_decel()));
        let myApparentDecel = p.get_cf_param(SUMO_ATTR_APPARENTDECEL, myDecel);
        let myCollisionMinGapFactor = p.get_cf_param(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 1.);
        let myHeadwayTime = p.get_cf_param(SUMO_ATTR_TAU, 1.0);
        let myStartupDelay = time2steps(p.get_cf_param(SUMO_ATTR_STARTUP_DELAY, 0.0));
        let maxAccelDefault = SUMOVTypeParameter::get_default_max_accel_profile(vc, myAccel);
        let myMaxAccelProfile = p.get_cf_profile(SUMO_ATTR_MAXACCEL_PROFILE, &maxAccelDefault);
        let desAccelDefault = SUMOVTypeParameter::get_default_des_accel_profile(vc, myAccel);
        let myDesAccelProfile = p.get_cf_profile(SUMO_ATTR_DESACCEL_PROFILE, &desAccelDefault);
        MSCFModel {
            myType: vtype,
            myAccel,
            myDecel,
            myEmergencyDecel,
            myApparentDecel,
            myCollisionMinGapFactor,
            myHeadwayTime,
            myStartupDelay,
            myMaxAccelProfile,
            myDesAccelProfile,
            vt: &DEFAULT_MSCF_MODEL_VT,
        }
    }

    /// C++: `void MSCFModel::~MSCFModel()` [virtual]
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is trivial: no owned resources to release
    }

    /// C++: `double MSCFModel::finalizeSpeed(MSVehicle *const, double) const` [const, virtual]
    /// Applies interaction with stops and lane changing model influences. Called at most once per simulation step (exactcly once per action step)
    // double
    // MSCFModel::finalizeSpeed(MSVehicle* const veh, double vPos) const {
    //     // save old v for optional acceleration computation
    //     const double oldV = veh->getSpeed();
    //     // process stops (includes update of stopping state)
    //     const double vStop = MIN2(vPos, veh->processNextStop(vPos));
    //     // apply deceleration bounds
    //     const double vMinEmergency = minNextSpeedEmergency(oldV, veh);
    //     // vPos contains the uppper bound on safe speed. allow emergency braking here
    //     const double vMin = MIN2(minNextSpeed(oldV, veh), MAX2(vPos, vMinEmergency));
    //     const double fric = veh->getFriction();
    //     // adapt speed limit of road to "perceived" friction
    //     const double factor = fric == 1. ? 1. : -0.3491 * fric * fric + 0.8922 * fric + 0.4493; //2nd degree polyfit
    //
    //     // aMax: Maximal admissible acceleration until the next action step, such that the vehicle's maximal
    //     // desired speed on the current lane will not be exceeded when the
    //     // acceleration is maintained until the next action step.
    //     double aMax = (MAX2(veh->getLane()->getVehicleMaxSpeed(veh), vPos) * factor - oldV) / veh->getActionStepLengthSecs();
    //     // apply planned speed constraints and acceleration constraints
    //     double vMax = MIN3(oldV + ACCEL2SPEED(aMax), maxNextSpeed(oldV, veh), vStop);
    //     // do not exceed max decel even if it is unsafe
    // #ifdef _DEBUG
    //     //if (vMin > vMax) {
    //     //    WRITE_WARNINGF(TL("Maximum speed of vehicle '%' is lower than the minimum speed (min: %, max: %)."), veh->getID(), toString(vMin), toString(vMax));
    //     //}
    // #endif
    //
    // #ifdef DEBUG_FINALIZE_SPEED
    //     if (DEBUG_COND) {
    //         std::cout << "\n" << SIMTIME << " FINALIZE_SPEED\n";
    //     }
    // #endif
    //
    //     vMax = MAX2(vMin, vMax);
    //     double vNext = patchSpeedBeforeLC(veh, vMin, vMax);
    // #ifdef DEBUG_FINALIZE_SPEED
    //     double vDawdle = vNext;
    // #endif
    //     assert(vNext >= vMin);
    //     assert(vNext <= vMax);
    //     // apply lane-changing related speed adaptations
    //     vNext = veh->getLaneChangeModel().patchSpeed(vMin, vNext, vMax, *this);
    // #ifdef DEBUG_FINALIZE_SPEED
    //     double vPatchLC = vNext;
    // #endif
    //     // apply further speed adaptations
    //     vNext = applyStartupDelay(veh, vMin, vNext);
    //
    //     assert(vNext >= vMinEmergency); // stronger braking is permitted in lane-changing related emergencies
    //     assert(vNext <= vMax);
    //
    // #ifdef DEBUG_FINALIZE_SPEED
    //     if (DEBUG_COND) {
    //         std::cout << std::setprecision(gPrecision)
    //                   << "veh '" << veh->getID() << "' oldV=" << oldV
    //                   << " vPos" << vPos
    //                   << " vMin=" << vMin
    //                   << " aMax=" << aMax
    //                   << " vMax=" << vMax
    //                   << " vStop=" << vStop
    //                   << " vDawdle=" << vDawdle
    //                   << " vPatchLC=" << vPatchLC
    //                   << " vNext=" << vNext
    //                   << "\n";
    //     }
    // #endif
    //     return vNext;
    // }
    #[allow(unused_variables)]
    pub fn finalize_speed_base(&self, veh: Ptr<MSVehicle>, vPos: f64) -> f64 {
        let oldV = veh.get_speed();
        let vStop = min2(vPos, veh.m().process_next_stop(vPos));
        let vMinEmergency = self.min_next_speed_emergency(oldV, veh);
        let vMin = min2(self.min_next_speed(oldV, veh), max2(vPos, vMinEmergency));
        let fric = veh.get_friction();
        let factor = if fric == 1. { 1. } else { -0.3491 * fric * fric + 0.8922 * fric + 0.4493 };
        let aMax = (max2(veh.get_lane().get_vehicle_max_speed(traffic_object(veh)), vPos) * factor - oldV) / veh.get_action_step_length_secs();
        let mut vMax = min3(oldV + accel2speed(aMax), self.max_next_speed(oldV, veh), vStop);
        vMax = max2(vMin, vMax);
        let mut vNext = self.patch_speed_before_lc(veh, vMin, vMax);
        vNext = veh.m().get_lane_change_model().patch_speed(vMin, vNext, vMax, self);
        vNext = self.apply_startup_delay(veh, vMin, vNext, 0);
        vNext
    }

    /// C++: `double MSCFModel::patchSpeedBeforeLC(const MSVehicle *, double, double) const` [const, virtual]
    /// apply custom speed adaptations within the given speed bounds
    // virtual double patchSpeedBeforeLC(const MSVehicle* veh, double vMin, double vMax) const {
    //         UNUSED_PARAMETER(veh);
    //         UNUSED_PARAMETER(vMin);
    //         return vMax;
    //     }
    #[allow(unused_variables)]
    pub fn patch_speed_before_lc_base(&self, veh: Ptr<MSVehicle>, vMin: f64, vMax: f64) -> f64 {
        let _ = (veh, vMin);
        vMax
    }

    /// C++: `double MSCFModel::applyStartupDelay(const MSVehicle *, const double, const double, const SUMOTime) const` [const, virtual]
    /// apply speed adaptation on startup
    // double
    // MSCFModel::applyStartupDelay(const MSVehicle* veh, const double vMin, const double vMax, const SUMOTime addTime) const {
    //     UNUSED_PARAMETER(vMin);
    //     // timeSinceStartup was already incremented by DELTA_T
    //     if (veh->getTimeSinceStartup() > 0 && veh->getTimeSinceStartup() - DELTA_T < myStartupDelay + addTime) {
    //         assert(veh->getSpeed() <= SUMO_const_haltingSpeed);
    //         const SUMOTime remainingDelay = myStartupDelay + addTime - (veh->getTimeSinceStartup() - DELTA_T);
    //         //std::cout << SIMTIME << " applyStartupDelay veh=" << veh->getID() << " remainingDelay=" << remainingDelay << "\n";
    //         if (remainingDelay >= DELTA_T) {
    //             // delay startup by at least a whole step
    //             return 0;
    //         } else {
    //             // reduce acceleration for fractional startup delay
    //             return (double)(DELTA_T - remainingDelay) / (double)DELTA_T * vMax;
    //         }
    //     }
    //     return vMax;
    // }
    #[allow(unused_variables)]
    pub fn apply_startup_delay(&self, veh: Ptr<MSVehicle>, vMin: f64, vMax: f64, addTime: SUMOTime) -> f64 {
        let _ = vMin;
        if veh.get_time_since_startup() > 0 && veh.get_time_since_startup() - delta_t() < self.myStartupDelay + addTime {
            let remainingDelay = self.myStartupDelay + addTime - (veh.get_time_since_startup() - delta_t());
            if remainingDelay >= delta_t() {
                // delay startup by at least a whole step
                return 0.;
            } else {
                // reduce acceleration for fractional startup delay
                return (delta_t() - remainingDelay) as f64 / delta_t() as f64 * vMax;
            }
        }
        vMax
    }

    /// C++: `double MSCFModel::interpolateProfile(const double, const std::vector<std::pair<double, double>>) const` [const, virtual]
    /// Get current interpolated value from a profile
    // double
    // MSCFModel::interpolateProfile(const double speed, const std::vector<std::pair<double, double> > profile) const {
    //     double val;
    //     // extrapolate, means using the first/last value of the array
    //     if (speed < profile[0].first) {
    //         val = profile[0].second;
    //     } else if (speed > profile.back().first) {
    //         val = profile.back().second;
    //     } else { // interpolate
    //         int x = 0;
    //         while (speed > profile[x + 1].first) {
    //             x++;
    //         }
    //         double diff = (profile[x + 1].second - profile[x].second) / (profile[x + 1].first - profile[x].first);
    //         val = profile[x].second + diff * (speed - profile[x].first);
    //     }
    //     return val;
    // }
    #[allow(unused_variables)]
    pub fn interpolate_profile(&self, speed: f64, profile: Vec<(f64, f64)>) -> f64 {
        let val;
        if speed < profile[0].0 {
            val = profile[0].1;
        } else if speed > profile[profile.len() - 1].0 {
            val = profile[profile.len() - 1].1;
        } else {
            let mut x = 0;
            while speed > profile[x + 1].0 {
                x += 1;
            }
            let diff = (profile[x + 1].1 - profile[x].1) / (profile[x + 1].0 - profile[x].0);
            val = profile[x].1 + diff * (speed - profile[x].0);
        }
        val
    }

    /// C++: `double MSCFModel::freeSpeed(const MSVehicle *const, double, double, double, const bool, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed without a leader
    // double
    // MSCFModel::freeSpeed(const MSVehicle* const veh, double speed, double seen, double maxSpeed, const bool onInsertion, const CalcReason /*usage*/) const {
    //     if (maxSpeed < 0.) {
    //         // can occur for ballistic update (in context of driving at red light)
    //         return maxSpeed;
    //     }
    //     double vSafe = freeSpeed(speed, myDecel, seen, maxSpeed, onInsertion, veh->getActionStepLengthSecs());
    //     return vSafe;
    // }
    #[allow(unused_variables)]
    pub fn free_speed_base(&self, veh: Ptr<MSVehicle>, speed: f64, seen: f64, maxSpeed: f64, onInsertion: bool, usage: MSCFModel_CalcReason) -> f64 {
        if maxSpeed < 0. {
            // can occur for ballistic update (in context of driving at red light)
            return maxSpeed;
        }
        let _ = usage;
        Self::free_speed_2(speed, self.myDecel, seen, maxSpeed, onInsertion, veh.get_action_step_length_secs())
    }

    /// C++: `double MSCFModel::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual, pure virtual]
    /// Computes the vehicle's follow speed (no dawdling)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        (self.vt.follow_speed)(self, veh, speed, gap2pred, predSpeed, predMaxDecel, pred, usage)
    }

    /// C++: `double MSCFModel::insertionFollowSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling) This method is used during the insertion stage. Whereas the method followSpeed returns the desired speed which may be lower than the safe speed, this method only considers safety constraints
    // double
    // MSCFModel::insertionFollowSpeed(const MSVehicle* const /* v */, double speed, double gap2pred, double predSpeed, double predMaxDecel, const MSVehicle* const /*pred*/) const {
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         return maximumSafeFollowSpeed(gap2pred, speed, predSpeed, predMaxDecel, true);
    //     } else {
    //         // NOTE: Even for ballistic update, the current speed is irrelevant at insertion, therefore passing 0. (Leo)
    //         return maximumSafeFollowSpeed(gap2pred, 0., predSpeed, predMaxDecel, true);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn insertion_follow_speed_base(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>) -> f64 {
        if g_semi_implicit_euler_update() {
            self.maximum_safe_follow_speed(gap2pred, speed, predSpeed, predMaxDecel, true)
        } else {
            // NOTE: Even for ballistic update, the current speed is irrelevant at insertion, therefore passing 0. (Leo)
            self.maximum_safe_follow_speed(gap2pred, 0., predSpeed, predMaxDecel, true)
        }
    }

    /// C++: `double MSCFModel::stopSpeed(const MSVehicle *const, const double, double, const CalcReason) const` [const]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // inline double stopSpeed(const MSVehicle* const veh, const double speed, double gap, const CalcReason usage = CalcReason::CURRENT) const {
    //         return stopSpeed(veh, speed, gap, myDecel, usage);
    //     }
    #[allow(unused_variables)]
    pub fn stop_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap: f64, usage: MSCFModel_CalcReason) -> f64 {
        self.stop_speed_2(veh, speed, gap, self.myDecel, usage)
    }

    /// C++: `double MSCFModel::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual, pure virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn stop_speed_2(&self, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 {
        (self.vt.stop_speed_2)(self, veh, speed, gap, decel, usage)
    }

    /// C++: `double MSCFModel::insertionStopSpeed(const MSVehicle *const, double, double) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching an obstacle at insertion without constraints due to acceleration capabilities and previous speeds.
    // double
    // MSCFModel::insertionStopSpeed(const MSVehicle* const veh, double speed, double gap) const {
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         return stopSpeed(veh, speed, gap, CalcReason::FUTURE);
    //     } else {
    //         return MIN2(maximumSafeStopSpeed(gap, myDecel, 0., true, 0., false), myType->getMaxSpeed());
    //     }
    // }
    #[allow(unused_variables)]
    pub fn insertion_stop_speed_base(&self, veh: Ptr<MSVehicle>, speed: f64, gap: f64) -> f64 {
        if g_semi_implicit_euler_update() {
            self.stop_speed(veh, speed, gap, MSCFModel_CalcReason::FUTURE)
        } else {
            min2(self.maximum_safe_stop_speed(gap, self.myDecel, 0., true, 0., false), self.myType.get_max_speed())
        }
    }

    /// C++: `double MSCFModel::followSpeedTransient(double, const MSVehicle *const, double, double, double, double) const` [const, virtual]
    /// Computes the vehicle's follow speed that avoids a collision for the given amount of time
    // double
    // MSCFModel::followSpeedTransient(double duration, const MSVehicle* const /*veh*/, double /*speed*/, double gap2pred, double predSpeed, double predMaxDecel) const {
    //     // minimium distance covered by the leader if braking
    //     double leaderMinDist = gap2pred + distAfterTime(duration, predSpeed, -predMaxDecel);
    //     // if ego would not brake it could drive with speed leaderMinDist / duration
    //     // due to potentential ego braking it can safely drive faster
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         // number of potential braking steps
    //         const int a = (int)ceil(duration / TS - TS);
    //         // can we brake for the whole time?
    //         if (brakeGap(a * myDecel, myDecel, 0) <= leaderMinDist) {
    //             // braking continuously for duration
    //             // distance reduction due to braking
    //             const double b = TS * getMaxDecel() * 0.5 * (a * a - a);
    //             if (gDebugFlag2) std::cout << "    followSpeedTransient"
    //                                            << " duration=" << duration
    //                                            << " gap=" << gap2pred
    //                                            << " leaderMinDist=" << leaderMinDist
    //                                            << " decel=" << getMaxDecel()
    //                                            << " a=" << a
    //                                            << " bg=" << brakeGap(a * myDecel, myDecel, 0)
    //                                            << " b=" << b
    //                                            << " x=" << (b + leaderMinDist) / duration
    //                                            << "\n";
    //             return (b + leaderMinDist) / duration;
    //         } else {
    //             // @todo improve efficiency
    //             double bg = 0;
    //             double speed = 0;
    //             while (bg < leaderMinDist) {
    //                 speed += ACCEL2SPEED(myDecel);
    //                 bg += SPEED2DIST(speed);
    //             }
    //             speed -= DIST2SPEED(bg - leaderMinDist);
    //             return speed;
    //         }
    //     } else {
    //         // can we brake for the whole time?
    //         const double fullBrakingSeconds = sqrt(leaderMinDist * 2 / myDecel);
    //         if (fullBrakingSeconds >= duration) {
    //             // braking continuously for duration
    //             // average speed after braking for duration is x2 = x - 0.5 * duration * myDecel
    //             // x2 * duration <= leaderMinDist must hold
    //             return leaderMinDist / duration + duration * getMaxDecel() / 2;
    //         } else {
    //             return fullBrakingSeconds * myDecel;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn follow_speed_transient_base(&self, duration: f64, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64) -> f64 {
        // minimium distance covered by the leader if braking
        let leaderMinDist = gap2pred + self.dist_after_time(duration, predSpeed, -predMaxDecel);
        // if ego would not brake it could drive with speed leaderMinDist / duration
        // due to potentential ego braking it can safely drive faster
        if g_semi_implicit_euler_update() {
            // number of potential braking steps
            let a = (duration / ts() - ts()).ceil() as i32;
            // can we brake for the whole time?
            if self.brake_gap_2(a as f64 * self.myDecel, self.myDecel, 0.) <= leaderMinDist {
                // braking continuously for duration
                // distance reduction due to braking
                let b = ts() * self.get_max_decel() * 0.5 * ((a * a - a) as f64);
                return (b + leaderMinDist) / duration;
            } else {
                let mut bg = 0.;
                let mut speed = 0.;
                while bg < leaderMinDist {
                    speed += accel2speed(self.myDecel);
                    bg += speed2dist(speed);
                }
                speed -= dist2speed(bg - leaderMinDist);
                return speed;
            }
        } else {
            // can we brake for the whole time?
            let fullBrakingSeconds = (leaderMinDist * 2. / self.myDecel).sqrt();
            if fullBrakingSeconds >= duration {
                // braking continuously for duration
                // average speed after braking for duration is x2 = x - 0.5 * duration * myDecel
                // x2 * duration <= leaderMinDist must hold
                return leaderMinDist / duration + duration * self.get_max_decel() / 2.;
            } else {
                return fullBrakingSeconds * self.myDecel;
            }
        }
    }

    /// C++: `double MSCFModel::interactionGap(const MSVehicle *const, double) const` [const, virtual]
    /// Returns the maximum gap at which an interaction between both vehicles occurs
    // double
    // MSCFModel::interactionGap(const MSVehicle* const veh, double vL) const {
    //     // Resolve the vsafe equation to gap. Assume predecessor has
    //     // speed != 0 and that vsafe will be the current speed plus acceleration,
    //     // i.e that with this gap there will be no interaction.
    //     const double vNext = MIN2(maxNextSpeed(veh->getSpeed(), veh), veh->getLane()->getVehicleMaxSpeed(veh));
    //     const double gap = (vNext - vL) *
    //                        ((veh->getSpeed() + vL) / (2.*myDecel) + myHeadwayTime) +
    //                        vL * myHeadwayTime;
    //
    //     // Don't allow timeHeadWay < deltaT situations.
    //     return MAX2(gap, SPEED2DIST(vNext));
    // }
    #[allow(unused_variables)]
    pub fn interaction_gap_base(&self, veh: Ptr<MSVehicle>, vL: f64) -> f64 {
        // Resolve the vsafe equation to gap. Assume predecessor has
        // speed != 0 and that vsafe will be the current speed plus acceleration,
        // i.e that with this gap there will be no interaction.
        let vNext = min2(self.max_next_speed(veh.get_speed(), veh), veh.get_lane().get_vehicle_max_speed(traffic_object(veh)));
        let gap = (vNext - vL) * ((veh.get_speed() + vL) / (2. * self.myDecel) + self.myHeadwayTime) + vL * self.myHeadwayTime;
        // Don't allow timeHeadWay < deltaT situations.
        max2(gap, speed2dist(vNext))
    }

    /// C++: `double MSCFModel::maximumLaneSpeedCF(const MSVehicle *const, double, double) const` [const, virtual]
    /// Returns the maximum velocity the CF-model wants to achieve in the next step
    // virtual double maximumLaneSpeedCF(const MSVehicle* const veh, double maxSpeed, double maxSpeedLane) const {
    //         double result = MIN2(maxSpeed, maxSpeedLane);
    //         applyOwnSpeedPerceptionError(veh, result);
    //         return result;
    //     }
    #[allow(unused_variables)]
    pub fn maximum_lane_speed_cf_base(&self, veh: Ptr<MSVehicle>, maxSpeed: f64, maxSpeedLane: f64) -> f64 {
        let mut result = min2(maxSpeed, maxSpeedLane);
        self.apply_own_speed_perception_error(veh, &mut result);
        result
    }

    /// C++: `int MSCFModel::getModelID() const` [const, virtual, pure virtual]
    /// Returns the model's ID; the XML-Tag number is used
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        (self.vt.get_model_id)(self)
    }

    /// C++: `MSCFModel * MSCFModel::duplicate(const MSVehicleType *) const` [const, virtual, pure virtual]
    /// Duplicates the car-following model
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        (self.vt.duplicate)(self, vtype)
    }

    /// C++: `VehicleVariables * MSCFModel::createVehicleVariables() const` [const, virtual]
    /// Returns model specific values which are stored inside a vehicle and must be used with casting
    // virtual VehicleVariables* createVehicleVariables() const {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn create_vehicle_variables_base(&self) -> Ptr<MSCFModel_VehicleVariables> {
        // C++ returns 0 (no model-specific variables)
        Ptr::default()
    }

    /// C++: `double MSCFModel::getMaxAccel() const` [const]
    /// Get the vehicle type's maximum acceleration [m/s^2]
    // inline double getMaxAccel() const {
    //         return myAccel;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_accel(&self) -> f64 {
        self.myAccel
    }

    /// C++: `double MSCFModel::getMaxDecel() const` [const]
    /// Get the vehicle type's maximal comfortable deceleration [m/s^2]
    // inline double getMaxDecel() const {
    //         return myDecel;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_decel(&self) -> f64 {
        self.myDecel
    }

    /// C++: `double MSCFModel::getEmergencyDecel() const` [const]
    /// Get the vehicle type's maximal physically possible deceleration [m/s^2]
    // inline double getEmergencyDecel() const {
    //         return myEmergencyDecel;
    //     }
    #[allow(unused_variables)]
    pub fn get_emergency_decel(&self) -> f64 {
        self.myEmergencyDecel
    }

    /// C++: `double MSCFModel::getApparentDecel() const` [const]
    /// Get the vehicle type's apparent deceleration [m/s^2] (the one regarded by its followers
    // inline double getApparentDecel() const {
    //         return myApparentDecel;
    //     }
    #[allow(unused_variables)]
    pub fn get_apparent_decel(&self) -> f64 {
        self.myApparentDecel
    }

    /// C++: `SUMOTime MSCFModel::getStartupDelay() const` [const]
    /// Get the vehicle type's startupDelay
    // inline SUMOTime getStartupDelay() const {
    //         return myStartupDelay;
    //     }
    #[allow(unused_variables)]
    pub fn get_startup_delay(&self) -> SUMOTime {
        self.myStartupDelay
    }

    /// C++: `double MSCFModel::getCurrentAccel(const double) const` [const, virtual]
    /// Get the vehicle type's maximum acceleration [m/s^2]
    // double
    // MSCFModel::getCurrentAccel(const double speed) const {
    //     double result = myAccel;
    //     if (!myDesAccelProfile.empty()) {
    //         result = MIN2(result, LinearApproxHelpers::getInterpolatedValue(myDesAccelProfile, speed));
    //     }
    //     if (!myMaxAccelProfile.empty()) {
    //         // @todo maxAccel should interact with slope
    //         result = MIN2(result, LinearApproxHelpers::getInterpolatedValue(myMaxAccelProfile, speed));
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_current_accel(&self, speed: f64) -> f64 {
        let mut result = self.myAccel;
        if !self.myDesAccelProfile.is_empty() {
            result = min2(result, get_interpolated_value(&self.myDesAccelProfile, speed));
        }
        if !self.myMaxAccelProfile.is_empty() {
            // @todo maxAccel should interact with slope
            result = min2(result, get_interpolated_value(&self.myMaxAccelProfile, speed));
        }
        result
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel::getMaxAccelProfile() const` [const]
    /// Get the vehicle type's maximum acceleration profile depending on the velocity [m/s^2]
    // inline LinearApproxHelpers::LinearApproxMap getMaxAccelProfile() const {
    //         return myMaxAccelProfile;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_accel_profile(&self) -> LinearApproxMap {
        self.myMaxAccelProfile.clone()
    }

    /// C++: `LinearApproxHelpers::LinearApproxMap MSCFModel::getDesAccelProfile() const` [const]
    /// Get the vehicle type's desired acceleration profile depending on the velocity [m/s^2]
    // inline LinearApproxHelpers::LinearApproxMap getDesAccelProfile() const {
    //         return myDesAccelProfile;
    //     }
    #[allow(unused_variables)]
    pub fn get_des_accel_profile(&self) -> LinearApproxMap {
        self.myDesAccelProfile.clone()
    }

    /// C++: `double MSCFModel::getCollisionMinGapFactor() const` [const]
    /// Get the factor of minGap that must be maintained to avoid a collision event
    // inline double getCollisionMinGapFactor() const {
    //         return myCollisionMinGapFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_collision_min_gap_factor(&self) -> f64 {
        self.myCollisionMinGapFactor
    }

    /// C++: `double MSCFModel::getImperfection() const` [const, virtual]
    /// Get the driver's imperfection
    // virtual double getImperfection() const {
    //         return -1;
    //     }
    #[allow(unused_variables)]
    pub fn get_imperfection_base(&self) -> f64 {
        -1.
    }

    /// C++: `double MSCFModel::getHeadwayTime() const` [const, virtual]
    /// Get the driver's desired headway [s]
    // virtual double getHeadwayTime() const {
    //         return myHeadwayTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_headway_time_base(&self) -> f64 {
        self.myHeadwayTime
    }

    /// C++: `bool MSCFModel::startupDelayStopped() const` [const, virtual]
    /// whether startupDelay should be applied after stopping
    // virtual bool startupDelayStopped() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn startup_delay_stopped_base(&self) -> bool {
        false
    }

    /// C++: `double MSCFModel::maxNextSpeed(double, const MSVehicle *const) const` [const, virtual]
    /// Returns the maximum speed given the current speed
    // double
    // MSCFModel::maxNextSpeed(double speed, const MSVehicle* const /*veh*/) const {
    //     return MIN2(speed + (double) ACCEL2SPEED(getCurrentAccel(speed)), myType->getMaxSpeed());
    // }
    #[allow(unused_variables)]
    pub fn max_next_speed_base(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        min2(speed + accel2speed(self.get_current_accel(speed)), self.myType.get_max_speed())
    }

    /// C++: `double MSCFModel::maxNextSafeMin(double, const MSVehicle *const) const` [const, virtual]
    /// Returns the maximum speed given the current speed and regarding driving dynamics
    // inline virtual double maxNextSafeMin(double speed, const MSVehicle* const veh = 0) const {
    //         return maxNextSpeed(speed, veh);
    //     }
    #[allow(unused_variables)]
    pub fn max_next_safe_min_base(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        self.max_next_speed(speed, veh)
    }

    /// C++: `double MSCFModel::minNextSpeed(double, const MSVehicle *const) const` [const, virtual]
    /// Returns the minimum speed given the current speed (depends on the numerical update scheme and its step width) Note that it wouldn't have to depend on the numerical update scheme if the semantics would rely on acceleration instead of velocity.
    // double
    // MSCFModel::minNextSpeed(double speed, const MSVehicle* const /*veh*/) const {
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         return MAX2(speed - ACCEL2SPEED(myDecel), 0.);
    //     } else {
    //         // NOTE: ballistic update allows for negative speeds to indicate a stop within the next timestep
    //         return speed - ACCEL2SPEED(myDecel);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn min_next_speed_base(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        if g_semi_implicit_euler_update() {
            max2(speed - accel2speed(self.myDecel), 0.)
        } else {
            // NOTE: ballistic update allows for negative speeds to indicate a stop within the next timestep
            speed - accel2speed(self.myDecel)
        }
    }

    /// C++: `double MSCFModel::minNextSpeedEmergency(double, const MSVehicle *const) const` [const, virtual]
    /// Returns the minimum speed after emergency braking, given the current speed (depends on the numerical update scheme and its step width) Note that it wouldn't have to depend on the numerical update scheme if the semantics would rely on acceleration instead of velocity.
    // double
    // MSCFModel::minNextSpeedEmergency(double speed, const MSVehicle* const /*veh*/) const {
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         return MAX2(speed - ACCEL2SPEED(myEmergencyDecel), 0.);
    //     } else {
    //         // NOTE: ballistic update allows for negative speeds to indicate a stop within the next timestep
    //         return speed - ACCEL2SPEED(myEmergencyDecel);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn min_next_speed_emergency_base(&self, speed: f64, veh: Ptr<MSVehicle>) -> f64 {
        if g_semi_implicit_euler_update() {
            max2(speed - accel2speed(self.myEmergencyDecel), 0.)
        } else {
            // NOTE: ballistic update allows for negative speeds to indicate a stop within the next timestep
            speed - accel2speed(self.myEmergencyDecel)
        }
    }

    /// C++: `double MSCFModel::brakeGap(const double) const` [const]
    /// Returns the distance the vehicle needs to halt including driver's reaction time tau (i.e. desired headway), assuming that during the reaction time, the speed remains constant
    // double brakeGap(const double speed) const {
    //         return brakeGap(speed, myDecel, myHeadwayTime);
    //     }
    #[allow(unused_variables)]
    pub fn brake_gap(&self, speed: f64) -> f64 {
        self.brake_gap_2(speed, self.myDecel, self.myHeadwayTime)
    }

    /// C++: `double MSCFModel::brakeGap(const double, const double, const double) const` [const, virtual]
    // double
    // MSCFModel::brakeGap(const double speed, const double decel, const double headwayTime) const {
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         return brakeGapEuler(speed, decel, headwayTime);
    //     } else {
    //         // ballistic
    //         if (speed <= 0) {
    //             return 0.;
    //         } else {
    //             return speed * (headwayTime + 0.5 * speed / decel);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn brake_gap_2_base(&self, speed: f64, decel: f64, headwayTime: f64) -> f64 {
        if g_semi_implicit_euler_update() {
            Self::brake_gap_euler(speed, decel, headwayTime)
        } else {
            // ballistic
            if speed <= 0. {
                0.
            } else {
                speed * (headwayTime + 0.5 * speed / decel)
            }
        }
    }

    /// C++: `double MSCFModel::brakeGapEuler(const double, const double, const double)` [static]
    // double
    // MSCFModel::brakeGapEuler(const double speed, const double decel, const double headwayTime) {
    //     /* one possibility to speed this up is to calculate speedReduction * steps * (steps+1) / 2
    //        for small values of steps (up to 10 maybe) and store them in an array */
    //     const double speedReduction = ACCEL2SPEED(decel);
    //     const int steps = int(speed / speedReduction);
    //     return SPEED2DIST(steps * speed - speedReduction * steps * (steps + 1) / 2) + speed * headwayTime;
    // }
    #[allow(unused_variables)]
    pub fn brake_gap_euler(speed: f64, decel: f64, headwayTime: f64) -> f64 {
        /* one possibility to speed this up is to calculate speedReduction * steps * (steps+1) / 2
           for small values of steps (up to 10 maybe) and store them in an array */
        let speedReduction = accel2speed(decel);
        let steps = (speed / speedReduction) as i32;
        speed2dist(steps as f64 * speed - speedReduction * steps as f64 * (steps + 1) as f64 / 2.) + speed * headwayTime
    }

    /// C++: `double MSCFModel::freeSpeed(const double, const double, const double, const double, const bool, const double)` [static]
    // double
    // MSCFModel::freeSpeed(const double currentSpeed, const double decel, const double dist, const double targetSpeed, const bool onInsertion, const double actionStepLength) {
    //     // XXX: (Leo) This seems to be exclusively called with decel = myDecel (max deceleration) and is not overridden
    //     // by any specific CFModel. That may cause undesirable hard braking (at junctions where the vehicle
    //     // changes to a road with a lower speed limit).
    //
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         // adapt speed to succeeding lane, no reaction time is involved
    //         // when breaking for y steps the following distance g is covered
    //         // (drive with v in the final step)
    //         // g = (y^2 + y) * 0.5 * b + y * v
    //         // y = ((((sqrt((b + 2.0*v)*(b + 2.0*v) + 8.0*b*g)) - b)*0.5 - v)/b)
    //         const double v = SPEED2DIST(targetSpeed);
    //         if (dist < v) {
    //             return targetSpeed;
    //         }
    //         const double b = ACCEL2DIST(decel);
    //         const double y = MAX2(0.0, ((sqrt((b + 2.0 * v) * (b + 2.0 * v) + 8.0 * b * dist) - b) * 0.5 - v) / b);
    //         const double yFull = floor(y);
    //         const double exactGap = (yFull * yFull + yFull) * 0.5 * b + yFull * v + (y > yFull ? v : 0.0);
    //         const double fullSpeedGain = (yFull + (onInsertion ? 1. : 0.)) * ACCEL2SPEED(decel);
    //         return DIST2SPEED(MAX2(0.0, dist - exactGap) / (yFull + 1)) + fullSpeedGain + targetSpeed;
    //     } else {
    //         // ballistic update (Leo)
    //         // calculate maximum next speed vN that is adjustable to vT=targetSpeed after a distance d=dist
    //         // and given a maximal deceleration b=decel, denote the current speed by v0.
    //         // the distance covered by a trajectory that attains vN in the next action step (length=dt) and decelerates afterwards
    //         // with b is given as
    //         // d = 0.5*dt*(v0+vN) + (t-dt)*vN - 0.5*b*(t-dt)^2, (1)
    //         // where time t of arrival at d with speed vT is
    //         // t = dt + (vN-vT)/b.  (2)
    //         // We insert (2) into (1) to obtain
    //         // d = 0.5*dt*(v0+vN) + vN*(vN-vT)/b - 0.5*b*((vN-vT)/b)^2
    //         // 0 = (dt*b*v0 - vT*vT - 2*b*d) + dt*b*vN + vN*vN
    //         // and solve for vN
    //
    //         assert(currentSpeed >= 0);
    //         assert(targetSpeed >= 0);
    //
    //         const double dt = onInsertion ? 0 : actionStepLength; // handles case that vehicle is inserted just now (at the end of move)
    //         const double v0 = currentSpeed;
    //         const double vT = targetSpeed;
    //         const double b = decel;
    //         const double d = dist - NUMERICAL_EPS; // prevent returning a value > targetSpeed due to rounding errors
    //
    //         // Solvability for positive vN (if d is small relative to v0):
    //         // 1) If 0.5*(v0+vT)*dt > d, we set vN=vT.
    //         // (In case vT<v0, this implies that on the interpolated trajectory there are points beyond d where
    //         //  the interpolated velocity is larger than vT, but at least on the temporal discretization grid, vT is not exceeded)
    //         // 2) We ignore the (possible) constraint vN >= v0 - b*dt, which could lead to a problem if v0 - t*b > vT.
    //         //    (finalizeSpeed() is responsible for assuring that the next velocity is chosen in accordance with maximal decelerations)
    //
    //         // If implied accel a leads to v0 + a*asl < vT, choose acceleration s.th. v0 + a*asl = vT
    //         if (0.5 * (v0 + vT)*dt >= d) {
    //             // Attain vT after time asl
    //             return v0 + TS * (vT - v0) / actionStepLength;
    //         } else {
    //             const double q = ((dt * v0 - 2 * d) * b - vT * vT); // (q < 0 is fulfilled because of (#))
    //             const double p = 0.5 * b * dt;
    //             const double vN = -p + sqrt(p * p - q); // target speed at time t0+asl
    //             return v0 + TS * (vN - v0) / actionStepLength;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn free_speed_2(currentSpeed: f64, decel: f64, dist: f64, maxSpeed: f64, onInsertion: bool, actionStepLength: f64) -> f64 {
        // XXX: (Leo) This seems to be exclusively called with decel = myDecel (max deceleration) and is not overridden
        // by any specific CFModel. That may cause undesirable hard braking (at junctions where the vehicle
        // changes to a road with a lower speed limit).
        if g_semi_implicit_euler_update() {
            // adapt speed to succeeding lane, no reaction time is involved
            // when breaking for y steps the following distance g is covered
            // (drive with v in the final step)
            // g = (y^2 + y) * 0.5 * b + y * v
            // y = ((((sqrt((b + 2.0*v)*(b + 2.0*v) + 8.0*b*g)) - b)*0.5 - v)/b)
            let v = speed2dist(maxSpeed);
            if dist < v {
                return maxSpeed;
            }
            let b = accel2dist(decel);
            let y = max2(0.0, ((((b + 2.0 * v) * (b + 2.0 * v) + 8.0 * b * dist).sqrt() - b) * 0.5 - v) / b);
            let yFull = y.floor();
            let exactGap = (yFull * yFull + yFull) * 0.5 * b + yFull * v + (if y > yFull { v } else { 0.0 });
            let fullSpeedGain = (yFull + (if onInsertion { 1. } else { 0. })) * accel2speed(decel);
            dist2speed(max2(0.0, dist - exactGap) / (yFull + 1.)) + fullSpeedGain + maxSpeed
        } else {
            // ballistic update (Leo)
            // calculate maximum next speed vN that is adjustable to vT=targetSpeed after a distance d=dist
            // and given a maximal deceleration b=decel, denote the current speed by v0.
            let dt = if onInsertion { 0. } else { actionStepLength }; // handles case that vehicle is inserted just now (at the end of move)
            let v0 = currentSpeed;
            let vT = maxSpeed;
            let b = decel;
            let d = dist - NUMERICAL_EPS; // prevent returning a value > targetSpeed due to rounding errors
            // If implied accel a leads to v0 + a*asl < vT, choose acceleration s.th. v0 + a*asl = vT
            if 0.5 * (v0 + vT) * dt >= d {
                // Attain vT after time asl
                v0 + ts() * (vT - v0) / actionStepLength
            } else {
                let q = (dt * v0 - 2. * d) * b - vT * vT; // (q < 0 is fulfilled because of (#))
                let p = 0.5 * b * dt;
                let vN = -p + (p * p - q).sqrt(); // target speed at time t0+asl
                v0 + ts() * (vN - v0) / actionStepLength
            }
        }
    }

    /// C++: `double MSCFModel::getSecureGap(const MSVehicle *const, const MSVehicle *const, const double, const double, const double) const` [const, virtual]
    /// Returns the minimum gap to reserve if the leader is braking at maximum (>=0)
    // double
    // MSCFModel::getSecureGap(const MSVehicle* const veh, const MSVehicle* const /*pred*/, const double speed, const double leaderSpeed, const double leaderMaxDecel) const {
    //     // The solution approach leaderBrakeGap >= followerBrakeGap is not
    //     // secure when the follower can brake harder than the leader because the paths may still cross.
    //     // As a workaround we use a value of leaderDecel which errs on the side of caution
    //     const double maxDecel = MAX2(myDecel, leaderMaxDecel);
    //     const double bgLeader = brakeGap(leaderSpeed, maxDecel, 0);
    //     double secureGap = MAX2(0.0, brakeGap(speed, myDecel, myHeadwayTime) - bgLeader);
    //     if (MSGlobals::gComputeLC && veh->getAcceleration() < -NUMERICAL_EPS) {
    //         // vehicle can react instantly in the next step
    //         // we only apply 'myHeadWayTime' to avoid sudden braking after lane change
    //         // thus we can reduce the required brakeGap if the vehicle is braking anyway
    //         // (but we shouldn't assume continued emergency deceleration)
    //         const double secureGapDecel = MAX2(0.0, brakeGap(speed, MIN2(-veh->getAcceleration(), myDecel), 0) - bgLeader);
    //         // the secureGapDecel doesn't leave room for lcAssertive behavior
    //         secureGap = MIN2(secureGap, secureGapDecel / veh->getLaneChangeModel().getSafetyFactor());
    //     }
    //     return secureGap;
    // }
    #[allow(unused_variables)]
    pub fn get_secure_gap_base(&self, veh: Ptr<MSVehicle>, arg1: Ptr<MSVehicle>, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64) -> f64 {
        // The solution approach leaderBrakeGap >= followerBrakeGap is not
        // secure when the follower can brake harder than the leader because the paths may still cross.
        // As a workaround we use a value of leaderDecel which errs on the side of caution
        let maxDecel = max2(self.myDecel, leaderMaxDecel);
        let bgLeader = self.brake_gap_2(leaderSpeed, maxDecel, 0.);
        let mut secureGap = max2(0.0, self.brake_gap_2(speed, self.myDecel, self.myHeadwayTime) - bgLeader);
        if g_compute_lc() && veh.get_acceleration() < -NUMERICAL_EPS {
            // vehicle can react instantly in the next step
            // we only apply 'myHeadWayTime' to avoid sudden braking after lane change
            // thus we can reduce the required brakeGap if the vehicle is braking anyway
            // (but we shouldn't assume continued emergency deceleration)
            let secureGapDecel = max2(0.0, self.brake_gap_2(speed, min2(-veh.get_acceleration(), self.myDecel), 0.) - bgLeader);
            // the secureGapDecel doesn't leave room for lcAssertive behavior
            secureGap = min2(secureGap, secureGapDecel / veh.m().get_lane_change_model().get_safety_factor());
        }
        secureGap
    }

    /// C++: `double MSCFModel::getSpeedAfterMaxDecel(double) const` [const, virtual]
    /// Returns the velocity after maximum deceleration
    // virtual /** @brief Returns the velocity after maximum deceleration
    //      * @param[in] v The velocity
    //      * @return The velocity after maximum deceleration
    //      */
    //     inline double getSpeedAfterMaxDecel(double v) const {
    //         return MAX2(0., v - ACCEL2SPEED(myDecel));
    //     }
    #[allow(unused_variables)]
    pub fn get_speed_after_max_decel(&self, v: f64) -> f64 {
        max2(0., v - accel2speed(self.myDecel))
    }

    /// C++: `SUMOTime MSCFModel::getMinimalArrivalTime(double, double, double) const` [const]
    /// Computes the minimal time needed to cover a distance given the desired speed at arrival.
    // SUMOTime
    // MSCFModel::getMinimalArrivalTime(double dist, double currentSpeed, double arrivalSpeed) const {
    //     if (dist <= 0.) {
    //         return 0;
    //     }
    //     // will either drive as fast as possible and decelerate as late as possible
    //     // or accelerate as fast as possible and then hold that speed
    //     arrivalSpeed = MAX2(0.0, arrivalSpeed); // ballistic model may call this with -std::numeric_limits<double>::max()
    //     const double accel = (arrivalSpeed >= currentSpeed) ? getMaxAccel() : -getMaxDecel();
    //     const double accelTime = accel == 0. ? 0. : (arrivalSpeed - currentSpeed) / accel;
    //     const double accelWay = accelTime * (arrivalSpeed + currentSpeed) * 0.5;
    //     if (dist >= accelWay) {
    //         const double nonAccelWay = dist - accelWay;
    //         const double nonAccelSpeed = MAX3(currentSpeed, arrivalSpeed, SUMO_const_haltingSpeed);
    //         return TIME2STEPS(accelTime + nonAccelWay / nonAccelSpeed);
    //     }
    //     // find time x so that
    //     // x * (currentSpeed + currentSpeed + x * accel) * 0.5 = dist
    //     return TIME2STEPS(-(currentSpeed - sqrt(currentSpeed * currentSpeed + 2 * accel * dist)) / accel);
    // }
    #[allow(unused_variables)]
    pub fn get_minimal_arrival_time(&self, dist: f64, currentSpeed: f64, arrivalSpeed: f64) -> SUMOTime {
        if dist <= 0. {
            return 0;
        }
        // will either drive as fast as possible and decelerate as late as possible
        // or accelerate as fast as possible and then hold that speed
        let arrivalSpeed = max2(0.0, arrivalSpeed); // ballistic model may call this with -std::numeric_limits<double>::max()
        let accel = if arrivalSpeed >= currentSpeed { self.get_max_accel() } else { -self.get_max_decel() };
        let accelTime = if accel == 0. { 0. } else { (arrivalSpeed - currentSpeed) / accel };
        let accelWay = accelTime * (arrivalSpeed + currentSpeed) * 0.5;
        if dist >= accelWay {
            let nonAccelWay = dist - accelWay;
            let nonAccelSpeed = max3(currentSpeed, arrivalSpeed, SUMO_CONST_HALTING_SPEED);
            return time2steps(accelTime + nonAccelWay / nonAccelSpeed);
        }
        // find time x so that
        // x * (currentSpeed + currentSpeed + x * accel) * 0.5 = dist
        time2steps(-(currentSpeed - (currentSpeed * currentSpeed + 2. * accel * dist).sqrt()) / accel)
    }

    /// C++: `double MSCFModel::estimateArrivalTime(double, double, double, double)` [static]
    /// Computes the time needed to travel a distance dist given an initial speed and constant acceleration. The speed during traveling is assumed not to exceed the max speed.
    // double
    // MSCFModel::estimateArrivalTime(double dist, double speed, double maxSpeed, double accel) {
    //     assert(speed >= 0.);
    //     assert(dist >= 0.);
    //
    //     if (dist < NUMERICAL_EPS) {
    //         return 0.;
    //     }
    //
    //     if ((accel < 0. && -0.5 * speed * speed / accel < dist) || (accel <= 0. && speed == 0.)) {
    //         // distance will never be covered with these values
    //         return INVALID_DOUBLE;
    //     }
    //
    //     if (fabs(accel) < NUMERICAL_EPS) {
    //         return dist / speed;
    //     }
    //
    //     double p = speed / accel;
    //
    //     if (accel < 0.) {
    //         // we already know, that the distance will be covered despite breaking
    //         return (-p - sqrt(p * p + 2 * dist / accel));
    //     }
    //
    //     // Here, accel > 0
    //     // t1 is the time to use the given acceleration
    //     double t1 = (maxSpeed - speed) / accel;
    //     // distance covered until t1
    //     double d1 = speed * t1 + 0.5 * accel * t1 * t1;
    //     if (d1 >= dist) {
    //         // dist is covered before changing the speed
    //         return (-p + sqrt(p * p + 2 * dist / accel));
    //     } else {
    //         return (-p + sqrt(p * p + 2 * d1 / accel)) + (dist - d1) / maxSpeed;
    //     }
    //
    // }
    #[allow(unused_variables)]
    pub fn estimate_arrival_time(dist: f64, speed: f64, maxSpeed: f64, accel: f64) -> f64 {
        // Computes the time needed to travel a distance dist given an initial speed and constant acceleration.
        if dist < NUMERICAL_EPS {
            return 0.;
        }
        if (accel < 0. && -0.5 * speed * speed / accel < dist) || (accel <= 0. && speed == 0.) {
            // distance will never be covered with these values
            return INVALID_DOUBLE;
        }
        if accel.abs() < NUMERICAL_EPS {
            return dist / speed;
        }
        let p = speed / accel;
        if accel < 0. {
            // we already know, that the distance will be covered despite breaking
            return -p - (p * p + 2. * dist / accel).sqrt();
        }
        // Here, accel > 0
        // t1 is the time to use the given acceleration
        let t1 = (maxSpeed - speed) / accel;
        // distance covered until t1
        let d1 = speed * t1 + 0.5 * accel * t1 * t1;
        if d1 >= dist {
            // dist is covered before changing the speed
            -p + (p * p + 2. * dist / accel).sqrt()
        } else {
            (-p + (p * p + 2. * d1 / accel).sqrt()) + (dist - d1) / maxSpeed
        }
    }

    /// C++: `double MSCFModel::estimateArrivalTime(double, double, double, double, double, double)` [static]
    /// Computes the time needed to travel a distance dist given an initial speed, arrival speed, constant acceleration and deceleration. The speed during traveling is assumed not to exceed the max speed.
    // double
    // MSCFModel::estimateArrivalTime(double dist, double initialSpeed, double arrivalSpeed, double maxSpeed, double accel, double decel) {
    //     UNUSED_PARAMETER(arrivalSpeed); // only in assertion
    //     UNUSED_PARAMETER(decel); // only in assertion
    //     if (dist <= 0) {
    //         return 0.;
    //     }
    //
    //     // stub-assumptions
    //     assert(accel == decel);
    //     assert(accel > 0);
    //     assert(initialSpeed == 0);
    //     assert(arrivalSpeed == 0);
    //     assert(maxSpeed > 0);
    //
    //
    //     double accelTime = (maxSpeed - initialSpeed) / accel;
    //     // "ballistic" estimate for the distance covered during acceleration phase
    //     double accelDist = accelTime * (initialSpeed + 0.5 * (maxSpeed - initialSpeed));
    //     double arrivalTime;
    //     if (accelDist >= dist * 0.5) {
    //         // maximal speed will not be attained during maneuver
    //         arrivalTime = 4 * sqrt(dist / accel);
    //     } else {
    //         // Calculate time to move with constant, maximal lateral speed
    //         const double constSpeedTime = (dist - accelDist * 2) / maxSpeed;
    //         arrivalTime = accelTime + constSpeedTime;
    //     }
    //     return arrivalTime;
    // }
    #[allow(unused_variables)]
    pub fn estimate_arrival_time_2(dist: f64, initialSpeed: f64, arrivalSpeed: f64, maxSpeed: f64, accel: f64, decel: f64) -> f64 {
        if dist <= 0. {
            return 0.;
        }
        // stub-assumptions (C++ asserts: accel == decel, accel > 0, initialSpeed == 0, arrivalSpeed == 0, maxSpeed > 0)
        let accelTime = (maxSpeed - initialSpeed) / accel;
        // "ballistic" estimate for the distance covered during acceleration phase
        let accelDist = accelTime * (initialSpeed + 0.5 * (maxSpeed - initialSpeed));
        let arrivalTime;
        if accelDist >= dist * 0.5 {
            // maximal speed will not be attained during maneuver
            arrivalTime = 4. * (dist / accel).sqrt();
        } else {
            // Calculate time to move with constant, maximal lateral speed
            let constSpeedTime = (dist - accelDist * 2.) / maxSpeed;
            arrivalTime = accelTime + constSpeedTime;
        }
        arrivalTime
    }

    /// C++: `double MSCFModel::avoidArrivalAccel(double, double, double, double)` [static]
    /// Computes the acceleration needed to arrive not before the given time
    // double
    // MSCFModel::avoidArrivalAccel(double dist, double time, double speed, double maxDecel) {
    //     assert(time > 0 || dist == 0);
    //     if (dist <= 0) {
    //         return -maxDecel;
    //     } else if (time * speed > 2 * dist) {
    //         // stop before dist is necessary. We need
    //         //            d = v*v/(2*a)
    //         return - 0.5 * speed * speed / dist;
    //     } else {
    //         // we seek the solution a of
    //         //            d = v*t + a*t*t/2
    //         return 2 * (dist / time - speed) / time;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn avoid_arrival_accel(dist: f64, time: f64, speed: f64, maxDecel: f64) -> f64 {
        // Computes the acceleration needed to arrive not before the given time
        if dist <= 0. {
            -maxDecel
        } else if time * speed > 2. * dist {
            // stop before dist is necessary. We need
            //            d = v*v/(2*a)
            -0.5 * speed * speed / dist
        } else {
            // we seek the solution a of
            //            d = v*t + a*t*t/2
            2. * (dist / time - speed) / time
        }
    }

    /// C++: `double MSCFModel::getMinimalArrivalSpeed(double, double) const` [const]
    /// Computes the minimal possible arrival speed after covering a given distance
    // double
    // MSCFModel::getMinimalArrivalSpeed(double dist, double currentSpeed) const {
    //     // ballistic update
    //     return estimateSpeedAfterDistance(dist - currentSpeed * getHeadwayTime(), currentSpeed, -getMaxDecel());
    // }
    #[allow(unused_variables)]
    pub fn get_minimal_arrival_speed(&self, dist: f64, currentSpeed: f64) -> f64 {
        // ballistic update
        self.estimate_speed_after_distance(dist - currentSpeed * self.get_headway_time(), currentSpeed, -self.get_max_decel())
    }

    /// C++: `double MSCFModel::getMinimalArrivalSpeedEuler(double, double) const` [const]
    /// Computes the minimal possible arrival speed after covering a given distance for Euler update
    // double
    // MSCFModel::getMinimalArrivalSpeedEuler(double dist, double currentSpeed) const {
    //     double arrivalSpeedBraking;
    //     // Because we use a continuous formula for computing the possible slow-down
    //     // we need to handle the mismatch with the discrete dynamics
    //     if (dist < currentSpeed) {
    //         arrivalSpeedBraking = INVALID_SPEED; // no time left for braking after this step
    //         //	(inserted max() to get rid of arrivalSpeed dependency within method) (Leo)
    //     } else if (2 * (dist - currentSpeed * getHeadwayTime()) * -getMaxDecel() + currentSpeed * currentSpeed >= 0) {
    //         arrivalSpeedBraking = estimateSpeedAfterDistance(dist - currentSpeed * getHeadwayTime(), currentSpeed, -getMaxDecel());
    //     } else {
    //         arrivalSpeedBraking = getMaxDecel();
    //     }
    //     return arrivalSpeedBraking;
    // }
    #[allow(unused_variables)]
    pub fn get_minimal_arrival_speed_euler(&self, dist: f64, currentSpeed: f64) -> f64 {
        let arrivalSpeedBraking;
        // Because we use a continuous formula for computing the possible slow-down
        // we need to handle the mismatch with the discrete dynamics
        if dist < currentSpeed {
            arrivalSpeedBraking = INVALID_SPEED; // no time left for braking after this step
        } else if 2. * (dist - currentSpeed * self.get_headway_time()) * -self.get_max_decel() + currentSpeed * currentSpeed >= 0. {
            arrivalSpeedBraking = self.estimate_speed_after_distance(dist - currentSpeed * self.get_headway_time(), currentSpeed, -self.get_max_decel());
        } else {
            arrivalSpeedBraking = self.get_max_decel();
        }
        arrivalSpeedBraking
    }

    /// C++: `double MSCFModel::gapExtrapolation(const double, const double, double, double, double, double, const double, const double)` [static]
    /// return the resulting gap if, starting with gap currentGap, two vehicles continue with constant accelerations (velocities bounded by 0 and maxSpeed) for a given timespan of length 'duration'.
    // double
    // MSCFModel::gapExtrapolation(const double duration, const double currentGap, double v1,  double v2, double a1, double a2, const double maxV1, const double maxV2) {
    //
    //     double newGap = currentGap;
    //
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         for (unsigned int steps = 1; steps * TS <= duration; ++steps) {
    //             v1 = MIN2(MAX2(v1 + a1, 0.), maxV1);
    //             v2 = MIN2(MAX2(v2 + a2, 0.), maxV2);
    //             newGap += TS * (v1 - v2);
    //         }
    //     } else {
    //         // determine times t1, t2 for which vehicles can break until stop (within duration)
    //         // and t3, t4 for which they reach their maximal speed on their current lanes.
    //         double t1 = 0, t2 = 0, t3 = 0, t4 = 0;
    //
    //         // t1: ego veh stops
    //         if (a1 < 0 && v1 > 0) {
    //             const double leaderStopTime =  - v1 / a1;
    //             t1 = MIN2(leaderStopTime, duration);
    //         } else if (a1 >= 0) {
    //             t1 = duration;
    //         }
    //         // t2: veh2 stops
    //         if (a2 < 0 && v2 > 0) {
    //             const double followerStopTime = -v2 / a2;
    //             t2 = MIN2(followerStopTime, duration);
    //         } else if (a2 >= 0) {
    //             t2 = duration;
    //         }
    //         // t3: ego veh reaches vMax
    //         if (a1 > 0 && v1 < maxV1) {
    //             const double leaderMaxSpeedTime = (maxV1 - v1) / a1;
    //             t3 = MIN2(leaderMaxSpeedTime, duration);
    //         } else if (a1 <= 0) {
    //             t3 = duration;
    //         }
    //         // t4: veh2 reaches vMax
    //         if (a2 > 0 && v2 < maxV2) {
    //             const double followerMaxSpeedTime = (maxV2 - v2) / a2;
    //             t4 = MIN2(followerMaxSpeedTime, duration);
    //         } else if (a2 <= 0) {
    //             t4 = duration;
    //         }
    //
    //         // NOTE: this assumes that the accelerations a1 and a2 are constant over the next
    //         //       followerBreakTime seconds (if no vehicle stops before or reaches vMax)
    //         std::list<double> l;
    //         l.push_back(t1);
    //         l.push_back(t2);
    //         l.push_back(t3);
    //         l.push_back(t4);
    //         l.sort();
    //         std::list<double>::const_iterator i;
    //         double tLast = 0.;
    //         for (i = l.begin(); i != l.end(); ++i) {
    //             if (*i != tLast) {
    //                 double dt = MIN2(*i, duration) - tLast; // time between *i and tLast
    //                 double dv = v1 - v2; // current velocity difference
    //                 double da = a1 - a2; // current acceleration difference
    //                 newGap += dv * dt + da * dt * dt / 2.; // update gap
    //                 v1 += dt * a1;
    //                 v2 += dt * a2;
    //             }
    //             if (*i == t1 || *i == t3) {
    //                 // ego veh reached velocity bound
    //                 a1 = 0.;
    //             }
    //
    //             if (*i == t2 || *i == t4) {
    //                 // veh2 reached velocity bound
    //                 a2 = 0.;
    //             }
    //
    //             tLast = MIN2(*i, duration);
    //             if (tLast == duration) {
    //                 break;
    //             }
    //         }
    //
    //         if (duration != tLast) {
    //             // (both vehicles have zero acceleration)
    //             assert(a1 == 0. && a2 == 0.);
    //             double dt = duration - tLast; // remaining time until duration
    //             double dv = v1 - v2; // current velocity difference
    //             newGap += dv * dt; // update gap
    //         }
    //     }
    //
    //     return newGap;
    // }
    #[allow(unused_variables)]
    pub fn gap_extrapolation(duration: f64, currentGap: f64, mut v1: f64, mut v2: f64, mut a1: f64, mut a2: f64, maxV1: f64, maxV2: f64) -> f64 {
        let mut newGap = currentGap;
        if g_semi_implicit_euler_update() {
            let mut steps: u32 = 1;
            while steps as f64 * ts() <= duration {
                v1 = min2(max2(v1 + a1, 0.), maxV1);
                v2 = min2(max2(v2 + a2, 0.), maxV2);
                newGap += ts() * (v1 - v2);
                steps += 1;
            }
        } else {
            // determine times t1, t2 for which vehicles can break until stop (within duration)
            // and t3, t4 for which they reach their maximal speed on their current lanes.
            let mut t1 = 0.;
            let mut t2 = 0.;
            let mut t3 = 0.;
            let mut t4 = 0.;
            // t1: ego veh stops
            if a1 < 0. && v1 > 0. {
                let leaderStopTime = -v1 / a1;
                t1 = min2(leaderStopTime, duration);
            } else if a1 >= 0. {
                t1 = duration;
            }
            // t2: veh2 stops
            if a2 < 0. && v2 > 0. {
                let followerStopTime = -v2 / a2;
                t2 = min2(followerStopTime, duration);
            } else if a2 >= 0. {
                t2 = duration;
            }
            // t3: ego veh reaches vMax
            if a1 > 0. && v1 < maxV1 {
                let leaderMaxSpeedTime = (maxV1 - v1) / a1;
                t3 = min2(leaderMaxSpeedTime, duration);
            } else if a1 <= 0. {
                t3 = duration;
            }
            // t4: veh2 reaches vMax
            if a2 > 0. && v2 < maxV2 {
                let followerMaxSpeedTime = (maxV2 - v2) / a2;
                t4 = min2(followerMaxSpeedTime, duration);
            } else if a2 <= 0. {
                t4 = duration;
            }
            // NOTE: this assumes that the accelerations a1 and a2 are constant over the next
            //       followerBreakTime seconds (if no vehicle stops before or reaches vMax)
            let mut l = vec![t1, t2, t3, t4];
            l.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mut tLast = 0.;
            for &i in l.iter() {
                if i != tLast {
                    let dt = min2(i, duration) - tLast; // time between *i and tLast
                    let dv = v1 - v2; // current velocity difference
                    let da = a1 - a2; // current acceleration difference
                    newGap += dv * dt + da * dt * dt / 2.; // update gap
                    v1 += dt * a1;
                    v2 += dt * a2;
                }
                if i == t1 || i == t3 {
                    // ego veh reached velocity bound
                    a1 = 0.;
                }
                if i == t2 || i == t4 {
                    // veh2 reached velocity bound
                    a2 = 0.;
                }
                tLast = min2(i, duration);
                if tLast == duration {
                    break;
                }
            }
            if duration != tLast {
                // (both vehicles have zero acceleration)
                let dt = duration - tLast; // remaining time until duration
                let dv = v1 - v2; // current velocity difference
                newGap += dv * dt; // update gap
            }
        }
        newGap
    }

    /// C++: `double MSCFModel::passingTime(const double, const double, const double, const double, const double)` [static]
    /// Calculates the time at which the position passedPosition has been passed In case of a ballistic update, the possibility of a stop within a time step requires more information about the last time-step than in case of the euler update to determine the last position if the currentSpeed is zero.
    // double
    // MSCFModel::passingTime(const double lastPos, const double passedPos, const double currentPos, const double lastSpeed, const double currentSpeed) {
    //
    //     assert(passedPos <= currentPos);
    //     assert(passedPos >= lastPos);
    //     assert(currentPos > lastPos);
    //     assert(currentSpeed >= 0);
    //
    //     if (passedPos > currentPos || passedPos < lastPos) {
    //         std::stringstream ss;
    //         // Debug (Leo)
    //         if (!MSGlobals::gSemiImplicitEulerUpdate) {
    //             // NOTE: error is guarded to maintain original test output for euler update (Leo).
    //             ss << "passingTime(): given argument passedPos = " << passedPos << " doesn't lie within [lastPos, currentPos] = [" << lastPos << ", " << currentPos << "]\nExtrapolating...";
    //             std::cout << ss.str() << "\n";
    //             WRITE_ERROR(ss.str());
    //         }
    //         const double lastCoveredDist = currentPos - lastPos;
    //         const double extrapolated = passedPos > currentPos ? TS * (passedPos - lastPos) / lastCoveredDist : TS * (currentPos - passedPos) / lastCoveredDist;
    //         return extrapolated;
    //     } else if (currentSpeed < 0) {
    //         WRITE_ERROR("passingTime(): given argument 'currentSpeed' is negative. This case is not handled yet.");
    //         return -1;
    //     }
    //
    //     const double distanceOldToPassed = passedPos - lastPos; // assert: >=0
    //
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         // euler update (constantly moving with currentSpeed during [0,TS])
    //         if (currentSpeed == 0) {
    //             return TS;
    //         }
    //         const double t = distanceOldToPassed / currentSpeed;
    //         return MIN2(TS, MAX2(0., t)); //rounding errors could give results out of the admissible result range
    //
    //     } else {
    //         // ballistic update (constant acceleration a during [0,TS], except in case of a stop)
    //
    //         // determine acceleration
    //         double a;
    //         if (currentSpeed > 0) {
    //             // the acceleration was constant within the last time step
    //             a = SPEED2ACCEL(currentSpeed - lastSpeed);
    //         } else {
    //             // the currentSpeed is zero (the last was not because lastPos<currentPos).
    //             assert(currentSpeed == 0 && lastSpeed != 0);
    //             // In general the stop has taken place within the last time step.
    //             // The acceleration (a<0) is obtained from
    //             // deltaPos = - lastSpeed^2/(2*a)
    //             a = lastSpeed * lastSpeed / (2 * (lastPos - currentPos));
    //
    //             assert(a < 0);
    //         }
    //
    //         // determine passing time t
    //         // we solve distanceOldToPassed = lastSpeed*t + a*t^2/2
    //         if (fabs(a) < NUMERICAL_EPS) {
    //             // treat as constant speed within [0, TS]
    //             const double t = 2 * distanceOldToPassed / (lastSpeed + currentSpeed);
    //             return MIN2(TS, MAX2(0., t)); //rounding errors could give results out of the admissible result range
    //         } else if (a > 0) {
    //             // positive acceleration => only one positive solution
    //             const double va = lastSpeed / a;
    //             const double t = -va + sqrt(va * va + 2 * distanceOldToPassed / a);
    //             assert(t < 1 && t >= 0);
    //             return t;
    //         } else {
    //             // negative acceleration => two positive solutions (pick the smaller one.)
    //             const double va = lastSpeed / a;
    //             const double t = -va - sqrt(va * va + 2 * distanceOldToPassed / a);
    //             // emergency braking at red light could give results out of the admissible result range
    //             // because the dynamics are euler-like (full forward speed with instant deceleration)
    //             return MIN2(TS, MAX2(0., t));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn passing_time(lastPos: f64, passedPos: f64, currentPos: f64, lastSpeed: f64, currentSpeed: f64) -> f64 {
        // Calculates the time at which the position passedPosition has been passed.
        // In case of a ballistic update, the possibility of a stop within a time step requires more information
        if passedPos > currentPos || passedPos < lastPos {
            // NOTE: error is guarded to maintain original test output for euler update (Leo).
            if !g_semi_implicit_euler_update() {
                let ss = format!("passingTime(): given argument passedPos = {} doesn't lie within [lastPos, currentPos] = [{}, {}]\nExtrapolating...", f64_to_string_prec(passedPos, 6), f64_to_string_prec(lastPos, 6), f64_to_string_prec(currentPos, 6));
                println!("{}", ss);
                utils::common::msg_handler::write_error(&ss);
            }
            let lastCoveredDist = currentPos - lastPos;
            let extrapolated = if passedPos > currentPos { ts() * (passedPos - lastPos) / lastCoveredDist } else { ts() * (currentPos - passedPos) / lastCoveredDist };
            return extrapolated;
        } else if currentSpeed < 0. {
            utils::common::msg_handler::write_error("passingTime(): given argument 'currentSpeed' is negative. This case is not handled yet.");
            return -1.;
        }
        let distanceOldToPassed = passedPos - lastPos; // assert: >=0
        if g_semi_implicit_euler_update() {
            // euler update (constantly moving with currentSpeed during [0,TS])
            if currentSpeed == 0. {
                return ts();
            }
            let t = distanceOldToPassed / currentSpeed;
            min2(ts(), max2(0., t)) //rounding errors could give results out of the admissible result range
        } else {
            // ballistic update (constant acceleration a during [0,TS], except in case of a stop)
            // determine acceleration
            let a;
            if currentSpeed > 0. {
                // the acceleration was constant within the last time step
                a = speed2accel(currentSpeed - lastSpeed);
            } else {
                // the currentSpeed is zero (the last was not because lastPos<currentPos).
                // In general the stop has taken place within the last time step.
                // The acceleration (a<0) is obtained from
                // deltaPos = - lastSpeed^2/(2*a)
                a = lastSpeed * lastSpeed / (2. * (lastPos - currentPos));
            }
            // determine passing time t
            // we solve distanceOldToPassed = lastSpeed*t + a*t^2/2
            if a.abs() < NUMERICAL_EPS {
                // treat as constant speed within [0, TS]
                let t = 2. * distanceOldToPassed / (lastSpeed + currentSpeed);
                min2(ts(), max2(0., t)) //rounding errors could give results out of the admissible result range
            } else if a > 0. {
                // positive acceleration => only one positive solution
                let va = lastSpeed / a;
                let t = -va + (va * va + 2. * distanceOldToPassed / a).sqrt();
                t
            } else {
                // negative acceleration => two positive solutions (pick the smaller one.)
                let va = lastSpeed / a;
                let t = -va - (va * va + 2. * distanceOldToPassed / a).sqrt();
                // emergency braking at red light could give results out of the admissible result range
                // because the dynamics are euler-like (full forward speed with instant deceleration)
                min2(ts(), max2(0., t))
            }
        }
    }

    /// C++: `double MSCFModel::speedAfterTime(const double, const double, const double)` [static]
    /// Calculates the speed after a time t [0,TS] given the initial speed and the distance traveled in an interval of step length TS.
    // double
    // MSCFModel::speedAfterTime(const double t, const double v0, const double dist) {
    //     assert(dist >= 0);
    //     assert(t >= 0 && t <= TS);
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         // euler: constant speed within [0,TS]
    //         return DIST2SPEED(dist);
    //     } else {
    //         // ballistic: piecewise constant acceleration in [0,TS] (may become 0 for a stop within TS)
    //         // We reconstruct acceleration at time t=0. Note that the covered distance in case
    //         // of a stop exactly at t=TS is TS*v0/2.
    //         if (dist <  TS * v0 / 2) {
    //             // stop must have occurred within [0,TS], use dist = -v0^2/(2a) (stopping dist),
    //             // i.e., a = -v0^2/(2*dist)
    //             const double accel = - v0 * v0 / (2 * dist);
    //             // The speed at time t is then
    //             return v0 + accel * t;
    //         } else {
    //             // no stop occurred within [0,TS], thus (from dist = v0*TS + accel*TS^2/2)
    //             const double accel = 2 * (dist / TS - v0) / TS;
    //             // The speed at time t is then
    //             return v0 + accel * t;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn speed_after_time(t: f64, oldSpeed: f64, dist: f64) -> f64 {
        if g_semi_implicit_euler_update() {
            // euler: constant speed within [0,TS]
            dist2speed(dist)
        } else {
            // ballistic: piecewise constant acceleration in [0,TS] (may become 0 for a stop within TS)
            // We reconstruct acceleration at time t=0. Note that the covered distance in case
            // of a stop exactly at t=TS is TS*v0/2.
            if dist < ts() * oldSpeed / 2. {
                // stop must have occurred within [0,TS], use dist = -v0^2/(2a) (stopping dist),
                // i.e., a = -v0^2/(2*dist)
                let accel = -oldSpeed * oldSpeed / (2. * dist);
                // The speed at time t is then
                oldSpeed + accel * t
            } else {
                // no stop occurred within [0,TS], thus (from dist = v0*TS + accel*TS^2/2)
                let accel = 2. * (dist / ts() - oldSpeed) / ts();
                // The speed at time t is then
                oldSpeed + accel * t
            }
        }
    }

    /// C++: `double MSCFModel::distAfterTime(double, double, double) const` [const, virtual]
    /// calculates the distance traveled after accelerating for time t
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn dist_after_time(&self, t: f64, speed: f64, accel: f64) -> f64 {
        // calculates the distance traveled after accelerating for time t
        if accel >= 0. {
            return (speed + 0.5 * accel * t) * t;
        }
        let decel = -accel;
        if speed <= decel * t {
            // braking to a full stop
            return self.brake_gap_2(speed, decel, 0.);
        }
        if g_semi_implicit_euler_update() {
            // @todo improve efficiency
            let mut result = 0.;
            let mut speed = speed;
            let mut t = t;
            while t > 0. {
                speed -= accel2speed(decel);
                result += max2(0.0, speed2dist(speed));
                t -= ts();
            }
            result
        } else {
            let speed2 = speed - t * decel;
            0.5 * (speed + speed2) * t
        }
    }

    /// C++: `double MSCFModel::estimateSpeedAfterDistance(const double, const double, const double) const` [const]
    // double
    // MSCFModel::estimateSpeedAfterDistance(const double dist, const double v, const double accel) const {
    //     // dist=v*t + 0.5*accel*t^2, solve for t and use v1 = v + accel*t
    //     return MIN2(myType->getMaxSpeed(),
    //                 (double)sqrt(MAX2(0., 2 * dist * accel + v * v)));
    // }
    #[allow(unused_variables)]
    pub fn estimate_speed_after_distance(&self, dist: f64, v: f64, accel: f64) -> f64 {
        // dist=v*t + 0.5*accel*t^2, solve for t and use v1 = v + accel*t
        min2(self.myType.get_max_speed(), max2(0., 2. * dist * accel + v * v).sqrt())
    }

    /// C++: `void MSCFModel::setMaxAccel(double)` [virtual]
    /// Sets a new value for maximum acceleration [m/s^2]
    // virtual void setMaxAccel(double accel) {
    //         myAccel = accel;
    //     }
    #[allow(unused_variables)]
    pub fn set_max_accel(&mut self, accel: f64) {
        self.myAccel = accel;
    }

    /// C++: `void MSCFModel::setMaxDecel(double)` [virtual]
    /// Sets a new value for maximal comfortable deceleration [m/s^2]
    // virtual void setMaxDecel(double decel) {
    //         myDecel = decel;
    //     }
    #[allow(unused_variables)]
    pub fn set_max_decel_base(&mut self, decel: f64) {
        self.myDecel = decel;
    }

    /// C++: `void MSCFModel::setEmergencyDecel(double)` [virtual]
    /// Sets a new value for maximal physically possible deceleration [m/s^2]
    // virtual void setEmergencyDecel(double decel) {
    //         myEmergencyDecel = decel;
    //     }
    #[allow(unused_variables)]
    pub fn set_emergency_decel(&mut self, decel: f64) {
        self.myEmergencyDecel = decel;
    }

    /// C++: `void MSCFModel::setApparentDecel(double)` [virtual]
    /// Sets a new value for the apparent deceleration [m/s^2]
    // virtual void setApparentDecel(double decel) {
    //         myApparentDecel = decel;
    //     }
    #[allow(unused_variables)]
    pub fn set_apparent_decel(&mut self, decel: f64) {
        self.myApparentDecel = decel;
    }

    /// C++: `void MSCFModel::setCollisionMinGapFactor(const double)`
    /// Sets a new value for the factor of minGap that must be maintained to avoid a collision event
    // inline void setCollisionMinGapFactor(const double factor) {
    //         myCollisionMinGapFactor = factor;
    //     }
    #[allow(unused_variables)]
    pub fn set_collision_min_gap_factor(&mut self, factor: f64) {
        self.myCollisionMinGapFactor = factor;
    }

    /// C++: `void MSCFModel::setMaxAccelProfile(const LinearApproxHelpers::LinearApproxMap &)` [virtual]
    /// Sets a new value for maximum acceleration profile [m/s^2]
    // virtual void setMaxAccelProfile(const LinearApproxHelpers::LinearApproxMap& accelProfile) {
    //         myMaxAccelProfile = accelProfile;
    //     }
    #[allow(unused_variables)]
    pub fn set_max_accel_profile(&mut self, accelProfile: &LinearApproxMap) {
        self.myMaxAccelProfile = accelProfile.clone();
    }

    /// C++: `void MSCFModel::setDesAccelProfile(const LinearApproxHelpers::LinearApproxMap &)` [virtual]
    /// Sets a new value for desired acceleration profile [m/s^2]
    // virtual void setDesAccelProfile(const LinearApproxHelpers::LinearApproxMap&  accelProfile) {
    //         myDesAccelProfile = accelProfile;
    //     }
    #[allow(unused_variables)]
    pub fn set_des_accel_profile(&mut self, accelProfile: &LinearApproxMap) {
        self.myDesAccelProfile = accelProfile.clone();
    }

    /// C++: `void MSCFModel::setImperfection(double)` [virtual]
    /// Sets a new value for driver imperfection
    // virtual void setImperfection(double imperfection) {
    //         UNUSED_PARAMETER(imperfection);
    //     }
    #[allow(unused_variables)]
    pub fn set_imperfection_base(&mut self, imperfection: f64) {
        let _ = imperfection;
    }

    /// C++: `void MSCFModel::setHeadwayTime(double)` [virtual]
    /// Sets a new value for desired headway [s]
    // virtual void setHeadwayTime(double headwayTime) {
    //         myHeadwayTime = headwayTime;
    //     }
    #[allow(unused_variables)]
    pub fn set_headway_time_base(&mut self, headwayTime: f64) {
        self.myHeadwayTime = headwayTime;
    }

    /// C++: `double MSCFModel::maximumSafeFollowSpeed(double, double, double, double, bool) const` [const]
    /// Returns the maximum safe velocity for following the given leader
    // double
    // MSCFModel::maximumSafeFollowSpeed(double gap, double egoSpeed, double predSpeed, double predMaxDecel, bool onInsertion) const {
    //     // the speed is safe if allows the ego vehicle to come to a stop behind the leader even if
    //     // the leaders starts braking hard until stopped
    //     // unfortunately it is not sufficient to compare stopping distances if the follower can brake harder than the leader
    //     // (the trajectories might intersect before both vehicles are stopped even if the follower has a shorter stopping distance than the leader)
    //     // To make things safe, we ensure that the leaders brake distance is computed with an deceleration that is at least as high as the follower's.
    //     // @todo: this is a conservative estimate for safe speed which could be increased
    //
    // //    // For negative gaps, we return the lowest meaningful value by convention
    // //    // XXX: check whether this is desireable (changes test results, therefore I exclude it for now (Leo), refs. #2575)
    //
    // //    //      It must be done. Otherwise, negative gaps at high speeds can create nonsense results from the call to maximumSafeStopSpeed() below
    //
    // //    if(gap<0){
    // //        if(MSGlobals::gSemiImplicitEulerUpdate){
    // //            return 0.;
    // //        } else {
    // //            return -INVALID_SPEED;
    // //        }
    // //    }
    //
    //     // The following commented code is a variant to assure brief stopping behind a stopped leading vehicle:
    //     // if leader is stopped, calculate stopSpeed without time-headway to prevent creeping stop
    //     // NOTE: this can lead to the strange phenomenon (for the Krauss-model at least) that if the leader comes to a stop,
    //     //       the follower accelerates for a short period of time. Refs #2310 (Leo)
    //     //    const double headway = predSpeed > 0. ? myHeadwayTime : 0.;
    //
    //     const double headway = myHeadwayTime;
    //     double x;
    //     if (gap >= 0 || MSGlobals::gComputeLC) {
    //         x = maximumSafeStopSpeed(gap + brakeGap(predSpeed, MAX2(myDecel, predMaxDecel), 0), myDecel, egoSpeed, onInsertion, headway, false);
    //     } else {
    //         x = egoSpeed - ACCEL2SPEED(myEmergencyDecel);
    //         if (MSGlobals::gSemiImplicitEulerUpdate) {
    //             x = MAX2(x, 0.);
    //         }
    //     }
    //
    //     if (myDecel != myEmergencyDecel && !onInsertion && !MSGlobals::gComputeLC) {
    //         double origSafeDecel = SPEED2ACCEL(egoSpeed - x);
    //         if (origSafeDecel > myDecel + NUMERICAL_EPS) {
    //             // Braking harder than myDecel was requested -> calculate required emergency deceleration.
    //             // Note that the resulting safeDecel can be smaller than the origSafeDecel, since the call to maximumSafeStopSpeed() above
    //             // can result in corrupted values (leading to intersecting trajectories) if, e.g. leader and follower are fast (leader still faster) and the gap is very small,
    //             // such that braking harder than myDecel is required.
    //
    //             double safeDecel = EMERGENCY_DECEL_AMPLIFIER * calculateEmergencyDeceleration(gap, egoSpeed, predSpeed, predMaxDecel);
    // #ifdef DEBUG_EMERGENCYDECEL
    //             if (DEBUG_COND2) {
    //                 std::cout << SIMTIME << " initial vsafe=" << x
    //                           << " egoSpeed=" << egoSpeed << " (origSafeDecel=" << origSafeDecel << ")"
    //                           << " predSpeed=" << predSpeed << " (predDecel=" << predMaxDecel << ")"
    //                           << " safeDecel=" << safeDecel
    //                           << std::endl;
    //             }
    // #endif
    //             // Don't be riskier than the usual method (myDecel <= safeDecel may occur, because a headway>0 is used above)
    //             safeDecel = MAX2(safeDecel, myDecel);
    //             // don't brake harder than originally planned (possible due to euler/ballistic mismatch)
    //             safeDecel = MIN2(safeDecel, origSafeDecel);
    //             x = egoSpeed - ACCEL2SPEED(safeDecel);
    //             if (MSGlobals::gSemiImplicitEulerUpdate) {
    //                 x = MAX2(x, 0.);
    //             }
    //
    // #ifdef DEBUG_EMERGENCYDECEL
    //             if (DEBUG_COND2) {
    //                 std::cout << "     -> corrected emergency deceleration: " << safeDecel << " newVSafe=" << x << std::endl;
    //             }
    // #endif
    //
    //         }
    //     }
    //     assert(x >= 0 || !MSGlobals::gSemiImplicitEulerUpdate);
    //     assert(!std::isnan(x));
    //     return x;
    // }
    #[allow(unused_variables)]
    pub fn maximum_safe_follow_speed(&self, gap: f64, egoSpeed: f64, predSpeed: f64, predMaxDecel: f64, onInsertion: bool) -> f64 {
        // the speed is safe if allows the ego vehicle to come to a stop behind the leader even if
        // the leaders starts braking hard until stopped
        // unfortunately it is not sufficient to compare stopping distances if the follower can brake harder than the leader
        // To make things safe, we ensure that the leaders brake distance is computed with an deceleration that is at least as high as the follower's.
        let headway = self.myHeadwayTime;
        let mut x;
        if gap >= 0. || g_compute_lc() {
            x = self.maximum_safe_stop_speed(gap + self.brake_gap_2(predSpeed, max2(self.myDecel, predMaxDecel), 0.), self.myDecel, egoSpeed, onInsertion, headway, false);
        } else {
            x = egoSpeed - accel2speed(self.myEmergencyDecel);
            if g_semi_implicit_euler_update() {
                x = max2(x, 0.);
            }
        }
        if self.myDecel != self.myEmergencyDecel && !onInsertion && !g_compute_lc() {
            let origSafeDecel = speed2accel(egoSpeed - x);
            if origSafeDecel > self.myDecel + NUMERICAL_EPS {
                // Braking harder than myDecel was requested -> calculate required emergency deceleration.
                let mut safeDecel = EMERGENCY_DECEL_AMPLIFIER * self.calculate_emergency_deceleration(gap, egoSpeed, predSpeed, predMaxDecel);
                // Don't be riskier than the usual method (myDecel <= safeDecel may occur, because a headway>0 is used above)
                safeDecel = max2(safeDecel, self.myDecel);
                // don't brake harder than originally planned (possible due to euler/ballistic mismatch)
                safeDecel = min2(safeDecel, origSafeDecel);
                x = egoSpeed - accel2speed(safeDecel);
                if g_semi_implicit_euler_update() {
                    x = max2(x, 0.);
                }
            }
        }
        x
    }

    /// C++: `double MSCFModel::calculateEmergencyDeceleration(double, double, double, double) const` [const]
    /// Returns the minimal deceleration for following the given leader safely
    // double
    // MSCFModel::calculateEmergencyDeceleration(double gap, double egoSpeed, double predSpeed, double predMaxDecel) const {
    //     // There are two cases:
    //     // 1) Either, stopping in time is possible with a deceleration b <= predMaxDecel, then this value is returned
    //     // 2) Or, b > predMaxDecel is required in this case the minimal value b allowing to stop safely under the assumption maxPredDecel=b is returned
    //     if (gap <= 0.) {
    //         return myEmergencyDecel;
    //     }
    //
    //     // Apparent braking distance for the leader
    //     const double predBrakeDist = 0.5 * predSpeed * predSpeed / predMaxDecel;
    //     // Required deceleration according to case 1)
    //     const double b1 = 0.5 * egoSpeed * egoSpeed / (gap + predBrakeDist);
    //
    // #ifdef DEBUG_EMERGENCYDECEL
    //     if (DEBUG_COND2) {
    //         std::cout << SIMTIME << " calculateEmergencyDeceleration()"
    //                   << " gap=" << gap << " egoSpeed=" << egoSpeed << " predSpeed=" << predSpeed
    //                   << " predBrakeDist=" << predBrakeDist
    //                   << " b1=" << b1
    //                   << std::endl;
    //     }
    // #endif
    //
    //     if (b1 <= predMaxDecel) {
    //         // Case 1) applies
    // #ifdef DEBUG_EMERGENCYDECEL
    //         if (DEBUG_COND2) {
    //             std::cout << "       case 1 ..." << std::endl;
    //         }
    // #endif
    //         return b1;
    //     }
    // #ifdef DEBUG_EMERGENCYDECEL
    //     if (DEBUG_COND2) {
    //         std::cout << "       case 2 ...";
    //     }
    // #endif
    //
    //     // Case 2) applies
    //     // Required deceleration according to case 2)
    //     const double b2 = 0.5 * (egoSpeed * egoSpeed - predSpeed * predSpeed) / gap;
    //
    // #ifdef DEBUG_EMERGENCYDECEL
    //     if (DEBUG_COND2) {
    //         std::cout << " b2=" << b2 << std::endl;
    //     }
    // #endif
    //     return b2;
    // }
    #[allow(unused_variables)]
    pub fn calculate_emergency_deceleration(&self, gap: f64, egoSpeed: f64, predSpeed: f64, predMaxDecel: f64) -> f64 {
        // There are two cases:
        // 1) Either, stopping in time is possible with a deceleration b <= predMaxDecel, then this value is returned
        // 2) Or, b > predMaxDecel is required in this case the minimal value b allowing to stop safely under the assumption maxPredDecel=b is returned
        if gap <= 0. {
            return self.myEmergencyDecel;
        }
        // Apparent braking distance for the leader
        let predBrakeDist = 0.5 * predSpeed * predSpeed / predMaxDecel;
        // Required deceleration according to case 1)
        let b1 = 0.5 * egoSpeed * egoSpeed / (gap + predBrakeDist);
        if b1 <= predMaxDecel {
            // Case 1) applies
            return b1;
        }
        // Case 2) applies
        // Required deceleration according to case 2)
        let b2 = 0.5 * (egoSpeed * egoSpeed - predSpeed * predSpeed) / gap;
        b2
    }

    /// C++: `double MSCFModel::maximumSafeStopSpeed(double, double, double, bool, double, bool) const` [const]
    /// Returns the maximum next velocity for stopping within gap
    // double
    // MSCFModel::maximumSafeStopSpeed(double gap, double decel, double currentSpeed, bool onInsertion, double headway, bool relaxEmergency) const {
    //     double vsafe;
    //     if (MSGlobals::gSemiImplicitEulerUpdate) {
    //         vsafe = maximumSafeStopSpeedEuler(gap, decel, onInsertion, headway);
    //     } else {
    //         vsafe = maximumSafeStopSpeedBallistic(gap, decel, currentSpeed, onInsertion, headway);
    //     }
    //
    //     if (relaxEmergency && myDecel != myEmergencyDecel) {
    // #ifdef DEBUG_EMERGENCYDECEL
    //         if (true) {
    //             std::cout << SIMTIME << " maximumSafeStopSpeed()"
    //                       << " g=" << gap
    //                       << " v=" << currentSpeed
    //                       << " initial vsafe=" << vsafe << "(decel=" << SPEED2ACCEL(v - vsafe) << ")" << std::endl;
    //         }
    // #endif
    //
    //         double origSafeDecel = SPEED2ACCEL(currentSpeed - vsafe);
    //         if (origSafeDecel > myDecel + NUMERICAL_EPS) {
    //             // emergency deceleration required
    //
    // #ifdef DEBUG_EMERGENCYDECEL
    //             if (true) {
    //                 std::cout << SIMTIME << " maximumSafeStopSpeed() results in emergency deceleration "
    //                           << "initial vsafe=" << vsafe  << " egoSpeed=" << v << "(decel=" << SPEED2ACCEL(v - vsafe) << ")" << std::endl;
    //             }
    // #endif
    //
    //             double safeDecel = EMERGENCY_DECEL_AMPLIFIER * calculateEmergencyDeceleration(gap, currentSpeed, 0., 1);
    //             // Don't be riskier than the usual method (myDecel <= safeDecel may occur, because a headway>0 is used above)
    //             safeDecel = MAX2(safeDecel, myDecel);
    //             // don't brake harder than originally planned (possible due to euler/ballistic mismatch)
    //             safeDecel = MIN2(safeDecel, origSafeDecel);
    //             vsafe = currentSpeed - ACCEL2SPEED(safeDecel);
    //             if (MSGlobals::gSemiImplicitEulerUpdate) {
    //                 vsafe = MAX2(vsafe, 0.);
    //             }
    //
    // #ifdef DEBUG_EMERGENCYDECEL
    //             if (true) {
    //                 std::cout << "     -> corrected emergency deceleration: " << SPEED2ACCEL(v - vsafe) << std::endl;
    //             }
    // #endif
    //
    //         }
    //     }
    //
    //     return vsafe;
    // }
    #[allow(unused_variables)]
    pub fn maximum_safe_stop_speed(&self, gap: f64, decel: f64, currentSpeed: f64, onInsertion: bool, headway: f64, relaxEmergency: bool) -> f64 {
        let mut vsafe;
        if g_semi_implicit_euler_update() {
            vsafe = self.maximum_safe_stop_speed_euler(gap, decel, onInsertion, headway);
        } else {
            vsafe = self.maximum_safe_stop_speed_ballistic(gap, decel, currentSpeed, onInsertion, headway);
        }
        if relaxEmergency && self.myDecel != self.myEmergencyDecel {
            let origSafeDecel = speed2accel(currentSpeed - vsafe);
            if origSafeDecel > self.myDecel + NUMERICAL_EPS {
                // emergency deceleration required
                let mut safeDecel = EMERGENCY_DECEL_AMPLIFIER * self.calculate_emergency_deceleration(gap, currentSpeed, 0., 1.);
                // Don't be riskier than the usual method (myDecel <= safeDecel may occur, because a headway>0 is used above)
                safeDecel = max2(safeDecel, self.myDecel);
                // don't brake harder than originally planned (possible due to euler/ballistic mismatch)
                safeDecel = min2(safeDecel, origSafeDecel);
                vsafe = currentSpeed - accel2speed(safeDecel);
                if g_semi_implicit_euler_update() {
                    vsafe = max2(vsafe, 0.);
                }
            }
        }
        vsafe
    }

    /// C++: `double MSCFModel::maximumSafeStopSpeedEuler(double, double, bool, double) const` [const]
    /// Returns the maximum next velocity for stopping within gap when using the semi-implicit Euler update
    // double
    // MSCFModel::maximumSafeStopSpeedEuler(double gap, double decel, bool /* onInsertion */, double headway) const {
    //     // decrease gap slightly (to avoid passing end of lane by values of magnitude ~1e-12, when exact stop is required)
    //     const double g = gap - NUMERICAL_EPS;
    //     if (g < 0.) {
    //         return 0.;
    //     }
    //     const double b = ACCEL2SPEED(decel);
    //     const double t = headway >= 0 ? headway : myHeadwayTime;
    //     const double s = TS;
    //
    //     // h = the distance that would be covered if it were possible to stop
    //     // exactly after gap and decelerate with b every simulation step
    //     // h = 0.5 * n * (n-1) * b * s + n * b * t (solve for n)
    //     //n = ((1.0/2.0) - ((t + (pow(((s*s) + (4.0*((s*((2.0*h/b) - t)) + (t*t)))), (1.0/2.0))*sign/2.0))/s));
    //     const double n = floor(.5 - ((t + (sqrt(((s * s) + (4.0 * ((s * (2.0 * g / b - t)) + (t * t))))) * -0.5)) / s));
    //     const double h = 0.5 * n * (n - 1) * b * s + n * b * t;
    //     assert(h <= g + NUMERICAL_EPS);
    //     // compute the additional speed that must be used during deceleration to fix
    //     // the discrepancy between g and h
    //     const double r = (g - h) / (n * s + t);
    //     const double x = n * b + r;
    //     assert(x >= 0);
    //     return x;
    // //    return onInsertion ? x + b: x; // see #2574
    // }
    #[allow(unused_variables)]
    pub fn maximum_safe_stop_speed_euler(&self, gap: f64, decel: f64, onInsertion: bool, headway: f64) -> f64 {
        // decrease gap slightly (to avoid passing end of lane by values of magnitude ~1e-12, when exact stop is required)
        let g = gap - NUMERICAL_EPS;
        if g < 0. {
            return 0.;
        }
        let b = accel2speed(decel);
        let t = if headway >= 0. { headway } else { self.myHeadwayTime };
        let s = ts();
        // h = the distance that would be covered if it were possible to stop
        // exactly after gap and decelerate with b every simulation step
        // h = 0.5 * n * (n-1) * b * s + n * b * t (solve for n)
        let sqrtArg = (s * s) + (4.0 * ((s * (2.0 * g / b - t)) + (t * t)));
        let n = (0.5 - ((t + (sqrtArg.sqrt() * -0.5)) / s)).floor();
        let h = 0.5 * n * (n - 1.) * b * s + n * b * t;
        // compute the additional speed that must be used during deceleration to fix
        // the discrepancy between g and h
        let r = (g - h) / (n * s + t);
        let x = n * b + r;
        x
    }

    /// C++: `double MSCFModel::maximumSafeStopSpeedBallistic(double, double, double, bool, double) const` [const]
    /// Returns the maximum next velocity for stopping within gap when using the ballistic positional update.
    // double
    // MSCFModel::maximumSafeStopSpeedBallistic(double gap, double decel, double currentSpeed, bool onInsertion, double headway) const {
    //     // decrease gap slightly (to avoid passing end of lane by values of magnitude ~1e-12, when exact stop is required)
    //     const double g = MAX2(0., gap - NUMERICAL_EPS);
    //     headway = headway >= 0 ? headway : myHeadwayTime;
    //
    //     // (Leo) Note that in contrast to the Euler update, for the ballistic update
    //     // the distance covered in the coming step depends on the current velocity, in general.
    //     // one exception is the situation when the vehicle is just being inserted.
    //     // In that case, it will not cover any distance until the next timestep by convention.
    //
    //     // We treat the latter case first:
    //     if (onInsertion) {
    //         // The distance covered with constant insertion speed v0 until time tau is given as
    //         // G1 = tau*v0
    //         // The distance covered between time tau and the stopping moment at time tau+v0/b is
    //         // G2 = v0^2/(2b),
    //         // where b is an assumed constant deceleration (= decel)
    //         // We solve g = G1 + G2 for v0:
    //         const double btau = decel * headway;
    //         const double v0 = -btau + sqrt(btau * btau + 2 * decel * g);
    //         return v0;
    //     }
    //
    //     // In the usual case during the driving task, the vehicle goes by
    //     // a current speed v0=v, and we seek to determine a safe acceleration a (possibly <0)
    //     // such that starting to break after accelerating with a for the time tau=headway
    //     // still allows us to stop in time.
    //
    //     const double tau = headway == 0 ? TS : headway;
    //     const double v0 = MAX2(0., currentSpeed);
    //     // We first consider the case that a stop has to take place within time tau
    //     // (the distance driven when decelerating from v0 to 0 in tau is v0 * tau / 2)
    //     if (g <= v0 * tau * 0.5) {
    //         if (g == 0.) {
    //             if (v0 > 0.) {
    //                 // indicate to brake as hard as possible
    //                 return -ACCEL2SPEED(myEmergencyDecel);
    //             } else {
    //                 // stay stopped
    //                 return 0.;
    //             }
    //         }
    //         // In general we solve g = v0^2/(-2a), where the rhs is the distance
    //         // covered until stop when braking with a<0
    //         const double a = -v0 * v0 / (2 * g);
    //         return v0 + a * TS;
    //     }
    //
    //     // The last case corresponds to a situation, where the vehicle may go with a positive
    //     // speed v1 = v0 + tau*a after time tau. (v1 is the maximum possible speed
    //     // for this and unconstrained by current speed or acceleration limits)
    //     //
    //     // The distance covered until time tau is given as
    //     // G1 = tau*(v0+v1)/2
    //     // The distance covered between time tau and the stopping moment at time tau+v1/b is
    //     // G2 = v1^2/(2b),
    //     // where b is an assumed constant deceleration (= decel)
    //     // We solve g = G1 + G2 for v1>0:
    //     // <=> 0 = v1^2 + b*tau*v1 + b*tau*v0 - 2bg
    //     //  => v1 = -b*tau/2 + sqrt( (b*tau)^2/4 + b(2g - tau*v0) )
    //
    //     const double btau2 = decel * tau / 2;
    //     const double v1 = -btau2 + sqrt(btau2 * btau2 + decel * (2 * g - tau * v0));
    //     const double a = (v1 - v0) / tau;
    //     return v0 + a * TS;
    // }
    #[allow(unused_variables)]
    pub fn maximum_safe_stop_speed_ballistic(&self, gap: f64, decel: f64, currentSpeed: f64, onInsertion: bool, headway: f64) -> f64 {
        // decrease gap slightly (to avoid passing end of lane by values of magnitude ~1e-12, when exact stop is required)
        let g = max2(0., gap - NUMERICAL_EPS);
        let headway = if headway >= 0. { headway } else { self.myHeadwayTime };
        // (Leo) Note that in contrast to the Euler update, for the ballistic update
        // the distance covered in the coming step depends on the current velocity, in general.
        // one exception is the situation when the vehicle is just being inserted.
        // In that case, it will not cover any distance until the next timestep by convention.
        // We treat the latter case first:
        if onInsertion {
            // The distance covered with constant insertion speed v0 until time tau is given as
            // G1 = tau*v0
            // The distance covered between time tau and the stopping moment at time tau+v0/b is
            // G2 = v0^2/(2b),
            // where b is an assumed constant deceleration (= decel)
            // We solve g = G1 + G2 for v0:
            let btau = decel * headway;
            let v0 = -btau + (btau * btau + 2. * decel * g).sqrt();
            return v0;
        }
        // In the usual case during the driving task, the vehicle goes by
        // a current speed v0=v, and we seek to determine a safe acceleration a (possibly <0)
        // such that starting to break after accelerating with a for the time tau=headway
        // still allows us to stop in time.
        let tau = if headway == 0. { ts() } else { headway };
        let v0 = max2(0., currentSpeed);
        // We first consider the case that a stop has to take place within time tau
        // (the distance driven when decelerating from v0 to 0 in tau is v0 * tau / 2)
        if g <= v0 * tau * 0.5 {
            if g == 0. {
                if v0 > 0. {
                    // indicate to brake as hard as possible
                    return -accel2speed(self.myEmergencyDecel);
                } else {
                    // stay stopped
                    return 0.;
                }
            }
            // In general we solve g = v0^2/(-2a), where the rhs is the distance
            // covered until stop when braking with a<0
            let a = -v0 * v0 / (2. * g);
            return v0 + a * ts();
        }
        // The last case corresponds to a situation, where the vehicle may go with a positive
        // speed v1 = v0 + tau*a after time tau. (v1 is the maximum possible speed
        // for this and unconstrained by current speed or acceleration limits)
        // => v1 = -b*tau/2 + sqrt( (b*tau)^2/4 + b(2g - tau*v0) )
        let btau2 = decel * tau / 2.;
        let v1 = -btau2 + (btau2 * btau2 + decel * (2. * g - tau * v0)).sqrt();
        let a = (v1 - v0) / tau;
        v0 + a * ts()
    }

    /// C++: `std::string MSCFModel::getParameter(const MSVehicle *, const std::string &) const` [const, virtual]
    /// try to get the given parameter for this carFollowingModel
    // virtual std::string getParameter(const MSVehicle* veh, const std::string& key) const {
    //         UNUSED_PARAMETER(veh);
    //         UNUSED_PARAMETER(key);
    //         return "";
    //     }
    pub fn get_parameter(&self, veh: Ptr<MSVehicle>, key: &str) -> String {
        (self.vt.get_parameter)(self, veh, key)
    }
    pub fn set_parameter(&self, veh: Ptr<MSVehicle>, key: &str, value: &str) -> Result<(), InvalidArgument> {
        (self.vt.set_parameter)(self, veh, key, value)
    }

    /// C++: `std::string MSCFModel::getParameter(const MSVehicle *, const std::string &) const` (base body)
    #[allow(unused_variables)]
    pub fn get_parameter_base(&self, veh: Ptr<MSVehicle>, key: &str) -> String {
        // base model has no parameters
        let _ = (veh, key);
        String::new()
    }

    /// C++: `void MSCFModel::setParameter(MSVehicle *, const std::string &, const std::string &) const` [const, virtual]
    /// try to set the given parameter for this carFollowingModel
    // virtual void setParameter(MSVehicle* veh, const std::string& key, const std::string& value) const {
    //         UNUSED_PARAMETER(veh);
    //         UNUSED_PARAMETER(key);
    //         UNUSED_PARAMETER(value);
    //         throw InvalidArgument("Setting parameter '" + key + "' is not supported by carFollowModel");
    //     }
    /// C++: `void MSCFModel::setParameter(...) const` (base body)
    #[allow(unused_variables)]
    pub fn set_parameter_base(&self, veh: Ptr<MSVehicle>, key: &str, value: &str) -> Result<(), InvalidArgument> {
        // C++ throws InvalidArgument for unknown keys
        let _ = (veh, value);
        Err(InvalidArgument(format!("Setting parameter '{}' is not supported by carFollowModel", key)))
    }

    /// C++: `void MSCFModel::applyOwnSpeedPerceptionError(const MSVehicle *const, double &) const` [const, protected]
    /// Overwrites sped by the perceived values obtained from the vehicle's driver state,
    // void
    // MSCFModel::applyOwnSpeedPerceptionError(const MSVehicle* const veh, double& speed) const {
    //     if (!veh->hasDriverState()) {
    //         return;
    //     }
    //     speed = veh->getDriverState()->getPerceivedOwnSpeed(speed);
    // }
    #[allow(unused_variables)]
    pub fn apply_own_speed_perception_error(&self, veh: Ptr<MSVehicle>, speed: &mut f64) {
        if !veh.has_driver_state() {
            return;
        }
        // C++ getDriverState() hands out a raw pointer, Rc only gives &: mutate through the same object
        let ds = veh.get_driver_state();
        let ds_mut = unsafe { &mut *(Rc::as_ptr(&ds) as *mut MSSimpleDriverState) };
        *speed = ds_mut.get_perceived_own_speed(*speed);
    }

    /// C++: `void MSCFModel::applyHeadwayAndSpeedDifferencePerceptionErrors(const MSVehicle *const, double, double &, double &, double, const MSVehicle *const) const` [const, protected]
    /// Overwrites gap2pred and predSpeed by the perceived values obtained from the vehicle's driver state,
    // void
    // MSCFModel::applyHeadwayAndSpeedDifferencePerceptionErrors(const MSVehicle* const veh, double speed, double& gap, double& predSpeed, double predMaxDecel, const MSVehicle* const pred) const {
    //     UNUSED_PARAMETER(speed);
    //     UNUSED_PARAMETER(predMaxDecel);
    //     if (!veh->hasDriverState()) {
    //         return;
    //     }
    //
    //     // Obtain perceived gap and headway from the driver state
    //     const double perceivedGap = veh->getDriverState()->getPerceivedHeadway(gap, pred);
    //     const double perceivedSpeedDifference = veh->getDriverState()->getPerceivedSpeedDifference(predSpeed - speed, gap, pred);
    //
    // #ifdef DEBUG_DRIVER_ERRORS
    //     if (DEBUG_COND) {
    //         if (!veh->getDriverState()->debugLocked()) {
    //             veh->getDriverState()->lockDebug();
    //             std::cout << SIMTIME << " veh '" << veh->getID() << "' -> MSCFModel_Krauss::applyHeadwayAndSpeedDifferencePerceptionErrors()\n"
    //                       << "  speed=" << speed << " gap=" << gap << " leaderSpeed=" << predSpeed
    //                       << "\n  perceivedGap=" << perceivedGap << " perceivedLeaderSpeed=" << speed + perceivedSpeedDifference
    //                       << " perceivedSpeedDifference=" << perceivedSpeedDifference
    //                       << std::endl;
    //             const double exactFollowSpeed = followSpeed(veh, speed, gap, predSpeed, predMaxDecel, pred, CalcReason::FUTURE);
    //             const double errorFollowSpeed = followSpeed(veh, speed, perceivedGap, speed + perceivedSpeedDifference, predMaxDecel, pred, CalcReason::FUTURE);
    //             const double accelError = SPEED2ACCEL(errorFollowSpeed - exactFollowSpeed);
    //             std::cout << "  gapError=" << perceivedGap - gap << "  dvError=" << perceivedSpeedDifference - (predSpeed - speed)
    //                       << "\n  resulting accelError: " << accelError << std::endl;
    //             veh->getDriverState()->unlockDebug();
    //         }
    //     }
    // #endif
    //
    //     gap = perceivedGap;
    //     predSpeed = speed + perceivedSpeedDifference;
    // }
    #[allow(unused_variables)]
    pub fn apply_headway_and_speed_difference_perception_errors(&self, veh: Ptr<MSVehicle>, speed: f64, gap: &mut f64, predSpeed: &mut f64, predMaxDecel: f64, pred: Ptr<MSVehicle>) {
        if !veh.has_driver_state() {
            return;
        }
        // Obtain perceived gap and headway from the driver state
        // C++ getDriverState() hands out a raw pointer, Rc only gives &: mutate through the same object
        let ds = veh.get_driver_state();
        let ds_mut = unsafe { &mut *(Rc::as_ptr(&ds) as *mut MSSimpleDriverState) };
        let perceivedGap = ds_mut.get_perceived_headway(*gap, Ptr::from_raw(pred.as_raw() as *mut ()));
        let perceivedSpeedDifference = ds_mut.get_perceived_speed_difference(*predSpeed - speed, *gap, Ptr::from_raw(pred.as_raw() as *mut ()));
        // DEBUG_DRIVER_ERRORS block is compiled out in MSCFModel.cpp (its #define is commented out), so nothing runs here
        *gap = perceivedGap;
        *predSpeed = speed + perceivedSpeedDifference;
    }

    /// C++: `void MSCFModel::applyHeadwayPerceptionError(const MSVehicle *const, double, double &) const` [const, protected]
    /// Overwrites gap by the perceived value obtained from the vehicle's driver state
    // void
    // MSCFModel::applyHeadwayPerceptionError(const MSVehicle* const veh, double speed, double& gap) const {
    //     UNUSED_PARAMETER(speed);
    //     if (!veh->hasDriverState()) {
    //         return;
    //     }
    //     // @todo: Provide objectID (e.g. pointer address for the relevant object at the given distance(gap))
    //     //        This is for item related management of known object and perception updates when the distance
    //     //        changes significantly. (Should not be too important for stationary objects though.)
    //
    //     // Obtain perceived gap from driver state
    //     const double perceivedGap = veh->getDriverState()->getPerceivedHeadway(gap);
    //
    // #ifdef DEBUG_DRIVER_ERRORS
    //     if (DEBUG_COND) {
    //         if (!veh->getDriverState()->debugLocked()) {
    //             veh->getDriverState()->lockDebug();
    //             std::cout << SIMTIME << " veh '" << veh->getID() << "' -> MSCFModel_Krauss::applyHeadwayPerceptionError()\n"
    //                       << "  speed=" << speed << " gap=" << gap << "\n  perceivedGap=" << perceivedGap << std::endl;
    //             const double exactStopSpeed = stopSpeed(veh, speed, gap, CalcReason::FUTURE);
    //             const double errorStopSpeed = stopSpeed(veh, speed, perceivedGap, CalcReason::FUTURE);
    //             const double accelError = SPEED2ACCEL(errorStopSpeed - exactStopSpeed);
    //             std::cout << "  gapError=" << perceivedGap - gap << "\n  resulting accelError: " << accelError << std::endl;
    //             veh->getDriverState()->unlockDebug();
    //         }
    //     }
    // #endif
    //
    //     gap = perceivedGap;
    // }
    #[allow(unused_variables)]
    pub fn apply_headway_perception_error(&self, veh: Ptr<MSVehicle>, speed: f64, gap: &mut f64) {
        if !veh.has_driver_state() {
            return;
        }
        // @todo: Provide objectID (e.g. pointer address for the relevant object at the given distance(gap))
        // Obtain perceived gap from driver state
        // C++ getDriverState() hands out a raw pointer, Rc only gives &: mutate through the same object
        let ds = veh.get_driver_state();
        let ds_mut = unsafe { &mut *(Rc::as_ptr(&ds) as *mut MSSimpleDriverState) };
        let perceivedGap = ds_mut.get_perceived_headway(*gap, Ptr::default());
        // DEBUG_DRIVER_ERRORS block is compiled out in MSCFModel.cpp (its #define is commented out), so nothing runs here
        *gap = perceivedGap;
    }

}

/// C++ class `MSCFModel::VehicleVariables` (microsim/cfmodels/MSCFModel.h)
#[repr(C)]
#[derive(Clone)]
pub struct MSCFModel_VehicleVariables {
}

impl MSCFModel_VehicleVariables {
    /// C++: `void VehicleVariables::~VehicleVariables()` [virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is trivial: no owned resources to release
    }

    /// C++: `void VehicleVariables::saveState(OutputDevice &, const MSCFModel &) const` [const, virtual]
    /// Saves the vehicle variables
    // void
    // MSCFModel::VehicleVariables::saveState(OutputDevice& /* out */, const MSCFModel& cfm) const {
    //     WRITE_WARNINGF(TL("carFollowModel of vType '%' cannot save state"), cfm.myType->getID());
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice, cfm: &MSCFModel) {
        cfm.save_vehicle_variables(self, out);
    }

    /// C++: `void VehicleVariables::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the vehicle variables from the given description
    // void
    // MSCFModel::VehicleVariables::loadState(const SUMOSAXAttributes& /* attrs */) {
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) {
        // C++ loadState is empty for the base model
        let _ = attrs;
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpolated_value_extrapolates_and_interpolates() {
        // C++ LinearApproxHelpers::getInterpolatedValue on {0:0, 10:20}
        let map: LinearApproxMap = vec![(0., 0.), (10., 20.)];
        assert_eq!(get_interpolated_value(&map, 5.), 10.);
        assert_eq!(get_interpolated_value(&map, 20.), 20.); // above the last key: last value
        assert_eq!(get_interpolated_value(&map, -1.), 0.); // below the first key: first value
    }

    #[test]
    fn wired_models_report_their_own_id_through_base_pointer() {
        use crate::microsim::cfmodels::mscf_model_acc::MSCFModel_ACC;
        use crate::microsim::cfmodels::mscf_model_cacc::MSCFModel_CACC;
        use crate::microsim::cfmodels::mscf_model_cc::MSCFModel_CC;
        use crate::microsim::cfmodels::mscf_model_daniel1::MSCFModel_Daniel1;
        use crate::microsim::cfmodels::mscf_model_eidm::MSCFModel_EIDM;
        use crate::microsim::cfmodels::mscf_model_idm::MSCFModel_IDM;
        use crate::microsim::cfmodels::mscf_model_kerner::MSCFModel_Kerner;
        use crate::microsim::cfmodels::mscf_model_na_sch::MSCFModel_NaSch;
        use crate::microsim::cfmodels::mscf_model_p_wag2009::MSCFModel_PWag2009;
        use crate::microsim::cfmodels::mscf_model_rail::MSCFModel_Rail;
        use crate::microsim::cfmodels::mscf_model_smart_sk::MSCFModel_SmartSK;
        use crate::microsim::cfmodels::mscf_model_w99::MSCFModel_W99;
        use crate::microsim::cfmodels::mscf_model_wiedemann::MSCFModel_Wiedemann;
        let mut vtp = SUMOVTypeParameter::new("passenger", utils::common::sumo_vehicle_class::SVC_PASSENGER);
        vtp.cfParameter.insert(SUMO_ATTR_CF_CC_LANES_COUNT, "2".to_string());
        let vtype = Ptr::from_mut(Box::leak(Box::new(MSVehicleType::new(&vtp))));
        let cases: Vec<(Ptr<MSCFModel>, i32)> = vec![
            (Ptr::from_mut(Box::leak(Box::new(MSCFModel_IDM::new(vtype, false)))).cast::<MSCFModel>(), SUMO_TAG_CF_IDM as i32),
            (Ptr::from_mut(Box::leak(Box::new(MSCFModel_EIDM::new(vtype)))).cast::<MSCFModel>(), SUMO_TAG_CF_EIDM as i32),
            (Ptr::from_mut(Box::leak(Box::new(MSCFModel_ACC::new(vtype)))).cast::<MSCFModel>(), SUMO_TAG_CF_ACC as i32),
            (Ptr::from_mut(Box::leak(Box::new(MSCFModel_CACC::new(vtype)))).cast::<MSCFModel>(), SUMO_TAG_CF_CACC as i32),
            (Ptr::from_mut(Box::leak(Box::new(MSCFModel_CC::new(vtype)))).cast::<MSCFModel>(), SUMO_TAG_CF_CC as i32),
            (Ptr::from_mut(Box::leak(Box::new(MSCFModel_Wiedemann::new(vtype)))).cast::<MSCFModel>(), SUMO_TAG_CF_WIEDEMANN as i32),
            (Ptr::from_mut(Box::leak(Box::new(MSCFModel_W99::new(vtype)))).cast::<MSCFModel>(), SUMO_TAG_CF_W99 as i32),
            (Ptr::from_mut(Box::leak(Box::new(MSCFModel_Kerner::new(vtype)))).cast::<MSCFModel>(), SUMO_TAG_CF_BKERNER as i32),
            (Ptr::from_mut(Box::leak(Box::new(MSCFModel_NaSch::new(vtype, 0.5)))).cast::<MSCFModel>(), SUMO_TAG_CF_NASCH as i32),
            (Ptr::from_mut(Box::leak(Box::new(MSCFModel_PWag2009::new(vtype)))).cast::<MSCFModel>(), SUMO_TAG_CF_PWAGNER2009 as i32),
            (Ptr::from_mut(Box::leak(Box::new(MSCFModel_SmartSK::new(vtype)))).cast::<MSCFModel>(), SUMO_TAG_CF_SMART_SK as i32),
            (Ptr::from_mut(Box::leak(Box::new(MSCFModel_Rail::new(vtype)))).cast::<MSCFModel>(), SUMO_TAG_CF_RAIL as i32),
            (Ptr::from_mut(Box::leak(Box::new(MSCFModel_Daniel1::new(vtype)))).cast::<MSCFModel>(), SUMO_TAG_CF_DANIEL1 as i32),
        ];
        for (cf, id) in cases {
            assert_eq!(cf.m().get_model_id(), id);
        }
    }

    #[test]
    fn brake_gap_euler_matches_c_formula() {
        // TS = 1 s (default DELTA_T): speedReduction = 4.5, steps = int(10/4.5) = 2
        // SPEED2DIST(2*10 - 4.5*2*3/2) + 10*1.0 = 6.5 + 10 = 16.5
        assert!((MSCFModel::brake_gap_euler(10., 4.5, 1.) - 16.5).abs() < 1e-9);
    }

    #[test]
    fn arrival_helpers_match_c_formulas() {
        // zero acceleration: dist / speed
        assert!((MSCFModel::estimate_arrival_time(100., 10., 20., 0.) - 10.).abs() < 1e-9);
        // accelerating from zero: avoidArrivalAccel(10, 2, 0, 4.5) = 2 * (10/2 - 0) / 2
        assert!((MSCFModel::avoid_arrival_accel(10., 2., 0., 4.5) - 5.).abs() < 1e-9);
    }

    #[test]
    fn passing_time_euler_inside_step() {
        // euler: t = (5 - 0) / 10 = 0.5, clamped to [0, TS]
        assert!((MSCFModel::passing_time(0., 5., 10., 10., 10.) - 0.5).abs() < 1e-9);
    }
}
