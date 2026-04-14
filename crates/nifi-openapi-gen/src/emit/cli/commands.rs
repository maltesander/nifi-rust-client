use std::collections::HashMap;

use crate::content_type::RequestBodyKind;
use crate::parser::{ApiSpec, Endpoint, HttpMethod, QueryParamType, TagGroup};
use crate::util::format_source;

/// Emit all generated CLI code.
///
/// Accepts all parsed specs (sorted oldest → newest). The **latest** spec
/// drives the endpoint surface; older specs are only used to establish
/// canonical fn_names that match the dynamic dispatch layer (which
/// canonicalizes by `(path, method)` using the oldest version's name).
///
/// Returns a `Vec<(filename, source)>` — currently a single
/// `"generated_cli.rs"` file containing all generated CLI code.
pub fn emit_cli(all_specs: &[(String, ApiSpec)]) -> Vec<(String, String)> {
    let (_latest_ver, spec) = all_specs
        .last()
        .expect("at least one NiFi version must be present");

    // Build canonical fn_name map: (tag, path, method) → canonical fn_name.
    // The first (oldest) version's fn_name wins so a command name stays
    // stable when newer specs rename an operation.
    let canonical = build_canonical_fn_names(all_specs);

    let mut out = String::new();

    out.push_str("// Auto-generated — do not edit\n");
    out.push_str("use serde_json::Value;\n\n");

    // Collect tags that have at least one emittable endpoint
    let emittable_tags: Vec<&TagGroup> = spec
        .tags
        .iter()
        .filter(|t| has_emittable_endpoints(t))
        .collect();

    // --- GeneratedResource enum ---
    emit_resource_enum(&mut out, &emittable_tags);

    // --- Top-level dispatch ---
    emit_top_dispatch(&mut out, &emittable_tags);

    // --- Per-tag code ---
    for tag in &emittable_tags {
        emit_tag_code(&mut out, tag, &canonical);
    }

    let formatted = format_source(&out);
    vec![("generated_cli.rs".to_string(), formatted)]
}

/// Key for canonical fn_name lookup: "tag\0path\0METHOD"
type CanonicalKey = String;

fn canonical_key(tag: &str, path: &str, method: &HttpMethod) -> CanonicalKey {
    let m = match method {
        HttpMethod::Get => "GET",
        HttpMethod::Post => "POST",
        HttpMethod::Put => "PUT",
        HttpMethod::Delete => "DELETE",
    };
    format!("{tag}\0{path}\0{m}")
}

/// Build a map from (tag, path, method) → canonical fn_name.
/// The canonical name is the fn_name from the oldest version that has the endpoint.
fn build_canonical_fn_names(all_specs: &[(String, ApiSpec)]) -> HashMap<CanonicalKey, String> {
    let mut map = HashMap::new();
    for (_ver, spec) in all_specs {
        for tag in &spec.tags {
            for ep in &tag.endpoints {
                let key = canonical_key(&tag.tag, &ep.path, &ep.method);
                // First version wins (oldest)
                map.entry(key).or_insert_with(|| ep.fn_name.clone());
            }
        }
    }
    map
}

/// Returns true if the tag has any non-form-encoded endpoints.
fn has_emittable_endpoints(tag: &TagGroup) -> bool {
    tag.endpoints.iter().any(|ep| !is_skipped_body_kind(ep))
}

fn is_skipped_body_kind(ep: &Endpoint) -> bool {
    matches!(
        ep.body_kind,
        Some(RequestBodyKind::FormEncoded)
            | Some(RequestBodyKind::OctetStream)
            | Some(RequestBodyKind::Multipart)
    )
}

/// The resource name used in CLI subcommand variants.
///
/// Task 2.1 flattened `TagGroup::struct_name` to the bare tag name
/// (e.g. `"Processors"` rather than `"ProcessorsApi"`), so this is a
/// direct clone of `struct_name`.
fn resource_name(tag: &TagGroup) -> String {
    tag.struct_name.clone()
}

