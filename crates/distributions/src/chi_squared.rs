use core::convert::Infallible;

use crate::{Continuous, Gamma, Statistics};

/// [Chi-squared
/// distribution](https://en.wikipedia.org/wiki/Chi-squared_distribution) which
/// is a special case of the
/// [Gamma](https://en.wikipedia.org/wiki/Gamma_distribution) distribution
/// (referenced [Here](./struct.Gamma.html))
#[derive(Copy, Clone, Debug)]
pub struct ChiSquared {
    freedom: f64,
    g: Gamma,
}

impl ChiSquared {
    pub fn new(freedom: f64) -> Self {
        ChiSquared {
            freedom,
            g: Gamma::new(freedom / 2.0, 0.5),
        }
    }

    pub fn try_new(freedom: f64) -> Option<Self> {
        Some(ChiSquared {
            freedom,
            g: Gamma::try_new(freedom / 2.0, 0.5)?,
        })
    }

    pub fn freedom(&self) -> f64 {
        self.freedom
    }

    pub fn shape(&self) -> f64 {
        self.g.shape()
    }

    pub fn scale(&self) -> f64 {
        self.g.scale()
    }
}

impl Continuous for ChiSquared {
    fn pdf(&self, x: f64) -> f64 {
        self.g.pdf(x)
    }

    fn ln_pdf(&self, x: f64) -> f64 {
        self.g.ln_pdf(x)
    }

    fn cdf(&self, x: f64) -> f64 {
        self.g.cdf(x)
    }

    fn sf(&self, x: f64) -> f64 {
        self.g.sf(x)
    }

    fn inverse_cdf(&self, p: f64) -> f64 {
        self.g.inverse_cdf(p)
    }

    /// Always 0
    fn lower(&self) -> f64 {
        0.0
    }

    /// Always infinity
    fn upper(&self) -> f64 {
        f64::INFINITY
    }

    fn support(&self) -> (f64, f64) {
        (0.0, f64::INFINITY)
    }
}

impl Statistics for ChiSquared {
    type MeanErr = Infallible;
    fn mean(&self) -> Result<f64, Infallible> {
        self.g.mean()
    }

    type MedianErr = Infallible;
    fn median(&self) -> Result<f64, Infallible> {
        let median = if self.freedom < 1.0 {
            // if k is small, calculate using expansion of formula
            self.freedom - 2.0 / 3.0 + 12.0 / (81.0 * self.freedom)
                - 8.0 / (729.0 * self.freedom * self.freedom)
        } else {
            // if k is large enough, median heads toward k - 2/3
            self.freedom - 2.0 / 3.0
        };
        Ok(median)
    }

    type ModeErr = Infallible;
    fn mode(&self) -> Result<f64, Infallible> {
        self.g.mode()
    }

    type VarianceErr = Infallible;
    fn variance(&self) -> Result<f64, Infallible> {
        self.g.variance()
    }

    type EntropyErr = Infallible;
    fn entropy(&self) -> Result<f64, Infallible> {
        self.g.entropy()
    }
}

#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
impl rand::distr::Distribution<f64> for ChiSquared {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        rand::distr::Distribution::sample(&self.g, rng)
    }
}
