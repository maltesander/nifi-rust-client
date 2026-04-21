//! OpenAPI spec diff engine.
//!
//! Computes per-endpoint, per-type, per-field diffs between two
//! [`ApiSpec`]s. The entry point is [`compute_diff`]; the submodules
//! are implementation details.
//!
//! - `endpoint` — endpoint-level diffing (paths, params, request
//!   bodies, responses).
//! - `types` — type-level diffing (struct fields, enum variants).
//! - `index` — transitive-type-reachability walk used to associate
//!   types with the tag (API group) that references them.
//! - `report` — summary / is-breaking / semver-bump methods on
//!   `VersionDiff`.

mod endpoint;
mod index;
mod report;
mod types;

use crate::parser::{ApiSpec, HttpMethod};
#[cfg(test)]
use crate::parser::{Endpoint, Field, FieldType, QueryParam, QueryParamType, TypeDef};
use serde::Serialize;

// ─── Public types ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct VersionDiff {
    pub from: String,
    pub to: String,
    pub endpoints: EndpointDiff,
    pub types: TypesDiff,
}

/// Suggested semantic version bump level for this diff.
#[derive(Serialize, Debug, PartialEq)]
pub enum SemverBump {
    /// Backwards-incompatible: removed endpoints/types/fields, required param added, type changed.
    Major,
    /// Backwards-compatible additions: new endpoints, optional fields, new types.
    Minor,
    /// No additions or removals: relaxed optionality, no net API changes.
    Patch,
}

#[derive(Serialize)]
pub struct EndpointDiff {
    pub added: Vec<EndpointSummary>,
    pub removed: Vec<EndpointSummary>,
    pub changed: Vec<EndpointChanges>,
}

#[derive(Serialize)]
pub struct EndpointSummary {
    pub method: HttpMethod,
    pub path: String,
    pub tag: String,
    pub doc: Option<String>,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum ContractAspect {
    RequestType,
    ResponseType,
    BodyKind,
}

#[derive(Serialize)]
pub struct ContractChange {
    pub aspect: ContractAspect,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Serialize)]
pub struct AddedParam {
    pub name: String,
    pub required: bool,
}

#[derive(Serialize)]
pub struct EndpointChanges {
    pub method: HttpMethod,
    pub path: String,
    pub tag: String,
    pub added_params: Vec<AddedParam>,
    pub removed_params: Vec<String>,
    pub changed_params: Vec<ParamChange>,
    pub added_path_params: Vec<String>,
    pub removed_path_params: Vec<String>,
    pub contract_changes: Vec<ContractChange>,
}

#[derive(Serialize)]
pub struct ParamChange {
    pub name: String,
    pub added_enum_values: Vec<String>,
    pub removed_enum_values: Vec<String>,
    pub type_changed: Option<(String, String)>,
}

#[derive(Serialize)]
pub struct TypesDiff {
    pub added: Vec<TypeEntry>,
    pub removed: Vec<TypeEntry>,
    pub changed: Vec<TypeChanges>,
}

