mod exponential;

pub use exponential::Exponential;

pub trait Continuous {
    fn pdf(&self, x: f64) -> f64;

    fn ln_pdf(&self, x: f64) -> f64 {
        self.pdf(x).ln()
    }

    fn cdf(&self, x: f64) -> f64;

    fn ln_cdf(&self, x: f64) -> f64 {
        self.cdf(x).ln()
    }

    fn sf(&self, x: f64) -> f64 {
        1.0 - self.cdf(x)
    }

    fn ln_sf(&self, x: f64) -> f64 {
        self.sf(x).ln()
    }

    #[doc(alias = "quantile function")]
    #[doc(alias = "ppf")]
    #[doc(alias = "percent point function")]
    fn inverse_cdf(&self, p: f64) -> f64 {
        if p == 0.0 {
            return self.lower();
        };
        if p == 1.0 {
            return self.upper();
        };
        let mut high = 2.0;
        let mut low = -2.0;
        while self.cdf(low) > p {
            low *= 2.0;
        }
        while self.cdf(high) < p {
            high *= 2.0;
        }
        for _ in 0..16 {
            let mid = (high + low) / 2.0;
            if self.cdf(mid) >= p {
                high = mid;
            } else {
                low = mid;
            }
        }
        (high + low) / 2.0
    }

    fn support(&self) -> (f64, f64);

    fn lower(&self) -> f64 {
        self.support().0
    }

    fn upper(&self) -> f64 {
        self.support().1
    }
}

pub trait Statistics {
    type MeanErr;
    fn mean(&self) -> Result<f64, Self::MeanErr>;

    type MedianErr;
    fn median(&self) -> Result<f64, Self::MedianErr>;

    type ModeErr;
    fn mode(&self) -> Result<f64, Self::ModeErr>;

    type VarianceErr;
    fn variance(&self) -> Result<f64, Self::VarianceErr>;
    fn std(&self) -> Result<f64, Self::VarianceErr> {
        self.variance().map(|v| v.sqrt())
    }

    type EntropyErr;
    fn entropy(&self) -> Result<f64, Self::EntropyErr>;
}
