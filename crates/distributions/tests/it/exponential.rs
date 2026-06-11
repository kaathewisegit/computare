use computare_distributions::Exponential;
use computare_testing::arbitrary::{arbtest, f64_range};

use super::compare_cdf_roundtrip;

#[test]
fn exponential_cdf_roundtrip() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Exponential::new(f64_range(u, 0.5, 5.0)?),
            f64_range(u, 1e-3, 0.99)?,
            1e-16,
        )
    });
}

#[test]
fn exponential_cdf_roundtrip_small_rate() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Exponential::new(f64_range(u, 0.1, 0.5)?),
            f64_range(u, 1e-3, 0.99)?,
            1e-16,
        )
    });
}

#[test]
fn exponential_cdf_roundtrip_large_rate() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Exponential::new(f64_range(u, 5.0, 20.0)?),
            f64_range(u, 0.1, 0.99)?,
            1e-16,
        )
    });
}
