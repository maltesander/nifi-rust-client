//! Emitter for `dynamic_v2/api/*.rs`.
//!
//! Generates one canonical struct per tag (e.g. `FlowApi<'a>`) holding a
//! borrow of `DynamicClientV2`. Endpoint methods start with a
//! `require_endpoint(...)` availability check, then dispatch via the
//! shared `NifiClient` HTTP helpers.
//!
//! Phase 4a (this file) handles only no-arg GET endpoints. Task 7 extends
//! to path params, query params, sub-resources, and request bodies.

use std::collections::BTreeMap;

use crate::canonical::CanonicalSpec;
use crate::content_type::ResponseBodyKind;
use crate::parser::{Endpoint, HttpMethod};

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
    mod_out.push_str("impl crate::dynamic_v2::DynamicClientV2 {\n");
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
    out.push_str("use crate::NifiError;\n");
    out.push_str("use crate::dynamic_v2::DynamicClientV2;\n");
    out.push_str("use crate::dynamic_v2::availability::Endpoint;\n\n");

    out.push_str(&format!(
        "pub struct {}<'a> {{\n    pub(crate) client: &'a DynamicClientV2,\n}}\n\n",
        meta.struct_name
    ));

    out.push_str("#[allow(clippy::too_many_arguments, clippy::let_unit_value)]\n");
    out.push_str(&format!("impl<'a> {}<'a> {{\n", meta.struct_name));
    for ep in endpoints {
        if !is_supported_in_4a_task6(ep.endpoint) {
            // Phase 4a Task 6: only no-arg GET endpoints at the tag root. Skip the rest with a stub.
            emit_unsupported_stub(&mut out, ep);
            continue;
        }
        emit_simple_get_method(&mut out, ep);
    }
    out.push_str("}\n");
    out
}

fn is_supported_in_4a_task6(ep: &Endpoint) -> bool {
    matches!(ep.method, HttpMethod::Get)
        && ep.path_params.is_empty()
        && ep.query_params.is_empty()
        && ep.body_kind.is_none()
}

fn emit_unsupported_stub(out: &mut String, ep: &IndexedEndpoint<'_>) {
    let variant = endpoint_variant_name(&ep.key.method, &ep.key.path);
    out.push_str(&format!(
        "    // TODO Task 7: emit body for {} {} (variant {variant})\n",
        ep.key.method.as_str(),
        ep.key.path,
    ));
}

fn emit_simple_get_method(out: &mut String, ep: &IndexedEndpoint<'_>) {
    let fn_name = &ep.endpoint.fn_name;
    let variant = endpoint_variant_name(&ep.key.method, &ep.key.path);
    let path = &ep.key.path;

    let return_ty = response_type_for(ep.endpoint);

    out.push_str(&format!(
        "    pub async fn {fn_name}(&self) -> Result<{return_ty}, NifiError> {{\n"
    ));
    out.push_str(&format!(
        "        self.client.require_endpoint(Endpoint::{variant}).await?;\n"
    ));
    out.push_str(&format!("        let path = \"{path}\";\n"));
    out.push_str("        tracing::debug!(method = \"GET\", path, \"NiFi API request\");\n");
    emit_get_body(out, ep);
    out.push_str("    }\n\n");
}

fn response_type_for(ep: &Endpoint) -> String {
    match (&ep.response_kind, &ep.response_inner, &ep.response_type) {
        (ResponseBodyKind::Empty, _, _) => "()".to_string(),
        (_, Some(inner), _) => format!("crate::dynamic_v2::types::{inner}"),
        (_, _, Some(rt)) => format!("crate::dynamic_v2::types::{rt}"),
        _ => "()".to_string(),
    }
}

fn emit_get_body(out: &mut String, ep: &IndexedEndpoint<'_>) {
    match (
        &ep.endpoint.response_kind,
        &ep.endpoint.response_inner,
        &ep.endpoint.response_field,
    ) {
        (ResponseBodyKind::Empty, _, _) => {
            out.push_str("        self.client.inner().get::<()>(path).await\n");
        }
        (ResponseBodyKind::Json { .. }, Some(inner), Some(field)) => {
            let entity = ep.endpoint.response_type.as_deref().unwrap_or(inner);
            out.push_str(&format!(
                "        let wrapper: crate::dynamic_v2::types::{entity} = self.client.inner().get(path).await?;\n"
            ));
            out.push_str(&format!("        Ok(wrapper.{field}.unwrap_or_default())\n"));
        }
        (ResponseBodyKind::Json { .. }, _, _) => {
            let rt = ep.endpoint.response_type.as_deref().unwrap_or("()");
            out.push_str(&format!(
                "        self.client.inner().get::<crate::dynamic_v2::types::{rt}>(path).await\n"
            ));
        }
        _ => {
            // Non-JSON / non-empty responses are out of Task 6 scope; Task 7 adds per-content-type emission.
            out.push_str(
                "        Err(NifiError::Auth { message: \"non-json responses unsupported in 4a Task 6\".into() })\n",
            );
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
}
