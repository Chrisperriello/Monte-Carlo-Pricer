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

    pub fn validate_delta(&self, bump: f64) -> Result<(), String> {
        if bump > self.s0 * 0.5 {
            return Err(
                "Bump is too big, must be less than or equal to half of the spot price".into(),
            );
        }

        if bump < 1e-12 {
            return Err("Bump is too small".into());
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum PricerError {
    InvalidInput(String),
}
