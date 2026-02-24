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
}

impl LubrizolOption {
    pub fn new(s0: f64, k: f64, t: f64, r: f64, sigma: f64) -> Result<Self, PricerError> {
        if sigma <= 0.0 {
            return Err(PricerError::InvalidInput(
                "Volatility must be positive".into(),
            ));
        }

        if t <= 0.0 {
            return Err(PricerError::InvalidInput(
                "Time to expiry must be positive".into(),
            ));
        }
        Ok(Self { s0, k, t, r, sigma })
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

            //
            //Calulate the payoff
            (st - opt.k).max(0.0)
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

pub fn price_call_delta(opt: &LubrizolOption, num_sims: u64) -> f64 {
    let normal = Normal::new(0.0, 1.0).unwrap();

    let total_sum: f64 = (0..num_sims)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let epsilon: f64 = normal.sample(&mut rng);

            //Calculate the final price using GBM formula
            let st_base = gbm(opt.s0, &opt, epsilon);
            //Calulate the payoff for S0
            let payoff_base = (st_base - opt.k).max(0.0);

            //Calculate price at S0 + bump
            let st_bump = gbm(opt.s0 + 1.0, opt, epsilon);
            let payoff_bumped = (st_bump - opt.k).max(0.0);

            payoff_bumped - payoff_base
        })
        .sum();

    let avg_delta = total_sum / (num_sims as f64);
    avg_delta * (-opt.r * opt.t).exp()
}
