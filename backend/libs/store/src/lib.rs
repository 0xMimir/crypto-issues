#[macro_use]
extern crate serde;

#[macro_use]
extern crate utoipa;

mod migrations;
pub mod objects;

pub use migrations::*;
