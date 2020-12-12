use crate::{drawing::Color, Float};

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub reflection: Float,
    pub transparency: Float,
    pub refraction_coefficient: Float,
}

impl Material {
    #[allow(dead_code)]
    pub fn new(
        color: Color,
        reflection: Float,
        transparency: Float,
        refraction_coefficient: Float,
    ) -> Self {
        Self {
            color,
            reflection,
            transparency,
            refraction_coefficient,
        }
    }

    #[allow(dead_code)]
    pub fn new_diffuse(color: Color) -> Self {
        Self {
            color,
            reflection: 0.0,
            transparency: 0.0,
            refraction_coefficient: 1.0,
        }
    }

    #[allow(dead_code)]
    pub fn new_mirror(color: Color, reflection: Float) -> Self {
        Self {
            color,
            reflection,
            transparency: 0.0,
            refraction_coefficient: 1.0,
        }
    }

    #[allow(dead_code)]
    pub fn new_transparent(
        color: Color,
        transparency: Float,
        refraction_coefficient: Float,
    ) -> Self {
        Self {
            color,
            reflection: 0.0,
            transparency,
            refraction_coefficient,
        }
    }
}
