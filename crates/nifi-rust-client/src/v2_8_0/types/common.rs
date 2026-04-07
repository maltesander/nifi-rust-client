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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<AccessPolicyDtoAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_reference: Option<ComponentReferenceEntity>,
    /// Whether this policy is configurable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurable: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The resource for this access policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// The set of user group IDs associated with this access policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_groups: Option<Vec<TenantEntity>>,
    /// The set of user IDs associated with this access policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<TenantEntity>>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<AccessPolicySummaryDtoAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_reference: Option<ComponentReferenceEntity>,
    /// Whether this policy is configurable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurable: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The resource for this access policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}
/// The access policies this user belongs to.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessPolicySummaryEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<AccessPolicySummaryDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_details: Option<ActionDetailsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_details: Option<ComponentDetailsDto>,
    /// The action id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// The operation that was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    /// The id of the source component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The name of the source component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// The type of the source component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// The timestamp of the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The identity of the user that performed the action.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The UUID of this component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The UUID of the Process Group that this component is in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_id: Option<String>,
    /// The type of this component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<AffectedComponentDtoReferenceType>,
    /// The scheduled state of a processor or reporting task referencing a controller service. If this component is another controller service, this field represents the controller service state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The validation errors for the component.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<AffectedComponentDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group: Option<ProcessGroupNameDto>,
    /// The type of component referenced
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<AffectedComponentEntityReferenceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowableValueDto {
    /// A description for this allowable value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A human readable value that is allowed for the property descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// A value that is allowed for the property descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
/// Allowable values for the property. If empty then the allowed values are not constrained.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowableValueEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowable_value: Option<AllowableValueDto>,
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
}
/// A list of identifiers of the assets that are referenced by the parameter
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetReferenceDto {
    /// The identifier of the referenced asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the referenced asset. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// The FlowFile attributes this processor writes/updates
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attribute {
    /// The description of the attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// The attributes of the flowfile for the event.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttributeDto {
    /// The attribute name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The value of the attribute before the event took place.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_value: Option<String>,
    /// The attribute value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
/// The batch settings for data transmission.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSettingsDto {
    /// Preferred number of flow files to include in a transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// Preferred amount of time that a transaction should span.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Preferred number of bytes to include in a transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}
/// The batch settings for data transmission.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSize {
    /// Preferred number of flow files to include in a transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    /// Preferred amount of time that a transaction should span.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Preferred number of bytes to include in a transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}
/// The build metadata for this component
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
    /// The compiler used for the build
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiler: Option<String>,
    /// The compiler flags used for the build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compiler_flags: Option<String>,
    /// The SCM revision id of the source code used for this build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    /// The target architecture of the built component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arch: Option<String>,
    /// The timestamp (milliseconds since Epoch) of the build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// The version number of the built component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinBoardPatternParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_pattern: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletinDto {
    /// The category of this bulletin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The group id of the source component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the bulletin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The level of the bulletin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// The bulletin message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// If clustered, the address of the node from which the bulletin originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_address: Option<String>,
    /// The id of the source component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The name of the source component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// The type of the source component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// The stack trace associated with the bulletin, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<String>,
    /// When this bulletin was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// When this bulletin was generated in ISO format with full date and milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_iso: Option<String>,
}
/// The details of the artifact that bundled this parameter provider.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
    /// The artifact of the bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    /// The group of the bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// The version of the bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
/// If the property identifies a controller service this returns the bundle of the type, null otherwise.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleDto {
    /// The artifact of the bundle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    /// The group of the bundle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// The version of the bundle.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    /// The number of bulletins that were cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins_cleared: Option<i32>,
    /// The id of the component for which bulletins were cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientIdParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The name of the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    /// The type of component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The differences in the component between the two flows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differences: Option<Vec<DifferenceDto>>,
    /// The ID of the Process Group that the component belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentHistoryDto {
    /// The component id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The history for the properties of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_history: Option<std::collections::HashMap<String, Option<PropertyHistoryDto>>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentHistoryEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_history: Option<ComponentHistoryDto>,
}
/// The full specification of the bundle contents
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentManifest {
    /// Public interfaces defined in this bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apis: Option<Vec<DefinedType>>,
    /// Controller Services provided in this bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_services: Option<Vec<ControllerServiceDefinition>>,
    /// Flow Analysis Rules provided in this bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_analysis_rules: Option<Vec<FlowAnalysisRuleDefinition>>,
    /// Flow Registry Clients provided in this bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_registry_clients: Option<Vec<FlowRegistryClientDefinition>>,
    /// Parameter Providers provided in this bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_providers: Option<Vec<ParameterProviderDefinition>>,
    /// Processors provided in this bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<ProcessorDefinition>>,
    /// Reporting Tasks provided in this bundle
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_tasks: Option<Vec<ReportingTaskDefinition>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentReferenceDto {
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}
/// Component this policy references if applicable.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentReferenceEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ComponentReferenceDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
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
/// Permissions for specific component restrictions.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentRestrictionPermissionDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_permission: Option<RequiredPermissionDto>,
}
/// The parameters that matched the search.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentSearchResultDto {
    /// The group id of the component that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the component that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// What matched the search from the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<String>>,
    /// The name of the component that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group: Option<SearchResultGroupDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_group: Option<SearchResultGroupDto>,
}
/// The component state.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStateDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_state: Option<StateMapDto>,
    /// The component identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// Whether dropping state by key is supported for this component. Defaults to false when not specified by the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_state_key_supported: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_state: Option<StateMapDto>,
    /// Description of the state this component persists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_description: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentStateEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_state: Option<ComponentStateDto>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// Whether or not the component is currently valid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currently_valid: Option<bool>,
    /// The UUID of this component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The UUID of the Process Group that this component is in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_id: Option<String>,
    /// The type of this component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<ComponentValidationResultDtoReferenceType>,
    /// The validation errors that will apply to the component if the Parameter Context is changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resultant_validation_errors: Option<Vec<String>>,
    /// Whether or not the component will be valid if the Parameter Context is changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results_valid: Option<bool>,
    /// The scheduled state of a processor or reporting task referencing a controller service. If this component is another controller service, this field represents the controller service state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The validation errors for the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
}
/// A List of ComponentValidationResultEntity, one for each component that is validated
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentValidationResultEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ComponentValidationResultDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
/// The Validation Results that were calculated for each component. This value may not be set until the request completes.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentValidationResultsEntity {
    /// A List of ComponentValidationResultEntity, one for each component that is validated
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// The outcome of the verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<ConfigVerificationResultDtoOutcome>,
    /// The name of the verification step
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_step_name: Option<String>,
}
/// The configuration analysis
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationAnalysisDto {
    /// The ID of the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The configured properties for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The attributes that are referenced by the properties, mapped to recently used values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_attributes: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether or not the component supports verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_verification: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationAnalysisEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_analysis: Option<ConfigurationAnalysisDto>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The id of the group that the connectable component resides in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the connectable component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The name of the connectable component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of component the connectable is.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// If the connectable component represents a remote port, indicates if the target exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    /// The id of the group that the connectable component resides in
    pub group_id: String,
    /// The id of the connectable component.
    pub id: String,
    /// The name of the connectable component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Reflects the current state of the connectable component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    /// If the connectable component represents a remote port, indicates if the target is configured to transmit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitting: Option<bool>,
    /// The type of component the connectable is.
    #[serde(rename = "type")]
    pub r#type: ConnectableDtoType,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}
/// The connections in this flow snippet.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionDto {
    /// The relationships that the source of the connection currently supports. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_relationships: Option<Vec<String>>,
    /// The object data size threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_pressure_data_size_threshold: Option<String>,
    /// The object count threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_pressure_object_threshold: Option<i64>,
    /// The bend points on the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bends: Option<Vec<PositionDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<ConnectableDto>,
    /// The amount of time a flow file may be in the flow before it will be automatically aged out of the flow. Once a flow file reaches this age it will be terminated from the flow the next time a processor attempts to start work on it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_expiration: Option<String>,
    /// The z index of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub getz_index: Option<i64>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The index of the bend point where to place the connection label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_index: Option<i32>,
    /// Whether or not data should be compressed when being transferred between nodes in the cluster. Possible returned values: DO_NOT_COMPRESS, COMPRESS_ATTRIBUTES_ONLY, COMPRESS_ATTRIBUTES_AND_CONTENT. See LoadBalanceCompression.class for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balance_compression: Option<String>,
    /// The FlowFile Attribute to use for determining which node a FlowFile will go to if the Load Balancing Strategy is set to PARTITION_BY_ATTRIBUTE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balance_partition_attribute: Option<String>,
    /// The current status of the Connection's Load Balancing Activities. Status can indicate that Load Balancing is not configured for the connection, that Load Balancing is configured but inactive (not currently transferring data to another node), or that Load Balancing is configured and actively transferring data to another node. Possible returned values: LOAD_BALANCE_NOT_CONFIGURED, LOAD_BALANCE_INACTIVE, LOAD_BALANCE_ACTIVE. See LoadBalanceStatus.class for more details. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balance_status: Option<String>,
    /// How to load balance the data in this Connection across the nodes in the cluster. Possible returned values: DO_NOT_LOAD_BALANCE, PARTITION_BY_ATTRIBUTE, ROUND_ROBIN, SINGLE_NODE. See LoadBalanceStrategy.class for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balance_strategy: Option<String>,
    /// The name of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The comparators used to prioritize the queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prioritizers: Option<Vec<String>>,
    /// The relationships from the source of the connection that are configured to be retried. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retried_relationships: Option<Vec<String>>,
    /// The selected relationship that comprise the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_relationships: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ConnectableDto>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bends: Option<Vec<PositionDto>>,
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ConnectionDto>,
    /// The identifier of the group of the destination of this connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_group_id: Option<String>,
    /// The identifier of the destination of this connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    /// The type of component the destination connectable is.
    pub destination_type: ConnectionEntityDestinationType,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The z index of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub getz_index: Option<i64>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The index of the bend point where to place the connection label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The identifier of the group of the source of this connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_group_id: Option<String>,
    /// The identifier of the source of this connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The type of component the source connectable is.
    pub source_type: ConnectionEntitySourceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ConnectionStatusDto>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatisticsDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<ConnectionStatisticsSnapshotDto>,
    /// The ID of the connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A list of status snapshots for each node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeConnectionStatisticsSnapshotDto>>,
    /// The timestamp of when the stats were last refreshed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_last_refreshed: Option<String>,
}
/// The connection status snapshot from the node.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatisticsSnapshotDto {
    /// The id of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The predicted total number of bytes in the queue at the next configured interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_bytes_at_next_interval: Option<i64>,
    /// The predicted number of queued objects at the next configured interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_count_at_next_interval: Option<i32>,
    /// The predicted number of milliseconds before the connection will have backpressure applied, based on the total number of bytes in the queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_millis_until_bytes_backpressure: Option<i64>,
    /// The predicted number of milliseconds before the connection will have backpressure applied, based on the queued count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_millis_until_count_backpressure: Option<i64>,
    /// The predicted percentage of bytes in the queue against current threshold at the next configured interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_percent_bytes: Option<i32>,
    /// The predicted percentage of queued objects at the next configured interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_percent_count: Option<i32>,
    /// The prediction interval in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_interval_millis: Option<i64>,
}
/// The status of the connection.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<ConnectionStatusSnapshotDto>,
    /// The ID of the destination component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    /// The name of the destination component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_name: Option<String>,
    /// The ID of the Process Group that the connection belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The ID of the connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A list of status snapshots for each node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeConnectionStatusSnapshotDto>>,
    /// The ID of the source component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The name of the source component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    /// The timestamp of when the stats were last refreshed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_last_refreshed: Option<String>,
}
/// Predictions, if available, for this connection (null if not available)
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusPredictionsSnapshotDto {
    /// The predicted total number of bytes in the queue at the next configured interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_bytes_at_next_interval: Option<i64>,
    /// The predicted number of queued objects at the next configured interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_count_at_next_interval: Option<i32>,
    /// The predicted number of milliseconds before the connection will have backpressure applied, based on the total number of bytes in the queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_millis_until_bytes_backpressure: Option<i64>,
    /// The predicted number of milliseconds before the connection will have backpressure applied, based on the queued count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_millis_until_count_backpressure: Option<i64>,
    /// Predicted connection percent use regarding queued flow files size and backpressure threshold if configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_percent_bytes: Option<i32>,
    /// Predicted connection percent use regarding queued flow files count and backpressure threshold if configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicted_percent_count: Option<i32>,
    /// The configured interval (in seconds) for predicting connection queue count and size (and percent usage).
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_in: Option<i64>,
    /// The number of bytes that have left the connection in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_out: Option<i64>,
    /// The size of the FlowFiles that are currently queued in the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_queued: Option<i64>,
    /// The id of the destination of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    /// The name of the destination of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_name: Option<String>,
    /// The availability of FlowFiles in this connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_availability: Option<String>,
    /// The number of FlowFiles that have come into the connection in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_in: Option<i32>,
    /// The number of FlowFiles that have left the connection in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_out: Option<i32>,
    /// The number of FlowFiles that are currently queued in the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_queued: Option<i32>,
    /// The id of the process group the connection belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The input count/size for the connection in the last 5 minutes, pretty printed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// The load balance status of the connection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balance_status: Option<ConnectionStatusSnapshotDtoLoadBalanceStatus>,
    /// The name of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The output count/size for the connection in the last 5 minutes, pretty printed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// Connection percent use regarding queued flow files size and backpressure threshold if configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_use_bytes: Option<i32>,
    /// Connection percent use regarding queued flow files count and backpressure threshold if configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_use_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictions: Option<ConnectionStatusPredictionsSnapshotDto>,
    /// The total count and size of queued flowfiles formatted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<String>,
    /// The number of flowfiles that are queued, pretty printed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_count: Option<String>,
    /// The total size of flowfiles that are queued formatted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_size: Option<String>,
    /// The id of the source of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The name of the source of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
}
/// The status of all connections in the process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status_snapshot: Option<ConnectionStatusSnapshotDto>,
    /// The id of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
