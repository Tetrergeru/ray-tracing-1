mod drawing;
mod entities;
mod geometry;
mod material;
mod trace;
mod world;

use world::World;

type Float = f64;

fn main() {
    trace::path_trace(&scene_2(), "path_result_2.png".to_string(), 200, 200);
}

#[allow(dead_code)]
fn ray_trace() {
    let start = std::time::Instant::now();
    trace::trace_parallel(&scene_2(), 500, 500)
        .to_image()
        .save("result.png")
        .expect("Could not save image file");
    println!("{}", (std::time::Instant::now() - start).as_secs_f64());
}

use crate::{
    drawing::Color,
    entities::{Entity, Plane, Sphere, Triangle},
    geometry::Point,
    material::Material,
};

use std::f64::consts;

#[allow(dead_code)]
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

#[allow(dead_code)]
fn scene_2() -> World {
    let mut entities = room(
        Point::new(-10.0, -10.0, -1.0),
        Point::new(10.0, 10.0, 20.0),
        &vec![
            Material::new_diffuse(Color::WHITE),
            Material::new_diffuse(Color::WHITE),
            Material::new_mirror(Color::GREEN, 0.5),
            Material::new_diffuse(Color::WHITE),
            Material::new_diffuse(Color::WHITE),
            Material::new_mirror(Color::WHITE, 0.5),
        ],
    );

    entities.splice(
        entities.len()..,
        cube(
            Point::new(-3.0, -9.5, 7.0),
            Point::new(6.0, 1.0, 6.0),
            Material::new_light(Color::WHITE, 10.0),
        ),
    );

    entities.splice(
        entities.len()..,
        cube(
            Point::new(-6.0, 1.0, 7.0),
            Point::new(4.0, 9.0, 4.0),
            Material::new_diffuse(Color::BROWN),
        ),
    );

    entities.push((
        Entity::Sphere(Sphere::new(Point::new(-4.0, -2.0, 9.0), 3.0)),
        Material::new_transparent(Color::CYAN, 0.8, 1.33),
    ));

    entities.push((
        Entity::Sphere(Sphere::new(Point::new(5.0, 7.0, 7.0), 3.0)),
        Material::new_light(Color::from_rgb(100, 100, 255), 10.0),
    ));

    entities.splice(
        entities.len()..,
        cube(
            Point::new(0.0, 3.0, 12.0),
            Point::new(2.0, 7.0, 2.0),
            Material::new_mirror(Color::RED, 0.5),
        ),
    );

    entities.push((
        Entity::Sphere(Sphere::new(Point::new(1.0, 2.5, 13.0), 0.5)),
        Material::new_diffuse(Color::MOCCASIN),
    ));

    entities.push((
        Entity::Sphere(Sphere::new(Point::new(15.0, 5.0, 14.0), 10.0)),
        Material::new_mirror(Color::GOLD, 0.3),
    ));

    entities.push((
        Entity::Plane(Plane::new(
            Point::new(0.0, 5.0, 0.0),
            Point::new(0.0, 5.0, 0.1),
            Point::new(0.1, 5.0, 0.0),
        )),
        //Material::new_diffuse(Color::from_rgb(100, 255, 255)),
        //Material::new(Color::from_rgb(100, 255, 255), 0.9, 0.0, 0.8, 1.33, 0.0),
        Material::new_transparent(Color::from_rgb(100, 255, 255), 0.9, 1.333),
    ));

    // entities.splice(
    //     entities.len()..,
    //     absolute_cube(
    //         Point::new(-20.0, 5.0, -20.0),
    //         Point::new(20.0, 13.0, 30.0),
    //         Material::new_transparent(Color::from_rgb(100, 255, 255), 0.8, 1.333),
    //     ),
    // );

    for i in 0..5 {
        entities.push((
            Entity::Sphere(Sphere::new(
                Point::new(-8.0 + i as Float * 4.0, -8.0, 18.0),
                2.0,
            )),
            Material::new_mirror(Color::MAGENTA, i as Float / 5.0),
        ))
    }

    World::new(
        entities,
        vec![Point::new(5.0, -7.0, 13.0), Point::new(-5.0, -5.0, 1.0)],
    )
}

