use core::f64::consts::EULER_GAMMA;

use super::{
    gamma,
    incomplete::{igam_fac, regularized_lower_gamma, regularized_upper_gamma},
    ln_gamma,
};
use crate::evaluate::polynomial;

// Computation of the Incomplete Gamma Function Ratios and their Inverse ARMIDO
// R. DIDONATO and ALFRED H. MORRIS, JR.  ACM Transactions on Mathematical
// Software, Vol. 12, No. 4, December 1986, Pages 377-393.
//
// See equation 32.
fn find_inverse_s(p: f64, q: f64) -> f64 {
    const A: [f64; 4] = [
        0.213623493715853,
        4.28342155967104,
        11.6616720288968,
        3.31125922108741,
    ];
    const B: [f64; 5] = [
        0.3611708101884203e-1,
        1.27364489782223,
        6.40691597760039,
        6.61053765625462,
        1.0,
    ];

    let t = if p < 0.5 {
        (-2.0 * p.ln()).sqrt()
    } else {
        (-2.0 * q.ln()).sqrt()
    };
    let mut s = t - polynomial(t, &A) / polynomial(t, &B);
    if p < 0.5 {
        s = -s;
    }
    s
}

// Computation of the Incomplete Gamma Function Ratios and their Inverse ARMIDO
// R. DIDONATO and ALFRED H. MORRIS, JR.  ACM Transactions on Mathematical
// Software, Vol. 12, No. 4, December 1986, Pages 377-393.
//
// See equation 34.
fn didonato_sn(a: f64, x: f64, n: u32, tolerance: f64) -> f64 {
    let mut sum = 1.0;

    if n >= 1 {
        let mut partial = x / (a + 1.0);
        sum += partial;
        for i in 2..=n {
            partial *= x / (a + f64::from(i));
            sum += partial;
            if partial < tolerance {
                break;
            }
        }
    }
    sum
}

// In order to understand what's going on here, you will need to refer to:
//
// Computation of the Incomplete Gamma Function Ratios and their Inverse ARMIDO
// R. DIDONATO and ALFRED H. MORRIS, JR.  ACM Transactions on Mathematical
// Software, Vol. 12, No. 4, December 1986, Pages 377-393.
fn find_inverse_gamma(a: f64, p: f64, q: f64) -> f64 {
    if a == 1.0 {
        return if q > 0.9 { -(-p).ln_1p() } else { -q.ln() };
    }

    if a < 1.0 {
        let g = gamma(a);
        let b = q * g;

        if (b > 0.6) || ((b >= 0.45) && (a >= 0.3)) {
            // Eq 21:
            //
            // There is a slight variation from DiDonato and Morris here: the
            // first form given here is unstable when p is close to 1, making it
            // impossible to compute the inverse of Q(a,x) for small q.
            // Fortunately the second form works perfectly well in this case.
            let u = if (b * q > 1e-8) && (q > 1e-5) {
                (p * g * a).powf(1.0 / a)
            } else {
                ((-q / a) - EULER_GAMMA).exp()
            };
            return u / (1.0 - (u / (a + 1.0)));
        }

        // Eq 22
        if (a < 0.3) && (b >= 0.35) {
            let t = (-EULER_GAMMA - b).exp();
            let u = t * t.exp();
            return t * u.exp();
        }

        // Eq 23
        if (b > 0.15) || (a >= 0.3) {
            let y = -b.ln();
            let u = y - (1.0 - a) * y.ln();
            return y - (1.0 - a) * u.ln() - (1.0 + (1.0 - a) / (1.0 + u)).ln();
        }

        // Eq 24
        if b > 0.1 {
            let y = -b.ln();
            let u = y - (1.0 - a) * y.ln();
            return y
                - (1.0 - a) * u.ln()
                - ((u * u + 2.0 * (3.0 - a) * u + (2.0 - a) * (3.0 - a))
                    / (u * u + (5.0 - a) * u + 2.0))
                    .ln();
        }

        // Eq 25
        let y = -b.ln();
        let c1 = (a - 1.0) * y.ln();
        let c1_2 = c1 * c1;
        let c1_3 = c1_2 * c1;
        let c1_4 = c1_2 * c1_2;
        let a_2 = a * a;
        let a_3 = a_2 * a;

        let c2 = (a - 1.0) * (1.0 + c1);
        let c3 =
            (a - 1.0) * (-c1_2 / 2.0 + (a - 2.0) * c1 + (3.0 * a - 5.0) / 2.0);
        let c4 = (a - 1.0)
            * (c1_3 / 3.0 - (3.0 * a - 5.0) * c1_2 / 2.0
                + (a_2 - 6.0 * a + 7.0) * c1
                + (11.0 * a_2 - 46.0 * a + 47.0) / 6.0);
        let c5 = (a - 1.0)
            * (-c1_4 / 4.0
                + (11.0 * a - 17.0) * c1_3 / 6.0
                + (-3.0 * a_2 + 13.0 * a - 13.0) * c1_2
                + (2.0 * a_3 - 25.0 * a_2 + 72.0 * a - 61.0) * c1 / 2.0
                + (25.0 * a_3 - 195.0 * a_2 + 477.0 * a - 379.0) / 12.0);

        let y_2 = y * y;
        let y_3 = y_2 * y;
        let y_4 = y_2 * y_2;
        return y + c1 + c2 / y + c3 / y_2 + c4 / y_3 + c5 / y_4;
    }

    // a >= 1, Eq 31
    let mut s = find_inverse_s(p, q);

    let s_2 = s * s;
    let s_3 = s_2 * s;
    let s_4 = s_2 * s_2;
    let s_5 = s_4 * s;
    let ra = a.sqrt();

    let mut w = a + s * ra + (s_2 - 1.0) / 3.0;
    w += (s_3 - 7.0 * s) / (36.0 * ra);
    w -= (3.0 * s_4 + 7.0 * s_2 - 16.0) / (810.0 * a);
    w += (9.0 * s_5 + 256.0 * s_3 - 433.0 * s) / (38880.0 * a * ra);

    if (a >= 500.0) && ((1.0 - w / a).abs() < 1e-6) {
        return w;
    }

    if p > 0.5 {
        if w < 3.0 * a {
            return w;
        }

        let d = 2.0_f64.max(a * (a - 1.0));
        let lg = ln_gamma(a);
        let lb = q.ln() + lg;

        if lb < -d * 2.3 {
            // Eq 25
            let y = -lb;
            let c1 = (a - 1.0) * y.ln();
            let c1_2 = c1 * c1;
            let c1_3 = c1_2 * c1;
            let c1_4 = c1_2 * c1_2;
            let a_2 = a * a;
            let a_3 = a_2 * a;

            let c2 = (a - 1.0) * (1.0 + c1);
            let c3 = (a - 1.0)
                * (-c1_2 / 2.0 + (a - 2.0) * c1 + (3.0 * a - 5.0) / 2.0);
            let c4 = (a - 1.0)
                * (c1_3 / 3.0 - (3.0 * a - 5.0) * c1_2 / 2.0
                    + (a_2 - 6.0 * a + 7.0) * c1
                    + (11.0 * a_2 - 46.0 * a + 47.0) / 6.0);
            let c5 = (a - 1.0)
                * (-c1_4 / 4.0
                    + (11.0 * a - 17.0) * c1_3 / 6.0
                    + (-3.0 * a_2 + 13.0 * a - 13.0) * c1_2
                    + (2.0 * a_3 - 25.0 * a_2 + 72.0 * a - 61.0) * c1 / 2.0
                    + (25.0 * a_3 - 195.0 * a_2 + 477.0 * a - 379.0) / 12.0);

            let y_2 = y * y;
            let y_3 = y_2 * y;
            let y_4 = y_2 * y_2;
            return y + c1 + c2 / y + c3 / y_2 + c4 / y_3 + c5 / y_4;
        }

        // Eq 33
        let u = -lb + (a - 1.0) * w.ln() - (1.0 + (1.0 - a) / (1.0 + w)).ln();
        return -lb + (a - 1.0) * u.ln() - (1.0 + (1.0 - a) / (1.0 + u)).ln();
    }

    // p <= 0.5
    let mut z = w;
    let ap1 = a + 1.0;
    let ap2 = a + 2.0;

    if w < 0.15 * ap1 {
        // Eq 35
        let v = p.ln() + ln_gamma(ap1);
        z = ((v + w) / a).exp();
        s = (z / ap1 * (1.0 + z / ap2)).ln_1p();
        z = ((v + z - s) / a).exp();
        s = (z / ap1 * (1.0 + z / ap2)).ln_1p();
        z = ((v + z - s) / a).exp();
        s = (z / ap1 * (1.0 + z / ap2 * (1.0 + z / (a + 3.0)))).ln_1p();
        z = ((v + z - s) / a).exp();
    }

    if (z <= 0.01 * ap1) || (z > 0.7 * ap1) {
        return z;
    }

    // Eq 36
    let ls = didonato_sn(a, z, 100, 1e-4).ln();
    let v = p.ln() + ln_gamma(ap1);
    z = ((v + z - ls) / a).exp();
    z * (1.0 - (a * z.ln() - z - v + ls) / (a - z))
}

