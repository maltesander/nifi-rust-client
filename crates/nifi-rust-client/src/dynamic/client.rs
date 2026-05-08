//! `DynamicClient` — canonical-superset dynamic client.
//!
//! Holds a `NifiClient`, a `VersionResolutionStrategy`, and a once-detected
//! version. Every endpoint method on the generated API structs starts with
//! `require_endpoint(Endpoint::FOO).await?` and then issues a single
//! canonical request. Cluster discovery mirrors the legacy `DynamicClient`.

use crate::{NifiClient, NifiError};

/// `DynamicClient` — the new canonical dispatch path. Reachable only via
/// the `#[doc(hidden)]` `dynamic` re-export in Phase 4a.
pub struct DynamicClient {
    client: NifiClient,
    version: tokio::sync::OnceCell<super::availability::DetectedVersion>,
    strategy: crate::dynamic::strategy::VersionResolutionStrategy,
    cluster_node_id: tokio::sync::OnceCell<Option<String>>,
}

impl std::fmt::Debug for DynamicClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicClient")
            .field("version", &self.version.get())
            .field("strategy", &self.strategy)
            .field("cluster_node_id", &self.cluster_node_id.get())
            .finish()
    }
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

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClusterSummaryResponse {
    cluster_summary: ClusterSummaryInner,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClusterSummaryInner {
    clustered: bool,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClusterResponse {
    cluster: ClusterInner,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClusterInner {
    nodes: Vec<ClusterNode>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClusterNode {
    node_id: Option<String>,
    status: Option<String>,
}

impl DynamicClient {
    /// Wrap a `NifiClient`. Uses the default `VersionResolutionStrategy`.
    /// Version detection happens lazily on the first call to a method that
    /// consults `require_endpoint`.
    pub fn new(client: NifiClient) -> Self {
        Self {
            client,
            version: tokio::sync::OnceCell::new(),
            strategy: crate::dynamic::strategy::VersionResolutionStrategy::default(),
            cluster_node_id: tokio::sync::OnceCell::new(),
        }
    }

    /// Wrap a `NifiClient` with a specific `VersionResolutionStrategy`.
    pub fn with_strategy(
        client: NifiClient,
        strategy: crate::dynamic::strategy::VersionResolutionStrategy,
    ) -> Self {
        Self {
            client,
            version: tokio::sync::OnceCell::new(),
            strategy,
            cluster_node_id: tokio::sync::OnceCell::new(),
        }
    }

    /// Async constructor that eagerly detects the version and discovers cluster
    /// topology. Mirrors the legacy `DynamicClient::from_client`.
    pub async fn from_client(client: NifiClient) -> Result<Self, NifiError> {
        let dc = Self::new(client);
        dc.detect_version().await?;
        dc.discover_cluster().await;
        Ok(dc)
    }

    /// Underlying `NifiClient` used by generated methods to issue HTTP requests.
    pub fn inner(&self) -> &NifiClient {
        &self.client
    }

    /// The configured `VersionResolutionStrategy`.
    pub fn strategy(&self) -> crate::dynamic::strategy::VersionResolutionStrategy {
        self.strategy
    }

    /// Authenticate and detect the server version. Mirrors the legacy
    /// `DynamicClient::login`.
    pub async fn login(&self, username: &str, password: &str) -> Result<(), NifiError> {
        self.client.login(username, password).await?;
        self.detect_version().await?;
        self.discover_cluster().await;
        Ok(())
    }

    /// Log out the current session.
    pub async fn logout(&self) -> Result<(), NifiError> {
        self.client.logout().await
    }

    /// Return the current bearer token, if any.
    ///
    /// Useful for persisting the token between process restarts. The returned
    /// [`zeroize::Zeroizing<String>`] is wiped on drop. See [`NifiClient::token`]
    /// for details.
    pub async fn token(&self) -> Option<zeroize::Zeroizing<String>> {
        self.client.token().await
    }

    /// Restore a previously obtained bearer token.
    ///
    /// Accepts both `String` and [`zeroize::Zeroizing<String>`] via [`Into`].
    /// See [`NifiClient::set_token`] for details.
    pub async fn set_token(&self, token: impl Into<zeroize::Zeroizing<String>>) {
        self.client.set_token(token).await;
    }

    /// Re-authenticate using the configured [`AuthProvider`](crate::config::auth::AuthProvider).
    ///
    /// See [`NifiClient::authenticate`] for details.
    pub async fn authenticate(&self) -> Result<(), NifiError> {
        self.client.authenticate().await
    }

    /// Detect the NiFi server version via `GET /flow/about`. Idempotent —
    /// subsequent calls return the cached result. Honors the configured
    /// [`VersionResolutionStrategy`](crate::dynamic::strategy::VersionResolutionStrategy).
    pub async fn detect_version(
        &self,
    ) -> Result<super::availability::DetectedVersion, NifiError> {
        let strategy = self.strategy;
        self.version
            .get_or_try_init(|| async {
                let resp: AboutResponse = self.client.get("/flow/about", &[]).await?;
                crate::dynamic::strategy::resolve_version(
                    &resp.about.version,
                    strategy,
                    super::availability::version_from_str,
                    super::availability::SUPPORTED_VERSIONS,
                )
            })
            .await
            .copied()
    }

    /// Currently-detected version, or `None` if `detect_version` has not yet
    /// completed successfully.
    pub fn detected_version(&self) -> Option<super::availability::DetectedVersion> {
        self.version.get().copied()
    }

    /// Currently-detected version string, or an empty string if
    /// `detect_version` has not yet completed. Returned as an owned `String`
    /// so the generated URL-building code can pass it by value into the
    /// `UnsupportedQueryParam` error constructor without cloning.
    pub fn detected_version_str(&self) -> String {
        self.version.get().map(|v| v.to_string()).unwrap_or_default()
    }

    /// The cluster node ID of the connected (primary) node, if this NiFi
    /// instance is part of a cluster. `None` for standalone instances or if
    /// cluster discovery has not yet been performed.
    pub fn cluster_node_id(&self) -> Option<&str> {
        self.cluster_node_id.get().and_then(|opt| opt.as_deref())
    }

    /// Discover cluster topology. Called from `login` and `from_client`.
    /// Silently ignores errors (non-clustered instances return a non-cluster
    /// summary; the cluster endpoint may be unauthorized).
    ///
    /// Audit follow-up A8: errors must NOT poison the `OnceCell`. Previously
    /// the closure was driven by `get_or_init` and returned `None` on every
    /// error path — that filled the cell with `Some(None)`, so a JWT refresh
    /// or transient 401 on the discovery request was permanently cached as
    /// "standalone instance". The fix splits success and failure: errors
    /// leave the cell empty so a future retry can reattempt; only a definitive
    /// answer (clustered → optional node id, or standalone → `None`) is
    /// recorded, and via `set` so `get_or_init`'s implicit retry-on-empty
    /// semantics are preserved.
    async fn discover_cluster(&self) {
        // Already discovered? Nothing to do.
        if self.cluster_node_id.get().is_some() {
            return;
        }

        let summary: Result<ClusterSummaryResponse, NifiError> =
            self.client.get("/flow/cluster/summary", &[]).await;
        let summary = match summary {
            Ok(s) => s,
            // Leave the cell empty so the next call retries. Standalone
            // instances reach this branch only via a real error — successful
            // discovery on a non-cluster goes through the `Ok` branch below
            // and stores `None`.
            Err(_) => return,
        };

        if !summary.cluster_summary.clustered {
            // Definitive standalone answer — cache it.
            let _ = self.cluster_node_id.set(None);
            return;
        }

        let cluster: Result<ClusterResponse, NifiError> =
            self.client.get("/controller/cluster", &[]).await;
        let cluster = match cluster {
            Ok(c) => c,
            // Cluster endpoint failed — don't poison; allow retry.
            Err(_) => return,
        };

        let node_id = cluster
            .cluster
            .nodes
            .iter()
            .find(|n| n.status.as_deref() == Some("CONNECTED"))
            .and_then(|n| n.node_id.clone());

        let _ = self.cluster_node_id.set(node_id);
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
            Some(v) => Self::supports_with(endpoint, &v.to_string()),
            None => false,
        }
    }

    fn supports_with(endpoint: super::availability::Endpoint, version: &str) -> bool {
        super::availability::ENDPOINT_AVAILABILITY
            .iter()
            .find(|(e, _)| *e == endpoint)
            .map(|(_, versions)| versions.contains(&version))
            .unwrap_or(false)
    }
}
