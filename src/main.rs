mod light;
mod material;
mod sphere;

use std::fs::File;
use std::io::{BufWriter, Write};

use nalgebra as na;

use crate::light::Light;
use crate::material::Material;
use crate::sphere::Sphere;

struct RaycastHit {
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

fn main() -> Result<(), Box<std::error::Error>> {
    let ivory = Material::new(
        na::Vector4::new(0.6, 0.3, 0.1, 0.),
        na::Vector3::new(0.4, 0.4, 0.3),
        50.,
        1.,
    );
    let glass = Material::new(
        na::Vector4::new(0., 0.5, 0.1, 0.8),
        na::Vector3::new(0.6, 0.7, 0.8),
        125.,
        1.5,
    );
    let red_rubber = Material::new(
        na::Vector4::new(0.9, 0.1, 0., 0.),
        na::Vector3::new(0.3, 0.1, 0.1),
        10.,
        1.,
    );
    let mirror = Material::new(
        na::Vector4::new(0., 10., 0.8, 0.),
        na::Vector3::new(1., 1., 1.),
        1425.,
        1.,
    );

    let spheres = vec![
        Sphere::new(na::Vector3::new(-3., 0., -16.), 2., ivory),
        Sphere::new(na::Vector3::new(-1., -1.5, -12.), 2., glass),
        Sphere::new(na::Vector3::new(1.5, -0.5, -18.), 3., red_rubber),
        Sphere::new(na::Vector3::new(7., 5., -18.), 4., mirror),
    ];

    let light = vec![
        Light::new(na::Vector3::new(-20., 20., 20.), 1.5),
        Light::new(na::Vector3::new(30., 50., -25.), 1.8),
        Light::new(na::Vector3::new(30., 20., 30.), 1.7),
    ];

    render(&spheres, &light)?;

    Ok(())
}

fn render(spheres: &[Sphere], lights: &[Light]) -> std::io::Result<()> {
    let width = 1024;
    let height = 768;
    let fov = std::f32::consts::PI / 2.;
    let mut framebuffer = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            let x2 = (2. * (x as f32 + 0.5) / width as f32 - 1.) * (fov / 2.).tan() * width as f32
                / height as f32;
            let y2 = -(2. * (y as f32 + 0.5) / height as f32 - 1.) * (fov / 2.).tan();
            let dir = na::Vector3::new(x2, y2, -1.).normalize();
            framebuffer.push(cast_ray(&na::Vector3::zeros(), &dir, &spheres, lights, 5));
        }
    }

    let file = File::create("out.ppm")?;
    let mut buf = BufWriter::with_capacity(10_000_000, file);
    write!(buf, "P6\n{} {}\n255\n", width, height)?;
    buf.write_all(
        &framebuffer
            .iter_mut()
            .flat_map(|v| {
                let max = *v
                    .iter()
                    .max_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap();
                if max > 1. {
                    *v *= 1. / max;
                }
                v.iter().map(|x| (na::clamp(x, &0., &1.) * 255.) as u8)
            })
            .collect::<Vec<_>>(),
    )?;

    Ok(())
}

fn cast_ray(
    origin: &na::Vector3<f32>,
    direction: &na::Vector3<f32>,
    spheres: &[Sphere],
    lights: &[Light],
    depth: u32,
) -> na::Vector3<f32> {
    let background_color = na::Vector3::new(0.2, 0.7, 0.8);

    if depth == 0 {
        return background_color;
    }

    if let Some(hit) = scene_intersect(origin, direction, spheres) {
        let reflect_dir = -reflect(direction, &hit.normal).normalize();
        let refract_dir = refract(direction, hit.normal, hit.material.refractive_index).normalize();
        let reflect_origin = hit.get_surface(&reflect_dir);
        let refract_origin = hit.get_surface(&refract_dir);
        let reflect_color = cast_ray(&reflect_origin, &reflect_dir, spheres, lights, depth - 1);
        let refract_color = cast_ray(&refract_origin, &refract_dir, spheres, lights, depth - 1);

        let mut diffuse_light_intensity = 0.;
        let mut specular_light_intensity = 0.;
        for light in lights {
            let light_dir = (light.position - hit.position).normalize();
            let light_dist = (light.position - hit.position).norm();

            let shadow_origin = hit.get_surface(&light_dir);

            if let Some(shadow_hit) = scene_intersect(&shadow_origin, &light_dir, spheres) {
                if (shadow_hit.position - shadow_origin).norm() < light_dist {
                    continue;
                }
            }

            diffuse_light_intensity += light.intensity * (light_dir.dot(&hit.normal)).max(0.);
            specular_light_intensity += reflect(&-light_dir, &hit.normal)
                .dot(direction)
                .max(0.)
                .powf(hit.material.specular_exponent)
                * light.intensity;
        }

        hit.material.diffuse_color * diffuse_light_intensity * hit.material.albedo[0]
            + na::Vector3::new(1., 1., 1.) * specular_light_intensity * hit.material.albedo[1]
            + reflect_color * hit.material.albedo[2]
            + refract_color * hit.material.albedo[3]
    } else {
        background_color
    }
}

fn scene_intersect(
    origin: &na::Vector3<f32>,
    direction: &na::Vector3<f32>,
    spheres: &[Sphere],
) -> Option<RaycastHit> {
    spheres
        .iter()
        .filter_map(|s| s.ray_intersect(origin, direction).map(|dist| (dist, s)))
        .min_by(|(x, _), (y, _)| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(dist, s)| {
            let hit_point = origin + direction * dist;
            RaycastHit {
                material: s.material,
                position: hit_point,
                normal: (hit_point - s.center).normalize(),
            }
        })
}

fn reflect(
    surface_to_light_dir: &na::Vector3<f32>,
    surface_normal: &na::Vector3<f32>,
) -> na::Vector3<f32> {
    2. * surface_to_light_dir.dot(surface_normal) * surface_normal - surface_to_light_dir
}

// Snell's law
fn refract(
    light_to_surface_dir: &na::Vector3<f32>,
    mut plane_normal: na::Vector3<f32>,
    mut refractive_index: f32,
) -> na::Vector3<f32> {
    let mut cosi = -na::clamp(plane_normal.dot(light_to_surface_dir), -1., 1.);
    let mut etai = 1.;
    if cosi < 0. {
        cosi = -cosi;
        std::mem::swap(&mut etai, &mut refractive_index);
        plane_normal = -plane_normal;
    }

    let eta = etai / refractive_index;
    let k = 1. - eta.powi(2) * (1. - cosi.powi(2));

    if k < 0. {
        na::Vector3::zeros()
    } else {
        eta * light_to_surface_dir + (eta * cosi - k.sqrt()) * plane_normal
    }
}
