#![allow(clippy::panic)]

use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    // The specs ship inside the nifi-openapi-gen crate. The helper resolves
    // them relative to nifi-openapi-gen's own manifest dir, which works whether
    // that crate is consumed via path dep (workspace dev build) or pulled from
    // the registry (downstream consumers, `cargo package` verify build).
    let specs_dir = nifi_openapi_gen::specs_dir();
    let specs_dir = specs_dir
        .canonicalize()
        .unwrap_or_else(|e| panic!("Cannot find specs dir {}: {e}", specs_dir.display()));

    let config = nifi_openapi_gen::build_api::GenerateConfig::from_cargo_env();
    nifi_openapi_gen::build_api::generate_client(&specs_dir, Path::new(&out_dir), &config);

    // Copy hand-written strategy.rs into $OUT_DIR/dynamic/ so the generated
    // dynamic/mod.rs can find it via `pub mod strategy;`.
    if config.dynamic {
        let strategy_src = Path::new(&manifest_dir).join("src/dynamic/strategy.rs");
        let strategy_dst = Path::new(&out_dir).join("dynamic/strategy.rs");
        if strategy_src.exists() {
            std::fs::copy(&strategy_src, &strategy_dst).unwrap_or_else(|e| {
                panic!(
                    "Failed to copy strategy.rs from {} to {}: {e}",
                    strategy_src.display(),
                    strategy_dst.display()
                )
            });
        }
    }

    println!("cargo::rerun-if-changed={}", specs_dir.display());
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/dynamic/strategy.rs");
}
