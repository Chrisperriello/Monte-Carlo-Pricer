use clap::Parser;
use monte_carlo_pricer::core::market_data::MarketData;
use monte_carlo_pricer::engines::mc::{price_delta, price_mc};
use monte_carlo_pricer::instruments::OptionType;
use monte_carlo_pricer::instruments::vanilla::VanillaOption;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 100.0)]
    s0: f64,
    #[arg(short = 'k', long, default_value_t = 100.0)]
    strike: f64,
    #[arg(short, long, default_value_t = 1.0)]
    time: f64,
    #[arg(short, long, default_value_t = 0.05)]
    rate: f64,
    #[arg(short, long, default_value_t = 0.2)]
    vol: f64,
    #[arg(short = 'o', long, default_value = "call")]
    kind: String,
}
fn main() {
    let args = Args::parse();
    let option = VanillaOption::new(
        args.strike,
        args.time,
        if args.kind == "call" {
            OptionType::Call
        } else {
            OptionType::Put
        },
    );

    let env = MarketData::new(args.s0, args.rate, args.vol).unwrap();

    let price = price_mc(&option, &env, 1_000_000);
    let delta = price_delta(&option, &env, 1_000_000, 0.01).unwrap();

    println!("--- Results for {} ---", args.kind.to_uppercase());
    println!(
        "Price: {:.4}, Low: {:.4}, High {:.4}",
        price.price,
        (price.price - price.standard_error),
        (price.price + price.standard_error)
    );
    println!(
        "Delta: {:.4}, Low: {:.4}, High {:.4}",
        delta.price,
        (delta.price - delta.standard_error),
        (delta.price + delta.standard_error)
    );

    /*
    match result {
        Ok(opt) => {
            // let price = price_option(&opt, 1_000_000);
            //let delta = price_delta(&opt, 1_000_000, 0.01);

            println!("--- Results for {} ---", args.kind.to_uppercase());
            /*    println!(
                "Price: {:.4}, Low: {:.4}, High {:.4}",
                price.price,
                (price.price - price.standard_error),
                (price.price + price.standard_error)
            );
            println!(
                "Delta: {:.4}, Low: {:.4}, High {:.4}",
                delta.price,
                (delta.price - delta.standard_error),
                (delta.price + delta.standard_error)
            );
            */
        }
        Err(e) => {
            // Instead of crashing, we print a clean error message
            println!("Error: {:?}", e);
        }
    }
     */
}
