use nifi_openapi_gen::emit::emit_dynamic_impls;
use nifi_openapi_gen::parser::{
    ApiSpec, Endpoint, HttpMethod, PathParam, QueryParam, QueryParamType, RequestBodyKind,
    SubGroup, TagGroup,
};

fn make_spec(tags: Vec<TagGroup>) -> ApiSpec {
    ApiSpec {
        all_types: vec![],
        tags,
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
        doc: Some("Retrieves details about this NiFi.".into()),
        description: None,
        path_params: vec![],
        request_type: None,
        body_doc: None,
        body_kind: None,
        response_type: Some("AboutEntity".into()),
        response_inner: Some("AboutDto".into()),
        response_field: Some("about".into()),
        query_params: vec![],
        success_responses: vec![],
        error_responses: vec![],
        security: None,
    }
}

fn void_endpoint() -> Endpoint {
    Endpoint {
        method: HttpMethod::Post,
        path: "/flow/generate".into(),
        fn_name: "generate_client_id".into(),
        doc: None,
        description: None,
        path_params: vec![],
        request_type: None,
        body_doc: None,
        body_kind: None,
        response_type: None,
        response_inner: None,
        response_field: None,
        query_params: vec![],
        success_responses: vec![],
        error_responses: vec![],
        security: None,
    }
}

fn body_endpoint() -> Endpoint {
    Endpoint {
        method: HttpMethod::Put,
        path: "/processors/{id}".into(),
        fn_name: "update_processor".into(),
        doc: Some("Updates a processor.".into()),
        description: None,
        path_params: vec![PathParam {
            name: "id".into(),
            doc: None,
        }],
        request_type: Some("ProcessorEntity".into()),
        body_doc: None,
        body_kind: Some(RequestBodyKind::Json),
        response_type: Some("ProcessorEntity".into()),
        response_inner: None,
        response_field: None,
        query_params: vec![],
        success_responses: vec![],
        error_responses: vec![],
        security: None,
    }
}

fn sub_group_tag() -> TagGroup {
    TagGroup {
        tag: "ProcessGroups".into(),
        struct_name: "ProcessGroupsApi".into(),
        module_name: "process_groups".into(),
        accessor_fn: "process_groups_api".into(),
        types: vec![],
        root_endpoints: vec![],
        sub_groups: vec![SubGroup {
            name: "connections".into(),
            struct_name: "ProcessGroupConnectionsApi".into(),
            accessor_fn: "connections".into(),
            primary_param: "id".into(),
            primary_param_doc: None,
            endpoints: vec![Endpoint {
                method: HttpMethod::Get,
                path: "/process-groups/{id}/connections".into(),
                fn_name: "get_connections".into(),
                doc: Some("Gets all connections.".into()),
                description: None,
                path_params: vec![PathParam {
                    name: "id".into(),
                    doc: None,
                }],
                request_type: None,
                body_doc: None,
                body_kind: None,
                response_type: Some("ConnectionsEntity".into()),
                response_inner: None,
                response_field: None,
                query_params: vec![],
                success_responses: vec![],
                error_responses: vec![],
                security: None,
            }],
        }],
    }
}

/// Helper to find a file by path prefix in the emitted files.
fn find_file<'a>(files: &'a [(String, String)], path: &str) -> Option<&'a str> {
    files
        .iter()
        .find(|(p, _)| p == path)
        .map(|(_, c)| c.as_str())
}

