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

    #[snafu(display("NiFi API error (status {status}): {message}"))]
    Api { status: u16, message: String },
}
