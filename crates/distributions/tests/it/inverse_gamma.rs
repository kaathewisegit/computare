use computare_distributions::{
    InverseGamma, InverseGammaUndefinedMean, InverseGammaUndefinedVariance,
    NoClosedFormError, Statistics,
};
use computare_testing::arbitrary::{arbtest, f64_range};

use super::compare_cdf_roundtrip;

#[test]
fn cdf_roundtrip_small_shape() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &InverseGamma::new(
                f64_range(u, 0.1, 1.0)?,
                f64_range(u, 0.1, 10.0)?,
            ),
            f64_range(u, 1e-4, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn mean_error() {
    assert_eq!(
        InverseGamma::new(1.0, 2.0).mean().unwrap_err(),
        InverseGammaUndefinedMean { shape: 1.0 },
    );
}

#[test]
fn median_error() {
    assert_eq!(
        InverseGamma::new(3.0, 2.0).median().unwrap_err(),
        NoClosedFormError,
    );
}

#[test]
fn variance_error() {
    assert_eq!(
        InverseGamma::new(2.0, 2.0).variance().unwrap_err(),
        InverseGammaUndefinedVariance { shape: 2.0 },
    );
}

#[test]
fn cdf_roundtrip_shape_1_10() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &InverseGamma::new(
                f64_range(u, 1.0, 10.0)?,
                f64_range(u, 0.1, 10.0)?,
            ),
            f64_range(u, 1e-6, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn cdf_roundtrip_shape_10_100() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &InverseGamma::new(
                f64_range(u, 10.0, 100.0)?,
                f64_range(u, 0.1, 10.0)?,
            ),
            f64_range(u, 1e-4, 0.99)?,
            1e-11,
        )
    });
}

#[test]
fn cdf_roundtrip_high_p() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &InverseGamma::new(
                f64_range(u, 1.0, 20.0)?,
                f64_range(u, 0.1, 10.0)?,
            ),
            f64_range(u, 0.9, 0.9999)?,
            1e-10,
        )
    });
}

#[test]
fn cdf_roundtrip_small_scale() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &InverseGamma::new(
                f64_range(u, 0.1, 10.0)?,
                f64_range(u, 1e-3, 0.1)?,
            ),
            f64_range(u, 1e-4, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn cdf_roundtrip_small_shape_recip_scale() {
    arbtest(|u| {
        let shape = f64_range(u, 0.05, 0.1)?;
        let scale = shape.recip();
        compare_cdf_roundtrip(
            &InverseGamma::new(shape, scale),
            f64_range(u, 0.01, 0.99)?,
            1e-11,
        )
    });
}
