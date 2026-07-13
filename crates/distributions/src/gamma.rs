use core::convert::Infallible;

use computare_special::gamma::{
    digamma, gamma, inverse_lower_gamma, ln_gamma, regularized_lower_gamma,
    regularized_upper_gamma,
};

use crate::{Continuous, Statistics};

#[derive(Clone, Copy, Debug)]
pub struct Gamma {
    shape: f64,
    scale: f64,
}

impl Gamma {
    pub fn try_new(shape: f64, scale: f64) -> Option<Self> {
        if shape > 0.0 && scale > 0.0 {
            Some(Gamma { shape, scale })
        } else {
            None
        }
    }

    pub fn new(shape: f64, scale: f64) -> Self {
        Self::try_new(shape, scale).unwrap()
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
        } else if self.shape == 1.0 {
            -self.scale.ln() - x / self.scale
        } else {
            -self.shape * self.scale.ln() + (self.shape - 1.0) * x.ln()
                - (x / self.scale)
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

    fn inverse_cdf(&self, p: f64) -> f64 {
        if !(0.0..=1.0).contains(&p) {
            f64::NAN
        } else if p == 0.0 {
            0.0
        } else if p == 1.0 {
            f64::INFINITY
        } else {
            self.scale * inverse_lower_gamma(self.shape, p)
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

#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
impl rand::distr::Distribution<f64> for Gamma {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        sample_unchecked(rng, self.shape, self.scale)
    }
}

/// Implementation from:
///
/// _"A Simple Method for Generating Gamma Variables"_ - Marsaglia & Tsang
///
/// ACM Transactions on Mathematical Software, Vol. 26, No. 3, September 2000,
/// Pages 363-372
#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
pub(crate) fn sample_unchecked<R: rand::Rng + ?Sized>(
    rng: &mut R,
    shape: f64,
    scale: f64,
) -> f64 {
    use rand::RngExt;

    let mut a = shape;
    let mut afix = 1.0;
    if shape < 1.0 {
        a = shape + 1.0;
        afix = rng.random::<f64>().powf(1.0 / shape);
    }

    let d = a - 1.0 / 3.0;
    let c = 1.0 / (9.0 * d).sqrt();
    loop {
        let mut x;
        let mut v;
        loop {
            x = super::normal::sample_unchecked(rng, 0.0, 1.0);
            v = 1.0 + c * x;
            if v > 0.0 {
                break;
            };
        }

        v = v * v * v;
        x = x * x;
        let u: f64 = rng.random();
        if u < 1.0 - 0.0331 * x * x || u.ln() < 0.5 * x + d * (1.0 - v + v.ln())
        {
            return afix * d * v * scale;
        }
    }
}
