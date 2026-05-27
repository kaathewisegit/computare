use core::hint::black_box as bb;

use linalg::ops::dot_u;

#[test]
fn dot_array() {
    let a = bb([1.0, 2.0, 3.0, 4.0]);
    let b = bb([5.0, 6.0, 7.0, 8.0]);

    let res = bb(unsafe { dot_u(&a, &b) });
    assert_eq!(res, 70.0)
}

#[test]
fn dot_slice() {
    let a = bb(vec![1.0, 2.0, 3.0, 4.0]);
    let b = bb(vec![5.0, 6.0, 7.0, 8.0]);

    let res = bb(unsafe { dot_u(&*a, &*b) });
    assert_eq!(res, 70.0)
}
