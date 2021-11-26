#![deny(warnings)]
#![deny(rust_2018_idioms)]

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate serde;

#[macro_use]
pub mod result;

pub mod cli;
pub mod git;
pub mod helpers;
pub mod paket;
pub mod pkg;

pub use crate::paket::*;
pub use crate::result::*;
