use std::num::IntErrorKind;

use crate::core::MarketData;
use crate::instruments::Payoff;
use crate::models::PricingResult;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;

fn gbm(start_price: f64, r: f64, sigma: f64, t: f64, epsilon: f64) -> f64 {
    start_price * ((r - ((sigma.powi(2)) / 2.0)) * t + (sigma * t.sqrt() * epsilon)).exp()
}

pub fn price_mc<I: Payoff + Sync>(
    instrument: &I,
    env: &MarketData,
    num_sims: u64,
) -> PricingResult {
    let normal = Normal::new(0.0, 1.0).unwrap();

    let (sum_y, sum_sq_y): (f64, f64) = (0..num_sims)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let epsilon: f64 = normal.sample(&mut rng);

            //Calculate the final price using GBM formula
            let st = gbm(env.s0, env.r, env.sigma, instrument.expiry(), epsilon);

            let payoff_pos = instrument.payoff(st);

            let st_neg = gbm(env.s0, env.r, env.sigma, instrument.expiry(), -epsilon);
            let payoff_neg = instrument.payoff(st_neg);
            let y = (payoff_pos + payoff_neg) / 2.0;

            //Return (y, y^2)
            (y, y * y)
        })
        .reduce(|| (0.0, 0.0), |a, b| (a.0 + b.0, a.1 + b.1));

    let m = num_sims as f64;
    let discount = (-env.r * instrument.expiry()).exp();

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

pub fn price_delta<I: Payoff + Sync>(
    instrument: &I,
    env: &MarketData,
    num_sims: u64,
    bump: f64,
) -> PricingResult {
    let normal = Normal::new(0.0, 1.0).unwrap();

    let (sum_y, sum_squr_y): (f64, f64) = (0..num_sims)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let epsilon: f64 = normal.sample(&mut rng);

            //Calculate the final price using GBM formula
            // to deal with Antithetic Vaiates we average the
            // positive and negative paths
            let st_base_pos = gbm(env.s0, env.r, env.sigma, instrument.expiry(), epsilon);
            //Calulate the payoff for S0
            let payoff_base = instrument.payoff(st_base_pos);

            let st_bump_pos = gbm(
                env.s0 + bump,
                env.r,
                env.sigma,
                instrument.expiry(),
                epsilon,
            );
            let payoff_bumped_pos = instrument.payoff(st_bump_pos);

            let payoff_pos = (payoff_bumped_pos - payoff_base) / bump;

            //Now for the negative
            let st_base_neg = gbm(env.s0, env.r, env.sigma, instrument.expiry(), -epsilon);
            //Calulate the payoff for S0
            let payoff_base_neg = instrument.payoff(st_base_neg);

            let st_bump_neg = gbm(
                env.s0 + bump,
                env.r,
                env.sigma,
                instrument.expiry(),
                -epsilon,
            );
            let payoff_bumped_neg = instrument.payoff(st_bump_neg);
            let payoff_neg = (payoff_bumped_neg - payoff_base_neg) / bump;

            let y = (payoff_pos + payoff_neg) / 2.0;

            (y, y * y)
        })
        .reduce(|| (0.0, 0.0), |a, b| (a.0 + b.0, a.1 + b.1));
    let m = num_sims as f64;
    let avg_delta = sum_y / (m);
    let discount = (-env.r * instrument.expiry()).exp();
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
