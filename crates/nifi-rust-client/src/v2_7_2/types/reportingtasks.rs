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
    pub disconnected_node_acknowledged: Option<bool>,
    pub revision: Option<RevisionDto>,
    /// The run status of the ReportingTask.
    pub state: Option<ReportingTaskRunStatusEntityState>,
}
