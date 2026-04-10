#![deny(missing_docs)]
use snafu::Snafu;

/// All errors that can be returned by the NiFi client.
///
/// Variants are `#[non_exhaustive]` — new variants may be added in future releases.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
#[non_exhaustive]
pub enum NifiError {
    /// An underlying HTTP transport error from `reqwest`.
    #[snafu(display("HTTP request failed: {source}"))]
    Http {
        /// The underlying reqwest error.
        source: reqwest::Error,
    },

    /// The base URL provided to [`NifiClientBuilder`](crate::NifiClientBuilder) could not be parsed.
    #[snafu(display("Failed to parse NiFi base URL: {source}"))]
    InvalidBaseUrl {
        /// The underlying URL parse error.
        source: url::ParseError,
    },

    /// Authentication against the NiFi `/access/token` endpoint failed.
    #[snafu(display("Authentication failed: {message}"))]
    Auth {
        /// A description of why authentication failed.
        message: String,
    },

    /// A custom CA certificate or client certificate could not be loaded.
    #[snafu(display("Invalid CA certificate: {source}"))]
    InvalidCertificate {
        /// The underlying reqwest error.
        source: reqwest::Error,
    },

    /// The NiFi server returned HTTP 401 — credentials are missing or expired.
    #[snafu(display("Unauthorized (401): {message}"))]
    Unauthorized {
        /// The error message returned by NiFi.
        message: String,
    },

    /// The NiFi server returned HTTP 403 — the authenticated user lacks permission.
    #[snafu(display("Forbidden (403): {message}"))]
    Forbidden {
        /// The error message returned by NiFi.
        message: String,
    },

    /// The NiFi server returned HTTP 404 — the requested resource does not exist.
    #[snafu(display("Not found (404): {message}"))]
    NotFound {
        /// The error message returned by NiFi.
        message: String,
    },

    /// The NiFi server returned HTTP 409 — the request conflicts with current state.
    #[snafu(display("Conflict (409): {message}"))]
    Conflict {
        /// The error message returned by NiFi.
        message: String,
    },

    /// The NiFi server returned an unexpected non-2xx HTTP status code.
    #[snafu(display("NiFi API error (status {status}): {message}"))]
    Api {
        /// The HTTP status code.
        status: u16,
        /// The error message returned by NiFi.
        message: String,
    },

    /// The detected NiFi version is not compiled into this client build.
    ///
    /// Enable the matching `nifi-x-y-z` feature flag or use the `dynamic` feature.
    #[snafu(display("NiFi version {detected} is not supported by this client build"))]
    UnsupportedVersion {
        /// The version string returned by the NiFi server.
        detected: String,
    },

    /// The requested endpoint does not exist in the active NiFi version.
    ///
    /// Occurs in dynamic mode when the server version predates a given endpoint.
    #[snafu(display("Endpoint {endpoint} is not available in NiFi {version}"))]
    UnsupportedEndpoint {
        /// The path of the unsupported endpoint.
        endpoint: String,
        /// The NiFi version that lacks the endpoint.
        version: String,
    },

    /// A response enum field contained a variant not known to this client build.
    #[snafu(display(
        "Enum variant '{variant}' of type '{type_name}' is not supported in NiFi {version}"
    ))]
    UnsupportedEnumVariant {
        /// The raw wire value of the unrecognised variant.
        variant: String,
        /// The Rust type name of the enum.
        type_name: String,
        /// The NiFi version that produced the variant.
        version: String,
    },

    /// A required field was absent when converting a dynamic response to a typed struct.
    #[snafu(display(
        "Required field '{field}' of type '{type_name}' is missing for NiFi {version}"
    ))]
    MissingRequiredField {
        /// The name of the missing field.
        field: String,
        /// The Rust type that expected the field.
        type_name: String,
        /// The NiFi version involved.
        version: String,
    },
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
