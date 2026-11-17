//! Port of microsim/cfmodels/MSCFModel_CACC.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use super::mscf_model::*;
use super::mscf_model_acc::*;
use crate::microsim::ms_globals::MSGlobals;
use utils::common::time::{delta_t, time2steps, steps2time, accel2speed, speed2accel, speed2dist, dist2speed, ts};
use utils::common::to_string::f64_to_string_prec;
use utils::geom::NUMERICAL_EPS;

fn cacc_free_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, seen: f64, maxSpeed: f64, onInsertion: bool, usage: MSCFModel_CalcReason) -> f64 { cf_cast::<MSCFModel_CACC>(m).free_speed(veh, speed, seen, maxSpeed, onInsertion, usage) }
fn cacc_follow_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 { cf_cast::<MSCFModel_CACC>(m).follow_speed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred, usage) }
fn cacc_insertion_follow_speed(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>) -> f64 { cf_cast::<MSCFModel_CACC>(m).insertion_follow_speed(veh, speed, gap2pred, predSpeed, predMaxDecel, pred) }
fn cacc_stop_speed_2(m: &MSCFModel, veh: Ptr<MSVehicle>, speed: f64, gap: f64, decel: f64, usage: MSCFModel_CalcReason) -> f64 { cf_cast::<MSCFModel_CACC>(m).stop_speed(veh, speed, gap, decel, usage) }
fn cacc_interaction_gap(m: &MSCFModel, veh: Ptr<MSVehicle>, vL: f64) -> f64 { cf_cast::<MSCFModel_CACC>(m).interaction_gap(veh, vL) }
fn cacc_get_secure_gap(m: &MSCFModel, veh: Ptr<MSVehicle>, pred: Ptr<MSVehicle>, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64) -> f64 { cf_cast::<MSCFModel_CACC>(m).get_secure_gap(veh, pred, speed, leaderSpeed, leaderMaxDecel) }
fn cacc_maximum_lane_speed_cf(m: &MSCFModel, veh: Ptr<MSVehicle>, maxSpeed: f64, maxSpeedLane: f64) -> f64 { cf_cast::<MSCFModel_CACC>(m).maximum_lane_speed_cf(veh, maxSpeed, maxSpeedLane) }
fn cacc_set_headway_time(m: &mut MSCFModel, headwayTime: f64) { cf_cast_mut::<MSCFModel_CACC>(m).set_headway_time(headwayTime) }
fn cacc_get_model_id(m: &MSCFModel) -> i32 { cf_cast::<MSCFModel_CACC>(m).get_model_id() }
fn cacc_duplicate(m: &MSCFModel, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> { cf_cast::<MSCFModel_CACC>(m).duplicate(vtype) }
fn cacc_create_vehicle_variables(m: &MSCFModel) -> Ptr<MSCFModel_VehicleVariables> { cf_cast::<MSCFModel_CACC>(m).create_vehicle_variables() }

/// Virtual table of MSCFModel_CACC (C++ subclass of MSCFModel_ACC): its overrides on top of the MSCFModel default table
pub const CACC_VT: MSCFModelVt = MSCFModelVt {
    free_speed: cacc_free_speed,
    follow_speed: cacc_follow_speed,
    insertion_follow_speed: cacc_insertion_follow_speed,
    stop_speed_2: cacc_stop_speed_2,
    interaction_gap: cacc_interaction_gap,
    get_secure_gap: cacc_get_secure_gap,
    maximum_lane_speed_cf: cacc_maximum_lane_speed_cf,
    set_headway_time: cacc_set_headway_time,
    get_model_id: cacc_get_model_id,
    duplicate: cacc_duplicate,
    create_vehicle_variables: cacc_create_vehicle_variables,
    ..DEFAULT_MSCF_MODEL_VT
};

// C++ #defines in MSCFModel_CACC.cpp
const DEFAULT_SC_GAIN_CACC: f64 = -0.4;
const DEFAULT_GCC_GAIN_GAP_CACC: f64 = 0.005;
const DEFAULT_GCC_GAIN_GAP_DOT_CACC: f64 = 0.05;
const DEFAULT_GC_GAIN_GAP_CACC: f64 = 0.45;
const DEFAULT_GC_GAIN_GAP_DOT_CACC: f64 = 0.0125;
const DEFAULT_CA_GAIN_GAP_CACC: f64 = 0.45;
const DEFAULT_CA_GAIN_GAP_DOT_CACC: f64 = 0.05;
const DEFAULT_HEADWAYTIME_ACC: f64 = 1.0;
const DEFAULT_SC_MIN_GAP: f64 = 1.66;
const DEFAULT_EMERGENCY_OVERRIDE_THRESHOLD: f64 = 2.0;

/// C++ `MSCFModel_CACC::CommunicationsOverrideModeMap[value]` (std::map operator[]: a missing key yields CACC_NO_OVERRIDE)
fn comm_override_mode_from_str(value: &str) -> MSCFModel_CACC_CommunicationsOverrideMode {
    match value {
        "0" => MSCFModel_CACC_CommunicationsOverrideMode::CACC_NO_OVERRIDE,
        "1" => MSCFModel_CACC_CommunicationsOverrideMode::CACC_MODE_NO_LEADER,
        "2" => MSCFModel_CACC_CommunicationsOverrideMode::CACC_MODE_LEADER_NO_CAV,
        "3" => MSCFModel_CACC_CommunicationsOverrideMode::CACC_MODE_LEADER_CAV,
        _ => MSCFModel_CACC_CommunicationsOverrideMode::CACC_NO_OVERRIDE,
    }
}

/// C++ `(CommunicationsOverrideMode)mode` for an int read from a saved state
fn comm_override_mode_from_i64(value: i64) -> MSCFModel_CACC_CommunicationsOverrideMode {
    // shortcut: C++ casts any int to the enum; unknown values fall back to CACC_NO_OVERRIDE here, since a Rust enum cannot hold them
    match value {
        1 => MSCFModel_CACC_CommunicationsOverrideMode::CACC_MODE_NO_LEADER,
        2 => MSCFModel_CACC_CommunicationsOverrideMode::CACC_MODE_LEADER_NO_CAV,
        3 => MSCFModel_CACC_CommunicationsOverrideMode::CACC_MODE_LEADER_CAV,
        _ => MSCFModel_CACC_CommunicationsOverrideMode::CACC_NO_OVERRIDE,
    }
}

/// C++ `MSCFModel_CACC::VehicleModeNames[mode]`
fn vehicle_mode_name(mode: MSCFModel_CACC_VehicleMode) -> &'static str {
    match mode {
        MSCFModel_CACC_VehicleMode::CC_MODE => "CC",
        MSCFModel_CACC_VehicleMode::ACC_MODE => "ACC",
        MSCFModel_CACC_VehicleMode::CACC_GAP_CLOSING_MODE => "CACC_GAP_CL",
        MSCFModel_CACC_VehicleMode::CACC_GAP_MODE => "CACC_GAP",
        MSCFModel_CACC_VehicleMode::CACC_COLLISION_AVOIDANCE_MODE => "CACC_CA",
    }
}

