#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces
)]

extern crate hex;
extern crate reqwest;
extern crate ring;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate serde_qs as qs;
extern crate tungstenite;
extern crate url;
extern crate thiserror;

mod client;
pub mod errors;
mod util;

pub mod account;
pub mod api;
pub mod general;
pub mod margin;
pub mod market;
pub mod rest_model;
pub mod userstream;
pub mod websockets;
pub mod ws_model;

pub use client::TEST_SPOT_API1_HOST;
