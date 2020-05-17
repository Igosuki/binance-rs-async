#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces
)]

#[macro_use]
extern crate error_chain;

extern crate hex;
extern crate reqwest;
extern crate ring;
extern crate serde;
extern crate serde_json;

extern crate tungstenite;
extern crate url;

#[macro_use]
extern crate serde_derive;

extern crate serde_qs as qs;

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
