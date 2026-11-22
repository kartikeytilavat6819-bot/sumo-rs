//! Port of utils/geom/Triangle.{h,cpp}
use super::{AbstractPoly, Boundary, Position, PositionVector};

/// epsilon used for certain comparisons
const EPSILON: f64 = 1e-9;

/// A simple triangle defined in 3D
#[derive(Clone, Debug)]
pub struct Triangle {
    a: Position,
    b: Position,
    c: Position,
    boundary: Boundary,
}

impl Default for Triangle {
    fn default() -> Self {
        Self::INVALID
    }
}

/// equality only compares the corners, as in C++
impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}

impl Triangle {
    /// C++ `const Triangle Triangle::INVALID = Triangle();` — default-constructed corners at (0,0,0), not `Position::INVALID`.
    pub const INVALID: Triangle =
        Triangle { a: Position::new(0., 0.), b: Position::new(0., 0.), c: Position::new(0., 0.), boundary: Boundary::new() };

    pub fn new(a: Position, b: Position, c: Position) -> Self {
        let mut boundary = Boundary::new();
        boundary.add_pos(&a);
        boundary.add_pos(&b);
        boundary.add_pos(&c);
        Triangle { a, b, c, boundary }
    }
    pub fn get_boundary(&self) -> &Boundary {
        &self.boundary
    }
    pub fn get_shape(&self) -> PositionVector {
        PositionVector(vec![self.a, self.b, self.c])
    }
    /// check if the given position is within this triangle
    pub fn is_position_within(&self, pos: &Position) -> bool {
        Self::is_position_within_abc(&self.a, &self.b, &self.c, pos)
    }
    /// check if the given boundary is FULL within this triangle
    pub fn is_boundary_full_within(&self, boundary: &Boundary) -> bool {
        self.is_position_within(&Position::new(boundary.xmax(), boundary.ymax()))
            && self.is_position_within(&Position::new(boundary.xmin(), boundary.ymin()))
            && self.is_position_within(&Position::new(boundary.xmax(), boundary.ymin()))
            && self.is_position_within(&Position::new(boundary.xmin(), boundary.ymax()))
    }
    /// check if the given shape is within or intersect with this triangle
    pub fn intersect_with_shape(&self, shape: &PositionVector) -> bool {
        self.intersect_with_shape_boundary(shape, &shape.get_box_boundary())
    }
    pub fn intersect_with_shape_boundary(&self, shape: &PositionVector, shape_boundary: &Boundary) -> bool {
        // Check if the triangle's vertices are within the shape
        if shape.around(&self.a, 0.) || shape.around(&self.b, 0.) || shape.around(&self.c, 0.) {
            return true;
        }
        // Check if at least two corners of the shape boundary are within the triangle.
        let mut corners_inside = 0;
        if self.is_position_within(&Position::new(shape_boundary.xmax(), shape_boundary.ymax())) {
            corners_inside += 1;
        }
        if self.is_position_within(&Position::new(shape_boundary.xmin(), shape_boundary.ymin())) {
            corners_inside += 1;
        }
        if corners_inside < 2 && self.is_position_within(&Position::new(shape_boundary.xmax(), shape_boundary.ymin())) {
            corners_inside += 1;
        }
        if corners_inside < 2 && self.is_position_within(&Position::new(shape_boundary.xmin(), shape_boundary.ymax())) {
            corners_inside += 1;
        }
        if corners_inside >= 2 {
            return true;
        }
        // check if any line of the shape (treated as closed polygon) intersects with the triangle
        let n = shape.len();
        for i in 0..n {
            let p1 = &shape[i];
            let p2 = &shape[(i + 1) % n];
            if p1 != p2 && self.line_intersects_triangle(p1, p2) {
                return true;
            }
        }
        false
    }
    /// check if the given circle intersect with this triangle
    pub fn intersect_with_circle(&self, center: &Position, radius: f64) -> bool {
        let squared_radius = radius * radius;
        (center.distance_squared_to2d(&self.a) <= squared_radius)
            || (center.distance_squared_to2d(&self.b) <= squared_radius)
            || (center.distance_squared_to2d(&self.c) <= squared_radius)
            || self.is_position_within(center)
            || Self::line_intersect_circle(&self.a, &self.b, center, radius)
            || Self::line_intersect_circle(&self.b, &self.c, center, radius)
            || Self::line_intersect_circle(&self.c, &self.a, center, radius)
    }
    /// triangulate using Ear Clipping algorithm
    pub fn triangulate(mut shape: PositionVector) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        shape.open_polygon();
        if shape.len() >= 3 {
            while shape.len() > 3 {
                let n = shape.len();
                let ear_index = (0..n).find(|&i| Self::is_ear(&shape[(i + n - 1) % n], &shape[i], &shape[(i + 1) % n], &shape));
                if let Some(e) = ear_index {
                    triangles.push(Triangle::new(shape[(e + n - 1) % n], shape[e], shape[(e + 1) % n]));
                    shape.remove(e);
                } else {
                    // simply remove the first three
                    triangles.push(Triangle::new(shape[0], shape[1], shape[2]));
                    shape.remove(1);
                }
            }
            triangles.push(Triangle::new(shape[0], shape[1], shape[2]));
        }
        triangles
    }

    fn is_position_within_abc(a: &Position, b: &Position, c: &Position, pos: &Position) -> bool {
        let cross_ab = Self::cross_product(a, b, pos);
        let cross_bc = Self::cross_product(b, c, pos);
        let cross_ca = Self::cross_product(c, a, pos);
        let all_positive = (cross_ab > -EPSILON) && (cross_bc > -EPSILON) && (cross_ca > -EPSILON);
        let all_negative = (cross_ab < EPSILON) && (cross_bc < EPSILON) && (cross_ca < EPSILON);
        all_positive || all_negative
    }
    fn is_ear(a: &Position, b: &Position, c: &Position, shape: &PositionVector) -> bool {
        // Check if triangle ABC is counter-clockwise
        if Self::cross_product(a, b, c) <= 0. {
            return false;
        }
        // Check if any other point in the polygon lies inside the triangle
        !shape.iter().any(|pos| pos != a && pos != b && pos != c && Self::is_position_within_abc(a, b, c, pos))
    }
    fn cross_product(a: &Position, b: &Position, c: &Position) -> f64 {
        (b.x() - a.x()) * (c.y() - a.y()) - (b.y() - a.y()) * (c.x() - a.x())
    }
    fn orientation(p: &Position, q: &Position, r: &Position) -> i32 {
        let val = (q.y() - p.y()) * (r.x() - q.x()) - (q.x() - p.x()) * (r.y() - q.y());
        if val.abs() < EPSILON {
            0
        } else if val > 0. {
            1
        } else {
            -1
        }
    }
    /// check if point q lies on segment pr
    fn on_segment(p: &Position, q: &Position, r: &Position) -> bool {
        let min = |a: f64, b: f64| if b < a { b } else { a };
        let max = |a: f64, b: f64| if a < b { b } else { a };
        (q.x() >= min(p.x(), r.x()) - EPSILON)
            && (q.x() <= max(p.x(), r.x()) + EPSILON)
            && (q.y() >= min(p.y(), r.y()) - EPSILON)
            && (q.y() <= max(p.y(), r.y()) + EPSILON)
    }
    fn segments_intersect(p1: &Position, q1: &Position, p2: &Position, q2: &Position) -> bool {
        let o1 = Self::orientation(p1, q1, p2);
        let o2 = Self::orientation(p1, q1, q2);
        let o3 = Self::orientation(p2, q2, p1);
        let o4 = Self::orientation(p2, q2, q1);
        (o1 != o2 && o3 != o4)
            || (o1 == 0 && Self::on_segment(p1, p2, q1))
            || (o2 == 0 && Self::on_segment(p1, q2, q1))
            || (o3 == 0 && Self::on_segment(p2, p1, q2))
            || (o4 == 0 && Self::on_segment(p2, q1, q2))
    }
    fn line_intersects_triangle(&self, p1: &Position, p2: &Position) -> bool {
        Self::segments_intersect(p1, p2, &self.a, &self.b)
            || Self::segments_intersect(p1, p2, &self.b, &self.c)
            || Self::segments_intersect(p1, p2, &self.c, &self.a)
    }
    fn line_intersect_circle(pos_a: &Position, pos_b: &Position, center: &Position, radius: f64) -> bool {
        let dx = pos_b.x() - pos_a.x();
        let dy = pos_b.y() - pos_a.y();
        let a = dx * dx + dy * dy;
        let b = 2. * (dx * (pos_a.x() - center.x()) + dy * (pos_a.y() - center.y()));
        let c = (pos_a.x() - center.x()) * (pos_a.x() - center.x()) + (pos_a.y() - center.y()) * (pos_a.y() - center.y())
            - radius * radius;
        let discriminant = b * b - 4. * a * c;
        if discriminant >= 0. {
            let sqrt_discriminant = discriminant.sqrt();
            let t1 = (-b + sqrt_discriminant) / (2. * a);
            let t2 = (-b - sqrt_discriminant) / (2. * a);
            (0. ..=1.).contains(&t1) || (0. ..=1.).contains(&t2)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_matches_default_constructed_cpp_triangle() {
        // C++ `Triangle::INVALID = Triangle()` has corners at Position() (0,0,0), not Position::INVALID.
        assert_eq!(Triangle::INVALID, Triangle::new(Position::new(0., 0.), Position::new(0., 0.), Position::new(0., 0.)));
        assert_eq!(Triangle::default(), Triangle::INVALID);
    }

    #[test]
    fn triangle() {
        let t = Triangle::new(Position::new(0., 0.), Position::new(4., 0.), Position::new(0., 4.));
        assert!(t.is_position_within(&Position::new(1., 1.)));
        assert!(!t.is_position_within(&Position::new(3., 3.)));
        assert!(t.intersect_with_circle(&Position::new(3., 3.), 1.5));
        assert!(!t.intersect_with_circle(&Position::new(5., 5.), 1.));
        let line: PositionVector = vec![Position::new(-1., 1.), Position::new(1., 1.)].into();
        assert!(t.intersect_with_shape(&line));
        let sq: PositionVector =
            vec![Position::new(0., 0.), Position::new(2., 0.), Position::new(2., 2.), Position::new(0., 2.)].into();
        let tris = Triangle::triangulate(sq);
        assert_eq!(tris.len(), 2);
        assert_eq!(tris[0], Triangle::new(Position::new(0., 2.), Position::new(0., 0.), Position::new(2., 0.)));
    }
}
