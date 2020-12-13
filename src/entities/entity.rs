use super::{IntersectionResult, Point, Sphere, Triangle};

#[derive(Clone)]
pub enum Entity {
    Sphere(Sphere),
    Triangle(Triangle),
}

impl Entity {
    pub fn intersect(&self, origin: Point, direction: Point) -> Option<IntersectionResult> {
        match self {
            Entity::Sphere(sphere) => sphere.intersect(origin, direction),
            Entity::Triangle(triangle) => triangle.intersect(origin, direction),
        }
    }
}
