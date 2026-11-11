//! Port of utils/geom/PositionVector.{h,cpp}
//!
//! C++ overloads get suffixed names: static segment variants end in `_seg`,
//! PositionVector-argument variants in `_pv`. C++ default arguments must be passed explicitly.
use std::cmp::Ordering;
use std::fmt;
use std::ops::{Deref, DerefMut};

use super::geom_helper::{self, deg2rad, rad2deg, INVALID_OFFSET, M_PI};
use super::{max2, min2, write_warning, AbstractPoly, Boundary, InvalidArgument, Position, NUMERICAL_EPS, POSITION_EPS};
use crate::common::std_defs::{g_precision, INVALID_DOUBLE};
use crate::common::to_string::f64_to_string;

/// A list of positions. Derefs to `Vec<Position>` for push/len/iter/insert/...;
/// python-style signed indexing (C++ `operator[]`) is [`PositionVector::at`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PositionVector(pub Vec<Position>);

impl Deref for PositionVector {
    type Target = Vec<Position>;
    fn deref(&self) -> &Vec<Position> {
        &self.0
    }
}
impl DerefMut for PositionVector {
    fn deref_mut(&mut self) -> &mut Vec<Position> {
        &mut self.0
    }
}
impl From<Vec<Position>> for PositionVector {
    fn from(v: Vec<Position>) -> Self {
        PositionVector(v)
    }
}
impl FromIterator<Position> for PositionVector {
    fn from_iter<I: IntoIterator<Item = Position>>(iter: I) -> Self {
        PositionVector(iter.into_iter().collect())
    }
}

/// segment intersection result of the private static C++ `intersects` (x, y, mu)
type Intersection = (f64, f64, f64);

impl PositionVector {
    pub const EMPTY: PositionVector = PositionVector(Vec::new());

    pub const fn new() -> Self {
        PositionVector(Vec::new())
    }
    /// Parameter Constructor used for lines
    pub fn from_line(p1: Position, p2: Position) -> Self {
        PositionVector(vec![p1, p2])
    }

    /// returns the position at the given index, negative indices are interpreted python style
    /// (C++ throws OutOfBoundsException; this panics)
    pub fn at(&self, index: i32) -> &Position {
        &self.0[self.py_index(index)]
    }
    pub fn at_mut(&mut self, index: i32) -> &mut Position {
        let i = self.py_index(index);
        &mut self.0[i]
    }
    fn py_index(&self, index: i32) -> usize {
        let n = self.0.len() as i32;
        if index >= 0 && index < n {
            index as usize
        } else if index < 0 && -index <= n {
            (n + index) as usize
        } else {
            panic!("Index out of range in bracket operator of PositionVector");
        }
    }
    /// C++ front() (panics when empty)
    pub fn front(&self) -> &Position {
        &self.0[0]
    }
    /// C++ back() (panics when empty)
    pub fn back(&self) -> &Position {
        &self.0[self.0.len() - 1]
    }
    pub fn back_mut(&mut self) -> &mut Position {
        let n = self.0.len();
        &mut self.0[n - 1]
    }

    /// number of segments (C++ loops `i != end() - 1`)
    fn nseg(&self) -> usize {
        self.0.len().saturating_sub(1)
    }

    pub fn get_overlap_with(&self, poly: &PositionVector, z_threshold: f64) -> f64 {
        let mut result = 0.;
        if self.is_empty() || poly.is_empty() {
            return result;
        }
        // this points within poly
        for i in &self.0[..self.nseg()] {
            if poly.around(i, 0.) {
                let closest = poly.position_at_offset2d(poly.nearest_offset_to_point2d(i, true), 0., false);
                if (closest.z() - i.z()).abs() < z_threshold {
                    result = max2(result, poly.distance2d(i, false));
                }
            }
        }
        // polys points within this
        for i in &poly.0[..poly.nseg()] {
            if self.around(i, 0.) {
                let closest = self.position_at_offset2d(self.nearest_offset_to_point2d(i, true), 0., false);
                if (closest.z() - i.z()).abs() < z_threshold {
                    result = max2(result, self.distance2d(i, false));
                }
            }
        }
        result
    }

    /// whether this list of points intersects the given line
    pub fn intersects(&self, p1: &Position, p2: &Position) -> bool {
        if self.len() < 2 {
            return false;
        }
        (0..self.nseg()).any(|i| Self::intersects_seg(&self[i], &self[i + 1], p1, p2, 0.).is_some())
    }

    /// whether this list of points intersects one of the given lines
    pub fn intersects_pv(&self, v1: &PositionVector) -> bool {
        if self.len() < 2 {
            return false;
        }
        (0..self.nseg()).any(|i| v1.intersects(&self[i], &self[i + 1]))
    }

    /// Returns the position of the intersection (C++ default within_dist = 0)
    pub fn intersection_position2d(&self, p1: &Position, p2: &Position, within_dist: f64) -> Position {
        for i in 0..self.nseg() {
            if let Some((x, y, _)) = Self::intersects_seg(&self[i], &self[i + 1], p1, p2, within_dist) {
                return Position::new(x, y);
            }
        }
        Position::INVALID
    }

    pub fn intersection_position2d_pv(&self, v1: &PositionVector) -> Position {
        for i in 0..self.nseg() {
            if v1.intersects(&self[i], &self[i + 1]) {
                return v1.intersection_position2d(&self[i], &self[i + 1], 0.);
            }
        }
        Position::INVALID
    }

    /// For all intersections between this vector and other, the 2D-length from this vector's start
    pub fn intersects_at_lengths2d_pv(&self, other: &PositionVector) -> Vec<f64> {
        let mut ret = Vec::new();
        for i in 0..other.nseg() {
            ret.extend(self.intersects_at_lengths2d(&other[i], &other[i + 1]));
        }
        ret
    }

    pub fn intersects_at_lengths2d(&self, lp1: &Position, lp2: &Position) -> Vec<f64> {
        let mut ret = Vec::new();
        let mut pos = 0.;
        for i in 0..self.nseg() {
            let p1 = &self[i];
            let p2 = &self[i + 1];
            if let Some((x, y, _)) = Self::intersects_seg(p1, p2, lp1, lp2, 0.) {
                ret.push(Position::new(x, y).distance_to2d(p1) + pos);
            }
            pos += p1.distance_to2d(p2);
        }
        ret
    }

    pub fn open_polygon(&mut self) {
        if self.len() > 1 && self.front() == self.back() {
            self.pop();
        }
    }

    pub fn close_polygon(&mut self) {
        if !self.is_empty() && self[0] != *self.back() {
            let p = self[0];
            self.push(p);
        }
    }

    /// Returns the position at the given length (C++ default lateral_offset = 0)
    pub fn position_at_offset(&self, pos: f64, lateral_offset: f64) -> Position {
        if self.is_empty() {
            return Position::INVALID;
        }
        if self.len() == 1 {
            return *self.front();
        }
        let mut seen_length = 0.;
        for i in 0..self.nseg() {
            let next_length = self[i].distance_to(&self[i + 1]);
            if seen_length + next_length > pos {
                return Self::position_at_offset_seg(&self[i], &self[i + 1], pos - seen_length, lateral_offset);
            }
            seen_length += next_length;
        }
        if lateral_offset == 0. || self.len() < 2 {
            *self.back()
        } else {
            let (a, b) = (self.at(-2), self.at(-1));
            Self::position_at_offset_seg(a, b, a.distance_to(b), lateral_offset)
        }
    }

    /// position similar to position_at_offset but the lateral offset is applied orthogonal to the given angle
    pub fn side_position_at_angle(&self, pos: f64, lateral_offset: f64, angle: f64) -> Position {
        if self.is_empty() {
            return Position::INVALID;
        }
        if self.len() == 1 {
            return *self.front();
        }
        let mut seen_length = 0.;
        for i in 0..self.nseg() {
            let next_length = self[i].distance_to(&self[i + 1]);
            if seen_length + next_length > pos {
                return Self::side_position_at_angle_seg(&self[i], &self[i + 1], pos - seen_length, lateral_offset, angle);
            }
            seen_length += next_length;
        }
        let (a, b) = (self.at(-2), self.at(-1));
        Self::side_position_at_angle_seg(a, b, a.distance_to(b), lateral_offset, angle)
    }

