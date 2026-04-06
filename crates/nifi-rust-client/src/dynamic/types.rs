// @generated

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AboutDto {
    #[serde(rename = "buildBranch")]
    pub build_branch: Option<String>,
    #[serde(rename = "buildRevision")]
    pub build_revision: Option<String>,
    #[serde(rename = "buildTag")]
    pub build_tag: Option<String>,
    #[serde(rename = "buildTimestamp")]
    pub build_timestamp: Option<String>,
    #[serde(rename = "contentViewerUrl")]
    pub content_viewer_url: Option<String>,
    pub timezone: Option<String>,
    pub title: Option<String>,
    pub uri: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AboutEntity {
    pub about: Option<AboutDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPolicyDto {
    pub action: Option<String>,
    #[serde(rename = "componentReference")]
    pub component_reference: Option<ComponentReferenceEntity>,
    pub configurable: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    pub resource: Option<String>,
    #[serde(rename = "userGroups")]
    pub user_groups: Option<Vec<TenantEntity>>,
    pub users: Option<Vec<TenantEntity>>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPolicyEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<AccessPolicyDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub generated: Option<String>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPolicySummaryDto {
    pub action: Option<String>,
    #[serde(rename = "componentReference")]
    pub component_reference: Option<ComponentReferenceEntity>,
    pub configurable: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    pub resource: Option<String>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPolicySummaryEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<AccessPolicySummaryDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDetailsDto {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDto {
    #[serde(rename = "actionDetails")]
    pub action_details: Option<ActionDetailsDto>,
    #[serde(rename = "componentDetails")]
    pub component_details: Option<ComponentDetailsDto>,
    pub id: Option<i32>,
    pub operation: Option<String>,
    #[serde(rename = "sourceId")]
    pub source_id: Option<String>,
    #[serde(rename = "sourceName")]
    pub source_name: Option<String>,
    #[serde(rename = "sourceType")]
    pub source_type: Option<String>,
    pub timestamp: Option<String>,
    #[serde(rename = "userIdentity")]
    pub user_identity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionEntity {
    pub action: Option<ActionDto>,
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    pub id: Option<i32>,
    #[serde(rename = "sourceId")]
    pub source_id: Option<String>,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivateControllerServicesEntity {
    pub components: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalDetailsEntity {
    #[serde(rename = "additionalDetails")]
    pub additional_details: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedComponentDto {
    #[serde(rename = "activeThreadCount")]
    pub active_thread_count: Option<i32>,
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "processGroupId")]
    pub process_group_id: Option<String>,
    #[serde(rename = "referenceType")]
    pub reference_type: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedComponentEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<AffectedComponentDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    #[serde(rename = "processGroup")]
    pub process_group: Option<ProcessGroupNameDto>,
    #[serde(rename = "referenceType")]
    pub reference_type: Option<String>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowableValueDto {
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowableValueEntity {
    #[serde(rename = "allowableValue")]
    pub allowable_value: Option<AllowableValueDto>,
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetDto {
    pub digest: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "missingContent")]
    pub missing_content: Option<bool>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AssetEntity {
    pub asset: Option<AssetDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetReferenceDto {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetsEntity {
    pub assets: Option<Vec<AssetEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    pub description: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeDto {
    pub name: Option<String>,
    #[serde(rename = "previousValue")]
    pub previous_value: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationConfigurationDto {
    #[serde(rename = "externalLoginRequired")]
    pub external_login_required: Option<bool>,
    #[serde(rename = "loginSupported")]
    pub login_supported: Option<bool>,
    #[serde(rename = "loginUri")]
    pub login_uri: Option<String>,
    #[serde(rename = "logoutUri")]
    pub logout_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AuthenticationConfigurationEntity {
    pub authentication_configuration: Option<AuthenticationConfigurationDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BannerDto {
    #[serde(rename = "footerText")]
    pub footer_text: Option<String>,
    #[serde(rename = "headerText")]
    pub header_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BannerEntity {
    pub banners: Option<BannerDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSettingsDto {
    pub count: Option<i32>,
    pub duration: Option<String>,
    pub size: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSize {
    pub count: Option<i32>,
    pub duration: Option<String>,
    pub size: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
    pub compiler: Option<String>,
    #[serde(rename = "compilerFlags")]
    pub compiler_flags: Option<String>,
    pub revision: Option<String>,
    #[serde(rename = "targetArch")]
    pub target_arch: Option<String>,
    pub timestamp: Option<i64>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinBoardDto {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub generated: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BulletinBoardEntity {
    pub bulletin_board: Option<BulletinBoardDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinBoardPatternParameter {
    pub pattern: Option<serde_json::Value>,
    #[serde(rename = "rawPattern")]
    pub raw_pattern: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinDto {
    pub category: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<i64>,
    pub level: Option<String>,
    pub message: Option<String>,
    #[serde(rename = "nodeAddress")]
    pub node_address: Option<String>,
    #[serde(rename = "sourceId")]
    pub source_id: Option<String>,
    #[serde(rename = "sourceName")]
    pub source_name: Option<String>,
    #[serde(rename = "sourceType")]
    pub source_type: Option<String>,
    #[serde(rename = "stackTrace")]
    pub stack_trace: Option<String>,
    pub timestamp: Option<String>,
    #[serde(rename = "timestampIso")]
    pub timestamp_iso: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinEntity {
    pub bulletin: Option<BulletinDto>,
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<i64>,
    #[serde(rename = "nodeAddress")]
    pub node_address: Option<String>,
    #[serde(rename = "sourceId")]
    pub source_id: Option<String>,
    pub timestamp: Option<String>,
    #[serde(rename = "timestampIso")]
    pub timestamp_iso: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    pub artifact: Option<String>,
    pub group: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleDto {
    pub artifact: Option<String>,
    pub group: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearBulletinsForGroupRequestEntity {
    pub components: Option<Vec<String>>,
    #[serde(rename = "fromTimestamp")]
    pub from_timestamp: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearBulletinsForGroupResultsEntity {
    #[serde(rename = "bulletinsCleared")]
    pub bulletins_cleared: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearBulletinsRequestEntity {
    #[serde(rename = "fromTimestamp")]
    pub from_timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearBulletinsResultEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(rename = "bulletinsCleared")]
    pub bulletins_cleared: Option<i32>,
    #[serde(rename = "componentId")]
    pub component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientIdParameter {
    #[serde(rename = "clientId")]
    pub client_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterDto {
    pub generated: Option<String>,
    pub nodes: Option<Vec<NodeDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClusterEntity {
    pub cluster: Option<ClusterDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSearchResultsEntity {
    #[serde(rename = "nodeResults")]
    pub node_results: Option<Vec<NodeSearchResultDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClusterSummaryDto {
    pub clustered: Option<bool>,
    #[serde(rename = "connectedNodeCount")]
    pub connected_node_count: Option<i32>,
    #[serde(rename = "connectedNodes")]
    pub connected_nodes: Option<String>,
    #[serde(rename = "connectedToCluster")]
    pub connected_to_cluster: Option<bool>,
    #[serde(rename = "totalNodeCount")]
    pub total_node_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClusterSummaryEntity {
    pub cluster_summary: Option<ClusterSummaryDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDetailsDto {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDifferenceDto {
    #[serde(rename = "componentId")]
    pub component_id: Option<String>,
    #[serde(rename = "componentName")]
    pub component_name: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    pub differences: Option<Vec<DifferenceDto>>,
    #[serde(rename = "processGroupId")]
    pub process_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentHistoryDto {
    #[serde(rename = "componentId")]
    pub component_id: Option<String>,
    #[serde(rename = "propertyHistory")]
    pub property_history: Option<std::collections::HashMap<String, Option<PropertyHistoryDto>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ComponentHistoryEntity {
    pub component_history: Option<ComponentHistoryDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentManifest {
    pub apis: Option<Vec<DefinedType>>,
    #[serde(rename = "controllerServices")]
    pub controller_services: Option<Vec<ControllerServiceDefinition>>,
    #[serde(rename = "flowAnalysisRules")]
    pub flow_analysis_rules: Option<Vec<FlowAnalysisRuleDefinition>>,
    #[serde(rename = "flowRegistryClients")]
    pub flow_registry_clients: Option<Vec<FlowRegistryClientDefinition>>,
    #[serde(rename = "parameterProviders")]
    pub parameter_providers: Option<Vec<ParameterProviderDefinition>>,
    pub processors: Option<Vec<ProcessorDefinition>>,
    #[serde(rename = "reportingTasks")]
    pub reporting_tasks: Option<Vec<ReportingTaskDefinition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentReferenceDto {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentReferenceEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ComponentReferenceDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentRestrictionPermissionDto {
    pub permissions: Option<PermissionsDto>,
    #[serde(rename = "requiredPermission")]
    pub required_permission: Option<RequiredPermissionDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentSearchResultDto {
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<String>,
    pub matches: Option<Vec<String>>,
    pub name: Option<String>,
    #[serde(rename = "parentGroup")]
    pub parent_group: Option<SearchResultGroupDto>,
    #[serde(rename = "versionedGroup")]
    pub versioned_group: Option<SearchResultGroupDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStateDto {
    #[serde(rename = "clusterState")]
    pub cluster_state: Option<StateMapDto>,
    #[serde(rename = "componentId")]
    pub component_id: Option<String>,
    #[serde(rename = "dropStateKeySupported")]
    pub drop_state_key_supported: Option<bool>,
    #[serde(rename = "localState")]
    pub local_state: Option<StateMapDto>,
    #[serde(rename = "stateDescription")]
    pub state_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ComponentStateEntity {
    pub component_state: Option<ComponentStateDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentValidationResultDto {
    #[serde(rename = "activeThreadCount")]
    pub active_thread_count: Option<i32>,
    #[serde(rename = "currentlyValid")]
    pub currently_valid: Option<bool>,
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "processGroupId")]
    pub process_group_id: Option<String>,
    #[serde(rename = "referenceType")]
    pub reference_type: Option<String>,
    #[serde(rename = "resultantValidationErrors")]
    pub resultant_validation_errors: Option<Vec<String>>,
    #[serde(rename = "resultsValid")]
    pub results_valid: Option<bool>,
    pub state: Option<String>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentValidationResultEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ComponentValidationResultDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentValidationResultsEntity {
    #[serde(rename = "validationResults")]
    pub validation_results: Option<Vec<ComponentValidationResultEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigVerificationResultDto {
    pub explanation: Option<String>,
    pub outcome: Option<String>,
    #[serde(rename = "verificationStepName")]
    pub verification_step_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationAnalysisDto {
    #[serde(rename = "componentId")]
    pub component_id: Option<String>,
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "referencedAttributes")]
    pub referenced_attributes: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "supportsVerification")]
    pub supports_verification: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConfigurationAnalysisEntity {
    pub configuration_analysis: Option<ConfigurationAnalysisDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectableComponent {
    pub comments: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "instanceIdentifier")]
    pub instance_identifier: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectableDto {
    pub comments: Option<String>,
    pub exists: Option<bool>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub running: Option<bool>,
    pub transmitting: Option<bool>,
    pub r#type: Option<String>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDto {
    #[serde(rename = "availableRelationships")]
    pub available_relationships: Option<Vec<String>>,
    #[serde(rename = "backPressureDataSizeThreshold")]
    pub back_pressure_data_size_threshold: Option<String>,
    #[serde(rename = "backPressureObjectThreshold")]
    pub back_pressure_object_threshold: Option<i64>,
    pub bends: Option<Vec<PositionDto>>,
    pub destination: Option<ConnectableDto>,
    #[serde(rename = "flowFileExpiration")]
    pub flow_file_expiration: Option<String>,
    #[serde(rename = "getzIndex")]
    pub getz_index: Option<i64>,
    pub id: Option<String>,
    #[serde(rename = "labelIndex")]
    pub label_index: Option<i32>,
    #[serde(rename = "loadBalanceCompression")]
    pub load_balance_compression: Option<String>,
    #[serde(rename = "loadBalancePartitionAttribute")]
    pub load_balance_partition_attribute: Option<String>,
    #[serde(rename = "loadBalanceStatus")]
    pub load_balance_status: Option<String>,
    #[serde(rename = "loadBalanceStrategy")]
    pub load_balance_strategy: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    pub prioritizers: Option<Vec<String>>,
    #[serde(rename = "retriedRelationships")]
    pub retried_relationships: Option<Vec<String>>,
    #[serde(rename = "selectedRelationships")]
    pub selected_relationships: Option<Vec<String>>,
    pub source: Option<ConnectableDto>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionEntity {
    pub bends: Option<Vec<PositionDto>>,
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ConnectionDto>,
    #[serde(rename = "destinationGroupId")]
    pub destination_group_id: Option<String>,
    #[serde(rename = "destinationId")]
    pub destination_id: Option<String>,
    #[serde(rename = "destinationType")]
    pub destination_type: Option<String>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "getzIndex")]
    pub getz_index: Option<i64>,
    pub id: Option<String>,
    #[serde(rename = "labelIndex")]
    pub label_index: Option<i32>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    #[serde(rename = "sourceGroupId")]
    pub source_group_id: Option<String>,
    #[serde(rename = "sourceId")]
    pub source_id: Option<String>,
    #[serde(rename = "sourceType")]
    pub source_type: Option<String>,
    pub status: Option<ConnectionStatusDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatisticsDto {
    #[serde(rename = "aggregateSnapshot")]
    pub aggregate_snapshot: Option<ConnectionStatisticsSnapshotDto>,
    pub id: Option<String>,
    #[serde(rename = "nodeSnapshots")]
    pub node_snapshots: Option<Vec<NodeConnectionStatisticsSnapshotDto>>,
    #[serde(rename = "statsLastRefreshed")]
    pub stats_last_refreshed: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatisticsEntity {
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    #[serde(rename = "connectionStatistics")]
    pub connection_statistics: Option<ConnectionStatisticsDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatisticsSnapshotDto {
    pub id: Option<String>,
    #[serde(rename = "predictedBytesAtNextInterval")]
    pub predicted_bytes_at_next_interval: Option<i64>,
    #[serde(rename = "predictedCountAtNextInterval")]
    pub predicted_count_at_next_interval: Option<i32>,
    #[serde(rename = "predictedMillisUntilBytesBackpressure")]
    pub predicted_millis_until_bytes_backpressure: Option<i64>,
    #[serde(rename = "predictedMillisUntilCountBackpressure")]
    pub predicted_millis_until_count_backpressure: Option<i64>,
    #[serde(rename = "predictedPercentBytes")]
    pub predicted_percent_bytes: Option<i32>,
    #[serde(rename = "predictedPercentCount")]
    pub predicted_percent_count: Option<i32>,
    #[serde(rename = "predictionIntervalMillis")]
    pub prediction_interval_millis: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusDto {
    #[serde(rename = "aggregateSnapshot")]
    pub aggregate_snapshot: Option<ConnectionStatusSnapshotDto>,
    #[serde(rename = "destinationId")]
    pub destination_id: Option<String>,
    #[serde(rename = "destinationName")]
    pub destination_name: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "nodeSnapshots")]
    pub node_snapshots: Option<Vec<NodeConnectionStatusSnapshotDto>>,
    #[serde(rename = "sourceId")]
    pub source_id: Option<String>,
    #[serde(rename = "sourceName")]
    pub source_name: Option<String>,
    #[serde(rename = "statsLastRefreshed")]
    pub stats_last_refreshed: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusEntity {
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    #[serde(rename = "connectionStatus")]
    pub connection_status: Option<ConnectionStatusDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusPredictionsSnapshotDto {
    #[serde(rename = "predictedBytesAtNextInterval")]
    pub predicted_bytes_at_next_interval: Option<i64>,
    #[serde(rename = "predictedCountAtNextInterval")]
    pub predicted_count_at_next_interval: Option<i32>,
    #[serde(rename = "predictedMillisUntilBytesBackpressure")]
    pub predicted_millis_until_bytes_backpressure: Option<i64>,
    #[serde(rename = "predictedMillisUntilCountBackpressure")]
    pub predicted_millis_until_count_backpressure: Option<i64>,
    #[serde(rename = "predictedPercentBytes")]
    pub predicted_percent_bytes: Option<i32>,
    #[serde(rename = "predictedPercentCount")]
    pub predicted_percent_count: Option<i32>,
    #[serde(rename = "predictionIntervalSeconds")]
    pub prediction_interval_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusSnapshotDto {
    #[serde(rename = "bytesIn")]
    pub bytes_in: Option<i64>,
    #[serde(rename = "bytesOut")]
    pub bytes_out: Option<i64>,
    #[serde(rename = "bytesQueued")]
    pub bytes_queued: Option<i64>,
    #[serde(rename = "destinationId")]
    pub destination_id: Option<String>,
    #[serde(rename = "destinationName")]
    pub destination_name: Option<String>,
    #[serde(rename = "flowFileAvailability")]
    pub flow_file_availability: Option<String>,
    #[serde(rename = "flowFilesIn")]
    pub flow_files_in: Option<i32>,
    #[serde(rename = "flowFilesOut")]
    pub flow_files_out: Option<i32>,
    #[serde(rename = "flowFilesQueued")]
    pub flow_files_queued: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<String>,
    pub input: Option<String>,
    #[serde(rename = "loadBalanceStatus")]
    pub load_balance_status: Option<String>,
    pub name: Option<String>,
    pub output: Option<String>,
    #[serde(rename = "percentUseBytes")]
    pub percent_use_bytes: Option<i32>,
    #[serde(rename = "percentUseCount")]
    pub percent_use_count: Option<i32>,
    pub predictions: Option<ConnectionStatusPredictionsSnapshotDto>,
    pub queued: Option<String>,
    #[serde(rename = "queuedCount")]
    pub queued_count: Option<String>,
    #[serde(rename = "queuedSize")]
    pub queued_size: Option<String>,
    #[serde(rename = "sourceId")]
    pub source_id: Option<String>,
    #[serde(rename = "sourceName")]
    pub source_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusSnapshotEntity {
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    #[serde(rename = "connectionStatusSnapshot")]
    pub connection_status_snapshot: Option<ConnectionStatusSnapshotDto>,
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionsEntity {
    pub connections: Option<Vec<ConnectionEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentViewerDto {
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "supportedMimeTypes")]
    pub supported_mime_types: Option<Vec<SupportedMimeTypesDto>>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentViewerEntity {
    #[serde(rename = "contentViewers")]
    pub content_viewers: Option<Vec<ContentViewerDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerBulletinsEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(rename = "controllerServiceBulletins")]
    pub controller_service_bulletins: Option<Vec<BulletinEntity>>,
    #[serde(rename = "flowAnalysisRuleBulletins")]
    pub flow_analysis_rule_bulletins: Option<Vec<BulletinEntity>>,
    #[serde(rename = "flowRegistryClientBulletins")]
    pub flow_registry_client_bulletins: Option<Vec<BulletinEntity>>,
    #[serde(rename = "parameterProviderBulletins")]
    pub parameter_provider_bulletins: Option<Vec<BulletinEntity>>,
    #[serde(rename = "reportingTaskBulletins")]
    pub reporting_task_bulletins: Option<Vec<BulletinEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerConfigurationDto {
    #[serde(rename = "maxTimerDrivenThreadCount")]
    pub max_timer_driven_thread_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerConfigurationEntity {
    pub component: Option<ControllerConfigurationDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub permissions: Option<PermissionsDto>,
    pub revision: Option<RevisionDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerDto {
    #[serde(rename = "activeRemotePortCount")]
    pub active_remote_port_count: Option<i32>,
    pub comments: Option<String>,
    #[serde(rename = "disabledCount")]
    pub disabled_count: Option<i32>,
    pub id: Option<String>,
    #[serde(rename = "inactiveRemotePortCount")]
    pub inactive_remote_port_count: Option<i32>,
    #[serde(rename = "inputPortCount")]
    pub input_port_count: Option<i32>,
    #[serde(rename = "inputPorts")]
    pub input_ports: Option<Vec<PortDto>>,
    #[serde(rename = "instanceId")]
    pub instance_id: Option<String>,
    #[serde(rename = "invalidCount")]
    pub invalid_count: Option<i32>,
    pub name: Option<String>,
    #[serde(rename = "outputPortCount")]
    pub output_port_count: Option<i32>,
    #[serde(rename = "outputPorts")]
    pub output_ports: Option<Vec<PortDto>>,
    #[serde(rename = "remoteSiteHttpListeningPort")]
    pub remote_site_http_listening_port: Option<i32>,
    #[serde(rename = "remoteSiteListeningPort")]
    pub remote_site_listening_port: Option<i32>,
    #[serde(rename = "runningCount")]
    pub running_count: Option<i32>,
    #[serde(rename = "siteToSiteSecure")]
    pub site_to_site_secure: Option<bool>,
    #[serde(rename = "stoppedCount")]
    pub stopped_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ControllerEntity {
    pub controller: Option<ControllerDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceAPI {
    pub bundle: Option<Bundle>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceApiDto {
    pub bundle: Option<BundleDto>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceDefinition {
    #[serde(rename = "additionalDetails")]
    pub additional_details: Option<bool>,
    pub artifact: Option<String>,
    #[serde(rename = "buildInfo")]
    pub build_info: Option<BuildInfo>,
    pub deprecated: Option<bool>,
    #[serde(rename = "deprecationAlternatives")]
    pub deprecation_alternatives: Option<Vec<String>>,
    #[serde(rename = "deprecationReason")]
    pub deprecation_reason: Option<String>,
    #[serde(rename = "dynamicProperties")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    #[serde(rename = "explicitRestrictions")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    pub group: Option<String>,
    #[serde(rename = "propertyDescriptors")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    #[serde(rename = "providedApiImplementations")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    pub restricted: Option<bool>,
    #[serde(rename = "restrictedExplanation")]
    pub restricted_explanation: Option<String>,
    #[serde(rename = "seeAlso")]
    pub see_also: Option<Vec<String>>,
    pub stateful: Option<Stateful>,
    #[serde(rename = "supportsDynamicProperties")]
    pub supports_dynamic_properties: Option<bool>,
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    #[serde(rename = "systemResourceConsiderations")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    pub tags: Option<Vec<String>>,
    pub r#type: Option<String>,
    #[serde(rename = "typeDescription")]
    pub type_description: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceDto {
    #[serde(rename = "annotationData")]
    pub annotation_data: Option<String>,
    #[serde(rename = "bulletinLevel")]
    pub bulletin_level: Option<String>,
    pub bundle: Option<BundleDto>,
    pub comments: Option<String>,
    #[serde(rename = "controllerServiceApis")]
    pub controller_service_apis: Option<Vec<ControllerServiceApiDto>>,
    #[serde(rename = "customUiUrl")]
    pub custom_ui_url: Option<String>,
    pub deprecated: Option<bool>,
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    #[serde(rename = "extensionMissing")]
    pub extension_missing: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "multipleVersionsAvailable")]
    pub multiple_versions_available: Option<bool>,
    pub name: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    #[serde(rename = "persistsState")]
    pub persists_state: Option<bool>,
    pub position: Option<PositionDto>,
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "referencingComponents")]
    pub referencing_components: Option<Vec<ControllerServiceReferencingComponentEntity>>,
    pub restricted: Option<bool>,
    #[serde(rename = "sensitiveDynamicPropertyNames")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    pub state: Option<String>,
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    pub r#type: Option<String>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Option<Vec<String>>,
    #[serde(rename = "validationStatus")]
    pub validation_status: Option<String>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ControllerServiceDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "operatePermissions")]
    pub operate_permissions: Option<PermissionsDto>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub status: Option<ControllerServiceStatusDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceReferencingComponentDto {
    #[serde(rename = "activeThreadCount")]
    pub active_thread_count: Option<i32>,
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "referenceCycle")]
    pub reference_cycle: Option<bool>,
    #[serde(rename = "referenceType")]
    pub reference_type: Option<String>,
    #[serde(rename = "referencingComponents")]
    pub referencing_components: Option<Vec<ControllerServiceReferencingComponentEntity>>,
    pub state: Option<String>,
    pub r#type: Option<String>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceReferencingComponentEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ControllerServiceReferencingComponentDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "operatePermissions")]
    pub operate_permissions: Option<PermissionsDto>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceReferencingComponentsEntity {
    #[serde(rename = "controllerServiceReferencingComponents")]
    pub controller_service_referencing_components:
        Option<Vec<ControllerServiceReferencingComponentEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceRunStatusEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub revision: Option<RevisionDto>,
    pub state: Option<String>,
    #[serde(rename = "uiOnly")]
    pub ui_only: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceStatusDto {
    #[serde(rename = "activeThreadCount")]
    pub active_thread_count: Option<i32>,
    #[serde(rename = "runStatus")]
    pub run_status: Option<String>,
    #[serde(rename = "validationStatus")]
    pub validation_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceTypesEntity {
    #[serde(rename = "controllerServiceTypes")]
    pub controller_service_types: Option<Vec<DocumentedTypeDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServicesEntity {
    #[serde(rename = "controllerServices")]
    pub controller_services: Option<Vec<ControllerServiceEntity>>,
    #[serde(rename = "currentTime")]
    pub current_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerStatusDto {
    #[serde(rename = "activeRemotePortCount")]
    pub active_remote_port_count: Option<i32>,
    #[serde(rename = "activeThreadCount")]
    pub active_thread_count: Option<i32>,
    #[serde(rename = "bytesQueued")]
    pub bytes_queued: Option<i64>,
    #[serde(rename = "disabledCount")]
    pub disabled_count: Option<i32>,
    #[serde(rename = "flowFilesQueued")]
    pub flow_files_queued: Option<i32>,
    #[serde(rename = "inactiveRemotePortCount")]
    pub inactive_remote_port_count: Option<i32>,
    #[serde(rename = "invalidCount")]
    pub invalid_count: Option<i32>,
    #[serde(rename = "locallyModifiedAndStaleCount")]
    pub locally_modified_and_stale_count: Option<i32>,
    #[serde(rename = "locallyModifiedCount")]
    pub locally_modified_count: Option<i32>,
    pub queued: Option<String>,
    #[serde(rename = "runningCount")]
    pub running_count: Option<i32>,
    #[serde(rename = "staleCount")]
    pub stale_count: Option<i32>,
    #[serde(rename = "stoppedCount")]
    pub stopped_count: Option<i32>,
    #[serde(rename = "syncFailureCount")]
    pub sync_failure_count: Option<i32>,
    #[serde(rename = "terminatedThreadCount")]
    pub terminated_thread_count: Option<i32>,
    #[serde(rename = "upToDateCount")]
    pub up_to_date_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ControllerStatusEntity {
    pub controller_status: Option<ControllerStatusDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyRequestEntity {
    pub connections: Option<Vec<String>>,
    pub funnels: Option<Vec<String>>,
    #[serde(rename = "inputPorts")]
    pub input_ports: Option<Vec<String>>,
    pub labels: Option<Vec<String>>,
    #[serde(rename = "outputPorts")]
    pub output_ports: Option<Vec<String>>,
    #[serde(rename = "processGroups")]
    pub process_groups: Option<Vec<String>>,
    pub processors: Option<Vec<String>>,
    #[serde(rename = "remoteProcessGroups")]
    pub remote_process_groups: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyResponseEntity {
    pub connections: Option<Vec<VersionedConnection>>,
    #[serde(rename = "externalControllerServiceReferences")]
    pub external_controller_service_references:
        Option<std::collections::HashMap<String, Option<ExternalControllerServiceReference>>>,
    pub funnels: Option<Vec<VersionedFunnel>>,
    pub id: Option<String>,
    #[serde(rename = "inputPorts")]
    pub input_ports: Option<Vec<VersionedPort>>,
    pub labels: Option<Vec<VersionedLabel>>,
    #[serde(rename = "outputPorts")]
    pub output_ports: Option<Vec<VersionedPort>>,
    #[serde(rename = "parameterContexts")]
    pub parameter_contexts:
        Option<std::collections::HashMap<String, Option<VersionedParameterContext>>>,
    #[serde(rename = "parameterProviders")]
    pub parameter_providers:
        Option<std::collections::HashMap<String, Option<ParameterProviderReference>>>,
    #[serde(rename = "processGroups")]
    pub process_groups: Option<Vec<VersionedProcessGroup>>,
    pub processors: Option<Vec<VersionedProcessor>>,
    #[serde(rename = "remoteProcessGroups")]
    pub remote_process_groups: Option<Vec<VersionedRemoteProcessGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopySnippetRequestEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "originX")]
    pub origin_x: Option<f64>,
    #[serde(rename = "originY")]
    pub origin_y: Option<f64>,
    #[serde(rename = "snippetId")]
    pub snippet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterDto {
    pub context: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "valueCount")]
    pub value_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CounterEntity {
    pub counter: Option<CounterDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountersDto {
    #[serde(rename = "aggregateSnapshot")]
    pub aggregate_snapshot: Option<CountersSnapshotDto>,
    #[serde(rename = "nodeSnapshots")]
    pub node_snapshots: Option<Vec<NodeCountersSnapshotDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CountersEntity {
    pub counters: Option<CountersDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountersSnapshotDto {
    pub counters: Option<Vec<CounterDto>>,
    pub generated: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateActiveRequestEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupId")]
    pub process_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUserEntity {
    pub anonymous: Option<bool>,
    #[serde(rename = "canVersionFlows")]
    pub can_version_flows: Option<bool>,
    #[serde(rename = "componentRestrictionPermissions")]
    pub component_restriction_permissions: Option<Vec<ComponentRestrictionPermissionDto>>,
    #[serde(rename = "controllerPermissions")]
    pub controller_permissions: Option<PermissionsDto>,
    #[serde(rename = "countersPermissions")]
    pub counters_permissions: Option<PermissionsDto>,
    pub identity: Option<String>,
    #[serde(rename = "logoutSupported")]
    pub logout_supported: Option<bool>,
    #[serde(rename = "parameterContextPermissions")]
    pub parameter_context_permissions: Option<PermissionsDto>,
    #[serde(rename = "policiesPermissions")]
    pub policies_permissions: Option<PermissionsDto>,
    #[serde(rename = "provenancePermissions")]
    pub provenance_permissions: Option<PermissionsDto>,
    #[serde(rename = "restrictedComponentsPermissions")]
    pub restricted_components_permissions: Option<PermissionsDto>,
    #[serde(rename = "systemPermissions")]
    pub system_permissions: Option<PermissionsDto>,
    #[serde(rename = "tenantsPermissions")]
    pub tenants_permissions: Option<PermissionsDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeParameter {
    #[serde(rename = "dateTime")]
    pub date_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefinedType {
    pub artifact: Option<String>,
    pub group: Option<String>,
    pub r#type: Option<String>,
    #[serde(rename = "typeDescription")]
    pub type_description: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DiagnosticLevel {
    #[serde(rename = "BASIC")]
    Basic,
    #[serde(rename = "VERBOSE")]
    Verbose,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DifferenceDto {
    pub difference: Option<String>,
    #[serde(rename = "differenceType")]
    pub difference_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionsDto {
    pub height: Option<f64>,
    pub width: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentedTypeDto {
    pub bundle: Option<BundleDto>,
    #[serde(rename = "controllerServiceApis")]
    pub controller_service_apis: Option<Vec<ControllerServiceApiDto>>,
    #[serde(rename = "deprecationReason")]
    pub deprecation_reason: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "explicitRestrictions")]
    pub explicit_restrictions: Option<Vec<ExplicitRestrictionDto>>,
    pub restricted: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub r#type: Option<String>,
    #[serde(rename = "usageRestriction")]
    pub usage_restriction: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropRequestDto {
    pub current: Option<String>,
    #[serde(rename = "currentCount")]
    pub current_count: Option<i32>,
    #[serde(rename = "currentSize")]
    pub current_size: Option<i64>,
    pub dropped: Option<String>,
    #[serde(rename = "droppedCount")]
    pub dropped_count: Option<i32>,
    #[serde(rename = "droppedSize")]
    pub dropped_size: Option<i64>,
    #[serde(rename = "failureReason")]
    pub failure_reason: Option<String>,
    pub finished: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
    pub original: Option<String>,
    #[serde(rename = "originalCount")]
    pub original_count: Option<i32>,
    #[serde(rename = "originalSize")]
    pub original_size: Option<i64>,
    #[serde(rename = "percentCompleted")]
    pub percent_completed: Option<i32>,
    pub state: Option<String>,
    #[serde(rename = "submissionTime")]
    pub submission_time: Option<String>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DropRequestEntity {
    pub drop_request: Option<DropRequestDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicProperty {
    pub description: Option<String>,
    #[serde(rename = "expressionLanguageScope")]
    pub expression_language_scope: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicRelationship {
    pub description: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplicitRestrictionDto {
    pub explanation: Option<String>,
    #[serde(rename = "requiredPermission")]
    pub required_permission: Option<RequiredPermissionDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalControllerServiceReference {
    pub identifier: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisResultEntity {
    #[serde(rename = "flowAnalysisPending")]
    pub flow_analysis_pending: Option<bool>,
    #[serde(rename = "ruleViolations")]
    pub rule_violations: Option<Vec<FlowAnalysisRuleViolationDto>>,
    pub rules: Option<Vec<FlowAnalysisRuleDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleDefinition {
    #[serde(rename = "additionalDetails")]
    pub additional_details: Option<bool>,
    pub artifact: Option<String>,
    #[serde(rename = "buildInfo")]
    pub build_info: Option<BuildInfo>,
    pub deprecated: Option<bool>,
    #[serde(rename = "deprecationAlternatives")]
    pub deprecation_alternatives: Option<Vec<String>>,
    #[serde(rename = "deprecationReason")]
    pub deprecation_reason: Option<String>,
    #[serde(rename = "dynamicProperties")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    #[serde(rename = "explicitRestrictions")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    pub group: Option<String>,
    #[serde(rename = "propertyDescriptors")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    #[serde(rename = "providedApiImplementations")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    pub restricted: Option<bool>,
    #[serde(rename = "restrictedExplanation")]
    pub restricted_explanation: Option<String>,
    #[serde(rename = "seeAlso")]
    pub see_also: Option<Vec<String>>,
    pub stateful: Option<Stateful>,
    #[serde(rename = "supportsDynamicProperties")]
    pub supports_dynamic_properties: Option<bool>,
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    #[serde(rename = "systemResourceConsiderations")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    pub tags: Option<Vec<String>>,
    pub r#type: Option<String>,
    #[serde(rename = "typeDescription")]
    pub type_description: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleDto {
    pub bundle: Option<BundleDto>,
    pub comments: Option<String>,
    pub deprecated: Option<bool>,
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    #[serde(rename = "enforcementPolicy")]
    pub enforcement_policy: Option<String>,
    #[serde(rename = "extensionMissing")]
    pub extension_missing: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "multipleVersionsAvailable")]
    pub multiple_versions_available: Option<bool>,
    pub name: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    #[serde(rename = "persistsState")]
    pub persists_state: Option<bool>,
    pub position: Option<PositionDto>,
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    pub restricted: Option<bool>,
    #[serde(rename = "sensitiveDynamicPropertyNames")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    pub state: Option<String>,
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    pub r#type: Option<String>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Option<Vec<String>>,
    #[serde(rename = "validationStatus")]
    pub validation_status: Option<String>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<FlowAnalysisRuleDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "operatePermissions")]
    pub operate_permissions: Option<PermissionsDto>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub status: Option<FlowAnalysisRuleStatusDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleRunStatusEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub revision: Option<RevisionDto>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleStatusDto {
    #[serde(rename = "activeThreadCount")]
    pub active_thread_count: Option<i32>,
    #[serde(rename = "runStatus")]
    pub run_status: Option<String>,
    #[serde(rename = "validationStatus")]
    pub validation_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleTypesEntity {
    #[serde(rename = "flowAnalysisRuleTypes")]
    pub flow_analysis_rule_types: Option<Vec<DocumentedTypeDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleViolationDto {
    pub enabled: Option<bool>,
    #[serde(rename = "enforcementPolicy")]
    pub enforcement_policy: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "issueId")]
    pub issue_id: Option<String>,
    #[serde(rename = "ruleId")]
    pub rule_id: Option<String>,
    pub scope: Option<String>,
    #[serde(rename = "subjectComponentType")]
    pub subject_component_type: Option<String>,
    #[serde(rename = "subjectDisplayName")]
    pub subject_display_name: Option<String>,
    #[serde(rename = "subjectId")]
    pub subject_id: Option<String>,
    #[serde(rename = "subjectPermissionDto")]
    pub subject_permission_dto: Option<PermissionsDto>,
    #[serde(rename = "violationMessage")]
    pub violation_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRulesEntity {
    #[serde(rename = "currentTime")]
    pub current_time: Option<String>,
    #[serde(rename = "flowAnalysisRules")]
    pub flow_analysis_rules: Option<Vec<FlowAnalysisRuleEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowBreadcrumbDto {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "versionControlInformation")]
    pub version_control_information: Option<VersionControlInformationDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowBreadcrumbEntity {
    pub breadcrumb: Option<FlowBreadcrumbDto>,
    pub id: Option<String>,
    #[serde(rename = "parentBreadcrumb")]
    pub parent_breadcrumb: Option<Box<FlowBreadcrumbEntity>>,
    pub permissions: Option<PermissionsDto>,
    #[serde(rename = "versionedFlowState")]
    pub versioned_flow_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowComparisonEntity {
    #[serde(rename = "componentDifferences")]
    pub component_differences: Option<Vec<ComponentDifferenceDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowConfigurationDto {
    #[serde(rename = "currentTime")]
    pub current_time: Option<String>,
    #[serde(rename = "defaultBackPressureDataSizeThreshold")]
    pub default_back_pressure_data_size_threshold: Option<String>,
    #[serde(rename = "defaultBackPressureObjectThreshold")]
    pub default_back_pressure_object_threshold: Option<i64>,
    #[serde(rename = "supportsConfigurableAuthorizer")]
    pub supports_configurable_authorizer: Option<bool>,
    #[serde(rename = "supportsConfigurableUsersAndGroups")]
    pub supports_configurable_users_and_groups: Option<bool>,
    #[serde(rename = "supportsManagedAuthorizer")]
    pub supports_managed_authorizer: Option<bool>,
    #[serde(rename = "timeOffset")]
    pub time_offset: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FlowConfigurationEntity {
    pub flow_configuration: Option<FlowConfigurationDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowDto {
    pub connections: Option<Vec<ConnectionEntity>>,
    pub funnels: Option<Vec<FunnelEntity>>,
    #[serde(rename = "inputPorts")]
    pub input_ports: Option<Vec<PortEntity>>,
    pub labels: Option<Vec<LabelEntity>>,
    #[serde(rename = "outputPorts")]
    pub output_ports: Option<Vec<PortEntity>>,
    #[serde(rename = "processGroups")]
    pub process_groups: Option<Vec<ProcessGroupEntity>>,
    pub processors: Option<Vec<ProcessorEntity>>,
    #[serde(rename = "remoteProcessGroups")]
    pub remote_process_groups: Option<Vec<RemoteProcessGroupEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FlowEntity {
    pub flow: Option<FlowDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowFileDto {
    pub attributes: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "clusterNodeAddress")]
    pub cluster_node_address: Option<String>,
    #[serde(rename = "clusterNodeId")]
    pub cluster_node_id: Option<String>,
    #[serde(rename = "contentClaimContainer")]
    pub content_claim_container: Option<String>,
    #[serde(rename = "contentClaimFileSize")]
    pub content_claim_file_size: Option<String>,
    #[serde(rename = "contentClaimFileSizeBytes")]
    pub content_claim_file_size_bytes: Option<i64>,
    #[serde(rename = "contentClaimIdentifier")]
    pub content_claim_identifier: Option<String>,
    #[serde(rename = "contentClaimOffset")]
    pub content_claim_offset: Option<i64>,
    #[serde(rename = "contentClaimSection")]
    pub content_claim_section: Option<String>,
    pub filename: Option<String>,
    #[serde(rename = "lineageDuration")]
    pub lineage_duration: Option<i64>,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub penalized: Option<bool>,
    #[serde(rename = "penaltyExpiresIn")]
    pub penalty_expires_in: Option<i64>,
    pub position: Option<i32>,
    #[serde(rename = "queuedDuration")]
    pub queued_duration: Option<i64>,
    pub size: Option<i64>,
    pub uri: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FlowFileEntity {
    pub flow_file: Option<FlowFileDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowFileSummaryDto {
    #[serde(rename = "clusterNodeAddress")]
    pub cluster_node_address: Option<String>,
    #[serde(rename = "clusterNodeId")]
    pub cluster_node_id: Option<String>,
    pub filename: Option<String>,
    #[serde(rename = "lineageDuration")]
    pub lineage_duration: Option<i64>,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub penalized: Option<bool>,
    #[serde(rename = "penaltyExpiresIn")]
    pub penalty_expires_in: Option<i64>,
    pub position: Option<i32>,
    #[serde(rename = "queuedDuration")]
    pub queued_duration: Option<i64>,
    pub size: Option<i64>,
    pub uri: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FlowMetricsReportingStrategy {
    #[serde(rename = "ALL_COMPONENTS")]
    AllComponents,
    #[serde(rename = "ALL_PROCESS_GROUPS")]
    AllProcessGroups,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBranchDto {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FlowRegistryBranchEntity {
    pub branch: Option<FlowRegistryBranchDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBranchesEntity {
    pub branches: Option<Vec<FlowRegistryBranchEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBucket {
    #[serde(rename = "createdTimestamp")]
    pub created_timestamp: Option<i64>,
    pub description: Option<String>,
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub permissions: Option<FlowRegistryPermissions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBucketDto {
    pub created: Option<i64>,
    pub description: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBucketEntity {
    pub bucket: Option<FlowRegistryBucketDto>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBucketsEntity {
    pub buckets: Option<Vec<FlowRegistryBucketEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientDefinition {
    #[serde(rename = "additionalDetails")]
    pub additional_details: Option<bool>,
    pub artifact: Option<String>,
    #[serde(rename = "buildInfo")]
    pub build_info: Option<BuildInfo>,
    pub deprecated: Option<bool>,
    #[serde(rename = "deprecationAlternatives")]
    pub deprecation_alternatives: Option<Vec<String>>,
    #[serde(rename = "deprecationReason")]
    pub deprecation_reason: Option<String>,
    #[serde(rename = "dynamicProperties")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    #[serde(rename = "explicitRestrictions")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    pub group: Option<String>,
    #[serde(rename = "propertyDescriptors")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    #[serde(rename = "providedApiImplementations")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    pub restricted: Option<bool>,
    #[serde(rename = "restrictedExplanation")]
    pub restricted_explanation: Option<String>,
    #[serde(rename = "seeAlso")]
    pub see_also: Option<Vec<String>>,
    pub stateful: Option<Stateful>,
    #[serde(rename = "supportsDynamicProperties")]
    pub supports_dynamic_properties: Option<bool>,
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    #[serde(rename = "systemResourceConsiderations")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    pub tags: Option<Vec<String>>,
    pub r#type: Option<String>,
    #[serde(rename = "typeDescription")]
    pub type_description: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientDto {
    #[serde(rename = "annotationData")]
    pub annotation_data: Option<String>,
    pub bundle: Option<BundleDto>,
    pub deprecated: Option<bool>,
    pub description: Option<String>,
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    #[serde(rename = "extensionMissing")]
    pub extension_missing: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "multipleVersionsAvailable")]
    pub multiple_versions_available: Option<bool>,
    pub name: Option<String>,
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    pub restricted: Option<bool>,
    #[serde(rename = "sensitiveDynamicPropertyNames")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    #[serde(rename = "supportsBranching")]
    pub supports_branching: Option<bool>,
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    pub r#type: Option<String>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Option<Vec<String>>,
    #[serde(rename = "validationStatus")]
    pub validation_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<FlowRegistryClientDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "operatePermissions")]
    pub operate_permissions: Option<PermissionsDto>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientTypesEntity {
    #[serde(rename = "flowRegistryClientTypes")]
    pub flow_registry_client_types: Option<Vec<DocumentedTypeDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientsEntity {
    #[serde(rename = "currentTime")]
    pub current_time: Option<String>,
    pub registries: Option<Vec<FlowRegistryClientEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryPermissions {
    #[serde(rename = "canDelete")]
    pub can_delete: Option<bool>,
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    #[serde(rename = "canWrite")]
    pub can_write: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowSnippetDto {
    pub connections: Option<Vec<ConnectionDto>>,
    #[serde(rename = "controllerServices")]
    pub controller_services: Option<Vec<ControllerServiceDto>>,
    pub funnels: Option<Vec<FunnelDto>>,
    #[serde(rename = "inputPorts")]
    pub input_ports: Option<Vec<PortDto>>,
    pub labels: Option<Vec<LabelDto>>,
    #[serde(rename = "outputPorts")]
    pub output_ports: Option<Vec<PortDto>>,
    #[serde(rename = "processGroups")]
    pub process_groups: Option<Vec<ProcessGroupDto>>,
    pub processors: Option<Vec<ProcessorDto>>,
    #[serde(rename = "remoteProcessGroups")]
    pub remote_process_groups: Option<Vec<RemoteProcessGroupDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunnelDto {
    pub id: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunnelEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<FunnelDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunnelsEntity {
    pub funnels: Option<Vec<FunnelEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GarbageCollectionDto {
    #[serde(rename = "collectionCount")]
    pub collection_count: Option<i64>,
    #[serde(rename = "collectionMillis")]
    pub collection_millis: Option<i64>,
    #[serde(rename = "collectionTime")]
    pub collection_time: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryDto {
    pub actions: Option<Vec<ActionEntity>>,
    #[serde(rename = "lastRefreshed")]
    pub last_refreshed: Option<String>,
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct HistoryEntity {
    pub history: Option<HistoryDto>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InputPortsEntity {
    #[serde(rename = "inputPorts")]
    pub input_ports: Option<Vec<PortEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegerParameter {
    pub integer: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JmxMetricsResultDto {
    #[serde(rename = "attributeName")]
    pub attribute_name: Option<String>,
    #[serde(rename = "attributeValue")]
    pub attribute_value: Option<serde_json::Value>,
    #[serde(rename = "beanName")]
    pub bean_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JmxMetricsResultsEntity {
    #[serde(rename = "jmxMetricsResults")]
    pub jmx_metrics_results: Option<Vec<JmxMetricsResultDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelDto {
    #[serde(rename = "getzIndex")]
    pub getz_index: Option<i64>,
    pub height: Option<f64>,
    pub id: Option<String>,
    pub label: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
    pub width: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<LabelDto>,
    pub dimensions: Option<DimensionsDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "getzIndex")]
    pub getz_index: Option<i64>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelsEntity {
    pub labels: Option<Vec<LabelEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestProvenanceEventsDto {
    #[serde(rename = "componentId")]
    pub component_id: Option<String>,
    #[serde(rename = "provenanceEvents")]
    pub provenance_events: Option<Vec<ProvenanceEventDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LatestProvenanceEventsEntity {
    pub latest_provenance_events: Option<LatestProvenanceEventsDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageDto {
    pub expiration: Option<String>,
    pub finished: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "percentCompleted")]
    pub percent_completed: Option<i32>,
    pub request: Option<LineageRequestDto>,
    pub results: Option<LineageResultsDto>,
    #[serde(rename = "submissionTime")]
    pub submission_time: Option<String>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LineageEntity {
    pub lineage: Option<LineageDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageRequestDto {
    #[serde(rename = "clusterNodeId")]
    pub cluster_node_id: Option<String>,
    #[serde(rename = "eventId")]
    pub event_id: Option<i64>,
    #[serde(rename = "lineageRequestType")]
    pub lineage_request_type: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageResultsDto {
    pub errors: Option<Vec<String>>,
    pub links: Option<Vec<ProvenanceLinkDto>>,
    pub nodes: Option<Vec<ProvenanceNodeDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenPortDto {
    #[serde(rename = "applicationProtocols")]
    pub application_protocols: Option<Vec<String>>,
    #[serde(rename = "componentClass")]
    pub component_class: Option<String>,
    #[serde(rename = "componentId")]
    pub component_id: Option<String>,
    #[serde(rename = "componentName")]
    pub component_name: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    #[serde(rename = "parentGroupName")]
    pub parent_group_name: Option<String>,
    #[serde(rename = "portName")]
    pub port_name: Option<String>,
    #[serde(rename = "portNumber")]
    pub port_number: Option<i32>,
    #[serde(rename = "transportProtocol")]
    pub transport_protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenPortsEntity {
    #[serde(rename = "listenPorts")]
    pub listen_ports: Option<Vec<ListenPortDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListingRequestDto {
    #[serde(rename = "destinationRunning")]
    pub destination_running: Option<bool>,
    #[serde(rename = "failureReason")]
    pub failure_reason: Option<String>,
    pub finished: Option<bool>,
    #[serde(rename = "flowFileSummaries")]
    pub flow_file_summaries: Option<Vec<FlowFileSummaryDto>>,
    pub id: Option<String>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
    #[serde(rename = "maxResults")]
    pub max_results: Option<i32>,
    #[serde(rename = "percentCompleted")]
    pub percent_completed: Option<i32>,
    #[serde(rename = "queueSize")]
    pub queue_size: Option<QueueSizeDto>,
    #[serde(rename = "sourceRunning")]
    pub source_running: Option<bool>,
    pub state: Option<String>,
    #[serde(rename = "submissionTime")]
    pub submission_time: Option<String>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListingRequestEntity {
    pub listing_request: Option<ListingRequestDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LongParameter {
    pub long: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiProcessorUseCase {
    pub configurations: Option<Vec<ProcessorConfiguration>>,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NarCoordinateDto {
    pub artifact: Option<String>,
    pub group: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NarDetailsEntity {
    #[serde(rename = "controllerServiceTypes")]
    pub controller_service_types: Option<Vec<DocumentedTypeDto>>,
    #[serde(rename = "dependentCoordinates")]
    pub dependent_coordinates: Option<Vec<NarCoordinateDto>>,
    #[serde(rename = "flowAnalysisRuleTypes")]
    pub flow_analysis_rule_types: Option<Vec<DocumentedTypeDto>>,
    #[serde(rename = "flowRegistryClientTypes")]
    pub flow_registry_client_types: Option<Vec<DocumentedTypeDto>>,
    #[serde(rename = "narSummary")]
    pub nar_summary: Option<NarSummaryDto>,
    #[serde(rename = "parameterProviderTypes")]
    pub parameter_provider_types: Option<Vec<DocumentedTypeDto>>,
    #[serde(rename = "processorTypes")]
    pub processor_types: Option<Vec<DocumentedTypeDto>>,
    #[serde(rename = "reportingTaskTypes")]
    pub reporting_task_types: Option<Vec<DocumentedTypeDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NarSummariesEntity {
    #[serde(rename = "currentTime")]
    pub current_time: Option<String>,
    #[serde(rename = "narSummaries")]
    pub nar_summaries: Option<Vec<NarSummaryEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NarSummaryDto {
    #[serde(rename = "buildTime")]
    pub build_time: Option<String>,
    pub coordinate: Option<NarCoordinateDto>,
    #[serde(rename = "createdBy")]
    pub created_by: Option<String>,
    #[serde(rename = "dependencyCoordinate")]
    pub dependency_coordinate: Option<NarCoordinateDto>,
    pub digest: Option<String>,
    #[serde(rename = "extensionCount")]
    pub extension_count: Option<i32>,
    #[serde(rename = "failureMessage")]
    pub failure_message: Option<String>,
    pub identifier: Option<String>,
    #[serde(rename = "installComplete")]
    pub install_complete: Option<bool>,
    #[serde(rename = "sourceIdentifier")]
    pub source_identifier: Option<String>,
    #[serde(rename = "sourceType")]
    pub source_type: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NarSummaryEntity {
    pub nar_summary: Option<NarSummaryDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConnectionStatisticsSnapshotDto {
    pub address: Option<String>,
    #[serde(rename = "apiPort")]
    pub api_port: Option<i32>,
    #[serde(rename = "nodeId")]
    pub node_id: Option<String>,
    #[serde(rename = "statisticsSnapshot")]
    pub statistics_snapshot: Option<ConnectionStatisticsSnapshotDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConnectionStatusSnapshotDto {
    pub address: Option<String>,
    #[serde(rename = "apiPort")]
    pub api_port: Option<i32>,
    #[serde(rename = "nodeId")]
    pub node_id: Option<String>,
    #[serde(rename = "statusSnapshot")]
    pub status_snapshot: Option<ConnectionStatusSnapshotDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeCountersSnapshotDto {
    pub address: Option<String>,
    #[serde(rename = "apiPort")]
    pub api_port: Option<i32>,
    #[serde(rename = "nodeId")]
    pub node_id: Option<String>,
    pub snapshot: Option<CountersSnapshotDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeDto {
    #[serde(rename = "activeThreadCount")]
    pub active_thread_count: Option<i32>,
    pub address: Option<String>,
    #[serde(rename = "apiPort")]
    pub api_port: Option<i32>,
    #[serde(rename = "bytesQueued")]
    pub bytes_queued: Option<i64>,
    #[serde(rename = "connectionRequested")]
    pub connection_requested: Option<String>,
    pub events: Option<Vec<NodeEventDto>>,
    #[serde(rename = "flowFileBytes")]
    pub flow_file_bytes: Option<i64>,
    #[serde(rename = "flowFilesQueued")]
    pub flow_files_queued: Option<i32>,
    pub heartbeat: Option<String>,
    #[serde(rename = "nodeId")]
    pub node_id: Option<String>,
    #[serde(rename = "nodeStartTime")]
    pub node_start_time: Option<String>,
    pub queued: Option<String>,
    pub roles: Option<Vec<String>>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NodeEntity {
    pub node: Option<NodeDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeEventDto {
    pub category: Option<String>,
    pub message: Option<String>,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePortStatusSnapshotDto {
    pub address: Option<String>,
    #[serde(rename = "apiPort")]
    pub api_port: Option<i32>,
    #[serde(rename = "nodeId")]
    pub node_id: Option<String>,
    #[serde(rename = "statusSnapshot")]
    pub status_snapshot: Option<PortStatusSnapshotDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeProcessGroupStatusSnapshotDto {
    pub address: Option<String>,
    #[serde(rename = "apiPort")]
    pub api_port: Option<i32>,
    #[serde(rename = "nodeId")]
    pub node_id: Option<String>,
    #[serde(rename = "statusSnapshot")]
    pub status_snapshot: Option<ProcessGroupStatusSnapshotDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeProcessorStatusSnapshotDto {
    pub address: Option<String>,
    #[serde(rename = "apiPort")]
    pub api_port: Option<i32>,
    #[serde(rename = "nodeId")]
    pub node_id: Option<String>,
    #[serde(rename = "statusSnapshot")]
    pub status_snapshot: Option<ProcessorStatusSnapshotDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeRemoteProcessGroupStatusSnapshotDto {
    pub address: Option<String>,
    #[serde(rename = "apiPort")]
    pub api_port: Option<i32>,
    #[serde(rename = "nodeId")]
    pub node_id: Option<String>,
    #[serde(rename = "statusSnapshot")]
    pub status_snapshot: Option<RemoteProcessGroupStatusSnapshotDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeReplayLastEventSnapshotDto {
    pub address: Option<String>,
    #[serde(rename = "apiPort")]
    pub api_port: Option<i32>,
    #[serde(rename = "nodeId")]
    pub node_id: Option<String>,
    pub snapshot: Option<ReplayLastEventSnapshotDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSearchResultDto {
    pub address: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatusSnapshotsDto {
    pub address: Option<String>,
    #[serde(rename = "apiPort")]
    pub api_port: Option<i32>,
    #[serde(rename = "nodeId")]
    pub node_id: Option<String>,
    #[serde(rename = "statusSnapshots")]
    pub status_snapshots: Option<Vec<StatusSnapshotDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSystemDiagnosticsSnapshotDto {
    pub address: Option<String>,
    #[serde(rename = "apiPort")]
    pub api_port: Option<i32>,
    #[serde(rename = "nodeId")]
    pub node_id: Option<String>,
    pub snapshot: Option<SystemDiagnosticsSnapshotDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputPortsEntity {
    #[serde(rename = "outputPorts")]
    pub output_ports: Option<Vec<PortEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextDto {
    #[serde(rename = "boundProcessGroups")]
    pub bound_process_groups: Option<Vec<ProcessGroupEntity>>,
    pub description: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "inheritedParameterContexts")]
    pub inherited_parameter_contexts: Option<Vec<ParameterContextReferenceEntity>>,
    pub name: Option<String>,
    #[serde(rename = "parameterProviderConfiguration")]
    pub parameter_provider_configuration: Option<ParameterProviderConfigurationEntity>,
    pub parameters: Option<Vec<ParameterEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ParameterContextDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ParameterContextHandlingStrategy {
    #[serde(rename = "KEEP_EXISTING")]
    KeepExisting,
    #[serde(rename = "REPLACE")]
    Replace,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextReferenceDto {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextReferenceEntity {
    pub component: Option<ParameterContextReferenceDto>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextUpdateEntity {
    #[serde(rename = "parameterContext")]
    pub parameter_context: Option<ParameterContextDto>,
    #[serde(rename = "parameterContextRevision")]
    pub parameter_context_revision: Option<RevisionDto>,
    #[serde(rename = "referencingComponents")]
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextUpdateRequestDto {
    pub complete: Option<bool>,
    #[serde(rename = "failureReason")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
    #[serde(rename = "parameterContext")]
    pub parameter_context: Option<ParameterContextDto>,
    #[serde(rename = "percentCompleted")]
    pub percent_completed: Option<i32>,
    #[serde(rename = "referencingComponents")]
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "submissionTime")]
    pub submission_time: Option<String>,
    #[serde(rename = "updateSteps")]
    pub update_steps: Option<Vec<ParameterContextUpdateStepDto>>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextUpdateRequestEntity {
    #[serde(rename = "parameterContextRevision")]
    pub parameter_context_revision: Option<RevisionDto>,
    pub request: Option<ParameterContextUpdateRequestDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextUpdateStepDto {
    pub complete: Option<bool>,
    pub description: Option<String>,
    #[serde(rename = "failureReason")]
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextValidationRequestDto {
    pub complete: Option<bool>,
    #[serde(rename = "componentValidationResults")]
    pub component_validation_results: Option<ComponentValidationResultsEntity>,
    #[serde(rename = "failureReason")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
    #[serde(rename = "parameterContext")]
    pub parameter_context: Option<ParameterContextDto>,
    #[serde(rename = "percentCompleted")]
    pub percent_completed: Option<i32>,
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "submissionTime")]
    pub submission_time: Option<String>,
    #[serde(rename = "updateSteps")]
    pub update_steps: Option<Vec<ParameterContextValidationStepDto>>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextValidationRequestEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub request: Option<ParameterContextValidationRequestDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextValidationStepDto {
    pub complete: Option<bool>,
    pub description: Option<String>,
    #[serde(rename = "failureReason")]
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextsEntity {
    #[serde(rename = "currentTime")]
    pub current_time: Option<String>,
    #[serde(rename = "parameterContexts")]
    pub parameter_contexts: Option<Vec<ParameterContextEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDto {
    pub description: Option<String>,
    pub inherited: Option<bool>,
    pub name: Option<String>,
    #[serde(rename = "parameterContext")]
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    pub provided: Option<bool>,
    #[serde(rename = "referencedAssets")]
    pub referenced_assets: Option<Vec<AssetReferenceDto>>,
    #[serde(rename = "referencingComponents")]
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
    pub sensitive: Option<bool>,
    pub value: Option<String>,
    #[serde(rename = "valueRemoved")]
    pub value_removed: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterEntity {
    #[serde(rename = "canWrite")]
    pub can_write: Option<bool>,
    pub parameter: Option<ParameterDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterGroupConfigurationEntity {
    #[serde(rename = "groupName")]
    pub group_name: Option<String>,
    #[serde(rename = "parameterContextName")]
    pub parameter_context_name: Option<String>,
    #[serde(rename = "parameterSensitivities")]
    pub parameter_sensitivities: Option<std::collections::HashMap<String, Option<String>>>,
    pub synchronized: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderApplyParametersRequestDto {
    pub complete: Option<bool>,
    #[serde(rename = "failureReason")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
    #[serde(rename = "parameterContextUpdates")]
    pub parameter_context_updates: Option<Vec<ParameterContextUpdateEntity>>,
    #[serde(rename = "parameterProvider")]
    pub parameter_provider: Option<ParameterProviderDto>,
    #[serde(rename = "percentCompleted")]
    pub percent_completed: Option<i32>,
    #[serde(rename = "referencingComponents")]
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "submissionTime")]
    pub submission_time: Option<String>,
    #[serde(rename = "updateSteps")]
    pub update_steps: Option<Vec<ParameterProviderApplyParametersUpdateStepDto>>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ParameterProviderApplyParametersRequestEntity {
    pub request: Option<ParameterProviderApplyParametersRequestDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderApplyParametersUpdateStepDto {
    pub complete: Option<bool>,
    pub description: Option<String>,
    #[serde(rename = "failureReason")]
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderConfigurationDto {
    #[serde(rename = "parameterGroupName")]
    pub parameter_group_name: Option<String>,
    #[serde(rename = "parameterProviderId")]
    pub parameter_provider_id: Option<String>,
    #[serde(rename = "parameterProviderName")]
    pub parameter_provider_name: Option<String>,
    pub synchronized: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderConfigurationEntity {
    pub component: Option<ParameterProviderConfigurationDto>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderDefinition {
    #[serde(rename = "additionalDetails")]
    pub additional_details: Option<bool>,
    pub artifact: Option<String>,
    #[serde(rename = "buildInfo")]
    pub build_info: Option<BuildInfo>,
    pub deprecated: Option<bool>,
    #[serde(rename = "deprecationAlternatives")]
    pub deprecation_alternatives: Option<Vec<String>>,
    #[serde(rename = "deprecationReason")]
    pub deprecation_reason: Option<String>,
    #[serde(rename = "dynamicProperties")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    #[serde(rename = "explicitRestrictions")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    pub group: Option<String>,
    #[serde(rename = "propertyDescriptors")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    #[serde(rename = "providedApiImplementations")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    pub restricted: Option<bool>,
    #[serde(rename = "restrictedExplanation")]
    pub restricted_explanation: Option<String>,
    #[serde(rename = "seeAlso")]
    pub see_also: Option<Vec<String>>,
    pub stateful: Option<Stateful>,
    #[serde(rename = "supportsDynamicProperties")]
    pub supports_dynamic_properties: Option<bool>,
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    #[serde(rename = "systemResourceConsiderations")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    pub tags: Option<Vec<String>>,
    pub r#type: Option<String>,
    #[serde(rename = "typeDescription")]
    pub type_description: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderDto {
    #[serde(rename = "affectedComponents")]
    pub affected_components: Option<Vec<AffectedComponentEntity>>,
    #[serde(rename = "annotationData")]
    pub annotation_data: Option<String>,
    pub bundle: Option<BundleDto>,
    pub comments: Option<String>,
    #[serde(rename = "customUiUrl")]
    pub custom_ui_url: Option<String>,
    pub deprecated: Option<bool>,
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    #[serde(rename = "extensionMissing")]
    pub extension_missing: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "multipleVersionsAvailable")]
    pub multiple_versions_available: Option<bool>,
    pub name: Option<String>,
    #[serde(rename = "parameterGroupConfigurations")]
    pub parameter_group_configurations: Option<Vec<ParameterGroupConfigurationEntity>>,
    #[serde(rename = "parameterStatus")]
    pub parameter_status: Option<Vec<ParameterStatusDto>>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    #[serde(rename = "persistsState")]
    pub persists_state: Option<bool>,
    pub position: Option<PositionDto>,
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "referencingParameterContexts")]
    pub referencing_parameter_contexts: Option<Vec<ParameterProviderReferencingComponentEntity>>,
    pub restricted: Option<bool>,
    pub r#type: Option<String>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Option<Vec<String>>,
    #[serde(rename = "validationStatus")]
    pub validation_status: Option<String>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ParameterProviderDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderParameterApplicationEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "parameterGroupConfigurations")]
    pub parameter_group_configurations: Option<Vec<ParameterGroupConfigurationEntity>>,
    pub revision: Option<RevisionDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderParameterFetchEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub revision: Option<RevisionDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReference {
    pub bundle: Option<Bundle>,
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReferencingComponentDto {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReferencingComponentEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ParameterProviderReferencingComponentDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReferencingComponentsEntity {
    #[serde(rename = "parameterProviderReferencingComponents")]
    pub parameter_provider_referencing_components:
        Option<Vec<ParameterProviderReferencingComponentEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderTypesEntity {
    #[serde(rename = "parameterProviderTypes")]
    pub parameter_provider_types: Option<Vec<DocumentedTypeDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProvidersEntity {
    #[serde(rename = "currentTime")]
    pub current_time: Option<String>,
    #[serde(rename = "parameterProviders")]
    pub parameter_providers: Option<Vec<ParameterProviderEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterStatusDto {
    pub parameter: Option<ParameterEntity>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasteRequestEntity {
    #[serde(rename = "copyResponse")]
    pub copy_response: Option<CopyResponseEntity>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
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
#[serde(rename_all = "camelCase")]
pub struct PeerDto {
    #[serde(rename = "flowFileCount")]
    pub flow_file_count: Option<i32>,
    pub hostname: Option<String>,
    pub port: Option<i32>,
    pub secure: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeersEntity {
    pub peers: Option<Vec<PeerDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsDto {
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    #[serde(rename = "canWrite")]
    pub can_write: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortDto {
    #[serde(rename = "allowRemoteAccess")]
    pub allow_remote_access: Option<bool>,
    pub comments: Option<String>,
    #[serde(rename = "concurrentlySchedulableTaskCount")]
    pub concurrently_schedulable_task_count: Option<i32>,
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    #[serde(rename = "portFunction")]
    pub port_function: Option<String>,
    pub position: Option<PositionDto>,
    pub state: Option<String>,
    pub transmitting: Option<bool>,
    pub r#type: Option<String>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Option<Vec<String>>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortEntity {
    #[serde(rename = "allowRemoteAccess")]
    pub allow_remote_access: Option<bool>,
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<PortDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "operatePermissions")]
    pub operate_permissions: Option<PermissionsDto>,
    pub permissions: Option<PermissionsDto>,
    #[serde(rename = "portType")]
    pub port_type: Option<String>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub status: Option<PortStatusDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortRunStatusEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub revision: Option<RevisionDto>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatusDto {
    #[serde(rename = "aggregateSnapshot")]
    pub aggregate_snapshot: Option<PortStatusSnapshotDto>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "nodeSnapshots")]
    pub node_snapshots: Option<Vec<NodePortStatusSnapshotDto>>,
    #[serde(rename = "runStatus")]
    pub run_status: Option<String>,
    #[serde(rename = "statsLastRefreshed")]
    pub stats_last_refreshed: Option<String>,
    pub transmitting: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatusEntity {
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    #[serde(rename = "portStatus")]
    pub port_status: Option<PortStatusDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatusSnapshotDto {
    #[serde(rename = "activeThreadCount")]
    pub active_thread_count: Option<i32>,
    #[serde(rename = "bytesIn")]
    pub bytes_in: Option<i64>,
    #[serde(rename = "bytesOut")]
    pub bytes_out: Option<i64>,
    #[serde(rename = "flowFilesIn")]
    pub flow_files_in: Option<i32>,
    #[serde(rename = "flowFilesOut")]
    pub flow_files_out: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<String>,
    pub input: Option<String>,
    pub name: Option<String>,
    pub output: Option<String>,
    #[serde(rename = "runStatus")]
    pub run_status: Option<String>,
    pub transmitting: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatusSnapshotEntity {
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "portStatusSnapshot")]
    pub port_status_snapshot: Option<PortStatusSnapshotDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionDto {
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviousValueDto {
    #[serde(rename = "previousValue")]
    pub previous_value: Option<String>,
    pub timestamp: Option<String>,
    #[serde(rename = "userIdentity")]
    pub user_identity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrioritizerTypesEntity {
    #[serde(rename = "prioritizerTypes")]
    pub prioritizer_types: Option<Vec<DocumentedTypeDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupDto {
    #[serde(rename = "activeRemotePortCount")]
    pub active_remote_port_count: Option<i32>,
    pub comments: Option<String>,
    pub contents: Option<FlowSnippetDto>,
    #[serde(rename = "defaultBackPressureDataSizeThreshold")]
    pub default_back_pressure_data_size_threshold: Option<String>,
    #[serde(rename = "defaultBackPressureObjectThreshold")]
    pub default_back_pressure_object_threshold: Option<i64>,
    #[serde(rename = "defaultFlowFileExpiration")]
    pub default_flow_file_expiration: Option<String>,
    #[serde(rename = "disabledCount")]
    pub disabled_count: Option<i32>,
    #[serde(rename = "executionEngine")]
    pub execution_engine: Option<String>,
    #[serde(rename = "flowfileConcurrency")]
    pub flowfile_concurrency: Option<String>,
    #[serde(rename = "flowfileOutboundPolicy")]
    pub flowfile_outbound_policy: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "inactiveRemotePortCount")]
    pub inactive_remote_port_count: Option<i32>,
    #[serde(rename = "inputPortCount")]
    pub input_port_count: Option<i32>,
    #[serde(rename = "invalidCount")]
    pub invalid_count: Option<i32>,
    #[serde(rename = "localInputPortCount")]
    pub local_input_port_count: Option<i32>,
    #[serde(rename = "localOutputPortCount")]
    pub local_output_port_count: Option<i32>,
    #[serde(rename = "locallyModifiedAndStaleCount")]
    pub locally_modified_and_stale_count: Option<i32>,
    #[serde(rename = "locallyModifiedCount")]
    pub locally_modified_count: Option<i32>,
    #[serde(rename = "logFileSuffix")]
    pub log_file_suffix: Option<String>,
    #[serde(rename = "maxConcurrentTasks")]
    pub max_concurrent_tasks: Option<i32>,
    pub name: Option<String>,
    #[serde(rename = "outputPortCount")]
    pub output_port_count: Option<i32>,
    #[serde(rename = "parameterContext")]
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    #[serde(rename = "publicInputPortCount")]
    pub public_input_port_count: Option<i32>,
    #[serde(rename = "publicOutputPortCount")]
    pub public_output_port_count: Option<i32>,
    #[serde(rename = "runningCount")]
    pub running_count: Option<i32>,
    #[serde(rename = "staleCount")]
    pub stale_count: Option<i32>,
    #[serde(rename = "statelessFlowTimeout")]
    pub stateless_flow_timeout: Option<String>,
    #[serde(rename = "statelessGroupScheduledState")]
    pub stateless_group_scheduled_state: Option<String>,
    #[serde(rename = "stoppedCount")]
    pub stopped_count: Option<i32>,
    #[serde(rename = "syncFailureCount")]
    pub sync_failure_count: Option<i32>,
    #[serde(rename = "upToDateCount")]
    pub up_to_date_count: Option<i32>,
    #[serde(rename = "versionControlInformation")]
    pub version_control_information: Option<VersionControlInformationDto>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupEntity {
    #[serde(rename = "activeRemotePortCount")]
    pub active_remote_port_count: Option<i32>,
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ProcessGroupDto>,
    #[serde(rename = "disabledCount")]
    pub disabled_count: Option<i32>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "inactiveRemotePortCount")]
    pub inactive_remote_port_count: Option<i32>,
    #[serde(rename = "inputPortCount")]
    pub input_port_count: Option<i32>,
    #[serde(rename = "invalidCount")]
    pub invalid_count: Option<i32>,
    #[serde(rename = "localInputPortCount")]
    pub local_input_port_count: Option<i32>,
    #[serde(rename = "localOutputPortCount")]
    pub local_output_port_count: Option<i32>,
    #[serde(rename = "locallyModifiedAndStaleCount")]
    pub locally_modified_and_stale_count: Option<i32>,
    #[serde(rename = "locallyModifiedCount")]
    pub locally_modified_count: Option<i32>,
    #[serde(rename = "outputPortCount")]
    pub output_port_count: Option<i32>,
    #[serde(rename = "parameterContext")]
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    #[serde(rename = "processGroupUpdateStrategy")]
    pub process_group_update_strategy: Option<String>,
    #[serde(rename = "publicInputPortCount")]
    pub public_input_port_count: Option<i32>,
    #[serde(rename = "publicOutputPortCount")]
    pub public_output_port_count: Option<i32>,
    pub revision: Option<RevisionDto>,
    #[serde(rename = "runningCount")]
    pub running_count: Option<i32>,
    #[serde(rename = "staleCount")]
    pub stale_count: Option<i32>,
    pub status: Option<ProcessGroupStatusDto>,
    #[serde(rename = "stoppedCount")]
    pub stopped_count: Option<i32>,
    #[serde(rename = "syncFailureCount")]
    pub sync_failure_count: Option<i32>,
    #[serde(rename = "upToDateCount")]
    pub up_to_date_count: Option<i32>,
    pub uri: Option<String>,
    #[serde(rename = "versionedFlowSnapshot")]
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
    #[serde(rename = "versionedFlowState")]
    pub versioned_flow_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupFlowDto {
    pub breadcrumb: Option<FlowBreadcrumbEntity>,
    pub flow: Option<FlowDto>,
    pub id: Option<String>,
    #[serde(rename = "lastRefreshed")]
    pub last_refreshed: Option<String>,
    #[serde(rename = "parameterContext")]
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupFlowEntity {
    pub permissions: Option<PermissionsDto>,
    #[serde(rename = "processGroupFlow")]
    pub process_group_flow: Option<ProcessGroupFlowDto>,
    pub revision: Option<RevisionDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupImportEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupRevision")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(rename = "versionedFlowSnapshot")]
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupNameDto {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupReplaceRequestDto {
    pub complete: Option<bool>,
    #[serde(rename = "failureReason")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
    #[serde(rename = "percentCompleted")]
    pub percent_completed: Option<i32>,
    #[serde(rename = "processGroupId")]
    pub process_group_id: Option<String>,
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    pub state: Option<String>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupReplaceRequestEntity {
    #[serde(rename = "processGroupRevision")]
    pub process_group_revision: Option<RevisionDto>,
    pub request: Option<ProcessGroupReplaceRequestDto>,
    #[serde(rename = "versionedFlowSnapshot")]
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusDto {
    #[serde(rename = "aggregateSnapshot")]
    pub aggregate_snapshot: Option<ProcessGroupStatusSnapshotDto>,
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "nodeSnapshots")]
    pub node_snapshots: Option<Vec<NodeProcessGroupStatusSnapshotDto>>,
    #[serde(rename = "statsLastRefreshed")]
    pub stats_last_refreshed: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusEntity {
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    #[serde(rename = "processGroupStatus")]
    pub process_group_status: Option<ProcessGroupStatusDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusSnapshotDto {
    #[serde(rename = "activeThreadCount")]
    pub active_thread_count: Option<i32>,
    #[serde(rename = "bytesIn")]
    pub bytes_in: Option<i64>,
    #[serde(rename = "bytesOut")]
    pub bytes_out: Option<i64>,
    #[serde(rename = "bytesQueued")]
    pub bytes_queued: Option<i64>,
    #[serde(rename = "bytesRead")]
    pub bytes_read: Option<i64>,
    #[serde(rename = "bytesReceived")]
    pub bytes_received: Option<i64>,
    #[serde(rename = "bytesSent")]
    pub bytes_sent: Option<i64>,
    #[serde(rename = "bytesTransferred")]
    pub bytes_transferred: Option<i64>,
    #[serde(rename = "bytesWritten")]
    pub bytes_written: Option<i64>,
    #[serde(rename = "connectionStatusSnapshots")]
    pub connection_status_snapshots: Option<Vec<ConnectionStatusSnapshotEntity>>,
    #[serde(rename = "flowFilesIn")]
    pub flow_files_in: Option<i32>,
    #[serde(rename = "flowFilesOut")]
    pub flow_files_out: Option<i32>,
    #[serde(rename = "flowFilesQueued")]
    pub flow_files_queued: Option<i32>,
    #[serde(rename = "flowFilesReceived")]
    pub flow_files_received: Option<i32>,
    #[serde(rename = "flowFilesSent")]
    pub flow_files_sent: Option<i32>,
    #[serde(rename = "flowFilesTransferred")]
    pub flow_files_transferred: Option<i32>,
    pub id: Option<String>,
    pub input: Option<String>,
    #[serde(rename = "inputPortStatusSnapshots")]
    pub input_port_status_snapshots: Option<Vec<PortStatusSnapshotEntity>>,
    pub name: Option<String>,
    pub output: Option<String>,
    #[serde(rename = "outputPortStatusSnapshots")]
    pub output_port_status_snapshots: Option<Vec<PortStatusSnapshotEntity>>,
    #[serde(rename = "processGroupStatusSnapshots")]
    pub process_group_status_snapshots: Option<Vec<ProcessGroupStatusSnapshotEntity>>,
    #[serde(rename = "processingNanos")]
    pub processing_nanos: Option<i64>,
    #[serde(rename = "processingPerformanceStatus")]
    pub processing_performance_status: Option<ProcessingPerformanceStatusDto>,
    #[serde(rename = "processorStatusSnapshots")]
    pub processor_status_snapshots: Option<Vec<ProcessorStatusSnapshotEntity>>,
    pub queued: Option<String>,
    #[serde(rename = "queuedCount")]
    pub queued_count: Option<String>,
    #[serde(rename = "queuedSize")]
    pub queued_size: Option<String>,
    pub read: Option<String>,
    pub received: Option<String>,
    #[serde(rename = "remoteProcessGroupStatusSnapshots")]
    pub remote_process_group_status_snapshots: Option<Vec<RemoteProcessGroupStatusSnapshotEntity>>,
    pub sent: Option<String>,
    #[serde(rename = "statelessActiveThreadCount")]
    pub stateless_active_thread_count: Option<i32>,
    #[serde(rename = "terminatedThreadCount")]
    pub terminated_thread_count: Option<i32>,
    pub transferred: Option<String>,
    #[serde(rename = "versionedFlowState")]
    pub versioned_flow_state: Option<String>,
    pub written: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusSnapshotEntity {
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "processGroupStatusSnapshot")]
    pub process_group_status_snapshot: Option<ProcessGroupStatusSnapshotDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupUploadEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "flowSnapshot")]
    pub flow_snapshot: Option<RegisteredFlowSnapshot>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "groupName")]
    pub group_name: Option<String>,
    #[serde(rename = "positionDTO")]
    pub position_d_t_o: Option<PositionDto>,
    #[serde(rename = "revisionDTO")]
    pub revision_d_t_o: Option<RevisionDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupsEntity {
    #[serde(rename = "processGroups")]
    pub process_groups: Option<Vec<ProcessGroupEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessingPerformanceStatusDto {
    #[serde(rename = "contentReadDuration")]
    pub content_read_duration: Option<i64>,
    #[serde(rename = "contentWriteDuration")]
    pub content_write_duration: Option<i64>,
    #[serde(rename = "cpuDuration")]
    pub cpu_duration: Option<i64>,
    #[serde(rename = "garbageCollectionDuration")]
    pub garbage_collection_duration: Option<i64>,
    pub identifier: Option<String>,
    #[serde(rename = "sessionCommitDuration")]
    pub session_commit_duration: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorConfigDto {
    #[serde(rename = "annotationData")]
    pub annotation_data: Option<String>,
    #[serde(rename = "autoTerminatedRelationships")]
    pub auto_terminated_relationships: Option<Vec<String>>,
    #[serde(rename = "backoffMechanism")]
    pub backoff_mechanism: Option<String>,
    #[serde(rename = "bulletinLevel")]
    pub bulletin_level: Option<String>,
    pub comments: Option<String>,
    #[serde(rename = "concurrentlySchedulableTaskCount")]
    pub concurrently_schedulable_task_count: Option<i32>,
    #[serde(rename = "customUiUrl")]
    pub custom_ui_url: Option<String>,
    #[serde(rename = "defaultConcurrentTasks")]
    pub default_concurrent_tasks: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "defaultSchedulingPeriod")]
    pub default_scheduling_period: Option<std::collections::HashMap<String, Option<String>>>,
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    #[serde(rename = "executionNode")]
    pub execution_node: Option<String>,
    #[serde(rename = "lossTolerant")]
    pub loss_tolerant: Option<bool>,
    #[serde(rename = "maxBackoffPeriod")]
    pub max_backoff_period: Option<String>,
    #[serde(rename = "penaltyDuration")]
    pub penalty_duration: Option<String>,
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "retriedRelationships")]
    pub retried_relationships: Option<Vec<String>>,
    #[serde(rename = "retryCount")]
    pub retry_count: Option<i32>,
    #[serde(rename = "runDurationMillis")]
    pub run_duration_millis: Option<i64>,
    #[serde(rename = "schedulingPeriod")]
    pub scheduling_period: Option<String>,
    #[serde(rename = "schedulingStrategy")]
    pub scheduling_strategy: Option<String>,
    #[serde(rename = "sensitiveDynamicPropertyNames")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    #[serde(rename = "yieldDuration")]
    pub yield_duration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorConfiguration {
    pub configuration: Option<String>,
    #[serde(rename = "processorClassName")]
    pub processor_class_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorDefinition {
    #[serde(rename = "additionalDetails")]
    pub additional_details: Option<bool>,
    pub artifact: Option<String>,
    #[serde(rename = "buildInfo")]
    pub build_info: Option<BuildInfo>,
    #[serde(rename = "defaultBulletinLevel")]
    pub default_bulletin_level: Option<String>,
    #[serde(rename = "defaultConcurrentTasksBySchedulingStrategy")]
    pub default_concurrent_tasks_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<i32>>>,
    #[serde(rename = "defaultPenaltyDuration")]
    pub default_penalty_duration: Option<String>,
    #[serde(rename = "defaultSchedulingPeriodBySchedulingStrategy")]
    pub default_scheduling_period_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "defaultSchedulingStrategy")]
    pub default_scheduling_strategy: Option<String>,
    #[serde(rename = "defaultYieldDuration")]
    pub default_yield_duration: Option<String>,
    pub deprecated: Option<bool>,
    #[serde(rename = "deprecationAlternatives")]
    pub deprecation_alternatives: Option<Vec<String>>,
    #[serde(rename = "deprecationReason")]
    pub deprecation_reason: Option<String>,
    #[serde(rename = "dynamicProperties")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    #[serde(rename = "dynamicRelationship")]
    pub dynamic_relationship: Option<DynamicRelationship>,
    #[serde(rename = "explicitRestrictions")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    pub group: Option<String>,
    #[serde(rename = "inputRequirement")]
    pub input_requirement: Option<String>,
    #[serde(rename = "multiProcessorUseCases")]
    pub multi_processor_use_cases: Option<Vec<MultiProcessorUseCase>>,
    #[serde(rename = "primaryNodeOnly")]
    pub primary_node_only: Option<bool>,
    #[serde(rename = "propertyDescriptors")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    #[serde(rename = "providedApiImplementations")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    #[serde(rename = "readsAttributes")]
    pub reads_attributes: Option<Vec<Attribute>>,
    pub restricted: Option<bool>,
    #[serde(rename = "restrictedExplanation")]
    pub restricted_explanation: Option<String>,
    #[serde(rename = "seeAlso")]
    pub see_also: Option<Vec<String>>,
    #[serde(rename = "sideEffectFree")]
    pub side_effect_free: Option<bool>,
    pub stateful: Option<Stateful>,
    #[serde(rename = "supportedRelationships")]
    pub supported_relationships: Option<Vec<Relationship>>,
    #[serde(rename = "supportedSchedulingStrategies")]
    pub supported_scheduling_strategies: Option<Vec<String>>,
    #[serde(rename = "supportsBatching")]
    pub supports_batching: Option<bool>,
    #[serde(rename = "supportsDynamicProperties")]
    pub supports_dynamic_properties: Option<bool>,
    #[serde(rename = "supportsDynamicRelationships")]
    pub supports_dynamic_relationships: Option<bool>,
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    #[serde(rename = "systemResourceConsiderations")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "triggerSerially")]
    pub trigger_serially: Option<bool>,
    #[serde(rename = "triggerWhenAnyDestinationAvailable")]
    pub trigger_when_any_destination_available: Option<bool>,
    #[serde(rename = "triggerWhenEmpty")]
    pub trigger_when_empty: Option<bool>,
    pub r#type: Option<String>,
    #[serde(rename = "typeDescription")]
    pub type_description: Option<String>,
    #[serde(rename = "useCases")]
    pub use_cases: Option<Vec<UseCase>>,
    pub version: Option<String>,
    #[serde(rename = "writesAttributes")]
    pub writes_attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorDto {
    pub bundle: Option<BundleDto>,
    pub config: Option<ProcessorConfigDto>,
    pub deprecated: Option<bool>,
    pub description: Option<String>,
    #[serde(rename = "executionNodeRestricted")]
    pub execution_node_restricted: Option<bool>,
    #[serde(rename = "extensionMissing")]
    pub extension_missing: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "inputRequirement")]
    pub input_requirement: Option<String>,
    #[serde(rename = "multipleVersionsAvailable")]
    pub multiple_versions_available: Option<bool>,
    pub name: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    #[serde(rename = "persistsState")]
    pub persists_state: Option<bool>,
    #[serde(rename = "physicalState")]
    pub physical_state: Option<String>,
    pub position: Option<PositionDto>,
    pub relationships: Option<Vec<RelationshipDto>>,
    pub restricted: Option<bool>,
    pub state: Option<String>,
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "supportsBatching")]
    pub supports_batching: Option<bool>,
    #[serde(rename = "supportsParallelProcessing")]
    pub supports_parallel_processing: Option<bool>,
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    pub r#type: Option<String>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Option<Vec<String>>,
    #[serde(rename = "validationStatus")]
    pub validation_status: Option<String>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ProcessorDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "inputRequirement")]
    pub input_requirement: Option<String>,
    #[serde(rename = "operatePermissions")]
    pub operate_permissions: Option<PermissionsDto>,
    pub permissions: Option<PermissionsDto>,
    #[serde(rename = "physicalState")]
    pub physical_state: Option<String>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub status: Option<ProcessorStatusDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorRunStatusDetailsDto {
    #[serde(rename = "activeThreadCount")]
    pub active_thread_count: Option<i32>,
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "runStatus")]
    pub run_status: Option<String>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorRunStatusDetailsEntity {
    pub permissions: Option<PermissionsDto>,
    pub revision: Option<RevisionDto>,
    #[serde(rename = "runStatusDetails")]
    pub run_status_details: Option<ProcessorRunStatusDetailsDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorRunStatusEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub revision: Option<RevisionDto>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorStatusDto {
    #[serde(rename = "aggregateSnapshot")]
    pub aggregate_snapshot: Option<ProcessorStatusSnapshotDto>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "nodeSnapshots")]
    pub node_snapshots: Option<Vec<NodeProcessorStatusSnapshotDto>>,
    #[serde(rename = "runStatus")]
    pub run_status: Option<String>,
    #[serde(rename = "statsLastRefreshed")]
    pub stats_last_refreshed: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorStatusEntity {
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    #[serde(rename = "processorStatus")]
    pub processor_status: Option<ProcessorStatusDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorStatusSnapshotDto {
    #[serde(rename = "activeThreadCount")]
    pub active_thread_count: Option<i32>,
    #[serde(rename = "bytesIn")]
    pub bytes_in: Option<i64>,
    #[serde(rename = "bytesOut")]
    pub bytes_out: Option<i64>,
    #[serde(rename = "bytesRead")]
    pub bytes_read: Option<i64>,
    #[serde(rename = "bytesWritten")]
    pub bytes_written: Option<i64>,
    #[serde(rename = "executionNode")]
    pub execution_node: Option<String>,
    #[serde(rename = "flowFilesIn")]
    pub flow_files_in: Option<i32>,
    #[serde(rename = "flowFilesOut")]
    pub flow_files_out: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<String>,
    pub input: Option<String>,
    pub name: Option<String>,
    pub output: Option<String>,
    #[serde(rename = "processingPerformanceStatus")]
    pub processing_performance_status: Option<ProcessingPerformanceStatusDto>,
    pub read: Option<String>,
    #[serde(rename = "runStatus")]
    pub run_status: Option<String>,
    #[serde(rename = "taskCount")]
    pub task_count: Option<i32>,
    pub tasks: Option<String>,
    #[serde(rename = "tasksDuration")]
    pub tasks_duration: Option<String>,
    #[serde(rename = "tasksDurationNanos")]
    pub tasks_duration_nanos: Option<i64>,
    #[serde(rename = "terminatedThreadCount")]
    pub terminated_thread_count: Option<i32>,
    pub r#type: Option<String>,
    pub written: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorStatusSnapshotEntity {
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "processorStatusSnapshot")]
    pub processor_status_snapshot: Option<ProcessorStatusSnapshotDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorTypesEntity {
    #[serde(rename = "processorTypes")]
    pub processor_types: Option<Vec<DocumentedTypeDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorsEntity {
    pub processors: Option<Vec<ProcessorEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorsRunStatusDetailsEntity {
    #[serde(rename = "runStatusDetails")]
    pub run_status_details: Option<Vec<ProcessorRunStatusDetailsEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyAllowableValue {
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDependency {
    #[serde(rename = "dependentValues")]
    pub dependent_values: Option<Vec<String>>,
    #[serde(rename = "propertyDisplayName")]
    pub property_display_name: Option<String>,
    #[serde(rename = "propertyName")]
    pub property_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDependencyDto {
    #[serde(rename = "dependentValues")]
    pub dependent_values: Option<Vec<String>>,
    #[serde(rename = "propertyName")]
    pub property_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptor {
    #[serde(rename = "allowableValues")]
    pub allowable_values: Option<Vec<PropertyAllowableValue>>,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<String>,
    pub dependencies: Option<Vec<PropertyDependency>>,
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub dynamic: Option<bool>,
    #[serde(rename = "expressionLanguageScope")]
    pub expression_language_scope: Option<String>,
    #[serde(rename = "expressionLanguageScopeDescription")]
    pub expression_language_scope_description: Option<String>,
    #[serde(rename = "listenPortDefinition")]
    pub listen_port_definition: Option<PropertyListenPortDefinition>,
    pub name: Option<String>,
    pub required: Option<bool>,
    #[serde(rename = "resourceDefinition")]
    pub resource_definition: Option<PropertyResourceDefinition>,
    pub sensitive: Option<bool>,
    #[serde(rename = "typeProvidedByValue")]
    pub type_provided_by_value: Option<DefinedType>,
    #[serde(rename = "validRegex")]
    pub valid_regex: Option<String>,
    pub validator: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptorDto {
    #[serde(rename = "allowableValues")]
    pub allowable_values: Option<Vec<AllowableValueEntity>>,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<String>,
    pub dependencies: Option<Vec<PropertyDependencyDto>>,
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub dynamic: Option<bool>,
    #[serde(rename = "expressionLanguageScope")]
    pub expression_language_scope: Option<String>,
    #[serde(rename = "identifiesControllerService")]
    pub identifies_controller_service: Option<String>,
    #[serde(rename = "identifiesControllerServiceBundle")]
    pub identifies_controller_service_bundle: Option<BundleDto>,
    pub name: Option<String>,
    pub required: Option<bool>,
    pub sensitive: Option<bool>,
    #[serde(rename = "supportsEl")]
    pub supports_el: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PropertyDescriptorEntity {
    pub property_descriptor: Option<PropertyDescriptorDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyHistoryDto {
    #[serde(rename = "previousValues")]
    pub previous_values: Option<Vec<PreviousValueDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyListenPortDefinition {
    #[serde(rename = "applicationProtocols")]
    pub application_protocols: Option<Vec<String>>,
    #[serde(rename = "transportProtocol")]
    pub transport_protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyResourceDefinition {
    pub cardinality: Option<String>,
    #[serde(rename = "resourceTypes")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceDto {
    pub expiration: Option<String>,
    pub finished: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "percentCompleted")]
    pub percent_completed: Option<i32>,
    pub request: Option<ProvenanceRequestDto>,
    pub results: Option<ProvenanceResultsDto>,
    #[serde(rename = "submissionTime")]
    pub submission_time: Option<String>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProvenanceEntity {
    pub provenance: Option<ProvenanceDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceEventDto {
    #[serde(rename = "alternateIdentifierUri")]
    pub alternate_identifier_uri: Option<String>,
    pub attributes: Option<Vec<AttributeDto>>,
    #[serde(rename = "childUuids")]
    pub child_uuids: Option<Vec<String>>,
    #[serde(rename = "clusterNodeAddress")]
    pub cluster_node_address: Option<String>,
    #[serde(rename = "clusterNodeId")]
    pub cluster_node_id: Option<String>,
    #[serde(rename = "componentId")]
    pub component_id: Option<String>,
    #[serde(rename = "componentName")]
    pub component_name: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    #[serde(rename = "contentEqual")]
    pub content_equal: Option<bool>,
    pub details: Option<String>,
    #[serde(rename = "eventDuration")]
    pub event_duration: Option<i64>,
    #[serde(rename = "eventId")]
    pub event_id: Option<i64>,
    #[serde(rename = "eventTime")]
    pub event_time: Option<String>,
    #[serde(rename = "eventTimestamp")]
    pub event_timestamp: Option<String>,
    #[serde(rename = "eventType")]
    pub event_type: Option<String>,
    #[serde(rename = "fileSize")]
    pub file_size: Option<String>,
    #[serde(rename = "fileSizeBytes")]
    pub file_size_bytes: Option<i64>,
    #[serde(rename = "flowFileUuid")]
    pub flow_file_uuid: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "inputContentAvailable")]
    pub input_content_available: Option<bool>,
    #[serde(rename = "inputContentClaimContainer")]
    pub input_content_claim_container: Option<String>,
    #[serde(rename = "inputContentClaimFileSize")]
    pub input_content_claim_file_size: Option<String>,
    #[serde(rename = "inputContentClaimFileSizeBytes")]
    pub input_content_claim_file_size_bytes: Option<i64>,
    #[serde(rename = "inputContentClaimIdentifier")]
    pub input_content_claim_identifier: Option<String>,
    #[serde(rename = "inputContentClaimOffset")]
    pub input_content_claim_offset: Option<i64>,
    #[serde(rename = "inputContentClaimSection")]
    pub input_content_claim_section: Option<String>,
    #[serde(rename = "lineageDuration")]
    pub lineage_duration: Option<i64>,
    #[serde(rename = "outputContentAvailable")]
    pub output_content_available: Option<bool>,
    #[serde(rename = "outputContentClaimContainer")]
    pub output_content_claim_container: Option<String>,
    #[serde(rename = "outputContentClaimFileSize")]
    pub output_content_claim_file_size: Option<String>,
    #[serde(rename = "outputContentClaimFileSizeBytes")]
    pub output_content_claim_file_size_bytes: Option<i64>,
    #[serde(rename = "outputContentClaimIdentifier")]
    pub output_content_claim_identifier: Option<String>,
    #[serde(rename = "outputContentClaimOffset")]
    pub output_content_claim_offset: Option<i64>,
    #[serde(rename = "outputContentClaimSection")]
    pub output_content_claim_section: Option<String>,
    #[serde(rename = "parentUuids")]
    pub parent_uuids: Option<Vec<String>>,
    pub relationship: Option<String>,
    #[serde(rename = "replayAvailable")]
    pub replay_available: Option<bool>,
    #[serde(rename = "replayExplanation")]
    pub replay_explanation: Option<String>,
    #[serde(rename = "sourceConnectionIdentifier")]
    pub source_connection_identifier: Option<String>,
    #[serde(rename = "sourceSystemFlowFileId")]
    pub source_system_flow_file_id: Option<String>,
    #[serde(rename = "transitUri")]
    pub transit_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProvenanceEventEntity {
    pub provenance_event: Option<ProvenanceEventDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceLinkDto {
    #[serde(rename = "flowFileUuid")]
    pub flow_file_uuid: Option<String>,
    pub millis: Option<i64>,
    #[serde(rename = "sourceId")]
    pub source_id: Option<String>,
    #[serde(rename = "targetId")]
    pub target_id: Option<String>,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceNodeDto {
    #[serde(rename = "childUuids")]
    pub child_uuids: Option<Vec<String>>,
    #[serde(rename = "clusterNodeIdentifier")]
    pub cluster_node_identifier: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    #[serde(rename = "eventType")]
    pub event_type: Option<String>,
    #[serde(rename = "flowFileUuid")]
    pub flow_file_uuid: Option<String>,
    pub id: Option<String>,
    pub millis: Option<i64>,
    #[serde(rename = "parentUuids")]
    pub parent_uuids: Option<Vec<String>>,
    pub timestamp: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceOptionsDto {
    #[serde(rename = "searchableFields")]
    pub searchable_fields: Option<Vec<ProvenanceSearchableFieldDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProvenanceOptionsEntity {
    pub provenance_options: Option<ProvenanceOptionsDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceRequestDto {
    #[serde(rename = "clusterNodeId")]
    pub cluster_node_id: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    #[serde(rename = "incrementalResults")]
    pub incremental_results: Option<bool>,
    #[serde(rename = "maxResults")]
    pub max_results: Option<i32>,
    #[serde(rename = "maximumFileSize")]
    pub maximum_file_size: Option<String>,
    #[serde(rename = "minimumFileSize")]
    pub minimum_file_size: Option<String>,
    #[serde(rename = "searchTerms")]
    pub search_terms: Option<std::collections::HashMap<String, Option<ProvenanceSearchValueDto>>>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    pub summarize: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceResultsDto {
    pub errors: Option<Vec<String>>,
    pub generated: Option<String>,
    #[serde(rename = "oldestEvent")]
    pub oldest_event: Option<String>,
    #[serde(rename = "provenanceEvents")]
    pub provenance_events: Option<Vec<ProvenanceEventDto>>,
    #[serde(rename = "timeOffset")]
    pub time_offset: Option<i32>,
    pub total: Option<String>,
    #[serde(rename = "totalCount")]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceSearchValueDto {
    pub inverse: Option<bool>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceSearchableFieldDto {
    pub field: Option<String>,
    pub id: Option<String>,
    pub label: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueSizeDto {
    #[serde(rename = "byteCount")]
    pub byte_count: Option<i64>,
    #[serde(rename = "objectCount")]
    pub object_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlow {
    pub branch: Option<String>,
    #[serde(rename = "bucketIdentifier")]
    pub bucket_identifier: Option<String>,
    #[serde(rename = "bucketName")]
    pub bucket_name: Option<String>,
    #[serde(rename = "createdTimestamp")]
    pub created_timestamp: Option<i64>,
    pub description: Option<String>,
    pub identifier: Option<String>,
    #[serde(rename = "lastModifiedTimestamp")]
    pub last_modified_timestamp: Option<i64>,
    pub name: Option<String>,
    pub permissions: Option<FlowRegistryPermissions>,
    #[serde(rename = "versionCount")]
    pub version_count: Option<i64>,
    #[serde(rename = "versionInfo")]
    pub version_info: Option<RegisteredFlowVersionInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlowSnapshot {
    pub bucket: Option<FlowRegistryBucket>,
    #[serde(rename = "externalControllerServices")]
    pub external_controller_services:
        Option<std::collections::HashMap<String, Option<ExternalControllerServiceReference>>>,
    pub flow: Option<RegisteredFlow>,
    #[serde(rename = "flowContents")]
    pub flow_contents: Option<VersionedProcessGroup>,
    #[serde(rename = "flowEncodingVersion")]
    pub flow_encoding_version: Option<String>,
    pub latest: Option<bool>,
    #[serde(rename = "parameterContexts")]
    pub parameter_contexts:
        Option<std::collections::HashMap<String, Option<VersionedParameterContext>>>,
    #[serde(rename = "parameterProviders")]
    pub parameter_providers:
        Option<std::collections::HashMap<String, Option<ParameterProviderReference>>>,
    #[serde(rename = "snapshotMetadata")]
    pub snapshot_metadata: Option<RegisteredFlowSnapshotMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlowSnapshotMetadata {
    pub author: Option<String>,
    pub branch: Option<String>,
    #[serde(rename = "bucketIdentifier")]
    pub bucket_identifier: Option<String>,
    pub comments: Option<String>,
    #[serde(rename = "flowIdentifier")]
    pub flow_identifier: Option<String>,
    #[serde(rename = "flowName")]
    pub flow_name: Option<String>,
    #[serde(rename = "registryIdentifier")]
    pub registry_identifier: Option<String>,
    #[serde(rename = "registryName")]
    pub registry_name: Option<String>,
    pub timestamp: Option<i64>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlowVersionInfo {
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub description: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipDto {
    #[serde(rename = "autoTerminate")]
    pub auto_terminate: Option<bool>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub retry: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemotePortRunStatusEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub revision: Option<RevisionDto>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupContentsDto {
    #[serde(rename = "inputPorts")]
    pub input_ports: Option<Vec<RemoteProcessGroupPortDto>>,
    #[serde(rename = "outputPorts")]
    pub output_ports: Option<Vec<RemoteProcessGroupPortDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupDto {
    #[serde(rename = "activeRemoteInputPortCount")]
    pub active_remote_input_port_count: Option<i32>,
    #[serde(rename = "activeRemoteOutputPortCount")]
    pub active_remote_output_port_count: Option<i32>,
    #[serde(rename = "authorizationIssues")]
    pub authorization_issues: Option<Vec<String>>,
    pub comments: Option<String>,
    #[serde(rename = "communicationsTimeout")]
    pub communications_timeout: Option<String>,
    pub contents: Option<RemoteProcessGroupContentsDto>,
    #[serde(rename = "flowRefreshed")]
    pub flow_refreshed: Option<String>,
    pub id: Option<String>,
    #[serde(rename = "inactiveRemoteInputPortCount")]
    pub inactive_remote_input_port_count: Option<i32>,
    #[serde(rename = "inactiveRemoteOutputPortCount")]
    pub inactive_remote_output_port_count: Option<i32>,
    #[serde(rename = "inputPortCount")]
    pub input_port_count: Option<i32>,
    #[serde(rename = "localNetworkInterface")]
    pub local_network_interface: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "outputPortCount")]
    pub output_port_count: Option<i32>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    #[serde(rename = "proxyHost")]
    pub proxy_host: Option<String>,
    #[serde(rename = "proxyPassword")]
    pub proxy_password: Option<String>,
    #[serde(rename = "proxyPort")]
    pub proxy_port: Option<i32>,
    #[serde(rename = "proxyUser")]
    pub proxy_user: Option<String>,
    #[serde(rename = "targetSecure")]
    pub target_secure: Option<bool>,
    #[serde(rename = "targetUri")]
    pub target_uri: Option<String>,
    #[serde(rename = "targetUris")]
    pub target_uris: Option<String>,
    pub transmitting: Option<bool>,
    #[serde(rename = "transportProtocol")]
    pub transport_protocol: Option<String>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Option<Vec<String>>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
    #[serde(rename = "yieldDuration")]
    pub yield_duration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<RemoteProcessGroupDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "inputPortCount")]
    pub input_port_count: Option<i32>,
    #[serde(rename = "operatePermissions")]
    pub operate_permissions: Option<PermissionsDto>,
    #[serde(rename = "outputPortCount")]
    pub output_port_count: Option<i32>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub status: Option<RemoteProcessGroupStatusDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupPortDto {
    #[serde(rename = "batchSettings")]
    pub batch_settings: Option<BatchSettingsDto>,
    pub comments: Option<String>,
    #[serde(rename = "concurrentlySchedulableTaskCount")]
    pub concurrently_schedulable_task_count: Option<i32>,
    pub connected: Option<bool>,
    pub exists: Option<bool>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "targetId")]
    pub target_id: Option<String>,
    #[serde(rename = "targetRunning")]
    pub target_running: Option<bool>,
    pub transmitting: Option<bool>,
    #[serde(rename = "useCompression")]
    pub use_compression: Option<bool>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupPortEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "operatePermissions")]
    pub operate_permissions: Option<PermissionsDto>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    #[serde(rename = "remoteProcessGroupPort")]
    pub remote_process_group_port: Option<RemoteProcessGroupPortDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusDto {
    #[serde(rename = "aggregateSnapshot")]
    pub aggregate_snapshot: Option<RemoteProcessGroupStatusSnapshotDto>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "nodeSnapshots")]
    pub node_snapshots: Option<Vec<NodeRemoteProcessGroupStatusSnapshotDto>>,
    #[serde(rename = "statsLastRefreshed")]
    pub stats_last_refreshed: Option<String>,
    #[serde(rename = "targetUri")]
    pub target_uri: Option<String>,
    #[serde(rename = "transmissionStatus")]
    pub transmission_status: Option<String>,
    #[serde(rename = "validationStatus")]
    pub validation_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusEntity {
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    #[serde(rename = "remoteProcessGroupStatus")]
    pub remote_process_group_status: Option<RemoteProcessGroupStatusDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusSnapshotDto {
    #[serde(rename = "activeThreadCount")]
    pub active_thread_count: Option<i32>,
    #[serde(rename = "bytesReceived")]
    pub bytes_received: Option<i64>,
    #[serde(rename = "bytesSent")]
    pub bytes_sent: Option<i64>,
    #[serde(rename = "flowFilesReceived")]
    pub flow_files_received: Option<i32>,
    #[serde(rename = "flowFilesSent")]
    pub flow_files_sent: Option<i32>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub received: Option<String>,
    pub sent: Option<String>,
    #[serde(rename = "targetUri")]
    pub target_uri: Option<String>,
    #[serde(rename = "transmissionStatus")]
    pub transmission_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusSnapshotEntity {
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "remoteProcessGroupStatusSnapshot")]
    pub remote_process_group_status_snapshot: Option<RemoteProcessGroupStatusSnapshotDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupsEntity {
    #[serde(rename = "remoteProcessGroups")]
    pub remote_process_groups: Option<Vec<RemoteProcessGroupEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayLastEventRequestEntity {
    #[serde(rename = "componentId")]
    pub component_id: Option<String>,
    pub nodes: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayLastEventResponseEntity {
    #[serde(rename = "aggregateSnapshot")]
    pub aggregate_snapshot: Option<ReplayLastEventSnapshotDto>,
    #[serde(rename = "componentId")]
    pub component_id: Option<String>,
    #[serde(rename = "nodeSnapshots")]
    pub node_snapshots: Option<Vec<NodeReplayLastEventSnapshotDto>>,
    pub nodes: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayLastEventSnapshotDto {
    #[serde(rename = "eventAvailable")]
    pub event_available: Option<bool>,
    #[serde(rename = "eventsReplayed")]
    pub events_replayed: Option<Vec<i64>>,
    #[serde(rename = "failureExplanation")]
    pub failure_explanation: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskDefinition {
    #[serde(rename = "additionalDetails")]
    pub additional_details: Option<bool>,
    pub artifact: Option<String>,
    #[serde(rename = "buildInfo")]
    pub build_info: Option<BuildInfo>,
    #[serde(rename = "defaultSchedulingPeriodBySchedulingStrategy")]
    pub default_scheduling_period_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "defaultSchedulingStrategy")]
    pub default_scheduling_strategy: Option<String>,
    pub deprecated: Option<bool>,
    #[serde(rename = "deprecationAlternatives")]
    pub deprecation_alternatives: Option<Vec<String>>,
    #[serde(rename = "deprecationReason")]
    pub deprecation_reason: Option<String>,
    #[serde(rename = "dynamicProperties")]
    pub dynamic_properties: Option<Vec<DynamicProperty>>,
    #[serde(rename = "explicitRestrictions")]
    pub explicit_restrictions: Option<Vec<Restriction>>,
    pub group: Option<String>,
    #[serde(rename = "propertyDescriptors")]
    pub property_descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptor>>>,
    #[serde(rename = "providedApiImplementations")]
    pub provided_api_implementations: Option<Vec<DefinedType>>,
    pub restricted: Option<bool>,
    #[serde(rename = "restrictedExplanation")]
    pub restricted_explanation: Option<String>,
    #[serde(rename = "seeAlso")]
    pub see_also: Option<Vec<String>>,
    pub stateful: Option<Stateful>,
    #[serde(rename = "supportedSchedulingStrategies")]
    pub supported_scheduling_strategies: Option<Vec<String>>,
    #[serde(rename = "supportsDynamicProperties")]
    pub supports_dynamic_properties: Option<bool>,
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    #[serde(rename = "systemResourceConsiderations")]
    pub system_resource_considerations: Option<Vec<SystemResourceConsideration>>,
    pub tags: Option<Vec<String>>,
    pub r#type: Option<String>,
    #[serde(rename = "typeDescription")]
    pub type_description: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskDto {
    #[serde(rename = "activeThreadCount")]
    pub active_thread_count: Option<i32>,
    #[serde(rename = "annotationData")]
    pub annotation_data: Option<String>,
    pub bundle: Option<BundleDto>,
    pub comments: Option<String>,
    #[serde(rename = "customUiUrl")]
    pub custom_ui_url: Option<String>,
    #[serde(rename = "defaultSchedulingPeriod")]
    pub default_scheduling_period: Option<std::collections::HashMap<String, Option<String>>>,
    pub deprecated: Option<bool>,
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    #[serde(rename = "extensionMissing")]
    pub extension_missing: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "multipleVersionsAvailable")]
    pub multiple_versions_available: Option<bool>,
    pub name: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    #[serde(rename = "persistsState")]
    pub persists_state: Option<bool>,
    pub position: Option<PositionDto>,
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    pub restricted: Option<bool>,
    #[serde(rename = "schedulingPeriod")]
    pub scheduling_period: Option<String>,
    #[serde(rename = "schedulingStrategy")]
    pub scheduling_strategy: Option<String>,
    #[serde(rename = "sensitiveDynamicPropertyNames")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    pub state: Option<String>,
    #[serde(rename = "supportsSensitiveDynamicProperties")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    pub r#type: Option<String>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Option<Vec<String>>,
    #[serde(rename = "validationStatus")]
    pub validation_status: Option<String>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ReportingTaskDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "operatePermissions")]
    pub operate_permissions: Option<PermissionsDto>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub status: Option<ReportingTaskStatusDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskRunStatusEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub revision: Option<RevisionDto>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskStatusDto {
    #[serde(rename = "activeThreadCount")]
    pub active_thread_count: Option<i32>,
    #[serde(rename = "runStatus")]
    pub run_status: Option<String>,
    #[serde(rename = "validationStatus")]
    pub validation_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskTypesEntity {
    #[serde(rename = "reportingTaskTypes")]
    pub reporting_task_types: Option<Vec<DocumentedTypeDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTasksEntity {
    #[serde(rename = "currentTime")]
    pub current_time: Option<String>,
    #[serde(rename = "reportingTasks")]
    pub reporting_tasks: Option<Vec<ReportingTaskEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequiredPermissionDto {
    pub id: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimDetailsDto {
    #[serde(rename = "awaitingDestruction")]
    pub awaiting_destruction: Option<bool>,
    #[serde(rename = "claimantCount")]
    pub claimant_count: Option<i32>,
    pub container: Option<String>,
    pub identifier: Option<String>,
    #[serde(rename = "inUse")]
    pub in_use: Option<bool>,
    pub section: Option<String>,
    pub writable: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceDto {
    pub identifier: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesEntity {
    pub resources: Option<Vec<ResourceDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Restriction {
    pub explanation: Option<String>,
    #[serde(rename = "requiredPermission")]
    pub required_permission: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionDto {
    #[serde(rename = "clientId")]
    pub client_id: Option<String>,
    #[serde(rename = "lastModifier")]
    pub last_modifier: Option<String>,
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunStatusDetailsRequestEntity {
    #[serde(rename = "processorIds")]
    pub processor_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeManifest {
    #[serde(rename = "agentType")]
    pub agent_type: Option<String>,
    #[serde(rename = "buildInfo")]
    pub build_info: Option<BuildInfo>,
    pub bundles: Option<Vec<Bundle>>,
    pub identifier: Option<String>,
    #[serde(rename = "schedulingDefaults")]
    pub scheduling_defaults: Option<SchedulingDefaults>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RuntimeManifestEntity {
    pub runtime_manifest: Option<RuntimeManifest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleComponentsEntity {
    pub components: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchedulingDefaults {
    #[serde(rename = "defaultConcurrentTasksBySchedulingStrategy")]
    pub default_concurrent_tasks_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<i32>>>,
    #[serde(rename = "defaultMaxConcurrentTasks")]
    pub default_max_concurrent_tasks: Option<String>,
    #[serde(rename = "defaultRunDurationNanos")]
    pub default_run_duration_nanos: Option<i64>,
    #[serde(rename = "defaultSchedulingPeriodMillis")]
    pub default_scheduling_period_millis: Option<i64>,
    #[serde(rename = "defaultSchedulingPeriodsBySchedulingStrategy")]
    pub default_scheduling_periods_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "defaultSchedulingStrategy")]
    pub default_scheduling_strategy: Option<String>,
    #[serde(rename = "penalizationPeriodMillis")]
    pub penalization_period_millis: Option<i64>,
    #[serde(rename = "yieldDurationMillis")]
    pub yield_duration_millis: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultGroupDto {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultsDto {
    #[serde(rename = "connectionResults")]
    pub connection_results: Option<Vec<ComponentSearchResultDto>>,
    #[serde(rename = "controllerServiceNodeResults")]
    pub controller_service_node_results: Option<Vec<ComponentSearchResultDto>>,
    #[serde(rename = "funnelResults")]
    pub funnel_results: Option<Vec<ComponentSearchResultDto>>,
    #[serde(rename = "inputPortResults")]
    pub input_port_results: Option<Vec<ComponentSearchResultDto>>,
    #[serde(rename = "labelResults")]
    pub label_results: Option<Vec<ComponentSearchResultDto>>,
    #[serde(rename = "outputPortResults")]
    pub output_port_results: Option<Vec<ComponentSearchResultDto>>,
    #[serde(rename = "parameterContextResults")]
    pub parameter_context_results: Option<Vec<ComponentSearchResultDto>>,
    #[serde(rename = "parameterProviderNodeResults")]
    pub parameter_provider_node_results: Option<Vec<ComponentSearchResultDto>>,
    #[serde(rename = "parameterResults")]
    pub parameter_results: Option<Vec<ComponentSearchResultDto>>,
    #[serde(rename = "processGroupResults")]
    pub process_group_results: Option<Vec<ComponentSearchResultDto>>,
    #[serde(rename = "processorResults")]
    pub processor_results: Option<Vec<ComponentSearchResultDto>>,
    #[serde(rename = "remoteProcessGroupResults")]
    pub remote_process_group_results: Option<Vec<ComponentSearchResultDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchResultsEntity {
    pub search_results_d_t_o: Option<SearchResultsDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetDto {
    pub connections: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    pub funnels: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    pub id: Option<String>,
    #[serde(rename = "inputPorts")]
    pub input_ports: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    pub labels: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    #[serde(rename = "outputPorts")]
    pub output_ports: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    #[serde(rename = "processGroups")]
    pub process_groups: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    pub processors: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    #[serde(rename = "remoteProcessGroups")]
    pub remote_process_groups: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub snippet: Option<SnippetDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartVersionControlRequestEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupRevision")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(rename = "versionedFlow")]
    pub versioned_flow: Option<VersionedFlowDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateEntryDto {
    #[serde(rename = "clusterNodeAddress")]
    pub cluster_node_address: Option<String>,
    #[serde(rename = "clusterNodeId")]
    pub cluster_node_id: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateMapDto {
    pub scope: Option<String>,
    pub state: Option<Vec<StateEntryDto>>,
    #[serde(rename = "totalEntryCount")]
    pub total_entry_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stateful {
    pub description: Option<String>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusDescriptorDto {
    pub description: Option<String>,
    pub field: Option<String>,
    pub formatter: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusHistoryDto {
    #[serde(rename = "aggregateSnapshots")]
    pub aggregate_snapshots: Option<Vec<StatusSnapshotDto>>,
    #[serde(rename = "componentDetails")]
    pub component_details: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "fieldDescriptors")]
    pub field_descriptors: Option<Vec<StatusDescriptorDto>>,
    pub generated: Option<String>,
    #[serde(rename = "nodeSnapshots")]
    pub node_snapshots: Option<Vec<NodeStatusSnapshotsDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusHistoryEntity {
    #[serde(rename = "canRead")]
    pub can_read: Option<bool>,
    #[serde(rename = "statusHistory")]
    pub status_history: Option<StatusHistoryDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusSnapshotDto {
    #[serde(rename = "statusMetrics")]
    pub status_metrics: Option<std::collections::HashMap<String, Option<i64>>>,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageUsageDto {
    #[serde(rename = "freeSpace")]
    pub free_space: Option<String>,
    #[serde(rename = "freeSpaceBytes")]
    pub free_space_bytes: Option<i64>,
    pub identifier: Option<String>,
    #[serde(rename = "totalSpace")]
    pub total_space: Option<String>,
    #[serde(rename = "totalSpaceBytes")]
    pub total_space_bytes: Option<i64>,
    #[serde(rename = "usedSpace")]
    pub used_space: Option<String>,
    #[serde(rename = "usedSpaceBytes")]
    pub used_space_bytes: Option<i64>,
    pub utilization: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitReplayRequestEntity {
    #[serde(rename = "clusterNodeId")]
    pub cluster_node_id: Option<String>,
    #[serde(rename = "eventId")]
    pub event_id: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportedMimeTypesDto {
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "mimeTypes")]
    pub mime_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDiagnosticsDto {
    #[serde(rename = "aggregateSnapshot")]
    pub aggregate_snapshot: Option<SystemDiagnosticsSnapshotDto>,
    #[serde(rename = "nodeSnapshots")]
    pub node_snapshots: Option<Vec<NodeSystemDiagnosticsSnapshotDto>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SystemDiagnosticsEntity {
    pub system_diagnostics: Option<SystemDiagnosticsDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDiagnosticsSnapshotDto {
    #[serde(rename = "availableProcessors")]
    pub available_processors: Option<i32>,
    #[serde(rename = "contentRepositoryStorageUsage")]
    pub content_repository_storage_usage: Option<Vec<StorageUsageDto>>,
    #[serde(rename = "daemonThreads")]
    pub daemon_threads: Option<i32>,
    #[serde(rename = "flowFileRepositoryStorageUsage")]
    pub flow_file_repository_storage_usage: Option<StorageUsageDto>,
    #[serde(rename = "freeHeap")]
    pub free_heap: Option<String>,
    #[serde(rename = "freeHeapBytes")]
    pub free_heap_bytes: Option<i64>,
    #[serde(rename = "freeNonHeap")]
    pub free_non_heap: Option<String>,
    #[serde(rename = "freeNonHeapBytes")]
    pub free_non_heap_bytes: Option<i64>,
    #[serde(rename = "garbageCollection")]
    pub garbage_collection: Option<Vec<GarbageCollectionDto>>,
    #[serde(rename = "heapUtilization")]
    pub heap_utilization: Option<String>,
    #[serde(rename = "maxHeap")]
    pub max_heap: Option<String>,
    #[serde(rename = "maxHeapBytes")]
    pub max_heap_bytes: Option<i64>,
    #[serde(rename = "maxNonHeap")]
    pub max_non_heap: Option<String>,
    #[serde(rename = "maxNonHeapBytes")]
    pub max_non_heap_bytes: Option<i64>,
    #[serde(rename = "nonHeapUtilization")]
    pub non_heap_utilization: Option<String>,
    #[serde(rename = "processorLoadAverage")]
    pub processor_load_average: Option<f64>,
    #[serde(rename = "provenanceRepositoryStorageUsage")]
    pub provenance_repository_storage_usage: Option<Vec<StorageUsageDto>>,
    #[serde(rename = "resourceClaimDetails")]
    pub resource_claim_details: Option<Vec<ResourceClaimDetailsDto>>,
    #[serde(rename = "statsLastRefreshed")]
    pub stats_last_refreshed: Option<String>,
    #[serde(rename = "totalHeap")]
    pub total_heap: Option<String>,
    #[serde(rename = "totalHeapBytes")]
    pub total_heap_bytes: Option<i64>,
    #[serde(rename = "totalNonHeap")]
    pub total_non_heap: Option<String>,
    #[serde(rename = "totalNonHeapBytes")]
    pub total_non_heap_bytes: Option<i64>,
    #[serde(rename = "totalThreads")]
    pub total_threads: Option<i32>,
    pub uptime: Option<String>,
    #[serde(rename = "usedHeap")]
    pub used_heap: Option<String>,
    #[serde(rename = "usedHeapBytes")]
    pub used_heap_bytes: Option<i64>,
    #[serde(rename = "usedNonHeap")]
    pub used_non_heap: Option<String>,
    #[serde(rename = "usedNonHeapBytes")]
    pub used_non_heap_bytes: Option<i64>,
    #[serde(rename = "versionInfo")]
    pub version_info: Option<VersionInfoDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemResourceConsideration {
    pub description: Option<String>,
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantDto {
    pub configurable: Option<bool>,
    pub id: Option<String>,
    pub identity: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<TenantDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantsEntity {
    #[serde(rename = "userGroups")]
    pub user_groups: Option<Vec<TenantEntity>>,
    pub users: Option<Vec<TenantEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResultEntity {
    #[serde(rename = "flowFileSent")]
    pub flow_file_sent: Option<i32>,
    pub message: Option<String>,
    #[serde(rename = "responseCode")]
    pub response_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateControllerServiceReferenceRequestEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    #[serde(rename = "referencingComponentRevisions")]
    pub referencing_component_revisions:
        Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    pub state: Option<String>,
    #[serde(rename = "uiOnly")]
    pub ui_only: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseCase {
    pub configuration: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "inputRequirement")]
    pub input_requirement: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    #[serde(rename = "accessPolicies")]
    pub access_policies: Option<Vec<AccessPolicySummaryEntity>>,
    pub configurable: Option<bool>,
    pub id: Option<String>,
    pub identity: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    #[serde(rename = "userGroups")]
    pub user_groups: Option<Vec<TenantEntity>>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<UserDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupDto {
    #[serde(rename = "accessPolicies")]
    pub access_policies: Option<Vec<AccessPolicyEntity>>,
    pub configurable: Option<bool>,
    pub id: Option<String>,
    pub identity: Option<String>,
    #[serde(rename = "parentGroupId")]
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    pub users: Option<Vec<TenantEntity>>,
    #[serde(rename = "versionedComponentId")]
    pub versioned_component_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupEntity {
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<UserGroupDto>,
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupsEntity {
    #[serde(rename = "userGroups")]
    pub user_groups: Option<Vec<UserGroupEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersEntity {
    pub generated: Option<String>,
    pub users: Option<Vec<UserEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyConfigRequestDto {
    pub attributes: Option<std::collections::HashMap<String, Option<String>>>,
    pub complete: Option<bool>,
    #[serde(rename = "componentId")]
    pub component_id: Option<String>,
    #[serde(rename = "failureReason")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
    #[serde(rename = "percentCompleted")]
    pub percent_completed: Option<i32>,
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    pub results: Option<Vec<ConfigVerificationResultDto>>,
    pub state: Option<String>,
    #[serde(rename = "submissionTime")]
    pub submission_time: Option<String>,
    #[serde(rename = "updateSteps")]
    pub update_steps: Option<Vec<VerifyConfigUpdateStepDto>>,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VerifyConfigRequestEntity {
    pub request: Option<VerifyConfigRequestDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyConfigUpdateStepDto {
    pub complete: Option<bool>,
    pub description: Option<String>,
    #[serde(rename = "failureReason")]
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionControlComponentMappingEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupRevision")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(rename = "versionControlComponentMapping")]
    pub version_control_component_mapping:
        Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "versionControlInformation")]
    pub version_control_information: Option<VersionControlInformationDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionControlInformationDto {
    pub branch: Option<String>,
    #[serde(rename = "bucketId")]
    pub bucket_id: Option<String>,
    #[serde(rename = "bucketName")]
    pub bucket_name: Option<String>,
    #[serde(rename = "flowDescription")]
    pub flow_description: Option<String>,
    #[serde(rename = "flowId")]
    pub flow_id: Option<String>,
    #[serde(rename = "flowName")]
    pub flow_name: Option<String>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "registryId")]
    pub registry_id: Option<String>,
    #[serde(rename = "registryName")]
    pub registry_name: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "stateExplanation")]
    pub state_explanation: Option<String>,
    #[serde(rename = "storageLocation")]
    pub storage_location: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionControlInformationEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupRevision")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(rename = "versionControlInformation")]
    pub version_control_information: Option<VersionControlInformationDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfoDto {
    #[serde(rename = "buildBranch")]
    pub build_branch: Option<String>,
    #[serde(rename = "buildRevision")]
    pub build_revision: Option<String>,
    #[serde(rename = "buildTag")]
    pub build_tag: Option<String>,
    #[serde(rename = "buildTimestamp")]
    pub build_timestamp: Option<String>,
    #[serde(rename = "javaVendor")]
    pub java_vendor: Option<String>,
    #[serde(rename = "javaVersion")]
    pub java_version: Option<String>,
    #[serde(rename = "niFiVersion")]
    pub ni_fi_version: Option<String>,
    #[serde(rename = "osArchitecture")]
    pub os_architecture: Option<String>,
    #[serde(rename = "osName")]
    pub os_name: Option<String>,
    #[serde(rename = "osVersion")]
    pub os_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedAsset {
    pub identifier: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedConnection {
    #[serde(rename = "backPressureDataSizeThreshold")]
    pub back_pressure_data_size_threshold: Option<String>,
    #[serde(rename = "backPressureObjectThreshold")]
    pub back_pressure_object_threshold: Option<i64>,
    pub bends: Option<Vec<Position>>,
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    pub destination: Option<ConnectableComponent>,
    #[serde(rename = "flowFileExpiration")]
    pub flow_file_expiration: Option<String>,
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<String>,
    pub identifier: Option<String>,
    #[serde(rename = "instanceIdentifier")]
    pub instance_identifier: Option<String>,
    #[serde(rename = "labelIndex")]
    pub label_index: Option<i32>,
    #[serde(rename = "loadBalanceCompression")]
    pub load_balance_compression: Option<String>,
    #[serde(rename = "loadBalanceStrategy")]
    pub load_balance_strategy: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "partitioningAttribute")]
    pub partitioning_attribute: Option<String>,
    pub position: Option<Position>,
    pub prioritizers: Option<Vec<String>>,
    #[serde(rename = "selectedRelationships")]
    pub selected_relationships: Option<Vec<String>>,
    pub source: Option<ConnectableComponent>,
    #[serde(rename = "zIndex")]
    pub z_index: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedControllerService {
    #[serde(rename = "annotationData")]
    pub annotation_data: Option<String>,
    #[serde(rename = "bulletinLevel")]
    pub bulletin_level: Option<String>,
    pub bundle: Option<Bundle>,
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    #[serde(rename = "controllerServiceApis")]
    pub controller_service_apis: Option<Vec<ControllerServiceAPI>>,
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<String>,
    pub identifier: Option<String>,
    #[serde(rename = "instanceIdentifier")]
    pub instance_identifier: Option<String>,
    pub name: Option<String>,
    pub position: Option<Position>,
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "propertyDescriptors")]
    pub property_descriptors:
        Option<std::collections::HashMap<String, Option<VersionedPropertyDescriptor>>>,
    #[serde(rename = "scheduledState")]
    pub scheduled_state: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowCoordinates {
    pub branch: Option<String>,
    #[serde(rename = "bucketId")]
    pub bucket_id: Option<String>,
    #[serde(rename = "flowId")]
    pub flow_id: Option<String>,
    pub latest: Option<bool>,
    #[serde(rename = "registryId")]
    pub registry_id: Option<String>,
    #[serde(rename = "storageLocation")]
    pub storage_location: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowDto {
    pub action: Option<String>,
    pub branch: Option<String>,
    #[serde(rename = "bucketId")]
    pub bucket_id: Option<String>,
    pub comments: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "flowId")]
    pub flow_id: Option<String>,
    #[serde(rename = "flowName")]
    pub flow_name: Option<String>,
    #[serde(rename = "registryId")]
    pub registry_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VersionedFlowEntity {
    pub versioned_flow: Option<VersionedFlowDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowSnapshotEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "processGroupRevision")]
    pub process_group_revision: Option<RevisionDto>,
    #[serde(rename = "registryId")]
    pub registry_id: Option<String>,
    #[serde(rename = "updateDescendantVersionedFlows")]
    pub update_descendant_versioned_flows: Option<bool>,
    #[serde(rename = "versionedFlow")]
    pub versioned_flow: Option<RegisteredFlowSnapshot>,
    #[serde(rename = "versionedFlowSnapshot")]
    pub versioned_flow_snapshot: Option<RegisteredFlowSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowSnapshotMetadataEntity {
    #[serde(rename = "registryId")]
    pub registry_id: Option<String>,
    #[serde(rename = "versionedFlowSnapshotMetadata")]
    pub versioned_flow_snapshot_metadata: Option<RegisteredFlowSnapshotMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowSnapshotMetadataSetEntity {
    #[serde(rename = "versionedFlowSnapshotMetadataSet")]
    pub versioned_flow_snapshot_metadata_set: Option<Vec<VersionedFlowSnapshotMetadataEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowUpdateRequestDto {
    pub complete: Option<bool>,
    #[serde(rename = "failureReason")]
    pub failure_reason: Option<String>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,
    #[serde(rename = "percentCompleted")]
    pub percent_completed: Option<i32>,
    #[serde(rename = "processGroupId")]
    pub process_group_id: Option<String>,
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    pub state: Option<String>,
    pub uri: Option<String>,
    #[serde(rename = "versionControlInformation")]
    pub version_control_information: Option<VersionControlInformationDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowUpdateRequestEntity {
    #[serde(rename = "processGroupRevision")]
    pub process_group_revision: Option<RevisionDto>,
    pub request: Option<VersionedFlowUpdateRequestDto>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowsEntity {
    #[serde(rename = "versionedFlows")]
    pub versioned_flows: Option<Vec<VersionedFlowEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFunnel {
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<String>,
    pub identifier: Option<String>,
    #[serde(rename = "instanceIdentifier")]
    pub instance_identifier: Option<String>,
    pub name: Option<String>,
    pub position: Option<Position>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedLabel {
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<String>,
    pub height: Option<f64>,
    pub identifier: Option<String>,
    #[serde(rename = "instanceIdentifier")]
    pub instance_identifier: Option<String>,
    pub label: Option<String>,
    pub name: Option<String>,
    pub position: Option<Position>,
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    pub width: Option<f64>,
    #[serde(rename = "zIndex")]
    pub z_index: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedListenPortDefinition {
    #[serde(rename = "applicationProtocols")]
    pub application_protocols: Option<Vec<String>>,
    #[serde(rename = "transportProtocol")]
    pub transport_protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedParameter {
    pub description: Option<String>,
    pub name: Option<String>,
    pub provided: Option<bool>,
    #[serde(rename = "referencedAssets")]
    pub referenced_assets: Option<Vec<VersionedAsset>>,
    pub sensitive: Option<bool>,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedParameterContext {
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<String>,
    pub identifier: Option<String>,
    #[serde(rename = "inheritedParameterContexts")]
    pub inherited_parameter_contexts: Option<Vec<String>>,
    #[serde(rename = "instanceIdentifier")]
    pub instance_identifier: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "parameterGroupName")]
    pub parameter_group_name: Option<String>,
    #[serde(rename = "parameterProvider")]
    pub parameter_provider: Option<String>,
    pub parameters: Option<Vec<VersionedParameter>>,
    pub position: Option<Position>,
    pub synchronized: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedPort {
    #[serde(rename = "allowRemoteAccess")]
    pub allow_remote_access: Option<bool>,
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    #[serde(rename = "concurrentlySchedulableTaskCount")]
    pub concurrently_schedulable_task_count: Option<i32>,
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<String>,
    pub identifier: Option<String>,
    #[serde(rename = "instanceIdentifier")]
    pub instance_identifier: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "portFunction")]
    pub port_function: Option<String>,
    pub position: Option<Position>,
    #[serde(rename = "scheduledState")]
    pub scheduled_state: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedProcessGroup {
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    pub connections: Option<Vec<VersionedConnection>>,
    #[serde(rename = "controllerServices")]
    pub controller_services: Option<Vec<VersionedControllerService>>,
    #[serde(rename = "defaultBackPressureDataSizeThreshold")]
    pub default_back_pressure_data_size_threshold: Option<String>,
    #[serde(rename = "defaultBackPressureObjectThreshold")]
    pub default_back_pressure_object_threshold: Option<i64>,
    #[serde(rename = "defaultFlowFileExpiration")]
    pub default_flow_file_expiration: Option<String>,
    #[serde(rename = "executionEngine")]
    pub execution_engine: Option<String>,
    #[serde(rename = "flowFileConcurrency")]
    pub flow_file_concurrency: Option<String>,
    #[serde(rename = "flowFileOutboundPolicy")]
    pub flow_file_outbound_policy: Option<String>,
    pub funnels: Option<Vec<VersionedFunnel>>,
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<String>,
    pub identifier: Option<String>,
    #[serde(rename = "inputPorts")]
    pub input_ports: Option<Vec<VersionedPort>>,
    #[serde(rename = "instanceIdentifier")]
    pub instance_identifier: Option<String>,
    pub labels: Option<Vec<VersionedLabel>>,
    #[serde(rename = "logFileSuffix")]
    pub log_file_suffix: Option<String>,
    #[serde(rename = "maxConcurrentTasks")]
    pub max_concurrent_tasks: Option<i32>,
    pub name: Option<String>,
    #[serde(rename = "outputPorts")]
    pub output_ports: Option<Vec<VersionedPort>>,
    #[serde(rename = "parameterContextName")]
    pub parameter_context_name: Option<String>,
    pub position: Option<Position>,
    #[serde(rename = "processGroups")]
    pub process_groups: Option<Vec<Box<VersionedProcessGroup>>>,
    pub processors: Option<Vec<VersionedProcessor>>,
    #[serde(rename = "remoteProcessGroups")]
    pub remote_process_groups: Option<Vec<VersionedRemoteProcessGroup>>,
    #[serde(rename = "scheduledState")]
    pub scheduled_state: Option<String>,
    #[serde(rename = "statelessFlowTimeout")]
    pub stateless_flow_timeout: Option<String>,
    #[serde(rename = "versionedFlowCoordinates")]
    pub versioned_flow_coordinates: Option<VersionedFlowCoordinates>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedProcessor {
    #[serde(rename = "annotationData")]
    pub annotation_data: Option<String>,
    #[serde(rename = "autoTerminatedRelationships")]
    pub auto_terminated_relationships: Option<Vec<String>>,
    #[serde(rename = "backoffMechanism")]
    pub backoff_mechanism: Option<String>,
    #[serde(rename = "bulletinLevel")]
    pub bulletin_level: Option<String>,
    pub bundle: Option<Bundle>,
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    #[serde(rename = "concurrentlySchedulableTaskCount")]
    pub concurrently_schedulable_task_count: Option<i32>,
    #[serde(rename = "executionNode")]
    pub execution_node: Option<String>,
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<String>,
    pub identifier: Option<String>,
    #[serde(rename = "instanceIdentifier")]
    pub instance_identifier: Option<String>,
    #[serde(rename = "maxBackoffPeriod")]
    pub max_backoff_period: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "penaltyDuration")]
    pub penalty_duration: Option<String>,
    pub position: Option<Position>,
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "propertyDescriptors")]
    pub property_descriptors:
        Option<std::collections::HashMap<String, Option<VersionedPropertyDescriptor>>>,
    #[serde(rename = "retriedRelationships")]
    pub retried_relationships: Option<Vec<String>>,
    #[serde(rename = "retryCount")]
    pub retry_count: Option<i32>,
    #[serde(rename = "runDurationMillis")]
    pub run_duration_millis: Option<i64>,
    #[serde(rename = "scheduledState")]
    pub scheduled_state: Option<String>,
    #[serde(rename = "schedulingPeriod")]
    pub scheduling_period: Option<String>,
    #[serde(rename = "schedulingStrategy")]
    pub scheduling_strategy: Option<String>,
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    pub r#type: Option<String>,
    #[serde(rename = "yieldDuration")]
    pub yield_duration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedPropertyDescriptor {
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub dynamic: Option<bool>,
    #[serde(rename = "identifiesControllerService")]
    pub identifies_controller_service: Option<bool>,
    #[serde(rename = "listenPortDefinition")]
    pub listen_port_definition: Option<VersionedListenPortDefinition>,
    pub name: Option<String>,
    #[serde(rename = "resourceDefinition")]
    pub resource_definition: Option<VersionedResourceDefinition>,
    pub sensitive: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedRemoteGroupPort {
    #[serde(rename = "batchSize")]
    pub batch_size: Option<BatchSize>,
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    #[serde(rename = "concurrentlySchedulableTaskCount")]
    pub concurrently_schedulable_task_count: Option<i32>,
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<String>,
    pub identifier: Option<String>,
    #[serde(rename = "instanceIdentifier")]
    pub instance_identifier: Option<String>,
    pub name: Option<String>,
    pub position: Option<Position>,
    #[serde(rename = "remoteGroupId")]
    pub remote_group_id: Option<String>,
    #[serde(rename = "scheduledState")]
    pub scheduled_state: Option<String>,
    #[serde(rename = "targetId")]
    pub target_id: Option<String>,
    #[serde(rename = "useCompression")]
    pub use_compression: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedRemoteProcessGroup {
    pub comments: Option<String>,
    #[serde(rename = "communicationsTimeout")]
    pub communications_timeout: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<String>,
    pub identifier: Option<String>,
    #[serde(rename = "inputPorts")]
    pub input_ports: Option<Vec<VersionedRemoteGroupPort>>,
    #[serde(rename = "instanceIdentifier")]
    pub instance_identifier: Option<String>,
    #[serde(rename = "localNetworkInterface")]
    pub local_network_interface: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "outputPorts")]
    pub output_ports: Option<Vec<VersionedRemoteGroupPort>>,
    pub position: Option<Position>,
    #[serde(rename = "proxyHost")]
    pub proxy_host: Option<String>,
    #[serde(rename = "proxyPassword")]
    pub proxy_password: Option<String>,
    #[serde(rename = "proxyPort")]
    pub proxy_port: Option<i32>,
    #[serde(rename = "proxyUser")]
    pub proxy_user: Option<String>,
    #[serde(rename = "targetUris")]
    pub target_uris: Option<String>,
    #[serde(rename = "transportProtocol")]
    pub transport_protocol: Option<String>,
    #[serde(rename = "yieldDuration")]
    pub yield_duration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedReportingTask {
    #[serde(rename = "annotationData")]
    pub annotation_data: Option<String>,
    pub bundle: Option<Bundle>,
    pub comments: Option<String>,
    #[serde(rename = "componentType")]
    pub component_type: Option<String>,
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<String>,
    pub identifier: Option<String>,
    #[serde(rename = "instanceIdentifier")]
    pub instance_identifier: Option<String>,
    pub name: Option<String>,
    pub position: Option<Position>,
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "propertyDescriptors")]
    pub property_descriptors:
        Option<std::collections::HashMap<String, Option<VersionedPropertyDescriptor>>>,
    #[serde(rename = "scheduledState")]
    pub scheduled_state: Option<String>,
    #[serde(rename = "schedulingPeriod")]
    pub scheduling_period: Option<String>,
    #[serde(rename = "schedulingStrategy")]
    pub scheduling_strategy: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedReportingTaskImportRequestEntity {
    #[serde(rename = "disconnectedNodeAcknowledged")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(rename = "reportingTaskSnapshot")]
    pub reporting_task_snapshot: Option<VersionedReportingTaskSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedReportingTaskImportResponseEntity {
    #[serde(rename = "controllerServices")]
    pub controller_services: Option<Vec<ControllerServiceEntity>>,
    #[serde(rename = "reportingTasks")]
    pub reporting_tasks: Option<Vec<ReportingTaskEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedReportingTaskSnapshot {
    #[serde(rename = "controllerServices")]
    pub controller_services: Option<Vec<VersionedControllerService>>,
    #[serde(rename = "reportingTasks")]
    pub reporting_tasks: Option<Vec<VersionedReportingTask>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedResourceDefinition {
    pub cardinality: Option<String>,
    #[serde(rename = "resourceTypes")]
    pub resource_types: Option<Vec<String>>,
}