#[allow(dead_code)]
fn scene_3() -> World {
    let mut entities = room(
        Point::new(-10.0, -10.0, -1.0),
        Point::new(10.0, 5.0, 20.0),
        &vec![
            Material::new_diffuse(Color::BLUE),
            Material::new_diffuse(Color::ORANGE),
            Material::new_diffuse(Color::GREEN),
            Material::new_mirror(Color::WHITE, 0.0),
            Material::new_diffuse(Color::WHITE),
            Material::new_diffuse(Color::PURPLE),
        ],
    );

    entities.push((
        Entity::Sphere(Sphere::new(Point::new(-4.0, 2.0, 9.0), 3.0)),
        Material::new_transparent(Color::WHITE, 0.5, 1.33),
    ));

    entities.push((
        Entity::Sphere(Sphere::new(Point::new(1.0, -11.0, 13.0), 5.0)),
        Material::new_light(Color::WHITE, 5.0),
    ));

    World::new(
        entities,
        vec![Point::new(5.0, -7.0, 13.0), Point::new(-5.0, -5.0, 1.0)],
    )
}

#[allow(dead_code)]
fn scene_4() -> World {
    let mut entities = room(
        Point::new(-10.0, -10.0, -1.0),
        Point::new(10.0, 6.0, 20.0),
        &vec![
            Material::new_diffuse(Color::BLUE),
            Material::new_diffuse(Color::ORANGE),
            Material::new_diffuse(Color::GREEN),
            Material::new_diffuse(Color::WHITE),
            Material::new_diffuse(Color::WHITE),
            Material::new_diffuse(Color::PURPLE),
        ],
    );

    entities.splice(
        entities.len()..,
        cube(
            Point::new(-5.0, 1.0, 10.0),
            Point::new(10.0, 5.0, 5.0),
            Material::new_transparent(Color::from_rgb(100, 255, 255), 0.8, 1.333),
        ),
    );

    entities.splice(
        entities.len()..,
        cube(
            Point::new(-2.0, -2.0, 12.0),
            Point::new(1.5, 9.0, 1.5),
            Material::new_diffuse(Color::from_rgb(100, 255, 255)),
        ),
    );

    World::new(
        entities,
        vec![Point::new(5.0, -7.0, 13.0), Point::new(-5.0, -5.0, 1.0)],
    )
}

fn cube(origin: Point, size: Point, material: Material) -> Vec<(Entity, Material)> {
    absolute_cube(origin, origin + size, material)
}

fn absolute_cube(p000: Point, p111: Point, material: Material) -> Vec<(Entity, Material)> {
    general_cube(p000, p111, &(0..6).map(|_| material).collect(), false)
}

fn room(p000: Point, p111: Point, materials: &Vec<Material>) -> Vec<(Entity, Material)> {
    general_cube(p000, p111, materials, true)
}

fn general_cube(
    p000: Point,
    p111: Point,
    materials: &Vec<Material>,
    reverse_normals: bool,
) -> Vec<(Entity, Material)> {
    let p001 = Point::new(p000.x, p000.y, p111.z);
    let p010 = Point::new(p000.x, p111.y, p000.z);
    let p100 = Point::new(p111.x, p000.y, p000.z);
    let p011 = Point::new(p000.x, p111.y, p111.z);
    let p101 = Point::new(p111.x, p000.y, p111.z);
    let p110 = Point::new(p111.x, p111.y, p000.z);
    vec![
        (triangle(p000, p100, p010, reverse_normals), materials[0]),
        (triangle(p100, p110, p010, reverse_normals), materials[0]),
        (triangle(p010, p110, p011, reverse_normals), materials[1]),
        (triangle(p110, p111, p011, reverse_normals), materials[1]),
        (triangle(p110, p100, p111, reverse_normals), materials[2]),
        (triangle(p100, p101, p111, reverse_normals), materials[2]),
        (triangle(p001, p101, p000, reverse_normals), materials[3]),
        (triangle(p101, p100, p000, reverse_normals), materials[3]),
        (triangle(p000, p001, p010, !reverse_normals), materials[4]),
        (triangle(p001, p011, p010, !reverse_normals), materials[4]),
        (triangle(p011, p111, p001, reverse_normals), materials[5]),
        (triangle(p111, p101, p001, reverse_normals), materials[5]),
    ]
}

fn triangle(p0: Point, p1: Point, p2: Point, reversed_normal: bool) -> Entity {
    if reversed_normal {
        Entity::Triangle(Triangle::new_room(p0, p1, p2))
    } else {
        Entity::Triangle(Triangle::new(p0, p1, p2))
    }
}
