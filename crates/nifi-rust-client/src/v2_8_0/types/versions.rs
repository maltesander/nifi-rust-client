#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateActiveRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The Process Group ID that this active request will update
    pub process_group_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartVersionControlRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    pub process_group_revision: Option<RevisionDto>,
    pub versioned_flow: Option<VersionedFlowDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionControlComponentMappingEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    pub process_group_revision: Option<RevisionDto>,
    /// The mapping of Versioned Component Identifiers to instance ID's
    pub version_control_component_mapping:
        Option<std::collections::HashMap<String, Option<String>>>,
    pub version_control_information: Option<VersionControlInformationDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionControlInformationEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    pub process_group_revision: Option<RevisionDto>,
    pub version_control_information: Option<VersionControlInformationDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowSnapshotEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    pub process_group_revision: Option<RevisionDto>,
    /// The ID of the Registry that this flow belongs to
    pub registry_id: Option<String>,
    /// If the Process Group to be updated has a child or descendant Process Group that is also under Version Control, this specifies whether or not the contents of that child/descendant Process Group should be updated.
    pub update_descendant_versioned_flows: Option<bool>,
    pub versioned_flow: Option<RegisteredFlowSnapshot>,
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowUpdateRequestEntity {
    pub process_group_revision: Option<RevisionDto>,
    pub request: Option<VersionedFlowUpdateRequestDto>,
}
