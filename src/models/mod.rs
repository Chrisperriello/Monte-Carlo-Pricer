use rand_distr::{Distribution, Normal};
use rayon::prelude::*;

#[derive(Debug)]
pub enum PricerError {
    InvalidInput(String),
}

#[derive(Debug, Clone, Copy)]
pub struct PricingResult {
    pub price: f64,
    pub standard_error: f64,
    pub num_sims: u64,
}

pub struct LubrizolOption {
    pub s0: f64,
    pub k: f64,
    pub t: f64,
    pub r: f64,
    pub sigma: f64,
    pub option_type: OptionType,
}

#[derive(Debug, Clone, Copy)]
pub enum OptionType {
    Call,
    Put,
}

impl PricingResult {
    pub fn confidence_interval_95(&self) -> (f64, f64) {
        let margin = 1.96 * self.standard_error;
        (self.price - margin, self.price + margin)
    }
}

impl LubrizolOption {
    pub fn new(
        s0: f64,
        k: f64,
        t: f64,
        r: f64,
        sigma: f64,
        option_type: OptionType,
    ) -> Result<Self, PricerError> {
        if sigma < 0.0 {
            return Err(PricerError::InvalidInput(
                "Volatility must be positive".into(),
            ));
        }

        if t < 0.0 {
            return Err(PricerError::InvalidInput(
                "Time to expiry must be positive".into(),
            ));
        }
        Ok(Self {
            s0,
            k,
            t,
            r,
            sigma,
            option_type,
        })
    }
}

fn gbm(start_price: f64, opt: &LubrizolOption, epsilon: f64) -> f64 {
    start_price
        * ((opt.r - ((opt.sigma.powi(2)) / 2.0)) * opt.t + (opt.sigma * opt.t.sqrt() * epsilon))
            .exp()
}

pub fn price_option(opt: &LubrizolOption, num_sims: u64) -> PricingResult {
    //let mut running_sum = 0.0;

    let normal = Normal::new(0.0, 1.0).unwrap();

    let (sum_y, sum_sq_y): (f64, f64) = (0..num_sims)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let epsilon: f64 = normal.sample(&mut rng);

            //Calculate the final price using GBM formula
            let st = gbm(opt.s0, &opt, epsilon);

            let payoff_pos;
            //Calulate the payoff
            match opt.option_type {
                OptionType::Call => payoff_pos = (st - opt.k).max(0.0),
                OptionType::Put => payoff_pos = (opt.k - st).max(0.0),
            };

            let st_neg = gbm(opt.s0, &opt, -epsilon);
            let payoff_neg = match opt.option_type {
                OptionType::Call => (st_neg - opt.k).max(0.0),
                OptionType::Put => (opt.k - st_neg).max(0.0),
            };
            let y = (payoff_pos + payoff_neg) / 2.0;

            //Return (y, y^2)
            (y, y * y)
        })
        .reduce(|| (0.0, 0.0), |a, b| (a.0 + b.0, a.1 + b.1));
    /*
    for _ in 0..num_sims {
        //Genreate the random schock
        let epsilon: f64 = normal.sample(&mut rng);

        //Calculate the final price using GBM formula
        let st = gbm(&opt, epsilon);

        //Calulate the payoff
        let pay = (st - opt.k).max(0.0);
        running_sum += pay;
    }
    */

    let m = num_sims as f64;
    let discount = (-opt.r * opt.t).exp();

    //1. Calc price
    let avg_payoff = sum_y / m;
    let price = avg_payoff * discount;

    // 2. Caluclate undiscounted sample
    let variance = (sum_sq_y - (sum_y * sum_y) / m) / (m - 1.0);

    //3. Calculate discounted standard error
    let standard_error = (variance / m).sqrt() * discount;

    PricingResult {
        price,
        standard_error,
        num_sims,
    }
}

