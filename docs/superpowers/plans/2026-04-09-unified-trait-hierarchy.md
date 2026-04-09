# Unified Trait Hierarchy Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make static and dynamic clients use the same sub-struct builder pattern for path parameters, via parallel per-version trait hierarchies with GATs.

**Architecture:** Two independent trait hierarchies (one per static version, one for dynamic) with identical shape but different type namespaces. The static traits are additive — existing inherent methods stay. The dynamic traits are refactored from flat to hierarchical, with new sub-resource dispatch structs. A new utility function `collect_tag_sub_groups` supports the hierarchical grouping.

**Tech Stack:** Rust, GATs (stable since 1.65), code generation via `nifi-openapi-gen`

**Spec:** `docs/superpowers/specs/2026-04-09-unified-trait-hierarchy-design.md`

---

### Task 1: Add `collect_tag_sub_groups` utility

The existing `collect_tag_endpoints` flattens all endpoints (root + sub-group) into one map. The new hierarchical traits need endpoints grouped by sub-group. Add a utility that preserves this structure.

**Files:**
- Modify: `crates/nifi-openapi-gen/src/util.rs`

- [ ] **Step 1: Write the test**

Add to the `#[cfg(test)] mod tests` block at the bottom of `util.rs`:

```rust
// --- collect_tag_sub_groups ---

#[test]
fn collect_tag_sub_groups_separates_root_and_sub() {
    use crate::parser::*;

    let ep_root = Endpoint {
        method: HttpMethod::Get,
        path: "/controller-services/{id}".to_string(),
        fn_name: "get_controller_service".to_string(),
        doc: None,
        description: None,
        path_params: vec![PathParam {
            name: "id".to_string(),
            doc: None,
        }],
        request_type: None,
        body_kind: None,
        body_doc: None,
        response_type: Some("ControllerServiceEntity".to_string()),
        response_inner: None,
        response_field: None,
        query_params: vec![],
        success_responses: vec![],
        error_responses: vec![],
        security: None,
    };

    let ep_sub = Endpoint {
        method: HttpMethod::Post,
        path: "/controller-services/{id}/config/analysis".to_string(),
        fn_name: "analyze_configuration".to_string(),
        doc: None,
        description: None,
        path_params: vec![PathParam {
            name: "id".to_string(),
            doc: None,
        }],
        request_type: Some("ConfigurationAnalysisEntity".to_string()),
        body_kind: Some(RequestBodyKind::Json),
        body_doc: None,
        response_type: Some("ConfigurationAnalysisEntity".to_string()),
        response_inner: Some("ConfigurationAnalysisDto".to_string()),
        response_field: Some("configuration_analysis".to_string()),
        query_params: vec![],
        success_responses: vec![],
        error_responses: vec![],
        security: None,
    };

    let spec = ApiSpec {
        tags: vec![TagGroup {
            tag: "ControllerServices".to_string(),
            struct_name: "ControllerServicesApi".to_string(),
            module_name: "controller_services".to_string(),
            accessor_fn: "controller_services_api".to_string(),
            types: vec![],
            root_endpoints: vec![ep_root],
            sub_groups: vec![SubGroup {
                name: "config".to_string(),
                struct_name: "ControllerServicesConfigApi".to_string(),
                accessor_fn: "config".to_string(),
                primary_param: "id".to_string(),
                primary_param_doc: Some("The controller service id.".to_string()),
                endpoints: vec![ep_sub],
            }],
        }],
        all_types: vec![],
    };

    let versions: Vec<(&str, &str, &str, &ApiSpec)> =
        vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];

    let result = collect_tag_sub_groups(&versions, "ControllerServices");

    // Root endpoints
    assert_eq!(result.root_endpoints.len(), 1);
    assert!(result.root_endpoints.contains_key("get_controller_service"));

    // Sub-groups
    assert_eq!(result.sub_groups.len(), 1);
    let sg = &result.sub_groups[0];
    assert_eq!(sg.struct_name, "ControllerServicesConfigApi");
    assert_eq!(sg.accessor_fn, "config");
    assert_eq!(sg.primary_param, "id");
    assert_eq!(sg.endpoints.len(), 1);
    assert!(sg.endpoints.contains_key("analyze_configuration"));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p nifi-openapi-gen collect_tag_sub_groups`
Expected: FAIL — `collect_tag_sub_groups` not found

- [ ] **Step 3: Implement `collect_tag_sub_groups`**

Add the following structs and function to `util.rs` (before the `#[cfg(test)]` block):

