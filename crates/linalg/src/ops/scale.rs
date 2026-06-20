use crate::vector::Vector;
use computare_core::NumAssignOps;

pub fn scale_vec<T, V>(v: &mut V, scale: T)
where
    T: NumAssignOps + Copy,
    V: Vector<T> + ?Sized,
{
    for i in 0..v.length() {
        unsafe { *v.at_mut_u(i) *= scale }
    }
}
