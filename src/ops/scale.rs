use crate::{num::NumAssign, vector::Vector};

pub fn scale_vec<T, V>(v: &mut V, scale: T)
where
    T: NumAssign,
    V: Vector<T> + ?Sized,
{
    for i in 0..v.length() {
        unsafe { *v.at_mut_u(i) *= scale }
    }
}
