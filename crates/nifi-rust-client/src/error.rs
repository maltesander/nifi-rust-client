use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum NifiError {
    #[snafu(display("HTTP request failed: {source}"))]
    Http { source: reqwest::Error },

    #[snafu(display("Failed to parse NiFi base URL: {source}"))]
    InvalidBaseUrl { source: url::ParseError },

    #[snafu(display("Authentication failed: {message}"))]
    Auth { message: String },

    #[snafu(display("Invalid CA certificate: {source}"))]
    InvalidCertificate { source: reqwest::Error },

    #[snafu(display("Unauthorized (401): {message}"))]
    Unauthorized { message: String },

    #[snafu(display("Forbidden (403): {message}"))]
    Forbidden { message: String },

    #[snafu(display("Not found (404): {message}"))]
    NotFound { message: String },

    #[snafu(display("Conflict (409): {message}"))]
    Conflict { message: String },

    #[snafu(display("NiFi API error (status {status}): {message}"))]
    Api { status: u16, message: String },

    #[snafu(display("NiFi version {detected} is not supported by this client build"))]
    UnsupportedVersion { detected: String },

    #[snafu(display("Endpoint {endpoint} is not available in NiFi {version}"))]
    UnsupportedEndpoint { endpoint: String, version: String },
}

impl NifiError {
    /// Returns the HTTP status code if this is an API error variant.
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::Unauthorized { .. } => Some(401),
            Self::Forbidden { .. } => Some(403),
            Self::NotFound { .. } => Some(404),
            Self::Conflict { .. } => Some(409),
            Self::Api { status, .. } => Some(*status),
            _ => None,
        }
    }

    /// True if this error is likely transient and worth retrying.
    pub fn is_retryable(&self) -> bool {
        matches!(self.status_code(), Some(408 | 429 | 500 | 502 | 503 | 504))
            || matches!(self, Self::Http { .. })
    }
}

/// Create the appropriate typed error from an HTTP status code and message.
///
/// Used by all HTTP helpers in `client.rs` to map response status codes
/// to typed error variants.
pub(crate) fn api_error(status: u16, message: String) -> NifiError {
    match status {
        401 => NifiError::Unauthorized { message },
        403 => NifiError::Forbidden { message },
        404 => NifiError::NotFound { message },
        409 => NifiError::Conflict { message },
        _ => NifiError::Api { status, message },
    }
}
