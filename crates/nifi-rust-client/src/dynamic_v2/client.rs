//! `DynamicClientV2` — canonical-superset dynamic client (Phase 4a parallel build).
//!
//! Holds a `NifiClient` plus a once-detected version. Instead of dispatching
//! to per-version generated code, every endpoint method on the generated API
//! structs starts with `require_endpoint(Endpoint::FOO).await?` and then
//! issues a single canonical request. The version is detected lazily via
//! `/flow/about` the same way `DynamicClient` does today.

use crate::{NifiClient, NifiError};

/// `DynamicClientV2` — the new canonical dispatch path. Reachable only via
/// the `#[doc(hidden)]` `__dynamic_v2` re-export in Phase 4a.
pub struct DynamicClientV2 {
    client: NifiClient,
    version: tokio::sync::OnceCell<String>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AboutResponse {
    about: AboutInner,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AboutInner {
    version: String,
}

impl DynamicClientV2 {
    /// Wrap a `NifiClient`. Version detection happens lazily on the first call
    /// to a method that consults `require_endpoint`.
    pub fn new(client: NifiClient) -> Self {
        Self {
            client,
            version: tokio::sync::OnceCell::new(),
        }
    }

    /// Underlying `NifiClient` used by generated methods to issue HTTP requests.
    pub fn inner(&self) -> &NifiClient {
        &self.client
    }

    /// Authenticate and detect the server version. Mirrors the legacy
    /// `DynamicClient::login`.
    pub async fn login(&self, username: &str, password: &str) -> Result<(), NifiError> {
        self.client.login(username, password).await?;
        self.detect_version().await?;
        Ok(())
    }

    /// Detect the NiFi server version via `GET /flow/about`. Idempotent.
    pub async fn detect_version(&self) -> Result<&str, NifiError> {
        let v = self
            .version
            .get_or_try_init(|| async {
                let resp: AboutResponse = self.client.get("/flow/about").await?;
                Ok::<String, NifiError>(resp.about.version)
            })
            .await?;
        Ok(v.as_str())
    }

    /// Currently-detected version string, or an empty string if
    /// `detect_version` has not yet completed. Returned as an owned `String`
    /// so the generated URL-building code can pass it by value into the
    /// `UnsupportedQueryParam` error constructor without cloning.
    pub fn detected_version_str(&self) -> String {
        self.version.get().cloned().unwrap_or_default()
    }

    /// Returns `Ok(())` if `endpoint` is supported by the currently-detected
    /// server version. Detects the version lazily if not yet done.
    pub async fn require_endpoint(
        &self,
        endpoint: super::availability::Endpoint,
    ) -> Result<(), NifiError> {
        let version = self.detect_version().await?.to_string();
        if Self::supports_with(endpoint, &version) {
            Ok(())
        } else {
            Err(NifiError::UnsupportedEndpoint {
                endpoint: endpoint.as_str().to_string(),
                version,
            })
        }
    }

    /// Synchronous variant: returns `true` if `endpoint` is supported by the
    /// currently-detected version. Returns `false` if no version has been
    /// detected yet (callers should `detect_version().await` first).
    pub fn supports(&self, endpoint: super::availability::Endpoint) -> bool {
        match self.version.get() {
            Some(v) => Self::supports_with(endpoint, v),
            None => false,
        }
    }

    fn supports_with(endpoint: super::availability::Endpoint, version: &str) -> bool {
        super::availability::ENDPOINT_AVAILABILITY
            .iter()
            .find(|(e, _)| *e == endpoint)
            .map(|(_, versions)| versions.iter().any(|v| *v == version))
            .unwrap_or(false)
    }
}
