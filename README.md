# binance-rs-async

Unofficial Rust Library for the [Binance API](https://github.com/binance-exchange/binance-official-api-docs)

This is a fully async api using [tokio](https://tokio.rs/).

## Current state

The current beta aims at implementing every single endpoint on the binance docs. Currently, futures and savings have
been implemented but not thoroughly tested.

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
binance-rs-async = "1.1.11"
```

## Roadmap

- 1.0.0 Completely tested margin sapi endpoints
- 1.0.* Changelog check to detect binance API changes
- 1.1.* Wallet API
- 1.2.* Complete tested futures (m-coin and usd-m futures)

## Risk Warning

It is a personal project, use at your own risk. I will not be responsible for your investment losses. Cryptocurrency
investment is subject to high market risk. Nonetheless, this crate is aimed at high performance and production use, I
have been using this to target Binance successfully for several years now.

### Using TLS

By default, the crate uses `native-tls` for tungstenite and reqwest because I believe it's simpler and faster to let the
user switch LibreSSL or OpenSSL versions rather than rebuild the program.

You can however disable default-features and use `rust-tls`, which might be helpful in certain situations such as CI or
dev box.

## Rust >= 1.37

```rust
rustup install stable
```

## Contribution

Simply create a pull request. Properly documented code and tests (using binance testnet) are a must.
