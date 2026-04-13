//! Emitter for `dynamic/api/*.rs`.
//!
//! Generates one canonical struct per tag (e.g. `FlowApi<'a>`) holding a
//! borrow of `DynamicClient`. Endpoint methods start with a
//! `require_endpoint(...)` availability check, then dispatch via the
//! shared `NifiClient` HTTP helpers.

use std::collections::BTreeMap;

use crate::canonical::CanonicalSpec;
use crate::parser::{Endpoint, HttpMethod};
use crate::util::escape_keyword;

use super::availability::endpoint_variant_name;
use super::index::{EndpointIndex, IndexedEndpoint};

pub fn emit_api(canonical: &CanonicalSpec, index: &EndpointIndex<'_>) -> Vec<(String, String)> {
    // Group indexed endpoints by tag.
    let mut by_tag: BTreeMap<&str, Vec<&IndexedEndpoint<'_>>> = BTreeMap::new();
    for ep in &index.endpoints {
        by_tag.entry(ep.tag).or_default().push(ep);
    }

    // Tag metadata (struct_name, module_name, accessor_fn) — pull from the latest
    // version's TagGroup definition. This survives 4a; 4b moves it onto Canonical.
    let tag_meta = collect_tag_meta(canonical);

    let mut files: Vec<(String, String)> = Vec::new();

    // mod.rs
    let mut mod_out = String::new();
    for tag in by_tag.keys() {
        if let Some(meta) = tag_meta.get(*tag) {
            mod_out.push_str(&format!("pub mod {};\n", meta.module_name));
        }
    }
    mod_out.push('\n');
    mod_out.push_str("impl crate::dynamic::DynamicClient {\n");
    for tag in by_tag.keys() {
        if let Some(meta) = tag_meta.get(*tag) {
            mod_out.push_str(&format!(
                "    /// Access the [{tag} API](https://nifi.apache.org/nifi-docs/rest-api.html) (canonical dynamic).\n",
            ));
            mod_out.push_str(&format!(
                "    pub fn {accessor}(&self) -> {module}::{struct_name}<'_> {{\n        {module}::{struct_name} {{ client: self }}\n    }}\n",
                accessor = meta.accessor_fn,
                module = meta.module_name,
                struct_name = meta.struct_name,
            ));
        }
    }
    mod_out.push_str("}\n");
    files.push(("mod.rs".to_string(), crate::util::format_source(&mod_out)));

    // Per-tag api file
    for (tag, eps) in &by_tag {
        if let Some(meta) = tag_meta.get(*tag) {
            let content = emit_tag_file(meta, eps);
            files.push((
                format!("{}.rs", meta.module_name),
                crate::util::format_source(&content),
            ));
        }
    }

    files
}

#[derive(Debug)]
struct TagMeta<'a> {
    struct_name: &'a str,
    module_name: &'a str,
    accessor_fn: &'a str,
}

fn collect_tag_meta(canonical: &CanonicalSpec) -> BTreeMap<&str, TagMeta<'_>> {
    let mut out: BTreeMap<&str, TagMeta<'_>> = BTreeMap::new();
    for spec in canonical.per_version_specs.values() {
        for tag in &spec.tags {
            out.entry(tag.tag.as_str()).or_insert(TagMeta {
                struct_name: tag.struct_name.as_str(),
                module_name: tag.module_name.as_str(),
                accessor_fn: tag.accessor_fn.as_str(),
            });
        }
    }
    out
}

