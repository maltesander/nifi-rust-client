use std::time::Duration;

use snafu::ResultExt as _;
use url::Url;

use crate::NifiClient;
use crate::NifiError;
use crate::error::{HttpSnafu, InvalidBaseUrlSnafu, InvalidCertificateSnafu};

/// Builder for [`NifiClient`].
///
/// Use this when you need to configure timeouts, proxies, or TLS options beyond
/// the defaults provided by the convenience constructors.
///
/// # Example
///
/// ```no_run
/// use std::time::Duration;
/// use nifi_rust_client::NifiClientBuilder;
/// use url::Url;
///
/// # async fn example() -> Result<(), nifi_rust_client::NifiError> {
/// let client = NifiClientBuilder::new("https://nifi.example.com:8443")?
///     .timeout(Duration::from_secs(60))
///     .connect_timeout(Duration::from_secs(10))
///     .proxy(Url::parse("http://proxy.internal:3128").unwrap())
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct NifiClientBuilder {
    base_url: Url,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    proxy_all: Option<Url>,
    proxy_http: Option<Url>,
    proxy_https: Option<Url>,
    danger_accept_invalid_certs: bool,
    root_certificates: Vec<Vec<u8>>,
}

impl NifiClientBuilder {
    /// Create a new builder targeting the given NiFi base URL.
    ///
    /// Returns an error if `base_url` cannot be parsed.
    pub fn new(base_url: &str) -> Result<Self, NifiError> {
        let base_url = Url::parse(base_url).context(InvalidBaseUrlSnafu)?;
        Ok(Self {
            base_url,
            timeout: None,
            connect_timeout: None,
            proxy_all: None,
            proxy_http: None,
            proxy_https: None,
            danger_accept_invalid_certs: false,
            root_certificates: Vec::new(),
        })
    }

    /// Set the total request timeout.
    ///
    /// The timeout applies from when the request starts connecting until the
    /// response body is fully received.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = Some(duration);
        self
    }

    /// Set the TCP connection timeout.
    pub fn connect_timeout(mut self, duration: Duration) -> Self {
        self.connect_timeout = Some(duration);
        self
    }

    /// Route all traffic (HTTP and HTTPS) through the given proxy.
    pub fn proxy(mut self, url: Url) -> Self {
        self.proxy_all = Some(url);
        self
    }

    /// Route HTTP traffic through the given proxy.
    pub fn http_proxy(mut self, url: Url) -> Self {
        self.proxy_http = Some(url);
        self
    }

    /// Route HTTPS traffic through the given proxy.
    pub fn https_proxy(mut self, url: Url) -> Self {
        self.proxy_https = Some(url);
        self
    }

    /// Skip TLS certificate verification.
    ///
    /// Only use this in development against self-signed certificates.
    pub fn danger_accept_invalid_certs(mut self, accept: bool) -> Self {
        self.danger_accept_invalid_certs = accept;
        self
    }

    /// Trust an additional PEM-encoded CA certificate.
    ///
    /// May be called multiple times to add more than one certificate.
    pub fn add_root_certificate(mut self, pem: &[u8]) -> Self {
        self.root_certificates.push(pem.to_vec());
        self
    }

    /// Build the [`NifiClient`].
    pub fn build(self) -> Result<NifiClient, NifiError> {
        let mut builder = reqwest::Client::builder()
            .danger_accept_invalid_certs(self.danger_accept_invalid_certs);

        if let Some(d) = self.timeout {
            builder = builder.timeout(d);
        }
        if let Some(d) = self.connect_timeout {
            builder = builder.connect_timeout(d);
        }

        for pem in &self.root_certificates {
            let cert = reqwest::Certificate::from_pem(pem).context(InvalidCertificateSnafu)?;
            builder = builder.add_root_certificate(cert);
        }

        if let Some(url) = self.proxy_all {
            let proxy = reqwest::Proxy::all(url.as_str()).context(HttpSnafu)?;
            builder = builder.proxy(proxy);
        }
        if let Some(url) = self.proxy_http {
            let proxy = reqwest::Proxy::http(url.as_str()).context(HttpSnafu)?;
            builder = builder.proxy(proxy);
        }
        if let Some(url) = self.proxy_https {
            let proxy = reqwest::Proxy::https(url.as_str()).context(HttpSnafu)?;
            builder = builder.proxy(proxy);
        }

        let http = builder.build().context(HttpSnafu)?;
        Ok(NifiClient::from_parts(self.base_url, http))
    }
}
