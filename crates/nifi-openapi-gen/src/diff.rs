use crate::parser::{
    ApiSpec, Endpoint, Field, FieldType, HttpMethod, QueryParam, QueryParamType, TypeDef,
};
use serde::Serialize;
use std::collections::HashMap;

// ─── Public types ─────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct VersionDiff {
    pub from: String,
    pub to: String,
    pub endpoints: EndpointDiff,
    pub types: TypesDiff,
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

#[derive(Serialize)]
pub struct EndpointChanges {
    pub method: HttpMethod,
    pub path: String,
    pub tag: String,
    pub added_params: Vec<String>,
    pub removed_params: Vec<String>,
    pub changed_params: Vec<ParamChange>,
}

#[derive(Serialize)]
pub struct ParamChange {
    pub name: String,
    pub added_enum_values: Vec<String>,
    pub removed_enum_values: Vec<String>,
}

#[derive(Serialize)]
pub struct TypesDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub changed: Vec<TypeChanges>,
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

#[derive(Serialize, Debug, PartialEq)]
pub enum FieldChangeKind {
    BecameOptional,
    BecameRequired,
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
        endpoints: diff_endpoints(from, to),
        types: diff_types(from, to),
    }
}

impl VersionDiff {
    /// Produces a short human-readable summary for use in the versions table.
    /// Example: "+4 endpoints, -1 endpoints, +12 fields, +2 enum values"
    pub fn summary(&self) -> String {
        let added_ep = self.endpoints.added.len() as i64;
        let removed_ep = self.endpoints.removed.len() as i64;

        let added_fields: usize = self
            .types
            .changed
            .iter()
            .map(|tc| tc.added_fields.len())
            .sum();
        let removed_fields: usize = self
            .types
            .changed
            .iter()
            .map(|tc| tc.removed_fields.len())
            .sum();

        let added_enum_vals: usize = self
            .endpoints
            .changed
            .iter()
            .flat_map(|ec| ec.changed_params.iter())
            .map(|pc| pc.added_enum_values.len())
            .sum::<usize>()
            + self
                .types
                .changed
                .iter()
                .map(|tc| tc.added_variants.len())
                .sum::<usize>();

        let removed_enum_vals: usize = self
            .endpoints
            .changed
            .iter()
            .flat_map(|ec| ec.changed_params.iter())
            .map(|pc| pc.removed_enum_values.len())
            .sum::<usize>()
            + self
                .types
                .changed
                .iter()
                .map(|tc| tc.removed_variants.len())
                .sum::<usize>();

        let added_types = self.types.added.len() as i64;
        let removed_types = self.types.removed.len() as i64;

        let mut parts: Vec<String> = Vec::new();

        if added_ep != 0 || removed_ep != 0 {
            if added_ep > 0 {
                parts.push(format!("+{added_ep} endpoints"));
            }
            if removed_ep > 0 {
                parts.push(format!("-{removed_ep} endpoints"));
            }
        }
        if added_types > 0 {
            parts.push(format!("+{added_types} types"));
        }
        if removed_types > 0 {
            parts.push(format!("-{removed_types} types"));
        }
        if added_fields > 0 {
            parts.push(format!("+{added_fields} fields"));
        }
        if removed_fields > 0 {
            parts.push(format!("-{removed_fields} fields"));
        }
        if added_enum_vals > 0 {
            parts.push(format!("+{added_enum_vals} enum values"));
        }
        if removed_enum_vals > 0 {
            parts.push(format!("-{removed_enum_vals} enum values"));
        }

        if parts.is_empty() {
            format!("no API changes vs {}", self.from)
        } else {
            format!("{} vs {}", parts.join(", "), self.from)
        }
    }
}

// ─── Private helpers ──────────────────────────────────────────────────────────

fn is_optional(ft: &FieldType) -> bool {
    matches!(ft, FieldType::Opt(_))
}

/// Collects all endpoints from a spec as (method, path) → (Endpoint ref, tag name).
fn endpoint_map(spec: &ApiSpec) -> HashMap<(String, String), (&Endpoint, String)> {
    let mut map = HashMap::new();
    for tag in &spec.tags {
        let tag_name = tag.tag.clone();
        for ep in &tag.root_endpoints {
            let key = (ep.method.as_str().to_string(), ep.path.clone());
            map.insert(key, (ep, tag_name.clone()));
        }
        for sg in &tag.sub_groups {
            for ep in &sg.endpoints {
                let key = (ep.method.as_str().to_string(), ep.path.clone());
                map.insert(key, (ep, tag_name.clone()));
            }
        }
    }
    map
}

