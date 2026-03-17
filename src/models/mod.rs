#[derive(Debug, Clone, Copy)]
pub struct PricingResult {
    pub price: f64,
    pub standard_error: f64,
    pub num_sims: u64,
}

impl PricingResult {
    pub fn confidence_interval_95(&self) -> (f64, f64) {
        let margin = 1.96 * self.standard_error;
        (self.price - margin, self.price + margin)
    }
}
