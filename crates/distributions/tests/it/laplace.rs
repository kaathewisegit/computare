use computare_distributions::Laplace;
use computare_testing::arbitrary::{arbtest, f64_range};

use super::compare_cdf_roundtrip;

#[test]
fn laplace_cdf_roundtrip() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Laplace::new(f64_range(u, 0.1, 10.0)?, f64_range(u, 0.1, 10.0)?),
            f64_range(u, 1e-6, 0.9999)?,
            1e-13,
        )
    });
}

#[test]
fn laplace_cdf_roundtrip_small_scale() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Laplace::new(f64_range(u, 0.1, 1.0)?, f64_range(u, 1e-3, 0.1)?),
            f64_range(u, 1e-4, 0.99)?,
            1e-13,
        )
    });
}

#[test]
fn laplace_cdf_roundtrip_wide() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Laplace::new(f64_range(u, 0.01, 50.0)?, f64_range(u, 0.01, 50.0)?),
            f64_range(u, 1e-4, 0.9999)?,
            1e-12,
        )
    });
}
