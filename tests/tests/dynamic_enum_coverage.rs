// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#![cfg(feature = "dynamic")]

mod helpers;

#[cfg(feature = "nifi-2-8-0")]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn enum_includedregistries_versioninfo_accepted() {
    use nifi_rust_client::dynamic::traits::FlowApi;
    use nifi_rust_client::dynamic::types::IncludedRegistries;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .flow_api()
        .get_flow_metrics(
            "prometheus",
            Some(IncludedRegistries::VersionInfo),
            None,
            None,
            None,
            None,
        )
        .await;
    assert!(
        result.is_ok(),
        "expected IncludedRegistries::VersionInfo to be accepted, got: {:?}",
        result.unwrap_err()
    );
}

#[cfg(not(feature = "nifi-2-8-0"))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn enum_includedregistries_versioninfo_unsupported() {
    use nifi_rust_client::dynamic::traits::FlowApi;
    use nifi_rust_client::dynamic::types::IncludedRegistries;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .flow_api()
        .get_flow_metrics(
            "prometheus",
            Some(IncludedRegistries::VersionInfo),
            None,
            None,
            None,
            None,
        )
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(
            err,
            nifi_rust_client::NifiError::UnsupportedEnumVariant { .. }
        ),
        "expected UnsupportedEnumVariant, got: {err:?}"
    );
}
