use computare_distributions::Chi;
use computare_testing::arbitrary::{arbtest, f64_range};

use super::compare_cdf_roundtrip;

#[test]
fn chi_cdf_roundtrip_freedom_1() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Chi::try_new(f64_range(u, 1.0, 2.0)?).unwrap(),
            f64_range(u, 1e-4, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn chi_cdf_roundtrip_small_freedom() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Chi::try_new(f64_range(u, 1.0, 5.0)?).unwrap(),
            f64_range(u, 1e-4, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn chi_cdf_roundtrip_freedom_2_20() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Chi::try_new(f64_range(u, 2.0, 20.0)?).unwrap(),
            f64_range(u, 1e-6, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn chi_cdf_roundtrip_freedom_20_200() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Chi::try_new(f64_range(u, 20.0, 200.0)?).unwrap(),
            f64_range(u, 1e-4, 0.99)?,
            1e-11,
        )
    });
}

#[test]
fn chi_cdf_roundtrip_high_p() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Chi::try_new(f64_range(u, 2.0, 40.0)?).unwrap(),
            f64_range(u, 0.9, 0.9999)?,
            1e-10,
        )
    });
}
