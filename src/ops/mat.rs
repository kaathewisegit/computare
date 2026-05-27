use num_traits::NumAssign;

use crate::matrix::Matrix;

pub unsafe fn mm_u<T, A, B, Dst>(a: &A, b: &B, dst: &mut Dst)
where
    T: NumAssign + Copy,
    A: Matrix<T> + ?Sized,
    B: Matrix<T> + ?Sized,
    Dst: Matrix<T> + ?Sized,
{
    let n = a.num_rows();
    let m = a.num_cols();
    let p = b.num_cols();

    debug_assert_eq!(m, b.num_rows());
    debug_assert_eq!(n, dst.num_rows());
    debug_assert_eq!(p, dst.num_cols());

    for i in 0..n {
        for j in 0..p {
            let mut acc = T::zero();
            for k in 0..m {
                acc += unsafe { *a.at_u(i, k) * *b.at_u(k, j) };
            }
            unsafe { *dst.at_mut_u(i, j) = acc };
        }
    }
}
