// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#![allow(dead_code, private_interfaces, unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
/// The bulletins for this component.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletin: Option<BulletinDto>,
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "nodeAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_address: Option<String>,
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// When this bulletin was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// When this bulletin was generated in ISO format with full date and milliseconds.
    #[serde(rename = "timestampIso")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_iso: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterDto {
    /// The timestamp the report was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<String>,
    /// The collection of nodes that are part of the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<NodeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterEntity {
    pub cluster: Option<ClusterDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerConfigurationEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ControllerConfigurationDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<FlowAnalysisRuleDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "operatePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FlowAnalysisRuleStatusDto>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleRunStatusEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The state of the FlowAnalysisRule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRulesEntity {
    /// The current time on the system.
    #[serde(rename = "currentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    #[serde(rename = "flowAnalysisRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_analysis_rules: Option<Vec<FlowAnalysisRuleEntity>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<FlowRegistryClientDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "operatePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientTypesEntity {
    #[serde(rename = "flowRegistryClientTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_registry_client_types: Option<Vec<DocumentedTypeDto>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NarDetailsEntity {
    /// The ControllerService types contained in the NAR
    #[serde(rename = "controllerServiceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_types: Option<Vec<DocumentedTypeDto>>,
    /// The coordinates of NARs that depend on this NAR
    #[serde(rename = "dependentCoordinates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_coordinates: Option<Vec<NarCoordinateDto>>,
    /// The FlowAnalysisRule types contained in the NAR
    #[serde(rename = "flowAnalysisRuleTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_analysis_rule_types: Option<Vec<DocumentedTypeDto>>,
    /// The FlowRegistryClient types contained in the NAR
    #[serde(rename = "flowRegistryClientTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_registry_client_types: Option<Vec<DocumentedTypeDto>>,
    #[serde(rename = "narSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nar_summary: Option<NarSummaryDto>,
    /// The ParameterProvider types contained in the NAR
    #[serde(rename = "parameterProviderTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_types: Option<Vec<DocumentedTypeDto>>,
    /// The Processor types contained in the NAR
    #[serde(rename = "processorTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_types: Option<Vec<DocumentedTypeDto>>,
    /// The ReportingTask types contained in the NAR
    #[serde(rename = "reportingTaskTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_task_types: Option<Vec<DocumentedTypeDto>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NarSummariesEntity {
    /// The current time on the system.
    #[serde(rename = "currentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    /// The NAR summaries
    #[serde(rename = "narSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nar_summaries: Option<Vec<NarSummaryEntity>>,
}
/// The NAR summary
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NarSummaryDto {
    /// The time the NAR was built according to it's MANIFEST
    #[serde(rename = "buildTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate: Option<NarCoordinateDto>,
    /// The plugin that created the NAR according to it's MANIFEST
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "dependencyCoordinate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_coordinate: Option<NarCoordinateDto>,
    /// The hex digest of the NAR contents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// The number of extensions contained in this NAR
    #[serde(rename = "extensionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_count: Option<i32>,
    /// Information about why the installation failed, only populated when the state is failed
    #[serde(rename = "failureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// The identifier of the NAR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Indicates if the install task has completed
    #[serde(rename = "installComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_complete: Option<bool>,
    /// The identifier of the source of this NAR
    #[serde(rename = "sourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    /// The source of this NAR
    #[serde(rename = "sourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// The state of the NAR (i.e. Installed, or not)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NarSummaryEntity {
    pub nar_summary: Option<NarSummaryDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeDto {
    /// The active threads for the NiFi on the node.
    #[serde(rename = "activeThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The node's host/ip address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The port the node is listening for API requests.
    #[serde(rename = "apiPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The total size of all FlowFiles that are queued up on the node
    #[serde(rename = "bytesQueued")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_queued: Option<i64>,
    /// The time of the node's last connection request.
    #[serde(rename = "connectionRequested")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_requested: Option<String>,
    /// The node's events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<NodeEventDto>>,
    #[serde(rename = "flowFileBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_bytes: Option<i64>,
    /// The number of FlowFiles that are queued up on the node
    #[serde(rename = "flowFilesQueued")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_queued: Option<i32>,
    /// the time of the nodes's last heartbeat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heartbeat: Option<String>,
    /// The id of the node.
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// The time at which this Node was last refreshed.
    #[serde(rename = "nodeStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_start_time: Option<String>,
    /// The queue the NiFi on the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<String>,
    /// The roles of this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// The node's status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeEntity {
    pub node: Option<NodeDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedReportingTaskImportRequestEntity {
    /// The disconnected node acknowledged flag
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "reportingTaskSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_task_snapshot: Option<VersionedReportingTaskSnapshot>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedReportingTaskImportResponseEntity {
    /// The controller services created by the import
    #[serde(rename = "controllerServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_services: Option<Vec<ControllerServiceEntity>>,
    /// The reporting tasks created by the import
    #[serde(rename = "reportingTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_tasks: Option<Vec<ReportingTaskEntity>>,
}
