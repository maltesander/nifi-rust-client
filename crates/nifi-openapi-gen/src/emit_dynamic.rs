use std::collections::BTreeMap;

use crate::parser::{ApiSpec, Endpoint, HttpMethod, QueryParam, RequestBodyKind, SubGroup, TagGroup};

/// Input tuple: `(version_str, mod_name, feature_flag, &ApiSpec)`
/// e.g. `("2.7.2", "v2_7_2", "nifi-2-7-2", &spec)`
///
/// Returns the content for `dynamic/mod.rs`.
pub fn emit_dynamic(versions: &[(&str, &str, &str, &ApiSpec)]) -> String {
    let mut out = String::new();

    // Module declarations
    out.push_str("pub mod types;\n");
    out.push_str("mod conversions;\n\n");

    // Imports
    out.push_str("use crate::{NifiClient, NifiError};\n\n");

    // DetectedVersion enum
    emit_detected_version(&mut out, versions);

    // version_from_str function
    emit_version_from_str(&mut out, versions);

    // AboutResponse / AboutInner deserialization structs
    emit_about_structs(&mut out);

    // DynamicClient struct and impl
    emit_dynamic_client(&mut out, versions);

    // Per-tag Dynamic*Api structs
    emit_dynamic_api_structs(&mut out, versions);

    format_source(&out)
}

fn version_to_variant(version: &str) -> String {
    format!("V{}", version.replace('.', "_"))
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
    out.push_str("        return Err(NifiError::UnsupportedVersion { detected: version.to_string() });\n");
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
    out.push_str("        _ => Err(NifiError::UnsupportedVersion { detected: version.to_string() }),\n");
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

fn emit_dynamic_client(out: &mut String, versions: &[(&str, &str, &str, &ApiSpec)]) {
    // Struct
    out.push_str("/// A dynamic NiFi client that detects the server version at connect time\n");
    out.push_str("/// and dispatches API calls to the correct version's generated code.\n");
    out.push_str("#[derive(Debug)]\n");
    out.push_str("pub struct DynamicClient {\n");
    out.push_str("    client: NifiClient,\n");
    out.push_str("    version: DetectedVersion,\n");
    out.push_str("}\n\n");

    out.push_str("impl DynamicClient {\n");

    // from_client()
    out.push_str("    /// Wrap an existing `NifiClient` and detect the NiFi server version via GET /flow/about.\n");
    out.push_str("    pub async fn from_client(client: NifiClient) -> Result<Self, NifiError> {\n");
    out.push_str("        let resp: AboutResponse = client.get(\"/flow/about\").await?;\n");
    out.push_str("        let version = version_from_str(&resp.about.version)?;\n");
    out.push_str("        Ok(Self { client, version })\n");
    out.push_str("    }\n\n");

    // detected_version()
    out.push_str("    /// Returns the detected NiFi server version.\n");
    out.push_str("    pub fn detected_version(&self) -> DetectedVersion {\n");
    out.push_str("        self.version\n");
    out.push_str("    }\n\n");

    // inner()
    out.push_str("    /// Returns a reference to the underlying `NifiClient`.\n");
    out.push_str("    pub fn inner(&self) -> &NifiClient {\n");
    out.push_str("        &self.client\n");
    out.push_str("    }\n\n");

    // inner_mut()
    out.push_str("    /// Returns a mutable reference to the underlying `NifiClient`.\n");
    out.push_str("    pub fn inner_mut(&mut self) -> &mut NifiClient {\n");
    out.push_str("        &mut self.client\n");
    out.push_str("    }\n\n");

    // login()
    out.push_str("    /// Authenticate with the NiFi instance.\n");
    out.push_str("    pub async fn login(&mut self, username: &str, password: &str) -> Result<(), NifiError> {\n");
    out.push_str("        self.client.login(username, password).await\n");
    out.push_str("    }\n\n");

    // logout()
    out.push_str("    /// Log out from the NiFi instance.\n");
    out.push_str("    pub async fn logout(&mut self) -> Result<(), NifiError> {\n");
    out.push_str("        self.client.logout().await\n");
    out.push_str("    }\n\n");

    // Per-tag accessor methods
    // Collect all unique tags across versions
    let all_tags = collect_all_tags(versions);
    for (tag, struct_name, module_name, accessor_fn) in &all_tags {
        let dynamic_struct = format!("Dynamic{struct_name}");
        out.push_str(&format!(
            "    /// Access the [{tag} API](https://nifi.apache.org/nifi-docs/rest-api.html) with dynamic dispatch.\n"
        ));
        out.push_str(&format!(
            "    pub fn {accessor_fn}(&self) -> {dynamic_struct}<'_> {{\n"
        ));
        out.push_str(&format!(
            "        {dynamic_struct} {{ client: &self.client, version: self.version }}\n"
        ));
        out.push_str("    }\n\n");
    }

    out.push_str("}\n\n");
}

/// Collect all unique (tag, struct_name, module_name, accessor_fn) across versions.
fn collect_all_tags(versions: &[(&str, &str, &str, &ApiSpec)]) -> Vec<(String, String, String, String)> {
    let mut seen: BTreeMap<String, (String, String, String)> = BTreeMap::new();
    for (_, _, _, spec) in versions {
        for tag in &spec.tags {
            seen.entry(tag.tag.clone())
                .or_insert_with(|| (tag.struct_name.clone(), tag.module_name.clone(), tag.accessor_fn.clone()));
        }
    }
    seen.into_iter()
        .map(|(tag, (sn, mn, af))| (tag, sn, mn, af))
        .collect()
}

fn emit_dynamic_api_structs(out: &mut String, versions: &[(&str, &str, &str, &ApiSpec)]) {
    let all_tags = collect_all_tags(versions);

    for (tag, struct_name, module_name, _accessor_fn) in &all_tags {
        let dynamic_struct = format!("Dynamic{struct_name}");

        out.push_str(&format!(
            "/// Dynamic dispatch wrapper for the {tag} API.\n"
        ));
        out.push_str(&format!(
            "pub struct {dynamic_struct}<'a> {{\n    client: &'a NifiClient,\n    version: DetectedVersion,\n}}\n\n"
        ));

        out.push_str(&format!(
            "#[allow(clippy::too_many_arguments, clippy::vec_init_then_push)]\n"
        ));
        out.push_str(&format!("impl<'a> {dynamic_struct}<'a> {{\n"));

        // Collect all endpoints across versions for this tag, keyed by fn_name
        let all_endpoints = collect_tag_endpoints(versions, tag);

        for (_fn_name, ep_by_version) in &all_endpoints {
            // Use the first available endpoint as the representative for signature
            let representative = ep_by_version.values().next().unwrap();
            emit_dynamic_method(out, versions, ep_by_version, representative, struct_name, module_name);
        }

        out.push_str("}\n\n");
    }
}

/// Info about an endpoint in the context of a specific version.
struct EndpointInfo<'a> {
    endpoint: &'a Endpoint,
    /// None for root endpoints, Some(struct_name) for sub-group endpoints.
    sub_struct_name: Option<&'a str>,
    /// The field name for the primary param in the sub-struct (e.g. "id", "port_id").
    primary_param: Option<&'a str>,
}