/// C++ `const_cast<SUMOVehicleParameter&>(veh->getParameter()).setParameter("caccVehicleMode", name)`
fn set_cacc_vehicle_mode(veh: Ptr<MSVehicle>, mode_name: &str) {
    veh.myParameter.m().set_parameter("caccVehicleMode", mode_name);
}

/// C++ enum `MSCFModel_CACC::CommunicationsOverrideMode`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSCFModel_CACC_CommunicationsOverrideMode {
    CACC_NO_OVERRIDE = 0,
    CACC_MODE_NO_LEADER = 1,
    CACC_MODE_LEADER_NO_CAV = 2,
    CACC_MODE_LEADER_CAV = 3,
}

/// C++ enum `MSCFModel_CACC::VehicleMode`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum MSCFModel_CACC_VehicleMode {
    CC_MODE = 0,
    ACC_MODE = 1,
    CACC_GAP_MODE = 2,
    CACC_GAP_CLOSING_MODE = 3,
    CACC_COLLISION_AVOIDANCE_MODE = 4,
}

/// C++ class `MSCFModel_CACC::CACCVehicleVariables` (microsim/cfmodels/MSCFModel_CACC.h)
#[repr(C)]
pub struct MSCFModel_CACC_CACCVehicleVariables {
    /// C++ base class `MSCFModel_ACC::ACCVehicleVariables`
    pub base: MSCFModel_ACC_ACCVehicleVariables,
    /// The vehicle's CACC precious time step gap error
    pub CACC_ControlMode: i32,
    pub CACC_CommunicationsOverrideMode: MSCFModel_CACC_CommunicationsOverrideMode,
}

impl std::ops::Deref for MSCFModel_CACC_CACCVehicleVariables {
    type Target = MSCFModel_ACC_ACCVehicleVariables;
    fn deref(&self) -> &MSCFModel_ACC_ACCVehicleVariables {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_CACC_CACCVehicleVariables {
    fn deref_mut(&mut self) -> &mut MSCFModel_ACC_ACCVehicleVariables {
        &mut self.base
    }
}

impl MSCFModel_CACC_CACCVehicleVariables {
    /// C++: `void CACCVehicleVariables::CACCVehicleVariables()`
    // CACCVehicleVariables() : CACC_ControlMode(0), CACC_CommunicationsOverrideMode(CACC_NO_OVERRIDE) {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSCFModel_CACC_CACCVehicleVariables {
            base: MSCFModel_ACC_ACCVehicleVariables::new(),
            CACC_ControlMode: 0,
            CACC_CommunicationsOverrideMode: MSCFModel_CACC_CommunicationsOverrideMode::CACC_NO_OVERRIDE,
        }
    }

    /// C++: `void CACCVehicleVariables::saveState(OutputDevice &, const MSCFModel &) const` [const, virtual]
    /// Saves the vehicle variables
    // void
    // MSCFModel_CACC::CACCVehicleVariables::saveState(OutputDevice& out, const MSCFModel& /*cfm*/) const {
    //     out.openTag(SUMO_TAG_CFM_VARIABLES);
    //     out.writeAttr(SUMO_ATTR_ID, "CACC");
    //     std::ostringstream internals;
    //     internals << CACC_ControlMode << " ";
    //     internals << CACC_CommunicationsOverrideMode;
    //     out.writeAttr(SUMO_ATTR_STATE, internals.str());
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&self, out: &mut OutputDevice, cfm: &MSCFModel) {
        out.open_tag_id(SUMO_TAG_CFM_VARIABLES);
        out.write_attr(SUMO_ATTR_ID, "CACC");
        let internals = format!("{} {}", self.CACC_ControlMode, self.CACC_CommunicationsOverrideMode as i64);
        out.write_attr(SUMO_ATTR_STATE, internals.as_str());
        out.close_tag("");
        let _ = cfm;
    }

    /// C++: `void CACCVehicleVariables::loadState(const SUMOSAXAttributes &)` [virtual]
    /// Loads the state of the vehicle variables from the given description
    // void
    // MSCFModel_CACC::CACCVehicleVariables::loadState(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string cfmID = attrs.get<std::string>(SUMO_ATTR_ID, nullptr, ok);
    //     if (cfmID != "CACC") {
    //         throw ProcessError(TLF("incompatible carFollowModel '%' when loading state for CACC", cfmID));
    //     }
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> CACC_ControlMode;
    //     int mode;
    //     bis >> mode;
    //     CACC_CommunicationsOverrideMode = (CommunicationsOverrideMode)mode;
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes) -> Result<(), utils::common::util_exceptions::ProcessError> {
        let mut ok = true;
        let cfmID: String = attrs.get(SUMO_ATTR_ID, "", &mut ok, true);
        if cfmID != "CACC" {
            return Err(utils::common::util_exceptions::ProcessError::new(format!("incompatible carFollowModel '{}' when loading state for CACC", cfmID)));
        }
        let state = attrs.get_string(SUMO_ATTR_STATE);
        let mut bis = state.split_whitespace();
        if let Some(v) = bis.next().and_then(|s| s.parse::<i32>().ok()) {
            self.CACC_ControlMode = v;
        }
        if let Some(v) = bis.next().and_then(|s| s.parse::<i64>().ok()) {
            self.CACC_CommunicationsOverrideMode = comm_override_mode_from_i64(v);
        }
        Ok(())
    }

}

/// Virtual table of MSCFModel_CACC: its overrides on top of the MSCFModel table
/// The CACC car-following model
/// C++ class `MSCFModel_CACC` (microsim/cfmodels/MSCFModel_CACC.h)
#[repr(C)]
pub struct MSCFModel_CACC {
    /// C++ base class `MSCFModel`
    pub base: MSCFModel,
    pub acc_CFM: MSCFModel_ACC,
    pub mySpeedControlGain: f64,
    pub myGapClosingControlGainGap: f64,
    pub myGapClosingControlGainGapDot: f64,
    pub myGapControlGainGap: f64,
    pub myGapControlGainGapDot: f64,
    pub myCollisionAvoidanceGainGap: f64,
    pub myCollisionAvoidanceGainGapDot: f64,
    pub myHeadwayTimeACC: f64,
    pub myApplyDriverstate: f64,
    pub myEmergencyThreshold: f64,
    pub mySpeedControlMinGap: f64,
}

impl std::ops::Deref for MSCFModel_CACC {
    type Target = MSCFModel;
    fn deref(&self) -> &MSCFModel {
        &self.base
    }
}
impl std::ops::DerefMut for MSCFModel_CACC {
    fn deref_mut(&mut self) -> &mut MSCFModel {
        &mut self.base
    }
}

// C++ static members of MSCFModel_CACC (port as module-level statics, see AGENTS.md):
//   static std::map<std::string, CommunicationsOverrideMode> MSCFModel_CACC::CommunicationsOverrideModeMap;
//   static std::map<VehicleMode, std::string> MSCFModel_CACC::VehicleModeNames;

