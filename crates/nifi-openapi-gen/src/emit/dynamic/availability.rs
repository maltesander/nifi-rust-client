//! Emitter for `dynamic/availability.rs`.

use crate::canonical::CanonicalSpec;

use super::index::EndpointIndex;

pub fn emit_availability(canonical: &CanonicalSpec, index: &EndpointIndex<'_>) -> String {
    let mut out = String::new();
    out.push_str("//! Generated endpoint availability table for canonical dynamic dispatch.\n\n");

    let versions = canonical_versions(canonical);
    emit_detected_version(&mut out, &versions);
    emit_version_from_str(&mut out, &versions);
    emit_supported_versions(&mut out, &versions);

    // Endpoint enum
    out.push_str("/// One variant per canonical endpoint. Public so callers can write\n");
    out.push_str("/// `if client.supports(Endpoint::GET_FLOW_ABOUT) { ... }`.\n");
    out.push_str("#[allow(non_camel_case_types)]\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    out.push_str("#[non_exhaustive]\n");
    out.push_str("pub enum Endpoint {\n");
    for ep in &index.endpoints {
        let variant = endpoint_variant_name(&ep.key.method, &ep.key.path);
        out.push_str(&format!("    {variant},\n"));
    }
    out.push_str("}\n\n");

    // Display as "METHOD /path" — used by error messages.
    out.push_str("impl Endpoint {\n");
    out.push_str("    /// `\"METHOD /path\"` form, used by error messages and tracing.\n");
    out.push_str("    pub fn as_str(&self) -> &'static str {\n");
    out.push_str("        match self {\n");
    for ep in &index.endpoints {
        let variant = endpoint_variant_name(&ep.key.method, &ep.key.path);
        let display = format!("{} {}", ep.key.method.as_str(), ep.key.path);
        out.push_str(&format!(
            "            Endpoint::{variant} => \"{display}\",\n"
        ));
    }
    out.push_str("        }\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // Per-endpoint version set.
    out.push_str("/// `(Endpoint, supported versions)` for every canonical endpoint.\n");
    out.push_str("/// The string slices are static — they live in the binary.\n");
    out.push_str("pub const ENDPOINT_AVAILABILITY: &[(Endpoint, &[&str])] = &[\n");
    for ep in &index.endpoints {
        let variant = endpoint_variant_name(&ep.key.method, &ep.key.path);
        let versions: Vec<String> = ep
            .versions
            .to_vec()
            .into_iter()
            .map(|v| format!("\"{v}\""))
            .collect();
        out.push_str(&format!(
            "    (Endpoint::{variant}, &[{}]),\n",
            versions.join(", ")
        ));
    }
    out.push_str("];\n\n");

    // Per-(endpoint, query_param) version set.
    out.push_str(
        "/// `((Endpoint, param wire name), supported versions)` for every query\n\
         /// parameter that does not exist in every supporting version. Used by the\n\
         /// generated URL builder to emit `UnsupportedQueryParam` when a caller sets\n\
         /// a value the detected server cannot understand.\n",
    );
    out.push_str("pub const QUERY_PARAM_AVAILABILITY: &[((Endpoint, &str), &[&str])] = &[\n");
    for ep in &index.endpoints {
        if ep.query_param_versions.is_empty() {
            continue;
        }
        let endpoint_versions = &ep.versions;
        for (param, versions) in &ep.query_param_versions {
            // Skip params that exist in every supporting version — no guard needed.
            if versions == endpoint_versions {
                continue;
            }
            let variant = endpoint_variant_name(&ep.key.method, &ep.key.path);
            let vs: Vec<String> = versions
                .to_vec()
                .into_iter()
                .map(|v| format!("\"{v}\""))
                .collect();
            out.push_str(&format!(
                "    ((Endpoint::{variant}, \"{param}\"), &[{}]),\n",
                vs.join(", ")
            ));
        }
    }
    out.push_str("];\n\n");

    // query_param_supported runtime helper.
    out.push_str(
        "/// True if `param` is supported by `endpoint` in the given `version`.\n\
         /// Returns `true` for params not in `QUERY_PARAM_AVAILABILITY` (i.e. params\n\
         /// available in every supporting version).\n\
         pub fn query_param_supported(endpoint: Endpoint, param: &str, version: &str) -> bool {\n\
             for ((ep, p), versions) in QUERY_PARAM_AVAILABILITY {\n\
                 if *ep == endpoint && *p == param {\n\
                     return versions.contains(&version);\n\
                 }\n\
             }\n\
             true\n\
         }\n",
    );

    crate::util::format_source(&out)
}

/// Versions discovered in the canonical spec, in semver order.
fn canonical_versions(canonical: &CanonicalSpec) -> Vec<String> {
    canonical.per_version_specs.keys().cloned().collect()
}

/// Rust variant name for a version string, e.g. `"2.9.0"` → `"V2_9_0"`.
fn version_to_variant(version: &str) -> String {
    format!("V{}", version.replace('.', "_"))
}

fn emit_detected_version(out: &mut String, versions: &[String]) {
    out.push_str("/// Represents a detected NiFi server version.\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n");
    out.push_str("pub enum DetectedVersion {\n");
    for ver in versions {
        out.push_str(&format!("    {},\n", version_to_variant(ver)));
    }
    out.push_str("}\n\n");

    out.push_str("impl std::fmt::Display for DetectedVersion {\n");
    out.push_str("    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n");
    out.push_str("        match self {\n");
    for ver in versions {
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

fn emit_version_from_str(out: &mut String, versions: &[String]) {
    use std::collections::BTreeMap;
    out.push_str("/// Match a version string by major.minor (ignoring patch).\n");
    out.push_str("pub(crate) fn version_from_str(version: &str) -> Result<DetectedVersion, crate::NifiError> {\n");
    out.push_str("    let parts: Vec<&str> = version.split('.').collect();\n");
    out.push_str("    if parts.len() < 2 {\n");
    out.push_str("        return Err(crate::NifiError::UnsupportedVersion { detected: version.to_string() });\n");
    out.push_str("    }\n");
    out.push_str("    let major_minor = format!(\"{}.{}\", parts[0], parts[1]);\n");
    out.push_str("    match major_minor.as_str() {\n");

    // Group versions by major.minor, pick the lowest patch per major.minor.
    let mut major_minor_map: BTreeMap<String, &String> = BTreeMap::new();
    for ver in versions {
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
    out.push_str("        _ => Err(crate::NifiError::UnsupportedVersion { detected: version.to_string() }),\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn emit_supported_versions(out: &mut String, versions: &[String]) {
    out.push_str("/// All supported versions compiled into this build, used by version resolution strategies.\n");
    out.push_str("pub(crate) const SUPPORTED_VERSIONS: &[(&str, DetectedVersion)] = &[\n");
    for ver in versions {
        out.push_str(&format!(
            "    (\"{}\", DetectedVersion::{}),\n",
            ver,
            version_to_variant(ver),
        ));
    }
    out.push_str("];\n\n");

    if let Some(latest) = versions.last() {
        out.push_str("/// The semver-latest NiFi version supported by this build.\n");
        out.push_str(&format!(
            "pub const LATEST_NIFI_VERSION: &str = \"{latest}\";\n\n"
        ));
    }
}

/// Build the upper-snake variant name for an endpoint, e.g.
/// `GET /flow/about` → `GET_FLOW_ABOUT`.
pub fn endpoint_variant_name(method: &crate::parser::HttpMethod, path: &str) -> String {
    let mut out = String::new();
    out.push_str(method.as_str());
    for segment in path.split('/').filter(|s| !s.is_empty()) {
        out.push('_');
        if let Some(inner) = segment.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
            out.push_str(&inner.to_ascii_uppercase().replace('-', "_"));
        } else {
            out.push_str(&segment.to_ascii_uppercase().replace('-', "_"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::canonicalize;
    use crate::parser::{ApiSpec, Endpoint, HttpMethod, TagGroup};

    fn minimal_spec() -> ApiSpec {
        ApiSpec {
            tags: vec![TagGroup {
                tag: "Flow".to_string(),
                struct_name: "Flow".to_string(),
                module_name: "flow".to_string(),
                accessor_fn: "flow".to_string(),
                types: vec![],
                endpoints: vec![Endpoint {
                    method: HttpMethod::Get,
                    path: "/flow/about".to_string(),
                    fn_name: "get_about_info".to_string(),
                    raw_operation_id: "getAboutInfo".to_string(),
                    doc: None,
                    description: None,
                    path_params: vec![],
                    request_type: None,
                    body_kind: None,
                    body_doc: None,
                    response_type: None,
                    response_inner: None,
                    response_field: None,
                    response_kind: crate::content_type::ResponseBodyKind::Empty,
                    query_params: vec![],
                    success_responses: vec![],
                    error_responses: vec![],
                    security: None,
                }],
            }],
            all_types: vec![],
        }
    }

    #[test]
    fn emit_availability_includes_endpoint_enum_and_table() {
        let canonical = canonicalize(&[("2.8.0".to_string(), minimal_spec())]);
        let index = EndpointIndex::build(&canonical);
        let src = emit_availability(&canonical, &index);
        assert!(src.contains("pub enum Endpoint"));
        assert!(src.contains("GET_FLOW_ABOUT"));
        assert!(src.contains("ENDPOINT_AVAILABILITY"));
        assert!(src.contains("\"2.8.0\""));
    }

    #[test]
    fn endpoint_variant_uppercases_path_params() {
        let v = endpoint_variant_name(&HttpMethod::Post, "/processors/{id}/run-status");
        assert_eq!(v, "POST_PROCESSORS_ID_RUN_STATUS");
    }

    #[test]
    fn emit_availability_includes_detected_version_enum_and_helpers() {
        use crate::canonical::canonicalize;
        let canonical = canonicalize(&[
            ("2.6.0".to_string(), minimal_spec()),
            ("2.9.0".to_string(), minimal_spec()),
        ]);
        let index = EndpointIndex::build(&canonical);
        let src = emit_availability(&canonical, &index);
        assert!(src.contains("pub enum DetectedVersion"));
        assert!(src.contains("V2_6_0"));
        assert!(src.contains("V2_9_0"));
        assert!(src.contains("fn version_from_str"));
        assert!(src.contains("SUPPORTED_VERSIONS"));
        assert!(src.contains("pub const LATEST_NIFI_VERSION"));
    }
}