/// For a given tag, collect all endpoints (including sub-group endpoints flattened) across versions.
/// Returns BTreeMap<fn_name, BTreeMap<version_str, EndpointInfo>>
fn collect_tag_endpoints<'a>(
    versions: &[(&'a str, &'a str, &'a str, &'a ApiSpec)],
    tag: &str,
) -> BTreeMap<String, BTreeMap<&'a str, EndpointInfo<'a>>> {
    let mut result: BTreeMap<String, BTreeMap<&'a str, EndpointInfo<'a>>> = BTreeMap::new();

    for (ver, _mod_name, _feature, spec) in versions {
        for tg in &spec.tags {
            if tg.tag != tag {
                continue;
            }
            for ep in &tg.root_endpoints {
                result.entry(ep.fn_name.clone())
                    .or_default()
                    .insert(ver, EndpointInfo { endpoint: ep, sub_struct_name: None, primary_param: None });
            }
            // Flatten sub-group endpoints
            for sg in &tg.sub_groups {
                for ep in &sg.endpoints {
                    result.entry(ep.fn_name.clone())
                        .or_default()
                        .insert(ver, EndpointInfo {
                            endpoint: ep,
                            sub_struct_name: Some(&sg.struct_name),
                            primary_param: Some(&sg.primary_param),
                        });
                }
            }
        }
    }

    result
}

