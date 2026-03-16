use monte_carlo_pricer::models::OptionType;

use crate::instruments::Payoff;

#[derive(Debug, Clone, Copy)]
struct VanillaOption {
    pub strike: f64,
    pub expiry: f64,
    pub option_type: OptionType,
}

impl VanillaOption for Payoff {
    fn payoff(&self, s_t: f64) -> f64 {
        match self.option_type {
            OptionType::Call => (s_t - self.strike).max(0.0),
            Option::Put => (self.strike - s_t).max(0.0),
        }
    }
}
