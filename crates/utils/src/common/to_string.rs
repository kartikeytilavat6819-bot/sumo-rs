//! Port of utils/common/ToString.h (numeric part): fixed notation with gPrecision decimals
use super::std_defs::g_precision;

pub fn f64_to_string_prec(v: f64, accuracy: i32) -> String {
    format!("{:.*}", accuracy.max(0) as usize, v)
}
pub fn f64_to_string(v: f64) -> String {
    f64_to_string_prec(v, g_precision())
}
