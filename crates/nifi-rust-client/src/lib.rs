// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

pub mod builder;
pub mod client;
pub mod error;
pub use builder::NifiClientBuilder;
pub use client::NifiClient;
pub use error::NifiError;

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
compile_error!("Enable exactly one NiFi version feature, e.g. features = [\"nifi-2-8-0\"]");

#[cfg(all(feature = "nifi-2-7-2", feature = "nifi-2-8-0"))]
compile_error!("Only one NiFi version feature may be enabled at a time");

#[cfg(feature = "nifi-2-7-2")]
mod v2_7_2;
#[cfg(all(feature = "nifi-2-7-2", not(feature = "nifi-2-8-0")))]
pub use v2_7_2::{api, types};

#[cfg(feature = "nifi-2-8-0")]
mod v2_8_0;
#[cfg(all(feature = "nifi-2-8-0", not(feature = "nifi-2-7-2")))]
pub use v2_8_0::{api, types};
