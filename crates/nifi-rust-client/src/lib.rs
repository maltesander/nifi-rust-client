// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

pub mod builder;
pub mod client;
pub mod error;
pub use builder::NifiClientBuilder;
pub use client::NifiClient;
pub use error::NifiError;

#[cfg(not(any(feature = "nifi-2-6-0", feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
compile_error!("Enable at least one NiFi version feature, e.g. features = [\"nifi-2-8-0\"]");

#[cfg(all(
    feature = "nifi-2-6-0",
    any(feature = "nifi-2-7-2", feature = "nifi-2-8-0"),
    not(feature = "dynamic")
))]
compile_error!("Multiple NiFi versions require the \"dynamic\" feature");

#[cfg(all(
    feature = "nifi-2-7-2",
    any(feature = "nifi-2-6-0", feature = "nifi-2-8-0"),
    not(feature = "dynamic")
))]
compile_error!("Multiple NiFi versions require the \"dynamic\" feature");

#[cfg(all(
    feature = "nifi-2-8-0",
    any(feature = "nifi-2-6-0", feature = "nifi-2-7-2"),
    not(feature = "dynamic")
))]
compile_error!("Multiple NiFi versions require the \"dynamic\" feature");

#[cfg(feature = "nifi-2-6-0")]
pub mod v2_6_0;
#[cfg(feature = "nifi-2-7-2")]
pub mod v2_7_2;
#[cfg(feature = "nifi-2-8-0")]
pub mod v2_8_0;

#[cfg(all(
    feature = "nifi-2-6-0",
    not(feature = "dynamic"),
    not(feature = "nifi-2-7-2"),
    not(feature = "nifi-2-8-0")
))]
pub use v2_6_0::{api, types};

#[cfg(all(
    feature = "nifi-2-7-2",
    not(feature = "dynamic"),
    not(feature = "nifi-2-6-0"),
    not(feature = "nifi-2-8-0")
))]
pub use v2_7_2::{api, types};

#[cfg(all(
    feature = "nifi-2-8-0",
    not(feature = "dynamic"),
    not(feature = "nifi-2-6-0"),
    not(feature = "nifi-2-7-2")
))]
pub use v2_8_0::{api, types};

#[cfg(feature = "dynamic")]
pub mod dynamic;
