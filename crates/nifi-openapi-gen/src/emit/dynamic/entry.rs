use std::collections::BTreeMap;

use crate::parser::ApiSpec;
use crate::util::{collect_all_tags, version_to_variant};

/// Input tuple: `(version_str, mod_name, feature_flag, &ApiSpec)`
/// e.g. `("2.7.2", "v2_7_2", "nifi-2-7-2", &spec)`
///
/// Returns the content for `dynamic/mod.rs`.
pub fn emit_dynamic(versions: &[(&str, &str, &str, &ApiSpec)]) -> String {
    let mut out = String::new();

    // Module declarations
    out.push_str("pub mod strategy;\n");
    out.push_str("pub mod traits;\n");
    out.push_str("pub mod dispatch;\n");
    out.push_str("pub mod types;\n");
    out.push_str("mod impls;\n");
    out.push_str("mod conversions;\n\n");

    out.push_str("pub use strategy::VersionResolutionStrategy;\n\n");

    // Imports
    out.push_str("use crate::{NifiClient, NifiError};\n\n");

    // DetectedVersion enum
    emit_detected_version(&mut out, versions);

    // version_from_str function
    emit_version_from_str(&mut out, versions);

    // AboutResponse / AboutInner deserialization structs
    emit_about_structs(&mut out);

    // ClusterSummaryResponse / ClusterResponse deserialization structs
    emit_cluster_structs(&mut out);

    // SUPPORTED_VERSIONS constant
    emit_supported_versions(&mut out, versions);

    // DynamicClient struct and impl
    emit_dynamic_client(&mut out, versions);

    crate::util::format_source(&out)
}

