mod entity;
mod sphere;

use super::{geometry::Point, Float};

pub use entity::Entity;
pub use sphere::Sphere;

pub struct IntersectionResult {
    intersection_point: Point,
    distance: Float,
    normal: Point,
}

impl IntersectionResult {
    pub fn new(intersection_point: Point, distance: Float, normal: Point) -> Self {
        Self {
            intersection_point,
            distance,
            normal,
        }
    }
}
