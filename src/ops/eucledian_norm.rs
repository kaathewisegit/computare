use crate::{
    int_utils::{div_ceil, div_floor},
    num::{Float, NumAssign},
    vector::Vector,
};

/// `∥x∥₂ ≡ √(∑xi²)`
///
/// This function is based on [Edward Anderson's 2017 update][p0] of [James. L.
/// Blue's original 1978 scaling algorithm].  It splits all input values into 3
/// bins: small, large, and big, and sums their squares separately.
///
/// [p0]: https://doi.org/10.1145/3061665
/// [p1]: https://doi.org/10.1145/355769.355771
pub fn eucledian_norm<T, V>(v: &V) -> T
where
    T: Float + NumAssign,
    V: Vector<T> + ?Sized,
{
    let radix = T::RADIX;
    let mantissa_digits = T::MANTISSA_DIGITS as i32;

    let threshold_small = radix.powi(div_ceil(T::MIN_EXP - 1, 2));
    let threshold_big =
        radix.powi(div_floor(T::MAX_EXP - mantissa_digits + 1, 2));
    let scale_small = radix.powi(-div_floor(T::MIN_EXP - mantissa_digits, 2));
    let scale_big = radix.powi(-div_ceil(T::MAX_EXP + mantissa_digits - 1, 2));

    let mut acc_small = T::ZERO;
    let mut acc_mid = T::ZERO;
    let mut acc_big = T::ZERO;

    let mut no_big = true;

    for i in 0..v.length() {
        // SAFETY: `at_u(i)` with `i < self.length()` is safe
        let value = unsafe { *v.at_u(i) };
        let abs = value.abs();

        // XXX: no_big is pretty branchy.  It might make sense to get rid of it,
        // but this needs benchmarking.
        if abs > threshold_big {
            acc_big += (abs * scale_big).powi(2);
            no_big = false;
        } else if abs < threshold_small && no_big {
            acc_small += (abs * scale_small).powi(2);
        } else {
            acc_mid += abs.powi(2);
        }
    }

    let (sum, scale) = if !no_big {
        acc_big += acc_mid * scale_big * scale_big;
        (acc_big, T::ONE / scale_big)
    } else if acc_small > T::ZERO {
        if acc_mid != T::ZERO {
            let sqrt_acc_mid = acc_mid.sqrt();
            let sqrt_acc_small = acc_small.sqrt();

            let (lower, upper) = if sqrt_acc_mid < sqrt_acc_small {
                (sqrt_acc_mid, sqrt_acc_small)
            } else {
                (sqrt_acc_small, sqrt_acc_mid)
            };

            let sum = upper.powi(2) * (T::ONE + (lower / upper).powi(2));
            (sum, T::ONE)
        } else {
            (acc_small, T::ONE / scale_small)
        }
    } else {
        (acc_mid, T::ONE)
    };

    scale * sum.sqrt()
}
