// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#![allow(dead_code, private_interfaces, unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutDto {
    /// Build branch
    #[serde(rename = "buildBranch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_branch: Option<String>,
    /// Build revision or commit hash
    #[serde(rename = "buildRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_revision: Option<String>,
    /// Build tag
    #[serde(rename = "buildTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_tag: Option<String>,
    /// Build timestamp
    #[serde(rename = "buildTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_timestamp: Option<String>,
    /// The URL for the content viewer if configured.
    #[serde(rename = "contentViewerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_viewer_url: Option<String>,
    /// The timezone of the NiFi instance.
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
    pub about: Option<AboutDto>,
}
/// The actions.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ActionDto>,
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The timestamp of the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateControllerServicesEntity {
    /// Optional services to schedule. If not specified, all authorized descendant controller services will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the ProcessGroup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The desired state of the descendant components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalDetailsEntity {
    #[serde(rename = "additionalDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerDto {
    /// The footer text.
    #[serde(rename = "footerText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_text: Option<String>,
    /// The header text.
    #[serde(rename = "headerText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_text: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerEntity {
    pub banners: Option<BannerDto>,
}
#[non_exhaustive]
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
    pub bulletin_board: Option<BulletinBoardDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearBulletinsForGroupRequestEntity {
    /// Optional component IDs for which to clear bulletins. If not specified, all authorized descendant components will be used.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<String>>,
    /// The timestamp from which to clear bulletins (inclusive). This field is required.
    #[serde(rename = "fromTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<String>,
    /// The id of the ProcessGroup
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearBulletinsForGroupResultsEntity {
    /// The total number of bulletins that were cleared.
    #[serde(rename = "bulletinsCleared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins_cleared: Option<i32>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSearchResultsEntity {
    #[serde(rename = "nodeResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_results: Option<Vec<NodeSearchResultDto>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSummaryDto {
    /// Whether this NiFi instance is clustered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clustered: Option<bool>,
    /// The number of nodes that are currently connected to the cluster
    #[serde(rename = "connectedNodeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_node_count: Option<i32>,
    /// When clustered, reports the number of nodes connected vs the number of nodes in the cluster.
    #[serde(rename = "connectedNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_nodes: Option<String>,
    /// Whether this NiFi instance is connected to a cluster.
    #[serde(rename = "connectedToCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_to_cluster: Option<bool>,
    /// The number of nodes in the cluster, regardless of whether or not they are connected
    #[serde(rename = "totalNodeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_node_count: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSummaryEntity {
    pub cluster_summary: Option<ClusterSummaryDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatisticsEntity {
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(rename = "connectionStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_statistics: Option<ConnectionStatisticsDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusEntity {
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(rename = "connectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<ConnectionStatusDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentViewerEntity {
    /// The Content Viewers.
    #[serde(rename = "contentViewers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_viewers: Option<Vec<ContentViewerDto>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerBulletinsEntity {
    /// System level bulletins to be reported to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    /// Controller service bulletins to be reported to the user.
    #[serde(rename = "controllerServiceBulletins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_bulletins: Option<Vec<BulletinEntity>>,
    /// Flow Analysis Rule bulletins to be reported to the user.
    #[serde(rename = "flowAnalysisRuleBulletins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_analysis_rule_bulletins: Option<Vec<BulletinEntity>>,
    /// Flow registry client bulletins to be reported to the user.
    #[serde(rename = "flowRegistryClientBulletins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_registry_client_bulletins: Option<Vec<BulletinEntity>>,
    /// Parameter provider bulletins to be reported to the user.
    #[serde(rename = "parameterProviderBulletins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_bulletins: Option<Vec<BulletinEntity>>,
    /// Reporting task bulletins to be reported to the user.
    #[serde(rename = "reportingTaskBulletins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_task_bulletins: Option<Vec<BulletinEntity>>,
}
/// Controller Services provided in this bundle
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceDefinition {
    /// Indicates if the component has additional details documentation
    #[serde(rename = "additionalDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[serde(rename = "buildInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfo>,
    /// Whether or not the component has been deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    #[serde(rename = "deprecationAlternatives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    #[serde(rename = "deprecationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    #[serde(rename = "dynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    #[serde(rename = "explicitRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    #[serde(rename = "propertyDescriptors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    #[serde(rename = "providedApiImplementations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    #[serde(rename = "restrictedExplanation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    #[serde(rename = "seeAlso")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful: Option<Stateful>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    #[serde(rename = "supportsDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    #[serde(rename = "systemResourceConsiderations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The description of the type.
    #[serde(rename = "typeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceTypesEntity {
    #[serde(rename = "controllerServiceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_types: Option<Vec<DocumentedTypeDto>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServicesEntity {
    #[serde(rename = "controllerServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_services: Option<Vec<ControllerServiceEntity>>,
    /// The current time on the system.
    #[serde(rename = "currentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerStatusDto {
    /// The number of active remote ports in the NiFi.
    #[serde(rename = "activeRemotePortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_remote_port_count: Option<i32>,
    /// The number of active threads in the NiFi.
    #[serde(rename = "activeThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The size of the FlowFiles queued across the entire flow
    #[serde(rename = "bytesQueued")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_queued: Option<i64>,
    /// The number of disabled components in the NiFi.
    #[serde(rename = "disabledCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_count: Option<i32>,
    /// The number of FlowFiles queued across the entire flow
    #[serde(rename = "flowFilesQueued")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_queued: Option<i32>,
    /// The number of inactive remote ports in the NiFi.
    #[serde(rename = "inactiveRemotePortCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_remote_port_count: Option<i32>,
    /// The number of invalid components in the NiFi.
    #[serde(rename = "invalidCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_count: Option<i32>,
    /// The number of locally modified and stale versioned process groups in the NiFi.
    #[serde(rename = "locallyModifiedAndStaleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locally_modified_and_stale_count: Option<i32>,
    /// The number of locally modified versioned process groups in the NiFi.
    #[serde(rename = "locallyModifiedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locally_modified_count: Option<i32>,
    /// The number of flowfiles queued in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<String>,
    /// The number of running components in the NiFi.
    #[serde(rename = "runningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    /// The number of stale versioned process groups in the NiFi.
    #[serde(rename = "staleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_count: Option<i32>,
    /// The number of stopped components in the NiFi.
    #[serde(rename = "stoppedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_count: Option<i32>,
    /// The number of versioned process groups in the NiFi that are unable to sync to a registry.
    #[serde(rename = "syncFailureCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_failure_count: Option<i32>,
    /// The number of terminated threads in the NiFi.
    #[serde(rename = "terminatedThreadCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated_thread_count: Option<i32>,
    /// The number of up to date versioned process groups in the NiFi.
    #[serde(rename = "upToDateCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_to_date_count: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerStatusEntity {
    pub controller_status: Option<ControllerStatusDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUserEntity {
    /// Whether the current user is anonymous.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<bool>,
    /// Whether the current user can version flows.
    #[serde(rename = "canVersionFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_version_flows: Option<bool>,
    /// Permissions for specific component restrictions.
    #[serde(rename = "componentRestrictionPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_restriction_permissions: Option<Vec<ComponentRestrictionPermissionDto>>,
    #[serde(rename = "controllerPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_permissions: Option<PermissionsDto>,
    #[serde(rename = "countersPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters_permissions: Option<PermissionsDto>,
    /// The user identity being serialized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// Whether the system is configured to support logout operations based on current user authentication status
    #[serde(rename = "logoutSupported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logout_supported: Option<bool>,
    #[serde(rename = "parameterContextPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context_permissions: Option<PermissionsDto>,
    #[serde(rename = "policiesPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies_permissions: Option<PermissionsDto>,
    #[serde(rename = "provenancePermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance_permissions: Option<PermissionsDto>,
    #[serde(rename = "restrictedComponentsPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_components_permissions: Option<PermissionsDto>,
    #[serde(rename = "systemPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_permissions: Option<PermissionsDto>,
    #[serde(rename = "tenantsPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenants_permissions: Option<PermissionsDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisResultEntity {
    #[serde(rename = "flowAnalysisPending")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_analysis_pending: Option<bool>,
    #[serde(rename = "ruleViolations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_violations: Option<Vec<FlowAnalysisRuleViolationDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<FlowAnalysisRuleDto>>,
}
/// Flow Analysis Rules provided in this bundle
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleDefinition {
    /// Indicates if the component has additional details documentation
    #[serde(rename = "additionalDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[serde(rename = "buildInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfo>,
    /// Whether or not the component has been deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    #[serde(rename = "deprecationAlternatives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    #[serde(rename = "deprecationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    #[serde(rename = "dynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    #[serde(rename = "explicitRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    #[serde(rename = "propertyDescriptors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    #[serde(rename = "providedApiImplementations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    #[serde(rename = "restrictedExplanation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    #[serde(rename = "seeAlso")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful: Option<Stateful>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    #[serde(rename = "supportsDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    #[serde(rename = "systemResourceConsiderations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The description of the type.
    #[serde(rename = "typeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleTypesEntity {
    #[serde(rename = "flowAnalysisRuleTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_analysis_rule_types: Option<Vec<DocumentedTypeDto>>,
}
/// The breadcrumb of the process group.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowBreadcrumbEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breadcrumb: Option<FlowBreadcrumbDto>,
    /// The id of this ancestor ProcessGroup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "parentBreadcrumb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_breadcrumb: Option<Box<FlowBreadcrumbEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    /// The current state of the Process Group, as it relates to the Versioned Flow
    #[serde(rename = "versionedFlowState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_state: Option<String>,
}
/// The controller configuration.
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowConfigurationDto {
    /// The current time on the system.
    #[serde(rename = "currentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    /// The default back pressure data size threshold.
    #[serde(rename = "defaultBackPressureDataSizeThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_data_size_threshold: Option<String>,
    /// The default back pressure object threshold.
    #[serde(rename = "defaultBackPressureObjectThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_object_threshold: Option<i64>,
    /// Whether this NiFi supports a configurable authorizer.
    #[serde(rename = "supportsConfigurableAuthorizer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_configurable_authorizer: Option<bool>,
    /// Whether this NiFi supports configurable users and groups.
    #[serde(rename = "supportsConfigurableUsersAndGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_configurable_users_and_groups: Option<bool>,
    /// Whether this NiFi supports a managed authorizer. Managed authorizers can visualize users, groups, and policies in the UI.
    #[serde(rename = "supportsManagedAuthorizer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_managed_authorizer: Option<bool>,
    /// The time offset of the system.
    #[serde(rename = "timeOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_offset: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowConfigurationEntity {
    pub flow_configuration: Option<FlowConfigurationDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum FlowMetricsReportingStrategy {
    #[serde(rename = "ALL_COMPONENTS")]
    AllComponents,
    #[serde(rename = "ALL_PROCESS_GROUPS")]
    AllProcessGroups,
}
impl std::fmt::Display for FlowMetricsReportingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlowMetricsReportingStrategy::AllComponents => write!(f, "ALL_COMPONENTS"),
            FlowMetricsReportingStrategy::AllProcessGroups => {
                write!(f, "ALL_PROCESS_GROUPS")
            }
        }
    }
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBranchesEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<FlowRegistryBranchEntity>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBucketsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<FlowRegistryBucketEntity>>,
}
/// Flow Registry Clients provided in this bundle
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientDefinition {
    /// Indicates if the component has additional details documentation
    #[serde(rename = "additionalDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[serde(rename = "buildInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfo>,
    /// Whether or not the component has been deprecated
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    #[serde(rename = "deprecationAlternatives")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    #[serde(rename = "deprecationReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    #[serde(rename = "dynamicProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    #[serde(rename = "explicitRestrictions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    #[serde(rename = "propertyDescriptors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    #[serde(rename = "providedApiImplementations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    #[serde(rename = "restrictedExplanation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    #[serde(rename = "seeAlso")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful: Option<Stateful>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    #[serde(rename = "supportsDynamicProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    #[serde(rename = "systemResourceConsiderations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The description of the type.
    #[serde(rename = "typeDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum IncludedRegistries {
    #[serde(rename = "BULLETIN")]
    Bulletin,
    #[serde(rename = "CLUSTER")]
    Cluster,
    #[serde(rename = "CONNECTION")]
    Connection,
    #[serde(rename = "JVM")]
    Jvm,
    #[serde(rename = "NIFI")]
    Nifi,
    #[serde(rename = "VERSION_INFO")]
    VersionInfo,
}
impl std::fmt::Display for IncludedRegistries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IncludedRegistries::Bulletin => write!(f, "BULLETIN"),
            IncludedRegistries::Cluster => write!(f, "CLUSTER"),
            IncludedRegistries::Connection => write!(f, "CONNECTION"),
            IncludedRegistries::Jvm => write!(f, "JVM"),
            IncludedRegistries::Nifi => write!(f, "NIFI"),
            IncludedRegistries::VersionInfo => write!(f, "VERSION_INFO"),
        }
    }
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenPortsEntity {
    /// A list of ingress ports that are currently configured
    #[serde(rename = "listenPorts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_ports: Option<Vec<ListenPortDto>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextsEntity {
    /// The current time on the system.
    #[serde(rename = "currentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    /// The Parameter Contexts
    #[serde(rename = "parameterContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_contexts: Option<Vec<ParameterContextEntity>>,
}
/// Parameter Providers provided in this bundle
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderDefinition {
    /// Indicates if the component has additional details documentation
    #[serde(rename = "additionalDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[serde(rename = "buildInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfo>,
    /// Whether or not the component has been deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    #[serde(rename = "deprecationAlternatives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    #[serde(rename = "deprecationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    #[serde(rename = "dynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    #[serde(rename = "explicitRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    #[serde(rename = "propertyDescriptors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    #[serde(rename = "providedApiImplementations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    #[serde(rename = "restrictedExplanation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    #[serde(rename = "seeAlso")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful: Option<Stateful>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    #[serde(rename = "supportsDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    #[serde(rename = "systemResourceConsiderations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The description of the type.
    #[serde(rename = "typeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderTypesEntity {
    #[serde(rename = "parameterProviderTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_types: Option<Vec<DocumentedTypeDto>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProvidersEntity {
    /// The current time on the system.
    #[serde(rename = "currentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    #[serde(rename = "parameterProviders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_providers: Option<Vec<ParameterProviderEntity>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatusEntity {
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(rename = "portStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_status: Option<PortStatusDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrioritizerTypesEntity {
    #[serde(rename = "prioritizerTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prioritizer_types: Option<Vec<DocumentedTypeDto>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupFlowEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(rename = "processGroupFlow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_flow: Option<ProcessGroupFlowDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusEntity {
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(rename = "processGroupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_status: Option<ProcessGroupStatusDto>,
}
/// Processors provided in this bundle
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorDefinition {
    /// Indicates if the component has additional details documentation
    #[serde(rename = "additionalDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[serde(rename = "buildInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfo>,
    /// The default bulletin level, such as WARN, INFO, DEBUG, etc.
    #[serde(rename = "defaultBulletinLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_bulletin_level: Option<String>,
    /// The default concurrent tasks for each scheduling strategy.
    #[serde(rename = "defaultConcurrentTasksBySchedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_concurrent_tasks_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<i32>>>,
    /// The default penalty duration as a time period, such as "30 sec".
    #[serde(rename = "defaultPenaltyDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_penalty_duration: Option<String>,
    /// The default scheduling period for each scheduling strategy. The scheduling period is expected to be a time period, such as "30 sec".
    #[serde(rename = "defaultSchedulingPeriodBySchedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_period_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<String>>>,
    /// The default scheduling strategy for the processor.
    #[serde(rename = "defaultSchedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_strategy: Option<String>,
    /// The default yield duration as a time period, such as "1 sec".
    #[serde(rename = "defaultYieldDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_yield_duration: Option<String>,
    /// Whether or not the component has been deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    #[serde(rename = "deprecationAlternatives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    #[serde(rename = "deprecationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    #[serde(rename = "dynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    #[serde(rename = "dynamicRelationship")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_relationship: Option<DynamicRelationship>,
    /// Explicit restrictions that indicate a require permission to use the component
    #[serde(rename = "explicitRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Any input requirements this processor has.
    #[serde(rename = "inputRequirement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_requirement: Option<String>,
    /// A list of use cases that have been documented that involve this Processor in conjunction with other Processors
    #[serde(rename = "multiProcessorUseCases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_processor_use_cases: Option<Vec<MultiProcessorUseCase>>,
    /// Whether or not this processor should be scheduled only on the primary node in a cluster.
    #[serde(rename = "primaryNodeOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_node_only: Option<bool>,
    /// Descriptions of configuration properties applicable to this component.
    #[serde(rename = "propertyDescriptors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    #[serde(rename = "providedApiImplementations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// The FlowFile attributes this processor reads
    #[serde(rename = "readsAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reads_attributes: Option<Vec<Attribute>>,
    /// Whether or not the component has a general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    #[serde(rename = "restrictedExplanation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    #[serde(rename = "seeAlso")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<String>>,
    /// Whether or not this processor is considered side-effect free. Side-effect free indicate that the processor's operations on FlowFiles can be safely repeated across process sessions.
    #[serde(rename = "sideEffectFree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_effect_free: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful: Option<Stateful>,
    /// The supported relationships for this processor.
    #[serde(rename = "supportedRelationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_relationships: Option<Vec<Relationship>>,
    /// The supported scheduling strategies, such as TIME_DRIVER, CRON, or EVENT_DRIVEN.
    #[serde(rename = "supportedSchedulingStrategies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_scheduling_strategies: Option<Vec<String>>,
    /// Whether or not this processor supports batching. If a Processor uses this annotation, it allows the Framework to batch calls to session commits, as well as allowing the Framework to return the same session multiple times.
    #[serde(rename = "supportsBatching")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_batching: Option<bool>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    #[serde(rename = "supportsDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this processor supports dynamic relationships.
    #[serde(rename = "supportsDynamicRelationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_relationships: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    #[serde(rename = "systemResourceConsiderations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Whether or not this processor should be triggered serially (i.e. no concurrent execution).
    #[serde(rename = "triggerSerially")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_serially: Option<bool>,
    /// Whether or not this processor should be triggered when any destination queue has room.
    #[serde(rename = "triggerWhenAnyDestinationAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_when_any_destination_available: Option<bool>,
    /// Whether or not this processor should be triggered when incoming queues are empty.
    #[serde(rename = "triggerWhenEmpty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_when_empty: Option<bool>,
    /// The fully-qualified class type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The description of the type.
    #[serde(rename = "typeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    /// A list of use cases that have been documented for this Processor
    #[serde(rename = "useCases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_cases: Option<Vec<UseCase>>,
    /// The version of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The FlowFile attributes this processor writes/updates
    #[serde(rename = "writesAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writes_attributes: Option<Vec<Attribute>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorStatusEntity {
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(rename = "processorStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_status: Option<ProcessorStatusDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorTypesEntity {
    #[serde(rename = "processorTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_types: Option<Vec<DocumentedTypeDto>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusEntity {
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(rename = "remoteProcessGroupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_group_status: Option<RemoteProcessGroupStatusDto>,
}
/// Reporting Tasks provided in this bundle
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskDefinition {
    /// Indicates if the component has additional details documentation
    #[serde(rename = "additionalDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    #[serde(rename = "buildInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfo>,
    /// The default scheduling period for each scheduling strategy. The scheduling period is expected to be a time period, such as "30 sec".
    #[serde(rename = "defaultSchedulingPeriodBySchedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_period_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<String>>>,
    /// The default scheduling strategy for the reporting task.
    #[serde(rename = "defaultSchedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_strategy: Option<String>,
    /// Whether or not the component has been deprecated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    #[serde(rename = "deprecationAlternatives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    #[serde(rename = "deprecationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    #[serde(rename = "dynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    #[serde(rename = "explicitRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    #[serde(rename = "propertyDescriptors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    #[serde(rename = "providedApiImplementations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    #[serde(rename = "restrictedExplanation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    #[serde(rename = "seeAlso")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateful: Option<Stateful>,
    /// The supported scheduling strategies, such as TIME_DRIVER or CRON.
    #[serde(rename = "supportedSchedulingStrategies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_scheduling_strategies: Option<Vec<String>>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    #[serde(rename = "supportsDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    #[serde(rename = "systemResourceConsiderations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The description of the type.
    #[serde(rename = "typeDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskTypesEntity {
    #[serde(rename = "reportingTaskTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_task_types: Option<Vec<DocumentedTypeDto>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTasksEntity {
    /// The current time on the system.
    #[serde(rename = "currentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    #[serde(rename = "reportingTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_tasks: Option<Vec<ReportingTaskEntity>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeManifest {
    /// The type of the runtime binary, e.g., 'minifi-java' or 'minifi-cpp'
    #[serde(rename = "agentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<String>,
    #[serde(rename = "buildInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_info: Option<BuildInfo>,
    /// All extension bundles included with this runtime
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundles: Option<Vec<Bundle>>,
    /// A unique identifier for the manifest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "schedulingDefaults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_defaults: Option<SchedulingDefaults>,
    /// The version of the runtime binary, e.g., '1.0.1'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeManifestEntity {
    pub runtime_manifest: Option<RuntimeManifest>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleComponentsEntity {
    /// Optional components to schedule. If not specified, all authorized descendant components will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(rename = "disconnectedNodeAcknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the ProcessGroup
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The desired state of the descendant components
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultsDto {
    /// The connections that matched the search.
    #[serde(rename = "connectionResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_results: Option<Vec<ComponentSearchResultDto>>,
    /// The controller service nodes that matched the search
    #[serde(rename = "controllerServiceNodeResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_node_results: Option<Vec<ComponentSearchResultDto>>,
    /// The funnels that matched the search.
    #[serde(rename = "funnelResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnel_results: Option<Vec<ComponentSearchResultDto>>,
    /// The input ports that matched the search.
    #[serde(rename = "inputPortResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_port_results: Option<Vec<ComponentSearchResultDto>>,
    /// The labels that matched the search.
    #[serde(rename = "labelResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_results: Option<Vec<ComponentSearchResultDto>>,
    /// The output ports that matched the search.
    #[serde(rename = "outputPortResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_port_results: Option<Vec<ComponentSearchResultDto>>,
    /// The parameter contexts that matched the search.
    #[serde(rename = "parameterContextResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context_results: Option<Vec<ComponentSearchResultDto>>,
    /// The parameter provider nodes that matched the search
    #[serde(rename = "parameterProviderNodeResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_node_results: Option<Vec<ComponentSearchResultDto>>,
    /// The parameters that matched the search.
    #[serde(rename = "parameterResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_results: Option<Vec<ComponentSearchResultDto>>,
    /// The process groups that matched the search.
    #[serde(rename = "processGroupResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_results: Option<Vec<ComponentSearchResultDto>>,
    /// The processors that matched the search.
    #[serde(rename = "processorResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_results: Option<Vec<ComponentSearchResultDto>>,
    /// The remote process groups that matched the search.
    #[serde(rename = "remoteProcessGroupResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_group_results: Option<Vec<ComponentSearchResultDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultsEntity {
    pub search_results_d_t_o: Option<SearchResultsDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusHistoryEntity {
    /// Indicates whether the user can read a given resource.
    #[serde(rename = "canRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(rename = "statusHistory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_history: Option<StatusHistoryDto>,
}
/// The versioned flow
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowDto {
    /// The action being performed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// The branch where the flow is stored
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// The ID of the bucket where the flow is stored
    #[serde(rename = "bucketId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_id: Option<String>,
    /// Comments for the changeset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// A description of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the flow
    #[serde(rename = "flowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    /// The name of the flow
    #[serde(rename = "flowName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_name: Option<String>,
    /// The ID of the registry that the flow is tracked to
    #[serde(rename = "registryId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowEntity {
    pub versioned_flow: Option<VersionedFlowDto>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowSnapshotMetadataSetEntity {
    #[serde(rename = "versionedFlowSnapshotMetadataSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot_metadata_set: Option<Vec<VersionedFlowSnapshotMetadataEntity>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowsEntity {
    #[serde(rename = "versionedFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flows: Option<Vec<VersionedFlowEntity>>,
}
#[non_exhaustive]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedReportingTaskSnapshot {
    /// The controller services
    #[serde(rename = "controllerServices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_services: Option<Vec<VersionedControllerService>>,
    /// The reporting tasks
    #[serde(rename = "reportingTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_tasks: Option<Vec<VersionedReportingTask>>,
}
