use rug::{Float, az::Az};

use super::PREC;
use computare_core::tolerance::assert_almost_eq;
use computare_special::gamma::psi;
use computare_testing::arbitrary::{Result, arbtest, f64_range, f64_unit};

fn compare_psi(f: f64, relative: f64) -> Result<()> {
    let rug_psi = Float::with_val(PREC, f).digamma().az::<f64>();
    let my_psi = psi(f);

    assert_almost_eq!(my_psi, rug_psi, relative = relative);
    Ok(())
}

#[test]
fn psi_unit() {
    arbtest(|u| compare_psi(f64_unit(u)?, 5e-8));
}

#[test]
fn psi_1_2() {
    arbtest(|u| compare_psi(f64_range(u, 1.0, 2.0)?, 5e-8));
}

#[test]
fn psi_2_10() {
    arbtest(|u| compare_psi(f64_range(u, 2.0, 10.0)?, 5e-8));
}

#[test]
fn psi_10_100() {
    arbtest(|u| compare_psi(f64_range(u, 10.0, 100.0)?, 1e-12));
}

#[test]
fn psi_100_1000() {
    arbtest(|u| compare_psi(f64_range(u, 100.0, 1000.0)?, 1e-12));
}
