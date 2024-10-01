use crate::color::Color;
use crate::ray::Ray;
use crate::geometry::hittable::HitRecord;
use crate::vec3::{random, reflect, Vector3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    fn new(albedo: &Color) -> Self {
        Self { albedo: albedo.clone() }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut random_unit_vector: Vector3;

        while let Ok(vector) = random().normalize() {
            random_unit_vector = vector;
            break;
        }

        let mut scatter_direction = rec.normal + random_unit_vector;
        
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        *scattered = Ray::new(&rec.p, &scatter_direction);
        *attenuation = self.albedo.clone();
        
        true
    }
}

pub struct Metal {
    albedo: Color
}

impl Metal {
    fn new(albedo: &Color) -> Self {
        Self { albedo: albedo.clone() }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(&r_in.direction(), &rec.normal);

        *scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo.clone();

        true
    }
}