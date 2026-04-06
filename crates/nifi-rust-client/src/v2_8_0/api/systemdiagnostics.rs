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
    pub async fn get_system_diagnostics(
        &self,
        nodewise: Option<bool>,
        diagnostic_level: Option<crate::v2_8_0::types::DiagnosticLevel>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_8_0::types::SystemDiagnosticsDto, NifiError> {
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
        let e: crate::v2_8_0::types::SystemDiagnosticsEntity = self
            .client
            .get_with_query("/system-diagnostics", &query)
            .await?;
        Ok(e.system_diagnostics)
    }
    /// Retrieve available JMX metrics
    ///
    /// Note: This endpoint is subject to change as NiFi and it's REST API evolve.
    ///
    /// Calls `GET /nifi-api/system-diagnostics/jmx-metrics`.
    ///
    /// # Parameters
    /// - `bean_name_filter`: Regular Expression Pattern to be applied against the ObjectName
    pub async fn get_jmx_metrics(
        &self,
        bean_name_filter: Option<&str>,
    ) -> Result<crate::v2_8_0::types::JmxMetricsResultsEntity, NifiError> {
        let mut query: Vec<(&str, String)> = vec![];
        if let Some(v) = bean_name_filter {
            query.push(("beanNameFilter", v.to_string()));
        }
        self.client
            .get_with_query("/system-diagnostics/jmx-metrics", &query)
            .await
    }
}