fn emit_resource_enum(out: &mut String, tags: &[&TagGroup]) {
    out.push_str("/// Resource subcommands\n");
    out.push_str("#[derive(clap::Subcommand)]\n");
    out.push_str("pub enum GeneratedResource {\n");
    for tag in tags {
        let variant = resource_name(tag);
        if let Some(doc) = first_tag_doc(tag) {
            out.push_str(&format!("    /// {doc}\n"));
        }
        out.push_str(&format!("    #[command(name = \"{}\")]\n", tag.module_name));
        out.push_str(&format!(
            "    {variant} {{\n        #[command(subcommand)]\n        command: {variant}Command,\n    }},\n"
        ));
    }
    out.push_str("}\n\n");
}

fn first_tag_doc(tag: &TagGroup) -> Option<String> {
    Some(format!("Manage {} resources", resource_name(tag)))
}

fn emit_top_dispatch(out: &mut String, tags: &[&TagGroup]) {
    out.push_str("pub async fn dispatch_generated(\n");
    out.push_str("    resource: GeneratedResource,\n");
    out.push_str("    client: &nifi_rust_client::dynamic::DynamicClient,\n");
    out.push_str(") -> Result<crate::output::CliOutput, crate::error::CliError> {\n");
    out.push_str("    match resource {\n");
    for tag in tags {
        let variant = resource_name(tag);
        let dispatch_fn = format!("dispatch_{}", tag.module_name);
        out.push_str(&format!(
            "        GeneratedResource::{variant} {{ command }} => {dispatch_fn}(command, client).await,\n"
        ));
    }
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn emit_tag_code(out: &mut String, tag: &TagGroup, canonical: &HashMap<CanonicalKey, String>) {
    let variant_base = resource_name(tag);

    out.push_str(&format!("// === {} ===\n\n", variant_base));

    // Command entries — one per emittable endpoint. Command name is derived
    // from `fn_name` (kebab-case), which is guaranteed unique per tag by
    // `naming::check_collisions`, so no deduplication pass is needed.
    let mut commands: Vec<CommandEntry> = Vec::new();
    for ep in &tag.endpoints {
        if is_skipped_body_kind(ep) {
            continue;
        }
        let cmd_name = ep.fn_name.replace('_', "-");
        let variant_name = pascal_case(&ep.fn_name);
        let args_name = format!("{variant_base}{variant_name}Args");
        commands.push(CommandEntry {
            command_name: cmd_name,
            variant_name,
            args_name,
            endpoint: ep,
        });
    }

    // --- Command enum ---
    out.push_str("#[derive(clap::Subcommand)]\n");
    out.push_str(&format!("pub enum {variant_base}Command {{\n"));
    for cmd in &commands {
        if let Some(doc) = &cmd.endpoint.doc {
            out.push_str(&format!("    /// {}\n", escape_doc(doc)));
        }
        out.push_str(&format!(
            "    #[command(name = \"{}\")]\n",
            cmd.command_name
        ));
        out.push_str(&format!("    {}({}),\n", cmd.variant_name, cmd.args_name));
    }
    out.push_str("}\n\n");

    // --- Args structs ---
    for cmd in &commands {
        emit_args_struct(out, cmd);
    }

    // --- Dispatch function ---
    let dispatch_fn = format!("dispatch_{}", tag.module_name);
    out.push_str(&format!("async fn {dispatch_fn}(\n"));
    out.push_str(&format!("    command: {variant_base}Command,\n"));
    out.push_str("    client: &nifi_rust_client::dynamic::DynamicClient,\n");
    out.push_str(") -> Result<crate::output::CliOutput, crate::error::CliError> {\n");
    out.push_str("    match command {\n");
    for cmd in &commands {
        let handler = handler_fn_name(&tag.module_name, &cmd.command_name);
        out.push_str(&format!(
            "        {variant_base}Command::{}(args) => {handler}(args, client).await,\n",
            cmd.variant_name
        ));
    }
    out.push_str("    }\n");
    out.push_str("}\n\n");

    // --- Handler functions ---
    for cmd in &commands {
        emit_handler(out, tag, cmd, canonical);
    }
}

struct CommandEntry<'a> {
    command_name: String,
    variant_name: String,
    args_name: String,
    endpoint: &'a Endpoint,
}

