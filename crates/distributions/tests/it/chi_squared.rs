use computare_core::tolerance::assert_almost_eq;
use computare_distributions::{ChiSquared, Continuous, Statistics};
use computare_testing::arbitrary::{arbtest, f64_range};

use super::compare_cdf_roundtrip;

#[test]
fn median() {
    let cases = [
        (0.5, 0.08573388203017833),
        (1.0, 1.0 - 2.0 / 3.0),
        (2.0, 2.0 - 2.0 / 3.0),
        (2.5, 2.5 - 2.0 / 3.0),
        (3.0, 3.0 - 2.0 / 3.0),
    ];
    for (freedom, expected) in cases {
        assert_almost_eq!(ChiSquared::new(freedom).median().unwrap(), expected);
    }
}

#[test]
fn lower() {
    let cases = [(1.0, 0.0), (2.0, 0.0), (3.0, 0.0)];
    for (freedom, expected) in cases {
        assert_almost_eq!(ChiSquared::new(freedom).lower(), expected);
    }
}

#[test]
fn upper() {
    let cases = [
        (1.0, f64::INFINITY),
        (2.0, f64::INFINITY),
        (3.0, f64::INFINITY),
    ];
    for (freedom, expected) in cases {
        assert_almost_eq!(ChiSquared::new(freedom).upper(), expected);
    }
}

#[test]
fn cdf_roundtrip() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &ChiSquared::new(f64_range(u, 0.01, 100.0)?),
            f64_range(u, 0.1, 0.9)?, // TODO: full range
            1e-13,
        )
    });
}
