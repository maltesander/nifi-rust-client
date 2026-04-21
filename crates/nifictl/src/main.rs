mod body;
mod cli;
mod client_factory;
mod config;
mod confirm;
mod dispatch;
mod dry_run;
mod error;
mod jwt;
mod output;
mod porcelain;
mod prompt;
mod table;
mod wait_wire;

// Include generated CLI code
#[allow(dead_code, unused_imports, unused_variables, clippy::all)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated_cli.rs"));
}

use std::process::ExitCode;

use clap::Parser;
use tracing_subscriber::EnvFilter;

use cli::Cli;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    let level = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level)),
        )
        .with_writer(std::io::stderr)
        .init();

    match dispatch::run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            if let Some(hint) = e.hint() {
                eprintln!("hint: {hint}");
            }
            e.exit_code()
        }
    }
}
