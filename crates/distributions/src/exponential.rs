use core::{convert::Infallible, f64::consts::LN_2};

use super::Continuous;
use crate::Statistics;

pub struct Exponential {
    rate: f64,
}

impl Exponential {
    pub fn new(rate: f64) -> Self {
        debug_assert!(rate > 0.0);
        Exponential { rate }
    }

    pub fn new_with_scale(scale: f64) -> Self {
        debug_assert!(scale > 0.0);
        Exponential {
            rate: scale.recip(),
        }
    }
}

impl Continuous for Exponential {
    fn pdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            0.0
        } else {
            self.rate * (-self.rate * x).exp()
        }
    }

    fn ln_pdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            f64::NEG_INFINITY
        } else {
            self.rate.ln() - self.rate * x
        }
    }

    fn cdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            0.0
        } else {
            1.0 - (-self.rate * x).exp()
        }
    }

    fn sf(&self, x: f64) -> f64 {
        if x < 0.0 { 1.0 } else { (-self.rate * x).exp() }
    }

    fn support(&self) -> (f64, f64) {
        (0.0, f64::INFINITY)
    }
}

impl Statistics for Exponential {
    type MeanErr = Infallible;
    fn mean(&self) -> Result<f64, Infallible> {
        Ok(self.rate.recip())
    }

    type MedianErr = Infallible;
    fn median(&self) -> Result<f64, Infallible> {
        Ok(LN_2 / self.rate)
    }

    type ModeErr = Infallible;
    fn mode(&self) -> Result<f64, Infallible> {
        Ok(0.0)
    }

    type VarianceErr = Infallible;
    fn variance(&self) -> Result<f64, Infallible> {
        Ok(1.0 / self.rate.powi(2))
    }
    fn std(&self) -> Result<f64, Infallible> {
        Ok(self.rate.recip())
    }

    type EntropyErr = Infallible;
    fn entropy(&self) -> Result<f64, Infallible> {
        Ok(1.0 - self.rate.ln())
    }
}
