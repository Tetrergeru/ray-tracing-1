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
                ),
            );
        }
    }
    matrix
}

fn vec_for_angle(x: Float, y: Float) -> Point {
    Point::new(x.sin(), y, x.cos())
}

fn trace_ray(world: &World, origin: Point, vector: Point) -> Color {
    let entity = match world.cast_ray(origin, vector) {
        None => return Color::new(0.0, 0.0, 0.0),
        Some(real_cast_result) => real_cast_result,
    };

    return entity.material.color;
}
