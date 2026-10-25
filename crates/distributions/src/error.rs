//! Error types returned by [`Statistics`] methods.

use core::fmt::{Display, Formatter, Result};

/// The mean of an [`InverseGamma`] distribution is undefined.
///
/// [`InverseGamma`]: crate::InverseGamma
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InverseGammaUndefinedMean {
    pub shape: f64,
}

/// The variance of an [`InverseGamma`] distribution is undefined.
///
/// [`InverseGamma`]: crate::InverseGamma
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InverseGammaUndefinedVariance {
    pub shape: f64,
}

/// The mode of a [`Beta`] distribution is undefined.
///
/// [`Beta`]: crate::Beta
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BetaUndefinedMode {
    pub shape_a: f64,
    pub shape_b: f64,
}

/// No simple closed form exists for the requested statistic.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoClosedFormError;

/// The distribution has no unique mode.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoUniqueModeError;

macro_rules! impl_error {
    ($(
        $t:ty => {$message:literal $(, $field:ident)* $(,)?}
    ),* $(,)?) => {
        $(
            impl Display for $t {
                fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                    write!(f, $message $(, self.$field)*)
                }
            }

            impl core::error::Error for $t {}
        )*
    };
}

impl_error!(
    InverseGammaUndefinedMean => {
        "the mean is undefined for inverse gamma distribution with shape {} (must be > 1)",
        shape,
    },
    InverseGammaUndefinedVariance => {
        "the variance is undefined for an inverse gamma distribution with shape {} (must be > 2)",
        shape,
    },
    BetaUndefinedMode => {
        "the mode is undefined for a beta distribution with arguments {} and {} (both must be > 1)",
        shape_a, shape_b,
    },
    NoClosedFormError => {"no simple closed form exists for this statistic"},
    NoUniqueModeError => {"the distribution has no unique mode"},
);
