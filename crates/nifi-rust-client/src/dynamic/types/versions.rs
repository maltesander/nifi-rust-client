// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#![allow(dead_code, private_interfaces, unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateActiveRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The Process Group ID that this active request will update
    #[serde(rename = "processGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_id: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartVersionControlRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(rename = "versionedFlow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow: Option<VersionedFlowDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionControlComponentMappingEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<RevisionDto>,
    /// The mapping of Versioned Component Identifiers to instance ID's
    #[serde(rename = "versionControlComponentMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_control_component_mapping:
        Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "versionControlInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_control_information: Option<VersionControlInformationDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionControlInformationEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(rename = "versionControlInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_control_information: Option<VersionControlInformationDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowSnapshotEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<RevisionDto>,
    /// The ID of the Registry that this flow belongs to
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// If the Process Group to be updated has a child or descendant Process Group that is also under Version Control, this specifies whether or not the contents of that child/descendant Process Group should be updated.
    #[serde(rename = "updateDescendantVersionedFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_descendant_versioned_flows: Option<bool>,
    #[serde(rename = "versionedFlow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow: Option<RegisteredFlowSnapshot>,
    #[serde(rename = "versionedFlowSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowUpdateRequestEntity {
    #[serde(rename = "processGroupRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<VersionedFlowUpdateRequestDto>,
}
