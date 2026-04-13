use nifi_openapi_gen::canonical::{CanonicalSpec, canonicalize, canonicalize_or_panic};
use nifi_openapi_gen::content_type::ResponseBodyKind;
use nifi_openapi_gen::non_additive_detector::{NonAdditiveChange, check};
use nifi_openapi_gen::non_additive_overrides::{NonAdditiveOverride, NonAdditiveOverrides};
use nifi_openapi_gen::parser::{
    ApiSpec, Endpoint, Field, FieldType, HttpMethod, TagGroup, TypeDef, TypeKind, load,
};

#[test]
fn detector_on_empty_canonical_returns_empty() {
    let canonical = CanonicalSpec::new();
    let spec = ApiSpec {
        tags: vec![],
        all_types: vec![],
    };
    let changes: Vec<NonAdditiveChange> = check(&canonical, "2.6.0", &spec);
    assert!(changes.is_empty());
}

fn make_endpoint(method: HttpMethod, path: &str, op_id: &str) -> Endpoint {
    Endpoint {
        method,
        path: path.to_string(),
        fn_name: "stub".to_string(),
        raw_operation_id: op_id.to_string(),
        doc: None,
        description: None,
        path_params: vec![],
        request_type: None,
        body_kind: None,
        body_doc: None,
        response_type: None,
        response_inner: None,
        response_field: None,
        response_kind: ResponseBodyKind::Empty,
        query_params: vec![],
        success_responses: vec![],
        error_responses: vec![],
        security: None,
    }
}

fn make_spec_with_endpoints(eps: Vec<Endpoint>) -> ApiSpec {
    ApiSpec {
        tags: vec![TagGroup {
            tag: "Flow".to_string(),
            struct_name: "FlowApi".to_string(),
            module_name: "flow".to_string(),
            accessor_fn: "flow_api".to_string(),
            types: vec![],
            root_endpoints: eps,
            sub_groups: vec![],
        }],
        all_types: vec![],
    }
}

#[test]
fn detects_endpoint_removed_in_next_version() {
    let spec_a = make_spec_with_endpoints(vec![make_endpoint(
        HttpMethod::Get,
        "/flow/about",
        "getAboutInfo",
    )]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a)]);

    let spec_b = make_spec_with_endpoints(vec![]);
    let changes = check(&canonical, "2.7.2", &spec_b);

    assert_eq!(changes.len(), 1);
    match &changes[0] {
        NonAdditiveChange::EndpointRemoved {
            method,
            path,
            previous_versions,
            missing_in,
        } => {
            assert_eq!(*method, HttpMethod::Get);
            assert_eq!(path, "/flow/about");
            assert_eq!(previous_versions, &vec!["2.6.0".to_string()]);
            assert_eq!(missing_in, "2.7.2");
        }
        other => panic!("expected EndpointRemoved, got {other:?}"),
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

fn make_type(name: &str, fields: Vec<Field>) -> TypeDef {
    TypeDef {
        name: name.to_string(),
        kind: TypeKind::Dto,
        fields,
        doc: None,
    }
}

fn make_spec_with_types(types: Vec<TypeDef>) -> ApiSpec {
    ApiSpec {
        tags: vec![],
        all_types: types,
    }
}

#[test]
fn does_not_report_added_endpoint() {
    let spec_a = make_spec_with_endpoints(vec![make_endpoint(
        HttpMethod::Get,
        "/flow/about",
        "getAboutInfo",
    )]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a)]);

    let spec_b = make_spec_with_endpoints(vec![
        make_endpoint(HttpMethod::Get, "/flow/about", "getAboutInfo"),
        make_endpoint(HttpMethod::Get, "/connectors", "listConnectors"),
    ]);
    let changes = check(&canonical, "2.9.0", &spec_b);
    assert!(changes.is_empty());
}

#[test]
fn detects_type_removed_in_next_version() {
    let spec_a = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a)]);

    let spec_b = make_spec_with_types(vec![]);
    let changes = check(&canonical, "2.7.2", &spec_b);

    assert_eq!(changes.len(), 1);
    match &changes[0] {
        NonAdditiveChange::TypeRemoved {
            type_name,
            previous_versions,
            missing_in,
        } => {
            assert_eq!(type_name, "AboutDto");
            assert_eq!(previous_versions, &vec!["2.6.0".to_string()]);
            assert_eq!(missing_in, "2.7.2");
        }
        other => panic!("expected TypeRemoved, got {other:?}"),
    }
}

