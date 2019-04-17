use nalgebra as na;

#[derive(Clone, Copy)]
pub struct Material {
    pub albedo: na::Vector4<f32>,
    pub diffuse_color: na::Vector3<f32>,
    pub specular_exponent: f32,
    pub refractive_index: f32,
}

impl Material {
    pub fn new(
        albedo: na::Vector4<f32>,
        diffuse_color: na::Vector3<f32>,
        specular_exponent: f32,
        refractive_index: f32,
    ) -> Material {
        Material {
            albedo,
            diffuse_color,
            specular_exponent,
            refractive_index,
        }
    }
}
