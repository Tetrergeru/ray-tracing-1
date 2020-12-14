mod entity;
mod sphere;
mod triangle;
mod plane;

use super::{geometry::Point, Float};

pub use entity::Entity;
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
