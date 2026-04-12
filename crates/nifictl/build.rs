use std::error::Error;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = std::env::var("OUT_DIR")?;
    let specs_dir = nifi_openapi_gen::specs_dir();
    let specs_dir = specs_dir
        .canonicalize()
        .map_err(|e| format!("Cannot find specs dir {}: {e}", specs_dir.display()))?;

    let config = nifi_openapi_gen::build_api::GenerateConfig::from_specs_dir(&specs_dir);

    nifi_openapi_gen::build_api::generate_cli(&specs_dir, Path::new(&out_dir), &config);

    println!("cargo::rerun-if-changed={}", specs_dir.display());
    println!("cargo::rerun-if-changed=build.rs");

    Ok(())
}
