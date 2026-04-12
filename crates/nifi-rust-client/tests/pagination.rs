// ---------------------------------------------------------------------------
// Static-mode pagination tests
// ---------------------------------------------------------------------------
#[cfg(not(feature = "dynamic"))]
mod static_tests {
    use nifi_rust_client::pagination::{flow_history, HistoryFilter};
    use nifi_rust_client::NifiClientBuilder;
    use nifi_rust_client::NifiError;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// Build a wiremock response body for `GET /nifi-api/flow/history`.
    /// Wire format: `{ "history": { "actions": [...], "total": N } }`
    fn history_response(offset: usize, count: usize, total: usize) -> serde_json::Value {
        let end = std::cmp::min(offset + count, total);
        let actions: Vec<serde_json::Value> = (offset..end)
            .map(|i| {
                serde_json::json!({
                    "id": i as i32,
                    "timestamp": "01/01/2026 00:00:00 UTC"
                })
            })
            .collect();
        serde_json::json!({
            "history": {
                "actions": actions,
                "total": total as i32
            }
        })
    }

    #[tokio::test]
    async fn static_three_page_walk() {
        let mock = MockServer::start().await;

        // Page 0: offset=0, count=100 → 100 actions
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/history"))
            .and(query_param("offset", "0"))
            .and(query_param("count", "100"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(history_response(0, 100, 250)),
            )
            .mount(&mock)
            .await;

        // Page 1: offset=100, count=100 → 100 actions
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/history"))
            .and(query_param("offset", "100"))
            .and(query_param("count", "100"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(history_response(100, 100, 250)),
            )
            .mount(&mock)
            .await;

        // Page 2: offset=200, count=100 → 50 actions
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/history"))
            .and(query_param("offset", "200"))
            .and(query_param("count", "100"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(history_response(200, 100, 250)),
            )
            .mount(&mock)
            .await;

        let client = NifiClientBuilder::new(&mock.uri())
            .expect("builder")
            .build()
            .expect("client");

        let mut pag = flow_history(&client, HistoryFilter::default(), 100);

        let mut all_ids: Vec<i32> = Vec::new();
        while let Some(page) = pag.next_page().await.unwrap() {
            for action in &page {
                all_ids.push(action.id.unwrap());
            }
        }
        assert_eq!(all_ids.len(), 250);
        assert_eq!(all_ids, (0..250).collect::<Vec<i32>>());
    }

    #[tokio::test]
    async fn static_401_propagation() {
        let mock = MockServer::start().await;

        // Page 0: succeeds
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/history"))
            .and(query_param("offset", "0"))
            .and(query_param("count", "100"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(history_response(0, 100, 250)),
            )
            .mount(&mock)
            .await;

        // Page 1: 401
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/history"))
            .and(query_param("offset", "100"))
            .and(query_param("count", "100"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&mock)
            .await;

        let client = NifiClientBuilder::new(&mock.uri())
            .expect("builder")
            .build()
            .expect("client");

        let mut pag = flow_history(&client, HistoryFilter::default(), 100);

        // First page succeeds
        let p1 = pag.next_page().await.unwrap().unwrap();
        assert_eq!(p1.len(), 100);

        // Second page yields Unauthorized
        let err = pag.next_page().await.unwrap_err();
        assert!(
            matches!(err, NifiError::Unauthorized { .. }),
            "expected Unauthorized, got: {err:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Dynamic-mode pagination tests
// ---------------------------------------------------------------------------
#[cfg(feature = "dynamic")]
mod dynamic_tests {
    use nifi_rust_client::pagination::{flow_history_dynamic, HistoryFilter};
    use nifi_rust_client::NifiClientBuilder;
    use nifi_rust_client::NifiError;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// Mount the `/flow/about` mock and build a `DynamicClient`.
    async fn dynamic_client_on(
        mock: &MockServer,
        version: &str,
    ) -> nifi_rust_client::dynamic::DynamicClient {
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/about"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "about": { "title": "NiFi", "version": version }
            })))
            .mount(mock)
            .await;

        let client = NifiClientBuilder::new(&mock.uri())
            .unwrap()
            .build()
            .unwrap();
        nifi_rust_client::dynamic::DynamicClient::from_client(client)
            .await
            .unwrap()
    }

    /// Build a wiremock response body for `GET /nifi-api/flow/history`.
    fn history_response(offset: usize, count: usize, total: usize) -> serde_json::Value {
        let end = std::cmp::min(offset + count, total);
        let actions: Vec<serde_json::Value> = (offset..end)
            .map(|i| {
                serde_json::json!({
                    "id": i as i32,
                    "timestamp": "01/01/2026 00:00:00 UTC"
                })
            })
            .collect();
        serde_json::json!({
            "history": {
                "actions": actions,
                "total": total as i32
            }
        })
    }

    #[tokio::test]
    async fn dynamic_three_page_walk() {
        let mock = MockServer::start().await;
        let client = dynamic_client_on(&mock, "2.9.0").await;

        // Page 0
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/history"))
            .and(query_param("offset", "0"))
            .and(query_param("count", "100"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(history_response(0, 100, 250)),
            )
            .mount(&mock)
            .await;

        // Page 1
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/history"))
            .and(query_param("offset", "100"))
            .and(query_param("count", "100"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(history_response(100, 100, 250)),
            )
            .mount(&mock)
            .await;

        // Page 2
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/history"))
            .and(query_param("offset", "200"))
            .and(query_param("count", "100"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(history_response(200, 100, 250)),
            )
            .mount(&mock)
            .await;

        let mut pag = flow_history_dynamic(&client, HistoryFilter::default(), 100);

        let mut all_ids: Vec<i32> = Vec::new();
        while let Some(page) = pag.next_page().await.unwrap() {
            for action in &page {
                all_ids.push(action.id.unwrap());
            }
        }
        assert_eq!(all_ids.len(), 250);
        assert_eq!(all_ids, (0..250).collect::<Vec<i32>>());
    }

    #[tokio::test]
    async fn dynamic_missing_field_on_null_actions() {
        let mock = MockServer::start().await;
        let client = dynamic_client_on(&mock, "2.9.0").await;

        // Response with actions: null → MissingField
        Mock::given(method("GET"))
            .and(path("/nifi-api/flow/history"))
            .and(query_param("offset", "0"))
            .and(query_param("count", "100"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "history": {
                        "actions": null,
                        "total": 10
                    }
                })),
            )
            .mount(&mock)
            .await;

        let mut pag = flow_history_dynamic(&client, HistoryFilter::default(), 100);

        let err = pag.next_page().await.unwrap_err();
        assert!(
            matches!(err, NifiError::MissingField { ref path } if path == "actions"),
            "expected MissingField {{ path: \"actions\" }}, got: {err:?}"
        );
    }
}
