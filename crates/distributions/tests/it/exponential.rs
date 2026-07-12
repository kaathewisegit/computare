use core::f64::consts::{LN_2, LN_10};

use computare_distributions::{Continuous, Exponential, Statistics};
use computare_testing::arbitrary::{arbtest, f64_range};

use super::{assert_almost_eq, compare_cdf_roundtrip};

#[test]
fn exponential_mean() {
    let cases = [(0.1, 10.0), (1.0, 1.0), (10.0, 0.1)];
    for (rate, expected) in cases {
        let d = Exponential::new(rate);
        assert_almost_eq!(d.mean().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn exponential_variance() {
    let cases = [(0.1, 100.0), (1.0, 1.0), (10.0, 0.01)];
    for (rate, expected) in cases {
        let d = Exponential::new(rate);
        assert_almost_eq!(d.variance().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn exponential_entropy() {
    let cases = [
        (0.1, 3.302585092994046),
        (1.0, 1.0),
        (10.0, -1.3025850929940457),
    ];
    for (rate, expected) in cases {
        let d = Exponential::new(rate);
        assert_almost_eq!(d.entropy().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn exponential_median() {
    let cases = [
        (0.1, 6.931471805599453),
        (1.0, LN_2),
        (10.0, 0.06931471805599453),
    ];
    for (rate, expected) in cases {
        let d = Exponential::new(rate);
        assert_almost_eq!(d.median().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn exponential_mode() {
    let cases = [(0.1, 0.0), (1.0, 0.0), (10.0, 0.0)];
    for (rate, expected) in cases {
        let d = Exponential::new(rate);
        assert_almost_eq!(d.mode().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn exponential_lower() {
    let cases = [(0.1, 0.0), (1.0, 0.0), (10.0, 0.0)];
    for (rate, expected) in cases {
        assert_almost_eq!(Exponential::new(rate).lower(), expected);
    }
}

#[test]
fn exponential_upper() {
    let cases = [
        (0.1, f64::INFINITY),
        (1.0, f64::INFINITY),
        (10.0, f64::INFINITY),
    ];
    for (rate, expected) in cases {
        assert_almost_eq!(Exponential::new(rate).upper(), expected);
    }
}

#[test]
fn exponential_pdf() {
    let cases = [
        (0.1, 0.0, 0.1),
        (1.0, 0.0, 1.0),
        (10.0, 0.0, 10.0),
        (0.1, 0.1, 0.09900498337491681),
        (1.0, 0.1, 0.9048374180359596),
        (10.0, 0.1, 3.6787944117144233),
        (0.1, 1.0, 0.09048374180359596),
        (1.0, 1.0, 0.36787944117144233),
        (10.0, 1.0, 4.539992976248485e-4),
        (0.1, f64::INFINITY, 0.0),
        (1.0, f64::INFINITY, 0.0),
        (10.0, f64::INFINITY, 0.0),
        // lt_0
        (0.1, -1.0, 0.0),
    ];
    for (rate, x, expected) in cases {
        let d = Exponential::new(rate);
        assert_almost_eq!(d.pdf(x), expected, relative = 1e-12);
    }
}

#[test]
fn exponential_pdf_nan() {
    let cases = [
        (f64::INFINITY, 0.0),
        (f64::INFINITY, 0.1),
        (f64::INFINITY, 1.0),
        (f64::INFINITY, f64::INFINITY),
    ];
    for (rate, x) in cases {
        assert!(Exponential::new(rate).pdf(x).is_nan());
    }
}

#[test]
fn exponential_ln_pdf() {
    let cases = [
        (0.1, 0.0, -LN_10),
        (1.0, 0.0, 0.0),
        (10.0, 0.0, LN_10),
        (0.1, 0.1, -2.3125850929940457),
        (1.0, 0.1, -0.1),
        (10.0, 0.1, 1.3025850929940457),
        (0.1, 1.0, -2.4025850929940455),
        (1.0, 1.0, -1.0),
        (10.0, 1.0, -7.697414907005954),
        (0.1, f64::INFINITY, f64::NEG_INFINITY),
        (1.0, f64::NEG_INFINITY, f64::NEG_INFINITY),
        (10.0, f64::NEG_INFINITY, f64::NEG_INFINITY),
        // lt_0
        (0.1, -1.0, f64::NEG_INFINITY),
    ];
    for (rate, x, expected) in cases {
        let d = Exponential::new(rate);
        assert_almost_eq!(d.ln_pdf(x), expected, relative = 1e-12);
    }
}

#[test]
fn exponential_ln_pdf_nan() {
    let cases = [
        (f64::INFINITY, 0.0),
        (f64::INFINITY, 0.1),
        (f64::INFINITY, 1.0),
        (f64::INFINITY, f64::INFINITY),
    ];
    for (rate, x) in cases {
        assert!(Exponential::new(rate).ln_pdf(x).is_nan());
    }
}

#[test]
fn exponential_cdf() {
    let cases = [
        (0.1, 0.0, 0.0),
        (1.0, 0.0, 0.0),
        (10.0, 0.0, 0.0),
        (0.1, 0.1, 0.009950166250831947),
        (1.0, 0.1, 0.09516258196404043),
        (10.0, 0.1, 0.6321205588285577),
        (f64::INFINITY, 0.1, 1.0),
        (0.1, 1.0, 0.09516258196404043),
        (1.0, 1.0, 0.6321205588285577),
        (10.0, 1.0, 0.9999546000702375),
        (f64::INFINITY, 1.0, 1.0),
        (0.1, f64::INFINITY, 1.0),
        (1.0, f64::INFINITY, 1.0),
        (10.0, f64::INFINITY, 1.0),
        (f64::INFINITY, f64::INFINITY, 1.0),
        // lt_0
        (0.1, -1.0, 0.0),
    ];
    for (rate, x, expected) in cases {
        let d = Exponential::new(rate);
        assert_almost_eq!(d.cdf(x), expected, relative = 1e-12);
    }

    assert!(Exponential::new(f64::INFINITY).cdf(0.0).is_nan());
}

#[test]
fn exponential_inverse_cdf_identity() {
    let rates = [0.42, 0.042, 0.0042, 0.33, 0.033, 0.0033];
    for rate in rates {
        let dist = Exponential::new(rate);
        assert_almost_eq!(
            dist.inverse_cdf(0.5),
            dist.median().unwrap(),
            relative = 1e-12
        );
    }
}

#[test]
fn exponential_sf() {
    let cases = [
        (0.1, 0.0, 1.0),
        (1.0, 0.0, 1.0),
        (10.0, 0.0, 1.0),
        (0.1, 0.1, 0.9900498337491681),
        (1.0, 0.1, 0.9048374180359595),
        (10.0, 0.1, 0.36787944117144233),
        (f64::INFINITY, 0.1, 0.0),
        // lt_0
        (0.1, -1.0, 1.0),
    ];
    for (rate, x, expected) in cases {
        let d = Exponential::new(rate);
        assert_almost_eq!(d.sf(x), expected, relative = 1e-12);
    }

    assert!(Exponential::new(f64::INFINITY).sf(0.0).is_nan());
}

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
