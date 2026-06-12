use rug::{Float, az::Az};

use super::PREC;
use computare_core::tolerance::assert_almost_eq;
use computare_special::gamma::digamma;
use computare_testing::arbitrary::{Result, arbtest, f64_range, f64_unit};

fn compare_digamma(x: f64, relative: f64) -> Result<()> {
    let rug_digamma = Float::with_val(PREC, x).digamma().az::<f64>();
    let my_digamma = digamma(x);

    assert_almost_eq!(my_digamma, rug_digamma, relative = relative);
    Ok(())
}

#[test]
fn digamma_unit() {
    arbtest(|u| compare_digamma(f64_unit(u)?, 5e-15));
}

#[test]
fn digamma_1_2() {
    arbtest(|u| compare_digamma(f64_range(u, 1.0, 2.0)?, 5e-8));
}

#[test]
fn digamma_2_10() {
    arbtest(|u| compare_digamma(f64_range(u, 2.0, 10.0)?, 1e-14));
}

#[test]
fn digamma_10_100() {
    arbtest(|u| compare_digamma(f64_range(u, 10.0, 100.0)?, 5e-15));
}

#[test]
fn digamma_100_1000() {
    arbtest(|u| compare_digamma(f64_range(u, 100.0, 1000.0)?, 5e-16));
}