/// The Content Viewers.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentViewerDto {
    /// The display name of the Content Viewer. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The mime types this Content Viewer supports. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_mime_types: Option<Vec<SupportedMimeTypesDto>>,
    /// The uri of the Content Viewer. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
/// The controller configuration.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerConfigurationDto {
    /// The maximum number of timer driven threads the NiFi has available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_timer_driven_thread_count: Option<i32>,
}
/// Lists the APIs this Controller Service implements.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceAPI {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Bundle>,
    /// The fully qualified name of the service interface.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
/// Lists the APIs this Controller Service implements.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceApiDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<BundleDto>,
    /// The fully qualified name of the service interface.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    /// The level at which the controller service will report bulletins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletin_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<BundleDto>,
    /// The comments for the controller service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Lists the APIs this Controller Service implements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_apis: Option<Vec<ControllerServiceApiDto>>,
    /// The URL for the controller services custom configuration UI if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ui_url: Option<String>,
    /// Whether the ontroller service has been deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// The descriptors for the controller service properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Whether the underlying extension is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_missing: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the controller service has multiple versions available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_versions_available: Option<bool>,
    /// The name of the controller service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// Whether the controller service persists state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persists_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The properties of the controller service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// All components referencing this controller service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_components: Option<Vec<ControllerServiceReferencingComponentEntity>>,
    /// Whether the controller service requires elevated privileges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// Set of sensitive dynamic property names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// The state of the controller service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ControllerServiceDtoState>,
    /// Whether the controller service supports sensitive dynamic properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The type of the controller service.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The validation errors from the controller service.
    /// These validation errors represent the problems with the controller service that must be resolved before it can be enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the ControllerService is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the ControllerService is valid) Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<ControllerServiceDtoValidationStatus>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ControllerServiceDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_permissions: Option<PermissionsDto>,
    /// The id of parent process group of this ControllerService.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ControllerServiceStatusDto>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The descriptors for the component properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// The group id for the component referencing a controller service. If this component is another controller service or a reporting task, this field is blank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the component referencing a controller service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the component referencing a controller service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The properties for the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// If the referencing component represents a controller service, this indicates whether it has already been represented in this hierarchy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_cycle: Option<bool>,
    /// The type of reference this is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_type: Option<ControllerServiceReferencingComponentDtoReferenceType>,
    /// If the referencing component represents a controller service, these are the components that reference it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_components: Option<Vec<ControllerServiceReferencingComponentEntity>>,
    /// The scheduled state of a processor or reporting task referencing a controller service. If this component is another controller service, this field represents the controller service state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The type of the component referencing a controller service in simple Java class name format without package name.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The validation errors for the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
}
/// All components referencing this controller service.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerServiceReferencingComponentEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ControllerServiceReferencingComponentDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The run status of this ControllerService Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<ControllerServiceStatusDtoRunStatus>,
    /// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid) Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<ControllerServiceStatusDtoValidationStatus>,
}
/// The counters from the node.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountersSnapshotDto {
    /// All counters in the NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counters: Option<Vec<CounterDto>>,
    /// The timestamp when the report was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateTimeParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time: Option<String>,
}
/// Indicates that this property is for selecting a controller service of the specified type
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DefinedType {
    /// The artifact name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    /// The group name of the bundle that provides the referenced type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
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
/// The differences in the component between the two flows
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DifferenceDto {
    /// Description of the difference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difference: Option<String>,
    /// The type of difference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difference_type: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DimensionsDto {
    /// The height of the label in pixels when at a 1:1 scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// The width of the label in pixels when at a 1:1 scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentedTypeDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<BundleDto>,
    /// If this type represents a ControllerService, this lists the APIs it implements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_apis: Option<Vec<ControllerServiceApiDto>>,
    /// The description of why the usage of this component is restricted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_reason: Option<String>,
    /// The description of the type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An optional collection of explicit restrictions. If specified, these explicit restrictions will be enfored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_restrictions: Option<Vec<ExplicitRestrictionDto>>,
    /// Whether this type is restricted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// The tags associated with this type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The fully qualified name of the type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The optional description of why the usage of this component is restricted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_restriction: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropRequestDto {
    /// The count and size of flow files currently queued.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<String>,
    /// The number of flow files currently queued.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_count: Option<i32>,
    /// The size of flow files currently queued in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_size: Option<i64>,
    /// The count and size of flow files that have been dropped thus far.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropped: Option<String>,
    /// The number of flow files that have been dropped thus far.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropped_count: Option<i32>,
    /// The size of flow files that have been dropped thus far in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropped_size: Option<i64>,
    /// The reason, if any, that this drop request failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// Whether the query has finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    /// The id for this drop request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The last time this drop request was updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// The count and size of flow files to be dropped as a result of this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<String>,
    /// The number of flow files to be dropped as a result of this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_count: Option<i32>,
    /// The size of flow files to be dropped as a result of this request in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_size: Option<i64>,
    /// The current percent complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// The current state of the drop request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The timestamp when the query was submitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The URI for future requests to this drop request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropRequestEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_request: Option<DropRequestDto>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The scope of the expression language support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_language_scope: Option<DynamicPropertyExpressionLanguageScope>,
    /// The description of the dynamic property name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the dynamic property value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
/// If the processor supports dynamic relationships, this describes the dynamic relationship
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicRelationship {
    /// The description of the dynamic relationship
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The description of the dynamic relationship name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// An optional collection of explicit restrictions. If specified, these explicit restrictions will be enfored.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplicitRestrictionDto {
    /// The description of why the usage of this component is restricted for this required permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_permission: Option<RequiredPermissionDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalControllerServiceReference {
    /// The identifier of the controller service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The name of the controller service
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<BundleDto>,
    /// The comments of the flow analysis rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Whether the flow analysis rule has been deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// The descriptors for the flow analysis rules properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Enforcement Policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement_policy: Option<String>,
    /// Whether the underlying extension is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_missing: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the flow analysis rule has multiple versions available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_versions_available: Option<bool>,
    /// The name of the flow analysis rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// Whether the flow analysis rule persists state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persists_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The properties of the flow analysis rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether the flow analysis rule requires elevated privileges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// Set of sensitive dynamic property names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// The state of the flow analysis rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<FlowAnalysisRuleDtoState>,
    /// Whether the flow analysis rule supports sensitive dynamic properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The fully qualified type of the flow analysis rule.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Gets the validation errors from the flow analysis rule. These validation errors represent the problems with the flow analysis rule that must be resolved before it can be scheduled to run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Flow Analysis Rule is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Flow Analysis Rule is valid) Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<FlowAnalysisRuleDtoValidationStatus>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The run status of this FlowAnalysisRule Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<FlowAnalysisRuleStatusDtoRunStatus>,
    /// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid) Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<FlowAnalysisRuleStatusDtoValidationStatus>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowAnalysisRuleViolationDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement_policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_component_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_permission_dto: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violation_message: Option<String>,
}
/// This breadcrumb.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowBreadcrumbDto {
    /// The id of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The id of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_control_information: Option<VersionControlInformationDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowComparisonEntity {
    /// The list of differences for each component in the flow that is not the same between the two flows
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_differences: Option<Vec<ComponentDifferenceDto>>,
}
/// The FlowFile summaries. The summaries will be populated once the request has completed.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowFileSummaryDto {
    /// The label for the node where this FlowFile resides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_address: Option<String>,
    /// The id of the node where this FlowFile resides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The FlowFile filename.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Duration since the FlowFile's greatest ancestor entered the flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_duration: Option<i64>,
    /// The FlowFile mime type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// If the FlowFile is penalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalized: Option<bool>,
    /// How long in milliseconds until the FlowFile penalty expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalty_expires_in: Option<i64>,
    /// The FlowFile's position in the queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    /// How long this FlowFile has been enqueued.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_duration: Option<i64>,
    /// The FlowFile file size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// The URI that can be used to access this FlowFile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// The FlowFile UUID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBranchDto {
    /// The branch name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBranchEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<FlowRegistryBranchDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBucket {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FlowRegistryPermissions>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBucketDto {
    /// The created timestamp of this bucket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    /// The bucket description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The bucket identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The bucket name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryBucketEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<FlowRegistryBucketDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<BundleDto>,
    /// Whether the registry client has been deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// The registry description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The descriptors for the registry client properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Whether the underlying extension is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_missing: Option<bool>,
    /// The registry identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the flow registry client has multiple versions available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_versions_available: Option<bool>,
    /// The registry name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The properties of the registry client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether the registry client requires elevated privileges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// Set of sensitive dynamic property names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// Whether the registry client supports branching.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_branching: Option<bool>,
    /// Whether the registry client supports sensitive dynamic properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The type of the registry client.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Gets the validation errors from the registry client. These validation errors represent the problems with the registry client that must be resolved before it can be used for interacting with the flow registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Registry Client is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Registry Client is valid) Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<FlowRegistryClientDtoValidationStatus>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryClientsEntity {
    /// The current time on the system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registries: Option<Vec<FlowRegistryClientEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowRegistryPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_write: Option<bool>,
}
/// The contents of this process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowSnippetDto {
    /// The connections in this flow snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<ConnectionDto>>,
    /// The controller services in this flow snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_services: Option<Vec<ControllerServiceDto>>,
    /// The funnels in this flow snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnels: Option<Vec<FunnelDto>>,
    /// The input ports in this flow snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<PortDto>>,
    /// The labels in this flow snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<LabelDto>>,
    /// The output ports in this flow snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<PortDto>>,
    /// The process groups in this flow snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<Vec<ProcessGroupDto>>,
    /// The processors in this flow snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<ProcessorDto>>,
    /// The remote process groups in this flow snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<Vec<RemoteProcessGroupDto>>,
}
/// The funnels in this flow snippet.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunnelDto {
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}
/// The funnels in this flow.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunnelEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<FunnelDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
/// The garbage collection details.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GarbageCollectionDto {
    /// The number of times garbage collection has run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_count: Option<i64>,
    /// The total number of milliseconds spent garbage collecting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_millis: Option<i64>,
    /// The total amount of time spent garbage collecting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_time: Option<String>,
    /// The name of the garbage collector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryDto {
    /// The actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<ActionEntity>>,
    /// The timestamp when the report was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refreshed: Option<String>,
    /// The number of number of actions that matched the search criteria..
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<HistoryDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegerParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JmxMetricsResultDto {
    /// The attribute name of the metrics bean's attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// The attribute value of the the metrics bean's attribute
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<serde_json::Value>,
    /// The bean name of the metrics bean.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bean_name: Option<String>,
}
/// The labels in this flow snippet.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelDto {
    /// The z index of the label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub getz_index: Option<i64>,
    /// The height of the label in pixels when at a 1:1 scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The text that appears in the label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The styles for this label (font-size : 12px, background-color : #eee, etc).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
    /// The width of the label in pixels when at a 1:1 scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
}
/// The labels in this flow.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LabelEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<LabelDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<DimensionsDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The z index of the label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub getz_index: Option<i64>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The event id that was used to generate this lineage, if applicable.
    /// The event id is allowed for any type of lineageRequestType.
    /// If the lineageRequestType is FLOWFILE and the flowfile uuid is also included in the request, the event id will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<i64>,
    /// The type of lineage request. PARENTS will return the lineage for the flowfiles that are parents of the specified event. CHILDREN will return the lineage for the flowfiles that are children of the specified event. FLOWFILE will return the lineage for the specified flowfile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_request_type: Option<LineageRequestDtoLineageRequestType>,
    /// The flowfile uuid that was used to generate the lineage. The flowfile uuid is only allowed when the lineageRequestType is FLOWFILE and will take precedence over event id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
