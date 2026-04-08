// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use crate::NifiError;
#[allow(unused_imports)]
use crate::dynamic::types;
/// The SystemDiagnostics API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait SystemDiagnosticsApi {
    /// Retrieve available JMX metrics
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/system-diagnostics/jmx-metrics`.
    ///
    /// # Parameters
    /// - `bean_name_filter`: Regular Expression Pattern to be applied against the ObjectName
    ///
    /// # Errors
    /// - `400`: NiFi was unable to complete the request because it was invalid. The request should not be retried without modification.
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    /// - `404`: The specified resource could not be found.
    /// - `409`: The request was valid but NiFi was not in the appropriate state to process it.
    ///
    /// # Permissions
    /// Requires `Read - /system`.
    async fn get_jmx_metrics(
        &self,
        bean_name_filter: Option<&str>,
    ) -> Result<types::JmxMetricsResultsEntity, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_jmx_metrics".to_string(),
            version: "unknown".to_string(),
        })
    }
    /// Gets the diagnostics for the system NiFi is running on
    ///
    /// Calls `GET /nifi-api/system-diagnostics`.
    ///
    /// # Parameters
    /// - `nodewise`: Whether or not to include the breakdown per node. Optional, defaults to false
    /// - `diagnostic_level`: Whether or not to include verbose details. Optional, defaults to false
    /// - `cluster_node_id`: The id of the node where to get the status.
    ///
    /// # Errors
    /// - `401`: Client could not be authenticated.
    /// - `403`: Client is not authorized to make this request.
    ///
    /// # Permissions
    /// Requires `Read - /system`.
    async fn get_system_diagnostics(
        &self,
        nodewise: Option<bool>,
        diagnostic_level: Option<types::DiagnosticLevel>,
        cluster_node_id: Option<&str>,
    ) -> Result<types::SystemDiagnosticsDto, NifiError> {
        Err(NifiError::UnsupportedEndpoint {
            endpoint: "get_system_diagnostics".to_string(),
            version: "unknown".to_string(),
        })
    }
}
