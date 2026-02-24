use rand_distr::Distribution;
use rand_distr::Normal;
use rayon::prelude::*;

struct LubrizolOption {
    s0: f64,    //Underlying price
    k: f64,     //Strike price
    t: f64,     // Time to epiry (in years)
    r: f64,     //Risk-free rate
    sigma: f64, // Volatitlity
}

fn gbm(opt: &LubrizolOption, epsilon: f64) -> f64 {
    opt.s0 * (opt.r - ((opt.sigma.powi(2)) / 2.0)) * opt.t
        + (opt.sigma * opt.t.sqrt() * epsilon).exp()
}

fn price_call_option(opt: &LubrizolOption, num_sims: u64) -> f64 {
    //let mut running_sum = 0.0;

    let mut rng = rand::thread_rng();

    let normal = Normal::new(0.0, 1.0).unwrap();

    let total_sum: f64 = (0..num_sims)
        .into_par_iter()
        .map(|_| {
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

    let average_payoff = running_sum / (num_sims as f64);
    average_payoff * (-opt.r * opt.t).exp()
}

fn main() {
    println!("Hello, world!");
}
