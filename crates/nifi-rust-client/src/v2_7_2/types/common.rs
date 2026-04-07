// @generated — do not edit; run `cargo run -p nifi-openapi-gen`

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum AccessPolicyDtoAction {
    #[default]
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPolicyDto {
    /// The action associated with this access policy.
    pub action: Option<AccessPolicyDtoAction>,
    pub component_reference: Option<ComponentReferenceEntity>,
    /// Whether this policy is configurable.
    pub configurable: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    /// The resource for this access policy.
    pub resource: Option<String>,
    /// The set of user group IDs associated with this access policy.
    pub user_groups: Option<Vec<TenantEntity>>,
    /// The set of user IDs associated with this access policy.
    pub users: Option<Vec<TenantEntity>>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum AccessPolicySummaryDtoAction {
    #[default]
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPolicySummaryDto {
    /// The action associated with this access policy.
    pub action: Option<AccessPolicySummaryDtoAction>,
    pub component_reference: Option<ComponentReferenceEntity>,
    /// Whether this policy is configurable.
    pub configurable: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    /// The resource for this access policy.
    pub resource: Option<String>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
/// The access policies this user belongs to.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPolicySummaryEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<AccessPolicySummaryDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDto {
    pub action_details: Option<ActionDetailsDto>,
    pub component_details: Option<ComponentDetailsDto>,
    /// The action id.
    pub id: Option<i32>,
    /// The operation that was performed.
    pub operation: Option<String>,
    /// The id of the source component.
    pub source_id: Option<String>,
    /// The name of the source component.
    pub source_name: Option<String>,
    /// The type of the source component.
    pub source_type: Option<String>,
    /// The timestamp of the action.
    pub timestamp: Option<String>,
    /// The identity of the user that performed the action.
    pub user_identity: Option<String>,
}
/// The details of the action.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDetailsDto {}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum AffectedComponentDtoReferenceType {
    #[default]
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "STATELESS_GROUP")]
    StatelessGroup,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedComponentDto {
    /// The number of active threads for the referencing component.
    pub active_thread_count: Option<i32>,
    /// The UUID of this component
    pub id: Option<String>,
    /// The name of this component.
    pub name: Option<String>,
    /// The UUID of the Process Group that this component is in
    pub process_group_id: Option<String>,
    /// The type of this component
    pub reference_type: Option<AffectedComponentDtoReferenceType>,
    /// The scheduled state of a processor or reporting task referencing a controller service. If this component is another controller service, this field represents the controller service state.
    pub state: Option<String>,
    /// The validation errors for the component.
    pub validation_errors: Option<Vec<String>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum AffectedComponentEntityReferenceType {
    #[default]
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
}
/// The set of all components in the flow that are referencing this Parameter
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedComponentEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<AffectedComponentDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub process_group: Option<ProcessGroupNameDto>,
    /// The type of component referenced
    pub reference_type: Option<AffectedComponentEntityReferenceType>,
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowableValueDto {
    /// A description for this allowable value.
    pub description: Option<String>,
    /// A human readable value that is allowed for the property descriptor.
    pub display_name: Option<String>,
    /// A value that is allowed for the property descriptor.
    pub value: Option<String>,
}
/// Allowable values for the property. If empty then the allowed values are not constrained.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowableValueEntity {
    pub allowable_value: Option<AllowableValueDto>,
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
}
/// A list of identifiers of the assets that are referenced by the parameter
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetReferenceDto {
    /// The identifier of the referenced asset.
    pub id: Option<String>,
    /// The name of the referenced asset. Read-only — set by NiFi.
    pub name: Option<String>,
}
/// The FlowFile attributes this processor writes/updates
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    /// The description of the attribute
    pub description: Option<String>,
    /// The name of the attribute
    pub name: Option<String>,
}
/// The attributes of the flowfile for the event.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeDto {
    /// The attribute name.
    pub name: Option<String>,
    /// The value of the attribute before the event took place.
    pub previous_value: Option<String>,
    /// The attribute value.
    pub value: Option<String>,
}
/// The batch settings for data transmission.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSettingsDto {
    /// Preferred number of flow files to include in a transaction.
    pub count: Option<i32>,
    /// Preferred amount of time that a transaction should span.
    pub duration: Option<String>,
    /// Preferred number of bytes to include in a transaction.
    pub size: Option<String>,
}
/// The batch settings for data transmission.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSize {
    /// Preferred number of flow files to include in a transaction.
    pub count: Option<i32>,
    /// Preferred amount of time that a transaction should span.
    pub duration: Option<String>,
    /// Preferred number of bytes to include in a transaction.
    pub size: Option<String>,
}
/// The build metadata for this component
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
    /// The compiler used for the build
    pub compiler: Option<String>,
    /// The compiler flags used for the build.
    pub compiler_flags: Option<String>,
    /// The SCM revision id of the source code used for this build.
    pub revision: Option<String>,
    /// The target architecture of the built component.
    pub target_arch: Option<String>,
    /// The timestamp (milliseconds since Epoch) of the build.
    pub timestamp: Option<i64>,
    /// The version number of the built component.
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinBoardPatternParameter {
    pub pattern: Option<serde_json::Value>,
    pub raw_pattern: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinDto {
    /// The category of this bulletin.
    pub category: Option<String>,
    /// The group id of the source component.
    pub group_id: Option<String>,
    /// The id of the bulletin.
    pub id: Option<i64>,
    /// The level of the bulletin.
    pub level: Option<String>,
    /// The bulletin message.
    pub message: Option<String>,
    /// If clustered, the address of the node from which the bulletin originated.
    pub node_address: Option<String>,
    /// The id of the source component.
    pub source_id: Option<String>,
    /// The name of the source component.
    pub source_name: Option<String>,
    /// The type of the source component
    pub source_type: Option<String>,
    /// The stack trace associated with the bulletin, if any.
    pub stack_trace: Option<String>,
    /// When this bulletin was generated.
    pub timestamp: Option<String>,
    /// When this bulletin was generated in ISO format with full date and milliseconds.
    pub timestamp_iso: Option<String>,
}
/// The details of the artifact that bundled this parameter provider.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    /// The artifact of the bundle
    pub artifact: Option<String>,
    /// The group of the bundle
    pub group: Option<String>,
    /// The version of the bundle
    pub version: Option<String>,
}
/// If the property identifies a controller service this returns the bundle of the type, null otherwise.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleDto {
    /// The artifact of the bundle.
    pub artifact: Option<String>,
    /// The group of the bundle.
    pub group: Option<String>,
    /// The version of the bundle.
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearBulletinsRequestEntity {
    /// The timestamp from which to clear bulletins (inclusive). This field is required.
    pub from_timestamp: String,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearBulletinsResultEntity {
    /// The current bulletins for the component after clearing.
    pub bulletins: Option<Vec<BulletinEntity>>,
    /// The number of bulletins that were cleared.
    pub bulletins_cleared: Option<i32>,
    /// The id of the component for which bulletins were cleared.
    pub component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientIdParameter {
    pub client_id: Option<String>,
}
/// The details of the source component.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDetailsDto {}
/// The list of differences for each component in the flow that is not the same between the two flows
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDifferenceDto {
    /// The ID of the component
    pub component_id: Option<String>,
    /// The name of the component
    pub component_name: Option<String>,
    /// The type of component
    pub component_type: Option<String>,
    /// The differences in the component between the two flows
    pub differences: Option<Vec<DifferenceDto>>,
    /// The ID of the Process Group that the component belongs to
    pub process_group_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentHistoryDto {
    /// The component id.
    pub component_id: Option<String>,
    /// The history for the properties of the component.
    pub property_history: Option<std::collections::HashMap<String, Option<PropertyHistoryDto>>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentHistoryEntity {
    pub component_history: ComponentHistoryDto,
}
/// The full specification of the bundle contents
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentManifest {
    /// Public interfaces defined in this bundle
    pub apis: Option<Vec<DefinedType>>,
    /// Controller Services provided in this bundle
    pub controller_services: Option<Vec<ControllerServiceDefinition>>,
    /// Flow Analysis Rules provided in this bundle
    pub flow_analysis_rules: Option<Vec<FlowAnalysisRuleDefinition>>,
    /// Flow Registry Clients provided in this bundle
    pub flow_registry_clients: Option<Vec<FlowRegistryClientDefinition>>,
    /// Parameter Providers provided in this bundle
    pub parameter_providers: Option<Vec<ParameterProviderDefinition>>,
    /// Processors provided in this bundle
    pub processors: Option<Vec<ProcessorDefinition>>,
    /// Reporting Tasks provided in this bundle
    pub reporting_tasks: Option<Vec<ReportingTaskDefinition>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentReferenceDto {
    /// The id of the component.
    pub id: Option<String>,
    /// The name of the component.
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
/// Component this policy references if applicable.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentReferenceEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ComponentReferenceDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
/// Permissions for specific component restrictions.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentRestrictionPermissionDto {
    pub permissions: Option<PermissionsDto>,
    pub required_permission: Option<RequiredPermissionDto>,
}
/// The parameters that matched the search.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentSearchResultDto {
    /// The group id of the component that matched the search.
    pub group_id: Option<String>,
    /// The id of the component that matched the search.
    pub id: Option<String>,
    /// What matched the search from the component.
    pub matches: Option<Vec<String>>,
    /// The name of the component that matched the search.
    pub name: Option<String>,
    pub parent_group: Option<SearchResultGroupDto>,
    pub versioned_group: Option<SearchResultGroupDto>,
}
/// The component state.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStateDto {
    pub cluster_state: Option<StateMapDto>,
    /// The component identifier.
    pub component_id: Option<String>,
    /// Whether dropping state by key is supported for this component. Defaults to false when not specified by the component.
    pub drop_state_key_supported: Option<bool>,
    pub local_state: Option<StateMapDto>,
    /// Description of the state this component persists.
    pub state_description: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStateEntity {
    pub component_state: ComponentStateDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ComponentValidationResultDtoReferenceType {
    #[default]
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "STATELESS_GROUP")]
    StatelessGroup,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentValidationResultDto {
    /// The number of active threads for the referencing component.
    pub active_thread_count: Option<i32>,
    /// Whether or not the component is currently valid
    pub currently_valid: Option<bool>,
    /// The UUID of this component
    pub id: Option<String>,
    /// The name of this component.
    pub name: Option<String>,
    /// The UUID of the Process Group that this component is in
    pub process_group_id: Option<String>,
    /// The type of this component
    pub reference_type: Option<ComponentValidationResultDtoReferenceType>,
    /// The validation errors that will apply to the component if the Parameter Context is changed
    pub resultant_validation_errors: Option<Vec<String>>,
    /// Whether or not the component will be valid if the Parameter Context is changed
    pub results_valid: Option<bool>,
    /// The scheduled state of a processor or reporting task referencing a controller service. If this component is another controller service, this field represents the controller service state.
    pub state: Option<String>,
    /// The validation errors for the component.
    pub validation_errors: Option<Vec<String>>,
}
/// A List of ComponentValidationResultEntity, one for each component that is validated
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentValidationResultEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ComponentValidationResultDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
/// The Validation Results that were calculated for each component. This value may not be set until the request completes.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentValidationResultsEntity {
    /// A List of ComponentValidationResultEntity, one for each component that is validated
    pub validation_results: Option<Vec<ComponentValidationResultEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ConfigVerificationResultDtoOutcome {
    #[default]
    #[serde(rename = "SUCCESSFUL")]
    Successful,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "SKIPPED")]
    Skipped,
}
/// The Results of the verification
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigVerificationResultDto {
    /// An explanation of why the step was or was not successful
    pub explanation: Option<String>,
    /// The outcome of the verification
    pub outcome: Option<ConfigVerificationResultDtoOutcome>,
    /// The name of the verification step
    pub verification_step_name: Option<String>,
}
/// The configuration analysis
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationAnalysisDto {
    /// The ID of the component
    pub component_id: Option<String>,
    /// The configured properties for the component
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The attributes that are referenced by the properties, mapped to recently used values
    pub referenced_attributes: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether or not the component supports verification
    pub supports_verification: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationAnalysisEntity {
    pub configuration_analysis: ConfigurationAnalysisDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ConnectableComponentType {
    #[default]
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
}
/// The destination of the connection.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectableComponent {
    /// The comments for the connectable component.
    pub comments: Option<String>,
    /// The id of the group that the connectable component resides in
    pub group_id: Option<String>,
    /// The id of the connectable component.
    pub id: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    pub instance_identifier: Option<String>,
    /// The name of the connectable component
    pub name: Option<String>,
    /// The type of component the connectable is.
    #[serde(rename = "type")]
    pub r#type: Option<ConnectableComponentType>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ConnectableDtoType {
    #[default]
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
}
/// The destination of the connection.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectableDto {
    /// The comments for the connectable component.
    pub comments: Option<String>,
    /// If the connectable component represents a remote port, indicates if the target exists.
    pub exists: Option<bool>,
    /// The id of the group that the connectable component resides in
    pub group_id: String,
    /// The id of the connectable component.
    pub id: String,
    /// The name of the connectable component
    pub name: Option<String>,
    /// Reflects the current state of the connectable component.
    pub running: Option<bool>,
    /// If the connectable component represents a remote port, indicates if the target is configured to transmit.
    pub transmitting: Option<bool>,
    /// The type of component the connectable is.
    #[serde(rename = "type")]
    pub r#type: ConnectableDtoType,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
/// The connections in this flow snippet.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDto {
    /// The relationships that the source of the connection currently supports. Read-only — set by NiFi.
    pub available_relationships: Option<Vec<String>>,
    /// The object data size threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue.
    pub back_pressure_data_size_threshold: Option<String>,
    /// The object count threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue.
    pub back_pressure_object_threshold: Option<i64>,
    /// The bend points on the connection.
    pub bends: Option<Vec<PositionDto>>,
    pub destination: Option<ConnectableDto>,
    /// The amount of time a flow file may be in the flow before it will be automatically aged out of the flow. Once a flow file reaches this age it will be terminated from the flow the next time a processor attempts to start work on it.
    pub flow_file_expiration: Option<String>,
    /// The z index of the connection.
    pub getz_index: Option<i64>,
    /// The id of the component.
    pub id: Option<String>,
    /// The index of the bend point where to place the connection label.
    pub label_index: Option<i32>,
    /// Whether or not data should be compressed when being transferred between nodes in the cluster. Possible returned values: DO_NOT_COMPRESS, COMPRESS_ATTRIBUTES_ONLY, COMPRESS_ATTRIBUTES_AND_CONTENT. See LoadBalanceCompression.class for more details.
    pub load_balance_compression: Option<String>,
    /// The FlowFile Attribute to use for determining which node a FlowFile will go to if the Load Balancing Strategy is set to PARTITION_BY_ATTRIBUTE
    pub load_balance_partition_attribute: Option<String>,
    /// The current status of the Connection's Load Balancing Activities. Status can indicate that Load Balancing is not configured for the connection, that Load Balancing is configured but inactive (not currently transferring data to another node), or that Load Balancing is configured and actively transferring data to another node. Possible returned values: LOAD_BALANCE_NOT_CONFIGURED, LOAD_BALANCE_INACTIVE, LOAD_BALANCE_ACTIVE. See LoadBalanceStatus.class for more details. Read-only — set by NiFi.
    pub load_balance_status: Option<String>,
    /// How to load balance the data in this Connection across the nodes in the cluster. Possible returned values: DO_NOT_LOAD_BALANCE, PARTITION_BY_ATTRIBUTE, ROUND_ROBIN, SINGLE_NODE. See LoadBalanceStrategy.class for more details.
    pub load_balance_strategy: Option<String>,
    /// The name of the connection.
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    /// The comparators used to prioritize the queue.
    pub prioritizers: Option<Vec<String>>,
    /// The relationships from the source of the connection that are configured to be retried. Read-only — set by NiFi.
    pub retried_relationships: Option<Vec<String>>,
    /// The selected relationship that comprise the connection.
    pub selected_relationships: Option<Vec<String>>,
    pub source: Option<ConnectableDto>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ConnectionEntityDestinationType {
    #[default]
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ConnectionEntitySourceType {
    #[default]
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
}
/// The connections in this flow.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionEntity {
    /// The bend points on the connection.
    pub bends: Option<Vec<PositionDto>>,
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ConnectionDto>,
    /// The identifier of the group of the destination of this connection.
    pub destination_group_id: Option<String>,
    /// The identifier of the destination of this connection.
    pub destination_id: Option<String>,
    /// The type of component the destination connectable is.
    pub destination_type: ConnectionEntityDestinationType,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The z index of the connection.
    pub getz_index: Option<i64>,
    /// The id of the component.
    pub id: Option<String>,
    /// The index of the bend point where to place the connection label.
    pub label_index: Option<i32>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    /// The identifier of the group of the source of this connection.
    pub source_group_id: Option<String>,
    /// The identifier of the source of this connection.
    pub source_id: Option<String>,
    /// The type of component the source connectable is.
    pub source_type: ConnectionEntitySourceType,
    pub status: Option<ConnectionStatusDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatisticsDto {
    pub aggregate_snapshot: Option<ConnectionStatisticsSnapshotDto>,
    /// The ID of the connection
    pub id: Option<String>,
    /// A list of status snapshots for each node
    pub node_snapshots: Option<Vec<NodeConnectionStatisticsSnapshotDto>>,
    /// The timestamp of when the stats were last refreshed
    pub stats_last_refreshed: Option<String>,
}
/// The connection status snapshot from the node.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatisticsSnapshotDto {
    /// The id of the connection.
    pub id: Option<String>,
    /// The predicted total number of bytes in the queue at the next configured interval.
    pub predicted_bytes_at_next_interval: Option<i64>,
    /// The predicted number of queued objects at the next configured interval.
    pub predicted_count_at_next_interval: Option<i32>,
    /// The predicted number of milliseconds before the connection will have backpressure applied, based on the total number of bytes in the queue.
    pub predicted_millis_until_bytes_backpressure: Option<i64>,
    /// The predicted number of milliseconds before the connection will have backpressure applied, based on the queued count.
    pub predicted_millis_until_count_backpressure: Option<i64>,
    /// The predicted percentage of bytes in the queue against current threshold at the next configured interval.
    pub predicted_percent_bytes: Option<i32>,
    /// The predicted percentage of queued objects at the next configured interval.
    pub predicted_percent_count: Option<i32>,
    /// The prediction interval in seconds
    pub prediction_interval_millis: Option<i64>,
}
/// The status of the connection.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusDto {
    pub aggregate_snapshot: Option<ConnectionStatusSnapshotDto>,
    /// The ID of the destination component
    pub destination_id: Option<String>,
    /// The name of the destination component
    pub destination_name: Option<String>,
    /// The ID of the Process Group that the connection belongs to
    pub group_id: Option<String>,
    /// The ID of the connection
    pub id: Option<String>,
    /// The name of the connection
    pub name: Option<String>,
    /// A list of status snapshots for each node
    pub node_snapshots: Option<Vec<NodeConnectionStatusSnapshotDto>>,
    /// The ID of the source component
    pub source_id: Option<String>,
    /// The name of the source component
    pub source_name: Option<String>,
    /// The timestamp of when the stats were last refreshed
    pub stats_last_refreshed: Option<String>,
}
/// Predictions, if available, for this connection (null if not available)
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusPredictionsSnapshotDto {
    /// The predicted total number of bytes in the queue at the next configured interval.
    pub predicted_bytes_at_next_interval: Option<i64>,
    /// The predicted number of queued objects at the next configured interval.
    pub predicted_count_at_next_interval: Option<i32>,
    /// The predicted number of milliseconds before the connection will have backpressure applied, based on the total number of bytes in the queue.
    pub predicted_millis_until_bytes_backpressure: Option<i64>,
    /// The predicted number of milliseconds before the connection will have backpressure applied, based on the queued count.
    pub predicted_millis_until_count_backpressure: Option<i64>,
    /// Predicted connection percent use regarding queued flow files size and backpressure threshold if configured.
    pub predicted_percent_bytes: Option<i32>,
    /// Predicted connection percent use regarding queued flow files count and backpressure threshold if configured.
    pub predicted_percent_count: Option<i32>,
    /// The configured interval (in seconds) for predicting connection queue count and size (and percent usage).
    pub prediction_interval_seconds: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ConnectionStatusSnapshotDtoLoadBalanceStatus {
    #[default]
    #[serde(rename = "LOAD_BALANCE_NOT_CONFIGURED")]
    LoadBalanceNotConfigured,
    #[serde(rename = "LOAD_BALANCE_ACTIVE")]
    LoadBalanceActive,
    #[serde(rename = "LOAD_BALANCE_INACTIVE")]
    LoadBalanceInactive,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusSnapshotDto {
    /// The size of the FlowFiles that have come into the connection in the last 5 minutes.
    pub bytes_in: Option<i64>,
    /// The number of bytes that have left the connection in the last 5 minutes.
    pub bytes_out: Option<i64>,
    /// The size of the FlowFiles that are currently queued in the connection.
    pub bytes_queued: Option<i64>,
    /// The id of the destination of the connection.
    pub destination_id: Option<String>,
    /// The name of the destination of the connection.
    pub destination_name: Option<String>,
    /// The availability of FlowFiles in this connection
    pub flow_file_availability: Option<String>,
    /// The number of FlowFiles that have come into the connection in the last 5 minutes.
    pub flow_files_in: Option<i32>,
    /// The number of FlowFiles that have left the connection in the last 5 minutes.
    pub flow_files_out: Option<i32>,
    /// The number of FlowFiles that are currently queued in the connection.
    pub flow_files_queued: Option<i32>,
    /// The id of the process group the connection belongs to.
    pub group_id: Option<String>,
    /// The id of the connection.
    pub id: Option<String>,
    /// The input count/size for the connection in the last 5 minutes, pretty printed.
    pub input: Option<String>,
    /// The load balance status of the connection
    pub load_balance_status: Option<ConnectionStatusSnapshotDtoLoadBalanceStatus>,
    /// The name of the connection.
    pub name: Option<String>,
    /// The output count/size for the connection in the last 5 minutes, pretty printed.
    pub output: Option<String>,
    /// Connection percent use regarding queued flow files size and backpressure threshold if configured.
    pub percent_use_bytes: Option<i32>,
    /// Connection percent use regarding queued flow files count and backpressure threshold if configured.
    pub percent_use_count: Option<i32>,
    pub predictions: Option<ConnectionStatusPredictionsSnapshotDto>,
    /// The total count and size of queued flowfiles formatted.
    pub queued: Option<String>,
    /// The number of flowfiles that are queued, pretty printed.
    pub queued_count: Option<String>,
    /// The total size of flowfiles that are queued formatted.
    pub queued_size: Option<String>,
    /// The id of the source of the connection.
    pub source_id: Option<String>,
    /// The name of the source of the connection.
    pub source_name: Option<String>,
}
/// The status of all connections in the process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
    pub connection_status_snapshot: Option<ConnectionStatusSnapshotDto>,
    /// The id of the connection.
    pub id: Option<String>,
}
/// The Content Viewers.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentViewerDto {
    /// The display name of the Content Viewer. Read-only — set by NiFi.
    pub display_name: Option<String>,
    /// The mime types this Content Viewer supports. Read-only — set by NiFi.
    pub supported_mime_types: Option<Vec<SupportedMimeTypesDto>>,
    /// The uri of the Content Viewer. Read-only — set by NiFi.
    pub uri: Option<String>,
}
/// The controller configuration.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerConfigurationDto {
    /// The maximum number of timer driven threads the NiFi has available.
    pub max_timer_driven_thread_count: Option<i32>,
}
/// Lists the APIs this Controller Service implements.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceAPI {
    pub bundle: Option<Bundle>,
    /// The fully qualified name of the service interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
/// Lists the APIs this Controller Service implements.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceApiDto {
    pub bundle: Option<BundleDto>,
    /// The fully qualified name of the service interface.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ControllerServiceDtoState {
    #[default]
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "ENABLING")]
    Enabling,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "DISABLING")]
    Disabling,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ControllerServiceDtoValidationStatus {
    #[default]
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "VALIDATING")]
    Validating,
}
/// The controller services in this flow snippet.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceDto {
    /// The annotation for the controller service. This is how the custom UI relays configuration to the controller service.
    pub annotation_data: Option<String>,
    /// The level at which the controller service will report bulletins.
    pub bulletin_level: Option<String>,
    pub bundle: Option<BundleDto>,
    /// The comments for the controller service.
    pub comments: Option<String>,
    /// Lists the APIs this Controller Service implements.
    pub controller_service_apis: Option<Vec<ControllerServiceApiDto>>,
    /// The URL for the controller services custom configuration UI if applicable.
    pub custom_ui_url: Option<String>,
    /// Whether the ontroller service has been deprecated.
    pub deprecated: Option<bool>,
    /// The descriptors for the controller service properties.
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Whether the underlying extension is missing.
    pub extension_missing: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    /// Whether the controller service has multiple versions available.
    pub multiple_versions_available: Option<bool>,
    /// The name of the controller service.
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    /// Whether the controller service persists state.
    pub persists_state: Option<bool>,
    pub position: Option<PositionDto>,
    /// The properties of the controller service.
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// All components referencing this controller service.
    pub referencing_components: Option<Vec<ControllerServiceReferencingComponentEntity>>,
    /// Whether the controller service requires elevated privileges.
    pub restricted: Option<bool>,
    /// Set of sensitive dynamic property names
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// The state of the controller service.
    pub state: Option<ControllerServiceDtoState>,
    /// Whether the controller service supports sensitive dynamic properties.
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The type of the controller service.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The validation errors from the controller service.
    /// These validation errors represent the problems with the controller service that must be resolved before it can be enabled.
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the ControllerService is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the ControllerService is valid) Read-only — set by NiFi.
    pub validation_status: Option<ControllerServiceDtoValidationStatus>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ControllerServiceDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    pub operate_permissions: Option<PermissionsDto>,
    /// The id of parent process group of this ControllerService.
    pub parent_group_id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub status: Option<ControllerServiceStatusDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ControllerServiceReferencingComponentDtoReferenceType {
    #[default]
    #[serde(rename = "Processor")]
    Processor,
    #[serde(rename = "ControllerService")]
    Controllerservice,
    #[serde(rename = "ReportingTask")]
    Reportingtask,
    #[serde(rename = "FlowRegistryClient")]
    Flowregistryclient,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceReferencingComponentDto {
    /// The number of active threads for the referencing component.
    pub active_thread_count: Option<i32>,
    /// The descriptors for the component properties.
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// The group id for the component referencing a controller service. If this component is another controller service or a reporting task, this field is blank.
    pub group_id: Option<String>,
    /// The id of the component referencing a controller service.
    pub id: Option<String>,
    /// The name of the component referencing a controller service.
    pub name: Option<String>,
    /// The properties for the component.
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// If the referencing component represents a controller service, this indicates whether it has already been represented in this hierarchy.
    pub reference_cycle: Option<bool>,
    /// The type of reference this is.
    pub reference_type: Option<ControllerServiceReferencingComponentDtoReferenceType>,
    /// If the referencing component represents a controller service, these are the components that reference it.
    pub referencing_components: Option<Vec<ControllerServiceReferencingComponentEntity>>,
    /// The scheduled state of a processor or reporting task referencing a controller service. If this component is another controller service, this field represents the controller service state.
    pub state: Option<String>,
    /// The type of the component referencing a controller service in simple Java class name format without package name.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The validation errors for the component.
    pub validation_errors: Option<Vec<String>>,
}
/// All components referencing this controller service.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceReferencingComponentEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ControllerServiceReferencingComponentDto>,
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
pub enum ControllerServiceStatusDtoRunStatus {
    #[default]
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "ENABLING")]
    Enabling,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "DISABLING")]
    Disabling,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ControllerServiceStatusDtoValidationStatus {
    #[default]
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "VALIDATING")]
    Validating,
}
/// The status for this ControllerService.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceStatusDto {
    /// The number of active threads for the component.
    pub active_thread_count: Option<i32>,
    /// The run status of this ControllerService Read-only — set by NiFi.
    pub run_status: Option<ControllerServiceStatusDtoRunStatus>,
    /// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid) Read-only — set by NiFi.
    pub validation_status: Option<ControllerServiceStatusDtoValidationStatus>,
}
/// The counters from the node.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountersSnapshotDto {
    /// All counters in the NiFi.
    pub counters: Option<Vec<CounterDto>>,
    /// The timestamp when the report was generated.
    pub generated: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeParameter {
    pub date_time: Option<String>,
}
/// Indicates that this property is for selecting a controller service of the specified type
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefinedType {
    /// The artifact name of the bundle that provides the referenced type.
    pub artifact: Option<String>,
    /// The group name of the bundle that provides the referenced type.
    pub group: Option<String>,
    /// The fully-qualified class type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The description of the type.
    pub type_description: Option<String>,
    /// The version of the bundle that provides the referenced type.
    pub version: Option<String>,
}
/// The differences in the component between the two flows
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DifferenceDto {
    /// Description of the difference
    pub difference: Option<String>,
    /// The type of difference
    pub difference_type: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionsDto {
    /// The height of the label in pixels when at a 1:1 scale.
    pub height: Option<f64>,
    /// The width of the label in pixels when at a 1:1 scale.
    pub width: Option<f64>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentedTypeDto {
    pub bundle: Option<BundleDto>,
    /// If this type represents a ControllerService, this lists the APIs it implements.
    pub controller_service_apis: Option<Vec<ControllerServiceApiDto>>,
    /// The description of why the usage of this component is restricted.
    pub deprecation_reason: Option<String>,
    /// The description of the type.
    pub description: Option<String>,
    /// An optional collection of explicit restrictions. If specified, these explicit restrictions will be enfored.
    pub explicit_restrictions: Option<Vec<ExplicitRestrictionDto>>,
    /// Whether this type is restricted.
    pub restricted: Option<bool>,
    /// The tags associated with this type.
    pub tags: Option<Vec<String>>,
    /// The fully qualified name of the type.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The optional description of why the usage of this component is restricted.
    pub usage_restriction: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropRequestDto {
    /// The count and size of flow files currently queued.
    pub current: Option<String>,
    /// The number of flow files currently queued.
    pub current_count: Option<i32>,
    /// The size of flow files currently queued in bytes.
    pub current_size: Option<i64>,
    /// The count and size of flow files that have been dropped thus far.
    pub dropped: Option<String>,
    /// The number of flow files that have been dropped thus far.
    pub dropped_count: Option<i32>,
    /// The size of flow files that have been dropped thus far in bytes.
    pub dropped_size: Option<i64>,
    /// The reason, if any, that this drop request failed.
    pub failure_reason: Option<String>,
    /// Whether the query has finished.
    pub finished: Option<bool>,
    /// The id for this drop request.
    pub id: Option<String>,
    /// The last time this drop request was updated.
    pub last_updated: Option<String>,
    /// The count and size of flow files to be dropped as a result of this request.
    pub original: Option<String>,
    /// The number of flow files to be dropped as a result of this request.
    pub original_count: Option<i32>,
    /// The size of flow files to be dropped as a result of this request in bytes.
    pub original_size: Option<i64>,
    /// The current percent complete.
    pub percent_completed: Option<i32>,
    /// The current state of the drop request.
    pub state: Option<String>,
    /// The timestamp when the query was submitted.
    pub submission_time: Option<String>,
    /// The URI for future requests to this drop request.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropRequestEntity {
    pub drop_request: DropRequestDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum DynamicPropertyExpressionLanguageScope {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "ENVIRONMENT")]
    Environment,
    #[serde(rename = "FLOWFILE_ATTRIBUTES")]
    FlowfileAttributes,
}
/// Describes the dynamic properties supported by this component
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicProperty {
    /// The description of the dynamic property
    pub description: Option<String>,
    /// The scope of the expression language support
    pub expression_language_scope: Option<DynamicPropertyExpressionLanguageScope>,
    /// The description of the dynamic property name
    pub name: Option<String>,
    /// The description of the dynamic property value
    pub value: Option<String>,
}
/// If the processor supports dynamic relationships, this describes the dynamic relationship
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicRelationship {
    /// The description of the dynamic relationship
    pub description: Option<String>,
    /// The description of the dynamic relationship name
    pub name: Option<String>,
}
/// An optional collection of explicit restrictions. If specified, these explicit restrictions will be enfored.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplicitRestrictionDto {
    /// The description of why the usage of this component is restricted for this required permission.
    pub explanation: Option<String>,
    pub required_permission: Option<RequiredPermissionDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalControllerServiceReference {
    /// The identifier of the controller service
    pub identifier: Option<String>,
    /// The name of the controller service
    pub name: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum FlowAnalysisRuleDtoState {
    #[default]
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum FlowAnalysisRuleDtoValidationStatus {
    #[default]
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "VALIDATING")]
    Validating,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleDto {
    pub bundle: Option<BundleDto>,
    /// The comments of the flow analysis rule.
    pub comments: Option<String>,
    /// Whether the flow analysis rule has been deprecated.
    pub deprecated: Option<bool>,
    /// The descriptors for the flow analysis rules properties.
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Enforcement Policy.
    pub enforcement_policy: Option<String>,
    /// Whether the underlying extension is missing.
    pub extension_missing: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    /// Whether the flow analysis rule has multiple versions available.
    pub multiple_versions_available: Option<bool>,
    /// The name of the flow analysis rule.
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    /// Whether the flow analysis rule persists state.
    pub persists_state: Option<bool>,
    pub position: Option<PositionDto>,
    /// The properties of the flow analysis rule.
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether the flow analysis rule requires elevated privileges.
    pub restricted: Option<bool>,
    /// Set of sensitive dynamic property names
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// The state of the flow analysis rule.
    pub state: Option<FlowAnalysisRuleDtoState>,
    /// Whether the flow analysis rule supports sensitive dynamic properties.
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The fully qualified type of the flow analysis rule.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Gets the validation errors from the flow analysis rule. These validation errors represent the problems with the flow analysis rule that must be resolved before it can be scheduled to run.
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Flow Analysis Rule is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Flow Analysis Rule is valid) Read-only — set by NiFi.
    pub validation_status: Option<FlowAnalysisRuleDtoValidationStatus>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum FlowAnalysisRuleStatusDtoRunStatus {
    #[default]
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum FlowAnalysisRuleStatusDtoValidationStatus {
    #[default]
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "VALIDATING")]
    Validating,
}
/// The status for this FlowAnalysisRule.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleStatusDto {
    /// The number of active threads for the component.
    pub active_thread_count: Option<i32>,
    /// The run status of this FlowAnalysisRule Read-only — set by NiFi.
    pub run_status: Option<FlowAnalysisRuleStatusDtoRunStatus>,
    /// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid) Read-only — set by NiFi.
    pub validation_status: Option<FlowAnalysisRuleStatusDtoValidationStatus>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleViolationDto {
    pub enabled: Option<bool>,
    pub enforcement_policy: Option<String>,
    pub group_id: Option<String>,
    pub issue_id: Option<String>,
    pub rule_id: Option<String>,
    pub scope: Option<String>,
    pub subject_component_type: Option<String>,
    pub subject_display_name: Option<String>,
    pub subject_id: Option<String>,
    pub subject_permission_dto: Option<PermissionsDto>,
    pub violation_message: Option<String>,
}
/// This breadcrumb.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowBreadcrumbDto {
    /// The id of the group.
    pub id: Option<String>,
    /// The id of the group.
    pub name: Option<String>,
    pub version_control_information: Option<VersionControlInformationDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowComparisonEntity {
    /// The list of differences for each component in the flow that is not the same between the two flows
    pub component_differences: Option<Vec<ComponentDifferenceDto>>,
}
/// The FlowFile summaries. The summaries will be populated once the request has completed.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowFileSummaryDto {
    /// The label for the node where this FlowFile resides.
    pub cluster_node_address: Option<String>,
    /// The id of the node where this FlowFile resides.
    pub cluster_node_id: Option<String>,
    /// The FlowFile filename.
    pub filename: Option<String>,
    /// Duration since the FlowFile's greatest ancestor entered the flow.
    pub lineage_duration: Option<i64>,
    /// The FlowFile mime type.
    pub mime_type: Option<String>,
    /// If the FlowFile is penalized.
    pub penalized: Option<bool>,
    /// How long in milliseconds until the FlowFile penalty expires.
    pub penalty_expires_in: Option<i64>,
    /// The FlowFile's position in the queue.
    pub position: Option<i32>,
    /// How long this FlowFile has been enqueued.
    pub queued_duration: Option<i64>,
    /// The FlowFile file size.
    pub size: Option<i64>,
    /// The URI that can be used to access this FlowFile.
    pub uri: Option<String>,
    /// The FlowFile UUID.
    pub uuid: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBranchDto {
    /// The branch name
    pub name: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBranchEntity {
    pub branch: FlowRegistryBranchDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBucket {
    pub created_timestamp: Option<i64>,
    pub description: Option<String>,
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub permissions: Option<FlowRegistryPermissions>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBucketDto {
    /// The created timestamp of this bucket
    pub created: Option<i64>,
    /// The bucket description
    pub description: Option<String>,
    /// The bucket identifier
    pub id: Option<String>,
    /// The bucket name
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
pub enum FlowRegistryClientDtoValidationStatus {
    #[default]
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "VALIDATING")]
    Validating,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientDto {
    /// The annotation data for the registry client. This is how the custom UI relays configuration to the registry client.
    pub annotation_data: Option<String>,
    pub bundle: Option<BundleDto>,
    /// Whether the registry client has been deprecated.
    pub deprecated: Option<bool>,
    /// The registry description
    pub description: Option<String>,
    /// The descriptors for the registry client properties.
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Whether the underlying extension is missing.
    pub extension_missing: Option<bool>,
    /// The registry identifier
    pub id: Option<String>,
    /// Whether the flow registry client has multiple versions available.
    pub multiple_versions_available: Option<bool>,
    /// The registry name
    pub name: Option<String>,
    /// The properties of the registry client.
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether the registry client requires elevated privileges.
    pub restricted: Option<bool>,
    /// Set of sensitive dynamic property names
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// Whether the registry client supports branching.
    pub supports_branching: Option<bool>,
    /// Whether the registry client supports sensitive dynamic properties.
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The type of the registry client.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Gets the validation errors from the registry client. These validation errors represent the problems with the registry client that must be resolved before it can be used for interacting with the flow registry.
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Registry Client is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Registry Client is valid) Read-only — set by NiFi.
    pub validation_status: Option<FlowRegistryClientDtoValidationStatus>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientsEntity {
    /// The current time on the system.
    pub current_time: Option<String>,
    pub registries: Option<Vec<FlowRegistryClientEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryPermissions {
    pub can_delete: Option<bool>,
    pub can_read: Option<bool>,
    pub can_write: Option<bool>,
}
/// The contents of this process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowSnippetDto {
    /// The connections in this flow snippet.
    pub connections: Option<Vec<ConnectionDto>>,
    /// The controller services in this flow snippet.
    pub controller_services: Option<Vec<ControllerServiceDto>>,
    /// The funnels in this flow snippet.
    pub funnels: Option<Vec<FunnelDto>>,
    /// The input ports in this flow snippet.
    pub input_ports: Option<Vec<PortDto>>,
    /// The labels in this flow snippet.
    pub labels: Option<Vec<LabelDto>>,
    /// The output ports in this flow snippet.
    pub output_ports: Option<Vec<PortDto>>,
    /// The process groups in this flow snippet.
    pub process_groups: Option<Vec<ProcessGroupDto>>,
    /// The processors in this flow snippet.
    pub processors: Option<Vec<ProcessorDto>>,
    /// The remote process groups in this flow snippet.
    pub remote_process_groups: Option<Vec<RemoteProcessGroupDto>>,
}
/// The funnels in this flow snippet.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunnelDto {
    /// The id of the component.
    pub id: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
/// The funnels in this flow.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunnelEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<FunnelDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
/// The garbage collection details.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GarbageCollectionDto {
    /// The number of times garbage collection has run.
    pub collection_count: Option<i64>,
    /// The total number of milliseconds spent garbage collecting.
    pub collection_millis: Option<i64>,
    /// The total amount of time spent garbage collecting.
    pub collection_time: Option<String>,
    /// The name of the garbage collector.
    pub name: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryDto {
    /// The actions.
    pub actions: Option<Vec<ActionEntity>>,
    /// The timestamp when the report was generated.
    pub last_refreshed: Option<String>,
    /// The number of number of actions that matched the search criteria..
    pub total: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntity {
    pub history: HistoryDto,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegerParameter {
    pub integer: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JmxMetricsResultDto {
    /// The attribute name of the metrics bean's attribute.
    pub attribute_name: Option<String>,
    /// The attribute value of the the metrics bean's attribute
    pub attribute_value: Option<serde_json::Value>,
    /// The bean name of the metrics bean.
    pub bean_name: Option<String>,
}
/// The labels in this flow snippet.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelDto {
    /// The z index of the label.
    pub getz_index: Option<i64>,
    /// The height of the label in pixels when at a 1:1 scale.
    pub height: Option<f64>,
    /// The id of the component.
    pub id: Option<String>,
    /// The text that appears in the label.
    pub label: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    /// The styles for this label (font-size : 12px, background-color : #eee, etc).
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
    /// The width of the label in pixels when at a 1:1 scale.
    pub width: Option<f64>,
}
/// The labels in this flow.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<LabelDto>,
    pub dimensions: Option<DimensionsDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The z index of the label.
    pub getz_index: Option<i64>,
    /// The id of the component.
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum LineageRequestDtoLineageRequestType {
    #[default]
    #[serde(rename = "PARENTS")]
    Parents,
    #[serde(rename = "CHILDREN")]
    Children,
    #[serde(rename = "FLOWFILE")]
    Flowfile,
}
/// The initial lineage result.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageRequestDto {
    /// The id of the node where this lineage originated if clustered.
    pub cluster_node_id: Option<String>,
    /// The event id that was used to generate this lineage, if applicable.
    /// The event id is allowed for any type of lineageRequestType.
    /// If the lineageRequestType is FLOWFILE and the flowfile uuid is also included in the request, the event id will be ignored.
    pub event_id: Option<i64>,
    /// The type of lineage request. PARENTS will return the lineage for the flowfiles that are parents of the specified event. CHILDREN will return the lineage for the flowfiles that are children of the specified event. FLOWFILE will return the lineage for the specified flowfile.
    pub lineage_request_type: Option<LineageRequestDtoLineageRequestType>,
    /// The flowfile uuid that was used to generate the lineage. The flowfile uuid is only allowed when the lineageRequestType is FLOWFILE and will take precedence over event id.
    pub uuid: Option<String>,
}
/// The results of the lineage query.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageResultsDto {
    /// Any errors that occurred while generating the lineage.
    pub errors: Option<Vec<String>>,
    /// The links between the nodes in the lineage.
    pub links: Option<Vec<ProvenanceLinkDto>>,
    /// The nodes in the lineage.
    pub nodes: Option<Vec<ProvenanceNodeDto>>,
}
/// A list of ingress ports that are currently configured
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenPortDto {
    /// Supported application protocols, if applicable
    pub application_protocols: Option<Vec<String>>,
    /// The class type of the component providing the listen port
    pub component_class: Option<String>,
    /// The id of the component providing the listen port
    pub component_id: Option<String>,
    /// The name of the component providing the listen port
    pub component_name: Option<String>,
    /// The type of component providing the listen port (e.g., Processor, ControllerService)
    pub component_type: Option<String>,
    /// The id of the process group containing the component providing the listen port, if applicable
    pub parent_group_id: Option<String>,
    /// The name of the process group containing the component providing the listen port, if applicable
    pub parent_group_name: Option<String>,
    /// The name of the the listen port. Useful context for components that provide multiple ports.
    pub port_name: Option<String>,
    /// The ingress port number
    pub port_number: Option<i32>,
    /// The ingress transport protocol (TCP or UDP)
    pub transport_protocol: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LongParameter {
    pub long: Option<i64>,
}
/// A list of use cases that have been documented that involve this Processor in conjunction with other Processors
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiProcessorUseCase {
    /// A description of how to configure the Processor to perform the task described in the use case
    pub configurations: Option<Vec<ProcessorConfiguration>>,
    /// A description of the use case
    pub description: Option<String>,
    /// Keywords that pertain to the use csae
    pub keywords: Option<Vec<String>>,
    /// Any pertinent notes about the use case
    pub notes: Option<String>,
}
/// The coordinate of another NAR that the this NAR is dependent on, or null if not dependent on another NAR.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NarCoordinateDto {
    /// The artifact id of the NAR.
    pub artifact: Option<String>,
    /// The group of the NAR.
    pub group: Option<String>,
    /// The version of the NAR.
    pub version: Option<String>,
}
/// A list of status snapshots for each node
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConnectionStatisticsSnapshotDto {
    /// The API address of the node
    pub address: Option<String>,
    /// The API port used to communicate with the node
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    pub node_id: Option<String>,
    pub statistics_snapshot: Option<ConnectionStatisticsSnapshotDto>,
}
/// A list of status snapshots for each node
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConnectionStatusSnapshotDto {
    /// The API address of the node
    pub address: Option<String>,
    /// The API port used to communicate with the node
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    pub node_id: Option<String>,
    pub status_snapshot: Option<ConnectionStatusSnapshotDto>,
}
/// A Counters snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeCountersSnapshotDto {
    /// The API address of the node
    pub address: Option<String>,
    /// The API port used to communicate with the node
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    pub node_id: Option<String>,
    pub snapshot: Option<CountersSnapshotDto>,
}
/// The node's events.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeEventDto {
    /// The category of the node event.
    pub category: Option<String>,
    /// The message in the node event.
    pub message: Option<String>,
    /// The timestamp of the node event.
    pub timestamp: Option<String>,
}
/// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePortStatusSnapshotDto {
    /// The API address of the node
    pub address: Option<String>,
    /// The API port used to communicate with the node
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    pub node_id: Option<String>,
    pub status_snapshot: Option<PortStatusSnapshotDto>,
}
/// The status reported by each node in the cluster. If the NiFi instance is a standalone instance, rather than a clustered instance, this value may be null.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeProcessGroupStatusSnapshotDto {
    /// The API address of the node
    pub address: Option<String>,
    /// The API port used to communicate with the node
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    pub node_id: Option<String>,
    pub status_snapshot: Option<ProcessGroupStatusSnapshotDto>,
}
/// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeProcessorStatusSnapshotDto {
    /// The API address of the node
    pub address: Option<String>,
    /// The API port used to communicate with the node
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    pub node_id: Option<String>,
    pub status_snapshot: Option<ProcessorStatusSnapshotDto>,
}
/// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeRemoteProcessGroupStatusSnapshotDto {
    /// The API address of the node
    pub address: Option<String>,
    /// The API port used to communicate with the node
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    pub node_id: Option<String>,
    pub status_snapshot: Option<RemoteProcessGroupStatusSnapshotDto>,
}
/// The node-wise results
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeReplayLastEventSnapshotDto {
    /// The API address of the node
    pub address: Option<String>,
    /// The API port used to communicate with the node
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    pub node_id: Option<String>,
    pub snapshot: Option<ReplayLastEventSnapshotDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSearchResultDto {
    /// The address of the node that matched the search.
    pub address: Option<String>,
    /// The id of the node that matched the search.
    pub id: Option<String>,
}
/// The NodeStatusSnapshotsDTO objects that provide the actual metric values for the component, for each node. If the NiFi instance is not clustered, this value will be null.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatusSnapshotsDto {
    /// The node's host/ip address.
    pub address: Option<String>,
    /// The port the node is listening for API requests.
    pub api_port: Option<i32>,
    /// The id of the node.
    pub node_id: Option<String>,
    /// A list of StatusSnapshotDTO objects that provide the actual metric values for the component for this node.
    pub status_snapshots: Option<Vec<StatusSnapshotDto>>,
}
/// A systems diagnostics snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSystemDiagnosticsSnapshotDto {
    /// The API address of the node
    pub address: Option<String>,
    /// The API port used to communicate with the node
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    pub node_id: Option<String>,
    pub snapshot: Option<SystemDiagnosticsSnapshotDto>,
}
/// The Parameter Context that is being operated on. This may not be populated until the request has successfully completed.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextDto {
    /// The Process Groups that are bound to this Parameter Context Read-only — set by NiFi.
    pub bound_process_groups: Option<Vec<ProcessGroupEntity>>,
    /// The Description of the Parameter Context.
    pub description: Option<String>,
    /// The ID the Parameter Context. Read-only — set by NiFi.
    pub id: Option<String>,
    /// A list of references of Parameter Contexts from which this one inherits parameters
    pub inherited_parameter_contexts: Option<Vec<ParameterContextReferenceEntity>>,
    /// The Name of the Parameter Context.
    pub name: Option<String>,
    pub parameter_provider_configuration: Option<ParameterProviderConfigurationEntity>,
    /// The Parameters for the Parameter Context
    pub parameters: Option<Vec<ParameterEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextReferenceDto {
    /// The ID of the Parameter Context
    pub id: Option<String>,
    /// The name of the Parameter Context
    pub name: Option<String>,
}
/// The Parameter Context, or null if no Parameter Context has been bound to the Process Group
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextReferenceEntity {
    pub component: Option<ParameterContextReferenceDto>,
    /// The id of the component.
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
}
/// The Parameter Contexts updated by this Parameter Provider. This may not be populated until the request has successfully completed.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextUpdateEntity {
    pub parameter_context: Option<ParameterContextDto>,
    pub parameter_context_revision: Option<RevisionDto>,
    /// The components that are referenced by the update. Read-only — set by NiFi.
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
}
/// The Update Request
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextUpdateRequestDto {
    /// Whether or not the request is completed Read-only — set by NiFi.
    pub complete: Option<bool>,
    /// The reason for the request failing, or null if the request has not failed Read-only — set by NiFi.
    pub failure_reason: Option<String>,
    /// The timestamp of when the request was last updated Read-only — set by NiFi.
    pub last_updated: Option<String>,
    pub parameter_context: Option<ParameterContextDto>,
    /// A value between 0 and 100 (inclusive) indicating how close the request is to completion Read-only — set by NiFi.
    pub percent_completed: Option<i32>,
    /// The components that are referenced by the update. Read-only — set by NiFi.
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
    /// The ID of the request Read-only — set by NiFi.
    pub request_id: Option<String>,
    /// A description of the current state of the request Read-only — set by NiFi.
    pub state: Option<String>,
    /// The timestamp of when the request was submitted Read-only — set by NiFi.
    pub submission_time: Option<String>,
    /// The steps that are required in order to complete the request, along with the status of each Read-only — set by NiFi.
    pub update_steps: Option<Vec<ParameterContextUpdateStepDto>>,
    /// The URI for the request Read-only — set by NiFi.
    pub uri: Option<String>,
}
/// The steps that are required in order to complete the request, along with the status of each
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextUpdateStepDto {
    /// Whether or not this step has completed Read-only — set by NiFi.
    pub complete: Option<bool>,
    /// Explanation of what happens in this step Read-only — set by NiFi.
    pub description: Option<String>,
    /// An explanation of why this step failed, or null if this step did not fail Read-only — set by NiFi.
    pub failure_reason: Option<String>,
}
/// The Update Request
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextValidationRequestDto {
    /// Whether or not the request is completed Read-only — set by NiFi.
    pub complete: Option<bool>,
    pub component_validation_results: Option<ComponentValidationResultsEntity>,
    /// The reason for the request failing, or null if the request has not failed Read-only — set by NiFi.
    pub failure_reason: Option<String>,
    /// The timestamp of when the request was last updated Read-only — set by NiFi.
    pub last_updated: Option<String>,
    pub parameter_context: Option<ParameterContextDto>,
    /// A value between 0 and 100 (inclusive) indicating how close the request is to completion Read-only — set by NiFi.
    pub percent_completed: Option<i32>,
    /// The ID of the request Read-only — set by NiFi.
    pub request_id: Option<String>,
    /// A description of the current state of the request Read-only — set by NiFi.
    pub state: Option<String>,
    /// The timestamp of when the request was submitted Read-only — set by NiFi.
    pub submission_time: Option<String>,
    /// The steps that are required in order to complete the request, along with the status of each Read-only — set by NiFi.
    pub update_steps: Option<Vec<ParameterContextValidationStepDto>>,
    /// The URI for the request Read-only — set by NiFi.
    pub uri: Option<String>,
}
/// The steps that are required in order to complete the request, along with the status of each
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextValidationStepDto {
    /// Whether or not this step has completed Read-only — set by NiFi.
    pub complete: Option<bool>,
    /// Explanation of what happens in this step Read-only — set by NiFi.
    pub description: Option<String>,
    /// An explanation of why this step failed, or null if this step did not fail Read-only — set by NiFi.
    pub failure_reason: Option<String>,
}
/// The parameter information
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDto {
    /// The description of the Parameter
    pub description: Option<String>,
    /// Whether or not the Parameter is inherited from another context Read-only — set by NiFi.
    pub inherited: Option<bool>,
    /// The name of the Parameter
    pub name: Option<String>,
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    /// Whether or not the Parameter is provided by a ParameterProvider
    pub provided: Option<bool>,
    /// A list of identifiers of the assets that are referenced by the parameter
    pub referenced_assets: Option<Vec<AssetReferenceDto>>,
    /// The set of all components in the flow that are referencing this Parameter
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
    /// Whether or not the Parameter is sensitive
    pub sensitive: Option<bool>,
    /// The value of the Parameter
    pub value: Option<String>,
    /// Whether or not the value of the Parameter was removed.
    /// When a request is made to change a parameter, the value may be null.
    /// The absence of the value may be used either to indicate that the value is not to be changed, or that the value is to be set to null (i.e., removed).
    /// This denotes which of the two scenarios is being encountered.
    pub value_removed: Option<bool>,
}
/// The name of the Parameter
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterEntity {
    /// Indicates whether the user can write a given resource. Read-only — set by NiFi.
    pub can_write: Option<bool>,
    pub parameter: Option<ParameterDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ParameterGroupConfigurationEntityParameterSensitivities {
    #[default]
    #[serde(rename = "SENSITIVE")]
    Sensitive,
    #[serde(rename = "NON_SENSITIVE")]
    NonSensitive,
}
/// Configuration for any fetched parameter groups.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterGroupConfigurationEntity {
    /// The name of the external parameter group to which the provided parameter names apply.
    pub group_name: Option<String>,
    /// The name of the ParameterContext that receives the parameters in this group
    pub parameter_context_name: Option<String>,
    /// All fetched parameter names that should be applied.
    pub parameter_sensitivities: Option<
        std::collections::HashMap<
            String,
            Option<ParameterGroupConfigurationEntityParameterSensitivities>,
        >,
    >,
    /// True if this group should be synchronized to a ParameterContext, including creating one if it does not exist.
    pub synchronized: Option<bool>,
}
/// The steps that are required in order to complete the request, along with the status of each
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderApplyParametersUpdateStepDto {
    /// Whether or not this step has completed Read-only — set by NiFi.
    pub complete: Option<bool>,
    /// Explanation of what happens in this step Read-only — set by NiFi.
    pub description: Option<String>,
    /// An explanation of why this step failed, or null if this step did not fail Read-only — set by NiFi.
    pub failure_reason: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderConfigurationDto {
    /// The Parameter Group name that maps to the Parameter Context
    pub parameter_group_name: Option<String>,
    /// The ID of the Parameter Provider
    pub parameter_provider_id: Option<String>,
    /// The name of the Parameter Provider
    pub parameter_provider_name: Option<String>,
    /// True if the Parameter Context should receive the parameters from the mapped Parameter Group
    pub synchronized: Option<bool>,
}
/// Optional configuration for a Parameter Provider
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderConfigurationEntity {
    pub component: Option<ParameterProviderConfigurationDto>,
    /// The id of the component.
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ParameterProviderDtoValidationStatus {
    #[default]
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "VALIDATING")]
    Validating,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderDto {
    /// The set of all components in the flow that are referencing Parameters provided by this provider
    pub affected_components: Option<Vec<AffectedComponentEntity>>,
    /// The annotation data for the parameter provider. This is how the custom UI relays configuration to the parameter provider.
    pub annotation_data: Option<String>,
    pub bundle: Option<BundleDto>,
    /// The comments of the parameter provider.
    pub comments: Option<String>,
    /// The URL for the custom configuration UI for the parameter provider.
    pub custom_ui_url: Option<String>,
    /// Whether the parameter provider has been deprecated.
    pub deprecated: Option<bool>,
    /// The descriptors for the parameter providers properties.
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Whether the underlying extension is missing.
    pub extension_missing: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    /// Whether the parameter provider has multiple versions available.
    pub multiple_versions_available: Option<bool>,
    /// The name of the parameter provider.
    pub name: Option<String>,
    /// Configuration for any fetched parameter groups.
    pub parameter_group_configurations: Option<Vec<ParameterGroupConfigurationEntity>>,
    /// The status of all provided parameters for this parameter provider
    pub parameter_status: Option<Vec<ParameterStatusDto>>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    /// Whether the parameter provider persists state.
    pub persists_state: Option<bool>,
    pub position: Option<PositionDto>,
    /// The properties of the parameter provider.
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The Parameter Contexts that reference this Parameter Provider Read-only — set by NiFi.
    pub referencing_parameter_contexts: Option<Vec<ParameterProviderReferencingComponentEntity>>,
    /// Whether the parameter provider requires elevated privileges.
    pub restricted: Option<bool>,
    /// The fully qualified type of the parameter provider.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Gets the validation errors from the parameter provider. These validation errors represent the problems with the parameter provider that must be resolved before it can be scheduled to run.
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Parameter Provider is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Parameter Provider is valid) Read-only — set by NiFi.
    pub validation_status: Option<ParameterProviderDtoValidationStatus>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ParameterProviderDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReference {
    pub bundle: Option<Bundle>,
    /// The identifier of the parameter provider
    pub identifier: Option<String>,
    /// The name of the parameter provider
    pub name: Option<String>,
    /// The fully qualified name of the parameter provider class.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReferencingComponentDto {
    /// The id of the component referencing a parameter provider.
    pub id: Option<String>,
    /// The name of the component referencing a parameter provider.
    pub name: Option<String>,
}
/// The Parameter Contexts that reference this Parameter Provider
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReferencingComponentEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ParameterProviderReferencingComponentDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ParameterStatusDtoStatus {
    #[default]
    #[serde(rename = "NEW")]
    New,
    #[serde(rename = "CHANGED")]
    Changed,
    #[serde(rename = "REMOVED")]
    Removed,
    #[serde(rename = "MISSING_BUT_REFERENCED")]
    MissingButReferenced,
    #[serde(rename = "UNCHANGED")]
    Unchanged,
}
/// The status of all provided parameters for this parameter provider
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterStatusDto {
    pub parameter: Option<ParameterEntity>,
    /// Indicates the status of the parameter, compared to the existing parameter context
    pub status: Option<ParameterStatusDtoStatus>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerDto {
    /// The number of flowFiles this peer holds.
    pub flow_file_count: Option<i32>,
    /// The hostname of this peer.
    pub hostname: Option<String>,
    /// The port number of this peer.
    pub port: Option<i32>,
    /// Returns if this peer connection is secure.
    pub secure: Option<bool>,
}
/// The permissions for this component.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsDto {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
    /// Indicates whether the user can write a given resource. Read-only — set by NiFi.
    pub can_write: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum PortDtoPortFunction {
    #[default]
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "FAILURE")]
    Failure,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum PortDtoState {
    #[default]
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "DISABLED")]
    Disabled,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum PortDtoType {
    #[default]
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
}
/// The output ports available to received data from the NiFi.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortDto {
    /// Whether this port can be accessed remotely via Site-to-Site protocol.
    pub allow_remote_access: Option<bool>,
    /// The comments for the port.
    pub comments: Option<String>,
    /// The number of tasks that should be concurrently scheduled for the port.
    pub concurrently_schedulable_task_count: Option<i32>,
    /// The id of the component.
    pub id: Option<String>,
    /// The name of the port.
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    /// Specifies how the Port functions
    pub port_function: Option<PortDtoPortFunction>,
    pub position: Option<PositionDto>,
    /// The state of the port.
    pub state: Option<PortDtoState>,
    /// Whether the port has incoming or output connections to a remote NiFi. This is only applicable when the port is allowed to be accessed remotely.
    pub transmitting: Option<bool>,
    /// The type of port.
    #[serde(rename = "type")]
    pub r#type: Option<PortDtoType>,
    /// Gets the validation errors from this port. These validation errors represent the problems with the port that must be resolved before it can be started.
    pub validation_errors: Option<Vec<String>>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