fn emit_detected_version(out: &mut String, versions: &[(&str, &str, &str, &ApiSpec)]) {
    out.push_str("/// Represents a detected NiFi server version.\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    out.push_str("pub enum DetectedVersion {\n");
    for (ver, _, _, _) in versions {
        out.push_str(&format!("    {},\n", version_to_variant(ver)));
    }
    out.push_str("}\n\n");

    // Display impl
    out.push_str("impl std::fmt::Display for DetectedVersion {\n");
    out.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    out.push_str("        match self {\n");
    for (ver, _, _, _) in versions {
        out.push_str(&format!(
            "            DetectedVersion::{} => write!(f, \"{}\"),\n",
            version_to_variant(ver),
            ver,
        ));
    }
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn emit_version_from_str(out: &mut String, versions: &[(&str, &str, &str, &ApiSpec)]) {
    out.push_str("/// Match a version string by major.minor (ignoring patch).\n");
    out.push_str("fn version_from_str(version: &str) -> Result<DetectedVersion, NifiError> {\n");
    out.push_str("    let parts: Vec<&str> = version.split('.').collect();\n");
    out.push_str("    if parts.len() < 2 {\n");
    out.push_str(
        "        return Err(NifiError::UnsupportedVersion { detected: version.to_string() });\n",
    );
    out.push_str("    }\n");
    out.push_str("    let major_minor = format!(\"{}.{}\", parts[0], parts[1]);\n");
    out.push_str("    match major_minor.as_str() {\n");

    // Group versions by major.minor, pick the first (lowest patch) for each
    let mut major_minor_map: BTreeMap<String, &str> = BTreeMap::new();
    for (ver, _, _, _) in versions {
        let parts: Vec<&str> = ver.split('.').collect();
        if parts.len() >= 2 {
            let mm = format!("{}.{}", parts[0], parts[1]);
            major_minor_map.entry(mm).or_insert(ver);
        }
    }
    for (mm, ver) in &major_minor_map {
        out.push_str(&format!(
            "        \"{}\" => Ok(DetectedVersion::{}),\n",
            mm,
            version_to_variant(ver),
        ));
    }
    out.push_str(
        "        _ => Err(NifiError::UnsupportedVersion { detected: version.to_string() }),\n",
    );
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn emit_about_structs(out: &mut String) {
    out.push_str("#[derive(serde::Deserialize)]\n");
    out.push_str("#[serde(rename_all = \"camelCase\")]\n");
    out.push_str("struct AboutResponse {\n");
    out.push_str("    about: AboutInner,\n");
    out.push_str("}\n\n");

    out.push_str("#[derive(serde::Deserialize)]\n");
    out.push_str("#[serde(rename_all = \"camelCase\")]\n");
    out.push_str("struct AboutInner {\n");
    out.push_str("    version: String,\n");
    out.push_str("}\n\n");
}

fn emit_cluster_structs(out: &mut String) {
    // ClusterSummaryResponse wraps ClusterSummaryInner
    out.push_str("#[derive(serde::Deserialize)]\n");
    out.push_str("#[serde(rename_all = \"camelCase\")]\n");
    out.push_str("struct ClusterSummaryResponse {\n");
    out.push_str("    cluster_summary: ClusterSummaryInner,\n");
    out.push_str("}\n\n");

    out.push_str("#[derive(serde::Deserialize)]\n");
    out.push_str("#[serde(rename_all = \"camelCase\")]\n");
    out.push_str("struct ClusterSummaryInner {\n");
    out.push_str("    clustered: bool,\n");
    out.push_str("}\n\n");

    // ClusterResponse wraps ClusterInner with a Vec of nodes
    out.push_str("#[derive(serde::Deserialize)]\n");
    out.push_str("#[serde(rename_all = \"camelCase\")]\n");
    out.push_str("struct ClusterResponse {\n");
    out.push_str("    cluster: ClusterInner,\n");
    out.push_str("}\n\n");

    out.push_str("#[derive(serde::Deserialize)]\n");
    out.push_str("#[serde(rename_all = \"camelCase\")]\n");
    out.push_str("struct ClusterInner {\n");
    out.push_str("    nodes: Vec<ClusterNode>,\n");
    out.push_str("}\n\n");

    out.push_str("#[derive(serde::Deserialize)]\n");
    out.push_str("#[serde(rename_all = \"camelCase\")]\n");
    out.push_str("struct ClusterNode {\n");
    out.push_str("    node_id: Option<String>,\n");
    out.push_str("    status: Option<String>,\n");
    out.push_str("}\n\n");
}

fn emit_supported_versions(out: &mut String, versions: &[(&str, &str, &str, &ApiSpec)]) {
    out.push_str("/// All supported versions compiled into this build, used by version resolution strategies.\n");
    out.push_str("const SUPPORTED_VERSIONS: &[(&str, DetectedVersion)] = &[\n");
    for (ver, _, _, _) in versions {
        out.push_str(&format!(
            "    (\"{}\", DetectedVersion::{}),\n",
            ver,
            version_to_variant(ver),
        ));
    }
    out.push_str("];\n\n");

    if let Some((latest, _, _, _)) = versions.last() {
        out.push_str("/// The semver-latest NiFi version supported by this build.\n");
        out.push_str("///\n");
        out.push_str("/// Updated automatically when a new spec is added via the generator.\n");
        out.push_str(&format!(
            "pub const LATEST_NIFI_VERSION: &str = \"{latest}\";\n\n"
        ));
    }
}

fn emit_dynamic_client(out: &mut String, versions: &[(&str, &str, &str, &ApiSpec)]) {
    // Struct
    out.push_str("/// A dynamic NiFi client that detects the server version lazily\n");
    out.push_str("/// and dispatches API calls to the correct version's generated code.\n");
    out.push_str("///\n");
    out.push_str("/// Version detection happens automatically on `login()`, or can be triggered\n");
    out.push_str("/// explicitly via `detect_version()` or `from_client()`.\n");
    out.push_str("pub struct DynamicClient {\n");
    out.push_str("    client: NifiClient,\n");
    out.push_str("    version: tokio::sync::OnceCell<DetectedVersion>,\n");
    out.push_str("    strategy: strategy::VersionResolutionStrategy,\n");
    out.push_str("    cluster_node_id: tokio::sync::OnceCell<Option<String>>,\n");
    out.push_str("}\n\n");

    // Manual Debug impl (OnceCell<T> is Debug if T is Debug)
    out.push_str("impl std::fmt::Debug for DynamicClient {\n");
    out.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    out.push_str("        f.debug_struct(\"DynamicClient\")\n");
    out.push_str("            .field(\"version\", &self.version.get())\n");
    out.push_str("            .field(\"strategy\", &self.strategy)\n");
    out.push_str("            .field(\"cluster_node_id\", &self.cluster_node_id.get())\n");
    out.push_str("            .finish()\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("impl DynamicClient {\n");

    // new()
    out.push_str(
        "    /// Create a new `DynamicClient` without detecting the server version yet.\n",
    );
    out.push_str("    /// Version detection will happen automatically on `login()` or can be\n");
    out.push_str("    /// triggered explicitly via `detect_version()`.\n");
    out.push_str("    ///\n");
    out.push_str("    /// Uses [`VersionResolutionStrategy::Strict`] by default.\n");
    out.push_str(
        "    /// Use [`with_strategy`](Self::with_strategy) to configure fallback behavior.\n",
    );
    out.push_str("    pub fn new(client: NifiClient) -> Self {\n");
    out.push_str("        Self { client, version: tokio::sync::OnceCell::new(), strategy: strategy::VersionResolutionStrategy::default(), cluster_node_id: tokio::sync::OnceCell::new() }\n");
    out.push_str("    }\n\n");

    // with_strategy()
    out.push_str(
        "    /// Create a new `DynamicClient` with a specific version resolution strategy.\n",
    );
    out.push_str("    ///\n");
    out.push_str("    /// See [`VersionResolutionStrategy`] for available strategies.\n");
    out.push_str("    pub fn with_strategy(client: NifiClient, strategy: strategy::VersionResolutionStrategy) -> Self {\n");
    out.push_str("        Self { client, version: tokio::sync::OnceCell::new(), strategy, cluster_node_id: tokio::sync::OnceCell::new() }\n");
    out.push_str("    }\n\n");

    // from_client() — eagerly detects (backwards-compatible)
    out.push_str("    /// Wrap an existing `NifiClient` and detect the NiFi server version\n");
    out.push_str("    /// immediately via GET /flow/about.\n");
    out.push_str("    ///\n");
    out.push_str(
        "    /// The client must already be authenticated if the NiFi instance requires it.\n",
    );
    out.push_str("    /// For unauthenticated setup, use `new()` + `login()` instead — login\n");
    out.push_str("    /// triggers version detection automatically.\n");
    out.push_str("    ///\n");
    out.push_str(
        "    /// Uses [`VersionResolutionStrategy::Strict`]. To configure fallback, use\n",
    );
    out.push_str("    /// [`with_strategy`](Self::with_strategy) + [`detect_version`](Self::detect_version) instead.\n");
    out.push_str("    pub async fn from_client(client: NifiClient) -> Result<Self, NifiError> {\n");
    out.push_str("        let dc = Self::new(client);\n");
    out.push_str("        dc.detect_version().await?;\n");
    out.push_str("        dc.discover_cluster().await;\n");
    out.push_str("        Ok(dc)\n");
    out.push_str("    }\n\n");

    // detect_version()
    out.push_str("    /// Detect the NiFi server version via GET /flow/about.\n");
    out.push_str("    /// Returns the cached version if already detected.\n");
    out.push_str("    ///\n");
    out.push_str(
        "    /// Resolution behavior depends on the configured [`VersionResolutionStrategy`].\n",
    );
    out.push_str(
        "    pub async fn detect_version(&self) -> Result<DetectedVersion, NifiError> {\n",
    );
    out.push_str("        let strategy = self.strategy;\n");
    out.push_str("        self.version.get_or_try_init(|| async {\n");
    out.push_str(
        "            let resp: AboutResponse = self.client.get(\"/flow/about\").await?;\n",
    );
    out.push_str("            strategy::resolve_version(&resp.about.version, strategy, version_from_str, SUPPORTED_VERSIONS)\n");
    out.push_str("        }).await.copied()\n");
    out.push_str("    }\n\n");

    // detected_version()
    out.push_str("    /// Returns the detected NiFi server version, if it has been detected.\n");
    out.push_str("    ///\n");
    out.push_str("    /// Returns `None` if called before [`login`](Self::login) or\n");
    out.push_str("    /// [`detect_version`](Self::detect_version) has populated the version.\n");
    out.push_str("    /// API calls on accessors like [`flow_api`](Self::flow_api) detect the\n");
    out.push_str("    /// version lazily and propagate transport errors normally — this method\n");
    out.push_str("    /// is only useful for inspection after detection has already happened.\n");
    out.push_str("    pub fn detected_version(&self) -> Option<DetectedVersion> {\n");
    out.push_str("        self.version.get().copied()\n");
    out.push_str("    }\n\n");

    // inner()
    out.push_str("    /// Returns a reference to the underlying `NifiClient`.\n");
    out.push_str("    pub fn inner(&self) -> &NifiClient {\n");
    out.push_str("        &self.client\n");
    out.push_str("    }\n\n");

    // strategy()
    out.push_str("    /// Returns the configured version resolution strategy.\n");
    out.push_str("    pub fn strategy(&self) -> strategy::VersionResolutionStrategy {\n");
    out.push_str("        self.strategy\n");
    out.push_str("    }\n\n");

    // discover_cluster()
    out.push_str(
        "    /// Discover the cluster node ID by querying the cluster summary and nodes.\n",
    );
    out.push_str(
        "    /// On standalone NiFi (not clustered or if the calls fail), stores `None`.\n",
    );
    out.push_str("    /// Idempotent — only runs the discovery once.\n");
    out.push_str("    async fn discover_cluster(&self) {\n");
    out.push_str("        let _ = self.cluster_node_id.get_or_init(|| async {\n");
    out.push_str("            let summary: Result<ClusterSummaryResponse, NifiError> = self.client.get(\"/flow/cluster/summary\").await;\n");
    out.push_str("            match summary {\n");
    out.push_str("                Ok(s) if s.cluster_summary.clustered => {\n");
    out.push_str("                    let cluster: Result<ClusterResponse, NifiError> = self.client.get(\"/controller/cluster\").await;\n");
    out.push_str("                    match cluster {\n");
    out.push_str("                        Ok(c) => c.cluster.nodes.iter()\n");
    out.push_str(
        "                            .find(|n| n.status.as_deref() == Some(\"CONNECTED\"))\n",
    );
    out.push_str("                            .and_then(|n| n.node_id.clone()),\n");
    out.push_str("                        Err(_) => None,\n");
    out.push_str("                    }\n");
    out.push_str("                }\n");
    out.push_str("                _ => None,\n");
    out.push_str("            }\n");
    out.push_str("        }).await;\n");
    out.push_str("    }\n\n");

    // cluster_node_id()
    out.push_str(
        "    /// Returns the discovered cluster node ID, if this NiFi instance is clustered.\n",
    );
    out.push_str("    ///\n");
    out.push_str("    /// Returns `None` for standalone NiFi instances, or if cluster discovery\n");
    out.push_str("    /// has not yet run (call [`login`](Self::login) or [`from_client`](Self::from_client) first).\n");
    out.push_str("    ///\n");
    out.push_str(
        "    /// Provenance and lineage dispatch methods auto-inject this value when the\n",
    );
    out.push_str("    /// caller passes `None` for `cluster_node_id`.\n");
    out.push_str("    pub fn cluster_node_id(&self) -> Option<&str> {\n");
    out.push_str("        self.cluster_node_id.get().and_then(|opt| opt.as_deref())\n");
    out.push_str("    }\n\n");

    // login() — now also triggers version detection
    out.push_str("    /// Authenticate with the NiFi instance and detect the server version.\n");
    out.push_str("    pub async fn login(&self, username: &str, password: &str) -> Result<(), NifiError> {\n");
    out.push_str("        self.client.login(username, password).await?;\n");
    out.push_str("        self.detect_version().await?;\n");
    out.push_str("        self.discover_cluster().await;\n");
    out.push_str("        Ok(())\n");
    out.push_str("    }\n\n");

    // logout()
    out.push_str("    /// Log out from the NiFi instance.\n");
    out.push_str("    pub async fn logout(&self) -> Result<(), NifiError> {\n");
    out.push_str("        self.client.logout().await\n");
    out.push_str("    }\n\n");

    // Per-tag accessor methods returning dispatch structs (infallible).
    // Version detection happens lazily inside each API method call.
    let all_tags = collect_all_tags(versions);
    for (tag, struct_name, _module_name, accessor_fn) in &all_tags {
        let dispatch_name = format!("{struct_name}Dispatch");
        out.push_str(&format!(
            "    /// Access the [{tag} API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.\n"
        ));
        out.push_str(
            "    ///\n    /// The returned accessor detects the NiFi server version lazily on the\n    /// first API call. Detection errors (transport, auth, unsupported version)\n    /// are propagated as regular [`NifiError`] values from that call.\n"
        );
        out.push_str(&format!(
            "    pub fn {accessor_fn}(&self) -> dispatch::{dispatch_name}<'_> {{\n"
        ));
        out.push_str(&format!(
            "        dispatch::{dispatch_name} {{ client: self }}\n"
        ));
        out.push_str("    }\n\n");
    }

    out.push_str("}\n\n");

    // Deref to NifiClient — lets callers use token(), set_token(), etc. directly.
    out.push_str("impl std::ops::Deref for DynamicClient {\n");
    out.push_str("    type Target = NifiClient;\n");
    out.push_str("    fn deref(&self) -> &NifiClient {\n");
    out.push_str("        &self.client\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{ApiSpec, Endpoint, HttpMethod, PathParam, QueryParam, TagGroup};

    fn minimal_spec_with_tag(
        tag: &str,
        struct_name: &str,
        module_name: &str,
        accessor_fn: &str,
        endpoints: Vec<Endpoint>,
    ) -> ApiSpec {
        ApiSpec {
            tags: vec![TagGroup {
                tag: tag.to_string(),
                struct_name: struct_name.to_string(),
                module_name: module_name.to_string(),
                accessor_fn: accessor_fn.to_string(),
                types: vec![],
                root_endpoints: endpoints,
                sub_groups: vec![],
            }],
            all_types: vec![],
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn make_endpoint(
        method: HttpMethod,
        path: &str,
        fn_name: &str,
        summary: Option<&str>,
        path_params: Vec<&str>,
        query_params: Vec<QueryParam>,
        body_kind: Option<crate::parser::RequestBodyKind>,
        request_type: Option<&str>,
        response_type: Option<&str>,
        response_inner: Option<&str>,
    ) -> Endpoint {
        Endpoint {
            method,
            path: path.to_string(),
            fn_name: fn_name.to_string(),
            doc: summary.map(|s| s.to_string()),
            description: None,
            path_params: path_params
                .into_iter()
                .map(|p| PathParam {
                    name: p.to_string(),
                    doc: None,
                })
                .collect(),
            request_type: request_type.map(|s| s.to_string()),
            body_kind,
            body_doc: None,
            response_type: response_type.map(|s| s.to_string()),
            response_inner: response_inner.map(|s| s.to_string()),
            response_field: response_inner.map(|_| "inner".to_string()),
            response_kind: match response_type {
                Some(rt) => crate::content_type::ResponseBodyKind::Json {
                    schema_ref: rt.to_string(),
                },
                None => crate::content_type::ResponseBodyKind::Empty,
            },
            query_params,
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        }
    }

    #[test]
    fn test_detected_version_enum() {
        let spec = minimal_spec_with_tag("Flow", "FlowApi", "flow", "flow_api", vec![]);
        let output = emit_dynamic(&[
            ("2.7.2", "v2_7_2", "nifi-2-7-2", &spec),
            ("2.8.0", "v2_8_0", "nifi-2-8-0", &spec),
        ]);
        assert!(output.contains("pub enum DetectedVersion"));
        assert!(output.contains("V2_7_2"));
        assert!(output.contains("V2_8_0"));
    }

    #[test]
    fn test_dynamic_client_struct() {
        let spec = minimal_spec_with_tag("Flow", "FlowApi", "flow", "flow_api", vec![]);
        let output = emit_dynamic(&[
            ("2.7.2", "v2_7_2", "nifi-2-7-2", &spec),
            ("2.8.0", "v2_8_0", "nifi-2-8-0", &spec),
        ]);
        assert!(output.contains("pub struct DynamicClient"));
        assert!(output.contains("pub fn flow_api("));
        assert!(output.contains("pub async fn login("));
        assert!(output.contains("pub async fn logout("));
        assert!(output.contains("pub fn detected_version("));
        assert!(output.contains("pub fn cluster_node_id("));
    }

    #[test]
    fn test_cluster_discovery_structs() {
        let spec = minimal_spec_with_tag("Flow", "FlowApi", "flow", "flow_api", vec![]);
        let output = emit_dynamic(&[("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)]);
        assert!(output.contains("struct ClusterSummaryResponse"));
        assert!(output.contains("struct ClusterSummaryInner"));
        assert!(output.contains("struct ClusterResponse"));
        assert!(output.contains("struct ClusterInner"));
        assert!(output.contains("struct ClusterNode"));
        assert!(output.contains("async fn discover_cluster("));
        assert!(output.contains("pub fn cluster_node_id("));
        // discover_cluster is called from both login and from_client
        assert_eq!(
            output.matches("self.discover_cluster().await;").count()
                + output.matches("dc.discover_cluster().await;").count(),
            2,
            "discover_cluster() should be called from both login() and from_client()"
        );
    }

    #[test]
    fn test_module_declarations() {
        let spec = minimal_spec_with_tag("Flow", "FlowApi", "flow", "flow_api", vec![]);
        let output = emit_dynamic(&[("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)]);
        assert!(output.contains("pub mod traits;"));
        assert!(output.contains("pub mod dispatch;"));
        assert!(output.contains("pub mod types;"));
        assert!(output.contains("mod impls;"));
        assert!(output.contains("mod conversions;"));
    }

    #[test]
    fn test_accessor_returns_dispatch_enum() {
        let ep = make_endpoint(
            HttpMethod::Get,
            "/flow/about",
            "get_about_info",
            Some("Get about info"),
            vec![],
            vec![],
            None,
            None,
            Some("AboutDto"),
            None,
        );
        let spec = minimal_spec_with_tag("Flow", "FlowApi", "flow", "flow_api", vec![ep]);
        let output = emit_dynamic(&[
            ("2.7.2", "v2_7_2", "nifi-2-7-2", &spec),
            ("2.8.0", "v2_8_0", "nifi-2-8-0", &spec),
        ]);
        // Accessor should return FlowApiDispatch
        assert!(output.contains("FlowApiDispatch"));
        // Should NOT contain old DynamicFlowApi struct
        assert!(!output.contains("pub struct DynamicFlowApi"));
        // Accessor is infallible and constructs a dispatch struct holding &DynamicClient.
        // rustfmt may split the brace construction across lines, so check both parts.
        assert!(output.contains("dispatch::FlowApiDispatch"));
        assert!(output.contains("client: self"));
        // The accessor no longer matches on DetectedVersion — dispatch is lazy.
        assert!(!output.contains("match self.detected_version()"));
    }

    #[test]
    fn test_no_dynamic_api_structs() {
        let ep = make_endpoint(
            HttpMethod::Get,
            "/flow/about",
            "get_about_info",
            None,
            vec![],
            vec![],
            None,
            None,
            Some("AboutDto"),
            None,
        );
        let spec = minimal_spec_with_tag("Flow", "FlowApi", "flow", "flow_api", vec![ep]);
        let output = emit_dynamic(&[("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)]);
        // Old inline dispatch methods should not exist
        assert!(!output.contains("pub async fn get_about_info("));
        // DynamicFlowApi struct should not exist
        assert!(!output.contains("DynamicFlowApi"));
    }
}
