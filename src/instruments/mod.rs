pub mod vanilla;

pub trait Payoff {
    fn payoff(&self, s_t: f64) -> f64;
    fn expiry(&self) -> f64;
}

#[derive(Debug, Clone, Copy)]
pub enum OptionType {
    Call,
    Put,
}
