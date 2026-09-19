use crate::vector::Vector;
use computare_core::{ConstZero, Num, NumAssign};

pub unsafe fn vec_hadamard_u<T, A, B, C>(a: &A, b: &B, c: &mut C)
where
    T: Copy + Num,
    A: Vector<Item = T> + ?Sized,
    B: Vector<Item = T> + ?Sized,
    C: Vector<Item = T> + ?Sized,
{
    debug_assert_eq!(a.length(), b.length());
    debug_assert_eq!(a.length(), c.length());

    for i in 0..a.length() {
        unsafe { *c.at_mut_u(i) = *a.at_u(i) * *b.at_u(i) }
    }
}

pub unsafe fn vec_dot_u<T, A, B>(a: &A, b: &B) -> T
where
    T: Copy + NumAssign + ConstZero,
    A: Vector<Item = T> + ?Sized,
    B: Vector<Item = T> + ?Sized,
{
    debug_assert_eq!(a.length(), b.length());

    let mut out = T::ZERO;

    for i in 0..a.length() {
        out += unsafe { *a.at_u(i) * *b.at_u(i) };
    }

    out
}
