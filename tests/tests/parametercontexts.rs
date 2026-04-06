mod helpers;

use nifi_rust_client::types::{ParameterContextDto, ParameterContextEntity};

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn parameter_context_crud_lifecycle() {
    let client = helpers::logged_in_client().await;

    // Create
    let body = ParameterContextEntity {
        component: Some(ParameterContextDto {
            name: Some("test-param-context".to_string()),
            description: Some("integration test parameter context".to_string()),
            parameters: Some(vec![]),
            ..Default::default()
        }),
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    let created = client
        .parametercontexts_api()
        .create_parameter_context(&body)
        .await
        .expect("failed to create parameter context");
    let ctx_id = created.id.clone().expect("created context has no id");
    let version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("created context has no revision version");

    // Get — verify name
    let fetched = client
        .parametercontexts_api()
        .get_parameter_context(&ctx_id, None)
        .await
        .expect("failed to get parameter context");
    assert_eq!(
        fetched.component.as_ref().and_then(|c| c.name.as_deref()),
        Some("test-param-context")
    );

    // Update — change description (parameter context updates go through an async
    // update request; here we use update_parameter_context for the synchronous path)
    let update_body = ParameterContextEntity {
        id: Some(ctx_id.clone()),
        component: Some(ParameterContextDto {
            id: Some(ctx_id.clone()),
            name: Some("test-param-context".to_string()),
            description: Some("updated description".to_string()),
            parameters: Some(vec![]),
            ..Default::default()
        }),
        revision: Some(helpers::revision(version)),
        ..Default::default()
    };
    let updated = client
        .parametercontexts_api()
        .update_parameter_context(&ctx_id, &update_body)
        .await
        .expect("failed to update parameter context");
    let version_after_update = updated
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("updated context has no revision version");

    // Delete
    client
        .parametercontexts_api()
        .delete_parameter_context(&ctx_id, Some(&version_after_update.to_string()), None, None)
        .await
        .expect("failed to delete parameter context");

    // Verify gone
    assert!(
        client
            .parametercontexts_api()
            .get_parameter_context(&ctx_id, None)
            .await
            .is_err(),
        "expected error fetching deleted parameter context"
    );
}