fn emit_args_struct(out: &mut String, cmd: &CommandEntry<'_>) {
    let ep = cmd.endpoint;
    out.push_str("#[derive(clap::Args)]\n");
    out.push_str(&format!("pub struct {} {{\n", cmd.args_name));

    // Path params — all positional, in declaration order.
    for pp in &ep.path_params {
        if let Some(doc) = &pp.doc {
            out.push_str(&format!("    /// {}\n", escape_doc(doc)));
        }
        out.push_str(&format!("    pub {}: String,\n", rust_ident(&pp.name)));
    }

    // Query params as --flags
    for qp in &ep.query_params {
        if let Some(doc) = &qp.doc {
            out.push_str(&format!("    /// {}\n", escape_doc(doc)));
        }
        let flag_name = qp.rust_name.replace('_', "-");
        let rust_type = cli_query_type(&qp.ty);
        let field = rust_ident(&qp.rust_name);
        if qp.required {
            out.push_str(&format!(
                "    #[arg(long = \"{flag_name}\")]\n    pub {field}: {rust_type},\n",
            ));
        } else {
            out.push_str(&format!(
                "    #[arg(long = \"{flag_name}\")]\n    pub {field}: Option<{rust_type}>,\n",
            ));
        }
    }

    // Body args for POST/PUT with request_type
    if ep.request_type.is_some() && (ep.method == HttpMethod::Post || ep.method == HttpMethod::Put)
    {
        out.push_str("    /// Request body as JSON string\n");
        out.push_str("    #[arg(long = \"body\")]\n");
        out.push_str("    pub body: Option<String>,\n");
        out.push_str("    /// Path to a file containing the request body JSON\n");
        out.push_str("    #[arg(long = \"body-file\")]\n");
        out.push_str("    pub body_file: Option<std::path::PathBuf>,\n");
    }

    out.push_str("}\n\n");
}

