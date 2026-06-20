use crate::num::NumAssign;

use crate::{matrix::Matrix, vector::Vector};

use super::dot_u;

pub unsafe fn apply_left<T, M, V, Dst>(m: &M, v: &V, dst: &mut Dst)
where
    T: Copy + NumAssign,
    M: Matrix<T> + ?Sized,
    V: Vector<T> + ?Sized,
    Dst: Vector<T> + ?Sized,
{
    debug_assert!(m.is_square());
    debug_assert_eq!(m.num_cols(), v.length());
    debug_assert_eq!(v.length(), dst.length());

    let n = v.length();

    for i in 0..n {
        unsafe { *dst.at_mut_u(i) = dot_u(m.row_u(i), v) }
    }
}
