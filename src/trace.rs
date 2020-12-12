use crate::{
    drawing::{Color, ColorMatrix},
    geometry::Point,
    world::World,
    Float,
};

const FOV: Float = std::f64::consts::PI / 1.5;

pub fn trace(world: &World, width: usize, height: usize) -> ColorMatrix {
    let mut matrix = ColorMatrix::new(width, height);
    for j in 0..height {
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

const MAX_DEPTH: usize = 10;

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
        let shadowed = match world.cast_ray(entity.intersection_point, light - entity.intersection_point) {
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

    if material.reflection > 0.0 {
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
