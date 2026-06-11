use computare_distributions::LogNormal;
use computare_testing::arbitrary::{arbtest, f64_range};

use super::compare_cdf_roundtrip;

#[test]
fn log_normal_cdf_roundtrip() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &LogNormal::new(f64_range(u, 0.1, 5.0)?, f64_range(u, 0.1, 5.0)?),
            f64_range(u, 1e-6, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn log_normal_cdf_roundtrip_small_scale() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &LogNormal::new(f64_range(u, 0.1, 2.0)?, f64_range(u, 1e-3, 0.1)?),
            f64_range(u, 1e-4, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn log_normal_cdf_roundtrip_wide_scale() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &LogNormal::new(f64_range(u, 0.1, 2.0)?, f64_range(u, 0.1, 10.0)?),
            f64_range(u, 1e-4, 0.9999)?,
            1e-11,
        )
    });
}

#[test]
fn log_normal_cdf_roundtrip_tails() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &LogNormal::new(f64_range(u, 0.1, 5.0)?, f64_range(u, 0.1, 5.0)?),
            f64_range(u, 0.9, 0.9999)?,
            1e-10,
        )
    });
}
