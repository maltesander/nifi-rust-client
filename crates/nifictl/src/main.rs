mod body;
mod client_factory;
mod config;
mod confirm;
mod dry_run;
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

    /// Timeout for --wait (e.g. "30s", "2m")
    #[arg(long = "wait-timeout", global = true, default_value = "30s")]
    wait_timeout: String,

    /// Print the request that would be sent and exit; send nothing
    #[arg(long = "dry-run", global = true)]
    dry_run: bool,

    /// Skip confirmation prompt on destructive commands
    #[arg(long = "yes", short = 'y', global = true)]
    yes: bool,

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

    /// Operator porcelain — bulk state changes on a process group
    #[command(subcommand)]
    Ops(OpsCommand),

    /// Generate shell completions
    Completions {
        /// Shell type
        shell: clap_complete::Shell,
    },

    // Generated resource subcommands — explicitly listed so we can
    // substitute our own `Flow` wrapper (see `FlowCommand`). When the
    // code generator adds a new tag, extend this list AND the matching
    // arm in `run()` below.
    /// Manage Access resources
    #[command(name = "access")]
    Access {
        #[command(subcommand)]
        command: generated::AccessCommand,
    },
    /// Manage Authentication resources
    #[command(name = "authentication")]
    Authentication {
        #[command(subcommand)]
        command: generated::AuthenticationCommand,
    },
    /// Manage Connections resources
    #[command(name = "connections")]
    Connections {
        #[command(subcommand)]
        command: generated::ConnectionsCommand,
    },
    /// Manage Connectors resources
    #[command(name = "connectors")]
    Connectors {
        #[command(subcommand)]
        command: generated::ConnectorsCommand,
    },
    /// Manage Controller resources
    #[command(name = "controller")]
    Controller {
        #[command(subcommand)]
        command: generated::ControllerCommand,
    },
    /// Manage ControllerServices resources
    #[command(name = "controller_services")]
    ControllerServices {
        #[command(subcommand)]
        command: generated::ControllerServicesCommand,
    },
    /// Manage Counters resources
    #[command(name = "counters")]
    Counters {
        #[command(subcommand)]
        command: generated::CountersCommand,
    },
    /// Manage DataTransfer resources
    #[command(name = "datatransfer")]
    DataTransfer {
        #[command(subcommand)]
        command: generated::DataTransferCommand,
    },
    /// Manage Flow resources
    #[command(name = "flow")]
    Flow {
        #[command(subcommand)]
        command: FlowCommand,
    },
    /// Manage FlowFileQueues resources
    #[command(name = "flowfilequeues")]
    FlowFileQueues {
        #[command(subcommand)]
        command: generated::FlowFileQueuesCommand,
    },
    /// Manage Funnels resources
    #[command(name = "funnels")]
    Funnels {
        #[command(subcommand)]
        command: generated::FunnelsCommand,
    },
    /// Manage InputPorts resources
    #[command(name = "inputports")]
    InputPorts {
        #[command(subcommand)]
        command: generated::InputPortsCommand,
    },
    /// Manage Labels resources
    #[command(name = "labels")]
    Labels {
        #[command(subcommand)]
        command: generated::LabelsCommand,
    },
    /// Manage OutputPorts resources
    #[command(name = "outputports")]
    OutputPorts {
        #[command(subcommand)]
        command: generated::OutputPortsCommand,
    },
    /// Manage ParameterContexts resources
    #[command(name = "parametercontexts")]
    ParameterContexts {
        #[command(subcommand)]
        command: generated::ParameterContextsCommand,
    },
    /// Manage ParameterProviders resources
    #[command(name = "parameterproviders")]
    ParameterProviders {
        #[command(subcommand)]
        command: generated::ParameterProvidersCommand,
    },
    /// Manage Policies resources
    #[command(name = "policies")]
    Policies {
        #[command(subcommand)]
        command: generated::PoliciesCommand,
    },
    /// Manage ProcessGroups resources
    #[command(name = "processgroups")]
    ProcessGroups {
        #[command(subcommand)]
        command: generated::ProcessGroupsCommand,
    },
    /// Manage Processors resources
    #[command(name = "processors")]
    Processors {
        #[command(subcommand)]
        command: generated::ProcessorsCommand,
    },
    /// Manage Provenance resources
    #[command(name = "provenance")]
    Provenance {
        #[command(subcommand)]
        command: generated::ProvenanceCommand,
    },
    /// Manage ProvenanceEvents resources
    #[command(name = "provenanceevents")]
    ProvenanceEvents {
        #[command(subcommand)]
        command: generated::ProvenanceEventsCommand,
    },
    /// Manage RemoteProcessGroups resources
    #[command(name = "remoteprocessgroups")]
    RemoteProcessGroups {
        #[command(subcommand)]
        command: generated::RemoteProcessGroupsCommand,
    },
    /// Manage ReportingTasks resources
    #[command(name = "reportingtasks")]
    ReportingTasks {
        #[command(subcommand)]
        command: generated::ReportingTasksCommand,
    },
    /// Manage Resources resources
    #[command(name = "resources")]
    Resources {
        #[command(subcommand)]
        command: generated::ResourcesCommand,
    },
    /// Manage SiteToSite resources
    #[command(name = "sitetosite")]
    SiteToSite {
        #[command(subcommand)]
        command: generated::SiteToSiteCommand,
    },
    /// Manage Snippets resources
    #[command(name = "snippets")]
    Snippets {
        #[command(subcommand)]
        command: generated::SnippetsCommand,
    },
    /// Manage SystemDiagnostics resources
    #[command(name = "systemdiagnostics")]
    SystemDiagnostics {
        #[command(subcommand)]
        command: generated::SystemDiagnosticsCommand,
    },
    /// Manage Tenants resources
    #[command(name = "tenants")]
    Tenants {
        #[command(subcommand)]
        command: generated::TenantsCommand,
    },
    /// Manage Versions resources
    #[command(name = "versions")]
    Versions {
        #[command(subcommand)]
        command: generated::VersionsCommand,
    },
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

