# Monte Carlo Option Pricer

A high-performance Monte Carlo option pricer written in Rust, utilizing Geometric Brownian Motion (GBM) for path simulation and Rayon for parallel execution.

## Overview

This project provides a robust framework for pricing European call options using Monte Carlo methods. By simulating a large number of potential future price paths for an underlying asset, it estimates the fair value of an option based on its expected payoff.

## Key Features

- **Geometric Brownian Motion (GBM):** Models stock price evolution using standard stochastic differential equations.
- **Parallel Simulation:** Leverages [Rayon](https://github.com/rayon-rs/rayon) for data parallelism, distributing simulations across all available CPU cores.
- **Greeks Calculation:** Supports calculation of **Delta** via finite difference methods within the Monte Carlo framework.
- **Robust Error Handling:** Includes validation for option parameters (e.g., ensuring positive volatility and time to expiry).
- **Type-Safe Implementation:** Built with Rust 2024 for memory safety and high performance.

## Mathematical Foundation

The price $S_t$ at time $t$ is modeled as:

$$S_t = S_0 \exp\left( \left(r - \frac{\sigma^2}{2}\right)t + \sigma \sqrt{t} \epsilon \right)$$

Where:
- $S_0$: Initial underlying price
- $r$: Risk-free interest rate
- $\sigma$: Volatility
- $t$: Time to expiry
- $\epsilon$: Standard normal random variable $\sim N(0,1)$

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Edition 2024)
- Cargo

## Usage

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/chrisperriello/monte_carlo_pricer.git
    cd monte_carlo_pricer
    ```

2.  **Run the simulation:**
    ```bash
    cargo run --release
    ```

## Example Output

```text
Price: $10.4506
Delta: 0.6124
```

## Project Structure

- `src/main.rs`: Entry point and example usage.
- `src/lib.rs`: Library interface.
- `src/models/mod.rs`: Core logic for `LubrizolOption`, GBM simulation, and pricing functions.

## Dependencies

- `rand`: Random number generation.
- `rand_distr`: Normal distribution sampling.
- `rayon`: High-performance data parallelism.

## Roadmap

- [ ] Support for Put Options.
- [ ] Calculation of additional Greeks (Gamma, Vega, Theta).
- [ ] Support for multiple time steps (Asian or American options).
- [ ] CLI arguments for dynamic option parameter input.
