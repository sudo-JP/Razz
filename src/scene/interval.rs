use std::f64::{NEG_INFINITY, INFINITY};

pub struct Interval {
    pub min: f64, 
    pub max: f64,
}


impl Interval {
    pub fn new() -> Self {
        Self { min: NEG_INFINITY, max: INFINITY }
    }

    pub fn new_with_val(min: f64, max: f64) -> Self {
        Self { min: min, max: max }
    }

    pub fn size(&self) -> f64 { self.max - self.min }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max 
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max 
    }

    pub const EMPTY: Interval = Interval {
        min: INFINITY, 
        max: NEG_INFINITY,
    };

    pub const UNIVERSE: Interval = Interval {
        min: NEG_INFINITY, 
        max: INFINITY
    };
}
