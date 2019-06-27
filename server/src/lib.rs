#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate failure;
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate log;

extern crate multi_party_ed25519;

mod routes;
mod storage;
pub mod api;

type Result<T> = std::result::Result<T, failure::Error>;