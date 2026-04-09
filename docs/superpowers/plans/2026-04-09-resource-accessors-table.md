# Resource Accessors Table Auto-Generation — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Auto-generate the resource accessors table in `crates/nifi-rust-client/README.md` from parsed OpenAPI specs, showing per-version endpoint counts.

**Architecture:** A new doc emitter (`resource_accessors.rs`) in `crates/nifi-openapi-gen/src/docs/` generates the markdown table from `Vec<(String, ApiSpec)>`. It follows the same pattern as `versions_table.rs` — a pure function returning a `String`, called from `generate.rs` which pipes the result through `update_file_between_markers()`.

**Tech Stack:** Rust, existing `ApiSpec`/`TagGroup` parser types, `update_file_between_markers` utility.

---

### Task 1: Add markers to the client README

**Files:**
- Modify: `crates/nifi-rust-client/README.md:261-290`

The current hand-written table (lines 261–290) gets wrapped with markers. The heading (line 251), intro paragraph (lines 253–259), and anything after the table stay untouched.

- [ ] **Step 1: Add markers around the existing table**

Replace the table block (lines 261–290) with:

```markdown
<!-- RESOURCE_ACCESSORS_START -->
| Accessor | Resource |
|----------|----------|
| `client.access_api()` | Token exchange, access configuration |
| `client.authentication_api()` | Authentication configuration |
| `client.connections_api()` | Connection CRUD, flow-file queue management |
| `client.controller_api()` | Controller-level config and bulletins |
| `client.controller_services_api()` | Controller service CRUD |
| `client.counters_api()` | Counter reset and retrieval |
| `client.datatransfer_api()` | Site-to-site data transfer |
| `client.flow_api()` | Flow status, about, current user, registry clients, … |
| `client.flowfilequeues_api()` | Flow-file queue inspection and purge |
| `client.funnels_api()` | Funnel CRUD |
| `client.inputports_api()` | Input port CRUD |
| `client.labels_api()` | Label CRUD |
| `client.outputports_api()` | Output port CRUD |
| `client.parametercontexts_api()` | Parameter context CRUD |
| `client.parameterproviders_api()` | Parameter provider CRUD |
| `client.policies_api()` | Access policy CRUD |
| `client.processgroups_api()` | Process group CRUD, instantiation, export |
| `client.processors_api()` | Processor CRUD |
| `client.provenance_api()` | Provenance queries |
| `client.provenanceevents_api()` | Provenance event retrieval |
| `client.remoteprocessgroups_api()` | Remote process group CRUD |
| `client.reportingtasks_api()` | Reporting task CRUD |
| `client.resources_api()` | Resource listing |
| `client.sitetosite_api()` | Site-to-site configuration |
| `client.snippets_api()` | Snippet CRUD |
| `client.systemdiagnostics_api()` | System diagnostics |
| `client.tenants_api()` | User and group CRUD |
| `client.versions_api()` | Flow version control |
<!-- RESOURCE_ACCESSORS_END -->
```

This preserves the current content so the README reads correctly before the generator runs.

- [ ] **Step 2: Verify the README renders correctly**

Run: `head -n 295 crates/nifi-rust-client/README.md | tail -n 45`

Expected: the table with `<!-- RESOURCE_ACCESSORS_START -->` before and `<!-- RESOURCE_ACCESSORS_END -->` after.

- [ ] **Step 3: Commit**

```bash
git add crates/nifi-rust-client/README.md
git commit -m "docs: add RESOURCE_ACCESSORS markers to client README"
```

---

### Task 2: Write the `resource_accessors.rs` doc emitter with tests

**Files:**
- Create: `crates/nifi-openapi-gen/src/docs/resource_accessors.rs`
- Modify: `crates/nifi-openapi-gen/src/docs/mod.rs`

- [ ] **Step 1: Write the test**

Create `crates/nifi-openapi-gen/src/docs/resource_accessors.rs` with a test that defines two fake specs (one with 2 tags, one with 3 tags where one is new) and asserts the generated table has the right structure:

