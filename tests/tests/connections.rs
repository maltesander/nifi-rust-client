#![cfg(not(feature = "dynamic"))]
mod helpers;

use nifi_rust_client::types::{
    ConnectableDto, ConnectableDtoType, ConnectionDto, ConnectionEntity, PositionDto,
    ProcessorConfigDto, ProcessorDto, ProcessorEntity, RevisionDto,
};

async fn create_generate_processor(
    client: &nifi_rust_client::NifiClient,
    pg_id: &str,
    name: &str,
    x: f64,
    y: f64,
) -> (String, i64) {
    let body = ProcessorEntity {
        component: Some(ProcessorDto {
            name: Some(name.to_string()),
            r#type: Some("org.apache.nifi.processors.standard.GenerateFlowFile".to_string()),
            position: Some(PositionDto {
                x: Some(x),
                y: Some(y),
            }),
            config: Some(ProcessorConfigDto {
                scheduling_period: Some("1 hour".to_string()),
                ..Default::default()
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
        .processgroups()
        .create_processor(pg_id, &body)
        .await
        .expect("failed to create processor");
    let id = created.id.clone().expect("processor has no id");
    let version = created
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("processor has no revision");
    (id, version)
}

#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn connection_crud_lifecycle() {
    let client = helpers::logged_in_client().await;
    let (pg_id, pg_version) =
        helpers::create_temp_process_group(&client, "test-connection-pg").await;

    // Create two GenerateFlowFile processors to connect.
    let (src_id, src_version) =
        create_generate_processor(&client, &pg_id, "test-conn-src", 0.0, 0.0).await;
    let (dst_id, dst_version) =
        create_generate_processor(&client, &pg_id, "test-conn-dst", 400.0, 0.0).await;

    // Create connection from src → dst on the "success" relationship.
    let conn_body = ConnectionEntity {
        component: Some(ConnectionDto {
            source: Some(ConnectableDto {
                id: src_id.clone(),
                group_id: pg_id.clone(),
                r#type: ConnectableDtoType::Processor,
                ..Default::default()
            }),
            destination: Some(ConnectableDto {
                id: dst_id.clone(),
                group_id: pg_id.clone(),
                r#type: ConnectableDtoType::Processor,
                ..Default::default()
            }),
            selected_relationships: Some(vec!["success".to_string()]),
            ..Default::default()
        }),
        revision: Some(RevisionDto {
            version: Some(0),
            ..Default::default()
        }),
        ..Default::default()
    };
    let created_conn = client
        .processgroups()
        .create_connection(&pg_id, &conn_body)
        .await
        .expect("failed to create connection");
    let conn_id = created_conn.id.clone().expect("connection has no id");
    let conn_version = created_conn
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("connection has no revision");

    // Verify connection exists.
    let fetched = client
        .connections()
        .get_connection(&conn_id)
        .await
        .expect("failed to get connection");
    assert_eq!(
        fetched
            .component
            .as_ref()
            .and_then(|c| c.source.as_ref())
            .map(|s| s.id.as_str()),
        Some(src_id.as_str())
    );
    assert_eq!(
        fetched
            .component
            .as_ref()
            .and_then(|c| c.destination.as_ref())
            .map(|d| d.id.as_str()),
        Some(dst_id.as_str())
    );

    // Delete connection.
    client
        .connections()
        .delete_connection(&conn_id, Some(&conn_version.to_string()), None, None)
        .await
        .expect("failed to delete connection");

    // Verify gone.
    assert!(
        client.connections().get_connection(&conn_id).await.is_err(),
        "expected error fetching deleted connection"
    );

    // Clean up processors and process group.
    client
        .processors()
        .delete_processor(&src_id, Some(&src_version.to_string()), None, None)
        .await
        .expect("failed to delete src processor");
    client
        .processors()
        .delete_processor(&dst_id, Some(&dst_version.to_string()), None, None)
        .await
        .expect("failed to delete dst processor");
    helpers::delete_temp_process_group(&client, &pg_id, pg_version).await;
}
