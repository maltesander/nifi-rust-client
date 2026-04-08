// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#![allow(dead_code, private_interfaces, unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum DiagnosticLevel {
    #[serde(rename = "BASIC")]
    Basic,
    #[serde(rename = "VERBOSE")]
    Verbose,
}
impl std::fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticLevel::Basic => write!(f, "BASIC"),
            DiagnosticLevel::Verbose => write!(f, "VERBOSE"),
        }
    }
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JmxMetricsResultsEntity {
    #[serde(rename = "jmxMetricsResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jmx_metrics_results: Option<Vec<JmxMetricsResultDto>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDiagnosticsDto {
    #[serde(rename = "aggregateSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<SystemDiagnosticsSnapshotDto>,
    /// A systems diagnostics snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    #[serde(rename = "nodeSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeSystemDiagnosticsSnapshotDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDiagnosticsEntity {
    pub system_diagnostics: Option<SystemDiagnosticsDto>,
}
