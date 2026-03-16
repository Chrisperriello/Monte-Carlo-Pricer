pub mod vanilla;

pub trait Payoff {
    fn payoff(&self, s_t: f64) -> f64;
}
