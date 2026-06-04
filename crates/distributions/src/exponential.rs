use super::Continuous;

pub struct Exponential {
    rate: f64,
}

impl Exponential {
    pub fn new(rate: f64) -> Self {
        Exponential { rate }
    }

    pub fn new_with_scale(scale: f64) -> Self {
        Exponential {
            rate: scale.recip(),
        }
    }
}

impl Continuous for Exponential {
    fn pdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            0.0
        } else {
            self.rate * (-self.rate * x).exp()
        }
    }

    fn ln_pdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            f64::NEG_INFINITY
        } else {
            self.rate.ln() - self.rate * x
        }
    }

    fn cdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            0.0
        } else {
            1.0 - (-self.rate * x).exp()
        }
    }

    fn sf(&self, x: f64) -> f64 {
        if x < 0.0 { 1.0 } else { (-self.rate * x).exp() }
    }

    fn support(&self) -> (f64, f64) {
        (0.0, f64::INFINITY)
    }
}
