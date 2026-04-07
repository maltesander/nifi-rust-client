/// Integration tests for flow status endpoint reads.
mod helpers;

use nifi_rust_client::types::{
    ConnectableDto, ConnectableDtoType, ConnectionDto, ConnectionEntity, PortDto, PortEntity,
    PositionDto, ProcessorConfigDto, ProcessorDto, ProcessorEntity,
};

async fn create_processor(
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
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    let created = client
        .processgroups_api()
        .processors(pg_id)
        .create_processor(&body)
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
async fn flow_status_reads_return_ok() {
    let client = helpers::logged_in_client().await;

    // ── process group status ──────────────────────────────────────────────────
    client
        .flow_api()
        .status("root")
        .get_process_group_status(None, None, None)
        .await
        .expect("failed to get process group status");

    // ── process group status history ──────────────────────────────────────────
    client
        .flow_api()
        .status("root")
        .get_process_group_status_history()
        .await
        .expect("failed to get process group status history");

    // Set up a temporary process group with processors, a connection, and ports.
    let (pg_id, pg_version) =
        helpers::create_temp_process_group(&client, "test-flow-status-pg").await;

    // ── processor status ──────────────────────────────────────────────────────
    let (src_id, src_version) =
        create_processor(&client, &pg_id, "test-status-src", 0.0, 0.0).await;
    let (dst_id, dst_version) =
        create_processor(&client, &pg_id, "test-status-dst", 400.0, 0.0).await;

    client
        .flow_api()
        .status(&src_id)
        .get_processor_status(None, None)
        .await
        .expect("failed to get processor status");

    // ── processor status history ──────────────────────────────────────────────
    client
        .flow_api()
        .status(&src_id)
        .get_processor_status_history()
        .await
        .expect("failed to get processor status history");

    // ── connection status ─────────────────────────────────────────────────────
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
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    let created_conn = client
        .processgroups_api()
        .connections(&pg_id)
        .create_connection(&conn_body)
        .await
        .expect("failed to create connection");
    let conn_id = created_conn.id.clone().expect("connection has no id");
    let conn_version = created_conn
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("connection has no revision");

    client
        .flow_api()
        .status(&conn_id)
        .get_connection_status(None, None)
        .await
        .expect("failed to get connection status");

    // ── connection status history ─────────────────────────────────────────────
    client
        .flow_api()
        .status(&conn_id)
        .get_connection_status_history()
        .await
        .expect("failed to get connection status history");

    // Note: get_connection_statistics (backpressure prediction) is omitted here because
    // it requires historical flowfile data to train the analytics model; on a fresh NiFi
    // instance with no traffic the endpoint returns 404 regardless of analytics config.

    // ── input port status ─────────────────────────────────────────────────────
    let ip_body = PortEntity {
        component: Some(PortDto {
            name: Some("test-input-port-status".to_string()),
            position: Some(PositionDto {
                x: Some(0.0),
                y: Some(200.0),
            }),
            ..Default::default()
        }),
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    let created_ip = client
        .processgroups_api()
        .input_ports(&pg_id)
        .create_input_port(&ip_body)
        .await
        .expect("failed to create input port");
    let input_port_id = created_ip.id.clone().expect("input port has no id");
    let input_port_version = created_ip
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("input port has no revision");

    client
        .flow_api()
        .status(&input_port_id)
        .get_input_port_status(None, None)
        .await
        .expect("failed to get input port status");

    // ── output port status ────────────────────────────────────────────────────
    let op_body = PortEntity {
        component: Some(PortDto {
            name: Some("test-output-port-status".to_string()),
            position: Some(PositionDto {
                x: Some(0.0),
                y: Some(300.0),
            }),
            ..Default::default()
        }),
        revision: Some(helpers::revision(0)),
        ..Default::default()
    };
    let created_op = client
        .processgroups_api()
        .output_ports(&pg_id)
        .create_output_port(&op_body)
        .await
        .expect("failed to create output port");
    let output_port_id = created_op.id.clone().expect("output port has no id");
    let output_port_version = created_op
        .revision
        .as_ref()
        .and_then(|r| r.version)
        .expect("output port has no revision");

    client
        .flow_api()
        .status(&output_port_id)
        .get_output_port_status(None, None)
        .await
        .expect("failed to get output port status");

    // ── cleanup (reverse order of creation) ──────────────────────────────────
    client
        .outputports_api()
        .remove_output_port(
            &output_port_id,
            Some(&output_port_version.to_string()),
            None,
            None,
        )
        .await
        .expect("failed to delete output port");
    client
        .inputports_api()
        .remove_input_port(
            &input_port_id,
            Some(&input_port_version.to_string()),
            None,
            None,
        )
        .await
        .expect("failed to delete input port");
    client
        .connections_api()
        .delete_connection(&conn_id, Some(&conn_version.to_string()), None, None)
        .await
        .expect("failed to delete connection");
    client
        .processors_api()
        .delete_processor(&dst_id, Some(&dst_version.to_string()), None, None)
        .await
        .expect("failed to delete dst processor");
    client
        .processors_api()
        .delete_processor(&src_id, Some(&src_version.to_string()), None, None)
        .await
        .expect("failed to delete src processor");
    helpers::delete_temp_process_group(&client, &pg_id, pg_version).await;
}
