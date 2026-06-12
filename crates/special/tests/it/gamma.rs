use rug::{Float, az::Az};

use super::PREC;
use computare_core::tolerance::assert_almost_eq;
use computare_special::gamma::{
    gamma, ln_gamma, recip_gamma, regularized_lower_gamma,
    regularized_upper_gamma,
};
use computare_testing::arbitrary::{Result, arbtest, f64_range, f64_unit};

fn compare_gamma(f: f64, relative: f64) -> Result<()> {
    let rug_gamma = Float::with_val(PREC, f).gamma().az::<f64>();
    let my_gamma = gamma(f);

    assert_almost_eq!(my_gamma, rug_gamma, relative = relative);
    Ok(())
}

#[test]
fn gamma_unit() {
    arbtest(|u| compare_gamma(f64_unit(u)?, 1e-15));
}

#[test]
fn gamma_1_5() {
    arbtest(|u| compare_gamma(f64_range(u, 1.0, 5.0)?, 1e-15));
}

#[test]
fn gamma_5_10() {
    arbtest(|u| compare_gamma(f64_range(u, 5.0, 10.0)?, 1e-15));
}

#[test]
fn gamma_10_33() {
    arbtest(|u| compare_gamma(f64_range(u, 5.0, 33.0)?, 1e-14));
}

fn compare_ln_gamma(x: f64, relative: f64) -> Result<()> {
    let rug_gamma = Float::with_val(PREC, x).ln_gamma().az::<f64>();
    let my_gamma = ln_gamma(x);

    assert_almost_eq!(my_gamma, rug_gamma, relative = relative);
    Ok(())
}

#[test]
fn ln_gamma_unit() {
    let prec = 1e-9;
    compare_ln_gamma(0.999997119981013, prec).unwrap();
    arbtest(|u| compare_ln_gamma(f64_unit(u)?, prec));
}

#[test]
fn ln_gamma_1_2() {
    // 0x8129fc7700010000
    arbtest(|u| compare_ln_gamma(f64_range(u, 1.0, 5.0)?, 1e-10));
}

#[test]
fn ln_gamma_2_5() {
    arbtest(|u| compare_ln_gamma(f64_range(u, 1.0, 5.0)?, 5e-11));
}

#[test]
fn ln_gamma_5_10() {
    arbtest(|u| compare_ln_gamma(f64_range(u, 5.0, 10.0)?, 1e-16));
}

#[test]
fn ln_gamma_10_33() {
    arbtest(|u| compare_ln_gamma(f64_range(u, 10.0, 33.0)?, 1e-16));
}

#[test]
fn ln_gamma_33_100() {
    arbtest(|u| compare_ln_gamma(f64_range(u, 33.0, 100.0)?, 1e-16));
}

fn rug_regularized_upper(a: f64, x: f64) -> Float {
    let rug_a = Float::with_val(PREC, a);
    let rug_x = Float::with_val(PREC, x);
    let upper_gamma = rug_a.clone().gamma_inc(&rug_x);
    let complete_gamma = rug_a.gamma();
    upper_gamma / complete_gamma
}

fn compare_regularized_lower(a: f64, x: f64, relative: f64) -> Result<()> {
    let rug_res =
        (Float::with_val(PREC, 1.0) - rug_regularized_upper(a, x)).az::<f64>();
    let my_res = regularized_lower_gamma(a, x);

    assert_almost_eq!(my_res, rug_res, relative = relative);
    Ok(())
}

#[test]
fn regularized_lower_gamma_unit() {
    arbtest(|u| compare_regularized_lower(f64_unit(u)?, f64_unit(u)?, 1e-14));
}

fn compare_regularized_upper(a: f64, x: f64, relative: f64) -> Result<()> {
    let rug_res = rug_regularized_upper(a, x).az::<f64>();
    let my_res = regularized_upper_gamma(a, x);

    assert_almost_eq!(my_res, rug_res, relative = relative);
    Ok(())
}

#[test]
fn regularized_upper_gamma_unit() {
    arbtest(|u| compare_regularized_upper(f64_unit(u)?, f64_unit(u)?, 1e-13));
}

fn compare_recip_gamma(f: f64, relative: f64) -> Result<()> {
    let rug_res = (Float::with_val(PREC, 1.0)
        / Float::with_val(PREC, f).gamma())
    .az::<f64>();
    let my_res = recip_gamma(f);

    assert_almost_eq!(my_res, rug_res, relative = relative);
    Ok(())
}

#[test]
fn recip_gamma_unit() {
    arbtest(|u| compare_recip_gamma(f64_unit(u)?, 1e-14));
}

#[test]
fn recip_gamma_1_5() {
    arbtest(|u| compare_recip_gamma(f64_range(u, 1.0, 5.0)?, 1e-14));
}

#[test]
fn recip_gamma_5_10() {
    arbtest(|u| compare_recip_gamma(f64_range(u, 5.0, 10.0)?, 1e-14));
}

#[test]
fn recip_gamma_neg4_0() {
    arbtest(|u| compare_recip_gamma(-f64_range(u, 0.0, 4.0)?, 1e-13));
}

#[test]
fn recip_gamma_neg10_neg4() {
    arbtest(|u| compare_recip_gamma(-f64_range(u, 4.0, 10.0)?, 1e-13));
}