#[test]
fn emits_per_version_trait_impl() {
    let spec_v1 = make_spec(vec![make_flow_tag(vec![about_endpoint()])]);
    let spec_v2 = make_spec(vec![make_flow_tag(vec![about_endpoint()])]);

    let versions: Vec<(&str, &str, &str, &ApiSpec)> = vec![
        ("2.7.2", "v2_7_2", "nifi-2-7-2", &spec_v1),
        ("2.8.0", "v2_8_0", "nifi-2-8-0", &spec_v2),
    ];

    let files = emit_dynamic_impls(&versions);

    // Top-level mod.rs should declare both version modules
    let mod_rs = find_file(&files, "mod.rs").expect("should have mod.rs");
    assert!(
        mod_rs.contains("pub(crate) mod v2_7_2;"),
        "mod.rs should declare v2_7_2: {mod_rs}"
    );
    assert!(
        mod_rs.contains("pub(crate) mod v2_8_0;"),
        "mod.rs should declare v2_8_0: {mod_rs}"
    );

    // v2_8_0/mod.rs should declare flow module and re-export wrapper struct
    let v280_mod = find_file(&files, "v2_8_0/mod.rs").expect("should have v2_8_0/mod.rs");
    assert!(
        v280_mod.contains("pub(crate) mod flow;"),
        "version mod.rs should declare flow module: {v280_mod}"
    );
    assert!(
        v280_mod.contains("pub(crate) use flow::V2_8_0FlowApi;"),
        "version mod.rs should re-export wrapper struct: {v280_mod}"
    );

    // v2_8_0/flow.rs should contain the wrapper struct and trait impl
    let flow_rs = find_file(&files, "v2_8_0/flow.rs").expect("should have v2_8_0/flow.rs");
    assert!(
        flow_rs.contains("pub(crate) struct V2_8_0FlowApi<'a>"),
        "should define wrapper struct: {flow_rs}"
    );
    assert!(
        flow_rs.contains("impl FlowApi for V2_8_0FlowApi<'_>"),
        "should impl FlowApi trait: {flow_rs}"
    );
    assert!(
        flow_rs.contains("crate::v2_8_0::api::flow::FlowApi"),
        "should construct version-specific API: {flow_rs}"
    );
    assert!(
        flow_rs.contains(".into()"),
        "should convert result with .into(): {flow_rs}"
    );

    // v2_7_2 should also have the impl
    let v272_flow = find_file(&files, "v2_7_2/flow.rs").expect("should have v2_7_2/flow.rs");
    assert!(
        v272_flow.contains("pub(crate) struct V2_7_2FlowApi<'a>"),
        "should define v2_7_2 wrapper struct: {v272_flow}"
    );
    assert!(
        v272_flow.contains("impl FlowApi for V2_7_2FlowApi<'_>"),
        "should impl FlowApi for v2_7_2: {v272_flow}"
    );
}

#[test]
fn impl_returns_unsupported_for_missing_endpoint() {
    let spec_v1 = make_spec(vec![make_flow_tag(vec![])]);
    let spec_v2 = make_spec(vec![make_flow_tag(vec![about_endpoint()])]);

    let versions: Vec<(&str, &str, &str, &ApiSpec)> = vec![
        ("2.7.2", "v2_7_2", "nifi-2-7-2", &spec_v1),
        ("2.8.0", "v2_8_0", "nifi-2-8-0", &spec_v2),
    ];

    let files = emit_dynamic_impls(&versions);

    // v1 has no endpoints for Flow, so its impl should NOT contain get_about_info
    let v272_flow = find_file(&files, "v2_7_2/flow.rs").expect("should have v2_7_2/flow.rs");
    assert!(
        !v272_flow.contains("get_about_info"),
        "v2_7_2 should NOT contain get_about_info (relies on trait default): {v272_flow}"
    );

    // v2 DOES have the endpoint, so its impl SHOULD contain get_about_info
    let v280_flow = find_file(&files, "v2_8_0/flow.rs").expect("should have v2_8_0/flow.rs");
    assert!(
        v280_flow.contains("get_about_info"),
        "v2_8_0 SHOULD contain get_about_info: {v280_flow}"
    );
}

#[test]
fn emits_void_return_method() {
    let spec = make_spec(vec![make_flow_tag(vec![void_endpoint()])]);
    let versions: Vec<(&str, &str, &str, &ApiSpec)> =
        vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];

    let files = emit_dynamic_impls(&versions);
    let flow_rs = find_file(&files, "v2_8_0/flow.rs").expect("should have v2_8_0/flow.rs");

    assert!(
        flow_rs.contains("-> Result<(), NifiError>"),
        "void endpoint should return Result<(), NifiError>: {flow_rs}"
    );
    // Void endpoints should NOT use .into() conversion
    assert!(
        !flow_rs.contains(".into()"),
        "void endpoint should not use .into(): {flow_rs}"
    );
}

