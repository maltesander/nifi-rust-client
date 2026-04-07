// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessorRunStatusEntityState {
    #[default]
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "RUN_ONCE")]
    RunOnce,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorRunStatusEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The run status of the Processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ProcessorRunStatusEntityState>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorsRunStatusDetailsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status_details: Option<Vec<ProcessorRunStatusDetailsEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunStatusDetailsRequestEntity {
    /// The IDs of all processors whose run status details should be provided
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_ids: Option<Vec<String>>,
}
