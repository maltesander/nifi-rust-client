// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#![allow(dead_code, private_interfaces, unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorRunStatusEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The run status of the Processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorsRunStatusDetailsEntity {
    #[serde(rename = "runStatusDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status_details: Option<Vec<ProcessorRunStatusDetailsEntity>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunStatusDetailsRequestEntity {
    /// The IDs of all processors whose run status details should be provided
    #[serde(rename = "processorIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_ids: Option<Vec<String>>,
}
