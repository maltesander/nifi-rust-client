// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JmxMetricsResultsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jmx_metrics_results: Option<Vec<JmxMetricsResultDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDiagnosticsDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<SystemDiagnosticsSnapshotDto>,
    /// A systems diagnostics snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeSystemDiagnosticsSnapshotDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDiagnosticsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_diagnostics: Option<SystemDiagnosticsDto>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DiagnosticLevel {
    #[serde(rename = "BASIC")]
    Basic,
    #[serde(rename = "VERBOSE")]
    Verbose,
}
impl std::fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DiagnosticLevel::Basic => "BASIC",
            DiagnosticLevel::Verbose => "VERBOSE",
        };
        f.write_str(s)
    }
}
