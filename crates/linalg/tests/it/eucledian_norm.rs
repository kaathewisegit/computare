use computare_core::assert_almost_eq;

use computare_linalg::ops::eucledian_norm;

// TODO: arbitrary tests (needs a precise baseline impl to compare to)

#[test]
fn stable() {
    assert_almost_eq!(eucledian_norm(&[3.0, 4.0]), 5.0);
    assert_almost_eq!(eucledian_norm(&[1.0, 1.0, 1.0]), 3f64.sqrt());
    assert_almost_eq!(eucledian_norm(&[0.0, 0.0, 0.0, 0.0]), 0.0);
}

#[test]
fn scaling_uniform() {
    fn check_uniform(value: f64) {
        for i in 2..20 {
            let values = vec![value; i];
            let norm = eucledian_norm(&values[..]);
            assert_almost_eq!(norm, value * (i as f64).sqrt());
        }
    }

    check_uniform(1e300);
    check_uniform(1e-300);
    check_uniform(5e-324); // subnormal
}

#[test]
fn scaling_varied() {
    assert_almost_eq!(eucledian_norm(&[1e200, 1.0]), 1e200);
    assert_almost_eq!(eucledian_norm(&[1.0, 1e200]), 1e200);
}

#[test]
fn many() {
    let values = vec![1e-200; 10_000];
    assert_almost_eq!(eucledian_norm(&values[..]), 1e-198);
}

#[test]
fn inf() {
    assert!(eucledian_norm(&[1.0, f64::INFINITY]).is_infinite());
    assert!(eucledian_norm(&[1.0, f64::NEG_INFINITY]).is_infinite());
}

#[test]
fn nan() {
    assert!(eucledian_norm(&[f64::NEG_INFINITY, f64::NAN]).is_nan());
    assert!(eucledian_norm(&[f64::NAN, f64::NAN]).is_nan());
    assert!(eucledian_norm(&[1.0, f64::NAN]).is_nan());
}
