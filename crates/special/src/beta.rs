use crate::{
    consts::MAX_LOG,
    gamma::{MAX_GAMMA, gamma, ln_gamma_sgn, recip_gamma},
};

const ASYMP_FACTOR: f64 = 1e6;

fn ln_beta_asymp(a: f64, b: f64, sgn: &mut i32) -> f64 {
    let mut r = ln_gamma_sgn(b, sgn);
    r -= b * a.ln();
    r += b * (1.0 - b) / (2.0 * a);
    r += b * (1.0 - b) * (1.0 - 2.0 * b) / (12.0 * a * a);
    r += -b * b * (1.0 - b) * (1.0 - b) / (12.0 * a * a * a);
    r
}

fn beta_negint(a: i32, b: f64) -> f64 {
    if b.fract() == 0.0 && (1.0 - a as f64 - b) > 0.0 {
        let sgn = if (b as i32) % 2 == 0 { 1.0 } else { -1.0 };
        sgn * beta(1.0 - a as f64 - b, b)
    } else {
        f64::INFINITY
    }
}

fn ln_beta_negint(a: i32, b: f64) -> f64 {
    if b.fract() == 0.0 && (1.0 - a as f64 - b) > 0.0 {
        ln_beta(1.0 - a as f64 - b, b)
    } else {
        f64::INFINITY
    }
}

pub fn beta(a: f64, b: f64) -> f64 {
    let mut sign = 1.0;

    if a <= 0.0 && a == a.floor() {
        if a >= f64::from(i32::MIN) && a <= f64::from(i32::MAX) {
            return beta_negint(a as i32, b);
        } else {
            return f64::INFINITY;
        }
    }

    if b <= 0.0 && b == b.floor() {
        if b >= f64::from(i32::MIN) && b <= f64::from(i32::MAX) {
            return beta_negint(b as i32, a);
        } else {
            return f64::INFINITY;
        }
    }

    let (a, b) = if a.abs() < b.abs() { (b, a) } else { (a, b) };

    if a.abs() > ASYMP_FACTOR * b.abs() && a > ASYMP_FACTOR {
        let mut sgn = 1;
        let y = ln_beta_asymp(a, b, &mut sgn);
        return (sgn as f64) * y.exp();
    }

    let y = a + b;
    if y.abs() > MAX_GAMMA || a.abs() > MAX_GAMMA || b.abs() > MAX_GAMMA {
        let mut sgngam = 1;
        let mut ly = ln_gamma_sgn(y, &mut sgngam);
        sign *= sgngam as f64;
        ly = ln_gamma_sgn(b, &mut sgngam) - ly;
        sign *= sgngam as f64;
        ly += ln_gamma_sgn(a, &mut sgngam);
        sign *= sgngam as f64;
        if ly > MAX_LOG {
            return sign * f64::INFINITY;
        }
        return sign * ly.exp();
    }

    let rgam_ab = recip_gamma(y);
    let gam_a = gamma(a);
    let gam_b = gamma(b);

    if rgam_ab.is_infinite() {
        return sign * f64::INFINITY;
    }

    let y = if ((gam_a * rgam_ab).abs() - 1.0).abs()
        > ((gam_b * rgam_ab).abs() - 1.0).abs()
    {
        gam_b * rgam_ab * gam_a
    } else {
        gam_a * rgam_ab * gam_b
    };

    sign * y
}

pub fn ln_beta(a: f64, b: f64) -> f64 {
    if a <= 0.0 && a == a.floor() {
        if a >= f64::from(i32::MIN) && a <= f64::from(i32::MAX) {
            return ln_beta_negint(a as i32, b);
        } else {
            return f64::INFINITY;
        }
    }

    if b <= 0.0 && b == b.floor() {
        if b >= f64::from(i32::MIN) && b <= f64::from(i32::MAX) {
            return ln_beta_negint(b as i32, a);
        } else {
            return f64::INFINITY;
        }
    }

    let (a, b) = if a.abs() < b.abs() { (b, a) } else { (a, b) };

    if a.abs() > ASYMP_FACTOR * b.abs() && a > ASYMP_FACTOR {
        let mut sgn = 1;
        return ln_beta_asymp(a, b, &mut sgn);
    }

    let mut _sgn = 1;
    let lga = ln_gamma_sgn(a, &mut _sgn);
    let mut _sgn = 1;
    let lgb = ln_gamma_sgn(b, &mut _sgn);
    let mut _sgn = 1;
    let lgab = ln_gamma_sgn(a + b, &mut _sgn);
    lga + lgb - lgab
}
