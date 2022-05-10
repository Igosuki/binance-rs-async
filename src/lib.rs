//! [![github]](https://github.com/Igosuki/binance-rs-async)&ensp;[![crates-io]](https://crates.io/Igosuki/binance-rs-async)&ensp;[![docs-rs]](https://docs.rs/binance-rs-async)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! This library provides access to all of Binance's APIs using .
//! [`async/std`]: https://docs.rs/async-std/1.9.0/async_std/
//!
//! <br>
//!
//! # Risk Warning
//!
//! It is a personal project, use at your own risk. I will not be responsible for your investment losses.
//! Cryptocurrency investment is subject to high market risk.
//! Nonetheless, this crate is aimed at high performance and production use.
//!
//! # Example
//!
//! This example simply pings the main binance api
//!
//! ```rust
//! # use std::io;
//! use binance::general::General;
//! use binance::api::Binance;
//! use binance::errors::Error as BinanceLibError;
//!
//! #[tokio::main]
//! async fn main() -> std::io::Result<()> {
//!     let general: General = Binance::new(None, None);
//!     let ping = general.ping().await;
//!     match ping {
//!         Ok(answer) => println!("{:?}", answer),
//!         Err(err) => {
//!             match err {
//!                 BinanceLibError::BinanceError { response } => match response.code {
//!                     -1000_i16 => println!("An unknown error occured while processing the request"),
//!                     _ => println!("Unknown code {}: {}", response.code, response.msg),
//!                 },
//!                 _ => println!("Other errors: {}.", err),
//!             };
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//!
//! <br>
//!
//! # Details
//!
//! - Credentials are not enforced, you will get authentication errors if you don't provide
//! credentials and they are required by an endpoint
//!
//! - Error codes are handled on a best effort basis as some are inconsistent and not even
//! documented on Binance's side
//!
//! - Errors are implemented using [![thiserror]](https://docs.rs/thiserror/1.0.25/thiserror/)
//!

#![deny(unstable_features, unused_must_use, unused_mut, unused_imports, unused_import_braces)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
extern crate serde_qs as qs;

pub use util::{bool_to_string, bool_to_string_some};

mod client;
pub mod errors;
pub mod util;

pub mod account;
pub mod api;
pub mod config;
#[cfg(feature = "delivery_api")]
pub mod delivery;
#[cfg(feature = "futures_api")]
pub mod futures;
pub mod general;
#[cfg(feature = "margin_api")]
pub mod margin;
pub mod market;
pub mod rest_model;
#[cfg(feature = "savings_api")]
pub mod savings;
pub mod userstream;
pub mod websockets;
pub mod ws_model;
