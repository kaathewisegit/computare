pub use arbitrary::{Arbitrary, Result, Unstructured};

pub fn f64_unit(u: &mut Unstructured) -> Result<f64> {
    const STEP: f64 = 1.1102230246251565e-16; // 2^-53
    let value = u.arbitrary::<u64>()?;
    Ok((value >> 11) as f64 * STEP)
}

pub fn f64_range(u: &mut Unstructured, low: f64, high: f64) -> Result<f64> {
    let low_bits = low.to_bits();
    let high_bits = high.to_bits();

    let range = high_bits - low_bits;
    let random = low_bits + (u.arbitrary::<u64>()? % range);

    Ok(f64::from_bits(random))
}
