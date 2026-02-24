use models::LubrizolOption;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;
pub mod models;

pub fn price_call_option(opt: &LubrizolOption, num_sims: u64) -> f64 {
    //let mut running_sum = 0.0;

    let normal = Normal::new(0.0, 1.0).unwrap();

    let total_sum: f64 = (0..num_sims)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let epsilon: f64 = normal.sample(&mut rng);

            //Calculate the final price using GBM formula
            let st = gbm(&opt, epsilon);

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

fn gbm(opt: &LubrizolOption, epsilon: f64) -> f64 {
    opt.s0
        * ((opt.r - ((opt.sigma.powi(2)) / 2.0)) * opt.t + (opt.sigma * opt.t.sqrt() * epsilon))
            .exp()
}
