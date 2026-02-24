use monte_carlo_pricer::models::LubrizolOption;
use monte_carlo_pricer::models::price_call_delta;
use monte_carlo_pricer::models::price_call_option;
fn main() {
    let result = LubrizolOption::new(100.0, 100.0, 0.1, 0.05, 0.2);

    match result {
        Ok(opt) => {
            let price = price_call_option(&opt, 1_000_000);
            let delta = price_call_delta(&opt, 1_000_000);

            println!("Price: ${:.4}", price);
            println!("Delta: {:.4}", delta);
        }
        Err(e) => {
            // Instead of crashing, we print a clean error message
            println!("Error: {:?}", e);
        }
    }
}
