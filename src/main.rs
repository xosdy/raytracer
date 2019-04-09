mod sphere;

use crate::sphere::Sphere;

use nalgebra as na;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> Result<(), Box<std::error::Error>> {
    let s = Sphere::new(na::Vector3::new(-3., 0., -16.), 2.);
    render(&s)?;

    Ok(())
}

fn render(sphere: &Sphere) -> std::io::Result<()> {
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
            framebuffer.push(cast_ray(&na::Vector3::zeros(), &dir, &sphere));
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
    sphere: &Sphere,
) -> na::Vector3<f32> {
    sphere
        .ray_intersect(origin, direction)
        .map(|_| na::Vector3::new(0.4, 0.4, 0.3))
        .unwrap_or(na::Vector3::new(0.2, 0.7, 0.8))
}
