use computare_core::debug_panic;

// Expansion coefficients for Euler-Maclaurin summation formula `(2k)! / B2k`,
// where B2k are Bernoulli numbers
const ZETA_A: [f64; 12] = [
    12.0,
    -720.0,
    30240.0,
    -1209600.0,
    47900160.0,
    -1.8924375803183792e9,
    7.47242496e10,
    -2.950130727918164e12,
    1.1646782814350067e14,
    -4.597978722407473e15,
    1.8152105401943546e17,
    -7.166165256175667e18,
];

pub fn zeta(x: f64, q: f64) -> f64 {
    if x == 1.0 {
        return f64::INFINITY;
    }

    if x < 1.0 {
        debug_panic!("domain error");
        return f64::NAN;
    }

    if q <= 0.0 {
        if q == q.floor() {
            debug_panic!("domain error");
            return f64::INFINITY;
        }
        if x != x.floor() {
            debug_panic!("domain error");
            return f64::NAN;
        }
    }

    // Asymptotic expansion: https://dlmf.nist.gov/25.11#E43
    if q > 1e8 {
        return (1.0 / (x - 1.0) + 1.0 / (2.0 * q)) * q.powf(1.0 - x);
    }

    // Euler-Maclaurin summation formula
    let mut s = q.powf(-x);
    let mut a = q;
    let mut i = 0;
    let mut b = 0.0;

    while (i < 9) || (a <= 9.0) {
        i += 1;
        a += 1.0;
        b = a.powf(-x);
        s += b;
        if (b / s).abs() < f64::EPSILON {
            return s;
        }
    }

    let w = a;
    s += b * w / (x - 1.0);
    s -= 0.5 * b;
    a = 1.0;
    let mut k = 0.0;

    for coef in &ZETA_A {
        a *= x + k;
        b /= w;
        let t = a * b / coef;
        s += t;
        if (t / s).abs() < f64::EPSILON {
            break;
        }
        k += 1.0;
        a *= x + k;
        b /= w;
        k += 1.0;
    }

    s
}