/// The results of the lineage query.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LineageResultsDto {
    /// Any errors that occurred while generating the lineage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// The links between the nodes in the lineage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ProvenanceLinkDto>>,
    /// The nodes in the lineage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<ProvenanceNodeDto>>,
}
/// A list of ingress ports that are currently configured
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenPortDto {
    /// Supported application protocols, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_protocols: Option<Vec<String>>,
    /// The class type of the component providing the listen port
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_class: Option<String>,
    /// The id of the component providing the listen port
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The name of the component providing the listen port
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    /// The type of component providing the listen port (e.g., Processor, ControllerService)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// The id of the process group containing the component providing the listen port, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// The name of the process group containing the component providing the listen port, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_name: Option<String>,
    /// The name of the the listen port. Useful context for components that provide multiple ports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_name: Option<String>,
    /// The ingress port number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_number: Option<i32>,
    /// The ingress transport protocol (TCP or UDP)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LongParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long: Option<i64>,
}
/// A list of use cases that have been documented that involve this Processor in conjunction with other Processors
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiProcessorUseCase {
    /// A description of how to configure the Processor to perform the task described in the use case
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<ProcessorConfiguration>>,
    /// A description of the use case
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Keywords that pertain to the use csae
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// Any pertinent notes about the use case
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
/// The coordinate of another NAR that the this NAR is dependent on, or null if not dependent on another NAR.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NarCoordinateDto {
    /// The artifact id of the NAR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<String>,
    /// The group of the NAR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// The version of the NAR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
/// A list of status snapshots for each node
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConnectionStatisticsSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics_snapshot: Option<ConnectionStatisticsSnapshotDto>,
}
/// A list of status snapshots for each node
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConnectionStatusSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_snapshot: Option<ConnectionStatusSnapshotDto>,
}
/// A Counters snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeCountersSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<CountersSnapshotDto>,
}
/// The node's events.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeEventDto {
    /// The category of the node event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The message in the node event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The timestamp of the node event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}
/// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodePortStatusSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_snapshot: Option<PortStatusSnapshotDto>,
}
/// The status reported by each node in the cluster. If the NiFi instance is a standalone instance, rather than a clustered instance, this value may be null.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeProcessGroupStatusSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_snapshot: Option<ProcessGroupStatusSnapshotDto>,
}
/// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeProcessorStatusSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_snapshot: Option<ProcessorStatusSnapshotDto>,
}
/// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeRemoteProcessGroupStatusSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_snapshot: Option<RemoteProcessGroupStatusSnapshotDto>,
}
/// The node-wise results
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeReplayLastEventSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<ReplayLastEventSnapshotDto>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSearchResultDto {
    /// The address of the node that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The id of the node that matched the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
/// The NodeStatusSnapshotsDTO objects that provide the actual metric values for the component, for each node. If the NiFi instance is not clustered, this value will be null.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatusSnapshotsDto {
    /// The node's host/ip address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The port the node is listening for API requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The id of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// A list of StatusSnapshotDTO objects that provide the actual metric values for the component for this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_snapshots: Option<Vec<StatusSnapshotDto>>,
}
/// A systems diagnostics snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSystemDiagnosticsSnapshotDto {
    /// The API address of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The API port used to communicate with the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_port: Option<i32>,
    /// The unique ID that identifies the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<SystemDiagnosticsSnapshotDto>,
}
/// The Parameter Context that is being operated on. This may not be populated until the request has successfully completed.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextDto {
    /// The Process Groups that are bound to this Parameter Context Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bound_process_groups: Option<Vec<ProcessGroupEntity>>,
    /// The Description of the Parameter Context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID the Parameter Context. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A list of references of Parameter Contexts from which this one inherits parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_parameter_contexts: Option<Vec<ParameterContextReferenceEntity>>,
    /// The Name of the Parameter Context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_configuration: Option<ParameterProviderConfigurationEntity>,
    /// The Parameters for the Parameter Context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterEntity>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextReferenceDto {
    /// The ID of the Parameter Context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the Parameter Context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// The Parameter Context, or null if no Parameter Context has been bound to the Process Group
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextReferenceEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ParameterContextReferenceDto>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
}
/// The Parameter Contexts updated by this Parameter Provider. This may not be populated until the request has successfully completed.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextUpdateEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<ParameterContextDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context_revision: Option<RevisionDto>,
    /// The components that are referenced by the update. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
}
/// The Update Request
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextUpdateRequestDto {
    /// Whether or not the request is completed Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// The reason for the request failing, or null if the request has not failed Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// The timestamp of when the request was last updated Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<ParameterContextDto>,
    /// A value between 0 and 100 (inclusive) indicating how close the request is to completion Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// The components that are referenced by the update. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
    /// The ID of the request Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// A description of the current state of the request Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The timestamp of when the request was submitted Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The steps that are required in order to complete the request, along with the status of each Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_steps: Option<Vec<ParameterContextUpdateStepDto>>,
    /// The URI for the request Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
/// The steps that are required in order to complete the request, along with the status of each
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextUpdateStepDto {
    /// Whether or not this step has completed Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// Explanation of what happens in this step Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An explanation of why this step failed, or null if this step did not fail Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}
/// The Update Request
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextValidationRequestDto {
    /// Whether or not the request is completed Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_validation_results: Option<ComponentValidationResultsEntity>,
    /// The reason for the request failing, or null if the request has not failed Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// The timestamp of when the request was last updated Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<ParameterContextDto>,
    /// A value between 0 and 100 (inclusive) indicating how close the request is to completion Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// The ID of the request Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// A description of the current state of the request Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The timestamp of when the request was submitted Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The steps that are required in order to complete the request, along with the status of each Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_steps: Option<Vec<ParameterContextValidationStepDto>>,
    /// The URI for the request Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
/// The steps that are required in order to complete the request, along with the status of each
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterContextValidationStepDto {
    /// Whether or not this step has completed Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// Explanation of what happens in this step Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An explanation of why this step failed, or null if this step did not fail Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}
/// The parameter information
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDto {
    /// The description of the Parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether or not the Parameter is inherited from another context Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    /// The name of the Parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    /// Whether or not the Parameter is provided by a ParameterProvider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided: Option<bool>,
    /// A list of identifiers of the assets that are referenced by the parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_assets: Option<Vec<AssetReferenceDto>>,
    /// The set of all components in the flow that are referencing this Parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_components: Option<Vec<AffectedComponentEntity>>,
    /// Whether or not the Parameter is sensitive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    /// The value of the Parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Whether or not the value of the Parameter was removed.
    /// When a request is made to change a parameter, the value may be null.
    /// The absence of the value may be used either to indicate that the value is not to be changed, or that the value is to be set to null (i.e., removed).
    /// This denotes which of the two scenarios is being encountered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_removed: Option<bool>,
}
/// The name of the Parameter
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterEntity {
    /// Indicates whether the user can write a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_write: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// The name of the ParameterContext that receives the parameters in this group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context_name: Option<String>,
    /// All fetched parameter names that should be applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_sensitivities: Option<
        std::collections::HashMap<
            String,
            Option<ParameterGroupConfigurationEntityParameterSensitivities>,
        >,
    >,
    /// True if this group should be synchronized to a ParameterContext, including creating one if it does not exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronized: Option<bool>,
}
/// The steps that are required in order to complete the request, along with the status of each
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderApplyParametersUpdateStepDto {
    /// Whether or not this step has completed Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// Explanation of what happens in this step Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An explanation of why this step failed, or null if this step did not fail Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderConfigurationDto {
    /// The Parameter Group name that maps to the Parameter Context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    /// The ID of the Parameter Provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_id: Option<String>,
    /// The name of the Parameter Provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider_name: Option<String>,
    /// True if the Parameter Context should receive the parameters from the mapped Parameter Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronized: Option<bool>,
}
/// Optional configuration for a Parameter Provider
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderConfigurationEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ParameterProviderConfigurationDto>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_components: Option<Vec<AffectedComponentEntity>>,
    /// The annotation data for the parameter provider. This is how the custom UI relays configuration to the parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<BundleDto>,
    /// The comments of the parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The URL for the custom configuration UI for the parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ui_url: Option<String>,
    /// Whether the parameter provider has been deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// The descriptors for the parameter providers properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Whether the underlying extension is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_missing: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the parameter provider has multiple versions available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_versions_available: Option<bool>,
    /// The name of the parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Configuration for any fetched parameter groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_configurations: Option<Vec<ParameterGroupConfigurationEntity>>,
    /// The status of all provided parameters for this parameter provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_status: Option<Vec<ParameterStatusDto>>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// Whether the parameter provider persists state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persists_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The properties of the parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The Parameter Contexts that reference this Parameter Provider Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referencing_parameter_contexts: Option<Vec<ParameterProviderReferencingComponentEntity>>,
    /// Whether the parameter provider requires elevated privileges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// The fully qualified type of the parameter provider.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Gets the validation errors from the parameter provider. These validation errors represent the problems with the parameter provider that must be resolved before it can be scheduled to run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Parameter Provider is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Parameter Provider is valid) Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<ParameterProviderDtoValidationStatus>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ParameterProviderDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Bundle>,
    /// The identifier of the parameter provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The name of the parameter provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The fully qualified name of the parameter provider class.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReferencingComponentDto {
    /// The id of the component referencing a parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the component referencing a parameter provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// The Parameter Contexts that reference this Parameter Provider
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterProviderReferencingComponentEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ParameterProviderReferencingComponentDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<ParameterEntity>,
    /// Indicates the status of the parameter, compared to the existing parameter context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ParameterStatusDtoStatus>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerDto {
    /// The number of flowFiles this peer holds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_count: Option<i32>,
    /// The hostname of this peer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The port number of this peer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// Returns if this peer connection is secure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
}
/// The permissions for this component.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionsDto {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    /// Indicates whether the user can write a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remote_access: Option<bool>,
    /// The comments for the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The number of tasks that should be concurrently scheduled for the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrently_schedulable_task_count: Option<i32>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// Specifies how the Port functions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_function: Option<PortDtoPortFunction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The state of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<PortDtoState>,
    /// Whether the port has incoming or output connections to a remote NiFi. This is only applicable when the port is allowed to be accessed remotely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitting: Option<bool>,
    /// The type of port.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PortDtoType>,
    /// Gets the validation errors from this port. These validation errors represent the problems with the port that must be resolved before it can be started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}
/// The output ports in this flow.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortEntity {
    /// Whether this port can be accessed remotely via Site-to-Site protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remote_access: Option<bool>,
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<PortDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PortStatusDto>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    /// The run status of the Port.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<PortStatusSnapshotDto>,
    /// The id of the parent process group of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodePortStatusSnapshotDto>>,
    /// The run status of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<PortStatusDtoRunStatus>,
    /// The time the status for the process group was last refreshed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_last_refreshed: Option<String>,
    /// Whether the port has incoming or outgoing connections to a remote NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The size of hte FlowFiles that have been accepted in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_in: Option<i64>,
    /// The number of bytes that have been processed in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_out: Option<i64>,
    /// The number of FlowFiles that have been accepted in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_in: Option<i32>,
    /// The number of FlowFiles that have been processed in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_out: Option<i32>,
    /// The id of the parent process group of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The count/size of flowfiles that have been accepted in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// The name of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The count/size of flowfiles that have been processed in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// The run status of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<PortStatusSnapshotDtoRunStatus>,
    /// Whether the port has incoming or outgoing connections to a remote NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitting: Option<bool>,
}
/// The status of all output ports in the process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PortStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    /// The id of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_status_snapshot: Option<PortStatusSnapshotDto>,
}
/// The position of a component on the graph
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// The x coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// The y coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}
/// The position of this component in the UI if applicable.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionDto {
    /// The x coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// The y coordinate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}
/// Previous values for a given property.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviousValueDto {
    /// The previous value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_value: Option<String>,
    /// The timestamp when the value was modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The user who changed the previous value.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_remote_port_count: Option<i32>,
    /// The comments for the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<FlowSnippetDto>,
    /// Default value used in this Process Group for the maximum data size of objects that can be queued before back pressure is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_data_size_threshold: Option<String>,
    /// Default value used in this Process Group for the maximum number of objects that can be queued before back pressure is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_object_threshold: Option<i64>,
    /// The default FlowFile Expiration for this Process Group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_flow_file_expiration: Option<String>,
    /// The number of disabled components in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_count: Option<i32>,
    /// The Execution Engine that should be used to run the flow represented by this Process Group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_engine: Option<ProcessGroupDtoExecutionEngine>,
    /// The FlowFile Concurrency for this Process Group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flowfile_concurrency: Option<ProcessGroupDtoFlowfileConcurrency>,
    /// The Outbound Policy that is used for determining how FlowFiles should be transferred out of the Process Group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flowfile_outbound_policy: Option<ProcessGroupDtoFlowfileOutboundPolicy>,
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
    /// The log file suffix for this Process Group for dedicated logging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_suffix: Option<String>,
    /// The maximum number of concurrent tasks to use when running the flow using the Stateless Engine
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_tasks: Option<i32>,
    /// The name of the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of output ports in the process group. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_port_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The number of public input ports in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_input_port_count: Option<i32>,
    /// The number of public output ports in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_output_port_count: Option<i32>,
    /// The number of running components in this process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    /// The number of stale versioned process groups in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stale_count: Option<i32>,
    /// The maximum amount of time that the flow can be run using the Stateless Engine before the flow times out
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_flow_timeout: Option<String>,
    /// If the Process Group is configured to run in using the Stateless Engine, represents the current state. Otherwise, will be STOPPED.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_group_scheduled_state: Option<ProcessGroupDtoStatelessGroupScheduledState>,
    /// The number of stopped components in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_count: Option<i32>,
    /// The number of versioned process groups in the process group that are unable to sync to a registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_failure_count: Option<i32>,
    /// The number of up to date versioned process groups in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_to_date_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_control_information: Option<VersionControlInformationDto>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupFlowDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breadcrumb: Option<FlowBreadcrumbEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<FlowDto>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The time the flow for the process group was last refreshed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refreshed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context: Option<ParameterContextReferenceEntity>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
/// The Process Group that the component belongs to
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupNameDto {
    /// The ID of the Process Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the Process Group, or the ID of the Process Group if the user does not have the READ policy for the Process Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// The Process Group Change Request
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupReplaceRequestDto {
    /// Whether or not this request has completed Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// An explanation of why this request failed, or null if this request has not failed Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// The last time this request was updated. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// The percentage complete for the request, between 0 and 100 Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// The unique ID of the Process Group being updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_id: Option<String>,
    /// The unique ID of this request. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The state of the request Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The URI for future requests to this drop request. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
/// The status of the process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<ProcessGroupStatusSnapshotDto>,
    /// The ID of the Process Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the Process Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The status reported by each node in the cluster. If the NiFi instance is a standalone instance, rather than a clustered instance, this value may be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeProcessGroupStatusSnapshotDto>>,
    /// The time the status for the process group was last refreshed.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The number of bytes that have come into this ProcessGroup in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_in: Option<i64>,
    /// The number of bytes transferred out of this ProcessGroup in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_out: Option<i64>,
    /// The number of bytes that are queued up in this ProcessGroup right now
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_queued: Option<i64>,
    /// The number of bytes read by components in this ProcessGroup in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_read: Option<i64>,
    /// The number of bytes received from external sources by components within this ProcessGroup in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_received: Option<i64>,
    /// The number of bytes sent to an external sink by components within this ProcessGroup in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_sent: Option<i64>,
    /// The number of bytes transferred in this ProcessGroup in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_transferred: Option<i64>,
    /// The number of bytes written by components in this ProcessGroup in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_written: Option<i64>,
    /// The status of all connections in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status_snapshots: Option<Vec<ConnectionStatusSnapshotEntity>>,
    /// The number of FlowFiles that have come into this ProcessGroup in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_in: Option<i32>,
    /// The number of FlowFiles transferred out of this ProcessGroup in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_out: Option<i32>,
    /// The number of FlowFiles that are queued up in this ProcessGroup right now
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_queued: Option<i32>,
    /// The number of FlowFiles received from external sources by components within this ProcessGroup in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_received: Option<i32>,
    /// The number of FlowFiles sent to an external sink by components within this ProcessGroup in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_sent: Option<i32>,
    /// The number of FlowFiles transferred in this ProcessGroup in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_transferred: Option<i32>,
    /// The id of the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The input count/size for the process group in the last 5 minutes (pretty printed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// The status of all input ports in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_port_status_snapshots: Option<Vec<PortStatusSnapshotEntity>>,
    /// The name of this process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The output count/size for the process group in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// The status of all output ports in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_port_status_snapshots: Option<Vec<PortStatusSnapshotEntity>>,
    /// The status of all process groups in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_status_snapshots: Option<Vec<ProcessGroupStatusSnapshotEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_nanos: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_performance_status: Option<ProcessingPerformanceStatusDto>,
    /// The status of all processors in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_status_snapshots: Option<Vec<ProcessorStatusSnapshotEntity>>,
    /// The count/size that is queued in the the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<String>,
    /// The count that is queued for the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_count: Option<String>,
    /// The size that is queued for the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_size: Option<String>,
    /// The number of bytes read in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<String>,
    /// The count/size sent to the process group in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received: Option<String>,
    /// The status of all remote process groups in the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_group_status_snapshots: Option<Vec<RemoteProcessGroupStatusSnapshotEntity>>,
    /// The count/size sent from this process group in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<String>,
    /// The current number of active threads for the Process Group, when running in Stateless mode. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_active_thread_count: Option<i32>,
    /// The number of threads currently terminated for the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated_thread_count: Option<i32>,
    /// The count/size transferred to/from queues in the process group in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred: Option<String>,
    /// The current state of the Process Group, as it relates to the Versioned Flow Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_state: Option<ProcessGroupStatusSnapshotDtoVersionedFlowState>,
    /// The number of bytes written in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub written: Option<String>,
}
/// The status of all process groups in the process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessGroupStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    /// The id of the process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_status_snapshot: Option<ProcessGroupStatusSnapshotDto>,
}
/// Represents the processor's processing performance.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessingPerformanceStatusDto {
    /// The number of nanoseconds has spent to read content in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_read_duration: Option<i64>,
    /// The number of nanoseconds has spent to write content in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_write_duration: Option<i64>,
    /// The number of nanoseconds has spent on CPU usage in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_duration: Option<i64>,
    /// The number of nanoseconds has spent running garbage collection in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub garbage_collection_duration: Option<i64>,
    /// The unique ID of the process group that the Processor belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The number of nanoseconds has spent running to commit sessions the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_commit_duration: Option<i64>,
}
/// The configuration details for the processor. These details will be included in a response if the verbose flag is included in a request.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorConfigDto {
    /// The annotation data for the processor used to relay configuration between a custom UI and the procesosr.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    /// The names of all relationships that cause a flow file to be terminated if the relationship is not connected elsewhere. This property differs from the 'isAutoTerminate' property of the RelationshipDTO in that the RelationshipDTO is meant to depict the current configuration, whereas this property can be set in a DTO when updating a Processor in order to change which Relationships should be auto-terminated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_terminated_relationships: Option<Vec<String>>,
    /// Determines whether the FlowFile should be penalized or the processor should be yielded between retries. Possible returned values: PENALIZE_FLOWFILE, YIELD_PROCESSOR. See BackoffMechanism.class for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff_mechanism: Option<String>,
    /// The level at which the processor will report bulletins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletin_level: Option<String>,
    /// The comments for the processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The number of tasks that should be concurrently schedule for the processor. If the processor doesn't allow parallol processing then any positive input will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrently_schedulable_task_count: Option<i32>,
    /// The URL for the processor's custom configuration UI if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ui_url: Option<String>,
    /// Maps default values for concurrent tasks for each applicable scheduling strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_concurrent_tasks: Option<std::collections::HashMap<String, Option<String>>>,
    /// Maps default values for scheduling period for each applicable scheduling strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_period: Option<std::collections::HashMap<String, Option<String>>>,
    /// Descriptors for the processor's properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Indicates the node where the process will execute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_node: Option<String>,
    /// Whether the processor is loss tolerant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loss_tolerant: Option<bool>,
    /// Maximum amount of time to be waited during a retry period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_backoff_period: Option<String>,
    /// The amount of time that is used when the process penalizes a flowfile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalty_duration: Option<String>,
    /// The properties for the processor. Properties whose value is not set will only contain the property name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// All the relationships should be retried.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retried_relationships: Option<Vec<String>>,
    /// Overall number of retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i32>,
    /// The run duration for the processor in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_duration_millis: Option<i64>,
    /// The frequency with which to schedule the processor. The format of the value will depend on th value of schedulingStrategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_period: Option<String>,
    /// Indicates how the processor should be scheduled to run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    /// Set of sensitive dynamic property names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// The amount of time that must elapse before this processor is scheduled again after yielding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_duration: Option<String>,
}
/// A description of how to configure the Processor to perform the task described in the use case
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorConfiguration {
    /// A description of how the Processor should be configured in order to accomplish the use case
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// The fully qualified classname of the Processor that should be used to accomplish the use case
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "RUN_ONCE")]
    RunOnce,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<BundleDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<ProcessorConfigDto>,
    /// Whether the processor has been deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// The description of the processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Indicates if the execution node of a processor is restricted to run only on the primary node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_node_restricted: Option<bool>,
    /// Whether the underlying extension is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_missing: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The input requirement for this processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_requirement: Option<String>,
    /// Whether the processor has multiple versions available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_versions_available: Option<bool>,
    /// The name of the processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// Whether the processor persists state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persists_state: Option<bool>,
    /// The physical state of the processor, including transition states
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_state: Option<ProcessorDtoPhysicalState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The available relationships that the processor currently supports. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<RelationshipDto>>,
    /// Whether the processor requires elevated privileges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// The state of the processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ProcessorDtoState>,
    /// Styles for the processor (background-color : #eee).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether the processor supports batching. This makes the run duration settings available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_batching: Option<bool>,
    /// Whether the processor supports parallel processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_parallel_processing: Option<bool>,
    /// Whether the processor supports sensitive dynamic properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The type of the processor.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The validation errors for the processor. These validation errors represent the problems with the processor that must be resolved before it can be started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Processor is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Processor is valid) Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<ProcessorDtoValidationStatus>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ProcessorDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The input requirement for this processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_requirement: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    /// The physical state of the processor, including transition states
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_state: Option<ProcessorEntityPhysicalState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ProcessorStatusDto>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The ID of the processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The run status of the processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<ProcessorRunStatusDetailsDtoRunStatus>,
    /// The processor's validation errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorRunStatusDetailsEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<ProcessorStatusSnapshotDto>,
    /// The unique ID of the process group that the Processor belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The unique ID of the Processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the Processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeProcessorStatusSnapshotDto>>,
    /// The run status of the Processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<ProcessorStatusDtoRunStatus>,
    /// The timestamp of when the stats were last refreshed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_last_refreshed: Option<String>,
    /// The type of the Processor
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The size of the FlowFiles that have been accepted in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_in: Option<i64>,
    /// The size of the FlowFiles transferred to a Connection in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_out: Option<i64>,
    /// The number of bytes read by this Processor in the last 5 mintues
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_read: Option<i64>,
    /// The number of bytes written by this Processor in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_written: Option<i64>,
    /// Indicates the node where the process will execute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_node: Option<ProcessorStatusSnapshotDtoExecutionNode>,
    /// The number of FlowFiles that have been accepted in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_in: Option<i32>,
    /// The number of FlowFiles transferred to a Connection in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_out: Option<i32>,
    /// The id of the parent process group to which the processor belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The count/size of flowfiles that have been accepted in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// The name of the prcessor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The count/size of flowfiles that have been processed in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_performance_status: Option<ProcessingPerformanceStatusDto>,
    /// The number of bytes read in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<String>,
    /// The state of the processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<ProcessorStatusSnapshotDtoRunStatus>,
    /// The number of times this Processor has run in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_count: Option<i32>,
    /// The total number of task this connectable has completed over the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<String>,
    /// The total duration of all tasks for this connectable over the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks_duration: Option<String>,
    /// The number of nanoseconds that this Processor has spent running in the last 5 minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks_duration_nanos: Option<i64>,
    /// The number of threads currently terminated for the processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminated_thread_count: Option<i32>,
    /// The type of the processor.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The number of bytes written in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub written: Option<String>,
}
/// The status of all processors in the process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    /// The id of the processor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_status_snapshot: Option<ProcessorStatusSnapshotDto>,
}
/// A list of the allowable values for the property
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyAllowableValue {
    /// The description of the value, e.g., the behavior it produces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The display name of the value, if different from the internal value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The internal value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
/// The dependencies that this property has on other properties
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDependency {
    /// The values that satisfy the dependency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_values: Option<Vec<String>>,
    /// The name of the property that is depended upon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_display_name: Option<String>,
    /// The name of the property that is depended upon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,
}
/// A list of dependencies that must be met in order for this Property to be relevant. If any of these dependencies is not met, the property described by this Property Descriptor is not relevant.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDependencyDto {
    /// The values for the property that satisfies the dependency, or null if the dependency is satisfied by the presence of any value for the associated property name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependent_values: Option<Vec<String>>,
    /// The name of the property that is being depended upon
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowable_values: Option<Vec<PropertyAllowableValue>>,
    /// The default value if a user-set value is not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// The dependencies that this property has on other properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<PropertyDependency>>,
    /// The description of what the property does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The display name of the property key, if different from the name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Whether or not the descriptor is for a dynamically added property
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
    /// The scope of expression language supported by this property
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_language_scope: Option<PropertyDescriptorExpressionLanguageScope>,
    /// The description of the expression language scope supported by this property Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_language_scope_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_port_definition: Option<PropertyListenPortDefinition>,
    /// The name of the property key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether or not  the property is required for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_definition: Option<PropertyResourceDefinition>,
    /// Whether or not  the value of the property is considered sensitive (e.g., passwords and keys)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_provided_by_value: Option<DefinedType>,
    /// A regular expression that can be used to validate the value of this property
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_regex: Option<String>,
    /// Name of the validator used for this property descriptor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<String>,
}
/// The descriptors for the reporting tasks properties.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptorDto {
    /// Allowable values for the property. If empty then the allowed values are not constrained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowable_values: Option<Vec<AllowableValueEntity>>,
    /// The default value for the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// A list of dependencies that must be met in order for this Property to be relevant. If any of these dependencies is not met, the property described by this Property Descriptor is not relevant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<PropertyDependencyDto>>,
    /// The description for the property. Used to relay additional details to a user or provide a mechanism of documenting intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The human readable name for the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Whether the property is dynamic (user-defined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
    /// Scope of the Expression Language evaluation for the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_language_scope: Option<String>,
    /// If the property identifies a controller service this returns the fully qualified type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifies_controller_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifies_controller_service_bundle: Option<BundleDto>,
    /// The name for the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the property is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Whether the property is sensitive and protected whenever stored or represented.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    /// Whether the property supports expression language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_el: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptorEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptor: Option<PropertyDescriptorDto>,
}
/// The history for the properties of the component.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PropertyHistoryDto {
    /// Previous values for a given property.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_protocols: Option<Vec<String>>,
    /// The transport protocol used by this listen port
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<PropertyResourceDefinitionCardinality>,
    /// The types of resources that can be referenced
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<PropertyResourceDefinitionResourceTypes>>,
}
/// The links between the nodes in the lineage.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceLinkDto {
    /// The flowfile uuid that traversed the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_uuid: Option<String>,
    /// The timestamp of this link in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub millis: Option<i64>,
    /// The source node id of the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// The target node id of the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// The timestamp of the link (based on the destination).
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_uuids: Option<Vec<String>>,
    /// The identifier of the node that this event/flowfile originated from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_identifier: Option<String>,
    /// If the type is EVENT, this is the type of the component that generated the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    /// If the type is EVENT, this is the type of event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    /// The uuid of the flowfile associated with the provenance event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_uuid: Option<String>,
    /// The id of the node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The timestamp of the node in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub millis: Option<i64>,
    /// The uuid of the parent flowfiles of the provenance event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_uuids: Option<Vec<String>>,
    /// The timestamp of the node formatted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// The type of the node.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ProvenanceNodeDtoType>,
}
/// The provenance request.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceRequestDto {
    /// The id of the node in the cluster where this provenance originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The latest event time to include in the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Whether or not incremental results are returned. If false, provenance events are only returned once the query completes. This property is true by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incremental_results: Option<bool>,
    /// The maximum number of results to include.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The maximum file size to include in the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_file_size: Option<String>,
    /// The minimum file size to include in the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_file_size: Option<String>,
    /// The search terms used to perform the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_terms: Option<std::collections::HashMap<String, Option<ProvenanceSearchValueDto>>>,
    /// The earliest event time to include in the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Whether or not to summarize provenance events returned. This property is false by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summarize: Option<bool>,
}
/// The provenance results.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceResultsDto {
    /// Any errors that occurred while performing the provenance request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// Then the search was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<String>,
    /// The oldest event available in the provenance repository.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_event: Option<String>,
    /// The provenance events that matched the search criteria.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance_events: Option<Vec<ProvenanceEventDto>>,
    /// The time offset of the server that's used for event time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_offset: Option<i32>,
    /// The total number of results formatted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<String>,
    /// The total number of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}
