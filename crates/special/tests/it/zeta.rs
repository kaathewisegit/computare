use rug::{Float, az::Az};

use super::PREC;
use computare_core::tolerance::assert_almost_eq;
use computare_special::zeta::zeta;
use computare_testing::arbitrary::{Result, arbtest, f64_range};

fn compare_zeta(f: f64, relative: f64) -> Result<()> {
    let rug_res = Float::with_val(PREC, f).zeta().az::<f64>();
    let my_res = zeta(f, 1.0);

    assert_almost_eq!(my_res, rug_res, relative = relative);
    Ok(())
}

#[test]
fn zeta_1_10() {
    arbtest(|u| compare_zeta(f64_range(u, 1.0, 10.0)?, 2e-14));
}

#[test]
fn zeta_10_100() {
    arbtest(|u| compare_zeta(f64_range(u, 10.0, 100.0)?, 2e-14));
}

#[test]
fn zeta_100_1000() {
    arbtest(|u| compare_zeta(f64_range(u, 100.0, 1000.0)?, 2e-14));
}
