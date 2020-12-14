use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use rand::prelude::*;

use crate::{
    drawing::{Color, ColorMatrix},
    geometry::Point,
    world::World,
    Float,
};

const FOV: Float = std::f64::consts::PI / 1.5;

const MAX_DEPTH: usize = 10;

const THREAD_NUMBER: usize = 2;

#[allow(dead_code)]
pub fn trace(world: &World, width: usize, height: usize) -> ColorMatrix {
    trace_in_vertical_bounds(world, width, height, 0, height)
}

#[allow(dead_code)]
pub fn trace_parallel(world: &World, width: usize, height: usize) -> ColorMatrix {
    let mut matrix = ColorMatrix::new(width, height);
    let (sender, receiver): (Sender<ColorMatrix>, Receiver<ColorMatrix>) = mpsc::channel();
    let batch_size = height / THREAD_NUMBER;
    let start = std::time::Instant::now();
    for i in 0..THREAD_NUMBER {
        let sender = sender.clone();
        let world = world.clone();
        let from = batch_size * i;
        let to = if i == THREAD_NUMBER - 1 {
            height
        } else {
            batch_size * i + batch_size
        };
        println!("{}", to - from);
        thread::spawn(move || {
            sender
                .send(trace_in_vertical_bounds(&world, width, height, from, to))
                .expect("Could not send result");
        });
    }
    println!(
        "All threads created for {} secs",
        (std::time::Instant::now() - start).as_secs_f64()
    );

    for _ in 0..THREAD_NUMBER {
        matrix += receiver.recv().expect("Could not receive result");
    }
    println!(
        "All messages received for {} secs",
        (std::time::Instant::now() - start).as_secs_f64()
    );
    matrix
}

fn trace_in_vertical_bounds(
    world: &World,
    width: usize,
    height: usize,
    from: usize,
    to: usize,
) -> ColorMatrix {
    let mut matrix = ColorMatrix::new(width, height);
    for j in from..to {
        for i in 0..width {
            matrix.set(
                i,
                j,
                trace_ray(
                    world,
                    Point::new(0.0, 0.0, 0.0),
                    vec_for_angle(
                        (i as Float - width as Float / 2.0) / width as Float * FOV,
                        (j as Float - height as Float / 2.0) / height as Float * FOV,
                    ),
                    0,
                ),
            );
        }
    }
    matrix
}

fn vec_for_angle(x: Float, y: Float) -> Point {
    Point::new(x.sin(), y, x.cos())
}

const KA: Float = 1.0;

const IA: Float = 0.1;

const KS: Float = 0.3;

const KD: Float = 1.0;

const SHINESS: Float = 80.0;

fn trace_ray(world: &World, origin: Point, direction: Point, depth: usize) -> Color {
    let entity = match world.cast_ray(origin, direction) {
        None => return Color::new(0.0, 0.0, 0.0),
        Some(real_cast_result) => real_cast_result,
    };

    let material = entity.material;
    let entity = entity.intersection;
    let mut color = material.color;

    let mut shade = 0.0;
    for &light in world.light.iter() {
        let shadowed =
            match world.cast_ray(entity.intersection_point, light - entity.intersection_point) {
                None => false,
                Some(shadow_entity) => {
                    shadow_entity.intersection.distance < (light - entity.intersection_point).len()
                }
            };
        if shadowed {
            shade += KA * IA;
        } else {
            let light_vector = (entity.intersection_point - light).normalize();
            shade += KA * IA
                + KD * (entity.normal * light_vector)
                + KS * (entity.normal * light_vector.reflect(entity.normal)).powf(SHINESS);
        }
    }
    color *= shade / world.light.len() as Float;

    if depth >= MAX_DEPTH {
        return color;
    }

    if material.reflection > 0.0 && material.diffuse.abs() < 0.000001 {
        let mirror = trace_ray(
            world,
            entity.intersection_point,
            direction.reflect(entity.normal),
            depth + 1,
        );
        color = color * (1.0 - material.reflection) + mirror * material.reflection;
    }

    if material.transparency > 0.0 {
        let visible_trough = trace_ray(
            world,
            entity.intersection_point,
            direction.refract(entity.normal, material.refraction_coefficient),
            depth + 1,
        );
        color = color * (1.0 - material.transparency) + visible_trough * material.transparency;
    }

    return color;
}