#[test]
fn detects_field_removed_from_existing_type() {
    let spec_a = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![
            make_field("title", FieldType::Str),
            make_field("build_tag", FieldType::Str),
        ],
    )]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a)]);

    let spec_b = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let changes = check(&canonical, "2.7.2", &spec_b);

    assert_eq!(changes.len(), 1);
    match &changes[0] {
        NonAdditiveChange::FieldRemoved {
            type_name,
            field,
            previous_versions,
            missing_in,
        } => {
            assert_eq!(type_name, "AboutDto");
            assert_eq!(field, "build_tag");
            assert_eq!(previous_versions, &vec!["2.6.0".to_string()]);
            assert_eq!(missing_in, "2.7.2");
        }
        other => panic!("expected FieldRemoved, got {other:?}"),
    }
}

#[test]
fn type_removed_suppresses_field_removed_for_same_type() {
    let spec_a = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a)]);

    let spec_b = make_spec_with_types(vec![]);
    let changes = check(&canonical, "2.7.2", &spec_b);

    // Only one TypeRemoved; no dangling FieldRemoved for AboutDto.title.
    assert_eq!(changes.len(), 1);
    assert!(matches!(changes[0], NonAdditiveChange::TypeRemoved { .. }));
}

#[test]
fn detects_field_type_changed_in_next_version() {
    let spec_a = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("build_revision", FieldType::Str)],
    )]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a)]);

    let spec_b = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("build_revision", FieldType::I64)],
    )]);
    let changes = check(&canonical, "2.7.2", &spec_b);

    assert_eq!(changes.len(), 1);
    match &changes[0] {
        NonAdditiveChange::FieldTypeChanged {
            type_name,
            field,
            from,
            to,
            previous_versions,
            changed_in,
        } => {
            assert_eq!(type_name, "AboutDto");
            assert_eq!(field, "build_revision");
            assert_eq!(*from, FieldType::Str);
            assert_eq!(*to, FieldType::I64);
            assert_eq!(previous_versions, &vec!["2.6.0".to_string()]);
            assert_eq!(changed_in, "2.7.2");
        }
        other => panic!("expected FieldTypeChanged, got {other:?}"),
    }
}

#[test]
fn same_field_type_is_not_reported() {
    let spec_a = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a)]);

    let spec_b = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let changes = check(&canonical, "2.7.2", &spec_b);
    assert!(changes.is_empty());
}

fn make_enum_type(name: &str, variants: Vec<&str>) -> TypeDef {
    TypeDef {
        name: name.to_string(),
        kind: TypeKind::StringEnum(variants.into_iter().map(String::from).collect()),
        fields: vec![],
        doc: None,
    }
}

#[test]
fn detects_enum_variant_removed() {
    let spec_a = make_spec_with_types(vec![make_enum_type(
        "ComponentType",
        vec!["PROCESSOR", "CONTROLLER_SERVICE"],
    )]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a)]);

    let spec_b = make_spec_with_types(vec![make_enum_type("ComponentType", vec!["PROCESSOR"])]);
    let changes = check(&canonical, "2.7.2", &spec_b);

    assert_eq!(changes.len(), 1);
    match &changes[0] {
        NonAdditiveChange::EnumVariantRemoved {
            enum_name,
            variant,
            previous_versions,
            missing_in,
        } => {
            assert_eq!(enum_name, "ComponentType");
            assert_eq!(variant, "CONTROLLER_SERVICE");
            assert_eq!(previous_versions, &vec!["2.6.0".to_string()]);
            assert_eq!(missing_in, "2.7.2");
        }
        other => panic!("expected EnumVariantRemoved, got {other:?}"),
    }
}

#[test]
fn added_enum_variant_is_not_reported() {
    let spec_a = make_spec_with_types(vec![make_enum_type("ComponentType", vec!["PROCESSOR"])]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a)]);

    let spec_b = make_spec_with_types(vec![make_enum_type(
        "ComponentType",
        vec!["PROCESSOR", "CONNECTOR"],
    )]);
    let changes = check(&canonical, "2.9.0", &spec_b);
    assert!(changes.is_empty());
}

