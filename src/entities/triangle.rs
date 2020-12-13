use super::{Float, IntersectionResult, Point};

#[derive(Clone)]
pub struct Triangle {
    origin: Point,
    u: Point,
    v: Point,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Self {
            origin: p1,
            u: p2 - p1,
            v: p3 - p1,
        }
    }

    pub fn new_room(p1: Point, p2: Point, p3: Point) -> Self {
        Self::new(p1, p3, p2)
    }

    fn normal(&self) -> Point {
        self.u.dot(self.v).normalize()
    }

    pub fn intersect(&self, origin: Point, direction: Point) -> Option<IntersectionResult> {
        let intersection_point = match self.intersect_plane(origin, direction) {
            None => return None,
            Some(point) => point,
        };
        if !self.triangle_contains(intersection_point) {
            return None;
        }
        Some(IntersectionResult::new(
            intersection_point,
            (intersection_point - origin).len(),
            self.normal(),
        ))
    }

    // plane equation: a x + b y + c z + d = 0 <=> (a, b, c) * (x, y, z) = -d
    // (a, b, c) * (origin + direction * t) = -d
    // t = -((a, b, c) * origin + d) / ((a, b, c) * direction)
    // origin + direction t = intersection_point
    fn intersect_plane(&self, origin: Point, direction: Point) -> Option<Point> {
        let abc = self.u.dot(self.v);
        let d = -(abc * self.origin);
        let coefficient = abc * direction;
        if coefficient.abs() < 0.00000001 {
            return None;
        }
        let free = -(abc * origin + d);
        let t = free / coefficient;
        if t < 0.0 {
            return None;
        }
        let intersection_point = origin + direction * t;
        Some(intersection_point)
    }

    // we find point coordinates in coordinate system with basis (u, v)
    // if u > 0, v > 0 and u + v < 1, then point is inside of the triangle
    #[allow(dead_code)]
    fn triangle_contains(&self, point: Point) -> bool {
        let point = point - self.origin;

        let det = Self::det(self.u, self.v);

        let point_u = Self::det(point, self.v) / det;
        if point_u < 0.0 {
            return false;
        }

        let point_v = Self::det(self.u, point) / det;
        if point_v < 0.0 || point_v + point_u > 1.0 {
            return false;
        }

        true
    }

    fn det(a: Point, b: Point) -> Float {
        a.dot(b).sum()
    }

    #[allow(dead_code)]
    fn triangle_contains_2(&self, point: Point) -> bool {
        let point = point - self.origin;
        Self::in_angle(Point::new(0.0, 0.0, 0.0), self.u, self.v, point)
            && Self::in_angle(self.u, self.v, Point::new(0.0, 0.0, 0.0), point)
            && Self::in_angle(self.v, Point::new(0.0, 0.0, 0.0), self.u, point)
    }

    fn in_angle(left: Point, middle: Point, right: Point, point: Point) -> bool {
        let left_vec = left - middle;
        let right_vec = right - middle;
        let point_vec = point - middle;

        let right_angle_cos = right_vec * left_vec / right_vec.len();
        let point_angle_cos = point_vec * left_vec / point_vec.len();

        return point_angle_cos >= right_angle_cos - 0.000001;
    }
}