    /// C++ defaults lateral_offset = 0, extrapolate_beyond = false
    pub fn position_at_offset2d(&self, pos: f64, lateral_offset: f64, extrapolate_beyond: bool) -> Position {
        if self.is_empty() {
            return Position::INVALID;
        }
        if self.len() == 1 {
            return *self.front();
        }
        let mut seen_length = 0.;
        for i in 0..self.nseg() {
            let next_length = self[i].distance_to2d(&self[i + 1]);
            if seen_length + next_length > pos {
                return Self::position_at_offset2d_seg(&self[i], &self[i + 1], pos - seen_length, lateral_offset, extrapolate_beyond);
            }
            seen_length += next_length;
        }
        if extrapolate_beyond {
            let (a, b) = (self.at(-2), self.at(-1));
            return Self::position_at_offset2d_seg(a, b, pos - seen_length + b.distance_to2d(a), lateral_offset, extrapolate_beyond);
        }
        *self.back()
    }

    pub fn rotation_at_offset(&self, mut pos: f64) -> f64 {
        if self.len() < 2 {
            return INVALID_DOUBLE;
        }
        if pos < 0. {
            pos += self.length();
        }
        let mut seen_length = 0.;
        for i in 0..self.nseg() {
            let p1 = &self[i];
            let p2 = &self[i + 1];
            let next_length = p1.distance_to(p2);
            if seen_length + next_length > pos {
                return p1.angle_to2d(p2);
            }
            seen_length += next_length;
        }
        self.at(-2).angle_to2d(self.back())
    }

    pub fn rotation_degree_at_offset(&self, pos: f64) -> f64 {
        geom_helper::legacy_degree(self.rotation_at_offset(pos), false)
    }

    pub fn slope_degree_at_offset(&self, pos: f64) -> f64 {
        // C++ only checks for empty; a single point is undefined behaviour there
        if self.len() < 2 {
            return INVALID_DOUBLE;
        }
        let mut seen_length = 0.;
        for i in 0..self.nseg() {
            let p1 = &self[i];
            let p2 = &self[i + 1];
            let next_length = p1.distance_to(p2);
            if seen_length + next_length > pos {
                return rad2deg(p1.slope_to2d(p2));
            }
            seen_length += next_length;
        }
        rad2deg(self.at(-2).slope_to2d(self.back()))
    }

    /// Returns the position between the two given points at the specified position
    pub fn position_at_offset_seg(p1: &Position, p2: &Position, pos: f64, lateral_offset: f64) -> Position {
        let dist = p1.distance_to(p2);
        if pos < 0. || dist < pos {
            return Position::INVALID;
        }
        if lateral_offset != 0. {
            if dist == 0. {
                return Position::INVALID;
            }
            let offset = Self::side_offset(p1, p2, -lateral_offset); // move in the same direction as Position::move2side
            if pos == 0. {
                return *p1 + offset;
            }
            return *p1 + (*p2 - *p1) * (pos / dist) + offset;
        }
        if pos == 0. {
            return *p1;
        }
        *p1 + (*p2 - *p1) * (pos / dist)
    }

    pub fn side_position_at_angle_seg(p1: &Position, p2: &Position, pos: f64, lateral_offset: f64, mut angle: f64) -> Position {
        let dist = p1.distance_to(p2);
        if pos < 0. || dist < pos || dist == 0. {
            return Position::INVALID;
        }
        angle -= deg2rad(90.);
        let offset = Position::new(angle.cos() * lateral_offset, angle.sin() * lateral_offset);
        *p1 + (*p2 - *p1) * (pos / dist) + offset
    }

    pub fn position_at_offset2d_seg(p1: &Position, p2: &Position, pos: f64, lateral_offset: f64, extrapolate_beyond: bool) -> Position {
        let dist = p1.distance_to2d(p2);
        if (pos < 0. || dist < pos) && !extrapolate_beyond {
            return Position::INVALID;
        }
        if dist == 0. {
            return *p1;
        }
        if lateral_offset != 0. {
            let offset = Self::side_offset(p1, p2, -lateral_offset);
            if pos == 0. {
                return *p1 + offset;
            }
            return *p1 + (*p2 - *p1) * (pos / dist) + offset;
        }
        if pos == 0. {
            return *p1;
        }
        *p1 + (*p2 - *p1) * (pos / dist)
    }

    pub fn get_box_boundary(&self) -> Boundary {
        let mut ret = Boundary::new();
        for i in self.iter() {
            ret.add_pos(i);
        }
        ret
    }

    /// arithmetic mean of all corner points (not the centroid!)
    pub fn get_polygon_center(&self) -> Position {
        if self.is_empty() {
            return Position::INVALID;
        }
        let (mut x, mut y, mut z) = (0., 0., 0.);
        for i in self.iter() {
            x += i.x();
            y += i.y();
            z += i.z();
        }
        let n = self.len() as f64;
        Position::new3(x / n, y / n, z / n)
    }

    /// Returns the centroid (closes the polygon if unclosed)
    pub fn get_centroid(&self) -> Position {
        match self.len() {
            0 => return Position::INVALID,
            1 => return self[0],
            2 => return (self[0] + self[1]) * 0.5,
            _ => {}
        }
        let mut tmp = self.clone();
        if !self.is_closed() {
            let first = tmp[0];
            tmp.push(first);
        }
        // shift to origin to increase numerical stability
        let offset = tmp[0];
        tmp.sub(&offset);
        let end_index = tmp.len() - 1;
        let mut div = 0.; // 6 * area including sign
        let mut x = 0.;
        let mut y = 0.;
        let result = if tmp.area() != 0. {
            for i in 0..end_index {
                let z = tmp[i].x() * tmp[i + 1].y() - tmp[i + 1].x() * tmp[i].y();
                div += z; // area formula
                x += (tmp[i].x() + tmp[i + 1].x()) * z;
                y += (tmp[i].y() + tmp[i + 1].y()) * z;
            }
            div *= 3.; //  6 / 2, the 2 comes from the area formula
            Position::new(x / div, y / div)
        } else {
            // compute by decomposing into line segments
            let mut length_sum = 0.;
            for i in 0..end_index {
                let length = tmp[i].distance_to(&tmp[i + 1]);
                x += (tmp[i].x() + tmp[i + 1].x()) * length / 2.;
                y += (tmp[i].y() + tmp[i + 1].y()) * length / 2.;
                length_sum += length;
            }
            // if lengthSum == 0, it is probably only one point
            if length_sum == 0. { tmp[0] } else { Position::new(x / length_sum, y / length_sum) }
        };
        result + offset
    }

    /// enlarges/shrinks the polygon by a factor based at the centroid
    pub fn scale_relative(&mut self, factor: f64) {
        let centroid = self.get_centroid();
        for p in self.iter_mut() {
            *p = centroid + ((*p - centroid) * factor);
        }
    }

    /// enlarges/shrinks the polygon by an absolute offset based at the centroid
    pub fn scale_absolute(&mut self, offset: f64) {
        let centroid = self.get_centroid();
        for p in self.iter_mut() {
            *p = centroid + ((*p - centroid) + offset);
        }
    }

    pub fn get_line_center(&self) -> Position {
        if self.len() == 1 { self[0] } else { self.position_at_offset(self.length() / 2., 0.) }
    }

    pub fn length(&self) -> f64 {
        let mut len = 0.;
        for i in 0..self.nseg() {
            len += self[i].distance_to(&self[i + 1]);
        }
        len
    }

    pub fn length2d(&self) -> f64 {
        let mut len = 0.;
        for i in 0..self.nseg() {
            len += self[i].distance_to2d(&self[i + 1]);
        }
        len
    }

    /// Returns the area (0 for less than three points)
    pub fn area(&self) -> f64 {
        if self.len() < 3 {
            return 0.;
        }
        let mut area = 0.;
        let mut tmp = self.clone();
        if !self.is_closed() {
            let first = tmp[0];
            tmp.push(first);
        }
        for i in 0..tmp.len() - 1 {
            area += tmp[i].x() * tmp[i + 1].y() - tmp[i + 1].x() * tmp[i].y();
        }
        if area < 0. {
            area *= -1.;
        }
        area / 2.
    }

