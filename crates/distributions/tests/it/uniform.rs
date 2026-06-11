use computare_distributions::Uniform;
use computare_testing::arbitrary::{arbtest, f64_range};

use super::compare_cdf_roundtrip;

#[test]
fn uniform_cdf_roundtrip() {
    arbtest(|u| {
        let min = f64_range(u, 0.1, 10.0)?;
        let max = min + f64_range(u, 0.1, 20.0)?;
        compare_cdf_roundtrip(
            &Uniform::new(min, max),
            f64_range(u, 1e-6, 0.9999)?,
            1e-8,
        )
    });
}

#[test]
fn uniform_cdf_roundtrip_narrow() {
    arbtest(|u| {
        let min = f64_range(u, 1e-4, 0.01)?;
        let max = min + f64_range(u, 1e-4, 0.01)?;
        compare_cdf_roundtrip(
            &Uniform::new(min, max),
            f64_range(u, 1e-4, 0.99)?,
            1e-10,
        )
    });
}

#[test]
fn uniform_cdf_roundtrip_wide() {
    arbtest(|u| {
        let min = f64_range(u, 1.0, 100.0)?;
        let max = min + f64_range(u, 1.0, 2000.0)?;
        compare_cdf_roundtrip(
            &Uniform::new(min, max),
            f64_range(u, 1e-4, 0.99)?,
            1e-10,
        )
    });
}
