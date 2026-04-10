use nifi_openapi_gen::parser::{self, FieldType, QueryParamType, TypeKind, load};

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name)
}

#[test]
fn parse_scalar_fields() {
    let spec = load(&fixture("simple_dto.json"));
    let about = spec
        .all_types
        .iter()
        .find(|t| t.name == "AboutDto")
        .expect("AboutDto not found");
    assert!(matches!(about.kind, TypeKind::Dto));

    let title = about
        .fields
        .iter()
        .find(|f| f.rust_name == "title")
        .expect("title");
    assert!(
        matches!(title.ty, FieldType::Opt(ref inner) if matches!(inner.as_ref(), FieldType::Str))
    );
    assert_eq!(
        title.doc.as_deref(),
        Some("The title of this NiFi instance.\nThis is configurable in nifi.properties.")
    );

    let count = about
        .fields
        .iter()
        .find(|f| f.rust_name == "count")
        .expect("count");
    assert!(
        matches!(count.ty, FieldType::Opt(ref inner) if matches!(inner.as_ref(), FieldType::I32))
    );

    let size = about
        .fields
        .iter()
        .find(|f| f.rust_name == "size")
        .expect("size");
    assert!(
        matches!(size.ty, FieldType::Opt(ref inner) if matches!(inner.as_ref(), FieldType::I64))
    );

    let ratio = about
        .fields
        .iter()
        .find(|f| f.rust_name == "ratio")
        .expect("ratio");
    assert!(
        matches!(ratio.ty, FieldType::Opt(ref inner) if matches!(inner.as_ref(), FieldType::F64))
    );

    let active = about
        .fields
        .iter()
        .find(|f| f.rust_name == "active")
        .expect("active");
    assert!(
        matches!(active.ty, FieldType::Opt(ref inner) if matches!(inner.as_ref(), FieldType::Bool))
    );
}

#[test]
fn parse_ref_and_array_fields() {
    let spec = load(&fixture("ref_and_array.json"));
    let container = spec
        .all_types
        .iter()
        .find(|t| t.name == "ContainerDto")
        .expect("ContainerDto");

    let item = container
        .fields
        .iter()
        .find(|f| f.rust_name == "item")
        .expect("item");
    assert!(
        matches!(&item.ty, FieldType::Opt(inner) if matches!(inner.as_ref(), FieldType::Ref(n) if n == "ItemDto"))
    );

    let items = container
        .fields
        .iter()
        .find(|f| f.rust_name == "items")
        .expect("items");
    assert!(
        matches!(&items.ty, FieldType::Opt(outer) if matches!(outer.as_ref(), FieldType::List(inner) if matches!(inner.as_ref(), FieldType::Ref(n) if n == "ItemDto")))
    );

    let tags = container
        .fields
        .iter()
        .find(|f| f.rust_name == "tags")
        .expect("tags");
    assert!(
        matches!(&tags.ty, FieldType::Opt(outer) if matches!(outer.as_ref(), FieldType::List(inner) if matches!(inner.as_ref(), FieldType::Str)))
    );
}

#[test]
fn parse_entity_wrapper() {
    let spec = load(&fixture("entity_wrapper.json"));
    let entity = spec
        .all_types
        .iter()
        .find(|t| t.name == "AboutEntity")
        .expect("AboutEntity");
    match &entity.kind {
        parser::TypeKind::Entity { field, inner } => {
            assert_eq!(field, "about");
            assert_eq!(inner, "AboutDto");
        }
        _ => panic!("expected Entity kind"),
    }
    // AboutDto should also be present
    let dto = spec
        .all_types
        .iter()
        .find(|t| t.name == "AboutDto")
        .expect("AboutDto");
    assert!(matches!(dto.kind, parser::TypeKind::Dto));
}

#[test]
fn parse_inline_enum() {
    let spec = load(&fixture("inline_enum.json"));
    let dto = spec
        .all_types
        .iter()
        .find(|t| t.name == "PolicyDto")
        .expect("PolicyDto");
    let action = dto
        .fields
        .iter()
        .find(|f| f.rust_name == "action")
        .expect("action");
    match &action.ty {
        FieldType::Opt(inner) => match inner.as_ref() {
            FieldType::Enum(variants) => {
                assert_eq!(variants, &vec!["read".to_string(), "write".to_string()]);
            }
            _ => panic!("expected Enum inner type"),
        },
        _ => panic!("expected Opt(Enum(...))"),
    }
}

