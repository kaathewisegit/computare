use computare_linalg::{
    matrix::Matrix, matrix::MatrixRef, ops::mm_u, vector::Vector,
};

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

#[test]
fn col_u_array() {
    let a = [[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];

    let col0 = unsafe { a.col_u(0) };
    assert_eq!(col0.length(), 3);
    assert_eq!(col0.stride(), 2);
    assert_eq!(*col0.at(0), 1.0);
    assert_eq!(*col0.at(1), 3.0);
    assert_eq!(*col0.at(2), 5.0);

    let col1 = unsafe { a.col_u(1) };
    assert_eq!(*col1.at(0), 2.0);
    assert_eq!(*col1.at(1), 4.0);
    assert_eq!(*col1.at(2), 6.0);
}

#[test]
fn col_mut_u_array() {
    let mut a = [[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];

    {
        let col0 = unsafe { a.col_mut_u(0) };
        assert_eq!(col0.length(), 3);
        assert_eq!(col0.stride(), 2);
        *col0.at_mut(0) = 10.0;
        *col0.at_mut(1) = 30.0;
        *col0.at_mut(2) = 50.0;
    }

    assert_eq!(a, [[10.0, 2.0], [30.0, 4.0], [50.0, 6.0]]);
}

#[test]
fn col_u_ref() {
    let a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let m = MatrixRef::from_slice(&a, 3, 2);

    let col0 = unsafe { m.col_u(0) };
    assert_eq!(col0.length(), 3);
    assert_eq!(col0.stride(), 2);
    assert_eq!(*col0.at(0), 1.0);
    assert_eq!(*col0.at(1), 3.0);
    assert_eq!(*col0.at(2), 5.0);

    let col1 = unsafe { m.col_u(1) };
    assert_eq!(col1.length(), 3);
    assert_eq!(col1.stride(), 2);
    assert_eq!(*col1.at(0), 2.0);
    assert_eq!(*col1.at(1), 4.0);
    assert_eq!(*col1.at(2), 6.0);
}

#[test]
fn col_mut_u_ref() {
    let mut a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let m = MatrixRef::from_slice_mut(&mut a, 3, 2);

    {
        let col1 = unsafe { m.col_mut_u(1) };
        *col1.at_mut(0) = 20.0;
        *col1.at_mut(1) = 40.0;
        *col1.at_mut(2) = 60.0;
    }

    assert_eq!(a, vec![1.0, 20.0, 3.0, 40.0, 5.0, 60.0]);
}

#[test]
fn col_mut_preserves_other_cols() {
    let mut a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let m = MatrixRef::from_slice_mut(&mut a, 2, 4);

    {
        let col1 = unsafe { m.col_mut_u(1) };
        *col1.at_mut(0) = 0.0;
        *col1.at_mut(1) = 0.0;
    }

    assert_eq!(a, vec![1.0, 0.0, 3.0, 4.0, 5.0, 0.0, 7.0, 8.0]);
}
