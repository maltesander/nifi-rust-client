#![cfg(not(feature = "dynamic"))]
use nifi_rust_client::NifiClientBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_system_diagnostics_returns_heap_and_processor_count() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/nifi-api/system-diagnostics"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "systemDiagnostics": {
                "aggregateSnapshot": {
                    "totalHeap": "512 MB",
                    "usedHeap": "128 MB",
                    "freeHeap": "384 MB",
                    "heapUtilization": "25.0%",
                    "maxHeap": "1 GB",
                    "availableProcessors": 8,
                    "totalThreads": 42,
                    "daemonThreads": 20,
                    "uptime": "1:00:00.000",
                    "statsLastRefreshed": "01/01/2026 00:00:00 UTC"
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = NifiClientBuilder::new(&mock_server.uri())
        .unwrap()
        .build()
        .unwrap();
    let diag = client
        .systemdiagnostics_api()
        .get_system_diagnostics(None, None, None)
        .await
        .unwrap();

    let snapshot = diag.aggregate_snapshot.as_ref().unwrap();
    assert_eq!(snapshot.available_processors, Some(8));
    assert_eq!(snapshot.total_threads, Some(42));
    assert_eq!(snapshot.total_heap.as_deref(), Some("512 MB"));
    assert_eq!(snapshot.free_heap.as_deref(), Some("384 MB"));
}
