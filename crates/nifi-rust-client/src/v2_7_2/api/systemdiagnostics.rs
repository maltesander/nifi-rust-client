// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiClient;
use crate::NifiError;
pub struct SystemDiagnosticsApi<'a> {
    pub(crate) client: &'a NifiClient,
}
#[allow(
    private_interfaces,
    clippy::too_many_arguments,
    clippy::vec_init_then_push
)]
impl<'a> SystemDiagnosticsApi<'a> {
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
    pub async fn get_system_diagnostics(
        &self,
        nodewise: Option<bool>,
        diagnostic_level: Option<crate::v2_7_2::types::DiagnosticLevel>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_7_2::types::SystemDiagnosticsDto, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = nodewise {
            query.push(("nodewise", v.to_string()));
        }
        if let Some(v) = diagnostic_level {
            query.push(("diagnosticLevel", v.to_string()));
        }
        if let Some(v) = cluster_node_id {
            query.push(("clusterNodeId", v.to_string()));
        }
        let e: crate::v2_7_2::types::SystemDiagnosticsEntity = self
            .client
            .get_with_query("/system-diagnostics", &query)
            .await?;
        Ok(e.system_diagnostics.unwrap_or_default())
    }
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
    pub async fn get_jmx_metrics(
        &self,
        bean_name_filter: Option<&str>,
    ) -> Result<crate::v2_7_2::types::JmxMetricsResultsEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = bean_name_filter {
            query.push(("beanNameFilter", v.to_string()));
        }
        self.client
            .get_with_query("/system-diagnostics/jmx-metrics", &query)
            .await
    }
}
#[allow(clippy::too_many_arguments)]
impl crate::v2_7_2::traits::SystemDiagnosticsApi for SystemDiagnosticsApi<'_> {
    async fn get_system_diagnostics(
        &self,
        nodewise: Option<bool>,
        diagnostic_level: Option<crate::v2_7_2::types::DiagnosticLevel>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_7_2::types::SystemDiagnosticsDto, NifiError> {
        self.get_system_diagnostics(nodewise, diagnostic_level, cluster_node_id)
            .await
    }
    async fn get_jmx_metrics(
        &self,
        bean_name_filter: Option<&str>,
    ) -> Result<crate::v2_7_2::types::JmxMetricsResultsEntity, NifiError> {
        self.get_jmx_metrics(bean_name_filter).await
    }
}
