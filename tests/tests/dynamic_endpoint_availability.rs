#![cfg(feature = "dynamic")]

mod helpers;

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_controller_services_clear_bulletins_unsupported() {
    use nifi_rust_client::dynamic::traits::ControllerServicesApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .controller_services_api()
        .clear_bulletins("root", Default::default())
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_controller_clear_flow_analysis_rule_bulletins_unsupported() {
    use nifi_rust_client::dynamic::traits::ControllerApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .controller_api()
        .clear_flow_analysis_rule_bulletins("root", Default::default())
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_controller_clear_parameter_provider_bulletins_unsupported() {
    use nifi_rust_client::dynamic::traits::ControllerApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .controller_api()
        .clear_parameter_provider_bulletins("root", Default::default())
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_controller_clear_registry_client_bulletins_unsupported() {
    use nifi_rust_client::dynamic::traits::ControllerApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .controller_api()
        .clear_registry_client_bulletins("root", Default::default())
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_controller_analyze_flow_registry_client_configuration_unsupported() {
    use nifi_rust_client::dynamic::traits::ControllerApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .controller_api()
        .analyze_flow_registry_client_configuration("root", Default::default())
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_controller_submit_registry_client_config_verification_request_unsupported() {
    use nifi_rust_client::dynamic::traits::ControllerApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .controller_api()
        .submit_registry_client_config_verification_request("root", Default::default())
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_controller_delete_registry_client_verification_request_unsupported() {
    use nifi_rust_client::dynamic::traits::ControllerApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .controller_api()
        .delete_registry_client_verification_request("root", "test-request_id")
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_controller_get_registry_client_verification_request_unsupported() {
    use nifi_rust_client::dynamic::traits::ControllerApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .controller_api()
        .get_registry_client_verification_request("root", "test-request_id")
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_flow_get_flow_registry_client_definition_unsupported() {
    use nifi_rust_client::dynamic::traits::FlowApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .flow_api()
        .get_flow_registry_client_definition(
            "test-group",
            "test-artifact",
            "test-version",
            "test-type",
        )
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(feature = "nifi-2-7-2")]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_flow_get_listen_ports_available() {
    use nifi_rust_client::dynamic::traits::FlowApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client.flow_api().get_listen_ports().await;
    assert!(
        result.is_ok(),
        "expected endpoint to be available, got: {:?}",
        result.unwrap_err()
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_flow_get_listen_ports_unsupported() {
    use nifi_rust_client::dynamic::traits::FlowApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client.flow_api().get_listen_ports().await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(feature = "nifi-2-7-2")]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_flow_clear_bulletins_1_available() {
    use nifi_rust_client::dynamic::traits::FlowApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .flow_api()
        .clear_bulletins_1("root", Default::default())
        .await;
    assert!(
        result.is_ok(),
        "expected endpoint to be available, got: {:?}",
        result.unwrap_err()
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_flow_clear_bulletins_1_unsupported() {
    use nifi_rust_client::dynamic::traits::FlowApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .flow_api()
        .clear_bulletins_1("root", Default::default())
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_inputports_clear_bulletins_2_unsupported() {
    use nifi_rust_client::dynamic::traits::InputPortsApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .inputports_api()
        .clear_bulletins_2("root", Default::default())
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_outputports_clear_bulletins_3_unsupported() {
    use nifi_rust_client::dynamic::traits::OutputPortsApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .outputports_api()
        .clear_bulletins_3("root", Default::default())
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_parameterproviders_clear_bulletins_4_unsupported() {
    use nifi_rust_client::dynamic::traits::ParameterProvidersApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .parameterproviders_api()
        .clear_bulletins_4("root", Default::default())
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_processors_clear_bulletins_5_unsupported() {
    use nifi_rust_client::dynamic::traits::ProcessorsApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .processors_api()
        .clear_bulletins_5("root", Default::default())
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(feature = "nifi-2-7-2")]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_remoteprocessgroups_clear_bulletins_6_available() {
    use nifi_rust_client::dynamic::traits::RemoteProcessGroupsApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .remoteprocessgroups_api()
        .clear_bulletins_6("root", Default::default())
        .await;
    assert!(
        result.is_ok(),
        "expected endpoint to be available, got: {:?}",
        result.unwrap_err()
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_remoteprocessgroups_clear_bulletins_6_unsupported() {
    use nifi_rust_client::dynamic::traits::RemoteProcessGroupsApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .remoteprocessgroups_api()
        .clear_bulletins_6("root", Default::default())
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}

#[cfg(not(any(feature = "nifi-2-7-2", feature = "nifi-2-8-0")))]
#[tokio::test]
#[ignore = "requires a running NiFi instance (use tests/run.sh)"]
async fn endpoint_reportingtasks_clear_bulletins_7_unsupported() {
    use nifi_rust_client::dynamic::traits::ReportingTasksApi;

    let client = helpers::dynamic_logged_in_client().await;
    let result = client
        .reportingtasks_api()
        .clear_bulletins_7("root", Default::default())
        .await;
    let err = result.unwrap_err();
    assert!(
        matches!(err, nifi_rust_client::NifiError::UnsupportedEndpoint { .. }),
        "expected UnsupportedEndpoint, got: {err:?}"
    );
}
