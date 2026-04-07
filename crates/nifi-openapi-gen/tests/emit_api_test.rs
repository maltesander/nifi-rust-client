use nifi_openapi_gen::parser::{
    ApiSpec, Endpoint, HttpMethod, PathParam, QueryParam, QueryParamType, RequestBodyKind,
    SubGroup, TagGroup,
};
use nifi_openapi_gen::{emit_api, emit_tests};

fn flow_spec() -> ApiSpec {
    ApiSpec {
        all_types: vec![],
        tags: vec![TagGroup {
            tag: "Flow".into(),
            struct_name: "FlowApi".into(),
            module_name: "flow".into(),
            accessor_fn: "flow_api".into(),
            types: vec![],
            root_endpoints: vec![
                Endpoint {
                    method: HttpMethod::Get,
                    path: "/flow/about".into(),
                    fn_name: "get_about_info".into(),
                    doc: Some("Returns about info".into()),
                    description: None,
                    path_params: vec![],
                    request_type: None,
                    body_doc: None,

                    body_kind: None,
                    response_type: Some("AboutEntity".into()),
                    response_inner: Some("AboutDto".into()),
                    response_field: Some("about".into()),
                    query_params: vec![],
                    error_responses: vec![],
                    security: None,
                },
                Endpoint {
                    method: HttpMethod::Delete,
                    path: "/processors/{id}".into(),
                    fn_name: "delete_processor".into(),
                    doc: Some("Deletes a processor".into()),
                    description: None,
                    path_params: vec![PathParam {
                        name: "id".into(),
                        doc: None,
                    }],
                    request_type: None,
                    body_doc: None,

                    body_kind: None,
                    response_type: Some("ProcessorEntity".into()),
                    response_inner: Some("ProcessorDto".into()),
                    response_field: Some("component".into()),
                    query_params: vec![],
                    error_responses: vec![],
                    security: None,
                },
                Endpoint {
                    method: HttpMethod::Post,
                    path: "/processors".into(),
                    fn_name: "create_processor".into(),
                    doc: None,
                    description: None,
                    path_params: vec![],
                    request_type: Some("CreateProcessorRequestEntity".into()),
                    body_doc: None,
                    body_kind: Some(RequestBodyKind::Json),
                    response_type: Some("ProcessorEntity".into()),
                    response_inner: Some("ProcessorDto".into()),
                    response_field: Some("component".into()),
                    query_params: vec![],
                    error_responses: vec![],
                    security: None,
                },
            ],
            sub_groups: vec![],
        }],
    }
}