```rust
use crate::parser::{ApiSpec, Endpoint, TagGroup};

/// Per-tag endpoint count: root endpoints + all sub-group endpoints.
fn tag_endpoint_count(tag: &TagGroup) -> usize {
    tag.root_endpoints.len()
        + tag
            .sub_groups
            .iter()
            .map(|sg| sg.endpoints.len())
            .sum::<usize>()
}

/// Split a PascalCase tag name into words, lowercase all but the first.
/// e.g. "ControllerServices" → "Controller services"
fn tag_to_description(tag: &str) -> String {
    let mut words = Vec::new();
    let mut current = String::new();
    for ch in tag.chars() {
        if ch.is_uppercase() && !current.is_empty() {
            words.push(current);
            current = String::new();
        }
        current.push(ch);
    }
    if !current.is_empty() {
        words.push(current);
    }
    if words.is_empty() {
        return String::new();
    }
    let first = &words[0];
    let rest: Vec<String> = words[1..].iter().map(|w| w.to_lowercase()).collect();
    if rest.is_empty() {
        first.clone()
    } else {
        format!("{} {}", first, rest.join(" "))
    }
}

const MAX_VERSION_COLUMNS: usize = 10;

/// Generate the resource accessors table content.
/// `all_specs` must be sorted semver-ascending (oldest first).
pub fn generate_resource_accessors_content(all_specs: &[(String, ApiSpec)]) -> String {
    // Collect up to MAX_VERSION_COLUMNS most recent versions
    let version_count = all_specs.len().min(MAX_VERSION_COLUMNS);
    let recent_specs = &all_specs[all_specs.len() - version_count..];

    // Collect union of all tag accessor_fn → tag name across all recent specs
    let mut all_tags: std::collections::BTreeMap<String, String> = std::collections::BTreeMap::new();
    for (_version, spec) in recent_specs {
        for tag in &spec.tags {
            all_tags
                .entry(tag.accessor_fn.clone())
                .or_insert_with(|| tag.tag.clone());
        }
    }

    // Build header
    let mut header = "| Accessor | Resource".to_string();
    for (version, _) in recent_specs {
        header.push_str(&format!(" | {version}"));
    }
    header.push_str(" |");

    let mut separator = "|----------|----------".to_string();
    for _ in recent_specs {
        separator.push_str("|---");
    }
    separator.push('|');

    // Build rows
    let mut rows = Vec::new();
    for (accessor_fn, tag_name) in &all_tags {
        let description = tag_to_description(tag_name);
        let mut row = format!("| `client.{accessor_fn}()` | {description}");
        for (_version, spec) in recent_specs {
            let count = spec
                .tags
                .iter()
                .find(|t| &t.accessor_fn == accessor_fn)
                .map(|t| tag_endpoint_count(t).to_string())
                .unwrap_or_else(|| "\u{2014}".to_string());
            row.push_str(&format!(" | {count}"));
        }
        row.push_str(" |");
        rows.push(row);
    }

    let legend = "\n> Numbers indicate the endpoint count available for each accessor in that NiFi version. \u{2014} means the accessor is not available in that version.";

    format!("{header}\n{separator}\n{}\n{legend}", rows.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Endpoint, SubGroup};

    fn make_endpoint(path: &str, method: &str) -> Endpoint {
        Endpoint {
            path: path.to_string(),
            method: method.to_string(),
            operation_id: String::new(),
            summary: None,
            fn_name: String::new(),
            params: vec![],
            query_params: vec![],
            request_body: None,
            response_type: None,
            response_wrapper_field: None,
            produces_octet_stream: false,
            response_is_void: false,
        }
    }

    fn make_tag(tag: &str, accessor_fn: &str, endpoint_count: usize) -> TagGroup {
        let endpoints: Vec<Endpoint> = (0..endpoint_count)
            .map(|i| make_endpoint(&format!("/api/{tag}/{i}"), "GET"))
            .collect();
        TagGroup {
            tag: tag.to_string(),
            struct_name: format!("{tag}Api"),
            module_name: accessor_fn.to_string(),
            accessor_fn: format!("{accessor_fn}_api"),
            types: vec![],
            root_endpoints: endpoints,
            sub_groups: vec![],
        }
    }

    #[test]
    fn generates_table_with_version_columns() {
        let specs = vec![
            (
                "2.6.0".to_string(),
                ApiSpec {
                    tags: vec![
                        make_tag("Flow", "flow", 10),
                        make_tag("Processors", "processors", 5),
                    ],
                    all_types: vec![],
                },
            ),
            (
                "2.7.0".to_string(),
                ApiSpec {
                    tags: vec![
                        make_tag("Flow", "flow", 12),
                        make_tag("Processors", "processors", 5),
                        make_tag("AiServices", "ai_services", 3),
                    ],
                    all_types: vec![],
                },
            ),
        ];

        let result = generate_resource_accessors_content(&specs);

        // Header has version columns
        assert!(result.contains("| 2.6.0 | 2.7.0 |"));
        // Flow row shows counts in both versions
        assert!(result.contains("| `client.flow_api()` | Flow | 10 | 12 |"));
        // AiServices only in 2.7.0
        assert!(result.contains("| `client.ai_services_api()` | Ai services | \u{2014} | 3 |"));
        // Processors in both
        assert!(result.contains("| `client.processors_api()` | Processors | 5 | 5 |"));
        // Legend present
        assert!(result.contains("> Numbers indicate"));
    }

    #[test]
    fn caps_at_ten_versions() {
        let specs: Vec<(String, ApiSpec)> = (0..12)
            .map(|i| {
                (
                    format!("2.{i}.0"),
                    ApiSpec {
                        tags: vec![make_tag("Flow", "flow", 10 + i)],
                        all_types: vec![],
                    },
                )
            })
            .collect();

        let result = generate_resource_accessors_content(&specs);

        // Should NOT contain versions 2.0.0 and 2.1.0 (oldest two)
        assert!(!result.contains("2.0.0"));
        assert!(!result.contains("2.1.0"));
        // Should contain 2.2.0 through 2.11.0 (most recent 10)
        assert!(result.contains("2.2.0"));
        assert!(result.contains("2.11.0"));
    }

    #[test]
    fn tag_to_description_splits_pascal_case() {
        assert_eq!(tag_to_description("ControllerServices"), "Controller services");
        assert_eq!(tag_to_description("Flow"), "Flow");
        assert_eq!(tag_to_description("ProcessGroups"), "Process groups");
        assert_eq!(tag_to_description("SiteToSite"), "Site to site");
    }

    #[test]
    fn includes_sub_group_endpoints_in_count() {
        let tag = TagGroup {
            tag: "Controllers".to_string(),
            struct_name: "ControllersApi".to_string(),
            module_name: "controllers".to_string(),
            accessor_fn: "controllers_api".to_string(),
            types: vec![],
            root_endpoints: vec![make_endpoint("/api/a", "GET")],
            sub_groups: vec![SubGroup {
                name: "config".to_string(),
                struct_name: "ControllersConfigApi".to_string(),
                accessor_fn: "config".to_string(),
                primary_param: "id".to_string(),
                primary_param_doc: None,
                endpoints: vec![
                    make_endpoint("/api/a/{id}/config/x", "GET"),
                    make_endpoint("/api/a/{id}/config/y", "PUT"),
                ],
            }],
        };

        let specs = vec![(
            "2.8.0".to_string(),
            ApiSpec {
                tags: vec![tag],
                all_types: vec![],
            },
        )];

        let result = generate_resource_accessors_content(&specs);
        // 1 root + 2 sub-group = 3
        assert!(result.contains("| 3 |"));
    }
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test -p nifi-openapi-gen resource_accessors -- --nocapture 2>&1 | head -30`

