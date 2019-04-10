use nalgebra as na;

pub struct Light {
    pub position: na::Vector3<f32>,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: na::Vector3<f32>, intensity: f32) -> Light {
        Light {
            position,
            intensity,
        }
    }
}