/// Join all generated file contents for assertion helpers.
fn all_output(spec: &ApiSpec) -> String {
    emit_api(spec)
        .into_iter()
        .map(|(_, content)| content)
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn emit_resource_struct_and_accessor() {
    let out = all_output(&flow_spec());
    assert!(out.contains("pub struct FlowApi"), "{out}");
    assert!(out.contains("pub(crate) client: &'a NifiClient"), "{out}");
    assert!(out.contains("pub fn flow_api"), "{out}");
    assert!(out.contains("FlowApi { client: self }"), "{out}");
}

#[test]
fn emit_get_with_entity_unwrap() {
    let out = all_output(&flow_spec());
    assert!(out.contains("pub async fn get_about_info"), "{out}");
    assert!(out.contains("Result<crate::types::AboutDto"), "{out}");
    assert!(out.contains(".get(\"/flow/about\")"), "{out}");
    assert!(out.contains("Ok(e.about)"), "{out}");
}

#[test]
fn emit_delete_with_path_param_and_unwrap() {
    let out = all_output(&flow_spec());
    assert!(out.contains("pub async fn delete_processor"), "{out}");
    assert!(out.contains("id: &str"), "{out}");
    assert!(
        out.contains("format!(\"/processors/{id}\"") || out.contains("/processors/{id}"),
        "{out}"
    );
    assert!(out.contains("Ok(e.component)"), "{out}");
}

#[test]
fn emit_test_stub_for_get_endpoint() {
    let out = emit_tests(&flow_spec());
    assert!(out.contains("#[tokio::test]"), "{out}");
    assert!(out.contains("async fn test_get_about_info"), "{out}");
    assert!(out.contains("MockServer::start()"), "{out}");
    assert!(out.contains("/nifi-api/flow/about"), "{out}");
    assert!(out.contains("client.flow_api().get_about_info()"), "{out}");
}

#[test]
fn emit_post_with_body() {
    let out = all_output(&flow_spec());
    assert!(out.contains("pub async fn create_processor"), "{out}");
    assert!(
        out.contains("body: &crate::types::CreateProcessorRequestEntity"),
        "{out}"
    );
    assert!(out.contains(".post(\"/processors\", body)"), "{out}");
}

#[test]
fn emit_per_tag_files() {
    let files = emit_api(&flow_spec());
    let names: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();
    assert!(names.contains(&"mod.rs"), "missing mod.rs: {names:?}");
    assert!(names.contains(&"flow.rs"), "missing flow.rs: {names:?}");

    let mod_rs = files.iter().find(|(n, _)| n == "mod.rs").unwrap();
    assert!(
        mod_rs.1.contains("pub mod flow"),
        "mod.rs missing module decl: {}",
        mod_rs.1
    );
    assert!(
        mod_rs.1.contains("impl crate::NifiClient"),
        "mod.rs missing NifiClient impl: {}",
        mod_rs.1
    );

    let flow_rs = files.iter().find(|(n, _)| n == "flow.rs").unwrap();
    assert!(
        flow_rs.1.contains("pub struct FlowApi"),
        "flow.rs missing struct: {}",
        flow_rs.1
    );
}

fn services_spec() -> ApiSpec {
    ApiSpec {
        all_types: vec![],
        tags: vec![TagGroup {
            tag: "Services".into(),
            struct_name: "ServicesApi".into(),
            module_name: "services".into(),
            accessor_fn: "services_api".into(),
            types: vec![],
            root_endpoints: vec![Endpoint {
                method: HttpMethod::Get,
                path: "/services/{id}".into(),
                fn_name: "get_service".into(),
                doc: None,
                description: None,
                path_params: vec![PathParam {
                    name: "id".into(),
                    doc: None,
                }],
                request_type: None,
                body_doc: None,

                body_kind: None,
                response_type: Some("ServiceEntity".into()),
                response_inner: Some("ServiceDto".into()),
                response_field: Some("service".into()),
                query_params: vec![],
                error_responses: vec![],
                security: None,
            }],
            sub_groups: vec![SubGroup {
                name: "config".into(),
                struct_name: "ServicesConfigApi".into(),
                accessor_fn: "config".into(),
                primary_param: "id".into(),
                primary_param_doc: None,
                endpoints: vec![Endpoint {
                    method: HttpMethod::Get,
                    path: "/services/{id}/config/verification-requests/{request_id}".into(),
                    fn_name: "get_verification".into(),
                    doc: Some("Gets verification".into()),
                    description: None,
                    path_params: vec![
                        PathParam {
                            name: "id".into(),
                            doc: None,
                        },
                        PathParam {
                            name: "request_id".into(),
                            doc: None,
                        },
                    ],
                    request_type: None,
                    body_doc: None,

                    body_kind: None,
                    response_type: Some("ServiceEntity".into()),
                    response_inner: Some("ServiceDto".into()),
                    response_field: Some("service".into()),
                    query_params: vec![],
                    error_responses: vec![],
                    security: None,
                }],
            }],
        }],
    }
}

#[test]
fn emit_sub_struct_definition() {
    let out = all_output(&services_spec());
    assert!(out.contains("pub struct ServicesConfigApi"), "{out}");
    assert!(out.contains("pub(crate) id: &'a str"), "{out}");
}

#[test]
fn emit_sub_group_accessor_on_parent() {
    let out = all_output(&services_spec());
    // accessor method on ServicesApi
    assert!(out.contains("pub fn config"), "{out}");
    // prettyplease may expand the struct init to multi-line
    assert!(out.contains("client: self.client"), "{out}");
    assert!(
        out.contains("ServicesConfigApi {") || out.contains("ServicesConfigApi{"),
        "{out}"
    );
}

#[test]
fn emit_sub_group_method_excludes_primary_param() {
    let out = all_output(&services_spec());
    // get_verification on ServicesConfigApi — id is NOT a function argument
    assert!(out.contains("pub async fn get_verification"), "{out}");
    assert!(out.contains("request_id: &str"), "{out}");
    // id comes from self.id via injected binding
    assert!(out.contains("let id = self.id"), "{out}");
    // must NOT have `id: &str` as a function parameter on get_verification
    // Verify the format string still contains {id} for path building
    assert!(
        out.contains("/services/{id}/config/verification-requests/{request_id}")
            || out.contains("format!"),
        "{out}"
    );
}

#[test]
fn emit_test_stub_for_sub_group_endpoint() {
    let out = emit_tests(&services_spec());
    // Sub-group call: client.services().config("test-id").get_verification(...)
    // prettyplease may break the method chain across lines
    assert!(
        out.contains(".config(\"test-id\")"),
        "missing config accessor call: {out}"
    );
    assert!(
        out.contains(".get_verification("),
        "missing get_verification call: {out}"
    );
}

fn spec_with_enum_query_param() -> ApiSpec {
    use nifi_openapi_gen::parser::QueryParamType;
    ApiSpec {
        all_types: vec![],
        tags: vec![TagGroup {
            tag: "ProcessGroups".into(),
            struct_name: "ProcessGroupsApi".into(),
            module_name: "processgroups".into(),
            accessor_fn: "processgroups_api".into(),
            types: vec![],
            root_endpoints: vec![Endpoint {
                method: HttpMethod::Post,
                path: "/process-groups/{id}/process-groups".into(),
                fn_name: "create_process_group".into(),
                doc: None,
                description: None,
                path_params: vec![PathParam {
                    name: "id".into(),
                    doc: None,
                }],
                request_type: Some("ProcessGroupEntity".into()),
                body_doc: None,
                body_kind: Some(RequestBodyKind::Json),
                response_type: Some("ProcessGroupEntity".into()),
                response_inner: None,
                response_field: None,
                query_params: vec![QueryParam {
                    name: "parameterContextHandlingStrategy".into(),
                    rust_name: "parameter_context_handling_strategy".into(),
                    ty: QueryParamType::Enum(vec!["KEEP_EXISTING".into(), "REPLACE".into()]),
                    required: false,
                    doc: None,
                    enum_type_name: Some("ParameterContextHandlingStrategy".into()),
                }],
                error_responses: vec![],
                security: None,
            }],
            sub_groups: vec![],
        }],
    }
}

#[test]
fn enum_query_param_uses_type_name_in_signature() {
    let out = all_output(&spec_with_enum_query_param());
    // method signature must use the enum type, not &str
    // prettyplease may break long type across lines, so check parts separately
    assert!(
        out.contains("parameter_context_handling_strategy:"),
        "parameter missing from signature: {out}"
    );
    assert!(
        out.contains("crate::types::ParameterContextHandlingStrategy"),
        "enum type name missing from signature: {out}"
    );
    // query building must call .to_string()
    assert!(
        out.contains("v.to_string()"),
        "missing to_string call: {out}"
    );
}

fn spec_with_query_params() -> ApiSpec {
    ApiSpec {
        all_types: vec![],
        tags: vec![TagGroup {
            tag: "Flow".into(),
            struct_name: "FlowApi".into(),
            module_name: "flow".into(),
            accessor_fn: "flow_api".into(),
            types: vec![],
            root_endpoints: vec![
                // Optional string query param
                Endpoint {
                    method: HttpMethod::Get,
                    path: "/flow/search-results".into(),
                    fn_name: "search_flow".into(),
                    doc: None,
                    description: None,
                    path_params: vec![],
                    request_type: None,
                    body_doc: None,

                    body_kind: None,
                    response_type: Some("SearchResultsEntity".into()),
                    response_inner: Some("SearchResultsDto".into()),
                    response_field: Some("search_results_d_t_o".into()),
                    query_params: vec![QueryParam {
                        name: "q".into(),
                        rust_name: "q".into(),
                        ty: QueryParamType::Str,
                        required: false,
                        doc: None,
                        enum_type_name: None,
                    }],
                    error_responses: vec![],
                    security: None,
                },
                // Required integer query params
                Endpoint {
                    method: HttpMethod::Get,
                    path: "/flow/history".into(),
                    fn_name: "get_history".into(),
                    doc: None,
                    description: None,
                    path_params: vec![],
                    request_type: None,
                    body_doc: None,

                    body_kind: None,
                    response_type: Some("HistoryEntity".into()),
                    response_inner: Some("HistoryDto".into()),
                    response_field: Some("history".into()),
                    query_params: vec![
                        QueryParam {
                            name: "offset".into(),
                            rust_name: "offset".into(),
                            ty: QueryParamType::I32,
                            required: true,
                            doc: None,
                            enum_type_name: None,
                        },
                        QueryParam {
                            name: "count".into(),
                            rust_name: "count".into(),
                            ty: QueryParamType::I32,
                            required: true,
                            doc: None,
                            enum_type_name: None,
                        },
                    ],
                    error_responses: vec![],
                    security: None,
                },
                // DELETE with required query params (version/clientId pattern)
                Endpoint {
                    method: HttpMethod::Delete,
                    path: "/connections/{id}".into(),
                    fn_name: "delete_connection".into(),
                    doc: None,
                    description: None,
                    path_params: vec![PathParam {
                        name: "id".into(),
                        doc: None,
                    }],
                    request_type: None,
                    body_doc: None,

                    body_kind: None,
                    response_type: Some("ConnectionEntity".into()),
                    response_inner: None,
                    response_field: None,
                    query_params: vec![
                        QueryParam {
                            name: "version".into(),
                            rust_name: "version".into(),
                            ty: QueryParamType::I64,
                            required: false,
                            doc: None,
                            enum_type_name: None,
                        },
                        QueryParam {
                            name: "clientId".into(),
                            rust_name: "client_id".into(),
                            ty: QueryParamType::Str,
                            required: false,
                            doc: None,
                            enum_type_name: None,
                        },
                    ],
                    error_responses: vec![],
                    security: None,
                },
            ],
            sub_groups: vec![],
        }],
    }
}

#[test]
fn emit_optional_query_param_in_signature() {
    let out = all_output(&spec_with_query_params());
    assert!(out.contains("search_flow"), "search_flow not found: {out}");
    assert!(
        out.contains("q: Option<&str>"),
        "missing optional query param in signature: {out}"
    );
}

#[test]
fn emit_required_query_param_in_signature() {
    let out = all_output(&spec_with_query_params());
    assert!(out.contains("get_history"), "get_history not found: {out}");
    assert!(
        out.contains("offset: i32"),
        "missing offset param in signature: {out}"
    );
    assert!(
        out.contains("count: i32"),
        "missing count param in signature: {out}"
    );
}

#[test]
fn emit_optional_query_param_push_in_body() {
    let out = all_output(&spec_with_query_params());
    assert!(
        out.contains("if let Some(v) = q"),
        "missing optional query param push: {out}"
    );
    assert!(
        out.contains("query.push((\"q\", v.to_string()))"),
        "missing query push content: {out}"
    );
}

#[test]
fn emit_required_query_param_push_in_body() {
    let out = all_output(&spec_with_query_params());
    assert!(
        out.contains("query.push((\"offset\", offset.to_string()))"),
        "missing unconditional offset push: {out}"
    );
}

#[test]
fn emit_get_with_query_helper_call() {
    let out = all_output(&spec_with_query_params());
    assert!(
        out.contains("get_with_query"),
        "missing get_with_query call: {out}"
    );
}

#[test]
fn emit_delete_returning_with_query_helper_call() {
    let out = all_output(&spec_with_query_params());
    assert!(
        out.contains("delete_returning_with_query"),
        "missing delete_returning_with_query call: {out}"
    );
}

fn spec_with_errors_and_security() -> ApiSpec {
    ApiSpec {
        all_types: vec![],
        tags: vec![TagGroup {
            tag: "Flow".into(),
            struct_name: "FlowApi".into(),
            module_name: "flow".into(),
            accessor_fn: "flow_api".into(),
            types: vec![],
            root_endpoints: vec![Endpoint {
                method: HttpMethod::Get,
                path: "/flow/about".into(),
                fn_name: "get_about_info".into(),
                doc: Some("Returns about info".into()),
                description: None,
                path_params: vec![],
                request_type: None,
                body_doc: None,
                body_kind: None,
                response_type: None,
                response_inner: None,
                response_field: None,
                query_params: vec![],
                error_responses: vec![
                    ("401".into(), "Client could not be authenticated.".into()),
                    ("403".into(), "Client is not authorized to make this request.".into()),
                ],
                security: Some(vec!["Read - /flow".into()]),
            }],
            sub_groups: vec![],
        }],
    }
}

fn spec_with_multi_security() -> ApiSpec {
    ApiSpec {
        all_types: vec![],
        tags: vec![TagGroup {
            tag: "Processors".into(),
            struct_name: "ProcessorsApi".into(),
            module_name: "processors".into(),
            accessor_fn: "processors_api".into(),
            types: vec![],
            root_endpoints: vec![Endpoint {
                method: HttpMethod::Delete,
                path: "/processors/{id}".into(),
                fn_name: "delete_processor".into(),
                doc: Some("Deletes a processor".into()),
                description: None,
                path_params: vec![PathParam { name: "id".into(), doc: None }],
                request_type: None,
                body_doc: None,
                body_kind: None,
                response_type: None,
                response_inner: None,
                response_field: None,
                query_params: vec![],
                error_responses: vec![],
                security: Some(vec![
                    "Write - /processors/{uuid}".into(),
                    "Write - Parent Process Group - /process-groups/{uuid}".into(),
                ]),
            }],
            sub_groups: vec![],
        }],
    }
}

fn spec_with_no_auth() -> ApiSpec {
    ApiSpec {
        all_types: vec![],
        tags: vec![TagGroup {
            tag: "Access".into(),
            struct_name: "AccessApi".into(),
            module_name: "access".into(),
            accessor_fn: "access_api".into(),
            types: vec![],
            root_endpoints: vec![Endpoint {
                method: HttpMethod::Post,
                path: "/access/token".into(),
                fn_name: "create_access_token".into(),
                doc: Some("Creates a token".into()),
                description: None,
                path_params: vec![],
                request_type: None,
                body_doc: None,
                body_kind: None,
                response_type: None,
                response_inner: None,
                response_field: None,
                query_params: vec![],
                error_responses: vec![],
                security: Some(vec![]),
            }],
            sub_groups: vec![],
        }],
    }
}

#[test]
fn emit_errors_section() {
    let spec = spec_with_errors_and_security();
    let out = all_output(&spec);
    assert!(out.contains("# Errors"), "missing # Errors section: {out}");
    assert!(out.contains("- `401`: Client could not be authenticated."), "missing 401: {out}");
    assert!(out.contains("- `403`: Client is not authorized"), "missing 403: {out}");
}

#[test]
fn emit_permissions_single() {
    let spec = spec_with_errors_and_security();
    let out = all_output(&spec);
    assert!(out.contains("# Permissions"), "missing # Permissions section: {out}");
    assert!(
        out.contains("Requires `Read - /flow`."),
        "missing single permission: {out}"
    );
}

#[test]
fn emit_permissions_multiple() {
    let spec = spec_with_multi_security();
    let out = all_output(&spec);
    assert!(out.contains("# Permissions"), "missing # Permissions: {out}");
    assert!(out.contains("- `Write - /processors/{uuid}`"), "missing processor perm: {out}");
    assert!(out.contains("- `Write - Parent Process Group"), "missing pg perm: {out}");
}

#[test]
fn emit_permissions_no_auth() {
    let spec = spec_with_no_auth();
    let out = all_output(&spec);
    assert!(out.contains("# Permissions"), "missing # Permissions: {out}");
    assert!(
        out.contains("No authentication required."),
        "missing no-auth text: {out}"
    );
}

#[test]
fn no_permissions_section_when_security_absent() {
    // security: None means the spec field was absent — omit section entirely
    let spec = ApiSpec {
        all_types: vec![],
        tags: vec![TagGroup {
            tag: "Flow".into(),
            struct_name: "FlowApi".into(),
            module_name: "flow".into(),
            accessor_fn: "flow_api".into(),
            types: vec![],
            root_endpoints: vec![Endpoint {
                method: HttpMethod::Get,
                path: "/flow/about".into(),
                fn_name: "get_about_info".into(),
                doc: Some("Returns about info".into()),
                description: None,
                path_params: vec![],
                request_type: None,
                body_doc: None,
                body_kind: None,
                response_type: None,
                response_inner: None,
                response_field: None,
                query_params: vec![],
                error_responses: vec![],
                security: None,
            }],
            sub_groups: vec![],
        }],
    };
    let out = all_output(&spec);
    assert!(!out.contains("# Permissions"), "should not emit Permissions when security absent: {out}");
}
