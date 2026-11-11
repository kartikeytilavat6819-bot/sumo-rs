//! Port of utils/geom/GeomHelper.{h,cpp}
use std::f64::consts::PI;

use super::{min2, write_error, Boundary, Position, PositionVector};

pub const M_PI: f64 = PI;
pub const GRAVITY: f64 = 9.80665;
/// a value to signify offsets outside the range of [0, Line.length()]
pub const INVALID_OFFSET: f64 = -1.;

/// DEG2RAD macro: `x * M_PI / 180.`
#[inline]
pub fn deg2rad(x: f64) -> f64 {
    x * M_PI / 180.
}
/// RAD2DEG macro: `x * 180. / M_PI`
#[inline]
pub fn rad2deg(x: f64) -> f64 {
    x * 180. / M_PI
}

/// Returns the positions (0-1 along the line's length) the given circle is crossed by the given line
pub fn find_line_circle_intersections(c: &Position, radius: f64, p1: &Position, p2: &Position, into: &mut Vec<f64>) {
    let dx = p2.x() - p1.x();
    let dy = p2.y() - p1.y();

    let a = dx * dx + dy * dy;
    let b = 2. * (dx * (p1.x() - c.x()) + dy * (p1.y() - c.y()));
    let cc = (p1.x() - c.x()) * (p1.x() - c.x()) + (p1.y() - c.y()) * (p1.y() - c.y()) - radius * radius;

    let det = b * b - 4. * a * cc;
    if (a <= 0.0000001) || (det < 0.) {
        // No real solutions.
        return;
    }
    if det == 0. {
        // One solution.
        let t = -b / (2. * a);
        if (0. ..=1.).contains(&t) {
            into.push(t);
        }
    } else {
        // Two solutions.
        let t = (-b + det.sqrt()) / (2. * a);
        if (0. ..=1.).contains(&t) {
            into.push(t);
        }
        let t2 = (-b - det.sqrt()) / (2. * a);
        if (0. ..=1.).contains(&t2) {
            into.push(t2);
        }
    }
}

/// Returns the angle between two vectors on a plane, from vector 1 to vector 2,
/// positive anticlockwise, between -pi and pi
pub fn angle2d(p1: &Position, p2: &Position) -> f64 {
    angle_diff(p1.y().atan2(p1.x()), p2.y().atan2(p2.x()))
}

/// C++ default perpendicular = true
pub fn nearest_offset_on_line_to_point2d(line_start: &Position, line_end: &Position, p: &Position, perpendicular: bool) -> f64 {
    let line_length2d = line_start.distance_to2d(line_end);
    if line_length2d == 0. {
        return 0.;
    }
    // scalar product equals length of orthogonal projection times length of vector being projected onto
    // dividing the scalar product by the distance gives the relative position
    let u = (((p.x() - line_start.x()) * (line_end.x() - line_start.x()))
        + ((p.y() - line_start.y()) * (line_end.y() - line_start.y())))
        / line_length2d;
    if u < 0. || u > line_length2d {
        // closest point does not fall within the line segment
        if perpendicular {
            return INVALID_OFFSET;
        }
        if u < 0. {
            return 0.;
        }
        return line_length2d;
    }
    u
}

pub fn nearest_offset_on_line_to_point25d(line_start: &Position, line_end: &Position, p: &Position, perpendicular: bool) -> f64 {
    let mut result = nearest_offset_on_line_to_point2d(line_start, line_end, p, perpendicular);
    if result != INVALID_OFFSET {
        let line_length2d = line_start.distance_to2d(line_end);
        let line_length = line_start.distance_to(line_end);
        result *= line_length / line_length2d;
    }
    result
}

/// C++ throws `1` when there is no crossing; here that is `None`
pub fn cross_point(b: &Boundary, v: &PositionVector) -> Option<Position> {
    let sides = [
        (Position::new(b.xmin(), b.ymin()), Position::new(b.xmin(), b.ymax())),
        (Position::new(b.xmax(), b.ymin()), Position::new(b.xmax(), b.ymax())),
        (Position::new(b.xmin(), b.ymin()), Position::new(b.xmax(), b.ymin())),
        (Position::new(b.xmin(), b.ymax()), Position::new(b.xmax(), b.ymax())),
    ];
    sides
        .iter()
        .find(|(p1, p2)| v.intersects(p1, p2))
        .map(|(p1, p2)| v.intersection_position2d(p1, p2, 0.))
}

