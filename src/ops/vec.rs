use core::ptr;
use num_traits::{Num, NumAssign};

use crate::vector::Vector;

pub fn swap_u<T, A, B>(a: &mut A, b: &mut B)
where
    A: Vector<T> + ?Sized,
    B: Vector<T> + ?Sized,
{
    let a_ptr = unsafe { a.at_mut_u(0) } as *mut T;
    let b_ptr = unsafe { b.at_mut_u(0) } as *mut T;

    unsafe { ptr::swap_nonoverlapping(a_ptr, b_ptr, a.length()) }
}

pub unsafe fn hadamard_u<T, A, B, C>(a: &A, b: &B, c: &mut C)
where
    T: Copy + Num,
    A: Vector<T> + ?Sized,
    B: Vector<T> + ?Sized,
    C: Vector<T> + ?Sized,
{
    debug_assert_eq!(a.length(), b.length());
    debug_assert_eq!(a.length(), c.length());

    for i in 0..a.length() {
        unsafe { *c.at_mut_u(i) = *a.at_u(i) * *b.at_u(i) }
    }
}

pub unsafe fn dot_u<T, A, B>(a: &A, b: &B) -> T
where
    T: Copy + NumAssign,
    A: Vector<T> + ?Sized,
    B: Vector<T> + ?Sized,
{
    debug_assert_eq!(a.length(), b.length());

    let mut out = T::zero();

    for i in 0..a.length() {
        out += unsafe { *a.at_u(i) * *b.at_u(i) };
    }

    out
}
