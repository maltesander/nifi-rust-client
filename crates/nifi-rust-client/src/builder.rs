use std::sync::Arc;
use std::time::Duration;

use snafu::ResultExt as _;
use url::Url;

use crate::NifiClient;
use crate::NifiError;
use crate::credentials::CredentialProvider;
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
pub struct NifiClientBuilder {
    base_url: Url,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    proxy_all: Option<Url>,
    proxy_http: Option<Url>,
    proxy_https: Option<Url>,
    danger_accept_invalid_certs: bool,
    root_certificates: Vec<Vec<u8>>,
    credential_provider: Option<Arc<dyn CredentialProvider>>,
    retry_policy: Option<crate::retry::RetryPolicy>,
    #[cfg(feature = "dynamic")]
    version_strategy: Option<crate::dynamic::VersionResolutionStrategy>,
}

impl std::fmt::Debug for NifiClientBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("NifiClientBuilder");
        s.field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .field("connect_timeout", &self.connect_timeout)
            .field("proxy_all", &self.proxy_all)
            .field("proxy_http", &self.proxy_http)
            .field("proxy_https", &self.proxy_https)
            .field(
                "danger_accept_invalid_certs",
                &self.danger_accept_invalid_certs,
            )
            .field(
                "root_certificates",
                &format!("[{} certs]", self.root_certificates.len()),
            )
            .field(
                "credential_provider",
                &self.credential_provider.as_ref().map(|c| format!("{c:?}")),
            )
            .field("retry_policy", &self.retry_policy);
        #[cfg(feature = "dynamic")]
        s.field("version_strategy", &self.version_strategy);
        s.finish()
    }
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
            credential_provider: None,
            retry_policy: None,
            #[cfg(feature = "dynamic")]
            version_strategy: None,
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

    /// Configure a [`CredentialProvider`] for automatic token refresh.
    ///
    /// When set, the client will automatically re-authenticate on 401 responses
    /// by calling the provider to obtain fresh credentials and then re-issuing
    /// the failed request.
    pub fn credential_provider(mut self, provider: impl CredentialProvider + 'static) -> Self {
        self.credential_provider = Some(Arc::new(provider));
        self
    }

    /// Configure a [`RetryPolicy`](crate::retry::RetryPolicy) for transient error retry.
    ///
    /// When set, HTTP helpers automatically retry
    /// [retryable](crate::NifiError::is_retryable) errors using exponential backoff.
    pub fn retry_policy(mut self, policy: crate::retry::RetryPolicy) -> Self {
        self.retry_policy = Some(policy);
        self
    }

    /// Configure a [`VersionResolutionStrategy`](crate::dynamic::VersionResolutionStrategy)
    /// for the dynamic client.
    ///
    /// Controls how the client resolves a detected NiFi version to a supported
    /// client version. Default is `Strict`.
    #[cfg(feature = "dynamic")]
    pub fn version_strategy(mut self, strategy: crate::dynamic::VersionResolutionStrategy) -> Self {
        self.version_strategy = Some(strategy);
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
        Ok(NifiClient::from_parts(
            self.base_url,
            http,
            self.credential_provider,
            self.retry_policy,
        ))
    }

    /// Build a [`DynamicClient`](crate::dynamic::DynamicClient) that auto-detects the NiFi version.
    ///
    /// Version detection happens lazily — either when `login()` is called
    /// (recommended) or when `detect_version()` is called explicitly.
    ///
    /// Uses the configured [`VersionResolutionStrategy`](crate::dynamic::VersionResolutionStrategy)
    /// (default: `Strict`). Set via [`.version_strategy()`](Self::version_strategy).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), nifi_rust_client::NifiError> {
    /// use nifi_rust_client::NifiClientBuilder;
    /// use nifi_rust_client::dynamic::VersionResolutionStrategy;
    ///
    /// let client = NifiClientBuilder::new("https://nifi.example.com:8443")?
    ///     .danger_accept_invalid_certs(true)
    ///     .version_strategy(VersionResolutionStrategy::Closest)
    ///     .build_dynamic()?;
    ///
    /// // login() authenticates AND detects the NiFi version automatically.
    /// client.login("admin", "password").await?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "dynamic")]
    pub fn build_dynamic(self) -> Result<crate::dynamic::DynamicClient, NifiError> {
        let strategy = self.version_strategy.unwrap_or_default();
        let client = self.build()?;
        Ok(crate::dynamic::DynamicClient::with_strategy(
            client, strategy,
        ))
    }
}
