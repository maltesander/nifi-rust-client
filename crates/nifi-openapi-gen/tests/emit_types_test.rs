use nifi_openapi_gen::emit_types;
use nifi_openapi_gen::parser::{
    ApiSpec, Endpoint, Field, FieldType, HttpMethod, PathParam, TagGroup, TypeDef, TypeKind,
};

/// Join all generated file contents for assertions across all files.
fn all_output(spec: &ApiSpec) -> String {
    emit_types(spec)
        .into_iter()
        .map(|(_, content)| content)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Returns content of a specific output file.
fn file_output(spec: &ApiSpec, filename: &str) -> String {
    emit_types(spec)
        .into_iter()
        .find(|(name, _)| name == filename)
        .map(|(_, content)| content)
        .unwrap_or_default()
}

fn spec_with_types(types: Vec<TypeDef>) -> ApiSpec {
    ApiSpec {
        tags: vec![],
        all_types: types,
    }
}

#[test]
fn emit_simple_dto() {
    let spec = spec_with_types(vec![TypeDef {
        name: "AboutDto".into(),
        kind: TypeKind::Dto,
        fields: vec![
            Field {
                rust_name: "title".into(),
                serde_name: "title".into(),
                ty: FieldType::Opt(Box::new(FieldType::Str)),
                doc: Some("The title".into()),
                read_only: false,
            },
            Field {
                rust_name: "count".into(),
                serde_name: "count".into(),
                ty: FieldType::I32,
                doc: None,
                read_only: false,
            },
        ],
        doc: Some("About NiFi".into()),
    }]);
    let out = all_output(&spec);
    assert!(out.contains("pub struct AboutDto"), "missing struct: {out}");
    assert!(
        out.contains("pub title: Option<String>"),
        "missing title: {out}"
    );
    assert!(out.contains("pub count: i32"), "missing count: {out}");
    assert!(out.contains("/// The title"), "missing doc: {out}");
    assert!(out.contains("/// About NiFi"), "missing struct doc: {out}");
    assert!(
        out.contains("#[serde(rename_all = \"camelCase\")]"),
        "missing serde attr: {out}"
    );
    assert!(
        !out.contains("#[serde(rename_all = \"camelCase\", default)]"),
        "struct-level serde(default) must not appear: {out}"
    );
}

#[test]
fn emit_entity_wrapper() {
    let spec = spec_with_types(vec![TypeDef {
        name: "AboutEntity".into(),
        kind: TypeKind::Entity {
            field: "about".into(),
            inner: "AboutDto".into(),
        },
        fields: vec![],
        doc: None,
    }]);
    let out = all_output(&spec);
    assert!(
        out.contains("pub struct AboutEntity"),
        "missing entity: {out}"
    );
    assert!(out.contains("pub about: AboutDto"), "missing field: {out}");
}

#[test]
fn emit_field_types() {
    let spec = spec_with_types(vec![TypeDef {
        name: "SampleDto".into(),
        kind: TypeKind::Dto,
        fields: vec![
            Field {
                rust_name: "a".into(),
                serde_name: "a".into(),
                ty: FieldType::I64,
                doc: None,
                read_only: false,
            },
            Field {
                rust_name: "b".into(),
                serde_name: "b".into(),
                ty: FieldType::F64,
                doc: None,
                read_only: false,
            },
            Field {
                rust_name: "c".into(),
                serde_name: "c".into(),
                ty: FieldType::Bool,
                doc: None,
                read_only: false,
            },
            Field {
                rust_name: "items".into(),
                serde_name: "items".into(),
                ty: FieldType::Opt(Box::new(FieldType::List(Box::new(FieldType::Ref(
                    "OtherDto".into(),
                ))))),
                doc: None,
                read_only: false,
            },
        ],
        doc: None,
    }]);
    let out = all_output(&spec);
    assert!(out.contains("pub a: i64"), "{out}");
    assert!(out.contains("pub b: f64"), "{out}");
    assert!(out.contains("pub c: bool"), "{out}");
    assert!(out.contains("pub items: Option<Vec<OtherDto>>"), "{out}");
}

#[test]
fn emit_returns_multiple_files() {
    let spec = spec_with_types(vec![]);
    let files = emit_types(&spec);
    let names: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();
    assert!(names.contains(&"mod.rs"), "missing mod.rs: {names:?}");
    assert!(names.contains(&"common.rs"), "missing common.rs: {names:?}");
}

#[test]
fn mod_rs_reexports_all_modules() {
    // A spec with two tags each using a unique type, plus one shared type.
    let spec = ApiSpec {
        all_types: vec![
            TypeDef {
                name: "FlowDto".into(),
                kind: TypeKind::Dto,
                fields: vec![],
                doc: None,
            },
            TypeDef {
                name: "ProcDto".into(),
                kind: TypeKind::Dto,
                fields: vec![],
                doc: None,
            },
            TypeDef {
                name: "SharedDto".into(),
                kind: TypeKind::Dto,
                fields: vec![],
                doc: None,
            },
        ],
        tags: vec![
            TagGroup {
                tag: "Flow".into(),
                struct_name: "FlowApi".into(),
                module_name: "flow".into(),
                accessor_fn: "flow_api".into(),
                types: vec![],
                root_endpoints: vec![
                    Endpoint {
                        method: HttpMethod::Get,
                        path: "/flow".into(),
                        fn_name: "get_flow".into(),
                        doc: None,
                        description: None,
                        path_params: vec![],
                        request_type: None,
                        body_doc: None,

                        body_kind: None,
                        response_type: Some("FlowDto".into()),
                        response_inner: None,
                        response_field: None,
                        query_params: vec![],
                        error_responses: vec![],
                        security: None,
                    },
                    Endpoint {
                        method: HttpMethod::Get,
                        path: "/flow/shared".into(),
                        fn_name: "get_shared".into(),
                        doc: None,
                        description: None,
                        path_params: vec![],
                        request_type: None,
                        body_doc: None,

                        body_kind: None,
                        response_type: Some("SharedDto".into()),
                        response_inner: None,
                        response_field: None,
                        query_params: vec![],
                        error_responses: vec![],
                        security: None,
                    },
                ],
                sub_groups: vec![],
            },
            TagGroup {
                tag: "Processors".into(),
                struct_name: "ProcessorsApi".into(),
                module_name: "processors".into(),
                accessor_fn: "processors_api".into(),
                types: vec![],
                root_endpoints: vec![
                    Endpoint {
                        method: HttpMethod::Get,
                        path: "/processors/{id}".into(),
                        fn_name: "get_processor".into(),
                        doc: None,
                        description: None,
                        path_params: vec![PathParam {
                            name: "id".into(),
                            doc: None,
                        }],
                        request_type: None,
                        body_doc: None,

                        body_kind: None,
                        response_type: Some("ProcDto".into()),
                        response_inner: None,
                        response_field: None,
                        query_params: vec![],
                        error_responses: vec![],
                        security: None,
                    },
                    Endpoint {
                        method: HttpMethod::Get,
                        path: "/processors/shared".into(),
                        fn_name: "get_shared_proc".into(),
                        doc: None,
                        description: None,
                        path_params: vec![],
                        request_type: None,
                        body_doc: None,

                        body_kind: None,
                        response_type: Some("SharedDto".into()),
                        response_inner: None,
                        response_field: None,
                        query_params: vec![],
                        error_responses: vec![],
                        security: None,
                    },
                ],
                sub_groups: vec![],
            },
        ],
    };

    let mod_content = file_output(&spec, "mod.rs");
    assert!(mod_content.contains("pub mod common"), "{mod_content}");
    assert!(mod_content.contains("pub mod flow"), "{mod_content}");
    assert!(mod_content.contains("pub mod processors"), "{mod_content}");
    assert!(mod_content.contains("pub use common::*"), "{mod_content}");
    assert!(mod_content.contains("pub use flow::*"), "{mod_content}");
    assert!(
        mod_content.contains("pub use processors::*"),
        "{mod_content}"
    );

    // FlowDto should be in flow.rs, not common.rs
    let flow_content = file_output(&spec, "flow.rs");
    assert!(
        flow_content.contains("pub struct FlowDto"),
        "{flow_content}"
    );

    // SharedDto referenced by both tags → common.rs
    let common_content = file_output(&spec, "common.rs");
    assert!(
        common_content.contains("pub struct SharedDto"),
        "{common_content}"
    );
}

#[test]
fn emit_string_enum_generates_enum_and_display() {
    use nifi_openapi_gen::emit_types;
    use nifi_openapi_gen::parser::{ApiSpec, TypeDef, TypeKind};

    let spec = ApiSpec {
        all_types: vec![TypeDef {
            name: "ParameterContextHandlingStrategy".into(),
            kind: TypeKind::StringEnum(vec!["KEEP_EXISTING".into(), "REPLACE".into()]),
            fields: vec![],
            doc: None,
        }],
        tags: vec![],
    };

    // find the file that contains this type — with no tags it goes to common.rs
    let files = emit_types(&spec);
    let common = files.iter().find(|(n, _)| n == "common.rs").unwrap();

    // enum declaration
    assert!(
        common
            .1
            .contains("pub enum ParameterContextHandlingStrategy"),
        "missing enum: {}",
        common.1
    );
    // variants exist with serde rename
    assert!(
        common.1.contains("KeepExisting") && common.1.contains("\"KEEP_EXISTING\""),
        "missing KeepExisting variant: {}",
        common.1
    );
    assert!(
        common.1.contains("Replace") && common.1.contains("\"REPLACE\""),
        "missing Replace variant: {}",
        common.1
    );
    // Display impl exists and outputs wire value
    assert!(
        common
            .1
            .contains("impl std::fmt::Display for ParameterContextHandlingStrategy"),
        "missing Display impl: {}",
        common.1
    );
    assert!(
        common.1.contains("\"KEEP_EXISTING\"") && common.1.contains("\"REPLACE\""),
        "Display impl missing wire values: {}",
        common.1
    );
    // standalone enum must NOT derive Default
    assert!(
        !common.1.contains("Default"),
        "standalone enum must not derive Default: {}",
        common.1
    );
}

#[test]
fn emit_multiline_field_doc() {
    let spec = spec_with_types(vec![TypeDef {
        name: "AboutDto".into(),
        kind: TypeKind::Dto,
        fields: vec![Field {
            rust_name: "title".into(),
            serde_name: "title".into(),
            ty: FieldType::Opt(Box::new(FieldType::Str)),
            doc: Some(
                "The title of this NiFi instance.\nThis is configurable in nifi.properties.".into(),
            ),
            read_only: false,
        }],
        doc: Some(
            "Information about this NiFi instance.\nProvides version and build details.".into(),
        ),
    }]);
    let out = all_output(&spec);
    assert!(
        out.contains("/// The title of this NiFi instance."),
        "missing first doc line: {out}"
    );
    assert!(
        out.contains("/// This is configurable in nifi.properties."),
        "missing second field doc line: {out}"
    );
    assert!(
        out.contains("/// Information about this NiFi instance."),
        "missing first struct doc line: {out}"
    );
    assert!(
        out.contains("/// Provides version and build details."),
        "missing second struct doc line: {out}"
    );
}

#[test]
fn emit_read_only_field_has_doc_annotation() {
    let spec = spec_with_types(vec![TypeDef {
        name: "AboutDto".into(),
        kind: TypeKind::Dto,
        fields: vec![
            Field {
                rust_name: "build_tag".into(),
                serde_name: "buildTag".into(),
                ty: FieldType::Opt(Box::new(FieldType::Str)),
                doc: Some("The build tag.".into()),
                read_only: true,
            },
            Field {
                rust_name: "version".into(),
                serde_name: "version".into(),
                ty: FieldType::Opt(Box::new(FieldType::Str)),
                doc: None,
                read_only: false,
            },
        ],
        doc: None,
    }]);
    let out = all_output(&spec);

    // read-only field gets "Read-only" annotation in doc
    assert!(out.contains("Read-only"), "missing Read-only annotation: {out}");
    // read-only fields are NOT skip_serializing (NiFi requires them in update requests)
    assert!(
        !out.contains("#[serde(skip_serializing)]"),
        "read-only fields must not have skip_serializing: {out}"
    );
}
