// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<ConnectionEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyRequestEntity {
    /// The ids of the connections to be copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<String>>,
    /// The ids of the funnels to be copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnels: Option<Vec<String>>,
    /// The ids of the input ports to be copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<String>>,
    /// The ids of the labels to be copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// The ids of the output ports to be copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<String>>,
    /// The ids of the process groups to be copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<Vec<String>>,
    /// The ids of the processors to be copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<String>>,
    /// The ids of the remote process groups to be copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<Vec<String>>,
}
/// The response from copying.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyResponseEntity {
    /// The connections being copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<VersionedConnection>>,
    /// The external controller service references.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_controller_service_references:
        Option<std::collections::HashMap<String, Option<ExternalControllerServiceReference>>>,
    /// The funnels being copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnels: Option<Vec<VersionedFunnel>>,
    /// The id for this copy action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The input ports being copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<VersionedPort>>,
    /// The labels being copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<VersionedLabel>>,
    /// The output ports being copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<VersionedPort>>,
    /// The referenced parameter contexts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_contexts:
        Option<std::collections::HashMap<String, Option<VersionedParameterContext>>>,
    /// The referenced parameter providers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_providers:
        Option<std::collections::HashMap<String, Option<ParameterProviderReference>>>,
    /// The process groups being copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<Vec<VersionedProcessGroup>>,
    /// The processors being copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<VersionedProcessor>>,
    /// The remote process groups being copied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<Vec<VersionedRemoteProcessGroup>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopySnippetRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The x coordinate of the origin of the bounding box where the new components will be placed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_x: Option<f64>,
    /// The y coordinate of the origin of the bounding box where the new components will be placed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_y: Option<f64>,
    /// The identifier of the snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet_id: Option<String>,
}
/// Flow containing the components that were created as part of this paste action.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowDto {
    /// The connections in this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<ConnectionEntity>>,
    /// The funnels in this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnels: Option<Vec<FunnelEntity>>,
    /// The input ports in this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<PortEntity>>,
    /// The labels in this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<LabelEntity>>,
    /// The output ports in this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<PortEntity>>,
    /// The process groups in this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<Vec<ProcessGroupEntity>>,
    /// The processors in this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<ProcessorEntity>>,
    /// The remote process groups in this flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<Vec<RemoteProcessGroupEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<FlowDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunnelsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnels: Option<Vec<FunnelEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputPortsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<PortEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<LabelEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputPortsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<PortEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasteRequestEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_response: Option<CopyResponseEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasteResponseEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<FlowDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessGroupEntityProcessGroupUpdateStrategy {
    #[default]
    #[serde(rename = "CURRENT_GROUP")]
    CurrentGroup,
    #[serde(rename = "CURRENT_GROUP_WITH_CHILDREN")]
    CurrentGroupWithChildren,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessGroupEntityVersionedFlowState {
    #[default]
    #[serde(rename = "LOCALLY_MODIFIED")]
    LocallyModified,
    #[serde(rename = "STALE")]
    Stale,
    #[serde(rename = "LOCALLY_MODIFIED_AND_STALE")]
    LocallyModifiedAndStale,
    #[serde(rename = "UP_TO_DATE")]
    UpToDate,
    #[serde(rename = "SYNC_FAILURE")]
    SyncFailure,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupEntity {
    /// The number of active remote ports in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_remote_port_count: Option<i32>,
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ProcessGroupDto>,
    /// The number of disabled components in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_count: Option<i32>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The number of inactive remote ports in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_remote_port_count: Option<i32>,
    /// The number of input ports in the process group. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_port_count: Option<i32>,
    /// The number of invalid components in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_count: Option<i32>,
    /// The number of local input ports in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_input_port_count: Option<i32>,
    /// The number of local output ports in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_output_port_count: Option<i32>,
    /// The number of locally modified and stale versioned process groups in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locally_modified_and_stale_count: Option<i32>,
    /// The number of locally modified versioned process groups in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locally_modified_count: Option<i32>,
    /// The number of output ports in the process group. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_port_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// Determines the process group update strategy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_update_strategy: Option<ProcessGroupEntityProcessGroupUpdateStrategy>,
    /// The number of public input ports in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_input_port_count: Option<i32>,
    /// The number of public output ports in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_output_port_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The number of running components in this process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    /// The number of stale versioned process groups in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProcessGroupStatusDto>,
    /// The number of stopped components in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_count: Option<i32>,
    /// The number of versioned process groups in the process group that are unable to sync to a registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_failure_count: Option<i32>,
    /// The number of up to date versioned process groups in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_to_date_count: Option<i32>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
    /// The current state of the Process Group, as it relates to the Versioned Flow Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_state: Option<ProcessGroupEntityVersionedFlowState>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupImportEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupReplaceRequestEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<ProcessGroupReplaceRequestDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupUploadEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_snapshot: Option<RegisteredFlowSnapshot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_d_t_o: Option<PositionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_d_t_o: Option<RevisionDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<Vec<ProcessGroupEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<ProcessorEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<Vec<RemoteProcessGroupEntity>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ParameterContextHandlingStrategy {
    #[serde(rename = "KEEP_EXISTING")]
    KeepExisting,
    #[serde(rename = "REPLACE")]
    Replace,
}
impl std::fmt::Display for ParameterContextHandlingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ParameterContextHandlingStrategy::KeepExisting => "KEEP_EXISTING",
            ParameterContextHandlingStrategy::Replace => "REPLACE",
        };
        f.write_str(s)
    }
}
