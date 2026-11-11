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
// shortcut: local stand-in for UtilExceptions; switch to common's exception types once ported
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidArgument(pub String);

// shortcut: prints like MsgHandler's default console output; route through MsgHandler once it is ported
pub(crate) fn write_warning(msg: &str) {
    eprintln!("Warning: {msg}");
}
pub(crate) fn write_error(msg: &str) {
    eprintln!("Error: {msg}");
}

/// MIN2/MAX2 from StdDefs.h (differ from f64::min/max for NaN)
#[inline]
pub(crate) fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
#[inline]
pub(crate) fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// shortcut: minimal StringTokenizer(str, token) splitting, replace with common::StringTokenizer once ported
pub(crate) fn tokenize<'a>(s: &'a str, token: &str) -> Vec<&'a str> {
    if s.is_empty() { Vec::new() } else { s.split(token).collect() }
}