fn emit_dynamic_method(
    out: &mut String,
    versions: &[(&str, &str, &str, &ApiSpec)],
    ep_by_version: &BTreeMap<&str, EndpointInfo<'_>>,
    representative: &EndpointInfo<'_>,
    tag_struct_name: &str,
    tag_module_name: &str,
) {
    let ep = representative.endpoint;

    // Skip form-encoded endpoints
    if ep.body_kind == Some(RequestBodyKind::FormEncoded) {
        return;
    }

    // Determine return type
    let return_ty = match &ep.response_inner {
        Some(inner) => format!("types::{inner}"),
        None => match &ep.response_type {
            Some(ty) => format!("types::{ty}"),
            None => "()".into(),
        },
    };
    let return_result = format!("Result<{return_ty}, NifiError>");
    let is_void = ep.response_type.is_none() && ep.response_inner.is_none();

    // Build path param args.
    // For sub-group endpoints, ensure the primary param is included even if the
    // representative endpoint's path_params doesn't list it (e.g. some sub-group
    // endpoints use a different param name structure).
    let mut path_param_names: Vec<String> = Vec::new();
    if let Some(primary) = representative.primary_param {
        path_param_names.push(primary.to_string());
    }
    for p in &ep.path_params {
        if !path_param_names.contains(&p.name) {
            path_param_names.push(p.name.clone());
        }
    }
    let path_param_args: String = path_param_names
        .iter()
        .map(|name| format!(", {}: &str", escape_keyword(name)))
        .collect();

    // Query params — build the superset (union) across all versions.
    // Params that don't exist in every version are forced to Option even if required in some.
    let merged_query_params = merge_query_params(ep_by_version);
    let all_version_count = ep_by_version.len();
    let query_param_args: String = merged_query_params
        .iter()
        .map(|(qp, presence_count)| {
            let rust_type = dynamic_query_param_type(qp);
            // If the param is not present in every version, force it to Option
            let force_option = *presence_count < all_version_count;
            if qp.required && !force_option {
                format!(", {}: {rust_type}", escape_keyword(&qp.rust_name))
            } else {
                format!(", {}: Option<{rust_type}>", escape_keyword(&qp.rust_name))
            }
        })
        .collect();

    // Body param — use &serde_json::Value for JSON bodies
    let body_arg = if ep.method == HttpMethod::Delete {
        String::new()
    } else {
        match &ep.body_kind {
            Some(RequestBodyKind::Json) => ", body: &serde_json::Value".to_string(),
            Some(RequestBodyKind::OctetStream) => {
                ", filename: Option<&str>, data: Vec<u8>".to_string()
            }
            Some(RequestBodyKind::FormEncoded) | None => String::new(),
        }
    };

    // Doc comment
    if let Some(doc) = &ep.doc {
        out.push_str(&format!("    /// {doc}\n"));
    }

    out.push_str(&format!(
        "    pub async fn {fn_name}(&self{path_param_args}{query_param_args}{body_arg}) -> {return_result} {{\n",
        fn_name = ep.fn_name,
    ));

    // Match on version
    out.push_str("        match self.version {\n");

    for (ver, mod_name, _feature, _spec) in versions {
        let variant = version_to_variant(ver);
        if let Some(info) = ep_by_version.get(ver) {
            emit_version_arm(out, ver, mod_name, &variant, info, tag_struct_name, tag_module_name, &merged_query_params, all_version_count, is_void);
        } else {
            // Endpoint not available in this version
            out.push_str(&format!(
                "            DetectedVersion::{variant} => Err(NifiError::UnsupportedEndpoint {{ endpoint: \"{fn_name}\".to_string(), version: \"{ver}\".to_string() }}),\n",
                fn_name = ep.fn_name,
            ));
        }
    }

    out.push_str("        }\n");
    out.push_str("    }\n\n");
}

