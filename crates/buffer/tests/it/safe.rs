use computare_buffer::Buffer;

#[test]
fn from_slice() {
    let buf = Buffer::<i32, 0>::from_slice(&[1, 2, 3]);
    assert_eq!(buf.len(), 3);
    assert_eq!(buf.as_slice(), &[1, 2, 3]);
    assert_eq!(buf[1], 2);
    assert_eq!(buf.get(3), None);
}

#[test]
fn edit() {
    let mut buf = Buffer::<i32>::from_slice(&[1, 2]);
    buf[0] = 3;
    assert_eq!(buf.as_slice(), [3, 2]);
}

#[test]
fn index_range() {
    let buf = Buffer::<i32, 0>::from_slice(&[1, 2, 3, 4]);
    assert_eq!(&buf[1..3], &[2, 3]);
    assert_eq!(&buf[0..4], &[1, 2, 3, 4]);
    assert_eq!(&buf[2..2], &[] as &[i32]);

    let mut buf = Buffer::<i32, 0>::from_slice(&[1, 2, 3, 4]);
    buf[1..3].copy_from_slice(&[5, 6]);
    assert_eq!(buf.as_slice(), &[1, 5, 6, 4]);
}

#[test]
#[should_panic]
fn index_range_out_of_bounds() {
    let buf = Buffer::<i32, 0>::from_slice(&[1, 2, 3]);
    let _ = &buf[1..4];
}

#[test]
#[should_panic]
#[expect(clippy::reversed_empty_ranges)]
fn index_range_reversed() {
    let buf = Buffer::<i32, 0>::from_slice(&[1, 2, 3]);
    let _ = &buf[2..1];
}

#[test]
fn index_range_to() {
    let buf = Buffer::<i32, 0>::from_slice(&[1, 2, 3, 4]);
    assert_eq!(&buf[..2], &[1, 2]);
    assert_eq!(&buf[..4], &[1, 2, 3, 4]);
    assert_eq!(&buf[..0], &[] as &[i32]);

    let mut buf = Buffer::<i32, 0>::from_slice(&[1, 2, 3, 4]);
    buf[..2].copy_from_slice(&[7, 8]);
    assert_eq!(buf.as_slice(), &[7, 8, 3, 4]);
}

#[test]
#[should_panic]
fn index_range_to_out_of_bounds() {
    let buf = Buffer::<i32, 0>::from_slice(&[1, 2, 3]);
    let _ = &buf[..4];
}

#[test]
fn zeroed() {
    let buf = Buffer::<i32, 0>::zeroed(4);
    assert_eq!(buf.len(), 4);
    assert_eq!(buf.as_slice(), &[0; 4]);

    let empty = Buffer::<i32, 0>::zeroed(0);
    assert!(empty.is_empty());
    assert_eq!(empty.as_slice(), &[] as &[i32]);
}

#[test]
fn clone() {
    let buf = Buffer::<i32, 8>::from_slice(&[1, 2, 3]);
    assert_eq!(buf.clone().as_slice(), [1, 2, 3]);
    let empty = Buffer::<i32, 8>::from_slice(&[]);
    assert!(empty.is_empty());
    assert_eq!(empty.clone().as_slice(), &[] as &[i32]);
}
