use libm::erfc;

use computare_core::debug_panic;

use super::{
    incomplete_coefs::{IGAM_ASYMP_COEFF_D, IGAM_ASYMP_COEFF_N},
    ln_gamma,
};
use crate::{
    consts::MAX_LOG,
    lanczos::{LANCZOS_G, lanczos_sum_expg_scaled},
};

fn log1pmx(_x: f64) -> f64 {
    todo!()
}
fn expm1(_x: f64) -> f64 {
    todo!()
}
fn lgam1p(_a: f64) -> f64 {
    todo!()
}

const IGAM_MAXITER: i32 = 2000;
const IGAM: i32 = 1;
const IGAMC: i32 = 0;
const IGAM_SMALL: f64 = 20.0;
const IGAM_LARGE: f64 = 200.0;
const IGAM_SMALLRATIO: f64 = 0.3;
const IGAM_LARGERATIO: f64 = 4.5;

const IGAM_BIG: f64 = 4.503599627370496e15;
const IGAM_BIGINV: f64 = 2.220446049250313e-16;

pub fn igam_fac(a: f64, x: f64) -> f64 {
    let ax: f64;

    if (a - x).abs() > 0.4 * a.abs() {
        ax = a * x.ln() - x - ln_gamma(a);
        if ax < -MAX_LOG {
            debug_panic!("underflow");
            return 0.0;
        }
        return ax.exp();
    }

    let fac = a + LANCZOS_G - 0.5;
    let res = (fac / 1.0f64.exp()).sqrt() / lanczos_sum_expg_scaled(a);

    if a < 200.0 && x < 200.0 {
        res * (a - x).exp() * (x / fac).powf(a)
    } else {
        let num = x - a - LANCZOS_G + 0.5;
        res * (a * log1pmx(num / fac) + x * (0.5 - LANCZOS_G) / fac).exp()
    }
}

pub fn igamc_continued_fraction(a: f64, x: f64) -> f64 {
    let ax = igam_fac(a, x);
    if ax == 0.0 {
        return 0.0;
    }

    let mut y = 1.0 - a;
    let mut z = x + y + 1.0;
    let mut c = 0.0;
    let mut pkm2 = 1.0;
    let mut qkm2 = x;
    let mut pkm1 = x + 1.0;
    let mut qkm1 = z * x;
    let mut ans = pkm1 / qkm1;

    for _ in 0..IGAM_MAXITER {
        c += 1.0;
        y += 1.0;
        z += 2.0;
        let yc = y * c;
        let pk = pkm1 * z - pkm2 * yc;
        let qk = qkm1 * z - qkm2 * yc;

        let t = if qk != 0.0 {
            let r = pk / qk;
            let t_val = ((ans - r) / r).abs();
            ans = r;
            t_val
        } else {
            1.0
        };

        pkm2 = pkm1;
        pkm1 = pk;
        qkm2 = qkm1;
        qkm1 = qk;

        if pk.abs() > IGAM_BIG {
            pkm2 *= IGAM_BIGINV;
            pkm1 *= IGAM_BIGINV;
            qkm2 *= IGAM_BIGINV;
            qkm1 *= IGAM_BIGINV;
        }
        if t <= f64::EPSILON {
            break;
        }
    }

    ans * ax
}

fn igam_series(a: f64, x: f64) -> f64 {
    let ax = igam_fac(a, x);
    if ax == 0.0 {
        return 0.0;
    }

    let mut r = a;
    let mut c = 1.0;
    let mut ans = 1.0;

    for _ in 0..IGAM_MAXITER {
        r += 1.0;
        c *= x / r;
        ans += c;
        if c <= f64::EPSILON * ans {
            break;
        }
    }

    ans * ax / a
}

fn igamc_series(a: f64, x: f64) -> f64 {
    let mut fac = 1.0;
    let mut sum = 0.0;

    for n in 1..IGAM_MAXITER {
        fac *= -x / f64::from(n);
        let term = fac / (a + f64::from(n));
        sum += term;
        if term.abs() <= f64::EPSILON * sum.abs() {
            break;
        }
    }

    let logx = x.ln();
    let term = -expm1(a * logx - lgam1p(a));
    term - (a * logx - ln_gamma(a)).exp() * sum
}

