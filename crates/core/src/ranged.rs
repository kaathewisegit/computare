use core::ops::Deref;

use crate::{One, Zero};

/// A wrapper type whose value is always positive (stricter larger than zero)
pub struct Positive<T>(T);

impl<T: Zero + PartialOrd> Positive<T> {
    pub fn try_new(value: T) -> Option<Self> {
        if value > T::zero() {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Panics if `value <= 0`
    pub fn new(value: T) -> Self {
        Self::try_new(value).unwrap()
    }
}

impl<T> Deref for Positive<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/// A wrapper type whose value is always negative (stricter smaller than zero)
pub struct Negative<T>(T);

impl<T: Zero + PartialOrd> Negative<T> {
    pub fn try_new(value: T) -> Option<Self> {
        if value < T::zero() {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Panics if `value <= 0`
    pub fn new(value: T) -> Self {
        Self::try_new(value).unwrap()
    }
}

impl<T> Deref for Negative<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

/// A wrapper over `F` whose value always lies in `[0, 1]`
pub struct Unit<F>(F);

impl<T: Zero + One + PartialOrd> Unit<T> {
    pub fn try_new(value: T) -> Option<Self> {
        if T::zero() <= value && value <= T::one() {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Panics if `value` is out of bounds
    pub fn new(value: T) -> Self {
        Self::try_new(value).unwrap()
    }
}

impl<T> Deref for Unit<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub type Probability<F> = Unit<F>;
