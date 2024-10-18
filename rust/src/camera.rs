use std::io::{self, BufWriter, Write};

use crate::point::Point3;
use crate::ray::Ray;
use crate::geometry::hittable::{Hittable, HitRecord};
use crate::util::{degrees_to_radians, random_double};
use crate::util::{INFINITY, interval::Interval};
use crate::color::{write_color, Color};
use crate::vec3::{Vector3, cross};

pub struct Builder {
    // image
    image_width: u32,
    image_aspect_ratio: f64, // width: height

    // viewport
    vfov: f64,
    lookfrom: Point3,
    lookat: Point3,
    vup: Vector3,

    // samples
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            image_width: 0,
            image_aspect_ratio: 0.0,
            vfov: 0.0,
            lookfrom: Point3::new_default(),
            lookat: Point3::new_default(),
            vup: Vector3::new_default(),
            samples_per_pixel: 0,
            max_depth: 0,
        }
    }
    pub fn set_image_width(&mut self, width: u32) -> &mut Self {
        self.image_width = width;
        self
    }

    pub fn set_image_aspect_ratio(&mut self, ratio: f64) -> &mut Self {
        self.image_aspect_ratio = ratio;
        self
    }

    pub fn set_vfov(&mut self, vfov: f64) -> &mut Self {
        self.vfov = vfov;
        self
    }

    pub fn set_samples_per_pixel(&mut self, samples_per_pixel: u32) -> &mut Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn set_max_depth(&mut self, max_depth: u32) -> &mut Self {
        self.max_depth = max_depth;
        self
    }

    pub fn set_lookfrom(&mut self, lookfrom: &Point3) -> &mut Self {
        self.lookfrom = lookfrom.clone();
        self
    }

    pub fn set_lookat(&mut self, lookat: &Point3) -> &mut Self {
        self.lookat = lookat.clone();
        self
    }

    pub fn set_vup(&mut self, vup: &Vector3) -> &mut Self {
        self.vup = vup.clone();
        self
    }

    pub fn build(&self) -> Camera {
        // image
        let mut image_height = ((self.image_width as f64) / self.image_aspect_ratio) as u32;
        image_height = u32::max(image_height, 1);

        // viewport
        let vec_to_target = self.lookfrom - self.lookat;

        let center = self.lookfrom; // world space
        let focal_length = vec_to_target.length();
        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * ((self.image_width as f64) / (image_height as f64));

        let w = vec_to_target.normalize().unwrap();
        let u = cross(&self.vup, &w).normalize().unwrap();
        let v = cross(&w, &u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u * (1.0 / (self.image_width as f64));
        let pixel_delta_v = viewport_v * (1.0 / (image_height as f64));

        let viewport_upper_left = center
            - (focal_length * w)
            - (viewport_u * 0.5)
            - (viewport_v * 0.5);

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v); 

        Camera {
            image_width: self.image_width,
            image_height,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            center,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
        }
    }
}

pub struct Camera {
    // image 
    image_width: u32,
    image_height: u32,

    // viewport
    pixel_delta_u: Point3,
    pixel_delta_v: Point3,
    pixel00_loc: Point3,

    // center
    center: Point3,

    // sampling
    samples_per_pixel: u32,
    max_depth: u32,
}


impl Camera {
    // builder pattern
    pub fn render(&self, world: &dyn Hittable) -> std::io::Result<()> {
        let mut stdout = BufWriter::new(io::stdout().lock());
        let mut stderr = BufWriter::new(io::stderr().lock());
        stdout.write(b"P3\n")?;
        stdout.write((format!("{} {}\n255\n", self.image_width, self.image_height)).as_bytes())?;

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                stderr.write(format!("\rScanlines remaining: {} ", self.image_height - j).as_bytes())?;
                stderr.flush()?;

                let mut pixel_color = Color::new_default();

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&ray, self.max_depth, world);
                }

                write_color(&mut stdout, &(pixel_color * (1.0 / self.samples_per_pixel as f64)))?;
            }
        }

        stderr.write("\rDone.                 \n".as_bytes())?;
        stderr.flush()?;
        stdout.flush()?;

        Ok(())
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Point3::new(random_double() - 0.5, random_double() - 0.5, 0.0);
        let pixel_sample = self.pixel00_loc + ((i as f64) + offset.x()) * self.pixel_delta_u + ((j as f64) + offset.y()) * self.pixel_delta_v;

        let ray_direction = pixel_sample - self.center;

        Ray::new(&self.center, &ray_direction)
    }

    fn ray_color(ray: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new();

        if world.hit(ray, Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::new_default();
            let mut attenuation = Color::new_default(); 
            let material = rec.mat.clone();

            if material.unwrap().scatter(ray, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Camera::ray_color(&scattered, depth-1, world);
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }

        } 

        let unit_direction = ray.direction().normalize().unwrap();
        let alpha = 0.5 * (unit_direction.y() + 1.0); 

        (1.0 - alpha) * Color::new(1.0, 1.0, 1.0) + alpha * Color::new(0.5, 0.7, 1.0)
    }
}
