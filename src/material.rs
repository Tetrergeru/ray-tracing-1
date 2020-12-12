use crate::drawing::Color;

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Color,
}

impl Material {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
