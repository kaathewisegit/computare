#![expect(clippy::undocumented_unsafe_blocks)]

use arbtest::arbtest;

use computare_buffer::RawBuffer;

#[test]
fn ptr() {
    let len: usize = 10;
    let mut buf = RawBuffer::<i32>::uninit(len);
    let start = buf.ptr().as_ptr();
    for i in 0..len {
        let expected = unsafe { start.add(i) };
        assert_eq!(unsafe { buf.get(i) }, expected);
        assert_eq!(unsafe { buf.get_mut(i) as *const i32 }, expected);
    }
    unsafe { buf.deallocate(len) };
}

#[test]
fn zeroed() {
    unsafe {
        let len: usize = 10;
        let mut buf = RawBuffer::<i32>::zeroed(len);
        assert_eq!(&*buf.get_raw_slice(0, len), &[0; 10]);
        buf.deallocate(len);
    }
}

#[test]
fn reallocate_smaller() {
    unsafe {
        let len_0: usize = 10;
        let mut buf = RawBuffer::<usize>::uninit(len_0);
        for i in 0..len_0 {
            *buf.get_mut(i) = i;
        }

        let len_1: usize = 5;
        buf.reallocate(len_0, len_1);
        assert_eq!(&*buf.get_raw_slice(0, len_1), &[0, 1, 2, 3, 4]);

        buf.deallocate(len_1);
    }
}

#[test]
fn reallocate_larger() {
    unsafe {
        let len_0: usize = 3;
        let mut buf = RawBuffer::<usize>::uninit(len_0);
        for i in 0..len_0 {
            *buf.get_mut(i) = i;
        }

        let len_1: usize = 5;
        buf.reallocate(len_0, len_1);
        for i in len_0..len_1 {
            *buf.get_mut(i) = 10 * i;
        }
        assert_eq!(&*buf.get_raw_slice(0, len_1), &[0, 1, 2, 30, 40]);

        buf.deallocate(len_1);
    }
}

#[test]
fn duplicate() {
    unsafe {
        let len: usize = 10;
        let mut buf = RawBuffer::<i32>::uninit(len);
        for i in 0..len {
            *buf.get_mut(i) = i as i32 * 2;
        }

        let mut dup = buf.duplicate(len);
        assert_eq!(
            &*dup.get_raw_slice(0, len),
            &[0, 2, 4, 6, 8, 10, 12, 14, 16, 18]
        );
        assert_ne!(buf.ptr(), dup.ptr());

        buf.deallocate(len);
        dup.deallocate(len);
    }
}

#[test]
fn duplicate_empty() {
    unsafe {
        let buf = RawBuffer::<i32>::zeroed(0);
        let dup = buf.duplicate(0);
        assert_eq!(&*dup.get_raw_slice(0, 0), &[]);
    }
}

#[test]
fn duplicate_zst() {
    unsafe {
        let len: usize = 5;
        let buf = RawBuffer::<(), 0>::uninit(len);
        let dup = buf.duplicate(len);
        assert_eq!(&*dup.get_raw_slice(0, len), &vec![(); len]);
    }
}

fn duplicate_aligned<const A: usize>() {
    arbtest(|u| unsafe {
        let vec = u.arbitrary::<Vec<i32>>()?;
        let mut raw = RawBuffer::<i32, A>::from_slice(&vec);
        let mut dup = raw.duplicate(vec.len());
        assert_eq!(&*dup.get_raw_slice(0, vec.len()), &vec);
        if !vec.is_empty() {
            raw.deallocate(vec.len());
            dup.deallocate(vec.len());
        }
        Ok(())
    });
}

#[test]
fn test_duplicate_aligned_0() {
    duplicate_aligned::<0>();
}
#[test]
fn test_duplicate_aligned_8() {
    duplicate_aligned::<8>();
}
#[test]
fn test_duplicate_aligned_16() {
    duplicate_aligned::<16>();
}

fn from_slice<const A: usize>() {
    arbtest(|u| unsafe {
        let vec = u.arbitrary::<Vec<i32>>()?;
        let mut raw = RawBuffer::<i32, A>::from_slice(&vec);
        assert_eq!(&*raw.get_raw_slice(0, vec.len()), &vec);
        if !vec.is_empty() {
            raw.deallocate(vec.len());
        }
        Ok(())
    });
}

#[test]
fn test_from_slice_0() {
    from_slice::<0>();
}
#[test]
fn test_from_slice_8() {
    from_slice::<8>();
}
#[test]
fn test_from_slice_16() {
    from_slice::<16>();
}

#[test]
fn zst() {
    fn check_zst(mut buf: RawBuffer<(), 0>, len: usize) {
        unsafe {
            assert_eq!(&*buf.get_raw_slice(0, len), &vec![(); len]);
            assert_eq!(
                &*buf.get_raw_slice(len / 2, len / 2),
                &vec![(); len / 2]
            );

            buf.reallocate(len, len * 2);
            assert_eq!(&*buf.get_raw_slice(0, len), &vec![(); len]);

            buf.reallocate(len * 2, len / 2);
            assert_eq!(&*buf.get_raw_slice(0, len / 2), &vec![(); len / 2]);

            buf.deallocate(len);
        }
    }

    let slice = &[(); 5];
    check_zst(RawBuffer::from_slice(slice), slice.len());
    check_zst(RawBuffer::uninit(6), 6);
    check_zst(RawBuffer::zeroed(7), 7);
}

// TODO: simulation testing

#[test]
fn repeat() {
    unsafe {
        let len = 5;
        let buf = RawBuffer::<i32, 0>::repeat(7, len);
        assert_eq!(&*buf.get_raw_slice(0, len), &[7; 5]);

        let empty = RawBuffer::<i32, 0>::repeat(7, 0);
        assert_eq!(&*empty.get_raw_slice(0, 0), &[] as &[i32]);
    }
}
