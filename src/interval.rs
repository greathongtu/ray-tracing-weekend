#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: std::f64::INFINITY,
            max: std::f64::NEG_INFINITY,
        }
    }
}

impl Interval {
    pub const EMPTY: Self = Interval {
        min: std::f64::INFINITY,
        max: std::f64::NEG_INFINITY,
    };
    pub const UNIVERSE: Self = Interval {
        min: std::f64::NEG_INFINITY,
        max: std::f64::INFINITY,
    };
    pub fn new(min: f64, max: f64) -> Self {
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
