use core::{
    convert::Infallible,
    f64::consts::{E, SQRT_2},
};

use crate::{Continuous, Statistics};

pub struct Laplace {
    location: f64,
    scale: f64,
}

impl Laplace {
    pub fn new(location: f64, scale: f64) -> Self {
        debug_assert!(scale > 0.0);
        Laplace { location, scale }
    }

    pub fn location(&self) -> f64 {
        self.location
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }
}

impl Continuous for Laplace {
    fn pdf(&self, x: f64) -> f64 {
        let (location, scale) = (self.location, self.scale);
        (-(x - location).abs() / scale).exp() / (2.0 * scale)
    }

    fn ln_pdf(&self, x: f64) -> f64 {
        let (location, scale) = (self.location, self.scale);
        (-(x - location).abs() / scale) - (2.0 * scale).ln()
    }

    fn cdf(&self, x: f64) -> f64 {
        let (location, scale) = (self.location, self.scale);
        let z_score = (x - location) / scale;
        if x <= location {
            z_score.exp() / 2.0
        } else {
            1.0 - (-z_score).exp() / 2.0
        }
    }

    fn inverse_cdf(&self, p: f64) -> f64 {
        let (location, scale) = (self.location, self.scale);
        if p <= 0.5 {
            location + scale * (2.0 * p).ln()
        } else {
            location - scale * (2.0 - 2.0 * p).ln()
        }
    }

    fn support(&self) -> (f64, f64) {
        (f64::NEG_INFINITY, f64::INFINITY)
    }
}

impl Statistics for Laplace {
    type MeanErr = Infallible;
    fn mean(&self) -> Result<f64, Infallible> {
        Ok(self.location)
    }

    type MedianErr = Infallible;
    fn median(&self) -> Result<f64, Infallible> {
        Ok(self.location)
    }

    type ModeErr = Infallible;
    fn mode(&self) -> Result<f64, Infallible> {
        Ok(self.location)
    }

    type VarianceErr = Infallible;
    fn variance(&self) -> Result<f64, Infallible> {
        Ok(2.0 * self.scale.powi(2))
    }
    fn std(&self) -> Result<f64, Infallible> {
        Ok(SQRT_2 * self.scale)
    }

    type EntropyErr = Infallible;
    fn entropy(&self) -> Result<f64, Infallible> {
        Ok((2.0 * self.scale * E).ln())
    }
}

#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
impl rand::distr::Distribution<f64> for Laplace {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        use rand::RngExt;
        let x: f64 = rng.random_range(-0.5..0.5);
        self.location - self.scale * x.signum() * (1.0 - 2.0 * x.abs()).ln()
    }
}
