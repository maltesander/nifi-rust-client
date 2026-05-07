#![deny(missing_docs)]
use std::sync::Arc;
use std::time::Duration;

use snafu::ResultExt as _;
use url::Url;

use crate::NifiClient;
use crate::NifiError;
use crate::config::auth::AuthProvider;
use crate::error::{HttpSnafu, InvalidBaseUrlSnafu, InvalidCertificateSnafu};

/// Fires once per process when a `NifiClient` is built with TLS
/// verification disabled. Test-visible so unit tests can reset / inspect.
static INVALID_CERTS_WARNED: std::sync::OnceLock<()> = std::sync::OnceLock::new();

fn warn_invalid_certs_once() {
    if INVALID_CERTS_WARNED.set(()).is_ok() {
        eprintln!("warning: TLS verification disabled — production use is dangerous");
    }
}

/// Validate the structural shape of a `X-ProxiedEntitiesChain` value.
///
/// Implements the regex `^(<[^<>\r\n]+>)+$` as a single-pass char walk so
/// we don't pull in `regex` as a runtime dependency. Each entity must be
/// angle-bracketed, non-empty, and contain no embedded `<`, `>`, CR, or LF.
fn validate_proxied_entities_chain(s: &str) -> Result<(), NifiError> {
    let invalid = || NifiError::Configuration {
        message: format!(
            "proxied_entities_chain({s:?}) must match `<id1><id2>…` — \
             one or more angle-bracketed entities, no embedded `<`, `>`, CR, or LF"
        ),
    };

    if s.is_empty() {
        return Err(invalid());
    }

    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'<' {
            return Err(invalid());
        }
        i += 1;
        let entity_start = i;
        while i < bytes.len()
            && bytes[i] != b'>'
            && bytes[i] != b'<'
            && bytes[i] != b'\r'
            && bytes[i] != b'\n'
        {
            i += 1;
        }
        if i == entity_start || i >= bytes.len() || bytes[i] != b'>' {
            return Err(invalid());
        }
        i += 1; // consume the closing '>'
    }
    Ok(())
}

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
/// let proxy_url = Url::parse("http://proxy.internal:3128")
///     .expect("hard-coded proxy URL is valid");
/// let client = NifiClientBuilder::new("https://nifi.example.com:8443")?
///     .timeout(Duration::from_secs(60))
///     .connect_timeout(Duration::from_secs(10))
///     .proxy(proxy_url)
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
    auth_provider: Option<Arc<dyn AuthProvider>>,
    client_identity: Option<reqwest::Identity>,
    proxied_entities_chain: Option<String>,
    retry_policy: Option<crate::config::retry::RetryPolicy>,
    request_id_header: Option<String>,
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
                "auth_provider",
                &self.auth_provider.as_ref().map(|c| format!("{c:?}")),
            )
            .field(
                "client_identity",
                &self.client_identity.as_ref().map(|_| "<identity>"),
            )
            .field("proxied_entities_chain", &self.proxied_entities_chain)
            .field("retry_policy", &self.retry_policy)
            .field("request_id_header", &self.request_id_header);
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
            auth_provider: None,
            client_identity: None,
            proxied_entities_chain: None,
            retry_policy: None,
            request_id_header: None,
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

    /// Configure an [`AuthProvider`] for authentication and automatic token refresh.
    ///
    /// When set, the client will automatically re-authenticate on 401 responses
    /// by calling the provider and then re-issuing the failed request.
    pub fn auth_provider(mut self, provider: impl AuthProvider + 'static) -> Self {
        self.auth_provider = Some(Arc::new(provider));
        self
    }

    /// Attach a PEM-encoded client identity for mTLS.
    ///
    /// The `pem` bytes should contain the concatenated PEM-encoded private key
    /// and certificate chain. The private key must be in RSA, SEC1 Elliptic
    /// Curve, or PKCS#8 format.
    pub fn client_identity_pem(mut self, pem: &[u8]) -> Result<Self, NifiError> {
        let identity = reqwest::Identity::from_pem(pem).context(InvalidCertificateSnafu)?;
        self.client_identity = Some(identity);
        Ok(self)
    }

    /// Set the `X-ProxiedEntitiesChain` header sent with every request.
    ///
    /// This header is used in NiFi proxy deployments to propagate the end-user
    /// identity through one or more reverse proxies. The value must match the
    /// shape `<id1><id2>…<idN>` — at least one angle-bracketed entity, no
    /// embedded `<`, `>`, CR, or LF inside an entity. Invalid values surface
    /// as `NifiError::Configuration` instead of producing a confusing 400
    /// from NiFi on the first request.
    pub fn proxied_entities_chain(mut self, chain: impl Into<String>) -> Result<Self, NifiError> {
        let chain = chain.into();
        validate_proxied_entities_chain(&chain)?;
        self.proxied_entities_chain = Some(chain);
        Ok(self)
    }

    /// Configure a [`RetryPolicy`](crate::config::retry::RetryPolicy) for transient error retry.
    ///
    /// When set, HTTP helpers automatically retry
    /// [retryable](crate::NifiError::is_retryable) errors using exponential backoff.
    pub fn retry_policy(mut self, policy: crate::config::retry::RetryPolicy) -> Self {
        self.retry_policy = Some(policy);
        self
    }

    /// Enable per-request correlation IDs.
    ///
    /// When `Some(name)`, every outgoing request carries a fresh UUIDv4
    /// in the given header, and the same id is attached to the per-request
    /// tracing span as the `request_id` field.
    ///
    /// Example: `.request_id_header(Some("X-Request-Id"))`.
    ///
    /// When `None` (default), no header is sent and no `request_id` field
    /// is recorded.
    pub fn request_id_header(mut self, name: Option<impl Into<String>>) -> Self {
        self.request_id_header = name.map(Into::into);
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
        if self.danger_accept_invalid_certs {
            warn_invalid_certs_once();
        }

        // Validate the configured request-id header name eagerly so that
        // a misconfiguration surfaces at `.build()` rather than as a 400
        // (or panic) on the first outbound request. We only need to
        // verify acceptance — the validated `HeaderName` is rebuilt in
        // `client.rs` per request.
        if let Some(name) = self.request_id_header.as_deref() {
            reqwest::header::HeaderName::try_from(name).map_err(|_| NifiError::Configuration {
                message: format!("request_id_header({name:?}) is not a valid HTTP header name"),
            })?;
        }

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

        if let Some(identity) = self.client_identity {
            builder = builder.identity(identity);
        }

        let http = builder.build().context(HttpSnafu)?;
        Ok(NifiClient::from_parts(
            self.base_url,
            http,
            self.auth_provider,
            self.proxied_entities_chain,
            self.retry_policy,
            self.request_id_header,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Building with `danger_accept_invalid_certs(true)` must trip the
    /// process-wide `OnceLock` guard so the warning is emitted at most
    /// once. We can't easily capture stderr from a `eprintln!` inside the
    /// same process without forking, so this test verifies the lock state
    /// instead — the audit doc explicitly allows this fallback.
    #[test]
    fn invalid_certs_sets_once_lock() {
        let _ = NifiClientBuilder::new("https://example.com")
            .expect("hard-coded URL is valid")
            .danger_accept_invalid_certs(true)
            .build()
            .expect("builder succeeds with invalid-certs flag");
        assert!(
            INVALID_CERTS_WARNED.get().is_some(),
            "OnceLock should be set after a build with invalid-certs"
        );
    }

    #[test]
    fn request_id_header_rejects_invalid_name_at_build_time() {
        let err = NifiClientBuilder::new("https://example.com")
            .expect("hard-coded URL is valid")
            .request_id_header(Some("X Foo")) // space → invalid HTTP header
            .build()
            .expect_err("invalid header should error at build time");
        match err {
            NifiError::Configuration { message } => {
                assert!(
                    message.contains("X Foo"),
                    "error should name the offending value: {message}"
                );
            }
            other => panic!("expected NifiError::Configuration, got {other:?}"),
        }
    }

    #[test]
    fn request_id_header_accepts_valid_name() {
        let _ = NifiClientBuilder::new("https://example.com")
            .expect("hard-coded URL is valid")
            .request_id_header(Some("X-Request-Id"))
            .build()
            .expect("valid header name should build");
    }

    #[test]
    fn proxied_entities_chain_rejects_unframed_value() {
        let err = NifiClientBuilder::new("https://example.com")
            .expect("hard-coded URL is valid")
            .proxied_entities_chain("alice")
            .expect_err("unframed entity must error at builder time");
        match err {
            NifiError::Configuration { message } => {
                assert!(
                    message.contains("alice"),
                    "error should name the offending value: {message}"
                );
            }
            other => panic!("expected NifiError::Configuration, got {other:?}"),
        }
    }

    #[test]
    fn proxied_entities_chain_accepts_well_formed_value() {
        let _ = NifiClientBuilder::new("https://example.com")
            .expect("hard-coded URL is valid")
            .proxied_entities_chain("<alice>")
            .expect("single framed entity should validate")
            .build()
            .expect("build should succeed");
    }

    #[test]
    fn proxied_entities_chain_accepts_multiple_entities() {
        let _ = NifiClientBuilder::new("https://example.com")
            .expect("hard-coded URL is valid")
            .proxied_entities_chain("<alice><bob><CN=svc>")
            .expect("multiple framed entities should validate");
    }

    #[test]
    fn proxied_entities_chain_rejects_empty_entity() {
        assert!(matches!(
            validate_proxied_entities_chain("<>"),
            Err(NifiError::Configuration { .. })
        ));
    }

    #[test]
    fn proxied_entities_chain_rejects_unclosed() {
        assert!(matches!(
            validate_proxied_entities_chain("<alice"),
            Err(NifiError::Configuration { .. })
        ));
    }

    #[test]
    fn proxied_entities_chain_rejects_embedded_newline() {
        assert!(matches!(
            validate_proxied_entities_chain("<ali\nce>"),
            Err(NifiError::Configuration { .. })
        ));
    }

    #[test]
    fn proxied_entities_chain_rejects_empty_string() {
        assert!(matches!(
            validate_proxied_entities_chain(""),
            Err(NifiError::Configuration { .. })
        ));
    }
}
