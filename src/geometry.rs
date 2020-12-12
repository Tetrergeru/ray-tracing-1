use crate::Float;

use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Point {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Point {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    pub fn len(&self) -> Float {
        (*self * *self).sqrt()
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl SubAssign<Point> for Point {
    fn sub_assign(&mut self, other: Point) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl Mul<Point> for Point {
    type Output = Float;

    fn mul(self, other: Point) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Mul<Float> for Point {
    type Output = Point;

    fn mul(self, other: Float) -> Point {
        Point::new(self.x * other, self.y * other, self.z * other)
    }
}
