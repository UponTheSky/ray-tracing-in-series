use std::rc::Rc;

use crate::material::Material;
use crate::{point::Point3, util::interval::Interval};
use crate::ray::Ray;
use crate::vec3::dot;
use super::hittable::{Hittable, HitRecord};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center: center.clone(),
            radius: f64::max(radius, 0.0),
            mat: Rc::clone(&mat)
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center.clone() - ray.origin().clone();

        let a = ray.direction().length_squared();
        let h = dot(ray.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);

        let mut root = (h - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);

        let outward_normal = (rec.p - self.center) * (1.0 / self.radius);
        rec.set_face_normal(ray, &outward_normal).unwrap();
        rec.set_material(&self.mat);

        return true;

    }
}
