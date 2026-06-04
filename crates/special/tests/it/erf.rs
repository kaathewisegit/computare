use rug::{Float, az::Az};

use super::PREC;
use computare_core::tolerance::assert_almost_eq;
use computare_special::erf::{erf, erfc, inverse_erf, inverse_erfc};
use computare_testing::arbitrary::{Result, arbtest, f64_range, f64_unit};

fn compare_erf(f: f64, relative: f64) -> Result<()> {
    let rug_erf = Float::with_val(PREC, f).erf().az::<f64>();
    let my_erf = erf(f);

    assert_almost_eq!(my_erf, rug_erf, relative = relative);
    Ok(())
}

#[test]
fn erf_unit() {
    arbtest(|u| compare_erf(f64_unit(u)?, 1e-16));
}

#[test]
fn erf_1_5() {
    arbtest(|u| compare_erf(f64_range(u, 1.0, 5.0)?, 1e-16));
}

#[test]
fn erf_5_10() {
    arbtest(|u| compare_erf(f64_range(u, 5.0, 10.0)?, 1e-16));
}

#[test]
fn erf_neg() {
    arbtest(|u| compare_erf(-f64_unit(u)?, 1e-16));
}

fn compare_erfc(f: f64, relative: f64) -> Result<()> {
    let rug_erfc = Float::with_val(PREC, f).erfc().az::<f64>();
    let my_erfc = erfc(f);

    assert_almost_eq!(my_erfc, rug_erfc, relative = relative);
    Ok(())
}

#[test]
fn erfc_unit() {
    arbtest(|u| compare_erfc(f64_unit(u)?, 1e-16));
}

#[test]
fn erfc_1_5() {
    arbtest(|u| compare_erfc(f64_range(u, 1.0, 5.0)?, 1e-16));
}

#[test]
fn erfc_5_10() {
    arbtest(|u| compare_erfc(f64_range(u, 5.0, 10.0)?, 1e-16));
}

#[test]
fn erfc_neg() {
    arbtest(|u| compare_erfc(-f64_unit(u)?, 1e-16));
}

fn compare_inverse_erf(y: f64, relative: f64) -> Result<()> {
    let my_x = inverse_erf(y);
    let rug_roundtrip = Float::with_val(PREC, my_x).erf().az::<f64>();

    assert_almost_eq!(rug_roundtrip, y, relative = relative);
    Ok(())
}

#[test]
fn inverse_erf_unit() {
    arbtest(|u| compare_inverse_erf(f64_unit(u)?, 1e-15));
}

#[test]
fn inverse_erf_neg() {
    arbtest(|u| compare_inverse_erf(-f64_unit(u)?, 1e-15));
}

#[test]
fn inverse_erf_0_1() {
    arbtest(|u| compare_inverse_erf(f64_range(u, 0.0, 1.0)?, 1e-15));
}

fn compare_inverse_erf_neg(y: f64, relative: f64) -> Result<()> {
    let my_x = inverse_erf(-y);
    let rug_roundtrip = Float::with_val(PREC, my_x).erf().az::<f64>();

    assert_almost_eq!(rug_roundtrip, -y, relative = relative);
    Ok(())
}

#[test]
fn inverse_erf_neg1_0() {
    arbtest(|u| compare_inverse_erf_neg(f64_unit(u)?, 1e-15));
}

fn compare_inverse_erfc(y: f64, relative: f64) -> Result<()> {
    let my_x = inverse_erfc(y);
    let rug_roundtrip = Float::with_val(PREC, my_x).erfc().az::<f64>();

    assert_almost_eq!(rug_roundtrip, y, relative = relative);
    Ok(())
}

#[test]
fn inverse_erfc_unit() {
    arbtest(|u| compare_inverse_erfc(f64_unit(u)?, 2e-15));
}

#[test]
fn inverse_erfc_0_1() {
    arbtest(|u| compare_inverse_erfc(f64_range(u, 0.0, 1.0)?, 2e-15));
}

#[test]
fn inverse_erfc_1_2() {
    arbtest(|u| compare_inverse_erfc(f64_range(u, 1.0, 2.0)?, 1e-15));
}
