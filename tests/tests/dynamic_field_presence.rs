// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#![cfg(feature = "dynamic")]

mod helpers;

#[cfg(feature = "nifi-2-7-2")]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn field_processorentity_physical_state_present() {
    let client = helpers::dynamic_logged_in_client().await;
    let item = helpers::get_test_processor_entity(&client).await;
    assert!(
        item.physical_state.is_some(),
        "expected physical_state to be Some on version 2.7.2"
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn field_processorentity_physical_state_absent() {
    let client = helpers::dynamic_logged_in_client().await;
    let item = helpers::get_test_processor_entity(&client).await;
    assert!(
        item.physical_state.is_none(),
        "expected physical_state to be None on version older than 2.7.2"
    );
}

#[cfg(feature = "nifi-2-8-0")]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn field_provenanceeventdto_event_timestamp_present() {
    let client = helpers::dynamic_logged_in_client().await;
    let item = helpers::get_test_provenance_event(&client).await;
    assert!(
        item.event_timestamp.is_some(),
        "expected event_timestamp to be Some on version 2.8.0"
    );
}

#[cfg(not(feature = "nifi-2-8-0"))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn field_provenanceeventdto_event_timestamp_absent() {
    let client = helpers::dynamic_logged_in_client().await;
    let item = helpers::get_test_provenance_event(&client).await;
    assert!(
        item.event_timestamp.is_none(),
        "expected event_timestamp to be None on version older than 2.8.0"
    );
}