#[derive(clap::Subcommand)]
enum OpsCommand {
    /// Start (schedule) all authorized processors in a process group
    StartPg {
        /// The process group id
        pg_id: String,
    },
    /// Stop (unschedule) all processors in a process group
    StopPg {
        /// The process group id
        pg_id: String,
    },
    /// Enable all authorized controller services in a process group
    EnableServices {
        /// The process group id
        pg_id: String,
    },
    /// Disable all controller services in a process group
    DisableServices {
        /// The process group id
        pg_id: String,
    },
}

#[derive(clap::Subcommand)]
enum FlowCommand {
    /// Export a process group's versioned flow snapshot to stdout or a file
    Export(FlowExportArgs),
    /// Import a flow snapshot as a new child process group
    Import(FlowImportArgs),
    /// Replace a process group's contents from a flow snapshot (destructive)
    Replace(FlowReplaceArgs),
    /// Generated flow subcommands (get-about-info, get-banners, etc.)
    #[command(flatten)]
    Generated(generated::FlowCommand),
}

#[derive(clap::Args)]
struct FlowExportArgs {
    /// Source process group id
    pg_id: String,
    /// Output file path. Writes to stdout if omitted.
    // `id` must differ from the global `Cli::output` arg id. The global has
    // `global = true` so its `--output` propagates into every subcommand;
    // without this distinct id, clap rejects duplicate registration even
    // though the local long flag is `--output-file`.
    #[arg(id = "flow_export_output", long = "output-file")]
    output: Option<PathBuf>,
    /// Include externally-referenced controller services in the snapshot
    #[arg(long = "include-referenced-services")]
    include_referenced_services: bool,
}

#[derive(clap::Args)]
struct FlowImportArgs {
    /// Parent process group id — the new child PG will be created under this one
    parent_pg_id: String,
    /// Snapshot file previously produced by `nifictl flow export`
    file: PathBuf,
    /// Name for the new child process group. Defaults to the file stem.
    #[arg(long = "name")]
    name: Option<String>,
}

