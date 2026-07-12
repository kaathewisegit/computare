use core::convert::Infallible;

use computare_special::{
    beta::{inverse_regularized_beta, ln_beta, regularized_incomplete_beta},
    gamma::{digamma, gamma, ln_gamma},
};

use crate::{Continuous, Statistics};

#[derive(Clone, Copy, Debug)]
pub struct Beta {
    shape_a: f64,
    shape_b: f64,
}

impl Beta {
    pub fn new(shape_a: f64, shape_b: f64) -> Self {
        debug_assert!(shape_a > 0.0);
        debug_assert!(shape_b > 0.0);
        Beta { shape_a, shape_b }
    }

    pub fn shape_a(&self) -> f64 {
        self.shape_a
    }

    pub fn shape_b(&self) -> f64 {
        self.shape_b
    }
}

impl Continuous for Beta {
    fn pdf(&self, x: f64) -> f64 {
        let (a, b) = (self.shape_a, self.shape_b);
        if !(0.0..=1.0).contains(&x) {
            0.0
        } else if a == 1.0 && b == 1.0 {
            1.0
        } else if a > 80.0 || b > 80.0 {
            self.ln_pdf(x).exp()
        } else {
            let bb = gamma(a + b) / (gamma(a) * gamma(b));
            bb * x.powf(a - 1.0) * (1.0 - x).powf(b - 1.0)
        }
    }

    fn ln_pdf(&self, x: f64) -> f64 {
        let (a, b) = (self.shape_a, self.shape_b);
        if !(0.0..=1.0).contains(&x) {
            f64::NEG_INFINITY
        } else if a == 1.0 && b == 1.0 {
            0.0
        } else {
            let aa = ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b);
            let bb = if a == 1.0 && x == 0.0 {
                0.0
            } else if x == 0.0 {
                f64::NEG_INFINITY
            } else {
                (a - 1.0) * x.ln()
            };
            let cc = if b == 1.0 && x == 1.0 {
                0.0
            } else if x == 1.0 {
                f64::NEG_INFINITY
            } else {
                (b - 1.0) * (1.0 - x).ln()
            };
            aa + bb + cc
        }
    }

    fn cdf(&self, x: f64) -> f64 {
        let (a, b) = (self.shape_a, self.shape_b);
        if x < 0.0 {
            0.0
        } else if x >= 1.0 {
            1.0
        } else if a == 1.0 && b == 1.0 {
            x
        } else {
            regularized_incomplete_beta(a, b, x)
        }
    }

    fn sf(&self, x: f64) -> f64 {
        let (a, b) = (self.shape_a, self.shape_b);
        if x < 0.0 {
            1.0
        } else if x >= 1.0 {
            0.0
        } else if a == 1.0 && b == 1.0 {
            1.0 - x
        } else {
            regularized_incomplete_beta(b, a, 1.0 - x)
        }
    }

    fn inverse_cdf(&self, p: f64) -> f64 {
        inverse_regularized_beta(self.shape_a, self.shape_b, p)
    }

    fn support(&self) -> (f64, f64) {
        (0.0, 1.0)
    }
}

impl Statistics for Beta {
    type MeanErr = Infallible;
    fn mean(&self) -> Result<f64, Infallible> {
        let (a, b) = (self.shape_a, self.shape_b);
        Ok(a / (a + b))
    }

    type MedianErr = Infallible;
    fn median(&self) -> Result<f64, Infallible> {
        Ok(inverse_regularized_beta(self.shape_a, self.shape_b, 0.5))
    }

    type ModeErr = &'static str;
    fn mode(&self) -> Result<f64, &'static str> {
        let (a, b) = (self.shape_a, self.shape_b);
        if a <= 1.0 || b <= 1.0 {
            Err("Mode is only defined for α > 1 and β > 1")
        } else {
            Ok((a - 1.0) / (a + b - 2.0))
        }
    }

    type VarianceErr = Infallible;
    fn variance(&self) -> Result<f64, Infallible> {
        let (a, b) = (self.shape_a, self.shape_b);
        Ok(a * b / ((a + b) * (a + b) * (a + b + 1.0)))
    }

    type EntropyErr = Infallible;
    fn entropy(&self) -> Result<f64, Infallible> {
        let (a, b) = (self.shape_a, self.shape_b);
        Ok(
            ln_beta(a, b) - (a - 1.0) * digamma(a) - (b - 1.0) * digamma(b)
                + (a + b - 2.0) * digamma(a + b),
        )
    }
}

#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
impl rand::distr::Distribution<f64> for Beta {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let x = super::gamma::sample_unchecked(rng, self.shape_a, 1.0);
        let y = super::gamma::sample_unchecked(rng, self.shape_b, 1.0);
        x / (x + y)
    }
}
