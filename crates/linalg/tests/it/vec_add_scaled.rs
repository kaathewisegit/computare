use computare_core::tolerance::assert_almost_eq;
use computare_linalg::{StridedVectorRef, Vector, ops::vec_add_scaled};
use computare_testing::arbitrary::arbtest;

use crate::generate_f64_vec_nonnan;

#[test]
fn basic() {
    arbtest(|u| {
        let len = u.int_in_range(0..=1000usize)?;
        let alpha: f64 = u.arbitrary()?;
        let x: Vec<f64> = generate_f64_vec_nonnan(u, len)?;
        let y_old: Vec<f64> = generate_f64_vec_nonnan(u, len)?;

        let mut y_new = y_old.clone();
        unsafe { vec_add_scaled(alpha, &x[..], &mut y_new[..]) };

        for i in 0..len {
            let expected = y_old[i] + alpha * x[i];
            assert_almost_eq!(expected, y_new[i]);
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

        let alpha: f64 = u.arbitrary()?;
        let x_data: Vec<f64> = generate_f64_vec_nonnan(u, total_len)?;
        let x = StridedVectorRef::from_slice(&x_data, len, stride);

        let y_old_data: Vec<f64> = generate_f64_vec_nonnan(u, total_len)?;
        let mut y_new_data = y_old_data.clone();

        let x_ref = StridedVectorRef::from_slice(&x_data, len, stride);
        let y_ref =
            StridedVectorRef::from_slice_mut(&mut y_new_data, len, stride);

        unsafe { vec_add_scaled(alpha, x_ref, y_ref) };

        let y_old = StridedVectorRef::from_slice(&y_old_data, len, stride);
        let y_new = StridedVectorRef::from_slice(&y_new_data, len, stride);

        for i in 0..len {
            let expected = y_old.at(i) + alpha * x.at(i);
            assert_almost_eq!(expected, y_new.at(i));
        }

        Ok(())
    })
    .size_min(2u32.pow(12));
}
