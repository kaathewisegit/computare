use rug::{Float, az::Az, ops::Pow};

use super::PREC;
use computare_core::tolerance::assert_almost_eq;
use computare_special::beta::{
    beta, inverse_regularized_beta, ln_beta, regularized_incomplete_beta,
};
use computare_testing::arbitrary::{Result, arbtest, f64_range};

fn rug_ln_beta(a: &Float, b: &Float) -> Float {
    let lga = a.clone().ln_gamma();
    let lgb = b.clone().ln_gamma();
    let ab = Float::with_val(PREC, a + b);
    let lgab = ab.ln_gamma();
    Float::with_val(PREC, Float::with_val(PREC, lga + lgb) - lgab)
}

fn rug_beta(a: &Float, b: &Float) -> Float {
    rug_ln_beta(a, b).exp()
}

fn rug_incomplete_beta(a: &Float, b: &Float, x: &Float) -> Float {
    let one = Float::with_val(PREC, 1);
    let ai = Float::with_val(PREC, &one / a);

    let mut t = Float::with_val(PREC, Float::with_val(PREC, &one - b) * x);
    let a_plus_1 = Float::with_val(PREC, a + &one);
    let t1 = Float::with_val(PREC, &t / &a_plus_1);

    let mut s = Float::with_val(PREC, 0);
    let mut n: u32 = 2;
    let threshold = Float::with_val(PREC, &ai * Float::with_val(PREC, 1e-80));
    loop {
        let nf = Float::with_val(PREC, n);
        let u = Float::with_val(
            PREC,
            Float::with_val(PREC, Float::with_val(PREC, &nf - b) * x) / &nf,
        );
        t = Float::with_val(PREC, &t * &u);
        let a_plus_n = Float::with_val(PREC, a + &nf);
        let v = Float::with_val(PREC, &t / &a_plus_n);
        s = Float::with_val(PREC, &s + &v);
        if n > 2000 {
            break;
        }
        if v.clone().abs() < threshold {
            break;
        }
        n += 1;
    }

    s = Float::with_val(PREC, Float::with_val(PREC, s + &t1) + &ai);

    // B_x(a,b) = s * x^a
    let x_a = Float::with_val(PREC, x).pow(a);
    Float::with_val(PREC, &s * &x_a)
}

fn rug_regularized_incomplete_beta(a: f64, b: f64, x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    if x == 1.0 {
        return 1.0;
    }
    let (a, b, x, flip) = if x > a / (a + b) {
        (b, a, 1.0 - x, true)
    } else {
        (a, b, x, false)
    };

    let ra = Float::with_val(PREC, a);
    let rb = Float::with_val(PREC, b);
    let rx = Float::with_val(PREC, x);

    let bx = rug_incomplete_beta(&ra, &rb, &rx);
    let bab = rug_beta(&ra, &rb);
    let mut r = Float::with_val(PREC, &bx / &bab).az::<f64>();
    if flip {
        r = 1.0 - r;
    }
    r
}

fn compare_beta(a: f64, b: f64, relative: f64) -> Result<()> {
    let rug_res =
        rug_beta(&Float::with_val(PREC, a), &Float::with_val(PREC, b))
            .az::<f64>();
    let my_res = beta(a, b);
    assert_almost_eq!(my_res, rug_res, relative = relative);
    Ok(())
}

fn compare_ln_beta(a: f64, b: f64, relative: f64) -> Result<()> {
    let rug_res =
        rug_ln_beta(&Float::with_val(PREC, a), &Float::with_val(PREC, b))
            .az::<f64>();
    let my_res = ln_beta(a, b);
    assert_almost_eq!(my_res, rug_res, relative = relative);
    Ok(())
}

fn compare_regularized_incomplete_beta(
    a: f64,
    b: f64,
    x: f64,
    relative: f64,
) -> Result<()> {
    let rug_res = rug_regularized_incomplete_beta(a, b, x);
    let my_res = regularized_incomplete_beta(a, b, x);
    assert_almost_eq!(my_res, rug_res, relative = relative);
    Ok(())
}

#[test]
fn beta_unit() {
    arbtest(|u| {
        compare_beta(f64_range(u, 0.1, 2.0)?, f64_range(u, 0.1, 2.0)?, 1e-14)
    });
}

