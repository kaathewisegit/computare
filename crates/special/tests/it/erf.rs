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

fn inverse_erf_roundtrip(y: f64, relative: f64) -> Result<()> {
    let round = erf(inverse_erf(y));
    assert_almost_eq!(y, round, relative = relative);
    Ok(())
}

#[test]
fn inverse_erf_unit() {
    arbtest(|u| inverse_erf_roundtrip(f64_unit(u)?, 1e-15));
}

#[test]
fn inverse_erf_neg() {
    arbtest(|u| inverse_erf_roundtrip(-f64_unit(u)?, 1e-15));
}

fn inverse_erfc_roundtrip(y: f64, relative: f64) -> Result<()> {
    let round = erfc(inverse_erfc(y));
    assert_almost_eq!(y, round, relative = relative);
    Ok(())
}

#[test]
fn inverse_erfc_unit() {
    let relative = 1e-14;
    inverse_erfc_roundtrip(0.10398463424912519, relative).unwrap();
    arbtest(|u| inverse_erfc_roundtrip(f64_unit(u)?, relative));
}

#[test]
fn inverse_erfc_1_2() {
    arbtest(|u| inverse_erfc_roundtrip(f64_range(u, 1.0, 2.0)?, 2e-15));
}
