use nifi_openapi_gen::parser::{
    ApiSpec, Endpoint, Field, FieldType, HttpMethod, TagGroup, TypeDef, TypeKind,
};

#[test]
fn can_construct_internal_model() {
    let spec = ApiSpec {
        tags: vec![TagGroup {
            tag: "Flow".into(),
            struct_name: "FlowApi".into(),
            module_name: "flow".into(),
            accessor_fn: "flow_api".into(),
            types: vec![TypeDef {
                name: "AboutDto".into(),
                kind: TypeKind::Dto,
                fields: vec![Field {
                    rust_name: "title".into(),
                    serde_name: "title".into(),
                    ty: FieldType::Opt(Box::new(FieldType::Str)),
                    doc: Some("The title".into()),
                    read_only: false,
                    deprecated: false,
                }],
                doc: None,
            }],
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
                response_type: Some("AboutEntity".into()),
                response_inner: Some("AboutDto".into()),
                response_field: Some("about".into()),
                response_kind: nifi_openapi_gen::content_type::ResponseBodyKind::Json {
                    schema_ref: "AboutEntity".into(),
                },
                query_params: vec![],
                success_responses: vec![],
                error_responses: vec![],
                security: None,
            }],
            sub_groups: vec![],
        }],
        all_types: vec![],
    };
    assert_eq!(spec.tags[0].tag, "Flow");
    assert_eq!(spec.tags[0].types[0].name, "AboutDto");
    assert_eq!(spec.tags[0].root_endpoints[0].fn_name, "get_about_info");
}
