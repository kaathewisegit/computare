use computare_distributions::{Continuous, Normal, Statistics};
use computare_testing::arbitrary::{arbtest, f64_range};

use super::{assert_almost_eq, compare_cdf_roundtrip};

#[test]
fn normal_variance() {
    let cases = [
        ((0.0, 0.1), 0.1 * 0.1),
        ((0.0, 1.0), 1.0),
        ((0.0, 10.0), 100.0),
        ((0.0, f64::INFINITY), f64::INFINITY),
    ];
    for ((mean, std), expected) in cases {
        let n = Normal::new(mean, std);
        assert_almost_eq!(n.variance().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn normal_entropy() {
    let cases = [
        ((0.0, 0.1), -0.8836465597893729),
        ((0.0, 1.0), 1.4189385332046727),
        ((0.0, 10.0), 3.7215236261987186),
        ((0.0, f64::INFINITY), f64::INFINITY),
    ];
    for ((mean, std), expected) in cases {
        let n = Normal::new(mean, std);
        assert_almost_eq!(n.entropy().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn normal_mode() {
    let cases = [
        ((-0.0, 1.0), 0.0),
        ((0.0, 1.0), 0.0),
        ((0.1, 1.0), 0.1),
        ((1.0, 1.0), 1.0),
        ((-10.0, 1.0), -10.0),
        ((f64::INFINITY, 1.0), f64::INFINITY),
    ];
    for ((mean, std), expected) in cases {
        let n = Normal::new(mean, std);
        assert_almost_eq!(n.mode().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn normal_median() {
    let cases = [
        ((-0.0, 1.0), 0.0),
        ((0.0, 1.0), 0.0),
        ((0.1, 1.0), 0.1),
        ((1.0, 1.0), 1.0),
        ((-0.0, 1.0), -0.0),
        ((f64::INFINITY, 1.0), f64::INFINITY),
    ];
    for ((mean, std), expected) in cases {
        let n = Normal::new(mean, std);
        assert_almost_eq!(n.median().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn normal_lower() {
    let cases = [
        ((0.0, 0.1), f64::NEG_INFINITY),
        ((-3.0, 10.0), f64::NEG_INFINITY),
    ];
    for ((mean, std), expected) in cases {
        assert_almost_eq!(Normal::new(mean, std).lower(), expected);
    }
}

#[test]
fn normal_upper() {
    let cases = [((0.0, 0.1), f64::INFINITY), ((-3.0, 10.0), f64::INFINITY)];
    for ((mean, std), expected) in cases {
        assert_almost_eq!(Normal::new(mean, std).upper(), expected);
    }
}

#[test]
fn normal_pdf() {
    let cases = [
        ((10.0, 0.1), 8.5, 5.530709549844416E-49),
        ((10.0, 0.1), 9.8, 0.5399096651318805),
        ((10.0, 0.1), 10.0, 3.989422804014327),
        ((10.0, 0.1), 10.2, 0.5399096651318805),
        ((10.0, 0.1), 11.5, 5.530709549844416E-49),
        ((-5.0, 1.0), -10.0, 1.4867195147342977E-6),
        ((-5.0, 1.0), -7.5, 0.017528300493568537),
        ((-5.0, 1.0), -5.0, 0.3989422804014327),
        ((-5.0, 1.0), -2.5, 0.017528300493568537),
        ((-5.0, 1.0), 0.0, 1.4867195147342977E-6),
        ((0.0, 10.0), -5.0, 0.035206532676429945),
        ((0.0, 10.0), -2.5, 0.03866681168028492),
        ((0.0, 10.0), 0.0, 0.03989422804014327),
        ((0.0, 10.0), 2.5, 0.03866681168028492),
        ((0.0, 10.0), 5.0, 0.035206532676429945),
        ((10.0, 100.0), -200.0, 4.3983595980427194E-4),
        ((10.0, 100.0), -100.0, 0.0021785217703255053),
        ((10.0, 100.0), 0.0, 0.003969525474770118),
        ((10.0, 100.0), 100.0, 0.0026608524989875483),
        ((10.0, 100.0), 200.0, 6.561581477467659E-4),
        ((-5.0, f64::INFINITY), -5.0, 0.0),
        ((-5.0, f64::INFINITY), 0.0, 0.0),
        ((-5.0, f64::INFINITY), 100.0, 0.0),
    ];
    for ((mean, std), x, expected) in cases {
        let n = Normal::new(mean, std);
        assert_almost_eq!(n.pdf(x), expected, relative = 1e-12);
    }
}

#[test]
fn normal_ln_pdf() {
    let cases = [
        ((10.0, 0.1), 8.5, 5.530709549844416E-49_f64.ln()),
        ((10.0, 0.1), 9.8, 0.5399096651318805_f64.ln()),
        ((10.0, 0.1), 10.0, 3.989422804014327_f64.ln()),
        ((10.0, 0.1), 10.2, 0.5399096651318805_f64.ln()),
        ((10.0, 0.1), 11.5, 5.530709549844416E-49_f64.ln()),
        ((-5.0, 1.0), -10.0, 1.4867195147342977E-6_f64.ln()),
        ((-5.0, 1.0), -7.5, 0.017528300493568537_f64.ln()),
        ((-5.0, 1.0), -5.0, 0.3989422804014327_f64.ln()),
        ((-5.0, 1.0), -2.5, 0.017528300493568537_f64.ln()),
        ((-5.0, 1.0), 0.0, 1.4867195147342977E-6_f64.ln()),
        ((0.0, 10.0), -5.0, 0.035206532676429945_f64.ln()),
        ((0.0, 10.0), -2.5, 0.03866681168028492_f64.ln()),
        ((0.0, 10.0), 0.0, 0.03989422804014327_f64.ln()),
        ((0.0, 10.0), 2.5, 0.03866681168028492_f64.ln()),
        ((0.0, 10.0), 5.0, 0.035206532676429945_f64.ln()),
        ((10.0, 100.0), -200.0, 4.3983595980427194E-4_f64.ln()),
        ((10.0, 100.0), -100.0, 0.0021785217703255053_f64.ln()),
        ((10.0, 100.0), 0.0, 0.003969525474770118_f64.ln()),
        ((10.0, 100.0), 100.0, 0.0026608524989875483_f64.ln()),
        ((10.0, 100.0), 200.0, 6.561581477467659E-4_f64.ln()),
        ((-5.0, f64::INFINITY), -5.0, f64::NEG_INFINITY),
        ((-5.0, f64::INFINITY), 0.0, f64::NEG_INFINITY),
        ((-5.0, f64::INFINITY), 100.0, f64::NEG_INFINITY),
    ];
    for ((mean, std), x, expected) in cases {
        let n = Normal::new(mean, std);
        assert_almost_eq!(n.ln_pdf(x), expected, relative = 1e-12);
    }
}

#[test]
fn normal_cdf() {
    let cases = [
        ((5.0, 2.0), f64::NEG_INFINITY, 0.0),
        ((5.0, 2.0), -5.0, 0.0000002866515718),
        ((5.0, 2.0), -2.0, 0.0002326290790),
        ((5.0, 2.0), 0.0, 0.006209665325),
        ((5.0, 2.0), 4.0, 0.3085375387259869),
        ((5.0, 2.0), 5.0, 0.5),
        ((5.0, 2.0), 6.0, 0.6914624612740131),
        ((5.0, 2.0), 10.0, 0.993790334674),
    ];
    for ((mean, std), x, expected) in cases {
        let n = Normal::new(mean, std);
        assert_almost_eq!(n.cdf(x), expected, absolute = 1e-12);
    }
}

#[test]
fn normal_sf() {
    let cases = [
        ((5.0, 2.0), f64::NEG_INFINITY, 1.0),
        ((5.0, 2.0), -5.0, 0.9999997133484281),
        ((5.0, 2.0), -2.0, 0.9997673709209455),
        ((5.0, 2.0), 0.0, 0.9937903346744879),
        ((5.0, 2.0), 4.0, 0.6914624612740131),
        ((5.0, 2.0), 5.0, 0.5),
        ((5.0, 2.0), 6.0, 0.3085375387259869),
        ((5.0, 2.0), 10.0, 0.006209665325512148),
    ];
    for ((mean, std), x, expected) in cases {
        let n = Normal::new(mean, std);
        assert_almost_eq!(n.sf(x), expected, absolute = 1e-12);
    }
}

#[test]
fn normal_inverse_cdf() {
    let cases = [
        ((5.0, 2.0), 0.0, f64::NEG_INFINITY),
        ((5.0, 2.0), 0.0000002866515718791939, -5.0),
        ((5.0, 2.0), 0.00023262907903552504, -2.0),
        // TODO: non-zero results, diff = 1e-15
        // ((5.0, 2.0), 0.006209665325776135, -0.0),
        // ((5.0, 2.0), 0.006209665325776135, 0.0),
        ((5.0, 2.0), 0.3085375387259869, 4.0),
        ((5.0, 2.0), 0.5, 5.0),
        ((5.0, 2.0), 0.6914624612740131, 6.0),
        ((5.0, 2.0), 0.9937903346742238, 10.0),
        ((5.0, 2.0), 1.0, f64::INFINITY),
    ];
    for ((mean, std), p, expected) in cases {
        let n = Normal::new(mean, std);
        assert_almost_eq!(n.inverse_cdf(p), expected, relative = 1e-14);
    }
}

#[test]
fn normal_cdf_roundtrip() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Normal::new(f64_range(u, 0.1, 10.0)?, f64_range(u, 0.1, 10.0)?),
            f64_range(u, 1e-6, 0.9999)?,
            1e-13,
        )
    });
}

#[test]
fn normal_cdf_roundtrip_wide() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Normal::new(
                f64_range(u, 0.01, 100.0)?,
                f64_range(u, 0.01, 100.0)?,
            ),
            f64_range(u, 1e-4, 0.9999)?,
            1e-11,
        )
    });
}

#[test]
fn normal_cdf_roundtrip_small_std() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Normal::new(f64_range(u, 0.1, 1.0)?, f64_range(u, 1e-3, 0.1)?),
            f64_range(u, 1e-4, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn normal_cdf_roundtrip_tails() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Normal::new(f64_range(u, 0.1, 5.0)?, f64_range(u, 0.1, 5.0)?),
            f64_range(u, 0.9, 0.9999)?,
            1e-11,
        )
    });
}
