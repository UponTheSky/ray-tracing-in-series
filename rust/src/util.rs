// constants
pub const INFINITY: f64 = std::f64::INFINITY; 
pub const NEG_INFINITY: f64 = std::f64::NEG_INFINITY;
pub const PI: f64 = std::f64::consts::PI;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub mod interval {
    use super::{INFINITY, NEG_INFINITY};

    #[derive(Clone, Copy)]
    pub struct Interval {
        pub min: f64, 
        pub max: f64
    }

    impl Interval {
        pub const fn new_default() -> Self {
            Self { min: NEG_INFINITY, max: INFINITY }
        } 

        pub const fn new(min: f64, max: f64) -> Self {
            Self { min, max }
        }

        pub fn size(&self) -> f64 {
            self.max - self.min
        }

        pub fn contains(&self, x: f64) -> bool {
            self.min <= x && x <= self.max
        }

        pub fn surrounds(&self, x: f64) -> bool {
            self.min < x && x < self.max
        }
    }

    pub static EMPTY: Interval = Interval::new(INFINITY, -INFINITY);
    pub static UNIVERSE: Interval = Interval::new(-INFINITY, INFINITY);
}