use crate::Float;

use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
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

    pub fn normalize(&self) -> Point {
        *self / self.len()
    }

    pub fn reflect(&self, normal: Point) -> Point {
        *self - normal * (2.0 * (normal * *self))
    }

    pub fn refract(&self, normal: Point, coefficient: Float) -> Point {
        let vec = self.normalize();
        let n1 = vec.len();
        let n2 = coefficient;
        vec + normal
            * (((n2 * n2 - n1 * n1) / (vec * normal).powi(2) + 1.0).sqrt() - 1.0)
            * (vec * normal)
    }

    pub fn dot(self, other: Point) -> Point {
        Point::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn sum(self) -> Float {
        self.x + self.y + self.z
    }

    pub fn rotate(&self, angles: Point) -> Point {
        let sin = Point::new(angles.x.sin(), angles.y.sin(), angles.z.sin());
        let cos = Point::new(angles.x.cos(), angles.y.cos(), angles.z.cos());

        let vec = Point::new(
            self.x,
            self.y * cos.x + self.z * sin.x,
            self.y * -sin.x + self.z * cos.x,
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

impl Div<Float> for Point {
    type Output = Point;

    fn div(self, other: Float) -> Point {
        Point::new(self.x / other, self.y / other, self.z / other)
    }
}