/// distance of second angle from first angle counter-clockwise (degrees)
pub fn get_ccw_angle_diff(angle1: f64, angle2: f64) -> f64 {
    let mut v = angle2 - angle1;
    if v < 0. {
        v = 360. + v;
    }
    v
}

/// distance of second angle from first angle clockwise (degrees)
pub fn get_cw_angle_diff(angle1: f64, angle2: f64) -> f64 {
    let mut v = angle1 - angle2;
    if v < 0. {
        v = 360. + v;
    }
    v
}

pub fn get_min_angle_diff(angle1: f64, angle2: f64) -> f64 {
    min2(get_cw_angle_diff(angle1, angle2), get_ccw_angle_diff(angle1, angle2))
}

/// difference of the second angle to the first angle in radians, between -pi and pi
pub fn angle_diff(angle1: f64, angle2: f64) -> f64 {
    let mut dtheta = angle2 - angle1;
    while dtheta > M_PI {
        dtheta -= 2.0 * M_PI;
    }
    while dtheta < -M_PI {
        dtheta += 2.0 * M_PI;
    }
    dtheta
}

/// mathematical radians to navigational degrees (0 is up, clockwise), in [0, 360)
pub fn navi_degree(angle: f64) -> f64 {
    let mut degree = rad2deg(M_PI / 2. - angle);
    if degree.is_infinite() {
        return 0.;
    }
    while degree >= 360. {
        degree -= 360.;
    }
    while degree < 0. {
        degree += 360.;
    }
    degree
}

pub fn from_navi_degree(angle: f64) -> f64 {
    M_PI / 2. - deg2rad(angle)
}

/// mathematical radians to legacy sumo degrees (0 is down, clockwise);
/// [0, 360) if positive else [-180, 180). C++ default positive = false
pub fn legacy_degree(angle: f64, positive: bool) -> f64 {
    let mut degree = -rad2deg(M_PI / 2. + angle);
    if positive {
        while degree >= 360. {
            degree -= 360.;
        }
        while degree < 0. {
            degree += 360.;
        }
    } else {
        while degree >= 180. {
            degree -= 360.;
        }
        while degree < -180. {
            degree += 360.;
        }
    }
    degree
}

pub fn make_circle(radius: f64, center: &Position, n_points: u32) -> PositionVector {
    if n_points < 3 {
        write_error("GeomHelper::makeCircle() requires nPoints>=3");
    }
    let mut circle = PositionVector::new();
    circle.push(Position::new(radius, 0.));
    for i in 1..n_points {
        let a = 2.0 * M_PI * i as f64 / n_points as f64;
        circle.push(Position::new(radius * a.cos(), radius * a.sin()));
    }
    circle.push(Position::new(radius, 0.));
    circle.add(center);
    circle
}

pub fn make_ring(radius1: f64, radius2: f64, center: &Position, n_points: u32) -> PositionVector {
    if n_points < 3 {
        write_error("GeomHelper::makeRing() requires nPoints>=3");
    }
    if radius1 >= radius2 {
        write_error("GeomHelper::makeRing() requires radius2>radius1");
    }
    let mut ring = PositionVector::new();
    ring.push(Position::new(radius1, 0.));
    ring.push(Position::new(radius2, 0.));
    for i in 1..n_points {
        let a = 2.0 * M_PI * i as f64 / n_points as f64;
        ring.push(Position::new(radius2 * a.cos(), radius2 * a.sin()));
    }
    ring.push(Position::new(radius2, 0.));
    ring.push(Position::new(radius1, 0.));
    for i in 1..n_points {
        let a = -2.0 * M_PI * i as f64 / n_points as f64;
        ring.push(Position::new(radius1 * a.cos(), radius1 * a.sin()));
    }
    ring.push(Position::new(radius1, 0.));
    ring.add(center);
    ring
}

