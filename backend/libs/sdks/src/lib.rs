#![feature(iterator_try_collect)]

#[macro_use]
extern crate serde;

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate log;

pub mod coingecko;
pub mod github;
