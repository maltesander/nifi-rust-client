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

    /// Returns `true` if the diff contains any backwards-incompatible change.
    pub fn is_breaking(&self) -> bool {
        // Removed endpoints or types
        if !self.endpoints.removed.is_empty() || !self.types.removed.is_empty() {
            return true;
        }

        // Changes to existing endpoints
        for ec in &self.endpoints.changed {
            // Removed query or path params, or added path params (breaking)
            if !ec.removed_params.is_empty()
                || !ec.removed_path_params.is_empty()
                || !ec.added_path_params.is_empty()
            {
                return true;
            }
            // Contract changes (request/response type, body kind)
            if !ec.contract_changes.is_empty() {
                return true;
            }
            // Required query param added
            if ec.added_params.iter().any(|p| p.required) {
                return true;
            }
            // Query param enum values removed or type changed
            if ec
                .changed_params
                .iter()
                .any(|pc| !pc.removed_enum_values.is_empty() || pc.type_changed.is_some())
            {
                return true;
            }
        }

        // Changes to existing types
        for tc in &self.types.changed {
            // Removed fields or enum variants
            if !tc.removed_fields.is_empty() || !tc.removed_variants.is_empty() {
                return true;
            }
            // Fields that became required, changed type, or lost enum variants
            if tc.changed_fields.iter().any(|fc| match &fc.kind {
                FieldChangeKind::BecameRequired
                | FieldChangeKind::TypeChanged { .. }
                | FieldChangeKind::NarrowedToEnum { .. } => true,
                FieldChangeKind::InlineEnumChanged { removed, .. } => !removed.is_empty(),
                _ => false,
            }) {
                return true;
            }
        }

        false
    }

    /// Suggests a semantic version bump level for this diff.
    pub fn semver_bump(&self) -> SemverBump {
        if self.is_breaking() {
            return SemverBump::Major;
        }
        // Additive changes → Minor
        let is_additive = !self.endpoints.added.is_empty()
            || !self.types.added.is_empty()
            || self
                .types
                .changed
                .iter()
                .any(|tc| !tc.added_fields.is_empty() || !tc.added_variants.is_empty())
            || self
                .endpoints
                .changed
                .iter()
                .any(|ec| ec.added_params.iter().any(|p| !p.required));
        if is_additive {
            SemverBump::Minor
        } else {
            SemverBump::Patch
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
            let (added_path_params, removed_path_params) = diff_path_params(ep, from_ep);
            let contract_changes = diff_contract(ep, from_ep);
            if ec.added_params.is_empty()
                && ec.removed_params.is_empty()
                && ec.changed_params.is_empty()
                && added_path_params.is_empty()
                && removed_path_params.is_empty()
                && contract_changes.is_empty()
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
                added_path_params,
                removed_path_params,
                contract_changes,
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
    added_params: Vec<AddedParam>,
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
            added_params.push(AddedParam {
                name: name.to_string(),
                required: to_p.required,
            });
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

    added_params.sort_by(|a, b| a.name.cmp(&b.name));
    removed_params.sort();
    changed_params.sort_by(|a, b| a.name.cmp(&b.name));

    RawEndpointChanges {
        added_params,
        removed_params,
        changed_params,
    }
}

fn query_param_type_name(ty: &QueryParamType) -> &'static str {
    match ty {
        QueryParamType::Str => "string",
        QueryParamType::Bool => "boolean",
        QueryParamType::I32 => "integer",
        QueryParamType::I64 => "int64",
        QueryParamType::F64 => "number",
        QueryParamType::Enum(_) => "enum",
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

    // Detect type changes (e.g. Str → Enum) by comparing discriminants
    let type_changed = if std::mem::discriminant(&from_p.ty) != std::mem::discriminant(&to_p.ty) {
        Some((
            query_param_type_name(&from_p.ty).to_string(),
            query_param_type_name(&to_p.ty).to_string(),
        ))
    } else {
        None
    };

    if added.is_empty() && removed.is_empty() && type_changed.is_none() {
        return None;
    }

    Some(ParamChange {
        name: name.to_string(),
        added_enum_values: added,
        removed_enum_values: removed,
        type_changed,
    })
}

fn diff_path_params(to_ep: &Endpoint, from_ep: &Endpoint) -> (Vec<String>, Vec<String>) {
    use std::collections::HashSet;
    let from_names: HashSet<&str> = from_ep
        .path_params
        .iter()
        .map(|p| p.name.as_str())
        .collect();
    let to_names: HashSet<&str> = to_ep.path_params.iter().map(|p| p.name.as_str()).collect();
    let mut added: Vec<String> = to_names
        .difference(&from_names)
        .map(|s| s.to_string())
        .collect();
    let mut removed: Vec<String> = from_names
        .difference(&to_names)
        .map(|s| s.to_string())
        .collect();
    added.sort();
    removed.sort();
    (added, removed)
}

fn diff_contract(to_ep: &Endpoint, from_ep: &Endpoint) -> Vec<ContractChange> {
    use crate::parser::RequestBodyKind;
    let mut changes = Vec::new();

    if from_ep.request_type != to_ep.request_type {
        changes.push(ContractChange {
            aspect: ContractAspect::RequestType,
            from: from_ep.request_type.clone(),
            to: to_ep.request_type.clone(),
        });
    }

    if from_ep.response_type != to_ep.response_type {
        changes.push(ContractChange {
            aspect: ContractAspect::ResponseType,
            from: from_ep.response_type.clone(),
            to: to_ep.response_type.clone(),
        });
    }

    let from_body = from_ep.body_kind.as_ref().map(RequestBodyKind::as_str);
    let to_body = to_ep.body_kind.as_ref().map(RequestBodyKind::as_str);
    if from_body != to_body {
        changes.push(ContractChange {
            aspect: ContractAspect::BodyKind,
            from: from_body.map(str::to_string),
            to: to_body.map(str::to_string),
        });
    }

    changes
}

/// Build a mapping from type name to the sorted, deduplicated list of tags that use it.
///
/// Walks endpoint request/response types and their fields transitively to
/// associate types with their API tags.
fn type_tag_index(spec: &ApiSpec) -> HashMap<&str, Vec<String>> {
    let type_map: HashMap<&str, &crate::parser::TypeDef> = spec
        .all_types
        .iter()
        .map(|t| (t.name.as_str(), t))
        .collect();
    let mut index: HashMap<&str, Vec<String>> = HashMap::new();

    for tag_group in &spec.tags {
        // Collect direct endpoint types
        let mut frontier: Vec<&str> = Vec::new();
        let all_eps = tag_group.root_endpoints.iter().chain(
            tag_group
                .sub_groups
                .iter()
                .flat_map(|sg| sg.endpoints.iter()),
        );
        for ep in all_eps {
            if let Some(t) = &ep.request_type {
                frontier.push(t.as_str());
            }
            if let Some(t) = &ep.response_type {
                frontier.push(t.as_str());
            }
            if let Some(t) = &ep.response_inner {
                frontier.push(t.as_str());
            }
        }

        // Walk fields transitively
        let mut visited: std::collections::HashSet<&str> = std::collections::HashSet::new();
        while let Some(name) = frontier.pop() {
            if !visited.insert(name) {
                continue;
            }
            if let Some(td) = type_map.get(name) {
                for f in &td.fields {
                    if let Some(ref_name) = field_type_ref_name(&f.ty) {
                        frontier.push(ref_name);
                    }
                }
            }
        }

        for name in visited {
            index.entry(name).or_default().push(tag_group.tag.clone());
        }
    }
    // Deduplicate and sort
    for tags in index.values_mut() {
        tags.sort();
        tags.dedup();
    }
    index
}

/// Extract the type name from a field type if it's a named reference.
fn field_type_ref_name(ty: &crate::parser::FieldType) -> Option<&str> {
    use crate::parser::FieldType;
    match ty {
        FieldType::Ref(name) => Some(name.as_str()),
        FieldType::List(inner) | FieldType::Opt(inner) | FieldType::Map(inner) => {
            field_type_ref_name(inner)
        }
        _ => None,
    }
}

fn diff_types(from: &ApiSpec, to: &ApiSpec) -> TypesDiff {
    let from_map = type_map(from);
    let to_map = type_map(to);
    let to_tags = type_tag_index(to);
    let from_tags = type_tag_index(from);

    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut changed = Vec::new();

    for (name, to_type) in &to_map {
        if !from_map.contains_key(name) {
            added.push(TypeEntry {
                name: name.to_string(),
                tags: to_tags.get(name).cloned().unwrap_or_default(),
            });
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
            removed.push(TypeEntry {
                name: name.to_string(),
                tags: from_tags.get(name).cloned().unwrap_or_default(),
            });
        }
    }

    added.sort_by(|a, b| a.name.cmp(&b.name));
    removed.sort_by(|a, b| a.name.cmp(&b.name));
    changed.sort_by(|a, b| a.name.cmp(&b.name));

    TypesDiff {
        added,
        removed,
        changed,
    }
}

fn unwrap_opt(ty: &FieldType) -> &FieldType {
    match ty {
        FieldType::Opt(inner) => inner.as_ref(),
        other => other,
    }
}

fn field_type_display(ty: &FieldType) -> String {
    match ty {
        FieldType::Str => "String".to_string(),
        FieldType::Bool => "bool".to_string(),
        FieldType::I32 => "i32".to_string(),
        FieldType::I64 => "i64".to_string(),
        FieldType::F64 => "f64".to_string(),
        FieldType::Opt(inner) => format!("Option<{}>", field_type_display(inner)),
        FieldType::List(inner) => format!("Vec<{}>", field_type_display(inner)),
        FieldType::Ref(name) => name.clone(),
        FieldType::Map(inner) => format!("HashMap<String, {}>", field_type_display(inner)),
        FieldType::Enum(variants) => {
            let mut sorted = variants.clone();
            sorted.sort();
            format!("Enum({})", sorted.join(" | "))
        }
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
            let from_base = unwrap_opt(&from_f.ty);
            let to_base = unwrap_opt(&to_f.ty);

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
            if from_f.deprecated != to_f.deprecated {
                changed_fields.push(FieldChange {
                    name: fname.to_string(),
                    kind: FieldChangeKind::DeprecationChanged {
                        now_deprecated: to_f.deprecated,
                    },
                });
            }
            // For inline enum fields, compare variant sets (order-insensitive).
            let types_differ = match (from_base, to_base) {
                (FieldType::Enum(fv), FieldType::Enum(tv)) => {
                    let mut a = fv.clone();
                    let mut b = tv.clone();
                    a.sort();
                    b.sort();
                    a != b
                }
                _ => from_base != to_base,
            };
            if types_differ {
                let kind = match (from_base, to_base) {
                    (FieldType::Enum(fv), FieldType::Enum(tv)) => {
                        let fset: std::collections::HashSet<_> = fv.iter().collect();
                        let tset: std::collections::HashSet<_> = tv.iter().collect();
                        let mut added: Vec<String> =
                            tset.difference(&fset).map(|s| s.to_string()).collect();
                        let mut removed: Vec<String> =
                            fset.difference(&tset).map(|s| s.to_string()).collect();
                        added.sort();
                        removed.sort();
                        FieldChangeKind::InlineEnumChanged { added, removed }
                    }
                    (FieldType::Str, FieldType::Enum(tv)) => {
                        let mut variants = tv.clone();
                        variants.sort();
                        FieldChangeKind::NarrowedToEnum { variants }
                    }
                    _ => FieldChangeKind::TypeChanged {
                        from: field_type_display(from_base),
                        to: field_type_display(to_base),
                    },
                };
                changed_fields.push(FieldChange {
                    name: fname.to_string(),
                    kind,
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
            response_type: None,
            response_inner: None,
            response_field: None,
            response_kind: crate::content_type::ResponseBodyKind::Empty,
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
