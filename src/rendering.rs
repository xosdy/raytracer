use nalgebra as na;

use crate::material::Material;

pub struct Ray {
    pub origin: na::Vector3<f32>,
    pub direction: na::Vector3<f32>,
}

impl Ray {
    pub fn new(origin: na::Vector3<f32>, direction: na::Vector3<f32>) -> Self {
        Ray { origin, direction }
    }
}

pub struct RaycastHit {
    pub material: Material,
    pub position: na::Vector3<f32>,
    pub normal: na::Vector3<f32>,
}

impl RaycastHit {
    pub fn get_surface(&self, dir: &na::Vector3<f32>) -> na::Vector3<f32> {
        if dir.dot(&self.normal) < 0. {
            self.position - self.normal * 1e-3
        } else {
            self.position + self.normal * 1e-3
        }
    }
}

pub trait Intersetable {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    fn material(&self) -> Material;
    fn surface_normal(&self, hit_point: &na::Vector3<f32>) -> na::Vector3<f32>;
}
