use crate::{
    entities::{Entity, IntersectionResult},
    geometry::Point,
    material::Material,
    Float,
};

pub struct World {
    entities: Vec<(Entity, Material)>,
    pub light: Vec<Point>,
}

impl World {
    pub fn new(entities: Vec<(Entity, Material)>, light: Vec<Point>) -> Self {
        Self { entities, light }
    }

    pub fn cast_ray(&self, origin: Point, direction: Point) -> Option<CastResult> {
        let origin = origin + direction * 0.00001;
        let mut intersection = None;
        let mut distance = Float::INFINITY;
        let mut entity_idx = 0;
        for i in 0..self.entities.len() {
            let maybe_intersection = self.entities[i].0.intersect(origin, direction);
            match maybe_intersection {
                None => continue,
                Some(real_intersection) => {
                    if real_intersection.distance < distance {
                        distance = real_intersection.distance;
                        entity_idx = i;
                        intersection = Some(real_intersection);
                    }
                }
            }
        }
        match intersection {
            None => None,
            Some(real_intersection) => Some(CastResult::new(
                real_intersection,
                self.entities[entity_idx].1,
            )),
        }
    }
}

pub struct CastResult {
    pub intersection: IntersectionResult,
    pub material: Material,
}

impl CastResult {
    fn new(intersection: IntersectionResult, material: Material) -> Self {
        CastResult {
            intersection,
            material,
        }
    }
}
