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

impl CliError {
    /// Optional one-line remediation hint printed on a second stderr line.
    ///
    /// Returns `None` when the error's own Display message already tells
    /// the operator what to do (e.g. `UnsupportedEndpoint`, `User`).
    pub fn hint(&self) -> Option<&'static str> {
        match self {
            CliError::Nifi(NifiError::Unauthorized { .. }) => Some("run 'nifictl login'"),
            CliError::Nifi(NifiError::Forbidden { .. }) => {
                Some("user lacks the required NiFi policy — check /users in the UI")
            }
            CliError::Nifi(NifiError::NotFound { .. }) => Some(
                "verify the id with 'nifictl <resource> list' \
                 or check 'nifictl status' for the NiFi version",
            ),
            CliError::Nifi(NifiError::InvalidCertificate { .. }) => {
                Some("pass --insecure for dev environments only")
            }
            CliError::Nifi(NifiError::Http { source })
                if is_tls_handshake_error(source as &dyn std::error::Error) =>
            {
                Some("pass --insecure for dev environments only")
            }
            _ => None,
        }
    }
}

/// Fuzzy sniff for TLS/handshake errors in the source chain of
/// a transport error. Walks the full chain and checks each layer's
/// Display output for common TLS keywords (including rustls's
/// "corrupt message" phrasing for record-level handshake failures).
/// Used only to decide whether to append the `--insecure` hint on a
/// transport error.
fn is_tls_handshake_error(err: &dyn std::error::Error) -> bool {
    use std::error::Error;
    let mut current: Option<&dyn Error> = Some(err);
    while let Some(e) = current {
        let msg = e.to_string().to_lowercase();
        if msg.contains("certificate")
            || msg.contains("unknownissuer")
            || msg.contains("tls")
            || msg.contains("handshake")
            || msg.contains("corrupt message")
        {
            return true;
        }
        current = e.source();
    }
    false
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hint_for_unauthorized() {
        let e = CliError::Nifi(NifiError::Unauthorized {
            message: "expired".to_string(),
        });
        assert_eq!(e.hint(), Some("run 'nifictl login'"));
    }

    #[test]
    fn hint_for_forbidden() {
        let e = CliError::Nifi(NifiError::Forbidden {
            message: "denied".to_string(),
        });
        assert_eq!(
            e.hint(),
            Some("user lacks the required NiFi policy — check /users in the UI")
        );
    }

    #[test]
    fn hint_for_not_found() {
        let e = CliError::Nifi(NifiError::NotFound {
            message: "missing".to_string(),
        });
        assert_eq!(
            e.hint(),
            Some(
                "verify the id with 'nifictl <resource> list' or check 'nifictl status' for the NiFi version"
            )
        );
    }

    #[test]
    fn hint_for_unsupported_endpoint_is_none() {
        // UnsupportedEndpoint's own Display is already clear; no hint.
        let e = CliError::Nifi(NifiError::UnsupportedEndpoint {
            endpoint: "POST /foo".to_string(),
            version: "2.6.0".to_string(),
        });
        assert_eq!(e.hint(), None);
    }

    #[test]
    fn hint_for_user_body_file_parse_error_is_none() {
        // CliError::User with a body-file parse message is passthrough;
        // the "include file path + line:col" requirement is already
        // satisfied by serde_json::Error's Display — no hint to add.
        let e = CliError::User(
            "invalid JSON in /path/to/body.json: expected value at line 3 column 5".to_string(),
        );
        assert_eq!(e.hint(), None);
    }

    #[test]
    fn is_tls_handshake_error_matches_keywords() {
        use std::error::Error;
        use std::fmt;

        #[derive(Debug)]
        struct Fake(&'static str);
        impl fmt::Display for Fake {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.0)
            }
        }
        impl Error for Fake {}

        assert!(is_tls_handshake_error(&Fake("invalid peer certificate")));
        assert!(is_tls_handshake_error(&Fake("TLS handshake failure")));
        assert!(is_tls_handshake_error(&Fake("UnknownIssuer")));
        assert!(is_tls_handshake_error(&Fake(
            "received corrupt message of type InvalidContentType"
        )));
        assert!(!is_tls_handshake_error(&Fake("connection refused")));
    }
}