/// The search terms used to perform the search.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceSearchValueDto {
    /// Query for all except for search value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverse: Option<bool>,
    /// The search value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
/// The available searchable field for the NiFi.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvenanceSearchableFieldDto {
    /// The searchable field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// The id of the searchable field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The label for the searchable field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The type of the searchable field.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
/// The size of the queue
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueSizeDto {
    /// The size of objects in a queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_count: Option<i64>,
    /// The count of objects in a queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<i32>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<FlowRegistryPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_info: Option<RegisteredFlowVersionInfo>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlowSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<FlowRegistryBucket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_controller_services:
        Option<std::collections::HashMap<String, Option<ExternalControllerServiceReference>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<RegisteredFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_contents: Option<VersionedProcessGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_encoding_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_contexts:
        Option<std::collections::HashMap<String, Option<VersionedParameterContext>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_providers:
        Option<std::collections::HashMap<String, Option<ParameterProviderReference>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_metadata: Option<RegisteredFlowSnapshotMetadata>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlowSnapshotMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredFlowVersionInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
/// The supported relationships for this processor.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    /// The description of the relationship
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the relationship
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// The available relationships that the processor currently supports.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipDto {
    /// Whether or not flowfiles sent to this relationship should auto terminate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_terminate: Option<bool>,
    /// The relationship description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The relationship name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether or not flowfiles sent to this relationship should retry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<bool>,
}
/// The contents of the remote process group. Will contain available input/output ports.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupContentsDto {
    /// The input ports to which data can be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<RemoteProcessGroupPortDto>>,
    /// The output ports from which data can be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<RemoteProcessGroupPortDto>>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupDto {
    /// The number of active remote input ports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_remote_input_port_count: Option<i32>,
    /// The number of active remote output ports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_remote_output_port_count: Option<i32>,
    /// Any remote authorization issues for the remote process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_issues: Option<Vec<String>>,
    /// The comments for the remote process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The time period used for the timeout when communicating with the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communications_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<RemoteProcessGroupContentsDto>,
    /// The timestamp when this remote process group was last refreshed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_refreshed: Option<String>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The number of inactive remote input ports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_remote_input_port_count: Option<i32>,
    /// The number of inactive remote output ports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_remote_output_port_count: Option<i32>,
    /// The number of remote input ports currently available on the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_port_count: Option<i32>,
    /// The local network interface to send/receive data. If not specified, any local address is used. If clustered, all nodes must have an interface with this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_network_interface: Option<String>,
    /// The name of the remote process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of remote output ports currently available on the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_port_count: Option<i32>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_user: Option<String>,
    /// Whether the target is running securely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_secure: Option<bool>,
    /// The target URI of the remote process group. If target uri is not set, but uris are set, then returns the first url in the urls. If neither target uri nor uris are set, then returns null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_uri: Option<String>,
    /// The target URI of the remote process group. If target uris is not set but target uri is set, then returns a collection containing the single target uri. If neither target uris nor uris are set, then returns null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_uris: Option<String>,
    /// Whether the remote process group is actively transmitting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<String>,
    /// The validation errors for the remote process group.
    /// These validation errors represent the problems with the remote process group that must be resolved before it can transmit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
    /// When yielding, this amount of time must elapse before the remote process group is scheduled again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_duration: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<RemoteProcessGroupDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The number of remote input ports currently available on the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_port_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_permissions: Option<PermissionsDto>,
    /// The number of remote output ports currently available on the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_port_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RemoteProcessGroupStatusDto>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