fn emit_tag_file(meta: &TagMeta<'_>, endpoints: &[&IndexedEndpoint<'_>]) -> String {
    let mut out = String::new();
    out.push_str("#[allow(unused_imports)]\n");
    out.push_str("use crate::NifiError;\n");
    out.push_str("use crate::dynamic::DynamicClient;\n");
    out.push_str("use crate::dynamic::availability::Endpoint;\n\n");

    out.push_str(&format!(
        "pub struct {}<'a> {{\n    pub(crate) client: &'a DynamicClient,\n}}\n\n",
        meta.struct_name
    ));

    // Group endpoints into root vs sub-group.
    let mut roots: Vec<&IndexedEndpoint<'_>> = Vec::new();
    let mut by_subgroup: BTreeMap<&str, Vec<&IndexedEndpoint<'_>>> = BTreeMap::new();
    for ep in endpoints {
        if let Some(sg) = ep.sub_group {
            by_subgroup
                .entry(sg.struct_name.as_str())
                .or_default()
                .push(ep);
        } else {
            roots.push(ep);
        }
    }

    // Collect one representative SubGroup metadata per sub-struct name.
    let mut sg_meta: BTreeMap<&str, &crate::parser::SubGroup> = BTreeMap::new();
    for ep in endpoints {
        if let Some(sg) = ep.sub_group {
            sg_meta.entry(sg.struct_name.as_str()).or_insert(sg);
        }
    }

    out.push_str("#[allow(clippy::too_many_arguments, clippy::let_unit_value, clippy::vec_init_then_push, unused_variables)]\n");
    out.push_str(&format!("impl<'a> {}<'a> {{\n", meta.struct_name));
    for ep in &roots {
        emit_method(&mut out, ep, SelfKind::Root);
    }
    // Sub-group accessors on the root struct.
    for (sg_struct, sg) in &sg_meta {
        let esc_param = escape_keyword(&sg.primary_param);
        out.push_str(&format!(
            "    pub fn {accessor}<'b>(&'b self, {esc_param}: &'b str) -> {sg_struct}<'b> {{\n        {sg_struct} {{ client: self.client, {raw_param}: {esc_param}.to_string() }}\n    }}\n\n",
            accessor = sg.accessor_fn,
            raw_param = sg.primary_param,
        ));
    }
    out.push_str("}\n\n");

    // Emit sub-group structs and their impls.
    for (sg_struct, eps) in &by_subgroup {
        let sg = sg_meta[sg_struct];
        out.push_str(&format!(
            "pub struct {sg_struct}<'a> {{\n    pub(crate) client: &'a DynamicClient,\n    pub(crate) {param}: String,\n}}\n\n",
            param = sg.primary_param,
        ));
        out.push_str("#[allow(clippy::too_many_arguments, clippy::let_unit_value, clippy::vec_init_then_push, unused_variables)]\n");
        out.push_str(&format!("impl<'a> {sg_struct}<'a> {{\n"));
        for ep in eps {
            emit_method(&mut out, ep, SelfKind::SubGroup(sg.primary_param.as_str()));
        }
        out.push_str("}\n\n");
    }

    out
}

#[derive(Copy, Clone)]
enum SelfKind<'a> {
    Root,
    SubGroup(&'a str),
}

fn emit_method(out: &mut String, ep: &IndexedEndpoint<'_>, self_kind: SelfKind<'_>) {
    let fn_name = &ep.endpoint.fn_name;
    let variant = endpoint_variant_name(&ep.key.method, &ep.key.path);

    // Build argument list (skip the primary sub-group param — it's on `self`).
    let mut args: Vec<String> = Vec::new();
    let skip_param = match self_kind {
        SelfKind::SubGroup(p) => Some(p),
        SelfKind::Root => None,
    };
    for pp in &ep.endpoint.path_params {
        if Some(pp.name.as_str()) == skip_param {
            continue;
        }
        args.push(format!("{}: &str", escape_keyword(&pp.name)));
    }
    for qp in &ep.endpoint.query_params {
        let ty = qp_rust_type(qp);
        if qp.required {
            args.push(format!("{}: {}", escape_keyword(&qp.rust_name), ty));
        } else {
            args.push(format!("{}: Option<{}>", escape_keyword(&qp.rust_name), ty));
        }
    }
    use crate::content_type::RequestBodyKind;
    match &ep.endpoint.body_kind {
        Some(RequestBodyKind::Json) => {
            if let Some(rt) = &ep.endpoint.request_type {
                args.push(format!("body: &crate::dynamic::types::{rt}"));
            }
        }
        Some(RequestBodyKind::OctetStream) => {
            args.push("filename: Option<&str>".to_string());
            args.push("data: Vec<u8>".to_string());
        }
        Some(RequestBodyKind::Multipart) => {
            args.push("filename: &str".to_string());
            args.push("data: Vec<u8>".to_string());
        }
        _ => {}
    }

    let return_ty = response_type_for(ep.endpoint);
    let args_joined = if args.is_empty() {
        String::new()
    } else {
        format!(", {}", args.join(", "))
    };
    out.push_str(&format!(
        "    pub async fn {fn_name}(&self{args_joined}) -> Result<{return_ty}, NifiError> {{\n"
    ));
    out.push_str(&format!(
        "        self.client.require_endpoint(Endpoint::{variant}).await?;\n"
    ));

    // Path interpolation
    let mut path_expr = format!("\"{}\".to_string()", ep.key.path);
    for pp in &ep.endpoint.path_params {
        let placeholder = format!("{{{}}}", pp.name);
        let value = if Some(pp.name.as_str()) == skip_param {
            format!("&self.{}", escape_keyword(&pp.name))
        } else {
            escape_keyword(&pp.name)
        };
        path_expr = format!("{path_expr}.replace(\"{placeholder}\", {value})");
    }
    out.push_str(&format!("        let path = {path_expr};\n"));

    // Query vector (used when there are query params)
    let has_query = !ep.endpoint.query_params.is_empty();
    if has_query {
        out.push_str("        let mut query: Vec<(&str, String)> = Vec::new();\n");
        for qp in &ep.endpoint.query_params {
            emit_query_push(out, ep, qp);
        }
    }

    // Tracing + dispatch
    let method_lit = ep.key.method.as_str();
    out.push_str(&format!(
        "        tracing::debug!(method = \"{method_lit}\", path = path.as_str(), \"NiFi API request\");\n"
    ));

    emit_dispatch(out, ep, has_query);

    out.push_str("    }\n\n");
}

