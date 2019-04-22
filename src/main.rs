mod light;
mod material;
mod rendering;
mod scene;

use nalgebra as na;

use crate::light::Light;
use crate::material::Material;
use crate::rendering::{Intersetable, Ray, RaycastHit};
use crate::scene::Sphere;

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

    let elements = vec![
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

    render(&elements, &light)?;

    Ok(())
}

fn render(elements: &[impl Intersetable], lights: &[Light]) -> std::io::Result<()> {
    let width = 1024;
    let height = 768;
    let fov = std::f32::consts::PI / 2.;
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let x2 = (2. * (x as f32 + 0.5) / width as f32 - 1.) * (fov / 2.).tan() * width as f32
            / height as f32;
        let y2 = -(2. * (y as f32 + 0.5) / height as f32 - 1.) * (fov / 2.).tan();
        let dir = na::Vector3::new(x2, y2, -1.).normalize();
        let ray = Ray::new(na::Vector3::zeros(), dir);
        let color = cast_ray(&ray, &elements, lights, 5);
        *pixel = color_to_rgb(&color);
    }

    std::fs::create_dir_all("./out")?;
    imgbuf.save("./out/render.png")
}

fn cast_ray(
    ray: &Ray,
    elements: &[impl Intersetable],
    lights: &[Light],
    depth: u32,
) -> na::Vector3<f32> {
    let background_color = na::Vector3::new(0.2, 0.7, 0.8);

    if depth == 0 {
        return background_color;
    }

    if let Some(hit) = scene_intersect(&ray, elements) {
        let reflect_dir = -reflect(&ray.direction, &hit.normal).normalize();
        let refract_dir =
            refract(&ray.direction, hit.normal, hit.material.refractive_index).normalize();
        let reflect_origin = hit.get_surface(&reflect_dir);
        let refract_origin = hit.get_surface(&refract_dir);
        let reflect_color = cast_ray(
            &Ray::new(reflect_origin, reflect_dir),
            elements,
            lights,
            depth - 1,
        );
        let refract_color = cast_ray(
            &Ray::new(refract_origin, refract_dir),
            elements,
            lights,
            depth - 1,
        );

        let mut diffuse_light_intensity = 0.;
        let mut specular_light_intensity = 0.;
        for light in lights {
            let light_dir = (light.position - hit.position).normalize();
            let light_dist = (light.position - hit.position).norm();

            let shadow_origin = hit.get_surface(&light_dir);

            if let Some(shadow_hit) = scene_intersect(&Ray::new(shadow_origin, light_dir), elements)
            {
                if (shadow_hit.position - shadow_origin).norm() < light_dist {
                    continue;
                }
            }

            diffuse_light_intensity += light.intensity * (light_dir.dot(&hit.normal)).max(0.);
            specular_light_intensity += reflect(&-light_dir, &hit.normal)
                .dot(&ray.direction)
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

fn scene_intersect(ray: &Ray, elements: &[impl Intersetable]) -> Option<RaycastHit> {
    elements
        .iter()
        .filter_map(|elem| elem.intersect(ray).map(|dist| (dist, elem)))
        .min_by(|(x, _), (y, _)| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(dist, elem)| {
            let hit_point = ray.origin + ray.direction * dist;
            RaycastHit {
                material: elem.material(),
                position: hit_point,
                normal: elem.surface_normal(&hit_point),
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

fn color_to_rgb(color: &na::Vector3<f32>) -> image::Rgb<u8> {
    let max = *color
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();

    let c = if max > 1. {
        color / max * 255.
    } else {
        color * 255.
    };

    image::Rgb([
        na::clamp(c.x, 0., 255.) as u8,
        na::clamp(c.y, 0., 255.) as u8,
        na::clamp(c.z, 0., 255.) as u8,
    ])
}
