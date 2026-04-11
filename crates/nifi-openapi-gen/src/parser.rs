use serde_json::Value;
use std::collections::HashMap;

pub use crate::content_type::RequestBodyKind;

pub struct ApiSpec {
    pub tags: Vec<TagGroup>,
    /// Flat map of ALL schema definitions by name (for $ref resolution).
    pub all_types: Vec<TypeDef>,
}

pub struct TagGroup {
    pub tag: String,
    pub struct_name: String, // e.g. "FlowApi"
    pub module_name: String, // e.g. "flow"  — used for file/module names
    pub accessor_fn: String, // e.g. "flow_api" — used for NifiClient method name
    pub types: Vec<TypeDef>, // types used by this tag's endpoints
    pub root_endpoints: Vec<Endpoint>,
    pub sub_groups: Vec<SubGroup>,
}

#[derive(Debug, Clone)]
pub struct SubGroup {
    pub name: String,                      // path segment after {id}, e.g. "config"
    pub struct_name: String,               // e.g. "ControllerServicesConfigApi"
    pub accessor_fn: String,               // e.g. "config"
    pub primary_param: String,             // the param name baked into the sub-struct, e.g. "id"
    pub primary_param_doc: Option<String>, // description for that param, e.g. "The process group id."
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Clone)]
pub enum TypeKind {
    Dto,
    Entity { field: String, inner: String }, // field name + inner DTO name
    StringEnum(Vec<String>), // wire-value strings, e.g. ["KEEP_EXISTING", "REPLACE"]
}

