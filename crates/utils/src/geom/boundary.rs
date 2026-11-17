//! Port of utils/geom/Boundary.{h,cpp}
use std::fmt;

use super::{max2, min2, AbstractPoly, Position, PositionVector};
use crate::common::std_defs::g_precision;
use crate::common::to_string::f64_to_string_prec;

/// A class that stores a 2D geometrical boundary
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Boundary {
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    zmin: f64,
    zmax: f64,
    was_initialised: bool,
}

impl Default for Boundary {
    fn default() -> Self {
        Self::new()
    }
}

impl Boundary {
    /// the boundary is unset
    pub const fn new() -> Self {
        Boundary {
            xmin: 10000000000.0,
            xmax: -10000000000.0,
            ymin: 10000000000.0,
            ymax: -10000000000.0,
            zmin: 10000000000.0,
            zmax: -10000000000.0,
            was_initialised: false,
        }
    }
    pub fn from_xy(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        let mut b = Self::new();
        b.add(x1, y1, 0.);
        b.add(x2, y2, 0.);
        b
    }
    pub fn from_xyz(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Self {
        let mut b = Self::new();
        b.add(x1, y1, z1);
        b.add(x2, y2, z2);
        b
    }
    pub fn reset(&mut self) {
        *self = Self::new();
    }
    /// Makes the boundary include the given coordinate (C++ default z = 0)
    pub fn add(&mut self, x: f64, y: f64, z: f64) {
        if !self.was_initialised {
            self.ymin = y;
            self.ymax = y;
            self.xmin = x;
            self.xmax = x;
            self.zmin = z;
            self.zmax = z;
        } else {
            self.xmin = if self.xmin < x { self.xmin } else { x };
            self.xmax = if self.xmax > x { self.xmax } else { x };
            self.ymin = if self.ymin < y { self.ymin } else { y };
            self.ymax = if self.ymax > y { self.ymax } else { y };
            self.zmin = if self.zmin < z { self.zmin } else { z };
            self.zmax = if self.zmax > z { self.zmax } else { z };
        }
        self.was_initialised = true;
    }
    pub fn add_pos(&mut self, p: &Position) {
        self.add(p.x(), p.y(), p.z());
    }
    pub fn add_boundary(&mut self, p: &Boundary) {
        self.add(p.xmin(), p.ymin(), p.zmin());
        self.add(p.xmax(), p.ymax(), p.zmax());
    }
    pub fn get_center(&self) -> Position {
        Position::new3((self.xmin + self.xmax) / 2.0, (self.ymin + self.ymax) / 2.0, (self.zmin + self.zmax) / 2.0)
    }
    pub fn xmin(&self) -> f64 {
        self.xmin
    }
    pub fn xmax(&self) -> f64 {
        self.xmax
    }
    pub fn ymin(&self) -> f64 {
        self.ymin
    }
    pub fn ymax(&self) -> f64 {
        self.ymax
    }
    pub fn zmin(&self) -> f64 {
        self.zmin
    }
    pub fn zmax(&self) -> f64 {
        self.zmax
    }
    pub fn get_width(&self) -> f64 {
        self.xmax - self.xmin
    }
    pub fn get_height(&self) -> f64 {
        self.ymax - self.ymin
    }
    pub fn get_z_range(&self) -> f64 {
        self.zmax - self.zmin
    }
    pub fn around2d(&self, p: &Position, offset: f64) -> bool {
        ((p.x() <= self.xmax + offset) && (p.x() >= self.xmin - offset))
            && ((p.y() <= self.ymax + offset) && (p.y() >= self.ymin - offset))
    }
    pub fn around2d_xy(&self, x: f64, y: f64) -> bool {
        ((x <= self.xmax) && (x >= self.xmin)) && ((y <= self.ymax) && (y >= self.ymin))
    }
    pub fn contains2d(&self, b: &Boundary) -> bool {
        (self.xmin <= b.xmin()) && (self.ymin <= b.ymin()) && (self.xmax >= b.xmax()) && (self.ymax >= b.ymax())
    }
    pub fn overlaps2d(&self, b: &Boundary) -> bool {
        self.around2d_xy(b.xmin, b.ymin)
            || self.around2d_xy(b.xmin, b.ymax)
            || self.around2d_xy(b.xmax, b.ymin)
            || self.around2d_xy(b.xmax, b.ymax)
    }
    pub fn is_initialised(&self) -> bool {
        self.was_initialised
    }
    pub fn distance_to2d(&self, p: &Position) -> f64 {
        Self::dist(self.xmin - p.x(), p.x() - self.xmax, self.ymin - p.y(), p.y() - self.ymax)
    }
    pub fn distance_to2d_boundary(&self, b: &Boundary) -> f64 {
        Self::dist(self.xmin - b.xmax, b.xmin - self.xmax, self.ymin - b.ymax, b.ymin - self.ymax)
    }
    fn dist(left_dist: f64, right_dist: f64, bottom_dist: f64, top_dist: f64) -> f64 {
        if left_dist > 0. {
            if bottom_dist > 0. {
                return (left_dist * left_dist + bottom_dist * bottom_dist).sqrt();
            }
            if top_dist > 0. {
                return (left_dist * left_dist + top_dist * top_dist).sqrt();
            }
            return left_dist;
        }
        if right_dist > 0. {
            if bottom_dist > 0. {
                return (right_dist * right_dist + bottom_dist * bottom_dist).sqrt();
            }
            if top_dist > 0. {
                return (right_dist * right_dist + top_dist * top_dist).sqrt();
            }
            return right_dist;
        }
        if bottom_dist > 0. {
            return bottom_dist;
        }
        if top_dist > 0. {
            return top_dist;
        }
        0.
    }
    pub fn grow(&mut self, by: f64) -> &mut Self {
        self.xmax += by;
        self.ymax += by;
        self.xmin -= by;
        self.ymin -= by;
        self
    }
    pub fn scale(&mut self, by: f64) -> &mut Self {
        self.grow_width(by * (self.xmax - self.xmin));
        self.grow_height(by * (self.ymax - self.ymin));
        self
    }
    pub fn grow_width(&mut self, by: f64) {
        self.xmin -= by;
        self.xmax += by;
    }
    pub fn grow_height(&mut self, by: f64) {
        self.ymin -= by;
        self.ymax += by;
    }
    pub fn flip_y(&mut self) {
        self.ymin *= -1.0;
        self.ymax *= -1.0;
        std::mem::swap(&mut self.ymin, &mut self.ymax);
    }
    /// Sets the boundary to the given values (fixing min > max)
    pub fn set(&mut self, xmin: f64, ymin: f64, xmax: f64, ymax: f64) {
        self.xmin = min2(xmin, xmax);
        self.ymin = min2(ymin, ymax);
        self.xmax = max2(xmin, xmax);
        self.ymax = max2(ymin, ymax);
    }
    /// Sets the boundary to the given values, ignoring min < max constraints
    pub fn set_offsets(&mut self, xmin: f64, ymin: f64, xmax: f64, ymax: f64) {
        self.xmin = xmin;
        self.ymin = ymin;
        self.xmax = xmax;
        self.ymax = ymax;
    }
    /// Moves the boundary by the given amount (C++ default z = 0)
    pub fn moveby(&mut self, x: f64, y: f64, z: f64) {
        self.xmin += x;
        self.ymin += y;
        self.zmin += z;
        self.xmax += x;
        self.ymax += y;
        self.zmax += z;
    }
    pub fn get_shape(&self, close_shape: bool) -> PositionVector {
        let mut shape = PositionVector::new();
        shape.push(Position::new(self.xmin, self.ymin));
        shape.push(Position::new(self.xmin, self.ymax));
        shape.push(Position::new(self.xmax, self.ymax));
        shape.push(Position::new(self.xmax, self.ymin));
        if close_shape {
            shape.push(Position::new(self.xmin, self.ymin));
        }
        shape
    }
    /// C++ `os << boundary` with fixed notation and the given precision
    pub fn to_string_prec(&self, precision: i32) -> String {
        format!(
            "{},{},{},{}",
            f64_to_string_prec(self.xmin, precision),
            f64_to_string_prec(self.ymin, precision),
            f64_to_string_prec(self.xmax, precision),
            f64_to_string_prec(self.ymax, precision)
        )
    }
}

impl fmt::Display for Boundary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prec = f.precision().map(|p| p as i32).unwrap_or_else(g_precision);
        f.write_str(&self.to_string_prec(prec))
    }
}

