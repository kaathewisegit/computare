use core::f64::consts::{LN_2, LN_10};

use computare_core::tolerance::assert_almost_eq;
use computare_distributions::{Continuous, Statistics, Uniform};
use computare_testing::arbitrary::{arbtest, f64_range};

use super::compare_cdf_roundtrip;

#[test]
fn variance() {
    let cases = [
        ((-0.0, 2.0), 1.0 / 3.0),
        ((0.0, 2.0), 1.0 / 3.0),
        ((0.1, 4.0), 1.2675),
        ((10.0, 11.0), 1.0 / 12.0),
    ];
    for ((min, max), expected) in cases {
        assert_almost_eq!(Uniform::new(min, max).variance().unwrap(), expected);
    }
}

#[test]
fn entropy() {
    let cases = [
        ((-0.0, 2.0), LN_2),
        ((0.0, 2.0), LN_2),
        ((0.1, 4.0), 1.3609765531356008),
        ((1.0, 10.0), 2.1972245773362196),
        ((10.0, 11.0), 0.0),
    ];
    for ((min, max), expected) in cases {
        assert_almost_eq!(Uniform::new(min, max).entropy().unwrap(), expected);
    }
}

#[test]
fn mode() {
    let cases = [
        (-0.0, 2.0),
        (0.0, 2.0),
        (0.1, 4.0),
        (1.0, 10.0),
        (10.0, 11.0),
    ];
    for (min, max) in cases {
        Uniform::new(min, max).mode().unwrap_err();
    }
}

#[test]
fn median() {
    let cases = [
        ((-0.0, 2.0), 1.0),
        ((0.0, 2.0), 1.0),
        ((0.1, 4.0), 2.05),
        ((1.0, 10.0), 5.5),
        ((10.0, 11.0), 10.5),
    ];
    for ((min, max), expected) in cases {
        assert_almost_eq!(
            Uniform::new(min, max).median().unwrap(),
            expected,
            relative = 1e-17
        );
    }
}

#[test]
fn pdf() {
    let cases = [
        ((0.0, 0.1), -5.0, 0.0),
        ((0.0, 0.1), 0.05, 10.0),
        ((0.0, 0.1), 5.0, 0.0),
        ((0.0, 1.0), -5.0, 0.0),
        ((0.0, 1.0), 0.5, 1.0),
        ((0.0, 0.1), 5.0, 0.0),
        ((0.0, 10.0), -5.0, 0.0),
        ((0.0, 10.0), 1.0, 0.1),
        ((0.0, 10.0), 5.0, 0.1),
        ((0.0, 10.0), 11.0, 0.0),
        ((-5.0, 100.0), -10.0, 0.0),
        ((-5.0, 100.0), -5.0, 0.009523809523809525),
        ((-5.0, 100.0), 0.0, 0.009523809523809525),
        ((-5.0, 100.0), 101.0, 0.0),
    ];
    for ((min, max), p, expected) in cases {
        assert_almost_eq!(
            Uniform::new(min, max).pdf(p),
            expected,
            relative = 1e-17
        );
    }
}

#[test]
fn ln_pdf() {
    let cases = [
        ((0.0, 0.1), -5.0, f64::NEG_INFINITY),
        ((0.0, 0.1), 0.05, LN_10),
        ((0.0, 0.1), 5.0, f64::NEG_INFINITY),
        ((0.0, 1.0), -5.0, f64::NEG_INFINITY),
        ((0.0, 1.0), 0.5, 0.0),
        ((0.0, 0.1), 5.0, f64::NEG_INFINITY),
        ((0.0, 10.0), -5.0, f64::NEG_INFINITY),
        ((0.0, 10.0), 1.0, -LN_10),
        ((0.0, 10.0), 5.0, -LN_10),
        ((0.0, 10.0), 11.0, f64::NEG_INFINITY),
        ((-5.0, 100.0), -10.0, f64::NEG_INFINITY),
        ((-5.0, 100.0), -5.0, -4.653960350157523),
        ((-5.0, 100.0), 0.0, -4.653960350157523),
        ((-5.0, 100.0), 101.0, f64::NEG_INFINITY),
    ];
    for ((min, max), p, expected) in cases {
        assert_almost_eq!(Uniform::new(min, max).ln_pdf(p), expected);
    }
}

#[test]
fn cdf() {
    let cases = [
        ((0.0, 0.1), 0.05, 0.5),
        ((0.0, 1.0), 0.5, 0.5),
        ((0.0, 10.0), 1.0, 0.1),
        ((0.0, 10.0), 5.0, 0.5),
        ((-5.0, 100.0), -5.0, 0.0),
        ((-5.0, 100.0), 0.0, 0.047619047619047616),
        ((0.0, 3.0), -1.0, 0.0), // test_cdf_lower_bound
        ((0.0, 3.0), 5.0, 1.0),  // test_cdf_upper_bound
    ];
    for ((min, max), p, expected) in cases {
        assert_almost_eq!(
            Uniform::new(min, max).cdf(p),
            expected,
            relative = 1e-17
        );
    }
}

#[test]
fn inverse_cdf() {
    let cases = [
        ((0.0, 0.1), 0.5, 0.05),
        ((0.0, 10.0), 0.5, 5.0),
        ((1.0, 10.0), 0.0, 1.0),
        ((1.0, 10.0), 1.0 / 3.0, 4.0),
        ((1.0, 10.0), 1.0, 10.0),
    ];
    for ((min, max), p, expected) in cases {
        let dist = Uniform::new(min, max);
        assert_almost_eq!(dist.inverse_cdf(p), expected);
    }
}

#[test]
fn sf() {
    let cases = [
        ((0.0, 0.1), 0.05, 0.5),
        ((0.0, 1.0), 0.5, 0.5),
        ((0.0, 10.0), 1.0, 0.9),
        ((0.0, 10.0), 5.0, 0.5),
        ((-5.0, 100.0), -5.0, 1.0),
        ((-5.0, 100.0), 0.0, 0.9523809523809523),
        ((0.0, 3.0), -1.0, 1.0), // test_sf_lower_bound
        ((0.0, 3.0), 5.0, 0.0),  // test_sf_upper_bound
    ];
    for ((min, max), p, expected) in cases {
        assert_almost_eq!(
            Uniform::new(min, max).sf(p),
            expected,
            relative = 1e-17
        );
    }
}

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
