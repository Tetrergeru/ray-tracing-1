use super::{IntersectionResult, Point, Sphere};

pub trait Entity {
    fn intersect(&self, origin: Point, vector: Point) -> Option<IntersectionResult>;
}

pub enum Entities {
    Sphere(Sphere),
}

impl Entity for Entities {
    fn intersect(&self, origin: Point, vector: Point) -> Option<IntersectionResult> {
        match self {
            Entities::Sphere(sphere) => sphere.intersect(origin, vector),
        }
    }
}
