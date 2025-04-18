use crate::color::Color;
use crate::geometry::hittable::HitRecord;
use crate::ray::Ray;
use crate::util::random_double;
use crate::vec3::{dot, random, reflect, refract, Vector3};

pub trait Material: Sync + Send {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Self {
        Self {
            albedo: albedo.clone(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Self {
        Self {
            albedo: albedo.clone(),
            fuzz: f64::min(fuzz, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
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

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r * r;

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction().normalize().unwrap();
        let cos_theta = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let direction =
            if (ri * sin_theta > 1.0) || Dielectric::reflectance(cos_theta, ri) > random_double() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, ri)
            };

        *scattered = Ray::new(&rec.p, &direction);

        true
    }
}
