use crate::point::Point3;
use crate::ray::Ray;
use crate::vec3::dot;

pub fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = center.clone() - ray.origin().clone();

    let a = ray.direction().length_squared();
    let h = dot(ray.direction(), &oc);
    let c = oc.length_squared() - radius * radius;

    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return -1.0
    }

    (h - f64::sqrt(discriminant)) / a
}