/// A type name with the API tag(s) it belongs to.
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TypeEntry {
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
pub struct TypeChanges {
    pub name: String,
    pub added_fields: Vec<String>,
    pub removed_fields: Vec<String>,
    pub changed_fields: Vec<FieldChange>,
    pub added_variants: Vec<String>,   // for StringEnum types
    pub removed_variants: Vec<String>, // for StringEnum types
}

// NOTE: `CompositionChanged` (oneOf/anyOf/allOf appearing on a field) is
// intentionally NOT detected here. The parser's strict `None =>` arm in
// `parse_field_type` panics on any schema without `type` or `$ref`, which
// includes composition schemas. A future NiFi spec that introduces
// composition will surface as a parse-time panic pointing at the exact
// JSON location, not as a diff entry — that's the desired Phase 1 behavior.
#[derive(Serialize, Debug, PartialEq)]
pub enum FieldChangeKind {
    BecameOptional,
    BecameRequired,
    TypeChanged {
        from: String,
        to: String,
    },
    InlineEnumChanged {
        added: Vec<String>,
        removed: Vec<String>,
    },
    /// Field was newly marked deprecated, or had its deprecation removed.
    DeprecationChanged {
        now_deprecated: bool,
    },
    /// A `string`-typed field narrowed to an inline enum with these variants.
    /// Detected as a specific case of `TypeChanged` at parse time.
    NarrowedToEnum {
        variants: Vec<String>,
    },
}

#[derive(Serialize)]
pub struct FieldChange {
    pub name: String,
    pub kind: FieldChangeKind,
}

pub fn compute_diff(from: &ApiSpec, to: &ApiSpec, from_ver: &str, to_ver: &str) -> VersionDiff {
    VersionDiff {
        from: from_ver.to_string(),
        to: to_ver.to_string(),
        endpoints: endpoint::diff_endpoints(from, to),
        types: types::diff_types(from, to),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{PathParam, TagGroup, TypeKind};

    // ─── Helpers ─────────────────────────────────────────────────────────────

    fn make_endpoint(method: HttpMethod, path: &str) -> Endpoint {
        Endpoint {
            method,
            path: path.to_string(),
            fn_name: path.replace('/', "_"),
            raw_operation_id: String::new(),
            doc: None,
            description: None,
            path_params: vec![],
            request_type: None,
            body_kind: None,
            body_doc: None,
            multipart_fields: Vec::new(),
            response_type: None,
            response_inner: None,
            response_field: None,
            response_kind: crate::content_type::ResponseBodyKind::Empty,
            query_params: vec![],
            header_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        }
    }

    fn make_endpoint_with_param(method: HttpMethod, path: &str, param: QueryParam) -> Endpoint {
        let mut ep = make_endpoint(method, path);
        ep.query_params.push(param);
        ep
    }

    fn make_endpoint_with_path_param(method: HttpMethod, path: &str, param_name: &str) -> Endpoint {
        let mut ep = make_endpoint(method, path);
        ep.path_params.push(PathParam {
            name: param_name.to_string(),
            doc: None,
        });
        ep
    }

    fn make_str_param(name: &str) -> QueryParam {
        QueryParam {
            name: name.to_string(),
            rust_name: name.to_string(),
            ty: QueryParamType::Str,
            required: false,
            doc: None,
            enum_type_name: None,
        }
    }

    fn make_enum_param(name: &str, values: &[&str]) -> QueryParam {
        QueryParam {
            name: name.to_string(),
            rust_name: name.to_string(),
            ty: QueryParamType::Enum(values.iter().map(|s| s.to_string()).collect()),
            required: false,
            doc: None,
            enum_type_name: None,
        }
    }

    fn make_tag(tag: &str, endpoints: Vec<Endpoint>) -> TagGroup {
        TagGroup {
            tag: tag.to_string(),
            struct_name: format!("{tag}Api"),
            module_name: tag.to_lowercase(),
            accessor_fn: format!("{}_api", tag.to_lowercase()),
            types: vec![],
            endpoints,
        }
    }

    fn make_spec(tags: Vec<TagGroup>, types: Vec<TypeDef>) -> ApiSpec {
        ApiSpec {
            tags,
            all_types: types,
        }
    }

    fn make_dto(name: &str, fields: Vec<Field>) -> TypeDef {
        TypeDef {
            name: name.to_string(),
            kind: TypeKind::Dto,
            fields,
            doc: None,
        }
    }

    fn make_field(name: &str, ty: FieldType) -> Field {
        Field {
            rust_name: name.to_string(),
            serde_name: name.to_string(),
            ty,
            doc: None,
            read_only: false,
            deprecated: false,
        }
    }

    fn make_field_deprecated(name: &str, ty: FieldType, deprecated: bool) -> Field {
        Field {
            rust_name: name.to_string(),
            serde_name: name.to_string(),
            ty,
            doc: None,
            read_only: false,
            deprecated,
        }
    }

    fn make_endpoint_with_request_type(method: HttpMethod, path: &str, req_type: &str) -> Endpoint {
        use crate::parser::RequestBodyKind;
        let mut ep = make_endpoint(method, path);
        ep.request_type = Some(req_type.to_string());
        ep.body_kind = Some(RequestBodyKind::Json);
        ep
    }

    fn make_endpoint_with_response_type(
        method: HttpMethod,
        path: &str,
        resp_type: &str,
    ) -> Endpoint {
        let mut ep = make_endpoint(method, path);
        ep.response_type = Some(resp_type.to_string());
        ep.response_kind = crate::content_type::ResponseBodyKind::Json {
            schema_ref: resp_type.to_string(),
        };
        ep
    }

    // ─── Tests ───────────────────────────────────────────────────────────────

    #[test]
    fn test_added_endpoint() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint(HttpMethod::Get, "/flow/about")],
            )],
            vec![],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![
                    make_endpoint(HttpMethod::Get, "/flow/about"),
                    make_endpoint(HttpMethod::Get, "/flow/metrics"),
                ],
            )],
            vec![],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.endpoints.added.len(), 1);
        assert_eq!(diff.endpoints.added[0].path, "/flow/metrics");
        assert_eq!(diff.endpoints.removed.len(), 0);
    }

    #[test]
    fn test_removed_endpoint() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![
                    make_endpoint(HttpMethod::Get, "/flow/about"),
                    make_endpoint(HttpMethod::Get, "/flow/old"),
                ],
            )],
            vec![],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint(HttpMethod::Get, "/flow/about")],
            )],
            vec![],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.endpoints.removed.len(), 1);
        assert_eq!(diff.endpoints.removed[0].path, "/flow/old");
        assert_eq!(diff.endpoints.added.len(), 0);
    }

    #[test]
    fn test_added_query_param() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint(HttpMethod::Get, "/flow/about")],
            )],
            vec![],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint_with_param(
                    HttpMethod::Get,
                    "/flow/about",
                    make_str_param("verbose"),
                )],
            )],
            vec![],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.endpoints.changed.len(), 1);
        let change = &diff.endpoints.changed[0];
        assert_eq!(change.added_params.len(), 1);
        assert_eq!(change.added_params[0].name, "verbose");
        assert!(!change.added_params[0].required);
        assert!(change.removed_params.is_empty());
    }

    #[test]
    fn test_added_required_query_param() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint(HttpMethod::Get, "/flow/about")],
            )],
            vec![],
        );
        let mut required_param = make_str_param("clusterId");
        required_param.required = true;
        let mut ep = make_endpoint(HttpMethod::Get, "/flow/about");
        ep.query_params.push(required_param);
        let to = make_spec(vec![make_tag("Flow", vec![ep])], vec![]);
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.endpoints.changed.len(), 1);
        let ec = &diff.endpoints.changed[0];
        assert_eq!(ec.added_params.len(), 1);
        assert_eq!(ec.added_params[0].name, "clusterId");
        assert!(ec.added_params[0].required);
    }

    #[test]
    fn test_query_param_type_changed_str_to_enum() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint_with_param(
                    HttpMethod::Get,
                    "/flow/about",
                    make_str_param("strategy"),
                )],
            )],
            vec![],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint_with_param(
                    HttpMethod::Get,
                    "/flow/about",
                    make_enum_param("strategy", &["A", "B"]),
                )],
            )],
            vec![],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.endpoints.changed.len(), 1);
        let ec = &diff.endpoints.changed[0];
        assert_eq!(ec.changed_params.len(), 1);
        assert!(ec.changed_params[0].type_changed.is_some());
        let (from_t, to_t) = ec.changed_params[0].type_changed.as_ref().unwrap();
        assert_eq!(from_t, "string");
        assert_eq!(to_t, "enum");
    }

    #[test]
    fn test_enum_value_added() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint_with_param(
                    HttpMethod::Get,
                    "/flow/about",
                    make_enum_param("strategy", &["A", "B"]),
                )],
            )],
            vec![],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint_with_param(
                    HttpMethod::Get,
                    "/flow/about",
                    make_enum_param("strategy", &["A", "B", "C"]),
                )],
            )],
            vec![],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.endpoints.changed.len(), 1);
        let change = &diff.endpoints.changed[0];
        assert_eq!(change.changed_params.len(), 1);
        assert_eq!(change.changed_params[0].added_enum_values, vec!["C"]);
    }

    #[test]
    fn test_summary_with_removed_enum_values() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint_with_param(
                    HttpMethod::Get,
                    "/flow/about",
                    make_enum_param("strategy", &["A", "B", "C"]),
                )],
            )],
            vec![],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint_with_param(
                    HttpMethod::Get,
                    "/flow/about",
                    make_enum_param("strategy", &["A", "B"]),
                )],
            )],
            vec![],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        let summary = diff.summary();
        assert!(summary.contains("-1 enum values"), "got: {summary}");
    }

    #[test]
    fn test_added_type() {
        let from = make_spec(vec![], vec![make_dto("AboutDTO", vec![])]);
        let to = make_spec(
            vec![],
            vec![make_dto("AboutDTO", vec![]), make_dto("MetricsDTO", vec![])],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.types.added.len(), 1);
        assert_eq!(diff.types.added[0].name, "MetricsDTO");
        assert!(diff.types.removed.is_empty());
    }

    #[test]
    fn test_added_field() {
        let from = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![make_field("version", FieldType::Str)],
            )],
        );
        let to = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![
                    make_field("version", FieldType::Str),
                    make_field("build_tag", FieldType::Opt(Box::new(FieldType::Str))),
                ],
            )],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.types.changed.len(), 1);
        assert_eq!(diff.types.changed[0].added_fields, vec!["build_tag"]);
    }

    #[test]
    fn test_field_became_optional() {
        let from = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![make_field("version", FieldType::Str)],
            )],
        );
        let to = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![make_field(
                    "version",
                    FieldType::Opt(Box::new(FieldType::Str)),
                )],
            )],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.types.changed.len(), 1);
        let tc = &diff.types.changed[0];
        assert_eq!(tc.changed_fields.len(), 1);
        assert_eq!(tc.changed_fields[0].name, "version");
        assert_eq!(tc.changed_fields[0].kind, FieldChangeKind::BecameOptional);
    }

    #[test]
    fn test_field_newly_deprecated() {
        let from = make_spec(
            vec![],
            vec![make_dto(
                "FooDto",
                vec![make_field_deprecated("foo", FieldType::Str, false)],
            )],
        );
        let to = make_spec(
            vec![],
            vec![make_dto(
                "FooDto",
                vec![make_field_deprecated("foo", FieldType::Str, true)],
            )],
        );
        let diff = compute_diff(&from, &to, "1.0.0", "2.0.0");
        assert_eq!(diff.types.changed.len(), 1);
        let tc = &diff.types.changed[0];
        assert_eq!(tc.name, "FooDto");
        assert!(tc.changed_fields.iter().any(|fc| matches!(
            fc.kind,
            FieldChangeKind::DeprecationChanged {
                now_deprecated: true
            }
        )));
        // Deprecation alone must not be treated as a breaking change.
        assert!(!diff.is_breaking());
    }

    #[test]
    fn test_field_deprecation_removed() {
        let from = make_spec(
            vec![],
            vec![make_dto(
                "FooDto",
                vec![make_field_deprecated("foo", FieldType::Str, true)],
            )],
        );
        let to = make_spec(
            vec![],
            vec![make_dto(
                "FooDto",
                vec![make_field_deprecated("foo", FieldType::Str, false)],
            )],
        );
        let diff = compute_diff(&from, &to, "1.0.0", "2.0.0");
        let tc = &diff.types.changed[0];
        assert!(tc.changed_fields.iter().any(|fc| matches!(
            fc.kind,
            FieldChangeKind::DeprecationChanged {
                now_deprecated: false
            }
        )));
    }

    #[test]
    fn test_string_narrowed_to_enum() {
        let from = make_spec(
            vec![],
            vec![make_dto(
                "FooDto",
                vec![make_field("color", FieldType::Str)],
            )],
        );
        let to = make_spec(
            vec![],
            vec![make_dto(
                "FooDto",
                vec![make_field(
                    "color",
                    FieldType::Enum(vec!["red".into(), "green".into(), "blue".into()]),
                )],
            )],
        );
        let diff = compute_diff(&from, &to, "1.0.0", "2.0.0");
        assert_eq!(diff.types.changed.len(), 1);
        let tc = &diff.types.changed[0];
        assert_eq!(tc.changed_fields.len(), 1);
        assert_eq!(tc.changed_fields[0].name, "color");
        match &tc.changed_fields[0].kind {
            FieldChangeKind::NarrowedToEnum { variants } => {
                assert_eq!(
                    variants,
                    &vec!["blue".to_string(), "green".to_string(), "red".to_string()]
                );
            }
            other => panic!("expected NarrowedToEnum, got {other:?}"),
        }
        // Narrowing the accepted value set removes previously-valid inputs.
        assert!(diff.is_breaking());
    }

    #[test]
    fn test_enum_widened_to_string_is_plain_type_change() {
        // The reverse direction (enum → string) is a widening and should
        // report as a generic TypeChanged, not NarrowedToEnum.
        let from = make_spec(
            vec![],
            vec![make_dto(
                "FooDto",
                vec![make_field(
                    "color",
                    FieldType::Enum(vec!["red".into(), "blue".into()]),
                )],
            )],
        );
        let to = make_spec(
            vec![],
            vec![make_dto(
                "FooDto",
                vec![make_field("color", FieldType::Str)],
            )],
        );
        let diff = compute_diff(&from, &to, "1.0.0", "2.0.0");
        let tc = &diff.types.changed[0];
        assert!(matches!(
            tc.changed_fields[0].kind,
            FieldChangeKind::TypeChanged { .. }
        ));
    }

    #[test]
    fn test_field_became_required() {
        let from = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![make_field(
                    "version",
                    FieldType::Opt(Box::new(FieldType::Str)),
                )],
            )],
        );
        let to = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![make_field("version", FieldType::Str)],
            )],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        let tc = &diff.types.changed[0];
        assert_eq!(tc.changed_fields[0].kind, FieldChangeKind::BecameRequired);
    }

    #[test]
    fn test_identical_specs_produce_empty_diff() {
        let spec = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint(HttpMethod::Get, "/flow/about")],
            )],
            vec![make_dto(
                "AboutDTO",
                vec![make_field("version", FieldType::Str)],
            )],
        );
        let diff = compute_diff(&spec, &spec, "2.7.2", "2.8.0");
        assert!(diff.endpoints.added.is_empty());
        assert!(diff.endpoints.removed.is_empty());
        assert!(diff.endpoints.changed.is_empty());
        assert!(diff.types.added.is_empty());
        assert!(diff.types.removed.is_empty());
        assert!(diff.types.changed.is_empty());
    }

    #[test]
    fn test_summary_with_changes() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint(HttpMethod::Get, "/flow/about")],
            )],
            vec![make_dto(
                "AboutDTO",
                vec![make_field("version", FieldType::Str)],
            )],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![
                    make_endpoint(HttpMethod::Get, "/flow/about"),
                    make_endpoint(HttpMethod::Get, "/flow/metrics"),
                ],
            )],
            vec![make_dto(
                "AboutDTO",
                vec![
                    make_field("version", FieldType::Str),
                    make_field("build_tag", FieldType::Opt(Box::new(FieldType::Str))),
                ],
            )],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        let summary = diff.summary();
        assert!(summary.contains("+1 endpoints"), "got: {summary}");
        assert!(summary.contains("+1 fields"), "got: {summary}");
    }

    #[test]
    fn test_summary_no_changes() {
        let spec = make_spec(vec![], vec![]);
        let diff = compute_diff(&spec, &spec, "2.7.2", "2.8.0");
        assert_eq!(diff.summary(), "no API changes vs 2.7.2");
    }

    #[test]
    fn test_string_enum_variant_added() {
        use crate::parser::TypeKind;
        let make_enum_type = |name: &str, variants: &[&str]| TypeDef {
            name: name.to_string(),
            kind: TypeKind::StringEnum(variants.iter().map(|s| s.to_string()).collect()),
            fields: vec![],
            doc: None,
        };
        let from = make_spec(vec![], vec![make_enum_type("Strategy", &["A", "B"])]);
        let to = make_spec(vec![], vec![make_enum_type("Strategy", &["A", "B", "C"])]);
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.types.changed.len(), 1);
        assert_eq!(diff.types.changed[0].added_variants, vec!["C"]);
        assert!(diff.types.changed[0].removed_variants.is_empty());
    }

    #[test]
    fn test_string_enum_variant_removed() {
        use crate::parser::TypeKind;
        let make_enum_type = |name: &str, variants: &[&str]| TypeDef {
            name: name.to_string(),
            kind: TypeKind::StringEnum(variants.iter().map(|s| s.to_string()).collect()),
            fields: vec![],
            doc: None,
        };
        let from = make_spec(vec![], vec![make_enum_type("Strategy", &["A", "B", "C"])]);
        let to = make_spec(vec![], vec![make_enum_type("Strategy", &["A", "B"])]);
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.types.changed.len(), 1);
        assert_eq!(diff.types.changed[0].removed_variants, vec!["C"]);
        assert!(diff.types.changed[0].added_variants.is_empty());
    }

    #[test]
    fn test_summary_with_removed() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![
                    make_endpoint(HttpMethod::Get, "/flow/about"),
                    make_endpoint(HttpMethod::Get, "/flow/old"),
                ],
            )],
            vec![],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint(HttpMethod::Get, "/flow/about")],
            )],
            vec![],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        let summary = diff.summary();
        assert!(summary.contains("-1 endpoints"), "got: {summary}");
    }

    #[test]
    fn test_endpoint_changes_includes_tag() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint(HttpMethod::Get, "/flow/about")],
            )],
            vec![],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint_with_param(
                    HttpMethod::Get,
                    "/flow/about",
                    make_str_param("verbose"),
                )],
            )],
            vec![],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.endpoints.changed[0].tag, "Flow");
    }

    #[test]
    fn test_path_param_added() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint(HttpMethod::Get, "/flow/about")],
            )],
            vec![],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint_with_path_param(
                    HttpMethod::Get,
                    "/flow/about",
                    "cluster_id",
                )],
            )],
            vec![],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.endpoints.changed.len(), 1);
        assert_eq!(
            diff.endpoints.changed[0].added_path_params,
            vec!["cluster_id"]
        );
        assert!(diff.endpoints.changed[0].removed_path_params.is_empty());
    }

    #[test]
    fn test_path_param_removed() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint_with_path_param(
                    HttpMethod::Get,
                    "/flow/about",
                    "cluster_id",
                )],
            )],
            vec![],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint(HttpMethod::Get, "/flow/about")],
            )],
            vec![],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.endpoints.changed.len(), 1);
        assert_eq!(
            diff.endpoints.changed[0].removed_path_params,
            vec!["cluster_id"]
        );
        assert!(diff.endpoints.changed[0].added_path_params.is_empty());
    }

    #[test]
    fn test_request_type_changed() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint_with_request_type(
                    HttpMethod::Post,
                    "/flow/process-groups",
                    "ProcessGroupDto",
                )],
            )],
            vec![],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint_with_request_type(
                    HttpMethod::Post,
                    "/flow/process-groups",
                    "ProcessGroupEntity",
                )],
            )],
            vec![],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.endpoints.changed.len(), 1);
        let ec = &diff.endpoints.changed[0];
        assert_eq!(ec.contract_changes.len(), 1);
        assert_eq!(ec.contract_changes[0].aspect, ContractAspect::RequestType);
        assert_eq!(
            ec.contract_changes[0].from.as_deref(),
            Some("ProcessGroupDto")
        );
        assert_eq!(
            ec.contract_changes[0].to.as_deref(),
            Some("ProcessGroupEntity")
        );
    }

    #[test]
    fn test_response_type_changed() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint_with_response_type(
                    HttpMethod::Get,
                    "/flow/about",
                    "AboutDto",
                )],
            )],
            vec![],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint_with_response_type(
                    HttpMethod::Get,
                    "/flow/about",
                    "AboutEntity",
                )],
            )],
            vec![],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.endpoints.changed.len(), 1);
        let ec = &diff.endpoints.changed[0];
        assert_eq!(ec.contract_changes.len(), 1);
        assert_eq!(ec.contract_changes[0].aspect, ContractAspect::ResponseType);
        assert_eq!(ec.contract_changes[0].from.as_deref(), Some("AboutDto"));
        assert_eq!(ec.contract_changes[0].to.as_deref(), Some("AboutEntity"));
    }

    #[test]
    fn test_field_base_type_changed() {
        let from = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![make_field("count", FieldType::I32)],
            )],
        );
        let to = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![make_field("count", FieldType::I64)],
            )],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert_eq!(diff.types.changed.len(), 1);
        let tc = &diff.types.changed[0];
        assert_eq!(tc.changed_fields.len(), 1);
        assert_eq!(tc.changed_fields[0].name, "count");
        assert!(matches!(
            &tc.changed_fields[0].kind,
            FieldChangeKind::TypeChanged { from, to }
            if from == "i32" && to == "i64"
        ));
    }

    #[test]
    fn test_field_optionality_and_type_both_changed() {
        // When both optionality and base type change, we get two FieldChanges for the same field
        let from = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![make_field("count", FieldType::I32)],
            )],
        );
        let to = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![make_field(
                    "count",
                    FieldType::Opt(Box::new(FieldType::I64)),
                )],
            )],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        let tc = &diff.types.changed[0];
        let kinds: Vec<_> = tc.changed_fields.iter().map(|f| &f.kind).collect();
        assert!(
            kinds
                .iter()
                .any(|k| matches!(k, FieldChangeKind::BecameOptional))
        );
        assert!(
            kinds
                .iter()
                .any(|k| matches!(k, FieldChangeKind::TypeChanged { .. }))
        );
    }

    #[test]
    fn test_is_breaking_endpoint_removed() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![
                    make_endpoint(HttpMethod::Get, "/flow/about"),
                    make_endpoint(HttpMethod::Get, "/flow/old"),
                ],
            )],
            vec![],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint(HttpMethod::Get, "/flow/about")],
            )],
            vec![],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert!(diff.is_breaking());
        assert_eq!(diff.semver_bump(), SemverBump::Major);
    }

    #[test]
    fn test_is_breaking_endpoint_added_only() {
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint(HttpMethod::Get, "/flow/about")],
            )],
            vec![],
        );
        let to = make_spec(
            vec![make_tag(
                "Flow",
                vec![
                    make_endpoint(HttpMethod::Get, "/flow/about"),
                    make_endpoint(HttpMethod::Get, "/flow/metrics"),
                ],
            )],
            vec![],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert!(!diff.is_breaking());
        assert_eq!(diff.semver_bump(), SemverBump::Minor);
    }

    #[test]
    fn test_is_breaking_field_became_required() {
        let from = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![make_field(
                    "version",
                    FieldType::Opt(Box::new(FieldType::Str)),
                )],
            )],
        );
        let to = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![make_field("version", FieldType::Str)],
            )],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert!(diff.is_breaking());
        assert_eq!(diff.semver_bump(), SemverBump::Major);
    }

    #[test]
    fn test_is_breaking_field_became_optional_only() {
        let from = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![make_field("version", FieldType::Str)],
            )],
        );
        let to = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![make_field(
                    "version",
                    FieldType::Opt(Box::new(FieldType::Str)),
                )],
            )],
        );
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert!(!diff.is_breaking());
        assert_eq!(diff.semver_bump(), SemverBump::Patch);
    }

    #[test]
    fn test_is_not_breaking_no_changes() {
        let spec = make_spec(vec![], vec![]);
        let diff = compute_diff(&spec, &spec, "2.7.2", "2.8.0");
        assert!(!diff.is_breaking());
        assert_eq!(diff.semver_bump(), SemverBump::Patch);
    }

    #[test]
    fn test_is_breaking_path_param_added() {
        use crate::parser::PathParam;
        let from = make_spec(
            vec![make_tag(
                "Flow",
                vec![make_endpoint(HttpMethod::Get, "/flow/about")],
            )],
            vec![],
        );
        let mut ep = make_endpoint(HttpMethod::Get, "/flow/about");
        ep.path_params.push(PathParam {
            name: "cluster_id".to_string(),
            doc: None,
        });
        let to = make_spec(vec![make_tag("Flow", vec![ep])], vec![]);
        let diff = compute_diff(&from, &to, "2.7.2", "2.8.0");
        assert!(diff.is_breaking());
        assert_eq!(diff.semver_bump(), SemverBump::Major);
    }
}
