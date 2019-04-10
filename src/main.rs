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
    material: Material,
    position: na::Vector3<f32>,
    normal: na::Vector3<f32>,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let ivory = Material::new(na::Vector3::new(0.4, 0.4, 0.3));
    let red_rubber = Material::new(na::Vector3::new(0.3, 0.1, 0.1));

    let mut spheres = vec![];
    spheres.push(Sphere::new(na::Vector3::new(-3., 0., -16.), 2., ivory));
    spheres.push(Sphere::new(
        na::Vector3::new(-1., -1.5, -12.),
        2.,
        red_rubber,
    ));
    spheres.push(Sphere::new(
        na::Vector3::new(1.5, -0.5, -18.),
        3.,
        red_rubber,
    ));
    spheres.push(Sphere::new(na::Vector3::new(7., 5., -18.), 4., ivory));

    let light = vec![Light::new(na::Vector3::new(-20., 20., 20.), 1.5)];

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
            framebuffer.push(cast_ray(&na::Vector3::zeros(), &dir, &spheres, lights));
        }
    }

    let file = File::create("out.ppm")?;
    let mut buf = BufWriter::with_capacity(10_000_000, file);
    write!(buf, "P6\n{} {}\n255\n", width, height)?;
    buf.write_all(
        &framebuffer
            .iter()
            .flat_map(|v| v.iter().map(|x| (na::clamp(x, &0., &1.) * 255.) as u8))
            .collect::<Vec<_>>(),
    )?;

    Ok(())
}

fn cast_ray(
    origin: &na::Vector3<f32>,
    direction: &na::Vector3<f32>,
    spheres: &[Sphere],
    lights: &[Light],
) -> na::Vector3<f32> {
    if let Some(hit) = scene_intersect(origin, direction, spheres) {
        let mut diffuse_light_intensity = 0.;
        for light in lights {
            let light_dir = (light.position - hit.position).normalize();
            diffuse_light_intensity += light.intensity * (light_dir.dot(&hit.normal)).max(0.);
        }

        hit.material.diffuse_color * diffuse_light_intensity
    } else {
        na::Vector3::new(0.2, 0.7, 0.8)
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
