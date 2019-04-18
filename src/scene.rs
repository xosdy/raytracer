use nalgebra as na;

use crate::material::Material;
use crate::rendering::{Intersetable, Ray};

pub struct Sphere {
    pub center: na::Vector3<f32>,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: na::Vector3<f32>, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Intersetable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let oc = self.center - ray.origin;
        let tca = oc.dot(&ray.direction);
        let d2 = oc.dot(&oc) - tca.powi(2);
        if d2 > self.radius.powi(2) {
            return None;
        }

        let thc = (self.radius.powi(2) - d2).sqrt();
        let t0 = tca - thc;
        if t0 < 0. {
            let t1 = tca + thc;
            if t1 < 0. {
                None
            } else {
                Some(t1)
            }
        } else {
            Some(t0)
        }
    }
}
