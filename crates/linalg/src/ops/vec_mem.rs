use core::mem;

use crate::Vector;

/// Copy elements of `src` into `dst`
///
/// # Safety
///
/// - `src` and `dst` must have the same length.
pub unsafe fn vec_copy_u<T, A, B>(src: &A, dst: &mut B)
where
    T: Copy,
    A: Vector<Item = T> + ?Sized,
    B: Vector<Item = T> + ?Sized,
{
    debug_assert_eq!(src.length(), dst.length());
    for i in 0..src.length() {
        unsafe { *dst.at_mut(i) = *src.at_u(i) };
    }
}

/// Swap elements of `a` and `b`
///
/// # Safety
///
/// - `a` and `b` must have the same length.
pub unsafe fn vec_swap_u<T, A, B>(a: &mut A, b: &mut B)
where
    A: Vector<Item = T> + ?Sized,
    B: Vector<Item = T> + ?Sized,
{
    debug_assert_eq!(a.length(), b.length());
    for i in 0..a.length() {
        unsafe { mem::swap(a.at_mut_u(i), b.at_mut_u(i)) }
    }
}
