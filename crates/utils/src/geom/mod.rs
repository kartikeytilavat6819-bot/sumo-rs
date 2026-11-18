//! Port of utils/geom
pub mod abstract_poly;
pub mod boundary;
pub mod bresenham;
pub mod geo_conv_helper;
pub mod geom_conv_helper;
pub mod geom_helper;
pub mod position;
pub mod position_vector;
pub mod triangle;

pub use abstract_poly::AbstractPoly;
pub use boundary::Boundary;
pub use position::Position;
pub use position_vector::PositionVector;
pub use triangle::Triangle;

// shortcut: config.h constants live here until common ports config.h; move them there when it does
pub const POSITION_EPS: f64 = 0.1;
pub const NUMERICAL_EPS: f64 = 0.001;

/// C++ `InvalidArgument`
// shortcut: local stand-in for UtilExceptions::InvalidArgument (common has ProcessError::InvalidArgument);
// switching changes the return type of split_at / move2side_custom, whose ~33 callers in sumo_core are outside this area
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidArgument(pub String);

pub(crate) use crate::common::msg_handler::{write_error, write_warning};

/// MIN2/MAX2 from StdDefs.h (differ from f64::min/max for NaN)
#[inline]
pub(crate) fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
#[inline]
pub(crate) fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// StringTokenizer(str, token) (no empty tokens, like C++)
pub(crate) fn tokenize(s: &str, token: &str) -> Vec<String> {
    crate::common::string_tokenizer::StringTokenizer::with_token(s, token, false).get_vector()
}

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    fn tokenize_keeps_empty_segments() {
        // C++ StringTokenizer::prepare pushes every segment, so a double separator gives an empty token
        assert_eq!(tokenize("1,2  3,4", " "), ["1,2", "", "3,4"]);
        assert_eq!(tokenize("1,,2,", ","), ["1", "", "2", ""]);
        assert_eq!(tokenize("", " "), Vec::<String>::new());
    }
}
