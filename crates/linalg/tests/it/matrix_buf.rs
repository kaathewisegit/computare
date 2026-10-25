use computare_linalg::{Matrix, RowMatrix};

#[test]
fn from_slice_basic() {
    let source = [1, 2, 3, 4, 5, 6];
    let matrix = RowMatrix::from_slice(&source, 2, 3);

    assert_eq!(matrix.num_rows(), 2);
    assert_eq!(matrix.num_cols(), 3);
    assert_eq!(matrix.as_slice(), &[1, 2, 3, 4, 5, 6]);
    unsafe {
        assert_eq!(matrix.row_u(0), &[1, 2, 3]);
        assert_eq!(matrix.row_u(1), &[4, 5, 6]);
    }
    assert_eq!(matrix.as_slice(), source);
}

#[test]
fn from_slice_empty() {
    let matrix = RowMatrix::<u8>::from_slice(&[], 0, 0);
    assert_eq!(matrix.num_rows(), 0);
    assert_eq!(matrix.num_cols(), 0);
    assert_eq!(matrix.as_slice(), &[]);
}

#[test]
fn from_slice_separate_allocations() {
    let source = [1, 2, 3, 4];
    let mut matrix = RowMatrix::from_slice(&source, 2, 2);
    unsafe { *matrix.at_mut_u(0, 0) = 10 };

    assert_eq!(source, [1, 2, 3, 4]);
    assert_eq!(matrix.as_slice(), &[10, 2, 3, 4]);
}

#[test]
fn zst() {
    let matrix = RowMatrix::from_slice(&[(); 4], 2, 2);
    assert_eq!(matrix.num_rows(), 2);
    assert_eq!(matrix.num_cols(), 2);

    for row in 0..matrix.num_rows() {
        for col in 0..matrix.num_cols() {
            assert_eq!(*matrix.at(row, col), ());
        }
    }
}

#[test]
fn zeroed() {
    let matrix = RowMatrix::<f64>::zeroed(3, 2);

    assert_eq!(matrix.num_rows(), 2);
    assert_eq!(matrix.num_cols(), 3);
    assert_eq!(matrix.num_elements(), 6);
    assert_eq!(matrix.len(), 6);
    matrix.for_each(|&el| assert_eq!(el, 0.0));
}

#[test]
fn zero_dimension() {
    for (num_cols, num_rows) in [(3, 0), (0, 3)] {
        let buf = RowMatrix::<u8>::zeroed(num_cols, num_rows);

        assert!(buf.is_empty());
        assert_eq!(buf.num_rows(), 0);
        assert_eq!(buf.num_cols(), 0);
        assert_eq!(buf.len(), 0);
    }
}

#[test]
fn clone_copies_values_and_shape() {
    let source = [1, 2, 3, 4, 5, 6];
    let matrix = RowMatrix::from_slice(&source, 2, 3);
    let cloned = matrix.clone();

    assert_eq!(cloned.num_rows(), 2);
    assert_eq!(cloned.num_cols(), 3);
    assert_eq!(cloned.as_slice(), matrix.as_slice());
    unsafe {
        assert_eq!(cloned.row_u(0), &[1, 2, 3]);
        assert_eq!(cloned.row_u(1), &[4, 5, 6]);
    }
}

#[test]
fn clone_is_deep() {
    let source = [1, 2, 3, 4];
    let mut matrix = RowMatrix::from_slice(&source, 2, 2);
    let mut cloned = matrix.clone();

    unsafe { *cloned.at_mut_u(0, 0) = 10 };

    assert_eq!(matrix.as_slice(), &[1, 2, 3, 4]);
    assert_eq!(cloned.as_slice(), &[10, 2, 3, 4]);
}

#[test]
fn clone_empty() {
    let matrix = RowMatrix::<u8>::from_slice(&[], 0, 0);
    let cloned = matrix.clone();

    assert!(cloned.is_empty());
    assert_eq!(cloned.num_rows(), 0);
    assert_eq!(cloned.num_cols(), 0);
    assert_eq!(cloned.as_slice(), &[]);
}
