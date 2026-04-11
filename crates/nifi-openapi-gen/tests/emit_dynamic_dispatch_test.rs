use nifi_openapi_gen::emit::emit_dynamic_dispatch;
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

#[test]
fn emits_dispatch_struct_with_trait_impl() {
    let spec_v1 = make_spec(make_flow_tag(vec![about_endpoint()]));
    let spec_v2 = make_spec(make_flow_tag(vec![about_endpoint()]));
    let versions: Vec<(&str, &str, &str, &ApiSpec)> = vec![
        ("2.6.0", "v2_6_0", "nifi-2-6-0", &spec_v1),
        ("2.8.0", "v2_8_0", "nifi-2-8-0", &spec_v2),
    ];

    let files = emit_dynamic_dispatch(&versions);

    // Should produce mod.rs + flow.rs
    assert_eq!(files.len(), 2);
    assert_eq!(files[0].0, "mod.rs");
    assert_eq!(files[1].0, "flow.rs");

    let mod_rs = &files[0].1;
    assert!(
        mod_rs.contains("mod flow;"),
        "mod.rs should declare mod flow: {mod_rs}"
    );
    assert!(
        mod_rs.contains("pub use flow::FlowApiDispatch;"),
        "mod.rs should re-export FlowApiDispatch: {mod_rs}"
    );

    let flow_rs = &files[1].1;

    // Doc comment (lazy dispatch design)
    assert!(
        flow_rs.contains("Dynamic dispatch struct for the Flow API"),
        "Should have struct doc comment: {flow_rs}"
    );

    // #[non_exhaustive]
    assert!(
        flow_rs.contains("#[non_exhaustive]"),
        "Should have #[non_exhaustive]: {flow_rs}"
    );

    // Struct definition holding &DynamicClient
    assert!(
        flow_rs.contains("pub struct FlowApiDispatch"),
        "Should define FlowApiDispatch struct: {flow_rs}"
    );
    assert!(
        flow_rs.contains("pub(crate) client: &'a crate::dynamic::DynamicClient"),
        "Struct should borrow &DynamicClient: {flow_rs}"
    );
    assert!(
        !flow_rs.contains("pub enum FlowApiDispatch"),
        "Should no longer be an enum: {flow_rs}"
    );

    // Trait impl
    assert!(
        flow_rs.contains("impl FlowApi for FlowApiDispatch"),
        "Should impl FlowApi trait: {flow_rs}"
    );

    // Method with delegation
    assert!(
        flow_rs.contains("async fn get_about_info"),
        "Should have get_about_info method: {flow_rs}"
    );

    // Lazy version detection + dispatch to per-version impls
    assert!(
        flow_rs.contains("self.client.detect_version().await?"),
        "Should lazy-detect version in each method: {flow_rs}"
    );
    assert!(
        flow_rs.contains("crate::dynamic::impls::v2_6_0::V2_6_0FlowApi"),
        "Should construct v2_6_0 per-version impl: {flow_rs}"
    );
    assert!(
        flow_rs.contains("crate::dynamic::impls::v2_8_0::V2_8_0FlowApi"),
        "Should construct v2_8_0 per-version impl: {flow_rs}"
    );
    assert!(
        flow_rs.contains("client: &self.client.client"),
        "Per-version impl should be built from &self.client.client: {flow_rs}"
    );

    // Return type uses types::
    assert!(
        flow_rs.contains("types::AboutDto"),
        "Should use types:: prefix for return type: {flow_rs}"
    );

    // Imports
    assert!(
        flow_rs.contains("use crate::NifiError;"),
        "Should import NifiError: {flow_rs}"
    );
    assert!(
        flow_rs.contains("use crate::dynamic::types;"),
        "Should import dynamic types: {flow_rs}"
    );
    assert!(
        flow_rs.contains("use crate::dynamic::traits::FlowApi;"),
        "Should import FlowApi trait: {flow_rs}"
    );
}

#[test]
fn dispatch_forwards_all_params() {
    let ep = Endpoint {
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
        response_inner: Some("ProcessorDto".into()),
        response_field: Some("component".into()),
        query_params: vec![QueryParam {
            name: "includeDescendants".into(),
            rust_name: "include_descendants".into(),
            ty: QueryParamType::Bool,
            required: false,
            doc: None,
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

    let files = emit_dynamic_dispatch(&versions);
    let proc_rs = &files[1].1;

    // Method signature has path param, query param, and body
    assert!(
        proc_rs.contains("id: &str"),
        "Should have path param: {proc_rs}"
    );
    assert!(
        proc_rs.contains("include_descendants: Option<bool>"),
        "Should have query param: {proc_rs}"
    );
    assert!(
        proc_rs.contains("body: &types::ProcessorEntity"),
        "Should have borrowed body param: {proc_rs}"
    );

    // Per-version impl delegation forwards all three params
    assert!(
        proc_rs.contains("api.update_processor(id, include_descendants, body)"),
        "Should forward all params to per-version impl: {proc_rs}"
    );
}

#[test]
fn dispatch_skips_form_encoded_endpoints() {
    let ep = Endpoint {
        method: HttpMethod::Post,
        path: "/access/token".into(),
        fn_name: "create_access_token".into(),
        doc: Some("Creates a token.".into()),
        description: None,
        path_params: vec![],
        request_type: None,
        body_doc: None,
        body_kind: Some(RequestBodyKind::FormEncoded),
        response_type: None,
        response_inner: None,
        response_field: None,
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

    let files = emit_dynamic_dispatch(&versions);
    let access_rs = &files[1].1;

    assert!(
        !access_rs.contains("create_access_token"),
        "Should skip form-encoded endpoints: {access_rs}"
    );
}
