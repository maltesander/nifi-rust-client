// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
/// The bulletins for this component.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinEntity {
    pub bulletin: Option<BulletinDto>,
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
    pub group_id: Option<String>,
    pub id: Option<i64>,
    pub node_address: Option<String>,
    pub source_id: Option<String>,
    /// When this bulletin was generated.
    pub timestamp: Option<String>,
    /// When this bulletin was generated in ISO format with full date and milliseconds.
    pub timestamp_iso: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterDto {
    /// The timestamp the report was generated.
    pub generated: Option<String>,
    /// The collection of nodes that are part of the cluster.
    pub nodes: Option<Vec<NodeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterEntity {
    pub cluster: ClusterDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerConfigurationEntity {
    pub component: Option<ControllerConfigurationDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    pub permissions: Option<PermissionsDto>,
    pub revision: Option<RevisionDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<FlowAnalysisRuleDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    pub operate_permissions: Option<PermissionsDto>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub status: Option<FlowAnalysisRuleStatusDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum FlowAnalysisRuleRunStatusEntityState {
    #[default]
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleRunStatusEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    pub revision: Option<RevisionDto>,
    /// The state of the FlowAnalysisRule.
    pub state: Option<FlowAnalysisRuleRunStatusEntityState>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRulesEntity {
    /// The current time on the system.
    pub current_time: Option<String>,
    pub flow_analysis_rules: Option<Vec<FlowAnalysisRuleEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<FlowRegistryClientDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    pub operate_permissions: Option<PermissionsDto>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientTypesEntity {
    pub flow_registry_client_types: Option<Vec<DocumentedTypeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NarDetailsEntity {
    /// The ControllerService types contained in the NAR
    pub controller_service_types: Option<Vec<DocumentedTypeDto>>,
    /// The coordinates of NARs that depend on this NAR
    pub dependent_coordinates: Option<Vec<NarCoordinateDto>>,
    /// The FlowAnalysisRule types contained in the NAR
    pub flow_analysis_rule_types: Option<Vec<DocumentedTypeDto>>,
    /// The FlowRegistryClient types contained in the NAR
    pub flow_registry_client_types: Option<Vec<DocumentedTypeDto>>,
    pub nar_summary: Option<NarSummaryDto>,
    /// The ParameterProvider types contained in the NAR
    pub parameter_provider_types: Option<Vec<DocumentedTypeDto>>,
    /// The Processor types contained in the NAR
    pub processor_types: Option<Vec<DocumentedTypeDto>>,
    /// The ReportingTask types contained in the NAR
    pub reporting_task_types: Option<Vec<DocumentedTypeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NarSummariesEntity {
    /// The current time on the system.
    pub current_time: Option<String>,
    /// The NAR summaries
    pub nar_summaries: Option<Vec<NarSummaryEntity>>,
}
/// The NAR summary
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NarSummaryDto {
    /// The time the NAR was built according to it's MANIFEST
    pub build_time: Option<String>,
    pub coordinate: Option<NarCoordinateDto>,
    /// The plugin that created the NAR according to it's MANIFEST
    pub created_by: Option<String>,
    pub dependency_coordinate: Option<NarCoordinateDto>,
    /// The hex digest of the NAR contents
    pub digest: Option<String>,
    /// The number of extensions contained in this NAR
    pub extension_count: Option<i32>,
    /// Information about why the installation failed, only populated when the state is failed
    pub failure_message: Option<String>,
    /// The identifier of the NAR.
    pub identifier: Option<String>,
    /// Indicates if the install task has completed
    pub install_complete: Option<bool>,
    /// The identifier of the source of this NAR
    pub source_identifier: Option<String>,
    /// The source of this NAR
    pub source_type: Option<String>,
    /// The state of the NAR (i.e. Installed, or not)
    pub state: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NarSummaryEntity {
    pub nar_summary: NarSummaryDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeDto {
    /// The active threads for the NiFi on the node. Read-only — set by NiFi.
    pub active_thread_count: Option<i32>,
    /// The node's host/ip address. Read-only — set by NiFi.
    pub address: Option<String>,
    /// The port the node is listening for API requests. Read-only — set by NiFi.
    pub api_port: Option<i32>,
    /// The total size of all FlowFiles that are queued up on the node Read-only — set by NiFi.
    pub bytes_queued: Option<i64>,
    /// The time of the node's last connection request. Read-only — set by NiFi.
    pub connection_requested: Option<String>,
    /// The node's events. Read-only — set by NiFi.
    pub events: Option<Vec<NodeEventDto>>,
    pub flow_file_bytes: Option<i64>,
    /// The number of FlowFiles that are queued up on the node Read-only — set by NiFi.
    pub flow_files_queued: Option<i32>,
    /// the time of the nodes's last heartbeat. Read-only — set by NiFi.
    pub heartbeat: Option<String>,
    /// The id of the node. Read-only — set by NiFi.
    pub node_id: Option<String>,
    /// The time at which this Node was last refreshed. Read-only — set by NiFi.
    pub node_start_time: Option<String>,
    /// The queue the NiFi on the node. Read-only — set by NiFi.
    pub queued: Option<String>,
    /// The roles of this node. Read-only — set by NiFi.
    pub roles: Option<Vec<String>>,
    /// The node's status.
    pub status: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeEntity {
    pub node: NodeDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedReportingTaskImportRequestEntity {
    /// The disconnected node acknowledged flag
    pub disconnected_node_acknowledged: Option<bool>,
    pub reporting_task_snapshot: Option<VersionedReportingTaskSnapshot>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedReportingTaskImportResponseEntity {
    /// The controller services created by the import
    pub controller_services: Option<Vec<ControllerServiceEntity>>,
    /// The reporting tasks created by the import
    pub reporting_tasks: Option<Vec<ReportingTaskEntity>>,
}
