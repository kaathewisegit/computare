use core::{
    convert::Infallible,
    f64::consts::{E, PI, SQRT_2},
};

use crate::{Continuous, Statistics};
use computare_core::Float;
use computare_special::erf::{erfc, inverse_erfc};

pub struct Normal {
    mean: f64,
    std: f64,
}

impl Normal {
    pub fn new(mean: f64, scale: f64) -> Self {
        Normal { mean, std: scale }
    }

    pub fn mean(&self) -> f64 {
        self.mean
    }

    pub fn std(&self) -> f64 {
        self.std
    }
}

impl Continuous for Normal {
    fn pdf(&self, x: f64) -> f64 {
        let (mean, std) = (self.mean, self.std);
        let z_score = (x - mean) / std;
        f64::FRAC_1_SQRT_2PI / std * (-z_score.powi(2) / 2.0).exp()
    }

    fn ln_pdf(&self, x: f64) -> f64 {
        let (mean, std) = (self.mean, self.std);
        let z_score = (x - mean) / std;
        -std.ln() - f64::LN_SQRT_2PI - z_score.powi(2) / 2.0
    }

    fn cdf(&self, x: f64) -> f64 {
        let (mean, std) = (self.mean, self.std);
        0.5 * erfc((mean - x) / (std * SQRT_2))
    }

    fn inverse_cdf(&self, p: f64) -> f64 {
        let (mean, std) = (self.mean, self.std);
        mean - (std * SQRT_2 * inverse_erfc(2.0 * p))
    }

    fn sf(&self, x: f64) -> f64 {
        let (mean, std) = (self.mean, self.std);
        0.5 * erfc((x - mean) / (std * SQRT_2))
    }

    fn support(&self) -> (f64, f64) {
        (f64::NEG_INFINITY, f64::INFINITY)
    }
}

impl Statistics for Normal {
    type MeanErr = Infallible;
    fn mean(&self) -> Result<f64, Infallible> {
        Ok(self.mean)
    }

    type MedianErr = Infallible;
    fn median(&self) -> Result<f64, Infallible> {
        Ok(self.mean)
    }

    type ModeErr = Infallible;
    fn mode(&self) -> Result<f64, Infallible> {
        Ok(self.mean)
    }

    type VarianceErr = Infallible;
    fn variance(&self) -> Result<f64, Infallible> {
        Ok(self.std.powi(2))
    }
    fn std(&self) -> Result<f64, Infallible> {
        Ok(self.std)
    }

    type EntropyErr = Infallible;
    fn entropy(&self) -> Result<f64, Infallible> {
        Ok((2.0 * PI * E * self.std.powi(2)).ln() / 2.0)
    }
}

#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
impl rand::distr::Distribution<f64> for Normal {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        self.mean + self.std * crate::ziggurat::sample_std_normal(rng)
    }
}

#[cfg(feature = "rand")]
pub(crate) fn sample_unchecked<R>(rng: &mut R, mean: f64, std_dev: f64) -> f64
where
    R: rand::Rng + ?Sized,
{
    mean + std_dev * crate::ziggurat::sample_std_normal(rng)
}
