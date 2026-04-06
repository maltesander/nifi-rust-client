#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionsEntity {
    pub connections: Option<Vec<ConnectionEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyRequestEntity {
    /// The ids of the connections to be copied.
    pub connections: Option<Vec<String>>,
    /// The ids of the funnels to be copied.
    pub funnels: Option<Vec<String>>,
    /// The ids of the input ports to be copied.
    pub input_ports: Option<Vec<String>>,
    /// The ids of the labels to be copied.
    pub labels: Option<Vec<String>>,
    /// The ids of the output ports to be copied.
    pub output_ports: Option<Vec<String>>,
    /// The ids of the process groups to be copied.
    pub process_groups: Option<Vec<String>>,
    /// The ids of the processors to be copied.
    pub processors: Option<Vec<String>>,
    /// The ids of the remote process groups to be copied.
    pub remote_process_groups: Option<Vec<String>>,
}
/// The response from copying.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyResponseEntity {
    /// The connections being copied.
    pub connections: Option<Vec<VersionedConnection>>,
    /// The external controller service references.
    pub external_controller_service_references:
        Option<std::collections::HashMap<String, Option<ExternalControllerServiceReference>>>,
    /// The funnels being copied.
    pub funnels: Option<Vec<VersionedFunnel>>,
    /// The id for this copy action.
    pub id: Option<String>,
    /// The input ports being copied.
    pub input_ports: Option<Vec<VersionedPort>>,
    /// The labels being copied.
    pub labels: Option<Vec<VersionedLabel>>,
    /// The output ports being copied.
    pub output_ports: Option<Vec<VersionedPort>>,
    /// The referenced parameter contexts.
    pub parameter_contexts:
        Option<std::collections::HashMap<String, Option<VersionedParameterContext>>>,
    /// The referenced parameter providers.
    pub parameter_providers:
        Option<std::collections::HashMap<String, Option<ParameterProviderReference>>>,
    /// The process groups being copied.
    pub process_groups: Option<Vec<VersionedProcessGroup>>,
    /// The processors being copied.
    pub processors: Option<Vec<VersionedProcessor>>,
    /// The remote process groups being copied.
    pub remote_process_groups: Option<Vec<VersionedRemoteProcessGroup>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopySnippetRequestEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The x coordinate of the origin of the bounding box where the new components will be placed.
    pub origin_x: Option<f64>,
    /// The y coordinate of the origin of the bounding box where the new components will be placed.
    pub origin_y: Option<f64>,
    /// The identifier of the snippet.
    pub snippet_id: Option<String>,
}
/// Flow containing the components that were created as part of this paste action.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowDto {
    /// The connections in this flow.
    pub connections: Option<Vec<ConnectionEntity>>,
    /// The funnels in this flow.
    pub funnels: Option<Vec<FunnelEntity>>,
    /// The input ports in this flow.
    pub input_ports: Option<Vec<PortEntity>>,
    /// The labels in this flow.
    pub labels: Option<Vec<LabelEntity>>,
    /// The output ports in this flow.
    pub output_ports: Option<Vec<PortEntity>>,
    /// The process groups in this flow.
    pub process_groups: Option<Vec<ProcessGroupEntity>>,
    /// The processors in this flow.
    pub processors: Option<Vec<ProcessorEntity>>,
    /// The remote process groups in this flow.
    pub remote_process_groups: Option<Vec<RemoteProcessGroupEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowEntity {
    pub flow: FlowDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunnelsEntity {
    pub funnels: Option<Vec<FunnelEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputPortsEntity {
    pub input_ports: Option<Vec<PortEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelsEntity {
    pub labels: Option<Vec<LabelEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputPortsEntity {
    pub output_ports: Option<Vec<PortEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasteRequestEntity {
    pub copy_response: Option<CopyResponseEntity>,
    pub disconnected_node_acknowledged: Option<bool>,
    pub revision: Option<RevisionDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasteResponseEntity {
    pub flow: Option<FlowDto>,
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
    pub active_remote_port_count: Option<i32>,
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ProcessGroupDto>,
    /// The number of disabled components in the process group.
    pub disabled_count: Option<i32>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    /// The number of inactive remote ports in the process group.
    pub inactive_remote_port_count: Option<i32>,
    /// The number of input ports in the process group.
    pub input_port_count: Option<i32>,
    /// The number of invalid components in the process group.
    pub invalid_count: Option<i32>,
    /// The number of local input ports in the process group.
    pub local_input_port_count: Option<i32>,
    /// The number of local output ports in the process group.
    pub local_output_port_count: Option<i32>,
    /// The number of locally modified and stale versioned process groups in the process group.
    pub locally_modified_and_stale_count: Option<i32>,
    /// The number of locally modified versioned process groups in the process group.
    pub locally_modified_count: Option<i32>,
    /// The number of output ports in the process group.
    pub output_port_count: Option<i32>,
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    /// Determines the process group update strategy
    pub process_group_update_strategy: Option<ProcessGroupEntityProcessGroupUpdateStrategy>,
    /// The number of public input ports in the process group.
    pub public_input_port_count: Option<i32>,
    /// The number of public output ports in the process group.
    pub public_output_port_count: Option<i32>,
    pub revision: Option<RevisionDto>,
    /// The number of running components in this process group.
    pub running_count: Option<i32>,
    /// The number of stale versioned process groups in the process group.
    pub stale_count: Option<i32>,
    pub status: Option<ProcessGroupStatusDto>,
    /// The number of stopped components in the process group.
    pub stopped_count: Option<i32>,
    /// The number of versioned process groups in the process group that are unable to sync to a registry.
    pub sync_failure_count: Option<i32>,
    /// The number of up to date versioned process groups in the process group.
    pub up_to_date_count: Option<i32>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
    /// The current state of the Process Group, as it relates to the Versioned Flow
    pub versioned_flow_state: Option<ProcessGroupEntityVersionedFlowState>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupImportEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    pub process_group_revision: Option<RevisionDto>,
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupReplaceRequestEntity {
    pub process_group_revision: Option<RevisionDto>,
    pub request: Option<ProcessGroupReplaceRequestDto>,
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupUploadEntity {
    pub disconnected_node_acknowledged: Option<bool>,
    pub flow_snapshot: Option<RegisteredFlowSnapshot>,
    pub group_id: Option<String>,
    pub group_name: Option<String>,
    pub position_d_t_o: Option<PositionDto>,
    pub revision_d_t_o: Option<RevisionDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupsEntity {
    pub process_groups: Option<Vec<ProcessGroupEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorsEntity {
    pub processors: Option<Vec<ProcessorEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupsEntity {
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
