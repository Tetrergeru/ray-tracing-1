mod entity;
mod plane;
mod sphere;
mod triangle;

use super::{geometry::Point, Float};

pub use entity::Entity;
pub use plane::Plane;
pub use sphere::Sphere;
pub use triangle::Triangle;

pub struct IntersectionResult {
    pub intersection_point: Point,
    pub distance: Float,
    pub normal: Point,
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
