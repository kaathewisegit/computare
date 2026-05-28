use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub,
    SubAssign,
};

pub trait Zero {
    const ZERO: Self;
}
impl Zero for f32 {
    const ZERO: Self = 0.0;
}
impl Zero for f64 {
    const ZERO: Self = 0.0;
}

pub trait One {
    const ONE: Self;
}
impl One for f32 {
    const ONE: Self = 1.0;
}
impl One for f64 {
    const ONE: Self = 1.0;
}

pub trait NumOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
{
}
impl<T, Rhs, Output> NumOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
        + Rem<Rhs, Output = Output>
{
}

pub trait NumAssignOps<Rhs = Self>:
    AddAssign<Rhs>
    + SubAssign<Rhs>
    + MulAssign<Rhs>
    + DivAssign<Rhs>
    + RemAssign<Rhs>
{
}
impl<T, Rhs> NumAssignOps<Rhs> for T where
    T: AddAssign<Rhs>
        + SubAssign<Rhs>
        + MulAssign<Rhs>
        + DivAssign<Rhs>
        + RemAssign<Rhs>
{
}

pub trait Num: Copy + Zero + One + PartialEq + NumOps {}
impl<T> Num for T where T: Copy + Zero + One + PartialEq + NumOps {}

pub trait NumAssign: Num + NumAssignOps {}
impl<T> NumAssign for T where T: Num + NumAssignOps {}

pub trait Float: Num + PartialOrd + Neg<Output = Self> {
    const NAN: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;

    const EPSILON: Self;

    const MIN: Self;
    const MIN_POSITIVE: Self;
    const MAX: Self;

    const MIN_EXP: i32;
    const MAX_EXP: i32;

    const RADIX: Self;
    const MANTISSA_DIGITS: u32;

    fn is_nan(self) -> bool;

    fn abs(self) -> Self;

    fn sqrt(self) -> Self;

    fn powi(self, n: i32) -> Self;

    fn signum(self) -> Self;
}

macro_rules! impl_float {
    ($t:ty) => {
        impl Float for $t {
            const NAN: Self = <$t>::NAN;
            const INFINITY: Self = <$t>::INFINITY;
            const NEG_INFINITY: Self = <$t>::NEG_INFINITY;

            const EPSILON: Self = <$t>::EPSILON;

            const MIN: Self = <$t>::MIN;
            const MIN_POSITIVE: Self = <$t>::MIN_POSITIVE;
            const MAX: Self = <$t>::MAX;

            const MIN_EXP: i32 = <$t>::MIN_EXP;
            const MAX_EXP: i32 = <$t>::MAX_EXP;

            const RADIX: $t = 2.0;
            const MANTISSA_DIGITS: u32 = <$t>::MANTISSA_DIGITS;

            #[inline]
            fn is_nan(self) -> bool {
                <$t>::is_nan(self)
            }

            #[inline]
            fn abs(self) -> Self {
                <$t>::abs(self)
            }

            #[inline]
            fn sqrt(self) -> Self {
                <$t>::sqrt(self)
            }

            #[inline]
            fn powi(self, n: i32) -> Self {
                <$t>::powi(self, n)
            }

            #[inline]
            fn signum(self) -> Self {
                <$t>::signum(self)
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);
