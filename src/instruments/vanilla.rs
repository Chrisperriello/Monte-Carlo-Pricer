use crate::instruments::Payoff;
use crate::models::OptionType;

#[derive(Debug, Clone, Copy)]
struct VanillaOption {
    pub strike: f64,
    pub expiry: f64,
    pub option_type: OptionType,
}

impl Payoff for VanillaOption {
    fn payoff(&self, s_t: f64) -> f64 {
        match self.option_type {
            OptionType::Call => (s_t - self.strike).max(0.0),
            OptionType::Put => (self.strike - s_t).max(0.0),
        }
    }
}
