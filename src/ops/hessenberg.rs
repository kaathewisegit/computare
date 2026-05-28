#![allow(unused)]

use crate::{matrix::Matrix, num::Float, vector::Vector};

pub unsafe fn hessenberg_upper_u<T, M, VT>(
    m: &mut M,
    tau: &mut VT,
    low: usize,
    high: usize,
    scratch: &mut Vec<T>,
) where
    T: Float,
    M: Matrix<T> + ?Sized,
    VT: Vector<T> + ?Sized,
{
    let n = m.num_cols();
    if n <= 1 {
        return;
    }

    debug_assert!(m.is_square());
    debug_assert!(tau.length() == n - 1);
    debug_assert!(low < high);
    debug_assert!(high < n);

    for i in 1..(low - 1) {
        unsafe { *tau.at_mut_u(i) = T::ZERO };
    }
    for i in high..(n - 1) {
        // SAFETY: `i < n - 1 = tau.length()`
        unsafe { *tau.at_mut_u(i) = T::ZERO };
    }

    unsafe { unblocked(m, tau, low, high, scratch) };
}

unsafe fn unblocked<T, M, VT>(
    m: &mut M,
    tau: &mut VT,
    low: usize,
    high: usize,
    #[expect(clippy::ptr_arg)] scratch: &mut Vec<T>,
) where
    T: Float,
    M: Matrix<T> + ?Sized,
    VT: Vector<T> + ?Sized,
{
    for i in low..high {
        // get reflector

        // apply reflector on the right
        // and the left
    }
}
