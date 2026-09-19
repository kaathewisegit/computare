use crate::vector::Vector;
use computare_core::{Float, Num};

pub unsafe fn vec_hadamard_u<T, X, Y, Z>(x: &X, y: &Y, z: &mut Z)
where
    T: Copy + Num,
    X: Vector<Item = T> + ?Sized,
    Y: Vector<Item = T> + ?Sized,
    Z: Vector<Item = T> + ?Sized,
{
    debug_assert_eq!(x.length(), y.length());
    debug_assert_eq!(x.length(), z.length());

    for i in 0..x.length() {
        unsafe { *z.at_mut_u(i) = *x.at_u(i) * *y.at_u(i) }
    }
}

pub unsafe fn vec_dot_u<T, X, Y>(x: &X, y: &Y) -> T
where
    T: Float,
    X: Vector<Item = T> + ?Sized,
    Y: Vector<Item = T> + ?Sized,
{
    debug_assert_eq!(x.length(), y.length());

    let mut out = T::ZERO;

    for i in 0..x.length() {
        let prod = unsafe { x.at_u(i).algebraic_mul(*y.at_u(i)) };
        out = out.algebraic_add(prod);
    }

    out
}

/// Executes `y += a * x`
pub unsafe fn vec_add_scaled<T, X, Y>(a: T, x: &X, y: &mut Y)
where
    T: Float,
    X: Vector<Item = T> + ?Sized,
    Y: Vector<Item = T> + ?Sized,
{
    debug_assert_eq!(x.length(), y.length());

    for i in 0..x.length() {
        unsafe { *y.at_mut_u(i) = y.at_mut_u(i).algebraic_add(a * *x.at_u(i)) }
    }
}
