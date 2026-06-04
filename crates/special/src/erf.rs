use core::f64::consts::{FRAC_1_SQRT_2, FRAC_2_SQRT_PI};

use crate::ndtri::ndtri;
use computare_core::debug_panic;

pub fn erf(x: f64) -> f64 {
    libm::erf(x)
}

pub fn erfc(x: f64) -> f64 {
    libm::erfc(x)
}

/// Inverse of the error function.
///
/// If `y ∉ (-1, 1)` this function panics in debug builds and returns `NaN` in
/// release ones.
pub fn inverse_erf(y: f64) -> f64 {
    const DOMAIN_LB: f64 = -1.0;
    const DOMAIN_UB: f64 = 1.0;
    const THRESH: f64 = 1e-7;

    // For small arguments, use the Taylor expansion `erf(y) = 2/√π (y - y^3 / 3
    // + O(y^5))`, `y → 0` where we only retain the linear term.  Otherwise, `y
    // + 1` loses precision for `|y| << 1`.
    if -THRESH < y && y < THRESH {
        return y / FRAC_2_SQRT_PI;
    }

    if DOMAIN_LB < y && y < DOMAIN_UB {
        ndtri(0.5 * (y + 1.0)) * FRAC_1_SQRT_2
    } else if y == DOMAIN_LB {
        f64::NEG_INFINITY
    } else if y == DOMAIN_UB {
        f64::INFINITY
    } else if y.is_nan() {
        y
    } else {
        debug_panic!("inverse_erf argument outside of (-1, 1)");
        f64::NAN
    }
}

/// Inverse of the complementary error function.
///
/// If `y ∉ (0, 2)` this function panics in debug builds and returns `NaN` in
/// release ones.
pub fn inverse_erfc(y: f64) -> f64 {
    const DOMAIN_LB: f64 = 0.0;
    const DOMAIN_UB: f64 = 2.0;

    if DOMAIN_LB < y && y < DOMAIN_UB {
        -ndtri(0.5 * y) * FRAC_1_SQRT_2
    } else if y == DOMAIN_LB {
        f64::INFINITY
    } else if y == DOMAIN_UB {
        f64::NEG_INFINITY
    } else if y.is_nan() {
        y
    } else {
        debug_panic!("inverse_erfc argument outside of (0, 2)");
        f64::NAN
    }
}
