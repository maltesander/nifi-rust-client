#![cfg(feature = "dynamic")]

mod helpers;

include!(concat!(env!("OUT_DIR"), "/dynamic_enum_coverage.rs"));
include!(concat!(
    env!("OUT_DIR"),
    "/dynamic_endpoint_availability.rs"
));
include!(concat!(env!("OUT_DIR"), "/dynamic_field_presence.rs"));
include!(concat!(env!("OUT_DIR"), "/dynamic_query_param_coverage.rs"));
