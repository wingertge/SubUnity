#[allow(unused_imports)] // This is actually used, the analyzer is just a tad buggy
#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate serde;

mod proto;

pub use proto::*;
use crate::proto::subtitles::Chunk;

impl AsRef<[u8]> for Chunk {
    fn as_ref(&self) -> &[u8] {
        &self.content
    }
}