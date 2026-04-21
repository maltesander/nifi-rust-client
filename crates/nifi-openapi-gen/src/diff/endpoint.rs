//! Endpoint-level diff (params, request body, response).

use crate::parser::{ApiSpec, Endpoint, QueryParam, QueryParamType};
use std::collections::HashMap;

use super::{
    AddedParam, ContractAspect, ContractChange, EndpointChanges, EndpointDiff, EndpointSummary,
    ParamChange,
};

/// Collects all endpoints from a spec as (method, path) → (Endpoint ref, tag name).
fn endpoint_map(spec: &ApiSpec) -> HashMap<(String, String), (&Endpoint, String)> {
    let mut map = HashMap::new();
    for tag in &spec.tags {
        let tag_name = tag.tag.clone();
        for ep in &tag.endpoints {
            let key = (ep.method.as_str().to_string(), ep.path.clone());
            map.insert(key, (ep, tag_name.clone()));
        }
    }
    map
}

pub(super) fn diff_endpoints(from: &ApiSpec, to: &ApiSpec) -> EndpointDiff {
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