    /// Returns the two lists made when this list vector is splitted at the given point
    pub fn split_at(&self, where_: f64, use2d: bool) -> Result<(PositionVector, PositionVector), InvalidArgument> {
        let len = if use2d { self.length2d() } else { self.length() };
        if self.len() < 2 {
            return Err(InvalidArgument("Vector to short for splitting".into()));
        }
        if where_ < 0. || where_ > len {
            return Err(InvalidArgument(format!(
                "Invalid split position {} for vector of length {}",
                f64_to_string(where_),
                f64_to_string(len)
            )));
        }
        if where_ <= POSITION_EPS || where_ >= len - POSITION_EPS {
            write_warning(&format!(
                "Splitting vector close to end (pos: {}, length: {})",
                f64_to_string(where_),
                f64_to_string(len)
            ));
        }
        let dist = |a: &Position, b: &Position| if use2d { a.distance_to2d(b) } else { a.distance_to(b) };
        let mut first = PositionVector::new();
        let mut second = PositionVector::new();
        first.push(self[0]);
        let mut seen = 0.;
        let mut it = 1;
        let mut next = dist(first.back(), &self[it]);
        // see how many points we can add to first
        while where_ >= seen + next + POSITION_EPS {
            seen += next;
            first.push(self[it]);
            it += 1;
            next = dist(first.back(), &self[it]);
        }
        if (where_ - (seen + next)).abs() > POSITION_EPS || it == self.len() - 1 {
            // we need to insert a new point because 'where' is not close to an
            // existing point or it is to close to the endpoint
            let p = if use2d {
                Self::position_at_offset2d_seg(first.back(), &self[it], where_ - seen, 0., false)
            } else {
                Self::position_at_offset_seg(first.back(), &self[it], where_ - seen, 0.)
            };
            first.push(p);
            second.push(p);
        } else {
            first.push(self[it]);
        }
        // add the remaining points to second
        second.extend_from_slice(&self[it..]);
        debug_assert!(first.len() >= 2 && second.len() >= 2);
        Ok((first, second))
    }

    /// C++ `os << geom` with fixed notation and the given precision
    pub fn to_string_prec(&self, precision: i32) -> String {
        self.iter().map(|p| p.to_string_prec(precision)).collect::<Vec<_>>().join(" ")
    }

    /// sort as polygon CW by angle
    pub fn sort_as_poly_cw_by_angle(&mut self) {
        let centroid = self.iter().fold(Position::new(0., 0.), |a, b| a + *b) / self.len() as f64;
        self.sub(&centroid);
        self.0.sort_by(|p1, p2| ordering_from_less(as_poly_cw_less, p1, p2));
        self.add(&centroid);
    }

    /// sort by increasing X-Y Positions
    pub fn sort_by_increasing_xy(&mut self) {
        self.0.sort_by(|p1, p2| ordering_from_less(increasing_x_y_less, p1, p2));
    }

    pub fn add_xyz(&mut self, xoff: f64, yoff: f64, zoff: f64) {
        for p in self.iter_mut() {
            p.add_xyz(xoff, yoff, zoff);
        }
    }
    pub fn add(&mut self, offset: &Position) {
        self.add_xyz(offset.x(), offset.y(), offset.z());
    }
    pub fn sub(&mut self, offset: &Position) {
        self.add_xyz(-offset.x(), -offset.y(), -offset.z());
    }
    /// adds a position without modifying the vector itself but returning the result
    pub fn added(&self, offset: &Position) -> PositionVector {
        self.iter().map(|p| *p + *offset).collect()
    }
    /// mirror coordinates along the x-axis
    pub fn mirror_x(&mut self) {
        for p in self.iter_mut() {
            p.mul_xy(1., -1.);
        }
    }

    /// signed area of the triangle; > 0 if p2 is left of the line p0->p1
    pub fn is_left(p0: &Position, p1: &Position, p2: &Position) -> f64 {
        (p1.x() - p0.x()) * (p2.y() - p0.y()) - (p2.x() - p0.x()) * (p1.y() - p0.y())
    }

    /// append the given vector to this one (C++ default same_threshold = 2.0)
    pub fn append(&mut self, v: &PositionVector, same_threshold: f64) {
        if !self.is_empty() && !v.is_empty() && self.back().distance_to(&v[0]) < same_threshold {
            self.extend_from_slice(&v[1..]);
        } else {
            self.extend_from_slice(v);
        }
    }

    /// prepend the given vector to this one (C++ default same_threshold = 2.0)
    pub fn prepend(&mut self, v: &PositionVector, same_threshold: f64) {
        let take = if !self.is_empty() && !v.is_empty() && self.front().distance_to(v.back()) < same_threshold {
            v.len() - 1
        } else {
            v.len()
        };
        self.0.splice(0..0, v[..take].iter().copied());
    }

    pub fn get_subpart(&self, begin_offset: f64, end_offset: f64) -> PositionVector {
        self.subpart(begin_offset, end_offset, false)
    }

    pub fn get_subpart2d(&self, begin_offset: f64, end_offset: f64) -> PositionVector {
        self.subpart(begin_offset, end_offset, true)
    }

    fn subpart(&self, begin_offset: f64, end_offset: f64, two_d: bool) -> PositionVector {
        if self.is_empty() {
            // C++ getSubpart (3D) is undefined behaviour here
            return PositionVector::new();
        }
        let dist = |a: &Position, b: &Position| if two_d { a.distance_to2d(b) } else { a.distance_to(b) };
        let at_offset = |o: f64| if two_d { self.position_at_offset2d(o, 0., false) } else { self.position_at_offset(o, 0.) };
        let mut ret = PositionVector::new();
        let mut beg_pos = *self.front();
        if begin_offset > POSITION_EPS {
            beg_pos = at_offset(begin_offset);
        }
        let mut end_pos = *self.back();
        let length = if two_d { self.length2d() } else { self.length() };
        if end_offset < length - POSITION_EPS {
            end_pos = at_offset(end_offset);
        }
        ret.push(beg_pos);

        let mut seen = 0.;
        let mut i = 0;
        let n = self.len();
        // skip previous segments
        while i + 1 != n && seen + dist(&self[i], &self[i + 1]) < begin_offset {
            seen += dist(&self[i], &self[i + 1]);
            i += 1;
        }
        // append segments in between
        while i + 1 != n && seen + dist(&self[i], &self[i + 1]) < end_offset {
            ret.push_back_no_double_pos(&self[i + 1]);
            seen += dist(&self[i], &self[i + 1]);
            i += 1;
        }
        // append end
        ret.push_back_no_double_pos(&end_pos);
        if ret.len() == 1 {
            ret.push(end_pos);
        }
        ret
    }

    pub fn get_subpart_by_index(&self, mut begin_index: i32, count: i32) -> PositionVector {
        if self.is_empty() {
            return PositionVector::new();
        }
        if begin_index < 0 {
            begin_index += self.len() as i32;
        }
        debug_assert!(count >= 0 && begin_index < self.len() as i32 && begin_index + count <= self.len() as i32);
        (begin_index..begin_index + count).map(|i| *self.at(i)).collect()
    }

    /// angle in radians of the line connecting the first and the last position
    pub fn begin_end_angle(&self) -> f64 {
        if self.is_empty() {
            return INVALID_DOUBLE;
        }
        self.front().angle_to2d(self.back())
    }

    /// return the nearest offset to point 2D (C++ default perpendicular = true)
    pub fn nearest_offset_to_point2d(&self, p: &Position, perpendicular: bool) -> f64 {
        if self.is_empty() {
            return INVALID_DOUBLE;
        }
        let mut min_dist = f64::MAX;
        let mut nearest_pos = INVALID_OFFSET;
        let mut seen = 0.;
        for i in 0..self.nseg() {
            let pos = geom_helper::nearest_offset_on_line_to_point2d(&self[i], &self[i + 1], p, perpendicular);
            let dist2 = if pos == INVALID_OFFSET {
                min_dist
            } else {
                p.distance_squared_to2d(&Self::position_at_offset2d_seg(&self[i], &self[i + 1], pos, 0., false))
            };
            if dist2 < min_dist {
                nearest_pos = pos + seen;
                min_dist = dist2;
            }
            if perpendicular && i != 0 && pos == INVALID_OFFSET {
                // even if perpendicular is set we still need to check the distance to the inner points
                let corner_dist2 = p.distance_squared_to2d(&self[i]);
                if corner_dist2 < min_dist {
                    let pos1 = geom_helper::nearest_offset_on_line_to_point2d(&self[i - 1], &self[i], p, false);
                    let pos2 = geom_helper::nearest_offset_on_line_to_point2d(&self[i], &self[i + 1], p, false);
                    if pos1 == self[i - 1].distance_to2d(&self[i]) && pos2 == 0. {
                        nearest_pos = seen;
                        min_dist = corner_dist2;
                    }
                }
            }
            seen += self[i].distance_to2d(&self[i + 1]);
        }
        nearest_pos
    }

