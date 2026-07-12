use core::num::NonZeroU32;

use computare_distributions::{Chi, Continuous, Statistics};
use computare_testing::arbitrary::{arbtest, f64_range};

use super::{assert_almost_eq, compare_cdf_roundtrip};

fn nz(n: u32) -> NonZeroU32 {
    NonZeroU32::new(n).unwrap()
}

#[test]
fn chi_mean() {
    let cases = [
        (1u32, 0.7978845608028654),
        (2, 1.2533141373155003),
        (5, 2.127692162140974),
        (336, 18.31666925443713),
    ];
    for (freedom, expected) in cases {
        let chi = Chi::new(nz(freedom));
        assert_almost_eq!(chi.mean().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn chi_large_dof_mean_not_nan() {
    for i in 1u32..2000 {
        let mean = Chi::new(nz(i)).mean().unwrap();
        assert!(!mean.is_nan(), "Chi mean for {i} dof was {mean}");
    }
}

#[test]
fn chi_variance() {
    let cases = [
        (1u32, 0.3633802276324187),
        (2, 0.4292036732051034),
        (3, 0.45352091052967464),
    ];
    for (freedom, expected) in cases {
        let chi = Chi::new(nz(freedom));
        assert_almost_eq!(chi.variance().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn chi_entropy() {
    let cases = [
        (1u32, 0.7257913526447274),
        (2, 0.9420342421707938),
        (3, 0.9961541981062056),
    ];
    for (freedom, expected) in cases {
        let chi = Chi::new(nz(freedom));
        assert_almost_eq!(chi.entropy().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn chi_mode() {
    let cases = [(1u32, 0.0), (2, 1.0), (3, core::f64::consts::SQRT_2)];
    for (freedom, expected) in cases {
        let chi = Chi::new(nz(freedom));
        assert_almost_eq!(chi.mode().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn chi_lower() {
    let cases = [(1u32, 0.0), (2, 0.0), (3, 0.0)];
    for (freedom, expected) in cases {
        assert_almost_eq!(Chi::new(nz(freedom)).lower(), expected);
    }
}

#[test]
fn chi_upper() {
    for freedom in 1..100 {
        assert_almost_eq!(Chi::new(nz(freedom)).upper(), f64::INFINITY);
    }
}

#[test]
fn chi_pdf() {
    let cases = [
        (1u32, 0.0, 0.0),
        (1, 0.1, 0.7939050949540235),
        (1, 1.0, 0.4839414490382867),
        (1, 5.5, 2.1539520085086552e-7),
        (1, f64::INFINITY, 0.0),
        (2, 0.0, 0.0),
        (2, 0.1, 0.09950124791926823),
        (2, 1.0, 0.6065306597126334),
        (2, 5.5, 1.4847681768496578e-6),
        (2, f64::INFINITY, 0.0),
        (170, 13.0, 0.5644678498668441),
    ];
    for (freedom, x, expected) in cases {
        let chi = Chi::new(nz(freedom));
        assert_almost_eq!(chi.pdf(x), expected, relative = 1e-12);
    }
}

#[test]
fn chi_neg_pdf() {
    let cases = [(1u32, -1.0, 0.0)];
    for (freedom, x, expected) in cases {
        assert_almost_eq!(Chi::new(nz(freedom)).pdf(x), expected);
    }
}

#[test]
fn chi_ln_pdf() {
    let cases = [
        (1u32, 0.0, f64::NEG_INFINITY),
        (1, 0.1, -0.23079135264472744),
        (1, 1.0, -0.7257913526447274),
        (1, 5.5, -15.350791352644727),
        (1, f64::INFINITY, f64::NEG_INFINITY),
        (2, 0.0, f64::NEG_INFINITY),
        (2, 0.1, -2.307585092994046),
        (2, 1.0, -0.5),
        (2, 5.5, -13.420251907761575),
        (2, f64::INFINITY, f64::NEG_INFINITY),
        (170, 13.0, -0.5718718503060052),
    ];
    for (freedom, x, expected) in cases {
        let chi = Chi::new(nz(freedom));
        assert_almost_eq!(chi.ln_pdf(x), expected, relative = 1e-12);
    }
}

#[test]
fn chi_neg_ln_pdf() {
    let cases = [(1u32, -1.0, f64::NEG_INFINITY)];
    for (freedom, x, expected) in cases {
        assert_almost_eq!(Chi::new(nz(freedom)).ln_pdf(x), expected);
    }
}

#[test]
fn chi_cdf() {
    let cases = [
        (1u32, 0.0, 0.0),
        (1, 0.1, 0.07965567455405796),
        (1, 1.0, 0.6826894921370859),
        (1, 5.5, 0.999999962020875),
        (1, f64::INFINITY, 1.0),
        (2, 0.0, 0.0),
        (2, 0.1, 0.004987520807317686),
        (2, f64::INFINITY, 1.0),
    ];
    for (freedom, x, expected) in cases {
        let chi = Chi::new(nz(freedom));
        assert_almost_eq!(chi.cdf(x), expected, relative = 1e-12);
    }
}

#[test]
fn chi_sf() {
    let cases = [
        (1u32, 0.0, 1.0),
        (1, 0.1, 0.920344325445942),
        (1, 1.0, 0.31731050786291404),
        (1, 5.5, 3.797912493177544e-8),
        (1, f64::INFINITY, 0.0),
        (2, 0.0, 1.0),
        (2, 0.1, 0.9950124791926823),
        (2, 1.0, 0.6065306597126333),
        (2, 5.5, 2.699578503363014e-7),
        (2, f64::INFINITY, 0.0),
    ];
    for (freedom, x, expected) in cases {
        let chi = Chi::new(nz(freedom));
        assert_almost_eq!(chi.sf(x), expected, relative = 1e-12);
    }
}

#[test]
fn chi_neg_cdf() {
    let cases = [(1u32, -1.0, 0.0)];
    for (freedom, x, expected) in cases {
        assert_almost_eq!(Chi::new(nz(freedom)).cdf(x), expected);
    }
}

#[test]
fn chi_neg_sf() {
    let cases = [(1u32, -1.0, 1.0)];
    for (freedom, x, expected) in cases {
        assert_almost_eq!(Chi::new(nz(freedom)).sf(x), expected);
    }
}

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
