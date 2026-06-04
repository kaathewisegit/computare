use core::f64::consts::{EULER_GAMMA, FRAC_1_SQRT_2, FRAC_PI_4, SQRT_2};

use crate::{
    evaluate::{polynomial, polynomial1},
    gamma::ln_gamma,
    zeta::zeta,
};

const LOG1PMX_MAXITER: u32 = 500;

const UNITY_LP: [f64; 7] = [
    4.52700008624452e-5,
    4.9854102823193375e-1,
    6.578732594206104,
    2.9911919328553072e1,
    6.094966798098779e1,
    5.711296359058554e1,
    2.0039553499201283e1,
];

const UNITY_LQ: [f64; 6] = [
    1.5062909083469192e1,
    8.304756596796722e1,
    2.2176239823732857e2,
    3.0909872225312057e2,
    2.1642788614495947e2,
    6.011866049760384e1,
];

const UNITY_COSCOF: [f64; 7] = [
    4.737750796424621e-14,
    -1.147028484342536e-11,
    2.087675428708152e-9,
    -2.755731921499979e-7,
    2.480158730157055e-5,
    -1.3888888888888872e-3,
    4.1666666666666664e-2,
];

fn ln_gamma_1p_taylor(x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    let mut res = -EULER_GAMMA * x;
    let mut xfac = -x;
    for n in 2..42 {
        xfac *= -x;
        let coeff = zeta(n as f64, 1.0) * xfac / (n as f64);
        res += coeff;
        if coeff.abs() < f64::EPSILON * res.abs() {
            break;
        }
    }
    res
}

pub fn log1p(x: f64) -> f64 {
    let mut z = 1.0 + x;
    if !(FRAC_1_SQRT_2..=SQRT_2).contains(&z) {
        return z.ln();
    }
    z = x * x;
    z = -0.5 * z
        + x * (z * polynomial(x, &UNITY_LP) / polynomial1(x, &UNITY_LQ));
    x + z
}

pub fn log1pmx(x: f64) -> f64 {
    if x.abs() < 0.5 {
        let mut xfac = x;
        let mut res = 0.0;

        for n in 2..LOG1PMX_MAXITER {
            xfac *= -x;
            let term = xfac / f64::from(n);
            res += term;
            if term.abs() < f64::EPSILON * res.abs() {
                break;
            }
        }
        res
    } else {
        log1p(x) - x
    }
}

pub fn cosm1(x: f64) -> f64 {
    if !(-FRAC_PI_4..=FRAC_PI_4).contains(&x) {
        return x.cos() - 1.0;
    }
    let xx = x * x;
    -0.5 * xx + xx * xx * polynomial(xx, &UNITY_COSCOF)
}

pub fn ln_gamma_1p(x: f64) -> f64 {
    if x.abs() <= 0.5 {
        ln_gamma_1p_taylor(x)
    } else if (x - 1.0).abs() < 0.5 {
        x.ln() + ln_gamma_1p_taylor(x - 1.0)
    } else {
        ln_gamma(x + 1.0)
    }
}
