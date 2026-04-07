// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutDto {
    /// Build branch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_branch: Option<String>,
    /// Build revision or commit hash
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_revision: Option<String>,
    /// Build tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_tag: Option<String>,
    /// Build timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_timestamp: Option<String>,
    /// The URL for the content viewer if configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_viewer_url: Option<String>,
    /// The timezone of the NiFi instance. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// The title to be used on the page and in the about dialog.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URI for the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The version of this NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<AboutDto>,
}
/// The actions.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionDto>,
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The timestamp of the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ActivateControllerServicesEntityState {
    #[default]
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateControllerServicesEntity {
    /// Optional services to schedule. If not specified, all authorized descendant controller services will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the ProcessGroup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The desired state of the descendant components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ActivateControllerServicesEntityState>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalDetailsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerDto {
    /// The footer text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_text: Option<String>,
    /// The header text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_text: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banners: Option<BannerDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinBoardDto {
    /// The bulletins in the bulletin board, that matches the supplied request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    /// The timestamp when this report was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinBoardEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletin_board: Option<BulletinBoardDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearBulletinsForGroupRequestEntity {
    /// Optional component IDs for which to clear bulletins. If not specified, all authorized descendant components will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<String>>,
    /// The timestamp from which to clear bulletins (inclusive). This field is required.
    pub from_timestamp: String,
    /// The id of the ProcessGroup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearBulletinsForGroupResultsEntity {
    /// The total number of bulletins that were cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins_cleared: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSearchResultsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_results: Option<Vec<NodeSearchResultDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSummaryDto {
    /// Whether this NiFi instance is clustered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clustered: Option<bool>,
    /// The number of nodes that are currently connected to the cluster
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_node_count: Option<i32>,
    /// When clustered, reports the number of nodes connected vs the number of nodes in the cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_nodes: Option<String>,
    /// Whether this NiFi instance is connected to a cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_to_cluster: Option<bool>,
    /// The number of nodes in the cluster, regardless of whether or not they are connected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_node_count: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSummaryEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_summary: Option<ClusterSummaryDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatisticsEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_statistics: Option<ConnectionStatisticsDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<ConnectionStatusDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentViewerEntity {
    /// The Content Viewers. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_viewers: Option<Vec<ContentViewerDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerBulletinsEntity {
    /// System level bulletins to be reported to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    /// Controller service bulletins to be reported to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_bulletins: Option<Vec<BulletinEntity>>,
    /// Flow Analysis Rule bulletins to be reported to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_analysis_rule_bulletins: Option<Vec<BulletinEntity>>,
    /// Flow registry client bulletins to be reported to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_registry_client_bulletins: Option<Vec<BulletinEntity>>,
    /// Parameter provider bulletins to be reported to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_bulletins: Option<Vec<BulletinEntity>>,
    /// Reporting task bulletins to be reported to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_task_bulletins: Option<Vec<BulletinEntity>>,
}
/// Controller Services provided in this bundle
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceDefinition {
    /// Indicates if the component has additional details documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfo>,
    /// Whether or not the component has been deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful: Option<Stateful>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The description of the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceTypesEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_types: Option<Vec<DocumentedTypeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServicesEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_services: Option<Vec<ControllerServiceEntity>>,
    /// The current time on the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerStatusDto {
    /// The number of active remote ports in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_remote_port_count: Option<i32>,
    /// The number of active threads in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The size of the FlowFiles queued across the entire flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_queued: Option<i64>,
    /// The number of disabled components in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_count: Option<i32>,
    /// The number of FlowFiles queued across the entire flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_queued: Option<i32>,
    /// The number of inactive remote ports in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_remote_port_count: Option<i32>,
    /// The number of invalid components in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_count: Option<i32>,
    /// The number of locally modified and stale versioned process groups in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locally_modified_and_stale_count: Option<i32>,
    /// The number of locally modified versioned process groups in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locally_modified_count: Option<i32>,
    /// The number of flowfiles queued in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<String>,
    /// The number of running components in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    /// The number of stale versioned process groups in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_count: Option<i32>,
    /// The number of stopped components in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_count: Option<i32>,
    /// The number of versioned process groups in the NiFi that are unable to sync to a registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_failure_count: Option<i32>,
    /// The number of terminated threads in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated_thread_count: Option<i32>,
    /// The number of up to date versioned process groups in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_to_date_count: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerStatusEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_status: Option<ControllerStatusDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUserEntity {
    /// Whether the current user is anonymous.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<bool>,
    /// Whether the current user can version flows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_version_flows: Option<bool>,
    /// Permissions for specific component restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_restriction_permissions: Option<Vec<ComponentRestrictionPermissionDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters_permissions: Option<PermissionsDto>,
    /// The user identity being serialized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// Whether the system is configured to support logout operations based on current user authentication status Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_supported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context_permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies_permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance_permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_components_permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenants_permissions: Option<PermissionsDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisResultEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_analysis_pending: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_violations: Option<Vec<FlowAnalysisRuleViolationDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<FlowAnalysisRuleDto>>,
}
/// Flow Analysis Rules provided in this bundle
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleDefinition {
    /// Indicates if the component has additional details documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfo>,
    /// Whether or not the component has been deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful: Option<Stateful>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The description of the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleTypesEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_analysis_rule_types: Option<Vec<DocumentedTypeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum FlowBreadcrumbEntityVersionedFlowState {
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
/// The breadcrumb of the process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowBreadcrumbEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breadcrumb: Option<FlowBreadcrumbDto>,
    /// The id of this ancestor ProcessGroup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_breadcrumb: Option<Box<FlowBreadcrumbEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    /// The current state of the Process Group, as it relates to the Versioned Flow Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_state: Option<FlowBreadcrumbEntityVersionedFlowState>,
}
/// The controller configuration.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowConfigurationDto {
    /// The current time on the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    /// The default back pressure data size threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_data_size_threshold: Option<String>,
    /// The default back pressure object threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_object_threshold: Option<i64>,
    /// Whether this NiFi supports a configurable authorizer. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_configurable_authorizer: Option<bool>,
    /// Whether this NiFi supports configurable users and groups. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_configurable_users_and_groups: Option<bool>,
    /// Whether this NiFi supports a managed authorizer. Managed authorizers can visualize users, groups, and policies in the UI. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_managed_authorizer: Option<bool>,
    /// The time offset of the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_offset: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowConfigurationEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_configuration: Option<FlowConfigurationDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBranchesEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<FlowRegistryBranchEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBucketsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<FlowRegistryBucketEntity>>,
}
/// Flow Registry Clients provided in this bundle
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientDefinition {
    /// Indicates if the component has additional details documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfo>,
    /// Whether or not the component has been deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful: Option<Stateful>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The description of the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenPortsEntity {
    /// A list of ingress ports that are currently configured
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_ports: Option<Vec<ListenPortDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextsEntity {
    /// The current time on the system. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    /// The Parameter Contexts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_contexts: Option<Vec<ParameterContextEntity>>,
}
/// Parameter Providers provided in this bundle
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderDefinition {
    /// Indicates if the component has additional details documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfo>,
    /// Whether or not the component has been deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful: Option<Stateful>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The description of the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderTypesEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_types: Option<Vec<DocumentedTypeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProvidersEntity {
    /// The current time on the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_providers: Option<Vec<ParameterProviderEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatusEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_status: Option<PortStatusDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrioritizerTypesEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prioritizer_types: Option<Vec<DocumentedTypeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupFlowEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_flow: Option<ProcessGroupFlowDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_status: Option<ProcessGroupStatusDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessorDefinitionInputRequirement {
    #[default]
    #[serde(rename = "INPUT_REQUIRED")]
    InputRequired,
    #[serde(rename = "INPUT_ALLOWED")]
    InputAllowed,
    #[serde(rename = "INPUT_FORBIDDEN")]
    InputForbidden,
}
/// Processors provided in this bundle
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorDefinition {
    /// Indicates if the component has additional details documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfo>,
    /// The default bulletin level, such as WARN, INFO, DEBUG, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_bulletin_level: Option<String>,
    /// The default concurrent tasks for each scheduling strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_concurrent_tasks_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<i32>>>,
    /// The default penalty duration as a time period, such as "30 sec".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_penalty_duration: Option<String>,
    /// The default scheduling period for each scheduling strategy. The scheduling period is expected to be a time period, such as "30 sec".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_period_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<String>>>,
    /// The default scheduling strategy for the processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_strategy: Option<String>,
    /// The default yield duration as a time period, such as "1 sec".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_yield_duration: Option<String>,
    /// Whether or not the component has been deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_relationship: Option<DynamicRelationship>,
    /// Explicit restrictions that indicate a require permission to use the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Any input requirements this processor has.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_requirement: Option<ProcessorDefinitionInputRequirement>,
    /// A list of use cases that have been documented that involve this Processor in conjunction with other Processors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_processor_use_cases: Option<Vec<MultiProcessorUseCase>>,
    /// Whether or not this processor should be scheduled only on the primary node in a cluster.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_node_only: Option<bool>,
    /// Descriptions of configuration properties applicable to this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// The FlowFile attributes this processor reads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reads_attributes: Option<Vec<Attribute>>,
    /// Whether or not the component has a general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<String>>,
    /// Whether or not this processor is considered side-effect free. Side-effect free indicate that the processor's operations on FlowFiles can be safely repeated across process sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_effect_free: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful: Option<Stateful>,
    /// The supported relationships for this processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_relationships: Option<Vec<Relationship>>,
    /// The supported scheduling strategies, such as TIME_DRIVER, CRON, or EVENT_DRIVEN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_scheduling_strategies: Option<Vec<String>>,
    /// Whether or not this processor supports batching. If a Processor uses this annotation, it allows the Framework to batch calls to session commits, as well as allowing the Framework to return the same session multiple times.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_batching: Option<bool>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this processor supports dynamic relationships.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_relationships: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Whether or not this processor should be triggered serially (i.e. no concurrent execution).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_serially: Option<bool>,
    /// Whether or not this processor should be triggered when any destination queue has room.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_when_any_destination_available: Option<bool>,
    /// Whether or not this processor should be triggered when incoming queues are empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_when_empty: Option<bool>,
    /// The fully-qualified class type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The description of the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    /// A list of use cases that have been documented for this Processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_cases: Option<Vec<UseCase>>,
    /// The version of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The FlowFile attributes this processor writes/updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writes_attributes: Option<Vec<Attribute>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorStatusEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_status: Option<ProcessorStatusDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorTypesEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_types: Option<Vec<DocumentedTypeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_group_status: Option<RemoteProcessGroupStatusDto>,
}
/// Reporting Tasks provided in this bundle
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskDefinition {
    /// Indicates if the component has additional details documentation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfo>,
    /// The default scheduling period for each scheduling strategy. The scheduling period is expected to be a time period, such as "30 sec".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_period_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<String>>>,
    /// The default scheduling strategy for the reporting task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_strategy: Option<String>,
    /// Whether or not the component has been deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful: Option<Stateful>,
    /// The supported scheduling strategies, such as TIME_DRIVER or CRON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_scheduling_strategies: Option<Vec<String>>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The description of the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskTypesEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_task_types: Option<Vec<DocumentedTypeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTasksEntity {
    /// The current time on the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_tasks: Option<Vec<ReportingTaskEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeManifest {
    /// The type of the runtime binary, e.g., 'minifi-java' or 'minifi-cpp'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfo>,
    /// All extension bundles included with this runtime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundles: Option<Vec<Bundle>>,
    /// A unique identifier for the manifest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_defaults: Option<SchedulingDefaults>,
    /// The version of the runtime binary, e.g., '1.0.1'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeManifestEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_manifest: Option<RuntimeManifest>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ScheduleComponentsEntityState {
    #[default]
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleComponentsEntity {
    /// Optional components to schedule. If not specified, all authorized descendant components will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the ProcessGroup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The desired state of the descendant components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ScheduleComponentsEntityState>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultsDto {
    /// The connections that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_results: Option<Vec<ComponentSearchResultDto>>,
    /// The controller service nodes that matched the search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_node_results: Option<Vec<ComponentSearchResultDto>>,
    /// The funnels that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnel_results: Option<Vec<ComponentSearchResultDto>>,
    /// The input ports that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_port_results: Option<Vec<ComponentSearchResultDto>>,
    /// The labels that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_results: Option<Vec<ComponentSearchResultDto>>,
    /// The output ports that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_port_results: Option<Vec<ComponentSearchResultDto>>,
    /// The parameter contexts that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context_results: Option<Vec<ComponentSearchResultDto>>,
    /// The parameter provider nodes that matched the search
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_node_results: Option<Vec<ComponentSearchResultDto>>,
    /// The parameters that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_results: Option<Vec<ComponentSearchResultDto>>,
    /// The process groups that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_results: Option<Vec<ComponentSearchResultDto>>,
    /// The processors that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_results: Option<Vec<ComponentSearchResultDto>>,
    /// The remote process groups that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_group_results: Option<Vec<ComponentSearchResultDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_results_d_t_o: Option<SearchResultsDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusHistoryEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_history: Option<StatusHistoryDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedFlowDtoAction {
    #[default]
    #[serde(rename = "COMMIT")]
    Commit,
    #[serde(rename = "FORCE_COMMIT")]
    ForceCommit,
}
/// The versioned flow
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowDto {
    /// The action being performed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<VersionedFlowDtoAction>,
    /// The branch where the flow is stored
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// The ID of the bucket where the flow is stored
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_id: Option<String>,
    /// Comments for the changeset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// A description of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    /// The name of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_name: Option<String>,
    /// The ID of the registry that the flow is tracked to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow: Option<VersionedFlowDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowSnapshotMetadataSetEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot_metadata_set: Option<Vec<VersionedFlowSnapshotMetadataEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flows: Option<Vec<VersionedFlowEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedReportingTaskSnapshot {
    /// The controller services
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_services: Option<Vec<VersionedControllerService>>,
    /// The reporting tasks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_tasks: Option<Vec<VersionedReportingTask>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum IncludedRegistries {
    #[serde(rename = "NIFI")]
    Nifi,
    #[serde(rename = "JVM")]
    Jvm,
    #[serde(rename = "BULLETIN")]
    Bulletin,
    #[serde(rename = "CONNECTION")]
    Connection,
    #[serde(rename = "CLUSTER")]
    Cluster,
}
impl std::fmt::Display for IncludedRegistries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            IncludedRegistries::Nifi => "NIFI",
            IncludedRegistries::Jvm => "JVM",
            IncludedRegistries::Bulletin => "BULLETIN",
            IncludedRegistries::Connection => "CONNECTION",
            IncludedRegistries::Cluster => "CLUSTER",
        };
        f.write_str(s)
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FlowMetricsReportingStrategy {
    #[serde(rename = "ALL_PROCESS_GROUPS")]
    AllProcessGroups,
    #[serde(rename = "ALL_COMPONENTS")]
    AllComponents,
}
impl std::fmt::Display for FlowMetricsReportingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            FlowMetricsReportingStrategy::AllProcessGroups => "ALL_PROCESS_GROUPS",
            FlowMetricsReportingStrategy::AllComponents => "ALL_COMPONENTS",
        };
        f.write_str(s)
    }
}
