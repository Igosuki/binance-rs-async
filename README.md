# binance-rs-async

Unofficial Rust Library for the [Binance API](https://github.com/binance-exchange/binance-official-api-docs)

This is a fully async api using [async-std](https://docs.rs/async-std/1.5.0/async_std/).
 
##

## Risk Warning

It is a personal project, use at your own risk. I will not be responsible for your investment losses.
Cryptocurrency investment is subject to high market risk.
Nonetheless, this crate is aimed at high performance and production use, I have been using this to target Binance successfully for several years now.

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
binance-rs-async = "1.0.0-beta.3"
```

## Rust >= 1.37

```rust
rustup install stable
```

## Contribution

Simply create a pull request. 
Properly documented code and tests (using binance testnet) are a must.
