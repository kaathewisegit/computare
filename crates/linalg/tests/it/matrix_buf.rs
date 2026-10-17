use computare_linalg::{Matrix, RowMatrixBuf};

#[test]
fn from_slice_basic() {
    let source = [1, 2, 3, 4, 5, 6];
    let matrix = RowMatrixBuf::from_slice(&source, 2, 3);

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
    let matrix = RowMatrixBuf::<u8>::from_slice(&[], 0, 0);
    assert_eq!(matrix.num_rows(), 0);
    assert_eq!(matrix.num_cols(), 0);
    assert_eq!(matrix.as_slice(), &[]);
}

#[test]
fn from_slice_separate_allocations() {
    let source = [1, 2, 3, 4];
    let mut matrix = RowMatrixBuf::from_slice(&source, 2, 2);
    unsafe { *matrix.at_mut_u(0, 0) = 10 };

    assert_eq!(source, [1, 2, 3, 4]);
    assert_eq!(matrix.as_slice(), &[10, 2, 3, 4]);
}

#[test]
fn zst() {
    let matrix = RowMatrixBuf::from_slice(&[(); 4], 2, 2);
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
    let matrix = RowMatrixBuf::<f64>::zeroed(3, 2);

    assert_eq!(matrix.num_rows(), 2);
    assert_eq!(matrix.num_cols(), 3);
    assert_eq!(matrix.num_elements(), 6);
    assert_eq!(matrix.len(), 6);
    matrix.for_each(|&el| assert_eq!(el, 0.0));
}

#[test]
fn zero_dimension() {
    for (num_cols, num_rows) in [(3, 0), (0, 3)] {
        let buf = RowMatrixBuf::<u8>::zeroed(num_cols, num_rows);

        assert!(buf.is_empty());
        assert_eq!(buf.num_rows(), 0);
        assert_eq!(buf.num_cols(), 0);
        assert_eq!(buf.len(), 0);
    }
}
