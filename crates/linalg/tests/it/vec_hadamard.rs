use computare_core::tolerance::assert_almost_eq;
use computare_linalg::{StridedVectorRef, Vector, ops::vec_hadamard_u};
use computare_testing::arbitrary::arbtest;

use crate::generate_f64_vec;

#[test]
fn basic() {
    arbtest(|u| {
        let len = u.int_in_range(0..=1000usize)?;
        let x: Vec<f64> = generate_f64_vec(u, len)?;
        let y: Vec<f64> = generate_f64_vec(u, len)?;
        let mut out = vec![0.0; len];

        unsafe { vec_hadamard_u(&x[..], &y[..], &mut out[..]) };

        for i in 0..len {
            assert_almost_eq!(x[i] * y[i], out[i]);
        }

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

        let x: Vec<f64> = generate_f64_vec(u, total_len)?;
        let x = StridedVectorRef::from_slice(&x, len, stride);
        let y: Vec<f64> = generate_f64_vec(u, total_len)?;
        let y = StridedVectorRef::from_slice(&y, len, stride);

        let mut out = vec![0.0; len];

        unsafe { vec_hadamard_u(x, y, &mut out[..]) };

        #[expect(clippy::needless_range_loop)]
        for i in 0..len {
            assert_almost_eq!(x.at(i) * y.at(i), out[i]);
        }

        Ok(())
    })
    .size_min(2u32.pow(12));
}
