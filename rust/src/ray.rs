use crate::point::Point3;
use crate::vec3::Vector3;

pub struct Ray {
    orig: Point3,
    dir: Vector3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vector3) -> Self {
        Self {
            orig: origin.clone(),
            dir: direction.clone(),
        }
    }

    pub fn new_default() -> Self {
        Self {
            orig: Point3::new_default(),
            dir: Vector3::new_default(),
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vector3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
