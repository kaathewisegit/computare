use linalg::ops::mm_u;

#[test]
fn basic() {
    let a = [[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];
    let b = [[0.0, 1.0], [2.0, 3.0]];

    let mut dst = [[0.0; 2]; 3];
    unsafe { mm_u(&a, &b, &mut dst) };

    let expected = [[4.0, 7.0], [8.0, 15.0], [12.0, 23.0]];
    assert_eq!(dst, expected);
}