fn emit_version_arm(
    out: &mut String,
    ver: &str,
    mod_name: &str,
    variant: &str,
    info: &EndpointInfo<'_>,
    tag_struct_name: &str,
    tag_module_name: &str,
    merged_query_params: &[(QueryParam, usize)],
    all_version_count: usize,
    is_void: bool,
) {
    let ep = info.endpoint;
    out.push_str(&format!(
        "            DetectedVersion::{variant} => {{\n"
    ));
    match (info.sub_struct_name, info.primary_param) {
        (Some(sub_struct), Some(primary_param)) => {
            // Sub-group endpoint: instantiate the sub-struct with client + primary_param
            // primary_param is already in rust form (e.g. "port_id", "id")
            out.push_str(&format!(
                "                let api = crate::{mod_name}::api::{tag_module_name}::{sub_struct} {{ client: self.client, {primary_param}: {arg} }};\n",
                arg = escape_keyword(primary_param),
            ));
        }
        _ => {
            out.push_str(&format!(
                "                let api = crate::{mod_name}::api::{tag_module_name}::{tag_struct_name} {{ client: self.client }};\n"
            ));
        }
    }

    // Build the call arguments
    let mut call_args = Vec::new();
    // For sub-group endpoints, skip the primary param if it's in the endpoint's path params
    // (it's baked into the sub-struct via self.{primary_param})
    let primary_to_skip = info.primary_param;
    for p in &ep.path_params {
        if primary_to_skip.is_some_and(|pp| pp == p.name) {
            continue;
        }
        call_args.push(escape_keyword(&p.name));
    }
    // Query params — only pass params this version actually has.
    // Check the merged superset to see if the param was forced to Option in the signature.
    for qp in &ep.query_params {
        // Was this param forced to Option in the method signature?
        let forced_option = merged_query_params.iter()
            .find(|(mq, _)| mq.rust_name == qp.rust_name)
            .map(|(_, count)| *count < all_version_count)
            .unwrap_or(false);

        if qp.enum_type_name.is_some() {
            // Convert &str to the version-specific enum type
            let type_name = qp.enum_type_name.as_deref().unwrap();
            if qp.required && !forced_option {
                call_args.push(format!(
                    "serde_json::from_value::<crate::{mod_name}::types::{type_name}>(serde_json::Value::String({name}.to_string())).map_err(|_| NifiError::UnsupportedEndpoint {{ endpoint: \"{fn_name}\".to_string(), version: \"{ver}\".to_string() }})?",
                    name = escape_keyword(&qp.rust_name),
                    fn_name = ep.fn_name,
                ));
            } else if qp.required && forced_option {
                // Required in this version but Option in the signature — unwrap with error
                call_args.push(format!(
                    "serde_json::from_value::<crate::{mod_name}::types::{type_name}>(serde_json::Value::String({name}.ok_or_else(|| NifiError::UnsupportedEndpoint {{ endpoint: \"{fn_name} (missing required param {raw_name})\".to_string(), version: \"{ver}\".to_string() }})?.to_string())).map_err(|_| NifiError::UnsupportedEndpoint {{ endpoint: \"{fn_name}\".to_string(), version: \"{ver}\".to_string() }})?",
                    name = escape_keyword(&qp.rust_name),
                    fn_name = ep.fn_name,
                    raw_name = qp.rust_name,
                ));
            } else {
                call_args.push(format!(
                    "{name}.map(|v| serde_json::from_value::<crate::{mod_name}::types::{type_name}>(serde_json::Value::String(v.to_string()))).transpose().map_err(|_| NifiError::UnsupportedEndpoint {{ endpoint: \"{fn_name}\".to_string(), version: \"{ver}\".to_string() }})?",
                    name = escape_keyword(&qp.rust_name),
                    fn_name = ep.fn_name,
                ));
            }
        } else if qp.required && forced_option {
            // Required in this version but Option in the signature — unwrap with error
            call_args.push(format!(
                "{name}.ok_or_else(|| NifiError::UnsupportedEndpoint {{ endpoint: \"{fn_name} (missing required param {raw_name})\".to_string(), version: \"{ver}\".to_string() }})?",
                name = escape_keyword(&qp.rust_name),
                fn_name = ep.fn_name,
                raw_name = qp.rust_name,
            ));
        } else {
            call_args.push(escape_keyword(&qp.rust_name));
        }
    }
    // Body param
    if ep.method != HttpMethod::Delete {
        match &ep.body_kind {
            Some(RequestBodyKind::Json) => {
                let req_type = ep.request_type.as_deref().unwrap_or("serde_json::Value");
                call_args.push(format!(
                    "&serde_json::from_value::<crate::{mod_name}::types::{req_type}>(body.clone()).map_err(|e| NifiError::UnsupportedEndpoint {{ endpoint: format!(\"{{}} (body deserialize: {{}})\", \"{fn_name}\", e), version: \"{ver}\".to_string() }})?",
                    fn_name = ep.fn_name,
                ));
            }
            Some(RequestBodyKind::OctetStream) => {
                call_args.push("filename".to_string());
                call_args.push("data".to_string());
            }
            Some(RequestBodyKind::FormEncoded) | None => {}
        }
    }

    let args_str = call_args.join(", ");

    if is_void {
        out.push_str(&format!(
            "                api.{fn_name}({args_str}).await\n",
            fn_name = ep.fn_name,
        ));
    } else {
        out.push_str(&format!(
            "                Ok(api.{fn_name}({args_str}).await?.into())\n",
            fn_name = ep.fn_name,
        ));
    }

    out.push_str("            }\n");
}