impl crate::iodevices::output_device::OutputValue for Boundary {
    fn to_output_string(&self, precision: i32) -> std::borrow::Cow<str> {
        std::borrow::Cow::Owned(self.to_string_prec(precision))
    }
}

impl AbstractPoly for Boundary {
    fn around(&self, p: &Position, offset: f64) -> bool {
        ((p.x() <= self.xmax + offset) && (p.x() >= self.xmin - offset))
            && ((p.y() <= self.ymax + offset) && (p.y() >= self.ymin - offset))
            && ((p.z() <= self.zmax + offset) && (p.z() >= self.zmin - offset))
    }
    fn overlaps_with(&self, p: &dyn AbstractPoly, offset: f64) -> bool {
        if self.partial_within(p, offset) || p.partial_within(self, offset) {
            return true;
        }
        let (xmin, xmax, ymin, ymax) = (self.xmin, self.xmax, self.ymin, self.ymax);
        p.crosses(&Position::new(xmax + offset, ymax + offset), &Position::new(xmin - offset, ymax + offset))
            || p.crosses(&Position::new(xmin - offset, ymax + offset), &Position::new(xmin - offset, ymin - offset))
            || p.crosses(&Position::new(xmin - offset, ymin - offset), &Position::new(xmax + offset, ymin - offset))
            || p.crosses(&Position::new(xmax + offset, ymin - offset), &Position::new(xmax + offset, ymax + offset))
    }
    fn partial_within(&self, poly: &dyn AbstractPoly, offset: f64) -> bool {
        poly.around(&Position::new(self.xmax, self.ymax), offset)
            || poly.around(&Position::new(self.xmin, self.ymax), offset)
            || poly.around(&Position::new(self.xmax, self.ymin), offset)
            || poly.around(&Position::new(self.xmin, self.ymin), offset)
    }
    fn crosses(&self, p1: &Position, p2: &Position) -> bool {
        let line = PositionVector::from_line(*p1, *p2);
        let (xmin, xmax, ymin, ymax) = (self.xmin, self.xmax, self.ymin, self.ymax);
        line.intersects(&Position::new(xmax, ymax), &Position::new(xmin, ymax))
            || line.intersects(&Position::new(xmin, ymax), &Position::new(xmin, ymin))
            || line.intersects(&Position::new(xmin, ymin), &Position::new(xmax, ymin))
            || line.intersects(&Position::new(xmax, ymin), &Position::new(xmax, ymax))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boundary() {
        let mut b = Boundary::from_xy(0., 0., 10., 5.);
        assert_eq!(b.to_string_prec(2), "0.00,0.00,10.00,5.00");
        assert!(b.around(&Position::new(10., 5.), 0.));
        assert!(!b.around(&Position::new(10.1, 5.), 0.));
        assert_eq!(b.distance_to2d(&Position::new(13., 9.)), 5.);
        assert!(b.crosses(&Position::new(-1., 2.), &Position::new(1., 2.)));
        assert!(!b.crosses(&Position::new(1., 1.), &Position::new(2., 2.)));
        b.flip_y();
        assert_eq!((b.ymin(), b.ymax()), (-5., -0.));
    }
}
