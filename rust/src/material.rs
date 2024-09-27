use crate::color::Color;
use crate::ray::Ray;
use crate::geometry::hittable::HitRecord;

pub trait Material {
    fn scatter(r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}
