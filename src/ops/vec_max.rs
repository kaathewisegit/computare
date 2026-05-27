use crate::{num::Float, vector::Vector};

pub fn vec_max_abs_idx<T, V>(v: &V) -> usize
where
    T: Float,
    V: Vector<T> + ?Sized,
{
    if v.length() == 0 {
        return 0;
    }

    // SAFETY: we checked above that the vector isn't empty
    let mut max_value = unsafe { *v.at_u(0) }.abs();
    let mut max_index = 0;

    for i in 1..v.length() {
        let value = unsafe { *v.at_u(i) }.abs();
        if value > max_value {
            max_value = value;
            max_index = i;
        }
    }
    max_index
}
