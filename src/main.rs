mod drawing;
mod entities;
mod geometry;
mod material;
mod trace;
mod world;

use world::World;

type Float = f64;

fn main() {
    trace::trace(&scene_1(), 100, 100)
        .to_image()
        .save("foo.png")
        .expect("Could not save image file");
}

use crate::{
    drawing::Color,
    entities::{Entity, Sphere},
    geometry::Point,
    material::Material,
};

use std::f64::consts;

fn scene_1() -> World {
    let mut entities = vec![(
        Entity::Sphere(Sphere::new_room(Point::new(0.0, 0.0, 0.0), 50.0)),
        Material::new_mirror(Color::new(0.0, 1.0, 1.0), 0.5),
    )];
    let mut angle = -consts::PI;
    while angle < consts::PI {
        entities.push((
            Entity::Sphere(Sphere::new(
                Point::new(angle.sin() * 20.0, 0.0, angle.cos() * 20.0 - 10.0),
                3.0,
            )),
            Material::new_mirror(Color::new(1.0, 0.0, 0.0), 0.5),
        ));
        entities.push((
            Entity::Sphere(Sphere::new(
                Point::new(0.0, angle.sin() * 15.0, angle.cos() * 15.0 + 5.0),
                1.5,
            )),
            Material::new_mirror(Color::new(0.0, 0.0, 1.0), 0.5),
        ));
        angle += consts::PI / 6.0;
    }
    World::new(
        entities,
        vec![Point::new(5.0, -7.0, -13.0), Point::new(-5.0, -5.0, 1.0)],
    )
}
