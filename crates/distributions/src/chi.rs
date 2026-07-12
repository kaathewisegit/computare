use core::{convert::Infallible, f64::consts, num::NonZeroU32};

use computare_special::gamma::{
    digamma, gamma, inverse_lower_gamma, ln_gamma, regularized_lower_gamma,
    regularized_upper_gamma,
};

use crate::{Continuous, Statistics};

/// [Chi distribution](https://en.wikipedia.org/wiki/Chi_distribution)
#[derive(Clone, Copy, Debug)]
pub struct Chi {
    freedom: NonZeroU32,
}

impl Chi {
    pub fn try_new(freedom: f64) -> Option<Self> {
        if freedom >= 1.0 {
            let clamped = freedom.clamp(1.0, f64::from(u32::MAX));
            Some(Chi {
                freedom: NonZeroU32::new(clamped as u32).unwrap(),
            })
        } else {
            None
        }
    }

    pub fn new(freedom: NonZeroU32) -> Self {
        Chi { freedom }
    }

    pub fn freedom(&self) -> NonZeroU32 {
        self.freedom
    }

    pub fn freedom_u(&self) -> u32 {
        self.freedom.get()
    }

    pub fn freedom_f(&self) -> f64 {
        f64::from(self.freedom_u())
    }
}

impl Continuous for Chi {
    /// `(2^(1 - (k / 2)) * x^(k - 1) * e^(-x^2 / 2)) / Γ(k / 2)`
    ///
    /// Where `k` is the degrees of freedom and `Γ` is the gamma function.
    fn pdf(&self, x: f64) -> f64 {
        if x == f64::INFINITY || x <= 0.0 {
            0.0
        } else if self.freedom_u() > 160 {
            self.ln_pdf(x).exp()
        } else {
            let freedom = self.freedom_f();
            (2.0f64).powf(1.0 - freedom / 2.0)
                * x.powf(freedom - 1.0)
                * (-x * x / 2.0).exp()
                / gamma(freedom / 2.0)
        }
    }

    /// `ln((2^(1 - (k / 2)) * x^(k - 1) * e^(-x^2 / 2)) / Γ(k / 2))`
    fn ln_pdf(&self, x: f64) -> f64 {
        if x == f64::INFINITY || x <= 0.0 {
            f64::NEG_INFINITY
        } else {
            let freedom = self.freedom_f();
            (1.0 - freedom / 2.0) * (2.0f64).ln() + (freedom - 1.0) * x.ln()
                - x * x / 2.0
                - ln_gamma(freedom / 2.0)
        }
    }

    /// `P(k / 2, x^2 / 2)`
    ///
    /// `P` is the regularized lower incomplete Gamma function.
    fn cdf(&self, x: f64) -> f64 {
        if x == f64::INFINITY {
            1.0
        } else if x <= 0.0 {
            0.0
        } else {
            regularized_lower_gamma(self.freedom_f() / 2.0, x * x / 2.0)
        }
    }

    /// `P(k / 2, x^2 / 2)`
    ///
    /// Where `P` is the regularized upper incomplete Gamma function.
    fn sf(&self, x: f64) -> f64 {
        if x == f64::INFINITY {
            0.0
        } else if x <= 0.0 {
            1.0
        } else {
            regularized_upper_gamma(self.freedom_f() / 2.0, x * x / 2.0)
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
            (2.0 * inverse_lower_gamma(self.freedom_f() / 2.0, p)).sqrt()
        }
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

impl Statistics for Chi {
    type MeanErr = Infallible;
    /// `sqrt2 * Γ((k + 1) / 2) / Γ(k / 2)`
    fn mean(&self) -> Result<f64, Infallible> {
        let freedom = self.freedom_f();

        if self.freedom_u() > 300 {
            // Large n approximation based on the Stirling series approximation
            // to the Gamma function This avoids call the Gamma function with
            // large arguments and returning NaN
            //
            // Relative accuracy follows O(1/n^4) and at 300 d.o.f. is better
            // than 1e-12 For a f32 impl the threshold should be changed to 150
            Ok(freedom.sqrt()
                / ((1.0 + 0.25 / freedom)
                    * (1.0 + 0.03125 / (freedom * freedom))
                    * (1.0 - 0.046875 / (freedom * freedom * freedom))))
        } else {
            Ok(consts::SQRT_2 * gamma((freedom + 1.0) / 2.0)
                / gamma(freedom / 2.0))
        }
    }

    type MedianErr = &'static str;
    fn median(&self) -> Result<f64, &'static str> {
        Err("Simple closed form does not exist")
    }

    type ModeErr = Infallible;
    /// `sqrt(k - 1)`
    fn mode(&self) -> Result<f64, Infallible> {
        if self.freedom_u() >= 1 {
            Ok((self.freedom_f() - 1.0).sqrt())
        } else {
            Ok(0.0)
        }
    }

    type VarianceErr = Infallible;
    /// `k - μ^2`, where `μ` is the [mean]
    ///
    /// [mean]: Statistics::mean
    fn variance(&self) -> Result<f64, Infallible> {
        let mean = self.mean().unwrap();
        Ok(self.freedom_f() - mean * mean)
    }

    type EntropyErr = Infallible;
    /// `ln(Γ(k / 2)) + 0.5 * (k - ln2 - (k - 1) * ψ(k / 2))`
    ///
    /// Where `k` is degrees of freedom, `Γ` is the gamma function, and `ψ`
    /// is the digamma function.
    fn entropy(&self) -> Result<f64, Infallible> {
        let freedom = self.freedom_f();
        Ok(ln_gamma(freedom / 2.0)
            + (freedom
                - (2.0f64).ln()
                - (freedom - 1.0) * digamma(freedom / 2.0))
                / 2.0)
    }
}

#[cfg(feature = "rand")]
#[cfg_attr(docsrs, doc(cfg(feature = "rand")))]
impl rand::distr::Distribution<f64> for Chi {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        (0..self.freedom_u() as usize)
            .fold(0.0, |acc, _| {
                acc + super::normal::sample_unchecked(rng, 0.0, 1.0).powf(2.0)
            })
            .sqrt()
    }
}
