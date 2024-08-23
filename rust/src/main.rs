mod color;
mod point;
mod ray;
mod vec3;

use color::{write_color, Color};
use ray::Ray;
use point::Point3;
use std::cmp::max;
use std::io::{self, BufWriter, Write};
use vec3::Vector3;

// image
const IMAGE_WIDTH: u32 = 400;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

// camera
const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0;

fn ray_color(ray: &Ray) -> Color {
    // we're sure the ray is not a zero vector, so we unwrap()
    let unit_direction = ray.direction().normalize().unwrap();
    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
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
    let viewport_v = Vector3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    let pixel_delta_u = viewport_u * (1.0 / (IMAGE_WIDTH as f64));
    let pixel_delta_v = viewport_v * (1.0 / (image_height as f64));

    let viewport_upper_left = camera_center
        - Vector3::new(0.0, 0.0, FOCAL_LENGTH)
        - (viewport_u * 0.5)
        - (viewport_v * 0.5);

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v); 

    stdout.write(b"P3\n")?;
    stdout.write((format!("{IMAGE_WIDTH} {image_height}\n255\n")).as_bytes())?;

    for j in 0..image_height {
        for i in 0..IMAGE_WIDTH {
            stderr.write(format!("\rScanlines remaining: {} ", image_height - j).as_bytes())?;
            stderr.flush()?;

            let pixel_center = pixel00_loc + ((i as f64) * pixel_delta_u) + ((j as f64) * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(&camera_center, &ray_direction);

            let pixel_color = ray_color(&ray);
            write_color(&mut stdout, &pixel_color)?;
        }
    }

    stderr.write("\rDone.                 \n".as_bytes())?;
    stderr.flush()?;
    stdout.flush()?;

    Ok(())
}
