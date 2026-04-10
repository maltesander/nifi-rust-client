use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = std::env::var("OUT_DIR")?;
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;

    // The specs ship inside the nifi-openapi-gen crate. The helper resolves
    // them relative to nifi-openapi-gen's own manifest dir, which works whether
    // that crate is consumed via path dep (workspace dev build) or pulled from
    // the registry (downstream consumers, `cargo package` verify build).
    let specs_dir = nifi_openapi_gen::specs_dir();
    let specs_dir = specs_dir
        .canonicalize()
        .map_err(|e| format!("Cannot find specs dir {}: {e}", specs_dir.display()))?;

    let config = nifi_openapi_gen::build_api::GenerateConfig::from_cargo_env();

    // Guard against the "no version feature enabled" configuration. This can
    // only happen with --no-default-features and no explicit nifi-x-y-z or
    // `dynamic` feature. Without this guard the build silently produces an
    // empty API surface. The same guard is enforced statically in lib.rs via
    // compile_error!; this build-side check gives an earlier, clearer message.
    if config.versions.is_empty() {
        return Err(
            "No NiFi version feature enabled. Enable one of `nifi-2-6-0`, \
             `nifi-2-7-2`, `nifi-2-8-0`, or the `dynamic` feature in your \
             Cargo.toml."
                .into(),
        );
    }

    nifi_openapi_gen::build_api::generate_client(&specs_dir, Path::new(&out_dir), &config);

    // Copy hand-written strategy.rs into $OUT_DIR/dynamic/ so the generated
    // dynamic/mod.rs can find it via `pub mod strategy;`.
    if config.dynamic {
        let strategy_src = Path::new(&manifest_dir).join("src/dynamic/strategy.rs");
        let strategy_dst = Path::new(&out_dir).join("dynamic/strategy.rs");
        if strategy_src.exists() {
            std::fs::copy(&strategy_src, &strategy_dst).map_err(|e| {
                format!(
                    "Failed to copy strategy.rs from {} to {}: {e}",
                    strategy_src.display(),
                    strategy_dst.display()
                )
            })?;
        }
    }

    println!("cargo::rerun-if-changed={}", specs_dir.display());
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/dynamic/strategy.rs");

    Ok(())
}
