mod drawing;

use drawing::{
    Color,
    ColorMatrix,
};

type Float = f64;

fn main() {
    let mut matrix = ColorMatrix::new(100, 100);
    for x in 10..90 {
        for y in 10..90 {
            matrix.set(x, y, Color::new(1.0, 0.0, 0.0))
        }
    }
    matrix.to_image().save("foo.png").expect("Could not save image file");
}