impl MSCFModel_CACC {
    /// C++: `void MSCFModel_CACC::MSCFModel_CACC(const MSVehicleType *)`
    /// Constructor
    // MSCFModel_CACC::MSCFModel_CACC(const MSVehicleType* vtype) :
    //     MSCFModel(vtype), acc_CFM(MSCFModel_ACC(vtype)),
    //     mySpeedControlGain(vtype->getParameter().getCFParam(SUMO_ATTR_SC_GAIN_CACC, DEFAULT_SC_GAIN_CACC)),
    //     myGapClosingControlGainGap(vtype->getParameter().getCFParam(SUMO_ATTR_GCC_GAIN_GAP_CACC, DEFAULT_GCC_GAIN_GAP_CACC)),
    //     myGapClosingControlGainGapDot(vtype->getParameter().getCFParam(SUMO_ATTR_GCC_GAIN_GAP_DOT_CACC, DEFAULT_GCC_GAIN_GAP_DOT_CACC)),
    //     myGapControlGainGap(vtype->getParameter().getCFParam(SUMO_ATTR_GC_GAIN_GAP_CACC, DEFAULT_GC_GAIN_GAP_CACC)),
    //     myGapControlGainGapDot(vtype->getParameter().getCFParam(SUMO_ATTR_GC_GAIN_GAP_DOT_CACC, DEFAULT_GC_GAIN_GAP_DOT_CACC)),
    //     myCollisionAvoidanceGainGap(vtype->getParameter().getCFParam(SUMO_ATTR_CA_GAIN_GAP_CACC, DEFAULT_CA_GAIN_GAP_CACC)),
    //     myCollisionAvoidanceGainGapDot(vtype->getParameter().getCFParam(SUMO_ATTR_CA_GAIN_GAP_DOT_CACC, DEFAULT_CA_GAIN_GAP_DOT_CACC)),
    //     myHeadwayTimeACC(vtype->getParameter().getCFParam(SUMO_ATTR_HEADWAY_TIME_CACC_TO_ACC, DEFAULT_HEADWAYTIME_ACC)),
    //     myApplyDriverstate(vtype->getParameter().getCFParam(SUMO_ATTR_APPLYDRIVERSTATE, 0)),
    //     myEmergencyThreshold(vtype->getParameter().getCFParam(SUMO_ATTR_CA_OVERRIDE, DEFAULT_EMERGENCY_OVERRIDE_THRESHOLD)),
    //     mySpeedControlMinGap(vtype->getParameter().getCFParam(SUMO_ATTR_SC_MIN_GAP, DEFAULT_SC_MIN_GAP)) {
    //     myCollisionMinGapFactor = vtype->getParameter().getCFParam(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
    //     acc_CFM.setHeadwayTime(myHeadwayTimeACC);
    // }
    #[allow(unused_variables)]
    pub fn new(vtype: Ptr<MSVehicleType>) -> Self {
        let p = vtype.get_parameter();
        let mut base = MSCFModel::new(vtype);
        base.vt = &CACC_VT;
        let acc_CFM = MSCFModel_ACC::new(vtype);
        let mySpeedControlGain = p.get_cf_param(SUMO_ATTR_SC_GAIN_CACC, DEFAULT_SC_GAIN_CACC);
        let myGapClosingControlGainGap = p.get_cf_param(SUMO_ATTR_GCC_GAIN_GAP_CACC, DEFAULT_GCC_GAIN_GAP_CACC);
        let myGapClosingControlGainGapDot = p.get_cf_param(SUMO_ATTR_GCC_GAIN_GAP_DOT_CACC, DEFAULT_GCC_GAIN_GAP_DOT_CACC);
        let myGapControlGainGap = p.get_cf_param(SUMO_ATTR_GC_GAIN_GAP_CACC, DEFAULT_GC_GAIN_GAP_CACC);
        let myGapControlGainGapDot = p.get_cf_param(SUMO_ATTR_GC_GAIN_GAP_DOT_CACC, DEFAULT_GC_GAIN_GAP_DOT_CACC);
        let myCollisionAvoidanceGainGap = p.get_cf_param(SUMO_ATTR_CA_GAIN_GAP_CACC, DEFAULT_CA_GAIN_GAP_CACC);
        let myCollisionAvoidanceGainGapDot = p.get_cf_param(SUMO_ATTR_CA_GAIN_GAP_DOT_CACC, DEFAULT_CA_GAIN_GAP_DOT_CACC);
        let myHeadwayTimeACC = p.get_cf_param(SUMO_ATTR_HEADWAY_TIME_CACC_TO_ACC, DEFAULT_HEADWAYTIME_ACC);
        let myApplyDriverstate = p.get_cf_param(SUMO_ATTR_APPLYDRIVERSTATE, 0.);
        let myEmergencyThreshold = p.get_cf_param(SUMO_ATTR_CA_OVERRIDE, DEFAULT_EMERGENCY_OVERRIDE_THRESHOLD);
        let mySpeedControlMinGap = p.get_cf_param(SUMO_ATTR_SC_MIN_GAP, DEFAULT_SC_MIN_GAP);
        base.myCollisionMinGapFactor = p.get_cf_param(SUMO_ATTR_COLLISION_MINGAP_FACTOR, 0.1);
        let mut cfm = MSCFModel_CACC {
            base,
            acc_CFM,
            mySpeedControlGain,
            myGapClosingControlGainGap,
            myGapClosingControlGainGapDot,
            myGapControlGainGap,
            myGapControlGainGapDot,
            myCollisionAvoidanceGainGap,
            myCollisionAvoidanceGainGapDot,
            myHeadwayTimeACC,
            myApplyDriverstate,
            myEmergencyThreshold,
            mySpeedControlMinGap,
        };
        cfm.acc_CFM.set_headway_time(myHeadwayTimeACC);
        cfm.base.vt = &CACC_VT;
        cfm
    }

