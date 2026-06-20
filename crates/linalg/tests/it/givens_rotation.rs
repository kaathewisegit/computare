use approx::assert_ulps_eq;

use std::f64::consts::PI;

use computare_linalg::ops::givens_rotation;

#[test]
fn basic() {
    let c = (PI / 4.0).cos();
    let s = c;

    let a = [1.0, 2.0, 3.0];
    let b = [4.0, 5.0, 6.0];

    let mut a_new = a;
    let mut b_new = b;

    givens_rotation(&mut a_new, &mut b_new, c, s);

    for i in 0..3 {
        assert_ulps_eq!(a_new[i], c * (a[i] + b[i]));
        assert_ulps_eq!(b_new[i], c * (-a[i] + b[i]));
    }
}