```rust
/// Info about a sub-group collected across all versions.
pub(crate) struct CollectedSubGroup<'a> {
    pub struct_name: String,
    pub accessor_fn: String,
    pub primary_param: String,
    pub primary_param_doc: Option<String>,
    /// fn_name → (version_str → EndpointInfo)
    pub endpoints: BTreeMap<String, BTreeMap<&'a str, EndpointInfo<'a>>>,
}

/// Hierarchical grouping of endpoints for a tag across versions.
pub(crate) struct TagSubGroups<'a> {
    /// Root endpoints: fn_name → (version_str → EndpointInfo)
    pub root_endpoints: BTreeMap<String, BTreeMap<&'a str, EndpointInfo<'a>>>,
    /// Sub-groups with their endpoints grouped
    pub sub_groups: Vec<CollectedSubGroup<'a>>,
}

/// Like `collect_tag_endpoints`, but preserves the sub-group hierarchy instead of flattening.
pub(crate) fn collect_tag_sub_groups<'a>(
    versions: &[(&'a str, &'a str, &'a str, &'a ApiSpec)],
    tag: &str,
) -> TagSubGroups<'a> {
    let mut root_endpoints: BTreeMap<String, BTreeMap<&'a str, EndpointInfo<'a>>> =
        BTreeMap::new();
    // accessor_fn → CollectedSubGroup (using accessor_fn as stable key across versions)
    let mut sub_map: BTreeMap<String, CollectedSubGroup<'a>> = BTreeMap::new();

    for (ver, _mod_name, _feature, spec) in versions {
        for tg in &spec.tags {
            if tg.tag != tag {
                continue;
            }
            for ep in &tg.root_endpoints {
                root_endpoints
                    .entry(ep.fn_name.clone())
                    .or_default()
                    .insert(
                        ver,
                        EndpointInfo {
                            endpoint: ep,
                            sub_struct_name: None,
                            primary_param: None,
                        },
                    );
            }
            for sg in &tg.sub_groups {
                let collected = sub_map.entry(sg.accessor_fn.clone()).or_insert_with(|| {
                    CollectedSubGroup {
                        struct_name: sg.struct_name.clone(),
                        accessor_fn: sg.accessor_fn.clone(),
                        primary_param: sg.primary_param.clone(),
                        primary_param_doc: sg.primary_param_doc.clone(),
                        endpoints: BTreeMap::new(),
                    }
                });
                for ep in &sg.endpoints {
                    collected
                        .endpoints
                        .entry(ep.fn_name.clone())
                        .or_default()
                        .insert(
                            ver,
                            EndpointInfo {
                                endpoint: ep,
                                sub_struct_name: Some(&sg.struct_name),
                                primary_param: Some(&sg.primary_param),
                            },
                        );
                }
            }
        }
    }

    TagSubGroups {
        root_endpoints,
        sub_groups: sub_map.into_values().collect(),
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p nifi-openapi-gen collect_tag_sub_groups`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/nifi-openapi-gen/src/util.rs
git commit -m "feat(openapi-gen): add collect_tag_sub_groups utility for hierarchical endpoint grouping"
```

---

### Task 2: Add static trait emitter (`emit/traits.rs`)

New emitter that generates per-version trait modules. Each API group gets a root trait with GAT accessors and sub-resource traits with leaf methods.

**Files:**
- Create: `crates/nifi-openapi-gen/src/emit/traits.rs`
- Modify: `crates/nifi-openapi-gen/src/emit/mod.rs`

- [ ] **Step 1: Write the test**

Create the test at the bottom of the new file `traits.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;

    fn make_spec() -> ApiSpec {
        let ep_root = Endpoint {
            method: HttpMethod::Get,
            path: "/controller-services/{id}".to_string(),
            fn_name: "get_controller_service".to_string(),
            doc: Some("Gets a controller service".to_string()),
            description: None,
            path_params: vec![PathParam {
                name: "id".to_string(),
                doc: Some("The controller service id.".to_string()),
            }],
            request_type: None,
            body_kind: None,
            body_doc: None,
            response_type: Some("ControllerServiceEntity".to_string()),
            response_inner: Some("ControllerServiceDto".to_string()),
            response_field: Some("component".to_string()),
            query_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        };

        let ep_sub = Endpoint {
            method: HttpMethod::Post,
            path: "/controller-services/{id}/config/analysis".to_string(),
            fn_name: "analyze_configuration".to_string(),
            doc: Some("Performs analysis of the component's configuration".to_string()),
            description: None,
            path_params: vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            request_type: Some("ConfigurationAnalysisEntity".to_string()),
            body_kind: Some(RequestBodyKind::Json),
            body_doc: None,
            response_type: Some("ConfigurationAnalysisEntity".to_string()),
            response_inner: Some("ConfigurationAnalysisDto".to_string()),
            response_field: Some("configuration_analysis".to_string()),
            query_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        };

        ApiSpec {
            tags: vec![TagGroup {
                tag: "ControllerServices".to_string(),
                struct_name: "ControllerServicesApi".to_string(),
                module_name: "controller_services".to_string(),
                accessor_fn: "controller_services_api".to_string(),
                types: vec![],
                root_endpoints: vec![ep_root],
                sub_groups: vec![SubGroup {
                    name: "config".to_string(),
                    struct_name: "ControllerServicesConfigApi".to_string(),
                    accessor_fn: "config".to_string(),
                    primary_param: "id".to_string(),
                    primary_param_doc: Some("The controller service id.".to_string()),
                    endpoints: vec![ep_sub],
                }],
            }],
            all_types: vec![],
        }
    }

    #[test]
    fn emit_static_traits_generates_root_and_sub_traits() {
        let spec = make_spec();
        let files = emit_static_traits(&spec, "crate::v2_8_0");
        assert!(files.len() >= 2); // mod.rs + controller_services.rs

        let (_, mod_content) = files.iter().find(|(f, _)| f == "mod.rs").unwrap();
        assert!(mod_content.contains("pub mod controller_services;"));

        let (_, content) = files
            .iter()
            .find(|(f, _)| f == "controller_services.rs")
            .unwrap();

        // Root trait with GAT
        assert!(content.contains("pub trait ControllerServicesApi"));
        assert!(content.contains("type ConfigApi<'b>: ControllerServicesConfigApi"));
        assert!(content.contains("fn config<'b>(&'b self, id: &'b str) -> Self::ConfigApi<'b>"));

        // Root method
        assert!(content.contains("async fn get_controller_service("));
        assert!(content.contains("id: &str"));

        // Sub-resource trait
        assert!(content.contains("pub trait ControllerServicesConfigApi"));
        assert!(content.contains("async fn analyze_configuration("));
        assert!(content.contains("body: &crate::v2_8_0::types::ConfigurationAnalysisEntity"));

        // Sub-resource trait method should NOT have id param
        // Find the analyze_configuration signature and check it doesn't contain ", id: &str"
        let config_trait_start = content.find("pub trait ControllerServicesConfigApi").unwrap();
        let config_trait_section = &content[config_trait_start..];
        let analyze_sig_start = config_trait_section.find("fn analyze_configuration").unwrap();
        let analyze_sig_end = config_trait_section[analyze_sig_start..]
            .find('{')
            .unwrap();
        let analyze_sig =
            &config_trait_section[analyze_sig_start..analyze_sig_start + analyze_sig_end];
        assert!(
            !analyze_sig.contains("id: &str"),
            "Sub-resource trait method should not include primary path param 'id'"
        );
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p nifi-openapi-gen emit_static_traits_generates`
Expected: FAIL — module `traits` not found

- [ ] **Step 3: Implement `emit_static_traits`**

Create `crates/nifi-openapi-gen/src/emit/traits.rs`:

```rust
use crate::parser::{ApiSpec, HttpMethod, RequestBodyKind};
use crate::util::{escape_keyword, format_source};

/// Emits per-version static trait definitions.
/// Returns `Vec<(filename, content)>` for `vx_y_z/traits/`.
///
/// `types_prefix` is the crate path to the version's types module,
/// e.g. `"crate::v2_8_0"`.
pub fn emit_static_traits(spec: &ApiSpec, types_prefix: &str) -> Vec<(String, String)> {
    let mut files: Vec<(String, String)> = Vec::new();

    // mod.rs
    let mut mod_out = String::new();
    for tag in &spec.tags {
        mod_out.push_str(&format!("pub mod {};\n", tag.module_name));
        mod_out.push_str(&format!(
            "pub use {}::{};\n",
            tag.module_name, tag.struct_name
        ));
        for sg in &tag.sub_groups {
            mod_out.push_str(&format!(
                "pub use {}::{};\n",
                tag.module_name, sg.struct_name
            ));
        }
    }
    files.push(("mod.rs".to_string(), format_source(&mod_out)));

    // Per-tag trait files
    for tag in &spec.tags {
        let content = emit_tag_traits(tag, types_prefix);
        files.push((format!("{}.rs", tag.module_name), format_source(&content)));
    }

    files
}

fn emit_tag_traits(tag: &crate::parser::TagGroup, types_prefix: &str) -> String {
    let mut out = String::new();
    out.push_str("use crate::NifiError;\n\n");

    // --- Sub-resource traits first (so the root trait can reference them) ---
    for sg in &tag.sub_groups {
        emit_sub_resource_trait(&mut out, sg, types_prefix);
    }

    // --- Root trait ---
    out.push_str(&format!("/// The {} API.\n", tag.tag));
    out.push_str(
        "#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]\n",
    );
    out.push_str(&format!("pub trait {} {{\n", tag.struct_name));

    // GAT declarations + accessor methods for each sub-group
    for sg in &tag.sub_groups {
        out.push_str(&format!(
            "    type {}<'b>: {} where Self: 'b;\n",
            gat_name(&sg.struct_name),
            sg.struct_name,
        ));
    }
    if !tag.sub_groups.is_empty() {
        out.push('\n');
    }
    for sg in &tag.sub_groups {
        if let Some(doc) = &sg.primary_param_doc {
            out.push_str(&format!(
                "    /// Scope operations to the `{}` sub-resource.\n    ///\n    /// - `{}`: {}\n",
                sg.name, sg.primary_param, doc
            ));
        } else {
            out.push_str(&format!(
                "    /// Scope operations to the `{}` sub-resource.\n",
                sg.name
            ));
        }
        out.push_str(&format!(
            "    fn {}<'b>(&'b self, {}: &'b str) -> Self::{}<'b>;\n\n",
            sg.accessor_fn,
            escape_keyword(&sg.primary_param),
            gat_name(&sg.struct_name),
        ));
    }

    // Root-level async methods
    for ep in &tag.root_endpoints {
        emit_trait_method(&mut out, ep, types_prefix, None);
    }

    out.push_str("}\n\n");
    out
}

fn emit_sub_resource_trait(
    out: &mut String,
    sg: &crate::parser::SubGroup,
    types_prefix: &str,
) {
    out.push_str(&format!(
        "/// Sub-resource trait for `{}` operations.\n",
        sg.name
    ));
    out.push_str(
        "#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]\n",
    );
    out.push_str(&format!("pub trait {} {{\n", sg.struct_name));

    for ep in &sg.endpoints {
        emit_trait_method(out, ep, types_prefix, Some(&sg.primary_param));
    }

    out.push_str("}\n\n");
}

fn emit_trait_method(
    out: &mut String,
    ep: &crate::parser::Endpoint,
    types_prefix: &str,
    skip_primary: Option<&str>,
) {
    if ep.body_kind == Some(RequestBodyKind::FormEncoded) {
        return;
    }

    // Doc comment
    if let Some(doc) = &ep.doc {
        out.push_str(&format!("    /// {doc}\n"));
    }

    // Return type
    let return_ty = match &ep.response_inner {
        Some(inner) => format!("{types_prefix}::types::{inner}"),
        None => match &ep.response_type {
            Some(ty) => format!("{types_prefix}::types::{ty}"),
            None => "()".into(),
        },
    };
    let return_result = format!("Result<{return_ty}, NifiError>");

    // Path params (excluding primary if inside a sub-resource trait)
    let path_param_args: String = ep
        .path_params
        .iter()
        .filter(|p| skip_primary.is_none_or(|pp| p.name != pp))
        .map(|p| format!(", {}: &str", escape_keyword(&p.name)))
        .collect();

    // Query params
    let query_param_args: String = ep
        .query_params
        .iter()
        .map(|qp| {
            let rust_type = crate::emit::common::query_param_rust_type(qp);
            if qp.required {
                format!(", {}: {rust_type}", escape_keyword(&qp.rust_name))
            } else {
                format!(", {}: Option<{rust_type}>", escape_keyword(&qp.rust_name))
            }
        })
        .collect();

    // Body param
    let body_arg = if ep.method == HttpMethod::Delete {
        String::new()
    } else {
        match &ep.body_kind {
            Some(RequestBodyKind::Json) => {
                let ty = ep.request_type.as_deref().unwrap_or("serde_json::Value");
                format!(", body: &{types_prefix}::types::{ty}")
            }
            Some(RequestBodyKind::OctetStream) => {
                ", filename: Option<&str>, data: Vec<u8>".to_string()
            }
            Some(RequestBodyKind::FormEncoded) | None => String::new(),
        }
    };

    // Signature
    out.push_str(&format!(
        "    async fn {fn_name}(&self{path_param_args}{query_param_args}{body_arg}) -> {return_result};\n\n",
        fn_name = ep.fn_name,
    ));
}

/// Convert a sub-struct name like "ControllerServicesConfigApi" to a GAT name
/// by stripping the parent prefix would be complex, so just use the struct name directly.
fn gat_name(struct_name: &str) -> String {
    struct_name.to_string()
}
```

- [ ] **Step 4: Wire into `emit/mod.rs`**

Add to `crates/nifi-openapi-gen/src/emit/mod.rs`:

```rust
mod traits;
pub use traits::emit_static_traits;
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test -p nifi-openapi-gen emit_static_traits_generates`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add crates/nifi-openapi-gen/src/emit/traits.rs crates/nifi-openapi-gen/src/emit/mod.rs
git commit -m "feat(openapi-gen): add static trait emitter for per-version trait hierarchies"
```

---

### Task 3: Add static trait impl emission to `emit/api.rs`

Extend the existing API emitter to append trait impl blocks for each struct and sub-struct.

**Files:**
- Modify: `crates/nifi-openapi-gen/src/emit/api.rs`

- [ ] **Step 1: Write the test**

Add to the existing `#[cfg(test)]` block in `api.rs` (or create one if absent):

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;

    fn make_spec_with_sub_group() -> ApiSpec {
        let ep_root = Endpoint {
            method: HttpMethod::Get,
            path: "/controller-services/{id}".to_string(),
            fn_name: "get_controller_service".to_string(),
            doc: Some("Gets a controller service".to_string()),
            description: None,
            path_params: vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            request_type: None,
            body_kind: None,
            body_doc: None,
            response_type: Some("ControllerServiceEntity".to_string()),
            response_inner: Some("ControllerServiceDto".to_string()),
            response_field: Some("component".to_string()),
            query_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        };

        let ep_sub = Endpoint {
            method: HttpMethod::Post,
            path: "/controller-services/{id}/config/analysis".to_string(),
            fn_name: "analyze_configuration".to_string(),
            doc: Some("Performs analysis".to_string()),
            description: None,
            path_params: vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            request_type: Some("ConfigurationAnalysisEntity".to_string()),
            body_kind: Some(RequestBodyKind::Json),
            body_doc: None,
            response_type: Some("ConfigurationAnalysisEntity".to_string()),
            response_inner: Some("ConfigurationAnalysisDto".to_string()),
            response_field: Some("configuration_analysis".to_string()),
            query_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        };

        ApiSpec {
            tags: vec![TagGroup {
                tag: "ControllerServices".to_string(),
                struct_name: "ControllerServicesApi".to_string(),
                module_name: "controller_services".to_string(),
                accessor_fn: "controller_services_api".to_string(),
                types: vec![],
                root_endpoints: vec![ep_root],
                sub_groups: vec![SubGroup {
                    name: "config".to_string(),
                    struct_name: "ControllerServicesConfigApi".to_string(),
                    accessor_fn: "config".to_string(),
                    primary_param: "id".to_string(),
                    primary_param_doc: None,
                    endpoints: vec![ep_sub],
                }],
            }],
            all_types: vec![],
        }
    }

    #[test]
    fn emit_api_includes_trait_impls() {
        let spec = make_spec_with_sub_group();
        let files = emit_api_with_prefix(&spec, "crate::v2_8_0");
        let (_, content) = files
            .iter()
            .find(|(f, _)| f == "controller_services.rs")
            .unwrap();

        // Root trait impl
        assert!(
            content.contains("impl crate::v2_8_0::traits::ControllerServicesApi for ControllerServicesApi<'_>"),
            "Missing root trait impl"
        );
        assert!(
            content.contains("type ControllerServicesConfigApi<'b> = ControllerServicesConfigApi<'b>"),
            "Missing GAT binding"
        );

        // Sub-resource trait impl
        assert!(
            content.contains("impl crate::v2_8_0::traits::ControllerServicesConfigApi for ControllerServicesConfigApi<'_>"),
            "Missing sub-resource trait impl"
        );
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p nifi-openapi-gen emit_api_includes_trait_impls`
Expected: FAIL — trait impls not present in output

- [ ] **Step 3: Implement trait impl emission**

Add the following functions to `api.rs` and call `emit_trait_impls` from `emit_tag`:

```rust
/// Emit trait impl blocks for the root struct and all sub-structs.
fn emit_trait_impls(tag: &TagGroup, types_prefix: &str) -> String {
    let mut out = String::new();

    // Root trait impl
    out.push_str(&format!(
        "impl {types_prefix}::traits::{trait_name} for {struct_name}<'_> {{\n",
        trait_name = tag.struct_name,
        struct_name = tag.struct_name,
    ));

    // GAT type bindings
    for sg in &tag.sub_groups {
        out.push_str(&format!(
            "    type {}<'b> = {}<'b> where Self: 'b;\n",
            sg.struct_name, sg.struct_name,
        ));
    }
    if !tag.sub_groups.is_empty() {
        out.push('\n');
    }

    // Accessor methods
    for sg in &tag.sub_groups {
        out.push_str(&format!(
            "    fn {accessor}<'b>(&'b self, {param}: &'b str) -> Self::{gat}<'b> {{\n        {struct_name} {{ client: self.client, {param} }}\n    }}\n\n",
            accessor = sg.accessor_fn,
            param = escape_keyword(&sg.primary_param),
            gat = sg.struct_name,
            struct_name = sg.struct_name,
        ));
    }

    // Root async methods — delegate to inherent methods
    for ep in &tag.root_endpoints {
        emit_trait_impl_method(&mut out, ep, types_prefix, None);
    }

    out.push_str("}\n\n");

    // Sub-resource trait impls
    for sg in &tag.sub_groups {
        out.push_str(&format!(
            "impl {types_prefix}::traits::{trait_name} for {struct_name}<'_> {{\n",
            trait_name = sg.struct_name,
            struct_name = sg.struct_name,
        ));
        for ep in &sg.endpoints {
            emit_trait_impl_method(&mut out, ep, types_prefix, Some(&sg.primary_param));
        }
        out.push_str("}\n\n");
    }

    out
}

/// Emit a single trait method impl that delegates to the existing inherent method.
fn emit_trait_impl_method(
    out: &mut String,
    ep: &Endpoint,
    types_prefix: &str,
    skip_primary: Option<&str>,
) {
    if ep.body_kind == Some(crate::parser::RequestBodyKind::FormEncoded) {
        return;
    }

    let return_ty = match &ep.response_inner {
        Some(inner) => format!("{types_prefix}::types::{inner}"),
        None => match &ep.response_type {
            Some(ty) => format!("{types_prefix}::types::{ty}"),
            None => "()".into(),
        },
    };
    let return_result = format!("Result<{return_ty}, NifiError>");

    // Path params (excluding primary)
    let path_param_args: String = ep
        .path_params
        .iter()
        .filter(|p| skip_primary.is_none_or(|pp| p.name != pp))
        .map(|p| format!(", {}: &str", escape_keyword(&p.name)))
        .collect();

    let query_param_args: String = ep
        .query_params
        .iter()
        .map(|qp| {
            let rust_type = query_param_rust_type(qp);
            if qp.required {
                format!(", {}: {rust_type}", escape_keyword(&qp.rust_name))
            } else {
                format!(", {}: Option<{rust_type}>", escape_keyword(&qp.rust_name))
            }
        })
        .collect();

    let body_arg = if ep.method == HttpMethod::Delete {
        String::new()
    } else {
        match &ep.body_kind {
            Some(RequestBodyKind::Json) => {
                let ty = ep.request_type.as_deref().unwrap_or("serde_json::Value");
                format!(", body: &{types_prefix}::types::{ty}")
            }
            Some(RequestBodyKind::OctetStream) => {
                ", filename: Option<&str>, data: Vec<u8>".to_string()
            }
            Some(RequestBodyKind::FormEncoded) | None => String::new(),
        }
    };

    out.push_str(&format!(
        "    async fn {fn_name}(&self{path_param_args}{query_param_args}{body_arg}) -> {return_result} {{\n",
        fn_name = ep.fn_name,
    ));

    // Forward call to inherent method
    let mut call_args = Vec::new();
    for p in &ep.path_params {
        if skip_primary.is_none_or(|pp| p.name != pp) {
            call_args.push(escape_keyword(&p.name));
        }
    }
    for qp in &ep.query_params {
        call_args.push(escape_keyword(&qp.rust_name));
    }
    if ep.method != HttpMethod::Delete {
        match &ep.body_kind {
            Some(RequestBodyKind::Json) => call_args.push("body".to_string()),
            Some(RequestBodyKind::OctetStream) => {
                call_args.push("filename".to_string());
                call_args.push("data".to_string());
            }
            _ => {}
        }
    }
    let call_args_str = call_args.join(", ");

    out.push_str(&format!(
        "        self.{fn_name}({call_args_str}).await\n",
        fn_name = ep.fn_name,
    ));
    out.push_str("    }\n\n");
}
```

Then modify `emit_tag` to call `emit_trait_impls` when a `types_prefix` is available. Since `emit_tag` currently doesn't receive the prefix, change `emit_api_with_prefix` to pass it through. The simplest approach: add `emit_trait_impls` call in `emit_tag` by making it accept an optional `types_prefix`:

In `emit_tag`, after the sub-struct definitions (after line 68 `out.push_str(&emit_sub_struct(sg));`), add:

```rust
if types_prefix != "crate" {
    out.push_str(&emit_trait_impls(tag, types_prefix));
}
```

And update `emit_tag` to accept `types_prefix: &str`, threading it through from `emit_api_with_prefix`.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p nifi-openapi-gen emit_api_includes_trait_impls`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/nifi-openapi-gen/src/emit/api.rs
git commit -m "feat(openapi-gen): emit trait impl blocks for static API structs"
```

---

### Task 4: Wire static traits into `generate.rs`

Add the static trait generation step to the orchestrator so traits are emitted per version.

**Files:**
- Modify: `crates/nifi-openapi-gen/src/bin/generate.rs`

- [ ] **Step 1: Add trait generation to the per-version loop**

In `generate.rs`, inside the per-version loop (after the `emit_api_with_prefix` call at ~line 98), add:

```rust
for (filename, content) in emit_static_traits(spec, &types_prefix) {
    targets.push((versioned_src.join("traits").join(&filename), content));
}
```

Add the import at the top of the file:

```rust
use nifi_openapi_gen::emit_static_traits;
```

- [ ] **Step 2: Update the version `mod.rs` to include traits**

In `generate.rs`, find where `mod.rs` is emitted for each version (~line 99-102):

```rust
targets.push((
    versioned_src.join("mod.rs"),
    "pub mod api;\npub mod types;\n".to_string(),
));
```

Change to:

```rust
targets.push((
    versioned_src.join("mod.rs"),
    "pub mod api;\npub mod traits;\npub mod types;\n".to_string(),
));
```

- [ ] **Step 3: Add `emit_static_traits` to the public API**

In `crates/nifi-openapi-gen/src/emit/mod.rs`, the `pub use traits::emit_static_traits;` line was already added in Task 2 Step 4. Verify it's also re-exported from the crate root if needed. Check `crates/nifi-openapi-gen/src/lib.rs` and add the re-export if necessary:

```rust
pub use emit::emit_static_traits;
```

- [ ] **Step 4: Run the generator and verify it compiles**

```bash
cargo run -p nifi-openapi-gen
cargo build -p nifi-rust-client
cargo build -p nifi-rust-client --no-default-features --features nifi-2-6-0
```

Expected: All versions compile. New `traits/` directories appear under each version module.

- [ ] **Step 5: Run all tests**

```bash
cargo test --workspace
```

Expected: All existing tests pass. No regressions.

- [ ] **Step 6: Commit**

```bash
git add crates/nifi-openapi-gen/src/bin/generate.rs \
        crates/nifi-openapi-gen/src/lib.rs \
        crates/nifi-rust-client/src/
git commit -m "feat: generate per-version static trait modules"
```

---

### Task 5: Refactor dynamic traits from flat to hierarchical

Modify `emit/dynamic/traits.rs` to emit root traits with GAT accessors and sub-resource traits.

**Files:**
- Modify: `crates/nifi-openapi-gen/src/emit/dynamic/traits.rs`

- [ ] **Step 1: Update the existing test (or add a new one)**

The file currently has no tests. Add one at the bottom:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;

    fn make_spec() -> ApiSpec {
        let ep_root = Endpoint {
            method: HttpMethod::Get,
            path: "/controller-services/{id}".to_string(),
            fn_name: "get_controller_service".to_string(),
            doc: Some("Gets a controller service".to_string()),
            description: None,
            path_params: vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            request_type: None,
            body_kind: None,
            body_doc: None,
            response_type: Some("ControllerServiceEntity".to_string()),
            response_inner: Some("ControllerServiceDto".to_string()),
            response_field: Some("component".to_string()),
            query_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        };

        let ep_sub = Endpoint {
            method: HttpMethod::Post,
            path: "/controller-services/{id}/config/analysis".to_string(),
            fn_name: "analyze_configuration".to_string(),
            doc: Some("Performs analysis".to_string()),
            description: None,
            path_params: vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            request_type: Some("ConfigurationAnalysisEntity".to_string()),
            body_kind: Some(RequestBodyKind::Json),
            body_doc: None,
            response_type: Some("ConfigurationAnalysisEntity".to_string()),
            response_inner: Some("ConfigurationAnalysisDto".to_string()),
            response_field: Some("configuration_analysis".to_string()),
            query_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        };

        ApiSpec {
            tags: vec![TagGroup {
                tag: "ControllerServices".to_string(),
                struct_name: "ControllerServicesApi".to_string(),
                module_name: "controller_services".to_string(),
                accessor_fn: "controller_services_api".to_string(),
                types: vec![],
                root_endpoints: vec![ep_root],
                sub_groups: vec![SubGroup {
                    name: "config".to_string(),
                    struct_name: "ControllerServicesConfigApi".to_string(),
                    accessor_fn: "config".to_string(),
                    primary_param: "id".to_string(),
                    primary_param_doc: Some("The controller service id.".to_string()),
                    endpoints: vec![ep_sub],
                }],
            }],
            all_types: vec![],
        }
    }

    #[test]
    fn emit_dynamic_traits_hierarchical() {
        let spec = make_spec();
        let versions: Vec<(&str, &str, &str, &ApiSpec)> =
            vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];
        let files = emit_dynamic_traits(&versions);

        let (_, content) = files
            .iter()
            .find(|(f, _)| f == "controller_services.rs")
            .unwrap();

        // Root trait has GAT accessor
        assert!(content.contains("pub trait ControllerServicesApi"));
        assert!(content.contains("type ControllerServicesConfigApi<'b>"));
        assert!(content.contains("fn config<'b>(&'b self, id: &'b str)"));

        // Root method — id as method param (no sub-struct for root methods)
        assert!(content.contains("async fn get_controller_service("));

        // Sub-resource trait — no id param on methods
        assert!(content.contains("pub trait ControllerServicesConfigApi"));
        let config_trait_start = content.find("pub trait ControllerServicesConfigApi").unwrap();
        let config_section = &content[config_trait_start..];
        let analyze_start = config_section.find("fn analyze_configuration").unwrap();
        let analyze_end = config_section[analyze_start..].find('{').unwrap();
        let analyze_sig = &config_section[analyze_start..analyze_start + analyze_end];
        assert!(
            !analyze_sig.contains("id: &str"),
            "Sub-resource trait method should not have primary param"
        );

        // Body type uses dynamic types:: prefix
        assert!(content.contains("body: &types::ConfigurationAnalysisEntity"));

        // mod.rs re-exports both traits
        let (_, mod_content) = files.iter().find(|(f, _)| f == "mod.rs").unwrap();
        assert!(mod_content.contains("ControllerServicesApi"));
        assert!(mod_content.contains("ControllerServicesConfigApi"));
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p nifi-openapi-gen emit_dynamic_traits_hierarchical`
Expected: FAIL — current flat output doesn't match assertions

- [ ] **Step 3: Rewrite `emit_trait_file` to emit hierarchical traits**

Replace the `emit_trait_file` function with a hierarchical version. The key change: instead of iterating `collect_tag_endpoints` (flat), use `collect_tag_sub_groups` (hierarchical).

Replace the body of `emit_trait_file` with logic that:

1. Emits sub-resource traits first (one per sub-group), where each method excludes the primary param and uses `&types::T` for bodies
2. Emits the root trait with GAT declarations + accessor methods for each sub-group, plus root-level async methods that keep path params in the signature
3. Default impls on sub-resource traits return `UnsupportedEndpoint`

The existing `emit_trait_method` and `emit_doc_comments` functions need to be updated:
- `emit_trait_method` needs a `skip_primary: Option<&str>` parameter to exclude the primary path param from sub-resource methods
- Body args change from `body: types::T` (owned) to `body: &types::T` (borrowed) to match the design decision

Update `mod.rs` generation in `emit_dynamic_traits` to also re-export sub-resource trait names.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p nifi-openapi-gen emit_dynamic_traits_hierarchical`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/nifi-openapi-gen/src/emit/dynamic/traits.rs
git commit -m "refactor(openapi-gen): emit hierarchical dynamic traits with GAT accessors"
```

---

### Task 6: Add sub-resource dispatch structs

Modify `emit/dynamic/dispatch.rs` to emit sub-resource dispatch structs alongside the existing top-level dispatch enum.

**Files:**
- Modify: `crates/nifi-openapi-gen/src/emit/dynamic/dispatch.rs`

- [ ] **Step 1: Write the test**

Add to the bottom of `dispatch.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;

    fn make_spec() -> ApiSpec {
        // Identical to Task 5's make_spec — copy the full body from Task 5 Step 1.
        // Creates a ControllerServices tag with one root endpoint (get_controller_service)
        // and one sub-group "config" with one endpoint (analyze_configuration).
        let ep_root = Endpoint {
            method: HttpMethod::Get,
            path: "/controller-services/{id}".to_string(),
            fn_name: "get_controller_service".to_string(),
            doc: Some("Gets a controller service".to_string()),
            description: None,
            path_params: vec![PathParam { name: "id".to_string(), doc: None }],
            request_type: None, body_kind: None, body_doc: None,
            response_type: Some("ControllerServiceEntity".to_string()),
            response_inner: Some("ControllerServiceDto".to_string()),
            response_field: Some("component".to_string()),
            query_params: vec![], success_responses: vec![],
            error_responses: vec![], security: None,
        };
        let ep_sub = Endpoint {
            method: HttpMethod::Post,
            path: "/controller-services/{id}/config/analysis".to_string(),
            fn_name: "analyze_configuration".to_string(),
            doc: Some("Performs analysis".to_string()),
            description: None,
            path_params: vec![PathParam { name: "id".to_string(), doc: None }],
            request_type: Some("ConfigurationAnalysisEntity".to_string()),
            body_kind: Some(RequestBodyKind::Json), body_doc: None,
            response_type: Some("ConfigurationAnalysisEntity".to_string()),
            response_inner: Some("ConfigurationAnalysisDto".to_string()),
            response_field: Some("configuration_analysis".to_string()),
            query_params: vec![], success_responses: vec![],
            error_responses: vec![], security: None,
        };
        ApiSpec {
            tags: vec![TagGroup {
                tag: "ControllerServices".to_string(),
                struct_name: "ControllerServicesApi".to_string(),
                module_name: "controller_services".to_string(),
                accessor_fn: "controller_services_api".to_string(),
                types: vec![],
                root_endpoints: vec![ep_root],
                sub_groups: vec![SubGroup {
                    name: "config".to_string(),
                    struct_name: "ControllerServicesConfigApi".to_string(),
                    accessor_fn: "config".to_string(),
                    primary_param: "id".to_string(),
                    primary_param_doc: Some("The controller service id.".to_string()),
                    endpoints: vec![ep_sub],
                }],
            }],
            all_types: vec![],
        }
    }

    #[test]
    fn emit_dispatch_includes_sub_resource_structs() {
        let spec = make_spec();
        let versions: Vec<(&str, &str, &str, &ApiSpec)> =
            vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];
        let files = emit_dynamic_dispatch(&versions);

        let (_, content) = files
            .iter()
            .find(|(f, _)| f == "controller_services.rs")
            .unwrap();

        // Top-level dispatch enum still exists
        assert!(content.contains("pub enum ControllerServicesApiDispatch<'a>"));

        // Has GAT impl on top-level dispatch
        assert!(content.contains("type ControllerServicesConfigApi<'b>"));
        assert!(content.contains("fn config<'b>"));

        // Sub-resource dispatch struct
        assert!(content.contains("pub struct ControllerServicesConfigApiDispatch<'a>"));
        assert!(content.contains("id: String"));

        // Sub-resource dispatch implements the sub-resource trait
        assert!(content.contains(
            "impl crate::dynamic::traits::ControllerServicesConfigApi for ControllerServicesConfigApiDispatch<'_>"
        ));

        // mod.rs exports both dispatch types
        let (_, mod_content) = files.iter().find(|(f, _)| f == "mod.rs").unwrap();
        assert!(mod_content.contains("ControllerServicesApiDispatch"));
        assert!(mod_content.contains("ControllerServicesConfigApiDispatch"));
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p nifi-openapi-gen emit_dispatch_includes_sub_resource`
Expected: FAIL

- [ ] **Step 3: Implement sub-resource dispatch structs**

Modify `emit_dispatch_file` to:

1. Keep the existing top-level dispatch enum generation
2. After the enum, emit one dispatch struct per sub-group:
   ```rust
   pub struct ControllerServicesConfigApiDispatch<'a> {
       pub(crate) client: &'a crate::NifiClient,
       pub(crate) id: String,
       pub(crate) version: crate::dynamic::DetectedVersion,
   }
   ```
3. Each struct implements its sub-resource trait with version-match dispatch that constructs the static sub-struct internally
4. The top-level dispatch enum's trait impl gains GAT bindings and accessor methods that construct the sub-resource dispatch structs

The top-level dispatch enum needs helper methods `client(&self)` and `version(&self)` (or inline match arms in the accessor). Since the enum variants wrap per-version structs, add a match that extracts the client reference and constructs a `DetectedVersion`.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p nifi-openapi-gen emit_dispatch_includes_sub_resource`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/nifi-openapi-gen/src/emit/dynamic/dispatch.rs
git commit -m "feat(openapi-gen): emit sub-resource dispatch structs for dynamic mode"
```

---

### Task 7: Update dynamic impls for sub-resource traits

Modify `emit/dynamic/impls.rs` so per-version impls implement the sub-resource traits. The `id` param moves from method args to `self.id` on the dispatch struct.

**Files:**
- Modify: `crates/nifi-openapi-gen/src/emit/dynamic/impls.rs`

- [ ] **Step 1: Write the test**

Add a test that verifies the impl methods no longer have `id` as a parameter for sub-group endpoints:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::*;

    fn make_spec() -> ApiSpec {
        // Identical to Task 5/6's make_spec — copy the full body from Task 6 Step 1.
        // Creates a ControllerServices tag with one root endpoint (get_controller_service)
        // and one sub-group "config" with one endpoint (analyze_configuration).
        let ep_root = Endpoint {
            method: HttpMethod::Get,
            path: "/controller-services/{id}".to_string(),
            fn_name: "get_controller_service".to_string(),
            doc: Some("Gets a controller service".to_string()),
            description: None,
            path_params: vec![PathParam { name: "id".to_string(), doc: None }],
            request_type: None, body_kind: None, body_doc: None,
            response_type: Some("ControllerServiceEntity".to_string()),
            response_inner: Some("ControllerServiceDto".to_string()),
            response_field: Some("component".to_string()),
            query_params: vec![], success_responses: vec![],
            error_responses: vec![], security: None,
        };
        let ep_sub = Endpoint {
            method: HttpMethod::Post,
            path: "/controller-services/{id}/config/analysis".to_string(),
            fn_name: "analyze_configuration".to_string(),
            doc: Some("Performs analysis".to_string()),
            description: None,
            path_params: vec![PathParam { name: "id".to_string(), doc: None }],
            request_type: Some("ConfigurationAnalysisEntity".to_string()),
            body_kind: Some(RequestBodyKind::Json), body_doc: None,
            response_type: Some("ConfigurationAnalysisEntity".to_string()),
            response_inner: Some("ConfigurationAnalysisDto".to_string()),
            response_field: Some("configuration_analysis".to_string()),
            query_params: vec![], success_responses: vec![],
            error_responses: vec![], security: None,
        };
        ApiSpec {
            tags: vec![TagGroup {
                tag: "ControllerServices".to_string(),
                struct_name: "ControllerServicesApi".to_string(),
                module_name: "controller_services".to_string(),
                accessor_fn: "controller_services_api".to_string(),
                types: vec![],
                root_endpoints: vec![ep_root],
                sub_groups: vec![SubGroup {
                    name: "config".to_string(),
                    struct_name: "ControllerServicesConfigApi".to_string(),
                    accessor_fn: "config".to_string(),
                    primary_param: "id".to_string(),
                    primary_param_doc: Some("The controller service id.".to_string()),
                    endpoints: vec![ep_sub],
                }],
            }],
            all_types: vec![],
        }
    }

    #[test]
    fn emit_impls_sub_resource_methods_exclude_primary_param() {
        let spec = make_spec();
        let versions: Vec<(&str, &str, &str, &ApiSpec)> =
            vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];
        let files = emit_dynamic_impls(&versions);

        let impl_file = files
            .iter()
            .find(|(f, _)| f == "v2_8_0/controller_services.rs")
            .unwrap();
        let content = &impl_file.1;

        // Per-version struct implements the sub-resource trait
        assert!(content.contains(
            "impl crate::dynamic::traits::ControllerServicesConfigApi"
        ));

        // analyze_configuration should NOT have id as param
        let analyze_start = content.find("fn analyze_configuration").unwrap();
        let analyze_end = content[analyze_start..].find('{').unwrap();
        let analyze_sig = &content[analyze_start..analyze_start + analyze_end];
        assert!(
            !analyze_sig.contains("id: &str"),
            "Sub-resource impl method should get id from self, not params"
        );

        // Body should be borrowed
        assert!(analyze_sig.contains("body: &types::ConfigurationAnalysisEntity"));
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p nifi-openapi-gen emit_impls_sub_resource_methods`
Expected: FAIL

- [ ] **Step 3: Implement the changes**

Modify `emit_dynamic_impls` to:

1. Instead of emitting one large struct per version per tag that implements a single flat trait, emit:
   - A root struct implementing the root trait (with GAT bindings + accessor methods + root async methods)
   - Per-sub-group: the impl is on the dispatch struct from Task 6, not a new per-version struct. The dispatch struct's match arms construct the static sub-struct using `&self.id` and delegate.

2. Sub-resource method bodies change from receiving `id` as a param to using `self.id`:
   ```rust
   let api = crate::v2_8_0::api::controller_services::ControllerServicesConfigApi {
       client: self.client,
       id: &self.id,  // was: id (from method param)
   };
   ```

3. Body args change from `body: types::T` (owned) to `body: &types::T` (borrowed). The conversion call changes from `try_from(body)?` to `try_from(body.clone())?`.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p nifi-openapi-gen emit_impls_sub_resource_methods`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/nifi-openapi-gen/src/emit/dynamic/impls.rs
git commit -m "refactor(openapi-gen): update dynamic impls for sub-resource trait hierarchy"
```

---

### Task 8: Update dynamic entry.rs for DynamicClient accessor changes

The `DynamicClient` accessor methods (e.g., `fn controller_services_api()`) don't change shape — they still return dispatch enums. But the dispatch enum's trait impl now has GATs, so verify everything wires through.

**Files:**
- Modify: `crates/nifi-openapi-gen/src/emit/dynamic/entry.rs` (if needed)

- [ ] **Step 1: Run the full generator and build**

```bash
cargo run -p nifi-openapi-gen
cargo build -p nifi-rust-client --features dynamic
cargo clippy -p nifi-rust-client --features dynamic -- -D warnings
```

Expected: Compiles cleanly. If there are errors, fix them in the relevant emitter.

- [ ] **Step 2: Run all tests**

```bash
cargo test --workspace
```

Expected: All tests pass. Wiremock tests should be unaffected (they use inherent methods, not traits).

- [ ] **Step 3: Fix any compilation issues**

If the build or tests fail, trace the errors to the specific emitter and fix. Common issues:
- Missing trait imports in generated files
- GAT lifetime mismatches
- Type prefix mismatches in trait impl signatures vs trait definitions

- [ ] **Step 4: Commit**

```bash
git add crates/nifi-openapi-gen/ crates/nifi-rust-client/src/
git commit -m "feat: wire through hierarchical traits in dynamic mode"
```

---

### Task 9: Update dynamic tests

The existing `dynamic_tests.rs` (generated wiremock tests for dynamic mode) use the old flat call pattern. Update the test emitter to use the builder pattern.

**Files:**
- Modify: `crates/nifi-openapi-gen/src/emit/dynamic/tests.rs`

- [ ] **Step 1: Check current test emission pattern**

Read `crates/nifi-openapi-gen/src/emit/dynamic/tests.rs` to understand how tests are generated. The key change: test call sites that currently do `api.method_name(id, body)` need to become `api.sub_resource(id).method_name(&body)`.

- [ ] **Step 2: Modify test emission to use builder pattern**

For endpoints that belong to a sub-group, the test call should use the accessor pattern:

Before:
```rust
client.controller_services_api().analyze_configuration("test-id", body).await.unwrap();
```

After:
```rust
client.controller_services_api().config("test-id").analyze_configuration(&body).await.unwrap();
```

Update the emitter logic to detect sub-group endpoints (via `EndpointInfo::sub_struct_name`) and emit the accessor call.

- [ ] **Step 3: Regenerate and run tests**

```bash
cargo run -p nifi-openapi-gen
cargo test -p nifi-rust-client --features dynamic
```

Expected: All dynamic tests pass with the new call pattern.

- [ ] **Step 4: Commit**

```bash
git add crates/nifi-openapi-gen/src/emit/dynamic/tests.rs crates/nifi-rust-client/tests/
git commit -m "fix(openapi-gen): update dynamic test emission for builder pattern"
```

---

### Task 10: Full integration verification

Run the complete test matrix to ensure nothing is broken.

**Files:** None (verification only)

- [ ] **Step 1: Build all configurations**

```bash
cargo build -p nifi-rust-client
cargo build -p nifi-rust-client --no-default-features --features nifi-2-6-0
cargo build -p nifi-rust-client --no-default-features --features nifi-2-7-2
cargo build -p nifi-rust-client --no-default-features --features nifi-2-8-0
cargo build -p nifi-rust-client --features dynamic
```

- [ ] **Step 2: Clippy all configurations**

```bash
cargo clippy -p nifi-rust-client -- -D warnings
cargo clippy -p nifi-rust-client --features dynamic -- -D warnings
```

- [ ] **Step 3: Run full test suite**

```bash
cargo test --workspace
```

- [ ] **Step 4: Verify trait usability**

Manually inspect a generated trait file to confirm:
- GAT declarations are correct
- Accessor method signatures match
- Sub-resource trait methods exclude primary param
- Type prefixes are correct for both static and dynamic

```bash
head -80 crates/nifi-rust-client/src/v2_8_0/traits/controller_services.rs
head -80 crates/nifi-rust-client/src/dynamic/traits/controller_services.rs
```

- [ ] **Step 5: Commit any remaining fixes**

```bash
git add -A
git commit -m "fix: resolve remaining issues from trait hierarchy migration"
```

---

### Task 11: Documentation audit — READMEs and AGENTS.md

Update all documentation to reflect the new trait hierarchy and remove outdated path-parameter-difference notes.

**Files:**
- Modify: `AGENTS.md`
- Modify: `README.md`
- Modify: `crates/nifi-rust-client/README.md`
- Modify: `crates/nifi-openapi-gen/README.md` (if it documents emitters)

- [ ] **Step 1: Update AGENTS.md**

In the "Dynamic mode" section:
- Update the dispatch layer description to mention sub-resource traits and dispatch structs
- Remove or update the "API ergonomics — path parameters differ between modes" paragraph (added earlier in this conversation) — replace with a note that both modes now use the same builder pattern
- Add `traits/` to the workspace structure for version modules
- Update the code example to show the builder pattern for dynamic mode

- [ ] **Step 2: Update `crates/nifi-rust-client/README.md`**

- Remove the "Path parameter differences" subsection (parity achieved)
- Update the dynamic mode code example to show builder-chain calls
- Update trait import examples to show sub-resource trait imports
- Verify the "When to use which" table still applies

- [ ] **Step 3: Update `README.md` (root)**

- Update the dynamic mode code example to show builder pattern
- Remove the path-parameter-difference note from the "Two modes" paragraph
- Optionally mention that both modes support traits for mockability

- [ ] **Step 4: Check `crates/nifi-openapi-gen/README.md`**

Read the file and verify the emitter documentation (if any) reflects the new `emit/traits.rs` and modified dynamic emitters.

- [ ] **Step 5: Full documentation review pass**

Read through all modified READMEs end-to-end. Check for:
- Stale code examples that use the old flat dynamic API
- References to files/modules that no longer exist or have moved
- Version numbers or feature flags that need updating
- Internal links that may have broken

- [ ] **Step 6: Commit**

```bash
git add AGENTS.md README.md crates/nifi-rust-client/README.md crates/nifi-openapi-gen/README.md
git commit -m "docs: update all documentation for unified trait hierarchy"
```

- [ ] **Step 7: Run pre-commit hooks**

```bash
pre-commit run --all-files
```

Fix any formatting or linting issues, then commit fixes.
