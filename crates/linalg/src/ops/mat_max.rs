use crate::{matrix::Matrix, num::Float};

pub fn mat_max_abs<T, M>(m: &M) -> T
where
    T: Float,
    M: Matrix<T> + ?Sized,
{
    let mut max_value = T::ZERO;

    m.for_each(|v| {
        if *v > max_value {
            max_value = *v;
        }
    });

    max_value
}
