// Shared helpers for integration tests.
// This file is auto-discovered by Cargo as a test binary with no tests of its own;
// functions here are used via `mod helpers;` in sibling test files.
#![allow(dead_code)]
use nifi_rust_client::{
    NifiClient, NifiClientBuilder,
    types::{PositionDto, ProcessGroupDto, ProcessGroupEntity, RevisionDto},
};

pub fn nifi_url() -> String {
    std::env::var("NIFI_URL").unwrap_or_else(|_| "https://localhost:8443".to_string())
}

pub fn nifi_username() -> String {
    std::env::var("NIFI_USERNAME").unwrap_or_else(|_| "admin".to_string())
}

pub fn nifi_password() -> String {
    std::env::var("NIFI_PASSWORD").unwrap_or_else(|_| "adminpassword123".to_string())
}

pub async fn logged_in_client() -> NifiClient {
    let mut client = NifiClientBuilder::new(&nifi_url())
        .expect("failed to parse NiFi URL")
        .danger_accept_invalid_certs(true)
        .build()
        .expect("failed to build NiFi client");
    client
        .login(&nifi_username(), &nifi_password())
        .await
        .expect("failed to log in");
    client
}

/// Create a throwaway process group under root for use as a test parent.
/// Returns `(id, revision_version)`.
pub async fn create_temp_process_group(client: &NifiClient, name: &str) -> (String, i64) {
    let body = ProcessGroupEntity {
        component: Some(ProcessGroupDto {
            name: Some(name.to_string()),
            position: Some(PositionDto {
                x: Some(0.0),
                y: Some(0.0),
            }),
            ..Default::default()
        }),
        revision: Some(RevisionDto {
            version: Some(0),
            ..Default::default()
        }),
        ..Default::default()
    };
    let created = client
        .processgroups_api()
        .process_groups("root")
        .create_process_group(None, &body)
        .await
        .expect("failed to create temp process group");
    let id = created.id.clone().expect("created PG has no id");
    let version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("created PG has no revision version");
    (id, version)
}

pub async fn delete_temp_process_group(client: &NifiClient, id: &str, version: i64) {
    client
        .processgroups_api()
        .remove_process_group(id, Some(&version.to_string()), None, None)
        .await
        .expect("failed to delete temp process group");
}

pub fn revision(version: i64) -> RevisionDto {
    RevisionDto {
        version: Some(version),
        ..Default::default()
    }
}
