use rug::{Float, az::Az};

use super::PREC;
use computare_core::tolerance::assert_almost_eq;
use computare_special::beta::{beta, ln_beta};
use computare_testing::arbitrary::{Result, arbtest, f64_range};

fn compare_beta(a: f64, b: f64, relative: f64) -> Result<()> {
    let rug_res = {
        let ga = Float::with_val(PREC, a).gamma();
        let gb = Float::with_val(PREC, b).gamma();
        let gab = Float::with_val(
            PREC,
            Float::with_val(PREC, a) + Float::with_val(PREC, b),
        )
        .gamma();
        (ga * gb / gab).az::<f64>()
    };
    let my_res = beta(a, b);

    assert_almost_eq!(my_res, rug_res, relative = relative);
    Ok(())
}

fn compare_ln_beta(a: f64, b: f64, relative: f64) -> Result<()> {
    let rug_res = {
        let lga = Float::with_val(PREC, a).ln_gamma();
        let lgb = Float::with_val(PREC, b).ln_gamma();
        let lab = Float::with_val(
            PREC,
            Float::with_val(PREC, a) + Float::with_val(PREC, b),
        )
        .ln_gamma();
        (lga + lgb - lab).az::<f64>()
    };
    let my_res = ln_beta(a, b);

    assert_almost_eq!(my_res, rug_res, relative = relative);
    Ok(())
}

#[test]
fn beta_unit() {
    arbtest(|u| {
        compare_beta(f64_range(u, 0.1, 2.0)?, f64_range(u, 0.1, 2.0)?, 1e-14)
    });
}

#[test]
fn beta_2_10() {
    arbtest(|u| {
        compare_beta(f64_range(u, 2.0, 10.0)?, f64_range(u, 2.0, 10.0)?, 1e-14)
    });
}

#[test]
fn beta_10_30() {
    arbtest(|u| {
        compare_beta(f64_range(u, 10.0, 30.0)?, f64_range(u, 2.0, 10.0)?, 1e-13)
    });
}

#[test]
fn ln_beta_unit() {
    arbtest(|u| {
        compare_ln_beta(f64_range(u, 0.1, 2.0)?, f64_range(u, 0.1, 2.0)?, 1e-12)
    });
}

#[test]
fn ln_beta_2_10() {
    arbtest(|u| {
        compare_ln_beta(
            f64_range(u, 2.0, 10.0)?,
            f64_range(u, 2.0, 10.0)?,
            1e-14,
        )
    });
}

#[test]
fn ln_beta_10_100() {
    arbtest(|u| {
        compare_ln_beta(
            f64_range(u, 10.0, 100.0)?,
            f64_range(u, 2.0, 10.0)?,
            1e-13,
        )
    });
}

#[test]
fn ln_beta_100_1000() {
    arbtest(|u| {
        compare_ln_beta(
            f64_range(u, 100.0, 1000.0)?,
            f64_range(u, 2.0, 10.0)?,
            1e-10,
        )
    });
}

#[test]
fn beta_known_values() {
    assert_almost_eq!(beta(1.0, 1.0), 1.0, relative = 1e-15);
    assert_almost_eq!(beta(2.0, 3.0), 1.0 / 12.0, relative = 1e-15);
    assert_almost_eq!(beta(5.0, 3.0), 1.0 / 105.0, relative = 1e-15);
    assert_almost_eq!(beta(0.5, 0.5), core::f64::consts::PI, relative = 1e-15);
}

#[test]
fn ln_beta_known_values() {
    assert_almost_eq!(ln_beta(1.0, 1.0), 0.0, relative = 1e-15);
    assert_almost_eq!(
        ln_beta(2.0, 3.0),
        (1.0f64 / 12.0).ln(),
        relative = 1e-14
    );
    assert_almost_eq!(
        ln_beta(0.5, 0.5),
        core::f64::consts::PI.ln(),
        relative = 1e-15
    );
}
