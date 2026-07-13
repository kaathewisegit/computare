use computare_distributions::{Continuous, Gamma, Statistics};
use computare_testing::arbitrary::{arbtest, f64_range};

use super::{assert_almost_eq, compare_cdf_roundtrip};

#[test]
fn mean() {
    let cases = [
        ((1.0, 10.), 10.0),
        ((1.0, 1.0), 1.0),
        ((10.0, 0.1), 1.0),
        ((10.0, 1.0), 10.0),
    ];
    for ((shape, scale), expected) in cases {
        let d = Gamma::new(shape, scale);
        assert_almost_eq!(d.mean().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn variance() {
    let cases = [
        ((1.0, 10.0), 100.0),
        ((1.0, 1.0), 1.0),
        ((10.0, 0.1), 0.1),
        ((10.0, 1.0), 10.0),
    ];
    for ((shape, scale), expected) in cases {
        let d = Gamma::new(shape, scale);
        assert_almost_eq!(d.variance().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn entropy() {
    let cases = [
        ((1.0, 10.0), 3.3025850929940455),
        ((1.0, 1.0), 1.0),
        ((10.0, 0.1), 0.23346908548693396),
        ((10.0, 1.0), 2.53605417848098),
    ];
    for ((shape, scale), expected) in cases {
        let d = Gamma::new(shape, scale);
        assert_almost_eq!(d.entropy().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn mode() {
    let cases = [
        ((1.0, 10.0), 0.0),
        ((1.0, 1.0), 0.0),
        ((10.0, 0.1), 0.9),
        ((10.0, 1.0), 9.0),
    ];
    for ((shape, scale), expected) in cases {
        let d = Gamma::new(shape, scale);
        assert_almost_eq!(d.mode().unwrap(), expected, relative = 1e-12);
    }
}

#[test]
fn lower() {
    let cases = [
        ((1.0, 10.0), 0.0),
        ((1.0, 1.0), 0.0),
        ((10.0, 0.1), 0.0),
        ((10.0, 1.0), 0.0),
    ];
    for ((shape, scale), expected) in cases {
        assert_almost_eq!(Gamma::new(shape, scale).lower(), expected);
    }
}

#[test]
fn upper() {
    let cases = [
        ((1.0, 10.0), f64::INFINITY),
        ((1.0, 1.0), f64::INFINITY),
        ((10.0, 0.1), f64::INFINITY),
        ((10.0, 1.0), f64::INFINITY),
    ];
    for ((shape, scale), expected) in cases {
        assert_almost_eq!(Gamma::new(shape, scale).upper(), expected);
    }
}

#[test]
fn pdf() {
    let cases = [
        ((1.0, 10.0), 1.0, 0.09048374180359596),
        ((1.0, 10.0), 10.0, 0.036787944117144235),
        ((1.0, 1.0), 1.0, 0.36787944117144233),
        ((1.0, 1.0), 10.0, 0.000045399929762484854),
        ((10.0, 0.1), 1.0, 1.251100357211333),
        ((10.0, 0.1), 10.0, 1.0251532120868705e-30),
        ((10.0, 1.0), 1.0, 0.0000010137771196302974),
        ((10.0, 1.0), 10.0, 0.1251100357211333),
        ((1.0, 10.0), 0.0, 0.1),
    ];
    for ((shape, scale), x, expected) in cases {
        let d = Gamma::new(shape, scale);
        assert_almost_eq!(d.pdf(x), expected, relative = 1e-12);
    }
}

#[test]
fn ln_pdf() {
    let cases = [
        ((1.0, 10.0), 1.0, -2.4025850929940455),
        ((1.0, 10.0), 10.0, -3.3025850929940455),
        ((1.0, 1.0), 1.0, -1.0),
        ((1.0, 1.0), 10.0, -10.0),
        ((10.0, 0.1), 1.0, 0.22402344985898723),
        ((10.0, 0.1), 10.0, -69.0527107131946),
        ((10.0, 1.0), 1.0, -13.801827480081469),
        ((10.0, 1.0), 10.0, -2.0785616431350586),
        ((10.0, f64::INFINITY), f64::INFINITY, f64::NEG_INFINITY),
        ((1.0, 10.0), 0.0, 0.1f64.ln()),
    ];
    for ((shape, scale), x, expected) in cases {
        let d = Gamma::new(shape, scale);
        assert_almost_eq!(d.ln_pdf(x), expected, relative = 1e-12);
    }
}

#[test]
fn cdf() {
    let cases = [
        ((1.0, 10.0), 1.0, 0.09516258196404043),
        ((1.0, 10.0), 10.0, 0.6321205588285577),
        ((1.0, 1.0), 1.0, 0.6321205588285577),
        ((1.0, 1.0), 10.0, 0.9999546000702375),
        ((10.0, 0.1), 1.0, 0.5420702855281478),
        ((10.0, 0.1), 10.0, 0.999999999999999999999999),
        ((10.0, 1.0), 1.0, 0.00000011142547833872067),
        ((10.0, 1.0), 10.0, 0.5420702855281478),
        ((1.0, 0.1), 0.0, 0.0),
    ];
    for ((shape, rate), x, expected) in cases {
        let d = Gamma::new(shape, rate);
        assert_almost_eq!(d.cdf(x), expected, relative = 1e-12);
    }
}

#[test]
fn sf() {
    let cases = [
        ((1.0, 10.0), 1.0, 0.9048374180359595),
        ((1.0, 10.0), 10.0, 0.3678794411714419),
        ((1.0, 1.0), 1.0, 0.3678794411714419),
        ((1.0, 1.0), 10.0, 4.539992976249074e-5),
        ((10.0, 0.1), 1.0, 0.4579297144718528),
        ((10.0, 0.1), 10.0, 1.1253473960842808e-31),
        ((10.0, 1.0), 1.0, 0.9999998885745217),
        ((10.0, 1.0), 10.0, 0.4579297144718528),
        ((1.0, 10.0), 0.0, 1.0),
    ];
    for ((shape, rate), x, expected) in cases {
        let d = Gamma::new(shape, rate);
        assert_almost_eq!(d.sf(x), expected, relative = 1e-12);
    }
}

#[test]
fn cdf_inverse_identity() {
    let cases = [
        (1.0, 10.0),
        (1.0, 1.0),
        (10.0, 0.1),
        (10.0, 1.0),
        (100.0, 0.005),
    ];

    for (shape, rate) in cases {
        for n in -5..0 {
            let p = 10.0f64.powi(n);
            let d = Gamma::new(shape, rate);
            assert_almost_eq!(d.cdf(d.inverse_cdf(p)), p, relative = 1e-12);
        }
    }

    // https://github.com/statrs-dev/statrs/issues/200
    let d = Gamma::new(3.0, 0.5);
    let x = 20.5567;
    // TODO: poor precision
    assert_almost_eq!(d.inverse_cdf(d.cdf(x)), x, relative = 1e-3);
}

#[test]
fn cdf_roundtrip_small_shape() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Gamma::new(f64_range(u, 0.1, 1.0)?, f64_range(u, 0.1, 10.0)?),
            f64_range(u, 1e-4, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn cdf_roundtrip_shape_1_10() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Gamma::new(f64_range(u, 1.0, 10.0)?, f64_range(u, 0.1, 10.0)?),
            f64_range(u, 1e-6, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn cdf_roundtrip_shape_10_100() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Gamma::new(f64_range(u, 10.0, 100.0)?, f64_range(u, 0.1, 10.0)?),
            f64_range(u, 1e-4, 0.99)?,
            1e-11,
        )
    });
}

#[test]
fn cdf_roundtrip_high_p() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Gamma::new(f64_range(u, 1.0, 20.0)?, f64_range(u, 0.1, 10.0)?),
            f64_range(u, 0.9, 0.9999)?,
            1e-10,
        )
    });
}

#[test]
fn cdf_roundtrip_small_scale() {
    arbtest(|u| {
        compare_cdf_roundtrip(
            &Gamma::new(f64_range(u, 0.1, 10.0)?, f64_range(u, 1e-3, 0.1)?),
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
            &Gamma::new(shape, scale),
            f64_range(u, 0.01, 0.99)?,
            1e-11,
        )
    });
}
