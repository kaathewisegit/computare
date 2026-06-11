use super::ziggurat_tables;
use rand::distr::hidden_export::IntoFloat;
use rand::{Rng, RngExt, distr::Open01};

pub fn sample_std_normal<R: Rng + ?Sized>(rng: &mut R) -> f64 {
    fn pdf(x: f64) -> f64 {
        (-x * x / 2.0).exp()
    }

    fn zero_case<R: Rng + ?Sized>(rng: &mut R, u: f64) -> f64 {
        let mut x = 1.0f64;
        let mut y = 0.0f64;
        while -2.0 * y < x * x {
            let x_: f64 = rng.sample(Open01);
            let y_: f64 = rng.sample(Open01);

            x = x_.ln() / ziggurat_tables::ZIG_NORM_R;
            y = y_.ln();
        }
        if u < 0.0 {
            x - ziggurat_tables::ZIG_NORM_R
        } else {
            ziggurat_tables::ZIG_NORM_R - x
        }
    }

    ziggurat(
        rng,
        true,
        &ziggurat_tables::ZIG_NORM_X,
        &ziggurat_tables::ZIG_NORM_F,
        pdf,
        zero_case,
    )
}

pub fn sample_exp_1<R: Rng + ?Sized>(rng: &mut R) -> f64 {
    fn pdf(x: f64) -> f64 {
        (-x).exp()
    }

    fn zero_case<R: Rng + ?Sized>(rng: &mut R, _u: f64) -> f64 {
        ziggurat_tables::ZIG_EXP_R - rng.random::<f64>().ln()
    }

    ziggurat(
        rng,
        false,
        &ziggurat_tables::ZIG_EXP_X,
        &ziggurat_tables::ZIG_EXP_F,
        pdf,
        zero_case,
    )
}

// Ziggurat method for sampling a random number based on the ZIGNOR variant from
// Doornik 2005
//
// Code borrowed from [`rand`][r]
//
// [r]: https://github.com/rust-random/rand_distr/blob/master/src/utils.rs#L50
fn ziggurat<R: Rng + ?Sized>(
    rng: &mut R,
    symmetric: bool,
    // Ziggurat tables
    x_tab: &'static [f64; 257],
    f_tab: &'static [f64; 257],
    mut pdf: impl FnMut(f64) -> f64,
    mut zero_case: impl FnMut(&mut R, f64) -> f64,
) -> f64 {
    loop {
        // As an optimisation we re-implement the conversion to a f64.  From the
        // remaining 12 most significant bits we use 8 to construct `i`.  This
        // saves us generating a whole extra random number, while the added
        // precision of using 64 bits for f64 does not buy us much.
        let bits = rng.next_u64();
        let i = bits as usize & 0xff;

        let u = if symmetric {
            // Convert to a value in the range [2,4) and subtract to get [-1,1)
            // We can't convert to an open range directly, that would require
            // subtracting `3.0 - EPSILON`, which is not representable.  It is
            // possible with an extra step, but an open range does not seem
            // necessary for the ziggurat algorithm anyway.
            (bits >> 12).into_float_with_exponent(1) - 3.0
        } else {
            // Convert to a value in the range [1,2) and subtract to get (0,1)
            (bits >> 12).into_float_with_exponent(0)
                - (1.0 - f64::EPSILON / 2.0)
        };
        let x = u * x_tab[i];

        let test_x = if symmetric { x.abs() } else { x };

        // algebraically equivalent to |u| < x_tab[i+1]/x_tab[i] (or u <
        // x_tab[i+1]/x_tab[i])
        if test_x < x_tab[i + 1] {
            return x;
        }
        if i == 0 {
            return zero_case(rng, u);
        }
        // algebraically equivalent to f1 + DRanU()*(f0 - f1) < 1
        if f_tab[i + 1] + (f_tab[i] - f_tab[i + 1]) * rng.random::<f64>()
            < pdf(x)
        {
            return x;
        }
    }
}