fn asymptotic_series(a: f64, x: f64, func: i32) -> f64 {
    let mut maxpow = 0;
    let lambda = x / a;
    let sigma = (x - a) / a;
    let eta;
    let mut absoldterm = f64::INFINITY;
    let mut etapow = [1.0; IGAM_ASYMP_COEFF_N];
    let mut sum = 0.0;
    let mut afac = 1.0;

    let sgn = if func == IGAM { -1.0 } else { 1.0 };

    if lambda > 1.0 {
        eta = (-2.0 * log1pmx(sigma)).sqrt();
    } else if lambda < 1.0 {
        eta = -(-2.0 * log1pmx(sigma)).sqrt();
    } else {
        eta = 0.0;
    }

    let mut res = 0.5 * erfc(sgn * eta * (a / 2.0).sqrt());

    for coefficients in IGAM_ASYMP_COEFF_D {
        let mut ck = coefficients[0];
        for n in 1..IGAM_ASYMP_COEFF_N {
            if n > maxpow {
                etapow[n] = eta * etapow[n - 1];
                maxpow += 1;
            }
            let ckterm = coefficients[n] * etapow[n];
            ck += ckterm;
            if ckterm.abs() < f64::EPSILON * ck.abs() {
                break;
            }
        }
        let term = ck * afac;
        let absterm = term.abs();
        if absterm > absoldterm {
            break;
        }
        sum += term;
        if absterm < f64::EPSILON * sum.abs() {
            break;
        }
        absoldterm = absterm;
        afac /= a;
    }

    res += sgn * (-0.5 * a * eta * eta).exp() * sum
        / (2.0 * std::f64::consts::PI * a).sqrt();

    res
}

pub fn regularized_lower_gamma(a: f64, x: f64) -> f64 {
    if a.is_nan() || x.is_nan() {
        return f64::NAN;
    }

    if x < 0.0 || a < 0.0 {
        debug_panic!("domain error");
        return f64::NAN;
    } else if a == 0.0 {
        if x > 0.0 {
            return 1.0;
        } else {
            return f64::NAN;
        }
    } else if x == 0.0 {
        return 0.0;
    } else if a.is_infinite() {
        if x.is_infinite() {
            return f64::NAN;
        }
        return 0.0;
    } else if x.is_infinite() {
        return 1.0;
    }

    let absxma_a = (x - a).abs() / a;
    if (a > IGAM_SMALL && a < IGAM_LARGE && absxma_a < IGAM_SMALLRATIO)
        || (a > IGAM_LARGE && absxma_a < IGAM_LARGERATIO / a.sqrt())
    {
        return asymptotic_series(a, x, IGAM);
    }

    if x > 1.0 && x > a {
        return 1.0 - regularized_upper_gamma(a, x);
    }

    igam_series(a, x)
}

pub fn regularized_upper_gamma(a: f64, x: f64) -> f64 {
    if a.is_nan() || x.is_nan() {
        return f64::NAN;
    }

    if x < 0.0 || a < 0.0 {
        debug_panic!("domain error");
        return f64::NAN;
    } else if a == 0.0 {
        if x > 0.0 {
            return 0.0;
        } else {
            return f64::NAN;
        }
    } else if x == 0.0 {
        return 1.0;
    } else if a.is_infinite() {
        if x.is_infinite() {
            return f64::NAN;
        }
        return 1.0;
    } else if x.is_infinite() {
        return 0.0;
    }

    let absxma_a = (x - a).abs() / a;
    if (a > IGAM_SMALL && a < IGAM_LARGE && absxma_a < IGAM_SMALLRATIO)
        || (a > IGAM_LARGE && absxma_a < IGAM_LARGERATIO / a.sqrt())
    {
        return asymptotic_series(a, x, IGAMC);
    }

    if x > 1.1 {
        if x < a {
            1.0 - igam_series(a, x)
        } else {
            igamc_continued_fraction(a, x)
        }
    } else if x <= 0.5 {
        if -0.4 / x.ln() < a {
            1.0 - igam_series(a, x)
        } else {
            igamc_series(a, x)
        }
    } else {
        if x * 1.1 < a {
            1.0 - igam_series(a, x)
        } else {
            igamc_series(a, x)
        }
    }
}
