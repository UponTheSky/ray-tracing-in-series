#[derive(Clone, Debug)]
pub struct Vector3(f64, f64, f64);

impl Vector3 {
    pub fn new() -> Self {
        Self(0.0, 0.0, 0.0)
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
        Self(-self.0, -self.1, -self.2)
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

    pub fn normalize(&self) -> Self {
        let mut clone = self.clone();

        clone.scalar_multiply(clone.length_squared());

        clone
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

// TODO: add tests
