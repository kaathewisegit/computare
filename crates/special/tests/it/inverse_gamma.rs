use computare_core::tolerance::assert_almost_eq;
use computare_special::gamma::{
    inverse_lower_gamma, inverse_upper_gamma, regularized_lower_gamma,
    regularized_upper_gamma,
};
use computare_testing::arbitrary::{Result, arbtest, f64_range};

fn compare_inverse_lower_gamma(a: f64, p: f64, relative: f64) -> Result<()> {
    let x = inverse_lower_gamma(a, p);
    if x == 0.0 && p == 0.0 {
        return Ok(());
    }
    let roundtrip = regularized_lower_gamma(a, x);
    assert_almost_eq!(roundtrip, p, relative = relative);
    Ok(())
}

fn compare_inverse_upper_gamma(a: f64, q: f64, relative: f64) -> Result<()> {
    let x = inverse_upper_gamma(a, q);
    if x.is_infinite() && q == 0.0 {
        return Ok(());
    }
    let roundtrip = regularized_upper_gamma(a, x);
    assert_almost_eq!(roundtrip, q, relative = relative);
    Ok(())
}

#[test]
fn inverse_gamma_small_a() {
    arbtest(|u| {
        compare_inverse_lower_gamma(
            f64_range(u, 0.1, 1.0)?,
            f64_range(u, 1e-4, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn inverse_gamma_a_1_10() {
    arbtest(|u| {
        compare_inverse_lower_gamma(
            f64_range(u, 1.0, 10.0)?,
            f64_range(u, 1e-6, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn inverse_gamma_a_10_100() {
    arbtest(|u| {
        compare_inverse_lower_gamma(
            f64_range(u, 10.0, 100.0)?,
            f64_range(u, 1e-4, 0.99)?,
            1e-11,
        )
    });
}

#[test]
fn inverse_gamma_high_p() {
    arbtest(|u| {
        compare_inverse_lower_gamma(
            f64_range(u, 1.0, 20.0)?,
            f64_range(u, 0.9, 0.9999)?,
            1e-10,
        )
    });
}

#[test]
fn inverse_gamma_comp_small_a() {
    arbtest(|u| {
        compare_inverse_upper_gamma(
            f64_range(u, 0.01, 1.0)?,
            f64_range(u, 1e-6, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn inverse_gamma_comp_a_1_10() {
    arbtest(|u| {
        compare_inverse_upper_gamma(
            f64_range(u, 1.0, 10.0)?,
            f64_range(u, 1e-6, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn inverse_gamma_comp_a_10_100() {
    arbtest(|u| {
        compare_inverse_upper_gamma(
            f64_range(u, 10.0, 100.0)?,
            f64_range(u, 1e-4, 0.99)?,
            1e-11,
        )
    });
}

#[test]
fn inverse_gamma_comp_high_q() {
    arbtest(|u| {
        compare_inverse_upper_gamma(
            f64_range(u, 1.0, 20.0)?,
            f64_range(u, 0.9, 0.9999)?,
            1e-10,
        )
    });
}