pub fn calculate_lot_space_position(shape: &PositionVector, index: i32, space_dim: f64, angle: f64, width: f64, length: f64) -> Position {
    let mut pos = Position::default();
    let start_offset = shape.position_at_offset(space_dim * index as f64, 0.);
    let end_offset = shape.position_at_offset(space_dim * (index + 1) as f64, 0.);
    if angle == 0. {
        // parking parallel to the road
        pos = end_offset;
    } else {
        // angled parking
        let mut norm_angle = angle;
        while norm_angle < 0. {
            norm_angle += 360.;
        }
        norm_angle %= 360.; // fmod
        let radian_angle = norm_angle / 180. * M_PI;
        let space_extension = width * radian_angle.sin() + length * radian_angle.cos();
        let hlp_angle = (end_offset.y() - start_offset.y()).atan2(end_offset.x() - start_offset.x());
        let x_offset = 0.5 * width * radian_angle.sin() - 0.5 * (space_extension - space_dim);
        pos.setx(start_offset.x() + x_offset + length * radian_angle.cos());
        if norm_angle <= 90. {
            pos.sety(start_offset.y() + 0.5 * width * (1. - radian_angle.cos()) - length * radian_angle.sin());
        } else if norm_angle <= 180. {
            pos.sety(start_offset.y() + 0.5 * width * (1. + radian_angle.cos()) - length * radian_angle.sin());
        } else if angle <= 270. {
            pos.sety(start_offset.y() + 0.5 * width * (1. - (radian_angle - M_PI).cos()));
        } else {
            pos.sety(start_offset.y() + 0.5 * width * (1. + (radian_angle - M_PI).cos()));
        }
        pos.setz((start_offset.z() + end_offset.z()) / 2.);
        pos = pos.rotate_around2d(hlp_angle, &start_offset);
    }
    pos
}

pub fn calculate_lot_space_angle(shape: &PositionVector, index: i32, space_dim: f64, angle: f64) -> f64 {
    let start_offset = shape.position_at_offset(space_dim * index as f64, 0.);
    let end_offset = shape.position_at_offset(space_dim * (index + 1) as f64, 0.);
    ((end_offset.x() - start_offset.x()).atan2(start_offset.y() - end_offset.y()) * 180.0 / M_PI) - angle
}

pub fn calculate_lot_space_slope(shape: &PositionVector, index: i32, space_dim: f64) -> f64 {
    shape.slope_degree_at_offset(space_dim * (index + 1) as f64)
}

pub fn vectorize(val: f64, radian_angle: f64) -> Position {
    Position::new(val * radian_angle.cos(), val * radian_angle.sin())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn angles() {
        // angleDiff wraps into [-pi, pi]
        assert_eq!(angle_diff(0.1, 0.3), 0.3 - 0.1);
        assert!((angle_diff(3.0, -3.0) - (-3.0 - 3.0 + 2. * M_PI)).abs() < 1e-15);
        // east (0 rad) is 90 navi degrees and -90 legacy degrees
        assert_eq!(navi_degree(0.), 90.);
        assert_eq!(legacy_degree(0., false), -90.);
        assert_eq!(legacy_degree(0., true), 270.);
        assert_eq!(navi_degree(M_PI / 2.), 0.);
        assert_eq!(get_ccw_angle_diff(350., 10.), 20.);
        assert_eq!(get_cw_angle_diff(350., 10.), 340.);
        assert_eq!(get_min_angle_diff(350., 10.), 20.);
        assert_eq!(from_navi_degree(90.), M_PI / 2. - deg2rad(90.));
        assert_eq!(angle2d(&Position::new(1., 0.), &Position::new(0., 1.)), M_PI / 2.);
    }
    #[test]
    fn nearest_offset_on_line() {
        let a = Position::new(0., 0.);
        let b = Position::new(10., 0.);
        assert_eq!(nearest_offset_on_line_to_point2d(&a, &b, &Position::new(3., 4.), true), 3.);
        assert_eq!(nearest_offset_on_line_to_point2d(&a, &b, &Position::new(-3., 4.), true), INVALID_OFFSET);
        assert_eq!(nearest_offset_on_line_to_point2d(&a, &b, &Position::new(-3., 4.), false), 0.);
        assert_eq!(nearest_offset_on_line_to_point2d(&a, &b, &Position::new(13., 4.), false), 10.);
        let mut into = Vec::new();
        find_line_circle_intersections(&Position::new(5., 0.), 2., &a, &b, &mut into);
        assert_eq!(into, [0.7, 0.3]);
    }
}
