use core::convert::Infallible;

use crate::{Continuous, Statistics};

pub struct Uniform {
    min: f64,
    max: f64,
}

impl Uniform {
    pub fn new(min: f64, max: f64) -> Self {
        debug_assert!(min <= max);
        Uniform { min, max }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }
}

impl Continuous for Uniform {
    fn pdf(&self, x: f64) -> f64 {
        let (min, max) = (self.min, self.max);
        if !(min..=max).contains(&x) {
            0.0
        } else {
            1.0 / (max - min)
        }
    }

    fn cdf(&self, x: f64) -> f64 {
        let (min, max) = (self.min, self.max);
        if x < self.min {
            0.0
        } else if x > self.max {
            1.0
        } else {
            (x - min) / (max - min)
        }
    }

    fn inverse_cdf(&self, p: f64) -> f64 {
        if !(0.0..=1.0).contains(&p) {
            f64::NAN
        } else if p == 0.0 {
            self.min
        } else if p == 1.0 {
            self.max
        } else {
            (self.max - self.min) * p + self.min
        }
    }

    fn support(&self) -> (f64, f64) {
        (self.min, self.max)
    }
}

impl Statistics for Uniform {
    type MeanErr = Infallible;
    fn mean(&self) -> Result<f64, Infallible> {
        Ok((self.min + self.max) / 2.0)
    }

    type MedianErr = Infallible;
    fn median(&self) -> Result<f64, Infallible> {
        self.mean()
    }

    type ModeErr = &'static str;
    fn mode(&self) -> Result<f64, &'static str> {
        Err("any value in (self.min, self.max)")
    }

    type VarianceErr = Infallible;
    fn variance(&self) -> Result<f64, Infallible> {
        Ok((self.max - self.min).powi(2) / 12.0)
    }
    fn std(&self) -> Result<f64, Infallible> {
        Ok((self.max - self.min) / 12.0f64.sqrt())
    }

    type EntropyErr = Infallible;
    fn entropy(&self) -> Result<f64, Infallible> {
        Ok((self.max - self.min).ln())
    }
}
