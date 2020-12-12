use super::{IntersectionResult, Point, Sphere};

pub enum Entity {
    Sphere(Sphere),
}

impl Entity {
    pub fn intersect(&self, origin: Point, vector: Point) -> Option<IntersectionResult> {
        match self {
            Entity::Sphere(sphere) => sphere.intersect(origin, vector),
        }
    }
}
