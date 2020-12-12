use crate::{
    drawing::Color,
    entities::{Entity, IntersectionResult, Sphere},
    geometry::Point,
    material::Material,
    Float,
};

pub struct World {
    entities: Vec<(Entity, Material)>,
}

impl World {
    pub fn new(entities: Vec<(Entity, Material)>) -> Self {
        Self { entities }
    }

    #[allow(dead_code)]
    pub fn scene_1() -> Self {
        Self::new(vec![
            (
                Entity::Sphere(Sphere::new_room(Point::new(0.0, 0.0, 0.0), 50.0)),
                Material::new(Color::new(1.0, 0.0, 0.0)),
            ),
            (
                Entity::Sphere(Sphere::new(Point::new(0.0, 0.0, 20.0), 5.0)),
                Material::new(Color::new(0.0, 0.0, 1.0)),
            ),
        ])
    }

    pub fn cast_ray(&self, origin: Point, vector: Point) -> Option<CastResult> {
        let mut intersection = None;
        let mut distance = Float::INFINITY;
        let mut entity_idx = 0;
        for i in 0..self.entities.len() {
            let maybe_intersection = self.entities[i].0.intersect(origin, vector);
            match maybe_intersection {
                None => continue,
                Some(real_intersection) => {
                    if real_intersection.distance < distance && real_intersection.distance > 0.0001
                    {
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
