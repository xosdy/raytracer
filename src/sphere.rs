use nalgebra as na;

pub struct Sphere {
    center: na::Vector3<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(center: na::Vector3<f32>, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    pub fn ray_intersect(
        &self,
        origin: &na::Vector3<f32>,
        direction: &na::Vector3<f32>,
    ) -> Option<f32> {
        let oc = self.center - origin;
        let tca = oc.dot(direction);
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