/// The output ports in this flow.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortEntity {
    /// Whether this port can be accessed remotely via Site-to-Site protocol.
    pub allow_remote_access: Option<bool>,
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<PortDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    pub operate_permissions: Option<PermissionsDto>,
    pub permissions: Option<PermissionsDto>,
    pub port_type: Option<String>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub status: Option<PortStatusDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum PortRunStatusEntityState {
    #[default]
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "DISABLED")]
    Disabled,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortRunStatusEntity {
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    pub revision: Option<RevisionDto>,
    /// The run status of the Port.
    pub state: Option<PortRunStatusEntityState>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum PortStatusDtoRunStatus {
    #[default]
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Stopped")]
    Stopped,
    #[serde(rename = "Validating")]
    Validating,
    #[serde(rename = "Disabled")]
    Disabled,
    #[serde(rename = "Invalid")]
    Invalid,
}
/// The status of the port.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatusDto {
    pub aggregate_snapshot: Option<PortStatusSnapshotDto>,
    /// The id of the parent process group of the port.
    pub group_id: Option<String>,
    /// The id of the port.
    pub id: Option<String>,
    /// The name of the port.
    pub name: Option<String>,
    /// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    pub node_snapshots: Option<Vec<NodePortStatusSnapshotDto>>,
    /// The run status of the port.
    pub run_status: Option<PortStatusDtoRunStatus>,
    /// The time the status for the process group was last refreshed.
    pub stats_last_refreshed: Option<String>,
    /// Whether the port has incoming or outgoing connections to a remote NiFi.
    pub transmitting: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum PortStatusSnapshotDtoRunStatus {
    #[default]
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Stopped")]
    Stopped,
    #[serde(rename = "Validating")]
    Validating,
    #[serde(rename = "Disabled")]
    Disabled,
    #[serde(rename = "Invalid")]
    Invalid,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatusSnapshotDto {
    /// The active thread count for the port.
    pub active_thread_count: Option<i32>,
    /// The size of hte FlowFiles that have been accepted in the last 5 minutes.
    pub bytes_in: Option<i64>,
    /// The number of bytes that have been processed in the last 5 minutes.
    pub bytes_out: Option<i64>,
    /// The number of FlowFiles that have been accepted in the last 5 minutes.
    pub flow_files_in: Option<i32>,
    /// The number of FlowFiles that have been processed in the last 5 minutes.
    pub flow_files_out: Option<i32>,
    /// The id of the parent process group of the port.
    pub group_id: Option<String>,
    /// The id of the port.
    pub id: Option<String>,
    /// The count/size of flowfiles that have been accepted in the last 5 minutes.
    pub input: Option<String>,
    /// The name of the port.
    pub name: Option<String>,
    /// The count/size of flowfiles that have been processed in the last 5 minutes.
    pub output: Option<String>,
    /// The run status of the port.
    pub run_status: Option<PortStatusSnapshotDtoRunStatus>,
    /// Whether the port has incoming or outgoing connections to a remote NiFi.
    pub transmitting: Option<bool>,
}
/// The status of all output ports in the process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
    /// The id of the port.
    pub id: Option<String>,
    pub port_status_snapshot: Option<PortStatusSnapshotDto>,
}
/// The position of a component on the graph
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// The x coordinate.
    pub x: Option<f64>,
    /// The y coordinate.
    pub y: Option<f64>,
}
/// The position of this component in the UI if applicable.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionDto {
    /// The x coordinate.
    pub x: Option<f64>,
    /// The y coordinate.
    pub y: Option<f64>,
}
/// Previous values for a given property.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviousValueDto {
    /// The previous value.
    pub previous_value: Option<String>,
    /// The timestamp when the value was modified.
    pub timestamp: Option<String>,
    /// The user who changed the previous value.
    pub user_identity: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessGroupDtoExecutionEngine {
    #[default]
    #[serde(rename = "STATELESS")]
    Stateless,
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "INHERITED")]
    Inherited,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessGroupDtoFlowfileConcurrency {
    #[default]
    #[serde(rename = "UNBOUNDED")]
    Unbounded,
    #[serde(rename = "SINGLE_FLOWFILE_PER_NODE")]
    SingleFlowfilePerNode,
    #[serde(rename = "SINGLE_BATCH_PER_NODE")]
    SingleBatchPerNode,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessGroupDtoFlowfileOutboundPolicy {
    #[default]
    #[serde(rename = "STREAM_WHEN_AVAILABLE")]
    StreamWhenAvailable,
    #[serde(rename = "BATCH_OUTPUT")]
    BatchOutput,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessGroupDtoStatelessGroupScheduledState {
    #[default]
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "RUNNING")]
    Running,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupDto {
    /// The number of active remote ports in the process group.
    pub active_remote_port_count: Option<i32>,
    /// The comments for the process group.
    pub comments: Option<String>,
    pub contents: Option<FlowSnippetDto>,
    /// Default value used in this Process Group for the maximum data size of objects that can be queued before back pressure is applied.
    pub default_back_pressure_data_size_threshold: Option<String>,
    /// Default value used in this Process Group for the maximum number of objects that can be queued before back pressure is applied.
    pub default_back_pressure_object_threshold: Option<i64>,
    /// The default FlowFile Expiration for this Process Group.
    pub default_flow_file_expiration: Option<String>,
    /// The number of disabled components in the process group.
    pub disabled_count: Option<i32>,
    /// The Execution Engine that should be used to run the flow represented by this Process Group.
    pub execution_engine: Option<ProcessGroupDtoExecutionEngine>,
    /// The FlowFile Concurrency for this Process Group.
    pub flowfile_concurrency: Option<ProcessGroupDtoFlowfileConcurrency>,
    /// The Outbound Policy that is used for determining how FlowFiles should be transferred out of the Process Group.
    pub flowfile_outbound_policy: Option<ProcessGroupDtoFlowfileOutboundPolicy>,
    /// The id of the component.
    pub id: Option<String>,
    /// The number of inactive remote ports in the process group.
    pub inactive_remote_port_count: Option<i32>,
    /// The number of input ports in the process group. Read-only — set by NiFi.
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
    /// The log file suffix for this Process Group for dedicated logging.
    pub log_file_suffix: Option<String>,
    /// The maximum number of concurrent tasks to use when running the flow using the Stateless Engine
    pub max_concurrent_tasks: Option<i32>,
    /// The name of the process group.
    pub name: Option<String>,
    /// The number of output ports in the process group. Read-only — set by NiFi.
    pub output_port_count: Option<i32>,
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    /// The number of public input ports in the process group.
    pub public_input_port_count: Option<i32>,
    /// The number of public output ports in the process group.
    pub public_output_port_count: Option<i32>,
    /// The number of running components in this process group.
    pub running_count: Option<i32>,
    /// The number of stale versioned process groups in the process group.
    pub stale_count: Option<i32>,
    /// The maximum amount of time that the flow can be run using the Stateless Engine before the flow times out
    pub stateless_flow_timeout: Option<String>,
    /// If the Process Group is configured to run in using the Stateless Engine, represents the current state. Otherwise, will be STOPPED.
    pub stateless_group_scheduled_state: Option<ProcessGroupDtoStatelessGroupScheduledState>,
    /// The number of stopped components in the process group.
    pub stopped_count: Option<i32>,
    /// The number of versioned process groups in the process group that are unable to sync to a registry.
    pub sync_failure_count: Option<i32>,
    /// The number of up to date versioned process groups in the process group.
    pub up_to_date_count: Option<i32>,
    pub version_control_information: Option<VersionControlInformationDto>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupFlowDto {
    pub breadcrumb: Option<FlowBreadcrumbEntity>,
    pub flow: Option<FlowDto>,
    /// The id of the component.
    pub id: Option<String>,
    /// The time the flow for the process group was last refreshed.
    pub last_refreshed: Option<String>,
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
/// The Process Group that the component belongs to
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupNameDto {
    /// The ID of the Process Group
    pub id: Option<String>,
    /// The name of the Process Group, or the ID of the Process Group if the user does not have the READ policy for the Process Group
    pub name: Option<String>,
}
/// The Process Group Change Request
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupReplaceRequestDto {
    /// Whether or not this request has completed Read-only — set by NiFi.
    pub complete: Option<bool>,
    /// An explanation of why this request failed, or null if this request has not failed Read-only — set by NiFi.
    pub failure_reason: Option<String>,
    /// The last time this request was updated. Read-only — set by NiFi.
    pub last_updated: Option<String>,
    /// The percentage complete for the request, between 0 and 100 Read-only — set by NiFi.
    pub percent_completed: Option<i32>,
    /// The unique ID of the Process Group being updated
    pub process_group_id: Option<String>,
    /// The unique ID of this request. Read-only — set by NiFi.
    pub request_id: Option<String>,
    /// The state of the request Read-only — set by NiFi.
    pub state: Option<String>,
    /// The URI for future requests to this drop request. Read-only — set by NiFi.
    pub uri: Option<String>,
}
/// The status of the process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusDto {
    pub aggregate_snapshot: Option<ProcessGroupStatusSnapshotDto>,
    /// The ID of the Process Group
    pub id: Option<String>,
    /// The name of the Process Group
    pub name: Option<String>,
    /// The status reported by each node in the cluster. If the NiFi instance is a standalone instance, rather than a clustered instance, this value may be null.
    pub node_snapshots: Option<Vec<NodeProcessGroupStatusSnapshotDto>>,
    /// The time the status for the process group was last refreshed.
    pub stats_last_refreshed: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessGroupStatusSnapshotDtoVersionedFlowState {
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
/// The process group status snapshot from the node.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusSnapshotDto {
    /// The active thread count for this process group.
    pub active_thread_count: Option<i32>,
    /// The number of bytes that have come into this ProcessGroup in the last 5 minutes
    pub bytes_in: Option<i64>,
    /// The number of bytes transferred out of this ProcessGroup in the last 5 minutes
    pub bytes_out: Option<i64>,
    /// The number of bytes that are queued up in this ProcessGroup right now
    pub bytes_queued: Option<i64>,
    /// The number of bytes read by components in this ProcessGroup in the last 5 minutes
    pub bytes_read: Option<i64>,
    /// The number of bytes received from external sources by components within this ProcessGroup in the last 5 minutes
    pub bytes_received: Option<i64>,
    /// The number of bytes sent to an external sink by components within this ProcessGroup in the last 5 minutes
    pub bytes_sent: Option<i64>,
    /// The number of bytes transferred in this ProcessGroup in the last 5 minutes
    pub bytes_transferred: Option<i64>,
    /// The number of bytes written by components in this ProcessGroup in the last 5 minutes
    pub bytes_written: Option<i64>,
    /// The status of all connections in the process group.
    pub connection_status_snapshots: Option<Vec<ConnectionStatusSnapshotEntity>>,
    /// The number of FlowFiles that have come into this ProcessGroup in the last 5 minutes
    pub flow_files_in: Option<i32>,
    /// The number of FlowFiles transferred out of this ProcessGroup in the last 5 minutes
    pub flow_files_out: Option<i32>,
    /// The number of FlowFiles that are queued up in this ProcessGroup right now
    pub flow_files_queued: Option<i32>,
    /// The number of FlowFiles received from external sources by components within this ProcessGroup in the last 5 minutes
    pub flow_files_received: Option<i32>,
    /// The number of FlowFiles sent to an external sink by components within this ProcessGroup in the last 5 minutes
    pub flow_files_sent: Option<i32>,
    /// The number of FlowFiles transferred in this ProcessGroup in the last 5 minutes
    pub flow_files_transferred: Option<i32>,
    /// The id of the process group.
    pub id: Option<String>,
    /// The input count/size for the process group in the last 5 minutes (pretty printed).
    pub input: Option<String>,
    /// The status of all input ports in the process group.
    pub input_port_status_snapshots: Option<Vec<PortStatusSnapshotEntity>>,
    /// The name of this process group.
    pub name: Option<String>,
    /// The output count/size for the process group in the last 5 minutes.
    pub output: Option<String>,
    /// The status of all output ports in the process group.
    pub output_port_status_snapshots: Option<Vec<PortStatusSnapshotEntity>>,
    /// The status of all process groups in the process group.
    pub process_group_status_snapshots: Option<Vec<ProcessGroupStatusSnapshotEntity>>,
    pub processing_nanos: Option<i64>,
    pub processing_performance_status: Option<ProcessingPerformanceStatusDto>,
    /// The status of all processors in the process group.
    pub processor_status_snapshots: Option<Vec<ProcessorStatusSnapshotEntity>>,
    /// The count/size that is queued in the the process group.
    pub queued: Option<String>,
    /// The count that is queued for the process group.
    pub queued_count: Option<String>,
    /// The size that is queued for the process group.
    pub queued_size: Option<String>,
    /// The number of bytes read in the last 5 minutes.
    pub read: Option<String>,
    /// The count/size sent to the process group in the last 5 minutes.
    pub received: Option<String>,
    /// The status of all remote process groups in the process group.
    pub remote_process_group_status_snapshots: Option<Vec<RemoteProcessGroupStatusSnapshotEntity>>,
    /// The count/size sent from this process group in the last 5 minutes.
    pub sent: Option<String>,
    /// The current number of active threads for the Process Group, when running in Stateless mode. Read-only — set by NiFi.
    pub stateless_active_thread_count: Option<i32>,
    /// The number of threads currently terminated for the process group.
    pub terminated_thread_count: Option<i32>,
    /// The count/size transferred to/from queues in the process group in the last 5 minutes.
    pub transferred: Option<String>,
    /// The current state of the Process Group, as it relates to the Versioned Flow Read-only — set by NiFi.
    pub versioned_flow_state: Option<ProcessGroupStatusSnapshotDtoVersionedFlowState>,
    /// The number of bytes written in the last 5 minutes.
    pub written: Option<String>,
}
/// The status of all process groups in the process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
    /// The id of the process group.
    pub id: Option<String>,
    pub process_group_status_snapshot: Option<ProcessGroupStatusSnapshotDto>,
}
/// Represents the processor's processing performance.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessingPerformanceStatusDto {
    /// The number of nanoseconds has spent to read content in the last 5 minutes.
    pub content_read_duration: Option<i64>,
    /// The number of nanoseconds has spent to write content in the last 5 minutes.
    pub content_write_duration: Option<i64>,
    /// The number of nanoseconds has spent on CPU usage in the last 5 minutes.
    pub cpu_duration: Option<i64>,
    /// The number of nanoseconds has spent running garbage collection in the last 5 minutes.
    pub garbage_collection_duration: Option<i64>,
    /// The unique ID of the process group that the Processor belongs to
    pub identifier: Option<String>,
    /// The number of nanoseconds has spent running to commit sessions the last 5 minutes.
    pub session_commit_duration: Option<i64>,
}
/// The configuration details for the processor. These details will be included in a response if the verbose flag is included in a request.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorConfigDto {
    /// The annotation data for the processor used to relay configuration between a custom UI and the procesosr.
    pub annotation_data: Option<String>,
    /// The names of all relationships that cause a flow file to be terminated if the relationship is not connected elsewhere. This property differs from the 'isAutoTerminate' property of the RelationshipDTO in that the RelationshipDTO is meant to depict the current configuration, whereas this property can be set in a DTO when updating a Processor in order to change which Relationships should be auto-terminated.
    pub auto_terminated_relationships: Option<Vec<String>>,
    /// Determines whether the FlowFile should be penalized or the processor should be yielded between retries. Possible returned values: PENALIZE_FLOWFILE, YIELD_PROCESSOR. See BackoffMechanism.class for more details.
    pub backoff_mechanism: Option<String>,
    /// The level at which the processor will report bulletins.
    pub bulletin_level: Option<String>,
    /// The comments for the processor.
    pub comments: Option<String>,
    /// The number of tasks that should be concurrently schedule for the processor. If the processor doesn't allow parallol processing then any positive input will be ignored.
    pub concurrently_schedulable_task_count: Option<i32>,
    /// The URL for the processor's custom configuration UI if applicable.
    pub custom_ui_url: Option<String>,
    /// Maps default values for concurrent tasks for each applicable scheduling strategy.
    pub default_concurrent_tasks: Option<std::collections::HashMap<String, Option<String>>>,
    /// Maps default values for scheduling period for each applicable scheduling strategy.
    pub default_scheduling_period: Option<std::collections::HashMap<String, Option<String>>>,
    /// Descriptors for the processor's properties.
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Indicates the node where the process will execute.
    pub execution_node: Option<String>,
    /// Whether the processor is loss tolerant.
    pub loss_tolerant: Option<bool>,
    /// Maximum amount of time to be waited during a retry period.
    pub max_backoff_period: Option<String>,
    /// The amount of time that is used when the process penalizes a flowfile.
    pub penalty_duration: Option<String>,
    /// The properties for the processor. Properties whose value is not set will only contain the property name.
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// All the relationships should be retried.
    pub retried_relationships: Option<Vec<String>>,
    /// Overall number of retries.
    pub retry_count: Option<i32>,
    /// The run duration for the processor in milliseconds.
    pub run_duration_millis: Option<i64>,
    /// The frequency with which to schedule the processor. The format of the value will depend on th value of schedulingStrategy.
    pub scheduling_period: Option<String>,
    /// Indicates how the processor should be scheduled to run.
    pub scheduling_strategy: Option<String>,
    /// Set of sensitive dynamic property names
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// The amount of time that must elapse before this processor is scheduled again after yielding.
    pub yield_duration: Option<String>,
}
/// A description of how to configure the Processor to perform the task described in the use case
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorConfiguration {
    /// A description of how the Processor should be configured in order to accomplish the use case
    pub configuration: Option<String>,
    /// The fully qualified classname of the Processor that should be used to accomplish the use case
    pub processor_class_name: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessorDtoPhysicalState {
    #[default]
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "STOPPING")]
    Stopping,
    #[serde(rename = "RUN_ONCE")]
    RunOnce,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessorDtoState {
    #[default]
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "DISABLED")]
    Disabled,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessorDtoValidationStatus {
    #[default]
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "VALIDATING")]
    Validating,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorDto {
    pub bundle: Option<BundleDto>,
    pub config: Option<ProcessorConfigDto>,
    /// Whether the processor has been deprecated.
    pub deprecated: Option<bool>,
    /// The description of the processor.
    pub description: Option<String>,
    /// Indicates if the execution node of a processor is restricted to run only on the primary node
    pub execution_node_restricted: Option<bool>,
    /// Whether the underlying extension is missing.
    pub extension_missing: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    /// The input requirement for this processor.
    pub input_requirement: Option<String>,
    /// Whether the processor has multiple versions available.
    pub multiple_versions_available: Option<bool>,
    /// The name of the processor.
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    /// Whether the processor persists state.
    pub persists_state: Option<bool>,
    /// The physical state of the processor, including transition states
    pub physical_state: Option<ProcessorDtoPhysicalState>,
    pub position: Option<PositionDto>,
    /// The available relationships that the processor currently supports. Read-only — set by NiFi.
    pub relationships: Option<Vec<RelationshipDto>>,
    /// Whether the processor requires elevated privileges.
    pub restricted: Option<bool>,
    /// The state of the processor
    pub state: Option<ProcessorDtoState>,
    /// Styles for the processor (background-color : #eee).
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether the processor supports batching. This makes the run duration settings available.
    pub supports_batching: Option<bool>,
    /// Whether the processor supports parallel processing.
    pub supports_parallel_processing: Option<bool>,
    /// Whether the processor supports sensitive dynamic properties.
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The type of the processor.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The validation errors for the processor. These validation errors represent the problems with the processor that must be resolved before it can be started.
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Processor is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Processor is valid) Read-only — set by NiFi.
    pub validation_status: Option<ProcessorDtoValidationStatus>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessorEntityPhysicalState {
    #[default]
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "STOPPING")]
    Stopping,
    #[serde(rename = "RUN_ONCE")]
    RunOnce,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ProcessorDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    /// The input requirement for this processor.
    pub input_requirement: Option<String>,
    pub operate_permissions: Option<PermissionsDto>,
    pub permissions: Option<PermissionsDto>,
    /// The physical state of the processor, including transition states
    pub physical_state: Option<ProcessorEntityPhysicalState>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub status: Option<ProcessorStatusDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessorRunStatusDetailsDtoRunStatus {
    #[default]
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Stopped")]
    Stopped,
    #[serde(rename = "Invalid")]
    Invalid,
    #[serde(rename = "Validating")]
    Validating,
    #[serde(rename = "Disabled")]
    Disabled,
}
/// The details of a Processor's run status
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorRunStatusDetailsDto {
    /// The current number of threads that the processor is currently using
    pub active_thread_count: Option<i32>,
    /// The ID of the processor
    pub id: Option<String>,
    /// The name of the processor
    pub name: Option<String>,
    /// The run status of the processor
    pub run_status: Option<ProcessorRunStatusDetailsDtoRunStatus>,
    /// The processor's validation errors
    pub validation_errors: Option<Vec<String>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorRunStatusDetailsEntity {
    pub permissions: Option<PermissionsDto>,
    pub revision: Option<RevisionDto>,
    pub run_status_details: Option<ProcessorRunStatusDetailsDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessorStatusDtoRunStatus {
    #[default]
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Stopped")]
    Stopped,
    #[serde(rename = "Validating")]
    Validating,
    #[serde(rename = "Disabled")]
    Disabled,
    #[serde(rename = "Invalid")]
    Invalid,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorStatusDto {
    pub aggregate_snapshot: Option<ProcessorStatusSnapshotDto>,
    /// The unique ID of the process group that the Processor belongs to
    pub group_id: Option<String>,
    /// The unique ID of the Processor
    pub id: Option<String>,
    /// The name of the Processor
    pub name: Option<String>,
    /// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    pub node_snapshots: Option<Vec<NodeProcessorStatusSnapshotDto>>,
    /// The run status of the Processor
    pub run_status: Option<ProcessorStatusDtoRunStatus>,
    /// The timestamp of when the stats were last refreshed
    pub stats_last_refreshed: Option<String>,
    /// The type of the Processor
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessorStatusSnapshotDtoExecutionNode {
    #[default]
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "PRIMARY")]
    Primary,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProcessorStatusSnapshotDtoRunStatus {
    #[default]
    #[serde(rename = "Running")]
    Running,
    #[serde(rename = "Stopped")]
    Stopped,
    #[serde(rename = "Validating")]
    Validating,
    #[serde(rename = "Disabled")]
    Disabled,
    #[serde(rename = "Invalid")]
    Invalid,
}
/// The processor status snapshot from the node.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorStatusSnapshotDto {
    /// The number of threads currently executing in the processor.
    pub active_thread_count: Option<i32>,
    /// The size of the FlowFiles that have been accepted in the last 5 minutes
    pub bytes_in: Option<i64>,
    /// The size of the FlowFiles transferred to a Connection in the last 5 minutes
    pub bytes_out: Option<i64>,
    /// The number of bytes read by this Processor in the last 5 mintues
    pub bytes_read: Option<i64>,
    /// The number of bytes written by this Processor in the last 5 minutes
    pub bytes_written: Option<i64>,
    /// Indicates the node where the process will execute.
    pub execution_node: Option<ProcessorStatusSnapshotDtoExecutionNode>,
    /// The number of FlowFiles that have been accepted in the last 5 minutes
    pub flow_files_in: Option<i32>,
    /// The number of FlowFiles transferred to a Connection in the last 5 minutes
    pub flow_files_out: Option<i32>,
    /// The id of the parent process group to which the processor belongs.
    pub group_id: Option<String>,
    /// The id of the processor.
    pub id: Option<String>,
    /// The count/size of flowfiles that have been accepted in the last 5 minutes.
    pub input: Option<String>,
    /// The name of the prcessor.
    pub name: Option<String>,
    /// The count/size of flowfiles that have been processed in the last 5 minutes.
    pub output: Option<String>,
    pub processing_performance_status: Option<ProcessingPerformanceStatusDto>,
    /// The number of bytes read in the last 5 minutes.
    pub read: Option<String>,
    /// The state of the processor.
    pub run_status: Option<ProcessorStatusSnapshotDtoRunStatus>,
    /// The number of times this Processor has run in the last 5 minutes
    pub task_count: Option<i32>,
    /// The total number of task this connectable has completed over the last 5 minutes.
    pub tasks: Option<String>,
    /// The total duration of all tasks for this connectable over the last 5 minutes.
    pub tasks_duration: Option<String>,
    /// The number of nanoseconds that this Processor has spent running in the last 5 minutes
    pub tasks_duration_nanos: Option<i64>,
    /// The number of threads currently terminated for the processor.
    pub terminated_thread_count: Option<i32>,
    /// The type of the processor.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The number of bytes written in the last 5 minutes.
    pub written: Option<String>,
}
/// The status of all processors in the process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
    /// The id of the processor.
    pub id: Option<String>,
    pub processor_status_snapshot: Option<ProcessorStatusSnapshotDto>,
}
/// A list of the allowable values for the property
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyAllowableValue {
    /// The description of the value, e.g., the behavior it produces.
    pub description: Option<String>,
    /// The display name of the value, if different from the internal value
    pub display_name: Option<String>,
    /// The internal value
    pub value: Option<String>,
}
/// The dependencies that this property has on other properties
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDependency {
    /// The values that satisfy the dependency
    pub dependent_values: Option<Vec<String>>,
    /// The name of the property that is depended upon
    pub property_display_name: Option<String>,
    /// The name of the property that is depended upon
    pub property_name: Option<String>,
}
/// A list of dependencies that must be met in order for this Property to be relevant. If any of these dependencies is not met, the property described by this Property Descriptor is not relevant.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDependencyDto {
    /// The values for the property that satisfies the dependency, or null if the dependency is satisfied by the presence of any value for the associated property name
    pub dependent_values: Option<Vec<String>>,
    /// The name of the property that is being depended upon
    pub property_name: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum PropertyDescriptorExpressionLanguageScope {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "ENVIRONMENT")]
    Environment,
    #[serde(rename = "FLOWFILE_ATTRIBUTES")]
    FlowfileAttributes,
}
/// Descriptions of configuration properties applicable to this component.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptor {
    /// A list of the allowable values for the property
    pub allowable_values: Option<Vec<PropertyAllowableValue>>,
    /// The default value if a user-set value is not specified
    pub default_value: Option<String>,
    /// The dependencies that this property has on other properties
    pub dependencies: Option<Vec<PropertyDependency>>,
    /// The description of what the property does
    pub description: Option<String>,
    /// The display name of the property key, if different from the name
    pub display_name: Option<String>,
    /// Whether or not the descriptor is for a dynamically added property
    pub dynamic: Option<bool>,
    /// The scope of expression language supported by this property
    pub expression_language_scope: Option<PropertyDescriptorExpressionLanguageScope>,
    /// The description of the expression language scope supported by this property Read-only — set by NiFi.
    pub expression_language_scope_description: Option<String>,
    pub listen_port_definition: Option<PropertyListenPortDefinition>,
    /// The name of the property key
    pub name: Option<String>,
    /// Whether or not  the property is required for the component
    pub required: Option<bool>,
    pub resource_definition: Option<PropertyResourceDefinition>,
    /// Whether or not  the value of the property is considered sensitive (e.g., passwords and keys)
    pub sensitive: Option<bool>,
    pub type_provided_by_value: Option<DefinedType>,
    /// A regular expression that can be used to validate the value of this property
    pub valid_regex: Option<String>,
    /// Name of the validator used for this property descriptor
    pub validator: Option<String>,
}
/// The descriptors for the reporting tasks properties.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptorDto {
    /// Allowable values for the property. If empty then the allowed values are not constrained.
    pub allowable_values: Option<Vec<AllowableValueEntity>>,
    /// The default value for the property.
    pub default_value: Option<String>,
    /// A list of dependencies that must be met in order for this Property to be relevant. If any of these dependencies is not met, the property described by this Property Descriptor is not relevant.
    pub dependencies: Option<Vec<PropertyDependencyDto>>,
    /// The description for the property. Used to relay additional details to a user or provide a mechanism of documenting intent.
    pub description: Option<String>,
    /// The human readable name for the property.
    pub display_name: Option<String>,
    /// Whether the property is dynamic (user-defined).
    pub dynamic: Option<bool>,
    /// Scope of the Expression Language evaluation for the property.
    pub expression_language_scope: Option<String>,
    /// If the property identifies a controller service this returns the fully qualified type.
    pub identifies_controller_service: Option<String>,
    pub identifies_controller_service_bundle: Option<BundleDto>,
    /// The name for the property.
    pub name: Option<String>,
    /// Whether the property is required.
    pub required: Option<bool>,
    /// Whether the property is sensitive and protected whenever stored or represented.
    pub sensitive: Option<bool>,
    /// Whether the property supports expression language.
    pub supports_el: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptorEntity {
    pub property_descriptor: PropertyDescriptorDto,
}
/// The history for the properties of the component.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyHistoryDto {
    /// Previous values for a given property.
    pub previous_values: Option<Vec<PreviousValueDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum PropertyListenPortDefinitionTransportProtocol {
    #[default]
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}
/// Indicates that this property defines a listen port
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyListenPortDefinition {
    /// The application protocols that this listen port could support (if any)
    pub application_protocols: Option<Vec<String>>,
    /// The transport protocol used by this listen port
    pub transport_protocol: Option<PropertyListenPortDefinitionTransportProtocol>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum PropertyResourceDefinitionCardinality {
    #[default]
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "MULTIPLE")]
    Multiple,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum PropertyResourceDefinitionResourceTypes {
    #[default]
    #[serde(rename = "FILE")]
    File,
    #[serde(rename = "DIRECTORY")]
    Directory,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "URL")]
    Url,
}
/// Indicates that this property references external resources
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyResourceDefinition {
    /// The cardinality of the resource definition (i.e. single or multiple)
    pub cardinality: Option<PropertyResourceDefinitionCardinality>,
    /// The types of resources that can be referenced
    pub resource_types: Option<Vec<PropertyResourceDefinitionResourceTypes>>,
}
/// The links between the nodes in the lineage.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceLinkDto {
    /// The flowfile uuid that traversed the link.
    pub flow_file_uuid: Option<String>,
    /// The timestamp of this link in milliseconds.
    pub millis: Option<i64>,
    /// The source node id of the link.
    pub source_id: Option<String>,
    /// The target node id of the link.
    pub target_id: Option<String>,
    /// The timestamp of the link (based on the destination).
    pub timestamp: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ProvenanceNodeDtoType {
    #[default]
    #[serde(rename = "FLOWFILE")]
    Flowfile,
    #[serde(rename = "EVENT")]
    Event,
}
/// The nodes in the lineage.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceNodeDto {
    /// The uuid of the childrent flowfiles of the provenance event.
    pub child_uuids: Option<Vec<String>>,
    /// The identifier of the node that this event/flowfile originated from.
    pub cluster_node_identifier: Option<String>,
    /// If the type is EVENT, this is the type of event.
    pub event_type: Option<String>,
    /// The uuid of the flowfile associated with the provenance event.
    pub flow_file_uuid: Option<String>,
    /// The id of the node.
    pub id: Option<String>,
    /// The timestamp of the node in milliseconds.
    pub millis: Option<i64>,
    /// The uuid of the parent flowfiles of the provenance event.
    pub parent_uuids: Option<Vec<String>>,
    /// The timestamp of the node formatted.
    pub timestamp: Option<String>,
    /// The type of the node.
    #[serde(rename = "type")]
    pub r#type: Option<ProvenanceNodeDtoType>,
}
/// The provenance request.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceRequestDto {
    /// The id of the node in the cluster where this provenance originated.
    pub cluster_node_id: Option<String>,
    /// The latest event time to include in the query.
    pub end_date: Option<String>,
    /// Whether or not incremental results are returned. If false, provenance events are only returned once the query completes. This property is true by default.
    pub incremental_results: Option<bool>,
    /// The maximum number of results to include.
    pub max_results: Option<i32>,
    /// The maximum file size to include in the query.
    pub maximum_file_size: Option<String>,
    /// The minimum file size to include in the query.
    pub minimum_file_size: Option<String>,
    /// The search terms used to perform the search.
    pub search_terms: Option<std::collections::HashMap<String, Option<ProvenanceSearchValueDto>>>,
    /// The earliest event time to include in the query.
    pub start_date: Option<String>,
    /// Whether or not to summarize provenance events returned. This property is false by default.
    pub summarize: Option<bool>,
}
/// The provenance results.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceResultsDto {
    /// Any errors that occurred while performing the provenance request.
    pub errors: Option<Vec<String>>,
    /// Then the search was performed.
    pub generated: Option<String>,
    /// The oldest event available in the provenance repository.
    pub oldest_event: Option<String>,
    /// The provenance events that matched the search criteria.
    pub provenance_events: Option<Vec<ProvenanceEventDto>>,
    /// The time offset of the server that's used for event time.
    pub time_offset: Option<i32>,
    /// The total number of results formatted.
    pub total: Option<String>,
    /// The total number of results.
    pub total_count: Option<i64>,
}
/// The search terms used to perform the search.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceSearchValueDto {
    /// Query for all except for search value.
    pub inverse: Option<bool>,
    /// The search value.
    pub value: Option<String>,
}
/// The available searchable field for the NiFi.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceSearchableFieldDto {
    /// The searchable field.
    pub field: Option<String>,
    /// The id of the searchable field.
    pub id: Option<String>,
    /// The label for the searchable field.
    pub label: Option<String>,
    /// The type of the searchable field.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
