#[derive(Debug, Clone, Copy)]
pub struct MarketData {
    pub s0: f64,
    pub r: f64,
    pub sigma: f64,
}

impl MarketData {
    pub fn new(s0: f64, r: f64, sigma: f64) -> Result<Self, PricerError> {
        if sigma < 0.0 {
            return Err(PricerError::InvalidInput(
                "Volatility must be positive".into(),
            ));
        }

        Ok(Self { s0, r, sigma })
    }
}

#[derive(Debug)]
pub enum PricerError {
    InvalidInput(String),
}
