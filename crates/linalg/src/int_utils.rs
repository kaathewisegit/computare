/// Copied from [`std`][c]
///
/// [c]: https://doc.rust-lang.org/stable/src/core/num/int_macros.rs.html
pub const fn div_ceil(lhs: i32, rhs: i32) -> i32 {
    let d = lhs / rhs;
    let r = lhs % rhs;

    let correction = 1 + ((lhs ^ rhs) >> (i32::BITS - 1));
    if r != 0 { d + correction } else { d }
}

pub const fn div_floor(lhs: i32, rhs: i32) -> i32 {
    let d = lhs / rhs;
    let r = lhs % rhs;

    let correction = (lhs ^ rhs) >> (i32::BITS - 1);
    if r != 0 { d + correction } else { d }
}
