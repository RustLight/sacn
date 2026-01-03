# STM32F746G-DISC Demo in Rust

This is an example sACN receiver that runs on an STM32F746G-DISCO development board.
It awaits a discovery packet and then joins the multicast groups for the discovered
universes. It was based on [this demo crate](https://github.com/ProfFan/f7disco-rust-demo)

This example uses Embassy [`embassy-rs`](https://embassy.dev/) as the async
framework and network stack and [`probe-rs`](https://probe.rs/) as runner.
Please refer to their respective documentation for instructions on how to install
the necessary tools.

## Building

Change your working directory to the example's root directory.

```sh
cd examples/stm32f7/
```

Build or run using cargo.

```sh
cargo build -r
cargo run -r
```
