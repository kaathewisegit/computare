use std::ops::Neg;

use crate::{ConstOne, ConstZero, Num};

pub trait Float:
    Copy + PartialOrd + Num + ConstOne + ConstZero + Neg<Output = Self>
{
    const NAN: Self;
    const INFINITY: Self;
    const NEG_INFINITY: Self;

    const NEG_ZERO: Self;

    const EPSILON: Self;

    const MIN: Self;
    const MIN_POSITIVE: Self;
    const MAX: Self;

    const MIN_EXP: i32;
    const MAX_EXP: i32;

    const RADIX: Self;
    const MANTISSA_DIGITS: u32;

    fn is_nan(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_finite(self) -> bool;
    fn is_normal(self) -> bool;
    fn is_subnormal(self) -> bool;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn is_sign_positive(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
    fn recip(self) -> Self;
    fn powi(self, exp: i32) -> Self;
    fn copysign(self, sign: Self) -> Self;
}

macro_rules! impl_float {
    ($t:ty) => {
        impl Float for $t {
            const NAN: Self = <$t>::NAN;
            const INFINITY: Self = <$t>::INFINITY;
            const NEG_INFINITY: Self = <$t>::NEG_INFINITY;

            // f32::NEG_ZERO doesn't exist; -0.0 works for both types
            const NEG_ZERO: Self = -0.0;

            const EPSILON: Self = <$t>::EPSILON;

            const MIN: Self = <$t>::MIN;
            const MIN_POSITIVE: Self = <$t>::MIN_POSITIVE;
            const MAX: Self = <$t>::MAX;

            const MIN_EXP: i32 = <$t>::MIN_EXP;
            const MAX_EXP: i32 = <$t>::MAX_EXP;

            const RADIX: Self = <$t>::RADIX as Self;
            const MANTISSA_DIGITS: u32 = <$t>::MANTISSA_DIGITS;

            #[inline]
            fn is_nan(self) -> bool {
                self.is_nan()
            }
            #[inline]
            fn is_infinite(self) -> bool {
                self.is_infinite()
            }
            #[inline]
            fn is_finite(self) -> bool {
                self.is_finite()
            }
            #[inline]
            fn is_normal(self) -> bool {
                self.is_normal()
            }
            #[inline]
            fn is_subnormal(self) -> bool {
                self.is_subnormal()
            }
            #[inline]
            fn floor(self) -> Self {
                self.floor()
            }
            #[inline]
            fn ceil(self) -> Self {
                self.ceil()
            }
            #[inline]
            fn round(self) -> Self {
                self.round()
            }
            #[inline]
            fn trunc(self) -> Self {
                self.trunc()
            }
            #[inline]
            fn fract(self) -> Self {
                self.fract()
            }
            #[inline]
            fn abs(self) -> Self {
                self.abs()
            }
            #[inline]
            fn signum(self) -> Self {
                self.signum()
            }
            #[inline]
            fn is_sign_positive(self) -> bool {
                self.is_sign_positive()
            }
            #[inline]
            fn is_sign_negative(self) -> bool {
                self.is_sign_negative()
            }
            #[inline]
            fn min(self, other: Self) -> Self {
                self.min(other)
            }
            #[inline]
            fn max(self, other: Self) -> Self {
                self.max(other)
            }
            #[inline]
            fn clamp(self, min: Self, max: Self) -> Self {
                self.clamp(min, max)
            }
            #[inline]
            fn recip(self) -> Self {
                self.recip()
            }
            #[inline]
            fn powi(self, exp: i32) -> Self {
                self.powi(exp)
            }
            #[inline]
            fn copysign(self, sign: Self) -> Self {
                self.copysign(sign)
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);

pub trait FloatMath: Float {
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn powf(self, n: Self) -> Self;
    fn sqrt(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn ln(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log2(self) -> Self;
    fn log10(self) -> Self;
    fn cbrt(self) -> Self;
    fn hypot(self, other: Self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn exp_m1(self) -> Self;
    fn ln_1p(self) -> Self;
    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;
    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;
}

macro_rules! impl_float_math {
    ($t:ty) => {
        impl FloatMath for $t {
            #[inline]
            fn mul_add(self, a: Self, b: Self) -> Self {
                self.mul_add(a, b)
            }
            #[inline]
            fn powf(self, n: Self) -> Self {
                self.powf(n)
            }
            #[inline]
            fn sqrt(self) -> Self {
                self.sqrt()
            }
            #[inline]
            fn exp(self) -> Self {
                self.exp()
            }
            #[inline]
            fn exp2(self) -> Self {
                self.exp2()
            }
            #[inline]
            fn ln(self) -> Self {
                self.ln()
            }
            #[inline]
            fn log(self, base: Self) -> Self {
                self.log(base)
            }
            #[inline]
            fn log2(self) -> Self {
                self.log2()
            }
            #[inline]
            fn log10(self) -> Self {
                self.log10()
            }
            #[inline]
            fn cbrt(self) -> Self {
                self.cbrt()
            }
            #[inline]
            fn hypot(self, other: Self) -> Self {
                self.hypot(other)
            }
            #[inline]
            fn sin(self) -> Self {
                self.sin()
            }
            #[inline]
            fn cos(self) -> Self {
                self.cos()
            }
            #[inline]
            fn tan(self) -> Self {
                self.tan()
            }
            #[inline]
            fn asin(self) -> Self {
                self.asin()
            }
            #[inline]
            fn acos(self) -> Self {
                self.acos()
            }
            #[inline]
            fn atan(self) -> Self {
                self.atan()
            }
            #[inline]
            fn atan2(self, other: Self) -> Self {
                self.atan2(other)
            }
            #[inline]
            fn sin_cos(self) -> (Self, Self) {
                self.sin_cos()
            }
            #[inline]
            fn exp_m1(self) -> Self {
                self.exp_m1()
            }
            #[inline]
            fn ln_1p(self) -> Self {
                self.ln_1p()
            }
            #[inline]
            fn sinh(self) -> Self {
                self.sinh()
            }
            #[inline]
            fn cosh(self) -> Self {
                self.cosh()
            }
            #[inline]
            fn tanh(self) -> Self {
                self.tanh()
            }
            #[inline]
            fn asinh(self) -> Self {
                self.asinh()
            }
            #[inline]
            fn acosh(self) -> Self {
                self.acosh()
            }
            #[inline]
            fn atanh(self) -> Self {
                self.atanh()
            }
        }
    };
}

impl_float_math!(f32);
impl_float_math!(f64);
