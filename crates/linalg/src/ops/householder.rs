#![allow(unused)]

use crate::{
    matrix::Matrix,
    ops::{eucledian_norm, scale_vec},
    vector::Vector,
};
use computare_core::{Float, FloatMath, NumAssign};

pub unsafe fn householder_reflector_u<T, V>(alpha: T, v: &mut V) -> T
where
    T: FloatMath + NumAssign,
    V: Vector<T> + ?Sized,
{
    if v.length() == 0 {
        return T::ZERO;
    }

    let norm = eucledian_norm(v);
    if norm == T::ZERO {
        return T::ZERO;
    }

    let beta = (alpha.powi(2) + norm.powi(2)).sqrt() * -alpha.signum();

    let tau = (beta - alpha) / beta;

    let scale = T::ONE / (alpha - beta);
    scale_vec(v, scale);

    beta
}

pub unsafe fn householder_apply_left<T, V, M>(c: &M, v: &V)
where
    V: Vector<T> + ?Sized,
    M: Matrix<T> + ?Sized,
{
}