#[test]
fn parse_endpoints_grouped_by_tag() {
    let spec = load(&fixture("endpoints.json"));
    assert_eq!(spec.tags.len(), 3);

    let flow = spec
        .tags
        .iter()
        .find(|t| t.tag == "Flow")
        .expect("Flow tag");
    assert_eq!(flow.struct_name, "FlowApi");
    assert_eq!(flow.module_name, "flow");
    assert_eq!(flow.accessor_fn, "flow_api");
    assert_eq!(flow.root_endpoints.len(), 1);

    let about = &flow.root_endpoints[0];
    assert_eq!(about.fn_name, "get_about_info");
    assert!(matches!(about.method, parser::HttpMethod::Get));
    assert_eq!(about.path, "/flow/about");
    assert_eq!(about.response_type.as_deref(), Some("AboutEntity"));
    assert_eq!(about.response_inner.as_deref(), Some("AboutDto"));
    assert_eq!(about.response_field.as_deref(), Some("about"));
    assert!(about.path_params.is_empty());

    let proc_tag = spec
        .tags
        .iter()
        .find(|t| t.tag == "Processors")
        .expect("Processors tag");
    let del = &proc_tag.root_endpoints[0];
    assert_eq!(del.fn_name, "delete_processor");
    assert!(matches!(del.method, parser::HttpMethod::Delete));
    assert_eq!(
        del.path_params
            .iter()
            .map(|p| p.name.as_str())
            .collect::<Vec<_>>(),
        vec!["id"]
    );
    assert_eq!(del.response_inner.as_deref(), Some("ProcessorDto"));
}

#[test]
fn parse_read_only_field() {
    let spec = load(&fixture("simple_dto.json"));
    let about = spec
        .all_types
        .iter()
        .find(|t| t.name == "AboutDto")
        .unwrap();

    let build_tag = about
        .fields
        .iter()
        .find(|f| f.rust_name == "build_tag")
        .unwrap();
    assert!(build_tag.read_only, "build_tag should be read_only");

    let title = about
        .fields
        .iter()
        .find(|f| f.rust_name == "title")
        .unwrap();
    assert!(!title.read_only, "title should not be read_only");
}

#[test]
fn load_full_nifi_spec() {
    let path = format!("{}/specs/nifi-api.json", env!("CARGO_MANIFEST_DIR"));
    // Only run if the spec file is present (it's gitignored)
    if !std::path::Path::new(&path).exists() {
        eprintln!("skipping: spec not found at {path}");
        return;
    }
    let spec = load(&path);
    assert!(
        spec.tags.len() >= 20,
        "expected at least 20 tags, got {}",
        spec.tags.len()
    );
    assert!(
        spec.all_types.len() >= 200,
        "expected at least 200 types, got {}",
        spec.all_types.len()
    );

    // Verify FlowApi exists with about endpoint
    let flow = spec
        .tags
        .iter()
        .find(|t| t.tag == "Flow")
        .expect("Flow tag missing");
    assert_eq!(flow.struct_name, "FlowApi");
    let about = flow
        .root_endpoints
        .iter()
        .chain(flow.sub_groups.iter().flat_map(|sg| sg.endpoints.iter()))
        .find(|e| e.fn_name == "get_about_info");
    assert!(
        about.is_some(),
        "get_about_info endpoint missing from Flow tag"
    );
}

#[test]
fn parse_sub_path_grouping() {
    let spec = load(&fixture("sub_paths.json"));
    let services = spec
        .tags
        .iter()
        .find(|t| t.tag == "Services")
        .expect("Services");

    // /services/{id} → root
    assert_eq!(services.root_endpoints.len(), 1);
    assert_eq!(services.root_endpoints[0].fn_name, "get_service");

    // /services/{id}/config/* → sub-group "config"
    // /services/{id}/run-status → sub-group "run_status"
    assert_eq!(
        services.sub_groups.len(),
        2,
        "expected config + run_status sub-groups"
    );

    let config = services
        .sub_groups
        .iter()
        .find(|sg| sg.accessor_fn == "config")
        .expect("config");
    assert_eq!(config.struct_name, "ServicesConfigApi");
    assert_eq!(config.primary_param, "id");
    assert_eq!(
        config.endpoints.len(),
        2,
        "expected analyze_config + get_verification"
    );

    let run_status = services
        .sub_groups
        .iter()
        .find(|sg| sg.accessor_fn == "run_status")
        .expect("run_status");
    assert_eq!(run_status.struct_name, "ServicesRunStatusApi");
    assert_eq!(run_status.endpoints.len(), 1);
}

#[test]
fn parse_query_params() {
    let spec = load(&fixture("query_params.json"));
    let flow = spec.tags.iter().find(|t| t.tag == "Flow").expect("Flow");

    // Optional string query param
    let search = flow
        .root_endpoints
        .iter()
        .find(|e| e.fn_name == "search_flow")
        .expect("search_flow");
    assert_eq!(search.query_params.len(), 1);
    assert_eq!(search.query_params[0].name, "q");
    assert_eq!(search.query_params[0].rust_name, "q");
    assert!(!search.query_params[0].required);
    assert!(matches!(search.query_params[0].ty, QueryParamType::Str));

    // Required string query param
    let cluster = flow
        .root_endpoints
        .iter()
        .find(|e| e.fn_name == "search_cluster")
        .expect("search_cluster");
    assert_eq!(cluster.query_params.len(), 1);
    assert_eq!(cluster.query_params[0].name, "q");
    assert!(cluster.query_params[0].required);

    // Two required integer query params
    let history = flow
        .root_endpoints
        .iter()
        .find(|e| e.fn_name == "query_history")
        .expect("query_history");
    assert_eq!(history.query_params.len(), 2);
    assert_eq!(history.query_params[0].name, "offset");
    assert!(history.query_params[0].required);
    assert!(matches!(history.query_params[0].ty, QueryParamType::I32));
    assert_eq!(history.query_params[1].name, "count");
    assert!(matches!(history.query_params[1].ty, QueryParamType::I32));
}

