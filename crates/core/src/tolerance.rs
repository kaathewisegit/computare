use crate::{Float, Num};

pub trait Tolerance {
    type Absolute: Num + PartialOrd;
    type Relative: Float;
    type Ulps: Num + PartialOrd;

    const DEFAULT_ABSOLUTE: Self::Absolute;
    const DEFAULT_RELATIVE: Self::Relative;
    const DEFAULT_ULPS: Self::Ulps;

    fn absolute_diff(&self, other: &Self) -> Self::Absolute;

    fn relative_diff(&self, other: &Self) -> Self::Relative;

    fn ulps_diff(&self, other: &Self) -> Self::Ulps;
}

pub struct ToleranceConfig<T: Tolerance> {
    pub absolute: T::Absolute,
    pub relative: T::Relative,
    pub ulps: T::Ulps,
}

impl<T: Tolerance> Default for ToleranceConfig<T> {
    fn default() -> Self {
        Self {
            absolute: T::DEFAULT_ABSOLUTE,
            relative: T::DEFAULT_RELATIVE,
            ulps: T::DEFAULT_ULPS,
        }
    }
}

pub fn is_close<T>(a: &T, b: &T, config: ToleranceConfig<T>) -> bool
where
    T: Tolerance + std::fmt::Debug,
    T::Absolute: std::fmt::Display,
    T::Relative: std::fmt::Display,
{
    a.absolute_diff(b) < config.absolute
        || a.relative_diff(b) < config.relative
        || a.ulps_diff(b) < config.ulps
}

#[macro_export]
macro_rules! almost_eq {
	($a:expr, $b:expr $(, $opt:ident = $val:expr)* $(,)?) => {{
		let mut c = $crate::tolerance::ToleranceConfig::default();
		$(c.$opt = $val;)*

		$crate::tolerance::is_close(&$a, &$b, c)
	}};
}

#[macro_export]
macro_rules! assert_almost_eq {
	($a:expr, $b:expr $(, $opt:ident = $val:expr)* $(,)?) => {{
		if !$crate::almost_eq!($a, $b, $($opt = $val),*) {
			use $crate::tolerance::Tolerance;
			panic!(
				"assert_almost_eq!({}, {}) failed
     got: {:?}
expected: {:?}
 ---------
abs diff: {}
relative: {}
    ulps: {}",
    		stringify!($a),
    		stringify!($b),
		$a,
		$b,
		$a.absolute_diff(&$b),
		$a.relative_diff(&$b),
		$a.ulps_diff(&$b),
			);
		}
	}};
}

pub use crate::{almost_eq, assert_almost_eq};

impl Tolerance for f64 {
    type Absolute = f64;
    type Relative = f64;
    type Ulps = u64;

    const DEFAULT_ABSOLUTE: Self::Absolute = f64::EPSILON;
    const DEFAULT_RELATIVE: Self::Relative = 1e-16;
    const DEFAULT_ULPS: Self::Ulps = 4;

    fn absolute_diff(&self, other: &Self) -> f64 {
        (self - other).abs()
    }

    fn relative_diff(&self, other: &Self) -> f64 {
        if self.signum() != other.signum() {
            return f64::INFINITY;
        }

        if self == other {
            return 0.0;
        }

        let a = self.abs();
        let b = other.abs();

        let (min, max) = (a.min(b), a.max(b));

        (max - min) / min
    }

    fn ulps_diff(&self, other: &Self) -> u64 {
        let self_bits = self.to_bits();
        let other_bits = other.to_bits();

        self_bits.abs_diff(other_bits)
    }
}
