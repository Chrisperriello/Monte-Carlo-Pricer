use clap::Parser;
use monte_carlo_pricer::models::{LubrizolOption, OptionType, price_delta, price_option};

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 100.0)]
    s0: f64,
    #[arg(short, long, default_value_t = 100.0)]
    strike: f64,
    #[arg(short, long, default_value_t = 1.0)]
    time: f64,
    #[arg(short, long, default_value_t = 0.05)]
    rate: f64,
    #[arg(short, long, default_value_t = 0.2)]
    vol: f64,
    #[arg(short, long, default_value = "call")]
    kind: String,
}
fn main() {
    let args = Args::parse();

    let opt_type;
    if args.kind.to_lowercase() == "put" {
        opt_type = OptionType::Put;
    } else {
        opt_type = OptionType::Call;
    }

    let result = LubrizolOption::new(
        args.s0,
        args.strike,
        args.time,
        args.rate,
        args.vol,
        opt_type,
    );
    match result {
        Ok(opt) => {
            let price = price_option(&opt, 1_000_000);
            let delta = price_delta(&opt, 1_000_000, 0.01);

            println!("--- Results for {} ---", args.kind.to_uppercase());
            println!("Price: ${:.4}", price);
            println!("Delta: {:.4}", delta);
        }
        Err(e) => {
            // Instead of crashing, we print a clean error message
            println!("Error: {:?}", e);
        }
    }
}