Expected: compilation error — `mod resource_accessors` not declared yet.

- [ ] **Step 3: Register the module in `docs/mod.rs`**

In `crates/nifi-openapi-gen/src/docs/mod.rs`, add:

```rust
mod resource_accessors;

pub use resource_accessors::generate_resource_accessors_content;
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p nifi-openapi-gen resource_accessors -- --nocapture`

Expected: all 4 tests pass.

- [ ] **Step 5: Run clippy**

Run: `cargo clippy -p nifi-openapi-gen -- -D warnings`

Expected: no warnings.

- [ ] **Step 6: Commit**

```bash
git add crates/nifi-openapi-gen/src/docs/resource_accessors.rs crates/nifi-openapi-gen/src/docs/mod.rs
git commit -m "feat: add resource_accessors doc emitter with tests"
```

---

### Task 3: Integrate emitter into the generator

**Files:**
- Modify: `crates/nifi-openapi-gen/src/bin/generate.rs` (around line 286, near other doc update calls)

- [ ] **Step 1: Add the call to `generate.rs`**

In `generate.rs`, after the existing doc update calls (around line 286 where `update_client_readme_examples` is called), add a new block:

```rust
        // Resource accessors table
        {
            const START: &str = "<!-- RESOURCE_ACCESSORS_START -->";
            const END: &str = "<!-- RESOURCE_ACCESSORS_END -->";
            let content = generate_resource_accessors_content(&all_parsed);
            update_file_between_markers(
                &workspace_root.join("crates/nifi-rust-client/README.md"),
                START,
                END,
                &content,
            );
        }
```

