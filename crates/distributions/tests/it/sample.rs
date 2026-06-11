//! Very basic smoke tests

use computare_distributions::{
    Continuous, Exponential, Gamma, Laplace, LogNormal, Normal, Uniform,
};
use rand::{SeedableRng, distr::Distribution, rngs::SmallRng};

const N: usize = 100;

fn rng() -> SmallRng {
    SmallRng::from_seed([4; 32])
}

fn assert_in_support(dist: &dyn Continuous, x: f64) {
    assert!(x.is_finite(), "sample is not finite: {x}");
    let (lo, hi) = dist.support();
    if lo.is_finite() {
        assert!(x >= lo, "sample {x} below lower bound {lo}");
    }
    if hi.is_finite() {
        assert!(x <= hi, "sample {x} above upper bound {hi}");
    }
}

#[test]
fn normal_sample() {
    let dist = Normal::new(0.0, 1.0);
    let mut rng = rng();
    for _ in 0..N {
        let x = dist.sample(&mut rng);
        assert_in_support(&dist, x);
    }
}

#[test]
fn normal_sample_shifted() {
    let dist = Normal::new(5.0, 2.0);
    let mut rng = rng();
    for _ in 0..N {
        let x = dist.sample(&mut rng);
        assert_in_support(&dist, x);
    }
}

#[test]
fn log_normal_sample() {
    let dist = LogNormal::new(0.0, 1.0);
    let mut rng = rng();
    for _ in 0..N {
        let x = dist.sample(&mut rng);
        assert_in_support(&dist, x);
        assert!(x > 0.0, "log-normal sample must be positive: {x}");
    }
}

#[test]
fn log_normal_sample_shifted() {
    let dist = LogNormal::new(2.0, 0.5);
    let mut rng = rng();
    for _ in 0..N {
        let x = dist.sample(&mut rng);
        assert_in_support(&dist, x);
        assert!(x > 0.0, "log-normal sample must be positive: {x}");
    }
}

#[test]
fn uniform_sample() {
    let dist = Uniform::new(1.0, 10.0);
    let mut rng = rng();
    for _ in 0..N {
        let x = dist.sample(&mut rng);
        assert_in_support(&dist, x);
    }
}

#[test]
fn uniform_sample_negative_range() {
    let dist = Uniform::new(-5.0, 5.0);
    let mut rng = rng();
    for _ in 0..N {
        let x = dist.sample(&mut rng);
        assert_in_support(&dist, x);
    }
}

#[test]
fn laplace_sample() {
    let dist = Laplace::new(0.0, 1.0);
    let mut rng = rng();
    for _ in 0..N {
        let x = dist.sample(&mut rng);
        assert_in_support(&dist, x);
    }
}

#[test]
fn laplace_sample_shifted() {
    let dist = Laplace::new(3.0, 0.5);
    let mut rng = rng();
    for _ in 0..N {
        let x = dist.sample(&mut rng);
        assert_in_support(&dist, x);
    }
}

#[test]
fn exponential_sample() {
    let dist = Exponential::new(1.0);
    let mut rng = rng();
    for _ in 0..N {
        let x = dist.sample(&mut rng);
        assert_in_support(&dist, x);
        assert!(x >= 0.0, "exponential sample must be non-negative: {x}");
    }
}

#[test]
fn exponential_sample_high_rate() {
    let dist = Exponential::new(10.0);
    let mut rng = rng();
    for _ in 0..N {
        let x = dist.sample(&mut rng);
        assert_in_support(&dist, x);
        assert!(x >= 0.0, "exponential sample must be non-negative: {x}");
    }
}

#[test]
fn gamma_sample() {
    let dist = Gamma::new(1.0, 1.0);
    let mut rng = rng();
    for _ in 0..N {
        let x = dist.sample(&mut rng);
        assert_in_support(&dist, x);
        assert!(x >= 0.0, "gamma sample must be non-negative: {x}");
    }
}

#[test]
fn gamma_sample_small_shape() {
    let dist = Gamma::new(0.5, 1.0);
    let mut rng = rng();
    for _ in 0..N {
        let x = dist.sample(&mut rng);
        assert_in_support(&dist, x);
        assert!(x >= 0.0, "gamma sample must be non-negative: {x}");
    }
}

#[test]
fn gamma_sample_large_shape() {
    let dist = Gamma::new(10.0, 2.0);
    let mut rng = rng();
    for _ in 0..N {
        let x = dist.sample(&mut rng);
        assert_in_support(&dist, x);
        assert!(x >= 0.0, "gamma sample must be non-negative: {x}");
    }
}