    /// C++: `void MSCFModel_CACC::~MSCFModel_CACC() noexcept` [virtual]
    /// Destructor
    // MSCFModel_CACC::~MSCFModel_CACC() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor is trivial: no owned resources to release
    }

    /// C++: `double MSCFModel_CACC::freeSpeed(const MSVehicle *const, double, double, double, const bool, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed without a leader
    // double
    // MSCFModel_CACC::freeSpeed(const MSVehicle* const veh, double speed, double seen, double maxSpeed, const bool onInsertion, const CalcReason usage) const {
    //     if (!MSGlobals::gComputeLC && usage == CalcReason::CURRENT) {
    //         CACCVehicleVariables* vars = (CACCVehicleVariables*)veh->getCarFollowVariables();
    //         if (vars->lastUpdateTime != MSNet::getInstance()->getCurrentTimeStep()) {
    //             // _v was not called in this step
    //             const_cast<SUMOVehicleParameter&>(veh->getParameter()).setParameter("caccVehicleMode", VehicleModeNames[CC_MODE]);
    //         }
    //     }
    //     return MSCFModel::freeSpeed(veh, speed, seen, maxSpeed, onInsertion, usage);
    // }
    #[allow(unused_variables)]
    pub fn free_speed(&self, veh: Ptr<MSVehicle>, speed: f64, seen: f64, maxSpeed: f64, onInsertion: bool, usage: MSCFModel_CalcReason) -> f64 {
        if !MSGlobals::compute_lc() && usage == MSCFModel_CalcReason::CURRENT {
            let vars = veh.get_car_follow_variables().cast::<MSCFModel_CACC_CACCVehicleVariables>();
            if vars.lastUpdateTime != MSNet::get_instance().get_current_time_step() {
                // _v was not called in this step
                set_cacc_vehicle_mode(veh, vehicle_mode_name(MSCFModel_CACC_VehicleMode::CC_MODE));
            }
        }
        self.base.free_speed_base(veh, speed, seen, maxSpeed, onInsertion, usage)
    }

    /// C++: `double MSCFModel_CACC::followSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed (no dawdling)
    // double
    // MSCFModel_CACC::followSpeed(const MSVehicle* const veh, double speed, double gap2pred, double predSpeed, double predMaxDecel, const MSVehicle* const pred, const CalcReason usage) const {
    //     if (myApplyDriverstate) {
    //         applyHeadwayAndSpeedDifferencePerceptionErrors(veh, speed, gap2pred, predSpeed, predMaxDecel, pred);
    //     }
    //
    //     const double desSpeed = veh->getLane()->getVehicleMaxSpeed(veh);
    //     const double vCACC = _v(veh, pred, gap2pred, speed, predSpeed, desSpeed, true, usage);
    //     // using onInsertion=true disables emergencyOverides emergency deceleration smoothing
    //     const double vSafe = maximumSafeFollowSpeed(gap2pred, speed, predSpeed, predMaxDecel, true);
    //
    // #if DEBUG_CACC == 1
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " veh=" << veh->getID() << " pred=" << Named::getIDSecure(pred)
    //                   << " v=" << speed << " vL=" << predSpeed << " gap=" << gap2pred
    //                   << " predDecel=" << predMaxDecel << " vCACC=" << vCACC << " vSafe=" << vSafe << "\n";
    //     }
    // #else
    //     UNUSED_PARAMETER(pred);
    // #endif
    //     const double speedOverride = MIN2(myEmergencyThreshold, gap2pred);
    //     if (vSafe + speedOverride < vCACC) {
    // #if DEBUG_CACC == 1
    //         if (DEBUG_COND) {
    //             std::cout << "Apply Safe speed, override=" << speedOverride << "\n";
    //         }
    // #endif
    //         return MAX2(0.0, vSafe + speedOverride);
    //     }
    //     return vCACC;
    // }
    #[allow(unused_variables)]
    pub fn follow_speed(&self, veh: Ptr<MSVehicle>, speed: f64, gap2pred: f64, predSpeed: f64, predMaxDecel: f64, pred: Ptr<MSVehicle>, usage: MSCFModel_CalcReason) -> f64 {
        let mut gap2pred = gap2pred;
        let mut predSpeed = predSpeed;
        if self.myApplyDriverstate != 0. {
            self.apply_headway_and_speed_difference_perception_errors(veh, speed, &mut gap2pred, &mut predSpeed, predMaxDecel, pred);
        }

        let desSpeed = veh.get_lane().get_vehicle_max_speed(traffic_object(veh));
        let vCACC = self.v(veh, pred, gap2pred, speed, predSpeed, desSpeed, true, usage);
        // using onInsertion=true disables emergencyOverides emergency deceleration smoothing
        let vSafe = self.maximum_safe_follow_speed(gap2pred, speed, predSpeed, predMaxDecel, true);

        let speedOverride = min2(self.myEmergencyThreshold, gap2pred);
        if vSafe + speedOverride < vCACC {
            return max2(0.0, vSafe + speedOverride);
        }
        vCACC
    }

    /// C++: `double MSCFModel_CACC::stopSpeed(const MSVehicle *const, const double, double, double, const CalcReason) const` [const, virtual]
    /// Computes the vehicle's safe speed for approaching a non-moving obstacle (no dawdling)
    // double
    // MSCFModel_CACC::stopSpeed(const MSVehicle* const veh, const double speed, double gap, double decel, const CalcReason /*usage*/) const {
    //     if (myApplyDriverstate) {
    //         applyHeadwayPerceptionError(veh, speed, gap);
    //     }
    //
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

    /// C++: `double MSCFModel_CACC::getSecureGap(const MSVehicle *const, const MSVehicle *const, const double, const double, const double) const` [const, virtual]
    /// Returns the a gap such that the gap mode acceleration of the follower is zero
    // double
    // MSCFModel_CACC::getSecureGap(const MSVehicle* const veh, const MSVehicle* const pred, const double speed, const double leaderSpeed, const double leaderMaxDecel) const {
    //     // Accel in gap mode should vanish:
    //     double desSpacing;
    //     if (pred->getCarFollowModel().getModelID() != SUMO_TAG_CF_CACC) {
    //         //      0 = myGapControlGainSpeed * (leaderSpeed - speed) + myGapControlGainSpace * (g - myHeadwayTime * speed);
    //         // <=>  myGapControlGainSpace * g = - myGapControlGainSpeed * (leaderSpeed - speed) + myGapControlGainSpace * myHeadwayTime * speed;
    //         // <=>  g = - myGapControlGainSpeed * (leaderSpeed - speed) / myGapControlGainSpace + myHeadwayTime * speed;
    //         desSpacing = acc_CFM.myGapControlGainSpeed * (speed - leaderSpeed) / acc_CFM.myGapControlGainSpace + myHeadwayTimeACC * speed; // MSCFModel_ACC::accelGapControl
    //     } else {
    //         desSpacing = myHeadwayTime * speed; // speedGapControl
    //     }
    //     const double desSpacingDefault = MSCFModel::getSecureGap(veh, pred, speed, leaderSpeed, leaderMaxDecel);
    // #if DEBUG_CACC_SECURE_GAP == 1
    //     std::cout << SIMTIME << "MSCFModel_ACC::getSecureGap speed=" << speed << " leaderSpeed=" << leaderSpeed
    //               << " desSpacing=" << desSpacing << " desSpacingDefault=" << desSpacingDefault << "\n";
    // #endif
    //     return MAX2(desSpacing, desSpacingDefault);
    // }
    #[allow(unused_variables)]
    pub fn get_secure_gap(&self, veh: Ptr<MSVehicle>, pred: Ptr<MSVehicle>, speed: f64, leaderSpeed: f64, leaderMaxDecel: f64) -> f64 {
        // Accel in gap mode should vanish:
        let desSpacing;
        if pred.get_car_follow_model().get_model_id() != SUMO_TAG_CF_CACC as i32 {
            //      0 = myGapControlGainSpeed * (leaderSpeed - speed) + myGapControlGainSpace * (g - myHeadwayTime * speed);
            // <=>  myGapControlGainSpace * g = - myGapControlGainSpeed * (leaderSpeed - speed) + myGapControlGainSpace * myHeadwayTime * speed;
            // <=>  g = - myGapControlGainSpeed * (leaderSpeed - speed) / myGapControlGainSpace + myHeadwayTime * speed;
            desSpacing = self.acc_CFM.myGapControlGainSpeed * (speed - leaderSpeed) / self.acc_CFM.myGapControlGainSpace + self.myHeadwayTimeACC * speed; // MSCFModel_ACC::accelGapControl
        } else {
            desSpacing = self.myHeadwayTime * speed; // speedGapControl
        }
        let desSpacingDefault = self.base.get_secure_gap_base(veh, pred, speed, leaderSpeed, leaderMaxDecel);
        max2(desSpacing, desSpacingDefault)
    }

    /// C++: `double MSCFModel_CACC::insertionFollowSpeed(const MSVehicle *const, double, double, double, double, const MSVehicle *const) const` [const, virtual]
    /// Computes the vehicle's acceptable speed at insertion
    // double
    // MSCFModel_CACC::insertionFollowSpeed(const MSVehicle* const veh, double speed, double gap2pred, double predSpeed, double predMaxDecel, const MSVehicle* const pred) const {
    // #if DEBUG_CACC_INSERTION_FOLLOW_SPEED == 1
    //     if (DEBUG_COND) {
    //         std::cout << "MSCFModel_ACC::insertionFollowSpeed(), speed=" << speed << " gap2pred=" << gap2pred << " predSpeed=" << predSpeed << "\n";
    //     }
    // #endif
    //     // iterate to find a stationary value for
    //     //    speed = followSpeed(v, speed, gap2pred, predSpeed, predMaxDecel, nullptr, CalcReason::FUTURE)
    //     const int max_iter = 50;
    //     int n_iter = 0;
    //     const double tol = 0.1;
    //     double damping = 0.8;
    //
    //     double res = speed;
    //     while (n_iter < max_iter) {
    //         // proposed acceleration
    //         const double vCACC = _v(veh, pred, gap2pred, res, predSpeed, speed, true);
    //         const double vSafe = maximumSafeFollowSpeed(gap2pred, res, predSpeed, predMaxDecel, true);
    //         const double a = MIN2(vCACC, vSafe) - res;
    //         res = res + damping * a;
    // #if DEBUG_CACC_INSERTION_FOLLOW_SPEED == 1
    //         if (DEBUG_COND) {
    //             std::cout << "   n_iter=" << n_iter << " vSafe=" << vSafe << " vCACC=" << vCACC << " a=" << a << " damping=" << damping << " res=" << res << std::endl;
    //         }
    // #endif
    //         damping *= 0.9;
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
        let mut damping = 0.8;

        let mut res = speed;
        while n_iter < max_iter {
            // proposed acceleration
            let vCACC = self.v(v, pred, gap2pred, res, predSpeed, speed, true, MSCFModel_CalcReason::CURRENT);
            let vSafe = self.maximum_safe_follow_speed(gap2pred, res, predSpeed, predMaxDecel, true);
            let a = min2(vCACC, vSafe) - res;
            res = res + damping * a;
            damping *= 0.9;
            if a.abs() < tol {
                break;
            } else {
                n_iter += 1;
            }
        }
        res
    }

    /// C++: `double MSCFModel_CACC::interactionGap(const MSVehicle *const, double) const` [const, virtual]
    /// Returns the maximum gap at which an interaction between both vehicles occurs
    // double
    // MSCFModel_CACC::interactionGap(const MSVehicle* const /* veh */, double /* vL */) const {
    //     /*maximum radar range is CACC is enabled*/
    //     return 250;
    // }
    #[allow(unused_variables)]
    pub fn interaction_gap(&self, arg0: Ptr<MSVehicle>, vL: f64) -> f64 {
        /*maximum radar range is CACC is enabled*/
        250.
    }

    /// C++: `void MSCFModel_CACC::setHeadwayTime(double)` [virtual]
    /// Sets a new value for desired headway [s]
    // void setHeadwayTime(double headwayTime) {
    //         myHeadwayTime = headwayTime;
    //         myHeadwayTimeACC = headwayTime;
    //         acc_CFM.setHeadwayTime(headwayTime);
    //     }
    #[allow(unused_variables)]
    pub fn set_headway_time(&mut self, headwayTime: f64) {
        self.myHeadwayTime = headwayTime;
        self.myHeadwayTimeACC = headwayTime;
        self.acc_CFM.set_headway_time(headwayTime);
    }

    /// C++: `std::string MSCFModel_CACC::getParameter(const MSVehicle *, const std::string &) const` [const, virtual]
    /// try to get the given parameter for this carFollowingModel
    // std::string
    // MSCFModel_CACC::getParameter(const MSVehicle* veh, const std::string& key) const {
    //     CACCVehicleVariables* vars = (CACCVehicleVariables*) veh->getCarFollowVariables();
    //
    //     if (key.compare("caccCommunicationsOverrideMode") == 0) {
    //         return toString(vars->CACC_CommunicationsOverrideMode);
    //     }
    //
    //     return "";
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, veh: Ptr<MSVehicle>, key: &str) -> String {
        let vars = veh.get_car_follow_variables().cast::<MSCFModel_CACC_CACCVehicleVariables>();

        if key == "caccCommunicationsOverrideMode" {
            return (vars.CACC_CommunicationsOverrideMode as i64).to_string();
        }

        String::new()
    }

    /// C++: `void MSCFModel_CACC::setParameter(MSVehicle *, const std::string &, const std::string &) const` [const, virtual]
    /// try to set the given parameter for this carFollowingModel
    // void
    // MSCFModel_CACC::setParameter(MSVehicle* veh, const std::string& key, const std::string& value) const {
    //     CACCVehicleVariables* vars = (CACCVehicleVariables*) veh->getCarFollowVariables();
    //
    //     try {
    //         if (key.compare("caccCommunicationsOverrideMode") == 0) {
    //             vars->CACC_CommunicationsOverrideMode = CommunicationsOverrideModeMap[value];
    //         }
    //     } catch (NumberFormatException&) {
    //         throw InvalidArgument("Invalid value '" + value + "' for parameter '" + key + "' for vehicle '" + veh->getID() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&self, veh: Ptr<MSVehicle>, key: &str, value: &str) {
        let vars = veh.get_car_follow_variables().cast::<MSCFModel_CACC_CACCVehicleVariables>();

        // C++ try/catch(NumberFormatException) around the map lookup: the Rust map lookup cannot throw
        if key == "caccCommunicationsOverrideMode" {
            vars.m().CACC_CommunicationsOverrideMode = comm_override_mode_from_str(value);
        }
    }

    /// C++: `int MSCFModel_CACC::getModelID() const` [const, virtual]
    /// Returns the model's name
    // int getModelID() const {
    //         return SUMO_TAG_CF_CACC;
    //     }
    #[allow(unused_variables)]
    pub fn get_model_id(&self) -> i32 {
        SUMO_TAG_CF_CACC as i32
    }

    /// C++: `double MSCFModel_CACC::maximumLaneSpeedCF(const MSVehicle *const, double, double) const` [const, virtual]
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

    /// C++: `MSCFModel * MSCFModel_CACC::duplicate(const MSVehicleType *) const` [const, virtual]
    /// Duplicates the car-following model
    // MSCFModel*
    // MSCFModel_CACC::duplicate(const MSVehicleType* vtype) const {
    //     return new MSCFModel_CACC(vtype);
    // }
    #[allow(unused_variables)]
    pub fn duplicate(&self, vtype: Ptr<MSVehicleType>) -> Ptr<MSCFModel> {
        // shortcut: duplicate() boxes the copy like C++ `new`; the owner must free it once model ownership is ported
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_CACC::new(vtype))) as *mut MSCFModel)
    }

    /// C++: `MSCFModel::VehicleVariables * MSCFModel_CACC::createVehicleVariables() const` [const, virtual]
    // virtual MSCFModel::VehicleVariables* createVehicleVariables() const {
    //         CACCVehicleVariables* ret = new CACCVehicleVariables();
    //         ret->ACC_ControlMode = 0;
    //         ret->CACC_ControlMode = 0;
    //         ret->CACC_CommunicationsOverrideMode = CACC_NO_OVERRIDE;
    //         ret->lastUpdateTime = 0;
    //         return ret;
    //     }
    #[allow(unused_variables)]
    pub fn create_vehicle_variables(&self) -> Ptr<MSCFModel_VehicleVariables> {
        // shortcut: allocated with Box::into_raw like C++ `new`; the owning vehicle must free it once ported
        Ptr::from_raw(Box::into_raw(Box::new(MSCFModel_CACC_CACCVehicleVariables::new())) as *mut MSCFModel_VehicleVariables)
    }

    /// C++: `double MSCFModel_CACC::_v(const MSVehicle *const, const MSVehicle *const, const double, const double, const double, const double, const bool, const CalcReason) const` [const, private]
    // double
    // MSCFModel_CACC::_v(const MSVehicle* const veh, const MSVehicle* const pred, const double gap2pred, const double speed,
    //                    const double predSpeed, const double desSpeed, const bool /* respectMinGap */, const CalcReason usage) const {
    //     double newSpeed = 0.0;
    //     VehicleMode vehMode = CC_MODE;
    //
    // #if DEBUG_CACC == 1
    //     if (DEBUG_COND) {
    //         std::cout << SIMTIME << " MSCFModel_CACC::_v() for veh '" << veh->getID()
    //                   << " gap=" << gap2pred << " speed="  << speed << " predSpeed=" << predSpeed
    //                   << " desSpeed=" << desSpeed << std::endl;
    //     }
    // #endif
    //
    //     /* Velocity error */
    //     double vErr = speed - desSpeed;
    //     bool setControlMode = false;
    //     CACCVehicleVariables* vars = (CACCVehicleVariables*)veh->getCarFollowVariables();
    //     if (vars->lastUpdateTime != MSNet::getInstance()->getCurrentTimeStep()) {
    //         vars->lastUpdateTime = MSNet::getInstance()->getCurrentTimeStep();
    //         setControlMode = true;
    //     }
    //
    //     CommunicationsOverrideMode commMode = vars->CACC_CommunicationsOverrideMode;
    //
    //     if (commMode == CACC_NO_OVERRIDE) { // old CACC logic (rely on time gap from predecessor)
    //         // @note: using (gap2pred + minGap) here increases oscillations but may
    //         // actually be a good idea once the acceleration-spike-problem is fixed
    //         double time_gap = gap2pred / MAX2(NUMERICAL_EPS, speed);
    //         double spacingErr = gap2pred - myHeadwayTime * speed;
    //
    //         if (time_gap > 2 && spacingErr > mySpeedControlMinGap) {
    // #if DEBUG_CACC == 1
    //             if (DEBUG_COND) {
    //                 std::cout << "        applying speedControl" << "  time_gap=" << time_gap << std::endl;
    //             }
    // #endif
    //             // Find acceleration - Speed control law
    //             newSpeed = speedSpeedControl(speed, vErr, vehMode);
    //             // Set cl to vehicle parameters
    //             if (setControlMode) {
    //                 vars->CACC_ControlMode = 0;
    //             }
    //         } else if (time_gap < 1.5) {
    //             // Find acceleration - Gap control law
    // #if DEBUG_CACC == 1
    //             if (DEBUG_COND) {
    //                 std::cout << "        speedGapControl" << "  time_gap=" << time_gap << std::endl;
    //             }
    // #endif
    //
    //             newSpeed = speedGapControl(veh, gap2pred, speed, predSpeed, desSpeed, vErr, pred, vehMode);
    //             // Set cl to vehicle parameters
    //             if (setControlMode) {
    //                 vars->CACC_ControlMode = 1;
    //             }
    //         } else {
    //             // Follow previous applied law
    //             int cm = vars->CACC_ControlMode;
    //             if (!cm) {
    //                 // CACC_ControlMode = speed control
    //
    // #if DEBUG_CACC == 1
    //                 if (DEBUG_COND) {
    //                     std::cout << "        applying speedControl (previous)" <<  "  time_gap=" << time_gap << std::endl;
    //                 }
    // #endif
    //                 newSpeed = speedSpeedControl(speed, vErr, vehMode);
    //             } else {
    //                 // CACC_ControlMode = gap control
    // #if DEBUG_CACC == 1
    //                 if (DEBUG_COND) {
    //                     std::cout << "        previous speedGapControl (previous)" << "  time_gap=" << time_gap << std::endl;
    //                 }
    // #endif
    //                 newSpeed = speedGapControl(veh, gap2pred, speed, predSpeed, desSpeed, vErr, pred, vehMode);
    //             }
    //         }
    //     } else if (commMode == CACC_MODE_NO_LEADER) {
    //         newSpeed = speedSpeedControl(speed, vErr, vehMode);
    //     } else if (commMode == CACC_MODE_LEADER_NO_CAV) {
    //         newSpeed = acc_CFM._v(veh, gap2pred, speed, predSpeed, desSpeed, true);
    //         vehMode = ACC_MODE;
    //     } else if (commMode == CACC_MODE_LEADER_CAV) {
    //         double desSpacing = myHeadwayTime * speed;
    //         double spacingErr = gap2pred - desSpacing;
    //         double accel = veh->getAcceleration();
    //         double speedErr = predSpeed - speed - myHeadwayTime * accel;
    //
    //         if ((spacingErr > 0 && spacingErr < 0.2) && (vErr < 0.1)) {
    //             // gap mode
    //             //newSpeed = speed + 0.45 * spacingErr + 0.0125 *speedErr;
    //             if (DEBUG_COND) {
    //                 std::cout << "        applying CACC_GAP_MODE " << std::endl;
    //             }
    //             newSpeed = speed + myGapControlGainGap * spacingErr + myGapControlGainGapDot * speedErr;
    //             vehMode = CACC_GAP_MODE;
    //         } else if (spacingErr < 0) {
    //             // collision avoidance mode
    //             //newSpeed = speed + 0.45 * spacingErr + 0.05 *speedErr;
    //             if (DEBUG_COND) {
    //                 std::cout << "        applying CACC_COLLISION_AVOIDANCE_MODE " << std::endl;
    //             }
    //             newSpeed = speed + myCollisionAvoidanceGainGap * spacingErr + myCollisionAvoidanceGainGapDot * speedErr;
    //             vehMode = CACC_COLLISION_AVOIDANCE_MODE;
    //         } else {
    //             // gap closing mode
    //             if (DEBUG_COND) {
    //                 std::cout << "        applying CACC_GAP_CLOSING_MODE " << std::endl;
    //             }
    //             newSpeed = speed + myGapClosingControlGainGap * spacingErr + myGapClosingControlGainGapDot * speedErr;
    //             vehMode = CACC_GAP_CLOSING_MODE;
    //         }
    //     }
    //
    //     if (setControlMode && !MSGlobals::gComputeLC && usage == CalcReason::CURRENT) {
    //         const_cast<SUMOVehicleParameter&>(veh->getParameter()).setParameter("caccVehicleMode", VehicleModeNames[vehMode]);
    //     }
    //
    //     //std::cout << veh->getID() << " commMode: " << commMode << ", caccVehicleMode: " << VehicleModeNames[vehMode]
    //     //            << ", gap2pred: " << gap2pred << ", newSpeed: " << newSpeed << std::endl;
    //
    //     // newSpeed is meant for step length 0.1 but this would cause extreme
    //     // accelerations at lower step length
    //     double newSpeedScaled = newSpeed;
    //     if (DELTA_T < 100) {
    //         const double accel01 = (newSpeed - speed) * 10;
    //         newSpeedScaled = speed + ACCEL2SPEED(accel01);
    //     }
    //
    // #if DEBUG_CACC == 1
    //     if (DEBUG_COND) {
    //         std::cout << "        result: rawAccel=" << SPEED2ACCEL(newSpeed - speed) << " newSpeed=" << newSpeed << " newSpeedScaled=" << newSpeedScaled << "\n";
    //     }
    // #endif
    //
    //     return MAX2(0., newSpeedScaled);
    // }
    #[allow(unused_variables)]
    pub fn v(&self, veh: Ptr<MSVehicle>, pred: Ptr<MSVehicle>, gap2pred: f64, mySpeed: f64, predSpeed: f64, desSpeed: f64, respectMinGap: bool, usage: MSCFModel_CalcReason) -> f64 {
        let mut newSpeed = 0.0;
        let mut vehMode = MSCFModel_CACC_VehicleMode::CC_MODE;
        let speed = mySpeed;

        /* Velocity error */
        let vErr = speed - desSpeed;
        let mut setControlMode = false;
        let vars = veh.get_car_follow_variables().cast::<MSCFModel_CACC_CACCVehicleVariables>();
        let now = MSNet::get_instance().get_current_time_step();
        if vars.lastUpdateTime != now {
            vars.m().lastUpdateTime = now;
            setControlMode = true;
        }

        let commMode = vars.CACC_CommunicationsOverrideMode;

        if commMode == MSCFModel_CACC_CommunicationsOverrideMode::CACC_NO_OVERRIDE { // old CACC logic (rely on time gap from predecessor)
            // @note: using (gap2pred + minGap) here increases oscillations but may
            // actually be a good idea once the acceleration-spike-problem is fixed
            let time_gap = gap2pred / max2(NUMERICAL_EPS, speed);
            let spacingErr = gap2pred - self.myHeadwayTime * speed;

            if time_gap > 2. && spacingErr > self.mySpeedControlMinGap {
                // Find acceleration - Speed control law
                newSpeed = self.speed_speed_control(speed, vErr, &mut vehMode);
                // Set cl to vehicle parameters
                if setControlMode {
                    vars.m().CACC_ControlMode = 0;
                }
            } else if time_gap < 1.5 {
                // Find acceleration - Gap control law
                newSpeed = self.speed_gap_control(veh, gap2pred, speed, predSpeed, desSpeed, vErr, pred, &mut vehMode);
                // Set cl to vehicle parameters
                if setControlMode {
                    vars.m().CACC_ControlMode = 1;
                }
            } else {
                // Follow previous applied law
                let cm = vars.CACC_ControlMode;
                if cm == 0 {
                    // CACC_ControlMode = speed control
                    newSpeed = self.speed_speed_control(speed, vErr, &mut vehMode);
                } else {
                    // CACC_ControlMode = gap control
                    newSpeed = self.speed_gap_control(veh, gap2pred, speed, predSpeed, desSpeed, vErr, pred, &mut vehMode);
                }
            }
        } else if commMode == MSCFModel_CACC_CommunicationsOverrideMode::CACC_MODE_NO_LEADER {
            newSpeed = self.speed_speed_control(speed, vErr, &mut vehMode);
        } else if commMode == MSCFModel_CACC_CommunicationsOverrideMode::CACC_MODE_LEADER_NO_CAV {
            newSpeed = self.acc_CFM.v(veh, gap2pred, speed, predSpeed, desSpeed, true);
            vehMode = MSCFModel_CACC_VehicleMode::ACC_MODE;
        } else if commMode == MSCFModel_CACC_CommunicationsOverrideMode::CACC_MODE_LEADER_CAV {
            let desSpacing = self.myHeadwayTime * speed;
            let spacingErr = gap2pred - desSpacing;
            let accel = veh.get_acceleration();
            let speedErr = predSpeed - speed - self.myHeadwayTime * accel;

            if (spacingErr > 0. && spacingErr < 0.2) && (vErr < 0.1) {
                // gap mode
                newSpeed = speed + self.myGapControlGainGap * spacingErr + self.myGapControlGainGapDot * speedErr;
                vehMode = MSCFModel_CACC_VehicleMode::CACC_GAP_MODE;
            } else if spacingErr < 0. {
                // collision avoidance mode
                newSpeed = speed + self.myCollisionAvoidanceGainGap * spacingErr + self.myCollisionAvoidanceGainGapDot * speedErr;
                vehMode = MSCFModel_CACC_VehicleMode::CACC_COLLISION_AVOIDANCE_MODE;
            } else {
                // gap closing mode
                newSpeed = speed + self.myGapClosingControlGainGap * spacingErr + self.myGapClosingControlGainGapDot * speedErr;
                vehMode = MSCFModel_CACC_VehicleMode::CACC_GAP_CLOSING_MODE;
            }
        }

        if setControlMode && !MSGlobals::compute_lc() && usage == MSCFModel_CalcReason::CURRENT {
            set_cacc_vehicle_mode(veh, vehicle_mode_name(vehMode));
        }

        // newSpeed is meant for step length 0.1 but this would cause extreme
        // accelerations at lower step length
        let mut newSpeedScaled = newSpeed;
        if delta_t() < 100 {
            let accel01 = (newSpeed - speed) * 10.;
            newSpeedScaled = speed + accel2speed(accel01);
        }

        max2(0., newSpeedScaled)
    }

    /// C++: `double MSCFModel_CACC::speedSpeedControl(const double, double, VehicleMode &) const` [const, private]
    // double
    // MSCFModel_CACC::speedSpeedControl(const double speed, double vErr, VehicleMode& vehMode) const {
    //     // Speed control law
    //     vehMode = CC_MODE;
    //     double sclAccel = mySpeedControlGain * vErr;
    //     double newSpeed = speed + ACCEL2SPEED(sclAccel);
    //     return newSpeed;
    // }
    #[allow(unused_variables)]
    pub fn speed_speed_control(&self, speed: f64, vErr: f64, vehMode: &mut MSCFModel_CACC_VehicleMode) -> f64 {
        // Speed control law
        *vehMode = MSCFModel_CACC_VehicleMode::CC_MODE;
        let sclAccel = self.mySpeedControlGain * vErr;
        let newSpeed = speed + accel2speed(sclAccel);
        newSpeed
    }

    /// C++: `double MSCFModel_CACC::speedGapControl(const MSVehicle *const, const double, const double, const double, const double, double, const MSVehicle *const, VehicleMode &) const` [const, private]
    // double
    // MSCFModel_CACC::speedGapControl(const MSVehicle* const veh, const double gap2pred,
    //                                 const double speed, const double predSpeed, const double desSpeed, double vErr,
    //                                 const MSVehicle* const pred, VehicleMode& vehMode) const {
    //     // Gap control law
    //     double newSpeed = 0.0;
    //
    //     if (pred != nullptr) {
    //         if (pred->getCarFollowModel().getModelID() != SUMO_TAG_CF_CACC) {
    //             vehMode = ACC_MODE;
    //             newSpeed = acc_CFM._v(veh, gap2pred, speed, predSpeed, desSpeed, true);
    // #if DEBUG_CACC == 1
    //             if (DEBUG_COND) {
    //                 std::cout << "        acc control mode" << std::endl;
    //             }
    // #endif
    //         } else {
    // #if DEBUG_CACC == 1
    //             if (DEBUG_COND) {
    //                 std::cout << "        CACC control mode" << std::endl;
    //             }
    // #endif
    //             double desSpacing = myHeadwayTime * speed;
    //             double spacingErr = gap2pred - desSpacing;
    //             double accel = veh->getAcceleration();
    //             double speedErr = predSpeed - speed - myHeadwayTime * accel;
    //
    //             if ((spacingErr > 0 && spacingErr < 0.2) && (vErr < 0.1)) {
    //                 // gap mode
    //                 //newSpeed = speed + 0.45 * spacingErr + 0.0125 *speedErr;
    // #if DEBUG_CACC == 1
    //                 if (DEBUG_COND) {
    //                     std::cout << "        applying gap control:  err=" << spacingErr << " speedErr=" << speedErr << std::endl;
    //                 }
    // #endif
    //                 newSpeed = speed + myGapControlGainGap * spacingErr + myGapControlGainGapDot * speedErr;
    //
    //                 vehMode = CACC_GAP_MODE;
    //             } else if (spacingErr < 0) {
    //                 // collision avoidance mode
    //                 //newSpeed = speed + 0.45 * spacingErr + 0.05 *speedErr;
    // #if DEBUG_CACC == 1
    //                 if (DEBUG_COND) {
    //                     std::cout << "        applying collision avoidance:  err=" << spacingErr << " speedErr=" << speedErr << "\n";
    //                 }
    // #endif
    //                 newSpeed = speed + myCollisionAvoidanceGainGap * spacingErr + myCollisionAvoidanceGainGapDot * speedErr;
    //                 vehMode = CACC_COLLISION_AVOIDANCE_MODE;
    //             } else {
    //                 // gap closing mode
    // #if DEBUG_CACC == 1
    //                 if (DEBUG_COND) {
    //                     std::cout << "        applying gap closing err=" << spacingErr << " speedErr=" << speedErr << " predSpeed=" << predSpeed << " speed=" << speed << " accel=" << accel << "\n";
    //                 }
    // #endif
    //                 newSpeed = speed + myGapClosingControlGainGap * spacingErr + myGapClosingControlGainGapDot * speedErr;
    //
    //                 vehMode = CACC_GAP_CLOSING_MODE;
    //             }
    //         }
    //     } else {
    //         /* no leader */
    // #if DEBUG_CACC == 1
    //         if (DEBUG_COND) {
    //             std::cout << "        no leader" << std::endl;
    //         }
    // #endif
    //         newSpeed = speedSpeedControl(speed, vErr, vehMode);
    //     }
    //
    //     return newSpeed;
    // }
    #[allow(unused_variables)]
    pub fn speed_gap_control(&self, veh: Ptr<MSVehicle>, gap2pred: f64, speed: f64, predSpeed: f64, desSpeed: f64, vErr: f64, pred: Ptr<MSVehicle>, vehMode: &mut MSCFModel_CACC_VehicleMode) -> f64 {
        // Gap control law
        let mut newSpeed = 0.0;

        if !pred.is_null() {
            if pred.get_car_follow_model().get_model_id() != SUMO_TAG_CF_CACC as i32 {
                *vehMode = MSCFModel_CACC_VehicleMode::ACC_MODE;
                newSpeed = self.acc_CFM.v(veh, gap2pred, speed, predSpeed, desSpeed, true);
            } else {
                let desSpacing = self.myHeadwayTime * speed;
                let spacingErr = gap2pred - desSpacing;
                let accel = veh.get_acceleration();
                let speedErr = predSpeed - speed - self.myHeadwayTime * accel;

                if (spacingErr > 0. && spacingErr < 0.2) && (vErr < 0.1) {
                    // gap mode
                    newSpeed = speed + self.myGapControlGainGap * spacingErr + self.myGapControlGainGapDot * speedErr;

                    *vehMode = MSCFModel_CACC_VehicleMode::CACC_GAP_MODE;
                } else if spacingErr < 0. {
                    // collision avoidance mode
                    newSpeed = speed + self.myCollisionAvoidanceGainGap * spacingErr + self.myCollisionAvoidanceGainGapDot * speedErr;
                    *vehMode = MSCFModel_CACC_VehicleMode::CACC_COLLISION_AVOIDANCE_MODE;
                } else {
                    // gap closing mode
                    newSpeed = speed + self.myGapClosingControlGainGap * spacingErr + self.myGapClosingControlGainGapDot * speedErr;

                    *vehMode = MSCFModel_CACC_VehicleMode::CACC_GAP_CLOSING_MODE;
                }
            }
        } else {
            /* no leader */
            newSpeed = self.speed_speed_control(speed, vErr, vehMode);
        }

        newSpeed
    }

    /// C++: `MSCFModel_CACC & MSCFModel_CACC::operator=(const MSCFModel_CACC &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MSCFModel_CACC) -> &mut MSCFModel_CACC {
        // shortcut: C++ operator= is private and never defined (C++ cannot call it); panic mirrors that
        panic!("MSCFModel_CACC::operator= is private and not defined in C++")
    }

}

