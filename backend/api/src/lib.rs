#[macro_use]
extern crate log;

#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate validify;

#[cfg(test)]
#[macro_use]
extern crate adtest;

mod api;
pub mod jobs;

pub use api::create_api;
pub use jobs::setup as setup_jobs;
