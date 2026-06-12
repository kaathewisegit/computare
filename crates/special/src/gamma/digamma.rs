use core::f64::consts::PI;

pub fn digamma(x: f64) -> f64 {
    // The implementation is based on "Algorithm AS 103", Jose Bernardo, Applied
    // Statistics, Volume 25, Number 3 1976, pages 315 - 317.

    let c = 12.0;
    let d1 = -0.5772156649015329;
    let d2 = 1.6449340668482264;
    let s = 1e-6;
    let s3 = 1.0 / 12.0;
    let s4 = 1.0 / 120.0;
    let s5 = 1.0 / 252.0;
    let s6 = 1.0 / 240.0;
    let s7 = 1.0 / 132.0;

    if x == f64::NEG_INFINITY || x.is_nan() {
        return f64::NAN;
    }
    if x <= 0.0 && x.floor() == x {
        return f64::NEG_INFINITY;
    }
    if x < 0.0 {
        return digamma(1.0 - x) + PI / (-PI * x).tan();
    }
    if x <= s {
        return d1 - 1.0 / x + d2 * x;
    }

    let mut result = 0.0;
    let mut z = x;
    while z < c {
        result -= 1.0 / z;
        z += 1.0;
    }

    if z >= c {
        let mut r = 1.0 / z;
        result += z.ln() - 0.5 * r;
        r *= r;

        result -= r * (s3 - r * (s4 - r * (s5 - r * (s6 - r * s7))));
    }
    result
}
