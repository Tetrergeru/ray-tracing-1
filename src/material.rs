use crate::{drawing::Color, Float};

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub reflection: Float,
    pub diffuse: Float,
    pub transparency: Float,
    pub refraction_coefficient: Float,
    pub light: Float,
}

impl Material {
    #[allow(dead_code)]
    pub fn new(
        color: Color,
        reflection: Float,
        diffuse: Float,
        transparency: Float,
        refraction_coefficient: Float,
        light: Float,
    ) -> Self {
        Self {
            color,
            reflection,
            diffuse,
            transparency,
            refraction_coefficient,
            light: light,
        }
    }

    #[allow(dead_code)]
    pub fn new_light(color: Color, light: Float) -> Self {
        Self {
            color,
            diffuse: 0.0,
            reflection: 0.0,
            transparency: 0.0,
            refraction_coefficient: 1.0,
            light,
        }
    }

    #[allow(dead_code)]
    pub fn new_diffuse(color: Color) -> Self {
        Self {
            color,
            diffuse: 1.0,
            reflection: 0.9,
            transparency: 0.0,
            refraction_coefficient: 1.0,
            light: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn new_mirror(color: Color, reflection: Float) -> Self {
        Self {
            color,
            reflection,
            diffuse: 0.0,
            transparency: 0.0,
            refraction_coefficient: 1.0,
            light: 0.0,
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
            diffuse: 0.0,
            transparency,
            refraction_coefficient,
            light: 0.0,
        }
    }
}
