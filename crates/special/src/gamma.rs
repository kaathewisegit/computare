use core::f64::consts::PI;

use computare_core::{FloatMath, debug_panic};

use crate::polynomial::{polynomial, polynomial1};

#[expect(clippy::excessive_precision)]
pub fn gamma(mut x: f64) -> f64 {
    const GAMMA_P: [f64; 7] = [
        1.60119522476751861407e-4,
        1.19135147006586384913e-3,
        1.04213797561761569935e-2,
        4.76367800457137231464e-2,
        2.07448227648435975150e-1,
        4.94214826801497100753e-1,
        9.99999999999999996796e-1,
    ];

    const GAMMA_Q: [f64; 8] = [
        -2.31581873324120129819e-5,
        5.39605580493303397842e-4,
        -4.45641913851797240494e-3,
        1.18139785222060435552e-2,
        3.58236398605498653373e-2,
        -2.34591795718243348568e-1,
        7.14304917030273074085e-2,
        1.00000000000000000320e0,
    ];

    const GAMMA_STIRLING: [f64; 5] = [
        7.87311395793093628397e-4,
        -2.29549961613378126380e-4,
        -2.68132617805781232825e-3,
        3.47222221605458667310e-3,
        8.33333333333482257126e-2,
    ];

    const MAX_STIRLING: f64 = 143.01608;
    const MAX_GAMMA: f64 = 171.624376956302725;
    const SQRT2PI: f64 = 2.506628274631000502415765284811045253006;

    fn stirf(x: f64) -> f64 {
        if x >= MAX_GAMMA {
            return f64::INFINITY;
        }
        let mut w = 1.0 / x;
        w = 1.0 + w * polynomial(w, &GAMMA_STIRLING);
        let mut y = x.exp();
        if x > MAX_STIRLING {
            let v = x.powf(0.5 * x - 0.25);
            y = v * (v / y);
        } else {
            y = x.powf(x - 0.5) / y;
        }
        SQRT2PI * y * w
    }

    let mut sgngam = 1.0;

    if !x.is_finite() {
        if x == f64::INFINITY {
            return x;
        }
        return f64::NAN;
    }

    if x == 0.0 {
        return f64::INFINITY.copysign(x);
    }

    let q = x.abs();

    if q > 33.0 {
        let z = if x < 0.0 {
            let p = q.floor();
            if p == q {
                debug_panic!("pole: {x}");
                return f64::NAN;
            }
            let i = p as i32;
            if (i & 1) == 0 {
                sgngam = -1.0;
            }
            let mut z = q - p;
            let mut p_mut = p;
            if z > 0.5 {
                p_mut += 1.0;
                z = q - p_mut;
            }
            z = q * z.sinpi();
            if z == 0.0 {
                return sgngam * f64::INFINITY;
            }
            z = z.abs();
            PI / (z * stirf(q))
        } else {
            stirf(x)
        };
        return sgngam * z;
    }

    fn small(x: f64, z: f64) -> f64 {
        if x == 0.0 {
            debug_panic!("pole: {x}");
            f64::NAN
        } else {
            z / ((1.0 + 0.5772156649015329 * x) * x)
        }
    }

    let mut z = 1.0;
    while x >= 3.0 {
        x -= 1.0;
        z *= x;
    }

    while x < 0.0 {
        if x > -1.0e-9 {
            return small(x, z);
        }
        z /= x;
        x += 1.0;
    }

    while x < 2.0 {
        if x < 1.0e-9 {
            return small(x, z);
        }
        z /= x;
        x += 1.0;
    }

    if x == 2.0 {
        return z;
    }

    x -= 2.0;
    let p = polynomial(x, &GAMMA_P);
    let q = polynomial(x, &GAMMA_Q);
    z * p / q
}

pub fn ln_gamma(x: f64) -> f64 {
    const GAMMA_A: [f64; 5] = [
        8.116141674705085e-4,
        -5.950619042843014e-4,
        7.936503404577169e-4,
        -2.777777777300997e-3,
        8.333333333333319e-2,
    ];

    const GAMMA_B: [f64; 6] = [
        -1.3782515256912086e3,
        -3.8801631513463784e4,
        -3.316129927388712e5,
        -1.162370974927623e6,
        -1.7217370082083966e6,
        -8.535556642457654e5,
    ];

    const GAMMA_C: [f64; 6] = [
        -3.5181570143652345e2,
        -1.7064210665188115e4,
        -2.2052859055385445e5,
        -1.1393344436798252e6,
        -2.5325230717758294e6,
        -2.0188914143353277e6,
    ];

    const MAX_LN_GAMMA: f64 = 2.556348e305;
    const LN_SQRT_2PI: f64 = 0.9189385332046728; // ln(√2π)
    const LN_PI: f64 = 1.1447298858494002;

    fn singularity() -> f64 {
        debug_panic!("singularity");
        f64::INFINITY
    }

    fn ln_gamma_large_x(x: f64) -> f64 {
        let q = (x - 0.5) * x.ln() - x + LN_SQRT_2PI;
        if x > 1.0e8 {
            return q;
        }
        let p = 1.0 / (x * x);
        let p = ((7.936507936507937e-4 * p - 2.777777777777778e-3) * p
            + 0.08333333333333333)
            / x;
        q + p
    }

    fn ln_gamma_sgn(x: f64, sign: &mut i32) -> f64 {
        *sign = 1;

        if !x.is_finite() {
            return x;
        }

        if x < -34.0 {
            let q = -x;
            let w = ln_gamma_sgn(q, sign);
            let mut p = q.floor();
            if p == q {
                return singularity();
            }
            let i = p as i64;
            if (i & 1) == 0 {
                *sign = -1;
            } else {
                *sign = 1;
            }
            let mut z = q - p;
            if z > 0.5 {
                p += 1.0;
                z = p - q;
            }
            z = q * (z * PI).sin();
            if z == 0.0 {
                return singularity();
            }
            z = LN_PI - z.ln() - w;
            return z;
        }

        if x < 13.0 {
            let mut z = 1.0;
            let mut p = 0.0;
            let mut u = x;
            while u >= 3.0 {
                p -= 1.0;
                u = x + p;
                z *= u;
            }
            while u < 2.0 {
                if u == 0.0 {
                    return singularity();
                }
                z /= u;
                p += 1.0;
                u = x + p;
            }
            if z < 0.0 {
                *sign = -1;
                z = -z;
            } else {
                *sign = 1;
            }
            if u == 2.0 {
                return z.ln();
            }
            p -= 2.0;
            let x = x + p;
            let p_val = x * polynomial(x, &GAMMA_B) / polynomial1(x, &GAMMA_C);
            return z.ln() + p_val;
        }

        if x > MAX_LN_GAMMA {
            return (*sign as f64) * f64::INFINITY;
        }

        if x >= 1000.0 {
            return ln_gamma_large_x(x);
        }

        let q = (x - 0.5) * x.ln() - x + LN_SQRT_2PI;
        let p = 1.0 / (x * x);
        q + polynomial(p, &GAMMA_A) / x
    }

    let mut sign = 0;
    ln_gamma_sgn(x, &mut sign)
}
