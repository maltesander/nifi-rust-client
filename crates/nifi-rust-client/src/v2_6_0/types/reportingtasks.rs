// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ReportingTaskRunStatusEntityState {
    #[default]
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STOPPED")]
    Stopped,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskRunStatusEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The run status of the ReportingTask.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ReportingTaskRunStatusEntityState>,
}
