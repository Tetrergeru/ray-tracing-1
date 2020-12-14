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

    fn normal(&self, u: Float, v: Float) -> Point {
        let vec = self.u.dot(self.v).normalize();

        vec
    }

    pub fn intersect(&self, origin: Point, direction: Point) -> Option<IntersectionResult> {
        let intersection_point = match self.intersect_plane(origin, direction) {
            None => return None,
            Some(point) => point,
        };
        let (u, v) = self.plane_coordinates(intersection_point);
        Some(IntersectionResult::new(
            intersection_point,
            (intersection_point - origin).len(),
            self.normal(u, v),
        ))
    }

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

    fn plane_coordinates(&self, point: Point) -> (Float, Float) {
        let point = point - self.origin;

        let det = Self::det(self.u, self.v);

        let point_u = Self::det(point, self.v) / det;

        let point_v = Self::det(self.u, point) / det;
        
        (point_u, point_v)
    }

    fn det(a: Point, b: Point) -> Float {
        a.dot(b).sum()
    }
}