#[test]
fn emits_body_param_with_try_from() {
    let tag = TagGroup {
        tag: "Processors".into(),
        struct_name: "ProcessorsApi".into(),
        module_name: "processors".into(),
        accessor_fn: "processors_api".into(),
        types: vec![],
        root_endpoints: vec![body_endpoint()],
        sub_groups: vec![],
    };
    let spec = make_spec(vec![tag]);
    let versions: Vec<(&str, &str, &str, &ApiSpec)> =
        vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];

    let files = emit_dynamic_impls(&versions);
    let proc_rs =
        find_file(&files, "v2_8_0/processors.rs").expect("should have v2_8_0/processors.rs");

    assert!(
        proc_rs.contains("body: types::ProcessorEntity"),
        "should accept body by value: {proc_rs}"
    );
    assert!(
        proc_rs.contains("crate::v2_8_0::types::ProcessorEntity::try_from(body)?"),
        "should convert body with try_from: {proc_rs}"
    );
}

#[test]
fn emits_sub_group_endpoint() {
    let spec = make_spec(vec![sub_group_tag()]);
    let versions: Vec<(&str, &str, &str, &ApiSpec)> =
        vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];

    let files = emit_dynamic_impls(&versions);
    let pg_rs = find_file(&files, "v2_8_0/process_groups.rs")
        .expect("should have v2_8_0/process_groups.rs");

    // Should construct the sub-group struct
    assert!(
        pg_rs.contains("ProcessGroupConnectionsApi"),
        "should reference sub-group struct: {pg_rs}"
    );
    assert!(
        pg_rs.contains("client: self.client"),
        "should pass client to sub-group: {pg_rs}"
    );
}

#[test]
fn emits_enum_query_param_conversion() {
    let ep = Endpoint {
        method: HttpMethod::Get,
        path: "/system-diagnostics".into(),
        fn_name: "get_system_diagnostics".into(),
        doc: None,
        description: None,
        path_params: vec![],
        request_type: None,
        body_doc: None,
        body_kind: None,
        response_type: Some("SystemDiagnosticsEntity".into()),
        response_inner: None,
        response_field: None,
        query_params: vec![
            QueryParam {
                name: "nodewise".into(),
                rust_name: "nodewise".into(),
                ty: QueryParamType::Bool,
                required: false,
                doc: None,
                enum_type_name: None,
            },
            QueryParam {
                name: "diagnosticLevel".into(),
                rust_name: "diagnostic_level".into(),
                ty: QueryParamType::Enum(vec!["BASIC".into(), "VERBOSE".into()]),
                required: false,
                doc: None,
                enum_type_name: Some("DiagnosticLevel".into()),
            },
        ],
        success_responses: vec![],
        error_responses: vec![],
        security: None,
    };
    let tag = TagGroup {
        tag: "SystemDiagnostics".into(),
        struct_name: "SystemDiagnosticsApi".into(),
        module_name: "system_diagnostics".into(),
        accessor_fn: "system_diagnostics_api".into(),
        types: vec![],
        root_endpoints: vec![ep],
        sub_groups: vec![],
    };
    let spec = make_spec(vec![tag]);
    let versions: Vec<(&str, &str, &str, &ApiSpec)> =
        vec![("2.8.0", "v2_8_0", "nifi-2-8-0", &spec)];

    let files = emit_dynamic_impls(&versions);
    let sd_rs = find_file(&files, "v2_8_0/system_diagnostics.rs")
        .expect("should have v2_8_0/system_diagnostics.rs");

    // Enum query param should use try_from conversion
    assert!(
        sd_rs.contains("crate::v2_8_0::types::DiagnosticLevel::try_from"),
        "should convert enum query param via try_from: {sd_rs}"
    );
    // Non-enum optional param should just be passed through
    assert!(
        sd_rs.contains("nodewise"),
        "should pass through non-enum query param: {sd_rs}"
    );
}