/// Merge query params from all versions of an endpoint into a superset.
/// Returns each unique param (by rust_name) along with the count of versions it appears in.
/// Preserves insertion order: params appear in the order first seen across versions.
fn merge_query_params(ep_by_version: &BTreeMap<&str, EndpointInfo<'_>>) -> Vec<(QueryParam, usize)> {
    let mut result: Vec<(QueryParam, usize)> = Vec::new();
    for info in ep_by_version.values() {
        for qp in &info.endpoint.query_params {
            if let Some((existing, count)) = result.iter_mut().find(|(q, _)| q.rust_name == qp.rust_name) {
                *count += 1;
                // If any version marks it optional, keep it optional
                if !qp.required {
                    existing.required = false;
                }
            } else {
                result.push((qp.clone(), 1));
            }
        }
    }
    result
}

/// For dynamic mode, all query params use simple types (strings for enums).
fn dynamic_query_param_type(qp: &QueryParam) -> String {
    match &qp.ty {
        crate::parser::QueryParamType::Str => "&str".to_string(),
        crate::parser::QueryParamType::Bool => "bool".to_string(),
        crate::parser::QueryParamType::I32 => "i32".to_string(),
        crate::parser::QueryParamType::I64 => "i64".to_string(),
        crate::parser::QueryParamType::F64 => "f64".to_string(),
        crate::parser::QueryParamType::Enum(_) => "&str".to_string(),
    }
}

fn prop_name_to_rust(name: &str) -> String {
    let mut out = String::new();
    for (i, ch) in name.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}

fn escape_keyword(name: &str) -> String {
    match name {
        "type" | "ref" | "use" | "mod" | "fn" | "let" | "match" | "for" | "if" | "else"
        | "return" | "struct" | "enum" | "impl" | "trait" | "pub" | "super" | "self" | "crate"
        | "where" | "true" | "false" | "in" | "loop" | "while" | "break" | "continue" | "mut"
        | "move" | "async" | "await" | "dyn" | "box" | "const" | "static" | "extern" | "unsafe"
        | "as" => format!("r#{name}"),
        _ => name.to_string(),
    }
}

fn format_source(src: &str) -> String {
    match syn::parse_file(src) {
        Ok(file) => prettyplease::unparse(&file),
        Err(_) => src.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{
        ApiSpec, Endpoint, Field, FieldType, HttpMethod, PathParam, QueryParam, QueryParamType,
        SubGroup, TagGroup, TypeDef, TypeKind,
    };

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

    fn make_endpoint(
        method: HttpMethod,
        path: &str,
        fn_name: &str,
        summary: Option<&str>,
        path_params: Vec<&str>,
        query_params: Vec<QueryParam>,
        body_kind: Option<RequestBodyKind>,
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
            query_params,
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
    }

    #[test]
    fn test_dispatch_struct_generated() {
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
        assert!(output.contains("pub struct DynamicFlowApi"));
        assert!(output.contains("pub async fn get_about_info("));
        assert!(output.contains("DetectedVersion::V2_7_2"));
        assert!(output.contains("DetectedVersion::V2_8_0"));
    }

    #[test]
    fn test_endpoint_missing_in_one_version() {
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
        let spec_with = minimal_spec_with_tag("Flow", "FlowApi", "flow", "flow_api", vec![ep]);
        let spec_without = minimal_spec_with_tag("Flow", "FlowApi", "flow", "flow_api", vec![]);
        let output = emit_dynamic(&[
            ("2.7.2", "v2_7_2", "nifi-2-7-2", &spec_without),
            ("2.8.0", "v2_8_0", "nifi-2-8-0", &spec_with),
        ]);
        assert!(output.contains("UnsupportedEndpoint"));
    }

    #[test]
    fn test_void_return_endpoint() {
        let ep = make_endpoint(
            HttpMethod::Post,
            "/flow/generate",
            "generate_client_id",
            None,
            vec![],
            vec![],
            None,
            None,
            None,
            None,
        );
        let spec = minimal_spec_with_tag("Flow", "FlowApi", "flow", "flow_api", vec![ep]);
        let output = emit_dynamic(&[("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)]);
        assert!(output.contains("-> Result<(), NifiError>"));
    }
}
