// Internal `__any_version` marker — enabled transitively by every
// `nifi-x-y-z` feature via Cargo.toml. When `--no-default-features` is
// used without an explicit version (or `dynamic`) feature, none of the
// `nifi-x-y-z` features are active, so `__any_version` is also off, and
// this compile_error! fires. The generator's cargo_features patcher
// maintains the dependency so adding a new NiFi version requires no
// manual update here.
#[cfg(not(feature = "__any_version"))]
compile_error!(
    "nifi-rust-client requires at least one version feature to be enabled. \
     Enable one of `nifi-2-6-0`, `nifi-2-7-2`, `nifi-2-8-0`, or the `dynamic` \
     feature in your Cargo.toml."
);

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
