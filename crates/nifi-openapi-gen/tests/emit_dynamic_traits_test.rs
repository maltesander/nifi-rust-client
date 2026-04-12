use nifi_openapi_gen::emit::emit_dynamic_traits;
use nifi_openapi_gen::parser::{
    ApiSpec, Endpoint, HttpMethod, PathParam, QueryParam, QueryParamType, RequestBodyKind, TagGroup,
};

fn make_spec(tag: TagGroup) -> ApiSpec {
    ApiSpec {
        all_types: vec![],
        tags: vec![tag],
    }
}

fn make_flow_tag(endpoints: Vec<Endpoint>) -> TagGroup {
    TagGroup {
        tag: "Flow".into(),
        struct_name: "FlowApi".into(),
        module_name: "flow".into(),
        accessor_fn: "flow_api".into(),
        types: vec![],
        root_endpoints: endpoints,
        sub_groups: vec![],
    }
}

fn about_endpoint() -> Endpoint {
    Endpoint {
        method: HttpMethod::Get,
        path: "/flow/about".into(),
        fn_name: "get_about_info".into(),
        raw_operation_id: String::new(),
        doc: Some("Retrieves details about this NiFi to put in the About dialog.".into()),
        description: None,
        path_params: vec![],
        request_type: None,
        body_doc: None,
        body_kind: None,
        response_type: Some("AboutEntity".into()),
        response_inner: Some("AboutDto".into()),
        response_field: Some("about".into()),
        response_kind: nifi_openapi_gen::content_type::ResponseBodyKind::Json {
            schema_ref: "AboutEntity".into(),
        },
        query_params: vec![],
        success_responses: vec![],
        error_responses: vec![
            ("401".into(), "Client could not be authenticated.".into()),
            (
                "403".into(),
                "Client is not authorized to make this request.".into(),
            ),
        ],
        security: Some(vec!["Read - /flow".into()]),
    }
}

#[test]
fn emits_trait_with_doc_and_default_impl() {
    let spec = make_spec(make_flow_tag(vec![about_endpoint()]));
    let versions: Vec<(&str, &str, &str, &ApiSpec)> =
        vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];

    let files = emit_dynamic_traits(&versions);

    // Should produce mod.rs + flow.rs
    assert_eq!(files.len(), 2);
    assert_eq!(files[0].0, "mod.rs");
    assert_eq!(files[1].0, "flow.rs");

    let mod_rs = &files[0].1;
    assert!(
        mod_rs.contains("mod flow;"),
        "mod.rs should declare mod flow"
    );
    assert!(
        mod_rs.contains("pub use flow::FlowApi;"),
        "mod.rs should re-export FlowApi"
    );

    let flow_rs = &files[1].1;

    // Trait name
    assert!(
        flow_rs.contains("pub trait FlowApi"),
        "Should define FlowApi trait: {flow_rs}"
    );

    // Doc comment
    assert!(
        flow_rs.contains("Retrieves details about this NiFi"),
        "Should have method doc: {flow_rs}"
    );

    // HTTP call doc
    assert!(
        flow_rs.contains("Calls `GET /nifi-api/flow/about`"),
        "Should have HTTP call doc: {flow_rs}"
    );

    // Error docs
    assert!(
        flow_rs.contains("# Errors"),
        "Should have Errors heading: {flow_rs}"
    );
    assert!(
        flow_rs.contains("`401`: Client could not be authenticated."),
        "Should have 401 error: {flow_rs}"
    );
    assert!(
        flow_rs.contains("`403`: Client is not authorized"),
        "Should have 403 error: {flow_rs}"
    );

    // Permissions
    assert!(
        flow_rs.contains("# Permissions"),
        "Should have Permissions heading: {flow_rs}"
    );
    assert!(
        flow_rs.contains("Requires `Read - /flow`."),
        "Should have permission line: {flow_rs}"
    );

    // Default impl with UnsupportedEndpoint
    assert!(
        flow_rs.contains("UnsupportedEndpoint"),
        "Should have UnsupportedEndpoint default: {flow_rs}"
    );
    assert!(
        flow_rs.contains("\"get_about_info\""),
        "Should reference fn name in error: {flow_rs}"
    );

    // Return type uses types::
    assert!(
        flow_rs.contains("types::AboutDto"),
        "Should use types:: prefix for return type: {flow_rs}"
    );

    // Should use crate::NifiError
    assert!(
        flow_rs.contains("use crate::NifiError"),
        "Should import NifiError: {flow_rs}"
    );

    // Should NOT have version availability note (all versions have it)
    assert!(
        !flow_rs.contains("Supported in NiFi"),
        "Should not have version note when all versions have endpoint: {flow_rs}"
    );
}