#[test]
fn overrides_silence_matching_change() {
    let change = NonAdditiveChange::FieldRemoved {
        type_name: "AboutDto".to_string(),
        field: "build_tag".to_string(),
        previous_versions: vec!["2.6.0".to_string()],
        missing_in: "2.7.2".to_string(),
    };

    let empty = NonAdditiveOverrides::empty();
    assert!(!empty.allows(&change));

    let mut overrides = NonAdditiveOverrides::empty();
    overrides.allow(NonAdditiveOverride::FieldRemoved {
        type_name: "AboutDto".to_string(),
        field: "build_tag".to_string(),
        reason: "field was deprecated in 2.7 and server stopped sending it".to_string(),
    });
    assert!(overrides.allows(&change));
}

#[test]
fn overrides_do_not_match_different_location() {
    let change = NonAdditiveChange::FieldRemoved {
        type_name: "AboutDto".to_string(),
        field: "build_tag".to_string(),
        previous_versions: vec!["2.6.0".to_string()],
        missing_in: "2.7.2".to_string(),
    };

    let mut overrides = NonAdditiveOverrides::empty();
    overrides.allow(NonAdditiveOverride::FieldRemoved {
        type_name: "AboutDto".to_string(),
        field: "some_other_field".to_string(),
        reason: "unrelated".to_string(),
    });
    assert!(!overrides.allows(&change));
}

#[test]
fn panic_message_for_field_removed_mentions_rule_and_location() {
    let change = NonAdditiveChange::FieldRemoved {
        type_name: "AboutDto".to_string(),
        field: "build_tag".to_string(),
        previous_versions: vec!["2.6.0".to_string(), "2.7.2".to_string()],
        missing_in: "2.8.0".to_string(),
    };
    let msg = change.panic_message("specs/2.8.0/nifi-api.json");
    assert!(msg.contains("FieldRemoved"));
    assert!(msg.contains("AboutDto.build_tag"));
    assert!(msg.contains("2.6.0, 2.7.2"));
    assert!(msg.contains("2.8.0"));
    assert!(msg.contains("specs/2.8.0/nifi-api.json"));
    assert!(msg.contains("non_additive_overrides.rs"));
}

#[test]
fn canonicalize_or_panic_passes_on_additive_chain() {
    let spec_a = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let spec_b = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![
            make_field("title", FieldType::Str),
            make_field("version", FieldType::Str),
        ],
    )]);
    let canonical = canonicalize_or_panic(
        &[("2.6.0".to_string(), spec_a), ("2.7.2".to_string(), spec_b)],
        |_| "<test>".to_string(),
        &NonAdditiveOverrides::empty(),
    );
    assert_eq!(canonical.types.get("AboutDto").unwrap().fields.len(), 2);
}

#[test]
#[should_panic(expected = "FieldRemoved")]
fn canonicalize_or_panic_panics_on_non_additive_change() {
    let spec_a = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![
            make_field("title", FieldType::Str),
            make_field("build_tag", FieldType::Str),
        ],
    )]);
    let spec_b = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    canonicalize_or_panic(
        &[("2.6.0".to_string(), spec_a), ("2.7.2".to_string(), spec_b)],
        |_| "<test>".to_string(),
        &NonAdditiveOverrides::empty(),
    );
}

#[test]
fn inline_enum_variant_addition_is_additive() {
    let spec_a = make_spec_with_types(vec![make_type(
        "ProcessorDto",
        vec![make_field(
            "state",
            FieldType::Opt(Box::new(FieldType::Enum(vec![
                "RUNNING".to_string(),
                "STOPPED".to_string(),
            ]))),
        )],
    )]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a)]);

    let spec_b = make_spec_with_types(vec![make_type(
        "ProcessorDto",
        vec![make_field(
            "state",
            FieldType::Opt(Box::new(FieldType::Enum(vec![
                "RUNNING".to_string(),
                "STOPPED".to_string(),
                "RUN_ONCE".to_string(),
            ]))),
        )],
    )]);
    let changes = check(&canonical, "2.8.0", &spec_b);
    assert!(
        changes.is_empty(),
        "inline enum growth should be additive, got: {changes:?}"
    );
}

