//! Standalone OpenAPI spec diff tool.
//!
//! Usage:
//!   cargo run --bin openapi-diff --features cli -- compare old.json new.json
//!   cargo run --bin openapi-diff --features cli -- summary spec.json

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "openapi-diff",
    about = "Compare OpenAPI specs and report changes"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compare two OpenAPI spec files and report differences
    Compare {
        /// Path to the older (baseline) spec
        old: PathBuf,
        /// Path to the newer spec
        new: PathBuf,
        /// Output format
        #[arg(long, default_value = "markdown")]
        format: OutputFormat,
    },
    /// Show a summary of endpoints and types in a spec
    Summary {
        /// Path to the spec file
        spec: PathBuf,
        /// Output format
        #[arg(long, default_value = "markdown")]
        format: OutputFormat,
    },
}

#[derive(Clone, clap::ValueEnum)]
enum OutputFormat {
    Markdown,
    Json,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Compare { old, new, format } => {
            let old_spec = nifi_openapi_gen::parser::load(old.to_str().expect("UTF-8 path"));
            let new_spec = nifi_openapi_gen::parser::load(new.to_str().expect("UTF-8 path"));

            let old_version = old
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("old");
            let new_version = new
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("new");

            let diff = nifi_openapi_gen::diff::compute_diff(
                &old_spec,
                &new_spec,
                old_version,
                new_version,
            );

            match format {
                OutputFormat::Markdown => {
                    println!("## {} -> {}\n", old_version, new_version);
                    println!("{}", nifi_openapi_gen::docs::format_diff_body(&diff));
                }
                OutputFormat::Json => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&diff).expect("serialize diff")
                    );
                }
            }
        }
        Commands::Summary { spec, format } => {
            let parsed = nifi_openapi_gen::parser::load(spec.to_str().expect("UTF-8 path"));
            let endpoint_count: usize = parsed
                .tags
                .iter()
                .map(|t| {
                    t.root_endpoints.len()
                        + t.sub_groups
                            .iter()
                            .map(|sg| sg.endpoints.len())
                            .sum::<usize>()
                })
                .sum();
            let type_count = parsed.all_types.len();
            let tags: Vec<&str> = parsed.tags.iter().map(|t| t.tag.as_str()).collect();

            match format {
                OutputFormat::Markdown => {
                    println!("# Spec Summary\n");
                    println!("- **Endpoints:** {endpoint_count}");
                    println!("- **Types:** {type_count}");
                    println!("- **Tags:** {}", tags.join(", "));
                }
                OutputFormat::Json => {
                    let summary = serde_json::json!({
                        "endpoints": endpoint_count,
                        "types": type_count,
                        "tags": tags,
                    });
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&summary).expect("serialize summary")
                    );
                }
            }
        }
    }
}
