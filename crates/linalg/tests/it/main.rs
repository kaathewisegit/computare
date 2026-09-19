// I use it to test dynamic versions of routines
#![allow(clippy::useless_vec)]

mod eucledian_norm;
mod givens_rotation;
mod matrix;
mod strided_vec_ref;
mod swap;
mod vec_dot;
mod vec_hadamard;
mod vec_max;
mod vector;

use computare_testing::arbitrary::{Result, Unstructured};

pub fn generate_f64_vec(u: &mut Unstructured, len: usize) -> Result<Vec<f64>> {
    let mut out = vec![0.0; len];
    for item in &mut out {
        *item = u.arbitrary()?;
    }
    Ok(out)
}

pub fn generate_f64_vec_nonnan(
    u: &mut Unstructured,
    len: usize,
) -> Result<Vec<f64>> {
    let mut out = vec![f64::NAN; len];
    for item in &mut out {
        while item.is_nan() {
            *item = u.arbitrary()?;
        }
    }
    Ok(out)
}

pub fn assert_eq_f64_slices(a: &[f64], b: &[f64]) {
    assert_eq!(a.len(), b.len());
    for i in 0..a.len() {
        assert_eq!(a[i].to_be_bytes(), b[i].to_be_bytes(), "index {i}");
    }
}