fn emit_handler(
    out: &mut String,
    tag: &TagGroup,
    cmd: &CommandEntry<'_>,
    canonical: &HashMap<CanonicalKey, String>,
) {
    let ep = cmd.endpoint;
    let handler_name = handler_fn_name(&tag.module_name, &cmd.command_name);

    out.push_str(&format!("async fn {handler_name}(\n"));
    out.push_str(&format!("    args: {},\n", cmd.args_name));
    out.push_str("    client: &nifi_rust_client::dynamic::DynamicClient,\n");
    out.push_str(") -> Result<crate::output::CliOutput, crate::error::CliError> {\n");

    // Canonical dynamic path has no traits — the concrete resource struct is
    // returned directly and its methods are inherent. No import needed.
    let _ = cmd;

    // Flat API: a single accessor returns the tag struct, then the method
    // is called directly with all path params as regular arguments.
    let api_access = format!("client.{}()", tag.accessor_fn);

    // Body resolution for POST/PUT
    if let Some(req_type) = &ep.request_type
        && (ep.method == HttpMethod::Post || ep.method == HttpMethod::Put)
    {
        out.push_str("    let body_json = crate::body::resolve_body(args.body.as_deref(), args.body_file.as_deref())?;\n");
        out.push_str(&format!(
            "    let body: nifi_rust_client::dynamic::types::{req_type} = serde_json::from_value(body_json)\n"
        ));
        out.push_str("        .map_err(|e| crate::error::CliError::User(format!(\"invalid request body: {e}\")))?;\n");
    }

    // Build method call arguments — all path params are passed in order.
    let mut call_args = Vec::new();
    for pp in &ep.path_params {
        call_args.push(format!("&args.{}", rust_ident(&pp.name)));
    }

    // Query params
    for qp in &ep.query_params {
        let field = rust_ident(&qp.rust_name);
        match &qp.ty {
            QueryParamType::Enum(_) => {
                // Enum query params: parse CLI string into the enum type via serde.
                let enum_type = qp
                    .enum_type_name
                    .as_deref()
                    .expect("Enum query param must have enum_type_name");
                let parse_var = format!("parsed_{}", qp.rust_name);
                if qp.required {
                    out.push_str(&format!(
                        "    let {parse_var}: nifi_rust_client::dynamic::types::{enum_type} = \
                         serde_json::from_value(serde_json::Value::String(args.{field}.clone()))\n        \
                         .map_err(|e| crate::error::CliError::User(format!(\"invalid value for --{}: {{e}}\", )))?;\n",
                        qp.rust_name.replace('_', "-"),
                    ));
                    call_args.push(parse_var);
                } else {
                    out.push_str(&format!(
                        "    let {parse_var}: Option<nifi_rust_client::dynamic::types::{enum_type}> = args.{field}\n        \
                         .as_ref()\n        \
                         .map(|s| serde_json::from_value(serde_json::Value::String(s.clone())))\n        \
                         .transpose()\n        \
                         .map_err(|e| crate::error::CliError::User(format!(\"invalid value for --{}: {{e}}\")))?;\n",
                        qp.rust_name.replace('_', "-"),
                    ));
                    call_args.push(parse_var);
                }
            }
            QueryParamType::Str => {
                if qp.required {
                    call_args.push(format!("&args.{field}"));
                } else {
                    call_args.push(format!("args.{field}.as_deref()"));
                }
            }
            QueryParamType::Bool => {
                call_args.push(format!("args.{field}"));
            }
            QueryParamType::I32 | QueryParamType::I64 | QueryParamType::F64 => {
                call_args.push(format!("args.{field}"));
            }
        }
    }

    // Body param
    if ep.request_type.is_some() && (ep.method == HttpMethod::Post || ep.method == HttpMethod::Put)
    {
        call_args.push("&body".to_string());
    }

    let args_str = call_args.join(", ");

    // Use canonical fn_name (oldest version's name) to match the dynamic trait method name.
    let key = canonical_key(&tag.tag, &ep.path, &ep.method);
    let method_name = canonical.get(&key).unwrap_or(&ep.fn_name);

    out.push_str(&format!(
        "    let result = {api_access}.{method_name}({args_str}).await?;\n",
    ));

    // Return value
    if ep.response_type.is_some() {
        out.push_str("    let value = serde_json::to_value(&result)\n");
        out.push_str("        .map_err(|e| crate::error::CliError::User(format!(\"serialization error: {e}\")))?;\n");
        out.push_str("    Ok(crate::output::CliOutput::Single(value))\n");
    } else {
        out.push_str("    Ok(crate::output::CliOutput::Empty)\n");
    }

    out.push_str("}\n\n");
}

fn handler_fn_name(module_name: &str, command_name: &str) -> String {
    format!("handle_{}_{}", module_name, command_name.replace('-', "_"))
}

fn cli_query_type(ty: &QueryParamType) -> &'static str {
    match ty {
        QueryParamType::Str => "String",
        QueryParamType::Bool => "bool",
        QueryParamType::I32 => "i32",
        QueryParamType::I64 => "i64",
        QueryParamType::F64 => "f64",
        QueryParamType::Enum(_) => "String",
    }
}

fn pascal_case(s: &str) -> String {
    s.split(['-', '_'])
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => {
                    let mut s = c.to_uppercase().to_string();
                    s.extend(chars);
                    s
                }
            }
        })
        .collect()
}

fn escape_doc(s: &str) -> String {
    s.replace('\n', " ").replace('\r', "")
}

