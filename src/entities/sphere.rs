use super::{Float, IntersectionResult, Point};

pub struct Sphere {
    origin: Point,
    radius: Float,
}

impl Sphere {
    pub fn new(origin: Point, radius: Float) -> Self {
        Sphere {
            origin,
            radius: radius,
        }
    }

    pub fn new_room(origin: Point, radius: Float) -> Self {
        Sphere {
            origin,
            radius: -radius,
        }
    }

    fn normal(&self, point: Point) -> Point {
        if self.radius > 0.0 {
            (self.origin - point).normalize()
        } else {
            (point - self.origin).normalize()
        }
    }
}

impl Sphere {
    pub fn intersect(&self, origin: Point, direction: Point) -> Option<IntersectionResult> {
        let shifted_origin = origin - self.origin;
        let a = direction * direction;
        let b = 2.0 * (shifted_origin * direction);
        let c = shifted_origin * shifted_origin - self.radius * self.radius;
        let mut discr = b * b - 4.0 * a * c;
        if discr < 0.0 {
            return None;
        }
        discr = discr.sqrt();
        let mut root_1 = (-b - discr) / (a * 2.0);
        let mut root_2 = (-b + discr) / (a * 2.0);
        if root_1 < 0.0 {
            root_1 = Float::INFINITY
        };
        if root_2 < 0.0 {
            root_2 = Float::INFINITY
        };
        let min_root = min(root_1, root_2);
        if min_root == Float::INFINITY {
            return None;
        }
        let delta = direction * min_root;
        let point = origin + delta;
        Some(IntersectionResult::new(
            point,
            delta.len(),
            self.normal(point),
        ))
    }
}

fn min(a: Float, b: Float) -> Float {
    if a < b {
        a
    } else {
        b
    }
}
