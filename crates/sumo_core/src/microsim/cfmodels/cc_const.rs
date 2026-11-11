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

