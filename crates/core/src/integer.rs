use crate::{Num, Zero};

pub trait Integer: Sized + Num + PartialOrd + Ord + Eq {
    fn div_floor(&self, other: &Self) -> Self;
    fn mod_floor(&self, other: &Self) -> Self;
    fn div_ceil(&self, other: &Self) -> Self;
    fn gcd(&self, other: &Self) -> Self;
    fn lcm(&self, other: &Self) -> Self;
    fn gcd_lcm(&self, other: &Self) -> (Self, Self);
    fn is_multiple_of(&self, other: &Self) -> bool;
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
    fn div_rem(&self, other: &Self) -> (Self, Self);

    fn dec(&mut self)
    where
        Self: Clone,
    {
        *self -= Self::one();
    }

    fn inc(&mut self)
    where
        Self: Clone,
    {
        *self += Self::one();
    }
}

macro_rules! impl_unsinged_integer {
    ($t:ty) => {
        impl Integer for $t {
            #[inline]
            fn div_floor(&self, other: &Self) -> Self {
                *self / *other
            }

            #[inline]
            fn mod_floor(&self, other: &Self) -> Self {
                *self % *other
            }

            #[inline]
            fn div_ceil(&self, other: &Self) -> Self {
                *self / *other + (0 != *self % *other) as Self
            }

            #[inline]
            fn gcd(&self, other: &Self) -> Self {
                // Use Stein's algorithm
                let mut m = *self;
                let mut n = *other;
                if m == 0 || n == 0 {
                    return m | n;
                }

                // find common factors of 2
                let shift = (m | n).trailing_zeros();

                // divide n and m by 2 until odd
                m >>= m.trailing_zeros();
                n >>= n.trailing_zeros();

                while m != n {
                    if m > n {
                        m -= n;
                        m >>= m.trailing_zeros();
                    } else {
                        n -= m;
                        n >>= n.trailing_zeros();
                    }
                }
                m << shift
            }

            #[inline]
            fn lcm(&self, other: &Self) -> Self {
                self.gcd_lcm(other).1
            }

            #[inline]
            fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
                if self.is_zero() && other.is_zero() {
                    return (Self::zero(), Self::zero());
                }
                let gcd = self.gcd(other);
                let lcm = *self * (*other / gcd);
                (gcd, lcm)
            }

            #[inline]
            fn is_multiple_of(&self, other: &Self) -> bool {
                if other.is_zero() {
                    return self.is_zero();
                }
                *self % *other == 0
            }

            #[inline]
            fn is_even(&self) -> bool {
                *self % 2 == 0
            }

            #[inline]
            fn is_odd(&self) -> bool {
                !self.is_even()
            }

            #[inline]
            fn div_rem(&self, other: &Self) -> (Self, Self) {
                (*self / *other, *self % *other)
            }
        }
    };
}

impl_unsinged_integer!(u8);
impl_unsinged_integer!(u16);
impl_unsinged_integer!(u32);
impl_unsinged_integer!(u64);
impl_unsinged_integer!(u128);
impl_unsinged_integer!(usize);

macro_rules! impl_signed_integer {
    ($t:ty) => {
        impl Integer for $t {
            #[inline]
            fn div_floor(&self, other: &Self) -> Self {
                // Algorithm from [Daan Leijen. Division and Modulus for
                // Computer Scientists, December 2001][l]
                //
                // [l]: http://research.microsoft.com/pubs/151917/divmodnote-letter.pdf
                let (d, r) = self.div_rem(other);
                if (r > 0 && *other < 0) || (r < 0 && *other > 0) {
                    d - 1
                } else {
                    d
                }
            }

            #[inline]
            fn mod_floor(&self, other: &Self) -> Self {
                let r = *self % *other;
                if (r > 0 && *other < 0) || (r < 0 && *other > 0) {
                    r + *other
                } else {
                    r
                }
            }

            #[inline]
            fn div_ceil(&self, other: &Self) -> Self {
                let (d, r) = self.div_rem(other);
                if (r > 0 && *other > 0) || (r < 0 && *other < 0) {
                    d + 1
                } else {
                    d
                }
            }

            #[inline]
            fn gcd(&self, other: &Self) -> Self {
                // Use Stein's algorithm
                let mut m = *self;
                let mut n = *other;
                if m == 0 || n == 0 {
                    return (m | n).abs();
                }

                // find common factors of 2
                let shift = (m | n).trailing_zeros();

                // The algorithm needs positive numbers, but the minimum value
                // can't be represented as a positive one.  It's also a power of
                // two, so the gcd can be calculated by bitshifting in that case

                // Assuming two's complement, the number created by the shift is
                // positive for all numbers except gcd = abs(min value)
                if m == Self::MIN || n == Self::MIN {
                    let out = 1 << shift;
                    debug_assert!(out > 0);
                    return out;
                }

                // guaranteed to be positive now, rest like unsigned algorithm
                m = m.abs();
                n = n.abs();

                // divide n and m by 2 until odd
                m >>= m.trailing_zeros();
                n >>= n.trailing_zeros();

                while m != n {
                    if m > n {
                        m -= n;
                        m >>= m.trailing_zeros();
                    } else {
                        n -= m;
                        n >>= n.trailing_zeros();
                    }
                }
                m << shift
            }

            #[inline]
            fn lcm(&self, other: &Self) -> Self {
                self.gcd_lcm(other).1
            }

            #[inline]
            fn gcd_lcm(&self, other: &Self) -> (Self, Self) {
                if self.is_zero() && other.is_zero() {
                    return (Self::zero(), Self::zero());
                }
                let gcd = self.gcd(other);
                // should not have to recalculate abs
                let lcm = (*self * (*other / gcd)).abs();
                (gcd, lcm)
            }

            #[inline]
            fn is_multiple_of(&self, other: &Self) -> bool {
                if other.is_zero() {
                    return self.is_zero();
                }
                *self % *other == 0
            }

            #[inline]
            fn is_even(&self) -> bool {
                *self % 2 == 0
            }

            #[inline]
            fn is_odd(&self) -> bool {
                !self.is_even()
            }

            #[inline]
            fn div_rem(&self, other: &Self) -> (Self, Self) {
                (*self / *other, *self % *other)
            }
        }
    };
}

impl_signed_integer!(i8);
impl_signed_integer!(i16);
impl_signed_integer!(i32);
impl_signed_integer!(i64);
impl_signed_integer!(i128);
impl_signed_integer!(isize);