#[test]
fn query_param_enum_produces_typedef_and_type_name() {
    // The real spec has parameterContextHandlingStrategy on POST /process-groups/{id}/process-groups
    let spec_path = nifi_openapi_gen::specs_dir().join("2.8.0").join("nifi-api.json");
    let spec = load(spec_path.to_str().expect("UTF-8 spec path"));

    // Find the ProcessGroups tag
    let pg = spec
        .tags
        .iter()
        .find(|t| t.tag == "ProcessGroups")
        .expect("ProcessGroups tag");

    // Find the create_process_group endpoint
    let ep = pg
        .sub_groups
        .iter()
        .flat_map(|sg| sg.endpoints.iter())
        .chain(pg.root_endpoints.iter())
        .find(|ep| ep.fn_name == "create_process_group")
        .expect("create_process_group endpoint");

    // The query param must carry the enum type name
    let qp = ep
        .query_params
        .iter()
        .find(|qp| qp.name == "parameterContextHandlingStrategy")
        .expect("parameterContextHandlingStrategy param");
    assert_eq!(
        qp.enum_type_name.as_deref(),
        Some("ParameterContextHandlingStrategy")
    );

    // A TypeDef for this enum must exist in all_types
    let typedef = spec
        .all_types
        .iter()
        .find(|t| t.name == "ParameterContextHandlingStrategy")
        .expect("ParameterContextHandlingStrategy in all_types");
    assert!(matches!(typedef.kind, TypeKind::StringEnum(_)));
    if let TypeKind::StringEnum(variants) = &typedef.kind {
        assert_eq!(variants, &["KEEP_EXISTING", "REPLACE"]);
    }
}

#[test]
fn parse_error_responses() {
    let spec = load(&fixture("endpoints.json"));
    let tags = &spec.tags;
    let processors = tags.iter().find(|t| t.tag == "Processors").unwrap();
    let delete_ep = processors
        .root_endpoints
        .iter()
        .find(|e| e.fn_name == "delete_processor")
        .unwrap();

    assert_eq!(delete_ep.error_responses.len(), 5);
    let codes: Vec<&str> = delete_ep
        .error_responses
        .iter()
        .map(|(c, _)| c.as_str())
        .collect();
    assert!(codes.contains(&"400"), "expected 400: {codes:?}");
    assert!(codes.contains(&"401"));
    assert!(codes.contains(&"403"));
    assert!(codes.contains(&"404"));
    assert!(codes.contains(&"409"));
    let bad_req = delete_ep
        .error_responses
        .iter()
        .find(|(c, _)| c == "400")
        .unwrap();
    assert!(
        bad_req.1.contains("invalid"),
        "unexpected desc: {}",
        bad_req.1
    );
}

#[test]
fn parse_security_single() {
    let spec = load(&fixture("endpoints.json"));
    let flow = spec.tags.iter().find(|t| t.tag == "Flow").unwrap();
    let about_ep = flow
        .root_endpoints
        .iter()
        .find(|e| e.fn_name == "get_about_info")
        .unwrap();

    let sec = about_ep.security.as_ref().expect("security should be Some");
    assert_eq!(sec, &["Read - /flow"]);
}

#[test]
fn parse_security_multiple() {
    let spec = load(&fixture("endpoints.json"));
    let processors = spec.tags.iter().find(|t| t.tag == "Processors").unwrap();
    let delete_ep = processors
        .root_endpoints
        .iter()
        .find(|e| e.fn_name == "delete_processor")
        .unwrap();

    let sec = delete_ep
        .security
        .as_ref()
        .expect("security should be Some");
    assert_eq!(sec.len(), 2);
    assert!(
        sec.iter().any(|s| s.contains("processors")),
        "missing processor policy: {sec:?}"
    );
    assert!(
        sec.iter().any(|s| s.contains("process-groups")),
        "missing pg policy: {sec:?}"
    );
}

#[test]
fn parse_security_no_auth_required() {
    let spec = load(&fixture("endpoints.json"));
    let access = spec.tags.iter().find(|t| t.tag == "Access").unwrap();
    let token_ep = access
        .root_endpoints
        .iter()
        .find(|e| e.fn_name == "create_access_token")
        .unwrap();

    let sec = token_ep
        .security
        .as_ref()
        .expect("security should be Some([])");
    assert!(
        sec.is_empty(),
        "expected empty vec for no-auth, got: {sec:?}"
    );
}
