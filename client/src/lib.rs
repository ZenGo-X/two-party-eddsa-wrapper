extern crate reqwest;

#[macro_use]
extern crate failure;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate time;

extern crate multi_party_ed25519;

pub mod api;
mod utilities;

pub type Result<T> = std::result::Result<T, failure::Error>;
