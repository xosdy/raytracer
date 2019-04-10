use nalgebra as na;

#[derive(Clone, Copy)]
pub struct Material {
    pub diffuse_color: na::Vector3<f32>,
}

impl Material {
    pub fn new(diffuse_color: na::Vector3<f32>) -> Material {
        Material { diffuse_color }
    }
}
