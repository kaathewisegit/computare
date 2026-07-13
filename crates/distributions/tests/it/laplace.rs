use core::f64::consts::{E, LN_2, LN_10};

use computare_distributions::{Continuous, Laplace, Statistics};
use computare_testing::arbitrary::{arbtest, f64_range};

use super::{assert_almost_eq, compare_cdf_roundtrip};

#[test]
fn mean() {
    let cases = [
        ((f64::NEG_INFINITY, 0.1), f64::NEG_INFINITY),
        ((-6.0, 1.0), -6.0),
        ((0.0, 5.0), 0.0),
        ((1.0, 10.0), 1.0),
        ((f64::INFINITY, f64::INFINITY), f64::INFINITY),
    ];
    for ((location, scale), expected) in cases {
        let d = Laplace::new(location, scale);
        assert_almost_eq!(d.mean().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn variance() {
    let cases = [
        ((f64::NEG_INFINITY, 0.1), 0.02),
        ((-6.0, 1.0), 2.0),
        ((0.0, 5.0), 50.0),
        ((1.0, 7.0), 98.0),
        ((5.0, 10.0), 200.0),
        ((f64::INFINITY, f64::INFINITY), f64::INFINITY),
    ];
    for ((location, scale), expected) in cases {
        let d = Laplace::new(location, scale);
        assert_almost_eq!(d.variance().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn entropy() {
    let cases = [
        ((f64::NEG_INFINITY, 0.1), (2.0 * E * 0.1).ln()),
        ((-6.0, 1.0), (2.0 * E).ln()),
        ((1.0, 7.0), (2.0 * E * 7.0).ln()),
        ((5.0, 10.0), (2.0 * E * 10.0).ln()),
        ((f64::INFINITY, f64::INFINITY), f64::INFINITY),
    ];
    for ((location, scale), expected) in cases {
        let d = Laplace::new(location, scale);
        assert_almost_eq!(d.entropy().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn mode() {
    let cases = [
        ((f64::NEG_INFINITY, 0.1), f64::NEG_INFINITY),
        ((-6.0, 1.0), -6.0),
        ((1.0, 7.0), 1.0),
        ((5.0, 10.0), 5.0),
        ((f64::INFINITY, f64::INFINITY), f64::INFINITY),
    ];
    for ((location, scale), expected) in cases {
        let d = Laplace::new(location, scale);
        assert_almost_eq!(d.mode().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn median() {
    let cases = [
        ((f64::NEG_INFINITY, 0.1), f64::NEG_INFINITY),
        ((-6.0, 1.0), -6.0),
        ((1.0, 7.0), 1.0),
        ((5.0, 10.0), 5.0),
        ((f64::INFINITY, f64::INFINITY), f64::INFINITY),
    ];
    for ((location, scale), expected) in cases {
        let d = Laplace::new(location, scale);
        assert_almost_eq!(d.median().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn lower() {
    let cases = [((0.0, 1.0), f64::NEG_INFINITY)];
    for ((location, scale), expected) in cases {
        assert_almost_eq!(Laplace::new(location, scale).lower(), expected);
    }
}

#[test]
fn upper() {
    let cases = [((0.0, 1.0), f64::INFINITY)];
    for ((location, scale), expected) in cases {
        assert_almost_eq!(Laplace::new(location, scale).upper(), expected);
    }
}

#[test]
fn pdf() {
    let cases = [
        ((0.0, 0.1), 1.5, 1.529511602509129e-6),
        ((1.0, 0.1), 2.8, 7.614989872356341e-8),
        ((-1.0, 0.1), -5.4, 3.8905661205668983e-19),
        ((5.0, 0.1), -4.9, 5.056107463052243e-43),
        ((-5.0, 0.1), 2.0, 1.9877248679543235e-30),
        ((f64::INFINITY, 0.1), 5.5, 0.0),
        ((f64::NEG_INFINITY, 0.1), -0.0, 0.0),
        ((0.0, 1.0), f64::INFINITY, 0.0),
        ((1.0, 1.0), 5.0, 0.00915781944436709),
        ((-1.0, 1.0), -1.0, 0.5),
        ((5.0, 1.0), -1.0, 0.0012393760883331792),
        ((-5.0, 1.0), 2.5, 0.0002765421850739168),
        ((f64::INFINITY, 0.1), 2.0, 0.0),
        ((f64::NEG_INFINITY, 0.1), 15.0, 0.0),
        ((0.0, f64::INFINITY), 89.3, 0.0),
        ((1.0, f64::INFINITY), -0.1, 0.0),
        ((-1.0, f64::INFINITY), 0.1, 0.0),
        ((5.0, f64::INFINITY), -6.1, 0.0),
        ((-5.0, f64::INFINITY), -10.0, 0.0),
    ];
    for ((location, scale), x, expected) in cases {
        let d = Laplace::new(location, scale);
        assert_almost_eq!(d.pdf(x), expected, relative = 1e-12);
    }

    let dist = Laplace::new(f64::INFINITY, f64::INFINITY);
    for x in [2.0, -5.1] {
        assert!(dist.pdf(x).is_nan());
    }
}

#[test]
fn ln_pdf() {
    let cases = [
        ((0.0, 0.1), 1.5, -13.3905620875659),
        ((1.0, 0.1), 2.8, -16.390562087565897),
        ((-1.0, 0.1), -5.4, -42.39056208756591),
        ((5.0, 0.1), -4.9, -97.3905620875659),
        ((-5.0, 0.1), 2.0, -68.3905620875659),
        ((f64::INFINITY, 0.1), 5.5, f64::NEG_INFINITY),
        ((f64::NEG_INFINITY, 0.1), -0.0, f64::NEG_INFINITY),
        ((0.0, 1.0), f64::INFINITY, f64::NEG_INFINITY),
        ((1.0, 1.0), 5.0, -4.693147180559945),
        ((-1.0, 1.0), -1.0, -LN_2),
        ((5.0, 1.0), -1.0, -6.693147180559945),
        ((-5.0, 1.0), 2.5, -8.193147180559945),
        ((f64::INFINITY, 0.1), 2.0, f64::NEG_INFINITY),
        ((f64::NEG_INFINITY, 0.1), 15.0, f64::NEG_INFINITY),
        ((0.0, f64::INFINITY), 89.3, f64::NEG_INFINITY),
        ((1.0, f64::INFINITY), -0.1, f64::NEG_INFINITY),
        ((-1.0, f64::INFINITY), 0.1, f64::NEG_INFINITY),
        ((5.0, f64::INFINITY), -6.1, f64::NEG_INFINITY),
        ((-5.0, f64::INFINITY), -10.0, f64::NEG_INFINITY),
    ];
    for ((location, scale), x, expected) in cases {
        let d = Laplace::new(location, scale);
        assert_almost_eq!(d.ln_pdf(x), expected, relative = 1e-12);
    }

    let dist = Laplace::new(f64::INFINITY, f64::INFINITY);
    for x in [2.0, -5.1] {
        assert!(dist.ln_pdf(x).is_nan());
    }
}

#[test]
fn cdf() {
    let cases = [
        ((0.0, 1.0), 0.5, 0.6967346701436833),
        ((0.0, 1.0), -0.5, 0.3032653298563167),
        ((0.0, 1.0), -100.0, 1.860037988010418e-44),
    ];
    for ((location, scale), x, expected) in cases {
        let d = Laplace::new(location, scale);
        assert_almost_eq!(d.cdf(x), expected, relative = 1e-12);
    }
}

#[test]
fn sf() {
    let cases = [
        ((0.0, 1.0), 0.5, 0.3032653298563167),
        ((0.0, 1.0), -0.5, 0.6967346701436833),
        ((0.0, 1.0), 100.0, 1.860037988010418e-44),
    ];
    for ((location, scale), x, expected) in cases {
        let d = Laplace::new(location, scale);
        assert_almost_eq!(d.sf(x), expected, relative = 1e-12);
    }
}

#[test]
fn inverse_cdf() {
    let cases = [
        ((0.0, 1.0), 1e-10, -22.33270374938051),
        ((0.0, 1.0), 0.001, -6.214608098422191),
        ((0.0, 1.0), 0.95, LN_10),
    ];
    for ((location, scale), p, expected) in cases {
        let d = Laplace::new(location, scale);
        assert_almost_eq!(d.inverse_cdf(p), expected, relative = 1e-12);
    }
}

#[test]
fn cdf_roundtrip() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Laplace::new(f64_range(u, 0.1, 10.0)?, f64_range(u, 0.1, 10.0)?),
            f64_range(u, 1e-6, 0.9999)?,
            1e-13,
        )
    });
}

#[test]
fn cdf_roundtrip_small_scale() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Laplace::new(f64_range(u, 0.1, 1.0)?, f64_range(u, 1e-3, 0.1)?),
            f64_range(u, 1e-4, 0.99)?,
            1e-13,
        )
    });
}

#[test]
fn cdf_roundtrip_wide() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Laplace::new(f64_range(u, 0.01, 50.0)?, f64_range(u, 0.01, 50.0)?),
            f64_range(u, 1e-4, 0.9999)?,
            1e-12,
        )
    });
}
