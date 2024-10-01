use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::util;

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vector3(f64, f64, f64);

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3(-self.0, -self.1, -self.2)
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn new_default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn normalize(&self) -> Result<Self, &'static str> {
        let length = self.length_squared();

        if length < 1e-170 {
            return Err("The vector is not normalizable: the length is too short");
        }

        let mut clone = self.clone();

        clone *= 1.0 / clone.length();

        Ok(clone)
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;

        f64::abs(self.0) < s && f64::abs(self.1) < s && f64::abs(self.2) < s
    }
}

#[inline]
pub fn dot(v1: &Vector3, v2: &Vector3) -> f64 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

#[inline]
pub fn cross(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3(
        v1.1 * v2.2 - v2.1 * v1.2,
        v1.2 * v2.0 - v1.0 * v2.2,
        v1.0 * v2.1 - v2.0 * v1.1,
    )
}

#[inline]
pub fn random() -> Vector3 {
    Vector3::new(
        util::random_double(),
        util::random_double(),
        util::random_double()
    )
}

#[inline]
pub fn random_in_range(min: f64, max: f64) -> Vector3 {
    Vector3::new(
        util::random_double_in_range(min, max),
        util::random_double_in_range(min, max),
        util::random_double_in_range(min, max),
    )
} 

#[inline]
fn random_unit_vector() -> Vector3 {
    let mut p: Vector3;

    loop {
        p = random_in_range(-1.0, 1.0);

        if p.length_squared() < 1e-160 {
            continue
        }

        break
    }

    p.normalize().unwrap()
}

#[inline]
pub fn random_on_hemisphere(normal: &Vector3) -> Vector3 {
    let random_vec = random_unit_vector();

    if dot(&random_vec, normal) > 0.0 {
        random_vec 
    } else {
        -random_vec
    }
}

#[inline]
pub fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
    v.clone() - 2.0 * dot(v, n) * n.clone()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_correctly() {
        let x = 1.0;
        let y = 2.0;
        let z = 3.0;
        let vec = Vector3(x, y, z);

        assert_eq!(vec.x(), x);
        assert_eq!(vec.y(), y);
        assert_eq!(vec.z(), z);
    }

    #[test]
    fn get_length_correctly() {
        let vec = Vector3::new(1.0, 2.0, 3.0);
        let expected: f64 = 14.0;
        let squared = vec.length_squared();

        assert_eq!(
            squared, expected,
            "result {} is different from expected {}",
            squared, expected
        );
    }

    #[test]
    fn normalize_correctly() {
        let vec = Vector3::new(1.0, 2.0, 3.0);
        let normalized = vec.normalize().unwrap();

        assert!(
            f64::abs(normalized.length() - 1.0) < 0.00005,
            "normalization is not correct"
        );
    }

    #[test]
    fn dot() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(2.0, 3.0, 4.0);

        assert_eq!(super::dot(&v1, &v2), 20.0);
    }

    #[test]
    fn cross() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(2.0, 3.0, 4.0);

        assert_eq!(super::cross(&v1, &v2), Vector3::new(-1.0, 2.0, -1.0));
    }
}