#[test]
fn trait_includes_version_specific_endpoints() {
    let extra_endpoint = Endpoint {
        method: HttpMethod::Get,
        path: "/flow/metrics".into(),
        fn_name: "get_flow_metrics".into(),
        raw_operation_id: String::new(),
        doc: Some("Gets flow metrics.".into()),
        description: None,
        path_params: vec![],
        request_type: None,
        body_doc: None,
        body_kind: None,
        response_type: Some("FlowMetricsEntity".into()),
        response_inner: None,
        response_field: None,
        response_kind: nifi_openapi_gen::content_type::ResponseBodyKind::Json {
            schema_ref: "FlowMetricsEntity".into(),
        },
        query_params: vec![],
        success_responses: vec![],
        error_responses: vec![],
        security: None,
    };

    let spec_v1 = make_spec(make_flow_tag(vec![about_endpoint()]));
    let spec_v2 = make_spec(make_flow_tag(vec![about_endpoint(), extra_endpoint]));

    let versions: Vec<(&str, &str, &str, &ApiSpec)> = vec![
        ("2.7.2", "v2_7_2", "nifi-2-7-2", &spec_v1),
        ("2.8.0", "v2_8_0", "nifi-2-8-0", &spec_v2),
    ];

    let files = emit_dynamic_traits(&versions);
    let flow_rs = &files[1].1;

    // Both endpoints should be in the trait
    assert!(
        flow_rs.contains("fn get_about_info"),
        "Should have get_about_info: {flow_rs}"
    );
    assert!(
        flow_rs.contains("fn get_flow_metrics"),
        "Should have get_flow_metrics: {flow_rs}"
    );

    // get_about_info exists in both versions, should NOT have version note
    // (search between the two method definitions)
    assert!(
        !flow_rs.contains("Retrieves details about this NiFi")
            || !flow_rs.contains("*Supported in NiFi: 2.7.2, 2.8.0*"),
        "get_about_info should not have version note since it's in all versions"
    );

    // get_flow_metrics only exists in v2 — should have version note
    assert!(
        flow_rs.contains("*Supported in NiFi: 2.8.0*"),
        "get_flow_metrics should note it's only in 2.8.0: {flow_rs}"
    );

    // Return type for get_flow_metrics should use types::
    assert!(
        flow_rs.contains("types::FlowMetricsEntity"),
        "Should use types:: for FlowMetricsEntity: {flow_rs}"
    );
}

#[test]
fn trait_method_with_path_and_query_params() {
    let ep = Endpoint {
        method: HttpMethod::Get,
        path: "/processors/{id}".into(),
        fn_name: "get_processor".into(),
        raw_operation_id: String::new(),
        doc: Some("Gets a processor.".into()),
        description: None,
        path_params: vec![PathParam {
            name: "id".into(),
            doc: Some("The processor id.".into()),
        }],
        request_type: None,
        body_doc: None,
        body_kind: None,
        response_type: Some("ProcessorEntity".into()),
        response_inner: Some("ProcessorDto".into()),
        response_field: Some("component".into()),
        response_kind: nifi_openapi_gen::content_type::ResponseBodyKind::Json {
            schema_ref: "ProcessorEntity".into(),
        },
        query_params: vec![QueryParam {
            name: "includeDescendants".into(),
            rust_name: "include_descendants".into(),
            ty: QueryParamType::Bool,
            required: false,
            doc: Some("Whether to include descendant groups.".into()),
            enum_type_name: None,
        }],
        success_responses: vec![],
        error_responses: vec![],
        security: None,
    };

    let tag = TagGroup {
        tag: "Processors".into(),
        struct_name: "ProcessorsApi".into(),
        module_name: "processors".into(),
        accessor_fn: "processors_api".into(),
        types: vec![],
        root_endpoints: vec![ep],
        sub_groups: vec![],
    };
    let spec = make_spec(tag);
    let versions: Vec<(&str, &str, &str, &ApiSpec)> =
        vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];

    let files = emit_dynamic_traits(&versions);
    let proc_rs = &files[1].1;

    // Should have path param
    assert!(
        proc_rs.contains("id: &str"),
        "Should have path param: {proc_rs}"
    );

    // Should have query param as Option<bool>
    assert!(
        proc_rs.contains("include_descendants: Option<bool>"),
        "Should have optional query param: {proc_rs}"
    );

    // Should have Parameters section
    assert!(
        proc_rs.contains("# Parameters"),
        "Should have Parameters heading: {proc_rs}"
    );
    assert!(
        proc_rs.contains("`id`: The processor id."),
        "Should have param doc: {proc_rs}"
    );
    assert!(
        proc_rs.contains("`include_descendants`: Whether to include descendant groups."),
        "Should have query param doc: {proc_rs}"
    );
}

