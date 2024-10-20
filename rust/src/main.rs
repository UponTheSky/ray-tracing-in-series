mod color;
mod point;
mod ray;
mod vec3;
mod geometry;
mod util;
mod camera;
mod material;

use std::sync::Arc;
use color::Color;
use geometry::sphere::Sphere;
use material::{Dielectric, Lambertian, Metal};
use point::Point3;
use geometry::hittable::HittableList;
use vec3::Vector3;

fn main() -> std::io::Result<()> {
    // world
    let mut world = HittableList::new();

    // materials
    let material_ground = Arc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(&Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.50));
    let material_bubble = Arc::new(Dielectric::new(1.00 / 1.50));
    let material_right = Arc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 1.0));


    world.add(Arc::new(Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Arc::new(Sphere::new(&Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Arc::new(Sphere::new(&Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Arc::new(Sphere::new(&Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.add(Arc::new(Sphere::new(&Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    // camera
    let camera = camera::Builder::new()
        .set_image_width(400)
        .set_image_aspect_ratio(16.0 / 9.0)
        .set_samples_per_pixel(100)
        .set_max_depth(50)
        .set_vfov(20.0)
        .set_lookfrom(&Point3::new(-2.0, 2.0, 1.0))
        .set_lookat(&Point3::new(0.0, 0.0, -1.0))
        .set_vup(&Vector3::new(0.0, 1.0, 0.0))
        .set_defocus_angle(0.5)
        .set_focus_dist(3.4)
        .build();

    camera.render(&world)?;

    Ok(())
}
