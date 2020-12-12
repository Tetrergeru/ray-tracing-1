use image::{
    Rgb,
    RgbImage,
};

use crate::Float;

#[derive(Clone, Copy)]
pub struct Color {
    r: Float,
    g: Float,
    b: Float,
}

impl Color {
    pub fn new(r: Float, g: Float, b: Float) -> Self {
        Self {r, g, b}
    }

    pub fn to_rgb(&self) -> Rgb<u8> {
        Rgb([to_byte(self.r), to_byte(self.g), to_byte(self.b)])
    }
}

fn to_byte(number: Float) -> u8 {
    if number > 255.0 {
        255
    } else if number < 0.0 {
        0
    } else {
        number as u8
    }
}

pub struct ColorMatrix {
    matrix: Vec<Vec<Color>>,
}

impl ColorMatrix {
    pub fn new(width: usize, height: usize) -> Self {
        let mut matrix = Vec::new();
        for y in 0..height {
            matrix.push(Vec::new());
            for _ in 0..width {
                matrix[y].push(Color::new(0.0, 0.0, 0.0));
            }
        }
        ColorMatrix {
            matrix,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        self.matrix[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, c: Color) {
        self.matrix[y][x] = c;
    }

    pub fn to_image(&self) -> RgbImage {
        let width = self.matrix[0].len();
        let height = self.matrix.len();
        let mut img = RgbImage::new(width as u32, height as u32);
        for x in 0..width {
            for y in 0..height {
                img.put_pixel(x as u32, y as u32, self.matrix[y][x].to_rgb());
            }
        }
        img
    }
}