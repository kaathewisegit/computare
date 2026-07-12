use computare_distributions::Beta;
use computare_testing::arbitrary::{arbtest, f64_range};

use super::compare_cdf_roundtrip;

#[test]
fn beta_cdf_roundtrip_small_shape() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Beta::new(f64_range(u, 0.1, 0.95)?, f64_range(u, 0.1, 0.95)?),
            f64_range(u, 1e-4, 0.99)?,
            // TODO: blocked on inverse_regularized_beta
            1e-1,
        )
    });
}

#[test]
fn beta_cdf_roundtrip_shape_1_10() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Beta::new(f64_range(u, 1.0, 10.0)?, f64_range(u, 1.0, 10.0)?),
            f64_range(u, 1e-6, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn beta_cdf_roundtrip_shape_10_100() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Beta::new(f64_range(u, 10.0, 100.0)?, f64_range(u, 10.0, 100.0)?),
            f64_range(u, 1e-4, 0.99)?,
            1e-11,
        )
    });
}

#[test]
fn beta_cdf_roundtrip_high_p() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Beta::new(f64_range(u, 1.0, 20.0)?, f64_range(u, 1.0, 20.0)?),
            f64_range(u, 0.9, 0.9999)?,
            1e-10,
        )
    });
}

#[test]
fn beta_cdf_roundtrip_asymmetric() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Beta::new(f64_range(u, 0.1, 1.0)?, f64_range(u, 10.0, 100.0)?),
            f64_range(u, 1e-4, 0.99)?,
            1e-11,
        )
    });
}

#[test]
fn beta_cdf_roundtrip_symmetric() {
    arbtest(|u| {
        let shape = f64_range(u, 0.5, 10.0)?;
        compare_cdf_roundtrip(
            &Beta::new(shape, shape),
            f64_range(u, 1e-4, 0.99)?,
            1e-12,
        )
    });
}
