//! Port of utils/common/ToString.h (numeric part): fixed notation with gPrecision decimals
use super::std_defs::g_precision;

pub fn f64_to_string_prec(v: f64, accuracy: i32) -> String {
    format!("{:.*}", accuracy.max(0) as usize, v)
}
pub fn f64_to_string(v: f64) -> String {
    f64_to_string_prec(v, g_precision())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ties_round_like_fmt() {
        // fmt / printf round the exact binary value, ties to even: same as Rust's formatter
        assert_eq!(f64_to_string_prec(0.125, 2), "0.12");
        assert_eq!(f64_to_string_prec(0.375, 2), "0.38");
        assert_eq!(f64_to_string_prec(2.675, 2), "2.67"); // 2.675 is stored as 2.67499999...
        assert_eq!(f64_to_string_prec(-0.0, 2), "-0.00");
    }
}