Add `generate_resource_accessors_content` to the existing docs import (line 11–13):

```rust
use nifi_openapi_gen::docs::{
    generate_api_changes_content, generate_resource_accessors_content,
    generate_versions_table_content, update_client_readme_examples,
};
```

- [ ] **Step 2: Verify it compiles**

Run: `cargo build -p nifi-openapi-gen`

Expected: compiles successfully.

- [ ] **Step 3: Run the generator to verify it updates the README**

Run: `cargo run -p nifi-openapi-gen`

Expected: output includes `wrote crates/nifi-rust-client/README.md` (because the table now has version columns).

- [ ] **Step 4: Verify the generated table looks correct**

Run: `grep -A 35 "RESOURCE_ACCESSORS_START" crates/nifi-rust-client/README.md`

Expected: a table with `| Accessor | Resource | X.Y.Z | ... |` columns, endpoint counts per cell, em dashes for missing tags, and the legend line.

- [ ] **Step 5: Run full test suite**

Run: `cargo test --workspace`

Expected: all tests pass.

- [ ] **Step 6: Run pre-commit**

Run: `pre-commit run --all-files`

Expected: all checks pass.

- [ ] **Step 7: Commit**

```bash
git add crates/nifi-openapi-gen/src/bin/generate.rs crates/nifi-rust-client/README.md
git commit -m "feat: integrate resource accessors table into generator pipeline"
```

---

### Task 4: Update AGENTS.md and nifi-openapi-gen README

**Files:**
- Modify: `AGENTS.md`
- Modify: `crates/nifi-openapi-gen/README.md`

- [ ] **Step 1: Update the Architecture table in `crates/nifi-openapi-gen/README.md`**

Add a row for the new module in the `src/docs/` section of the architecture table:

```markdown
| `src/docs/resource_accessors.rs` | Generate the resource accessors table in `crates/nifi-rust-client/README.md` |
```

- [ ] **Step 2: Update AGENTS.md**

In the `Workspace Structure` section, if `src/docs/` listing exists, ensure `resource_accessors.rs` is mentioned. In the doc emitter references, add a note that the resource accessors table is auto-generated.

- [ ] **Step 3: Commit**

```bash
git add AGENTS.md crates/nifi-openapi-gen/README.md
git commit -m "docs: document resource_accessors emitter in AGENTS.md and gen README"
```