    /// return the nearest offset to point 2D projected onto the 3D geometry
    pub fn nearest_offset_to_point25d(&self, p: &Position, perpendicular: bool) -> f64 {
        if self.is_empty() {
            return INVALID_DOUBLE;
        }
        let mut min_dist = f64::MAX;
        let mut nearest_pos = INVALID_OFFSET;
        let mut seen = 0.;
        for i in 0..self.nseg() {
            let pos = geom_helper::nearest_offset_on_line_to_point2d(&self[i], &self[i + 1], p, perpendicular);
            let dist = if pos == INVALID_OFFSET {
                min_dist
            } else {
                p.distance_to2d(&Self::position_at_offset2d_seg(&self[i], &self[i + 1], pos, 0., false))
            };
            if dist < min_dist {
                let pos25d = pos * self[i].distance_to(&self[i + 1]) / self[i].distance_to2d(&self[i + 1]);
                nearest_pos = pos25d + seen;
                min_dist = dist;
            }
            if perpendicular && i != 0 && pos == INVALID_OFFSET {
                let corner_dist = p.distance_to2d(&self[i]);
                if corner_dist < min_dist {
                    let pos1 = geom_helper::nearest_offset_on_line_to_point2d(&self[i - 1], &self[i], p, false);
                    let pos2 = geom_helper::nearest_offset_on_line_to_point2d(&self[i], &self[i + 1], p, false);
                    if pos1 == self[i - 1].distance_to2d(&self[i]) && pos2 == 0. {
                        nearest_pos = seen;
                        min_dist = corner_dist;
                    }
                }
            }
            seen += self[i].distance_to(&self[i + 1]);
        }
        nearest_pos
    }

    /// return position p within the length-wise coordinate system defined by this vector:
    /// x is the offset (as nearest_offset_to_point2d), y the perpendicular distance (right is positive).
    /// If extend is true the vector is extended on both sides. (C++ default extend = false)
    pub fn transform_to_vector_coordinates(&self, p: &Position, extend: bool) -> Position {
        if self.is_empty() {
            return Position::INVALID;
        }
        if extend {
            let mut extended = self.clone();
            let dist = 2. * self.distance2d(p, false);
            extended.extrapolate(dist, false, false);
            return extended.transform_to_vector_coordinates(p, false) - Position::new(dist, 0.);
        }
        let mut min_dist = f64::MAX;
        let mut nearest_pos = -1.;
        let mut seen = 0.;
        let mut sign = 1;
        for i in 0..self.nseg() {
            let pos = geom_helper::nearest_offset_on_line_to_point2d(&self[i], &self[i + 1], p, true);
            let dist = if pos < 0. {
                min_dist
            } else {
                p.distance_to2d(&Self::position_at_offset_seg(&self[i], &self[i + 1], pos, 0.))
            };
            if dist < min_dist {
                nearest_pos = pos + seen;
                min_dist = dist;
                sign = if Self::is_left(&self[i], &self[i + 1], p) >= 0. { -1 } else { 1 };
            }
            if i != 0 && pos == INVALID_OFFSET {
                let corner_dist = p.distance_to2d(&self[i]);
                if corner_dist < min_dist {
                    let pos1 = geom_helper::nearest_offset_on_line_to_point2d(&self[i - 1], &self[i], p, false);
                    let pos2 = geom_helper::nearest_offset_on_line_to_point2d(&self[i], &self[i + 1], p, false);
                    if pos1 == self[i - 1].distance_to2d(&self[i]) && pos2 == 0. {
                        nearest_pos = seen;
                        min_dist = corner_dist;
                        sign = if Self::is_left(&self[i - 1], &self[i], p) >= 0. { -1 } else { 1 };
                    }
                }
            }
            seen += self[i].distance_to2d(&self[i + 1]);
        }
        if nearest_pos != -1. {
            Position::new(nearest_pos, sign as f64 * min_dist)
        } else {
            Position::INVALID
        }
    }

