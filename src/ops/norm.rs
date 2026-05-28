use crate::{
    int_utils::{div_ceil, div_floor},
    num::{Float, NumAssign},
    vector::Vector,
};

fn threshold_small<const P: u8, T: Float>() -> T {
    T::RADIX.powi(div_ceil(T::MIN_EXP - 1, P as i32))
}

fn threshold_big<const P: u8, T: Float>() -> T {
    T::RADIX.powi(div_floor(
        T::MAX_EXP - T::MANTISSA_DIGITS as i32 + 1,
        P as i32,
    ))
}

fn scale_small<const P: u8, T: Float>() -> T {
    T::RADIX.powi(-div_floor(T::MIN_EXP - T::MANTISSA_DIGITS as i32, P as i32))
}

fn scale_big<const P: u8, T: Float>() -> T {
    T::RADIX.powi(-div_ceil(
        T::MAX_EXP + T::MANTISSA_DIGITS as i32 - 1,
        P as i32,
    ))
}

fn accumulate<const P: u8, T: Float + NumAssign>(
    acc_small: T,
    acc_mid: T,
    acc_big: T,
    scale_small: T,
    scale_big: T,
) -> T {
    let p = P as i32;
    let (sum, scale) = if acc_big > T::ZERO {
        (acc_big + acc_mid * scale_big.powi(p), T::ONE / scale_big)
    } else if acc_small > T::ZERO {
        if acc_mid != T::ZERO {
            let sqrt_acc_mid = acc_mid.sqrt();
            let sqrt_acc_small = acc_small.sqrt();

            let (lower, upper) = if sqrt_acc_mid < sqrt_acc_small {
                (sqrt_acc_mid, sqrt_acc_small)
            } else {
                (sqrt_acc_small, sqrt_acc_mid)
            };

            let sum = upper.powi(p) * (T::ONE + (lower / upper).powi(p));
            (sum, T::ONE)
        } else {
            (acc_small, T::ONE / scale_small)
        }
    } else {
        (acc_mid, T::ONE)
    };

    if P == 2 {
        scale * sum.sqrt()
    } else if P == 3 {
        scale * sum.cbrt()
    } else {
        scale * sum.nth_root(P)
    }
}

/// Calculates the `p-norm`: `∥v∥_p ≡ (∑v_i^p)^(1/p)`
///
/// This is a generalization of the [Edward Anderson's 2017 update][p0] of
/// [James. L.  Blue's original 1978 scaling algorithm][p1].  It splits all
/// input values into 3 bins: small, large, and big, and sums their squares
/// separately.  The scaling is done in a way that `∀x < small : x^p` doesn't
/// underflow (and the same idea for the upper threshold).  The scaling
/// constants are powers of two to make multiplication easier.
///
/// [p0]: https://doi.org/10.1145/3061665
/// [p1]: https://doi.org/10.1145/355769.355771
pub fn p_norm<const P: u8, T, V>(v: &V) -> T
where
    T: Float + NumAssign,
    V: Vector<T> + ?Sized,
{
    let p = P as i32;
    let threshold_small = threshold_small::<P, T>();
    let threshold_big = threshold_big::<P, T>();
    let scale_small = scale_small::<P, T>();
    let scale_big = scale_big::<P, T>();

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
            acc_big += (abs * scale_big).powi(p);
            no_big = false;
        } else if abs < threshold_small && no_big {
            acc_small += (abs * scale_small).powi(p);
        } else {
            acc_mid += abs.powi(p);
        }
    }

    accumulate::<P, T>(acc_small, acc_mid, acc_big, scale_small, scale_big)
}

/// `∥x∥₂ ≡ √(∑xi²)`
///
/// This function is a wrapper over [`p_norm`].
pub fn eucledian_norm<T, V>(v: &V) -> T
where
    T: Float + NumAssign,
    V: Vector<T> + ?Sized,
{
    p_norm::<2, T, V>(v)
}
