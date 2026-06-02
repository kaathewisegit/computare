use rug::{Float, az::Az};

use computare_core::tolerance::assert_almost_eq;
use computare_special::gamma::gamma;
use computare_testing::arbitrary::{Result, arbtest, f64_range, f64_unit};

fn compare(f: f64, relative: f64) -> Result<()> {
    let rug_gamma = Float::with_val_64(500, f).gamma().az::<f64>();
    let my_gamma = gamma(f);

    assert_almost_eq!(my_gamma, rug_gamma, relative = relative);
    Ok(())
}

#[test]
fn gamma_unit() {
    arbtest(|u| compare(f64_unit(u)?, 1e-15));
}

#[test]
fn gamma_1_5() {
    arbtest(|u| compare(f64_range(u, 1.0, 5.0)?, 1e-15));
}

#[test]
fn gamma_5_10() {
    arbtest(|u| compare(f64_range(u, 5.0, 10.0)?, 1e-15));
}

#[test]
fn gamma_10_33() {
    arbtest(|u| compare(f64_range(u, 5.0, 33.0)?, 1e-14));
}
