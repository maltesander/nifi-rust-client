mod body;
mod client_factory;
mod config;
mod error;
mod output;
mod porcelain;
mod table;
mod wait_wire;

// Include generated CLI code
#[allow(dead_code, unused_imports, unused_variables, clippy::all)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated_cli.rs"));
}

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "nifictl", version, about = "CLI tool for Apache NiFi 2.x")]
struct Cli {
    /// Path to the config file
    #[arg(long, env = "NIFICTL_CONFIG", global = true)]
    config: Option<PathBuf>,

    /// Override the active context from the config file
    #[arg(long, global = true)]
    context: Option<String>,

    /// NiFi base URL (overrides context)
    #[arg(long, env = "NIFI_URL", global = true)]
    url: Option<String>,

    /// NiFi username (overrides context)
    #[arg(long, env = "NIFI_USERNAME", global = true)]
    username: Option<String>,

    /// NiFi password (overrides context)
    #[arg(long, env = "NIFI_PASSWORD", global = true)]
    password: Option<String>,

    /// NiFi bearer token (overrides context)
    #[arg(long, env = "NIFI_TOKEN", global = true)]
    token: Option<String>,

    /// Disable TLS certificate verification
    #[arg(long, global = true)]
    insecure: bool,

    /// Output format: auto, json, yaml, table
    #[arg(short, long, default_value = "auto", global = true)]
    output: String,

    /// Increase log verbosity (-v info, -vv debug, -vvv trace)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// Wait for the operation to reach a terminal state (supported commands only)
    #[arg(long, global = true)]
    wait: bool,

    /// Timeout for --wait (e.g. "30s", "2m"). Default: 30s.
    #[arg(long = "wait-timeout", global = true, default_value = "30s")]
    wait_timeout: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Authenticate and cache token
    Login,
    /// Clear cached token
    Logout,
    /// Show NiFi cluster status
    Status,

    /// Manage CLI configuration and contexts
    #[command(subcommand)]
    Config(ConfigCommand),

    /// Generate shell completions
    Completions {
        /// Shell type
        shell: clap_complete::Shell,
    },

    /// Generated resource subcommands
    #[command(flatten)]
    Resource(Box<generated::GeneratedResource>),
}

#[derive(clap::Subcommand)]
enum ConfigCommand {
    /// Show resolved configuration
    View,
    /// List available contexts
    Contexts,
    /// Switch active context
    UseContext {
        /// Context name to activate
        name: String,
    },
    /// Delete a context from the config file
    DeleteContext {
        /// Context name to delete
        name: String,
    },
}

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

    match run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            e.exit_code()
        }
    }
}

async fn run(cli: Cli) -> Result<(), error::CliError> {
    match cli.command {
        Commands::Login => {
            let cfg = load_config(cli.config.as_deref())?;
            let context_name = resolve_context_name(&cfg, cli.context.as_deref());
            let context = resolve_context(&cfg, cli.context.as_deref())?;
            let params = client_factory::ResolvedParams::resolve(
                cli.url,
                cli.username,
                cli.password,
                cli.token,
                cli.insecure,
                context,
            )?;
            porcelain::login::login(&params, &context_name).await
        }
        Commands::Logout => {
            let cfg = load_config(cli.config.as_deref())?;
            let context_name = resolve_context_name(&cfg, cli.context.as_deref());
            porcelain::login::logout(&context_name)
        }
        Commands::Status => {
            let cfg = load_config(cli.config.as_deref())?;
            let context = resolve_context(&cfg, cli.context.as_deref())?;
            let params = client_factory::ResolvedParams::resolve(
                cli.url,
                cli.username,
                cli.password,
                cli.token,
                cli.insecure,
                context,
            )?;
            let client = params.build_client().await?;
            let result = porcelain::status::status(&client).await?;
            let fmt = output::OutputFormat::parse(&cli.output).map_err(error::CliError::User)?;
            let resolved = fmt.resolve();
            output::render(&result, &resolved, &[], &mut std::io::stdout())
                .map_err(error::CliError::Io)
        }
        Commands::Config(cmd) => handle_config(cmd, cli.config.as_deref()).await,
        Commands::Completions { shell } => {
            use clap::CommandFactory;
            clap_complete::generate(
                shell,
                &mut Cli::command(),
                "nifictl",
                &mut std::io::stdout(),
            );
            Ok(())
        }
        Commands::Resource(resource) => {
            let cfg = load_config(cli.config.as_deref())?;
            let context = resolve_context(&cfg, cli.context.as_deref())?;
            let params = client_factory::ResolvedParams::resolve(
                cli.url,
                cli.username,
                cli.password,
                cli.token,
                cli.insecure,
                context,
            )?;
            let client = params.build_client().await?;
            let result = generated::dispatch_generated(*resource, &client).await?;
            let fmt = output::OutputFormat::parse(&cli.output).map_err(error::CliError::User)?;
            let resolved = fmt.resolve();
            output::render(&result, &resolved, &[], &mut std::io::stdout())
                .map_err(error::CliError::Io)
        }
    }
}

