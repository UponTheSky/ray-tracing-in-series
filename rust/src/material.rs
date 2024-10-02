use crate::color::Color;
use crate::ray::Ray;
use crate::geometry::hittable::HitRecord;
use crate::vec3::{random, reflect, Vector3, dot};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: albedo.clone() }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut random_unit_vector = Vector3::new_default();

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
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Self {
        Self { 
            albedo: albedo.clone(),
            fuzz: f64::min(fuzz, 1.0)
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected = reflect(&r_in.direction(), &rec.normal);
        let mut random_unit_vector = Vector3::new_default();

        while let Ok(vec) = random().normalize() {
            random_unit_vector = vec;
            break;
        } 

        reflected = reflected.normalize().unwrap() + (self.fuzz * random_unit_vector);

        *scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo.clone();

        dot(scattered.direction(), &rec.normal) > 0.0
    }
}