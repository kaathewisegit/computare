use computare_testing::arbitrary::arbtest;

use std::cmp::Ordering;

use computare_linalg::ops::vec_max_abs_idx;

#[test]
fn max_abs_idx_basic() {
    arbtest(|u| {
        let values: Vec<f64> = u.arbitrary()?;

        let max_index = values
            .iter()
            .map(|v| v.abs())
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                a.partial_cmp(b).unwrap_or(Ordering::Greater)
            })
            .map(|(index, _)| index)
            .unwrap_or(0);

        assert_eq!(max_index, vec_max_abs_idx(&values[..]), "{values:?}");
        Ok(())
    })
    .size_min(2u32.pow(10));
}
