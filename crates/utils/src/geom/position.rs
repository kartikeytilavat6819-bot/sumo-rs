//! Port of utils/geom/Position.{h,cpp}
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use super::POSITION_EPS;
use crate::common::std_defs::{g_precision, round_decimal_to_even};
use crate::common::to_string::f64_to_string_prec;

/// A point in 2D or 3D with translation and scaling methods.
/// `PartialOrd` is the lexicographic C++ `operator<`.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Position {
    x: f64,
    y: f64,
    z: f64,
}

impl Position {
    /// Position 1Mio km below the surface should suffice for signaling invalidity inside the solar system
    pub const INVALID: Position = Position::new3(-1073741824., -1073741824., -1073741824.);

    pub const fn new(x: f64, y: f64) -> Self {
        Position { x, y, z: 0. }
    }
    pub const fn new3(x: f64, y: f64, z: f64) -> Self {
        Position { x, y, z }
    }
    #[inline]
    pub fn x(&self) -> f64 {
        self.x
    }
    #[inline]
    pub fn y(&self) -> f64 {
        self.y
    }
    #[inline]
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn setx(&mut self, x: f64) {
        self.x = x;
    }
    pub fn sety(&mut self, y: f64) {
        self.y = y;
    }
    pub fn setz(&mut self, z: f64) {
        self.z = z;
    }
    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
    pub fn set3(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
    pub fn mul(&mut self, val: f64) {
        self.x *= val;
        self.y *= val;
        self.z *= val;
    }
    pub fn div(&mut self, val: f64) {
        self.x /= val;
        self.y /= val;
        self.z /= val;
    }
    pub fn mul_xy(&mut self, mx: f64, my: f64) {
        self.x *= mx;
        self.y *= my;
    }
    pub fn mul_xyz(&mut self, mx: f64, my: f64, mz: f64) {
        self.x *= mx;
        self.y *= my;
        self.z *= mz;
    }
    pub fn add(&mut self, pos: &Position) {
        self.x += pos.x;
        self.y += pos.y;
        self.z += pos.z;
    }
    pub fn add_xy(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
    pub fn add_xyz(&mut self, dx: f64, dy: f64, dz: f64) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }
    pub fn sub(&mut self, pos: &Position) {
        self.x -= pos.x;
        self.y -= pos.y;
        self.z -= pos.z;
    }
    pub fn sub_xy(&mut self, dx: f64, dy: f64) {
        self.x -= dx;
        self.y -= dy;
    }
    pub fn sub_xyz(&mut self, dx: f64, dy: f64, dz: f64) {
        self.x -= dx;
        self.y -= dy;
        self.z -= dz;
    }
    #[inline]
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    #[inline]
    pub fn length2d(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn norm2d(&mut self) {
        let val = self.length2d();
        if val != 0. {
            self.x /= val;
            self.y /= val;
        }
    }
    /// check whether the other position has a euclidean distance of less than max_div (C++ default POSITION_EPS)
    pub fn almost_same(&self, p2: &Position, max_div: f64) -> bool {
        self.distance_to(p2) < max_div
    }
    pub fn almost_same_eps(&self, p2: &Position) -> bool {
        self.almost_same(p2, POSITION_EPS)
    }
    #[inline]
    pub fn distance_to(&self, p2: &Position) -> f64 {
        self.distance_squared_to(p2).sqrt()
    }
    #[inline]
    pub fn distance_squared_to(&self, p2: &Position) -> f64 {
        (self.x - p2.x) * (self.x - p2.x) + (self.y - p2.y) * (self.y - p2.y) + (self.z - p2.z) * (self.z - p2.z)
    }
    #[inline]
    pub fn distance_to2d(&self, p2: &Position) -> f64 {
        self.distance_squared_to2d(p2).sqrt()
    }
    #[inline]
    pub fn distance_squared_to2d(&self, p2: &Position) -> f64 {
        (self.x - p2.x) * (self.x - p2.x) + (self.y - p2.y) * (self.y - p2.y)
    }
    /// angle in the plane of the vector pointing from here to the other position (radians, -PI..PI)
    #[inline]
    pub fn angle_to2d(&self, other: &Position) -> f64 {
        (other.y - self.y).atan2(other.x - self.x)
    }
    #[inline]
    pub fn slope_to2d(&self, other: &Position) -> f64 {
        (other.z - self.z).atan2(self.distance_to2d(other))
    }
    pub fn cross_product(&self, pos: &Position) -> Position {
        Position::new3(
            self.y * pos.z - self.z * pos.y,
            self.z * pos.x - self.x * pos.z,
            self.x * pos.y - self.y * pos.x,
        )
    }
    #[inline]
    pub fn dot_product(&self, pos: &Position) -> f64 {
        self.x * pos.x + self.y * pos.y + self.z * pos.z
    }
    /// rotate this position by rad around origin and return the result (z is dropped, as in C++)
    pub fn rotate_around2d(&self, rad: f64, origin: &Position) -> Position {
        let s = rad.sin();
        let c = rad.cos();
        let p = *self - *origin;
        Position::new(p.x * c - p.y * s, p.x * s + p.y * c) + *origin
    }
    pub fn swap_xy(&mut self) {
        std::mem::swap(&mut self.x, &mut self.y);
    }
    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }
    /// round all coordinates to the given precision (bankers rounding as in iostream)
    pub fn round(&mut self, precision: i32) {
        self.x = round_decimal_to_even(self.x, precision);
        self.y = round_decimal_to_even(self.y, precision);
        self.z = round_decimal_to_even(self.z, precision);
    }
    /// C++ `os << pos` with fixed notation and the given precision
    pub fn to_string_prec(&self, precision: i32) -> String {
        let mut s = format!(
            "{},{}",
            f64_to_string_prec(self.x + 0.0, precision),
            f64_to_string_prec(self.y + 0.0, precision)
        );
        if self.z != 0.0 {
            s.push(',');
            s.push_str(&f64_to_string_prec(self.z, precision));
        }
        s
    }
}

/// Formats like the C++ stream operator in an OutputDevice: fixed, gPrecision decimals
/// (or `{:.N}` precision), z only if non-zero.
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prec = f.precision().map(|p| p as i32).unwrap_or_else(g_precision);
        f.write_str(&self.to_string_prec(prec))
    }
}

