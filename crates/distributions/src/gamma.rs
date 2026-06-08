use core::convert::Infallible;

use computare_special::gamma::{
    digamma, gamma, ln_gamma, regularized_lower_gamma, regularized_upper_gamma,
};

use crate::{Continuous, Statistics};

pub struct Gamma {
    shape: f64,
    scale: f64,
}

impl Gamma {
    pub fn new(shape: f64, scale: f64) -> Self {
        debug_assert!(shape > 0.0);
        debug_assert!(scale > 0.0);
        Gamma { shape, scale }
    }

    pub fn new_with_rate(shape: f64, rate: f64) -> Self {
        debug_assert!(shape > 0.0);
        debug_assert!(rate > 0.0);
        Gamma {
            shape,
            scale: rate.recip(),
        }
    }

    pub fn shape(&self) -> f64 {
        self.shape
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn rate(&self) -> f64 {
        self.scale.recip()
    }
}

impl Continuous for Gamma {
    fn pdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            0.0
        } else if x == f64::INFINITY {
            1.0
        } else {
            (x / self.scale).powf(self.shape - 1.0) * (-x / self.scale).exp()
                / (self.scale * gamma(self.shape))
        }
    }

    fn ln_pdf(&self, x: f64) -> f64 {
        if x < 0.0 || x == f64::INFINITY {
            f64::NEG_INFINITY
        } else {
            (self.shape - 1.0) * x.ln()
                - x / self.scale
                - self.shape * self.scale.ln()
                - ln_gamma(self.shape)
        }
    }

    fn cdf(&self, x: f64) -> f64 {
        if x <= 0.0 {
            0.0
        } else if x == f64::INFINITY {
            1.0
        } else {
            regularized_lower_gamma(self.shape, x / self.scale)
        }
    }

    fn sf(&self, x: f64) -> f64 {
        if x <= 0.0 || x == f64::INFINITY {
            1.0
        } else {
            regularized_upper_gamma(self.shape, x / self.scale)
        }
    }

    fn support(&self) -> (f64, f64) {
        (0.0, f64::INFINITY)
    }
}

impl Statistics for Gamma {
    type MeanErr = Infallible;
    fn mean(&self) -> Result<f64, Infallible> {
        let (shape, scale) = (self.shape, self.scale);
        Ok(shape * scale)
    }

    type MedianErr = &'static str;
    fn median(&self) -> Result<f64, &'static str> {
        Err("Simple closed form doesn not exist")
    }

    type ModeErr = Infallible;
    fn mode(&self) -> Result<f64, Infallible> {
        let (shape, scale) = (self.shape, self.scale);
        if shape >= 1.0 {
            Ok((shape - 1.0) * scale)
        } else {
            Ok(0.0)
        }
    }

    type VarianceErr = Infallible;
    fn variance(&self) -> Result<f64, Infallible> {
        let (shape, scale) = (self.shape, self.scale);
        Ok(shape * scale.powi(2))
    }
    fn std(&self) -> Result<f64, Infallible> {
        let (shape, scale) = (self.shape, self.scale);
        Ok(shape.sqrt() * scale)
    }

    type EntropyErr = Infallible;
    fn entropy(&self) -> Result<f64, Infallible> {
        let (shape, scale) = (self.shape, self.scale);
        Ok(shape
            + scale.ln()
            + ln_gamma(shape)
            + (1.0 - shape) * digamma(shape))
    }
}
