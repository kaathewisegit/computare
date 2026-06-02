pub fn polynomial(x: f64, coefficients: &[f64]) -> f64 {
    let Some(mut sum) = coefficients.first().copied() else {
        return 0.0;
    };
    for c in coefficients.iter().skip(1) {
        sum = *c + x * sum;
    }
    sum
}
