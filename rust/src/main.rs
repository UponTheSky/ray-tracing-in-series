mod color;
mod point;
mod ray;
mod vec3;

use color::{write_color, Color};
use point::Point3;
use vec3::Vector3;
use std::io::{self, BufWriter, Write};
use std::cmp::max;

// image
const IMAGE_WIDTH: u32 = 400;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

// camera
const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0;

fn ray_color() -> Color {
    Color::new_default()
}

fn main() -> std::io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout().lock());
    let mut stderr = BufWriter::new(io::stderr().lock());

    // image height
    let mut image_height = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u32;
    image_height = max(image_height, 1);

    // camera
    let viewport_width = VIEWPORT_HEIGHT * ((IMAGE_WIDTH as f64) / (image_height as f64));
    let camera_center = Point3::new_default();
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(VIEWPORT_HEIGHT, 0.0, 0.0);
    
    // TODO: start from pixel_delta_u
    

    stdout.write(b"P3\n")?;
    stdout.write((format!("{IMAGE_WIDTH} {image_height}\n255\n")).as_bytes())?;

    for j in 0..image_height {
        for i in 0..IMAGE_WIDTH {
            stderr.write(format!("\rScanlines remaining: {} ", image_height - j).as_bytes())?;
            stderr.flush()?;

            let color = Color::new(
                (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                (j as f64) / ((image_height - 1) as f64),
                0.0,
            );
            write_color(&mut stdout, &color)?;
        }
    }

    stderr.write("\rDone.                 \n".as_bytes())?;
    stderr.flush()?;
    stdout.flush()?;

    Ok(())
}