#[test]
fn inline_enum_variant_removal_still_reports_field_type_changed() {
    let spec_a = make_spec_with_types(vec![make_type(
        "ProcessorDto",
        vec![make_field(
            "state",
            FieldType::Enum(vec!["RUNNING".to_string(), "STOPPED".to_string()]),
        )],
    )]);
    let canonical = canonicalize(&[("2.6.0".to_string(), spec_a)]);

    let spec_b = make_spec_with_types(vec![make_type(
        "ProcessorDto",
        vec![make_field(
            "state",
            FieldType::Enum(vec!["RUNNING".to_string()]),
        )],
    )]);
    let changes = check(&canonical, "2.7.2", &spec_b);
    assert_eq!(changes.len(), 1);
    assert!(matches!(
        changes[0],
        NonAdditiveChange::FieldTypeChanged { .. }
    ));
}

#[test]
fn real_spec_chain_passes_non_additive_detector() {
    let specs_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("specs");
    let versions = ["2.6.0", "2.7.2", "2.8.0", "2.9.0"];
    let all_parsed: Vec<(String, ApiSpec)> = versions
        .iter()
        .map(|v| {
            let p = specs_dir.join(v).join("nifi-api.json");
            (v.to_string(), load(p.to_str().unwrap()))
        })
        .collect();

    // This MUST pass. If it starts failing, a real non-additive change
    // landed in one of the shipped specs and needs a human decision.
    let canonical = canonicalize_or_panic(
        &all_parsed,
        |v| format!("crates/nifi-openapi-gen/specs/{v}/nifi-api.json"),
        &NonAdditiveOverrides::empty(),
    );

    // Sanity check — at least as many endpoints as the largest single spec.
    assert!(canonical.endpoints.len() >= 237);
}

#[test]
fn canonicalize_or_panic_respects_overrides() {
    let spec_a = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![
            make_field("title", FieldType::Str),
            make_field("build_tag", FieldType::Str),
        ],
    )]);
    let spec_b = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let mut overrides = NonAdditiveOverrides::empty();
    overrides.allow(NonAdditiveOverride::FieldRemoved {
        type_name: "AboutDto".to_string(),
        field: "build_tag".to_string(),
        reason: "deprecated, dropped server-side in 2.7".to_string(),
    });
    let canonical = canonicalize_or_panic(
        &[("2.6.0".to_string(), spec_a), ("2.7.2".to_string(), spec_b)],
        |_| "<test>".to_string(),
        &overrides,
    );
    // build_tag is still in canonical (it got merged in 2.6.0); the override
    // silences the detector but does not rewrite history.
    assert!(
        canonical
            .types
            .get("AboutDto")
            .unwrap()
            .fields
            .contains_key("build_tag")
    );
}

#[test]
fn canonicalize_or_panic_populates_per_version_specs() {
    use nifi_openapi_gen::canonical::{canonicalize_or_panic, project};
    use nifi_openapi_gen::non_additive_overrides::NonAdditiveOverrides;

    let spec_a = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![make_field("title", FieldType::Str)],
    )]);
    let spec_b = make_spec_with_types(vec![make_type(
        "AboutDto",
        vec![
            make_field("title", FieldType::Str),
            make_field("version", FieldType::Str),
        ],
    )]);

    let canonical = canonicalize_or_panic(
        &[("2.6.0".to_string(), spec_a), ("2.7.2".to_string(), spec_b)],
        |v| format!("specs/{v}/nifi-api.json"),
        &NonAdditiveOverrides::empty(),
    );

    assert_eq!(canonical.per_version_specs.len(), 2);
    assert!(project(&canonical, "2.6.0").is_some());
    assert!(project(&canonical, "2.7.2").is_some());

    // Per-version field counts confirm the stored specs are distinct, not shared.
    assert_eq!(
        project(&canonical, "2.6.0").unwrap().all_types[0]
            .fields
            .len(),
        1
    );
    assert_eq!(
        project(&canonical, "2.7.2").unwrap().all_types[0]
            .fields
            .len(),
        2
    );
}
