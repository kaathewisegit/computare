use crate::gamma::ln_gamma;

const MAX_FACTORIAL: usize = 170;

const FCACHE: [f64; MAX_FACTORIAL + 1] = {
    let mut fcache = [1.0; MAX_FACTORIAL + 1];

    // `const` only allow while loops (because `next` on `Iterator` isn't
    // `const`)
    let mut i = 1;
    while i < MAX_FACTORIAL + 1 {
        fcache[i] = fcache[i - 1] * i as f64;
        i += 1;
    }

    fcache
};

/// `ln(x!)`
///
/// Returns `0.0` if `x <= 1`.
pub fn ln_factorial(x: u64) -> f64 {
    let x = x as usize;
    FCACHE
        .get(x)
        .map_or_else(|| ln_gamma(x as f64 + 1.0), |&fac| fac.ln())
}

/// Natural logarithm of the binomial coefficient
///
/// Returns negative infinity if `k > n`.
pub fn ln_binomial(n: u64, k: u64) -> f64 {
    if k > n {
        f64::NEG_INFINITY
    } else {
        ln_factorial(n) - ln_factorial(k) - ln_factorial(n - k)
    }
}
