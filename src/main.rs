mod sphere;

use crate::sphere::Sphere;

use nalgebra as na;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> Result<(), Box<std::error::Error>> {
    let mut spheres = vec![];
    spheres.push(Sphere::new(na::Vector3::new(-3., 0., -16.), 2.));
    spheres.push(Sphere::new(na::Vector3::new(-1., -1.5, -12.), 2.));
    spheres.push(Sphere::new(na::Vector3::new(1.5, -0.5, -18.), 3.));
    spheres.push(Sphere::new(na::Vector3::new(7., 5., -18.), 4.));

    render(&spheres)?;

    Ok(())
}

fn render(spheres: &Vec<Sphere>) -> std::io::Result<()> {
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
            framebuffer.push(cast_ray(&na::Vector3::zeros(), &dir, &spheres));
        }
    }

    let file = File::create("out.ppm")?;
    let mut buf = BufWriter::with_capacity(10_000_000, file);
    write!(buf, "P6\n{} {}\n255\n", width, height)?;
    buf.write_all(
        &framebuffer
            .iter()
            .flat_map(|v| v.iter().map(|x| (x * 255.) as u8))
            .collect::<Vec<_>>(),
    )?;

    Ok(())
}

fn cast_ray(
    origin: &na::Vector3<f32>,
    direction: &na::Vector3<f32>,
    spheres: &Vec<Sphere>,
) -> na::Vector3<f32> {
    if scene_intersect(origin, direction, spheres) {
        na::Vector3::new(0.4, 0.4, 0.3)
    } else {
        na::Vector3::new(0.2, 0.7, 0.8)
    }
}

fn scene_intersect(
    origin: &na::Vector3<f32>,
    direction: &na::Vector3<f32>,
    spheres: &Vec<Sphere>,
) -> bool {
    spheres
        .iter()
        .filter_map(|s| s.ray_intersect(origin, direction))
        .min_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
        .is_some()
}
