use std::ops::{Add, AddAssign, Mul, MulAssign};

use image::{Rgb, RgbImage};

use crate::Float;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: Float,
    g: Float,
    b: Float,
}

macro_rules! define_colors {
    ($($color_name:ident ($r:expr, $g:expr, $b:expr))*) => {
        $(
            #[allow(dead_code)]
            pub const $color_name: Color =
            Color {
                r: $r as Float / 255.0,
                g: $g as Float / 255.0,
                b: $b as Float / 255.0
            };)*
    };
}

impl Color {
    define_colors![
        WHITE (255, 255, 255)
        BLACK (0, 0, 0)
        RED (255, 0, 0)
        GREEN (0, 255, 0)
        BLUE (0, 0, 255)
        ORANGE (255, 165, 0)
        PURPLE (128, 0, 128)
        BROWN (165, 42, 42)
        CYAN (0, 255, 255)
        DARK_GREEN (0, 100, 0)
        MOCCASIN (255, 228, 181)
        GOLD (255, 215, 0)
        MAGENTA (255, 0, 255)
    ];

    pub fn new(r: Float, g: Float, b: Float) -> Self {
        Self { r, g, b }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r as Float / 255.0, g as Float / 255.0, b as Float / 255.0)
    }

    pub fn to_rgb(&self) -> Rgb<u8> {
        Rgb([to_byte(self.r), to_byte(self.g), to_byte(self.b)])
    }
}

fn to_byte(number: Float) -> u8 {
    if number >= 1.0 {
        255
    } else if number <= 0.0 {
        0
    } else {
        (number * 255.0) as u8
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl MulAssign<Float> for Color {
    fn mul_assign(&mut self, other: Float) {
        self.r *= other;
        self.g *= other;
        self.b *= other;
    }
}

impl Mul<Float> for Color {
    type Output = Color;

    fn mul(self, other: Float) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
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
        ColorMatrix { matrix }
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
                img.put_pixel(x as u32, y as u32, self.get(x, y).to_rgb());
            }
        }
        img
    }
}

impl AddAssign<ColorMatrix> for ColorMatrix {
    fn add_assign(&mut self, other: ColorMatrix) {
        let width = self.matrix[0].len();
        let height = self.matrix.len();
        for x in 0..width {
            for y in 0..height {
                self.set(x, y, self.get(x, y) + other.get(x, y));
            }
        }
    }
}