/// The output ports from which data can be retrieved.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupPortDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_settings: Option<BatchSettingsDto>,
    /// The comments as configured on the target port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The number of task that may transmit flowfiles to the target port concurrently.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrently_schedulable_task_count: Option<i32>,
    /// Whether the port has either an incoming or outgoing connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    /// Whether the target port exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    /// The id of the remote process group that the port resides in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the target port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of the target port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// Whether the target port is running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_running: Option<bool>,
    /// Whether the remote port is configured for transmission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmitting: Option<bool>,
    /// Whether the flowfiles are compressed when sent to the target port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_compression: Option<bool>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshot: Option<RemoteProcessGroupStatusSnapshotDto>,
    /// The unique ID of the process group that the Processor belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The unique ID of the Processor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the remote process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A status snapshot for each node in the cluster. If the NiFi instance is a standalone instance, rather than a cluster, this may be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeRemoteProcessGroupStatusSnapshotDto>>,
    /// The time the status for the process group was last refreshed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_last_refreshed: Option<String>,
    /// The URI of the target system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_uri: Option<String>,
    /// The transmission status of the remote process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmission_status: Option<String>,
    /// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid) Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<RemoteProcessGroupStatusDtoValidationStatus>,
}
/// The remote process group status snapshot from the node.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusSnapshotDto {
    /// The number of active threads for the remote process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The size of the FlowFiles received from the remote process group in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_received: Option<i64>,
    /// The size of the FlowFiles sent to the remote process group in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_sent: Option<i64>,
    /// The number of FlowFiles received from the remote process group in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_received: Option<i32>,
    /// The number of FlowFiles sent to the remote process group in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_files_sent: Option<i32>,
    /// The id of the parent process group the remote process group resides in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The id of the remote process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the remote process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The count/size of the flowfiles received from the remote process group in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received: Option<String>,
    /// The count/size of the flowfiles sent to the remote process group in the last 5 minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent: Option<String>,
    /// The URI of the target system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_uri: Option<String>,
    /// The transmission status of the remote process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transmission_status: Option<String>,
}
/// The status of all remote process groups in the process group.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteProcessGroupStatusSnapshotEntity {
    /// Indicates whether the user can read a given resource. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read: Option<bool>,
    /// The id of the remote process group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_group_status_snapshot: Option<RemoteProcessGroupStatusSnapshotDto>,
}
/// The snapshot from the node
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayLastEventSnapshotDto {
    /// Whether or not an event was available. This may not be populated if there was a failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_available: Option<bool>,
    /// The IDs of the events that were successfully replayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_replayed: Option<Vec<i64>>,
    /// If unable to replay an event, specifies why the event could not be replayed
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The annotation data for the repoting task. This is how the custom UI relays configuration to the reporting task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<BundleDto>,
    /// The comments of the reporting task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The URL for the custom configuration UI for the reporting task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ui_url: Option<String>,
    /// The default scheduling period for the different scheduling strategies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_period: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether the reporting task has been deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// The descriptors for the reporting tasks properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptors: Option<std::collections::HashMap<String, Option<PropertyDescriptorDto>>>,
    /// Whether the underlying extension is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension_missing: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether the reporting task has multiple versions available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_versions_available: Option<bool>,
    /// The name of the reporting task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// Whether the reporting task persists state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persists_state: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The properties of the reporting task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether the reporting task requires elevated privileges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    /// The frequency with which to schedule the reporting task. The format of the value will depend on the value of the schedulingStrategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_period: Option<String>,
    /// The scheduling strategy that determines how the schedulingPeriod value should be interpreted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    /// Set of sensitive dynamic property names
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_dynamic_property_names: Option<Vec<String>>,
    /// The state of the reporting task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ReportingTaskDtoState>,
    /// Whether the reporting task supports sensitive dynamic properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_sensitive_dynamic_properties: Option<bool>,
    /// The fully qualified type of the reporting task.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Gets the validation errors from the reporting task. These validation errors represent the problems with the reporting task that must be resolved before it can be scheduled to run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<String>>,
    /// Indicates whether the Reporting Task is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the Reporting Task is valid) Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<ReportingTaskDtoValidationStatus>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportingTaskEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<ReportingTaskDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PermissionsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReportingTaskStatusDto>,
    /// The URI for futures requests to the component.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_thread_count: Option<i32>,
    /// The run status of this ReportingTask Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_status: Option<ReportingTaskStatusDtoRunStatus>,
    /// Indicates whether the component is valid, invalid, or still in the process of validating (i.e., it is unknown whether or not the component is valid) Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<ReportingTaskStatusDtoValidationStatus>,
}
/// The required permission necessary for this restriction.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequiredPermissionDto {
    /// The required sub-permission necessary for this restriction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The label for the required sub-permission necessary for this restriction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceClaimDetailsDto {
    /// Whether or not the Resource Claim is awaiting destruction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awaiting_destruction: Option<bool>,
    /// The number of FlowFiles that have a claim to the Resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claimant_count: Option<i32>,
    /// The container of the Content Repository in which the Resource Claim exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// The identifier of the Resource Claim
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Whether or not the Resource Claim is in use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use: Option<bool>,
    /// The section of the Content Repository in which the Resource Claim exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    /// Whether or not the Resource Claim can still have more data written to it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writable: Option<bool>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceDto {
    /// The identifier of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The name of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// Explicit restrictions that indicate a require permission to use the component
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Restriction {
    /// The explanation of this restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// The permission required for this restriction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_permission: Option<String>,
}
/// The revision of the Process Group
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevisionDto {
    /// A client identifier used to make a request.
    /// By including a client identifier, the API can allow multiple requests without needing the current revision.
    /// Due to the asynchronous nature of requests/responses this was implemented to allow the client to make numerous requests without having to wait for the previous response to come back
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// The user that last modified the flow. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modifier: Option<String>,
    /// NiFi employs an optimistic locking strategy where the client must include a revision in their request when performing an update.
    /// In a response to a mutable flow request, this field represents the updated base version.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_concurrent_tasks_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<i32>>>,
    /// The default concurrent tasks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_max_concurrent_tasks: Option<String>,
    /// The default run duration in nano-seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_run_duration_nanos: Option<i64>,
    /// The default scheduling period in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_period_millis: Option<i64>,
    /// The default scheduling period for each scheduling strategy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_periods_by_scheduling_strategy:
        Option<std::collections::HashMap<String, Option<String>>>,
    /// The name of the default scheduling strategy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_scheduling_strategy: Option<SchedulingDefaultsDefaultSchedulingStrategy>,
    /// The default penalization period in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalization_period_millis: Option<i64>,
    /// The default yield duration in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_duration_millis: Option<i64>,
}
/// The nearest versioned ancestor group of the component that matched the search.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultGroupDto {
    /// The id of the group.
    pub id: String,
    /// The name of the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