#[test]
fn beta_2_10() {
    arbtest(|u| {
        compare_beta(f64_range(u, 2.0, 10.0)?, f64_range(u, 2.0, 10.0)?, 1e-14)
    });
}

#[test]
fn beta_10_30() {
    arbtest(|u| {
        compare_beta(f64_range(u, 10.0, 30.0)?, f64_range(u, 2.0, 10.0)?, 1e-13)
    });
}

#[test]
fn ln_beta_unit() {
    let prec = 2e-12;
    compare_ln_beta(0.8269127728025633, 1.2420184412806519, prec).unwrap();
    arbtest(|u| {
        compare_ln_beta(f64_range(u, 0.1, 2.0)?, f64_range(u, 0.1, 2.0)?, prec)
    });
}

#[test]
fn ln_beta_2_10() {
    arbtest(|u| {
        compare_ln_beta(
            f64_range(u, 2.0, 10.0)?,
            f64_range(u, 2.0, 10.0)?,
            1e-14,
        )
    });
}

#[test]
fn ln_beta_10_100() {
    arbtest(|u| {
        compare_ln_beta(
            f64_range(u, 10.0, 100.0)?,
            f64_range(u, 2.0, 10.0)?,
            1e-13,
        )
    });
}

#[test]
fn ln_beta_100_1000() {
    arbtest(|u| {
        compare_ln_beta(
            f64_range(u, 100.0, 1000.0)?,
            f64_range(u, 2.0, 10.0)?,
            1e-10,
        )
    });
}

#[test]
fn beta_known_values() {
    assert_almost_eq!(beta(1.0, 1.0), 1.0, relative = 1e-15);
    assert_almost_eq!(beta(2.0, 3.0), 1.0 / 12.0, relative = 1e-15);
    assert_almost_eq!(beta(5.0, 3.0), 1.0 / 105.0, relative = 1e-15);
    assert_almost_eq!(beta(0.5, 0.5), core::f64::consts::PI, relative = 1e-15);
}

#[test]
fn ln_beta_known_values() {
    assert_almost_eq!(ln_beta(1.0, 1.0), 0.0, relative = 1e-15);
    assert_almost_eq!(
        ln_beta(2.0, 3.0),
        (1.0f64 / 12.0).ln(),
        relative = 1e-14
    );
    assert_almost_eq!(
        ln_beta(0.5, 0.5),
        core::f64::consts::PI.ln(),
        relative = 1e-15
    );
}

#[test]
fn regularized_incomplete_beta_boundary() {
    assert_almost_eq!(regularized_incomplete_beta(1.0, 1.0, 0.0), 0.0);
    assert_almost_eq!(regularized_incomplete_beta(1.0, 1.0, 1.0), 1.0);
    assert_almost_eq!(regularized_incomplete_beta(2.0, 3.0, 0.0), 0.0);
    assert_almost_eq!(regularized_incomplete_beta(2.0, 3.0, 1.0), 1.0);
    assert_almost_eq!(regularized_incomplete_beta(0.5, 0.5, 0.0), 0.0);
    assert_almost_eq!(regularized_incomplete_beta(0.5, 0.5, 1.0), 1.0);
}

#[test]
fn regularized_incomplete_beta_domain_errors() {
    assert!(regularized_incomplete_beta(-1.0, 2.0, 0.5).is_nan());
    assert!(regularized_incomplete_beta(2.0, -1.0, 0.5).is_nan());
    assert!(regularized_incomplete_beta(2.0, 3.0, -0.1).is_nan());
    assert!(regularized_incomplete_beta(2.0, 3.0, 1.1).is_nan());
}

