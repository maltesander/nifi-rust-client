#![allow(clippy::panic)]

use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    let specs_dir = Path::new(&manifest_dir).join("../crates/nifi-openapi-gen/specs");
    let specs_dir = specs_dir
        .canonicalize()
        .unwrap_or_else(|e| panic!("Cannot find specs dir {}: {e}", specs_dir.display()));

    let config = nifi_openapi_gen::build_api::GenerateConfig::from_specs_dir(&specs_dir);
    nifi_openapi_gen::build_api::generate_integration_tests(
        &specs_dir,
        Path::new(&out_dir),
        &config,
    );

    println!("cargo::rerun-if-changed=../crates/nifi-openapi-gen/specs");
    println!("cargo::rerun-if-changed=../crates/nifi-openapi-gen/src");
    println!("cargo::rerun-if-changed=build.rs");
}
