mod color;
mod point;
mod ray;
mod vec3;
mod geometry;
mod util;
mod camera;

use geometry::sphere::Sphere;
use point::Point3;
use geometry::hittable::{HittableList};

fn main() -> std::io::Result<()> {
    // world
    let mut world = HittableList::new();
    world.add(std::rc::Rc::new(Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(std::rc::Rc::new(Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0)));

    // camera
    let camera = camera::Builder::new()
        .set_image_width(400)
        .set_image_aspect_ratio(16.0 / 9.0)
        .set_center(&Point3::new_default())
        .set_viewport_height(2.0)
        .set_focal_length(1.0)
        .set_samples_per_pixel(100)
        .build();

    camera.render(&world)?;

    Ok(())
}
