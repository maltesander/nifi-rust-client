// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

use crate::NifiError;
/// The SystemDiagnostics API.
#[allow(unused_variables, async_fn_in_trait, clippy::too_many_arguments)]
pub trait SystemDiagnosticsApi {
    /// Gets the diagnostics for the system NiFi is running on
    async fn get_system_diagnostics(
        &self,
        nodewise: Option<bool>,
        diagnostic_level: Option<crate::v2_6_0::types::DiagnosticLevel>,
        cluster_node_id: Option<&str>,
    ) -> Result<crate::v2_6_0::types::SystemDiagnosticsDto, NifiError>;
    /// Retrieve available JMX metrics
    async fn get_jmx_metrics(
        &self,
        bean_name_filter: Option<&str>,
    ) -> Result<crate::v2_6_0::types::JmxMetricsResultsEntity, NifiError>;
}
