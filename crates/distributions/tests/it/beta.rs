use computare_distributions::{Beta, Continuous, Statistics};
use computare_testing::arbitrary::{arbtest, f64_range};

use super::{assert_almost_eq, compare_cdf_roundtrip};

#[test]
fn mean() {
    let cases = [
        ((1.0, 1.0), 0.5),
        ((9.0, 1.0), 0.9),
        ((5.0, 100.0), 0.047619047619047616),
    ];
    for ((a, b), expected) in cases {
        let d = Beta::new(a, b);
        assert_almost_eq!(d.mean().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn variance() {
    let cases = [
        ((1.0, 1.0), 1.0 / 12.0),
        ((9.0, 1.0), 9.0 / 1100.0),
        ((5.0, 100.0), 500.0 / 1168650.0),
    ];
    for ((a, b), expected) in cases {
        let d = Beta::new(a, b);
        assert_almost_eq!(d.variance().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn entropy() {
    let cases = [
        ((9.0, 1.0), -1.3083356884473305),
        ((5.0, 100.0), -2.520162318760274),
        ((1.0, 1.0), 0.0),
    ];
    for ((a, b), expected) in cases {
        let d = Beta::new(a, b);
        assert_almost_eq!(
            d.entropy().unwrap(),
            expected,
            absolute = f64::EPSILON * 4.0,
            relative = 1e-13
        );
    }
}

#[test]
fn mode() {
    let cases = [((5.0, 100.0), 0.038834951456310676)];
    for ((a, b), expected) in cases {
        let d = Beta::new(a, b);
        assert_almost_eq!(d.mode().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn mode_none() {
    let cases = [(1.0, 5.0), (5.0, 1.0)];
    for (a, b) in cases {
        assert!(Beta::new(a, b).mode().is_err());
    }
}

#[test]
fn lower() {
    let cases = [((1.0, 1.0), 0.0), ((1e10, 1e-10), 0.0)];
    for ((a, b), expected) in cases {
        assert_almost_eq!(Beta::new(a, b).lower(), expected);
    }
}

#[test]
fn upper() {
    let cases = [((1.0, 1.0), 1.0), ((1e10, 1e-10), 1.0)];
    for ((a, b), expected) in cases {
        assert_almost_eq!(Beta::new(a, b).upper(), expected);
    }
}

#[test]
fn pdf() {
    let cases = [
        ((1.0, 1.0), 0.0, 1.0),
        ((1.0, 1.0), 0.5, 1.0),
        ((1.0, 1.0), 1.0, 1.0),
        ((9.0, 1.0), 0.0, 0.0),
        ((9.0, 1.0), 0.5, 0.03515625),
        ((9.0, 1.0), 1.0, 9.0),
        ((5.0, 100.0), 0.0, 0.0),
        ((5.0, 100.0), 0.5, 4.5341022983503377e-23),
        ((5.0, 100.0), 1.0, 0.0),
        ((5.0, 100.0), 1.0, 0.0),
        // lt_0
        ((1.0, 1.0), -1.0, 0.0),
        // gt_1
        ((1.0, 1.0), 2.0, 0.0),
    ];
    for ((a, b), x, expected) in cases {
        let d = Beta::new(a, b);
        assert_almost_eq!(d.pdf(x), expected, relative = 1e-12);
    }
}

#[test]
fn ln_pdf() {
    let cases = [
        ((1.0, 1.0), 0.0, 0.0),
        ((1.0, 1.0), 0.5, 0.0),
        ((1.0, 1.0), 1.0, 0.0),
        ((9.0, 1.0), 0.0, f64::NEG_INFINITY),
        ((9.0, 1.0), 0.5, -3.347952867143343),
        ((9.0, 1.0), 1.0, 2.1972245773362196),
        ((5.0, 100.0), 0.0, f64::NEG_INFINITY),
        ((5.0, 100.0), 0.5, -51.44783002453768),
        ((5.0, 100.0), 1.0, f64::NEG_INFINITY),
        // lt_0
        ((1.0, 1.0), -1.0, f64::NEG_INFINITY),
        // gt_1
        ((1.0, 1.0), 2.0, f64::NEG_INFINITY),
    ];
    for ((a, b), x, expected) in cases {
        let d = Beta::new(a, b);
        assert_almost_eq!(d.ln_pdf(x), expected, relative = 1e-12);
    }
}

#[test]
fn cdf() {
    let cases = [
        ((1.0, 1.0), 0.0, 0.0),
        ((1.0, 1.0), 0.5, 0.5),
        ((1.0, 1.0), 1.0, 1.0),
        ((9.0, 1.0), 0.0, 0.0),
        ((9.0, 1.0), 0.5, 0.001953125),
        ((9.0, 1.0), 1.0, 1.0),
        ((5.0, 100.0), 0.0, 0.0),
        ((5.0, 100.0), 0.5, 1.0),
        ((5.0, 100.0), 1.0, 1.0),
    ];
    for ((a, b), x, expected) in cases {
        let d = Beta::new(a, b);
        assert_almost_eq!(d.cdf(x), expected, relative = 1e-12);
    }
}

#[test]
fn sf() {
    let cases = [
        ((1.0, 1.0), 0.0, 1.0),
        ((1.0, 1.0), 0.5, 0.5),
        ((1.0, 1.0), 1.0, 0.0),
        ((9.0, 1.0), 0.0, 1.0),
        ((9.0, 1.0), 0.5, 0.998046875),
        ((9.0, 1.0), 1.0, 0.0),
        ((5.0, 100.0), 0.0, 1.0),
        ((5.0, 100.0), 0.5, 0.0),
        ((5.0, 100.0), 1.0, 0.0),
        // lt_0
        ((1.0, 1.0), -1.0, 1.0),
        // gt_1
        ((1.0, 1.0), 2.0, 0.0),
    ];
    for ((a, b), x, expected) in cases {
        let d = Beta::new(a, b);
        assert_almost_eq!(d.sf(x), expected, relative = 1e-12);
    }
}

#[test]
fn inverse_cdf() {
    let cases = [
        ((1.0, 1.0), 0.0, 0.0),
        ((1.0, 1.0), 0.5, 0.5),
        ((1.0, 1.0), 1.0, 1.0),
        ((9.0, 1.0), 0.0, 0.0),
        ((9.0, 1.0), 0.001953125, 0.001953125),
        ((9.0, 1.0), 0.5, 0.5),
        ((9.0, 1.0), 1.0, 1.0),
        ((5.0, 100.0), 0.0, 0.0),
        ((5.0, 100.0), 0.01, 0.01),
        ((5.0, 100.0), 1.0, 1.0),
        // lt_0
        ((1.0, 1.0), -1.0, 0.0),
        // gt_1
        ((1.0, 1.0), 2.0, 1.0),
    ];
    for ((a, b), p, expected) in cases {
        let d = Beta::new(a, b);
        assert_almost_eq!(d.inverse_cdf(d.cdf(p)), expected, relative = 1e-12);
    }
}

#[test]
fn cdf_roundtrip_small_shape() {
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
fn cdf_roundtrip_shape_1_10() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Beta::new(f64_range(u, 1.0, 10.0)?, f64_range(u, 1.0, 10.0)?),
            f64_range(u, 1e-6, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn cdf_roundtrip_shape_10_100() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Beta::new(f64_range(u, 10.0, 100.0)?, f64_range(u, 10.0, 100.0)?),
            f64_range(u, 1e-4, 0.99)?,
            1e-11,
        )
    });
}

#[test]
fn cdf_roundtrip_high_p() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Beta::new(f64_range(u, 1.0, 20.0)?, f64_range(u, 1.0, 20.0)?),
            f64_range(u, 0.9, 0.9999)?,
            1e-10,
        )
    });
}

#[test]
fn cdf_roundtrip_asymmetric() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Beta::new(f64_range(u, 0.1, 1.0)?, f64_range(u, 10.0, 100.0)?),
            f64_range(u, 1e-4, 0.99)?,
            1e-11,
        )
    });
}

#[test]
fn cdf_roundtrip_symmetric() {
    arbtest(|u| {
        let shape = f64_range(u, 0.5, 10.0)?;
        compare_cdf_roundtrip(
            &Beta::new(shape, shape),
            f64_range(u, 1e-4, 0.99)?,
            1e-12,
        )
    });
}