pub fn price_delta(opt: &LubrizolOption, num_sims: u64, bump: f64) -> PricingResult {
    let normal = Normal::new(0.0, 1.0).unwrap();

    let (sum_y, sum_squr_y): (f64, f64) = (0..num_sims)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let epsilon: f64 = normal.sample(&mut rng);

            //Calculate the final price using GBM formula
            // to deal with Antithetic Vaiates we average the
            // positive and negative paths
            let st_base_pos = gbm(opt.s0, &opt, epsilon);
            //Calulate the payoff for S0
            let payoff_base;

            //Calculate price at S0 + bump
            //Calulate the payoff
            match opt.option_type {
                OptionType::Call => payoff_base = (st_base_pos - opt.k).max(0.0),
                OptionType::Put => payoff_base = (opt.k - st_base_pos).max(0.0),
            }

            let st_bump_pos = gbm(opt.s0 + bump, opt, epsilon);
            let payoff_bumped_pos;

            match opt.option_type {
                OptionType::Call => payoff_bumped_pos = (st_bump_pos - opt.k).max(0.0),
                OptionType::Put => payoff_bumped_pos = (opt.k - st_bump_pos).max(0.0),
            }
            let payoff_pos = (payoff_bumped_pos - payoff_base) / bump;

            //Now for the negative
            let st_base_neg = gbm(opt.s0, &opt, -epsilon);
            //Calulate the payoff for S0
            let payoff_base_neg;

            //Calculate price at S0 + bump
            //Calulate the payoff
            match opt.option_type {
                OptionType::Call => payoff_base_neg = (st_base_neg - opt.k).max(0.0),
                OptionType::Put => payoff_base_neg = (opt.k - st_base_neg).max(0.0),
            }

            let st_bump_neg = gbm(opt.s0 + bump, opt, -epsilon);
            let payoff_bumped_neg;

            match opt.option_type {
                OptionType::Call => payoff_bumped_neg = (st_bump_neg - opt.k).max(0.0),
                OptionType::Put => payoff_bumped_neg = (opt.k - st_bump_neg).max(0.0),
            }
            let payoff_neg = (payoff_bumped_neg - payoff_base_neg) / bump;

            let y = (payoff_pos + payoff_neg) / 2.0;

            (y, y * y)
        })
        .reduce(|| (0.0, 0.0), |a, b| (a.0 + b.0, a.1 + b.1));
    let m = num_sims as f64;
    let avg_delta = sum_y / (m);
    let discount = (-opt.r * opt.t).exp();
    let price = avg_delta * discount;

    // 2. Caluclate undiscounted sample
    let variance = (sum_squr_y - (sum_y * sum_y) / m) / (m - 1.0);

    //3. Calculate discounted standard error
    let standard_error = (variance / m).sqrt() * discount;

    PricingResult {
        price,
        standard_error,
        num_sims,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gbm_no_drift_no_vol() {
        let opt = LubrizolOption::new(100.0, 100.0, 1.0, 0.0, 0.0, OptionType::Call).unwrap();
        let epsilon = 1.5;
        let final_price = gbm(opt.s0, &opt, epsilon);

        //Price should stay the same
        let diff = (final_price - 100.0).abs();
        assert!(diff < 1e-10);
    }
    /*
     * DEPRECIATED: Test does not reflect the structure
     * of the structure of the project and function
    #[test]
    fn test_price_call_option_vs_benchmark() {
        let opt = LubrizolOption::new(100.0, 100.0, 1.0, 0.05, 0.2, OptionType::Call).unwrap();
        let num_sims = 1_000_000;

        let price = price_option(&opt, num_sims);

        let real = 10.4506;

        let diff = (price - real).abs();
        assert!(diff < 0.01);
    }
    */

    #[test]
    fn test_standard_error_convergence() {
        let opt = LubrizolOption {
            s0: 100.0,
            k: 100.0,
            t: 1.0,
            r: 0.05,
            sigma: 0.2,
            option_type: OptionType::Call,
        };

        let result_10k = price_option(&opt, 10_000);
        let result_40k = price_option(&opt, 40_000);

        let ratio = result_10k.standard_error / result_40k.standard_error;

        assert!(
            ratio > 1.8 && ratio < 2.2,
            "SE did not scale by ~2x, actual ratio: {}",
            ratio
        );

        let (lower, upper) = result_40k.confidence_interval_95();
        println!(
            "Price: {:.4}, 95% CI: [{:.4}, {:.4}]",
            result_40k.price, lower, upper
        );
    }
}
