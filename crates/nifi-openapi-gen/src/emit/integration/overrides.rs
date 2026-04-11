//! Central registry of NiFi-specific exceptions for integration test generation.
//!
//! The generator walks every endpoint and every added field uniformly, but some
//! endpoints or fields need bespoke handling that cannot be derived from the
//! OpenAPI spec (NiFi emits no `x-*` extensions, and conditional optionality
//! lives only in human-readable descriptions).
//!
//! Every exception lives here so new NiFi quirks surface in one place. To add
//! a new override on a version bump, append to [`ENDPOINT_OVERRIDES`] or
//! [`FIELD_PRESENCE_OVERRIDES`] — the emitters consult these at build time.
//!
//! Extension point: when a NiFi resource needs a positive availability test
//! but can't be satisfied by the default sentinel (e.g. `/connectors/{id}`
//! once a test helper creates a real connector), add a `CustomSetup` variant
//! to [`EndpointBehavior`] that references a helper fn name, and extend
//! `endpoint_availability::emit_endpoint_availability_tests` to emit a call
//! to that helper before the request.

use crate::diff::VersionDiff;
use crate::parser::{ApiSpec, HttpMethod};

/// Describes which endpoint(s) an override applies to.
#[derive(Debug)]
#[allow(dead_code)] // Exact and PathPrefix are scaffolding for future overrides.
pub enum EndpointScope {
    /// Match one specific method + path exactly.
    Exact {
        method: HttpMethod,
        path: &'static str,
    },
    /// Match any endpoint under a given OpenAPI tag, e.g. "Connectors".
    Tag { name: &'static str },
    /// Match any endpoint whose path starts with the given prefix.
    PathPrefix { prefix: &'static str },
}

/// What the emitter should do with an overridden endpoint.
#[derive(Debug)]
pub enum EndpointBehavior {
    /// Skip the positive availability test. The negative test (asserting
    /// `UnsupportedEndpoint` on older versions) is still emitted.
    SkipPositiveTest { reason: &'static str },
}

#[derive(Debug)]
pub struct EndpointOverride {
    pub scope: EndpointScope,
    pub behavior: EndpointBehavior,
}

#[derive(Debug)]
pub struct FieldPresenceOverride {
    /// Rust type name as it appears in `TypeDef::name`, e.g. `ProvenanceEventDto`.
    pub type_name: &'static str,
    /// Rust snake_case field name, e.g. `connector_id`.
    pub field_name: &'static str,
    pub reason: &'static str,
}

/// Phrases in field descriptions that strongly suggest the field is only
/// populated under certain conditions. Used by the conditional-wording scan
/// to nudge developers during version bumps.
#[allow(dead_code)] // used by scan_* functions which are test-only today.
const CONDITIONAL_WORDING_PATTERNS: &[&str] = &[
    "will not be set",
    "may not be set",
    "will not be populated",
    "may not be populated",
    "if not set",
    "is not set",
];

pub const ENDPOINT_OVERRIDES: &[EndpointOverride] = &[EndpointOverride {
    scope: EndpointScope::Tag { name: "Connectors" },
    behavior: EndpointBehavior::SkipPositiveTest {
        reason: "Connector ids have no \"root\" sentinel; positive tests need a real \
                 connector fixture before they can run. See overrides.rs for the extension point.",
    },
}];

pub const FIELD_PRESENCE_OVERRIDES: &[FieldPresenceOverride] = &[FieldPresenceOverride {
    type_name: "ProvenanceEventDto",
    field_name: "connector_id",
    reason: "Only set when the component that produced the event is managed by a connector; \
             the stock test flow uses unmanaged GenerateFlowFile/LogAttribute processors so \
             the field is legitimately None.",
}];

/// Find an endpoint override, if any. Lookup order: `Exact` → `PathPrefix` →
/// `Tag`, first match wins. This lets an `Exact` entry override a tag-wide
/// skip without deleting the tag entry.
pub fn lookup_endpoint_override(
    method: &HttpMethod,
    path: &str,
    tag: &str,
) -> Option<&'static EndpointOverride> {
    for ov in ENDPOINT_OVERRIDES {
        if let EndpointScope::Exact { method: m, path: p } = &ov.scope {
            if m == method && *p == path {
                return Some(ov);
            }
        }
    }
    for ov in ENDPOINT_OVERRIDES {
        if let EndpointScope::PathPrefix { prefix } = &ov.scope {
            if path.starts_with(prefix) {
                return Some(ov);
            }
        }
    }
    for ov in ENDPOINT_OVERRIDES {
        if let EndpointScope::Tag { name } = &ov.scope {
            if *name == tag {
                return Some(ov);
            }
        }
    }
    None
}

pub fn lookup_field_presence_override(
    type_name: &str,
    field_name: &str,
) -> Option<&'static FieldPresenceOverride> {
    FIELD_PRESENCE_OVERRIDES
        .iter()
        .find(|ov| ov.type_name == type_name && ov.field_name == field_name)
}

/// A description-based warning flagged by [`scan_conditional_fields`].
#[derive(Debug)]
#[allow(dead_code)] // consumed by the conditional-wording scan tests.
pub struct ConditionalFieldWarning {
    pub type_name: String,
    pub field_name: String,
    pub matched_phrase: &'static str,
    pub description: String,
}

/// Walk all field descriptions in the spec and flag any whose wording suggests
/// conditional optionality but which are not already in
/// [`FIELD_PRESENCE_OVERRIDES`]. This scans *every* field in the spec, which
/// is useful for unit testing the primitive but noisy for real specs — prefer
/// [`scan_new_conditional_fields`] for version-bump gating.
#[allow(dead_code)] // test-only primitive.
pub fn scan_conditional_fields(spec: &ApiSpec) -> Vec<ConditionalFieldWarning> {
    let mut out = Vec::new();
    for td in &spec.all_types {
        for f in &td.fields {
            if let Some(w) = check_field(&td.name, f) {
                out.push(w);
            }
        }
    }
    out
}

/// Walk only the *newly added* fields across version diffs and flag any whose
/// description wording suggests conditional optionality but which are not in
/// [`FIELD_PRESENCE_OVERRIDES`].
///
/// This is the scope that matters for the field-presence emitter: it only
/// generates positive tests for fields added in a version, so we only need to
/// nudge developers when a new conditional field lands. On version bumps, a
/// unit test runs this scan against the bundled specs and fails if any
/// warning remains — prompting the developer to either add an override or
/// confirm the field really is always-populated on the stock test flow.
#[allow(dead_code)] // consumed by the version-bump regression test.
pub fn scan_new_conditional_fields(
    all_specs: &[(String, ApiSpec)],
    diffs: &[VersionDiff],
) -> Vec<ConditionalFieldWarning> {
    let mut out = Vec::new();
    for diff in diffs {
        let Some((_, to_spec)) = all_specs.iter().find(|(v, _)| v == &diff.to) else {
            continue;
        };
        for tc in &diff.types.changed {
            if tc.added_fields.is_empty() {
                continue;
            }
            let Some(td) = to_spec.all_types.iter().find(|t| t.name == tc.name) else {
                continue;
            };
            for added_field in &tc.added_fields {
                let Some(f) = td.fields.iter().find(|f| &f.rust_name == added_field) else {
                    continue;
                };
                if let Some(w) = check_field(&td.name, f) {
                    out.push(w);
                }
            }
        }
    }
    out
}

#[allow(dead_code)] // test-only helper.
fn check_field(type_name: &str, f: &crate::parser::Field) -> Option<ConditionalFieldWarning> {
    let doc = f.doc.as_ref()?;
    let lower = doc.to_lowercase();
    for phrase in CONDITIONAL_WORDING_PATTERNS {
        if lower.contains(phrase) {
            if lookup_field_presence_override(type_name, &f.rust_name).is_some() {
                return None;
            }
            return Some(ConditionalFieldWarning {
                type_name: type_name.to_string(),
                field_name: f.rust_name.clone(),
                matched_phrase: phrase,
                description: doc.clone(),
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{Field, FieldType, TypeDef, TypeKind};

    fn dto(name: &str, fields: Vec<Field>) -> TypeDef {
        TypeDef {
            name: name.to_string(),
            kind: TypeKind::Dto,
            fields,
            doc: None,
        }
    }

    fn opt_str_field(rust_name: &str, serde_name: &str, doc: Option<&str>) -> Field {
        Field {
            rust_name: rust_name.to_string(),
            serde_name: serde_name.to_string(),
            ty: FieldType::Opt(Box::new(FieldType::Str)),
            doc: doc.map(String::from),
            read_only: false,
            deprecated: false,
        }
    }

    #[test]
    fn lookup_endpoint_override_returns_none_for_unknown_endpoint() {
        let found = lookup_endpoint_override(&HttpMethod::Get, "/flow/about", "Flow");
        assert!(found.is_none());
    }

    #[test]
    fn tag_wide_override_matches_any_endpoint_in_tag() {
        let found = lookup_endpoint_override(&HttpMethod::Get, "/connectors/{id}", "Connectors");
        assert!(matches!(
            found.map(|o| &o.behavior),
            Some(EndpointBehavior::SkipPositiveTest { .. })
        ));
    }

    #[test]
    fn tag_wide_override_matches_sub_paths_in_tag() {
        let found = lookup_endpoint_override(
            &HttpMethod::Get,
            "/connectors/{id}/configuration-steps",
            "Connectors",
        );
        assert!(found.is_some());
    }

    #[test]
    fn non_connectors_tag_not_matched_by_connectors_override() {
        let found = lookup_endpoint_override(&HttpMethod::Get, "/flow/connectors", "Flow");
        assert!(found.is_none());
    }

    #[test]
    fn lookup_field_presence_override_finds_known_entry() {
        let found = lookup_field_presence_override("ProvenanceEventDto", "connector_id");
        assert!(found.is_some());
    }

    #[test]
    fn lookup_field_presence_override_misses_other_fields() {
        let found = lookup_field_presence_override("ProvenanceEventDto", "event_timestamp");
        assert!(found.is_none());
    }

    #[test]
    fn registry_has_no_duplicate_field_overrides() {
        let mut seen = std::collections::HashSet::new();
        for ov in FIELD_PRESENCE_OVERRIDES {
            let key = (ov.type_name, ov.field_name);
            assert!(
                seen.insert(key),
                "duplicate field presence override: {key:?}"
            );
        }
    }

    #[test]
    fn registry_has_no_duplicate_exact_endpoint_scopes() {
        let mut seen = std::collections::HashSet::new();
        for ov in ENDPOINT_OVERRIDES {
            if let EndpointScope::Exact { method, path } = &ov.scope {
                let key = (format!("{method:?}"), *path);
                assert!(
                    seen.insert(key.clone()),
                    "duplicate exact override: {key:?}"
                );
            }
        }
    }

    #[test]
    fn scan_flags_unregistered_conditional_field() {
        let spec = ApiSpec {
            tags: vec![],
            all_types: vec![dto(
                "FooDto",
                vec![opt_str_field(
                    "bar_id",
                    "barId",
                    Some("The bar id. If no bar exists, this will not be set."),
                )],
            )],
        };
        let warnings = scan_conditional_fields(&spec);
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].type_name, "FooDto");
        assert_eq!(warnings[0].field_name, "bar_id");
        assert_eq!(warnings[0].matched_phrase, "will not be set");
    }

    #[test]
    fn scan_silences_registered_conditional_field() {
        let spec = ApiSpec {
            tags: vec![],
            all_types: vec![dto(
                "ProvenanceEventDto",
                vec![opt_str_field(
                    "connector_id",
                    "connectorId",
                    Some(
                        "The id of the connector. If the component is not managed by \
                         a connector, this will not be set.",
                    ),
                )],
            )],
        };
        let warnings = scan_conditional_fields(&spec);
        assert!(
            warnings.is_empty(),
            "expected registered override to silence warning, got: {warnings:?}"
        );
    }

    #[test]
    fn scan_ignores_fields_without_conditional_wording() {
        let spec = ApiSpec {
            tags: vec![],
            all_types: vec![dto(
                "FooDto",
                vec![opt_str_field("bar_id", "barId", Some("The id of the bar."))],
            )],
        };
        let warnings = scan_conditional_fields(&spec);
        assert!(warnings.is_empty());
    }

    #[test]
    fn scan_new_conditional_fields_flags_only_added_fields() {
        use crate::diff::{EndpointDiff, TypeChanges, TypesDiff, VersionDiff};

        let from_spec = ApiSpec {
            tags: vec![],
            all_types: vec![dto("FooDto", vec![])],
        };
        let to_spec = ApiSpec {
            tags: vec![],
            all_types: vec![dto(
                "FooDto",
                vec![
                    opt_str_field("old_field", "oldField", Some("never set in old flows")),
                    opt_str_field(
                        "new_field",
                        "newField",
                        Some("If the thing is missing, this will not be set."),
                    ),
                ],
            )],
        };

        let all_specs = vec![
            ("2.8.0".to_string(), from_spec),
            ("2.9.0".to_string(), to_spec),
        ];
        let diffs = vec![VersionDiff {
            from: "2.8.0".to_string(),
            to: "2.9.0".to_string(),
            endpoints: EndpointDiff {
                added: vec![],
                removed: vec![],
                changed: vec![],
            },
            types: TypesDiff {
                added: vec![],
                removed: vec![],
                changed: vec![TypeChanges {
                    name: "FooDto".to_string(),
                    added_fields: vec!["new_field".to_string()],
                    removed_fields: vec![],
                    changed_fields: vec![],
                    added_variants: vec![],
                    removed_variants: vec![],
                }],
            },
        }];

        let warnings = scan_new_conditional_fields(&all_specs, &diffs);
        // `old_field` has conditional wording but is not in added_fields → not flagged.
        // `new_field` has conditional wording and IS added → flagged.
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].field_name, "new_field");
    }

    #[test]
    fn scan_new_conditional_fields_on_real_specs_has_no_unregistered_warnings() {
        let specs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("specs");

        let mut versions: Vec<(String, ApiSpec)> = std::fs::read_dir(&specs_dir)
            .expect("read specs dir")
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
            .filter_map(|e| {
                let version = e.file_name().into_string().ok()?;
                let spec_path = e.path().join("nifi-api.json");
                if !spec_path.exists() {
                    return None;
                }
                let spec = crate::parser::load(spec_path.to_str()?);
                Some((version, spec))
            })
            .collect();
        versions.sort_by(|a, b| {
            let av = semver::Version::parse(&a.0).expect("valid semver");
            let bv = semver::Version::parse(&b.0).expect("valid semver");
            av.cmp(&bv)
        });

        let diffs: Vec<_> = versions
            .windows(2)
            .map(|pair| crate::diff::compute_diff(&pair[0].1, &pair[1].1, &pair[0].0, &pair[1].0))
            .collect();

        let warnings = scan_new_conditional_fields(&versions, &diffs);

        let rendered: Vec<String> = warnings
            .iter()
            .map(|w| {
                format!(
                    "{}.{} matched \"{}\" in: {}",
                    w.type_name, w.field_name, w.matched_phrase, w.description
                )
            })
            .collect();

        assert!(
            rendered.is_empty(),
            "scan_new_conditional_fields found {} unregistered newly-added fields — add \
             an entry to FIELD_PRESENCE_OVERRIDES for each (or verify the stock test flow \
             actually populates them):\n{}",
            rendered.len(),
            rendered.join("\n")
        );
    }
}