    /// index of the closest position to p (-1 if empty). C++ default two_d = false
    pub fn index_of_closest(&self, p: &Position, two_d: bool) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let mut min_dist = f64::MAX;
        let mut closest = 0;
        for (i, p2) in self.iter().enumerate() {
            let dist = if two_d { p.distance_to2d(p2) } else { p.distance_to(p2) };
            if dist < min_dist {
                closest = i as i32;
                min_dist = dist;
            }
        }
        closest
    }

    /// inserts p between the two closest positions and returns the insertion index
    pub fn insert_at_closest(&mut self, p: &Position, interpolate_z: bool) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let mut min_dist = f64::MAX;
        let mut insertion_index = 1;
        for i in 0..self.nseg() {
            let length = geom_helper::nearest_offset_on_line_to_point2d(&self[i], &self[i + 1], p, false);
            let out_intersection = Self::position_at_offset2d_seg(&self[i], &self[i + 1], length, 0., false);
            let dist = p.distance_to2d(&out_intersection);
            if dist < min_dist {
                insertion_index = i + 1;
                min_dist = dist;
            }
        }
        if interpolate_z {
            let previous_z = self[insertion_index - 1].z();
            let next_z = self[insertion_index].z();
            self.insert(insertion_index, Position::new3(p.x(), p.y(), (previous_z + next_z) / 2.0));
        } else {
            self.insert(insertion_index, *p);
        }
        insertion_index as i32
    }

    /// removes the point closest to p and return the removal index
    pub fn remove_closest(&mut self, p: &Position) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let mut min_dist = f64::MAX;
        let mut removal_index = 0;
        for (i, q) in self.iter().enumerate() {
            let dist = p.distance_to2d(q);
            if dist < min_dist {
                removal_index = i;
                min_dist = dist;
            }
        }
        self.remove(removal_index);
        removal_index as i32
    }

    /// C++ defaults only_first = false, only_last = false
    pub fn extrapolate(&mut self, val: f64, only_first: bool, only_last: bool) {
        if self.len() > 1 {
            let (p1, p2) = (self[0], self[1]);
            let offset = (p2 - p1) * (val / p1.distance_to(&p2));
            if !only_last {
                self[0].sub(&offset);
            }
            if !only_first {
                if self.len() == 2 {
                    self[1].add(&offset);
                } else {
                    let (e1, e2) = (*self.at(-2), *self.at(-1));
                    self.at_mut(-1).sub(&((e1 - e2) * (val / e1.distance_to(&e2))));
                }
            }
        }
    }

    /// C++ default only_first = false
    pub fn extrapolate2d(&mut self, val: f64, only_first: bool) {
        if self.len() > 1 {
            let (p1, p2) = (self[0], self[1]);
            if p1.distance_to2d(&p2) > 0. {
                let offset = (p2 - p1) * (val / p1.distance_to2d(&p2));
                self[0].sub(&offset);
                if !only_first {
                    if self.len() == 2 {
                        self[1].add(&offset);
                    } else {
                        let (e1, e2) = (*self.at(-2), *self.at(-1));
                        self.at_mut(-1).sub(&((e1 - e2) * (val / e1.distance_to2d(&e2))));
                    }
                }
            }
        }
    }

    /// returns a reversed copy (shadows the in-place slice reverse)
    pub fn reverse(&self) -> PositionVector {
        self.iter().rev().copied().collect()
    }

    /// get a side position of position vector using a offset
    pub fn side_offset(beg: &Position, end: &Position, amount: f64) -> Position {
        let scale = amount / beg.distance_to2d(end);
        Position::new((beg.y() - end.y()) * scale, (end.x() - beg.x()) * scale)
    }

    /// move position vector to side using certain amount (C++ default max_extension = 100)
    pub fn move2side(&mut self, amount: f64, max_extension: f64) {
        if self.len() < 2 {
            return;
        }
        self.remove_double_points(POSITION_EPS, true, 0, 0, false);
        if self.length2d() == 0. || amount == 0. {
            return;
        }
        let mut shape = PositionVector::new();
        let mut recheck: Vec<usize> = Vec::new();
        let n = self.len();
        for i in 0..n {
            if i == 0 {
                let (from, to) = (&self[i], &self[i + 1]);
                if from != to {
                    shape.push(*from - Self::side_offset(from, to, amount));
                }
            } else if i == n - 1 {
                let (from, to) = (&self[i - 1], &self[i]);
                if from != to {
                    shape.push(*to - Self::side_offset(from, to, amount));
                }
            } else {
                let (from, me, to) = (self[i - 1], self[i], self[i + 1]);
                let mut from_me = PositionVector::from_line(from, me);
                from_me.extrapolate2d(me.distance_to2d(&to), false);
                let extrapolate_dev = from_me[1].distance_to2d(&to);
                if extrapolate_dev.abs() < POSITION_EPS {
                    // parallel case, just shift the middle point
                    shape.push(me - Self::side_offset(&from, &to, amount));
                } else if (extrapolate_dev - 2. * me.distance_to2d(&to)).abs() < POSITION_EPS {
                    // counterparallel case, just shift the middle point
                    let mut from_me2 = PositionVector::from_line(from, me);
                    from_me2.extrapolate2d(amount, false);
                    shape.push(from_me2[1]);
                } else {
                    let offsets = Self::side_offset(&from, &me, amount);
                    let offsets2 = Self::side_offset(&me, &to, amount);
                    let l1 = PositionVector::from_line(from - offsets, me - offsets);
                    let l2 = PositionVector::from_line(me - offsets2, to - offsets2);
                    let mut me_new = l1.intersection_position2d(&l2[0], &l2[1], max_extension);
                    if me_new == Position::INVALID {
                        recheck.push(i);
                        continue;
                    }
                    me_new = me_new + Position::new3(0., 0., me.z());
                    shape.push(me_new);
                }
                // copy original z value
                let b = shape.back_mut();
                b.set3(b.x(), b.y(), me.z());
                let angle = Self::local_angle(&from, &me, &to);
                if angle.abs() > NUMERICAL_EPS {
                    let length = from.distance_to2d(&me) + me.distance_to2d(&to);
                    let radius = length / angle;
                    if (radius < 0. && -radius < amount * 1.8) || rad2deg(angle).abs() > 170. {
                        recheck.push(i);
                    }
                }
            }
        }
        if !recheck.is_empty() {
            // try to adjust positions to avoid clipping
            shape = self.clone();
            for &i in recheck.iter().rev() {
                shape.remove(i);
            }
            shape.move2side(amount, max_extension);
        }
        *self = shape;
    }

    /// move position vector to side using a custom offset for each geometry point
    pub fn move2side_custom(&mut self, amount: &[f64], max_extension: f64) -> Result<(), InvalidArgument> {
        if self.len() < 2 {
            return Ok(());
        }
        if self.length2d() == 0. {
            return Ok(());
        }
        if self.len() != amount.len() {
            return Err(InvalidArgument(format!(
                "Numer of offsets ({}) does not match number of points ({})",
                amount.len(),
                self.len()
            )));
        }
        let mut shape = PositionVector::new();
        let n = self.len();
        for i in 0..n {
            if i == 0 {
                let (from, to) = (&self[i], &self[i + 1]);
                if from != to {
                    shape.push(*from - Self::side_offset(from, to, amount[i]));
                }
            } else if i == n - 1 {
                let (from, to) = (&self[i - 1], &self[i]);
                if from != to {
                    shape.push(*to - Self::side_offset(from, to, amount[i]));
                }
            } else {
                let (from, me, to) = (self[i - 1], self[i], self[i + 1]);
                let mut from_me = PositionVector::from_line(from, me);
                from_me.extrapolate2d(me.distance_to2d(&to), false);
                let extrapolate_dev = from_me[1].distance_to2d(&to);
                if extrapolate_dev.abs() < POSITION_EPS {
                    shape.push(me - Self::side_offset(&from, &to, amount[i]));
                } else if (extrapolate_dev - 2. * me.distance_to2d(&to)).abs() < POSITION_EPS {
                    let mut from_me2 = PositionVector::from_line(from, me);
                    from_me2.extrapolate2d(amount[i], false);
                    shape.push(from_me2[1]);
                } else {
                    let offsets = Self::side_offset(&from, &me, amount[i]);
                    let offsets2 = Self::side_offset(&me, &to, amount[i]);
                    let l1 = PositionVector::from_line(from - offsets, me - offsets);
                    let l2 = PositionVector::from_line(me - offsets2, to - offsets2);
                    let mut me_new = l1.intersection_position2d(&l2[0], &l2[1], max_extension);
                    if me_new == Position::INVALID {
                        continue;
                    }
                    me_new = me_new + Position::new3(0., 0., me.z());
                    shape.push(me_new);
                }
                let b = shape.back_mut();
                b.set3(b.x(), b.y(), me.z());
            }
        }
        *self = shape;
        Ok(())
    }

    pub fn local_angle(from: &Position, pos: &Position, to: &Position) -> f64 {
        geom_helper::angle_diff(from.angle_to2d(pos), pos.angle_to2d(to))
    }

    /// angle at the given index (radians, -PI..PI)
    pub fn angle_at2d(&self, pos: i32) -> f64 {
        if pos + 1 < self.len() as i32 {
            return self.at(pos).angle_to2d(self.at(pos + 1));
        }
        INVALID_DOUBLE
    }

    /// distances of all my points to s and all of s points to myself
    pub fn distances(&self, s: &PositionVector, perpendicular: bool) -> Vec<f64> {
        let mut ret = Vec::new();
        for i in self.iter() {
            let dist = s.distance2d(i, perpendicular);
            if dist != INVALID_OFFSET {
                ret.push(dist);
            }
        }
        for i in s.iter() {
            let dist = self.distance2d(i, perpendicular);
            if dist != INVALID_OFFSET {
                ret.push(dist);
            }
        }
        ret
    }

    /// closest 2D-distance to point p (or -1 if perpendicular is true and the point is beyond this vector)
    pub fn distance2d(&self, p: &Position, perpendicular: bool) -> f64 {
        if self.is_empty() {
            return f64::MAX;
        } else if self.len() == 1 {
            return self.front().distance_to2d(p);
        }
        let nearest_offset = self.nearest_offset_to_point2d(p, perpendicular);
        if nearest_offset == INVALID_OFFSET {
            INVALID_OFFSET
        } else {
            p.distance_to2d(&self.position_at_offset2d(nearest_offset, 0., false))
        }
    }

    pub fn push_front(&mut self, p: Position) {
        self.insert(0, p);
    }

    /// C++ throws ProcessError when empty; this panics
    pub fn pop_front(&mut self) {
        if self.is_empty() {
            panic!("PositionVector is empty");
        }
        self.remove(0);
    }

    pub fn push_back_no_double_pos(&mut self, p: &Position) {
        if self.is_empty() || !p.almost_same(self.back(), POSITION_EPS) {
            self.push(*p);
        }
    }

    pub fn push_front_no_double_pos(&mut self, p: &Position) {
        if self.is_empty() || !p.almost_same(self.front(), POSITION_EPS) {
            self.push_front(*p);
        }
    }

    pub fn insert_no_double_pos(&mut self, at: usize, p: &Position) {
        if at == 0 {
            self.push_front_no_double_pos(p);
        } else if at == self.len() {
            self.push_back_no_double_pos(p);
        } else if !p.almost_same(&self[at], POSITION_EPS) && !p.almost_same(&self[at - 1], POSITION_EPS) {
            self.insert(at, *p);
        }
    }

    pub fn is_closed(&self) -> bool {
        self.len() >= 2 && self[0] == *self.back()
    }

    pub fn is_nan(&self) -> bool {
        self.iter().any(|p| p.is_nan())
    }

    /// round all coordinates to the given precision (C++ default avoid_degeneration = true)
    pub fn round(&mut self, precision: i32, avoid_degeneration: bool) {
        if avoid_degeneration && self.len() > 1 {
            self.ensure_min_length(precision);
        }
        for p in self.iter_mut() {
            p.round(precision);
        }
    }

    /// ensure minimum length so that the geometry will not degenerate to 0-length on writing with the given precision
    pub fn ensure_min_length(&mut self, precision: i32) {
        let limit = 2. * 10f64.powf(-precision as f64);
        if self.length2d() < limit {
            self.extrapolate2d(limit, false);
        }
    }

    /// Removes positions if too near. C++ defaults: min_dist = POSITION_EPS, assert_length = false,
    /// begin_offset = 0, end_offset = 0, resample = false
    pub fn remove_double_points(&mut self, min_dist: f64, assert_length: bool, begin_offset: usize, end_offset: usize, resample: bool) {
        let mut cur_size = self.len() as i64 - begin_offset as i64 - end_offset as i64;
        if cur_size > 1 {
            let mut last = begin_offset;
            let mut i = last + 1;
            while i != self.len() - end_offset && (!assert_length || cur_size > 2) {
                if self[last].almost_same(&self[i], min_dist) {
                    if i + 1 == self.len() - end_offset {
                        // special case: keep the last point and remove the next-to-last
                        if resample && last > 0 && self[last - 1].distance_to(&self[i]) >= 2. * min_dist {
                            // resample rather than remove point after a long segment
                            let shift_back = min_dist - self[last].distance_to(&self[i]);
                            let (bl, l) = (self[last - 1], self[last]);
                            self[last] = Self::position_at_offset_seg(&bl, &l, bl.distance_to(&l) - shift_back, 0.);
                            last = i;
                            i += 1;
                        } else {
                            self.remove(last);
                            i = self.len() - end_offset;
                        }
                    } else if resample && i + 1 != self.len() && self[last].distance_to(&self[i + 1]) >= 2. * min_dist {
                        // resample rather than remove points before a long segment
                        let shift_forward = min_dist - self[last].distance_to(&self[i]);
                        let (a, b) = (self[i], self[i + 1]);
                        self[i] = Self::position_at_offset_seg(&a, &b, shift_forward, 0.);
                        last = i;
                        i += 1;
                    } else {
                        self.remove(i);
                    }
                    cur_size -= 1;
                } else {
                    last = i;
                    i += 1;
                }
            }
        }
    }

    // shortcut: C++ operator-/operator+ on two PositionVectors never advance their iterators
    // (infinite loop), so they are not ported; port a fixed version if a caller needs it

    /// check if the two vectors have the same length and pairwise similar positions (C++ default max_div = POSITION_EPS)
    pub fn almost_same(&self, v2: &PositionVector, max_div: f64) -> bool {
        self.len() == v2.len() && self.iter().zip(v2.iter()).all(|(a, b)| a.almost_same(b, max_div))
    }

    /// return whether two positions differ in z-coordinate
    pub fn has_elevation(&self) -> bool {
        self.len() >= 2 && self.iter().any(|p| p.z() != 0.)
    }

    /// return whether the line segments p11,p12 and p21,p22 intersect, with (x, y, mu) of the intersection
    fn intersects_seg(p11: &Position, p12: &Position, p21: &Position, p22: &Position, within_dist: f64) -> Option<Intersection> {
        let eps = f64::EPSILON;
        let denominator = (p22.y() - p21.y()) * (p12.x() - p11.x()) - (p22.x() - p21.x()) * (p12.y() - p11.y());
        let numera = (p22.x() - p21.x()) * (p11.y() - p21.y()) - (p22.y() - p21.y()) * (p11.x() - p21.x());
        let numerb = (p12.x() - p11.x()) * (p11.y() - p21.y()) - (p12.y() - p11.y()) * (p11.x() - p21.x());
        // Are the lines coincident?
        if numera.abs() < eps && numerb.abs() < eps && denominator.abs() < eps {
            let (a1, a2, a3, a4);
            let mut a = -1e12;
            if p11.x() != p12.x() {
                a1 = if p11.x() < p12.x() { p11.x() } else { p12.x() };
                a2 = if p11.x() < p12.x() { p12.x() } else { p11.x() };
                a3 = if p21.x() < p22.x() { p21.x() } else { p22.x() };
                a4 = if p21.x() < p22.x() { p22.x() } else { p21.x() };
            } else {
                a1 = if p11.y() < p12.y() { p11.y() } else { p12.y() };
                a2 = if p11.y() < p12.y() { p12.y() } else { p11.y() };
                a3 = if p21.y() < p22.y() { p21.y() } else { p22.y() };
                a4 = if p21.y() < p22.y() { p22.y() } else { p21.y() };
            }
            if a1 <= a3 && a3 <= a2 {
                a = if a4 < a2 { (a3 + a4) / 2. } else { (a2 + a3) / 2. };
            }
            if a3 <= a1 && a1 <= a4 {
                a = if a2 < a4 { (a1 + a2) / 2. } else { (a1 + a4) / 2. };
            }
            if a != -1e12 {
                return Some(if p11.x() != p12.x() {
                    let mu = (a - p11.x()) / (p12.x() - p11.x());
                    (a, p11.y() + mu * (p12.y() - p11.y()), mu)
                } else {
                    let mu = if p12.y() == p11.y() { 0. } else { (a - p11.y()) / (p12.y() - p11.y()) };
                    (p11.x(), a, mu)
                });
            }
            return None;
        }
        // Are the lines parallel
        if denominator.abs() < eps {
            return None;
        }
        // Is the intersection along the segments
        let mut mua = numera / denominator;
        // reduce rounding errors for lines ending in the same point
        if (p12.x() - p22.x()).abs() < eps && (p12.y() - p22.y()).abs() < eps {
            mua = 1.;
        } else {
            let offseta = within_dist / p11.distance_to2d(p12);
            let offsetb = within_dist / p21.distance_to2d(p22);
            let mub = numerb / denominator;
            if mua < -offseta || mua > 1. + offseta || mub < -offsetb || mub > 1. + offsetb {
                return None;
            }
        }
        Some((p11.x() + mua * (p12.x() - p11.x()), p11.y() + mua * (p12.y() - p11.y()), mua))
    }

    /// rotate all points around (0,0) in the plane by the given angle
    pub fn rotate2d(&mut self, angle: f64) {
        let s = angle.sin();
        let c = angle.cos();
        for p in self.iter_mut() {
            let (x, y, z) = (p.x(), p.y(), p.z());
            let xnew = x * c - y * s;
            let ynew = x * s + y * c;
            p.set3(xnew, ynew, z);
        }
    }

    /// rotate all points around the given position in the plane by the given angle
    pub fn rotate2d_around(&mut self, pos: &Position, angle: f64) {
        self.sub(pos);
        self.rotate2d(angle);
        self.add(pos);
    }

    pub fn rotate_around_first_element2d(&mut self, angle: f64) {
        if self.len() > 1 {
            let offset = *self.front();
            self.sub(&offset);
            self.rotate2d(angle);
            self.add(&offset);
        }
    }

    /// return the same shape with intermediate colinear points removed
    pub fn simplified(&self) -> PositionVector {
        let mut result = self.clone();
        let mut changed = true;
        while changed && result.len() > 3 {
            changed = false;
            let n = result.len();
            for i in 0..n {
                let p1 = result[i];
                let p2 = result[(i + 2) % n];
                let middle_index = (i + 1) % n;
                let p0 = result[middle_index];
                let triangle_area2 =
                    ((p2.y() - p1.y()) * p0.x() - (p2.x() - p1.x()) * p0.y() + p2.x() * p1.y() - p2.y() * p1.x()).abs();
                let dist_ik = p1.distance_to2d(&p2);
                if dist_ik > NUMERICAL_EPS && triangle_area2 / dist_ik < NUMERICAL_EPS {
                    changed = true;
                    result.remove(middle_index);
                    break;
                }
            }
        }
        result
    }

    /// variation of Visvalingam-Whyatt using the distance instead of the area (C++ default eps = NUMERICAL_EPS)
    pub fn simplified2(&self, closed: bool, eps: f64) -> PositionVector {
        if self.len() < 3 {
            return self.clone();
        }
        let calc_score = |pv: &PositionVector, index: usize| -> f64 {
            let n = pv.len();
            if !closed && (index == 0 || index == n - 1) {
                return eps + 1.;
            }
            let p = &pv[index];
            let a = &pv[(index + n - 1) % n];
            let b = &pv[(index + 1) % n];
            let dist_ab = a.distance_to(b);
            if dist_ab < min2(eps, NUMERICAL_EPS) {
                // avoid division by 0 and degenerate cases due to very small baseline
                return (a.distance_to(p) + b.distance_to(p)) / 2.;
            }
            // calculating the distance of p to the line defined by a and b
            let dir = (*b - *a) / dist_ab;
            let projected_length = (*a - *p).dot_product(&dir);
            if projected_length <= -dist_ab {
                return b.distance_to(p);
            }
            if projected_length >= 0. {
                return a.distance_to(p);
            }
            let dist_vector = (*a - *p) - dir * projected_length;
            dist_vector.length()
        };
        let mut scores = Vec::with_capacity(self.len());
        let mut min_score = eps + 1.;
        let mut min_index = 0;
        for i in 0..self.len() {
            scores.push(calc_score(self, i));
            if scores[i] < min_score {
                min_score = scores[i];
                min_index = i;
            }
        }
        if min_score >= eps {
            return self.clone();
        }
        let mut result = self.clone();
        while min_score < eps {
            result.remove(min_index);
            if result.len() < 3 {
                break;
            }
            scores.remove(min_index);
            let n = result.len();
            let prev_index = (min_index + n - 1) % n;
            scores[prev_index] = calc_score(&result, prev_index);
            scores[min_index % n] = calc_score(&result, min_index % n);
            min_score = eps + 1.;
            for (i, &s) in scores.iter().enumerate() {
                if s < min_score {
                    min_score = s;
                    min_index = i;
                }
            }
        }
        result
    }

    /// return orthogonal through p (extending this vector if necessary).
    /// C++ defaults length = 1.0, deg = 90
    pub fn get_orthogonal(&self, p: &Position, extend: f64, before: bool, length: f64, mut deg: f64) -> PositionVector {
        let mut result = PositionVector::new();
        let mut tmp = self.clone();
        tmp.extrapolate2d(extend, false);
        let base_offset = tmp.nearest_offset_to_point2d(p, true);
        if base_offset == INVALID_OFFSET || self.len() < 2 {
            return result;
        }
        let base = tmp.position_at_offset2d(base_offset, 0., false);
        let closest_index = tmp.index_of_closest(&base, false);
        let closest_offset = tmp.offset_at_index2d(closest_index);
        result.push(base);
        if (base_offset - closest_offset).abs() > NUMERICAL_EPS {
            result.push(*tmp.at(closest_index));
            if (closest_offset < base_offset) != before {
                deg *= -1.;
            }
        } else if before {
            // take the segment before closestIndex if possible
            if closest_index > 0 {
                result.push(*tmp.at(closest_index - 1));
            } else {
                result.push(tmp[1]);
                deg *= -1.;
            }
        } else if closest_index < self.len() as i32 - 1 {
            // take the segment after closestIndex if possible
            result.push(*tmp.at(closest_index + 1));
        } else {
            result.push(*tmp.at(-1));
            deg *= -1.;
        }
        result = result.get_subpart2d(0., length);
        // rotate around base
        result.add(&(base * -1.));
        result.rotate2d(deg2rad(deg));
        result.add(&base);
        result
    }

    /// returned vector that is smoothed at the front (within dist; C++ default f64::MAX)
    pub fn smoothed_z_front(&self, mut dist: f64) -> PositionVector {
        let mut result = self.clone();
        if self.is_empty() {
            return result;
        }
        let z0 = self[0].z();
        // the z-delta of the first segment
        let dz = self.at(1).z() - z0;
        // if the shape only has 2 points it is as smooth as possible already
        if self.len() > 2 && dz != 0. {
            dist = min2(dist, self.length2d());
            // check whether we need to insert a new point at dist
            let p_dist = self.position_at_offset2d(dist, 0., false);
            let mut i_last = self.index_of_closest(&p_dist, false);
            // prevent close spacing to reduce impact of rounding errors in z-axis
            if p_dist.distance_to2d(self.at(i_last)) > POSITION_EPS * 20. {
                i_last = result.insert_at_closest(&p_dist, false);
            }
            let dist2 = result.offset_at_index2d(i_last);
            let dz2 = result.at(i_last).z() - z0;
            let mut seen = 0.;
            for i in 1..i_last as usize {
                seen += result[i].distance_to2d(&result[i - 1]);
                let r = &mut result[i];
                r.set3(r.x(), r.y(), z0 + dz2 * seen / dist2);
            }
        }
        result
    }

    /// returned vector that varies z smoothly over its length
    pub fn interpolate_z(&self, z_start: f64, z_end: f64) -> PositionVector {
        let mut result = self.clone();
        if self.is_empty() {
            return result;
        }
        result[0].setz(z_start);
        result.at_mut(-1).setz(z_end);
        let dz = z_end - z_start;
        let length = self.length2d();
        let mut seen = 0.;
        for i in 1..self.len().saturating_sub(1) {
            seen += result[i].distance_to2d(&result[i - 1]);
            result[i].setz(z_start + dz * seen / length);
        }
        result
    }

    /// resample shape (i.e. transform to segments, equal spacing)
    pub fn resample(&self, mut max_length: f64, adjust_end: bool) -> PositionVector {
        let mut result = PositionVector::new();
        if max_length == 0. {
            return result;
        }
        let length = self.length2d();
        if length < POSITION_EPS {
            return result;
        }
        max_length = length / (length / max_length).ceil();
        let mut pos = 0.;
        while pos <= length {
            result.push(self.position_at_offset2d(pos, 0., false));
            pos += max_length;
        }
        // check if we have to adjust last element
        if adjust_end && !result.is_empty() && result.back() != self.back() {
            result.push(*self.back());
        }
        result
    }

    /// return the offset at the given index
    pub fn offset_at_index2d(&self, index: i32) -> f64 {
        if index < 0 || index >= self.len() as i32 {
            return INVALID_OFFSET;
        }
        let mut seen = 0.;
        for i in 1..=index as usize {
            seen += self[i].distance_to2d(&self[i - 1]);
        }
        seen
    }

    /// return the maximum grade of all segments as a fraction of zRange/length2D
    /// (max_jump receives the maximum vertical jump with grade infinity)
    pub fn get_max_grade(&self, max_jump: &mut f64) -> f64 {
        let mut result = 0.;
        for i in 1..self.len() {
            let (p1, p2) = (&self[i - 1], &self[i]);
            let dist_z = (p1.z() - p2.z()).abs();
            let dist2d = p1.distance_to2d(p2);
            if dist2d == 0. {
                *max_jump = max2(*max_jump, dist_z);
            } else {
                result = max2(result, dist_z / dist2d);
            }
        }
        result
    }

    pub fn get_min_z(&self) -> f64 {
        self.iter().fold(f64::MAX, |m, p| min2(m, p.z()))
    }

    /// return a bezier interpolation
    pub fn bezier(&self, num_points: i32) -> PositionVector {
        // inspired by David F. Rogers
        assert!(self.len() < 33);
        const FAC: [f64; 33] = [
            1.0, 1.0, 2.0, 6.0, 24.0, 120.0, 720.0, 5040.0, 40320.0, 362880.0, 3628800.0, 39916800.0, 479001600.0,
            6227020800.0, 87178291200.0, 1307674368000.0, 20922789888000.0, 355687428096000.0, 6402373705728000.0,
            121645100408832000.0, 2432902008176640000.0, 51090942171709440000.0, 1124000727777607680000.0,
            25852016738884976640000.0, 620448401733239439360000.0, 15511210043330985984000000.0,
            403291461126605635584000000.0, 10888869450418352160768000000.0, 304888344611713860501504000000.0,
            8841761993739701954543616000000.0, 265252859812191058636308480000000.0,
            8222838654177922817725562880000000.0, 263130836933693530167218012160000000.0,
        ];
        let mut ret = PositionVector::new();
        let npts = self.len();
        // calculate the points on the Bezier curve
        let step = 1.0 / (num_points - 1) as f64;
        let mut t: f64 = 0.;
        let mut prev = Position::default();
        for _ in 0..num_points {
            if (1.0 - t) < 5e-6 {
                t = 1.0;
            }
            let (mut x, mut y, mut z) = (0., 0., 0.);
            for i in 0..npts {
                let ti = if i == 0 { 1.0 } else { t.powf(i as f64) };
                let tni = if npts == i + 1 { 1.0 } else { (1. - t).powf((npts - i - 1) as f64) };
                let basis = FAC[npts - 1] / (FAC[i] * FAC[npts - 1 - i]) * ti * tni;
                x += basis * self[i].x();
                y += basis * self[i].y();
                z += basis * self[i].z();
            }
            t += step;
            let current = Position::new3(x, y, z);
            if prev != current && !x.is_nan() && !y.is_nan() && !z.is_nan() {
                ret.push(current);
            }
            prev = current;
        }
        ret
    }

    /// checks if the polygon is clockwise-oriented (works for non-convex polygons).
    /// Takes `&mut self` because, as in C++, it shifts the points temporarily.
    pub fn is_clockwise_oriented(&mut self) -> bool {
        let mut area = 0.0;
        let mut y_min = self[0].y();
        for p in self.iter().skip(1) {
            if p.y() < y_min {
                y_min = p.y();
            }
        }
        let gap = if y_min > 0.0 { 0.0 } else { y_min };
        self.add_xyz(0., gap, 0.);
        let last = self.len() - 1;
        for i in 0..last {
            let (first_point, second_point) = (&self[i], &self[i + 1]);
            area += (second_point.x() - first_point.x()) / (second_point.y() + first_point.y()) / 2.0;
        }
        area += (self[0].x() - self[last].x()) / (self[0].y() + self[last].y()) / 2.0;
        self.add_xyz(0., -gap, 0.);
        area < 0.0
    }
}

