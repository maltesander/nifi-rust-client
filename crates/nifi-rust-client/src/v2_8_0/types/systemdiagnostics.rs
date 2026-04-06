#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JmxMetricsResultsEntity {
    pub jmx_metrics_results: Option<Vec<JmxMetricsResultDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDiagnosticsDto {
    pub aggregate_snapshot: Option<SystemDiagnosticsSnapshotDto>,
    /// A systems diagnostics snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    pub node_snapshots: Option<Vec<NodeSystemDiagnosticsSnapshotDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDiagnosticsEntity {
    pub system_diagnostics: SystemDiagnosticsDto,
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
