use crate::{
    consts::{MAX_LOG, MIN_LOG},
    gamma::{MAX_GAMMA, gamma, ln_gamma, ln_gamma_sgn, recip_gamma},
    ndtri::ndtri,
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

const INCBET_BIG: f64 = 4.503599627370496e15;
const INCBET_BIGINV: f64 = 2.220_446_049_250_313e-16;

// Power series for incomplete beta integral
//
// Use when `b ⋅ x` is small and `x` is not too close to 1.
fn incbet_pseries(a: f64, b: f64, x: f64) -> f64 {
    let ai = 1.0 / a;
    let u = (1.0 - b) * x;
    let mut v = u / (a + 1.0);
    let t1 = v;
    let mut t = u;
    let mut n = 2.0;
    let mut s = 0.0;
    let z = f64::EPSILON * ai;

    while v.abs() > z {
        let u = (n - b) * x / n;
        t *= u;
        v = t / (a + n);
        s += v;
        n += 1.0;
    }
    s += t1;
    s += ai;

    let u = a * x.ln();
    if (a + b) < MAX_GAMMA && u.abs() < MAX_LOG {
        let t = 1.0 / beta(a, b);
        (s * t) * x.powf(a)
    } else {
        let t = -ln_beta(a, b) + u + s.ln();
        if t < MIN_LOG { 0.0 } else { t.exp() }
    }
}

/// Continued fraction expansion #2 for incomplete beta integral
fn incbcf(a: f64, b: f64, x: f64) -> f64 {
    let mut k1 = a;
    let mut k2 = a + b;
    let mut k3 = a;
    let mut k4 = a + 1.0;
    let mut k5 = 1.0;
    let mut k6 = b - 1.0;
    let mut k7 = a + 1.0;
    let mut k8 = a + 2.0;

    let mut pkm2 = 0.0;
    let mut qkm2 = 1.0;
    let mut pkm1 = 1.0;
    let mut qkm1 = 1.0;
    let mut ans = 1.0;
    let mut r = 1.0;
    let thresh = 3.0 * f64::EPSILON;

    for _ in 0..300 {
        let xk = -(x * k1 * k2) / (k3 * k4);
        let pk = pkm1 + pkm2 * xk;
        let qk = qkm1 + qkm2 * xk;
        pkm2 = pkm1;
        pkm1 = pk;
        qkm2 = qkm1;
        qkm1 = qk;

        let xk = (x * k5 * k6) / (k7 * k8);
        let pk = pkm1 + pkm2 * xk;
        let qk = qkm1 + qkm2 * xk;
        pkm2 = pkm1;
        pkm1 = pk;
        qkm2 = qkm1;
        qkm1 = qk;

        if qk != 0.0 {
            r = pk / qk;
        }
        if r != 0.0 {
            let t = ((ans - r) / r).abs();
            ans = r;
            if t < thresh {
                return ans;
            }
        }

        k1 += 1.0;
        k2 += 1.0;
        k3 += 2.0;
        k4 += 2.0;
        k5 += 1.0;
        k6 -= 1.0;
        k7 += 2.0;
        k8 += 2.0;

        if (qk.abs() + pk.abs()) > INCBET_BIG {
            pkm2 *= INCBET_BIGINV;
            pkm1 *= INCBET_BIGINV;
            qkm2 *= INCBET_BIGINV;
            qkm1 *= INCBET_BIGINV;
        }
        if (qk.abs() < INCBET_BIGINV) || (pk.abs() < INCBET_BIGINV) {
            pkm2 *= INCBET_BIG;
            pkm1 *= INCBET_BIG;
            qkm2 *= INCBET_BIG;
            qkm1 *= INCBET_BIG;
        }
    }

    ans
}

/// Continued fraction expansion #2 for incomplete beta integral
fn incbd(a: f64, b: f64, x: f64) -> f64 {
    let mut k1 = a;
    let mut k2 = b - 1.0;
    let mut k3 = a;
    let mut k4 = a + 1.0;
    let mut k5 = 1.0;
    let mut k6 = a + b;
    let mut k7 = a + 1.0;
    let mut k8 = a + 2.0;

    let mut pkm2 = 0.0;
    let mut qkm2 = 1.0;
    let mut pkm1 = 1.0;
    let mut qkm1 = 1.0;
    let z = x / (1.0 - x);
    let mut ans = 1.0;
    let mut r = 1.0;
    let thresh = 3.0 * f64::EPSILON;

    for _ in 0..300 {
        let xk = -(z * k1 * k2) / (k3 * k4);
        let pk = pkm1 + pkm2 * xk;
        let qk = qkm1 + qkm2 * xk;
        pkm2 = pkm1;
        pkm1 = pk;
        qkm2 = qkm1;
        qkm1 = qk;

        let xk = (z * k5 * k6) / (k7 * k8);
        let pk = pkm1 + pkm2 * xk;
        let qk = qkm1 + qkm2 * xk;
        pkm2 = pkm1;
        pkm1 = pk;
        qkm2 = qkm1;
        qkm1 = qk;

        if qk != 0.0 {
            r = pk / qk;
        }
        if r != 0.0 {
            let t = ((ans - r) / r).abs();
            ans = r;
            if t < thresh {
                return ans;
            }
        }

        k1 += 1.0;
        k2 -= 1.0;
        k3 += 2.0;
        k4 += 2.0;
        k5 += 1.0;
        k6 += 1.0;
        k7 += 2.0;
        k8 += 2.0;

        if (qk.abs() + pk.abs()) > INCBET_BIG {
            pkm2 *= INCBET_BIGINV;
            pkm1 *= INCBET_BIGINV;
            qkm2 *= INCBET_BIGINV;
            qkm1 *= INCBET_BIGINV;
        }
        if (qk.abs() < INCBET_BIGINV) || (pk.abs() < INCBET_BIGINV) {
            pkm2 *= INCBET_BIG;
            pkm1 *= INCBET_BIG;
            qkm2 *= INCBET_BIG;
            qkm1 *= INCBET_BIG;
        }
    }

    ans
}

pub fn regularized_incomplete_beta(a: f64, b: f64, x: f64) -> f64 {
    if a <= 0.0 || b <= 0.0 {
        return f64::NAN;
    }

    if x <= 0.0 || x >= 1.0 {
        if x == 0.0 {
            return 0.0;
        }
        if x == 1.0 {
            return 1.0;
        }
        return f64::NAN;
    }

    if (b * x) <= 1.0 && x <= 0.95 {
        return incbet_pseries(a, b, x);
    }

    let w = 1.0 - x;

    // Reverse a and b if x is greater than the mean
    let (a, b, x, xc, swapped) = if x > (a / (a + b)) {
        (b, a, w, x, true)
    } else {
        (a, b, x, w, false)
    };

    if swapped && (b * x) <= 1.0 && x <= 0.95 {
        let t = incbet_pseries(a, b, x);
        return finish(t, swapped);
    }

    let w = {
        // Choose expansion for better convergence
        let y = x * (a + b - 2.0) - (a - 1.0);
        if y < 0.0 {
            incbcf(a, b, x)
        } else {
            incbd(a, b, x) / xc
        }
    };

    fn finish(t: f64, swapped: bool) -> f64 {
        if swapped {
            if t <= f64::EPSILON {
                1.0 - f64::EPSILON
            } else {
                1.0 - t
            }
        } else {
            t
        }
    }

    // Multiply w by the factor
    // a      b   _             _     _
    // x  (1-x)   | (a+b) / ( a | (a) | (b) )
    let t = {
        let y = a * x.ln();
        let tc = b * xc.ln();
        if (a + b) < MAX_GAMMA && y.abs() < MAX_LOG && tc.abs() < MAX_LOG {
            let mut t = xc.powf(b);
            t *= x.powf(a);
            t /= a;
            t *= w;
            t *= 1.0 / beta(a, b);
            t
        } else {
            // Resort to logarithms
            let y = y + tc - ln_beta(a, b) + (w / a).ln();
            if y < MIN_LOG { 0.0 } else { y.exp() }
        }
    };

    finish(t, swapped)
}

pub fn inverse_regularized_beta(aa: f64, bb: f64, yy0: f64) -> f64 {
    fn finish(x: f64, rflg: bool) -> f64 {
        if rflg {
            if x <= f64::EPSILON {
                1.0 - f64::EPSILON
            } else {
                1.0 - x
            }
        } else {
            x
        }
    }

    if aa.is_nan() || bb.is_nan() || yy0.is_nan() {
        return f64::NAN;
    }
    if aa <= 0.0 || bb <= 0.0 || !(0.0..=1.0).contains(&yy0) {
        return f64::NAN;
    }
    if yy0 == 0.0 {
        return 0.0;
    }
    if yy0 == 1.0 {
        return 1.0;
    }

    let mut x0 = 0.0;
    let mut yl = 0.0;
    let mut x1 = 1.0;
    let mut yh = 1.0;
    let mut nflg = false;

    let mut a;
    let mut b;
    let mut y0;
    let mut x;
    let mut y;
    let mut dithresh;
    let mut rflg;
    let mut skip_ihalve;

    if aa <= 1.0 || bb <= 1.0 {
        dithresh = 1.0e-6;
        rflg = false;
        a = aa;
        b = bb;
        y0 = yy0;
        x = a / (a + b);
        y = regularized_incomplete_beta(a, b, x);
        skip_ihalve = false;
    } else {
        dithresh = 1.0e-4;
        let mut yp = -ndtri(yy0);

        if yy0 > 0.5 {
            rflg = true;
            a = bb;
            b = aa;
            y0 = 1.0 - yy0;
            yp = -yp;
        } else {
            rflg = false;
            a = aa;
            b = bb;
            y0 = yy0;
        }

        let lgm = (yp * yp - 3.0) / 6.0;
        let xc = 2.0 / (1.0 / (2.0 * a - 1.0) + 1.0 / (2.0 * b - 1.0));
        let mut d = yp * (xc + lgm).sqrt() / xc
            - (1.0 / (2.0 * b - 1.0) - 1.0 / (2.0 * a - 1.0))
                * (lgm + 5.0 / 6.0 - 2.0 / (3.0 * xc));
        d *= 2.0;
        if d < MIN_LOG {
            return finish(0.0, rflg);
        }
        x = a / (a + b * d.exp());
        y = regularized_incomplete_beta(a, b, x);
        let yp_ratio = (y - y0) / y0;
        skip_ihalve = yp_ratio.abs() < 0.2;
    }

    'outer: loop {
        if skip_ihalve {
            skip_ihalve = false;
        } else {
            'ihalve: loop {
                let mut dir = 0;
                let mut di = 0.5;
                let mut converged = false;

                for i in 0..100 {
                    if i != 0 {
                        x = x0 + di * (x1 - x0);
                        if x == 1.0 {
                            x = 1.0 - f64::EPSILON;
                        }
                        if x == 0.0 {
                            di = 0.5;
                            x = x0 + di * (x1 - x0);
                            if x == 0.0 {
                                return finish(0.0, rflg);
                            }
                        }
                        y = regularized_incomplete_beta(a, b, x);
                        let yp_x = (x1 - x0) / (x1 + x0);
                        if yp_x.abs() < dithresh {
                            converged = true;
                            break;
                        }
                        let yp_y = (y - y0) / y0;
                        if yp_y.abs() < dithresh {
                            converged = true;
                            break;
                        }
                    }

                    if y < y0 {
                        x0 = x;
                        yl = y;
                        if dir < 0 {
                            dir = 0;
                            di = 0.5;
                        } else if dir > 3 {
                            di = 1.0 - (1.0 - di) * (1.0 - di);
                        } else if dir > 1 {
                            di = 0.5 * di + 0.5;
                        } else {
                            di = (y0 - y) / (yh - yl);
                        }
                        dir += 1;
                        if x0 > 0.75 {
                            if rflg {
                                rflg = false;
                                a = aa;
                                b = bb;
                                y0 = yy0;
                            } else {
                                rflg = true;
                                a = bb;
                                b = aa;
                                y0 = 1.0 - yy0;
                            }
                            x = 1.0 - x;
                            y = regularized_incomplete_beta(a, b, x);
                            x0 = 0.0;
                            yl = 0.0;
                            x1 = 1.0;
                            yh = 1.0;
                            continue 'ihalve;
                        }
                    } else {
                        x1 = x;
                        if rflg && x1 < f64::EPSILON {
                            return finish(0.0, rflg);
                        }
                        yh = y;
                        if dir > 0 {
                            dir = 0;
                            di = 0.5;
                        } else if dir < -3 {
                            di = di * di;
                        } else if dir < -1 {
                            di *= 0.5;
                        } else {
                            di = (y - y0) / (yh - yl);
                        }
                        dir -= 1;
                    }
                }

                if converged {
                    break 'ihalve;
                }

                if x0 >= 1.0 {
                    return finish(1.0 - f64::EPSILON, rflg);
                }
                if x <= 0.0 {
                    return finish(0.0, rflg);
                }

                break 'ihalve;
            }
        }

        // Newton refinement
        if nflg {
            return finish(x, rflg);
        }
        nflg = true;
        let lgm = ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b);

        let mut newton_done = false;
        for i in 0..8 {
            if i != 0 {
                y = regularized_incomplete_beta(a, b, x);
            }
            if y < yl {
                x = x0;
                y = yl;
            } else if y > yh {
                x = x1;
                y = yh;
            } else if y < y0 {
                x0 = x;
                yl = y;
            } else {
                x1 = x;
                yh = y;
            }
            if x == 1.0 || x == 0.0 {
                break;
            }
            let d = (a - 1.0) * x.ln() + (b - 1.0) * (1.0 - x).ln() + lgm;
            if d < MIN_LOG {
                newton_done = true;
                break;
            }
            if d > MAX_LOG {
                break;
            }
            let d = d.exp();
            let d = (y - y0) / d;
            let mut xt = x - d;
            if xt <= x0 {
                let yy = (x - x0) / (x1 - x0);
                xt = x0 + 0.5 * yy * (x - x0);
                if xt <= 0.0 {
                    break;
                }
            }
            if xt >= x1 {
                let yy = (x1 - x) / (x1 - x0);
                xt = x1 - 0.5 * yy * (x1 - x);
                if xt >= 1.0 {
                    break;
                }
            }
            x = xt;
            if (d / x).abs() < 128.0 * f64::EPSILON {
                newton_done = true;
                break;
            }
        }

        if newton_done {
            return finish(x, rflg);
        }

        // Did not converge, retry with tighter threshold
        dithresh = 256.0 * f64::EPSILON;
        continue 'outer;
    }
}
