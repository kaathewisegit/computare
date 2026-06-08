pub fn polynomial(x: f64, coefficients: &[f64]) -> f64 {
    let Some(mut sum) = coefficients.first().copied() else {
        return 0.0;
    };
    for &c in coefficients.iter().skip(1) {
        sum = c + x * sum;
    }
    sum
}

/// A polynomial whose highest `x^n` coefficients is exactly 1
///
/// This means that the length of `coefficients` should be `n - 1`.
pub fn polynomial1(x: f64, coefficients: &[f64]) -> f64 {
    let Some(mut sum) = coefficients.first().copied() else {
        return 0.0;
    };
    sum += x;
    for &c in coefficients.iter().skip(1) {
        sum = c + x * sum;
    }
    sum
}

/// `polynomial(x, numerator_coefficients) / polynomial(x, denominator_coefficents)`
pub fn rational_function(
    x: f64,
    numerator_coefficients: &[f64],
    denominator_coefficents: &[f64],
) -> f64 {
    let m = numerator_coefficients.len() - 1;
    let n = denominator_coefficents.len() - 1;

    let absx = x.abs();
    let dir: isize;
    let mut p_idx: isize;
    let y: f64;

    if absx > 1.0 {
        // Evaluate as a polynomial in 1/x
        dir = -1;
        p_idx = m as isize;
        y = 1.0 / x;
    } else {
        dir = 1;
        p_idx = 0;
        y = x;
    }

    // Evaluate the numerator
    let mut num_acc = numerator_coefficients[p_idx as usize];
    p_idx += dir;
    for _ in 1..=m {
        num_acc = num_acc * y + numerator_coefficients[p_idx as usize];
        p_idx += dir;
    }

    // Evaluate the denominator
    if absx > 1.0 {
        p_idx = n as isize;
    } else {
        p_idx = 0;
    }

    let mut denom_acc = denominator_coefficents[p_idx as usize];
    p_idx += dir;
    for _ in 1..=n {
        denom_acc = denom_acc * y + denominator_coefficents[p_idx as usize];
        p_idx += dir;
    }

    if absx > 1.0 {
        let i = (m as i32) - (n as i32);
        x.powi(i) * num_acc / denom_acc
    } else {
        num_acc / denom_acc
    }
}

pub fn chebyshev(x: f64, array: &[f64]) -> f64 {
    let mut b0 = array[0];
    let mut b1 = 0.0;
    let mut b2 = 0.0;

    for &coef in &array[1..] {
        b2 = b1;
        b1 = b0;
        b0 = x * b1 - b2 + coef;
    }

    0.5 * (b0 - b2)
}
