use computare_core::Float;

use crate::{matrix::Matrix, vector::Vector};

use super::vec_dot_u;

pub unsafe fn apply_left<T, M, V, Dst>(m: &M, v: &V, dst: &mut Dst)
where
    T: Float,
    M: Matrix<Item = T> + ?Sized,
    V: Vector<Item = T> + ?Sized,
    Dst: Vector<Item = T> + ?Sized,
{
    debug_assert!(m.is_square());
    debug_assert_eq!(m.num_cols(), v.length());
    debug_assert_eq!(v.length(), dst.length());

    let n = v.length();

    for i in 0..n {
        unsafe { *dst.at_mut_u(i) = vec_dot_u(m.row_u(i), v) }
    }
}
