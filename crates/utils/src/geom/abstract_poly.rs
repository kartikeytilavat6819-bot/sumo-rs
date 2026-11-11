//! Port of utils/geom/AbstractPoly.h
use super::Position;

/// C++ default argument `offset = 0` must be passed explicitly.
pub trait AbstractPoly {
    /// Returns whether the AbstractPoly the given coordinate
    fn around(&self, p: &Position, offset: f64) -> bool;
    /// Returns whether the AbstractPoly overlaps with the given polygon
    fn overlaps_with(&self, poly: &dyn AbstractPoly, offset: f64) -> bool;
    /// Returns whether the AbstractPoly is partially within the given polygon
    fn partial_within(&self, poly: &dyn AbstractPoly, offset: f64) -> bool;
    /// Returns whether the AbstractPoly crosses the given line
    fn crosses(&self, p1: &Position, p2: &Position) -> bool;
}
