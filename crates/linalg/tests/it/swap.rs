use computare_linalg::{StridedVectorRef, ops::swap_u};
use computare_testing::arbitrary::{Result, Unstructured, arbtest};

use crate::{assert_eq_f64_slices, generate_f64_vec};

#[test]
fn swap_slices() {
    arbtest(|u| {
        let len = u.int_in_range(0..=1000usize)?;
        let a_old: Vec<f64> = generate_f64_vec(u, len)?;
        let b_old: Vec<f64> = generate_f64_vec(u, len)?;

        let mut a_new = a_old.clone();
        let mut b_new = b_old.clone();

        unsafe { swap_u(&mut a_new[..], &mut b_new[..]) };

        assert_eq_f64_slices(&a_old, &b_new);
        assert_eq_f64_slices(&a_new, &b_old);

        Ok(())
    })
    .size_min(2u32.pow(12));
}

fn swap_array<const N: usize>(u: &mut Unstructured<'_>) -> Result<()> {
    let a_old = u.arbitrary::<[f64; N]>()?;
    let b_old = u.arbitrary::<[f64; N]>()?;

    let mut a_new = a_old;
    let mut b_new = b_old;

    unsafe { swap_u(&mut a_new[..], &mut b_new[..]) };

    assert_eq_f64_slices(&a_old, &b_new);
    assert_eq_f64_slices(&b_old, &a_new);

    Ok(())
}

#[test]
fn swap_array_low() {
    arbtest(|u| swap_array::<0>(u)).size_min(2u32.pow(10));
    arbtest(|u| swap_array::<1>(u)).size_min(2u32.pow(10));
    arbtest(|u| swap_array::<2>(u)).size_min(2u32.pow(10));
    arbtest(|u| swap_array::<3>(u)).size_min(2u32.pow(10));
}

#[test]
fn swap_array_10() {
    arbtest(|u| swap_array::<10>(u)).size_min(2u32.pow(10));
}

#[test]
fn swap_array_100() {
    arbtest(|u| swap_array::<100>(u)).size_min(2u32.pow(10));
}

#[test]
fn swap_strided() {
    arbtest(|u| {
        let len = u.int_in_range(11..=100usize)?;
        let stride = u.int_in_range(1..=10)?;
        let mut a_old = vec![0.0; len];
        let mut b_old = vec![0.0; len];

        for i in (0..len).step_by(stride) {
            a_old[i] = u.arbitrary()?;
            b_old[i] = u.arbitrary()?;
        }

        let mut a_new = a_old.clone();
        let mut b_new = b_old.clone();

        let strided_len = len.div_ceil(stride);
        let a_strided =
            StridedVectorRef::from_slice_mut(&mut a_new, strided_len, stride);
        let b_strided =
            StridedVectorRef::from_slice_mut(&mut b_new, strided_len, stride);

        unsafe { swap_u(a_strided, b_strided) };

        println!("a_new = {a_new:?}");
        println!("b_new = {b_new:?}");

        assert_eq_f64_slices(&a_old, &b_new);
        assert_eq_f64_slices(&a_new, &b_old);

        Ok(())
    })
    .size_min(2u32.pow(12));
}
