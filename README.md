# Monte Carlo Option Pricer

A high-performance Monte Carlo option pricer written in Rust, utilizing Geometric Brownian Motion (GBM) for path simulation and Rayon for parallel execution.

## Overview

This project provides a robust framework for pricing European call options using Monte Carlo methods. By simulating a large number of potential future price paths for an underlying asset, it estimates the fair value of an option based on its expected payoff.

## Key Features

- **Geometric Brownian Motion (GBM):** Accurately models stock price evolution using the standard stochastic differential equation.
- **Parallel Simulation:** Leverages [Rayon](https://github.com/rayon-rs/rayon) to distribute simulations across multiple CPU cores, significantly reducing computation time for large numbers of samples.
- **Type-Safe Implementation:** Built with Rust's strong typing to ensure mathematical correctness and memory safety.

## Mathematical Foundation

The price $S_t$ at time $t$ is modeled as:

$$S_t = S_0 \exp\left( \left(r - \frac{\sigma^2}{2}\right)t + \sigma \sqrt{t} \epsilon \right)$$

Where:
- $S_0$: Initial underlying price
- $r$: Risk-free interest rate
- $\sigma$: Volatility
- $t$: Time to expiry
- $\epsilon$: Standard normal random variable

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Edition 2024)
- Cargo

## Usage

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/monte_carlo_pricer.git
    cd monte_carlo_pricer
    ```

2.  **Run the simulation:**
    ```bash
    cargo run --release
    ```

## Example Output

```text
--- Monte Carlo Option Pricer ---
Simulations: 1000000
Estimated Price: $10.4523
```

## Dependencies

- `rand`: Random number generation.
- `rand_distr`: Normal distribution sampling.
- `rayon`: Data parallelism.

## Roadmap

- [ ] Support for Put Options.
- [ ] Calculation of Greeks (Delta, Gamma, Vega, etc.).
- [ ] Support for multiple time steps (Asian or American options).
- [ ] CLI arguments for option parameters.