#[test]
fn trait_method_with_json_body() {
    let ep = Endpoint {
        method: HttpMethod::Post,
        path: "/processors".into(),
        fn_name: "create_processor".into(),
        raw_operation_id: String::new(),
        doc: Some("Creates a processor.".into()),
        description: None,
        path_params: vec![],
        request_type: Some("ProcessorEntity".into()),
        body_doc: None,
        body_kind: Some(RequestBodyKind::Json),
        response_type: Some("ProcessorEntity".into()),
        response_inner: Some("ProcessorDto".into()),
        response_field: Some("component".into()),
        response_kind: nifi_openapi_gen::content_type::ResponseBodyKind::Json {
            schema_ref: "ProcessorEntity".into(),
        },
        query_params: vec![],
        success_responses: vec![],
        error_responses: vec![],
        security: None,
    };

    let tag = TagGroup {
        tag: "Processors".into(),
        struct_name: "ProcessorsApi".into(),
        module_name: "processors".into(),
        accessor_fn: "processors_api".into(),
        types: vec![],
        root_endpoints: vec![ep],
        sub_groups: vec![],
    };
    let spec = make_spec(tag);
    let versions: Vec<(&str, &str, &str, &ApiSpec)> =
        vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];

    let files = emit_dynamic_traits(&versions);
    let proc_rs = &files[1].1;

    // Should have body param with types:: prefix
    assert!(
        proc_rs.contains("body: &types::ProcessorEntity"),
        "Should have borrowed body param: {proc_rs}"
    );
}

#[test]
fn trait_skips_form_encoded_endpoints() {
    let ep = Endpoint {
        method: HttpMethod::Post,
        path: "/access/token".into(),
        fn_name: "create_access_token".into(),
        raw_operation_id: String::new(),
        doc: Some("Creates a token.".into()),
        description: None,
        path_params: vec![],
        request_type: None,
        body_doc: None,
        body_kind: Some(RequestBodyKind::FormEncoded),
        response_type: None,
        response_inner: None,
        response_field: None,
        response_kind: nifi_openapi_gen::content_type::ResponseBodyKind::Empty,
        query_params: vec![],
        success_responses: vec![],
        error_responses: vec![],
        security: None,
    };

    let tag = TagGroup {
        tag: "Access".into(),
        struct_name: "AccessApi".into(),
        module_name: "access".into(),
        accessor_fn: "access_api".into(),
        types: vec![],
        root_endpoints: vec![ep],
        sub_groups: vec![],
    };
    let spec = make_spec(tag);
    let versions: Vec<(&str, &str, &str, &ApiSpec)> =
        vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];

    let files = emit_dynamic_traits(&versions);
    let access_rs = &files[1].1;

    // Should NOT contain the form-encoded method
    assert!(
        !access_rs.contains("create_access_token"),
        "Should skip form-encoded endpoints: {access_rs}"
    );
}

#[test]
fn trait_method_void_return() {
    let ep = Endpoint {
        method: HttpMethod::Delete,
        path: "/processors/{id}".into(),
        fn_name: "delete_processor".into(),
        raw_operation_id: String::new(),
        doc: Some("Deletes a processor.".into()),
        description: None,
        path_params: vec![PathParam {
            name: "id".into(),
            doc: None,
        }],
        request_type: None,
        body_doc: None,
        body_kind: None,
        response_type: None,
        response_inner: None,
        response_field: None,
        response_kind: nifi_openapi_gen::content_type::ResponseBodyKind::Empty,
        query_params: vec![],
        success_responses: vec![],
        error_responses: vec![],
        security: None,
    };

    let tag = TagGroup {
        tag: "Processors".into(),
        struct_name: "ProcessorsApi".into(),
        module_name: "processors".into(),
        accessor_fn: "processors_api".into(),
        types: vec![],
        root_endpoints: vec![ep],
        sub_groups: vec![],
    };
    let spec = make_spec(tag);
    let versions: Vec<(&str, &str, &str, &ApiSpec)> =
        vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];

    let files = emit_dynamic_traits(&versions);
    let proc_rs = &files[1].1;

    // Return type should be Result<(), NifiError>
    assert!(
        proc_rs.contains("Result<(), NifiError>"),
        "Should have void return type: {proc_rs}"
    );
}
