//! Port of microsim/cfmodels/CC_Const.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ enum `Plexe::ACTIVE_CONTROLLER`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum ACTIVE_CONTROLLER {
    DRIVER = 0,
    ACC = 1,
    CACC = 2,
    FAKED_CACC = 3,
    PLOEG = 4,
    CONSENSUS = 5,
    FLATBED = 6,
}

/// C++ enum `Plexe::PLATOONING_LANE_CHANGE_ACTION`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum PLATOONING_LANE_CHANGE_ACTION {
    DRIVER_CHOICE = 0,
    STAY_IN_CURRENT_LANE = 3,
    MOVE_TO_FIXED_LANE = 4,
}

/// struct used as header for generic data passing to this model through traci
/// C++ class `Plexe::CCDataHeader` (microsim/cfmodels/CC_Const.h)
pub struct CCDataHeader {
    pub type_: i32,
    pub size: i32,
}

impl CCDataHeader {
}

/// Struct defining data passed about a vehicle
/// C++ class `Plexe::VEHICLE_DATA` (microsim/cfmodels/CC_Const.h)
#[derive(Clone, Copy, Default)]
pub struct VEHICLE_DATA {
    pub index: i32,
    pub speed: f64,
    pub acceleration: f64,
    pub positionX: f64,
    pub positionY: f64,
    pub time: f64,
    pub length: f64,
    pub u: f64,
    pub speedX: f64,
    pub speedY: f64,
    pub angle: f64,
}

impl VEHICLE_DATA {
}

// C++ #define constants from microsim/cfmodels/CC_Const.h
pub const FIX_LC: i32 = 0;
pub const DEFAULT_NOTRACI_LC: i32 = 0;
pub const MAX_N_CARS: i32 = 8;
pub const CC_ENGINE_MODEL_FOLM: i32 = 0;
pub const CC_ENGINE_MODEL_REALISTIC: i32 = 1;
pub const FOLM_PAR_TAU: &str = "tau_s";
pub const FOLM_PAR_DT: &str = "dt_s";
pub const ENGINE_PAR_VEHICLE: &str = "vehicle";
pub const ENGINE_PAR_XMLFILE: &str = "xmlFile";
pub const ENGINE_PAR_DT: &str = "dt_s";
pub const CC_PAR_VEHICLE_DATA: &str = "ccvd";
pub const CC_PAR_VEHICLE_POSITION: &str = "ccvp";
pub const CC_PAR_PLATOON_SIZE: &str = "ccps";
pub const CC_PAR_CACC_XI: &str = "ccxi";
pub const CC_PAR_CACC_OMEGA_N: &str = "ccon";
pub const CC_PAR_CACC_C1: &str = "ccc1";
pub const CC_PAR_ENGINE_TAU: &str = "cctau";
pub const CC_PAR_UMIN: &str = "ccumin";
pub const CC_PAR_UMAX: &str = "ccumax";
pub const CC_PAR_PLOEG_H: &str = "ccph";
pub const CC_PAR_PLOEG_KP: &str = "ccpkp";
pub const CC_PAR_PLOEG_KD: &str = "ccpkd";
pub const CC_PAR_FLATBED_KA: &str = "ccfka";
pub const CC_PAR_FLATBED_KV: &str = "ccfkv";
pub const CC_PAR_FLATBED_KP: &str = "ccfkp";
pub const CC_PAR_FLATBED_H: &str = "ccfh";
pub const CC_PAR_FLATBED_D: &str = "ccfd";
pub const CC_PAR_VEHICLE_ENGINE_MODEL: &str = "ccem";
pub const CC_PAR_VEHICLE_MODEL: &str = "ccvm";
pub const CC_PAR_VEHICLES_FILE: &str = "ccvf";
pub const PAR_CACC_SPACING: &str = "ccsp";
pub const PAR_ACC_ACCELERATION: &str = "ccacc";
pub const PAR_CRASHED: &str = "cccr";
pub const PAR_FIXED_ACCELERATION: &str = "ccfa";
pub const PAR_SPEED_AND_ACCELERATION: &str = "ccsa";
pub const PAR_LEADER_SPEED_AND_ACCELERATION: &str = "cclsa";
pub const PAR_USE_CONTROLLER_ACCELERATION: &str = "ccca";
pub const PAR_LANES_COUNT: &str = "cclc";
pub const PAR_CC_DESIRED_SPEED: &str = "ccds";
pub const PAR_ACTIVE_CONTROLLER: &str = "ccac";
pub const PAR_RADAR_DATA: &str = "ccrd";
pub const PAR_LEADER_FAKE_DATA: &str = "cclfd";
pub const PAR_FRONT_FAKE_DATA: &str = "ccffd";
pub const PAR_DISTANCE_TO_END: &str = "ccdte";
pub const PAR_DISTANCE_FROM_BEGIN: &str = "ccdfb";
pub const PAR_PRECEDING_SPEED_AND_ACCELERATION: &str = "ccpsa";
pub const PAR_ACC_HEADWAY_TIME: &str = "ccaht";
pub const PAR_ENGINE_DATA: &str = "cced";
pub const PAR_USE_AUTO_FEEDING: &str = "ccaf";
pub const PAR_USE_PREDICTION: &str = "ccup";
pub const PAR_ADD_MEMBER: &str = "ccam";
pub const PAR_REMOVE_MEMBER: &str = "ccrm";
pub const PAR_ENABLE_AUTO_LANE_CHANGE: &str = "ccalc";
pub const PAR_PLATOON_FIXED_LANE: &str = "ccpfl";