/// Collects all types from a spec as name → TypeDef ref.
fn type_map(spec: &ApiSpec) -> HashMap<&str, &TypeDef> {
    spec.all_types
        .iter()
        .map(|t| (t.name.as_str(), t))
        .collect()
}

fn diff_endpoints(from: &ApiSpec, to: &ApiSpec) -> EndpointDiff {
    let from_map = endpoint_map(from);
    let to_map = endpoint_map(to);

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut changed = Vec::new();

    for (key, (ep, tag)) in &to_map {
        if !from_map.contains_key(key) {
            added.push(EndpointSummary {
                method: ep.method.clone(),
                path: ep.path.clone(),
                tag: tag.clone(),
                doc: ep.doc.clone(),
            });
        } else {
            let (from_ep, _) = &from_map[key];
            let ec = diff_endpoint_params(ep, from_ep);
            if ec.added_params.is_empty()
                && ec.removed_params.is_empty()
                && ec.changed_params.is_empty()
            {
                continue;
            }
            changed.push(EndpointChanges {
                method: ep.method.clone(),
                path: ep.path.clone(),
                tag: tag.clone(),
                added_params: ec.added_params,
                removed_params: ec.removed_params,
                changed_params: ec.changed_params,
            });
        }
    }

    for (key, (ep, tag)) in &from_map {
        if !to_map.contains_key(key) {
            removed.push(EndpointSummary {
                method: ep.method.clone(),
                path: ep.path.clone(),
                tag: tag.clone(),
                doc: ep.doc.clone(),
            });
        }
    }

    // Sort for deterministic output
    added.sort_by(|a, b| (&a.path, a.method.as_str()).cmp(&(&b.path, b.method.as_str())));
    removed.sort_by(|a, b| (&a.path, a.method.as_str()).cmp(&(&b.path, b.method.as_str())));
    changed.sort_by(|a, b| (&a.path, a.method.as_str()).cmp(&(&b.path, b.method.as_str())));

    EndpointDiff {
        added,
        removed,
        changed,
    }
}

struct RawEndpointChanges {
    added_params: Vec<String>,
    removed_params: Vec<String>,
    changed_params: Vec<ParamChange>,
}

fn diff_endpoint_params(to_ep: &Endpoint, from_ep: &Endpoint) -> RawEndpointChanges {
    let from_params: HashMap<&str, &QueryParam> = from_ep
        .query_params
        .iter()
        .map(|p| (p.name.as_str(), p))
        .collect();
    let to_params: HashMap<&str, &QueryParam> = to_ep
        .query_params
        .iter()
        .map(|p| (p.name.as_str(), p))
        .collect();

    let mut added_params = Vec::new();
    let mut removed_params = Vec::new();
    let mut changed_params = Vec::new();

    for (name, to_p) in &to_params {
        if !from_params.contains_key(name) {
            added_params.push(name.to_string());
        } else {
            let from_p = from_params[name];
            if let Some(pc) = diff_param_enums(name, from_p, to_p) {
                changed_params.push(pc);
            }
        }
    }

    for name in from_params.keys() {
        if !to_params.contains_key(name) {
            removed_params.push(name.to_string());
        }
    }

    added_params.sort();
    removed_params.sort();
    changed_params.sort_by(|a, b| a.name.cmp(&b.name));

    RawEndpointChanges {
        added_params,
        removed_params,
        changed_params,
    }
}

fn diff_param_enums(name: &str, from_p: &QueryParam, to_p: &QueryParam) -> Option<ParamChange> {
    let from_vals: Vec<String> = match &from_p.ty {
        QueryParamType::Enum(vals) => vals.clone(),
        _ => vec![],
    };
    let to_vals: Vec<String> = match &to_p.ty {
        QueryParamType::Enum(vals) => vals.clone(),
        _ => vec![],
    };

    let from_set: std::collections::HashSet<&str> = from_vals.iter().map(String::as_str).collect();
    let to_set: std::collections::HashSet<&str> = to_vals.iter().map(String::as_str).collect();

    let added: Vec<String> = to_set
        .difference(&from_set)
        .map(|s| s.to_string())
        .collect();
    let removed: Vec<String> = from_set
        .difference(&to_set)
        .map(|s| s.to_string())
        .collect();

    if added.is_empty() && removed.is_empty() {
        return None;
    }

    let mut added = added;
    let mut removed = removed;
    added.sort();
    removed.sort();

    Some(ParamChange {
        name: name.to_string(),
        added_enum_values: added,
        removed_enum_values: removed,
    })
}

