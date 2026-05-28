use arbtest::arbtest;

use linalg::vector::{StridedVectorRef, Vector};

#[test]
fn basic() {
    arbtest(|u| {
        let len: usize = u.int_in_range(1..=100)?;
        let stride: usize = u.int_in_range(1..=10)?;
        let buf_size = len * stride;
        let data = (0..buf_size)
            .map(|_| u.arbitrary())
            .collect::<Result<Vec<i32>, _>>()?;

        let sv = StridedVectorRef::from_slice(&data, len, stride);
        assert_eq!(sv.length(), len);
        assert_eq!(sv.stride(), stride);

        for i in 0..len {
            assert_eq!(sv.at(i), &data[i * stride]);
        }

        Ok(())
    });
}

#[test]
fn strided_read_write_roundtrip() {
    arbtest(|u| {
        let len: usize = u.int_in_range(1..=100)?;
        let stride: usize = u.int_in_range(1..=10)?;
        let buf_size = (len - 1) * stride + 1;
        let mut data: Vec<i32> = vec![0; buf_size];
        let values: Vec<i32> = u.arbitrary()?;

        {
            let sv = StridedVectorRef::from_slice_mut(&mut data, len, stride);
            for (i, &v) in values.iter().enumerate().take(len) {
                *sv.at_mut(i) = v;
            }
            for (i, &v) in values.iter().enumerate().take(len) {
                assert_eq!(sv.at(i), &v);
            }
        }

        for i in 0..len.min(values.len()) {
            assert_eq!(data[i * stride], values[i]);
        }

        Ok(())
    });
}

#[test]
fn contiguous_read() {
    let data = [10_i32, 20, 30, 40, 50];
    let sv = StridedVectorRef::from_slice(&data, 5, 1);
    assert_eq!(sv.at(0), &10);
    assert_eq!(sv.at(1), &20);
    assert_eq!(sv.at(2), &30);
    assert_eq!(sv.at(3), &40);
    assert_eq!(sv.at(4), &50);
}

#[test]
fn strided_read() {
    let data = [10_i32, 0, 20, 0, 30];
    let sv = StridedVectorRef::from_slice(&data, 3, 2);
    assert_eq!(sv.at(0), &10);
    assert_eq!(sv.at(1), &20);
    assert_eq!(sv.at(2), &30);
}

#[test]
fn mutable_write_contiguous() {
    let mut data = [0_i32; 4];
    {
        let sv = StridedVectorRef::from_slice_mut(&mut data, 4, 1);
        *sv.at_mut(0) = 10;
        *sv.at_mut(1) = 20;
        *sv.at_mut(2) = 30;
        *sv.at_mut(3) = 40;
    }
    assert_eq!(data, [10, 20, 30, 40]);
}

#[test]
fn mutable_write_strided() {
    let mut data = [0_i32; 6];
    {
        let sv = StridedVectorRef::from_slice_mut(&mut data, 3, 2);
        *sv.at_mut(0) = 100;
        *sv.at_mut(1) = 200;
        *sv.at_mut(2) = 300;
    }
    assert_eq!(data, [100, 0, 200, 0, 300, 0]);
}

#[test]
#[should_panic]
fn at_panics_out_of_bounds() {
    let data = [1_i32, 2, 3];
    let sv = StridedVectorRef::from_slice(&data, 3, 1);
    sv.at(3);
}

#[test]
fn single_element() {
    let data = [42_i32];
    let sv = StridedVectorRef::from_slice(&data, 1, 1);
    assert_eq!(sv.length(), 1);
    assert_eq!(sv.stride(), 1);
    assert_eq!(sv.at(0), &42);
}

#[test]
fn strided_mutable_read_after_write() {
    let mut data = [0_i32; 5];
    {
        let sv = StridedVectorRef::from_slice_mut(&mut data, 3, 2);
        *sv.at_mut(0) = 100;
        *sv.at_mut(1) = 200;
        *sv.at_mut(2) = 300;
        assert_eq!(sv.at(0), &100);
        assert_eq!(sv.at(1), &200);
        assert_eq!(sv.at(2), &300);
    }
    assert_eq!(data, [100, 0, 200, 0, 300]);
}

#[test]
fn stride_2_overlapping_columns() {
    let data = [1_i32, 2, 3, 4, 5, 6];
    let col0 = StridedVectorRef::from_slice(&data, 3, 2);
    let col1 = StridedVectorRef::from_slice(&data[1..], 3, 2);
    assert_eq!(col0.at(0), &1);
    assert_eq!(col0.at(1), &3);
    assert_eq!(col0.at(2), &5);
    assert_eq!(col1.at(0), &2);
    assert_eq!(col1.at(1), &4);
    assert_eq!(col1.at(2), &6);
}
