use core::mem;

use crate::Vector;

/// Swap elements of `a` and `b`
///
/// # Safety
///
/// - The lengths of `a` and `b` must be identical.
pub unsafe fn swap_u<T, A, B>(a: &mut A, b: &mut B)
where
    A: Vector<T> + ?Sized,
    B: Vector<T> + ?Sized,
{
    for i in 0..a.length() {
        unsafe { mem::swap(a.at_mut_u(i), b.at_mut_u(i)) }
    }
}