#[derive(clap::Args)]
struct FlowReplaceArgs {
    /// Target process group id — its contents will be overwritten
    pg_id: String,
    /// Snapshot file previously produced by `nifictl flow export`
    file: PathBuf,
    /// Stop the PG, replace, then restart (handles NiFi's "components are running" constraint)
    #[arg(long = "stop-first")]
    stop_first: bool,
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

#[allow(clippy::too_many_arguments)]
async fn dispatch_resource(
    resource: generated::GeneratedResource,
    config_path: Option<&std::path::Path>,
    context_name_override: Option<&str>,
    url: Option<String>,
    username: Option<String>,
    password: Option<String>,
    token: Option<String>,
    insecure: bool,
    dry_run: bool,
    yes: bool,
    wait: bool,
    wait_timeout: &str,
    output: &str,
) -> Result<(), error::CliError> {
    let cfg = load_config(config_path)?;
    let context = resolve_context(&cfg, context_name_override)?;
    let params =
        client_factory::ResolvedParams::resolve(url, username, password, token, insecure, context)?;
    let base_url = params.url.clone();
    let ctx = dry_run::CliCtx {
        dry_run,
        yes,
        base_url: &base_url,
    };

    let client = if ctx.dry_run {
        nifi_rust_client::NifiClientBuilder::new(&base_url)?
            .danger_accept_invalid_certs(params.insecure)
            .build_dynamic()?
    } else {
        params.build_client().await?
    };

    let wait_plan = if wait {
        wait_wire::peek_wait_plan(&resource)?
    } else {
        None
    };

    let mut result = generated::dispatch_generated(resource, &client, &ctx).await?;

    if let Some(plan) = wait_plan {
        if ctx.dry_run {
            let timeout = wait_wire::parse_wait_timeout(wait_timeout)?;
            eprintln!(
                "  + would then {}",
                wait_wire::describe_wait_plan(&plan, timeout)
            );
        } else {
            let dispatch_value = match &result {
                output::CliOutput::Single(v) => v.clone(),
                _ => serde_json::Value::Null,
            };
            let timeout = wait_wire::parse_wait_timeout(wait_timeout)?;
            result = wait_wire::run_wait_plan(plan, dispatch_value, &client, timeout).await?;
        }
    } else if wait {
        return Err(error::CliError::User(
            "--wait is not supported on this command".to_string(),
        ));
    }

    let fmt = output::OutputFormat::parse(output).map_err(error::CliError::User)?;
    let resolved = fmt.resolve();
    output::render(&result, &resolved, &[], &mut std::io::stdout()).map_err(error::CliError::Io)
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
        Commands::Ops(cmd) => {
            if cli.wait {
                return Err(error::CliError::User(
                    "--wait on 'ops' subcommands is not yet supported; omit --wait to proceed without polling"
                        .to_string(),
                ));
            }
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
            let base_url = params.url.clone();
            let ctx = dry_run::CliCtx {
                dry_run: cli.dry_run,
                yes: cli.yes,
                base_url: &base_url,
            };

            // For destructive commands, run the confirmation gate before
            // attempting any network calls so that a non-TTY / missing --yes
            // refuses immediately without touching the server.
            if !ctx.dry_run {
                match &cmd {
                    OpsCommand::StopPg { pg_id } => {
                        confirm::confirm_destructive(&porcelain::ops::stop_pg_what(pg_id), &ctx)?;
                    }
                    OpsCommand::DisableServices { pg_id } => {
                        confirm::confirm_destructive(
                            &porcelain::ops::disable_services_what(pg_id),
                            &ctx,
                        )?;
                    }
                    OpsCommand::StartPg { .. } | OpsCommand::EnableServices { .. } => {}
                }
            }

            // On --dry-run, skip authentication: the handler short-circuits
            // before touching the client, so we only need a constructed
            // (not authenticated) DynamicClient for the signature.
            let client = if ctx.dry_run {
                nifi_rust_client::NifiClientBuilder::new(&base_url)?
                    .danger_accept_invalid_certs(params.insecure)
                    .build_dynamic()?
            } else {
                params.build_client().await?
            };

            let result = match cmd {
                OpsCommand::StartPg { pg_id } => {
                    porcelain::ops::start_pg(&client, &pg_id, &ctx).await?
                }
                OpsCommand::StopPg { pg_id } => {
                    // confirm already ran above; pass yes=true to skip the
                    // redundant check inside the porcelain helper.
                    let ctx_yes = dry_run::CliCtx { yes: true, ..ctx };
                    porcelain::ops::stop_pg(&client, &pg_id, &ctx_yes).await?
                }
                OpsCommand::EnableServices { pg_id } => {
                    porcelain::ops::enable_services(&client, &pg_id, &ctx).await?
                }
                OpsCommand::DisableServices { pg_id } => {
                    // confirm already ran above; pass yes=true to skip the
                    // redundant check inside the porcelain helper.
                    let ctx_yes = dry_run::CliCtx { yes: true, ..ctx };
                    porcelain::ops::disable_services(&client, &pg_id, &ctx_yes).await?
                }
            };
            let fmt = output::OutputFormat::parse(&cli.output).map_err(error::CliError::User)?;
            let resolved = fmt.resolve();
            output::render(&result, &resolved, &[], &mut std::io::stdout())
                .map_err(error::CliError::Io)
        }
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
        Commands::Access { command } => {
            dispatch_resource(
                generated::GeneratedResource::Access { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Authentication { command } => {
            dispatch_resource(
                generated::GeneratedResource::Authentication { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Connections { command } => {
            dispatch_resource(
                generated::GeneratedResource::Connections { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Connectors { command } => {
            dispatch_resource(
                generated::GeneratedResource::Connectors { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Controller { command } => {
            dispatch_resource(
                generated::GeneratedResource::Controller { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::ControllerServices { command } => {
            dispatch_resource(
                generated::GeneratedResource::ControllerServices { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Counters { command } => {
            dispatch_resource(
                generated::GeneratedResource::Counters { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::DataTransfer { command } => {
            dispatch_resource(
                generated::GeneratedResource::DataTransfer { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Flow { command } => match command {
            FlowCommand::Generated(inner) => {
                dispatch_resource(
                    generated::GeneratedResource::Flow { command: inner },
                    cli.config.as_deref(),
                    cli.context.as_deref(),
                    cli.url.clone(),
                    cli.username.clone(),
                    cli.password.clone(),
                    cli.token.clone(),
                    cli.insecure,
                    cli.dry_run,
                    cli.yes,
                    cli.wait,
                    &cli.wait_timeout,
                    &cli.output,
                )
                .await
            }
            FlowCommand::Export(_) | FlowCommand::Import(_) | FlowCommand::Replace(_) => {
                todo!("flow porcelain lands in Task 10")
            }
        },
        Commands::FlowFileQueues { command } => {
            dispatch_resource(
                generated::GeneratedResource::FlowFileQueues { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Funnels { command } => {
            dispatch_resource(
                generated::GeneratedResource::Funnels { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::InputPorts { command } => {
            dispatch_resource(
                generated::GeneratedResource::InputPorts { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Labels { command } => {
            dispatch_resource(
                generated::GeneratedResource::Labels { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::OutputPorts { command } => {
            dispatch_resource(
                generated::GeneratedResource::OutputPorts { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::ParameterContexts { command } => {
            dispatch_resource(
                generated::GeneratedResource::ParameterContexts { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::ParameterProviders { command } => {
            dispatch_resource(
                generated::GeneratedResource::ParameterProviders { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Policies { command } => {
            dispatch_resource(
                generated::GeneratedResource::Policies { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::ProcessGroups { command } => {
            dispatch_resource(
                generated::GeneratedResource::ProcessGroups { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Processors { command } => {
            dispatch_resource(
                generated::GeneratedResource::Processors { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Provenance { command } => {
            dispatch_resource(
                generated::GeneratedResource::Provenance { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::ProvenanceEvents { command } => {
            dispatch_resource(
                generated::GeneratedResource::ProvenanceEvents { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::RemoteProcessGroups { command } => {
            dispatch_resource(
                generated::GeneratedResource::RemoteProcessGroups { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::ReportingTasks { command } => {
            dispatch_resource(
                generated::GeneratedResource::ReportingTasks { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Resources { command } => {
            dispatch_resource(
                generated::GeneratedResource::Resources { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::SiteToSite { command } => {
            dispatch_resource(
                generated::GeneratedResource::SiteToSite { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Snippets { command } => {
            dispatch_resource(
                generated::GeneratedResource::Snippets { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::SystemDiagnostics { command } => {
            dispatch_resource(
                generated::GeneratedResource::SystemDiagnostics { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Tenants { command } => {
            dispatch_resource(
                generated::GeneratedResource::Tenants { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
        }
        Commands::Versions { command } => {
            dispatch_resource(
                generated::GeneratedResource::Versions { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                cli.password.clone(),
                cli.token.clone(),
                cli.insecure,
                cli.dry_run,
                cli.yes,
                cli.wait,
                &cli.wait_timeout,
                &cli.output,
            )
            .await
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
