use core::f64::consts::PI;

use computare_core::FloatMath;

use crate::polynomial::polynomial;

#[expect(clippy::excessive_precision)]
pub fn gamma(mut x: f64) -> f64 {
    pub const GAMMA_P: [f64; 7] = [
        1.60119522476751861407e-4,
        1.19135147006586384913e-3,
        1.04213797561761569935e-2,
        4.76367800457137231464e-2,
        2.07448227648435975150e-1,
        4.94214826801497100753e-1,
        9.99999999999999996796e-1,
    ];

    pub const GAMMA_Q: [f64; 8] = [
        -2.31581873324120129819e-5,
        5.39605580493303397842e-4,
        -4.45641913851797240494e-3,
        1.18139785222060435552e-2,
        3.58236398605498653373e-2,
        -2.34591795718243348568e-1,
        7.14304917030273074085e-2,
        1.00000000000000000320E0,
    ];

    pub const GAMMA_STIR: [f64; 5] = [
        7.87311395793093628397e-4,
        -2.29549961613378126380e-4,
        -2.68132617805781232825e-3,
        3.47222221605458667310e-3,
        8.33333333333482257126e-2,
    ];

    pub const MAXSTIR: f64 = 143.01608;
    pub const MAXGAM: f64 = 171.624376956302725;
    pub const SQRT2PI: f64 = 2.506628274631000502415765284811045253006;

    pub fn stirf(x: f64) -> f64 {
        if x >= MAXGAM {
            return f64::INFINITY;
        }
        let mut w = 1.0 / x;
        w = 1.0 + w * polynomial(w, &GAMMA_STIR);
        let mut y = x.exp();
        if x > MAXSTIR {
            let v = x.powf(0.5 * x - 0.25);
            y = v * (v / y);
        } else {
            y = x.powf(x - 0.5) / y;
        }
        SQRT2PI * y * w
    }

    let mut sgngam = 1.0;

    if !x.is_finite() {
        if x > 0.0 {
            return x;
        }
        return f64::NAN;
    }

    if x == 0.0 {
        return f64::copysign(f64::INFINITY, x);
    }

    let q = x.abs();

    if q > 33.0 {
        let z = if x < 0.0 {
            let p = q.floor();
            if p == q {
                // set_error("Gamma", SF_ERROR_SINGULAR);
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
