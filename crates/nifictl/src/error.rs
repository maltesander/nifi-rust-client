use std::fmt;
use std::process::ExitCode;

use nifi_rust_client::NifiError;

use crate::config::ConfigError;

/// All errors that can be returned by the CLI.
#[derive(Debug)]
pub enum CliError {
    /// An error from the NiFi client library.
    Nifi(NifiError),
    /// An error reading or parsing the config file.
    Config(ConfigError),
    /// An I/O error (e.g. reading a certificate file).
    Io(std::io::Error),
    /// A user-facing error message (not a bug — e.g. missing required flag
    /// or cancelled interactively by the user).
    User(String),
}

impl CliError {
    /// Map the error to a process exit code.
    ///
    /// | Source          | Code |
    /// |-----------------|------|
    /// | NiFi 401        | 4    |
    /// | NiFi 403        | 5    |
    /// | NiFi 404        | 6    |
    /// | NiFi 409        | 7    |
    /// | NiFi other      | 1    |
    /// | Config          | 2    |
    /// | Io              | 3    |
    /// | User            | 1    |
    pub fn exit_code(&self) -> ExitCode {
        match self {
            CliError::Nifi(NifiError::Unauthorized { .. }) => ExitCode::from(4),
            CliError::Nifi(NifiError::Forbidden { .. }) => ExitCode::from(5),
            CliError::Nifi(NifiError::NotFound { .. }) => ExitCode::from(6),
            CliError::Nifi(NifiError::Conflict { .. }) => ExitCode::from(7),
            CliError::Nifi(_) => ExitCode::FAILURE,
            CliError::Config(_) => ExitCode::from(2),
            CliError::Io(_) => ExitCode::from(3),
            CliError::User(_) => ExitCode::FAILURE,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Nifi(e) => write!(f, "{e}"),
            CliError::Config(e) => write!(f, "{e}"),
            CliError::Io(e) => write!(f, "{e}"),
            CliError::User(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<NifiError> for CliError {
    fn from(e: NifiError) -> Self {
        CliError::Nifi(e)
    }
}

impl From<ConfigError> for CliError {
    fn from(e: ConfigError) -> Self {
        CliError::Config(e)
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Io(e)
    }
}