fn qp_rust_type(qp: &crate::parser::QueryParam) -> String {
    use crate::parser::QueryParamType::*;
    match &qp.ty {
        Str => "&str".to_string(),
        Bool => "bool".to_string(),
        I32 => "i32".to_string(),
        I64 => "i64".to_string(),
        F64 => "f64".to_string(),
        Enum(_) => qp
            .enum_type_name
            .as_ref()
            .map(|n| format!("crate::dynamic::types::{n}"))
            .unwrap_or_else(|| "&str".to_string()),
    }
}

fn emit_query_push(out: &mut String, ep: &IndexedEndpoint<'_>, qp: &crate::parser::QueryParam) {
    let endpoint_versions = &ep.versions;
    let qp_versions = ep
        .query_param_versions
        .get(&qp.name)
        .cloned()
        .unwrap_or_else(|| endpoint_versions.clone());
    let needs_guard = qp_versions != *endpoint_versions;

    let escaped_rust_name = escape_keyword(&qp.rust_name);
    let to_string_expr = format!("{escaped_rust_name}.to_string()");

    if qp.required {
        if needs_guard {
            emit_guard(out, ep, &qp.name, &qp_versions);
        }
        out.push_str(&format!(
            "        query.push((\"{name}\", {value}));\n",
            name = qp.name,
            value = to_string_expr,
        ));
    } else {
        out.push_str(&format!(
            "        if let Some({n}) = {n} {{\n",
            n = escaped_rust_name
        ));
        if needs_guard {
            emit_guard_indented(out, ep, &qp.name, &qp_versions);
        }
        out.push_str(&format!(
            "            query.push((\"{name}\", {value}));\n",
            name = qp.name,
            value = to_string_expr,
        ));
        out.push_str("        }\n");
    }
}

fn emit_guard(
    out: &mut String,
    ep: &IndexedEndpoint<'_>,
    param: &str,
    versions: &crate::canonical::VersionSet,
) {
    let variant = endpoint_variant_name(&ep.key.method, &ep.key.path);
    let supported: Vec<String> = versions
        .to_vec()
        .into_iter()
        .map(|v| format!("\"{v}\".to_string()"))
        .collect();
    out.push_str(&format!(
        "        let __detected = self.client.detected_version_str();\n        if !crate::dynamic::availability::query_param_supported(Endpoint::{variant}, \"{param}\", &__detected) {{\n"
    ));
    out.push_str(&format!(
        "            return Err(NifiError::UnsupportedQueryParam {{ endpoint: Endpoint::{variant}.as_str(), param: \"{param}\", detected_version: __detected, supported_in: vec![{}] }});\n",
        supported.join(", "),
    ));
    out.push_str("        }\n");
}

