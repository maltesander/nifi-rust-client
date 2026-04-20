//! Operator porcelain — bulk state changes on a process group.
//!
//! Each function is a thin wrapper over a `bulk::*_dynamic` helper,
//! chosen for its value to daily operator workflow. Phase 1 does not
//! support `--wait` on these commands (requires wait helpers that do
//! not yet exist in the client library).

use nifi_rust_client::bulk;
use nifi_rust_client::dynamic::DynamicClient;

use crate::error::CliError;
use crate::output::CliOutput;

/// Start (schedule) all authorized processors in a process group.
pub async fn start_pg(client: &DynamicClient, pg_id: &str) -> Result<CliOutput, CliError> {
    let entity = bulk::start_process_group_dynamic(client, pg_id).await?;
    let value = serde_json::to_value(&entity)
        .map_err(|e| CliError::User(format!("serialization error: {e}")))?;
    Ok(CliOutput::Single(value))
}

#[cfg(test)]
mod tests {
    use nifi_rust_client::NifiClientBuilder;
    use nifi_rust_client::dynamic::DynamicClient;
    use serde_json::json;
    use wiremock::matchers::{body_partial_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn dynamic_client_on(mock: &MockServer, version: &str) -> DynamicClient {
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/about"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "about": { "title": "NiFi", "version": version }
            })))
            .mount(mock)
            .await;
        let client = NifiClientBuilder::new(&mock.uri()).unwrap().build().unwrap();
        DynamicClient::from_client(client).await.unwrap()
    }

    #[tokio::test]
    async fn start_pg_sends_running_body() {
        let mock = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/nifi-api/flow/process-groups/pg-1"))
            .and(body_partial_json(json!({ "id": "pg-1", "state": "RUNNING" })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "pg-1", "state": "RUNNING"
            })))
            .expect(1)
            .mount(&mock)
            .await;

        let client = dynamic_client_on(&mock, "2.9.0").await;
        let result = super::start_pg(&client, "pg-1").await.unwrap();
        match result {
            crate::output::CliOutput::Single(v) => {
                assert_eq!(v.get("state").and_then(|s| s.as_str()), Some("RUNNING"));
            }
            _ => panic!("expected Single"),
        }
    }
}
