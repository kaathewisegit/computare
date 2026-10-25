use core::convert::Infallible;

use computare_special::gamma::{
    digamma, gamma, inverse_upper_gamma, ln_gamma, regularized_lower_gamma,
    regularized_upper_gamma,
};

use crate::{
    Continuous, Statistics,
    error::{
        InverseGammaUndefinedMean, InverseGammaUndefinedVariance,
        NoClosedFormError,
    },
};

#[derive(Clone, Copy, Debug)]
pub struct InverseGamma {
    shape: f64,
    scale: f64,
}

impl InverseGamma {
    pub fn try_new(shape: f64, scale: f64) -> Option<Self> {
        if shape > 0.0 && scale > 0.0 {
            Some(InverseGamma { shape, scale })
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

impl Continuous for InverseGamma {
    fn pdf(&self, x: f64) -> f64 {
        if x <= 0.0 || x.is_infinite() {
            0.0
        } else {
            self.scale.powf(self.shape)
                * x.powf(-self.shape - 1.0)
                * (-self.scale / x).exp()
                / gamma(self.shape)
        }
    }

    fn ln_pdf(&self, x: f64) -> f64 {
        if x <= 0.0 || x.is_infinite() {
            f64::NEG_INFINITY
        } else {
            self.shape * self.scale.ln()
                - ln_gamma(self.shape)
                - (self.shape + 1.0) * x.ln()
                - self.scale / x
        }
    }

    fn cdf(&self, x: f64) -> f64 {
        if x <= 0.0 {
            0.0
        } else if x.is_infinite() {
            1.0
        } else {
            regularized_upper_gamma(self.shape, self.scale / x)
        }
    }

    fn sf(&self, x: f64) -> f64 {
        if x <= 0.0 {
            1.0
        } else if x.is_infinite() {
            0.0
        } else {
            regularized_lower_gamma(self.shape, self.scale / x)
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
            self.scale / inverse_upper_gamma(self.shape, p)
        }
    }

    fn support(&self) -> (f64, f64) {
        (0.0, f64::INFINITY)
    }
}

impl Statistics for InverseGamma {
    type MeanErr = InverseGammaUndefinedMean;
    fn mean(&self) -> Result<f64, Self::MeanErr> {
        if self.shape <= 1.0 {
            Err(InverseGammaUndefinedMean { shape: self.shape })
        } else {
            Ok(self.scale / (self.shape - 1.0))
        }
    }

    type MedianErr = NoClosedFormError;
    fn median(&self) -> Result<f64, Self::MedianErr> {
        Err(NoClosedFormError)
    }

    type ModeErr = Infallible;
    fn mode(&self) -> Result<f64, Infallible> {
        Ok(self.scale / (self.shape + 1.0))
    }

    type VarianceErr = InverseGammaUndefinedVariance;
    fn variance(&self) -> Result<f64, Self::VarianceErr> {
        if self.shape <= 2.0 {
            Err(InverseGammaUndefinedVariance { shape: self.shape })
        } else {
            let s = self.shape;
            Ok(self.scale * self.scale / ((s - 1.0) * (s - 1.0) * (s - 2.0)))
        }
    }

    type EntropyErr = Infallible;
    fn entropy(&self) -> Result<f64, Infallible> {
        let s = self.shape;
        Ok(s + self.scale.ln() + ln_gamma(s) - (1.0 + s) * digamma(s))
    }
}

#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
impl rand::distr::Distribution<f64> for InverseGamma {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        1.0 / super::gamma::sample_unchecked(
            rng,
            self.shape,
            self.scale.recip(),
        )
    }
}