fn emit_guard_indented(
    out: &mut String,
    ep: &IndexedEndpoint<'_>,
    param: &str,
    versions: &crate::canonical::VersionSet,
) {
    let variant = endpoint_variant_name(&ep.key.method, &ep.key.path);
    let supported: Vec<String> = versions
        .to_vec()
        .into_iter()
        .map(|v| format!("\"{v}\".to_string()"))
        .collect();
    out.push_str(&format!(
        "            let __detected = self.client.detected_version_str();\n            if !crate::dynamic::availability::query_param_supported(Endpoint::{variant}, \"{param}\", &__detected) {{\n"
    ));
    out.push_str(&format!(
        "                return Err(NifiError::UnsupportedQueryParam {{ endpoint: Endpoint::{variant}.as_str(), param: \"{param}\", detected_version: __detected, supported_in: vec![{}] }});\n",
        supported.join(", "),
    ));
    out.push_str("            }\n");
}

fn response_type_for(ep: &Endpoint) -> String {
    use crate::content_type::ResponseBodyKind;
    match (&ep.response_kind, &ep.response_inner, &ep.response_type) {
        (ResponseBodyKind::Empty, _, _) => "()".to_string(),
        (ResponseBodyKind::Text | ResponseBodyKind::Xml, _, _) => "String".to_string(),
        (ResponseBodyKind::OctetStream | ResponseBodyKind::Wildcard, _, _) => "Vec<u8>".to_string(),
        (_, Some(inner), _) => format!("crate::dynamic::types::{inner}"),
        (_, _, Some(rt)) => format!("crate::dynamic::types::{rt}"),
        _ => "()".to_string(),
    }
}

