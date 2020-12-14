use super::{IntersectionResult, Plane, Point, Sphere, Triangle};

#[derive(Clone)]
pub enum Entity {
    Sphere(Sphere),
    Triangle(Triangle),
    Plane(Plane),
}

impl Entity {
    pub fn intersect(&self, origin: Point, direction: Point) -> Option<IntersectionResult> {
        match self {
            Entity::Sphere(sphere) => sphere.intersect(origin, direction),
            Entity::Triangle(triangle) => triangle.intersect(origin, direction),
            Entity::Plane(plane) => plane.intersect(origin, direction),
        }
    }
}
