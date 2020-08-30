#[allow(unused_imports)] // This is actually used, the analyzer is just a tad buggy
#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate serde;

mod proto;

pub use proto::*;