async fn handle_config(
    cmd: ConfigCommand,
    config_path: Option<&std::path::Path>,
) -> Result<(), error::CliError> {
    match cmd {
        ConfigCommand::View => {
            let default_path = config::Config::default_path();
            let path = config_path.unwrap_or(&default_path);
            match load_config(Some(path))? {
                None => {
                    println!("No config file found");
                }
                Some(cfg) => {
                    println!("Config file: {}", path.display());
                    match &cfg.current_context {
                        Some(ctx) => println!("Current context: {ctx}"),
                        None => println!("Current context: (none)"),
                    }
                    println!("Contexts ({}):", cfg.contexts.len());
                    for ctx in &cfg.contexts {
                        let active = cfg
                            .current_context
                            .as_deref()
                            .map(|c| c == ctx.name)
                            .unwrap_or(false);
                        let marker = if active { "*" } else { " " };
                        println!("  {marker} {} ({})", ctx.name, ctx.url);
                    }
                }
            }
        }
        ConfigCommand::Contexts => {
            let default_path = config::Config::default_path();
            let path = config_path.unwrap_or(&default_path);
            match load_config(Some(path))? {
                None => {
                    println!("No config file found");
                }
                Some(cfg) => {
                    for ctx in &cfg.contexts {
                        let active = cfg
                            .current_context
                            .as_deref()
                            .map(|c| c == ctx.name)
                            .unwrap_or(false);
                        let marker = if active { "*" } else { " " };
                        println!("{marker} {}", ctx.name);
                    }
                }
            }
        }
        ConfigCommand::UseContext { name } => {
            let default_path = config::Config::default_path();
            let path = config_path.unwrap_or(&default_path);
            if !path.exists() {
                return Err(error::CliError::User(format!(
                    "config file not found: {}",
                    path.display()
                )));
            }
            porcelain::config_cmd::use_context(path, &name)?;
        }
        ConfigCommand::DeleteContext { name } => {
            let default_path = config::Config::default_path();
            let path = config_path.unwrap_or(&default_path);
            if !path.exists() {
                return Err(error::CliError::User(format!(
                    "config file not found: {}",
                    path.display()
                )));
            }
            porcelain::config_cmd::delete_context(path, &name)?;
        }
    }
    Ok(())
}

/// Load config from the given path, or the default path if `None`.
/// Returns `Ok(None)` if the file does not exist.
fn load_config(path: Option<&std::path::Path>) -> Result<Option<config::Config>, error::CliError> {
    let default_path = config::Config::default_path();
    let resolved = path.unwrap_or(&default_path);
    if !resolved.exists() {
        return Ok(None);
    }
    let cfg = config::Config::load(resolved)?;
    Ok(Some(cfg))
}

/// Resolve the context name for token caching purposes.
/// Uses --context flag, then config's current_context, then "default" as fallback.
fn resolve_context_name(cfg: &Option<config::Config>, name: Option<&str>) -> String {
    if let Some(n) = name {
        return n.to_string();
    }
    if let Some(config) = cfg
        && let Some(ctx) = &config.current_context
    {
        return ctx.clone();
    }
    "default".to_string()
}

/// Find a context by name, or return the active context from config.
fn resolve_context<'a>(
    cfg: &'a Option<config::Config>,
    name: Option<&str>,
) -> Result<Option<&'a config::Context>, error::CliError> {
    let Some(config) = cfg else {
        return Ok(None);
    };

    if let Some(n) = name {
        return config.find_context(n).map(Some).ok_or_else(|| {
            error::CliError::User(format!("context '{n}' not found in config file"))
        });
    }

    Ok(config.active_context())
}
