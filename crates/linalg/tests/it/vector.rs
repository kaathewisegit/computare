use core::hint::black_box as bb;

use computare_linalg::ops::dot_u;
use computare_linalg::vector::{StridedVectorRef, Vector};

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

#[test]
fn slice_array() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let s = a.slice(1, 4);
    assert_eq!(s.length(), 3);
    assert_eq!(s.at(0), &20);
    assert_eq!(s.at(1), &30);
    assert_eq!(s.at(2), &40);
}

#[test]
fn slice_mut_array() {
    let mut a: [i32; 4] = [0, 0, 0, 0];
    let s = a.slice_mut(1, 3);
    *s.at_mut(0) = 100;
    *s.at_mut(1) = 200;
    assert_eq!(a, [0, 100, 200, 0]);
}

#[test]
fn slice_strided_contiguous() {
    let data = [10_i32, 20, 30, 40, 50];
    let sv = StridedVectorRef::from_slice(&data, 5, 1);
    let s = sv.slice(1, 4);
    assert_eq!(s.length(), 3);
    assert_eq!(s.stride(), 1);
    assert_eq!(s.at(0), &20);
    assert_eq!(s.at(1), &30);
    assert_eq!(s.at(2), &40);
}

#[test]
fn slice_strided_preserves_stride() {
    let data = [10_i32, 0, 20, 0, 30, 0, 40, 0, 50];
    let sv = StridedVectorRef::from_slice(&data, 5, 2);
    let s = sv.slice(1, 4);
    assert_eq!(s.length(), 3);
    assert_eq!(s.stride(), 2);
    assert_eq!(s.at(0), &20);
    assert_eq!(s.at(1), &30);
    assert_eq!(s.at(2), &40);
}

#[test]
fn slice_mut_strided() {
    let mut data = [0_i32; 9];
    {
        let sv = StridedVectorRef::from_slice_mut(&mut data, 5, 2);
        let s = sv.slice_mut(1, 3);
        *s.at_mut(0) = 100;
        *s.at_mut(1) = 200;
    }
    assert_eq!(data, [0, 0, 100, 0, 200, 0, 0, 0, 0]);
}

#[test]
fn slice_strided_chain() {
    let data = [1_i32, 0, 2, 0, 3, 0, 4, 0, 5];
    let sv = StridedVectorRef::from_slice(&data, 5, 2);
    let s1 = sv.slice(0, 4);
    assert_eq!(s1.at(0), &1);
    let s2 = s1.slice(1, 3);
    assert_eq!(s2.length(), 2);
    assert_eq!(s2.stride(), 2);
    assert_eq!(s2.at(0), &2);
    assert_eq!(s2.at(1), &3);
}

#[test]
#[should_panic]
fn slice_panics_start_ge_end() {
    let data = [1_i32, 2, 3];
    let sv = StridedVectorRef::from_slice(&data, 3, 1);
    sv.slice(2, 2);
}

#[test]
#[should_panic]
fn slice_panics_end_out_of_bounds() {
    let data = [1_i32, 2, 3];
    let sv = StridedVectorRef::from_slice(&data, 3, 1);
    sv.slice(0, 3);
}
