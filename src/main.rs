use nalgebra as na;
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<std::error::Error>>{
    render()?;

    Ok(())
}

fn render() -> std::io::Result<()> {
    let width = 1024;
    let height = 768;
    let mut framebuffer = Vec::with_capacity(width * height);

    for y in 0..height {
        for x in 0..width {
            framebuffer.push(na::Vector3::new(
                y as f32 / height as f32,
                x as f32 / width as f32,
                0.0,
            ));
        }
    }

    let mut file = File::create("out.ppm")?;
    write!(file, "P6\n{} {}\n255\n", width, height)?;

    for v in framebuffer {
        for i in 0..3 {
            file.write(&[(v[i] * 255.) as u8])?;
        }
    }

    Ok(())
}
