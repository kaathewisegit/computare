use rug::{Float, az::Az};

use computare_core::tolerance::assert_almost_eq;
use computare_special::gamma::{gamma, ln_gamma};
use computare_testing::arbitrary::{Result, arbtest, f64_range, f64_unit};

fn compare_gamma(f: f64, relative: f64) -> Result<()> {
    let rug_gamma = Float::with_val_64(500, f).gamma().az::<f64>();
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

fn compare_ln_gamma(f: f64, relative: f64) -> Result<()> {
    let rug_gamma = Float::with_val_64(500, f).ln_gamma().az::<f64>();
    let my_gamma = ln_gamma(f);

    assert_almost_eq!(my_gamma, rug_gamma, relative = relative);
    Ok(())
}

#[test]
fn ln_gamma_unit() {
    arbtest(|u| compare_ln_gamma(f64_unit(u)?, 2e-10));
}

#[test]
fn ln_gamma_1_2() {
    arbtest(|u| compare_ln_gamma(f64_range(u, 1.0, 5.0)?, 1e-11));
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
fn ln_gamma_10_35() {
    arbtest(|u| compare_ln_gamma(f64_range(u, 5.0, 33.0)?, 1e-16));
}

#[test]
fn ln_gamma_35_100() {
    arbtest(|u| compare_ln_gamma(f64_range(u, 5.0, 33.0)?, 1e-16));
}
