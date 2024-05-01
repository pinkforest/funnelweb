#![warn(
    clippy::unwrap_used,
//    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

use thingbuf::StaticThingBuf;

mod command;

pub use command::{CommandType, ImapRequestItem, ImapResponseItem};

#[derive(Debug)]
pub enum ImapItem {
    Request(ImapRequestItem),
    Response(ImapResponseItem),
}

#[cfg(test)]
mod test {
    use super::*;
    use insta::assert_debug_snapshot;

    /*
        #[test]
        #[cfg(feature = "std")]
        fn basic() {
            assert_debug_snapshot!(Config::from_file("test_data/basic.toml"));
    }
        */
}
