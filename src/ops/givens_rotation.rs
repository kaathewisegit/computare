use crate::{
    num::{Float, NumAssign},
    vector::Vector,
};

pub unsafe fn givens_rotation_u<T, A, B>(a: &mut A, b: &mut B, c: T, s: T)
where
    T: Float + NumAssign,
    A: Vector<T> + ?Sized,
    B: Vector<T> + ?Sized,
{
    debug_assert_eq!(a.length(), b.length());

    for i in 0..a.length() {
        let value_a = unsafe { *a.at_u(i) };
        let value_b = unsafe { *b.at_u(i) };
        let new_a = c * value_a + s * value_b;
        let new_b = c * value_b - s * value_a;

        unsafe {
            *a.at_mut_u(i) = new_a;
            *b.at_mut_u(i) = new_b;
        }
    }
}

pub fn givens_rotation<T, A, B>(a: &mut A, b: &mut B, c: T, s: T)
where
    T: Float + NumAssign,
    A: Vector<T> + ?Sized,
    B: Vector<T> + ?Sized,
{
    assert_eq!(a.length(), b.length());
    unsafe { givens_rotation_u(a, b, c, s) };
}