fn diff_types(from: &ApiSpec, to: &ApiSpec) -> TypesDiff {
    let from_map = type_map(from);
    let to_map = type_map(to);

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut changed = Vec::new();

    for (name, to_type) in &to_map {
        if !from_map.contains_key(name) {
            added.push(name.to_string());
        } else {
            let from_type = from_map[name];
            let tc = diff_type_fields(name, from_type, to_type);
            if !tc.added_fields.is_empty()
                || !tc.removed_fields.is_empty()
                || !tc.changed_fields.is_empty()
                || !tc.added_variants.is_empty()
                || !tc.removed_variants.is_empty()
            {
                changed.push(tc);
            }
        }
    }

    for name in from_map.keys() {
        if !to_map.contains_key(name) {
            removed.push(name.to_string());
        }
    }

    added.sort();
    removed.sort();
    changed.sort_by(|a, b| a.name.cmp(&b.name));

    TypesDiff {
        added,
        removed,
        changed,
    }
}

fn diff_type_fields(name: &str, from_type: &TypeDef, to_type: &TypeDef) -> TypeChanges {
    use crate::parser::TypeKind;
    use std::collections::HashSet;

    // StringEnum: compare variant lists directly (no fields to diff)
    let (added_variants, removed_variants) = match (&from_type.kind, &to_type.kind) {
        (TypeKind::StringEnum(from_vals), TypeKind::StringEnum(to_vals)) => {
            let from_set: HashSet<&str> = from_vals.iter().map(String::as_str).collect();
            let to_set: HashSet<&str> = to_vals.iter().map(String::as_str).collect();
            let mut added: Vec<String> = to_set
                .difference(&from_set)
                .map(|s| s.to_string())
                .collect();
            let mut removed: Vec<String> = from_set
                .difference(&to_set)
                .map(|s| s.to_string())
                .collect();
            added.sort();
            removed.sort();
            (added, removed)
        }
        _ => (vec![], vec![]),
    };

    // Dto / Entity: compare fields
    let from_fields: HashMap<&str, &Field> = from_type
        .fields
        .iter()
        .map(|f| (f.rust_name.as_str(), f))
        .collect();
    let to_fields: HashMap<&str, &Field> = to_type
        .fields
        .iter()
        .map(|f| (f.rust_name.as_str(), f))
        .collect();

    let mut added_fields = Vec::new();
    let mut removed_fields = Vec::new();
    let mut changed_fields = Vec::new();

    for (fname, to_f) in &to_fields {
        if !from_fields.contains_key(fname) {
            added_fields.push(fname.to_string());
        } else {
            let from_f = from_fields[fname];
            let from_opt = is_optional(&from_f.ty);
            let to_opt = is_optional(&to_f.ty);
            if from_opt != to_opt {
                changed_fields.push(FieldChange {
                    name: fname.to_string(),
                    kind: if to_opt {
                        FieldChangeKind::BecameOptional
                    } else {
                        FieldChangeKind::BecameRequired
                    },
                });
            }
        }
    }

    for fname in from_fields.keys() {
        if !to_fields.contains_key(fname) {
            removed_fields.push(fname.to_string());
        }
    }

    added_fields.sort();
    removed_fields.sort();
    changed_fields.sort_by(|a, b| a.name.cmp(&b.name));

    TypeChanges {
        name: name.to_string(),
        added_fields,
        removed_fields,
        changed_fields,
        added_variants,
        removed_variants,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{TagGroup, TypeKind};

    // ─── Helpers ─────────────────────────────────────────────────────────────

    fn make_endpoint(method: HttpMethod, path: &str) -> Endpoint {
        Endpoint {
            method,
            path: path.to_string(),
            fn_name: path.replace('/', "_"),
            doc: None,
            description: None,
            path_params: vec![],
            request_type: None,
            body_kind: None,
            body_doc: None,
            response_type: None,
            response_inner: None,
            response_field: None,
            query_params: vec![],
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
            root_endpoints: endpoints,
            sub_groups: vec![],
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
        }
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
        assert_eq!(change.added_params, vec!["verbose"]);
        assert!(change.removed_params.is_empty());
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
        assert_eq!(diff.types.added, vec!["MetricsDTO"]);
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
    fn test_field_became_required() {
        let from = make_spec(
            vec![],
            vec![make_dto(
                "AboutDTO",
                vec![make_field("version", FieldType::Opt(Box::new(FieldType::Str)))],
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
}
