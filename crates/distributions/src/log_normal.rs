use core::{convert::Infallible, f64::consts::SQRT_2};

use computare_core::Float;
use computare_special::erf::erfc;

use crate::{Continuous, Statistics};

pub struct LogNormal {
    location: f64,
    scale: f64,
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

    // TODO: ln_pdf

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
            todo!("inverse erf")
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
