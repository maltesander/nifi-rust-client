//! clap CLI definitions for `nifictl`.
//!
//! The top-level `Cli` struct, the `Commands` enum (with every generated
//! resource listed explicitly — see the note in `AGENTS.md` about the
//! `flow` name collision with porcelain), and the accompanying `*Command`
//! / `*Args` structs all live here. The resource dispatch lives in
//! `crate::dispatch`; this module only describes the parse surface.

use std::path::PathBuf;

use crate::generated;

#[derive(clap::Parser)]
#[command(name = "nifictl", version, about = "CLI tool for Apache NiFi 2.x")]
pub(crate) struct Cli {
    /// Path to the config file
    #[arg(long, env = "NIFICTL_CONFIG", global = true)]
    pub(crate) config: Option<PathBuf>,

    /// Override the active context from the config file
    #[arg(long, global = true)]
    pub(crate) context: Option<String>,

    /// NiFi base URL (overrides context)
    #[arg(long, env = "NIFI_URL", global = true)]
    pub(crate) url: Option<String>,

    /// NiFi username (overrides context)
    #[arg(long, env = "NIFI_USERNAME", global = true)]
    pub(crate) username: Option<String>,

    /// Password for username/password authentication.
    ///
    /// Hidden from --help because passing a password via CLI leaks it to
    /// /proc/<pid>/cmdline and shell history. Prefer NIFI_PASSWORD, a
    /// context's password_env, or the interactive prompt.
    #[arg(long, global = true, hide = true)]
    pub(crate) password: Option<String>,

    /// NiFi bearer token (overrides context)
    #[arg(long, env = "NIFI_TOKEN", global = true)]
    pub(crate) token: Option<String>,

    /// Disable TLS certificate verification
    #[arg(long, global = true)]
    pub(crate) insecure: bool,

    /// Output format: auto, json, yaml, table
    #[arg(short, long, default_value = "auto", global = true)]
    pub(crate) output: String,

    /// Increase log verbosity (-v info, -vv debug, -vvv trace)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub(crate) verbose: u8,

    /// Wait for the operation to reach a terminal state (supported commands only)
    #[arg(long, global = true)]
    pub(crate) wait: bool,

    /// Timeout for --wait (e.g. "30s", "2m")
    #[arg(long = "wait-timeout", global = true, default_value = "30s")]
    pub(crate) wait_timeout: String,

    /// Print the request that would be sent and exit; send nothing
    #[arg(long = "dry-run", global = true)]
    pub(crate) dry_run: bool,

    /// Skip confirmation prompt on destructive commands
    #[arg(long = "yes", short = 'y', global = true)]
    pub(crate) yes: bool,

    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(clap::Subcommand)]
pub(crate) enum Commands {
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
    // arm in `crate::dispatch::run`.
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
pub(crate) enum ConfigCommand {
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
pub(crate) enum OpsCommand {
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
pub(crate) enum FlowCommand {
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
pub(crate) struct FlowExportArgs {
    /// Source process group id
    pub(crate) pg_id: String,
    /// Output file path. Writes to stdout if omitted.
    // `id` must differ from the global `Cli::output` arg id. The global has
    // `global = true` so its `--output` propagates into every subcommand;
    // without this distinct id, clap rejects duplicate registration even
    // though the local long flag is `--output-file`.
    #[arg(id = "flow_export_output", long = "output-file")]
    pub(crate) output: Option<PathBuf>,
    /// Include externally-referenced controller services in the snapshot
    #[arg(long = "include-referenced-services")]
    pub(crate) include_referenced_services: bool,
}

#[derive(clap::Args)]
pub(crate) struct FlowImportArgs {
    /// Parent process group id — the new child PG will be created under this one
    pub(crate) parent_pg_id: String,
    /// Snapshot file previously produced by `nifictl flow export`
    pub(crate) file: PathBuf,
    /// Name for the new child process group. Defaults to the file stem.
    #[arg(long = "name")]
    pub(crate) name: Option<String>,
}

#[derive(clap::Args)]
pub(crate) struct FlowReplaceArgs {
    /// Target process group id — its contents will be overwritten
    pub(crate) pg_id: String,
    /// Snapshot file previously produced by `nifictl flow export`
    pub(crate) file: PathBuf,
    /// Stop the PG, replace, then restart (handles NiFi's "components are running" constraint)
    #[arg(long = "stop-first")]
    pub(crate) stop_first: bool,
}
