// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutDto {
    /// Build branch
    pub build_branch: Option<String>,
    /// Build revision or commit hash
    pub build_revision: Option<String>,
    /// Build tag
    pub build_tag: Option<String>,
    /// Build timestamp
    pub build_timestamp: Option<String>,
    /// The URL for the content viewer if configured.
    pub content_viewer_url: Option<String>,
    /// The timezone of the NiFi instance. Read-only — set by NiFi.
    pub timezone: Option<String>,
    /// The title to be used on the page and in the about dialog.
    pub title: Option<String>,
    /// The URI for the NiFi.
    pub uri: Option<String>,
    /// The version of this NiFi.
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutEntity {
    pub about: AboutDto,
}
/// The actions.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionEntity {
    pub action: Option<ActionDto>,
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
    pub id: Option<i32>,
    pub source_id: Option<String>,
    /// The timestamp of the action.
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
    pub components: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the ProcessGroup
    pub id: Option<String>,
    /// The desired state of the descendant components
    pub state: Option<ActivateControllerServicesEntityState>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalDetailsEntity {
    pub additional_details: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerDto {
    /// The footer text.
    pub footer_text: Option<String>,
    /// The header text.
    pub header_text: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerEntity {
    pub banners: BannerDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinBoardDto {
    /// The bulletins in the bulletin board, that matches the supplied request.
    pub bulletins: Option<Vec<BulletinEntity>>,
    /// The timestamp when this report was generated.
    pub generated: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinBoardEntity {
    pub bulletin_board: BulletinBoardDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearBulletinsForGroupRequestEntity {
    /// Optional component IDs for which to clear bulletins. If not specified, all authorized descendant components will be used.
    pub components: Option<Vec<String>>,
    /// The timestamp from which to clear bulletins (inclusive). This field is required.
    pub from_timestamp: String,
    /// The id of the ProcessGroup
    pub id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearBulletinsForGroupResultsEntity {
    /// The total number of bulletins that were cleared.
    pub bulletins_cleared: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSearchResultsEntity {
    pub node_results: Option<Vec<NodeSearchResultDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSummaryDto {
    /// Whether this NiFi instance is clustered.
    pub clustered: Option<bool>,
    /// The number of nodes that are currently connected to the cluster
    pub connected_node_count: Option<i32>,
    /// When clustered, reports the number of nodes connected vs the number of nodes in the cluster.
    pub connected_nodes: Option<String>,
    /// Whether this NiFi instance is connected to a cluster.
    pub connected_to_cluster: Option<bool>,
    /// The number of nodes in the cluster, regardless of whether or not they are connected
    pub total_node_count: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSummaryEntity {
    pub cluster_summary: ClusterSummaryDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatisticsEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
    pub connection_statistics: Option<ConnectionStatisticsDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
    pub connection_status: Option<ConnectionStatusDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentViewerEntity {
    /// The Content Viewers. Read-only — set by NiFi.
    pub content_viewers: Option<Vec<ContentViewerDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerBulletinsEntity {
    /// System level bulletins to be reported to the user.
    pub bulletins: Option<Vec<BulletinEntity>>,
    /// Controller service bulletins to be reported to the user.
    pub controller_service_bulletins: Option<Vec<BulletinEntity>>,
    /// Flow Analysis Rule bulletins to be reported to the user.
    pub flow_analysis_rule_bulletins: Option<Vec<BulletinEntity>>,
    /// Flow registry client bulletins to be reported to the user.
    pub flow_registry_client_bulletins: Option<Vec<BulletinEntity>>,
    /// Parameter provider bulletins to be reported to the user.
    pub parameter_provider_bulletins: Option<Vec<BulletinEntity>>,
    /// Reporting task bulletins to be reported to the user.
    pub reporting_task_bulletins: Option<Vec<BulletinEntity>>,
}
/// Controller Services provided in this bundle
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceDefinition {
    /// Indicates if the component has additional details documentation
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    pub artifact: Option<String>,
    pub build_info: Option<BuildInfo>,
    /// Whether or not the component has been deprecated
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    pub see_also: Option<Vec<String>>,
    pub stateful: Option<Stateful>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The description of the type.
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceTypesEntity {
    pub controller_service_types: Option<Vec<DocumentedTypeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServicesEntity {
    pub controller_services: Option<Vec<ControllerServiceEntity>>,
    /// The current time on the system.
    pub current_time: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerStatusDto {
    /// The number of active remote ports in the NiFi.
    pub active_remote_port_count: Option<i32>,
    /// The number of active threads in the NiFi.
    pub active_thread_count: Option<i32>,
    /// The size of the FlowFiles queued across the entire flow
    pub bytes_queued: Option<i64>,
    /// The number of disabled components in the NiFi.
    pub disabled_count: Option<i32>,
    /// The number of FlowFiles queued across the entire flow
    pub flow_files_queued: Option<i32>,
    /// The number of inactive remote ports in the NiFi.
    pub inactive_remote_port_count: Option<i32>,
    /// The number of invalid components in the NiFi.
    pub invalid_count: Option<i32>,
    /// The number of locally modified and stale versioned process groups in the NiFi.
    pub locally_modified_and_stale_count: Option<i32>,
    /// The number of locally modified versioned process groups in the NiFi.
    pub locally_modified_count: Option<i32>,
    /// The number of flowfiles queued in the NiFi.
    pub queued: Option<String>,
    /// The number of running components in the NiFi.
    pub running_count: Option<i32>,
    /// The number of stale versioned process groups in the NiFi.
    pub stale_count: Option<i32>,
    /// The number of stopped components in the NiFi.
    pub stopped_count: Option<i32>,
    /// The number of versioned process groups in the NiFi that are unable to sync to a registry.
    pub sync_failure_count: Option<i32>,
    /// The number of terminated threads in the NiFi.
    pub terminated_thread_count: Option<i32>,
    /// The number of up to date versioned process groups in the NiFi.
    pub up_to_date_count: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerStatusEntity {
    pub controller_status: ControllerStatusDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUserEntity {
    /// Whether the current user is anonymous.
    pub anonymous: Option<bool>,
    /// Whether the current user can version flows.
    pub can_version_flows: Option<bool>,
    /// Permissions for specific component restrictions.
    pub component_restriction_permissions: Option<Vec<ComponentRestrictionPermissionDto>>,
    pub controller_permissions: Option<PermissionsDto>,
    pub counters_permissions: Option<PermissionsDto>,
    /// The user identity being serialized.
    pub identity: Option<String>,
    /// Whether the system is configured to support logout operations based on current user authentication status Read-only — set by NiFi.
    pub logout_supported: Option<bool>,
    pub parameter_context_permissions: Option<PermissionsDto>,
    pub policies_permissions: Option<PermissionsDto>,
    pub provenance_permissions: Option<PermissionsDto>,
    pub restricted_components_permissions: Option<PermissionsDto>,
    pub system_permissions: Option<PermissionsDto>,
    pub tenants_permissions: Option<PermissionsDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisResultEntity {
    pub flow_analysis_pending: Option<bool>,
    pub rule_violations: Option<Vec<FlowAnalysisRuleViolationDto>>,
    pub rules: Option<Vec<FlowAnalysisRuleDto>>,
}
/// Flow Analysis Rules provided in this bundle
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleDefinition {
    /// Indicates if the component has additional details documentation
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    pub artifact: Option<String>,
    pub build_info: Option<BuildInfo>,
    /// Whether or not the component has been deprecated
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    pub see_also: Option<Vec<String>>,
    pub stateful: Option<Stateful>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The description of the type.
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleTypesEntity {
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
    pub breadcrumb: Option<FlowBreadcrumbDto>,
    /// The id of this ancestor ProcessGroup.
    pub id: Option<String>,
    pub parent_breadcrumb: Option<Box<FlowBreadcrumbEntity>>,
    pub permissions: Option<PermissionsDto>,
    /// The current state of the Process Group, as it relates to the Versioned Flow Read-only — set by NiFi.
    pub versioned_flow_state: Option<FlowBreadcrumbEntityVersionedFlowState>,
}
/// The controller configuration.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowConfigurationDto {
    /// The current time on the system.
    pub current_time: Option<String>,
    /// The default back pressure data size threshold.
    pub default_back_pressure_data_size_threshold: Option<String>,
    /// The default back pressure object threshold.
    pub default_back_pressure_object_threshold: Option<i64>,
    /// Whether this NiFi supports a configurable authorizer. Read-only — set by NiFi.
    pub supports_configurable_authorizer: Option<bool>,
    /// Whether this NiFi supports configurable users and groups. Read-only — set by NiFi.
    pub supports_configurable_users_and_groups: Option<bool>,
    /// Whether this NiFi supports a managed authorizer. Managed authorizers can visualize users, groups, and policies in the UI. Read-only — set by NiFi.
    pub supports_managed_authorizer: Option<bool>,
    /// The time offset of the system.
    pub time_offset: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowConfigurationEntity {
    pub flow_configuration: FlowConfigurationDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBranchesEntity {
    pub branches: Option<Vec<FlowRegistryBranchEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBucketsEntity {
    pub buckets: Option<Vec<FlowRegistryBucketEntity>>,
}
/// Flow Registry Clients provided in this bundle
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientDefinition {
    /// Indicates if the component has additional details documentation
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    pub artifact: Option<String>,
    pub build_info: Option<BuildInfo>,
    /// Whether or not the component has been deprecated
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    pub see_also: Option<Vec<String>>,
    pub stateful: Option<Stateful>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The description of the type.
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenPortsEntity {
    /// A list of ingress ports that are currently configured
    pub listen_ports: Option<Vec<ListenPortDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextsEntity {
    /// The current time on the system. Read-only — set by NiFi.
    pub current_time: Option<String>,
    /// The Parameter Contexts
    pub parameter_contexts: Option<Vec<ParameterContextEntity>>,
}
/// Parameter Providers provided in this bundle
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderDefinition {
    /// Indicates if the component has additional details documentation
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    pub artifact: Option<String>,
    pub build_info: Option<BuildInfo>,
    /// Whether or not the component has been deprecated
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    pub see_also: Option<Vec<String>>,
    pub stateful: Option<Stateful>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The description of the type.
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderTypesEntity {
    pub parameter_provider_types: Option<Vec<DocumentedTypeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProvidersEntity {
    /// The current time on the system.
    pub current_time: Option<String>,
    pub parameter_providers: Option<Vec<ParameterProviderEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatusEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
    pub port_status: Option<PortStatusDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrioritizerTypesEntity {
    pub prioritizer_types: Option<Vec<DocumentedTypeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupFlowEntity {
    pub permissions: Option<PermissionsDto>,
    pub process_group_flow: Option<ProcessGroupFlowDto>,
    pub revision: Option<RevisionDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
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
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    pub artifact: Option<String>,
    pub build_info: Option<BuildInfo>,
    /// The default bulletin level, such as WARN, INFO, DEBUG, etc.
    pub default_bulletin_level: Option<String>,
    /// The default concurrent tasks for each scheduling strategy.
    pub default_concurrent_tasks_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<i32>>>,
    /// The default penalty duration as a time period, such as "30 sec".
    pub default_penalty_duration: Option<String>,
    /// The default scheduling period for each scheduling strategy. The scheduling period is expected to be a time period, such as "30 sec".
    pub default_scheduling_period_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<String>>>,
    /// The default scheduling strategy for the processor.
    pub default_scheduling_strategy: Option<String>,
    /// The default yield duration as a time period, such as "1 sec".
    pub default_yield_duration: Option<String>,
    /// Whether or not the component has been deprecated
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    pub dynamic_relationship: Option<DynamicRelationship>,
    /// Explicit restrictions that indicate a require permission to use the component
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    pub group: Option<String>,
    /// Any input requirements this processor has.
    pub input_requirement: Option<ProcessorDefinitionInputRequirement>,
    /// A list of use cases that have been documented that involve this Processor in conjunction with other Processors
    pub multi_processor_use_cases: Option<Vec<MultiProcessorUseCase>>,
    /// Whether or not this processor should be scheduled only on the primary node in a cluster.
    pub primary_node_only: Option<bool>,
    /// Descriptions of configuration properties applicable to this component.
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// The FlowFile attributes this processor reads
    pub reads_attributes: Option<Vec<Attribute>>,
    /// Whether or not the component has a general restriction
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    pub see_also: Option<Vec<String>>,
    /// Whether or not this processor is considered side-effect free. Side-effect free indicate that the processor's operations on FlowFiles can be safely repeated across process sessions.
    pub side_effect_free: Option<bool>,
    pub stateful: Option<Stateful>,
    /// The supported relationships for this processor.
    pub supported_relationships: Option<Vec<Relationship>>,
    /// The supported scheduling strategies, such as TIME_DRIVER, CRON, or EVENT_DRIVEN.
    pub supported_scheduling_strategies: Option<Vec<String>>,
    /// Whether or not this processor supports batching. If a Processor uses this annotation, it allows the Framework to batch calls to session commits, as well as allowing the Framework to return the same session multiple times.
    pub supports_batching: Option<bool>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this processor supports dynamic relationships.
    pub supports_dynamic_relationships: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    pub tags: Option<Vec<String>>,
    /// Whether or not this processor should be triggered serially (i.e. no concurrent execution).
    pub trigger_serially: Option<bool>,
    /// Whether or not this processor should be triggered when any destination queue has room.
    pub trigger_when_any_destination_available: Option<bool>,
    /// Whether or not this processor should be triggered when incoming queues are empty.
    pub trigger_when_empty: Option<bool>,
    /// The fully-qualified class type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The description of the type.
    pub type_description: Option<String>,
    /// A list of use cases that have been documented for this Processor
    pub use_cases: Option<Vec<UseCase>>,
    /// The version of the bundle that provides the referenced type.
    pub version: Option<String>,
    /// The FlowFile attributes this processor writes/updates
    pub writes_attributes: Option<Vec<Attribute>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorStatusEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
    pub processor_status: Option<ProcessorStatusDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorTypesEntity {
    pub processor_types: Option<Vec<DocumentedTypeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
    pub remote_process_group_status: Option<RemoteProcessGroupStatusDto>,
}
/// Reporting Tasks provided in this bundle
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskDefinition {
    /// Indicates if the component has additional details documentation
    pub additional_details: Option<bool>,
    /// The artifact name of the bundle that provides the referenced type.
    pub artifact: Option<String>,
    pub build_info: Option<BuildInfo>,
    /// The default scheduling period for each scheduling strategy. The scheduling period is expected to be a time period, such as "30 sec".
    pub default_scheduling_period_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<String>>>,
    /// The default scheduling strategy for the reporting task.
    pub default_scheduling_strategy: Option<String>,
    /// Whether or not the component has been deprecated
    pub deprecated: Option<bool>,
    /// If this component has been deprecated, this optional field provides alternatives to use
    pub deprecation_alternatives: Option<Vec<String>>,
    /// If this component has been deprecated, this optional field can be used to provide an explanation
    pub deprecation_reason: Option<String>,
    /// Describes the dynamic properties supported by this component
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    /// Explicit restrictions that indicate a require permission to use the component
    pub explicit_restrictions: Option<Vec<Restriction>>,
    /// The group name of the bundle that provides the referenced type.
    pub group: Option<String>,
    /// Descriptions of configuration properties applicable to this component.
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    /// If this type represents a provider for an interface, this lists the APIs it implements
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    /// Whether or not the component has a general restriction
    pub restricted: Option<bool>,
    /// An optional description of the general restriction
    pub restricted_explanation: Option<String>,
    /// The names of other component types that may be related
    pub see_also: Option<Vec<String>>,
    pub stateful: Option<Stateful>,
    /// The supported scheduling strategies, such as TIME_DRIVER or CRON.
    pub supported_scheduling_strategies: Option<Vec<String>>,
    /// Whether or not this component makes use of dynamic (user-set) properties.
    pub supports_dynamic_properties: Option<bool>,
    /// Whether or not this component makes use of sensitive dynamic (user-set) properties.
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The system resource considerations for the given component
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    /// The tags associated with this type
    pub tags: Option<Vec<String>>,
    /// The fully-qualified class type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The description of the type.
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskTypesEntity {
    pub reporting_task_types: Option<Vec<DocumentedTypeDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTasksEntity {
    /// The current time on the system.
    pub current_time: Option<String>,
    pub reporting_tasks: Option<Vec<ReportingTaskEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeManifest {
    /// The type of the runtime binary, e.g., 'minifi-java' or 'minifi-cpp'
    pub agent_type: Option<String>,
    pub build_info: Option<BuildInfo>,
    /// All extension bundles included with this runtime
    pub bundles: Option<Vec<Bundle>>,
    /// A unique identifier for the manifest
    pub identifier: Option<String>,
    pub scheduling_defaults: Option<SchedulingDefaults>,
    /// The version of the runtime binary, e.g., '1.0.1'
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeManifestEntity {
    pub runtime_manifest: RuntimeManifest,
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
    pub components: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the ProcessGroup
    pub id: Option<String>,
    /// The desired state of the descendant components
    pub state: Option<ScheduleComponentsEntityState>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultsDto {
    /// The connections that matched the search.
    pub connection_results: Option<Vec<ComponentSearchResultDto>>,
    /// The controller service nodes that matched the search
    pub controller_service_node_results: Option<Vec<ComponentSearchResultDto>>,
    /// The funnels that matched the search.
    pub funnel_results: Option<Vec<ComponentSearchResultDto>>,
    /// The input ports that matched the search.
    pub input_port_results: Option<Vec<ComponentSearchResultDto>>,
    /// The labels that matched the search.
    pub label_results: Option<Vec<ComponentSearchResultDto>>,
    /// The output ports that matched the search.
    pub output_port_results: Option<Vec<ComponentSearchResultDto>>,
    /// The parameter contexts that matched the search.
    pub parameter_context_results: Option<Vec<ComponentSearchResultDto>>,
    /// The parameter provider nodes that matched the search
    pub parameter_provider_node_results: Option<Vec<ComponentSearchResultDto>>,
    /// The parameters that matched the search.
    pub parameter_results: Option<Vec<ComponentSearchResultDto>>,
    /// The process groups that matched the search.
    pub process_group_results: Option<Vec<ComponentSearchResultDto>>,
    /// The processors that matched the search.
    pub processor_results: Option<Vec<ComponentSearchResultDto>>,
    /// The remote process groups that matched the search.
    pub remote_process_group_results: Option<Vec<ComponentSearchResultDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultsEntity {
    pub search_results_d_t_o: SearchResultsDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusHistoryEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
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
    pub action: Option<VersionedFlowDtoAction>,
    /// The branch where the flow is stored
    pub branch: Option<String>,
    /// The ID of the bucket where the flow is stored
    pub bucket_id: Option<String>,
    /// Comments for the changeset
    pub comments: Option<String>,
    /// A description of the flow
    pub description: Option<String>,
    /// The ID of the flow
    pub flow_id: Option<String>,
    /// The name of the flow
    pub flow_name: Option<String>,
    /// The ID of the registry that the flow is tracked to
    pub registry_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowEntity {
    pub versioned_flow: VersionedFlowDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowSnapshotMetadataSetEntity {
    pub versioned_flow_snapshot_metadata_set: Option<Vec<VersionedFlowSnapshotMetadataEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowsEntity {
    pub versioned_flows: Option<Vec<VersionedFlowEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedReportingTaskSnapshot {
    /// The controller services
    pub controller_services: Option<Vec<VersionedControllerService>>,
    /// The reporting tasks
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
    #[serde(rename = "VERSION_INFO")]
    VersionInfo,
}
impl std::fmt::Display for IncludedRegistries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            IncludedRegistries::Nifi => "NIFI",
            IncludedRegistries::Jvm => "JVM",
            IncludedRegistries::Bulletin => "BULLETIN",
            IncludedRegistries::Connection => "CONNECTION",
            IncludedRegistries::Cluster => "CLUSTER",
            IncludedRegistries::VersionInfo => "VERSION_INFO",
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
