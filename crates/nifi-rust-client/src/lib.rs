// `has_any_version` is a rustc-cfg emitted by build.rs whenever it runs
// successfully with at least one NiFi version feature enabled (or the
// `dynamic` feature, which pulls in all versions). The flag is invisible
// to users — it isn't a Cargo feature and can't be set externally. If
// build.rs is ever bypassed entirely (some `cargo doc` / rust-analyzer
// configurations), the flag is unset and this compile_error! fires with
// an actionable message. The primary zero-features guard is the runtime
// check in build.rs itself; this is defence in depth.
#[cfg(not(has_any_version))]
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