/// The size of the queue
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueSizeDto {
    /// The size of objects in a queue.
    pub byte_count: Option<i64>,
    /// The count of objects in a queue.
    pub object_count: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlow {
    pub branch: Option<String>,
    pub bucket_identifier: Option<String>,
    pub bucket_name: Option<String>,
    pub created_timestamp: Option<i64>,
    pub description: Option<String>,
    pub identifier: Option<String>,
    pub last_modified_timestamp: Option<i64>,
    pub name: Option<String>,
    pub permissions: Option<FlowRegistryPermissions>,
    pub version_count: Option<i64>,
    pub version_info: Option<RegisteredFlowVersionInfo>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlowSnapshot {
    pub bucket: Option<FlowRegistryBucket>,
    pub external_controller_services:
        Option<std::collections::HashMap<String, Option<ExternalControllerServiceReference>>>,
    pub flow: Option<RegisteredFlow>,
    pub flow_contents: Option<VersionedProcessGroup>,
    pub flow_encoding_version: Option<String>,
    pub latest: Option<bool>,
    pub parameter_contexts:
        Option<std::collections::HashMap<String, Option<VersionedParameterContext>>>,
    pub parameter_providers:
        Option<std::collections::HashMap<String, Option<ParameterProviderReference>>>,
    pub snapshot_metadata: Option<RegisteredFlowSnapshotMetadata>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlowSnapshotMetadata {
    pub author: Option<String>,
    pub branch: Option<String>,
    pub bucket_identifier: Option<String>,
    pub comments: Option<String>,
    pub flow_identifier: Option<String>,
    pub flow_name: Option<String>,
    pub registry_identifier: Option<String>,
    pub registry_name: Option<String>,
    pub timestamp: Option<i64>,
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlowVersionInfo {
    pub version: Option<i64>,
}
/// The supported relationships for this processor.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    /// The description of the relationship
    pub description: Option<String>,
    /// The name of the relationship
    pub name: Option<String>,
}
/// The available relationships that the processor currently supports.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipDto {
    /// Whether or not flowfiles sent to this relationship should auto terminate.
    pub auto_terminate: Option<bool>,
    /// The relationship description.
    pub description: Option<String>,
    /// The relationship name.
    pub name: Option<String>,
    /// Whether or not flowfiles sent to this relationship should retry.
    pub retry: Option<bool>,
}
/// The contents of the remote process group. Will contain available input/output ports.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupContentsDto {
    /// The input ports to which data can be sent.
    pub input_ports: Option<Vec<RemoteProcessGroupPortDto>>,
    /// The output ports from which data can be retrieved.
    pub output_ports: Option<Vec<RemoteProcessGroupPortDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupDto {
    /// The number of active remote input ports.
    pub active_remote_input_port_count: Option<i32>,
    /// The number of active remote output ports.
    pub active_remote_output_port_count: Option<i32>,
    /// Any remote authorization issues for the remote process group.
    pub authorization_issues: Option<Vec<String>>,
    /// The comments for the remote process group.
    pub comments: Option<String>,
    /// The time period used for the timeout when communicating with the target.
    pub communications_timeout: Option<String>,
    pub contents: Option<RemoteProcessGroupContentsDto>,
    /// The timestamp when this remote process group was last refreshed.
    pub flow_refreshed: Option<String>,
    /// The id of the component.
    pub id: Option<String>,
    /// The number of inactive remote input ports.
    pub inactive_remote_input_port_count: Option<i32>,
    /// The number of inactive remote output ports.
    pub inactive_remote_output_port_count: Option<i32>,
    /// The number of remote input ports currently available on the target.
    pub input_port_count: Option<i32>,
    /// The local network interface to send/receive data. If not specified, any local address is used. If clustered, all nodes must have an interface with this identifier.
    pub local_network_interface: Option<String>,
    /// The name of the remote process group.
    pub name: Option<String>,
    /// The number of remote output ports currently available on the target.
    pub output_port_count: Option<i32>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    pub proxy_host: Option<String>,
    pub proxy_password: Option<String>,
    pub proxy_port: Option<i32>,
    pub proxy_user: Option<String>,
    /// Whether the target is running securely.
    pub target_secure: Option<bool>,
    /// The target URI of the remote process group. If target uri is not set, but uris are set, then returns the first url in the urls. If neither target uri nor uris are set, then returns null.
    pub target_uri: Option<String>,
    /// The target URI of the remote process group. If target uris is not set but target uri is set, then returns a collection containing the single target uri. If neither target uris nor uris are set, then returns null.
    pub target_uris: Option<String>,
    /// Whether the remote process group is actively transmitting.
    pub transmitting: Option<bool>,
    pub transport_protocol: Option<String>,
    /// The validation errors for the remote process group.
    /// These validation errors represent the problems with the remote process group that must be resolved before it can transmit.
    pub validation_errors: Option<Vec<String>>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
    /// When yielding, this amount of time must elapse before the remote process group is scheduled again.
    pub yield_duration: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<RemoteProcessGroupDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    /// The number of remote input ports currently available on the target.
    pub input_port_count: Option<i32>,
    pub operate_permissions: Option<PermissionsDto>,
    /// The number of remote output ports currently available on the target.
    pub output_port_count: Option<i32>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub status: Option<RemoteProcessGroupStatusDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
/// The output ports from which data can be retrieved.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupPortDto {
    pub batch_settings: Option<BatchSettingsDto>,
    /// The comments as configured on the target port.
    pub comments: Option<String>,
    /// The number of task that may transmit flowfiles to the target port concurrently.
    pub concurrently_schedulable_task_count: Option<i32>,
    /// Whether the port has either an incoming or outgoing connection.
    pub connected: Option<bool>,
    /// Whether the target port exists.
    pub exists: Option<bool>,
    /// The id of the remote process group that the port resides in.
    pub group_id: Option<String>,
    /// The id of the port.
    pub id: Option<String>,
    /// The name of the target port.
    pub name: Option<String>,
    /// The id of the target port.
    pub target_id: Option<String>,
    /// Whether the target port is running.
    pub target_running: Option<bool>,
    /// Whether the remote port is configured for transmission.
    pub transmitting: Option<bool>,
    /// Whether the flowfiles are compressed when sent to the target port.
    pub use_compression: Option<bool>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum RemoteProcessGroupStatusDtoValidationStatus {
    #[default]
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "VALIDATING")]
    Validating,
}
/// The status of the remote process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusDto {
    pub aggregate_snapshot: Option<RemoteProcessGroupStatusSnapshotDto>,
    /// The unique ID of the process group that the Processor belongs to
    pub group_id: Option<String>,
    /// The unique ID of the Processor
    pub id: Option<String>,
    /// The name of the remote process group.
    pub name: Option<String>,
    /// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    pub node_snapshots: Option<Vec<NodeRemoteProcessGroupStatusSnapshotDto>>,
    /// The time the status for the process group was last refreshed.
    pub stats_last_refreshed: Option<String>,
    /// The URI of the target system.
    pub target_uri: Option<String>,
    /// The transmission status of the remote process group.
    pub transmission_status: Option<String>,
    /// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid) Read-only — set by NiFi.
    pub validation_status: Option<RemoteProcessGroupStatusDtoValidationStatus>,
}
/// The remote process group status snapshot from the node.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusSnapshotDto {
    /// The number of active threads for the remote process group.
    pub active_thread_count: Option<i32>,
    /// The size of the FlowFiles received from the remote process group in the last 5 minutes.
    pub bytes_received: Option<i64>,
    /// The size of the FlowFiles sent to the remote process group in the last 5 minutes.
    pub bytes_sent: Option<i64>,
    /// The number of FlowFiles received from the remote process group in the last 5 minutes.
    pub flow_files_received: Option<i32>,
    /// The number of FlowFiles sent to the remote process group in the last 5 minutes.
    pub flow_files_sent: Option<i32>,
    /// The id of the parent process group the remote process group resides in.
    pub group_id: Option<String>,
    /// The id of the remote process group.
    pub id: Option<String>,
    /// The name of the remote process group.
    pub name: Option<String>,
    /// The count/size of the flowfiles received from the remote process group in the last 5 minutes.
    pub received: Option<String>,
    /// The count/size of the flowfiles sent to the remote process group in the last 5 minutes.
    pub sent: Option<String>,
    /// The URI of the target system.
    pub target_uri: Option<String>,
    /// The transmission status of the remote process group.
    pub transmission_status: Option<String>,
}
/// The status of all remote process groups in the process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    pub can_read: Option<bool>,
    /// The id of the remote process group.
    pub id: Option<String>,
    pub remote_process_group_status_snapshot: Option<RemoteProcessGroupStatusSnapshotDto>,
}
/// The snapshot from the node
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayLastEventSnapshotDto {
    /// Whether or not an event was available. This may not be populated if there was a failure.
    pub event_available: Option<bool>,
    /// The IDs of the events that were successfully replayed
    pub events_replayed: Option<Vec<i64>>,
    /// If unable to replay an event, specifies why the event could not be replayed
    pub failure_explanation: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ReportingTaskDtoState {
    #[default]
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "DISABLED")]
    Disabled,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ReportingTaskDtoValidationStatus {
    #[default]
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "VALIDATING")]
    Validating,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskDto {
    /// The number of active threads for the reporting task.
    pub active_thread_count: Option<i32>,
    /// The annotation data for the repoting task. This is how the custom UI relays configuration to the reporting task.
    pub annotation_data: Option<String>,
    pub bundle: Option<BundleDto>,
    /// The comments of the reporting task.
    pub comments: Option<String>,
    /// The URL for the custom configuration UI for the reporting task.
    pub custom_ui_url: Option<String>,
    /// The default scheduling period for the different scheduling strategies.
    pub default_scheduling_period: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether the reporting task has been deprecated.
    pub deprecated: Option<bool>,
    /// The descriptors for the reporting tasks properties.
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Whether the underlying extension is missing.
    pub extension_missing: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    /// Whether the reporting task has multiple versions available.
    pub multiple_versions_available: Option<bool>,
    /// The name of the reporting task.
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    /// Whether the reporting task persists state.
    pub persists_state: Option<bool>,
    pub position: Option<PositionDto>,
    /// The properties of the reporting task.
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether the reporting task requires elevated privileges.
    pub restricted: Option<bool>,
    /// The frequency with which to schedule the reporting task. The format of the value will depend on the value of the schedulingStrategy.
    pub scheduling_period: Option<String>,
    /// The scheduling strategy that determines how the schedulingPeriod value should be interpreted.
    pub scheduling_strategy: Option<String>,
    /// Set of sensitive dynamic property names
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// The state of the reporting task.
    pub state: Option<ReportingTaskDtoState>,
    /// Whether the reporting task supports sensitive dynamic properties.
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The fully qualified type of the reporting task.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// Gets the validation errors from the reporting task. These validation errors represent the problems with the reporting task that must be resolved before it can be scheduled to run.
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Reporting Task is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Reporting Task is valid) Read-only — set by NiFi.
    pub validation_status: Option<ReportingTaskDtoValidationStatus>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<ReportingTaskDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    pub operate_permissions: Option<PermissionsDto>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    pub status: Option<ReportingTaskStatusDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ReportingTaskStatusDtoRunStatus {
    #[default]
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "STOPPED")]
    Stopped,
    #[serde(rename = "DISABLED")]
    Disabled,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum ReportingTaskStatusDtoValidationStatus {
    #[default]
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "VALIDATING")]
    Validating,
}
/// The status for this ReportingTask.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskStatusDto {
    /// The number of active threads for the component.
    pub active_thread_count: Option<i32>,
    /// The run status of this ReportingTask Read-only — set by NiFi.
    pub run_status: Option<ReportingTaskStatusDtoRunStatus>,
    /// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid) Read-only — set by NiFi.
    pub validation_status: Option<ReportingTaskStatusDtoValidationStatus>,
}
/// The required permission necessary for this restriction.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequiredPermissionDto {
    /// The required sub-permission necessary for this restriction.
    pub id: Option<String>,
    /// The label for the required sub-permission necessary for this restriction.
    pub label: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimDetailsDto {
    /// Whether or not the Resource Claim is awaiting destruction
    pub awaiting_destruction: Option<bool>,
    /// The number of FlowFiles that have a claim to the Resource
    pub claimant_count: Option<i32>,
    /// The container of the Content Repository in which the Resource Claim exists
    pub container: Option<String>,
    /// The identifier of the Resource Claim
    pub identifier: Option<String>,
    /// Whether or not the Resource Claim is in use
    pub in_use: Option<bool>,
    /// The section of the Content Repository in which the Resource Claim exists
    pub section: Option<String>,
    /// Whether or not the Resource Claim can still have more data written to it
    pub writable: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceDto {
    /// The identifier of the resource.
    pub identifier: Option<String>,
    /// The name of the resource.
    pub name: Option<String>,
}
/// Explicit restrictions that indicate a require permission to use the component
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Restriction {
    /// The explanation of this restriction
    pub explanation: Option<String>,
    /// The permission required for this restriction
    pub required_permission: Option<String>,
}
/// The revision of the Process Group
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionDto {
    /// A client identifier used to make a request.
    /// By including a client identifier, the API can allow multiple requests without needing the current revision.
    /// Due to the asynchronous nature of requests/responses this was implemented to allow the client to make numerous requests without having to wait for the previous response to come back
    pub client_id: Option<String>,
    /// The user that last modified the flow. Read-only — set by NiFi.
    pub last_modifier: Option<String>,
    /// NiFi employs an optimistic locking strategy where the client must include a revision in their request when performing an update.
    /// In a response to a mutable flow request, this field represents the updated base version.
    pub version: Option<i64>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum SchedulingDefaultsDefaultSchedulingStrategy {
    #[default]
    #[serde(rename = "TIMER_DRIVEN")]
    TimerDriven,
    #[serde(rename = "CRON_DRIVEN")]
    CronDriven,
}
/// Scheduling defaults for components defined in this manifest
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchedulingDefaults {
    /// The default concurrent tasks for each scheduling strategy
    pub default_concurrent_tasks_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<i32>>>,
    /// The default concurrent tasks
    pub default_max_concurrent_tasks: Option<String>,
    /// The default run duration in nano-seconds
    pub default_run_duration_nanos: Option<i64>,
    /// The default scheduling period in milliseconds
    pub default_scheduling_period_millis: Option<i64>,
    /// The default scheduling period for each scheduling strategy
    pub default_scheduling_periods_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<String>>>,
    /// The name of the default scheduling strategy
    pub default_scheduling_strategy: Option<SchedulingDefaultsDefaultSchedulingStrategy>,
    /// The default penalization period in milliseconds
    pub penalization_period_millis: Option<i64>,
    /// The default yield duration in milliseconds
    pub yield_duration_millis: Option<i64>,
}
/// The nearest versioned ancestor group of the component that matched the search.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultGroupDto {
    /// The id of the group.
    pub id: String,
    /// The name of the group.
    pub name: Option<String>,
}
/// The snippet.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetDto {
    /// The ids of the connections in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    pub connections: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The ids of the funnels in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    pub funnels: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The id of the snippet.
    pub id: Option<String>,
    /// The ids of the input ports in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    pub input_ports: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The ids of the labels in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    pub labels: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The ids of the output ports in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    pub output_ports: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The group id for the components in the snippet.
    pub parent_group_id: Option<String>,
    /// The ids of the process groups in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    pub process_groups: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The ids of the processors in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    pub processors: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The ids of the remote process groups in this snippet.
    /// These ids will be populated within each response.
    /// They can be specified when creating a snippet.
    /// However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    pub remote_process_groups: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The URI of the snippet.
    pub uri: Option<String>,
}
/// The state.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateEntryDto {
    /// The label for the node where the state originated.
    pub cluster_node_address: Option<String>,
    /// The identifier for the node where the state originated.
    pub cluster_node_id: Option<String>,
    /// The key for this state.
    pub key: Option<String>,
    /// The value for this state.
    pub value: Option<String>,
}
/// The local state for this component.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateMapDto {
    /// The scope of this StateMap.
    pub scope: Option<String>,
    /// The state.
    pub state: Option<Vec<StateEntryDto>>,
    /// The total number of state entries. When the state map is lengthy, only of portion of the entries are returned.
    pub total_entry_count: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum StatefulScopes {
    #[default]
    #[serde(rename = "CLUSTER")]
    Cluster,
    #[serde(rename = "LOCAL")]
    Local,
}
/// Indicates if the component stores state
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Stateful {
    /// Description of what information is being stored in the StateManager
    pub description: Option<String>,
    /// Indicates the Scope(s) associated with the State that is stored and retrieved
    pub scopes: Option<Vec<StatefulScopes>>,
}
/// The Descriptors that provide information on each of the metrics provided in the status history
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusDescriptorDto {
    /// The description of the status field.
    pub description: Option<String>,
    /// The name of the status field.
    pub field: Option<String>,
    /// The formatter for the status descriptor.
    pub formatter: Option<String>,
    /// The label for the status field.
    pub label: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusHistoryDto {
    /// A list of StatusSnapshotDTO objects that provide the actual metric values for the component. If the NiFi instance is clustered, this will represent the aggregate status across all nodes. If the NiFi instance is not clustered, this will represent the status of the entire NiFi instance.
    pub aggregate_snapshots: Option<Vec<StatusSnapshotDto>>,
    /// A Map of key/value pairs that describe the component that the status history belongs to
    pub component_details: Option<std::collections::HashMap<String, Option<String>>>,
    /// The Descriptors that provide information on each of the metrics provided in the status history
    pub field_descriptors: Option<Vec<StatusDescriptorDto>>,
    /// When the status history was generated.
    pub generated: Option<String>,
    /// The NodeStatusSnapshotsDTO objects that provide the actual metric values for the component, for each node. If the NiFi instance is not clustered, this value will be null.
    pub node_snapshots: Option<Vec<NodeStatusSnapshotsDto>>,
}
/// A list of StatusSnapshotDTO objects that provide the actual metric values for the component for this node.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusSnapshotDto {
    /// The status metrics.
    pub status_metrics: Option<std::collections::HashMap<String, Option<i64>>>,
    /// The timestamp of the snapshot.
    pub timestamp: Option<String>,
}
/// The provenance repository storage usage.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageUsageDto {
    /// Amount of free space.
    pub free_space: Option<String>,
    /// The number of bytes of free space.
    pub free_space_bytes: Option<i64>,
    /// The identifier of this storage location. The identifier will correspond to the identifier keyed in the storage configuration.
    pub identifier: Option<String>,
    /// Amount of total space.
    pub total_space: Option<String>,
    /// The number of bytes of total space.
    pub total_space_bytes: Option<i64>,
    /// Amount of used space.
    pub used_space: Option<String>,
    /// The number of bytes of used space.
    pub used_space_bytes: Option<i64>,
    /// Utilization of this storage location.
    pub utilization: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamingOutput {}