#[allow(dead_code)]
pub fn path_trace(world: &World, fname: String, width: usize, height: usize) {
    let mut matrix = ColorMatrix::new(width, height);
    let (sender, receiver): (Sender<ColorMatrix>, Receiver<ColorMatrix>) = mpsc::channel();
    for _ in 0..THREAD_NUMBER {
        let sender = sender.clone();
        let world = world.clone();
        thread::spawn(move || loop {
            let mut matrix = ColorMatrix::new(width, height);
            for j in 0..height {
                for i in 0..width {
                    matrix.set(
                        i,
                        j,
                        trace_path(
                            &world,
                            Point::new(0.0, 0.0, 0.0),
                            vec_for_angle(
                                (i as Float - width as Float / 2.0) / width as Float * FOV,
                                (j as Float - height as Float / 2.0) / height as Float * FOV,
                            ),
                            0,
                        ),
                    );
                }
            }
            sender.send(matrix).expect("could not send matrix");
        });
    }
    for iteration in 0.. {
        let new_matrix = receiver.recv().expect("failed to receive");
        matrix.add_iteration(new_matrix, iteration);
        if iteration % 8 == 0 {
            print!("iteration {} finished, ", iteration);
            match matrix.to_image().save(fname.as_str()) {
                Ok(_) => println!("image flushed"),
                Err(_) => println!("problems flushing image"),
            };
        }
    }
}

fn trace_path(world: &World, origin: Point, direction: Point, depth: usize) -> Color {
    let entity = match world.cast_ray(origin, direction) {
        None => return Color::new(0.0, 0.0, 0.0),
        Some(real_cast_result) => real_cast_result,
    };

    let material = entity.material;
    let entity = entity.intersection;
    let mut color = material.color;

    if material.light > 0.00001 {
        return color * material.light;
    }

    if depth == MAX_DEPTH {
        return Color::BLACK;
    }

    if material.transparency > 0.00001 {
        let visible_trough = trace_path(
            world,
            entity.intersection_point,
            direction.refract(entity.normal, material.refraction_coefficient),
            depth + 1,
        );
        color = visible_trough * color * material.transparency;
    }

    if material.reflection > 0.00001 {
        let mirror = trace_path(
            world,
            entity.intersection_point,
            if material.diffuse > 0.00001 {
                diffuse(entity.normal, material.diffuse)
            } else {
                direction.reflect(entity.normal)
            },
            depth + 1,
        );
        color = color * mirror * material.reflection;
    }

    return color;
}

fn diffuse(vec: Point, diffusion: Float) -> Point {
    let vec = vec.normalize();
    let max_angle = 1.0 * std::f64::consts::PI * diffusion;
    let mut rng = rand::thread_rng();

    let angle = Point::new(
        (rng.gen::<f64>() - 0.0) * max_angle,
        (rng.gen::<f64>() - 0.0) * max_angle,
        (rng.gen::<f64>() - 0.0) * max_angle,
    );

    let sin = Point::new(angle.x.sin(), angle.y.sin(), angle.z.sin());
    let cos = Point::new(angle.x.cos(), angle.y.cos(), angle.z.cos());

    let vec = Point::new(
        vec.x,
        vec.y * cos.x + vec.z * sin.x,
        vec.y * -sin.x + vec.z * cos.x,
    );
    let vec = Point::new(
        vec.x * cos.y - vec.z * sin.y,
        vec.y,
        vec.x * sin.y + vec.z * cos.y,
    );
    let vec = Point::new(
        vec.x * cos.z + vec.y * sin.z,
        -vec.x * sin.z + vec.y * cos.z,
        vec.z,
    );
    vec
}