#[derive(Debug, Clone)]
pub struct TypeDef {
    pub name: String,
    pub kind: TypeKind,
    pub fields: Vec<Field>,
    pub doc: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub rust_name: String,
    pub serde_name: String,
    pub ty: FieldType,
    pub doc: Option<String>,
    pub read_only: bool,
    /// Whether the OpenAPI schema marks this field as deprecated.
    pub deprecated: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    Str,
    Bool,
    I32,
    I64,
    F64,
    Opt(Box<FieldType>),
    List(Box<FieldType>),
    /// Inline string enum — emitter generates a named type `{StructName}{PropName}`
    Enum(Vec<String>),
    /// Reference to another named type
    Ref(String),
    /// Map with string keys and typed values (`additionalProperties`)
    Map(Box<FieldType>),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum QueryParamType {
    Str,
    Bool,
    I32,
    I64,
    F64,
    Enum(Vec<String>), // wire-value strings
}

#[derive(Debug, Clone)]
pub struct QueryParam {
    /// Original name from the spec (used as the URL query key), e.g. "propertyName"
    pub name: String,
    /// snake_case Rust identifier, e.g. "property_name"
    pub rust_name: String,
    pub ty: QueryParamType,
    pub required: bool,
    /// Human-readable description from the spec, if any.
    pub doc: Option<String>,
    /// Some("ParameterContextHandlingStrategy") if this param is an enum type.
    pub enum_type_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PathParam {
    /// snake_case Rust identifier (and path placeholder name).
    pub name: String,
    /// Human-readable description from the spec, if any.
    pub doc: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub method: HttpMethod,
    pub path: String,
    pub fn_name: String,
    /// Short summary from the spec (`summary` field).
    pub doc: Option<String>,
    /// Long-form description from the spec (`description` field), if present.
    pub description: Option<String>,
    pub path_params: Vec<PathParam>,
    pub request_type: Option<String>,
    pub body_kind: Option<RequestBodyKind>,
    /// Human-readable description of the request body, if any.
    pub body_doc: Option<String>,
    pub response_type: Option<String>,
    /// If response_type is an Entity, the inner DTO name.
    pub response_inner: Option<String>,
    /// If response_type is an Entity, the field name to unwrap.
    pub response_field: Option<String>,
    /// Content-type classification of the success response body.
    /// Currently informational only — emitters still consume
    /// `response_type`/`response_inner`/`response_field` for code generation.
    /// Task 3.2 will migrate emitters to consume `response_kind` directly.
    pub response_kind: crate::content_type::ResponseBodyKind,
    pub query_params: Vec<QueryParam>,
    /// 2xx responses: (status_code, description), e.g. ("206", "Partial Content...").
    pub success_responses: Vec<(String, String)>,
    /// Non-2xx responses: (status_code, description), e.g. ("400", "NiFi was unable to...").
    pub error_responses: Vec<(String, String)>,
    /// Security requirements from the spec.
    /// `None` = `security` key absent in spec.
    /// `Some([])` = no auth required (empty array in spec).
    /// `Some([...])` = list of policy strings, e.g. `["Read - /controller-services/{uuid}"]`.
    pub security: Option<Vec<String>>,
}

pub fn load(path: &str) -> ApiSpec {
    let text =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read spec at {path}: {e}"));
    let spec: Value =
        serde_json::from_str(&text).unwrap_or_else(|e| panic!("invalid JSON in {path}: {e}"));
    let schemas = spec["components"]["schemas"]
        .as_object()
        .unwrap_or_else(|| panic!("missing components.schemas"));
    let all_types = parse_all_types(schemas);
    let tags = parse_tags(&spec, schemas, &all_types);
    let mut all_types = all_types;

    // Collect StringEnum TypeDefs from query params across all tags
    for tag in &tags {
        let all_eps = tag
            .root_endpoints
            .iter()
            .chain(tag.sub_groups.iter().flat_map(|sg| sg.endpoints.iter()));
        for ep in all_eps {
            for qp in &ep.query_params {
                if let (Some(type_name), QueryParamType::Enum(variants)) =
                    (&qp.enum_type_name, &qp.ty)
                {
                    // Avoid duplicates (same enum may appear on multiple endpoints)
                    if !all_types.iter().any(|t| &t.name == type_name) {
                        all_types.push(TypeDef {
                            name: type_name.clone(),
                            kind: TypeKind::StringEnum(variants.clone()),
                            fields: vec![],
                            doc: None,
                        });
                    }
                }
            }
        }
    }

    ApiSpec { tags, all_types }
}

fn parse_all_types(schemas: &serde_json::Map<String, Value>) -> Vec<TypeDef> {
    schemas
        .iter()
        .filter_map(|(name, schema)| parse_type_def(name, schema))
        .collect()
}

fn parse_type_def(name: &str, schema: &Value) -> Option<TypeDef> {
    let rust_name = spec_name_to_rust(name);
    let props = match schema.get("properties").and_then(|p| p.as_object()) {
        Some(p) => p,
        None => {
            // Skip non-object schemas (string enums, arrays, etc. at top level)
            if schema
                .get("type")
                .and_then(|t| t.as_str())
                .is_some_and(|t| t != "object")
            {
                return None;
            }
            // Generate empty struct for object schemas without properties
            return Some(TypeDef {
                name: rust_name,
                kind: TypeKind::Dto,
                fields: vec![],
                doc: schema
                    .get("description")
                    .and_then(|d| d.as_str())
                    .map(String::from),
            });
        }
    };
    let required: Vec<String> = schema
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    // Entity detection: single property that is a $ref
    if props.len() == 1 {
        let (prop_name, prop_val) = props.iter().next().unwrap();
        if let Some(ref_name) = extract_ref(prop_val) {
            let inner = spec_name_to_rust(&ref_name);
            let field = prop_name_to_rust(prop_name);
            return Some(TypeDef {
                name: rust_name,
                kind: TypeKind::Entity { field, inner },
                fields: vec![],
                doc: schema
                    .get("description")
                    .and_then(|d| d.as_str())
                    .map(String::from),
            });
        }
    }

    let fields = props
        .iter()
        .map(|(prop_name, prop_val)| {
            let is_required = required.contains(prop_name);
            let base_ty = parse_field_type(
                prop_val,
                &format!("/components/schemas/{name}/properties/{prop_name}"),
            );
            let ty = if is_required {
                base_ty
            } else {
                FieldType::Opt(Box::new(base_ty))
            };
            Field {
                rust_name: prop_name_to_rust(prop_name),
                serde_name: prop_name.clone(),
                ty,
                doc: prop_val
                    .get("description")
                    .and_then(|d| d.as_str())
                    .map(String::from),
                read_only: prop_val
                    .get("readOnly")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
                deprecated: prop_val
                    .get("deprecated")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
            }
        })
        .collect();

    Some(TypeDef {
        name: rust_name,
        kind: TypeKind::Dto,
        fields,
        doc: schema
            .get("description")
            .and_then(|d| d.as_str())
            .map(String::from),
    })
}

fn parse_field_type(prop: &Value, json_pointer: &str) -> FieldType {
    if let Some(ref_name) = extract_ref(prop) {
        return FieldType::Ref(spec_name_to_rust(&ref_name));
    }
    match prop.get("type").and_then(|t| t.as_str()) {
        Some("string") => {
            if let Some(variants) = prop.get("enum").and_then(|e| e.as_array()) {
                let vs: Vec<String> = variants
                    .iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect();
                return FieldType::Enum(vs);
            }
            FieldType::Str
        }
        Some("boolean") => FieldType::Bool,
        Some("integer") => match prop.get("format").and_then(|f| f.as_str()) {
            Some("int64") => FieldType::I64,
            _ => FieldType::I32,
        },
        Some("number") => FieldType::F64,
        Some("array") => {
            let items = &prop["items"];
            let inner = parse_field_type(items, &format!("{json_pointer}/items"));
            FieldType::List(Box::new(inner))
        }
        Some("object") => {
            if let Some(additional) = prop.get("additionalProperties") {
                let value_ty =
                    parse_field_type(additional, &format!("{json_pointer}/additionalProperties"));
                // Wrap in Option: map values can be null in real NiFi responses
                // even when the spec says type: string (e.g. unset processor properties).
                FieldType::Map(Box::new(FieldType::Opt(Box::new(value_ty))))
            } else {
                // object without additionalProperties — treat as opaque JSON value
                FieldType::Ref("serde_json::Value".to_string())
            }
        }
        Some(other) => crate::parser_strict::panic_unknown(
            "field_type",
            json_pointer,
            &format!("type={other:?}"),
        ),
        None => crate::parser_strict::panic_unknown(
            "field_type",
            json_pointer,
            "schema has no \"type\" and no \"$ref\"",
        ),
    }
}

fn extract_ref(val: &Value) -> Option<String> {
    val.get("$ref")
        .and_then(|r| r.as_str())
        .map(|r| r.trim_start_matches("#/components/schemas/").to_string())
}

/// Convert spec schema name to Rust PascalCase DTO name.
/// "AboutDTO" → "AboutDto", "ProcessGroupEntity" → "ProcessGroupEntity"
fn spec_name_to_rust(name: &str) -> String {
    if let Some(stripped) = name.strip_suffix("DTO") {
        format!("{stripped}Dto")
    } else {
        name.to_string()
    }
}

/// Convert camelCase property name to snake_case.
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

/// Returns the first non-param path segment after the first `{param}`, or None.
/// "/controller-services/{id}/config/analysis"   → Some("config")
/// "/controller-services/{id}"                   → None
/// "/flow/about"                                  → None
/// "/flow/{group}/{artifact}/definitions/cs"      → Some("definitions")
/// "/policies/{action}/{resource}"                → None (both segments are params)
fn sub_group_segment(path: &str) -> Option<&str> {
    let mut after_param = false;
    for seg in path.split('/').filter(|s| !s.is_empty()) {
        if after_param && !seg.starts_with('{') {
            return Some(seg);
        }
        if seg.starts_with('{') && seg.ends_with('}') {
            after_param = true;
        }
    }
    None
}

/// "run-status" → "RunStatus", "config" → "Config"
fn segment_to_pascal(seg: &str) -> String {
    seg.split('-')
        .map(|part| {
            let mut c = part.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}

/// Returns the first `{param}` name in a path, snake_case-normalized.
/// "/controller-services/{id}/config" → Some("id")
fn first_path_param(path: &str) -> Option<String> {
    path.split('/')
        .find(|s| s.starts_with('{') && s.ends_with('}'))
        .map(|s| prop_name_to_rust(&s[1..s.len() - 1].replace('-', "_")))
}

/// Capitalise the first character of a camelCase param name to produce PascalCase.
/// "parameterContextHandlingStrategy" → "ParameterContextHandlingStrategy"
fn pascal_case_param(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn parse_tags(
    spec: &Value,
    _schemas: &serde_json::Map<String, Value>,
    all_types: &[TypeDef],
) -> Vec<TagGroup> {
    let paths = match spec["paths"].as_object() {
        Some(p) => p,
        None => return vec![],
    };

    let mut tag_map: HashMap<String, Vec<Endpoint>> = HashMap::new();

    for (raw_path, methods) in paths {
        let methods = match methods.as_object() {
            Some(m) => m,
            None => continue,
        };
        for (http_method, op) in methods {
            let method = match http_method.as_str() {
                "get" => HttpMethod::Get,
                "post" => HttpMethod::Post,
                "put" => HttpMethod::Put,
                "delete" => HttpMethod::Delete,
                _ => continue,
            };
            let tags = op["tags"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let tag = tags.into_iter().next().unwrap_or_else(|| "Other".into());

            let operation_id = op["operationId"].as_str().unwrap_or("unknown");
            let fn_name = operation_id_to_fn_name(operation_id);
            let doc = op.get("summary").and_then(|s| s.as_str()).map(String::from);
            let description = op
                .get("description")
                .and_then(|d| d.as_str())
                .map(String::from);

            let path_params: Vec<PathParam> = op["parameters"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter(|p| p.get("in").and_then(|i| i.as_str()) == Some("path"))
                .filter_map(|p| {
                    let raw = p.get("name").and_then(|n| n.as_str())?;
                    Some(PathParam {
                        name: prop_name_to_rust(&raw.replace('-', "_")),
                        doc: p
                            .get("description")
                            .and_then(|d| d.as_str())
                            .map(String::from),
                    })
                })
                .collect();

            let query_params: Vec<QueryParam> = op["parameters"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter(|p| p.get("in").and_then(|i| i.as_str()) == Some("query"))
                .filter_map(|p| {
                    let name = p.get("name").and_then(|n| n.as_str())?;
                    let rust_name = prop_name_to_rust(&name.replace('-', "_"));
                    let required = p.get("required").and_then(|r| r.as_bool()).unwrap_or(false);
                    // Schema may be at p["schema"] (OpenAPI 3) or p itself (OpenAPI 2)
                    let schema = p.get("schema").unwrap_or(p);

                    // Detect enum variants before resolving the scalar type
                    let enum_variants: Option<Vec<String>> =
                        schema.get("enum").and_then(|e| e.as_array()).map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect()
                        });

                    let (ty, enum_type_name) = if let Some(variants) = enum_variants {
                        let type_name = pascal_case_param(name);
                        (QueryParamType::Enum(variants), Some(type_name))
                    } else {
                        let scalar = match schema.get("type").and_then(|t| t.as_str()) {
                            Some("boolean") => QueryParamType::Bool,
                            Some("integer") => {
                                match schema.get("format").and_then(|f| f.as_str()) {
                                    Some("int64") => QueryParamType::I64,
                                    _ => QueryParamType::I32,
                                }
                            }
                            Some("number") => QueryParamType::F64,
                            Some("string") | None => QueryParamType::Str,
                            Some(other) => crate::parser_strict::panic_unknown(
                                "query_param_type",
                                &format!("/paths{raw_path}/{http_method}/parameters/{name}"),
                                &format!("type={other:?}"),
                            ),
                        };
                        (scalar, None)
                    };

                    Some(QueryParam {
                        name: name.to_string(),
                        rust_name,
                        ty,
                        required,
                        doc: p
                            .get("description")
                            .and_then(|d| d.as_str())
                            .map(String::from),
                        enum_type_name,
                    })
                })
                .collect();

            let request_body = op.get("requestBody");
            let request_type = request_body
                .and_then(|rb| rb["content"]["application/json"]["schema"]["$ref"].as_str())
                .map(|r| r.trim_start_matches("#/components/schemas/").to_string())
                .map(|n| spec_name_to_rust(&n));
            let body_doc = request_body
                .and_then(|rb| rb.get("description"))
                .and_then(|d| d.as_str())
                .map(String::from);
            let body_kind = request_body
                .and_then(|rb| rb.get("content"))
                .and_then(|content| {
                    let pointer = format!("/paths{raw_path}/{http_method}/requestBody");
                    crate::content_type::resolve_request_body(content, &pointer)
                });

            let responses = op.get("responses");
            // Pick the 2xx/default response to drive codegen. Prefer one that
            // actually declares a content schema — some endpoints expose a
            // body-less 200 (e.g. "no flow file to return") alongside a 202
            // carrying the real payload.
            let pick_response = |r: &serde_json::Value| -> Option<serde_json::Value> {
                let try_codes = ["200", "201", "202", "default"];
                // First pass: first code that has content.
                for code in try_codes {
                    if let Some(v) = r.get(code) {
                        if v.get("content").is_some() {
                            return Some(v.clone());
                        }
                    }
                }
                // Fallback: first code that exists, even without content.
                for code in try_codes {
                    if let Some(v) = r.get(code) {
                        return Some(v.clone());
                    }
                }
                None
            };
            let chosen_response = responses.and_then(pick_response);
            let response_schema_ref = chosen_response
                .as_ref()
                .and_then(|r| r["content"]["application/json"]["schema"]["$ref"].as_str())
                .map(|r| r.trim_start_matches("#/components/schemas/").to_string());

            let response_kind = chosen_response
                .as_ref()
                .and_then(|r| r.get("content"))
                .map(|content| {
                    let pointer = format!("/paths{raw_path}/{http_method}/responses/2xx");
                    crate::content_type::resolve_response_body(content, &pointer)
                })
                .unwrap_or(crate::content_type::ResponseBodyKind::Empty);

            let (response_type, response_inner, response_field) = match response_schema_ref {
                None => (None, None, None),
                Some(ref_name) => {
                    let rust_name = spec_name_to_rust(&ref_name);
                    let entity_info =
                        all_types
                            .iter()
                            .find(|t| t.name == rust_name)
                            .and_then(|t| {
                                if let TypeKind::Entity { field, inner } = &t.kind {
                                    Some((field.clone(), inner.clone()))
                                } else {
                                    None
                                }
                            });
                    match entity_info {
                        Some((field, inner)) => (Some(rust_name), Some(inner), Some(field)),
                        None => (Some(rust_name), None, None),
                    }
                }
            };

            let success_responses: Vec<(String, String)> = responses
                .and_then(|r| r.as_object())
                .map(|map| {
                    let mut v: Vec<(String, String)> = map
                        .iter()
                        .filter(|(code, _)| code.starts_with('2') || code.starts_with('3'))
                        .filter_map(|(code, resp)| {
                            let desc = resp.get("description")?.as_str()?;
                            Some((code.clone(), desc.to_string()))
                        })
                        .collect();
                    v.sort_by(|a, b| a.0.cmp(&b.0));
                    v
                })
                .unwrap_or_default();

            let error_responses: Vec<(String, String)> = responses
                .and_then(|r| r.as_object())
                .map(|map| {
                    let mut v: Vec<(String, String)> = map
                        .iter()
                        .filter(|(code, _)| {
                            let is_success = code.starts_with('2') || code.starts_with('3');
                            !is_success && code.as_str() != "default"
                        })
                        .filter_map(|(code, resp)| {
                            let desc = resp.get("description")?.as_str()?;
                            Some((code.clone(), desc.to_string()))
                        })
                        .collect();
                    v.sort_by(|a, b| a.0.cmp(&b.0));
                    v
                })
                .unwrap_or_default();

            let security: Option<Vec<String>> = op.get("security").map(|sec| {
                sec.as_array()
                    .map(|a| a.as_slice())
                    .unwrap_or(&[])
                    .iter()
                    .flat_map(|entry| {
                        entry
                            .as_object()
                            .map(|obj| obj.keys().cloned().collect::<Vec<_>>())
                            .unwrap_or_default()
                    })
                    .collect()
            });

            let endpoint = Endpoint {
                method,
                path: raw_path.clone(),
                fn_name,
                doc,
                description,
                path_params,
                request_type,
                body_kind,
                body_doc,
                response_type,
                response_inner,
                response_field,
                response_kind,
                query_params,
                success_responses,
                error_responses,
                security,
            };
            tag_map.entry(tag).or_default().push(endpoint);
        }
    }

    let mut groups: Vec<TagGroup> = tag_map
        .into_iter()
        .map(|(tag, all_endpoints)| {
            let struct_name = format!("{}Api", tag.replace(' ', ""));
            let module_name = tag.to_lowercase().replace(' ', "_");
            let accessor_fn = tag_to_accessor(&tag);
            let parent_base = struct_name
                .strip_suffix("Api")
                .unwrap_or(&struct_name)
                .to_string();

            // Split endpoints: root vs. sub-group.
            // BTreeMap preserves deterministic ordering by accessor_fn.
            // Tuple: (raw_seg, primary_param, endpoints)
            let mut root_endpoints: Vec<Endpoint> = vec![];
            let mut sub_map: std::collections::BTreeMap<String, (String, String, Vec<Endpoint>)> =
                std::collections::BTreeMap::new();

            for ep in all_endpoints {
                match sub_group_segment(&ep.path) {
                    None => root_endpoints.push(ep),
                    Some(raw_seg) => {
                        let accessor = raw_seg.replace('-', "_");
                        // Primary param is the first path param in this specific path,
                        // computed per sub-group to avoid mismatches in tags with
                        // multiple param names (e.g. {id} vs {group}).
                        let pp = first_path_param(&ep.path).unwrap_or_else(|| "id".to_string());
                        sub_map
                            .entry(accessor)
                            .or_insert_with(|| (raw_seg.to_string(), pp, vec![]))
                            .2
                            .push(ep);
                    }
                }
            }

            let sub_groups = sub_map
                .into_iter()
                .map(|(accessor_fn, (raw_seg, primary_param, endpoints))| {
                    let pascal = segment_to_pascal(&raw_seg);
                    // Pull the description of the primary path param from any endpoint in the group.
                    let primary_param_doc = endpoints.iter().find_map(|ep| {
                        ep.path_params
                            .iter()
                            .find(|p| p.name == primary_param)
                            .and_then(|p| p.doc.clone())
                    });
                    SubGroup {
                        name: accessor_fn.clone(),
                        struct_name: format!("{parent_base}{pascal}Api"),
                        accessor_fn,
                        primary_param: primary_param.clone(),
                        primary_param_doc,
                        endpoints,
                    }
                })
                .collect();

            TagGroup {
                tag,
                struct_name,
                module_name,
                accessor_fn,
                types: vec![],
                root_endpoints,
                sub_groups,
            }
        })
        .collect();
    groups.sort_by(|a, b| a.tag.cmp(&b.tag));
    groups
}

fn operation_id_to_fn_name(id: &str) -> String {
    let mut out = String::new();
    for (i, ch) in id.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(ch.to_ascii_lowercase());
    }
    out
}

fn tag_to_accessor(tag: &str) -> String {
    format!("{}_api", tag.to_lowercase().replace(' ', "_"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_http_method_as_str() {
        assert_eq!(HttpMethod::Get.as_str(), "GET");
        assert_eq!(HttpMethod::Post.as_str(), "POST");
        assert_eq!(HttpMethod::Put.as_str(), "PUT");
        assert_eq!(HttpMethod::Delete.as_str(), "DELETE");
    }

    #[test]
    #[should_panic(expected = "nifi-openapi-gen: unknown field_type")]
    fn unknown_field_type_panics() {
        let prop = json!({ "type": "frobnitz" });
        parse_field_type(&prop, "/components/schemas/Foo/properties/bar");
    }

    #[test]
    fn known_field_types_still_work() {
        let prop = json!({ "type": "string" });
        let ft = parse_field_type(&prop, "/x");
        assert!(matches!(ft, FieldType::Str));
    }

    #[test]
    #[should_panic(expected = "nifi-openapi-gen: unknown query_param_type")]
    fn unknown_query_param_type_panics() {
        let spec = json!({
            "openapi": "3.0.1",
            "info": {"title": "t", "version": "1"},
            "paths": {
                "/foo": {
                    "get": {
                        "tags": ["Flow"],
                        "operationId": "getFoo",
                        "parameters": [{
                            "name": "weird",
                            "in": "query",
                            "schema": { "type": "frobnitz" }
                        }],
                        "responses": {"200": {"description": "ok"}}
                    }
                }
            },
            "components": {"schemas": {}}
        });
        let schemas = spec["components"]["schemas"].as_object().unwrap().clone();
        let _ = parse_tags(&spec, &schemas, &[]);
    }

    #[test]
    fn multipart_upload_endpoint_is_captured_in_29() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("specs/2.9.0/nifi-api.json");
        let content = std::fs::read_to_string(&path).unwrap();
        let v: serde_json::Value = serde_json::from_str(&content).unwrap();
        let schemas = v["components"]["schemas"].as_object().unwrap().clone();
        let tags = parse_tags(&v, &schemas, &[]);
        let all_eps: Vec<&Endpoint> = tags
            .iter()
            .flat_map(|t| {
                t.root_endpoints
                    .iter()
                    .chain(t.sub_groups.iter().flat_map(|sg| sg.endpoints.iter()))
            })
            .collect();
        let ep = all_eps
            .iter()
            .find(|e| {
                e.path == "/process-groups/{id}/process-groups/upload"
                    && matches!(e.method, HttpMethod::Post)
            })
            .expect("upload endpoint missing from parsed 2.9.0 spec");
        assert_eq!(ep.body_kind, Some(RequestBodyKind::Multipart));
    }

    #[test]
    fn wildcard_body_endpoint_is_captured_in_29() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("specs/2.9.0/nifi-api.json");
        let content = std::fs::read_to_string(&path).unwrap();
        let v: serde_json::Value = serde_json::from_str(&content).unwrap();
        let schemas = v["components"]["schemas"].as_object().unwrap().clone();
        let tags = parse_tags(&v, &schemas, &[]);
        let all_eps: Vec<&Endpoint> = tags
            .iter()
            .flat_map(|t| {
                t.root_endpoints
                    .iter()
                    .chain(t.sub_groups.iter().flat_map(|sg| sg.endpoints.iter()))
            })
            .collect();
        let ep = all_eps
            .iter()
            .find(|e| {
                e.path == "/data-transfer/{portType}/{portId}/transactions"
                    && matches!(e.method, HttpMethod::Post)
            })
            .expect("data-transfer transactions endpoint missing");
        assert_eq!(ep.body_kind, Some(RequestBodyKind::Wildcard));
    }

    #[test]
    fn text_plain_response_is_captured_in_29() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("specs/2.9.0/nifi-api.json");
        let content = std::fs::read_to_string(&path).unwrap();
        let v: serde_json::Value = serde_json::from_str(&content).unwrap();
        let schemas = v["components"]["schemas"].as_object().unwrap().clone();
        let tags = parse_tags(&v, &schemas, &[]);
        let ep = tags
            .iter()
            .flat_map(|t| {
                t.root_endpoints
                    .iter()
                    .chain(t.sub_groups.iter().flat_map(|sg| sg.endpoints.iter()))
            })
            .find(|e| e.path == "/flow/client-id" && matches!(e.method, HttpMethod::Get))
            .expect("/flow/client-id missing from parsed 2.9.0 spec");
        assert!(matches!(
            ep.response_kind,
            crate::content_type::ResponseBodyKind::Text
        ));
    }

    #[test]
    fn octet_stream_response_is_captured_in_29() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("specs/2.9.0/nifi-api.json");
        let content = std::fs::read_to_string(&path).unwrap();
        let v: serde_json::Value = serde_json::from_str(&content).unwrap();
        let schemas = v["components"]["schemas"].as_object().unwrap().clone();
        let tags = parse_tags(&v, &schemas, &[]);
        let ep = tags
            .iter()
            .flat_map(|t| {
                t.root_endpoints
                    .iter()
                    .chain(t.sub_groups.iter().flat_map(|sg| sg.endpoints.iter()))
            })
            .find(|e| {
                e.path == "/connectors/{id}/assets/{assetId}" && matches!(e.method, HttpMethod::Get)
            })
            .expect("/connectors/{id}/assets/{assetId} missing");
        assert!(matches!(
            ep.response_kind,
            crate::content_type::ResponseBodyKind::OctetStream
        ));
    }

    // NOTE: no XML-only response exists in any bundled NiFi 2.x spec. The
    // only application/xml producer, /site-to-site/peers, also advertises
    // application/json, and resolve_response_body picks JSON first by
    // design. XML endpoint verification is deferred to Task 3.3.

    #[test]
    fn response_code_precedence_picks_first_2xx_with_content() {
        // /data-transfer/output-ports/{portId}/transactions/{transactionId}/flow-files
        // declares 200 (no content) and 202 (application/octet-stream).
        // The parser must prefer the 202 body so the endpoint emits Vec<u8>
        // instead of `()`. A naive 200 → 201 → 202 walker would silently
        // pick the empty 200 and strand the octet-stream body.
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("specs/2.9.0/nifi-api.json");
        let content = std::fs::read_to_string(&path).unwrap();
        let v: serde_json::Value = serde_json::from_str(&content).unwrap();
        let schemas = v["components"]["schemas"].as_object().unwrap().clone();
        let tags = parse_tags(&v, &schemas, &[]);
        let ep = tags
            .iter()
            .flat_map(|t| {
                t.root_endpoints
                    .iter()
                    .chain(t.sub_groups.iter().flat_map(|sg| sg.endpoints.iter()))
            })
            .find(|e| {
                e.path
                    == "/data-transfer/output-ports/{portId}/transactions/{transactionId}/flow-files"
                    && matches!(e.method, HttpMethod::Get)
            })
            .expect("data-transfer output-ports flow-files GET missing");
        assert!(matches!(
            ep.response_kind,
            crate::content_type::ResponseBodyKind::OctetStream
        ));
    }

    #[test]
    fn json_response_still_captures_ref() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("specs/2.9.0/nifi-api.json");
        let content = std::fs::read_to_string(&path).unwrap();
        let v: serde_json::Value = serde_json::from_str(&content).unwrap();
        let schemas = v["components"]["schemas"].as_object().unwrap().clone();
        let tags = parse_tags(&v, &schemas, &[]);
        let ep = tags
            .iter()
            .flat_map(|t| {
                t.root_endpoints
                    .iter()
                    .chain(t.sub_groups.iter().flat_map(|sg| sg.endpoints.iter()))
            })
            .find(|e| e.path == "/flow/about")
            .expect("/flow/about missing");
        assert!(matches!(
            &ep.response_kind,
            crate::content_type::ResponseBodyKind::Json { .. }
        ));
        // response_type should still be populated the old way
        assert!(ep.response_type.is_some());
    }
}
