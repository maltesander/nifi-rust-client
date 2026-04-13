#![cfg(not(feature = "dynamic"))]
mod helpers;

use nifi_rust_client::types::{UserDto, UserEntity, UserGroupDto, UserGroupEntity};

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn user_crud_lifecycle() {
    let client = helpers::logged_in_client().await;

    // Create
    let body = UserEntity {
        component: Some(UserDto {
            identity: Some("test-user-lifecycle@example.com".to_string()),
            ..Default::default()
        }),
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    let created = client
        .tenants()
        .create_user(&body)
        .await
        .expect("failed to create user");
    let user_id = created.id.clone().expect("created user has no id");
    let version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("created user has no revision version");

    // Get — verify identity
    let fetched = client
        .tenants()
        .get_user(&user_id)
        .await
        .expect("failed to get user");
    assert_eq!(
        fetched
            .component
            .as_ref()
            .and_then(|c| c.identity.as_deref()),
        Some("test-user-lifecycle@example.com")
    );

    // Update — change identity
    let update_body = UserEntity {
        id: Some(user_id.clone()),
        component: Some(UserDto {
            id: Some(user_id.clone()),
            identity: Some("test-user-lifecycle-updated@example.com".to_string()),
            ..Default::default()
        }),
        revision: Some(helpers::revision(version)),
        ..Default::default()
    };
    let updated = client
        .tenants()
        .update_user(&user_id, &update_body)
        .await
        .expect("failed to update user");
    assert_eq!(
        updated
            .component
            .as_ref()
            .and_then(|c| c.identity.as_deref()),
        Some("test-user-lifecycle-updated@example.com")
    );
    let version_after_update = updated
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("updated user has no revision version");

    // Delete
    client
        .tenants()
        .remove_user(
            &user_id,
            Some(&version_after_update.to_string()),
            None,
            None,
        )
        .await
        .expect("failed to delete user");

    // Verify gone
    assert!(
        client.tenants().get_user(&user_id).await.is_err(),
        "expected error fetching deleted user"
    );
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn user_group_crud_lifecycle() {
    let client = helpers::logged_in_client().await;

    // Create
    let body = UserGroupEntity {
        component: Some(UserGroupDto {
            identity: Some("test-group-lifecycle".to_string()),
            ..Default::default()
        }),
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    let created = client
        .tenants()
        .create_user_group(&body)
        .await
        .expect("failed to create user group");
    let group_id = created.id.clone().expect("created group has no id");
    let version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("created group has no revision version");

    // Get — verify identity
    let fetched = client
        .tenants()
        .get_user_group(&group_id)
        .await
        .expect("failed to get user group");
    assert_eq!(
        fetched
            .component
            .as_ref()
            .and_then(|c| c.identity.as_deref()),
        Some("test-group-lifecycle")
    );

    // Update — rename
    let update_body = UserGroupEntity {
        id: Some(group_id.clone()),
        component: Some(UserGroupDto {
            id: Some(group_id.clone()),
            identity: Some("test-group-lifecycle-renamed".to_string()),
            users: Some(vec![]),
            ..Default::default()
        }),
        revision: Some(helpers::revision(version)),
        ..Default::default()
    };
    let updated = client
        .tenants()
        .update_user_group(&group_id, &update_body)
        .await
        .expect("failed to update user group");
    assert_eq!(
        updated
            .component
            .as_ref()
            .and_then(|c| c.identity.as_deref()),
        Some("test-group-lifecycle-renamed")
    );
    let version_after_update = updated
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("updated group has no revision version");

    // Delete
    client
        .tenants()
        .remove_user_group(
            &group_id,
            Some(&version_after_update.to_string()),
            None,
            None,
        )
        .await
        .expect("failed to delete user group");

    // Verify gone
    assert!(
        client.tenants().get_user_group(&group_id).await.is_err(),
        "expected error fetching deleted user group"
    );
}
