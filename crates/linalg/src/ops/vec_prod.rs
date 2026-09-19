use crate::vector::Vector;
use computare_core::{Float, Num};

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
    T: Float,
    A: Vector<Item = T> + ?Sized,
    B: Vector<Item = T> + ?Sized,
{
    debug_assert_eq!(a.length(), b.length());

    let mut out = T::ZERO;

    for i in 0..a.length() {
        let prod = unsafe { a.at_u(i).algebraic_mul(*b.at_u(i)) };
        out = out.algebraic_add(prod);
    }

    out
}
