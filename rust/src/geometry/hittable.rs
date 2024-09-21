use std::rc::Rc;

use crate::ray::Ray;
use crate::point::Point3;
use crate::util::interval::Interval;
use crate::vec3::{dot, Vector3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new_default(),
            normal: Vector3::new_default(),
            t: 0.0,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) -> Result<(), &'static str> {
        if f64::abs(outward_normal.length() - 1.0) > 0.005 {
            return Err("outword normal is not a unit vector");
        }

        self.front_face = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { 
            outward_normal.clone() 
        } else { 
            -outward_normal.clone() 
        };

        Ok(())
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>, // REMARK: only for single threaded runtime
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![]
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        self.objects.iter().for_each(|object|{
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        });

        if hit_anything {
            *rec = temp_rec;
        }

        hit_anything
    }
}