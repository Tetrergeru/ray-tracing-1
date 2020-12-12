mod drawing;
mod entities;
mod geometry;
mod material;
mod trace;
mod world;

use world::World;

type Float = f64;

fn main() {
    trace::trace(&World::scene_1(), 500, 500)
        .to_image()
        .save("foo.png")
        .expect("Could not save image file");
}