/// The snippet.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnippetDto {
    /// The ids of the connections in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The ids of the funnels in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnels: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The id of the snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ids of the input ports in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The ids of the labels in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The ids of the output ports in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The group id for the components in the snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    /// The ids of the process groups in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The ids of the processors in this snippet. These ids will be populated within each response. They can be specified when creating a snippet. However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The ids of the remote process groups in this snippet.
    /// These ids will be populated within each response.
    /// They can be specified when creating a snippet.
    /// However, once a snippet has been created its contents cannot be modified (these ids are ignored during update requests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<std::collections::HashMap<String, Option<RevisionDto>>>,
    /// The URI of the snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
/// The state.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateEntryDto {
    /// The label for the node where the state originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_address: Option<String>,
    /// The identifier for the node where the state originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_node_id: Option<String>,
    /// The key for this state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The value for this state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
/// The local state for this component.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StateMapDto {
    /// The scope of this StateMap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// The state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Vec<StateEntryDto>>,
    /// The total number of state entries. When the state map is lengthy, only of portion of the entries are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Indicates the Scope(s) associated with the State that is stored and retrieved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<StatefulScopes>>,
}
/// The Descriptors that provide information on each of the metrics provided in the status history
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusDescriptorDto {
    /// The description of the status field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the status field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// The formatter for the status descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<String>,
    /// The label for the status field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusHistoryDto {
    /// A list of StatusSnapshotDTO objects that provide the actual metric values for the component. If the NiFi instance is clustered, this will represent the aggregate status across all nodes. If the NiFi instance is not clustered, this will represent the status of the entire NiFi instance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_snapshots: Option<Vec<StatusSnapshotDto>>,
    /// A Map of key/value pairs that describe the component that the status history belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_details: Option<std::collections::HashMap<String, Option<String>>>,
    /// The Descriptors that provide information on each of the metrics provided in the status history
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_descriptors: Option<Vec<StatusDescriptorDto>>,
    /// When the status history was generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated: Option<String>,
    /// The NodeStatusSnapshotsDTO objects that provide the actual metric values for the component, for each node. If the NiFi instance is not clustered, this value will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<Vec<NodeStatusSnapshotsDto>>,
}
/// A list of StatusSnapshotDTO objects that provide the actual metric values for the component for this node.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusSnapshotDto {
    /// The status metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_metrics: Option<std::collections::HashMap<String, Option<i64>>>,
    /// The timestamp of the snapshot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}
