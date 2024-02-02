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
binance-rs-async = "1.3.3"
```

## Roadmap

- 1.0.0 Completely tested margin sapi endpoints
- 1.0.* Changelog check to detect binance API changes
- 1.1.* Wallet API
- 1.2.* to 1.3.* Continuous updates for wallet and margin APIs
- 1.4.* Complete tested futures (m-coin and usd-m futures)

## Breaking changes
- 1.3.0 introduces optional fields for MarginOrderCancellationResult instead of definitely set fields

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

```shell
rustup install stable
```

## Cargo dependencies

```shell
cargo install cargo-semver-checks
```

## Checking documentation and release

```shell
cargo semver-checks check-release --package binance-rs-async --verbose
```

## Contribution

Simply create a pull request. Properly documented code and tests (using binance testnet) are a must.

## Running github actions locally : 

```shell
# Example that runs the make-release-pr workflow 
act -P ubuntu-latest=cimg/rust:1.66.1-node workflow_dispatch -j make-release-pr --eventpath release.json --secret-file=secrets
```
