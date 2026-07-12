use core::fmt::Debug;

use computare_core::tolerance::assert_almost_eq;
use computare_distributions::Continuous;
use computare_testing::arbitrary::Result;

mod beta;
mod chi;
mod exponential;
mod gamma;
mod inverse_gamma;
mod laplace;
mod log_normal;
mod normal;
#[cfg(feature = "rand")]
mod sample;
mod uniform;

#[track_caller]
fn compare_cdf_roundtrip(
    dist: &(impl Continuous + Debug),
    p: f64,
    relative: f64,
) -> Result<()> {
    if p == 0.0 || p == 1.0 {
        return Ok(());
    }
    let x = dist.inverse_cdf(p);
    if x == 0.0 && p == 0.0 {
        return Ok(());
    }
    if x.is_infinite() {
        return Ok(());
    }
    let roundtrip = dist.cdf(x);
    assert_almost_eq!(
        roundtrip,
        p,
        relative = relative;
        "d = {dist:?}, p = {p}, roundtrip = {roundtrip}"
    );
    Ok(())
}