fn emit_dispatch(out: &mut String, ep: &IndexedEndpoint<'_>, has_query: bool) {
    use crate::content_type::{RequestBodyKind, ResponseBodyKind};

    let return_kind = &ep.endpoint.response_kind;
    let request_kind = &ep.endpoint.body_kind;
    let returns_unit = matches!(return_kind, ResponseBodyKind::Empty);
    let returns_text = matches!(return_kind, ResponseBodyKind::Text | ResponseBodyKind::Xml);
    let returns_bytes = matches!(
        return_kind,
        ResponseBodyKind::OctetStream | ResponseBodyKind::Wildcard
    );

    let path_arg = "&path";
    let use_query = has_query;

    // ── Unit (Empty) returns ─────────────────────────────────────────────
    if returns_unit {
        match (&ep.key.method, request_kind, use_query) {
            (HttpMethod::Delete, _, true) => {
                out.push_str(&format!(
                    "        self.client.inner().delete_with_query({path_arg}, &query).await\n"
                ));
            }
            (HttpMethod::Delete, _, false) => {
                out.push_str(&format!(
                    "        self.client.inner().delete({path_arg}).await\n"
                ));
            }
            (HttpMethod::Post, Some(RequestBodyKind::Json), _) => {
                out.push_str(&format!(
                    "        self.client.inner().post_void({path_arg}, body).await\n"
                ));
            }
            (HttpMethod::Post, Some(RequestBodyKind::OctetStream), _) => {
                out.push_str(&format!(
                    "        self.client.inner().post_void_octet_stream({path_arg}, filename, data).await\n"
                ));
            }
            (HttpMethod::Post, _, _) => {
                out.push_str(&format!(
                    "        self.client.inner().post_void_no_body({path_arg}).await\n"
                ));
            }
            (HttpMethod::Put, Some(RequestBodyKind::Json), _) => {
                out.push_str(&format!(
                    "        self.client.inner().put_void({path_arg}, body).await\n"
                ));
            }
            (HttpMethod::Put, _, _) => {
                out.push_str(&format!(
                    "        self.client.inner().put_void_no_body({path_arg}).await\n"
                ));
            }
            _ => {
                out.push_str(&format!(
                    "        self.client.inner().get_void_with_query({path_arg}, &[]).await\n"
                ));
            }
        }
        return;
    }

    // ── Text / bytes returns ─────────────────────────────────────────────
    if returns_text {
        if use_query {
            out.push_str(
                "        todo!(\"text GET with query params not implemented in dynamic\")\n",
            );
        } else {
            out.push_str(&format!(
                "        self.client.inner().get_text({path_arg}).await\n"
            ));
        }
        return;
    }

    if returns_bytes {
        if use_query {
            out.push_str(&format!(
                "        self.client.inner().get_bytes_with_query({path_arg}, &query).await\n"
            ));
        } else {
            out.push_str(&format!(
                "        self.client.inner().get_bytes({path_arg}).await\n"
            ));
        }
        return;
    }

    // ── Non-unit JSON returns ────────────────────────────────────────────
    // Determine the effective call expression per method + body kind.
    let call_for_entity = |helper: &str, body: &str, q: &str| -> String {
        format!("self.client.inner().{helper}({path_arg}{body}{q}).await?")
    };

    let (base_helper, body_args, query_args): (&str, &str, String) =
        match (&ep.key.method, request_kind, use_query) {
            // GET
            (HttpMethod::Get, _, true) => ("get_with_query", "", ", &query".to_string()),
            (HttpMethod::Get, _, false) => ("get", "", String::new()),
            // DELETE with return value
            (HttpMethod::Delete, _, true) => {
                ("delete_returning_with_query", "", ", &query".to_string())
            }
            (HttpMethod::Delete, _, false) => ("delete_returning", "", String::new()),
            // POST
            (HttpMethod::Post, Some(RequestBodyKind::Json), true) => {
                ("post_with_query", ", body", ", &query".to_string())
            }
            (HttpMethod::Post, Some(RequestBodyKind::Json), false) => {
                ("post", ", body", String::new())
            }
            (HttpMethod::Post, Some(RequestBodyKind::OctetStream), _) => {
                ("post_octet_stream", ", filename, data", String::new())
            }
            (HttpMethod::Post, Some(RequestBodyKind::Multipart), _) => {
                ("post_multipart", ", filename, data", String::new())
            }
            (HttpMethod::Post, _, _) => ("post_no_body", "", String::new()),
            // PUT
            (HttpMethod::Put, Some(RequestBodyKind::Json), true) => {
                ("put_with_query", ", body", ", &query".to_string())
            }
            (HttpMethod::Put, Some(RequestBodyKind::Json), false) => {
                ("put", ", body", String::new())
            }
            (HttpMethod::Put, _, _) => ("put_no_body", "", String::new()),
        };

    match (&ep.endpoint.response_inner, &ep.endpoint.response_field) {
        (Some(inner), Some(field)) => {
            let entity = ep.endpoint.response_type.as_deref().unwrap_or(inner);
            let call_expr = call_for_entity(base_helper, body_args, &query_args);
            out.push_str(&format!(
                "        let wrapper: crate::dynamic::types::{entity} = {call_expr};\n"
            ));
            out.push_str(&format!(
                "        Ok(wrapper.{field}.unwrap_or_default())\n"
            ));
        }
        _ => {
            if let Some(_rt) = ep.endpoint.response_type.as_deref() {
                // post/put with body: Rust infers B from `body` and T from the outer
                // return type — no turbofish needed. For no-body helpers (single type
                // param T), we also let inference drive it.
                out.push_str(&format!(
                    "        self.client.inner().{base_helper}({path_arg}{body_args}{query_args}).await\n"
                ));
            } else {
                // No named response type — treat as unit.
                out.push_str(&format!(
                    "        self.client.inner().post_void_no_body({path_arg}).await\n"
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::canonicalize;
    use crate::parser::{ApiSpec, Endpoint, HttpMethod, TagGroup};

    fn flow_about_spec() -> ApiSpec {
        ApiSpec {
            tags: vec![TagGroup {
                tag: "Flow".to_string(),
                struct_name: "FlowApi".to_string(),
                module_name: "flow".to_string(),
                accessor_fn: "flow_api".to_string(),
                types: vec![],
                root_endpoints: vec![Endpoint {
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
                    response_type: Some("AboutEntity".to_string()),
                    response_inner: Some("AboutDto".to_string()),
                    response_field: Some("about".to_string()),
                    response_kind: crate::content_type::ResponseBodyKind::Json {
                        schema_ref: "AboutEntity".to_string(),
                    },
                    query_params: vec![],
                    success_responses: vec![],
                    error_responses: vec![],
                    security: None,
                }],
                sub_groups: vec![],
            }],
            all_types: vec![],
        }
    }

    #[test]
    fn emit_api_produces_struct_and_method() {
        let canonical = canonicalize(&[("2.8.0".to_string(), flow_about_spec())]);
        let index = EndpointIndex::build(&canonical);
        let files = emit_api(&canonical, &index);
        let flow = files.iter().find(|(n, _)| n == "flow.rs").unwrap();
        assert!(flow.1.contains("pub struct FlowApi"));
        assert!(flow.1.contains("pub async fn get_about_info"));
        assert!(flow.1.contains("Endpoint::GET_FLOW_ABOUT"));
        assert!(flow.1.contains("require_endpoint"));
        let mod_rs = files.iter().find(|(n, _)| n == "mod.rs").unwrap();
        assert!(mod_rs.1.contains("pub mod flow"));
        assert!(mod_rs.1.contains("pub fn flow_api"));
    }

    #[test]
    fn emit_api_handles_path_params_query_params_and_sub_groups() {
        use crate::parser::{PathParam, QueryParam, QueryParamType, SubGroup};
        let mut spec = flow_about_spec();
        let processors_tag = TagGroup {
            tag: "Processors".to_string(),
            struct_name: "ProcessorsApi".to_string(),
            module_name: "processors".to_string(),
            accessor_fn: "processors_api".to_string(),
            types: vec![],
            root_endpoints: vec![Endpoint {
                method: HttpMethod::Get,
                path: "/processors/{id}".to_string(),
                fn_name: "get_processor".to_string(),
                raw_operation_id: "getProcessor".to_string(),
                doc: None,
                description: None,
                path_params: vec![PathParam {
                    name: "id".to_string(),
                    doc: None,
                }],
                request_type: None,
                body_kind: None,
                body_doc: None,
                response_type: Some("ProcessorEntity".to_string()),
                response_inner: Some("ProcessorDto".to_string()),
                response_field: Some("component".to_string()),
                response_kind: crate::content_type::ResponseBodyKind::Json {
                    schema_ref: "ProcessorEntity".to_string(),
                },
                query_params: vec![QueryParam {
                    name: "verbose".to_string(),
                    rust_name: "verbose".to_string(),
                    ty: QueryParamType::Bool,
                    required: false,
                    doc: None,
                    enum_type_name: None,
                }],
                success_responses: vec![],
                error_responses: vec![],
                security: None,
            }],
            sub_groups: vec![SubGroup {
                name: "config".to_string(),
                struct_name: "ProcessorsConfigApi".to_string(),
                accessor_fn: "config".to_string(),
                primary_param: "id".to_string(),
                primary_param_doc: None,
                endpoints: vec![Endpoint {
                    method: HttpMethod::Get,
                    path: "/processors/{id}/config".to_string(),
                    fn_name: "get_config".to_string(),
                    raw_operation_id: "getConfig".to_string(),
                    doc: None,
                    description: None,
                    path_params: vec![PathParam {
                        name: "id".to_string(),
                        doc: None,
                    }],
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
        };
        spec.tags.push(processors_tag);
        let canonical = canonicalize(&[("2.8.0".to_string(), spec)]);
        let index = EndpointIndex::build(&canonical);
        let files = emit_api(&canonical, &index);
        let proc_file = files.iter().find(|(n, _)| n == "processors.rs").unwrap();
        let body = &proc_file.1;
        // Path param baked into URL
        assert!(body.contains("get_processor"));
        assert!(body.contains("/processors/"));
        // Query param assembled (guard call is optional — only emitted for version-skewed params)
        assert!(body.contains("verbose"));
        // Sub-resource accessor
        assert!(body.contains("pub fn config"));
        assert!(body.contains("ProcessorsConfigApi"));
        // Sub-resource struct + method
        assert!(body.contains("pub struct ProcessorsConfigApi"));
        assert!(body.contains("pub async fn get_config"));
    }
}
