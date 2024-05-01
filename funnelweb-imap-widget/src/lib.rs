#![warn(
    clippy::unwrap_used,
    //    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

use std::time::SystemTime;

mod connection;
mod connections;

pub use connection::{ImapConnection, ImapConnectionState};
pub use connections::{ImapConnections, ImapConnectionsWidget, SelectedImapConnection};