/// Escape Rust keywords so they can be used as identifiers.
fn rust_ident(name: &str) -> String {
    match name {
        "type" | "match" | "move" | "ref" | "self" | "super" | "crate" | "as" | "break"
        | "const" | "continue" | "else" | "enum" | "extern" | "false" | "fn" | "for" | "if"
        | "impl" | "in" | "let" | "loop" | "mod" | "mut" | "pub" | "return" | "static"
        | "struct" | "trait" | "true" | "unsafe" | "use" | "where" | "while" | "async"
        | "await" | "dyn" | "abstract" | "become" | "box" | "do" | "final" | "macro"
        | "override" | "priv" | "typeof" | "unsized" | "virtual" | "yield" | "try" => {
            format!("r#{name}")
        }
        _ => name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content_type::ResponseBodyKind;
    use crate::parser::{PathParam, TagGroup};

    fn make_endpoint(
        method: HttpMethod,
        fn_name: &str,
        path: &str,
        path_params: Vec<PathParam>,
        response_type: Option<&str>,
    ) -> Endpoint {
        Endpoint {
            method,
            path: path.to_string(),
            fn_name: fn_name.to_string(),
            raw_operation_id: String::new(),
            doc: Some(format!("Does {fn_name}")),
            description: None,
            path_params,
            request_type: None,
            body_kind: None,
            body_doc: None,
            response_type: response_type.map(str::to_string),
            response_inner: None,
            response_field: None,
            response_kind: if response_type.is_some() {
                ResponseBodyKind::Json {
                    schema_ref: "test".to_string(),
                }
            } else {
                ResponseBodyKind::Empty
            },
            query_params: vec![],
            header_params: vec![],
            success_responses: vec![],
            error_responses: vec![],
            security: None,
        }
    }

    fn make_tag(tag: &str, endpoints: Vec<Endpoint>) -> TagGroup {
        let module_name = tag.to_lowercase();
        TagGroup {
            tag: tag.to_string(),
            struct_name: tag.to_string(),
            module_name: module_name.clone(),
            accessor_fn: module_name.clone(),
            types: vec![],
            endpoints,
        }
    }

    fn make_spec(tags: Vec<TagGroup>) -> ApiSpec {
        ApiSpec {
            tags,
            all_types: vec![],
        }
    }

    #[test]
    fn emit_produces_resource_enum() {
        let ep_get = make_endpoint(
            HttpMethod::Get,
            "get_processor",
            "/processors/{id}",
            vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            Some("ProcessorEntity"),
        );
        let ep_delete = make_endpoint(
            HttpMethod::Delete,
            "delete_processor",
            "/processors/{id}",
            vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            None,
        );
        let spec = make_spec(vec![make_tag("Processors", vec![ep_get, ep_delete])]);
        let files = emit_cli(&[("2.8.0".to_string(), spec)]);

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].0, "generated_cli.rs");

        let src = &files[0].1;
        assert!(
            src.contains("GeneratedResource"),
            "missing GeneratedResource enum"
        );
        assert!(src.contains("Processors"), "missing Processors variant");
    }

    #[test]
    fn emit_produces_commands() {
        let ep_get = make_endpoint(
            HttpMethod::Get,
            "get_processor",
            "/processors/{id}",
            vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            Some("ProcessorEntity"),
        );
        let ep_delete = make_endpoint(
            HttpMethod::Delete,
            "delete_processor",
            "/processors/{id}",
            vec![PathParam {
                name: "id".to_string(),
                doc: None,
            }],
            None,
        );
        let spec = make_spec(vec![make_tag("Processors", vec![ep_get, ep_delete])]);
        let files = emit_cli(&[("2.8.0".to_string(), spec)]);
        let src = &files[0].1;

        assert!(
            src.contains("ProcessorsCommand"),
            "missing ProcessorsCommand enum"
        );
        // Flat command name is derived from fn_name — `get_processor` →
        // `get-processor`, variant `GetProcessor`, args struct
        // `ProcessorsGetProcessorArgs`.
        assert!(
            src.contains("ProcessorsGetProcessorArgs"),
            "missing ProcessorsGetProcessorArgs struct"
        );
        assert!(
            src.contains("ProcessorsDeleteProcessorArgs"),
            "missing ProcessorsDeleteProcessorArgs struct"
        );
        assert!(
            src.contains("name = \"get-processor\""),
            "missing get-processor command name"
        );
        assert!(
            src.contains("name = \"delete-processor\""),
            "missing delete-processor command name"
        );
    }
}