/// The provenance repository storage usage.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageUsageDto {
    /// Amount of free space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_space: Option<String>,
    /// The number of bytes of free space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_space_bytes: Option<i64>,
    /// The identifier of this storage location. The identifier will correspond to the identifier keyed in the storage configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Amount of total space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_space: Option<String>,
    /// The number of bytes of total space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_space_bytes: Option<i64>,
    /// Amount of used space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_space: Option<String>,
    /// The number of bytes of used space.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_space_bytes: Option<i64>,
    /// Utilization of this storage location.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The mime types this Content Viewer supports. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_types: Option<Vec<String>>,
}
/// The System Diagnostics snapshot from the node.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemDiagnosticsSnapshotDto {
    /// Number of available processors if supported by the underlying system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_processors: Option<i32>,
    /// The content repository storage usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_repository_storage_usage: Option<Vec<StorageUsageDto>>,
    /// Number of daemon threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daemon_threads: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_repository_storage_usage: Option<StorageUsageDto>,
    /// Amount of free heap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_heap: Option<String>,
    /// The number of bytes that are allocated to the JVM heap but not currently being used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_heap_bytes: Option<i64>,
    /// Amount of free non heap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_non_heap: Option<String>,
    /// Total number of free non-heap bytes available to the JVM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_non_heap_bytes: Option<i64>,
    /// The garbage collection details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub garbage_collection: Option<Vec<GarbageCollectionDto>>,
    /// Utilization of heap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heap_utilization: Option<String>,
    /// Maximum size of heap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_heap: Option<String>,
    /// The maximum number of bytes that can be used by the JVM
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_heap_bytes: Option<i64>,
    /// Maximum size of non heap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_non_heap: Option<String>,
    /// The maximum number of bytes that the JVM can use for non-heap purposes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_non_heap_bytes: Option<i64>,
    /// Utilization of non heap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_heap_utilization: Option<String>,
    /// The processor load average if supported by the underlying system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_load_average: Option<f64>,
    /// The provenance repository storage usage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance_repository_storage_usage: Option<Vec<StorageUsageDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_claim_details: Option<Vec<ResourceClaimDetailsDto>>,
    /// When the diagnostics were generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_last_refreshed: Option<String>,
    /// Total size of heap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_heap: Option<String>,
    /// The total number of bytes that are available for the JVM heap to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_heap_bytes: Option<i64>,
    /// Total size of non heap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_non_heap: Option<String>,
    /// Total number of bytes allocated to the JVM not used for heap
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_non_heap_bytes: Option<i64>,
    /// Total number of threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_threads: Option<i32>,
    /// The uptime of the Java virtual machine
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uptime: Option<String>,
    /// Amount of used heap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_heap: Option<String>,
    /// The number of bytes of JVM heap that are currently being used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_heap_bytes: Option<i64>,
    /// Amount of use non heap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_non_heap: Option<String>,
    /// Total number of bytes used by the JVM not in the heap space
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_non_heap_bytes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_info: Option<VersionInfoDto>,
}
/// The system resource considerations for the given component
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemResourceConsideration {
    /// The description of how the resource is affected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The resource to consider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantDto {
    /// Whether this tenant is configurable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurable: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The identity of the tenant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}
/// The set of user group IDs associated with this access policy.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantEntity {
    /// The bulletins for this component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletins: Option<Vec<BulletinEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<TenantDto>,
    /// Acknowledges that this node is disconnected to allow for mutable requests to proceed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disconnected_node_acknowledged: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// A description of the use case
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Specifies whether an incoming FlowFile is expected for this use case
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_requirement: Option<UseCaseInputRequirement>,
    /// Keywords that pertain to the use case
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    /// Any pertinent notes about the use case
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    /// The access policies this user belongs to. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<Vec<AccessPolicySummaryEntity>>,
    /// Whether this tenant is configurable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurable: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The identity of the tenant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The groups to which the user belongs. This field is read only and it provided for convenience. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_groups: Option<Vec<TenantEntity>>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupDto {
    /// The access policies this user group belongs to. This field was incorrectly defined as an AccessPolicyEntity. For compatibility reasons the field will remain of this type, however only the fields that are present in the AccessPolicySummaryEntity will be populated here. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<Vec<AccessPolicyEntity>>,
    /// Whether this tenant is configurable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurable: Option<bool>,
    /// The id of the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The identity of the tenant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// The id of parent process group of this component if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<PositionDto>,
    /// The users that belong to the user group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<TenantEntity>>,
    /// The ID of the corresponding component that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_component_id: Option<String>,
}
/// The request
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyConfigRequestDto {
    /// FlowFile Attributes that should be used to evaluate Expression Language for resolving property values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, Option<String>>>,
    /// Whether or not the request is completed Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// The ID of the component whose configuration was verified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_id: Option<String>,
    /// The reason for the request failing, or null if the request has not failed Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// The timestamp of when the request was last updated Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// A value between 0 and 100 (inclusive) indicating how close the request is to completion Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// The configured component properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The ID of the request Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The Results of the verification Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ConfigVerificationResultDto>>,
    /// A description of the current state of the request Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The timestamp of when the request was submitted Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_time: Option<String>,
    /// The steps that are required in order to complete the request, along with the status of each Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_steps: Option<Vec<VerifyConfigUpdateStepDto>>,
    /// The URI for the request Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyConfigRequestEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<VerifyConfigRequestDto>,
}
/// The steps that are required in order to complete the request, along with the status of each
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyConfigUpdateStepDto {
    /// Whether or not this step has completed Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// Explanation of what happens in this step Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An explanation of why this step failed, or null if this step did not fail Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// The ID of the bucket that the flow is stored in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_id: Option<String>,
    /// The name of the bucket that the flow is stored in Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// The description of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_description: Option<String>,
    /// The ID of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    /// The name of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_name: Option<String>,
    /// The ID of the Process Group that is under version control
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The ID of the registry that the flow is stored in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// The name of the registry that the flow is stored in Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_name: Option<String>,
    /// The current state of the Process Group, as it relates to the Versioned Flow Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<VersionControlInformationDtoState>,
    /// Explanation of why the group is in the specified state Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_explanation: Option<String>,
    /// The storage location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<String>,
    /// The version of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
/// The nifi, os, java, and build version information
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionInfoDto {
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
    /// Java JVM vendor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_vendor: Option<String>,
    /// Java version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_version: Option<String>,
    /// The version of this NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ni_fi_version: Option<String>,
    /// Host operating system architecture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_architecture: Option<String>,
    /// Host operating system name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    /// Host operating system version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
}
/// The assets that are referenced by this parameter
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedAsset {
    /// The identifier of the asset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The name of the asset
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_pressure_data_size_threshold: Option<String>,
    /// The object count threshold for determining when back pressure is applied. Updating this value is a passive change in the sense that it won't impact whether existing files over the limit are affected but it does help feeder processors to stop pushing too much into this work queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back_pressure_object_threshold: Option<i64>,
    /// The bend points on the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bends: Option<Vec<Position>>,
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<VersionedConnectionComponentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<ConnectableComponent>,
    /// The amount of time a flow file may be in the flow before it will be automatically aged out of the flow. Once a flow file reaches this age it will be terminated from the flow the next time a processor attempts to start work on it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_expiration: Option<String>,
    /// The ID of the Process Group that this component belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The index of the bend point where to place the connection label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_index: Option<i32>,
    /// Whether or not compression should be used when transferring FlowFiles between nodes Possible returned values: DO_NOT_COMPRESS, COMPRESS_ATTRIBUTES_ONLY, COMPRESS_ATTRIBUTES_AND_CONTENT. See LoadBalanceCompression.class for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balance_compression: Option<String>,
    /// The Strategy to use for load balancing data across the cluster, or null, if no Load Balance Strategy has been specified. Possible returned values: DO_NOT_LOAD_BALANCE, PARTITION_BY_ATTRIBUTE, ROUND_ROBIN, SINGLE_NODE. See LoadBalanceStrategy.class for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balance_strategy: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The attribute to use for partitioning data as it is load balanced across the cluster. If the Load Balance Strategy is configured to use PARTITION_BY_ATTRIBUTE, the value returned by this method is the name of the FlowFile Attribute that will be used to determine which node in the cluster should receive a given FlowFile. If the Load Balance Strategy is unset or is set to any other value, the Partitioning Attribute has no effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitioning_attribute: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// The comparators used to prioritize the queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prioritizers: Option<Vec<String>>,
    /// The selected relationship that comprise the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_relationships: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ConnectableComponent>,
    /// The z index of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    /// The level at which the controller service will report bulletins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletin_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Bundle>,
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<VersionedControllerServiceComponentType>,
    /// Lists the APIs this Controller Service implements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_service_apis: Option<Vec<ControllerServiceAPI>>,
    /// The ID of the Process Group that this component belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// The properties for the component. Properties whose value is not set will only contain the property name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The property descriptors for the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors:
        Option<std::collections::HashMap<String, Option<VersionedPropertyDescriptor>>>,
    /// The ScheduledState denoting whether the Controller Service is ENABLED or DISABLED
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_state: Option<VersionedControllerServiceScheduledState>,
    /// The type of the extension component
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}
/// The coordinates where the remote flow is stored, or null if the Process Group is not directly under Version Control
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowCoordinates {
    /// The name of the branch that the flow resides in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// The UUID of the bucket that the flow resides in
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_id: Option<String>,
    /// The UUID of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    /// Whether or not these coordinates point to the latest version of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest: Option<bool>,
    /// The identifier of the Flow Registry that contains the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    /// The location of the Flow Registry that stores the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<String>,
    /// The version of the flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowSnapshotMetadataEntity {
    /// The ID of the Registry that this flow belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned_flow_snapshot_metadata: Option<RegisteredFlowSnapshotMetadata>,
}
/// The Flow Update Request
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedFlowUpdateRequestDto {
    /// Whether or not this request has completed Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
    /// An explanation of why this request failed, or null if this request has not failed Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// The last time this request was updated. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// The percentage complete for the request, between 0 and 100 Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_completed: Option<i32>,
    /// The unique ID of the Process Group being updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_group_id: Option<String>,
    /// The unique ID of this request. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The state of the request Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The URI for future requests to this drop request. Read-only — set by NiFi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<VersionedFunnelComponentType>,
    /// The ID of the Process Group that this component belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<VersionedLabelComponentType>,
    /// The ID of the Process Group that this component belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The height of the label in pixels when at a 1:1 scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The text that appears in the label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// The styles for this label (font-size : 12px, background-color : #eee, etc).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    /// The width of the label in pixels when at a 1:1 scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// The z index of the connection.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_protocols: Option<Vec<String>>,
    /// The transport protocol used by the listen port
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<VersionedListenPortDefinitionTransportProtocol>,
}
/// The parameters in the context
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedParameter {
    /// The description of the param
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether or not the parameter value is provided by a ParameterProvider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provided: Option<bool>,
    /// The assets that are referenced by this parameter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referenced_assets: Option<Vec<VersionedAsset>>,
    /// Whether or not the parameter value is sensitive
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
    /// The value of the parameter
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<VersionedParameterContextComponentType>,
    /// The description of the parameter context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the Process Group that this component belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The names of additional parameter contexts from which to inherit parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_parameter_contexts: Option<Vec<String>>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The corresponding parameter group name fetched from the parameter provider, if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    /// The identifier of an optional parameter provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_provider: Option<String>,
    /// The parameters in the context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<VersionedParameter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// True if the parameter provider is set and the context should receive updates when its parameters are next fetched
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remote_access: Option<bool>,
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<VersionedPortComponentType>,
    /// The number of tasks that should be concurrently scheduled for the port.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrently_schedulable_task_count: Option<i32>,
    /// The ID of the Process Group that this component belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specifies how the Port should function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_function: Option<VersionedPortPortFunction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// The scheduled state of the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_state: Option<VersionedPortScheduledState>,
    /// The type of port.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<VersionedProcessGroupComponentType>,
    /// The Connections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<VersionedConnection>>,
    /// The Controller Services
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller_services: Option<Vec<VersionedControllerService>>,
    /// Default value used in this Process Group for the maximum data size of objects that can be queued before back pressure is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_data_size_threshold: Option<String>,
    /// Default value used in this Process Group for the maximum number of objects that can be queued before back pressure is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_back_pressure_object_threshold: Option<i64>,
    /// The default FlowFile Expiration for this Process Group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_flow_file_expiration: Option<String>,
    /// The Execution Engine that should be used to run the components within the group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_engine: Option<VersionedProcessGroupExecutionEngine>,
    /// The configured FlowFile Concurrency for the Process Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_concurrency: Option<String>,
    /// The FlowFile Outbound Policy for the Process Group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_file_outbound_policy: Option<String>,
    /// The Funnels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnels: Option<Vec<VersionedFunnel>>,
    /// The ID of the Process Group that this component belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The Input Ports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<VersionedPort>>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The Labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<VersionedLabel>>,
    /// The log file suffix for this Process Group for dedicated logging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_suffix: Option<String>,
    /// The maximum number of concurrent tasks that should be scheduled for this Process Group when using the Stateless Engine
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_tasks: Option<i32>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Output Ports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<VersionedPort>>,
    /// The name of the parameter context used by this process group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_context_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// The child Process Groups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_groups: Option<Vec<Box<VersionedProcessGroup>>>,
    /// The Processors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<VersionedProcessor>>,
    /// The Remote Process Groups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_process_groups: Option<Vec<VersionedRemoteProcessGroup>>,
    /// The Scheduled State of the Process Group, if the group is configured to use the Stateless Execution Engine. Otherwise, this value has no relevance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_state: Option<VersionedProcessGroupScheduledState>,
    /// The maximum amount of time that the flow is allows to run using the Stateless engine before it times out and is considered a failure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stateless_flow_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    /// The names of all relationships that cause a flow file to be terminated if the relationship is not connected elsewhere. This property differs from the 'isAutoTerminate' property of the RelationshipDTO in that the RelationshipDTO is meant to depict the current configuration, whereas this property can be set in a DTO when updating a Processor in order to change which Relationships should be auto-terminated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_terminated_relationships: Option<Vec<String>>,
    /// Determines whether the FlowFile should be penalized or the processor should be yielded between retries. Possible returned values: PENALIZE_FLOWFILE, YIELD_PROCESSOR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff_mechanism: Option<String>,
    /// The level at which the processor will report bulletins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulletin_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Bundle>,
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<VersionedProcessorComponentType>,
    /// The number of tasks that should be concurrently schedule for the processor. If the processor doesn't allow parallol processing then any positive input will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrently_schedulable_task_count: Option<i32>,
    /// Indicates the node where the process will execute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_node: Option<String>,
    /// The ID of the Process Group that this component belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// Maximum amount of time to be waited during a retry period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_backoff_period: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The amout of time that is used when the process penalizes a flowfile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub penalty_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// The properties for the component. Properties whose value is not set will only contain the property name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The property descriptors for the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors:
        Option<std::collections::HashMap<String, Option<VersionedPropertyDescriptor>>>,
    /// All the relationships should be retried.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retried_relationships: Option<Vec<String>>,
    /// Overall number of retries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i32>,
    /// The run duration for the processor in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_duration_millis: Option<i64>,
    /// The scheduled state of the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_state: Option<VersionedProcessorScheduledState>,
    /// The frequency with which to schedule the processor. The format of the value will depend on th value of schedulingStrategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_period: Option<String>,
    /// Indicates how the processor should be scheduled to run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    /// Stylistic data for rendering in a UI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<std::collections::HashMap<String, Option<String>>>,
    /// The type of the extension component
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The amount of time that must elapse before this processor is scheduled again after yielding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yield_duration: Option<String>,
}
/// The property descriptors for the component.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionedPropertyDescriptor {
    /// The display name of the property
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Whether or not the property is user-defined
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
    /// Whether or not the property provides the identifier of a Controller Service
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifies_controller_service: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_port_definition: Option<VersionedListenPortDefinition>,
    /// The name of the property
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_definition: Option<VersionedResourceDefinition>,
    /// Whether or not the property is considered sensitive
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<BatchSize>,
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<VersionedRemoteGroupPortComponentType>,
    /// The number of task that may transmit flowfiles to the target port concurrently.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrently_schedulable_task_count: Option<i32>,
    /// The ID of the Process Group that this component belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// The id of the remote process group that the port resides in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_group_id: Option<String>,
    /// The scheduled state of the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_state: Option<VersionedRemoteGroupPortScheduledState>,
    /// The ID of the port on the target NiFi instance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// Whether the flowfiles are compressed when sent to the target port.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The time period used for the timeout when communicating with the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub communications_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<VersionedRemoteProcessGroupComponentType>,
    /// The ID of the Process Group that this component belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// A Set of Input Ports that can be connected to, in order to send data to the remote NiFi instance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<Vec<VersionedRemoteGroupPort>>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The local network interface to send/receive data. If not specified, any local address is used. If clustered, all nodes must have an interface with this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_network_interface: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A Set of Output Ports that can be connected to, in order to pull data from the remote NiFi instance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<Vec<VersionedRemoteGroupPort>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_port: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_user: Option<String>,
    /// The target URIs of the remote process group. If target uris is not set but target uri is set, then returns the single target uri. If neither target uris nor target uri is set, then returns null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_uris: Option<String>,
    /// The Transport Protocol that is used for Site-to-Site communications. Possible returned values: RAW, HTTP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<String>,
    /// When yielding, this amount of time must elapse before the remote process group is scheduled again.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<Bundle>,
    /// The user-supplied comments for the component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<VersionedReportingTaskComponentType>,
    /// The ID of the Process Group that this component belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_identifier: Option<String>,
    /// The component's unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// The instance ID of an existing component that is described by this VersionedComponent, or null if this is not mapped to an instantiated component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identifier: Option<String>,
    /// The component's name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    /// The properties for the component. Properties whose value is not set will only contain the property name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, Option<String>>>,
    /// The property descriptors for the component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_descriptors:
        Option<std::collections::HashMap<String, Option<VersionedPropertyDescriptor>>>,
    /// Indicates the scheduled state for the Reporting Task
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_state: Option<VersionedReportingTaskScheduledState>,
    /// The frequency with which to schedule the reporting task. The format of the value will depend on the value of schedulingStrategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_period: Option<String>,
    /// Indicates scheduling strategy that should dictate how the reporting task is triggered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    /// The type of the extension component
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<VersionedResourceDefinitionCardinality>,
    /// The types of resource that the Property Descriptor is allowed to reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<VersionedResourceDefinitionResourceTypes>>,
}
