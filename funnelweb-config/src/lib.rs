#![warn(
    clippy::unwrap_used,
//    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![doc = include_str!("../README.md")]

use serde::Deserialize;
#[cfg(feature = "std")]
use std::{fs::File, io::prelude::*};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub imap: Vec<ImapConfig>,
    pub storage: Option<StorageConfig>,
    pub bonsaidb: Option<BonsaiDbConfig>,
    pub tantivy: Option<TantivyConfig>,
}

#[derive(Debug)]
pub enum ConfigError {
    FileOpen(String),
    Parse(String),
    Read(String),
}

impl Config {
    #[cfg(feature = "std")]
    pub fn from_file(file_location: &str) -> Result<Self, ConfigError> {
        let mut file =
            File::open(file_location).map_err(|e| ConfigError::FileOpen(e.to_string()))?;
        let mut data = String::new();
        file.read_to_string(&mut data)
            .map_err(|e| ConfigError::Read(e.to_string()))?;
        let config: Self = toml::from_str(&data).map_err(|e| ConfigError::Parse(e.to_string()))?;
        Ok(config)
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ImapConfig {
    pub name: Option<String>,
    pub host: String,
    pub port: Option<u32>,
    pub tls: Option<bool>,
    pub ca: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[allow(non_camel_case_types)]
pub enum StorageBackendChoice {
    bonsaidb,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct StorageConfig {
    pub backend: StorageBackendChoice,
}

#[derive(Debug, Deserialize, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IndexBackendChoice {
    tantivy,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct IndexConfig {
    pub backend: IndexBackendChoice,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct BonsaiDbConfig {
    pub file: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TantivyConfig {
    pub directory: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    #[cfg(feature = "std")]
    fn basic() {
        assert_debug_snapshot!(Config::from_file("test_data/basic.toml"));
    }
}
