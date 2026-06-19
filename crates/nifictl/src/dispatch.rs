//! Command dispatch for `nifictl`.
//!
//! The top-level [`run`] entry point matches on the parsed
//! [`crate::cli::Commands`] and routes each arm to either a porcelain
//! helper or the shared [`dispatch_resource`] path (which every
//! generated resource subcommand goes through).
//!
//! Auth-input resolution (`resolve_password_input`, `warn_password_flag_used`)
//! lives here because it is only consumed by arms in this module.

use crate::cli::{Cli, Commands, ConfigCommand, FlowCommand, OpsCommand};
use crate::{
    client_factory, config, confirm, dry_run, error, generated, output, porcelain, wait_wire,
};

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

/// Fires once per process on stderr when `--token` was passed on the CLI.
static TOKEN_FLAG_WARNED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

fn warn_token_flag_used() {
    if TOKEN_FLAG_WARNED.set(()).is_ok() {
        eprintln!(
            "warning: --token is visible to other local users via the process list; \
             prefer NIFI_TOKEN, a context's cached token, or `nifictl login`"
        );
    }
}

/// Resolve the token input the same way `resolve_password_input` resolves
/// the password — split clap from the env-var fallback so the visibility
/// warning fires only when the value actually came from the CLI.
fn resolve_token_input(cli_token: Option<String>) -> Option<String> {
    if cli_token.is_some() {
        warn_token_flag_used();
        cli_token
    } else {
        std::env::var("NIFI_TOKEN").ok()
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

    // Dry-run: skip auth resolution entirely. Only URL + insecure are
    // needed because porcelain handlers short-circuit before any
    // network call. The live path resolves full auth (including any
    // cached token) below.
    let (client, base_url) = if dry_run {
        let dr = client_factory::ResolvedParams::resolve_url_only(url, insecure, context)?;
        let client = nifi_rust_client::NifiClientBuilder::new(&dr.url)?
            .danger_accept_invalid_certs(dr.insecure)
            .build_dynamic()?;
        (client, dr.url)
    } else {
        let context_name = resolve_context_name(&cfg, context_name_override)?;
        let params = client_factory::ResolvedParams::resolve(
            url, username, password, token, insecure, context,
        )?;
        let url_clone = params.url.clone();
        let client = params.build_client_with_cache(&context_name).await?;
        (client, url_clone)
    };

    let ctx = dry_run::CliCtx {
        dry_run,
        yes,
        base_url: &base_url,
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

/// Route every uniform generated-resource variant through `dispatch_resource`
/// in one place — the body is identical for all ~28 arms, and listing each
/// one by hand drowned `run()` in boilerplate.
///
/// The macro takes the parsed `Cli` + the current arm's inner `command`
/// plus the `Commands` / `GeneratedResource` variant name and expands to
/// a single `dispatch_resource(...)` call. Using `.clone()` on the `Cli`
/// flag fields mirrors the previous hand-written arms exactly.
macro_rules! dispatch_uniform {
    ($cli:ident, $command:ident, $variant:ident) => {
        dispatch_resource(
            generated::GeneratedResource::$variant { command: $command },
            $cli.config.as_deref(),
            $cli.context.as_deref(),
            $cli.url.clone(),
            $cli.username.clone(),
            resolve_password_input($cli.password.clone()),
            resolve_token_input($cli.token.clone()),
            $cli.insecure,
            $cli.dry_run,
            $cli.yes,
            $cli.wait,
            &$cli.wait_timeout,
            &$cli.output,
        )
        .await
    };
}

pub(crate) async fn run(cli: Cli) -> Result<(), error::CliError> {
    match cli.command {
        Commands::Login => {
            let cfg = load_config(cli.config.as_deref())?;
            let context_name = resolve_context_name(&cfg, cli.context.as_deref())?;
            let context = resolve_context(&cfg, cli.context.as_deref())?;
            let params = client_factory::ResolvedParams::resolve(
                cli.url,
                cli.username,
                resolve_password_input(cli.password),
                resolve_token_input(cli.token),
                cli.insecure,
                context,
            )?;
            porcelain::login::login(&params, &context_name).await
        }
        Commands::Logout => {
            let cfg = load_config(cli.config.as_deref())?;
            let context_name = resolve_context_name(&cfg, cli.context.as_deref())?;
            porcelain::login::logout(&context_name)
        }
        Commands::Status => {
            let cfg = load_config(cli.config.as_deref())?;
            let context_name = resolve_context_name(&cfg, cli.context.as_deref())?;
            let context = resolve_context(&cfg, cli.context.as_deref())?;
            let params = client_factory::ResolvedParams::resolve(
                cli.url,
                cli.username,
                resolve_password_input(cli.password),
                resolve_token_input(cli.token),
                cli.insecure,
                context,
            )?;
            let client = params.build_client_with_cache(&context_name).await?;
            let result = porcelain::status::status(&client).await?;
            let fmt = output::OutputFormat::parse(&cli.output).map_err(error::CliError::User)?;
            let resolved = fmt.resolve();
            output::render(&result, &resolved, &[], &mut std::io::stdout())
                .map_err(error::CliError::Io)
        }
        Commands::Config(cmd) => handle_config(cmd, cli.config.as_deref()).await,
        Commands::Ops(cmd) => {
            let cfg = load_config(cli.config.as_deref())?;
            let context = resolve_context(&cfg, cli.context.as_deref())?;

            // Resolve the password / token input up-front: this fires the
            // `--password` / `--token` visibility warning regardless of
            // dry-run, since the secret was already exposed on the process
            // list the moment the user typed it. The dry-run branch then
            // ignores the value; the live branch consumes it.
            let password_input = resolve_password_input(cli.password);
            let token_input = resolve_token_input(cli.token);

            // Resolve URL + insecure first — needed both for the confirm
            // gate's display string and for the dry-run client. Auth is
            // resolved later, only on the live path, so --dry-run works
            // without --token / --username / context auth.
            let dr = client_factory::ResolvedParams::resolve_url_only(
                cli.url.clone(),
                cli.insecure,
                context,
            )?;
            let ctx = dry_run::CliCtx {
                dry_run: cli.dry_run,
                yes: cli.yes,
                base_url: &dr.url,
            };

            // Single canonical confirm gate for destructive Ops commands —
            // runs before any network call so a non-TTY / missing --yes
            // refuses immediately. Porcelain handlers no longer re-gate;
            // see `confirm::run_for_ops` for the full contract.
            confirm::run_for_ops(&cmd, &ctx)?;

            // On --dry-run, skip authentication: the handler short-circuits
            // before touching the client, so we only need a constructed
            // (not authenticated) DynamicClient for the signature.
            let client = if ctx.dry_run {
                nifi_rust_client::NifiClientBuilder::new(&dr.url)?
                    .danger_accept_invalid_certs(dr.insecure)
                    .build_dynamic()?
            } else {
                let context_name = resolve_context_name(&cfg, cli.context.as_deref())?;
                let params = client_factory::ResolvedParams::resolve(
                    cli.url,
                    cli.username,
                    password_input,
                    token_input,
                    cli.insecure,
                    context,
                )?;
                params.build_client_with_cache(&context_name).await?
            };

            let result = match &cmd {
                OpsCommand::StartPg { pg_id } => {
                    porcelain::ops::start_pg(&client, pg_id, &ctx).await?
                }
                OpsCommand::StopPg { pg_id } => {
                    porcelain::ops::stop_pg(&client, pg_id, &ctx).await?
                }
                OpsCommand::EnableServices { pg_id } => {
                    porcelain::ops::enable_services(&client, pg_id, &ctx).await?
                }
                OpsCommand::DisableServices { pg_id } => {
                    porcelain::ops::disable_services(&client, pg_id, &ctx).await?
                }
            };

            // `--wait` (live path only): block until the requested state is
            // reached. Dry-run never hits the server, so there is nothing to
            // poll.
            if cli.wait && !ctx.dry_run {
                let timeout = wait_wire::parse_wait_timeout(&cli.wait_timeout)?;
                let config = nifi_rust_client::wait::WaitConfig {
                    timeout,
                    ..Default::default()
                };
                porcelain::ops::wait_for_ops(&client, &cmd, config).await?;
            }

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
                    resolve_token_input(cli.token),
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

            // Resolve the password / token input up-front: this fires the
            // `--password` / `--token` visibility warning regardless of
            // dry-run, since the secret was already exposed on the process
            // list the moment the user typed it. The dry-run branch then
            // ignores the value; the live branch consumes it.
            let password_input = resolve_password_input(cli.password);
            let token_input = resolve_token_input(cli.token);

            // Resolve URL + insecure first — needed both for the confirm
            // gate's display string and for the dry-run client. Auth is
            // resolved later, only on the live path, so --dry-run works
            // without --token / --username / context auth.
            let dr = client_factory::ResolvedParams::resolve_url_only(
                cli.url.clone(),
                cli.insecure,
                context,
            )?;
            let ctx = dry_run::CliCtx {
                dry_run: cli.dry_run,
                yes: cli.yes,
                base_url: &dr.url,
            };

            // Single canonical confirm gate for destructive Flow porcelain
            // commands. Same contract as `confirm::run_for_ops`.
            confirm::run_for_flow(&command, &ctx)?;

            let client = if ctx.dry_run {
                nifi_rust_client::NifiClientBuilder::new(&dr.url)?
                    .danger_accept_invalid_certs(dr.insecure)
                    .build_dynamic()?
            } else {
                let context_name = resolve_context_name(&cfg, cli.context.as_deref())?;
                let params = client_factory::ResolvedParams::resolve(
                    cli.url,
                    cli.username,
                    password_input,
                    token_input,
                    cli.insecure,
                    context,
                )?;
                params.build_client_with_cache(&context_name).await?
            };

            let result = match command {
                FlowCommand::Export(args) => {
                    porcelain::flow::export(
                        &client,
                        &args.pg_id,
                        args.output.as_deref(),
                        args.include_referenced_services,
                        args.include_component_state,
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
                    porcelain::flow::replace(
                        &client,
                        &args.pg_id,
                        &args.file,
                        args.stop_first,
                        &ctx,
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
        // Every remaining arm has the same shape: unwrap the inner
        // `command`, wrap it in the matching `GeneratedResource` variant,
        // and call `dispatch_resource` with the per-invocation Cli flags.
        // The `dispatch_uniform!` macro expands to exactly that call.
        Commands::Access { command } => dispatch_uniform!(cli, command, Access),
        Commands::Authentication { command } => dispatch_uniform!(cli, command, Authentication),
        Commands::Connections { command } => dispatch_uniform!(cli, command, Connections),
        Commands::Connectors { command } => dispatch_uniform!(cli, command, Connectors),
        Commands::Controller { command } => dispatch_uniform!(cli, command, Controller),
        Commands::ControllerServices { command } => {
            dispatch_uniform!(cli, command, ControllerServices)
        }
        Commands::Counters { command } => dispatch_uniform!(cli, command, Counters),
        Commands::DataTransfer { command } => dispatch_uniform!(cli, command, DataTransfer),
        Commands::FlowFileQueues { command } => dispatch_uniform!(cli, command, FlowFileQueues),
        Commands::Funnels { command } => dispatch_uniform!(cli, command, Funnels),
        Commands::InputPorts { command } => dispatch_uniform!(cli, command, InputPorts),
        Commands::Labels { command } => dispatch_uniform!(cli, command, Labels),
        Commands::OutputPorts { command } => dispatch_uniform!(cli, command, OutputPorts),
        Commands::ParameterContexts { command } => {
            dispatch_uniform!(cli, command, ParameterContexts)
        }
        Commands::ParameterProviders { command } => {
            dispatch_uniform!(cli, command, ParameterProviders)
        }
        Commands::Policies { command } => dispatch_uniform!(cli, command, Policies),
        Commands::ProcessGroups { command } => dispatch_uniform!(cli, command, ProcessGroups),
        Commands::Processors { command } => dispatch_uniform!(cli, command, Processors),
        Commands::Provenance { command } => dispatch_uniform!(cli, command, Provenance),
        Commands::ProvenanceEvents { command } => {
            dispatch_uniform!(cli, command, ProvenanceEvents)
        }
        Commands::RemoteProcessGroups { command } => {
            dispatch_uniform!(cli, command, RemoteProcessGroups)
        }
        Commands::ReportingTasks { command } => dispatch_uniform!(cli, command, ReportingTasks),
        Commands::Resources { command } => dispatch_uniform!(cli, command, Resources),
        Commands::SiteToSite { command } => dispatch_uniform!(cli, command, SiteToSite),
        Commands::Snippets { command } => dispatch_uniform!(cli, command, Snippets),
        Commands::SystemDiagnostics { command } => {
            dispatch_uniform!(cli, command, SystemDiagnostics)
        }
        Commands::Tenants { command } => dispatch_uniform!(cli, command, Tenants),
        Commands::Versions { command } => dispatch_uniform!(cli, command, Versions),
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
///
/// The resolved name is later concatenated into a filesystem path
/// (`~/.nifictl/tokens/<context>`), so it MUST not contain path separators
/// or `..` segments. A user passing `--context "../etc/passwd"` would
/// otherwise read/write outside the tokens dir. We reject path-traversal
/// shapes up front with [`error::CliError::User`] before any filesystem
/// access.
fn resolve_context_name(
    cfg: &Option<config::Config>,
    name: Option<&str>,
) -> Result<String, error::CliError> {
    let resolved = if let Some(n) = name {
        n.to_string()
    } else if let Some(config) = cfg
        && let Some(ctx) = &config.current_context
    {
        ctx.clone()
    } else {
        "default".to_string()
    };
    validate_context_name(&resolved)?;
    Ok(resolved)
}

/// Reject context names that would escape `~/.nifictl/tokens/` when joined
/// into a path: separators (`/`, `\`), `..` segments, or NUL bytes. Empty
/// names are also rejected (would resolve to the tokens dir itself).
pub(crate) fn validate_context_name(name: &str) -> Result<(), error::CliError> {
    if name.is_empty() {
        return Err(error::CliError::User(
            "context name must not be empty".to_string(),
        ));
    }
    if name.contains('/')
        || name.contains('\\')
        || name.contains('\0')
        || name.split(['/', '\\']).any(|seg| seg == "..")
        || name == ".."
        || name == "."
    {
        return Err(error::CliError::User(format!(
            "invalid context name '{name}': must not contain '/', '\\\\', '..', or NUL"
        )));
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    /// B3: context names must be safe to concatenate into a path under
    /// `~/.nifictl/tokens/`. Anything that could escape (path separators,
    /// `..`, NUL) is rejected at command entry, before any filesystem call.
    #[test]
    fn validate_context_name_rejects_path_traversal() {
        for bad in [
            "../etc/passwd",
            "..",
            ".",
            "foo/bar",
            "foo\\bar",
            "with\0nul",
            "",
            "a/../b",
        ] {
            let err = validate_context_name(bad)
                .err()
                .unwrap_or_else(|| panic!("expected rejection for {bad:?}"));
            assert!(matches!(err, error::CliError::User(_)));
        }
    }

    #[test]
    fn validate_context_name_accepts_normal_names() {
        for good in ["default", "prod", "staging-1", "dev_test", "a.b", "team:1"] {
            assert!(
                validate_context_name(good).is_ok(),
                "expected {good:?} to be accepted"
            );
        }
    }
}