#[test]
fn regularized_incomplete_beta_small() {
    arbtest(|u| {
        compare_regularized_incomplete_beta(
            f64_range(u, 0.1, 2.0)?,
            f64_range(u, 0.1, 2.0)?,
            f64_range(u, 0.01, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn regularized_incomplete_beta_moderate() {
    arbtest(|u| {
        compare_regularized_incomplete_beta(
            f64_range(u, 2.0, 10.0)?,
            f64_range(u, 2.0, 10.0)?,
            f64_range(u, 0.01, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn regularized_incomplete_beta_large() {
    arbtest(|u| {
        compare_regularized_incomplete_beta(
            f64_range(u, 10.0, 50.0)?,
            f64_range(u, 10.0, 50.0)?,
            f64_range(u, 0.01, 0.99)?,
            1e-10,
        )
    });
}

#[test]
fn regularized_incomplete_beta_mixed() {
    arbtest(|u| {
        compare_regularized_incomplete_beta(
            f64_range(u, 0.1, 10.0)?,
            f64_range(u, 10.0, 50.0)?,
            f64_range(u, 0.01, 0.99)?,
            1e-11,
        )
    });
}

#[test]
fn regularized_incomplete_beta_near_tail() {
    arbtest(|u| {
        compare_regularized_incomplete_beta(
            f64_range(u, 2.0, 10.0)?,
            f64_range(u, 2.0, 10.0)?,
            f64_range(u, 0.99, 0.9999)?,
            1e-10,
        )
    });
}

#[test]
fn regularized_incomplete_beta_near_zero() {
    arbtest(|u| {
        compare_regularized_incomplete_beta(
            f64_range(u, 2.0, 10.0)?,
            f64_range(u, 2.0, 10.0)?,
            f64_range(u, 0.0001, 0.01)?,
            1e-10,
        )
    });
}

fn compare_inverse_regularized_beta(
    a: f64,
    b: f64,
    p: f64,
    relative: f64,
) -> Result<()> {
    let x = inverse_regularized_beta(a, b, p);
    if (x == 0.0 && p == 0.0) || (x == 1.0 && p == 1.0) {
        return Ok(());
    }
    let roundtrip = regularized_incomplete_beta(a, b, x);
    assert_almost_eq!(roundtrip, p, relative = relative);
    Ok(())
}

#[test]
fn inverse_regularized_beta_boundary() {
    assert_almost_eq!(inverse_regularized_beta(1.0, 1.0, 0.0), 0.0);
    assert_almost_eq!(inverse_regularized_beta(1.0, 1.0, 1.0), 1.0);
    assert_almost_eq!(inverse_regularized_beta(2.0, 3.0, 0.0), 0.0);
    assert_almost_eq!(inverse_regularized_beta(2.0, 3.0, 1.0), 1.0);
}

#[test]
fn inverse_regularized_beta_domain_errors() {
    assert!(inverse_regularized_beta(-1.0, 2.0, 0.5).is_nan());
    assert!(inverse_regularized_beta(2.0, -1.0, 0.5).is_nan());
    assert!(inverse_regularized_beta(2.0, 3.0, -0.1).is_nan());
    assert!(inverse_regularized_beta(2.0, 3.0, 1.1).is_nan());
}

#[test]
fn inverse_regularized_beta_small() {
    arbtest(|u| {
        compare_inverse_regularized_beta(
            f64_range(u, 0.1, 2.0)?,
            f64_range(u, 0.1, 2.0)?,
            f64_range(u, 1e-4, 0.99)?,
            1e-1, // TODO: fix
        )
    });
}

#[test]
fn inverse_regularized_beta_moderate() {
    arbtest(|u| {
        compare_inverse_regularized_beta(
            f64_range(u, 2.0, 10.0)?,
            f64_range(u, 2.0, 10.0)?,
            f64_range(u, 1e-6, 0.99)?,
            1e-12,
        )
    });
}

#[test]
fn inverse_regularized_beta_large() {
    arbtest(|u| {
        compare_inverse_regularized_beta(
            f64_range(u, 10.0, 50.0)?,
            f64_range(u, 10.0, 50.0)?,
            f64_range(u, 1e-4, 0.99)?,
            1e-10,
        )
    });
}

#[test]
fn inverse_regularized_beta_mixed() {
    arbtest(|u| {
        compare_inverse_regularized_beta(
            f64_range(u, 0.1, 10.0)?,
            f64_range(u, 10.0, 50.0)?,
            f64_range(u, 1e-4, 0.99)?,
            1e-10,
        )
    });
}

#[test]
fn inverse_regularized_beta_high_p() {
    arbtest(|u| {
        compare_inverse_regularized_beta(
            f64_range(u, 2.0, 20.0)?,
            f64_range(u, 2.0, 20.0)?,
            f64_range(u, 0.9, 0.9999)?,
            1e-10,
        )
    });
}
