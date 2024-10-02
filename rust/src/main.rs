mod color;
mod point;
mod ray;
mod vec3;
mod geometry;
mod util;
mod camera;
mod material;

use std::rc::Rc;
use color::Color;
use geometry::sphere::Sphere;
use material::{Lambertian, Metal};
use point::Point3;
use geometry::hittable::HittableList;

fn main() -> std::io::Result<()> {
    // world
    let mut world = HittableList::new();

    // materials
    let material_ground = Rc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(&Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(&Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 1.0));


    world.add(Rc::new(Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(&Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(&Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(&Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    // camera
    let camera = camera::Builder::new()
        .set_image_width(400)
        .set_image_aspect_ratio(16.0 / 9.0)
        .set_center(&Point3::new_default())
        .set_viewport_height(2.0)
        .set_focal_length(1.0)
        .set_samples_per_pixel(100)
        .set_max_depth(50)
        .build();

    camera.render(&world)?;

    Ok(())
}