impl Add for Position {
    type Output = Position;
    fn add(self, p2: Position) -> Position {
        Position::new3(self.x + p2.x, self.y + p2.y, self.z + p2.z)
    }
}
impl Sub for Position {
    type Output = Position;
    fn sub(self, p2: Position) -> Position {
        Position::new3(self.x - p2.x, self.y - p2.y, self.z - p2.z)
    }
}
impl Mul<f64> for Position {
    type Output = Position;
    fn mul(self, scalar: f64) -> Position {
        Position::new3(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}
impl Div<f64> for Position {
    type Output = Position;
    fn div(self, scalar: f64) -> Position {
        Position::new3(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}
/// keep the direction but modify the length of the (location) vector to length + offset
impl Add<f64> for Position {
    type Output = Position;
    fn add(self, offset: f64) -> Position {
        let length = self.distance_to(&Position::new3(0., 0., 0.));
        if length == 0. {
            return self;
        }
        let scalar = (length + offset) / length;
        Position::new3(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}
/// keep the direction but modify the length of the (location) vector to length - offset
impl Sub<f64> for Position {
    type Output = Position;
    fn sub(self, offset: f64) -> Position {
        let length = self.distance_to(&Position::new3(0., 0., 0.));
        if length == 0. {
            return self;
        }
        let scalar = (length - offset) / length;
        Position::new3(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn format_and_ops() {
        assert_eq!(Position::new(-0.0, 1.005).to_string_prec(2), "0.00,1.00");
        assert_eq!(Position::new3(1., 2., 3.).to_string_prec(2), "1.00,2.00,3.00");
        assert_eq!(format!("{:.1}", Position::new(1.26, 2.)), "1.3,2.0");
        assert_eq!((Position::new(3., 4.) + 5.), Position::new(6., 8.));
        assert!(Position::new(1., 2.) < Position::new(1., 3.));
        let r = Position::new(1., 0.).rotate_around2d(std::f64::consts::FRAC_PI_2, &Position::new(0., 0.));
        assert!((r.x()).abs() < 1e-15 && (r.y() - 1.).abs() < 1e-15);
    }
}
