use rand_distr::{Distribution, Normal};
use rayon::prelude::*;

#[derive(Debug)]
pub enum PricerError {
    InvalidInput(String),
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

pub fn price_call_option(opt: &LubrizolOption, num_sims: u64) -> f64 {
    //let mut running_sum = 0.0;

    let normal = Normal::new(0.0, 1.0).unwrap();

    let total_sum: f64 = (0..num_sims)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let epsilon: f64 = normal.sample(&mut rng);

            //Calculate the final price using GBM formula
            let st = gbm(opt.s0, &opt, epsilon);

            //Calulate the payoff
            match opt.option_type {
                OptionType::Call => (st - opt.k).max(0.0),
                OptionType::Put => (opt.k - st).max(0.0),
            }
        })
        .sum();
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

    let average_payoff = total_sum / (num_sims as f64);
    average_payoff * (-opt.r * opt.t).exp()
}

pub fn price_call_delta(opt: &LubrizolOption, num_sims: u64, bump: f64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();

    let total_sum: f64 = (0..num_sims)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let epsilon: f64 = normal.sample(&mut rng);

            //Calculate the final price using GBM formula
            let st_base = gbm(opt.s0, &opt, epsilon);
            //Calulate the payoff for S0
            let payoff_base;

            //Calculate price at S0 + bump
            //Calulate the payoff
            match opt.option_type {
                OptionType::Call => payoff_base = (st_base - opt.k).max(0.0),
                OptionType::Put => payoff_base = (opt.k - st_base).max(0.0),
            }

            let st_bump = gbm(opt.s0 + bump, opt, epsilon);
            let payoff_bumped;

            match opt.option_type {
                OptionType::Call => payoff_bumped = (st_bump - opt.k).max(0.0),
                OptionType::Put => payoff_bumped = (opt.k - st_bump).max(0.0),
            }
            (payoff_bumped - payoff_base) / bump
        })
        .sum();

    let avg_delta = total_sum / (num_sims as f64);
    avg_delta * (-opt.r * opt.t).exp()
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

    #[test]
    fn test_price_call_option_vs_benchmark() {
        let opt = LubrizolOption::new(100.0, 100.0, 1.0, 0.05, 0.2, OptionType::Call).unwrap();
        let num_sims = 1_000_000;

        let price = price_call_option(&opt, num_sims);

        let real = 10.4506;

        let diff = (price - real).abs();
        assert!(diff < 0.01);
    }
}