/// PositionVector::as_poly_cw_sorter
fn as_poly_cw_less(p1: &Position, p2: &Position) -> bool {
    let at_angle2d = |p: &Position| {
        let angle = p.y().atan2(p.x());
        if angle < 0.0 { angle } else { angle + 2.0 * M_PI }
    };
    let angle1 = at_angle2d(p1);
    let angle2 = at_angle2d(p2);
    if angle1 > angle2 {
        return true;
    }
    angle1 == angle2 && p1.dot_product(p1) < p2.dot_product(p2)
}

/// PositionVector::increasing_x_y_sorter
fn increasing_x_y_less(p1: &Position, p2: &Position) -> bool {
    if p1.x() != p2.x() {
        return p1.x() < p2.x();
    }
    p1.y() < p2.y()
}

fn ordering_from_less(less: fn(&Position, &Position) -> bool, a: &Position, b: &Position) -> Ordering {
    if less(a, b) {
        Ordering::Less
    } else if less(b, a) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

impl AbstractPoly for PositionVector {
    /// whether the position vector describes a polygon lying around the given point
    fn around(&self, p: &Position, offset: f64) -> bool {
        if self.len() < 2 {
            return false;
        }
        if offset != 0. {
            let mut tmp = self.clone();
            tmp.scale_absolute(offset);
            return tmp.around(p, 0.);
        }
        let rel = |q: &Position| Position::new(q.x() - p.x(), q.y() - p.y());
        let mut angle = 0.;
        // iterate over all points, and obtain angle between current and next
        for i in 0..self.nseg() {
            angle += geom_helper::angle2d(&rel(&self[i]), &rel(&self[i + 1]));
        }
        // add angle between last and first point
        angle += geom_helper::angle2d(&rel(self.back()), &rel(&self[0]));
        // if angle is less than PI, then point lying in Polygon
        !(angle.abs() < M_PI)
    }

    fn overlaps_with(&self, poly: &dyn AbstractPoly, offset: f64) -> bool {
        if self.partial_within(poly, offset) || poly.partial_within(self, offset) {
            return true;
        }
        if self.len() >= 2 {
            for i in 0..self.nseg() {
                if poly.crosses(&self[i], &self[i + 1]) {
                    return true;
                }
            }
            if self.len() > 2 && poly.crosses(self.back(), self.front()) {
                return true;
            }
        }
        false
    }

    fn partial_within(&self, poly: &dyn AbstractPoly, offset: f64) -> bool {
        if self.len() < 2 {
            return false;
        }
        self.iter().any(|i| poly.around(i, offset))
    }

    fn crosses(&self, p1: &Position, p2: &Position) -> bool {
        self.intersects(p1, p2)
    }
}

/// Formats like the C++ stream operator: positions separated by spaces, fixed gPrecision
/// (or `{:.N}`) decimals.
impl fmt::Display for PositionVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prec = f.precision().map(|p| p as i32).unwrap_or_else(g_precision);
        f.write_str(&self.to_string_prec(prec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pv(pts: &[(f64, f64)]) -> PositionVector {
        pts.iter().map(|&(x, y)| Position::new(x, y)).collect()
    }

    #[test]
    fn position_at_offset() {
        let l = pv(&[(0., 0.), (10., 0.), (10., 10.)]);
        assert_eq!(l.length(), 20.);
        assert_eq!(l.position_at_offset(5., 0.), Position::new(5., 0.));
        assert_eq!(l.position_at_offset(15., 0.), Position::new(10., 5.));
        // lateral offset moves to the right of the driving direction
        assert_eq!(l.position_at_offset(5., 1.), Position::new(5., -1.));
        assert_eq!(l.position_at_offset(25., 0.), Position::new(10., 10.));
        // beyond the end with lateral offset uses the last segment
        assert_eq!(l.position_at_offset(25., 1.), Position::new(11., 10.));
        assert_eq!(l.position_at_offset2d(25., 0., true), Position::new(10., 15.));
        assert_eq!(PositionVector::new().position_at_offset(1., 0.), Position::INVALID);
        assert_eq!(l.rotation_at_offset(5.), 0.);
        assert_eq!(l.rotation_at_offset(15.), M_PI / 2.);
        assert_eq!(l.rotation_at_offset(-1.), M_PI / 2.);
        assert_eq!(l.rotation_degree_at_offset(5.), -90.);
        assert_eq!(l.rotation_degree_at_offset(15.), 180. - 360.);
    }

    #[test]
    fn nearest_offset() {
        let l = pv(&[(0., 0.), (10., 0.), (10., 10.)]);
        assert_eq!(l.nearest_offset_to_point2d(&Position::new(3., 2.), true), 3.);
        assert_eq!(l.nearest_offset_to_point2d(&Position::new(12., 4.), true), 14.);
        // outside the corner: only reachable through the inner-point check
        assert_eq!(l.nearest_offset_to_point2d(&Position::new(11., -1.), true), 10.);
        assert_eq!(l.nearest_offset_to_point2d(&Position::new(-3., -1.), true), INVALID_OFFSET);
        assert_eq!(l.nearest_offset_to_point2d(&Position::new(-3., -1.), false), 0.);
        assert_eq!(l.distance2d(&Position::new(3., 2.), false), 2.);
        // right is positive
        assert_eq!(l.transform_to_vector_coordinates(&Position::new(3., -2.), false), Position::new(3., 2.));
        assert_eq!(l.transform_to_vector_coordinates(&Position::new(3., 2.), false), Position::new(3., -2.));
        let t = l.transform_to_vector_coordinates(&Position::new(-3., 2.), true);
        assert!((t.x() + 3.).abs() < 1e-9 && (t.y() + 2.).abs() < 1e-9);
    }

    #[test]
    fn move2side() {
        let mut l = pv(&[(0., 0.), (10., 0.), (10., 10.)]);
        l.move2side(1., 100.);
        // positive amount moves to the right
        assert_eq!(l, pv(&[(0., -1.), (11., -1.), (11., 10.)]));
        let mut s = pv(&[(0., 0.), (5., 0.), (10., 0.)]);
        s.move2side(-2., 100.);
        assert_eq!(s, pv(&[(0., 2.), (5., 2.), (10., 2.)]));
        let mut d = pv(&[(0., 0.), (0.05, 0.), (10., 0.)]);
        d.move2side(1., 100.);
        assert_eq!(d, pv(&[(0., -1.), (10., -1.)]));
    }

    #[test]
    fn intersects() {
        let l = pv(&[(0., 0.), (10., 0.)]);
        assert!(l.intersects(&Position::new(5., -1.), &Position::new(5., 1.)));
        assert!(!l.intersects(&Position::new(5., 1.), &Position::new(5., 2.)));
        assert_eq!(l.intersection_position2d(&Position::new(5., -1.), &Position::new(5., 1.), 0.), Position::new(5., 0.));
        // within_dist extends the segments
        assert_eq!(l.intersection_position2d(&Position::new(11., -1.), &Position::new(11., 1.), 2.), Position::new(11., 0.));
        // coincident segments meet in the middle of the overlap
        assert_eq!(l.intersection_position2d(&Position::new(8., 0.), &Position::new(14., 0.), 0.), Position::new(9., 0.));
        assert_eq!(l.intersects_at_lengths2d(&Position::new(4., -1.), &Position::new(4., 1.)), [4.]);
        // parallel
        assert!(!l.intersects(&Position::new(0., 1.), &Position::new(10., 1.)));
    }

    #[test]
    fn subpart_split_and_misc() {
        let l = pv(&[(0., 0.), (10., 0.), (10., 10.)]);
        assert_eq!(l.get_subpart(5., 15.), pv(&[(5., 0.), (10., 0.), (10., 5.)]));
        assert_eq!(l.get_subpart(0., 20.), l);
        let (a, b) = l.split_at(5., false).unwrap();
        assert_eq!((a, b), (pv(&[(0., 0.), (5., 0.)]), pv(&[(5., 0.), (10., 0.), (10., 10.)])));
        assert!(l.split_at(30., false).is_err());
        assert_eq!(l.to_string_prec(2), "0.00,0.00 10.00,0.00 10.00,10.00");
        let sq = pv(&[(0., 0.), (2., 0.), (2., 2.), (0., 2.)]);
        assert_eq!(sq.area(), 4.);
        assert_eq!(sq.get_centroid(), Position::new(1., 1.));
        assert!(sq.around(&Position::new(1., 1.), 0.));
        assert!(!sq.around(&Position::new(3., 1.), 0.));
        assert!(sq.overlaps_with(&pv(&[(1., -1.), (1., 3.)]), 0.));
        let mut dp = pv(&[(0., 0.), (0.05, 0.), (5., 0.), (5.01, 0.)]);
        dp.remove_double_points(POSITION_EPS, false, 0, 0, false);
        assert_eq!(dp, pv(&[(0., 0.), (5.01, 0.)]));
        assert_eq!(*l.at(-1), Position::new(10., 10.));
        let mut e = pv(&[(0., 0.), (10., 0.)]);
        e.extrapolate(1., false, false);
        assert_eq!(e, pv(&[(-1., 0.), (11., 0.)]));
    }
}
