use crate::evaluate::{polynomial, polynomial1};
use computare_core::Float;

pub fn ndtri(y0: f64) -> f64 {
    // Approximation for 0 <= |y - 0.5| <= 3/8
    const NDTRI_P0: [f64; 5] = [
        -5.996335010141079e1,
        9.800107541859997e1,
        -5.667628574690703e1,
        1.3931260938727968e1,
        -1.2391658386738125,
    ];

    const NDTRI_Q0: [f64; 8] = [
        1.9544885833814176,
        4.676279128988815,
        8.636024213908905e1,
        -2.2546268785411937e2,
        2.0026021238006066e2,
        -8.203722561683334e1,
        1.590562251262117e1,
        -1.1833162112133,
    ];

    // Approximation for interval z = sqrt(-2 log y ) between 2 and 8
    const NDTRI_P1: [f64; 9] = [
        4.0554489230596245,
        3.1525109459989388e1,
        5.716281922464213e1,
        4.408050738932008e1,
        1.4684956192885803e1,
        2.1866330685079025,
        -1.402560791713545e-1,
        -3.504246268278482e-2,
        -8.574567851546854e-4,
    ];

    const NDTRI_Q1: [f64; 8] = [
        1.5779988325646675e1,
        4.539076351288792e1,
        4.13172038254672e1,
        1.504253856929075e1,
        2.504649462083094,
        -1.4218292285478779e-1,
        -3.808064076915783e-2,
        -9.332594808954574e-4,
    ];

    // Approximation for interval z = sqrt(-2 log y ) between 8 and 64
    const NDTRI_P2: [f64; 9] = [
        3.2377489177694603,
        6.915228890689842,
        3.9388102529247444,
        1.3330346081580755,
        2.0148538954917908e-1,
        1.2371663481782003e-2,
        3.0158155350823543e-4,
        2.6580697468673755e-6,
        6.239745391849833e-9,
    ];

    const NDTRI_Q2: [f64; 8] = [
        6.02427039364742,
        3.6798356385616087,
        1.3770209948908132,
        2.1623699359449663e-1,
        1.3420400608854318e-2,
        3.2801446468212774e-4,
        2.8924786474538068e-6,
        6.790194080099813e-9,
    ];

    if !(0.0..=1.0).contains(&y0) {
        return f64::NAN;
    }
    if y0 == 0.0 {
        return f64::NEG_INFINITY;
    }
    if y0 == 1.0 {
        return f64::INFINITY;
    }

    let mut code = true;
    let mut y = y0;

    const EXP_M2: f64 = 0.1353352832366127;

    if y > (1.0 - EXP_M2) {
        y = 1.0 - y;
        code = false;
    }

    if y > EXP_M2 {
        y -= 0.5;
        let y2 = y * y;
        let mut x = y + y
            * (y2 * polynomial(y2, &NDTRI_P0) / polynomial1(y2, &NDTRI_Q0));
        x *= f64::SQRT_2PI;
        return x;
    }

    let mut x = (-2.0 * y.ln()).sqrt();
    let x0 = x - x.ln() / x;
    let z = 1.0 / x;
    let x1 = if x < 8.0 {
        z * polynomial(z, &NDTRI_P1) / polynomial1(z, &NDTRI_Q1)
    } else {
        z * polynomial(z, &NDTRI_P2) / polynomial1(z, &NDTRI_Q2)
    };

    x = x0 - x1;
    if code {
        x = -x;
    }

    x
}
