#[derive(Clone, Debug)]
pub struct Vector3(f64, f64, f64);

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

    pub fn opposite(&self) -> Self {
        Self::new(-self.0, -self.1, -self.2)
    }

    pub fn add(&mut self, other: &Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }

    pub fn scalar_multiply(&mut self, scalar: f64) {
        self.0 *= scalar;
        self.1 *= scalar;
        self.2 *= scalar;
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn normalize(&self) -> Result<Self, &'static str> {
        let length = self.length_squared();

        if length < 0.0005 {
            return Err("The vector is not normalizable: the length is too short");
        }

        let mut clone = self.clone();

        clone.scalar_multiply(1.0 / clone.length());

        Ok(clone)
    }
}

pub fn add(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3(v1.x() + v2.x(), v1.y() + v2.y(), v1.z() + v2.z())
}

pub fn subtract(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3(v1.x() - v2.x(), v1.y() - v2.y(), v1.z() - v2.z())
}

pub fn dot(v1: &Vector3, v2: &Vector3) -> f64 {
    v1.x() * v2.x() + v1.y() + v2.y() + v1.z() + v2.z()
}

pub fn cross(v1: &Vector3, v2: &Vector3) -> Vector3 {
    Vector3(
        v1.y() * v2.z() - v2.y() * v1.z(),
        v1.z() * v2.x() - v1.x() * v2.z(),
        v1.x() * v2.y() - v2.x() * v1.y(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_vec3_equal(v1: &Vector3, v2: &Vector3) -> bool {
        v1.x() == v2.x() && v1.y() == v2.y() && v1.z() == v2.z()
    }

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
    fn opposite_correctly() {
        let x = 1.0;
        let y = 2.0;
        let z = 3.0;

        let vec = Vector3::new(x, y, z);
        let vec_opposite = vec.opposite();
        let vec_expected = Vector3::new(-x, -y, -z);

        assert!(
            is_vec3_equal(&vec_opposite, &vec_expected),
            "result {:?} is different from expected {:?}",
            &vec_opposite,
            &vec_expected
        );
    }

    #[test]
    fn add_correctly() {
        let mut vec = Vector3::new(1.0, 2.0, 3.0);
        let vec_added = Vector3::new(4.0, 5.0, 6.0);
        let vec_expected = Vector3::new(5.0, 7.0, 9.0);

        vec.add(&vec_added);

        assert!(
            is_vec3_equal(&vec, &vec_expected),
            "result {:?} is different from expected {:?}",
            &vec,
            &vec_expected
        );
    }

    #[test]
    fn scalar_multiply_correctly() {
        let mut vec = Vector3::new(1.0, 2.0, 3.0);
        let vec_expected = Vector3::new(2.0, 4.0, 6.0);

        vec.scalar_multiply(2.0);

        assert!(
            is_vec3_equal(&vec, &vec_expected),
            "result {:?} is different from expected {:?}",
            &vec,
            &vec_expected
        );
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
}
