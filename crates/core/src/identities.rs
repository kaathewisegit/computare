use core::ops::{Add, Mul};

pub trait Zero: Sized + Add<Self, Output = Self> {
    fn zero() -> Self;

    fn is_zero(&self) -> bool;
}

pub trait ConstZero: Zero {
    const ZERO: Self;
}

macro_rules! impl_zero {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
            #[inline]
            fn zero() -> $t {
                $v
            }

            #[inline]
            fn is_zero(&self) -> bool {
                *self == $v
            }
        }

        impl ConstZero for $t {
            const ZERO: Self = $v;
        }
    };
}

impl_zero!(usize, 0);
impl_zero!(u8, 0);
impl_zero!(u16, 0);
impl_zero!(u32, 0);
impl_zero!(u64, 0);
impl_zero!(u128, 0);

impl_zero!(isize, 0);
impl_zero!(i8, 0);
impl_zero!(i16, 0);
impl_zero!(i32, 0);
impl_zero!(i64, 0);
impl_zero!(i128, 0);

impl_zero!(f32, 0.0);
impl_zero!(f64, 0.0);

pub trait One: Sized + Mul<Self, Output = Self> {
    fn one() -> Self;
}

pub trait ConstOne: One {
    const ONE: Self;
}

macro_rules! impl_one {
    ($t:ty, $v:expr) => {
        impl One for $t {
            #[inline]
            fn one() -> $t {
                $v
            }
        }

        impl ConstOne for $t {
            const ONE: Self = $v;
        }
    };
}

impl_one!(usize, 1);
impl_one!(u8, 1);
impl_one!(u16, 1);
impl_one!(u32, 1);
impl_one!(u64, 1);
impl_one!(u128, 1);

impl_one!(isize, 1);
impl_one!(i8, 1);
impl_one!(i16, 1);
impl_one!(i32, 1);
impl_one!(i64, 1);
impl_one!(i128, 1);

impl_one!(f32, 1.0);
impl_one!(f64, 1.0);
