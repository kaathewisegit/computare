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
