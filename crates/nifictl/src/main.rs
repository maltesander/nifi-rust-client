mod body;
mod cli;
mod client_factory;
mod config;
mod confirm;
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

use cli::{Cli, Commands, ConfigCommand, FlowCommand, OpsCommand};

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
            if let Some(hint) = e.hint() {
                eprintln!("hint: {hint}");
            }
            e.exit_code()
        }
    }
}

/// Fires once per process on stderr when `--password` was passed on the CLI.
static PASSWORD_FLAG_WARNED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

fn warn_password_flag_used() {
    if PASSWORD_FLAG_WARNED.set(()).is_ok() {
        eprintln!(
            "warning: --password is visible to other local users via the process list; \
             prefer NIFI_PASSWORD, a context's password_env, or the interactive prompt"
        );
    }
}

/// Resolve the password input for `ResolvedParams::resolve`:
///
/// - If `--password` was passed (`Some(_)`), warn once and return it.
/// - Otherwise fall back to the `NIFI_PASSWORD` env var (no warning).
///
/// The env-var fallback lives here because the clap attribute no longer
/// reads `NIFI_PASSWORD` automatically — that coupling made it impossible
/// to tell the two sources apart and fire the warning selectively.
fn resolve_password_input(cli_password: Option<String>) -> Option<String> {
    if cli_password.is_some() {
        warn_password_flag_used();
        cli_password
    } else {
        std::env::var("NIFI_PASSWORD").ok()
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
                resolve_password_input(cli.password),
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
                resolve_password_input(cli.password),
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
                resolve_password_input(cli.password),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
        Commands::Flow { command } => {
            // Generated flow subcommands fall through to the shared
            // dispatch_resource helper (same as any other resource).
            if let FlowCommand::Generated(generated_cmd) = command {
                return dispatch_resource(
                    generated::GeneratedResource::Flow {
                        command: generated_cmd,
                    },
                    cli.config.as_deref(),
                    cli.context.as_deref(),
                    cli.url,
                    cli.username,
                    resolve_password_input(cli.password),
                    cli.token,
                    cli.insecure,
                    cli.dry_run,
                    cli.yes,
                    cli.wait,
                    &cli.wait_timeout,
                    &cli.output,
                )
                .await;
            }

            // Porcelain flow verbs — export, import, replace.
            if cli.wait {
                return Err(error::CliError::User(
                    "--wait is not supported on flow porcelain commands".to_string(),
                ));
            }

            let cfg = load_config(cli.config.as_deref())?;
            let context = resolve_context(&cfg, cli.context.as_deref())?;
            let params = client_factory::ResolvedParams::resolve(
                cli.url,
                cli.username,
                resolve_password_input(cli.password),
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

            // Pre-client confirm gate for `replace` — matches the Ops arm.
            if !ctx.dry_run
                && let FlowCommand::Replace(args) = &command
            {
                confirm::confirm_destructive(&porcelain::flow::replace_what(&args.pg_id), &ctx)?;
            }

            let client = if ctx.dry_run {
                nifi_rust_client::NifiClientBuilder::new(&base_url)?
                    .danger_accept_invalid_certs(params.insecure)
                    .build_dynamic()?
            } else {
                params.build_client().await?
            };

            let result = match command {
                FlowCommand::Export(args) => {
                    porcelain::flow::export(
                        &client,
                        &args.pg_id,
                        args.output.as_deref(),
                        args.include_referenced_services,
                        &ctx,
                    )
                    .await?
                }
                FlowCommand::Import(args) => {
                    porcelain::flow::import(
                        &client,
                        &args.parent_pg_id,
                        &args.file,
                        args.name.as_deref(),
                        &ctx,
                    )
                    .await?
                }
                FlowCommand::Replace(args) => {
                    // Confirm already ran above; pass yes=true so the porcelain's
                    // internal gate is a no-op.
                    let ctx_yes = dry_run::CliCtx { yes: true, ..ctx };
                    porcelain::flow::replace(
                        &client,
                        &args.pg_id,
                        &args.file,
                        args.stop_first,
                        &ctx_yes,
                    )
                    .await?
                }
                FlowCommand::Generated(_) => unreachable!("handled above"),
            };

            let fmt = output::OutputFormat::parse(&cli.output).map_err(error::CliError::User)?;
            let resolved = fmt.resolve();
            output::render(&result, &resolved, &[], &mut std::io::stdout())
                .map_err(error::CliError::Io)
        }
        Commands::FlowFileQueues { command } => {
            dispatch_resource(
                generated::GeneratedResource::FlowFileQueues { command },
                cli.config.as_deref(),
                cli.context.as_deref(),
                cli.url.clone(),
                cli.username.clone(),
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
                resolve_password_input(cli.password.clone()),
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
