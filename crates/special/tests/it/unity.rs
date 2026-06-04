use rug::{Float, az::Az};

use super::PREC;
use computare_core::tolerance::assert_almost_eq;
use computare_special::unity::{cosm1, ln_gamma_1p, log1p, log1pmx};
use computare_testing::arbitrary::{Result, arbtest, f64_range, f64_unit};

fn compare_log1p(f: f64, relative: f64) -> Result<()> {
    let rug_res = Float::with_val(PREC, f).ln_1p().az::<f64>();
    let my_res = log1p(f);

    assert_almost_eq!(my_res, rug_res, relative = relative);
    Ok(())
}

#[test]
fn log1p_unit() {
    arbtest(|u| compare_log1p(f64_unit(u)?, 1e-15));
}

#[test]
fn log1p_0_half() {
    arbtest(|u| compare_log1p(f64_range(u, 0.0, 0.5)?, 1e-15));
}

#[test]
fn log1p_half_2() {
    arbtest(|u| compare_log1p(f64_range(u, 0.5, 2.0)?, 1e-14));
}

#[test]
fn log1p_2_10() {
    arbtest(|u| compare_log1p(f64_range(u, 2.0, 10.0)?, 1e-14));
}

fn compare_log1pmx(f: f64, relative: f64) -> Result<()> {
    let rug_res = (Float::with_val(PREC, f).ln_1p() - Float::with_val(PREC, f))
        .az::<f64>();
    let my_res = log1pmx(f);

    assert_almost_eq!(my_res, rug_res, relative = relative);
    Ok(())
}

#[test]
fn log1pmx_unit() {
    arbtest(|u| compare_log1pmx(f64_unit(u)?, 1e-14));
}

#[test]
fn log1pmx_0_half() {
    arbtest(|u| compare_log1pmx(f64_range(u, 0.0, 0.5)?, 1e-14));
}

#[test]
fn log1pmx_half_2() {
    arbtest(|u| compare_log1pmx(f64_range(u, 0.5, 2.0)?, 1e-13));
}

#[test]
fn log1pmx_2_10() {
    arbtest(|u| compare_log1pmx(f64_range(u, 2.0, 10.0)?, 1e-13));
}

fn compare_cosm1(f: f64, relative: f64) -> Result<()> {
    let rug_res =
        (Float::with_val(PREC, f).cos() - Float::with_val(PREC, 1)).az::<f64>();
    let my_res = cosm1(f);

    assert_almost_eq!(my_res, rug_res, relative = relative);
    Ok(())
}

#[test]
fn cosm1_unit() {
    arbtest(|u| compare_cosm1(f64_unit(u)?, 1e-15));
}

#[test]
fn cosm1_0_half() {
    arbtest(|u| compare_cosm1(f64_range(u, 0.0, 0.5)?, 1e-15));
}

#[test]
fn cosm1_half_2() {
    arbtest(|u| compare_cosm1(f64_range(u, 0.5, 2.0)?, 1e-14));
}

#[test]
fn cosm1_2_10() {
    arbtest(|u| compare_cosm1(f64_range(u, 2.0, 10.0)?, 1e-14));
}

fn compare_ln_gamma_1p(f: f64, relative: f64) -> Result<()> {
    let rug_res = Float::with_val(PREC, f + 1.0).ln_gamma().az::<f64>();
    let my_res = ln_gamma_1p(f);

    assert_almost_eq!(my_res, rug_res, relative = relative);
    Ok(())
}

#[test]
fn ln_gamma_1p_unit() {
    arbtest(|u| compare_ln_gamma_1p(f64_unit(u)?, 2e-10));
}

#[test]
fn ln_gamma_1p_0_half() {
    arbtest(|u| compare_ln_gamma_1p(f64_range(u, 0.0, 0.5)?, 2e-10));
}

#[test]
fn ln_gamma_1p_half_2() {
    arbtest(|u| compare_ln_gamma_1p(f64_range(u, 0.5, 2.0)?, 1e-13));
}

#[test]
fn ln_gamma_1p_2_10() {
    arbtest(|u| compare_ln_gamma_1p(f64_range(u, 2.0, 10.0)?, 1e-15));
}
