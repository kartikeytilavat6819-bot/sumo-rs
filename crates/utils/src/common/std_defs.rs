//! Port of utils/common/StdDefs.{h,cpp}
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering::Relaxed};

pub const SUMO_MAX_CONNECTIONS: usize = 256;
pub const SUMO_CONST_LANE_WIDTH: f64 = 3.2;
pub const SUMO_CONST_HALF_LANE_WIDTH: f64 = SUMO_CONST_LANE_WIDTH / 2.;
pub const SUMO_CONST_QUARTER_LANE_WIDTH: f64 = SUMO_CONST_LANE_WIDTH / 4.;
pub const SUMO_CONST_LANE_MARK_WIDTH: f64 = 0.1;
pub const SUMO_CONST_WAITING_PERSON_WIDTH: f64 = 0.8;
pub const SUMO_CONST_WAITING_PERSON_DEPTH: f64 = 0.67;
pub const SUMO_CONST_WAITING_CONTAINER_WIDTH: f64 = 2.5;
pub const SUMO_CONST_WAITING_CONTAINER_DEPTH: f64 = 6.2;
pub const SUMO_CONST_HALTING_SPEED: f64 = 0.1;
pub const INVALID_INT: i32 = i32::MAX;
pub const INVALID_DOUBLE: f64 = f64::MAX;
pub const NETWORK_VERSION: (i32, f64) = (1, 20.);

macro_rules! global {
    ($get:ident, $set:ident, $static:ident, $ty:ty, $atomic:ty, $init:expr) => {
        static $static: $atomic = <$atomic>::new($init);
        #[inline]
        pub fn $get() -> $ty {
            $static.load(Relaxed)
        }
        pub fn $set(v: $ty) {
            $static.store(v, Relaxed)
        }
    };
}
global!(g_precision, set_g_precision, G_PRECISION, i32, AtomicI32, 2);
global!(g_precision_emissions, set_g_precision_emissions, G_PRECISION_EMISSIONS, i32, AtomicI32, 2);
global!(g_precision_geo, set_g_precision_geo, G_PRECISION_GEO, i32, AtomicI32, 6);
global!(g_precision_random, set_g_precision_random, G_PRECISION_RANDOM, i32, AtomicI32, 4);
global!(g_human_readable_time, set_g_human_readable_time, G_HUMAN_READABLE_TIME, bool, AtomicBool, false);
global!(g_simulation, set_g_simulation, G_SIMULATION, bool, AtomicBool, false);
global!(g_ignore_unknown_vclass, set_g_ignore_unknown_vclass, G_IGNORE_UNKNOWN_VCLASS, bool, AtomicBool, false);
global!(g_routing_preferences, set_g_routing_preferences, G_ROUTING_PREFERENCES, bool, AtomicBool, false);
global!(g_taxi_classes, set_g_taxi_classes, G_TAXI_CLASSES, i32, AtomicI32, 0);

pub fn truncate(x: f64, fraction_bits: i32) -> f64 {
    let s = (1i64 << fraction_bits) as f64;
    (x * s).ceil() / s
}
pub fn round_bits(x: f64, fraction_bits: i32) -> f64 {
    let s = (1i64 << fraction_bits) as f64;
    let x2 = x * s;
    (if x2 < 0. { (x2 - 0.5).ceil() } else { (x2 + 0.5).floor() }) / s
}
pub fn round_decimal(x: f64, precision: i32) -> f64 {
    let p = 10f64.powi(precision);
    let x2 = x * p;
    (if x2 < 0. { (x2 - 0.5).ceil() } else { (x2 + 0.5).floor() }) / p
}
pub fn round_decimal_to_even(x: f64, precision: i32) -> f64 {
    let p = 10f64.powi(precision) as i32 as f64;
    (x * p).round_ties_even() / p
}
pub fn get_scaling_quota(frac: f64, loaded: i64) -> i32 {
    if frac < 0. || frac == 1. {
        return 1;
    }
    let base = frac as i32;
    let resolution = 1000i64;
    let int_frac = ((frac - base as f64) * resolution as f64 + 0.5).floor() as i64;
    if ((loaded % resolution) * int_frac) % resolution < int_frac {
        base + 1
    } else {
        base
    }
}
