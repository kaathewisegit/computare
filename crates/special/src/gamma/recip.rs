use super::gamma;
use crate::evaluate::chebyshev;

// Chebyshev coefficients for reciprocal Gamma function in interval 0 to 1.
// The function is 1/(x Gamma(x)) - 1.
const RGAMMA_R: [f64; 16] = [
    3.1317345823123e-17,
    -6.70718606477908e-16,
    2.2003907817225954e-15,
    2.4769163034825414e-13,
    -6.600741004112952e-12,
    5.13850186324227e-11,
    1.0896538645441867e-9,
    -3.3396463068683694e-8,
    2.6897599644059546e-7,
    2.960011775188017e-6,
    -8.048141249784711e-5,
    4.166091387096889e-4,
    5.065798640286087e-3,
    -6.419254361091582e-2,
    -4.985587286840036e-3,
    1.2754601561052395e-1,
];

pub fn recip_gamma(x: f64) -> f64 {
    if x == 0.0 {
        // This case is separate from below to get correct sign for zero
        return x;
    }

    if x < 0.0 && x == x.floor() {
        // Gamma poles
        return 0.0;
    }

    if x.abs() > 4.0 {
        return 1.0 / gamma(x);
    }

    let mut z = 1.0;
    let mut w = x;

    // Downward recurrence
    while w > 1.0 {
        w -= 1.0;
        z *= w;
    }
    // Upward recurrence
    while w < 0.0 {
        z /= w;
        w += 1.0;
    }
    // Nonpositive integer
    if w == 0.0 {
        return 0.0;
    }
    if w == 1.0 {
        return 1.0 / z;
    }

    w * (1.0 + chebyshev(4.0 * w - 2.0, &RGAMMA_R)) / z
}
