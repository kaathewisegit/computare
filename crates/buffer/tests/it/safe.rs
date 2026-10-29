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
