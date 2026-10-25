use computare_core::{ConstZero, NumAssign};

use crate::matrix::Matrix;

/// Multiply `a` and `b` and write the result into `dst`
///
/// `dst` is completely overwritten.
///
/// # Safety
///
/// If `a` is `n` by `m`, then `b` must be `m` by `p` and `dst` must be `n` by
/// `p`.
pub unsafe fn mat_mul_u<T, A, B, Dst>(a: &A, b: &B, dst: &mut Dst)
where
    T: NumAssign + ConstZero + Copy,
    A: Matrix<Item = T> + ?Sized,
    B: Matrix<Item = T> + ?Sized,
    Dst: Matrix<Item = T> + ?Sized,
{
    let n = a.num_rows();
    let m = a.num_cols();
    let p = b.num_cols();

    debug_assert_eq!(m, b.num_rows());
    debug_assert_eq!(n, dst.num_rows());
    debug_assert_eq!(p, dst.num_cols());

    for i in 0..n {
        for j in 0..p {
            let mut acc = T::ZERO;
            for k in 0..m {
                acc += unsafe { *a.at_u(i, k) * *b.at_u(k, j) };
            }
            unsafe { *dst.at_mut_u(i, j) = acc };
        }
    }
}

/// Safe version of [`mat_mul_u`]
pub fn mat_mul<T, A, B, Dst>(a: &A, b: &B, dst: &mut Dst)
where
    T: NumAssign + ConstZero + Copy,
    A: Matrix<Item = T> + ?Sized,
    B: Matrix<Item = T> + ?Sized,
    Dst: Matrix<Item = T> + ?Sized,
{
    assert_eq!(a.num_cols(), b.num_rows());
    assert_eq!(a.num_rows(), dst.num_rows());
    assert_eq!(b.num_cols(), dst.num_cols());

    // SAFETY: invariants checked above
    unsafe { mat_mul_u(a, b, dst) }
}
