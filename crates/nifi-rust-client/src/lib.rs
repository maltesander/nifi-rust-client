pub mod builder;
pub mod client;
pub mod config;
pub mod error;
pub use builder::NifiClientBuilder;
pub use client::NifiClient;
pub use config::credentials::CredentialProvider;
pub use error::NifiError;

// Generated: version modules, re-exports, dynamic module
include!(concat!(env!("OUT_DIR"), "/generated_lib.rs"));
