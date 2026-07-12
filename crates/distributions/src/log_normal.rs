use core::{convert::Infallible, f64::consts::SQRT_2};

use computare_core::Float;
use computare_special::erf::{erfc, inverse_erfc};

use crate::{Continuous, Statistics};

#[derive(Clone, Copy, Debug)]
pub struct LogNormal {
    location: f64,
    scale: f64,
}

impl LogNormal {
    pub fn try_new(location: f64, scale: f64) -> Option<Self> {
        if scale > 0.0 {
            Some(LogNormal { location, scale })
        } else {
            None
        }
    }

    pub fn new(location: f64, scale: f64) -> Self {
        Self::try_new(location, scale).unwrap()
    }

    pub fn location(&self) -> f64 {
        self.location
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }
}

impl Continuous for LogNormal {
    fn pdf(&self, x: f64) -> f64 {
        if x <= 0.0 || x == f64::INFINITY {
            0.0
        } else {
            let norm = (x * self.scale * f64::SQRT_2PI).recip();
            let z_score = (x.ln() - self.location) / self.scale;

            norm * (z_score.powi(2) / 2.0).exp()
        }
    }

    fn ln_pdf(&self, x: f64) -> f64 {
        if x <= 0.0 || x.is_infinite() {
            f64::NEG_INFINITY
        } else {
            let z_score = (x.ln() - self.location) / self.scale;
            -z_score.powi(2) / 2.0 - f64::LN_SQRT_2PI - (x * self.scale).ln()
        }
    }

    fn cdf(&self, x: f64) -> f64 {
        if x <= 0.0 {
            0.0
        } else if x == f64::INFINITY {
            1.0
        } else {
            let fr = -(x.ln() - self.location) / (self.scale * SQRT_2);
            0.5 * erfc(fr)
        }
    }

    fn inverse_cdf(&self, p: f64) -> f64 {
        if !(0.0..=1.0).contains(&p) {
            f64::NAN
        } else if p == 0.0 {
            0.0
        } else if p == 1.0 {
            f64::INFINITY
        } else {
            let (location, scale) = (self.location, self.scale);
            (location - (scale * SQRT_2 * inverse_erfc(2.0 * p))).exp()
        }
    }

    fn support(&self) -> (f64, f64) {
        (0.0, f64::INFINITY)
    }
}

impl Statistics for LogNormal {
    type MeanErr = Infallible;
    fn mean(&self) -> Result<f64, Infallible> {
        Ok((self.location + self.scale.powi(2) / 2.0).exp())
    }

    type MedianErr = Infallible;
    fn median(&self) -> Result<f64, Infallible> {
        Ok(self.location.exp())
    }

    type ModeErr = Infallible;
    fn mode(&self) -> Result<f64, Infallible> {
        Ok(self.location - self.scale.powi(2))
    }

    type VarianceErr = Infallible;
    fn variance(&self) -> Result<f64, Infallible> {
        let s2 = self.scale.powi(2);
        Ok((s2.exp() - 1.0) * (2.0 * self.location + s2).exp())
    }

    type EntropyErr = Infallible;
    fn entropy(&self) -> Result<f64, Infallible> {
        Ok(0.5 + self.scale.ln() + self.location + f64::LN_SQRT_2PI)
    }
}

#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
impl rand::distr::Distribution<f64> for LogNormal {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        crate::normal::sample_unchecked(rng, self.location, self.scale).exp()
    }
}