/// The mime types this Content Viewer supports.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportedMimeTypesDto {
    /// The display name of the mime types. Read-only — set by NiFi.
    pub display_name: Option<String>,
    /// The mime types this Content Viewer supports. Read-only — set by NiFi.
    pub mime_types: Option<Vec<String>>,
}
/// The System Diagnostics snapshot from the node.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDiagnosticsSnapshotDto {
    /// Number of available processors if supported by the underlying system.
    pub available_processors: Option<i32>,
    /// The content repository storage usage.
    pub content_repository_storage_usage: Option<Vec<StorageUsageDto>>,
    /// Number of daemon threads.
    pub daemon_threads: Option<i32>,
    pub flow_file_repository_storage_usage: Option<StorageUsageDto>,
    /// Amount of free heap.
    pub free_heap: Option<String>,
    /// The number of bytes that are allocated to the JVM heap but not currently being used
    pub free_heap_bytes: Option<i64>,
    /// Amount of free non heap.
    pub free_non_heap: Option<String>,
    /// Total number of free non-heap bytes available to the JVM
    pub free_non_heap_bytes: Option<i64>,
    /// The garbage collection details.
    pub garbage_collection: Option<Vec<GarbageCollectionDto>>,
    /// Utilization of heap.
    pub heap_utilization: Option<String>,
    /// Maximum size of heap.
    pub max_heap: Option<String>,
    /// The maximum number of bytes that can be used by the JVM
    pub max_heap_bytes: Option<i64>,
    /// Maximum size of non heap.
    pub max_non_heap: Option<String>,
    /// The maximum number of bytes that the JVM can use for non-heap purposes
    pub max_non_heap_bytes: Option<i64>,
    /// Utilization of non heap.
    pub non_heap_utilization: Option<String>,
    /// The processor load average if supported by the underlying system.
    pub processor_load_average: Option<f64>,
    /// The provenance repository storage usage.
    pub provenance_repository_storage_usage: Option<Vec<StorageUsageDto>>,
    pub resource_claim_details: Option<Vec<ResourceClaimDetailsDto>>,
    /// When the diagnostics were generated.
    pub stats_last_refreshed: Option<String>,
    /// Total size of heap.
    pub total_heap: Option<String>,
    /// The total number of bytes that are available for the JVM heap to use
    pub total_heap_bytes: Option<i64>,
    /// Total size of non heap.
    pub total_non_heap: Option<String>,
    /// Total number of bytes allocated to the JVM not used for heap
    pub total_non_heap_bytes: Option<i64>,
    /// Total number of threads.
    pub total_threads: Option<i32>,
    /// The uptime of the Java virtual machine
    pub uptime: Option<String>,
    /// Amount of used heap.
    pub used_heap: Option<String>,
    /// The number of bytes of JVM heap that are currently being used
    pub used_heap_bytes: Option<i64>,
    /// Amount of use non heap.
    pub used_non_heap: Option<String>,
    /// Total number of bytes used by the JVM not in the heap space
    pub used_non_heap_bytes: Option<i64>,
    pub version_info: Option<VersionInfoDto>,
}
/// The system resource considerations for the given component
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemResourceConsideration {
    /// The description of how the resource is affected
    pub description: Option<String>,
    /// The resource to consider
    pub resource: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantDto {
    /// Whether this tenant is configurable.
    pub configurable: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    /// The identity of the tenant.
    pub identity: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
/// The set of user group IDs associated with this access policy.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantEntity {
    /// The bulletins for this component.
    pub bulletins: Option<Vec<BulletinEntity>>,
    pub component: Option<TenantDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    pub permissions: Option<PermissionsDto>,
    pub position: Option<PositionDto>,
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum UseCaseInputRequirement {
    #[default]
    #[serde(rename = "INPUT_REQUIRED")]
    InputRequired,
    #[serde(rename = "INPUT_ALLOWED")]
    InputAllowed,
    #[serde(rename = "INPUT_FORBIDDEN")]
    InputForbidden,
}
/// A list of use cases that have been documented for this Processor
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UseCase {
    /// A description of how to configure the Processor to perform the task described in the use case
    pub configuration: Option<String>,
    /// A description of the use case
    pub description: Option<String>,
    /// Specifies whether an incoming FlowFile is expected for this use case
    pub input_requirement: Option<UseCaseInputRequirement>,
    /// Keywords that pertain to the use case
    pub keywords: Option<Vec<String>>,
    /// Any pertinent notes about the use case
    pub notes: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    /// The access policies this user belongs to. Read-only — set by NiFi.
    pub access_policies: Option<Vec<AccessPolicySummaryEntity>>,
    /// Whether this tenant is configurable.
    pub configurable: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    /// The identity of the tenant.
    pub identity: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    /// The groups to which the user belongs. This field is read only and it provided for convenience. Read-only — set by NiFi.
    pub user_groups: Option<Vec<TenantEntity>>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupDto {
    /// The access policies this user group belongs to. This field was incorrectly defined as an AccessPolicyEntity. For compatibility reasons the field will remain of this type, however only the fields that are present in the AccessPolicySummaryEntity will be populated here. Read-only — set by NiFi.
    pub access_policies: Option<Vec<AccessPolicyEntity>>,
    /// Whether this tenant is configurable.
    pub configurable: Option<bool>,
    /// The id of the component.
    pub id: Option<String>,
    /// The identity of the tenant.
    pub identity: Option<String>,
    /// The id of parent process group of this component if applicable.
    pub parent_group_id: Option<String>,
    pub position: Option<PositionDto>,
    /// The users that belong to the user group.
    pub users: Option<Vec<TenantEntity>>,
    /// The ID of the corresponding component that is under version control
    pub versioned_component_id: Option<String>,
}
/// The request
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyConfigRequestDto {
    /// FlowFile Attributes that should be used to evaluate Expression Language for resolving property values
    pub attributes: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether or not the request is completed Read-only — set by NiFi.
    pub complete: Option<bool>,
    /// The ID of the component whose configuration was verified
    pub component_id: Option<String>,
    /// The reason for the request failing, or null if the request has not failed Read-only — set by NiFi.
    pub failure_reason: Option<String>,
    /// The timestamp of when the request was last updated Read-only — set by NiFi.
    pub last_updated: Option<String>,
    /// A value between 0 and 100 (inclusive) indicating how close the request is to completion Read-only — set by NiFi.
    pub percent_completed: Option<i32>,
    /// The configured component properties
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The ID of the request Read-only — set by NiFi.
    pub request_id: Option<String>,
    /// The Results of the verification Read-only — set by NiFi.
    pub results: Option<Vec<ConfigVerificationResultDto>>,
    /// A description of the current state of the request Read-only — set by NiFi.
    pub state: Option<String>,
    /// The timestamp of when the request was submitted Read-only — set by NiFi.
    pub submission_time: Option<String>,
    /// The steps that are required in order to complete the request, along with the status of each Read-only — set by NiFi.
    pub update_steps: Option<Vec<VerifyConfigUpdateStepDto>>,
    /// The URI for the request Read-only — set by NiFi.
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyConfigRequestEntity {
    pub request: VerifyConfigRequestDto,
}
/// The steps that are required in order to complete the request, along with the status of each
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyConfigUpdateStepDto {
    /// Whether or not this step has completed Read-only — set by NiFi.
    pub complete: Option<bool>,
    /// Explanation of what happens in this step Read-only — set by NiFi.
    pub description: Option<String>,
    /// An explanation of why this step failed, or null if this step did not fail Read-only — set by NiFi.
    pub failure_reason: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionControlInformationDtoState {
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
/// The Version Control information
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionControlInformationDto {
    /// The ID of the branch that the flow is stored in
    pub branch: Option<String>,
    /// The ID of the bucket that the flow is stored in
    pub bucket_id: Option<String>,
    /// The name of the bucket that the flow is stored in Read-only — set by NiFi.
    pub bucket_name: Option<String>,
    /// The description of the flow
    pub flow_description: Option<String>,
    /// The ID of the flow
    pub flow_id: Option<String>,
    /// The name of the flow
    pub flow_name: Option<String>,
    /// The ID of the Process Group that is under version control
    pub group_id: Option<String>,
    /// The ID of the registry that the flow is stored in
    pub registry_id: Option<String>,
    /// The name of the registry that the flow is stored in Read-only — set by NiFi.
    pub registry_name: Option<String>,
    /// The current state of the Process Group, as it relates to the Versioned Flow Read-only — set by NiFi.
    pub state: Option<VersionControlInformationDtoState>,
    /// Explanation of why the group is in the specified state Read-only — set by NiFi.
    pub state_explanation: Option<String>,
    /// The storage location
    pub storage_location: Option<String>,
    /// The version of the flow
    pub version: Option<String>,
}
/// The nifi, os, java, and build version information
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfoDto {
    /// Build branch
    pub build_branch: Option<String>,
    /// Build revision or commit hash
    pub build_revision: Option<String>,
    /// Build tag
    pub build_tag: Option<String>,
    /// Build timestamp
    pub build_timestamp: Option<String>,
    /// Java JVM vendor
    pub java_vendor: Option<String>,
    /// Java version
    pub java_version: Option<String>,
    /// The version of this NiFi.
    pub ni_fi_version: Option<String>,
    /// Host operating system architecture
    pub os_architecture: Option<String>,
    /// Host operating system name
    pub os_name: Option<String>,
    /// Host operating system version
    pub os_version: Option<String>,
}
/// The assets that are referenced by this parameter
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedAsset {
    /// The identifier of the asset
    pub identifier: Option<String>,
    /// The name of the asset
    pub name: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedConnectionComponentType {
    #[default]
    #[serde(rename = "CONNECTION")]
    Connection,
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "PROCESS_GROUP")]
    ProcessGroup,
    #[serde(rename = "REMOTE_PROCESS_GROUP")]
    RemoteProcessGroup,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
    #[serde(rename = "LABEL")]
    Label,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "REPORTING_TASK")]
    ReportingTask,
    #[serde(rename = "FLOW_ANALYSIS_RULE")]
    FlowAnalysisRule,
    #[serde(rename = "PARAMETER_CONTEXT")]
    ParameterContext,
    #[serde(rename = "PARAMETER_PROVIDER")]
    ParameterProvider,
    #[serde(rename = "FLOW_REGISTRY_CLIENT")]
    FlowRegistryClient,
}
/// The Connections
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedConnection {
    /// The object data size threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue.
    pub back_pressure_data_size_threshold: Option<String>,
    /// The object count threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue.
    pub back_pressure_object_threshold: Option<i64>,
    /// The bend points on the connection.
    pub bends: Option<Vec<Position>>,
    /// The user-supplied comments for the component
    pub comments: Option<String>,
    pub component_type: Option<VersionedConnectionComponentType>,
    pub destination: Option<ConnectableComponent>,
    /// The amount of time a flow file may be in the flow before it will be automatically aged out of the flow. Once a flow file reaches this age it will be terminated from the flow the next time a processor attempts to start work on it.
    pub flow_file_expiration: Option<String>,
    /// The ID of the Process Group that this component belongs to
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    pub instance_identifier: Option<String>,
    /// The index of the bend point where to place the connection label.
    pub label_index: Option<i32>,
    /// Whether or not compression should be used when transferring FlowFiles between nodes Possible returned values: DO_NOT_COMPRESS, COMPRESS_ATTRIBUTES_ONLY, COMPRESS_ATTRIBUTES_AND_CONTENT. See LoadBalanceCompression.class for more details.
    pub load_balance_compression: Option<String>,
    /// The Strategy to use for load balancing data across the cluster, or null, if no Load Balance Strategy has been specified. Possible returned values: DO_NOT_LOAD_BALANCE, PARTITION_BY_ATTRIBUTE, ROUND_ROBIN, SINGLE_NODE. See LoadBalanceStrategy.class for more details.
    pub load_balance_strategy: Option<String>,
    /// The component's name
    pub name: Option<String>,
    /// The attribute to use for partitioning data as it is load balanced across the cluster. If the Load Balance Strategy is configured to use PARTITION_BY_ATTRIBUTE, the value returned by this method is the name of the FlowFile Attribute that will be used to determine which node in the cluster should receive a given FlowFile. If the Load Balance Strategy is unset or is set to any other value, the Partitioning Attribute has no effect.
    pub partitioning_attribute: Option<String>,
    pub position: Option<Position>,
    /// The comparators used to prioritize the queue.
    pub prioritizers: Option<Vec<String>>,
    /// The selected relationship that comprise the connection.
    pub selected_relationships: Option<Vec<String>>,
    pub source: Option<ConnectableComponent>,
    /// The z index of the connection.
    pub z_index: Option<i64>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedControllerServiceComponentType {
    #[default]
    #[serde(rename = "CONNECTION")]
    Connection,
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "PROCESS_GROUP")]
    ProcessGroup,
    #[serde(rename = "REMOTE_PROCESS_GROUP")]
    RemoteProcessGroup,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
    #[serde(rename = "LABEL")]
    Label,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "REPORTING_TASK")]
    ReportingTask,
    #[serde(rename = "FLOW_ANALYSIS_RULE")]
    FlowAnalysisRule,
    #[serde(rename = "PARAMETER_CONTEXT")]
    ParameterContext,
    #[serde(rename = "PARAMETER_PROVIDER")]
    ParameterProvider,
    #[serde(rename = "FLOW_REGISTRY_CLIENT")]
    FlowRegistryClient,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedControllerServiceScheduledState {
    #[default]
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "RUNNING")]
    Running,
}
/// The Controller Services
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedControllerService {
    /// The annotation for the controller service. This is how the custom UI relays configuration to the controller service.
    pub annotation_data: Option<String>,
    /// The level at which the controller service will report bulletins.
    pub bulletin_level: Option<String>,
    pub bundle: Option<Bundle>,
    /// The user-supplied comments for the component
    pub comments: Option<String>,
    pub component_type: Option<VersionedControllerServiceComponentType>,
    /// Lists the APIs this Controller Service implements.
    pub controller_service_apis: Option<Vec<ControllerServiceAPI>>,
    /// The ID of the Process Group that this component belongs to
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    pub instance_identifier: Option<String>,
    /// The component's name
    pub name: Option<String>,
    pub position: Option<Position>,
    /// The properties for the component. Properties whose value is not set will only contain the property name.
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The property descriptors for the component.
    pub property_descriptors:
        Option<std::collections::HashMap<String, Option<VersionedPropertyDescriptor>>>,
    /// The ScheduledState denoting whether the Controller Service is ENABLED or DISABLED
    pub scheduled_state: Option<VersionedControllerServiceScheduledState>,
    /// The type of the extension component
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
/// The coordinates where the remote flow is stored, or null if the Process Group is not directly under Version Control
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowCoordinates {
    /// The name of the branch that the flow resides in
    pub branch: Option<String>,
    /// The UUID of the bucket that the flow resides in
    pub bucket_id: Option<String>,
    /// The UUID of the flow
    pub flow_id: Option<String>,
    /// Whether or not these coordinates point to the latest version of the flow
    pub latest: Option<bool>,
    /// The identifier of the Flow Registry that contains the flow
    pub registry_id: Option<String>,
    /// The location of the Flow Registry that stores the flow
    pub storage_location: Option<String>,
    /// The version of the flow
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowSnapshotMetadataEntity {
    /// The ID of the Registry that this flow belongs to
    pub registry_id: Option<String>,
    pub versioned_flow_snapshot_metadata: Option<RegisteredFlowSnapshotMetadata>,
}
/// The Flow Update Request
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowUpdateRequestDto {
    /// Whether or not this request has completed Read-only — set by NiFi.
    pub complete: Option<bool>,
    /// An explanation of why this request failed, or null if this request has not failed Read-only — set by NiFi.
    pub failure_reason: Option<String>,
    /// The last time this request was updated. Read-only — set by NiFi.
    pub last_updated: Option<String>,
    /// The percentage complete for the request, between 0 and 100 Read-only — set by NiFi.
    pub percent_completed: Option<i32>,
    /// The unique ID of the Process Group being updated
    pub process_group_id: Option<String>,
    /// The unique ID of this request. Read-only — set by NiFi.
    pub request_id: Option<String>,
    /// The state of the request Read-only — set by NiFi.
    pub state: Option<String>,
    /// The URI for future requests to this drop request. Read-only — set by NiFi.
    pub uri: Option<String>,
    pub version_control_information: Option<VersionControlInformationDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedFunnelComponentType {
    #[default]
    #[serde(rename = "CONNECTION")]
    Connection,
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "PROCESS_GROUP")]
    ProcessGroup,
    #[serde(rename = "REMOTE_PROCESS_GROUP")]
    RemoteProcessGroup,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
    #[serde(rename = "LABEL")]
    Label,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "REPORTING_TASK")]
    ReportingTask,
    #[serde(rename = "FLOW_ANALYSIS_RULE")]
    FlowAnalysisRule,
    #[serde(rename = "PARAMETER_CONTEXT")]
    ParameterContext,
    #[serde(rename = "PARAMETER_PROVIDER")]
    ParameterProvider,
    #[serde(rename = "FLOW_REGISTRY_CLIENT")]
    FlowRegistryClient,
}
/// The Funnels
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFunnel {
    /// The user-supplied comments for the component
    pub comments: Option<String>,
    pub component_type: Option<VersionedFunnelComponentType>,
    /// The ID of the Process Group that this component belongs to
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    pub instance_identifier: Option<String>,
    /// The component's name
    pub name: Option<String>,
    pub position: Option<Position>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedLabelComponentType {
    #[default]
    #[serde(rename = "CONNECTION")]
    Connection,
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "PROCESS_GROUP")]
    ProcessGroup,
    #[serde(rename = "REMOTE_PROCESS_GROUP")]
    RemoteProcessGroup,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
    #[serde(rename = "LABEL")]
    Label,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "REPORTING_TASK")]
    ReportingTask,
    #[serde(rename = "FLOW_ANALYSIS_RULE")]
    FlowAnalysisRule,
    #[serde(rename = "PARAMETER_CONTEXT")]
    ParameterContext,
    #[serde(rename = "PARAMETER_PROVIDER")]
    ParameterProvider,
    #[serde(rename = "FLOW_REGISTRY_CLIENT")]
    FlowRegistryClient,
}
/// The Labels
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedLabel {
    /// The user-supplied comments for the component
    pub comments: Option<String>,
    pub component_type: Option<VersionedLabelComponentType>,
    /// The ID of the Process Group that this component belongs to
    pub group_identifier: Option<String>,
    /// The height of the label in pixels when at a 1:1 scale.
    pub height: Option<f64>,
    /// The component's unique identifier
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    pub instance_identifier: Option<String>,
    /// The text that appears in the label.
    pub label: Option<String>,
    /// The component's name
    pub name: Option<String>,
    pub position: Option<Position>,
    /// The styles for this label (font-size : 12px, background-color : #eee, etc).
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    /// The width of the label in pixels when at a 1:1 scale.
    pub width: Option<f64>,
    /// The z index of the connection.
    pub z_index: Option<i64>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedListenPortDefinitionTransportProtocol {
    #[default]
    #[serde(rename = "TCP")]
    Tcp,
    #[serde(rename = "UDP")]
    Udp,
}
/// Returns the Listen Port Definition for the port this property specifies, if applicable
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedListenPortDefinition {
    /// The application protocol(s) that the listen port could support (if any)
    pub application_protocols: Option<Vec<String>>,
    /// The transport protocol used by the listen port
    pub transport_protocol: Option<VersionedListenPortDefinitionTransportProtocol>,
}
/// The parameters in the context
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedParameter {
    /// The description of the param
    pub description: Option<String>,
    /// The name of the parameter
    pub name: Option<String>,
    /// Whether or not the parameter value is provided by a ParameterProvider
    pub provided: Option<bool>,
    /// The assets that are referenced by this parameter
    pub referenced_assets: Option<Vec<VersionedAsset>>,
    /// Whether or not the parameter value is sensitive
    pub sensitive: Option<bool>,
    /// The value of the parameter
    pub value: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedParameterContextComponentType {
    #[default]
    #[serde(rename = "CONNECTION")]
    Connection,
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "PROCESS_GROUP")]
    ProcessGroup,
    #[serde(rename = "REMOTE_PROCESS_GROUP")]
    RemoteProcessGroup,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
    #[serde(rename = "LABEL")]
    Label,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "REPORTING_TASK")]
    ReportingTask,
    #[serde(rename = "FLOW_ANALYSIS_RULE")]
    FlowAnalysisRule,
    #[serde(rename = "PARAMETER_CONTEXT")]
    ParameterContext,
    #[serde(rename = "PARAMETER_PROVIDER")]
    ParameterProvider,
    #[serde(rename = "FLOW_REGISTRY_CLIENT")]
    FlowRegistryClient,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedParameterContext {
    /// The user-supplied comments for the component
    pub comments: Option<String>,
    pub component_type: Option<VersionedParameterContextComponentType>,
    /// The description of the parameter context
    pub description: Option<String>,
    /// The ID of the Process Group that this component belongs to
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    pub identifier: Option<String>,
    /// The names of additional parameter contexts from which to inherit parameters
    pub inherited_parameter_contexts: Option<Vec<String>>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    pub instance_identifier: Option<String>,
    /// The component's name
    pub name: Option<String>,
    /// The corresponding parameter group name fetched from the parameter provider, if applicable
    pub parameter_group_name: Option<String>,
    /// The identifier of an optional parameter provider
    pub parameter_provider: Option<String>,
    /// The parameters in the context
    pub parameters: Option<Vec<VersionedParameter>>,
    pub position: Option<Position>,
    /// True if the parameter provider is set and the context should receive updates when its parameters are next fetched
    pub synchronized: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedPortComponentType {
    #[default]
    #[serde(rename = "CONNECTION")]
    Connection,
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "PROCESS_GROUP")]
    ProcessGroup,
    #[serde(rename = "REMOTE_PROCESS_GROUP")]
    RemoteProcessGroup,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
    #[serde(rename = "LABEL")]
    Label,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "REPORTING_TASK")]
    ReportingTask,
    #[serde(rename = "FLOW_ANALYSIS_RULE")]
    FlowAnalysisRule,
    #[serde(rename = "PARAMETER_CONTEXT")]
    ParameterContext,
    #[serde(rename = "PARAMETER_PROVIDER")]
    ParameterProvider,
    #[serde(rename = "FLOW_REGISTRY_CLIENT")]
    FlowRegistryClient,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedPortPortFunction {
    #[default]
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "FAILURE")]
    Failure,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedPortScheduledState {
    #[default]
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "RUNNING")]
    Running,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedPortType {
    #[default]
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
}
/// The Output Ports
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedPort {
    /// Whether or not this port allows remote access for site-to-site
    pub allow_remote_access: Option<bool>,
    /// The user-supplied comments for the component
    pub comments: Option<String>,
    pub component_type: Option<VersionedPortComponentType>,
    /// The number of tasks that should be concurrently scheduled for the port.
    pub concurrently_schedulable_task_count: Option<i32>,
    /// The ID of the Process Group that this component belongs to
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    pub instance_identifier: Option<String>,
    /// The component's name
    pub name: Option<String>,
    /// Specifies how the Port should function
    pub port_function: Option<VersionedPortPortFunction>,
    pub position: Option<Position>,
    /// The scheduled state of the component
    pub scheduled_state: Option<VersionedPortScheduledState>,
    /// The type of port.
    #[serde(rename = "type")]
    pub r#type: Option<VersionedPortType>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedProcessGroupComponentType {
    #[default]
    #[serde(rename = "CONNECTION")]
    Connection,
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "PROCESS_GROUP")]
    ProcessGroup,
    #[serde(rename = "REMOTE_PROCESS_GROUP")]
    RemoteProcessGroup,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
    #[serde(rename = "LABEL")]
    Label,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "REPORTING_TASK")]
    ReportingTask,
    #[serde(rename = "FLOW_ANALYSIS_RULE")]
    FlowAnalysisRule,
    #[serde(rename = "PARAMETER_CONTEXT")]
    ParameterContext,
    #[serde(rename = "PARAMETER_PROVIDER")]
    ParameterProvider,
    #[serde(rename = "FLOW_REGISTRY_CLIENT")]
    FlowRegistryClient,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedProcessGroupExecutionEngine {
    #[default]
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "STATELESS")]
    Stateless,
    #[serde(rename = "INHERITED")]
    Inherited,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedProcessGroupScheduledState {
    #[default]
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "RUNNING")]
    Running,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedProcessGroup {
    /// The user-supplied comments for the component
    pub comments: Option<String>,
    pub component_type: Option<VersionedProcessGroupComponentType>,
    /// The Connections
    pub connections: Option<Vec<VersionedConnection>>,
    /// The Controller Services
    pub controller_services: Option<Vec<VersionedControllerService>>,
    /// Default value used in this Process Group for the maximum data size of objects that can be queued before back pressure is applied.
    pub default_back_pressure_data_size_threshold: Option<String>,
    /// Default value used in this Process Group for the maximum number of objects that can be queued before back pressure is applied.
    pub default_back_pressure_object_threshold: Option<i64>,
    /// The default FlowFile Expiration for this Process Group.
    pub default_flow_file_expiration: Option<String>,
    /// The Execution Engine that should be used to run the components within the group.
    pub execution_engine: Option<VersionedProcessGroupExecutionEngine>,
    /// The configured FlowFile Concurrency for the Process Group
    pub flow_file_concurrency: Option<String>,
    /// The FlowFile Outbound Policy for the Process Group
    pub flow_file_outbound_policy: Option<String>,
    /// The Funnels
    pub funnels: Option<Vec<VersionedFunnel>>,
    /// The ID of the Process Group that this component belongs to
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    pub identifier: Option<String>,
    /// The Input Ports
    pub input_ports: Option<Vec<VersionedPort>>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    pub instance_identifier: Option<String>,
    /// The Labels
    pub labels: Option<Vec<VersionedLabel>>,
    /// The log file suffix for this Process Group for dedicated logging.
    pub log_file_suffix: Option<String>,
    /// The maximum number of concurrent tasks that should be scheduled for this Process Group when using the Stateless Engine
    pub max_concurrent_tasks: Option<i32>,
    /// The component's name
    pub name: Option<String>,
    /// The Output Ports
    pub output_ports: Option<Vec<VersionedPort>>,
    /// The name of the parameter context used by this process group
    pub parameter_context_name: Option<String>,
    pub position: Option<Position>,
    /// The child Process Groups
    pub process_groups: Option<Vec<Box<VersionedProcessGroup>>>,
    /// The Processors
    pub processors: Option<Vec<VersionedProcessor>>,
    /// The Remote Process Groups
    pub remote_process_groups: Option<Vec<VersionedRemoteProcessGroup>>,
    /// The Scheduled State of the Process Group, if the group is configured to use the Stateless Execution Engine. Otherwise, this value has no relevance.
    pub scheduled_state: Option<VersionedProcessGroupScheduledState>,
    /// The maximum amount of time that the flow is allows to run using the Stateless engine before it times out and is considered a failure
    pub stateless_flow_timeout: Option<String>,
    pub versioned_flow_coordinates: Option<VersionedFlowCoordinates>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedProcessorComponentType {
    #[default]
    #[serde(rename = "CONNECTION")]
    Connection,
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "PROCESS_GROUP")]
    ProcessGroup,
    #[serde(rename = "REMOTE_PROCESS_GROUP")]
    RemoteProcessGroup,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
    #[serde(rename = "LABEL")]
    Label,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "REPORTING_TASK")]
    ReportingTask,
    #[serde(rename = "FLOW_ANALYSIS_RULE")]
    FlowAnalysisRule,
    #[serde(rename = "PARAMETER_CONTEXT")]
    ParameterContext,
    #[serde(rename = "PARAMETER_PROVIDER")]
    ParameterProvider,
    #[serde(rename = "FLOW_REGISTRY_CLIENT")]
    FlowRegistryClient,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedProcessorScheduledState {
    #[default]
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "RUNNING")]
    Running,
}
/// The Processors
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedProcessor {
    /// The annotation data for the processor used to relay configuration between a custom UI and the procesosr.
    pub annotation_data: Option<String>,
    /// The names of all relationships that cause a flow file to be terminated if the relationship is not connected elsewhere. This property differs from the 'isAutoTerminate' property of the RelationshipDTO in that the RelationshipDTO is meant to depict the current configuration, whereas this property can be set in a DTO when updating a Processor in order to change which Relationships should be auto-terminated.
    pub auto_terminated_relationships: Option<Vec<String>>,
    /// Determines whether the FlowFile should be penalized or the processor should be yielded between retries. Possible returned values: PENALIZE_FLOWFILE, YIELD_PROCESSOR.
    pub backoff_mechanism: Option<String>,
    /// The level at which the processor will report bulletins.
    pub bulletin_level: Option<String>,
    pub bundle: Option<Bundle>,
    /// The user-supplied comments for the component
    pub comments: Option<String>,
    pub component_type: Option<VersionedProcessorComponentType>,
    /// The number of tasks that should be concurrently schedule for the processor. If the processor doesn't allow parallol processing then any positive input will be ignored.
    pub concurrently_schedulable_task_count: Option<i32>,
    /// Indicates the node where the process will execute.
    pub execution_node: Option<String>,
    /// The ID of the Process Group that this component belongs to
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    pub instance_identifier: Option<String>,
    /// Maximum amount of time to be waited during a retry period.
    pub max_backoff_period: Option<String>,
    /// The component's name
    pub name: Option<String>,
    /// The amout of time that is used when the process penalizes a flowfile.
    pub penalty_duration: Option<String>,
    pub position: Option<Position>,
    /// The properties for the component. Properties whose value is not set will only contain the property name.
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The property descriptors for the component.
    pub property_descriptors:
        Option<std::collections::HashMap<String, Option<VersionedPropertyDescriptor>>>,
    /// All the relationships should be retried.
    pub retried_relationships: Option<Vec<String>>,
    /// Overall number of retries.
    pub retry_count: Option<i32>,
    /// The run duration for the processor in milliseconds.
    pub run_duration_millis: Option<i64>,
    /// The scheduled state of the component
    pub scheduled_state: Option<VersionedProcessorScheduledState>,
    /// The frequency with which to schedule the processor. The format of the value will depend on th value of schedulingStrategy.
    pub scheduling_period: Option<String>,
    /// Indicates how the processor should be scheduled to run.
    pub scheduling_strategy: Option<String>,
    /// Stylistic data for rendering in a UI
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    /// The type of the extension component
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// The amount of time that must elapse before this processor is scheduled again after yielding.
    pub yield_duration: Option<String>,
}
/// The property descriptors for the component.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedPropertyDescriptor {
    /// The display name of the property
    pub display_name: Option<String>,
    /// Whether or not the property is user-defined
    pub dynamic: Option<bool>,
    /// Whether or not the property provides the identifier of a Controller Service
    pub identifies_controller_service: Option<bool>,
    pub listen_port_definition: Option<VersionedListenPortDefinition>,
    /// The name of the property
    pub name: Option<String>,
    pub resource_definition: Option<VersionedResourceDefinition>,
    /// Whether or not the property is considered sensitive
    pub sensitive: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedRemoteGroupPortComponentType {
    #[default]
    #[serde(rename = "CONNECTION")]
    Connection,
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "PROCESS_GROUP")]
    ProcessGroup,
    #[serde(rename = "REMOTE_PROCESS_GROUP")]
    RemoteProcessGroup,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
    #[serde(rename = "LABEL")]
    Label,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "REPORTING_TASK")]
    ReportingTask,
    #[serde(rename = "FLOW_ANALYSIS_RULE")]
    FlowAnalysisRule,
    #[serde(rename = "PARAMETER_CONTEXT")]
    ParameterContext,
    #[serde(rename = "PARAMETER_PROVIDER")]
    ParameterProvider,
    #[serde(rename = "FLOW_REGISTRY_CLIENT")]
    FlowRegistryClient,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedRemoteGroupPortScheduledState {
    #[default]
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "RUNNING")]
    Running,
}
/// A Set of Output Ports that can be connected to, in order to pull data from the remote NiFi instance
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedRemoteGroupPort {
    pub batch_size: Option<BatchSize>,
    /// The user-supplied comments for the component
    pub comments: Option<String>,
    pub component_type: Option<VersionedRemoteGroupPortComponentType>,
    /// The number of task that may transmit flowfiles to the target port concurrently.
    pub concurrently_schedulable_task_count: Option<i32>,
    /// The ID of the Process Group that this component belongs to
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    pub instance_identifier: Option<String>,
    /// The component's name
    pub name: Option<String>,
    pub position: Option<Position>,
    /// The id of the remote process group that the port resides in.
    pub remote_group_id: Option<String>,
    /// The scheduled state of the component
    pub scheduled_state: Option<VersionedRemoteGroupPortScheduledState>,
    /// The ID of the port on the target NiFi instance
    pub target_id: Option<String>,
    /// Whether the flowfiles are compressed when sent to the target port.
    pub use_compression: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedRemoteProcessGroupComponentType {
    #[default]
    #[serde(rename = "CONNECTION")]
    Connection,
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "PROCESS_GROUP")]
    ProcessGroup,
    #[serde(rename = "REMOTE_PROCESS_GROUP")]
    RemoteProcessGroup,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
    #[serde(rename = "LABEL")]
    Label,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "REPORTING_TASK")]
    ReportingTask,
    #[serde(rename = "FLOW_ANALYSIS_RULE")]
    FlowAnalysisRule,
    #[serde(rename = "PARAMETER_CONTEXT")]
    ParameterContext,
    #[serde(rename = "PARAMETER_PROVIDER")]
    ParameterProvider,
    #[serde(rename = "FLOW_REGISTRY_CLIENT")]
    FlowRegistryClient,
}
/// The Remote Process Groups
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedRemoteProcessGroup {
    /// The user-supplied comments for the component
    pub comments: Option<String>,
    /// The time period used for the timeout when communicating with the target.
    pub communications_timeout: Option<String>,
    pub component_type: Option<VersionedRemoteProcessGroupComponentType>,
    /// The ID of the Process Group that this component belongs to
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    pub identifier: Option<String>,
    /// A Set of Input Ports that can be connected to, in order to send data to the remote NiFi instance
    pub input_ports: Option<Vec<VersionedRemoteGroupPort>>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    pub instance_identifier: Option<String>,
    /// The local network interface to send/receive data. If not specified, any local address is used. If clustered, all nodes must have an interface with this identifier.
    pub local_network_interface: Option<String>,
    /// The component's name
    pub name: Option<String>,
    /// A Set of Output Ports that can be connected to, in order to pull data from the remote NiFi instance
    pub output_ports: Option<Vec<VersionedRemoteGroupPort>>,
    pub position: Option<Position>,
    pub proxy_host: Option<String>,
    pub proxy_password: Option<String>,
    pub proxy_port: Option<i32>,
    pub proxy_user: Option<String>,
    /// The target URIs of the remote process group. If target uris is not set but target uri is set, then returns the single target uri. If neither target uris nor target uri is set, then returns null.
    pub target_uris: Option<String>,
    /// The Transport Protocol that is used for Site-to-Site communications. Possible returned values: RAW, HTTP.
    pub transport_protocol: Option<String>,
    /// When yielding, this amount of time must elapse before the remote process group is scheduled again.
    pub yield_duration: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedReportingTaskComponentType {
    #[default]
    #[serde(rename = "CONNECTION")]
    Connection,
    #[serde(rename = "PROCESSOR")]
    Processor,
    #[serde(rename = "PROCESS_GROUP")]
    ProcessGroup,
    #[serde(rename = "REMOTE_PROCESS_GROUP")]
    RemoteProcessGroup,
    #[serde(rename = "INPUT_PORT")]
    InputPort,
    #[serde(rename = "OUTPUT_PORT")]
    OutputPort,
    #[serde(rename = "REMOTE_INPUT_PORT")]
    RemoteInputPort,
    #[serde(rename = "REMOTE_OUTPUT_PORT")]
    RemoteOutputPort,
    #[serde(rename = "FUNNEL")]
    Funnel,
    #[serde(rename = "LABEL")]
    Label,
    #[serde(rename = "CONTROLLER_SERVICE")]
    ControllerService,
    #[serde(rename = "REPORTING_TASK")]
    ReportingTask,
    #[serde(rename = "FLOW_ANALYSIS_RULE")]
    FlowAnalysisRule,
    #[serde(rename = "PARAMETER_CONTEXT")]
    ParameterContext,
    #[serde(rename = "PARAMETER_PROVIDER")]
    ParameterProvider,
    #[serde(rename = "FLOW_REGISTRY_CLIENT")]
    FlowRegistryClient,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedReportingTaskScheduledState {
    #[default]
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "RUNNING")]
    Running,
}
/// The reporting tasks
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedReportingTask {
    /// The annotation for the reporting task. This is how the custom UI relays configuration to the reporting task.
    pub annotation_data: Option<String>,
    pub bundle: Option<Bundle>,
    /// The user-supplied comments for the component
    pub comments: Option<String>,
    pub component_type: Option<VersionedReportingTaskComponentType>,
    /// The ID of the Process Group that this component belongs to
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    pub instance_identifier: Option<String>,
    /// The component's name
    pub name: Option<String>,
    pub position: Option<Position>,
    /// The properties for the component. Properties whose value is not set will only contain the property name.
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The property descriptors for the component.
    pub property_descriptors:
        Option<std::collections::HashMap<String, Option<VersionedPropertyDescriptor>>>,
    /// Indicates the scheduled state for the Reporting Task
    pub scheduled_state: Option<VersionedReportingTaskScheduledState>,
    /// The frequency with which to schedule the reporting task. The format of the value will depend on the value of schedulingStrategy.
    pub scheduling_period: Option<String>,
    /// Indicates scheduling strategy that should dictate how the reporting task is triggered.
    pub scheduling_strategy: Option<String>,
    /// The type of the extension component
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedResourceDefinitionCardinality {
    #[default]
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "MULTIPLE")]
    Multiple,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub enum VersionedResourceDefinitionResourceTypes {
    #[default]
    #[serde(rename = "FILE")]
    File,
    #[serde(rename = "DIRECTORY")]
    Directory,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "URL")]
    Url,
}
/// Returns the Resource Definition that defines which type(s) of resource(s) this property references, if any
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedResourceDefinition {
    /// The cardinality of the resource
    pub cardinality: Option<VersionedResourceDefinitionCardinality>,
    /// The types of resource that the Property Descriptor is allowed to reference
    pub resource_types: Option<Vec<VersionedResourceDefinitionResourceTypes>>,
}
