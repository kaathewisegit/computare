use core::f64::consts::{EULER_GAMMA, PI};

use crate::evaluate::polynomial;
use computare_core::debug_panic;

const PSI_A: [f64; 7] = [
    8.333333333333333e-2,
    -2.1092796092796094e-2,
    7.575757575757576e-3,
    -4.166666666666667e-3,
    3.968253968253968e-3,
    -8.333333333333333e-3,
    8.333333333333333e-2,
];

const PSI_Y: f64 = 0.9955816;
const PSI_ROOT1: f64 = 1569415565.0 / 1073741824.0;
const PSI_ROOT2: f64 = (381566830.0 / 1073741824.0) / 1073741824.0;
const PSI_ROOT3: f64 = 0.9016312093258695918615325266959189453125e-19;

const PSI_P: [f64; 6] = [
    -0.002071332116774595,
    -0.04525132144873906,
    -0.28919126444774784,
    -0.6503185377089651,
    -0.3255503118680449,
    0.25479851061131551,
];

const PSI_Q: [f64; 7] = [
    -5.578984132167551e-7,
    0.0021284987017821144,
    0.054151797245674225,
    0.43593529692665969,
    1.4606242909763515,
    2.076711702373047,
    1.0,
];

fn digamma_imp_1_2(x: f64) -> f64 {
    let g = x - PSI_ROOT1 - PSI_ROOT2 - PSI_ROOT3;
    let r = polynomial(x - 1.0, &PSI_P) / polynomial(x - 1.0, &PSI_Q);
    g * PSI_Y + g * r
}

fn psi_asy(x: f64) -> f64 {
    let y = if x < 1.0e17 {
        let z = 1.0 / (x * x);
        z * polynomial(z, &PSI_A)
    } else {
        0.0
    };
    x.ln() - (0.5 / x) - y
}

pub fn psi(mut x: f64) -> f64 {
    let mut y = 0.0;

    if x.is_nan() || x == f64::INFINITY {
        return x;
    } else if x == f64::NEG_INFINITY {
        return f64::NAN;
    } else if x == 0.0 {
        debug_panic!("singular");
        return f64::INFINITY.copysign(-x);
    } else if x < 0.0 {
        let r = x % 1.0;
        if r == 0.0 {
            debug_panic!("singular");
            return f64::NAN;
        }
        y = -PI / (PI * r).tan();
        x = 1.0 - x;
    }

    // check for positive integer up to 10
    if x <= 10.0 && x == x.floor() {
        let n = x as i32;
        for i in 1..n {
            y += 1.0 / (i as f64);
        }
        y -= EULER_GAMMA;
        return y;
    }

    // use the recurrence relation to move x into [1, 2]
    if x < 1.0 {
        y -= 1.0 / x;
        x += 1.0;
    } else if x < 10.0 {
        while x > 2.0 {
            x -= 1.0;
            y += 1.0 / x;
        }
    }

    if (1.0..=2.0).contains(&x) {
        y += digamma_imp_1_2(x);
        return y;
    }

    // x is large, use the asymptotic series
    y += psi_asy(x);
    y
}
