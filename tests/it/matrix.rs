use linalg::{matrix::MatrixRef, ops::mm_u};

#[test]
fn basic_array() {
    let a = [[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];
    let b = [[0.0, 1.0], [2.0, 3.0]];

    let mut dst = [[0.0; 2]; 3];
    unsafe { mm_u(&a, &b, &mut dst) };

    let expected = [[4.0, 7.0], [8.0, 15.0], [12.0, 23.0]];
    assert_eq!(dst, expected);
}

#[test]
fn basic_ref() {
    let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let b = vec![0.0, 1.0, 2.0, 3.0];

    let a_ref = unsafe { MatrixRef::from_raw_parts(a.as_ptr(), 3, 2) };
    let b_ref = unsafe { MatrixRef::from_raw_parts(b.as_ptr(), 2, 2) };

    let mut dst = vec![0.0; 6];
    let dst_ref =
        unsafe { MatrixRef::from_raw_parts_mut(dst.as_mut_ptr(), 3, 2) };
    unsafe { mm_u(a_ref, b_ref, dst_ref) };

    let expected = vec![4.0, 7.0, 8.0, 15.0, 12.0, 23.0];
    assert_eq!(dst, expected);
}
