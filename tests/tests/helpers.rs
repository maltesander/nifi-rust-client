// Shared helpers for integration tests.
// This file is auto-discovered by Cargo as a test binary with no tests of its own;
// functions here are used via `mod helpers;` in sibling test files.
#![allow(dead_code)]
use nifi_rust_client::{
    NifiClient, NifiClientBuilder,
    types::{
        PositionDto, ProcessGroupDto, ProcessGroupEntity, ProcessorRunStatusEntity,
        ProcessorRunStatusEntityState, RevisionDto,
    },
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

    // Reuse a cached token when multiple tests run concurrently to avoid hitting
    // NiFi's rate limit on the login endpoint (HTTP 429 Too Many Requests).
    // Token path: /tmp/nifi_test_token_<url_hash>
    let token_path = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        nifi_url().hash(&mut h);
        std::path::PathBuf::from(format!("/tmp/nifi_test_token_{:x}", h.finish()))
    };

    // Try reading a cached token first.
    if let Ok(token) = std::fs::read_to_string(&token_path) {
        let token = token.trim().to_string();
        if !token.is_empty() {
            client.set_token(token);
            // Quick sanity-check: if the token is still valid, use it.
            if client.flow_api().get_about_info().await.is_ok() {
                return client;
            }
            // Token expired — fall through to re-login below.
        }
    }

    client
        .login(&nifi_username(), &nifi_password())
        .await
        .expect("failed to log in");

    // Cache the fresh token for sibling tests in this run.
    if let Some(token) = client.token() {
        let _ = std::fs::write(&token_path, token);
    }

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

/// Start a processor and return the new revision version.
pub async fn start_processor(client: &NifiClient, proc_id: &str, version: i64) -> i64 {
    let result = client
        .processors_api()
        .run_status(proc_id)
        .update_run_status_4(&ProcessorRunStatusEntity {
            state: Some(ProcessorRunStatusEntityState::Running),
            revision: Some(revision(version)),
            ..Default::default()
        })
        .await
        .expect("failed to start processor");
    result
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("started entity has no revision version")
}

/// Stop a processor and return the new revision version.
pub async fn stop_processor(client: &NifiClient, proc_id: &str, version: i64) -> i64 {
    let result = client
        .processors_api()
        .run_status(proc_id)
        .update_run_status_4(&ProcessorRunStatusEntity {
            state: Some(ProcessorRunStatusEntityState::Stopped),
            revision: Some(revision(version)),
            ..Default::default()
        })
        .await
        .expect("failed to stop processor");
    result
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("stopped entity has no revision version")
}
