use computare_core::tolerance::assert_almost_eq;
use computare_linalg::{StridedVectorRef, Vector, ops::vec_dot_u};
use computare_testing::arbitrary::arbtest;

use crate::generate_f64_vec_nonnan;

#[test]
fn basic() {
    arbtest(|u| {
        let len = u.int_in_range(0..=1000usize)?;
        let x: Vec<f64> = generate_f64_vec_nonnan(u, len)?;
        let y: Vec<f64> = generate_f64_vec_nonnan(u, len)?;

        let out = unsafe { vec_dot_u(&x[..], &y[..]) };

        let expected: f64 = x.iter().zip(y.iter()).map(|(x, y)| x * y).sum();
        assert_almost_eq!(expected, out);

        Ok(())
    })
    .size_min(2u32.pow(12));
}

#[test]
fn strided() {
    arbtest(|u| {
        let total_len = u.int_in_range(10..=1000usize)?;
        let stride = u.int_in_range(1..=9usize)?;
        let len = total_len / stride;

        let x: Vec<f64> = generate_f64_vec_nonnan(u, total_len)?;
        let x = StridedVectorRef::from_slice(&x, len, stride);
        let y: Vec<f64> = generate_f64_vec_nonnan(u, total_len)?;
        let y = StridedVectorRef::from_slice(&y, len, stride);

        let out = unsafe { vec_dot_u(x, y) };

        let expected: f64 = (0..len).map(|i| x.at(i) * y.at(i)).sum();
        assert_almost_eq!(expected, out);

        Ok(())
    })
    .size_min(2u32.pow(12));
}