pub fn inverse_lower_gamma(a: f64, p: f64) -> f64 {
    if a.is_nan() || p.is_nan() {
        return f64::NAN;
    }
    if (a < 0.0) || !(0.0..=1.0).contains(&p) {
        return f64::NAN;
    }
    if p == 0.0 {
        return 0.0;
    }
    if p == 1.0 {
        return f64::INFINITY;
    }
    if p > 0.9 {
        return inverse_upper_gamma(a, 1.0 - p);
    }

    let mut x = find_inverse_gamma(a, p, 1.0 - p);
    // Halley's method
    for _ in 0..3 {
        let fac = igam_fac(a, x);
        if fac == 0.0 {
            return x;
        }
        let f_fp = (regularized_lower_gamma(a, x) - p) * x / fac;
        // The ratio of the first and second derivatives simplifies
        let fpp_fp = -1.0 + (a - 1.0) / x;
        if fpp_fp.is_infinite() {
            // Resort to Newton's method in the case of overflow
            x -= f_fp;
        } else {
            x -= f_fp / (1.0 - 0.5 * f_fp * fpp_fp);
        }
    }
    x
}

pub fn inverse_upper_gamma(a: f64, q: f64) -> f64 {
    if a.is_nan() || q.is_nan() {
        return f64::NAN;
    }
    if (a < 0.0) || !(0.0..=1.0).contains(&q) {
        return f64::NAN;
    }
    if q == 0.0 {
        return f64::INFINITY;
    }
    if q == 1.0 {
        return 0.0;
    }
    if q > 0.9 {
        return inverse_lower_gamma(a, 1.0 - q);
    }

    let mut x = find_inverse_gamma(a, 1.0 - q, q);
    for _ in 0..3 {
        let fac = igam_fac(a, x);
        if fac == 0.0 {
            return x;
        }
        let f_fp = (regularized_upper_gamma(a, x) - q) * x / (-fac);
        let fpp_fp = -1.0 + (a - 1.0) / x;
        if fpp_fp.is_infinite() {
            x -= f_fp;
        } else {
            x -= f_fp / (1.0 - 0.5 * f_fp * fpp_fp);
        }
    }
    x
}
